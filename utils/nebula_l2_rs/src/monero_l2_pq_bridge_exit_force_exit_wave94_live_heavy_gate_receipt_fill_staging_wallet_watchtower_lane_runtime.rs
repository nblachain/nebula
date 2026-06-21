use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave94-live-heavy-gate-receipt-fill-staging-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave94";
pub const SOURCE_WAVE_LABEL: &str = "wave93";
pub const TARGET_WAVE_LABEL: &str = "wave92";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-receipt-admission-quarantine-wallet-watchtower-lane";
pub const TARGET_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-registry-wallet-watchtower-lane";
pub const STAGING_LANE: &str =
    "force-exit-live-heavy-gate-receipt-fill-staging-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave94-fill-staging-root";
pub const DEFAULT_STAGE_EPOCH: u64 = 94;
pub const DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS: u64 = 4;
pub const DEFAULT_MIN_USER_REPLAY_ROOTS: u64 = 2;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: u64 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub target_wave_label: String,
    pub source_lane: String,
    pub target_lane: String,
    pub staging_lane: String,
    pub empty_root_marker: String,
    pub stage_epoch: u64,
    pub min_watchtower_quorum_roots: u64,
    pub min_user_replay_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub require_wallet_escape_dry_run: bool,
    pub require_watchtower_quorum: bool,
    pub require_user_runbook_replay: bool,
    pub require_redacted_recovery_proof: bool,
    pub require_wallet_visible_receipt: bool,
    pub require_operator_signoff: bool,
    pub require_roots_only_public_record: bool,
    pub require_wave93_admitted_binding: bool,
    pub require_wave92_slot_binding: bool,
    pub fail_closed_on_empty_stage: bool,
    pub production_enabled: bool,
    pub heavy_gate_execution_allowed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_label: WAVE_LABEL.to_string(),
            source_wave_label: SOURCE_WAVE_LABEL.to_string(),
            target_wave_label: TARGET_WAVE_LABEL.to_string(),
            source_lane: SOURCE_LANE.to_string(),
            target_lane: TARGET_LANE.to_string(),
            staging_lane: STAGING_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            stage_epoch: DEFAULT_STAGE_EPOCH,
            min_watchtower_quorum_roots: DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS,
            min_user_replay_roots: DEFAULT_MIN_USER_REPLAY_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            require_wallet_escape_dry_run: true,
            require_watchtower_quorum: true,
            require_user_runbook_replay: true,
            require_redacted_recovery_proof: true,
            require_wallet_visible_receipt: true,
            require_operator_signoff: true,
            require_roots_only_public_record: true,
            require_wave93_admitted_binding: true,
            require_wave92_slot_binding: true,
            fail_closed_on_empty_stage: true,
            production_enabled: false,
            heavy_gate_execution_allowed: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn min_roots_for_slot(&self, slot: SlotKind) -> u64 {
        match slot {
            SlotKind::WalletEscapeDryRun => 1,
            SlotKind::WatchtowerQuorum => self.min_watchtower_quorum_roots,
            SlotKind::UserRunbookReplay => self.min_user_replay_roots,
            SlotKind::RedactedRecoveryProof => 1,
            SlotKind::WalletVisibleReceipt => 1,
            SlotKind::OperatorSignoff => self.min_operator_signoff_roots,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_label": self.wave_label,
            "source_wave_label": self.source_wave_label,
            "target_wave_label": self.target_wave_label,
            "source_lane": self.source_lane,
            "target_lane": self.target_lane,
            "staging_lane": self.staging_lane,
            "empty_root_marker": self.empty_root_marker,
            "stage_epoch": self.stage_epoch,
            "min_watchtower_quorum_roots": self.min_watchtower_quorum_roots,
            "min_user_replay_roots": self.min_user_replay_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "require_wallet_escape_dry_run": self.require_wallet_escape_dry_run,
            "require_watchtower_quorum": self.require_watchtower_quorum,
            "require_user_runbook_replay": self.require_user_runbook_replay,
            "require_redacted_recovery_proof": self.require_redacted_recovery_proof,
            "require_wallet_visible_receipt": self.require_wallet_visible_receipt,
            "require_operator_signoff": self.require_operator_signoff,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave93_admitted_binding": self.require_wave93_admitted_binding,
            "require_wave92_slot_binding": self.require_wave92_slot_binding,
            "fail_closed_on_empty_stage": self.fail_closed_on_empty_stage,
            "production_enabled": self.production_enabled,
            "heavy_gate_execution_allowed": self.heavy_gate_execution_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotKind {
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl SlotKind {
    pub fn all() -> [Self; 6] {
        [
            Self::WalletEscapeDryRun,
            Self::WatchtowerQuorum,
            Self::UserRunbookReplay,
            Self::RedactedRecoveryProof,
            Self::WalletVisibleReceipt,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletEscapeDryRun => "wallet_escape_dry_run",
            Self::WatchtowerQuorum => "watchtower_quorum",
            Self::UserRunbookReplay => "user_runbook_replay",
            Self::RedactedRecoveryProof => "redacted_recovery_proof",
            Self::WalletVisibleReceipt => "wallet_visible_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FillStatus {
    EmptyBlocked,
    StagedBlocked,
    FillAttemptReady,
    Denied,
}

impl FillStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBlocked => "empty_blocked",
            Self::StagedBlocked => "staged_blocked",
            Self::FillAttemptReady => "fill_attempt_ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_attempt_fill(self) -> bool {
        matches!(self, Self::FillAttemptReady)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FillBlocker {
    EmptyStagedFill,
    Wave93AdmittedRootMissing,
    Wave92SlotRootMissing,
    EvidenceRootMissing,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    ProductionDenied,
    HeavyGateClaimPresent,
}

impl FillBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStagedFill => "empty_staged_fill",
            Self::Wave93AdmittedRootMissing => "wave93_admitted_root_missing",
            Self::Wave92SlotRootMissing => "wave92_slot_root_missing",
            Self::EvidenceRootMissing => "evidence_root_missing",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateClaimPresent => "heavy_gate_claim_present",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldBlockedSlot,
    StageWave93AdmittedRoot,
    BindWave92SlotRoot,
    RequestWalletEscapeDryRunRoot,
    RequestWatchtowerQuorumRoot,
    RequestUserRunbookReplayRoot,
    RequestRedactedRecoveryProofRoot,
    RequestWalletVisibleReceiptRoot,
    RequestOperatorSignoffRoot,
    AttemptWave92SlotFill,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldBlockedSlot => "hold_blocked_slot",
            Self::StageWave93AdmittedRoot => "stage_wave93_admitted_root",
            Self::BindWave92SlotRoot => "bind_wave92_slot_root",
            Self::RequestWalletEscapeDryRunRoot => "request_wallet_escape_dry_run_root",
            Self::RequestWatchtowerQuorumRoot => "request_watchtower_quorum_root",
            Self::RequestUserRunbookReplayRoot => "request_user_runbook_replay_root",
            Self::RequestRedactedRecoveryProofRoot => "request_redacted_recovery_proof_root",
            Self::RequestWalletVisibleReceiptRoot => "request_wallet_visible_receipt_root",
            Self::RequestOperatorSignoffRoot => "request_operator_signoff_root",
            Self::AttemptWave92SlotFill => "attempt_wave92_slot_fill",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub slot_kind: SlotKind,
    pub subject_root: String,
    pub command_root: String,
    pub allowed_while_fail_closed: bool,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, slot_kind: SlotKind, subject_root: &str) -> Self {
        let command_root = record_root(
            "command-hint",
            &json!({
                "kind": kind.as_str(),
                "slot_kind": slot_kind.as_str(),
                "subject_root": subject_root,
            }),
        );
        Self {
            kind,
            slot_kind,
            subject_root: subject_root.to_string(),
            command_root,
            allowed_while_fail_closed: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "subject_root": self.subject_root,
            "command_root": self.command_root,
            "allowed_while_fail_closed": self.allowed_while_fail_closed,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FillAttempt {
    pub slot_kind: SlotKind,
    pub wave93_admitted_root: String,
    pub wave92_slot_root: String,
    pub staging_root: String,
    pub evidence_roots: Vec<String>,
    pub blockers: Vec<FillBlocker>,
    pub command_hint: CommandHint,
    pub status: FillStatus,
    pub heavy_gate_ran: bool,
}

impl FillAttempt {
    pub fn empty(slot_kind: SlotKind, config: &Config) -> Self {
        let wave93_admitted_root = empty_root(&format!("{}-wave93-admitted", slot_kind.as_str()));
        let wave92_slot_root = empty_root(&format!("{}-wave92-slot", slot_kind.as_str()));
        Self::from_roots(
            slot_kind,
            wave93_admitted_root,
            wave92_slot_root,
            Vec::new(),
            false,
            config,
        )
    }

    pub fn from_roots(
        slot_kind: SlotKind,
        wave93_admitted_root: impl Into<String>,
        wave92_slot_root: impl Into<String>,
        evidence_roots: Vec<String>,
        heavy_gate_ran: bool,
        config: &Config,
    ) -> Self {
        let wave93_admitted_root = wave93_admitted_root.into();
        let wave92_slot_root = wave92_slot_root.into();
        let staging_root = staging_root(
            slot_kind,
            &wave93_admitted_root,
            &wave92_slot_root,
            &evidence_roots,
        );
        let mut attempt = Self {
            slot_kind,
            wave93_admitted_root,
            wave92_slot_root,
            staging_root,
            evidence_roots,
            blockers: Vec::new(),
            command_hint: CommandHint::new(CommandHintKind::HoldBlockedSlot, slot_kind, ""),
            status: FillStatus::EmptyBlocked,
            heavy_gate_ran,
        };
        attempt.recompute(config);
        attempt
    }

    pub fn recompute(&mut self, config: &Config) {
        self.evidence_roots.sort();
        self.evidence_roots.dedup();
        self.staging_root = staging_root(
            self.slot_kind,
            &self.wave93_admitted_root,
            &self.wave92_slot_root,
            &self.evidence_roots,
        );
        self.blockers = self.derive_blockers(config);
        self.status = if self.heavy_gate_ran {
            FillStatus::Denied
        } else if self.blockers.is_empty() {
            FillStatus::FillAttemptReady
        } else if is_empty_root(&self.wave93_admitted_root) && is_empty_root(&self.wave92_slot_root)
        {
            FillStatus::EmptyBlocked
        } else {
            FillStatus::StagedBlocked
        };
        self.command_hint = CommandHint::new(
            self.next_command_kind(config),
            self.slot_kind,
            &self.staging_root,
        );
    }

    pub fn derive_blockers(&self, config: &Config) -> Vec<FillBlocker> {
        let mut blockers = Vec::new();
        if is_empty_root(&self.wave93_admitted_root) && is_empty_root(&self.wave92_slot_root) {
            blockers.push(FillBlocker::EmptyStagedFill);
        }
        if config.require_wave93_admitted_binding && is_empty_root(&self.wave93_admitted_root) {
            blockers.push(FillBlocker::Wave93AdmittedRootMissing);
        }
        if config.require_wave92_slot_binding && is_empty_root(&self.wave92_slot_root) {
            blockers.push(FillBlocker::Wave92SlotRootMissing);
        }
        if self.live_evidence_root_count() < config.min_roots_for_slot(self.slot_kind) {
            blockers.push(FillBlocker::EvidenceRootMissing);
        }
        if !self.roots_are_shaped() {
            blockers.push(FillBlocker::RootShapeInvalid);
        }
        if !config.require_roots_only_public_record {
            blockers.push(FillBlocker::RootsOnlyRecordMissing);
        }
        if !config.production_enabled {
            blockers.push(FillBlocker::ProductionDenied);
        }
        if self.heavy_gate_ran || config.heavy_gate_execution_allowed {
            blockers.push(FillBlocker::HeavyGateClaimPresent);
        }
        blockers.sort();
        blockers.dedup();
        blockers
    }

    pub fn next_command_kind(&self, config: &Config) -> CommandHintKind {
        if self.status.can_attempt_fill() {
            return CommandHintKind::AttemptWave92SlotFill;
        }
        if is_empty_root(&self.wave93_admitted_root) {
            return CommandHintKind::StageWave93AdmittedRoot;
        }
        if is_empty_root(&self.wave92_slot_root) {
            return CommandHintKind::BindWave92SlotRoot;
        }
        if self.live_evidence_root_count() < config.min_roots_for_slot(self.slot_kind) {
            return command_for_slot(self.slot_kind);
        }
        CommandHintKind::HoldBlockedSlot
    }

    pub fn roots_are_shaped(&self) -> bool {
        is_root_like(&self.wave93_admitted_root)
            && is_root_like(&self.wave92_slot_root)
            && is_root_like(&self.staging_root)
            && self.evidence_roots.iter().all(|root| is_root_like(root))
    }

    pub fn live_evidence_root_count(&self) -> u64 {
        self.evidence_roots
            .iter()
            .filter(|root| !is_empty_root(root))
            .count() as u64
    }

    pub fn public_record_without_root(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave93_admitted_root": self.wave93_admitted_root,
            "wave92_slot_root": self.wave92_slot_root,
            "staging_root": self.staging_root,
            "evidence_roots": self.evidence_roots,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "command_hint": self.command_hint.public_record(),
            "status": self.status.as_str(),
            "heavy_gate_ran": self.heavy_gate_ran,
            "live_evidence_root_count": self.live_evidence_root_count(),
            "can_attempt_fill": self.status.can_attempt_fill(),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_root();
        if let Some(map) = record.as_object_mut() {
            map.insert(
                "fill_attempt_root".to_string(),
                Value::String(self.state_root()),
            );
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("fill-attempt", &self.public_record_without_root())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FillSummary {
    pub fail_closed: bool,
    pub production_denied: bool,
    pub fill_attempts_ready: u64,
    pub blocked_slots: u64,
    pub denied_slots: u64,
    pub staged_fills: u64,
    pub blocker_root: String,
    pub command_root: String,
    pub admitted_binding_root: String,
    pub fill_attempt_root: String,
    pub heavy_gate_ran: bool,
}

impl FillSummary {
    pub fn from_attempts(attempts: &BTreeMap<String, FillAttempt>) -> Self {
        let fill_attempts_ready = attempts
            .values()
            .filter(|attempt| attempt.status == FillStatus::FillAttemptReady)
            .count() as u64;
        let blocked_slots = attempts
            .values()
            .filter(|attempt| {
                matches!(
                    attempt.status,
                    FillStatus::EmptyBlocked | FillStatus::StagedBlocked
                )
            })
            .count() as u64;
        let denied_slots = attempts
            .values()
            .filter(|attempt| attempt.status == FillStatus::Denied)
            .count() as u64;
        let staged_fills = attempts
            .values()
            .filter(|attempt| !is_empty_root(&attempt.wave93_admitted_root))
            .count() as u64;
        let heavy_gate_ran = attempts.values().any(|attempt| attempt.heavy_gate_ran);
        let blocker_root = merkle_root(
            "WAVE94-WALLET-WATCHTOWER-SLOT-FILL-BLOCKERS",
            &attempts
                .values()
                .flat_map(|attempt| {
                    attempt.blockers.iter().map(|blocker| {
                        json!({
                            "slot_kind": attempt.slot_kind.as_str(),
                            "blocker": blocker.as_str(),
                            "fill_attempt_root": attempt.state_root(),
                        })
                    })
                })
                .collect::<Vec<_>>(),
        );
        let command_root = merkle_root(
            "WAVE94-WALLET-WATCHTOWER-COMMAND-HINTS",
            &attempts
                .values()
                .map(|attempt| attempt.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let admitted_binding_root = merkle_root(
            "WAVE94-WALLET-WATCHTOWER-ADMITTED-ROOT-BINDINGS",
            &attempts
                .values()
                .map(|attempt| {
                    json!({
                        "slot_kind": attempt.slot_kind.as_str(),
                        "wave93_admitted_root": attempt.wave93_admitted_root,
                        "wave92_slot_root": attempt.wave92_slot_root,
                        "staging_root": attempt.staging_root,
                    })
                })
                .collect::<Vec<_>>(),
        );
        let fill_attempt_root = merkle_root(
            "WAVE94-WALLET-WATCHTOWER-FILL-ATTEMPTS",
            &attempts
                .values()
                .map(|attempt| Value::String(attempt.state_root()))
                .collect::<Vec<_>>(),
        );
        Self {
            fail_closed: fill_attempts_ready == 0 || blocked_slots > 0 || denied_slots > 0,
            production_denied: true,
            fill_attempts_ready,
            blocked_slots,
            denied_slots,
            staged_fills,
            blocker_root,
            command_root,
            admitted_binding_root,
            fill_attempt_root,
            heavy_gate_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "production_denied": self.production_denied,
            "fill_attempts_ready": self.fill_attempts_ready,
            "blocked_slots": self.blocked_slots,
            "denied_slots": self.denied_slots,
            "staged_fills": self.staged_fills,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "admitted_binding_root": self.admitted_binding_root,
            "fill_attempt_root": self.fill_attempt_root,
            "heavy_gate_ran": self.heavy_gate_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("fill-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub fill_attempts: BTreeMap<String, FillAttempt>,
    pub summary: FillSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let fill_attempts = SlotKind::all()
            .iter()
            .map(|slot_kind| {
                let attempt = FillAttempt::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), attempt)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = FillSummary::from_attempts(&fill_attempts);
        Self {
            config,
            fill_attempts,
            summary,
        }
    }

    pub fn stage_fill_attempt(
        mut self,
        slot_kind: SlotKind,
        wave93_admitted_root: impl Into<String>,
        wave92_slot_root: impl Into<String>,
        evidence_roots: Vec<String>,
    ) -> Result<Self> {
        let attempt = FillAttempt::from_roots(
            slot_kind,
            wave93_admitted_root,
            wave92_slot_root,
            evidence_roots,
            false,
            &self.config,
        );
        self.fill_attempts
            .insert(slot_kind.as_str().to_string(), attempt);
        self.recompute();
        Ok(self)
    }

    pub fn deny_heavy_gate_claim(mut self, slot_kind: SlotKind) -> Result<Self> {
        let key = slot_kind.as_str().to_string();
        match self.fill_attempts.remove(&key) {
            Some(mut attempt) => {
                attempt.heavy_gate_ran = true;
                attempt.recompute(&self.config);
                self.fill_attempts.insert(key, attempt);
                self.recompute();
                Ok(self)
            }
            None => Err(format!("fill slot missing: {}", slot_kind.as_str())),
        }
    }

    pub fn recompute(&mut self) {
        for attempt in self.fill_attempts.values_mut() {
            attempt.recompute(&self.config);
        }
        self.summary = FillSummary::from_attempts(&self.fill_attempts);
    }

    pub fn fill_attempt_roots(&self) -> BTreeMap<String, String> {
        self.fill_attempts
            .iter()
            .map(|(key, attempt)| (key.clone(), attempt.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn fill_attempts_root(&self) -> String {
        merkle_root(
            "WAVE94-WALLET-WATCHTOWER-FILL-ATTEMPT-ROOTS",
            &self
                .fill_attempt_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "fill_attempts": self.fill_attempts.iter().map(|(key, attempt)| (key.clone(), attempt.public_record())).collect::<BTreeMap<_, _>>(),
            "fill_attempt_roots": self.fill_attempt_roots(),
            "fill_attempts_root": self.fill_attempts_root(),
            "summary": self.summary.public_record(),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Some(map) = record.as_object_mut() {
            map.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    State::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn empty_fill_staging_runtime() -> Runtime {
    devnet()
}

pub fn staging_root(
    slot_kind: SlotKind,
    wave93_admitted_root: &str,
    wave92_slot_root: &str,
    evidence_roots: &[String],
) -> String {
    record_root(
        "staging-root",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "source_wave_label": SOURCE_WAVE_LABEL,
            "target_wave_label": TARGET_WAVE_LABEL,
            "wave93_admitted_root": wave93_admitted_root,
            "wave92_slot_root": wave92_slot_root,
            "evidence_roots": evidence_roots,
            "roots_only": true,
            "raw_wallet_material_absent": true,
            "heavy_gate_ran": false,
        }),
    )
}

fn command_for_slot(slot_kind: SlotKind) -> CommandHintKind {
    match slot_kind {
        SlotKind::WalletEscapeDryRun => CommandHintKind::RequestWalletEscapeDryRunRoot,
        SlotKind::WatchtowerQuorum => CommandHintKind::RequestWatchtowerQuorumRoot,
        SlotKind::UserRunbookReplay => CommandHintKind::RequestUserRunbookReplayRoot,
        SlotKind::RedactedRecoveryProof => CommandHintKind::RequestRedactedRecoveryProofRoot,
        SlotKind::WalletVisibleReceipt => CommandHintKind::RequestWalletVisibleReceiptRoot,
        SlotKind::OperatorSignoff => CommandHintKind::RequestOperatorSignoffRoot,
    }
}

fn empty_root(label: &str) -> String {
    let root = record_root(
        "empty-fill-staging-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "label": label,
        }),
    );
    format!("{EMPTY_ROOT_MARKER}:{root}")
}

fn is_empty_root(root: &str) -> bool {
    root.is_empty() || root.contains(EMPTY_ROOT_MARKER)
}

fn is_root_like(root: &str) -> bool {
    !root.is_empty()
        && root.len() >= 16
        && root
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ':' | '-' | '_' | '.'))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    domain_hash(
        "WAVE94-WALLET-WATCHTOWER-RECEIPT-FILL-STAGING",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
