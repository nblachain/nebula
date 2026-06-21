use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave94-live-heavy-gate-receipt-fill-staging-v1";
const WAVE: u64 = 94;
const SLOT_REGISTRY_WAVE: u64 = 92;
const ADMISSION_WAVE: u64 = 93;
const MIN_FILL_HEIGHT: u64 = 940_000;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, FillError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FillError {
    LaneMissing,
    SlotMissing,
    ReceiptRootMissing,
    AdmissionRootMissing,
    SlotRootMissing,
    FillHeightTooLow,
    DuplicateFillRoot,
    FillStillBlocked,
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
            Self::Compile => "Compile receipt fill staging",
            Self::RuntimeReplay => "Runtime replay receipt fill staging",
            Self::AuditSecurity => "Audit security receipt fill staging",
            Self::BridgeCustody => "Bridge custody receipt fill staging",
            Self::WalletWatchtower => "Wallet watchtower receipt fill staging",
            Self::PqReservePrivacy => "PQ reserve privacy receipt fill staging",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FillStatus {
    Empty,
    Staged,
    Blocked,
    Filled,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FillBlockerKind {
    NoAdmittedReceiptRoot,
    NoWave92SlotRoot,
    NoWave93AdmissionRoot,
    NoGateEvidenceRoot,
    NoOperatorSignoffRoot,
    FillHeightTooLow,
    DuplicateFillRoot,
    SlotNotReleased,
}

impl FillBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoAdmittedReceiptRoot => "no_admitted_receipt_root",
            Self::NoWave92SlotRoot => "no_wave92_slot_root",
            Self::NoWave93AdmissionRoot => "no_wave93_admission_root",
            Self::NoGateEvidenceRoot => "no_gate_evidence_root",
            Self::NoOperatorSignoffRoot => "no_operator_signoff_root",
            Self::FillHeightTooLow => "fill_height_too_low",
            Self::DuplicateFillRoot => "duplicate_fill_root",
            Self::SlotNotReleased => "slot_not_released",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub slot_registry_wave: u64,
    pub admission_wave: u64,
    pub min_fill_height: u64,
    pub require_admitted_receipt_root: bool,
    pub require_wave92_slot_root: bool,
    pub require_wave93_admission_root: bool,
    pub require_gate_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub deny_release_when_any_slot_blocked: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            slot_registry_wave: SLOT_REGISTRY_WAVE,
            admission_wave: ADMISSION_WAVE,
            min_fill_height: MIN_FILL_HEIGHT,
            require_admitted_receipt_root: true,
            require_wave92_slot_root: true,
            require_wave93_admission_root: true,
            require_gate_evidence_root: true,
            require_operator_signoff_root: true,
            deny_release_when_any_slot_blocked: true,
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
            "slot_registry_wave": self.slot_registry_wave,
            "admission_wave": self.admission_wave,
            "min_fill_height": self.min_fill_height,
            "require_admitted_receipt_root": self.require_admitted_receipt_root,
            "require_wave92_slot_root": self.require_wave92_slot_root,
            "require_wave93_admission_root": self.require_wave93_admission_root,
            "require_gate_evidence_root": self.require_gate_evidence_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "deny_release_when_any_slot_blocked": self.deny_release_when_any_slot_blocked,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlotFillAttempt {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave92_slot_root: String,
    pub wave93_admission_root: String,
    pub admitted_receipt_root: Option<String>,
    pub gate_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub staged_fill_root: Option<String>,
    pub fill_height: Option<u64>,
    pub slot_release_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<FillBlockerKind>,
    pub status: FillStatus,
}

impl SlotFillAttempt {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        let lane_name = lane.as_str();
        let wave92_slot_root = label_root("wave92_slot", lane_name, slot_label, ordinal);
        let wave93_admission_root = label_root("wave93_admission", lane_name, slot_label, ordinal);
        let slot_release_root = label_root("slot_release_policy", lane_name, slot_label, ordinal);
        let command_hint_root = label_root("operator_fill_command", lane_name, slot_label, ordinal);
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave92_slot_root,
            wave93_admission_root,
            admitted_receipt_root: None,
            gate_evidence_root: None,
            operator_signoff_root: None,
            staged_fill_root: None,
            fill_height: None,
            slot_release_root,
            command_hint_root,
            blockers: initial_blockers(config),
            status: FillStatus::Blocked,
        }
    }

    pub fn stage(
        &self,
        admitted_receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        fill_height: u64,
        config: &Config,
        duplicate: bool,
    ) -> Result<Self> {
        if admitted_receipt_root.is_empty() {
            return Err(FillError::ReceiptRootMissing);
        }
        if self.wave92_slot_root.is_empty() {
            return Err(FillError::SlotRootMissing);
        }
        if self.wave93_admission_root.is_empty() {
            return Err(FillError::AdmissionRootMissing);
        }
        if fill_height < config.min_fill_height {
            return Err(FillError::FillHeightTooLow);
        }
        if duplicate {
            return Err(FillError::DuplicateFillRoot);
        }
        let staged_fill_root = fill_root(
            self.lane,
            &self.slot_label,
            self.ordinal,
            admitted_receipt_root,
            gate_evidence_root,
            operator_signoff_root,
            fill_height,
        );
        let mut next = self.clone();
        next.admitted_receipt_root = Some(admitted_receipt_root.to_string());
        next.gate_evidence_root = Some(gate_evidence_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.staged_fill_root = Some(staged_fill_root);
        next.fill_height = Some(fill_height);
        next.blockers = next.active_blockers(config, duplicate);
        next.status = if next.blockers.is_empty() {
            FillStatus::Staged
        } else {
            FillStatus::Blocked
        };
        Ok(next)
    }

    pub fn fill_if_clear(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(FillError::FillStillBlocked);
        }
        let mut next = self.clone();
        next.status = FillStatus::Filled;
        Ok(next)
    }

    fn active_blockers(&self, config: &Config, duplicate: bool) -> Vec<FillBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_admitted_receipt_root && self.admitted_receipt_root.is_none() {
            blockers.push(FillBlockerKind::NoAdmittedReceiptRoot);
        }
        if config.require_wave92_slot_root && self.wave92_slot_root.is_empty() {
            blockers.push(FillBlockerKind::NoWave92SlotRoot);
        }
        if config.require_wave93_admission_root && self.wave93_admission_root.is_empty() {
            blockers.push(FillBlockerKind::NoWave93AdmissionRoot);
        }
        if config.require_gate_evidence_root && self.gate_evidence_root.is_none() {
            blockers.push(FillBlockerKind::NoGateEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(FillBlockerKind::NoOperatorSignoffRoot);
        }
        match self.fill_height {
            Some(height) if height >= config.min_fill_height => {}
            _ => blockers.push(FillBlockerKind::FillHeightTooLow),
        }
        if duplicate {
            blockers.push(FillBlockerKind::DuplicateFillRoot);
        }
        if self.slot_release_root.is_empty() {
            blockers.push(FillBlockerKind::SlotNotReleased);
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
            "admitted_receipt_root": self.admitted_receipt_root,
            "gate_evidence_root": self.gate_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "staged_fill_root": self.staged_fill_root,
            "fill_height": self.fill_height,
            "slot_release_root": self.slot_release_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("slot_fill_attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneFillLedger {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave92_registry_root: String,
    pub wave93_admission_registry_root: String,
    pub fill_attempts: Vec<SlotFillAttempt>,
    pub staged_fill_root: String,
    pub blocked_fill_root: String,
    pub command_root: String,
    pub lane_status: FillStatus,
}

impl LaneFillLedger {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let fill_attempts = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| SlotFillAttempt::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        Self::from_attempts(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave92_lane_registry", lane.as_str(), "slot_registry", WAVE),
            label_root(
                "wave93_lane_admission",
                lane.as_str(),
                "admission_registry",
                WAVE,
            ),
            fill_attempts,
        )
    }

    pub fn stage_fill(
        &self,
        slot_label: &str,
        admitted_receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        fill_height: u64,
        config: &Config,
        known_fill_roots: &[String],
    ) -> Result<Self> {
        let mut found = false;
        let duplicate = known_fill_roots
            .iter()
            .any(|root| root.as_str() == admitted_receipt_root);
        let mut attempts = Vec::with_capacity(self.fill_attempts.len());
        for attempt in &self.fill_attempts {
            if attempt.slot_label == slot_label {
                attempts.push(attempt.stage(
                    admitted_receipt_root,
                    gate_evidence_root,
                    operator_signoff_root,
                    fill_height,
                    config,
                    duplicate,
                )?);
                found = true;
            } else {
                attempts.push(attempt.clone());
            }
        }
        if !found {
            return Err(FillError::SlotMissing);
        }
        Ok(Self::from_attempts(
            self.lane,
            self.lane_title_root.clone(),
            self.wave92_registry_root.clone(),
            self.wave93_admission_registry_root.clone(),
            attempts,
        ))
    }

    fn from_attempts(
        lane: LaneKind,
        lane_title_root: String,
        wave92_registry_root: String,
        wave93_admission_registry_root: String,
        fill_attempts: Vec<SlotFillAttempt>,
    ) -> Self {
        let staged_fill_root = root_from_strings(
            "wave94_staged_fill_root",
            fill_attempts.iter().filter_map(|attempt| {
                if matches!(attempt.status, FillStatus::Staged | FillStatus::Filled) {
                    attempt.staged_fill_root.clone()
                } else {
                    None
                }
            }),
        );
        let blocked_fill_root = blocked_root(&fill_attempts);
        let command_root = root_from_strings(
            "wave94_command_root",
            fill_attempts
                .iter()
                .map(|attempt| attempt.command_hint_root.clone()),
        );
        let lane_status = if fill_attempts
            .iter()
            .all(|attempt| attempt.status == FillStatus::Filled)
        {
            FillStatus::Filled
        } else if fill_attempts
            .iter()
            .any(|attempt| attempt.status == FillStatus::Staged)
        {
            FillStatus::Staged
        } else {
            FillStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave92_registry_root,
            wave93_admission_registry_root,
            fill_attempts,
            staged_fill_root,
            blocked_fill_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| !attempt.blockers.is_empty())
            .count()
    }

    pub fn staged_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| matches!(attempt.status, FillStatus::Staged | FillStatus::Filled))
            .count()
    }

    pub fn filled_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| attempt.status == FillStatus::Filled)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave92_registry_root": self.wave92_registry_root,
            "wave93_admission_registry_root": self.wave93_admission_registry_root,
            "fill_attempt_roots": self.fill_attempts.iter().map(SlotFillAttempt::state_root).collect::<Vec<_>>(),
            "staged_fill_root": self.staged_fill_root,
            "blocked_fill_root": self.blocked_fill_root,
            "command_root": self.command_root,
            "attempt_count": self.fill_attempts.len(),
            "blocked_count": self.blocked_count(),
            "staged_count": self.staged_count(),
            "filled_count": self.filled_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_fill_ledger", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FillSummary {
    pub ledger_root: String,
    pub staged_fill_root: String,
    pub blocked_fill_root: String,
    pub command_root: String,
    pub readiness_root: String,
    pub lane_count: usize,
    pub slot_count: usize,
    pub blocked_count: usize,
    pub staged_count: usize,
    pub filled_count: usize,
    pub all_slots_filled: bool,
    pub production_release_denied: bool,
}

impl FillSummary {
    pub fn from_ledgers(config: &Config, ledgers: &[LaneFillLedger]) -> Self {
        let ledger_root = root_from_strings(
            "wave94_ledger_root",
            ledgers.iter().map(LaneFillLedger::state_root),
        );
        let staged_fill_root = root_from_strings(
            "wave94_all_staged_fill_root",
            ledgers.iter().map(|ledger| ledger.staged_fill_root.clone()),
        );
        let blocked_fill_root = root_from_strings(
            "wave94_all_blocked_fill_root",
            ledgers
                .iter()
                .map(|ledger| ledger.blocked_fill_root.clone()),
        );
        let command_root = root_from_strings(
            "wave94_all_command_root",
            ledgers.iter().map(|ledger| ledger.command_root.clone()),
        );
        let slot_count = ledgers
            .iter()
            .map(|ledger| ledger.fill_attempts.len())
            .sum::<usize>();
        let blocked_count = ledgers
            .iter()
            .map(LaneFillLedger::blocked_count)
            .sum::<usize>();
        let staged_count = ledgers
            .iter()
            .map(LaneFillLedger::staged_count)
            .sum::<usize>();
        let filled_count = ledgers
            .iter()
            .map(LaneFillLedger::filled_count)
            .sum::<usize>();
        let all_slots_filled = slot_count > 0 && filled_count == slot_count;
        let production_release_denied =
            config.deny_release_when_any_slot_blocked && !all_slots_filled;
        let readiness_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "slot_count": slot_count,
            "blocked_count": blocked_count,
            "staged_count": staged_count,
            "filled_count": filled_count,
            "all_slots_filled": all_slots_filled,
            "production_release_denied": production_release_denied,
        });
        let readiness_root = record_root("readiness", &readiness_record);
        Self {
            ledger_root,
            staged_fill_root,
            blocked_fill_root,
            command_root,
            readiness_root,
            lane_count: ledgers.len(),
            slot_count,
            blocked_count,
            staged_count,
            filled_count,
            all_slots_filled,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "ledger_root": self.ledger_root,
            "staged_fill_root": self.staged_fill_root,
            "blocked_fill_root": self.blocked_fill_root,
            "command_root": self.command_root,
            "readiness_root": self.readiness_root,
            "lane_count": self.lane_count,
            "slot_count": self.slot_count,
            "blocked_count": self.blocked_count,
            "staged_count": self.staged_count,
            "filled_count": self.filled_count,
            "all_slots_filled": self.all_slots_filled,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("fill_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave92_slot_registry_root: String,
    pub wave93_admission_quarantine_root: String,
    pub ledgers: Vec<LaneFillLedger>,
    pub summary: FillSummary,
}

impl State {
    pub fn new(config: Config, ledgers: Vec<LaneFillLedger>) -> Self {
        let wave92_slot_registry_root =
            label_root("wave92_slot_registry", "all_lanes", "source", WAVE);
        let wave93_admission_quarantine_root =
            label_root("wave93_admission_quarantine", "all_lanes", "source", WAVE);
        let summary = FillSummary::from_ledgers(&config, &ledgers);
        Self {
            config,
            wave92_slot_registry_root,
            wave93_admission_quarantine_root,
            ledgers,
            summary,
        }
    }

    pub fn stage_fill(
        &self,
        lane: LaneKind,
        slot_label: &str,
        admitted_receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        fill_height: u64,
    ) -> Result<Self> {
        let known_roots = self.known_staged_roots();
        let mut found = false;
        let mut ledgers = Vec::with_capacity(self.ledgers.len());
        for ledger in &self.ledgers {
            if ledger.lane == lane {
                ledgers.push(ledger.stage_fill(
                    slot_label,
                    admitted_receipt_root,
                    gate_evidence_root,
                    operator_signoff_root,
                    fill_height,
                    &self.config,
                    &known_roots,
                )?);
                found = true;
            } else {
                ledgers.push(ledger.clone());
            }
        }
        if !found {
            return Err(FillError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), ledgers))
    }

    pub fn known_staged_roots(&self) -> Vec<String> {
        self.ledgers
            .iter()
            .flat_map(|ledger| {
                ledger
                    .fill_attempts
                    .iter()
                    .filter_map(|attempt| attempt.staged_fill_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "wave93_admission_quarantine_root": self.wave93_admission_quarantine_root,
            "ledger_roots": self.ledgers.iter().map(LaneFillLedger::state_root).collect::<Vec<_>>(),
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
    let ledgers = vec![
        LaneFillLedger::new(
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
        LaneFillLedger::new(
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
        LaneFillLedger::new(
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
        LaneFillLedger::new(
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
        LaneFillLedger::new(
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
        LaneFillLedger::new(
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
    State::new(config, ledgers)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<FillBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_admitted_receipt_root {
        blockers.push(FillBlockerKind::NoAdmittedReceiptRoot);
    }
    if config.require_gate_evidence_root {
        blockers.push(FillBlockerKind::NoGateEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(FillBlockerKind::NoOperatorSignoffRoot);
    }
    blockers.push(FillBlockerKind::FillHeightTooLow);
    blockers
}

fn blocked_root(attempts: &[SlotFillAttempt]) -> String {
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
    merkle_root("wave94_blocked_fill_attempts", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn fill_root(
    lane: LaneKind,
    slot_label: &str,
    ordinal: u64,
    admitted_receipt_root: &str,
    gate_evidence_root: &str,
    operator_signoff_root: &str,
    fill_height: u64,
) -> String {
    domain_hash(
        "wave94-live-heavy-gate-receipt-fill-staging-fill",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::U64(ordinal),
            HashPart::Str(admitted_receipt_root),
            HashPart::Str(gate_evidence_root),
            HashPart::Str(operator_signoff_root),
            HashPart::U64(fill_height),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave94-live-heavy-gate-receipt-fill-staging-record",
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
        "wave94-live-heavy-gate-receipt-fill-staging-label",
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
