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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave92-live-heavy-gate-receipt-slot-registry-wallet-watchtower-slot-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave92";
pub const SOURCE_WAVE_LABEL: &str = "wave91";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-execution-plan-wallet-watchtower-receipt-lane";
pub const REGISTRY_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-registry-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-live-heavy-gate-receipt-slot-root";
pub const DEFAULT_MIN_PLAN_ROOTS: u64 = 6;
pub const DEFAULT_MIN_RECEIPT_ROOTS_PER_SLOT: u64 = 1;
pub const DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS: u64 = 4;
pub const DEFAULT_MIN_REPLAY_ROOTS: u64 = 6;
pub const DEFAULT_MIN_RECOVERY_ROOTS: u64 = 2;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: u64 = 3;
pub const DEFAULT_CLEARANCE_EPOCH: u64 = 92;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub source_lane: String,
    pub registry_lane: String,
    pub empty_root_marker: String,
    pub min_plan_roots: u64,
    pub min_receipt_roots_per_slot: u64,
    pub min_watchtower_quorum_roots: u64,
    pub min_replay_roots: u64,
    pub min_recovery_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub clearance_epoch: u64,
    pub require_plan_root_intake: bool,
    pub require_wallet_escape_dry_run_slot: bool,
    pub require_watchtower_quorum_slot: bool,
    pub require_user_runbook_replay_slot: bool,
    pub require_redacted_recovery_proof_slot: bool,
    pub require_wallet_visible_receipt_slot: bool,
    pub require_operator_signoff_slot: bool,
    pub require_roots_only_privacy: bool,
    pub require_live_accepted_roots: bool,
    pub fail_closed_on_empty_slot: bool,
    pub fail_closed_on_pending_import: bool,
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
            source_lane: SOURCE_LANE.to_string(),
            registry_lane: REGISTRY_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            min_plan_roots: DEFAULT_MIN_PLAN_ROOTS,
            min_receipt_roots_per_slot: DEFAULT_MIN_RECEIPT_ROOTS_PER_SLOT,
            min_watchtower_quorum_roots: DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS,
            min_replay_roots: DEFAULT_MIN_REPLAY_ROOTS,
            min_recovery_roots: DEFAULT_MIN_RECOVERY_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            clearance_epoch: DEFAULT_CLEARANCE_EPOCH,
            require_plan_root_intake: true,
            require_wallet_escape_dry_run_slot: true,
            require_watchtower_quorum_slot: true,
            require_user_runbook_replay_slot: true,
            require_redacted_recovery_proof_slot: true,
            require_wallet_visible_receipt_slot: true,
            require_operator_signoff_slot: true,
            require_roots_only_privacy: true,
            require_live_accepted_roots: true,
            fail_closed_on_empty_slot: true,
            fail_closed_on_pending_import: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_slot_count(&self) -> u64 {
        [
            self.require_wallet_escape_dry_run_slot,
            self.require_watchtower_quorum_slot,
            self.require_user_runbook_replay_slot,
            self.require_redacted_recovery_proof_slot,
            self.require_wallet_visible_receipt_slot,
            self.require_operator_signoff_slot,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn min_roots_for_slot(&self, slot: ReceiptSlotKind) -> u64 {
        match slot {
            ReceiptSlotKind::WalletEscapeDryRun => self.min_receipt_roots_per_slot,
            ReceiptSlotKind::WatchtowerQuorum => self.min_watchtower_quorum_roots,
            ReceiptSlotKind::UserRunbookReplay => self.min_replay_roots,
            ReceiptSlotKind::RedactedRecoveryProof => self.min_recovery_roots,
            ReceiptSlotKind::WalletVisibleReceipt => self.min_receipt_roots_per_slot,
            ReceiptSlotKind::OperatorSignoff => self.min_operator_signoff_roots,
        }
    }

    pub fn slot_required(&self, slot: ReceiptSlotKind) -> bool {
        match slot {
            ReceiptSlotKind::WalletEscapeDryRun => self.require_wallet_escape_dry_run_slot,
            ReceiptSlotKind::WatchtowerQuorum => self.require_watchtower_quorum_slot,
            ReceiptSlotKind::UserRunbookReplay => self.require_user_runbook_replay_slot,
            ReceiptSlotKind::RedactedRecoveryProof => self.require_redacted_recovery_proof_slot,
            ReceiptSlotKind::WalletVisibleReceipt => self.require_wallet_visible_receipt_slot,
            ReceiptSlotKind::OperatorSignoff => self.require_operator_signoff_slot,
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
            "source_lane": self.source_lane,
            "registry_lane": self.registry_lane,
            "empty_root_marker": self.empty_root_marker,
            "min_plan_roots": self.min_plan_roots,
            "min_receipt_roots_per_slot": self.min_receipt_roots_per_slot,
            "min_watchtower_quorum_roots": self.min_watchtower_quorum_roots,
            "min_replay_roots": self.min_replay_roots,
            "min_recovery_roots": self.min_recovery_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "clearance_epoch": self.clearance_epoch,
            "require_plan_root_intake": self.require_plan_root_intake,
            "require_wallet_escape_dry_run_slot": self.require_wallet_escape_dry_run_slot,
            "require_watchtower_quorum_slot": self.require_watchtower_quorum_slot,
            "require_user_runbook_replay_slot": self.require_user_runbook_replay_slot,
            "require_redacted_recovery_proof_slot": self.require_redacted_recovery_proof_slot,
            "require_wallet_visible_receipt_slot": self.require_wallet_visible_receipt_slot,
            "require_operator_signoff_slot": self.require_operator_signoff_slot,
            "require_roots_only_privacy": self.require_roots_only_privacy,
            "require_live_accepted_roots": self.require_live_accepted_roots,
            "fail_closed_on_empty_slot": self.fail_closed_on_empty_slot,
            "fail_closed_on_pending_import": self.fail_closed_on_pending_import,
            "required_slot_count": self.required_slot_count(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptSlotKind {
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl ReceiptSlotKind {
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

    pub fn ordinal(self) -> u64 {
        match self {
            Self::WalletEscapeDryRun => 0,
            Self::WatchtowerQuorum => 1,
            Self::UserRunbookReplay => 2,
            Self::RedactedRecoveryProof => 3,
            Self::WalletVisibleReceipt => 4,
            Self::OperatorSignoff => 5,
        }
    }

    pub fn privacy_mode(self) -> PrivacyMode {
        match self {
            Self::WalletEscapeDryRun => PrivacyMode::WalletRootOnly,
            Self::WatchtowerQuorum => PrivacyMode::QuorumRootOnly,
            Self::UserRunbookReplay => PrivacyMode::RunbookRootOnly,
            Self::RedactedRecoveryProof => PrivacyMode::RedactedProofRootOnly,
            Self::WalletVisibleReceipt => PrivacyMode::WalletVisibleRootOnly,
            Self::OperatorSignoff => PrivacyMode::OperatorRootOnly,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyMode {
    WalletRootOnly,
    QuorumRootOnly,
    RunbookRootOnly,
    RedactedProofRootOnly,
    WalletVisibleRootOnly,
    OperatorRootOnly,
}

impl PrivacyMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletRootOnly => "wallet_root_only",
            Self::QuorumRootOnly => "quorum_root_only",
            Self::RunbookRootOnly => "runbook_root_only",
            Self::RedactedProofRootOnly => "redacted_proof_root_only",
            Self::WalletVisibleRootOnly => "wallet_visible_root_only",
            Self::OperatorRootOnly => "operator_root_only",
        }
    }

    pub fn is_roots_only(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotStatus {
    Empty,
    PendingImport,
    LiveAccepted,
    Clearable,
    Blocked,
}

impl SlotStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::PendingImport => "pending_import",
            Self::LiveAccepted => "live_accepted",
            Self::Clearable => "clearable",
            Self::Blocked => "blocked",
        }
    }

    pub fn can_clear(self) -> bool {
        matches!(self, Self::Clearable)
    }

    pub fn fail_closed(self) -> bool {
        matches!(self, Self::Empty | Self::PendingImport | Self::Blocked)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptedReceiptStatus {
    Placeholder,
    Attached,
    LiveAccepted,
    RejectedByRule,
}

impl AcceptedReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Placeholder => "placeholder",
            Self::Attached => "attached",
            Self::LiveAccepted => "live_accepted",
            Self::RejectedByRule => "rejected_by_rule",
        }
    }

    pub fn is_live(self) -> bool {
        matches!(self, Self::LiveAccepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanIntakeStatus {
    Missing,
    Imported,
    BoundToRegistry,
}

impl PlanIntakeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Imported => "imported",
            Self::BoundToRegistry => "bound_to_registry",
        }
    }

    pub fn clears(self) -> bool {
        matches!(self, Self::BoundToRegistry)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletSlotBlocker {
    PlanRootMissing,
    SlotRootMissing,
    ReceiptRootNotLiveAccepted,
    WatchtowerQuorumRootMissing,
    UserReplayRootMissing,
    RecoveryProofRootMissing,
    WalletVisibleRootMissing,
    OperatorSignoffRootMissing,
    PrivacyModeNotRootsOnly,
    ImportRuleNotSatisfied,
    SlotStillEmpty,
}

impl WalletSlotBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlanRootMissing => "plan_root_missing",
            Self::SlotRootMissing => "slot_root_missing",
            Self::ReceiptRootNotLiveAccepted => "receipt_root_not_live_accepted",
            Self::WatchtowerQuorumRootMissing => "watchtower_quorum_root_missing",
            Self::UserReplayRootMissing => "user_replay_root_missing",
            Self::RecoveryProofRootMissing => "recovery_proof_root_missing",
            Self::WalletVisibleRootMissing => "wallet_visible_root_missing",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::PrivacyModeNotRootsOnly => "privacy_mode_not_roots_only",
            Self::ImportRuleNotSatisfied => "import_rule_not_satisfied",
            Self::SlotStillEmpty => "slot_still_empty",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportRuleKind {
    RootShape,
    SourceWaveBinding,
    LiveAcceptance,
    SlotKindBinding,
    RootsOnlyPrivacy,
    MinimumRootCount,
}

impl ImportRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootShape => "root_shape",
            Self::SourceWaveBinding => "source_wave_binding",
            Self::LiveAcceptance => "live_acceptance",
            Self::SlotKindBinding => "slot_kind_binding",
            Self::RootsOnlyPrivacy => "roots_only_privacy",
            Self::MinimumRootCount => "minimum_root_count",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportRuleVerdict {
    Pending,
    Satisfied,
    Denied,
}

impl ImportRuleVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Satisfied => "satisfied",
            Self::Denied => "denied",
        }
    }

    pub fn clears(self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommandKind {
    Hold,
    AttachLiveReceiptRoot,
    RecheckWatchtowerQuorum,
    ReplayUserRunbook,
    PublishWalletVisibleRoot,
    RequestOperatorSignoff,
    ClearSlot,
}

impl OperatorCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hold => "hold",
            Self::AttachLiveReceiptRoot => "attach_live_receipt_root",
            Self::RecheckWatchtowerQuorum => "recheck_watchtower_quorum",
            Self::ReplayUserRunbook => "replay_user_runbook",
            Self::PublishWalletVisibleRoot => "publish_wallet_visible_root",
            Self::RequestOperatorSignoff => "request_operator_signoff",
            Self::ClearSlot => "clear_slot",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryDecision {
    FailClosed,
    WaitingForLiveRoots,
    Clearable,
}

impl RegistryDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::WaitingForLiveRoots => "waiting_for_live_roots",
            Self::Clearable => "clearable",
        }
    }

    pub fn can_clear(self) -> bool {
        matches!(self, Self::Clearable)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanRootIntake {
    pub source_wave_label: String,
    pub source_lane: String,
    pub execution_plan_root: String,
    pub denial_root: String,
    pub wallet_watchtower_plan_root: String,
    pub action_root: String,
    pub blocker_root: String,
    pub operator_hint_root: String,
    pub status: PlanIntakeStatus,
}

impl PlanRootIntake {
    pub fn empty(config: &Config) -> Self {
        Self {
            source_wave_label: config.source_wave_label.clone(),
            source_lane: config.source_lane.clone(),
            execution_plan_root: empty_root("wave91-execution-plan-root"),
            denial_root: empty_root("wave91-denial-root"),
            wallet_watchtower_plan_root: empty_root("wave91-wallet-watchtower-plan-root"),
            action_root: empty_root("wave91-action-root"),
            blocker_root: empty_root("wave91-blocker-root"),
            operator_hint_root: empty_root("wave91-operator-hint-root"),
            status: PlanIntakeStatus::Missing,
        }
    }

    pub fn imported(
        config: &Config,
        execution_plan_root: impl Into<String>,
        denial_root: impl Into<String>,
        wallet_watchtower_plan_root: impl Into<String>,
        action_root: impl Into<String>,
        blocker_root: impl Into<String>,
        operator_hint_root: impl Into<String>,
    ) -> Self {
        Self {
            source_wave_label: config.source_wave_label.clone(),
            source_lane: config.source_lane.clone(),
            execution_plan_root: execution_plan_root.into(),
            denial_root: denial_root.into(),
            wallet_watchtower_plan_root: wallet_watchtower_plan_root.into(),
            action_root: action_root.into(),
            blocker_root: blocker_root.into(),
            operator_hint_root: operator_hint_root.into(),
            status: PlanIntakeStatus::Imported,
        }
    }

    pub fn bind_to_registry(mut self) -> Self {
        self.status = PlanIntakeStatus::BoundToRegistry;
        self
    }

    pub fn roots(&self) -> Vec<String> {
        vec![
            self.execution_plan_root.clone(),
            self.denial_root.clone(),
            self.wallet_watchtower_plan_root.clone(),
            self.action_root.clone(),
            self.blocker_root.clone(),
            self.operator_hint_root.clone(),
        ]
    }

    pub fn live_root_count(&self) -> u64 {
        self.roots()
            .iter()
            .filter(|root| !is_empty_root(root))
            .count() as u64
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "source_wave_label": self.source_wave_label,
            "source_lane": self.source_lane,
            "execution_plan_root": self.execution_plan_root,
            "denial_root": self.denial_root,
            "wallet_watchtower_plan_root": self.wallet_watchtower_plan_root,
            "action_root": self.action_root,
            "blocker_root": self.blocker_root,
            "operator_hint_root": self.operator_hint_root,
            "status": self.status.as_str(),
            "live_root_count": self.live_root_count(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("plan-root-intake", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedReceiptPlaceholder {
    pub slot_kind: ReceiptSlotKind,
    pub receipt_root: String,
    pub accepted_root: String,
    pub attestation_root: String,
    pub live_epoch: u64,
    pub status: AcceptedReceiptStatus,
}

impl AcceptedReceiptPlaceholder {
    pub fn empty(slot_kind: ReceiptSlotKind) -> Self {
        Self {
            slot_kind,
            receipt_root: empty_root(slot_kind.as_str()),
            accepted_root: empty_root("accepted-root"),
            attestation_root: empty_root("attestation-root"),
            live_epoch: 0,
            status: AcceptedReceiptStatus::Placeholder,
        }
    }

    pub fn attached(
        slot_kind: ReceiptSlotKind,
        receipt_root: impl Into<String>,
        accepted_root: impl Into<String>,
        attestation_root: impl Into<String>,
        live_epoch: u64,
    ) -> Self {
        Self {
            slot_kind,
            receipt_root: receipt_root.into(),
            accepted_root: accepted_root.into(),
            attestation_root: attestation_root.into(),
            live_epoch,
            status: AcceptedReceiptStatus::Attached,
        }
    }

    pub fn live_accept(mut self) -> Self {
        self.status = AcceptedReceiptStatus::LiveAccepted;
        self
    }

    pub fn is_empty(&self) -> bool {
        is_empty_root(&self.receipt_root)
            || is_empty_root(&self.accepted_root)
            || is_empty_root(&self.attestation_root)
    }

    pub fn is_live_accepted(&self) -> bool {
        self.status.is_live() && !self.is_empty()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "receipt_root": self.receipt_root,
            "accepted_root": self.accepted_root,
            "attestation_root": self.attestation_root,
            "live_epoch": self.live_epoch,
            "status": self.status.as_str(),
            "is_live_accepted": self.is_live_accepted(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("accepted-receipt-placeholder", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportRule {
    pub rule_kind: ImportRuleKind,
    pub slot_kind: ReceiptSlotKind,
    pub rule_root: String,
    pub verdict: ImportRuleVerdict,
    pub blocker: Option<WalletSlotBlocker>,
}

impl ImportRule {
    pub fn pending(rule_kind: ImportRuleKind, slot_kind: ReceiptSlotKind) -> Self {
        Self {
            rule_kind,
            slot_kind,
            rule_root: empty_root(rule_kind.as_str()),
            verdict: ImportRuleVerdict::Pending,
            blocker: Some(WalletSlotBlocker::ImportRuleNotSatisfied),
        }
    }

    pub fn satisfied(
        rule_kind: ImportRuleKind,
        slot_kind: ReceiptSlotKind,
        rule_root: String,
    ) -> Self {
        Self {
            rule_kind,
            slot_kind,
            rule_root,
            verdict: ImportRuleVerdict::Satisfied,
            blocker: None,
        }
    }

    pub fn denied(
        rule_kind: ImportRuleKind,
        slot_kind: ReceiptSlotKind,
        rule_root: String,
        blocker: WalletSlotBlocker,
    ) -> Self {
        Self {
            rule_kind,
            slot_kind,
            rule_root,
            verdict: ImportRuleVerdict::Denied,
            blocker: Some(blocker),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "rule_kind": self.rule_kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "rule_root": self.rule_root,
            "verdict": self.verdict.as_str(),
            "blocker": self.blocker.map(WalletSlotBlocker::as_str),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("import-rule", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommandHint {
    pub slot_kind: ReceiptSlotKind,
    pub command_kind: OperatorCommandKind,
    pub command_root: String,
    pub hold_reason_root: String,
    pub can_clear_after_live_root: bool,
}

impl OperatorCommandHint {
    pub fn hold(slot_kind: ReceiptSlotKind, blocker: WalletSlotBlocker) -> Self {
        Self {
            slot_kind,
            command_kind: OperatorCommandKind::Hold,
            command_root: record_root(
                "operator-command-hold",
                &json!({"slot_kind": slot_kind.as_str(), "blocker": blocker.as_str()}),
            ),
            hold_reason_root: record_root(
                "operator-hold-reason",
                &json!({"slot_kind": slot_kind.as_str(), "blocker": blocker.as_str()}),
            ),
            can_clear_after_live_root: false,
        }
    }

    pub fn attach_live_root(slot_kind: ReceiptSlotKind) -> Self {
        Self {
            slot_kind,
            command_kind: OperatorCommandKind::AttachLiveReceiptRoot,
            command_root: record_root(
                "operator-command-attach-live-root",
                &json!({"slot_kind": slot_kind.as_str()}),
            ),
            hold_reason_root: empty_root("no-hold-reason"),
            can_clear_after_live_root: true,
        }
    }

    pub fn clear(slot_kind: ReceiptSlotKind) -> Self {
        Self {
            slot_kind,
            command_kind: OperatorCommandKind::ClearSlot,
            command_root: record_root(
                "operator-command-clear-slot",
                &json!({"slot_kind": slot_kind.as_str()}),
            ),
            hold_reason_root: empty_root("clearable"),
            can_clear_after_live_root: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "command_kind": self.command_kind.as_str(),
            "command_root": self.command_root,
            "hold_reason_root": self.hold_reason_root,
            "can_clear_after_live_root": self.can_clear_after_live_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator-command-hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptSlot {
    pub slot_kind: ReceiptSlotKind,
    pub slot_index: u64,
    pub privacy_mode: PrivacyMode,
    pub min_live_roots: u64,
    pub receipt_roots: Vec<String>,
    pub accepted_receipt: AcceptedReceiptPlaceholder,
    pub import_rules: Vec<ImportRule>,
    pub blockers: Vec<WalletSlotBlocker>,
    pub operator_hint: OperatorCommandHint,
    pub status: SlotStatus,
}

impl ReceiptSlot {
    pub fn empty(slot_kind: ReceiptSlotKind, config: &Config) -> Self {
        let blocker = blocker_for_slot(slot_kind);
        let import_rules = vec![
            ImportRule::pending(ImportRuleKind::RootShape, slot_kind),
            ImportRule::pending(ImportRuleKind::SourceWaveBinding, slot_kind),
            ImportRule::pending(ImportRuleKind::LiveAcceptance, slot_kind),
            ImportRule::pending(ImportRuleKind::SlotKindBinding, slot_kind),
            ImportRule::pending(ImportRuleKind::RootsOnlyPrivacy, slot_kind),
            ImportRule::pending(ImportRuleKind::MinimumRootCount, slot_kind),
        ];
        Self {
            slot_kind,
            slot_index: slot_kind.ordinal(),
            privacy_mode: slot_kind.privacy_mode(),
            min_live_roots: config.min_roots_for_slot(slot_kind),
            receipt_roots: Vec::new(),
            accepted_receipt: AcceptedReceiptPlaceholder::empty(slot_kind),
            import_rules,
            blockers: vec![blocker, WalletSlotBlocker::SlotStillEmpty],
            operator_hint: OperatorCommandHint::hold(slot_kind, blocker),
            status: SlotStatus::Empty,
        }
    }

    pub fn attach_live_receipt(
        mut self,
        receipt: AcceptedReceiptPlaceholder,
        extra_roots: Vec<String>,
    ) -> Self {
        let mut receipt_roots = extra_roots
            .into_iter()
            .filter(|root| !is_empty_root(root))
            .collect::<Vec<_>>();
        if !is_empty_root(&receipt.receipt_root) {
            receipt_roots.push(receipt.receipt_root.clone());
        }
        if !is_empty_root(&receipt.accepted_root) {
            receipt_roots.push(receipt.accepted_root.clone());
        }
        if !is_empty_root(&receipt.attestation_root) {
            receipt_roots.push(receipt.attestation_root.clone());
        }
        receipt_roots.sort();
        receipt_roots.dedup();
        self.receipt_roots = receipt_roots;
        self.accepted_receipt = receipt;
        self.recompute_import_rules();
        self.recompute_status();
        self
    }

    pub fn recompute_import_rules(&mut self) {
        let mut rules = Vec::new();
        let root_shape = if self.receipt_roots.iter().all(|root| is_root_like(root)) {
            ImportRule::satisfied(
                ImportRuleKind::RootShape,
                self.slot_kind,
                record_root("root-shape-rule", &json!(self.receipt_roots)),
            )
        } else {
            ImportRule::denied(
                ImportRuleKind::RootShape,
                self.slot_kind,
                record_root(
                    "root-shape-rule-denied",
                    &json!({"slot": self.slot_kind.as_str()}),
                ),
                WalletSlotBlocker::SlotRootMissing,
            )
        };
        rules.push(root_shape);
        rules.push(ImportRule::satisfied(
            ImportRuleKind::SourceWaveBinding,
            self.slot_kind,
            record_root(
                "source-wave-binding-rule",
                &json!({"source_wave_label": SOURCE_WAVE_LABEL, "slot": self.slot_kind.as_str()}),
            ),
        ));
        let live_rule = if self.accepted_receipt.is_live_accepted() {
            ImportRule::satisfied(
                ImportRuleKind::LiveAcceptance,
                self.slot_kind,
                self.accepted_receipt.state_root(),
            )
        } else {
            ImportRule::denied(
                ImportRuleKind::LiveAcceptance,
                self.slot_kind,
                self.accepted_receipt.state_root(),
                WalletSlotBlocker::ReceiptRootNotLiveAccepted,
            )
        };
        rules.push(live_rule);
        rules.push(ImportRule::satisfied(
            ImportRuleKind::SlotKindBinding,
            self.slot_kind,
            record_root(
                "slot-kind-binding-rule",
                &json!({"slot": self.slot_kind.as_str()}),
            ),
        ));
        let privacy_rule = if self.privacy_mode.is_roots_only() {
            ImportRule::satisfied(
                ImportRuleKind::RootsOnlyPrivacy,
                self.slot_kind,
                record_root(
                    "roots-only-privacy-rule",
                    &json!({"mode": self.privacy_mode.as_str()}),
                ),
            )
        } else {
            ImportRule::denied(
                ImportRuleKind::RootsOnlyPrivacy,
                self.slot_kind,
                record_root(
                    "roots-only-privacy-rule-denied",
                    &json!({"slot": self.slot_kind.as_str()}),
                ),
                WalletSlotBlocker::PrivacyModeNotRootsOnly,
            )
        };
        rules.push(privacy_rule);
        let count_rule = if self.live_root_count() >= self.min_live_roots {
            ImportRule::satisfied(
                ImportRuleKind::MinimumRootCount,
                self.slot_kind,
                record_root(
                    "minimum-root-count-rule",
                    &json!({"slot": self.slot_kind.as_str(), "count": self.live_root_count()}),
                ),
            )
        } else {
            ImportRule::denied(
                ImportRuleKind::MinimumRootCount,
                self.slot_kind,
                record_root(
                    "minimum-root-count-rule-denied",
                    &json!({"slot": self.slot_kind.as_str(), "count": self.live_root_count()}),
                ),
                blocker_for_slot(self.slot_kind),
            )
        };
        rules.push(count_rule);
        self.import_rules = rules;
    }

    pub fn recompute_status(&mut self) {
        self.blockers = self.derive_blockers();
        if self.receipt_roots.is_empty() {
            self.status = SlotStatus::Empty;
            self.operator_hint =
                OperatorCommandHint::hold(self.slot_kind, WalletSlotBlocker::SlotStillEmpty);
        } else if self.blockers.is_empty() && self.accepted_receipt.is_live_accepted() {
            self.status = SlotStatus::Clearable;
            self.operator_hint = OperatorCommandHint::clear(self.slot_kind);
        } else if self.accepted_receipt.is_live_accepted() {
            self.status = SlotStatus::LiveAccepted;
            self.operator_hint = OperatorCommandHint::attach_live_root(self.slot_kind);
        } else {
            self.status = SlotStatus::PendingImport;
            self.operator_hint = OperatorCommandHint::attach_live_root(self.slot_kind);
        }
    }

    pub fn derive_blockers(&self) -> Vec<WalletSlotBlocker> {
        let mut blockers = Vec::new();
        if self.receipt_roots.is_empty() {
            blockers.push(WalletSlotBlocker::SlotStillEmpty);
        }
        if self.live_root_count() < self.min_live_roots {
            blockers.push(blocker_for_slot(self.slot_kind));
        }
        if !self.accepted_receipt.is_live_accepted() {
            blockers.push(WalletSlotBlocker::ReceiptRootNotLiveAccepted);
        }
        for rule in &self.import_rules {
            if !rule.verdict.clears() {
                if let Some(blocker) = rule.blocker {
                    blockers.push(blocker);
                }
            }
        }
        blockers.sort();
        blockers.dedup();
        blockers
    }

    pub fn live_root_count(&self) -> u64 {
        self.receipt_roots
            .iter()
            .filter(|root| !is_empty_root(root))
            .count() as u64
    }

    pub fn can_clear(&self) -> bool {
        self.status.can_clear()
            && self.accepted_receipt.is_live_accepted()
            && self.import_rules.iter().all(|rule| rule.verdict.clears())
            && self.live_root_count() >= self.min_live_roots
            && self.blockers.is_empty()
    }

    pub fn slot_root(&self) -> String {
        record_root("receipt-slot", &self.public_record_without_root())
    }

    pub fn public_record_without_root(&self) -> PublicRecord {
        let rule_records = self
            .import_rules
            .iter()
            .map(ImportRule::public_record)
            .collect::<Vec<_>>();
        let blocker_records = self
            .blockers
            .iter()
            .map(|blocker| blocker.as_str())
            .collect::<Vec<_>>();
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "slot_index": self.slot_index,
            "privacy_mode": self.privacy_mode.as_str(),
            "min_live_roots": self.min_live_roots,
            "receipt_roots": self.receipt_roots,
            "accepted_receipt": self.accepted_receipt.public_record(),
            "import_rules": rule_records,
            "blockers": blocker_records,
            "operator_hint": self.operator_hint.public_record(),
            "status": self.status.as_str(),
            "live_root_count": self.live_root_count(),
            "can_clear": self.can_clear(),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_root();
        if let Some(map) = record.as_object_mut() {
            map.insert("slot_root".to_string(), Value::String(self.slot_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        self.slot_root()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotRegistryVerdict {
    pub decision: RegistryDecision,
    pub fail_closed: bool,
    pub clearable_slot_count: u64,
    pub blocked_slot_count: u64,
    pub pending_slot_count: u64,
    pub blocker_root: String,
    pub command_root: String,
    pub notes_root: String,
}

impl SlotRegistryVerdict {
    pub fn from_slots(config: &Config, intake: &PlanRootIntake, slots: &[ReceiptSlot]) -> Self {
        let mut blockers = Vec::new();
        if config.require_plan_root_intake && !intake.status.clears() {
            blockers.push(WalletSlotBlocker::PlanRootMissing);
        }
        for slot in slots {
            if config.slot_required(slot.slot_kind) {
                blockers.extend(slot.blockers.clone());
            }
        }
        blockers.sort();
        blockers.dedup();
        let clearable_slot_count = slots.iter().filter(|slot| slot.can_clear()).count() as u64;
        let blocked_slot_count = slots
            .iter()
            .filter(|slot| slot.status.fail_closed())
            .count() as u64;
        let pending_slot_count = slots
            .iter()
            .filter(|slot| {
                matches!(
                    slot.status,
                    SlotStatus::PendingImport | SlotStatus::LiveAccepted
                )
            })
            .count() as u64;
        let required_slots_clear = clearable_slot_count >= config.required_slot_count();
        let plan_clear = !config.require_plan_root_intake || intake.status.clears();
        let decision = if required_slots_clear && plan_clear && blockers.is_empty() {
            RegistryDecision::Clearable
        } else if slots.iter().any(|slot| !slot.receipt_roots.is_empty()) {
            RegistryDecision::WaitingForLiveRoots
        } else {
            RegistryDecision::FailClosed
        };
        let fail_closed = !decision.can_clear();
        let blocker_root = record_root(
            "slot-registry-blockers",
            &json!(blockers
                .iter()
                .map(|blocker| blocker.as_str())
                .collect::<Vec<_>>()),
        );
        let command_root = record_root(
            "slot-registry-commands",
            &json!(slots
                .iter()
                .map(|slot| slot.operator_hint.public_record())
                .collect::<Vec<_>>()),
        );
        let notes_root = record_root(
            "slot-registry-notes",
            &json!({
                "roots_only": true,
                "raw_wallet_material_absent": true,
                "clearance_requires_live_accepted_receipt_roots": true,
            }),
        );
        Self {
            decision,
            fail_closed,
            clearable_slot_count,
            blocked_slot_count,
            pending_slot_count,
            blocker_root,
            command_root,
            notes_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "decision": self.decision.as_str(),
            "fail_closed": self.fail_closed,
            "clearable_slot_count": self.clearable_slot_count,
            "blocked_slot_count": self.blocked_slot_count,
            "pending_slot_count": self.pending_slot_count,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "notes_root": self.notes_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("slot-registry-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub plan_intake: PlanRootIntake,
    pub slots: BTreeMap<String, ReceiptSlot>,
    pub verdict: SlotRegistryVerdict,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let plan_intake = PlanRootIntake::empty(&config);
        let slots = ReceiptSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let slot = ReceiptSlot::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), slot)
            })
            .collect::<BTreeMap<_, _>>();
        let slot_values = slots.values().cloned().collect::<Vec<_>>();
        let verdict = SlotRegistryVerdict::from_slots(&config, &plan_intake, &slot_values);
        Self {
            config,
            plan_intake,
            slots,
            verdict,
        }
    }

    pub fn with_plan_intake(mut self, intake: PlanRootIntake) -> Self {
        self.plan_intake = intake;
        self.recompute();
        self
    }

    pub fn attach_receipt(
        mut self,
        slot_kind: ReceiptSlotKind,
        receipt: AcceptedReceiptPlaceholder,
        extra_roots: Vec<String>,
    ) -> Result<Self> {
        let key = slot_kind.as_str().to_string();
        match self.slots.remove(&key) {
            Some(slot) => {
                self.slots
                    .insert(key, slot.attach_live_receipt(receipt, extra_roots));
                self.recompute();
                Ok(self)
            }
            None => Err(format!("receipt slot missing: {}", slot_kind.as_str())),
        }
    }

    pub fn recompute(&mut self) {
        for slot in self.slots.values_mut() {
            slot.recompute_import_rules();
            slot.recompute_status();
        }
        let slot_values = self.slots.values().cloned().collect::<Vec<_>>();
        self.verdict =
            SlotRegistryVerdict::from_slots(&self.config, &self.plan_intake, &slot_values);
    }

    pub fn slot(&self, slot_kind: ReceiptSlotKind) -> Option<&ReceiptSlot> {
        self.slots.get(slot_kind.as_str())
    }

    pub fn clearable_slots(&self) -> Vec<ReceiptSlotKind> {
        self.slots
            .values()
            .filter(|slot| slot.can_clear())
            .map(|slot| slot.slot_kind)
            .collect::<Vec<_>>()
    }

    pub fn fail_closed_slots(&self) -> Vec<ReceiptSlotKind> {
        self.slots
            .values()
            .filter(|slot| !slot.can_clear())
            .map(|slot| slot.slot_kind)
            .collect::<Vec<_>>()
    }

    pub fn slot_roots(&self) -> BTreeMap<String, String> {
        self.slots
            .iter()
            .map(|(key, slot)| (key.clone(), slot.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn slots_root(&self) -> String {
        let roots = self
            .slot_roots()
            .values()
            .cloned()
            .map(Value::String)
            .collect::<Vec<_>>();
        merkle_root("WAVE92-WALLET-WATCHTOWER-SLOT-ROOTS", &roots)
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        let slot_records = self
            .slots
            .iter()
            .map(|(key, slot)| (key.clone(), slot.public_record()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "config": self.config.public_record(),
            "plan_intake": self.plan_intake.public_record(),
            "slots": slot_records,
            "slot_roots": self.slot_roots(),
            "slots_root": self.slots_root(),
            "verdict": self.verdict.public_record(),
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

pub fn canonical_slot_kinds() -> Vec<ReceiptSlotKind> {
    ReceiptSlotKind::all().to_vec()
}

pub fn empty_slot_registry() -> Runtime {
    devnet()
}

pub fn intake_wave91_plan_roots(
    execution_plan_root: impl Into<String>,
    denial_root: impl Into<String>,
    wallet_watchtower_plan_root: impl Into<String>,
    action_root: impl Into<String>,
    blocker_root: impl Into<String>,
    operator_hint_root: impl Into<String>,
) -> PlanRootIntake {
    PlanRootIntake::imported(
        &Config::default(),
        execution_plan_root,
        denial_root,
        wallet_watchtower_plan_root,
        action_root,
        blocker_root,
        operator_hint_root,
    )
    .bind_to_registry()
}

pub fn accepted_receipt_placeholder(slot_kind: ReceiptSlotKind) -> AcceptedReceiptPlaceholder {
    AcceptedReceiptPlaceholder::empty(slot_kind)
}

pub fn live_accepted_receipt(
    slot_kind: ReceiptSlotKind,
    receipt_root: impl Into<String>,
    accepted_root: impl Into<String>,
    attestation_root: impl Into<String>,
    live_epoch: u64,
) -> AcceptedReceiptPlaceholder {
    AcceptedReceiptPlaceholder::attached(
        slot_kind,
        receipt_root,
        accepted_root,
        attestation_root,
        live_epoch,
    )
    .live_accept()
}

pub fn fail_closed_slot_registry_verdict() -> SlotRegistryVerdict {
    devnet().verdict
}

fn blocker_for_slot(slot_kind: ReceiptSlotKind) -> WalletSlotBlocker {
    match slot_kind {
        ReceiptSlotKind::WalletEscapeDryRun => WalletSlotBlocker::SlotRootMissing,
        ReceiptSlotKind::WatchtowerQuorum => WalletSlotBlocker::WatchtowerQuorumRootMissing,
        ReceiptSlotKind::UserRunbookReplay => WalletSlotBlocker::UserReplayRootMissing,
        ReceiptSlotKind::RedactedRecoveryProof => WalletSlotBlocker::RecoveryProofRootMissing,
        ReceiptSlotKind::WalletVisibleReceipt => WalletSlotBlocker::WalletVisibleRootMissing,
        ReceiptSlotKind::OperatorSignoff => WalletSlotBlocker::OperatorSignoffRootMissing,
    }
}

fn empty_root(label: &str) -> String {
    let root = record_root(
        "empty-receipt-slot-root",
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
        "WAVE92-WALLET-WATCHTOWER-RECEIPT-SLOT-REGISTRY",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
