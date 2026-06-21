use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave97-live-heavy-gate-release-claim-anti-equivocation-seal-v1";
const WAVE: u64 = 97;
const QUORUM_WAVE: u64 = 96;
const MIN_SEAL_HEIGHT: u64 = 970_000;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, SealError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SealError {
    LaneMissing,
    ClaimMissing,
    ReleaseClaimRootMissing,
    LaneQuorumRootMissing,
    TranscriptRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    SealHeightTooLow,
    DuplicateReleaseClaimRoot,
    ForkedReleaseClaimRoot,
    ReplayedTranscriptRoot,
    SealStillBlocked,
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
            Self::Compile => "Compile release-claim anti-equivocation seal",
            Self::RuntimeReplay => "Runtime replay release-claim anti-equivocation seal",
            Self::AuditSecurity => "Audit security release-claim anti-equivocation seal",
            Self::BridgeCustody => "Bridge custody release-claim anti-equivocation seal",
            Self::WalletWatchtower => "Wallet watchtower release-claim anti-equivocation seal",
            Self::PqReservePrivacy => "PQ reserve privacy release-claim anti-equivocation seal",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SealStatus {
    Empty,
    Blocked,
    Candidate,
    Sealed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EquivocationBlockerKind {
    MissingReleaseClaimRoot,
    MissingLaneQuorumRoot,
    MissingTranscriptRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    SealHeightTooLow,
    DuplicateReleaseClaimRoot,
    ForkedReleaseClaimRoot,
    ReplayedTranscriptRoot,
    LiveHeavyGateEvidenceMissing,
}

impl EquivocationBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReleaseClaimRoot => "missing_release_claim_root",
            Self::MissingLaneQuorumRoot => "missing_lane_quorum_root",
            Self::MissingTranscriptRoot => "missing_transcript_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::SealHeightTooLow => "seal_height_too_low",
            Self::DuplicateReleaseClaimRoot => "duplicate_release_claim_root",
            Self::ForkedReleaseClaimRoot => "forked_release_claim_root",
            Self::ReplayedTranscriptRoot => "replayed_transcript_root",
            Self::LiveHeavyGateEvidenceMissing => "live_heavy_gate_evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub quorum_wave: u64,
    pub min_seal_height: u64,
    pub lane_seal_threshold: u64,
    pub global_seal_threshold: u64,
    pub require_release_claim_root: bool,
    pub require_lane_quorum_root: bool,
    pub require_transcript_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub deny_release_when_any_lane_blocked: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            quorum_wave: QUORUM_WAVE,
            min_seal_height: MIN_SEAL_HEIGHT,
            lane_seal_threshold: 1,
            global_seal_threshold: 6,
            require_release_claim_root: true,
            require_lane_quorum_root: true,
            require_transcript_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_live_heavy_gate_evidence: true,
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
            "quorum_wave": self.quorum_wave,
            "min_seal_height": self.min_seal_height,
            "lane_seal_threshold": self.lane_seal_threshold,
            "global_seal_threshold": self.global_seal_threshold,
            "require_release_claim_root": self.require_release_claim_root,
            "require_lane_quorum_root": self.require_lane_quorum_root,
            "require_transcript_root": self.require_transcript_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "deny_release_when_any_lane_blocked": self.deny_release_when_any_lane_blocked,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseClaimSeal {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave96_quorum_anchor_root: String,
    pub release_claim_root: Option<String>,
    pub lane_quorum_root: Option<String>,
    pub transcript_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub seal_height: Option<u64>,
    pub duplicate_guard_root: String,
    pub fork_guard_root: String,
    pub replay_guard_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<EquivocationBlockerKind>,
    pub status: SealStatus,
}

impl ReleaseClaimSeal {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave96_quorum_anchor_root: label_root(
                "wave96_quorum_anchor",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            release_claim_root: None,
            lane_quorum_root: None,
            transcript_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            seal_height: None,
            duplicate_guard_root: label_root("duplicate_guard", lane.as_str(), slot_label, ordinal),
            fork_guard_root: label_root("fork_guard", lane.as_str(), slot_label, ordinal),
            replay_guard_root: label_root("replay_guard", lane.as_str(), slot_label, ordinal),
            command_hint_root: label_root(
                "release_claim_anti_equivocation_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: SealStatus::Blocked,
        }
    }

    pub fn stage_seal(
        &self,
        release_claim_root: &str,
        lane_quorum_root: &str,
        transcript_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        seal_height: u64,
        config: &Config,
        duplicate: bool,
        replay: bool,
    ) -> Result<Self> {
        if release_claim_root.is_empty() {
            return Err(SealError::ReleaseClaimRootMissing);
        }
        if lane_quorum_root.is_empty() {
            return Err(SealError::LaneQuorumRootMissing);
        }
        if transcript_root.is_empty() {
            return Err(SealError::TranscriptRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(SealError::OperatorSignoffRootMissing);
        }
        if reviewer_signoff_root.is_empty() {
            return Err(SealError::ReviewerSignoffRootMissing);
        }
        if seal_height < config.min_seal_height {
            return Err(SealError::SealHeightTooLow);
        }
        if let Some(existing) = &self.release_claim_root {
            if existing != release_claim_root {
                return Err(SealError::ForkedReleaseClaimRoot);
            }
            return Err(SealError::DuplicateReleaseClaimRoot);
        }
        if duplicate {
            return Err(SealError::DuplicateReleaseClaimRoot);
        }
        if replay {
            return Err(SealError::ReplayedTranscriptRoot);
        }

        let mut next = self.clone();
        next.release_claim_root = Some(release_claim_root.to_string());
        next.lane_quorum_root = Some(lane_quorum_root.to_string());
        next.transcript_root = Some(transcript_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.seal_height = Some(seal_height);
        next.duplicate_guard_root = guard_root(
            "duplicate",
            self.lane,
            &self.slot_label,
            release_claim_root,
            lane_quorum_root,
            transcript_root,
            seal_height,
        );
        next.fork_guard_root = guard_root(
            "fork",
            self.lane,
            &self.slot_label,
            release_claim_root,
            lane_quorum_root,
            transcript_root,
            seal_height,
        );
        next.replay_guard_root = guard_root(
            "replay",
            self.lane,
            &self.slot_label,
            release_claim_root,
            lane_quorum_root,
            transcript_root,
            seal_height,
        );
        next.blockers = next.active_blockers(config, duplicate, false, replay);
        next.status = if next.blockers.is_empty() {
            SealStatus::Candidate
        } else {
            SealStatus::Blocked
        };
        Ok(next)
    }

    pub fn mark_sealed(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(SealError::SealStillBlocked);
        }
        let mut next = self.clone();
        next.status = SealStatus::Sealed;
        Ok(next)
    }

    fn active_blockers(
        &self,
        config: &Config,
        duplicate: bool,
        fork: bool,
        replay: bool,
    ) -> Vec<EquivocationBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_release_claim_root && self.release_claim_root.is_none() {
            blockers.push(EquivocationBlockerKind::MissingReleaseClaimRoot);
        }
        if config.require_lane_quorum_root && self.lane_quorum_root.is_none() {
            blockers.push(EquivocationBlockerKind::MissingLaneQuorumRoot);
        }
        if config.require_transcript_root && self.transcript_root.is_none() {
            blockers.push(EquivocationBlockerKind::MissingTranscriptRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(EquivocationBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(EquivocationBlockerKind::MissingReviewerSignoffRoot);
        }
        match self.seal_height {
            Some(height) if height >= config.min_seal_height => {}
            _ => blockers.push(EquivocationBlockerKind::SealHeightTooLow),
        }
        if duplicate {
            blockers.push(EquivocationBlockerKind::DuplicateReleaseClaimRoot);
        }
        if fork {
            blockers.push(EquivocationBlockerKind::ForkedReleaseClaimRoot);
        }
        if replay {
            blockers.push(EquivocationBlockerKind::ReplayedTranscriptRoot);
        }
        if config.require_live_heavy_gate_evidence {
            blockers.push(EquivocationBlockerKind::LiveHeavyGateEvidenceMissing);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave96_quorum_anchor_root": self.wave96_quorum_anchor_root,
            "release_claim_root": self.release_claim_root,
            "lane_quorum_root": self.lane_quorum_root,
            "transcript_root": self.transcript_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "seal_height": self.seal_height,
            "duplicate_guard_root": self.duplicate_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "replay_guard_root": self.replay_guard_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_seal", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneAntiEquivocationSeal {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave96_release_quorum_root: String,
    pub claim_seals: Vec<ReleaseClaimSeal>,
    pub blocked_root: String,
    pub candidate_root: String,
    pub sealed_root: String,
    pub duplicate_guard_root: String,
    pub fork_guard_root: String,
    pub replay_guard_root: String,
    pub command_root: String,
    pub lane_status: SealStatus,
}

impl LaneAntiEquivocationSeal {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let claim_seals = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| ReleaseClaimSeal::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        Self::from_claims(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave96_release_quorum", lane.as_str(), "source", WAVE),
            claim_seals,
        )
    }

    pub fn stage_seal(
        &self,
        slot_label: &str,
        release_claim_root: &str,
        lane_quorum_root: &str,
        transcript_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        seal_height: u64,
        config: &Config,
        known_claim_roots: &[String],
        known_transcript_roots: &[String],
    ) -> Result<Self> {
        let duplicate = known_claim_roots
            .iter()
            .any(|root| root.as_str() == release_claim_root);
        let replay = known_transcript_roots
            .iter()
            .any(|root| root.as_str() == transcript_root);
        let mut found = false;
        let mut claims = Vec::with_capacity(self.claim_seals.len());
        for claim in &self.claim_seals {
            if claim.slot_label == slot_label {
                claims.push(claim.stage_seal(
                    release_claim_root,
                    lane_quorum_root,
                    transcript_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    seal_height,
                    config,
                    duplicate,
                    replay,
                )?);
                found = true;
            } else {
                claims.push(claim.clone());
            }
        }
        if !found {
            return Err(SealError::ClaimMissing);
        }
        Ok(Self::from_claims(
            self.lane,
            self.lane_title_root.clone(),
            self.wave96_release_quorum_root.clone(),
            claims,
        ))
    }

    fn from_claims(
        lane: LaneKind,
        lane_title_root: String,
        wave96_release_quorum_root: String,
        claim_seals: Vec<ReleaseClaimSeal>,
    ) -> Self {
        let blocked_root = blocked_root(&claim_seals);
        let candidate_root = root_from_strings(
            "wave97_candidate_claim_seals",
            claim_seals.iter().filter_map(|claim| {
                if claim.status == SealStatus::Candidate {
                    Some(claim.state_root())
                } else {
                    None
                }
            }),
        );
        let sealed_root = root_from_strings(
            "wave97_sealed_claim_seals",
            claim_seals.iter().filter_map(|claim| {
                if claim.status == SealStatus::Sealed {
                    Some(claim.state_root())
                } else {
                    None
                }
            }),
        );
        let duplicate_guard_root = root_from_strings(
            "wave97_duplicate_guards",
            claim_seals
                .iter()
                .map(|claim| claim.duplicate_guard_root.clone()),
        );
        let fork_guard_root = root_from_strings(
            "wave97_fork_guards",
            claim_seals
                .iter()
                .map(|claim| claim.fork_guard_root.clone()),
        );
        let replay_guard_root = root_from_strings(
            "wave97_replay_guards",
            claim_seals
                .iter()
                .map(|claim| claim.replay_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave97_release_claim_anti_equivocation_commands",
            claim_seals
                .iter()
                .map(|claim| claim.command_hint_root.clone()),
        );
        let lane_status = if claim_seals
            .iter()
            .all(|claim| claim.status == SealStatus::Sealed)
        {
            SealStatus::Sealed
        } else if claim_seals
            .iter()
            .any(|claim| claim.status == SealStatus::Candidate)
        {
            SealStatus::Candidate
        } else {
            SealStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave96_release_quorum_root,
            claim_seals,
            blocked_root,
            candidate_root,
            sealed_root,
            duplicate_guard_root,
            fork_guard_root,
            replay_guard_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.claim_seals
            .iter()
            .filter(|claim| !claim.blockers.is_empty())
            .count()
    }

    pub fn candidate_count(&self) -> usize {
        self.claim_seals
            .iter()
            .filter(|claim| claim.status == SealStatus::Candidate)
            .count()
    }

    pub fn sealed_count(&self) -> usize {
        self.claim_seals
            .iter()
            .filter(|claim| claim.status == SealStatus::Sealed)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave96_release_quorum_root": self.wave96_release_quorum_root,
            "claim_seal_roots": self.claim_seals.iter().map(ReleaseClaimSeal::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "sealed_root": self.sealed_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "replay_guard_root": self.replay_guard_root,
            "command_root": self.command_root,
            "claim_count": self.claim_seals.len(),
            "blocked_count": self.blocked_count(),
            "candidate_count": self.candidate_count(),
            "sealed_count": self.sealed_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_anti_equivocation_seal", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealSummary {
    pub anti_equivocation_root: String,
    pub blocked_root: String,
    pub candidate_root: String,
    pub sealed_root: String,
    pub duplicate_guard_root: String,
    pub fork_guard_root: String,
    pub replay_guard_root: String,
    pub command_root: String,
    pub release_denial_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub candidate_count: usize,
    pub sealed_count: usize,
    pub all_lanes_sealed: bool,
    pub live_heavy_gates_ran: bool,
    pub production_release_denied: bool,
}

impl SealSummary {
    pub fn from_lanes(config: &Config, lanes: &[LaneAntiEquivocationSeal]) -> Self {
        let anti_equivocation_root = root_from_strings(
            "wave97_anti_equivocation_root",
            lanes.iter().map(LaneAntiEquivocationSeal::state_root),
        );
        let blocked_root = root_from_strings(
            "wave97_blocked_root",
            lanes.iter().map(|lane| lane.blocked_root.clone()),
        );
        let candidate_root = root_from_strings(
            "wave97_candidate_root",
            lanes.iter().map(|lane| lane.candidate_root.clone()),
        );
        let sealed_root = root_from_strings(
            "wave97_sealed_root",
            lanes.iter().map(|lane| lane.sealed_root.clone()),
        );
        let duplicate_guard_root = root_from_strings(
            "wave97_duplicate_guard_root",
            lanes.iter().map(|lane| lane.duplicate_guard_root.clone()),
        );
        let fork_guard_root = root_from_strings(
            "wave97_fork_guard_root",
            lanes.iter().map(|lane| lane.fork_guard_root.clone()),
        );
        let replay_guard_root = root_from_strings(
            "wave97_replay_guard_root",
            lanes.iter().map(|lane| lane.replay_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave97_command_root",
            lanes.iter().map(|lane| lane.command_root.clone()),
        );
        let claim_count = lanes
            .iter()
            .map(|lane| lane.claim_seals.len())
            .sum::<usize>();
        let blocked_count = lanes
            .iter()
            .map(LaneAntiEquivocationSeal::blocked_count)
            .sum::<usize>();
        let candidate_count = lanes
            .iter()
            .map(LaneAntiEquivocationSeal::candidate_count)
            .sum::<usize>();
        let sealed_count = lanes
            .iter()
            .map(LaneAntiEquivocationSeal::sealed_count)
            .sum::<usize>();
        let all_lanes_sealed = lanes.len() as u64 >= config.global_seal_threshold
            && lanes
                .iter()
                .all(|lane| lane.lane_status == SealStatus::Sealed);
        let live_heavy_gates_ran = false;
        let production_release_denied = config.deny_release_when_any_lane_blocked
            && (!all_lanes_sealed || !live_heavy_gates_ran);
        let denial_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "quorum_wave": config.quorum_wave,
            "blocked_count": blocked_count,
            "candidate_count": candidate_count,
            "sealed_count": sealed_count,
            "all_lanes_sealed": all_lanes_sealed,
            "live_heavy_gates_ran": live_heavy_gates_ran,
            "production_release_denied": production_release_denied,
        });
        let release_denial_root = record_root("release_denial", &denial_record);
        Self {
            anti_equivocation_root,
            blocked_root,
            candidate_root,
            sealed_root,
            duplicate_guard_root,
            fork_guard_root,
            replay_guard_root,
            command_root,
            release_denial_root,
            lane_count: lanes.len(),
            claim_count,
            blocked_count,
            candidate_count,
            sealed_count,
            all_lanes_sealed,
            live_heavy_gates_ran,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "anti_equivocation_root": self.anti_equivocation_root,
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "sealed_root": self.sealed_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "replay_guard_root": self.replay_guard_root,
            "command_root": self.command_root,
            "release_denial_root": self.release_denial_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "candidate_count": self.candidate_count,
            "sealed_count": self.sealed_count,
            "all_lanes_sealed": self.all_lanes_sealed,
            "live_heavy_gates_ran": self.live_heavy_gates_ran,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("seal_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave96_release_readiness_quorum_root: String,
    pub lanes: Vec<LaneAntiEquivocationSeal>,
    pub summary: SealSummary,
}

impl State {
    pub fn new(config: Config, lanes: Vec<LaneAntiEquivocationSeal>) -> Self {
        let wave96_release_readiness_quorum_root = label_root(
            "wave96_release_readiness_quorum_transcript",
            "all_lanes",
            "source",
            WAVE,
        );
        let summary = SealSummary::from_lanes(&config, &lanes);
        Self {
            config,
            wave96_release_readiness_quorum_root,
            lanes,
            summary,
        }
    }

    pub fn stage_release_claim_seal(
        &self,
        lane: LaneKind,
        slot_label: &str,
        release_claim_root: &str,
        lane_quorum_root: &str,
        transcript_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        seal_height: u64,
    ) -> Result<Self> {
        let known_claim_roots = self.known_claim_roots();
        let known_transcript_roots = self.known_transcript_roots();
        let mut found = false;
        let mut lanes = Vec::with_capacity(self.lanes.len());
        for lane_seal in &self.lanes {
            if lane_seal.lane == lane {
                lanes.push(lane_seal.stage_seal(
                    slot_label,
                    release_claim_root,
                    lane_quorum_root,
                    transcript_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    seal_height,
                    &self.config,
                    &known_claim_roots,
                    &known_transcript_roots,
                )?);
                found = true;
            } else {
                lanes.push(lane_seal.clone());
            }
        }
        if !found {
            return Err(SealError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), lanes))
    }

    pub fn known_claim_roots(&self) -> Vec<String> {
        self.lanes
            .iter()
            .flat_map(|lane| {
                lane.claim_seals
                    .iter()
                    .filter_map(|claim| claim.release_claim_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn known_transcript_roots(&self) -> Vec<String> {
        self.lanes
            .iter()
            .flat_map(|lane| {
                lane.claim_seals
                    .iter()
                    .filter_map(|claim| claim.transcript_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave96_release_readiness_quorum_root": self.wave96_release_readiness_quorum_root,
            "lane_roots": self.lanes.iter().map(LaneAntiEquivocationSeal::state_root).collect::<Vec<_>>(),
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
    let lanes = vec![
        LaneAntiEquivocationSeal::new(
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
        LaneAntiEquivocationSeal::new(
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
        LaneAntiEquivocationSeal::new(
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
        LaneAntiEquivocationSeal::new(
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
        LaneAntiEquivocationSeal::new(
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
        LaneAntiEquivocationSeal::new(
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
    State::new(config, lanes)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<EquivocationBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_release_claim_root {
        blockers.push(EquivocationBlockerKind::MissingReleaseClaimRoot);
    }
    if config.require_lane_quorum_root {
        blockers.push(EquivocationBlockerKind::MissingLaneQuorumRoot);
    }
    if config.require_transcript_root {
        blockers.push(EquivocationBlockerKind::MissingTranscriptRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(EquivocationBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(EquivocationBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(EquivocationBlockerKind::SealHeightTooLow);
    if config.require_live_heavy_gate_evidence {
        blockers.push(EquivocationBlockerKind::LiveHeavyGateEvidenceMissing);
    }
    blockers
}

fn blocked_root(claims: &[ReleaseClaimSeal]) -> String {
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
    merkle_root("wave97_blocked_release_claim_seals", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn guard_root(
    guard_kind: &str,
    lane: LaneKind,
    slot_label: &str,
    release_claim_root: &str,
    lane_quorum_root: &str,
    transcript_root: &str,
    seal_height: u64,
) -> String {
    domain_hash(
        "wave97-live-heavy-gate-release-claim-anti-equivocation-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::Str(release_claim_root),
            HashPart::Str(lane_quorum_root),
            HashPart::Str(transcript_root),
            HashPart::U64(seal_height),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave97-live-heavy-gate-release-claim-anti-equivocation-record",
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
        "wave97-live-heavy-gate-release-claim-anti-equivocation-label",
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
