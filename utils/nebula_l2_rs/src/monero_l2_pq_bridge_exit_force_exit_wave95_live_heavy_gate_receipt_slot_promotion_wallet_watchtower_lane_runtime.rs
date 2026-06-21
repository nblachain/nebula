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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave95";
pub const SOURCE_WAVE_LABEL: &str = "wave94";
pub const TARGET_WAVE_LABEL: &str = "wave92";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-receipt-fill-staging-wallet-watchtower-lane";
pub const TARGET_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-registry-wallet-watchtower-lane";
pub const PROMOTION_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-promotion-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave95-slot-promotion-root";
pub const DEFAULT_PROMOTION_EPOCH: u64 = 95;
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
    pub promotion_lane: String,
    pub empty_root_marker: String,
    pub promotion_epoch: u64,
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
    pub require_wave94_staged_fill_binding: bool,
    pub require_wave92_slot_occupancy_placeholder: bool,
    pub fail_closed_on_empty_promotion: bool,
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
            promotion_lane: PROMOTION_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            promotion_epoch: DEFAULT_PROMOTION_EPOCH,
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
            require_wave94_staged_fill_binding: true,
            require_wave92_slot_occupancy_placeholder: true,
            fail_closed_on_empty_promotion: true,
            production_enabled: false,
            heavy_gate_execution_allowed: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn min_roots_for_slot(&self, slot: PromotionSlotKind) -> u64 {
        match slot {
            PromotionSlotKind::WalletEscapeDryRun => 1,
            PromotionSlotKind::WatchtowerQuorum => self.min_watchtower_quorum_roots,
            PromotionSlotKind::UserRunbookReplay => self.min_user_replay_roots,
            PromotionSlotKind::RedactedRecoveryProof => 1,
            PromotionSlotKind::WalletVisibleReceipt => 1,
            PromotionSlotKind::OperatorSignoff => self.min_operator_signoff_roots,
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
            "promotion_lane": self.promotion_lane,
            "empty_root_marker": self.empty_root_marker,
            "promotion_epoch": self.promotion_epoch,
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
            "require_wave94_staged_fill_binding": self.require_wave94_staged_fill_binding,
            "require_wave92_slot_occupancy_placeholder": self.require_wave92_slot_occupancy_placeholder,
            "fail_closed_on_empty_promotion": self.fail_closed_on_empty_promotion,
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
pub enum PromotionSlotKind {
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl PromotionSlotKind {
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
pub enum PromotionStatus {
    EmptyBlocked,
    StagedBlocked,
    PromotionReady,
    Denied,
}

impl PromotionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBlocked => "empty_blocked",
            Self::StagedBlocked => "staged_blocked",
            Self::PromotionReady => "promotion_ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_promote(self) -> bool {
        matches!(self, Self::PromotionReady)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBlocker {
    EmptyStagedFill,
    Wave94StagedFillRootMissing,
    Wave92SlotOccupancyPlaceholderMissing,
    EvidenceRootMissing,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    WalletEscapeDryRunMissing,
    WatchtowerQuorumMissing,
    UserRunbookReplayMissing,
    RedactedRecoveryProofMissing,
    WalletVisibleReceiptMissing,
    OperatorSignoffMissing,
    ProductionDenied,
    HeavyGateClaimPresent,
}

impl PromotionBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStagedFill => "empty_staged_fill",
            Self::Wave94StagedFillRootMissing => "wave94_staged_fill_root_missing",
            Self::Wave92SlotOccupancyPlaceholderMissing => {
                "wave92_slot_occupancy_placeholder_missing"
            }
            Self::EvidenceRootMissing => "evidence_root_missing",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::WalletEscapeDryRunMissing => "wallet_escape_dry_run_missing",
            Self::WatchtowerQuorumMissing => "watchtower_quorum_missing",
            Self::UserRunbookReplayMissing => "user_runbook_replay_missing",
            Self::RedactedRecoveryProofMissing => "redacted_recovery_proof_missing",
            Self::WalletVisibleReceiptMissing => "wallet_visible_receipt_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateClaimPresent => "heavy_gate_claim_present",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldBlockedPromotion,
    StageWave94FillRoot,
    BindWave92SlotOccupancyPlaceholder,
    RequestWalletEscapeDryRunRoot,
    RequestWatchtowerQuorumRoot,
    RequestUserRunbookReplayRoot,
    RequestRedactedRecoveryProofRoot,
    RequestWalletVisibleReceiptRoot,
    RequestOperatorSignoffRoot,
    PromoteToSlotOccupancyClaim,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldBlockedPromotion => "hold_blocked_promotion",
            Self::StageWave94FillRoot => "stage_wave94_fill_root",
            Self::BindWave92SlotOccupancyPlaceholder => "bind_wave92_slot_occupancy_placeholder",
            Self::RequestWalletEscapeDryRunRoot => "request_wallet_escape_dry_run_root",
            Self::RequestWatchtowerQuorumRoot => "request_watchtower_quorum_root",
            Self::RequestUserRunbookReplayRoot => "request_user_runbook_replay_root",
            Self::RequestRedactedRecoveryProofRoot => "request_redacted_recovery_proof_root",
            Self::RequestWalletVisibleReceiptRoot => "request_wallet_visible_receipt_root",
            Self::RequestOperatorSignoffRoot => "request_operator_signoff_root",
            Self::PromoteToSlotOccupancyClaim => "promote_to_slot_occupancy_claim",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub command_root: String,
    pub next_blocker_root: String,
}

impl CommandHint {
    pub fn new(
        kind: CommandHintKind,
        slot_kind: PromotionSlotKind,
        blockers: &[PromotionBlocker],
    ) -> Self {
        let blocker_names = blockers
            .iter()
            .map(|blocker| Value::String(blocker.as_str().to_string()))
            .collect::<Vec<_>>();
        let next_blocker_root =
            merkle_root("WAVE95-WALLET-WATCHTOWER-NEXT-BLOCKERS", &blocker_names);
        let command_root = record_root(
            "command-hint",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "hint": kind.as_str(),
                "next_blocker_root": next_blocker_root,
            }),
        );
        Self {
            kind,
            command_root,
            next_blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "next_blocker_root": self.next_blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub slot_kind: PromotionSlotKind,
    pub wave94_staged_fill_root: String,
    pub wave92_slot_occupancy_placeholder_root: String,
    pub evidence_roots: Vec<String>,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_claim_placeholder_root: String,
    pub blockers: Vec<PromotionBlocker>,
    pub status: PromotionStatus,
    pub command_hint: CommandHint,
    pub heavy_gate_claimed: bool,
    pub promoted: bool,
}

impl PromotionAttempt {
    pub fn empty(slot_kind: PromotionSlotKind, config: &Config) -> Self {
        Self::from_roots(
            slot_kind,
            empty_root("wave94-staged-fill"),
            empty_root("wave92-slot-occupancy-placeholder"),
            Vec::new(),
            false,
            config,
        )
    }

    pub fn from_roots(
        slot_kind: PromotionSlotKind,
        wave94_staged_fill_root: impl Into<String>,
        wave92_slot_occupancy_placeholder_root: impl Into<String>,
        evidence_roots: Vec<String>,
        heavy_gate_claimed: bool,
        config: &Config,
    ) -> Self {
        let wave94_staged_fill_root = wave94_staged_fill_root.into();
        let wave92_slot_occupancy_placeholder_root = wave92_slot_occupancy_placeholder_root.into();
        let staged_fill_binding_root = staged_fill_binding_root(
            slot_kind,
            &wave94_staged_fill_root,
            &wave92_slot_occupancy_placeholder_root,
            &evidence_roots,
        );
        let slot_occupancy_claim_placeholder_root = slot_occupancy_claim_placeholder_root(
            slot_kind,
            &wave94_staged_fill_root,
            &wave92_slot_occupancy_placeholder_root,
        );
        let mut attempt = Self {
            slot_kind,
            wave94_staged_fill_root,
            wave92_slot_occupancy_placeholder_root,
            evidence_roots,
            staged_fill_binding_root,
            slot_occupancy_claim_placeholder_root,
            blockers: Vec::new(),
            status: PromotionStatus::EmptyBlocked,
            command_hint: CommandHint::new(CommandHintKind::HoldBlockedPromotion, slot_kind, &[]),
            heavy_gate_claimed,
            promoted: false,
        };
        attempt.recompute(config);
        attempt
    }

    pub fn recompute(&mut self, config: &Config) {
        self.blockers = promotion_blockers(self, config);
        self.status = if self.heavy_gate_claimed {
            PromotionStatus::Denied
        } else if is_empty_root(&self.wave94_staged_fill_root) {
            PromotionStatus::EmptyBlocked
        } else if self.blockers.is_empty() {
            PromotionStatus::PromotionReady
        } else {
            PromotionStatus::StagedBlocked
        };
        self.promoted = self.status.can_promote() && config.production_enabled;
        self.command_hint =
            CommandHint::new(command_for_attempt(self), self.slot_kind, &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        merkle_root(
            "WAVE95-WALLET-WATCHTOWER-PROMOTION-BLOCKERS",
            &self
                .blockers
                .iter()
                .map(|blocker| Value::String(blocker.as_str().to_string()))
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "wave92_slot_occupancy_placeholder_root": self.wave92_slot_occupancy_placeholder_root,
            "evidence_root": merkle_root("WAVE95-WALLET-WATCHTOWER-EVIDENCE-ROOTS", &self.evidence_roots.iter().cloned().map(Value::String).collect::<Vec<_>>()),
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_claim_placeholder_root": self.slot_occupancy_claim_placeholder_root,
            "blocker_root": self.blocker_root(),
            "command_hint": self.command_hint.public_record(),
            "heavy_gate_claimed": self.heavy_gate_claimed,
            "promoted": self.promoted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion-attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionSummary {
    pub fail_closed: bool,
    pub production_denied: bool,
    pub promoted_slots: u64,
    pub promotion_attempts: u64,
    pub blocked_slots: u64,
    pub denied_slots: u64,
    pub blocker_root: String,
    pub command_root: String,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_placeholder_root: String,
    pub promotion_attempt_root: String,
    pub heavy_gate_claimed: bool,
}

impl PromotionSummary {
    pub fn from_attempts(attempts: &BTreeMap<String, PromotionAttempt>) -> Self {
        let promoted_slots = attempts.values().filter(|attempt| attempt.promoted).count() as u64;
        let promotion_attempts = attempts
            .values()
            .filter(|attempt| !is_empty_root(&attempt.wave94_staged_fill_root))
            .count() as u64;
        let blocked_slots = attempts
            .values()
            .filter(|attempt| {
                matches!(
                    attempt.status,
                    PromotionStatus::EmptyBlocked | PromotionStatus::StagedBlocked
                )
            })
            .count() as u64;
        let denied_slots = attempts
            .values()
            .filter(|attempt| matches!(attempt.status, PromotionStatus::Denied))
            .count() as u64;
        let heavy_gate_claimed = attempts.values().any(|attempt| attempt.heavy_gate_claimed);
        let blocker_root = merkle_root(
            "WAVE95-WALLET-WATCHTOWER-ALL-BLOCKERS",
            &attempts
                .values()
                .map(|attempt| Value::String(attempt.blocker_root()))
                .collect::<Vec<_>>(),
        );
        let command_root = merkle_root(
            "WAVE95-WALLET-WATCHTOWER-COMMAND-HINTS",
            &attempts
                .values()
                .map(|attempt| attempt.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let staged_fill_binding_root = merkle_root(
            "WAVE95-WALLET-WATCHTOWER-STAGED-FILL-BINDINGS",
            &attempts
                .values()
                .map(|attempt| Value::String(attempt.staged_fill_binding_root.clone()))
                .collect::<Vec<_>>(),
        );
        let slot_occupancy_placeholder_root = merkle_root(
            "WAVE95-WALLET-WATCHTOWER-SLOT-OCCUPANCY-PLACEHOLDERS",
            &attempts
                .values()
                .map(|attempt| Value::String(attempt.slot_occupancy_claim_placeholder_root.clone()))
                .collect::<Vec<_>>(),
        );
        let promotion_attempt_root = merkle_root(
            "WAVE95-WALLET-WATCHTOWER-PROMOTION-ATTEMPTS",
            &attempts
                .values()
                .map(|attempt| Value::String(attempt.state_root()))
                .collect::<Vec<_>>(),
        );
        Self {
            fail_closed: promoted_slots == 0 || blocked_slots > 0 || denied_slots > 0,
            production_denied: true,
            promoted_slots,
            promotion_attempts,
            blocked_slots,
            denied_slots,
            blocker_root,
            command_root,
            staged_fill_binding_root,
            slot_occupancy_placeholder_root,
            promotion_attempt_root,
            heavy_gate_claimed,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "production_denied": self.production_denied,
            "promoted_slots": self.promoted_slots,
            "promotion_attempts": self.promotion_attempts,
            "blocked_slots": self.blocked_slots,
            "denied_slots": self.denied_slots,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_placeholder_root": self.slot_occupancy_placeholder_root,
            "promotion_attempt_root": self.promotion_attempt_root,
            "heavy_gate_claimed": self.heavy_gate_claimed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promotion_attempts: BTreeMap<String, PromotionAttempt>,
    pub summary: PromotionSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let promotion_attempts = PromotionSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let attempt = PromotionAttempt::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), attempt)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = PromotionSummary::from_attempts(&promotion_attempts);
        Self {
            config,
            promotion_attempts,
            summary,
        }
    }

    pub fn stage_promotion_attempt(
        mut self,
        slot_kind: PromotionSlotKind,
        wave94_staged_fill_root: impl Into<String>,
        wave92_slot_occupancy_placeholder_root: impl Into<String>,
        evidence_roots: Vec<String>,
    ) -> Result<Self> {
        let attempt = PromotionAttempt::from_roots(
            slot_kind,
            wave94_staged_fill_root,
            wave92_slot_occupancy_placeholder_root,
            evidence_roots,
            false,
            &self.config,
        );
        self.promotion_attempts
            .insert(slot_kind.as_str().to_string(), attempt);
        self.recompute();
        Ok(self)
    }

    pub fn deny_heavy_gate_claim(mut self, slot_kind: PromotionSlotKind) -> Result<Self> {
        let key = slot_kind.as_str().to_string();
        match self.promotion_attempts.remove(&key) {
            Some(mut attempt) => {
                attempt.heavy_gate_claimed = true;
                attempt.recompute(&self.config);
                self.promotion_attempts.insert(key, attempt);
                self.recompute();
                Ok(self)
            }
            None => Err(format!("promotion slot missing: {}", slot_kind.as_str())),
        }
    }

    pub fn recompute(&mut self) {
        for attempt in self.promotion_attempts.values_mut() {
            attempt.recompute(&self.config);
        }
        self.summary = PromotionSummary::from_attempts(&self.promotion_attempts);
    }

    pub fn promotion_attempt_roots(&self) -> BTreeMap<String, String> {
        self.promotion_attempts
            .iter()
            .map(|(key, attempt)| (key.clone(), attempt.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn promotion_attempts_root(&self) -> String {
        merkle_root(
            "WAVE95-WALLET-WATCHTOWER-PROMOTION-ATTEMPT-ROOTS",
            &self
                .promotion_attempt_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "promotion_attempts": self.promotion_attempts.iter().map(|(key, attempt)| (key.clone(), attempt.public_record())).collect::<BTreeMap<_, _>>(),
            "promotion_attempt_roots": self.promotion_attempt_roots(),
            "promotion_attempts_root": self.promotion_attempts_root(),
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

pub fn slot_promotion_runtime() -> Runtime {
    devnet()
}

pub fn staged_fill_binding_root(
    slot_kind: PromotionSlotKind,
    wave94_staged_fill_root: &str,
    wave92_slot_occupancy_placeholder_root: &str,
    evidence_roots: &[String],
) -> String {
    record_root(
        "staged-fill-binding-root",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "source_wave_label": SOURCE_WAVE_LABEL,
            "target_wave_label": TARGET_WAVE_LABEL,
            "wave94_staged_fill_root": wave94_staged_fill_root,
            "wave92_slot_occupancy_placeholder_root": wave92_slot_occupancy_placeholder_root,
            "evidence_roots_root": merkle_root("WAVE95-WALLET-WATCHTOWER-STAGED-FILL-EVIDENCE", &evidence_roots.iter().cloned().map(Value::String).collect::<Vec<_>>()),
            "roots_only": true,
            "wallet_material_absent": true,
            "heavy_gate_claimed": false,
        }),
    )
}

pub fn slot_occupancy_claim_placeholder_root(
    slot_kind: PromotionSlotKind,
    wave94_staged_fill_root: &str,
    wave92_slot_occupancy_placeholder_root: &str,
) -> String {
    record_root(
        "slot-occupancy-claim-placeholder-root",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "wave94_staged_fill_root": wave94_staged_fill_root,
            "wave92_slot_occupancy_placeholder_root": wave92_slot_occupancy_placeholder_root,
            "claim_body_redacted": true,
            "promotion_claim_not_submitted": true,
        }),
    )
}

fn promotion_blockers(attempt: &PromotionAttempt, config: &Config) -> Vec<PromotionBlocker> {
    let mut blockers = Vec::new();
    if config.fail_closed_on_empty_promotion && is_empty_root(&attempt.wave94_staged_fill_root) {
        blockers.push(PromotionBlocker::EmptyStagedFill);
    }
    if config.require_wave94_staged_fill_binding && is_empty_root(&attempt.wave94_staged_fill_root)
    {
        blockers.push(PromotionBlocker::Wave94StagedFillRootMissing);
    }
    if config.require_wave92_slot_occupancy_placeholder
        && is_empty_root(&attempt.wave92_slot_occupancy_placeholder_root)
    {
        blockers.push(PromotionBlocker::Wave92SlotOccupancyPlaceholderMissing);
    }
    if attempt.evidence_roots.is_empty() {
        blockers.push(PromotionBlocker::EvidenceRootMissing);
    }
    if !is_root_like(&attempt.wave94_staged_fill_root)
        || !is_root_like(&attempt.wave92_slot_occupancy_placeholder_root)
        || !attempt.evidence_roots.iter().all(|root| is_root_like(root))
    {
        blockers.push(PromotionBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(attempt) {
        blockers.push(PromotionBlocker::RootsOnlyRecordMissing);
    }
    if config.require_wallet_escape_dry_run
        && attempt.slot_kind == PromotionSlotKind::WalletEscapeDryRun
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::WalletEscapeDryRunMissing);
    }
    if config.require_watchtower_quorum
        && attempt.slot_kind == PromotionSlotKind::WatchtowerQuorum
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::WatchtowerQuorumMissing);
    }
    if config.require_user_runbook_replay
        && attempt.slot_kind == PromotionSlotKind::UserRunbookReplay
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::UserRunbookReplayMissing);
    }
    if config.require_redacted_recovery_proof
        && attempt.slot_kind == PromotionSlotKind::RedactedRecoveryProof
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::RedactedRecoveryProofMissing);
    }
    if config.require_wallet_visible_receipt
        && attempt.slot_kind == PromotionSlotKind::WalletVisibleReceipt
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::WalletVisibleReceiptMissing);
    }
    if config.require_operator_signoff
        && attempt.slot_kind == PromotionSlotKind::OperatorSignoff
        && attempt.evidence_roots.len() < config.min_roots_for_slot(attempt.slot_kind) as usize
    {
        blockers.push(PromotionBlocker::OperatorSignoffMissing);
    }
    if !config.production_enabled {
        blockers.push(PromotionBlocker::ProductionDenied);
    }
    if attempt.heavy_gate_claimed || config.heavy_gate_execution_allowed {
        blockers.push(PromotionBlocker::HeavyGateClaimPresent);
    }
    blockers
}

fn roots_only_record_present(attempt: &PromotionAttempt) -> bool {
    is_root_like(&attempt.staged_fill_binding_root)
        && is_root_like(&attempt.slot_occupancy_claim_placeholder_root)
}

fn command_for_attempt(attempt: &PromotionAttempt) -> CommandHintKind {
    if attempt.blockers.is_empty() {
        return CommandHintKind::PromoteToSlotOccupancyClaim;
    }
    match attempt.blockers[0] {
        PromotionBlocker::EmptyStagedFill | PromotionBlocker::Wave94StagedFillRootMissing => {
            CommandHintKind::StageWave94FillRoot
        }
        PromotionBlocker::Wave92SlotOccupancyPlaceholderMissing => {
            CommandHintKind::BindWave92SlotOccupancyPlaceholder
        }
        PromotionBlocker::WalletEscapeDryRunMissing => {
            CommandHintKind::RequestWalletEscapeDryRunRoot
        }
        PromotionBlocker::WatchtowerQuorumMissing => CommandHintKind::RequestWatchtowerQuorumRoot,
        PromotionBlocker::UserRunbookReplayMissing => CommandHintKind::RequestUserRunbookReplayRoot,
        PromotionBlocker::RedactedRecoveryProofMissing => {
            CommandHintKind::RequestRedactedRecoveryProofRoot
        }
        PromotionBlocker::WalletVisibleReceiptMissing => {
            CommandHintKind::RequestWalletVisibleReceiptRoot
        }
        PromotionBlocker::OperatorSignoffMissing => CommandHintKind::RequestOperatorSignoffRoot,
        PromotionBlocker::EvidenceRootMissing
        | PromotionBlocker::RootShapeInvalid
        | PromotionBlocker::RootsOnlyRecordMissing
        | PromotionBlocker::ProductionDenied
        | PromotionBlocker::HeavyGateClaimPresent => CommandHintKind::HoldBlockedPromotion,
    }
}

fn empty_root(marker_name: &str) -> String {
    let root = record_root(
        "empty-slot-promotion-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "marker_name": marker_name,
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
        "WAVE95-WALLET-WATCHTOWER-RECEIPT-SLOT-PROMOTION",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
