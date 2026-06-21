use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;
pub type MoneroL2PqBridgeExitForceExitWave92LiveHeavyGateReceiptSlotRegistryFinalTranscriptRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE92_LIVE_HEAVY_GATE_RECEIPT_SLOT_REGISTRY_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave92-live-heavy-gate-receipt-slot-registry-final-transcript-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE92_LIVE_HEAVY_GATE_RECEIPT_SLOT_REGISTRY_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const SLOT_SUITE: &str = "monero-l2-pq-force-exit-live-heavy-gate-receipt-slot-registry-v1";
pub const DEFAULT_WAVE: u64 = 92;
pub const DEFAULT_SOURCE_PLAN_WAVE: u64 = 91;
pub const DEFAULT_SOURCE_DENIAL_WAVE: u64 = 90;
pub const DEFAULT_SOURCE_ARCHIVE_WAVE: u64 = 89;
pub const DEFAULT_REGISTRY_HEIGHT: u64 = 920_000;
pub const DEFAULT_MIN_SLOT_LANES: u16 = 6;
pub const DEFAULT_MIN_RECEIPT_SLOTS: u16 = 48;
pub const DEFAULT_MIN_IMPORT_RULES: u16 = 12;
pub const DEFAULT_MIN_RESERVED_WEIGHT: u64 = 120;
pub const DEFAULT_MAX_SLOT_AGE_BLOCKS: u64 = 144;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotLane {
    CompileSlot,
    RuntimeReplaySlot,
    AuditSecuritySlot,
    BridgeCustodySlot,
    WalletWatchtowerSlot,
    PqReservePrivacySlot,
}

impl SlotLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileSlot,
            Self::RuntimeReplaySlot,
            Self::AuditSecuritySlot,
            Self::BridgeCustodySlot,
            Self::WalletWatchtowerSlot,
            Self::PqReservePrivacySlot,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileSlot => "compile_slot",
            Self::RuntimeReplaySlot => "runtime_replay_slot",
            Self::AuditSecuritySlot => "audit_security_slot",
            Self::BridgeCustodySlot => "bridge_custody_slot",
            Self::WalletWatchtowerSlot => "wallet_watchtower_slot",
            Self::PqReservePrivacySlot => "pq_reserve_privacy_slot",
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::CompileSlot => "receipt-slot-compile-owner",
            Self::RuntimeReplaySlot => "receipt-slot-runtime-owner",
            Self::AuditSecuritySlot => "receipt-slot-security-owner",
            Self::BridgeCustodySlot => "receipt-slot-custody-owner",
            Self::WalletWatchtowerSlot => "receipt-slot-wallet-owner",
            Self::PqReservePrivacySlot => "receipt-slot-pq-reserve-privacy-owner",
        }
    }

    pub fn needs_custody_binding(self) -> bool {
        matches!(self, Self::BridgeCustodySlot | Self::PqReservePrivacySlot)
    }

    pub fn needs_privacy_binding(self) -> bool {
        matches!(
            self,
            Self::AuditSecuritySlot | Self::WalletWatchtowerSlot | Self::PqReservePrivacySlot
        )
    }

    pub fn needs_execution_binding(self) -> bool {
        matches!(self, Self::CompileSlot | Self::RuntimeReplaySlot)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotKind {
    Wave91PlanRoot,
    Wave90DenialRoot,
    CargoCheckReceipt,
    CargoTestReceipt,
    ClippyReceipt,
    RustfmtReceipt,
    RustcReceipt,
    RuntimeReplayReceipt,
    RollbackDrillReceipt,
    AdversarialReplayReceipt,
    AuditReviewReceipt,
    ThreatModelReceipt,
    BridgeCustodyReceipt,
    WatchtowerQuorumReceipt,
    WalletEscapeReceipt,
    PqSignerEpochReceipt,
    ReserveCoverageReceipt,
    PrivacyRedactionReceipt,
    NullifierSeparationReceipt,
    OperatorSignoffReceipt,
    ReleaseCaptainClearanceReceipt,
}

impl SlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave91PlanRoot => "wave91_plan_root",
            Self::Wave90DenialRoot => "wave90_denial_root",
            Self::CargoCheckReceipt => "cargo_check_receipt",
            Self::CargoTestReceipt => "cargo_test_receipt",
            Self::ClippyReceipt => "clippy_receipt",
            Self::RustfmtReceipt => "rustfmt_receipt",
            Self::RustcReceipt => "rustc_receipt",
            Self::RuntimeReplayReceipt => "runtime_replay_receipt",
            Self::RollbackDrillReceipt => "rollback_drill_receipt",
            Self::AdversarialReplayReceipt => "adversarial_replay_receipt",
            Self::AuditReviewReceipt => "audit_review_receipt",
            Self::ThreatModelReceipt => "threat_model_receipt",
            Self::BridgeCustodyReceipt => "bridge_custody_receipt",
            Self::WatchtowerQuorumReceipt => "watchtower_quorum_receipt",
            Self::WalletEscapeReceipt => "wallet_escape_receipt",
            Self::PqSignerEpochReceipt => "pq_signer_epoch_receipt",
            Self::ReserveCoverageReceipt => "reserve_coverage_receipt",
            Self::PrivacyRedactionReceipt => "privacy_redaction_receipt",
            Self::NullifierSeparationReceipt => "nullifier_separation_receipt",
            Self::OperatorSignoffReceipt => "operator_signoff_receipt",
            Self::ReleaseCaptainClearanceReceipt => "release_captain_clearance_receipt",
        }
    }

    pub fn required_for_lane(self, lane: SlotLane) -> bool {
        match self {
            Self::Wave91PlanRoot
            | Self::Wave90DenialRoot
            | Self::OperatorSignoffReceipt
            | Self::ReleaseCaptainClearanceReceipt => true,
            Self::CargoCheckReceipt
            | Self::CargoTestReceipt
            | Self::ClippyReceipt
            | Self::RustfmtReceipt
            | Self::RustcReceipt => matches!(lane, SlotLane::CompileSlot),
            Self::RuntimeReplayReceipt
            | Self::RollbackDrillReceipt
            | Self::AdversarialReplayReceipt => matches!(lane, SlotLane::RuntimeReplaySlot),
            Self::AuditReviewReceipt | Self::ThreatModelReceipt => {
                matches!(lane, SlotLane::AuditSecuritySlot)
            }
            Self::BridgeCustodyReceipt => lane.needs_custody_binding(),
            Self::WatchtowerQuorumReceipt | Self::WalletEscapeReceipt => {
                matches!(lane, SlotLane::WalletWatchtowerSlot)
            }
            Self::PqSignerEpochReceipt | Self::ReserveCoverageReceipt => {
                matches!(lane, SlotLane::PqReservePrivacySlot)
            }
            Self::PrivacyRedactionReceipt | Self::NullifierSeparationReceipt => {
                lane.needs_privacy_binding()
            }
        }
    }

    pub fn reserved_weight(self) -> u64 {
        match self {
            Self::Wave91PlanRoot | Self::Wave90DenialRoot => 8,
            Self::OperatorSignoffReceipt => 10,
            Self::ReleaseCaptainClearanceReceipt => 24,
            Self::CargoCheckReceipt
            | Self::CargoTestReceipt
            | Self::ClippyReceipt
            | Self::RustfmtReceipt
            | Self::RustcReceipt => 18,
            Self::RuntimeReplayReceipt
            | Self::RollbackDrillReceipt
            | Self::AdversarialReplayReceipt => 20,
            Self::AuditReviewReceipt | Self::ThreatModelReceipt => 22,
            Self::BridgeCustodyReceipt
            | Self::WatchtowerQuorumReceipt
            | Self::WalletEscapeReceipt => 21,
            Self::PqSignerEpochReceipt
            | Self::ReserveCoverageReceipt
            | Self::PrivacyRedactionReceipt
            | Self::NullifierSeparationReceipt => 23,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotStatus {
    ReservedEmpty,
    WaitingForLiveRun,
    WaitingForReviewer,
    WaitingForOperator,
    ReceiptRootAttached,
    AcceptedLiveReceipt,
    RejectedReceipt,
}

impl SlotStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReservedEmpty => "reserved_empty",
            Self::WaitingForLiveRun => "waiting_for_live_run",
            Self::WaitingForReviewer => "waiting_for_reviewer",
            Self::WaitingForOperator => "waiting_for_operator",
            Self::ReceiptRootAttached => "receipt_root_attached",
            Self::AcceptedLiveReceipt => "accepted_live_receipt",
            Self::RejectedReceipt => "rejected_receipt",
        }
    }

    pub fn clears_slot(self) -> bool {
        matches!(self, Self::AcceptedLiveReceipt)
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::RejectedReceipt => 10,
            Self::ReservedEmpty => 9,
            Self::WaitingForLiveRun => 8,
            Self::WaitingForReviewer => 7,
            Self::WaitingForOperator => 6,
            Self::ReceiptRootAttached => 4,
            Self::AcceptedLiveReceipt => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotBlockerKind {
    MissingPlanRoot,
    MissingDenialRoot,
    EmptyReceiptSlot,
    LiveReceiptMissing,
    ReviewerBindingMissing,
    OperatorBindingMissing,
    CustodyBindingMissing,
    PrivacyBindingMissing,
    ExecutionBindingMissing,
    ReceiptRejected,
    SlotLaneCountTooLow,
    ReceiptSlotCountTooLow,
    ImportRuleCountTooLow,
    ReservedWeightTooLow,
    SlotTooOld,
    EmptyRoot,
}

impl SlotBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingPlanRoot => "missing_plan_root",
            Self::MissingDenialRoot => "missing_denial_root",
            Self::EmptyReceiptSlot => "empty_receipt_slot",
            Self::LiveReceiptMissing => "live_receipt_missing",
            Self::ReviewerBindingMissing => "reviewer_binding_missing",
            Self::OperatorBindingMissing => "operator_binding_missing",
            Self::CustodyBindingMissing => "custody_binding_missing",
            Self::PrivacyBindingMissing => "privacy_binding_missing",
            Self::ExecutionBindingMissing => "execution_binding_missing",
            Self::ReceiptRejected => "receipt_rejected",
            Self::SlotLaneCountTooLow => "slot_lane_count_too_low",
            Self::ReceiptSlotCountTooLow => "receipt_slot_count_too_low",
            Self::ImportRuleCountTooLow => "import_rule_count_too_low",
            Self::ReservedWeightTooLow => "reserved_weight_too_low",
            Self::SlotTooOld => "slot_too_old",
            Self::EmptyRoot => "empty_root",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::MissingPlanRoot | Self::MissingDenialRoot | Self::ReceiptRejected => 10,
            Self::EmptyReceiptSlot
            | Self::LiveReceiptMissing
            | Self::CustodyBindingMissing
            | Self::PrivacyBindingMissing
            | Self::ExecutionBindingMissing => 9,
            Self::ReviewerBindingMissing | Self::OperatorBindingMissing | Self::SlotTooOld => 8,
            Self::SlotLaneCountTooLow
            | Self::ReceiptSlotCountTooLow
            | Self::ImportRuleCountTooLow
            | Self::ReservedWeightTooLow => 7,
            Self::EmptyRoot => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportRuleKind {
    BindToWave91Plan,
    BindToWave90Denial,
    RequireLiveAcceptedRoot,
    RequireReviewerRoot,
    RequireOperatorRoot,
    PreserveRootsOnly,
    KeepFailClosed,
}

impl ImportRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BindToWave91Plan => "bind_to_wave91_plan",
            Self::BindToWave90Denial => "bind_to_wave90_denial",
            Self::RequireLiveAcceptedRoot => "require_live_accepted_root",
            Self::RequireReviewerRoot => "require_reviewer_root",
            Self::RequireOperatorRoot => "require_operator_root",
            Self::PreserveRootsOnly => "preserve_roots_only",
            Self::KeepFailClosed => "keep_fail_closed",
        }
    }

    pub fn for_lane(lane: SlotLane) -> Vec<Self> {
        let mut rules = vec![
            Self::BindToWave91Plan,
            Self::BindToWave90Denial,
            Self::RequireLiveAcceptedRoot,
            Self::PreserveRootsOnly,
            Self::KeepFailClosed,
        ];
        if lane.needs_custody_binding() || lane.needs_privacy_binding() {
            rules.push(Self::RequireReviewerRoot);
        }
        rules.push(Self::RequireOperatorRoot);
        rules
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub wave: u64,
    pub source_plan_wave: u64,
    pub source_denial_wave: u64,
    pub source_archive_wave: u64,
    pub chain_id: String,
    pub protocol_version: String,
    pub registry_height: u64,
    pub max_slot_age_blocks: u64,
    pub min_slot_lanes: u16,
    pub min_receipt_slots: u16,
    pub min_import_rules: u16,
    pub min_reserved_weight: u64,
    pub require_plan_roots: bool,
    pub require_denial_roots: bool,
    pub require_accepted_live_roots: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            wave: DEFAULT_WAVE,
            source_plan_wave: DEFAULT_SOURCE_PLAN_WAVE,
            source_denial_wave: DEFAULT_SOURCE_DENIAL_WAVE,
            source_archive_wave: DEFAULT_SOURCE_ARCHIVE_WAVE,
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            registry_height: DEFAULT_REGISTRY_HEIGHT,
            max_slot_age_blocks: DEFAULT_MAX_SLOT_AGE_BLOCKS,
            min_slot_lanes: DEFAULT_MIN_SLOT_LANES,
            min_receipt_slots: DEFAULT_MIN_RECEIPT_SLOTS,
            min_import_rules: DEFAULT_MIN_IMPORT_RULES,
            min_reserved_weight: DEFAULT_MIN_RESERVED_WEIGHT,
            require_plan_roots: true,
            require_denial_roots: true,
            require_accepted_live_roots: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure(
            self.wave >= self.source_plan_wave,
            "wave must cover source plan wave",
        )?;
        ensure(
            self.source_plan_wave >= self.source_denial_wave,
            "source plan wave must cover denial wave",
        )?;
        ensure(
            self.source_denial_wave >= self.source_archive_wave,
            "source denial wave must cover archive wave",
        )?;
        ensure(self.registry_height > 0, "registry height must be positive")?;
        ensure(
            self.max_slot_age_blocks > 0,
            "max slot age must be positive",
        )?;
        ensure(self.min_slot_lanes > 0, "min slot lanes must be positive")?;
        ensure(
            self.min_receipt_slots > 0,
            "min receipt slots must be positive",
        )?;
        ensure(
            self.min_import_rules > 0,
            "min import rules must be positive",
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_live_receipt_slot_registry_config",
            "wave": self.wave,
            "source_plan_wave": self.source_plan_wave,
            "source_denial_wave": self.source_denial_wave,
            "source_archive_wave": self.source_archive_wave,
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "registry_height": self.registry_height,
            "max_slot_age_blocks": self.max_slot_age_blocks,
            "min_slot_lanes": self.min_slot_lanes,
            "min_receipt_slots": self.min_receipt_slots,
            "min_import_rules": self.min_import_rules,
            "min_reserved_weight": self.min_reserved_weight,
            "require_plan_roots": self.require_plan_roots,
            "require_denial_roots": self.require_denial_roots,
            "require_accepted_live_roots": self.require_accepted_live_roots,
            "hash_suite": HASH_SUITE,
            "slot_suite": SLOT_SUITE,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptSlot {
    pub slot_id: String,
    pub lane: SlotLane,
    pub slot_kind: SlotKind,
    pub status: SlotStatus,
    pub source_wave: u64,
    pub opened_height: u64,
    pub plan_root: String,
    pub denial_root: String,
    pub slot_commitment_root: String,
    pub accepted_receipt_root: Option<String>,
    pub reviewer_root: Option<String>,
    pub operator_root: Option<String>,
    pub import_rule_root: String,
    pub reserved_weight: u64,
    pub roots_only: bool,
}

impl ReceiptSlot {
    pub fn new(
        lane: SlotLane,
        slot_kind: SlotKind,
        status: SlotStatus,
        config: &Config,
        ordinal: u64,
    ) -> Result<Self> {
        let source_wave = match slot_kind {
            SlotKind::Wave91PlanRoot => config.source_plan_wave,
            SlotKind::Wave90DenialRoot => config.source_denial_wave,
            _ => config.wave,
        };
        let slot = Self {
            slot_id: stable_id("receipt-slot", lane.as_str(), slot_kind.as_str(), ordinal),
            lane,
            slot_kind,
            status,
            source_wave,
            opened_height: config.registry_height.saturating_add(ordinal),
            plan_root: sample_root("wave91-plan", lane.as_str(), slot_kind.as_str(), ordinal),
            denial_root: sample_root("wave90-denial", lane.as_str(), slot_kind.as_str(), ordinal),
            slot_commitment_root: sample_root(
                "slot-commitment",
                lane.as_str(),
                slot_kind.as_str(),
                ordinal,
            ),
            accepted_receipt_root: None,
            reviewer_root: None,
            operator_root: None,
            import_rule_root: sample_root(
                "import-rule",
                lane.as_str(),
                slot_kind.as_str(),
                ordinal,
            ),
            reserved_weight: slot_kind.reserved_weight(),
            roots_only: true,
        };
        slot.validate()?;
        Ok(slot)
    }

    pub fn devnet(lane: SlotLane, slot_kind: SlotKind, config: &Config, ordinal: u64) -> Self {
        let status = slot_status(slot_kind);
        match Self::new(lane, slot_kind, status, config, ordinal) {
            Ok(slot) => slot,
            Err(reason) => fallback_slot(lane, slot_kind, config, ordinal, reason),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("slot_id", &self.slot_id)?;
        ensure_non_empty("plan_root", &self.plan_root)?;
        ensure_non_empty("denial_root", &self.denial_root)?;
        ensure_non_empty("slot_commitment_root", &self.slot_commitment_root)?;
        ensure_non_empty("import_rule_root", &self.import_rule_root)?;
        ensure(
            self.slot_kind.required_for_lane(self.lane),
            "slot kind must belong to lane",
        )?;
        ensure(self.reserved_weight > 0, "reserved weight must be positive")
    }

    pub fn clears_slot(&self) -> bool {
        self.status.clears_slot()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_receipt_slot",
            "slot_id": self.slot_id,
            "lane": self.lane.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "source_wave": self.source_wave,
            "opened_height": self.opened_height,
            "plan_root": self.plan_root,
            "denial_root": self.denial_root,
            "slot_commitment_root": self.slot_commitment_root,
            "accepted_receipt_root": self.accepted_receipt_root,
            "reviewer_root": self.reviewer_root,
            "operator_root": self.operator_root,
            "import_rule_root": self.import_rule_root,
            "reserved_weight": self.reserved_weight,
            "roots_only": self.roots_only,
            "clears_slot": self.clears_slot(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("receipt-slot", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportRule {
    pub rule_id: String,
    pub lane: SlotLane,
    pub rule_kind: ImportRuleKind,
    pub rule_root: String,
    pub command_root: String,
    pub fail_closed: bool,
}

impl ImportRule {
    pub fn devnet(lane: SlotLane, rule_kind: ImportRuleKind, ordinal: u64) -> Self {
        Self {
            rule_id: stable_id("import-rule", lane.as_str(), rule_kind.as_str(), ordinal),
            lane,
            rule_kind,
            rule_root: sample_root("rule", lane.as_str(), rule_kind.as_str(), ordinal),
            command_root: sample_root("command", lane.as_str(), rule_kind.as_str(), ordinal),
            fail_closed: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_import_rule",
            "rule_id": self.rule_id,
            "lane": self.lane.as_str(),
            "rule_kind": self.rule_kind.as_str(),
            "rule_root": self.rule_root,
            "command_root": self.command_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("import-rule", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneSlotRegistry {
    pub lane: SlotLane,
    pub owner: String,
    pub slot_count: u16,
    pub empty_slot_count: u16,
    pub clear_slot_count: u16,
    pub import_rule_count: u16,
    pub reserved_weight: u64,
    pub max_severity: u8,
    pub plan_root: String,
    pub slot_root: String,
    pub import_rule_root: String,
    pub blocker_root: String,
    pub lane_clear: bool,
}

impl LaneSlotRegistry {
    pub fn build(
        lane: SlotLane,
        slots: &[ReceiptSlot],
        rules: &[ImportRule],
        blockers: &[SlotBlockerKind],
        ordinal: u64,
    ) -> Self {
        let empty_slot_count = slots.iter().filter(|slot| !slot.clears_slot()).count() as u16;
        let clear_slot_count = slots.iter().filter(|slot| slot.clears_slot()).count() as u16;
        let reserved_weight = slots.iter().map(|slot| slot.reserved_weight).sum::<u64>();
        Self {
            lane,
            owner: lane.owner().to_string(),
            slot_count: slots.len() as u16,
            empty_slot_count,
            clear_slot_count,
            import_rule_count: rules.len() as u16,
            reserved_weight,
            max_severity: max_blocker_severity(blockers),
            plan_root: sample_root("wave91-lane-plan", lane.as_str(), "slot-registry", ordinal),
            slot_root: roots_root(
                "wave92-lane-receipt-slots",
                slots.iter().map(ReceiptSlot::state_root),
            ),
            import_rule_root: roots_root(
                "wave92-lane-import-rules",
                rules.iter().map(ImportRule::state_root),
            ),
            blocker_root: blocker_list_root(lane, blockers),
            lane_clear: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_lane_slot_registry",
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "slot_count": self.slot_count,
            "empty_slot_count": self.empty_slot_count,
            "clear_slot_count": self.clear_slot_count,
            "import_rule_count": self.import_rule_count,
            "reserved_weight": self.reserved_weight,
            "max_severity": self.max_severity,
            "plan_root": self.plan_root,
            "slot_root": self.slot_root,
            "import_rule_root": self.import_rule_root,
            "blocker_root": self.blocker_root,
            "lane_clear": self.lane_clear,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-slot-registry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrySummary {
    pub slot_lane_count: u16,
    pub receipt_slot_count: u16,
    pub empty_slot_count: u16,
    pub import_rule_count: u16,
    pub total_reserved_weight: u64,
    pub max_blocker_severity: u8,
    pub all_slots_clear: bool,
    pub production_ready: bool,
    pub registry_root: String,
    pub release_hold_root: String,
}

impl RegistrySummary {
    pub fn build(
        config: &Config,
        lane_registries: &[LaneSlotRegistry],
        slots: &[ReceiptSlot],
        rules: &[ImportRule],
        blockers: &BTreeMap<String, Vec<SlotBlockerKind>>,
    ) -> Self {
        let slot_lane_count = lane_registries.len() as u16;
        let receipt_slot_count = slots.len() as u16;
        let empty_slot_count = slots.iter().filter(|slot| !slot.clears_slot()).count() as u16;
        let import_rule_count = rules.len() as u16;
        let total_reserved_weight = slots.iter().map(|slot| slot.reserved_weight).sum::<u64>();
        let max_blocker_severity = max_blocker_map_severity(blockers);
        let registry_root = roots_root(
            "wave92-slot-registry-root",
            lane_registries.iter().map(LaneSlotRegistry::state_root),
        );
        let release_hold_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-WAVE92-SLOT-REGISTRY-RELEASE-HOLD",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::U64(config.wave),
                HashPart::U64(receipt_slot_count as u64),
                HashPart::U64(empty_slot_count as u64),
                HashPart::Str(&registry_root),
            ],
            32,
        );
        Self {
            slot_lane_count,
            receipt_slot_count,
            empty_slot_count,
            import_rule_count,
            total_reserved_weight,
            max_blocker_severity,
            all_slots_clear: false,
            production_ready: false,
            registry_root,
            release_hold_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_live_receipt_slot_registry_summary",
            "slot_lane_count": self.slot_lane_count,
            "receipt_slot_count": self.receipt_slot_count,
            "empty_slot_count": self.empty_slot_count,
            "import_rule_count": self.import_rule_count,
            "total_reserved_weight": self.total_reserved_weight,
            "max_blocker_severity": self.max_blocker_severity,
            "all_slots_clear": self.all_slots_clear,
            "production_ready": self.production_ready,
            "registry_root": self.registry_root,
            "release_hold_root": self.release_hold_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("registry-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub registry_height: u64,
    pub lane_registries: Vec<LaneSlotRegistry>,
    pub receipt_slots: Vec<ReceiptSlot>,
    pub import_rules: Vec<ImportRule>,
    pub blockers: BTreeMap<String, Vec<SlotBlockerKind>>,
    pub lane_registry_root: String,
    pub receipt_slot_root: String,
    pub import_rule_root: String,
    pub blocker_root: String,
    pub summary: RegistrySummary,
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

impl State {
    pub fn new(
        config: Config,
        receipt_slots: Vec<ReceiptSlot>,
        import_rules: Vec<ImportRule>,
    ) -> Result<Self> {
        config.validate()?;
        let blockers = evaluate_blockers(&config, &receipt_slots, &import_rules);
        let lane_registries = build_lane_registries(&receipt_slots, &import_rules, &blockers);
        let lane_registry_root = roots_root(
            "wave92-lane-registry-root",
            lane_registries.iter().map(LaneSlotRegistry::state_root),
        );
        let receipt_slot_root = roots_root(
            "wave92-receipt-slot-root",
            receipt_slots.iter().map(ReceiptSlot::state_root),
        );
        let import_rule_root = roots_root(
            "wave92-import-rule-root",
            import_rules.iter().map(ImportRule::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary = RegistrySummary::build(
            &config,
            &lane_registries,
            &receipt_slots,
            &import_rules,
            &blockers,
        );
        Ok(Self {
            registry_height: config.registry_height,
            config,
            lane_registries,
            receipt_slots,
            import_rules,
            blockers,
            lane_registry_root,
            receipt_slot_root,
            import_rule_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut receipt_slots = Vec::new();
        let mut slot_ordinal = 1_u64;
        for lane in SlotLane::all() {
            for slot_kind in slot_kinds_for_lane(lane) {
                receipt_slots.push(ReceiptSlot::devnet(lane, slot_kind, &config, slot_ordinal));
                slot_ordinal = slot_ordinal.saturating_add(1);
            }
        }
        let mut import_rules = Vec::new();
        let mut rule_ordinal = 1_u64;
        for lane in SlotLane::all() {
            for rule_kind in ImportRuleKind::for_lane(lane) {
                import_rules.push(ImportRule::devnet(lane, rule_kind, rule_ordinal));
                rule_ordinal = rule_ordinal.saturating_add(1);
            }
        }
        match Self::new(config, receipt_slots, import_rules) {
            Ok(state) => state,
            Err(reason) => fail_closed_fallback(reason),
        }
    }

    pub fn production_ready(&self) -> bool {
        false
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_live_heavy_gate_receipt_slot_registry_final_transcript_state",
            "config": self.config.public_record(),
            "registry_height": self.registry_height,
            "lane_registry_root": self.lane_registry_root,
            "receipt_slot_root": self.receipt_slot_root,
            "import_rule_root": self.import_rule_root,
            "blocker_root": self.blocker_root,
            "summary": self.summary.public_record(),
            "lane_registries": self.lane_registries.iter().map(LaneSlotRegistry::public_record).collect::<Vec<_>>(),
            "production_ready": self.production_ready(),
            "slot_registry_only": true,
            "live_receipts_attached": false,
            "heavy_gates_deferred": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn slot_kinds_for_lane(lane: SlotLane) -> Vec<SlotKind> {
    let all = vec![
        SlotKind::Wave91PlanRoot,
        SlotKind::Wave90DenialRoot,
        SlotKind::CargoCheckReceipt,
        SlotKind::CargoTestReceipt,
        SlotKind::ClippyReceipt,
        SlotKind::RustfmtReceipt,
        SlotKind::RustcReceipt,
        SlotKind::RuntimeReplayReceipt,
        SlotKind::RollbackDrillReceipt,
        SlotKind::AdversarialReplayReceipt,
        SlotKind::AuditReviewReceipt,
        SlotKind::ThreatModelReceipt,
        SlotKind::BridgeCustodyReceipt,
        SlotKind::WatchtowerQuorumReceipt,
        SlotKind::WalletEscapeReceipt,
        SlotKind::PqSignerEpochReceipt,
        SlotKind::ReserveCoverageReceipt,
        SlotKind::PrivacyRedactionReceipt,
        SlotKind::NullifierSeparationReceipt,
        SlotKind::OperatorSignoffReceipt,
        SlotKind::ReleaseCaptainClearanceReceipt,
    ];
    all.into_iter()
        .filter(|kind| kind.required_for_lane(lane))
        .collect()
}

fn slot_status(slot_kind: SlotKind) -> SlotStatus {
    match slot_kind {
        SlotKind::Wave91PlanRoot | SlotKind::Wave90DenialRoot => SlotStatus::ReservedEmpty,
        SlotKind::OperatorSignoffReceipt | SlotKind::ReleaseCaptainClearanceReceipt => {
            SlotStatus::WaitingForOperator
        }
        SlotKind::AuditReviewReceipt
        | SlotKind::ThreatModelReceipt
        | SlotKind::BridgeCustodyReceipt
        | SlotKind::PrivacyRedactionReceipt
        | SlotKind::NullifierSeparationReceipt => SlotStatus::WaitingForReviewer,
        SlotKind::CargoCheckReceipt
        | SlotKind::CargoTestReceipt
        | SlotKind::ClippyReceipt
        | SlotKind::RustfmtReceipt
        | SlotKind::RustcReceipt
        | SlotKind::RuntimeReplayReceipt
        | SlotKind::RollbackDrillReceipt
        | SlotKind::AdversarialReplayReceipt
        | SlotKind::WatchtowerQuorumReceipt
        | SlotKind::WalletEscapeReceipt
        | SlotKind::PqSignerEpochReceipt
        | SlotKind::ReserveCoverageReceipt => SlotStatus::WaitingForLiveRun,
    }
}

fn evaluate_blockers(
    config: &Config,
    slots: &[ReceiptSlot],
    rules: &[ImportRule],
) -> BTreeMap<String, Vec<SlotBlockerKind>> {
    let mut blockers: BTreeMap<String, Vec<SlotBlockerKind>> = BTreeMap::new();
    let lanes = slots.iter().map(|slot| slot.lane).collect::<BTreeSet<_>>();
    if lanes.len() < config.min_slot_lanes as usize {
        blockers
            .entry("slot_lanes".to_string())
            .or_default()
            .push(SlotBlockerKind::SlotLaneCountTooLow);
    }
    if slots.len() < config.min_receipt_slots as usize {
        blockers
            .entry("receipt_slots".to_string())
            .or_default()
            .push(SlotBlockerKind::ReceiptSlotCountTooLow);
    }
    if rules.len() < config.min_import_rules as usize {
        blockers
            .entry("import_rules".to_string())
            .or_default()
            .push(SlotBlockerKind::ImportRuleCountTooLow);
    }
    let reserved_weight = slots.iter().map(|slot| slot.reserved_weight).sum::<u64>();
    if reserved_weight < config.min_reserved_weight {
        blockers
            .entry("reserved_weight".to_string())
            .or_default()
            .push(SlotBlockerKind::ReservedWeightTooLow);
    }
    for lane in SlotLane::all() {
        let lane_slots = slots
            .iter()
            .filter(|slot| slot.lane == lane)
            .collect::<Vec<_>>();
        if config.require_plan_roots
            && !lane_slots
                .iter()
                .any(|slot| slot.slot_kind == SlotKind::Wave91PlanRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::MissingPlanRoot);
        }
        if config.require_denial_roots
            && !lane_slots
                .iter()
                .any(|slot| slot.slot_kind == SlotKind::Wave90DenialRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::MissingDenialRoot);
        }
        if config.require_accepted_live_roots && !lane_slots.iter().any(|slot| slot.clears_slot()) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::LiveReceiptMissing);
        }
        if lane.needs_execution_binding() && !lane_slots.iter().any(|slot| slot.clears_slot()) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::ExecutionBindingMissing);
        }
        if lane.needs_custody_binding() && !lane_slots.iter().any(|slot| slot.clears_slot()) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::CustodyBindingMissing);
        }
        if lane.needs_privacy_binding() && !lane_slots.iter().any(|slot| slot.clears_slot()) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::PrivacyBindingMissing);
        }
        if !rules
            .iter()
            .any(|rule| rule.lane == lane && rule.rule_kind == ImportRuleKind::RequireOperatorRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::OperatorBindingMissing);
        }
    }
    for slot in slots {
        if slot.plan_root.trim().is_empty()
            || slot.denial_root.trim().is_empty()
            || slot.slot_commitment_root.trim().is_empty()
            || slot.import_rule_root.trim().is_empty()
        {
            blockers
                .entry(slot.slot_id.clone())
                .or_default()
                .push(SlotBlockerKind::EmptyRoot);
        }
        if slot.opened_height.saturating_sub(config.registry_height) > config.max_slot_age_blocks {
            blockers
                .entry(slot.slot_id.clone())
                .or_default()
                .push(SlotBlockerKind::SlotTooOld);
        }
        if slot.status == SlotStatus::RejectedReceipt {
            blockers
                .entry(slot.lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::ReceiptRejected);
        }
        if !slot.clears_slot() {
            blockers
                .entry(slot.lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::EmptyReceiptSlot);
        }
    }
    for rule in rules {
        if rule.rule_root.trim().is_empty() || rule.command_root.trim().is_empty() {
            blockers
                .entry(rule.rule_id.clone())
                .or_default()
                .push(SlotBlockerKind::EmptyRoot);
        }
        if rule.rule_kind == ImportRuleKind::RequireReviewerRoot {
            blockers
                .entry(rule.lane.as_str().to_string())
                .or_default()
                .push(SlotBlockerKind::ReviewerBindingMissing);
        }
    }
    blockers
}

fn build_lane_registries(
    slots: &[ReceiptSlot],
    rules: &[ImportRule],
    blockers: &BTreeMap<String, Vec<SlotBlockerKind>>,
) -> Vec<LaneSlotRegistry> {
    SlotLane::all()
        .into_iter()
        .enumerate()
        .map(|(index, lane)| {
            let lane_slots = slots
                .iter()
                .filter(|slot| slot.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let lane_rules = rules
                .iter()
                .filter(|rule| rule.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let lane_blockers = match blockers.get(lane.as_str()) {
                Some(found) => found.clone(),
                None => Vec::new(),
            };
            LaneSlotRegistry::build(
                lane,
                &lane_slots,
                &lane_rules,
                &lane_blockers,
                one_based(index),
            )
        })
        .collect()
}

fn fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let slot = fallback_slot(
        SlotLane::CompileSlot,
        SlotKind::CargoCheckReceipt,
        &config,
        1,
        reason,
    );
    let rule = ImportRule::devnet(SlotLane::CompileSlot, ImportRuleKind::KeepFailClosed, 1);
    let mut blockers = evaluate_blockers(&config, &[slot.clone()], &[rule.clone()]);
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(SlotBlockerKind::EmptyReceiptSlot);
    let lane_registries = build_lane_registries(&[slot.clone()], &[rule.clone()], &blockers);
    let lane_registry_root = roots_root(
        "wave92-fallback-lane-registry-root",
        lane_registries.iter().map(LaneSlotRegistry::state_root),
    );
    let receipt_slot_root = roots_root(
        "wave92-fallback-receipt-slots",
        [slot.state_root()].into_iter(),
    );
    let import_rule_root = roots_root(
        "wave92-fallback-import-rules",
        [rule.state_root()].into_iter(),
    );
    let blocker_root = blockers_root(&blockers);
    let summary = RegistrySummary::build(
        &config,
        &lane_registries,
        &[slot.clone()],
        &[rule.clone()],
        &blockers,
    );
    State {
        registry_height: config.registry_height,
        config,
        lane_registries,
        receipt_slots: vec![slot],
        import_rules: vec![rule],
        blockers,
        lane_registry_root,
        receipt_slot_root,
        import_rule_root,
        blocker_root,
        summary,
    }
}

fn fallback_slot(
    lane: SlotLane,
    slot_kind: SlotKind,
    config: &Config,
    ordinal: u64,
    reason: String,
) -> ReceiptSlot {
    ReceiptSlot {
        slot_id: stable_id(
            "fallback-receipt-slot",
            lane.as_str(),
            slot_kind.as_str(),
            ordinal,
        ),
        lane,
        slot_kind,
        status: SlotStatus::ReservedEmpty,
        source_wave: config.wave,
        opened_height: config.registry_height,
        plan_root: sample_root("fallback-plan", lane.as_str(), &reason, ordinal),
        denial_root: sample_root("fallback-denial", lane.as_str(), &reason, ordinal),
        slot_commitment_root: sample_root("fallback-slot", lane.as_str(), &reason, ordinal),
        accepted_receipt_root: None,
        reviewer_root: None,
        operator_root: None,
        import_rule_root: sample_root("fallback-rule", lane.as_str(), &reason, ordinal),
        reserved_weight: slot_kind.reserved_weight(),
        roots_only: true,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<SlotBlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            json!({
                "subject": subject,
                "blockers": blocker_list.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                "max_severity": max_blocker_severity(blocker_list),
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave92-live-receipt-slot-registry-blockers", &leaves)
}

fn blocker_list_root(lane: SlotLane, blockers: &[SlotBlockerKind]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| {
            json!({
                "lane": lane.as_str(),
                "blocker": blocker.as_str(),
                "severity": blocker.severity(),
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave92-live-receipt-slot-registry-lane-blockers", &leaves)
}

fn max_blocker_severity(blockers: &[SlotBlockerKind]) -> u8 {
    let mut max_value = 0_u8;
    for blocker in blockers {
        let severity = blocker.severity();
        if severity > max_value {
            max_value = severity;
        }
    }
    max_value
}

fn max_blocker_map_severity(blockers: &BTreeMap<String, Vec<SlotBlockerKind>>) -> u8 {
    let mut max_value = 0_u8;
    for list in blockers.values() {
        let severity = max_blocker_severity(list);
        if severity > max_value {
            max_value = severity;
        }
    }
    max_value
}

fn roots_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(kind: &str, record: &PublicRecord) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE92-LIVE-RECEIPT-SLOT-REGISTRY-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn stable_id(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE92-LIVE-RECEIPT-SLOT-REGISTRY-ID",
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

fn sample_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE92-LIVE-RECEIPT-SLOT-REGISTRY-SAMPLE-ROOT",
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

fn one_based(index: usize) -> u64 {
    index as u64 + 1
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{label} must be non-empty"),
    )
}
