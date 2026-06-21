use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave95-live-heavy-gate-receipt-slot-promotion-v1";
const WAVE: u64 = 95;
const SLOT_WAVE: u64 = 92;
const ADMISSION_WAVE: u64 = 93;
const FILL_WAVE: u64 = 94;
const MIN_PROMOTION_HEIGHT: u64 = 950_000;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, PromotionError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PromotionError {
    LaneMissing,
    SlotMissing,
    StagedFillRootMissing,
    AdmissionRootMissing,
    EvidenceRootMissing,
    SignoffRootMissing,
    PromotionHeightTooLow,
    DuplicateOccupancyRoot,
    PromotionStillBlocked,
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
            Self::Compile => "Compile receipt slot promotion",
            Self::RuntimeReplay => "Runtime replay receipt slot promotion",
            Self::AuditSecurity => "Audit security receipt slot promotion",
            Self::BridgeCustody => "Bridge custody receipt slot promotion",
            Self::WalletWatchtower => "Wallet watchtower receipt slot promotion",
            Self::PqReservePrivacy => "PQ reserve privacy receipt slot promotion",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PromotionStatus {
    Empty,
    Blocked,
    PromotionReady,
    Promoted,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PromotionBlockerKind {
    MissingStagedFillRoot,
    MissingAdmissionRoot,
    MissingEvidenceRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    MissingSlotReleaseRoot,
    PromotionHeightTooLow,
    DuplicateOccupancyRoot,
}

impl PromotionBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingStagedFillRoot => "missing_staged_fill_root",
            Self::MissingAdmissionRoot => "missing_admission_root",
            Self::MissingEvidenceRoot => "missing_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::MissingSlotReleaseRoot => "missing_slot_release_root",
            Self::PromotionHeightTooLow => "promotion_height_too_low",
            Self::DuplicateOccupancyRoot => "duplicate_occupancy_root",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub slot_wave: u64,
    pub admission_wave: u64,
    pub fill_wave: u64,
    pub min_promotion_height: u64,
    pub require_staged_fill_root: bool,
    pub require_admission_root: bool,
    pub require_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_slot_release_root: bool,
    pub deny_release_when_any_slot_unpromoted: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            slot_wave: SLOT_WAVE,
            admission_wave: ADMISSION_WAVE,
            fill_wave: FILL_WAVE,
            min_promotion_height: MIN_PROMOTION_HEIGHT,
            require_staged_fill_root: true,
            require_admission_root: true,
            require_evidence_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_slot_release_root: true,
            deny_release_when_any_slot_unpromoted: true,
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
            "slot_wave": self.slot_wave,
            "admission_wave": self.admission_wave,
            "fill_wave": self.fill_wave,
            "min_promotion_height": self.min_promotion_height,
            "require_staged_fill_root": self.require_staged_fill_root,
            "require_admission_root": self.require_admission_root,
            "require_evidence_root": self.require_evidence_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_slot_release_root": self.require_slot_release_root,
            "deny_release_when_any_slot_unpromoted": self.deny_release_when_any_slot_unpromoted,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromotionAttempt {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave92_slot_root: String,
    pub wave93_admission_root: String,
    pub wave94_staged_fill_root: Option<String>,
    pub live_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub slot_release_root: Option<String>,
    pub slot_occupancy_root: Option<String>,
    pub promotion_height: Option<u64>,
    pub command_hint_root: String,
    pub blockers: Vec<PromotionBlockerKind>,
    pub status: PromotionStatus,
}

impl PromotionAttempt {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave92_slot_root: label_root("wave92_slot_root", lane.as_str(), slot_label, ordinal),
            wave93_admission_root: label_root(
                "wave93_admission_root",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            wave94_staged_fill_root: None,
            live_evidence_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            slot_release_root: None,
            slot_occupancy_root: None,
            promotion_height: None,
            command_hint_root: label_root("promotion_command", lane.as_str(), slot_label, ordinal),
            blockers: initial_blockers(config),
            status: PromotionStatus::Blocked,
        }
    }

    pub fn stage_promotion(
        &self,
        staged_fill_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        slot_release_root: &str,
        promotion_height: u64,
        config: &Config,
        duplicate: bool,
    ) -> Result<Self> {
        if staged_fill_root.is_empty() {
            return Err(PromotionError::StagedFillRootMissing);
        }
        if self.wave93_admission_root.is_empty() {
            return Err(PromotionError::AdmissionRootMissing);
        }
        if live_evidence_root.is_empty() {
            return Err(PromotionError::EvidenceRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(PromotionError::SignoffRootMissing);
        }
        if promotion_height < config.min_promotion_height {
            return Err(PromotionError::PromotionHeightTooLow);
        }
        if duplicate {
            return Err(PromotionError::DuplicateOccupancyRoot);
        }
        let occupancy_root = occupancy_root(
            self.lane,
            &self.slot_label,
            self.ordinal,
            staged_fill_root,
            live_evidence_root,
            operator_signoff_root,
            reviewer_signoff_root,
            slot_release_root,
            promotion_height,
        );
        let mut next = self.clone();
        next.wave94_staged_fill_root = Some(staged_fill_root.to_string());
        next.live_evidence_root = Some(live_evidence_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.slot_release_root = Some(slot_release_root.to_string());
        next.slot_occupancy_root = Some(occupancy_root);
        next.promotion_height = Some(promotion_height);
        next.blockers = next.active_blockers(config, duplicate);
        next.status = if next.blockers.is_empty() {
            PromotionStatus::PromotionReady
        } else {
            PromotionStatus::Blocked
        };
        Ok(next)
    }

    pub fn promote_if_clear(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(PromotionError::PromotionStillBlocked);
        }
        let mut next = self.clone();
        next.status = PromotionStatus::Promoted;
        Ok(next)
    }

    fn active_blockers(&self, config: &Config, duplicate: bool) -> Vec<PromotionBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_staged_fill_root && self.wave94_staged_fill_root.is_none() {
            blockers.push(PromotionBlockerKind::MissingStagedFillRoot);
        }
        if config.require_admission_root && self.wave93_admission_root.is_empty() {
            blockers.push(PromotionBlockerKind::MissingAdmissionRoot);
        }
        if config.require_evidence_root && self.live_evidence_root.is_none() {
            blockers.push(PromotionBlockerKind::MissingEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(PromotionBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(PromotionBlockerKind::MissingReviewerSignoffRoot);
        }
        if config.require_slot_release_root && self.slot_release_root.is_none() {
            blockers.push(PromotionBlockerKind::MissingSlotReleaseRoot);
        }
        match self.promotion_height {
            Some(height) if height >= config.min_promotion_height => {}
            _ => blockers.push(PromotionBlockerKind::PromotionHeightTooLow),
        }
        if duplicate {
            blockers.push(PromotionBlockerKind::DuplicateOccupancyRoot);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave92_slot_root": self.wave92_slot_root,
            "wave93_admission_root": self.wave93_admission_root,
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "live_evidence_root": self.live_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "slot_release_root": self.slot_release_root,
            "slot_occupancy_root": self.slot_occupancy_root,
            "promotion_height": self.promotion_height,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LanePromotionGate {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave94_fill_ledger_root: String,
    pub wave92_slot_registry_root: String,
    pub attempts: Vec<PromotionAttempt>,
    pub blocked_root: String,
    pub ready_root: String,
    pub promoted_root: String,
    pub command_root: String,
    pub lane_status: PromotionStatus,
}

impl LanePromotionGate {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let attempts = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| PromotionAttempt::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        Self::from_attempts(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave94_fill_ledger", lane.as_str(), "source", WAVE),
            label_root("wave92_slot_registry", lane.as_str(), "source", WAVE),
            attempts,
        )
    }

    pub fn stage_promotion(
        &self,
        slot_label: &str,
        staged_fill_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        slot_release_root: &str,
        promotion_height: u64,
        config: &Config,
        known_occupancy_roots: &[String],
    ) -> Result<Self> {
        let mut found = false;
        let duplicate = known_occupancy_roots
            .iter()
            .any(|root| root.as_str() == staged_fill_root);
        let mut attempts = Vec::with_capacity(self.attempts.len());
        for attempt in &self.attempts {
            if attempt.slot_label == slot_label {
                attempts.push(attempt.stage_promotion(
                    staged_fill_root,
                    live_evidence_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    slot_release_root,
                    promotion_height,
                    config,
                    duplicate,
                )?);
                found = true;
            } else {
                attempts.push(attempt.clone());
            }
        }
        if !found {
            return Err(PromotionError::SlotMissing);
        }
        Ok(Self::from_attempts(
            self.lane,
            self.lane_title_root.clone(),
            self.wave94_fill_ledger_root.clone(),
            self.wave92_slot_registry_root.clone(),
            attempts,
        ))
    }

    fn from_attempts(
        lane: LaneKind,
        lane_title_root: String,
        wave94_fill_ledger_root: String,
        wave92_slot_registry_root: String,
        attempts: Vec<PromotionAttempt>,
    ) -> Self {
        let blocked_root = blocked_root(&attempts);
        let ready_root = root_from_strings(
            "wave95_ready_promotions",
            attempts.iter().filter_map(|attempt| {
                if attempt.status == PromotionStatus::PromotionReady {
                    attempt.slot_occupancy_root.clone()
                } else {
                    None
                }
            }),
        );
        let promoted_root = root_from_strings(
            "wave95_promoted_slots",
            attempts.iter().filter_map(|attempt| {
                if attempt.status == PromotionStatus::Promoted {
                    attempt.slot_occupancy_root.clone()
                } else {
                    None
                }
            }),
        );
        let command_root = root_from_strings(
            "wave95_promotion_commands",
            attempts
                .iter()
                .map(|attempt| attempt.command_hint_root.clone()),
        );
        let lane_status = if attempts
            .iter()
            .all(|attempt| attempt.status == PromotionStatus::Promoted)
        {
            PromotionStatus::Promoted
        } else if attempts
            .iter()
            .any(|attempt| attempt.status == PromotionStatus::PromotionReady)
        {
            PromotionStatus::PromotionReady
        } else {
            PromotionStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave94_fill_ledger_root,
            wave92_slot_registry_root,
            attempts,
            blocked_root,
            ready_root,
            promoted_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.attempts
            .iter()
            .filter(|attempt| !attempt.blockers.is_empty())
            .count()
    }

    pub fn ready_count(&self) -> usize {
        self.attempts
            .iter()
            .filter(|attempt| attempt.status == PromotionStatus::PromotionReady)
            .count()
    }

    pub fn promoted_count(&self) -> usize {
        self.attempts
            .iter()
            .filter(|attempt| attempt.status == PromotionStatus::Promoted)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave94_fill_ledger_root": self.wave94_fill_ledger_root,
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "attempt_roots": self.attempts.iter().map(PromotionAttempt::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "ready_root": self.ready_root,
            "promoted_root": self.promoted_root,
            "command_root": self.command_root,
            "attempt_count": self.attempts.len(),
            "blocked_count": self.blocked_count(),
            "ready_count": self.ready_count(),
            "promoted_count": self.promoted_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_promotion_gate", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromotionSummary {
    pub promotion_root: String,
    pub blocked_root: String,
    pub ready_root: String,
    pub promoted_root: String,
    pub command_root: String,
    pub readiness_root: String,
    pub lane_count: usize,
    pub slot_count: usize,
    pub blocked_count: usize,
    pub ready_count: usize,
    pub promoted_count: usize,
    pub all_slots_promoted: bool,
    pub production_release_denied: bool,
}

impl PromotionSummary {
    pub fn from_gates(config: &Config, gates: &[LanePromotionGate]) -> Self {
        let promotion_root = root_from_strings(
            "wave95_promotion_gate_root",
            gates.iter().map(LanePromotionGate::state_root),
        );
        let blocked_root = root_from_strings(
            "wave95_blocked_root",
            gates.iter().map(|gate| gate.blocked_root.clone()),
        );
        let ready_root = root_from_strings(
            "wave95_ready_root",
            gates.iter().map(|gate| gate.ready_root.clone()),
        );
        let promoted_root = root_from_strings(
            "wave95_promoted_root",
            gates.iter().map(|gate| gate.promoted_root.clone()),
        );
        let command_root = root_from_strings(
            "wave95_command_root",
            gates.iter().map(|gate| gate.command_root.clone()),
        );
        let slot_count = gates.iter().map(|gate| gate.attempts.len()).sum::<usize>();
        let blocked_count = gates
            .iter()
            .map(LanePromotionGate::blocked_count)
            .sum::<usize>();
        let ready_count = gates
            .iter()
            .map(LanePromotionGate::ready_count)
            .sum::<usize>();
        let promoted_count = gates
            .iter()
            .map(LanePromotionGate::promoted_count)
            .sum::<usize>();
        let all_slots_promoted = slot_count > 0 && promoted_count == slot_count;
        let production_release_denied =
            config.deny_release_when_any_slot_unpromoted && !all_slots_promoted;
        let readiness_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "slot_count": slot_count,
            "blocked_count": blocked_count,
            "ready_count": ready_count,
            "promoted_count": promoted_count,
            "all_slots_promoted": all_slots_promoted,
            "production_release_denied": production_release_denied,
        });
        let readiness_root = record_root("readiness", &readiness_record);
        Self {
            promotion_root,
            blocked_root,
            ready_root,
            promoted_root,
            command_root,
            readiness_root,
            lane_count: gates.len(),
            slot_count,
            blocked_count,
            ready_count,
            promoted_count,
            all_slots_promoted,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "promotion_root": self.promotion_root,
            "blocked_root": self.blocked_root,
            "ready_root": self.ready_root,
            "promoted_root": self.promoted_root,
            "command_root": self.command_root,
            "readiness_root": self.readiness_root,
            "lane_count": self.lane_count,
            "slot_count": self.slot_count,
            "blocked_count": self.blocked_count,
            "ready_count": self.ready_count,
            "promoted_count": self.promoted_count,
            "all_slots_promoted": self.all_slots_promoted,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave92_slot_registry_root: String,
    pub wave93_admission_root: String,
    pub wave94_fill_staging_root: String,
    pub gates: Vec<LanePromotionGate>,
    pub summary: PromotionSummary,
}

impl State {
    pub fn new(config: Config, gates: Vec<LanePromotionGate>) -> Self {
        let wave92_slot_registry_root =
            label_root("wave92_slot_registry", "all_lanes", "source", WAVE);
        let wave93_admission_root = label_root("wave93_admission", "all_lanes", "source", WAVE);
        let wave94_fill_staging_root =
            label_root("wave94_fill_staging", "all_lanes", "source", WAVE);
        let summary = PromotionSummary::from_gates(&config, &gates);
        Self {
            config,
            wave92_slot_registry_root,
            wave93_admission_root,
            wave94_fill_staging_root,
            gates,
            summary,
        }
    }

    pub fn stage_promotion(
        &self,
        lane: LaneKind,
        slot_label: &str,
        staged_fill_root: &str,
        live_evidence_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        slot_release_root: &str,
        promotion_height: u64,
    ) -> Result<Self> {
        let known_roots = self.known_occupancy_roots();
        let mut found = false;
        let mut gates = Vec::with_capacity(self.gates.len());
        for gate in &self.gates {
            if gate.lane == lane {
                gates.push(gate.stage_promotion(
                    slot_label,
                    staged_fill_root,
                    live_evidence_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    slot_release_root,
                    promotion_height,
                    &self.config,
                    &known_roots,
                )?);
                found = true;
            } else {
                gates.push(gate.clone());
            }
        }
        if !found {
            return Err(PromotionError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), gates))
    }

    pub fn known_occupancy_roots(&self) -> Vec<String> {
        self.gates
            .iter()
            .flat_map(|gate| {
                gate.attempts
                    .iter()
                    .filter_map(|attempt| attempt.slot_occupancy_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "wave93_admission_root": self.wave93_admission_root,
            "wave94_fill_staging_root": self.wave94_fill_staging_root,
            "gate_roots": self.gates.iter().map(LanePromotionGate::state_root).collect::<Vec<_>>(),
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
    let gates = vec![
        LanePromotionGate::new(
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
        LanePromotionGate::new(
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
        LanePromotionGate::new(
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
        LanePromotionGate::new(
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
        LanePromotionGate::new(
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
        LanePromotionGate::new(
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
    State::new(config, gates)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<PromotionBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_staged_fill_root {
        blockers.push(PromotionBlockerKind::MissingStagedFillRoot);
    }
    if config.require_evidence_root {
        blockers.push(PromotionBlockerKind::MissingEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(PromotionBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(PromotionBlockerKind::MissingReviewerSignoffRoot);
    }
    if config.require_slot_release_root {
        blockers.push(PromotionBlockerKind::MissingSlotReleaseRoot);
    }
    blockers.push(PromotionBlockerKind::PromotionHeightTooLow);
    blockers
}

fn blocked_root(attempts: &[PromotionAttempt]) -> String {
    let leaves = attempts
        .iter()
        .flat_map(|attempt| {
            attempt.blockers.iter().map(move |blocker| {
                json!({
                    "lane": attempt.lane.as_str(),
                    "slot_label": attempt.slot_label,
                    "blocker": blocker.as_str(),
                    "attempt_root": attempt.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave95_blocked_promotion_attempts", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn occupancy_root(
    lane: LaneKind,
    slot_label: &str,
    ordinal: u64,
    staged_fill_root: &str,
    live_evidence_root: &str,
    operator_signoff_root: &str,
    reviewer_signoff_root: &str,
    slot_release_root: &str,
    promotion_height: u64,
) -> String {
    domain_hash(
        "wave95-live-heavy-gate-receipt-slot-promotion-occupancy",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::U64(ordinal),
            HashPart::Str(staged_fill_root),
            HashPart::Str(live_evidence_root),
            HashPart::Str(operator_signoff_root),
            HashPart::Str(reviewer_signoff_root),
            HashPart::Str(slot_release_root),
            HashPart::U64(promotion_height),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave95-live-heavy-gate-receipt-slot-promotion-record",
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
        "wave95-live-heavy-gate-receipt-slot-promotion-label",
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
