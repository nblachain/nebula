use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave96-live-heavy-gate-release-readiness-quorum-v1";
const WAVE: u64 = 96;
const PROMOTION_WAVE: u64 = 95;
const MIN_QUORUM_HEIGHT: u64 = 960_000;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, QuorumError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuorumError {
    LaneMissing,
    ClaimMissing,
    PromotedSlotRootMissing,
    EvidenceRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    QuorumHeightTooLow,
    DuplicateReleaseClaimRoot,
    QuorumStillBlocked,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    Compile,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Compile => "Compile release-readiness quorum",
            Self::RuntimeReplay => "Runtime replay release-readiness quorum",
            Self::AuditSecurity => "Audit security release-readiness quorum",
            Self::BridgeCustody => "Bridge custody release-readiness quorum",
            Self::WalletWatchtower => "Wallet watchtower release-readiness quorum",
            Self::PqReservePrivacy => "PQ reserve privacy release-readiness quorum",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuorumStatus {
    Empty,
    Blocked,
    QuorumReady,
    ReleaseReady,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuorumBlockerKind {
    MissingPromotedSlotRoot,
    MissingEvidenceRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    MissingLaneQuorumRoot,
    MissingReleaseClaimRoot,
    QuorumHeightTooLow,
    DuplicateReleaseClaimRoot,
}

impl QuorumBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingPromotedSlotRoot => "missing_promoted_slot_root",
            Self::MissingEvidenceRoot => "missing_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::MissingLaneQuorumRoot => "missing_lane_quorum_root",
            Self::MissingReleaseClaimRoot => "missing_release_claim_root",
            Self::QuorumHeightTooLow => "quorum_height_too_low",
            Self::DuplicateReleaseClaimRoot => "duplicate_release_claim_root",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub promotion_wave: u64,
    pub min_quorum_height: u64,
    pub lane_quorum_threshold: u64,
    pub global_quorum_threshold: u64,
    pub require_promoted_slot_root: bool,
    pub require_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_lane_quorum_root: bool,
    pub deny_release_when_any_lane_blocked: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            promotion_wave: PROMOTION_WAVE,
            min_quorum_height: MIN_QUORUM_HEIGHT,
            lane_quorum_threshold: 1,
            global_quorum_threshold: 6,
            require_promoted_slot_root: true,
            require_evidence_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_lane_quorum_root: true,
            deny_release_when_any_lane_blocked: true,
            roots_only_public_records: true,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "wave": self.wave,
            "promotion_wave": self.promotion_wave,
            "min_quorum_height": self.min_quorum_height,
            "lane_quorum_threshold": self.lane_quorum_threshold,
            "global_quorum_threshold": self.global_quorum_threshold,
            "require_promoted_slot_root": self.require_promoted_slot_root,
            "require_evidence_root": self.require_evidence_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_lane_quorum_root": self.require_lane_quorum_root,
            "deny_release_when_any_lane_blocked": self.deny_release_when_any_lane_blocked,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseClaim {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave95_promotion_root: String,
    pub promoted_slot_root: Option<String>,
    pub live_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub lane_quorum_root: Option<String>,
    pub release_claim_root: Option<String>,
    pub quorum_height: Option<u64>,
    pub command_hint_root: String,
    pub blockers: Vec<QuorumBlockerKind>,
    pub status: QuorumStatus,
}

impl ReleaseClaim {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave95_promotion_root: label_root(
                "wave95_promotion",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            promoted_slot_root: None,
            live_evidence_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            lane_quorum_root: None,
            release_claim_root: None,
            quorum_height: None,
            command_hint_root: label_root(
                "release_readiness_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: QuorumStatus::Blocked,
        }
    }

    pub fn stage_release_claim(
        &self,
        promoted_slot_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        lane_quorum_root: &str,
        quorum_height: u64,
        config: &Config,
        duplicate: bool,
    ) -> Result<Self> {
        if promoted_slot_root.is_empty() {
            return Err(QuorumError::PromotedSlotRootMissing);
        }
        if live_evidence_root.is_empty() {
            return Err(QuorumError::EvidenceRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(QuorumError::OperatorSignoffRootMissing);
        }
        if reviewer_signoff_root.is_empty() {
            return Err(QuorumError::ReviewerSignoffRootMissing);
        }
        if quorum_height < config.min_quorum_height {
            return Err(QuorumError::QuorumHeightTooLow);
        }
        if duplicate {
            return Err(QuorumError::DuplicateReleaseClaimRoot);
        }
        let release_claim_root = release_claim_root(
            self.lane,
            &self.slot_label,
            self.ordinal,
            promoted_slot_root,
            live_evidence_root,
            operator_signoff_root,
            reviewer_signoff_root,
            lane_quorum_root,
            quorum_height,
        );
        let mut next = self.clone();
        next.promoted_slot_root = Some(promoted_slot_root.to_string());
        next.live_evidence_root = Some(live_evidence_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.lane_quorum_root = Some(lane_quorum_root.to_string());
        next.release_claim_root = Some(release_claim_root);
        next.quorum_height = Some(quorum_height);
        next.blockers = next.active_blockers(config, duplicate);
        next.status = if next.blockers.is_empty() {
            QuorumStatus::QuorumReady
        } else {
            QuorumStatus::Blocked
        };
        Ok(next)
    }

    pub fn mark_release_ready(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(QuorumError::QuorumStillBlocked);
        }
        let mut next = self.clone();
        next.status = QuorumStatus::ReleaseReady;
        Ok(next)
    }

    fn active_blockers(&self, config: &Config, duplicate: bool) -> Vec<QuorumBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_promoted_slot_root && self.promoted_slot_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingPromotedSlotRoot);
        }
        if config.require_evidence_root && self.live_evidence_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingReviewerSignoffRoot);
        }
        if config.require_lane_quorum_root && self.lane_quorum_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingLaneQuorumRoot);
        }
        if self.release_claim_root.is_none() {
            blockers.push(QuorumBlockerKind::MissingReleaseClaimRoot);
        }
        match self.quorum_height {
            Some(height) if height >= config.min_quorum_height => {}
            _ => blockers.push(QuorumBlockerKind::QuorumHeightTooLow),
        }
        if duplicate {
            blockers.push(QuorumBlockerKind::DuplicateReleaseClaimRoot);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave95_promotion_root": self.wave95_promotion_root,
            "promoted_slot_root": self.promoted_slot_root,
            "live_evidence_root": self.live_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "lane_quorum_root": self.lane_quorum_root,
            "release_claim_root": self.release_claim_root,
            "quorum_height": self.quorum_height,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneReleaseQuorum {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave95_promotion_gate_root: String,
    pub release_claims: Vec<ReleaseClaim>,
    pub blocked_root: String,
    pub quorum_ready_root: String,
    pub release_ready_root: String,
    pub command_root: String,
    pub lane_status: QuorumStatus,
}

impl LaneReleaseQuorum {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let release_claims = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| ReleaseClaim::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        Self::from_claims(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave95_promotion_gate", lane.as_str(), "source", WAVE),
            release_claims,
        )
    }

    pub fn stage_release_claim(
        &self,
        slot_label: &str,
        promoted_slot_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        lane_quorum_root: &str,
        quorum_height: u64,
        config: &Config,
        known_claim_roots: &[String],
    ) -> Result<Self> {
        let mut found = false;
        let duplicate = known_claim_roots
            .iter()
            .any(|root| root.as_str() == promoted_slot_root);
        let mut claims = Vec::with_capacity(self.release_claims.len());
        for claim in &self.release_claims {
            if claim.slot_label == slot_label {
                claims.push(claim.stage_release_claim(
                    promoted_slot_root,
                    live_evidence_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    lane_quorum_root,
                    quorum_height,
                    config,
                    duplicate,
                )?);
                found = true;
            } else {
                claims.push(claim.clone());
            }
        }
        if !found {
            return Err(QuorumError::ClaimMissing);
        }
        Ok(Self::from_claims(
            self.lane,
            self.lane_title_root.clone(),
            self.wave95_promotion_gate_root.clone(),
            claims,
        ))
    }

    fn from_claims(
        lane: LaneKind,
        lane_title_root: String,
        wave95_promotion_gate_root: String,
        release_claims: Vec<ReleaseClaim>,
    ) -> Self {
        let blocked_root = blocked_root(&release_claims);
        let quorum_ready_root = root_from_strings(
            "wave96_quorum_ready_claims",
            release_claims.iter().filter_map(|claim| {
                if claim.status == QuorumStatus::QuorumReady {
                    claim.release_claim_root.clone()
                } else {
                    None
                }
            }),
        );
        let release_ready_root = root_from_strings(
            "wave96_release_ready_claims",
            release_claims.iter().filter_map(|claim| {
                if claim.status == QuorumStatus::ReleaseReady {
                    claim.release_claim_root.clone()
                } else {
                    None
                }
            }),
        );
        let command_root = root_from_strings(
            "wave96_release_commands",
            release_claims
                .iter()
                .map(|claim| claim.command_hint_root.clone()),
        );
        let lane_status = if release_claims
            .iter()
            .all(|claim| claim.status == QuorumStatus::ReleaseReady)
        {
            QuorumStatus::ReleaseReady
        } else if release_claims
            .iter()
            .any(|claim| claim.status == QuorumStatus::QuorumReady)
        {
            QuorumStatus::QuorumReady
        } else {
            QuorumStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave95_promotion_gate_root,
            release_claims,
            blocked_root,
            quorum_ready_root,
            release_ready_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.release_claims
            .iter()
            .filter(|claim| !claim.blockers.is_empty())
            .count()
    }

    pub fn quorum_ready_count(&self) -> usize {
        self.release_claims
            .iter()
            .filter(|claim| claim.status == QuorumStatus::QuorumReady)
            .count()
    }

    pub fn release_ready_count(&self) -> usize {
        self.release_claims
            .iter()
            .filter(|claim| claim.status == QuorumStatus::ReleaseReady)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave95_promotion_gate_root": self.wave95_promotion_gate_root,
            "release_claim_roots": self.release_claims.iter().map(ReleaseClaim::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "quorum_ready_root": self.quorum_ready_root,
            "release_ready_root": self.release_ready_root,
            "command_root": self.command_root,
            "claim_count": self.release_claims.len(),
            "blocked_count": self.blocked_count(),
            "quorum_ready_count": self.quorum_ready_count(),
            "release_ready_count": self.release_ready_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_release_quorum", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseSummary {
    pub quorum_root: String,
    pub blocked_root: String,
    pub quorum_ready_root: String,
    pub release_ready_root: String,
    pub command_root: String,
    pub readiness_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub quorum_ready_count: usize,
    pub release_ready_count: usize,
    pub all_lanes_release_ready: bool,
    pub production_release_denied: bool,
}

impl ReleaseSummary {
    pub fn from_quorums(config: &Config, quorums: &[LaneReleaseQuorum]) -> Self {
        let quorum_root = root_from_strings(
            "wave96_release_quorum_root",
            quorums.iter().map(LaneReleaseQuorum::state_root),
        );
        let blocked_root = root_from_strings(
            "wave96_blocked_root",
            quorums.iter().map(|quorum| quorum.blocked_root.clone()),
        );
        let quorum_ready_root = root_from_strings(
            "wave96_quorum_ready_root",
            quorums
                .iter()
                .map(|quorum| quorum.quorum_ready_root.clone()),
        );
        let release_ready_root = root_from_strings(
            "wave96_release_ready_root",
            quorums
                .iter()
                .map(|quorum| quorum.release_ready_root.clone()),
        );
        let command_root = root_from_strings(
            "wave96_command_root",
            quorums.iter().map(|quorum| quorum.command_root.clone()),
        );
        let claim_count = quorums
            .iter()
            .map(|quorum| quorum.release_claims.len())
            .sum::<usize>();
        let blocked_count = quorums
            .iter()
            .map(LaneReleaseQuorum::blocked_count)
            .sum::<usize>();
        let quorum_ready_count = quorums
            .iter()
            .map(LaneReleaseQuorum::quorum_ready_count)
            .sum::<usize>();
        let release_ready_count = quorums
            .iter()
            .map(LaneReleaseQuorum::release_ready_count)
            .sum::<usize>();
        let all_lanes_release_ready = quorums.len() as u64 >= config.global_quorum_threshold
            && quorums
                .iter()
                .all(|quorum| quorum.lane_status == QuorumStatus::ReleaseReady);
        let production_release_denied =
            config.deny_release_when_any_lane_blocked && !all_lanes_release_ready;
        let readiness_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "claim_count": claim_count,
            "blocked_count": blocked_count,
            "quorum_ready_count": quorum_ready_count,
            "release_ready_count": release_ready_count,
            "all_lanes_release_ready": all_lanes_release_ready,
            "production_release_denied": production_release_denied,
        });
        let readiness_root = record_root("readiness", &readiness_record);
        Self {
            quorum_root,
            blocked_root,
            quorum_ready_root,
            release_ready_root,
            command_root,
            readiness_root,
            lane_count: quorums.len(),
            claim_count,
            blocked_count,
            quorum_ready_count,
            release_ready_count,
            all_lanes_release_ready,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "quorum_root": self.quorum_root,
            "blocked_root": self.blocked_root,
            "quorum_ready_root": self.quorum_ready_root,
            "release_ready_root": self.release_ready_root,
            "command_root": self.command_root,
            "readiness_root": self.readiness_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "quorum_ready_count": self.quorum_ready_count,
            "release_ready_count": self.release_ready_count,
            "all_lanes_release_ready": self.all_lanes_release_ready,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave95_promotion_transcript_root: String,
    pub quorums: Vec<LaneReleaseQuorum>,
    pub summary: ReleaseSummary,
}

impl State {
    pub fn new(config: Config, quorums: Vec<LaneReleaseQuorum>) -> Self {
        let wave95_promotion_transcript_root =
            label_root("wave95_promotion_transcript", "all_lanes", "source", WAVE);
        let summary = ReleaseSummary::from_quorums(&config, &quorums);
        Self {
            config,
            wave95_promotion_transcript_root,
            quorums,
            summary,
        }
    }

    pub fn stage_release_claim(
        &self,
        lane: LaneKind,
        slot_label: &str,
        promoted_slot_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        lane_quorum_root: &str,
        quorum_height: u64,
    ) -> Result<Self> {
        let known_roots = self.known_claim_roots();
        let mut found = false;
        let mut quorums = Vec::with_capacity(self.quorums.len());
        for quorum in &self.quorums {
            if quorum.lane == lane {
                quorums.push(quorum.stage_release_claim(
                    slot_label,
                    promoted_slot_root,
                    live_evidence_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    lane_quorum_root,
                    quorum_height,
                    &self.config,
                    &known_roots,
                )?);
                found = true;
            } else {
                quorums.push(quorum.clone());
            }
        }
        if !found {
            return Err(QuorumError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), quorums))
    }

    pub fn known_claim_roots(&self) -> Vec<String> {
        self.quorums
            .iter()
            .flat_map(|quorum| {
                quorum
                    .release_claims
                    .iter()
                    .filter_map(|claim| claim.release_claim_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave95_promotion_transcript_root": self.wave95_promotion_transcript_root,
            "quorum_roots": self.quorums.iter().map(LaneReleaseQuorum::state_root).collect::<Vec<_>>(),
            "summary": self.summary.public_record(),
            "roots_only": self.config.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::default();
    let quorums = vec![
        LaneReleaseQuorum::new(
            LaneKind::Compile,
            &[
                "cargo_check",
                "cargo_test",
                "clippy",
                "rustfmt",
                "rustc",
                "build_metadata",
                "operator_signoff",
            ],
            &config,
        ),
        LaneReleaseQuorum::new(
            LaneKind::RuntimeReplay,
            &[
                "replay_run",
                "rollback_drill",
                "adversarial_replay",
                "stale_archive_replacement",
                "live_execution_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneReleaseQuorum::new(
            LaneKind::AuditSecurity,
            &[
                "audit_review",
                "adversarial_scenario",
                "threat_model",
                "privacy_review",
                "reviewer_signoff",
                "operator_signoff",
            ],
            &config,
        ),
        LaneReleaseQuorum::new(
            LaneKind::BridgeCustody,
            &[
                "monero_watcher_quorum",
                "withdrawal_release",
                "reserve_coverage",
                "signer_quorum",
                "challenge_hold_review",
                "custody_operator_signoff",
            ],
            &config,
        ),
        LaneReleaseQuorum::new(
            LaneKind::WalletWatchtower,
            &[
                "wallet_escape_dry_run",
                "watchtower_quorum",
                "user_runbook_replay",
                "redacted_recovery_proof",
                "wallet_visible_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneReleaseQuorum::new(
            LaneKind::PqReservePrivacy,
            &[
                "ml_dsa_slh_dsa_authority_epoch",
                "pq_quorum",
                "reserve_coverage",
                "privacy_linkage",
                "metadata_redaction",
                "nullifier_separation",
                "operator_signoff",
            ],
            &config,
        ),
    ];
    State::new(config, quorums)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<QuorumBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_promoted_slot_root {
        blockers.push(QuorumBlockerKind::MissingPromotedSlotRoot);
    }
    if config.require_evidence_root {
        blockers.push(QuorumBlockerKind::MissingEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(QuorumBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(QuorumBlockerKind::MissingReviewerSignoffRoot);
    }
    if config.require_lane_quorum_root {
        blockers.push(QuorumBlockerKind::MissingLaneQuorumRoot);
    }
    blockers.push(QuorumBlockerKind::MissingReleaseClaimRoot);
    blockers.push(QuorumBlockerKind::QuorumHeightTooLow);
    blockers
}

fn blocked_root(claims: &[ReleaseClaim]) -> String {
    let leaves = claims
        .iter()
        .flat_map(|claim| {
            claim.blockers.iter().map(move |blocker| {
                json!({
                    "lane": claim.lane.as_str(),
                    "slot_label": claim.slot_label,
                    "blocker": blocker.as_str(),
                    "claim_root": claim.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave96_blocked_release_claims", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn release_claim_root(
    lane: LaneKind,
    slot_label: &str,
    ordinal: u64,
    promoted_slot_root: &str,
    live_evidence_root: &str,
    operator_signoff_root: &str,
    reviewer_signoff_root: &str,
    lane_quorum_root: &str,
    quorum_height: u64,
) -> String {
    domain_hash(
        "wave96-live-heavy-gate-release-readiness-quorum-claim",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::U64(ordinal),
            HashPart::Str(promoted_slot_root),
            HashPart::Str(live_evidence_root),
            HashPart::Str(operator_signoff_root),
            HashPart::Str(reviewer_signoff_root),
            HashPart::Str(lane_quorum_root),
            HashPart::U64(quorum_height),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave96-live-heavy-gate-release-readiness-quorum-record",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn label_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "wave96-live-heavy-gate-release-readiness-quorum-label",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}
