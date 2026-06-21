use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave92-live-heavy-gate-receipt-slot-registry-compile-slot-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "fnv1a64-domain-separated-roots-only-v1";
pub const WAVE_ID: &str = "wave92";
pub const PRIOR_WAVE_ID: &str = "wave91";
pub const LANE_ID: &str = "wave92-live-heavy-gate-receipt-slot-registry-compile-slot";
pub const PLAN_SOURCE_LANE_ID: &str = "wave91-live-heavy-gate-execution-plan";
pub const DEFAULT_CHAIN_ID: &str = "nebula-devnet";
pub const DEFAULT_MIN_PLAN_ROOTS: usize = 1;
pub const DEFAULT_MIN_RECEIPT_ROOT_CHARS: usize = 16;
pub const DEFAULT_MIN_OPERATOR_SIGNOFFS: usize = 2;
pub const DEFAULT_MAX_PLAN_ROOTS: usize = 32;
pub const DEFAULT_MAX_RECEIPTS_PER_SLOT: usize = 8;
pub const DEFAULT_MAX_COMMAND_HINTS: usize = 24;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptSlotKind {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl ReceiptSlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::Clippy => "clippy",
            Self::Rustfmt => "rustfmt",
            Self::Rustc => "rustc",
            Self::BuildMetadata => "build_metadata",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> [Self; 7] {
        [
            Self::CargoCheck,
            Self::CargoTest,
            Self::Clippy,
            Self::Rustfmt,
            Self::Rustc,
            Self::BuildMetadata,
            Self::OperatorSignoff,
        ]
    }

    pub fn is_operator_slot(self) -> bool {
        matches!(self, Self::OperatorSignoff)
    }

    pub fn canonical_blocker(self) -> SlotBlockerKind {
        match self {
            Self::CargoCheck => SlotBlockerKind::CargoCheckReceiptMissing,
            Self::CargoTest => SlotBlockerKind::CargoTestReceiptMissing,
            Self::Clippy => SlotBlockerKind::ClippyReceiptMissing,
            Self::Rustfmt => SlotBlockerKind::RustfmtReceiptMissing,
            Self::Rustc => SlotBlockerKind::RustcReceiptMissing,
            Self::BuildMetadata => SlotBlockerKind::BuildMetadataReceiptMissing,
            Self::OperatorSignoff => SlotBlockerKind::OperatorSignoffReceiptMissing,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanRootKind {
    ExecutionPlan,
    CompileReceipt,
    RuntimeReplayReceipt,
    FinalTranscript,
    AuditSecurityReceipt,
    BridgeCustodyReceipt,
    PqReservePrivacyReceipt,
    WalletWatchtowerReceipt,
}

impl PlanRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionPlan => "execution_plan",
            Self::CompileReceipt => "compile_receipt",
            Self::RuntimeReplayReceipt => "runtime_replay_receipt",
            Self::FinalTranscript => "final_transcript",
            Self::AuditSecurityReceipt => "audit_security_receipt",
            Self::BridgeCustodyReceipt => "bridge_custody_receipt",
            Self::PqReservePrivacyReceipt => "pq_reserve_privacy_receipt",
            Self::WalletWatchtowerReceipt => "wallet_watchtower_receipt",
        }
    }

    pub fn all() -> [Self; 8] {
        [
            Self::ExecutionPlan,
            Self::CompileReceipt,
            Self::RuntimeReplayReceipt,
            Self::FinalTranscript,
            Self::AuditSecurityReceipt,
            Self::BridgeCustodyReceipt,
            Self::PqReservePrivacyReceipt,
            Self::WalletWatchtowerReceipt,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Empty,
    Attached,
    Accepted,
    Rejected,
    Superseded,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Attached => "attached",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
        }
    }

    pub fn clears_slot(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotBlockerKind {
    NoPlanRootImported,
    PlanRootQuotaNotMet,
    ReceiptRootTooShort,
    ReceiptRootNotLiveAccepted,
    ReceiptRootDuplicate,
    ReceiptSlotFull,
    CargoCheckReceiptMissing,
    CargoTestReceiptMissing,
    ClippyReceiptMissing,
    RustfmtReceiptMissing,
    RustcReceiptMissing,
    BuildMetadataReceiptMissing,
    OperatorSignoffReceiptMissing,
    OperatorSignoffQuorumNotMet,
    ImportRuleDisabled,
    SlotStillFailClosed,
}

impl SlotBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoPlanRootImported => "no_plan_root_imported",
            Self::PlanRootQuotaNotMet => "plan_root_quota_not_met",
            Self::ReceiptRootTooShort => "receipt_root_too_short",
            Self::ReceiptRootNotLiveAccepted => "receipt_root_not_live_accepted",
            Self::ReceiptRootDuplicate => "receipt_root_duplicate",
            Self::ReceiptSlotFull => "receipt_slot_full",
            Self::CargoCheckReceiptMissing => "cargo_check_receipt_missing",
            Self::CargoTestReceiptMissing => "cargo_test_receipt_missing",
            Self::ClippyReceiptMissing => "clippy_receipt_missing",
            Self::RustfmtReceiptMissing => "rustfmt_receipt_missing",
            Self::RustcReceiptMissing => "rustc_receipt_missing",
            Self::BuildMetadataReceiptMissing => "build_metadata_receipt_missing",
            Self::OperatorSignoffReceiptMissing => "operator_signoff_receipt_missing",
            Self::OperatorSignoffQuorumNotMet => "operator_signoff_quorum_not_met",
            Self::ImportRuleDisabled => "import_rule_disabled",
            Self::SlotStillFailClosed => "slot_still_fail_closed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportRuleMode {
    Disabled,
    RootOnlyLiveAccepted,
}

impl ImportRuleMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::RootOnlyLiveAccepted => "root_only_live_accepted",
        }
    }

    pub fn permits_live_roots(self) -> bool {
        matches!(self, Self::RootOnlyLiveAccepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryVerdict {
    FailClosed,
    AwaitingLiveReceipts,
    Clearable,
}

impl RegistryVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::AwaitingLiveReceipts => "awaiting_live_receipts",
            Self::Clearable => "clearable",
        }
    }

    pub fn is_clearable(self) -> bool {
        matches!(self, Self::Clearable)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_id: String,
    pub prior_wave_id: String,
    pub lane_id: String,
    pub plan_source_lane_id: String,
    pub min_plan_roots: usize,
    pub min_receipt_root_chars: usize,
    pub min_operator_signoffs: usize,
    pub max_plan_roots: usize,
    pub max_receipts_per_slot: usize,
    pub max_command_hints: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: DEFAULT_CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            plan_source_lane_id: PLAN_SOURCE_LANE_ID.to_string(),
            min_plan_roots: DEFAULT_MIN_PLAN_ROOTS,
            min_receipt_root_chars: DEFAULT_MIN_RECEIPT_ROOT_CHARS,
            min_operator_signoffs: DEFAULT_MIN_OPERATOR_SIGNOFFS,
            max_plan_roots: DEFAULT_MAX_PLAN_ROOTS,
            max_receipts_per_slot: DEFAULT_MAX_RECEIPTS_PER_SLOT,
            max_command_hints: DEFAULT_MAX_COMMAND_HINTS,
        }
    }

    pub fn root(&self) -> String {
        root_for_parts(
            "config",
            &[
                self.chain_id.as_str(),
                self.protocol_version.as_str(),
                &self.schema_version.to_string(),
                self.hash_suite.as_str(),
                self.wave_id.as_str(),
                self.prior_wave_id.as_str(),
                self.lane_id.as_str(),
                self.plan_source_lane_id.as_str(),
                &self.min_plan_roots.to_string(),
                &self.min_receipt_root_chars.to_string(),
                &self.min_operator_signoffs.to_string(),
                &self.max_plan_roots.to_string(),
                &self.max_receipts_per_slot.to_string(),
                &self.max_command_hints.to_string(),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanRootIntake {
    pub kind: PlanRootKind,
    pub source_wave_id: String,
    pub source_lane_id: String,
    pub root: String,
    pub imported: bool,
}

impl PlanRootIntake {
    pub fn new(kind: PlanRootKind, root: impl Into<String>) -> Self {
        Self {
            kind,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: PLAN_SOURCE_LANE_ID.to_string(),
            root: root.into(),
            imported: true,
        }
    }

    pub fn empty(kind: PlanRootKind) -> Self {
        Self {
            kind,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: PLAN_SOURCE_LANE_ID.to_string(),
            root: String::new(),
            imported: false,
        }
    }

    pub fn record_root(&self) -> String {
        root_for_parts(
            "plan-root-intake",
            &[
                self.kind.as_str(),
                self.source_wave_id.as_str(),
                self.source_lane_id.as_str(),
                self.root.as_str(),
                bool_str(self.imported),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedReceiptPlaceholder {
    pub slot: ReceiptSlotKind,
    pub receipt_root: String,
    pub plan_root: String,
    pub live_acceptance_root: String,
    pub status: ReceiptStatus,
}

impl AcceptedReceiptPlaceholder {
    pub fn live_accepted(
        slot: ReceiptSlotKind,
        receipt_root: impl Into<String>,
        plan_root: impl Into<String>,
        live_acceptance_root: impl Into<String>,
    ) -> Self {
        Self {
            slot,
            receipt_root: receipt_root.into(),
            plan_root: plan_root.into(),
            live_acceptance_root: live_acceptance_root.into(),
            status: ReceiptStatus::Accepted,
        }
    }

    pub fn attached(
        slot: ReceiptSlotKind,
        receipt_root: impl Into<String>,
        plan_root: impl Into<String>,
        live_acceptance_root: impl Into<String>,
    ) -> Self {
        Self {
            slot,
            receipt_root: receipt_root.into(),
            plan_root: plan_root.into(),
            live_acceptance_root: live_acceptance_root.into(),
            status: ReceiptStatus::Attached,
        }
    }

    pub fn is_live_accepted(&self) -> bool {
        self.status.clears_slot()
            && !self.receipt_root.is_empty()
            && !self.plan_root.is_empty()
            && !self.live_acceptance_root.is_empty()
    }

    pub fn record_root(&self) -> String {
        root_for_parts(
            "accepted-receipt-placeholder",
            &[
                self.slot.as_str(),
                self.receipt_root.as_str(),
                self.plan_root.as_str(),
                self.live_acceptance_root.as_str(),
                self.status.as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotBlocker {
    pub kind: SlotBlockerKind,
    pub slot: Option<ReceiptSlotKind>,
    pub root: String,
}

impl SlotBlocker {
    pub fn new(
        kind: SlotBlockerKind,
        slot: Option<ReceiptSlotKind>,
        root: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            slot,
            root: root.into(),
        }
    }

    pub fn record_root(&self) -> String {
        let slot = match self.slot {
            Some(slot) => slot.as_str(),
            None => "registry",
        };
        root_for_parts(
            "slot-blocker",
            &[self.kind.as_str(), slot, self.root.as_str()],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportRule {
    pub slot: ReceiptSlotKind,
    pub mode: ImportRuleMode,
    pub requires_plan_root: bool,
    pub requires_live_acceptance_root: bool,
    pub root_only_privacy: bool,
}

impl ImportRule {
    pub fn live_root_only(slot: ReceiptSlotKind) -> Self {
        Self {
            slot,
            mode: ImportRuleMode::RootOnlyLiveAccepted,
            requires_plan_root: true,
            requires_live_acceptance_root: true,
            root_only_privacy: true,
        }
    }

    pub fn disabled(slot: ReceiptSlotKind) -> Self {
        Self {
            slot,
            mode: ImportRuleMode::Disabled,
            requires_plan_root: true,
            requires_live_acceptance_root: true,
            root_only_privacy: true,
        }
    }

    pub fn record_root(&self) -> String {
        root_for_parts(
            "import-rule",
            &[
                self.slot.as_str(),
                self.mode.as_str(),
                bool_str(self.requires_plan_root),
                bool_str(self.requires_live_acceptance_root),
                bool_str(self.root_only_privacy),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommandHint {
    pub slot: ReceiptSlotKind,
    pub command_id: String,
    pub action_root: String,
    pub reason_root: String,
}

impl OperatorCommandHint {
    pub fn new(
        slot: ReceiptSlotKind,
        command_id: impl Into<String>,
        action_root: impl Into<String>,
        reason_root: impl Into<String>,
    ) -> Self {
        Self {
            slot,
            command_id: command_id.into(),
            action_root: action_root.into(),
            reason_root: reason_root.into(),
        }
    }

    pub fn record_root(&self) -> String {
        root_for_parts(
            "operator-command-hint",
            &[
                self.slot.as_str(),
                self.command_id.as_str(),
                self.action_root.as_str(),
                self.reason_root.as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptSlot {
    pub kind: ReceiptSlotKind,
    pub canonical_root: String,
    pub fail_closed: bool,
    pub receipts: Vec<AcceptedReceiptPlaceholder>,
    pub blockers: Vec<SlotBlocker>,
}

impl ReceiptSlot {
    pub fn empty(kind: ReceiptSlotKind) -> Self {
        let canonical_root = root_for_parts("canonical-empty-receipt-slot", &[kind.as_str()]);
        Self {
            kind,
            canonical_root,
            fail_closed: true,
            receipts: Vec::new(),
            blockers: vec![SlotBlocker::new(kind.canonical_blocker(), Some(kind), "")],
        }
    }

    pub fn accepted_receipt_count(&self) -> usize {
        self.receipts
            .iter()
            .filter(|receipt| receipt.is_live_accepted())
            .count()
    }

    pub fn clearable(&self) -> bool {
        !self.fail_closed && self.accepted_receipt_count() > 0 && self.blockers.is_empty()
    }

    pub fn attach_receipt(
        &mut self,
        config: &Config,
        rule: &ImportRule,
        receipt: AcceptedReceiptPlaceholder,
    ) -> Result<()> {
        if !rule.mode.permits_live_roots() {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::ImportRuleDisabled,
                Some(self.kind),
                rule.record_root(),
            ));
            return Err("import rule disabled".to_string());
        }

        if receipt.slot != self.kind {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::SlotStillFailClosed,
                Some(self.kind),
                receipt.record_root(),
            ));
            return Err("receipt slot mismatch".to_string());
        }

        if receipt.receipt_root.len() < config.min_receipt_root_chars {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::ReceiptRootTooShort,
                Some(self.kind),
                receipt.record_root(),
            ));
            return Err("receipt root too short".to_string());
        }

        if !receipt.is_live_accepted() {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::ReceiptRootNotLiveAccepted,
                Some(self.kind),
                receipt.record_root(),
            ));
            return Err("receipt root lacks live acceptance".to_string());
        }

        if self.receipts.len() >= config.max_receipts_per_slot {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::ReceiptSlotFull,
                Some(self.kind),
                receipt.record_root(),
            ));
            return Err("receipt slot full".to_string());
        }

        if self
            .receipts
            .iter()
            .any(|stored| stored.receipt_root == receipt.receipt_root)
        {
            self.blockers.push(SlotBlocker::new(
                SlotBlockerKind::ReceiptRootDuplicate,
                Some(self.kind),
                receipt.record_root(),
            ));
            return Err("duplicate receipt root".to_string());
        }

        self.receipts.push(receipt);
        self.blockers.clear();
        self.fail_closed = false;
        Ok(())
    }

    pub fn record_root(&self) -> String {
        let receipt_roots = self
            .receipts
            .iter()
            .map(AcceptedReceiptPlaceholder::record_root)
            .collect::<Vec<_>>()
            .join("|");
        let blocker_roots = self
            .blockers
            .iter()
            .map(SlotBlocker::record_root)
            .collect::<Vec<_>>()
            .join("|");
        root_for_parts(
            "receipt-slot",
            &[
                self.kind.as_str(),
                self.canonical_root.as_str(),
                bool_str(self.fail_closed),
                receipt_roots.as_str(),
                blocker_roots.as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotRegistryVerdict {
    pub verdict: RegistryVerdict,
    pub clearable_slots: usize,
    pub blocked_slots: usize,
    pub blockers: Vec<SlotBlocker>,
    pub root: String,
}

impl SlotRegistryVerdict {
    pub fn fail_closed(blockers: Vec<SlotBlocker>) -> Self {
        let root = root_for_parts(
            "slot-registry-verdict",
            &[
                RegistryVerdict::FailClosed.as_str(),
                "0",
                &blockers.len().to_string(),
                &blockers
                    .iter()
                    .map(SlotBlocker::record_root)
                    .collect::<Vec<_>>()
                    .join("|"),
            ],
        );
        Self {
            verdict: RegistryVerdict::FailClosed,
            clearable_slots: 0,
            blocked_slots: blockers.len(),
            blockers,
            root,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub state_root: String,
    pub verdict_root: String,
    pub plan_root_count: usize,
    pub slot_count: usize,
    pub clearable_slot_count: usize,
    pub blocked_slot_count: usize,
    pub imported_plan_roots: Vec<String>,
    pub slot_roots: Vec<String>,
    pub command_hint_roots: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub plan_roots: BTreeMap<PlanRootKind, PlanRootIntake>,
    pub import_rules: BTreeMap<ReceiptSlotKind, ImportRule>,
    pub slots: BTreeMap<ReceiptSlotKind, ReceiptSlot>,
    pub operator_hints: Vec<OperatorCommandHint>,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut plan_roots = BTreeMap::new();
        for kind in PlanRootKind::all() {
            plan_roots.insert(kind, PlanRootIntake::empty(kind));
        }

        let mut import_rules = BTreeMap::new();
        let mut slots = BTreeMap::new();
        for kind in ReceiptSlotKind::all() {
            import_rules.insert(kind, ImportRule::live_root_only(kind));
            slots.insert(kind, ReceiptSlot::empty(kind));
        }

        Self {
            config,
            plan_roots,
            import_rules,
            slots,
            operator_hints: default_operator_hints(),
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet())
    }

    pub fn import_plan_root(&mut self, intake: PlanRootIntake) -> Result<()> {
        if self.imported_plan_root_count() >= self.config.max_plan_roots {
            return Err("plan root capacity reached".to_string());
        }
        self.plan_roots.insert(intake.kind, intake);
        Ok(())
    }

    pub fn imported_plan_root_count(&self) -> usize {
        self.plan_roots
            .values()
            .filter(|root| root.imported)
            .count()
    }

    pub fn imported_plan_roots(&self) -> Vec<String> {
        self.plan_roots
            .values()
            .filter(|root| root.imported)
            .map(PlanRootIntake::record_root)
            .collect()
    }

    pub fn attach_live_receipt(&mut self, receipt: AcceptedReceiptPlaceholder) -> Result<()> {
        if self.imported_plan_root_count() < self.config.min_plan_roots {
            return Err("plan root quota not met".to_string());
        }

        let rule = match self.import_rules.get(&receipt.slot) {
            Some(rule) => rule.clone(),
            None => return Err("missing import rule".to_string()),
        };

        match self.slots.get_mut(&receipt.slot) {
            Some(slot) => slot.attach_receipt(&self.config, &rule, receipt),
            None => Err("missing receipt slot".to_string()),
        }
    }

    pub fn clearable_slot_count(&self) -> usize {
        self.slots.values().filter(|slot| slot.clearable()).count()
    }

    pub fn blocked_slot_count(&self) -> usize {
        self.slots.len().saturating_sub(self.clearable_slot_count())
    }

    pub fn operator_signoff_count(&self) -> usize {
        match self.slots.get(&ReceiptSlotKind::OperatorSignoff) {
            Some(slot) => slot.accepted_receipt_count(),
            None => 0,
        }
    }

    pub fn blocker_set(&self) -> Vec<SlotBlocker> {
        let mut blockers = Vec::new();
        if self.imported_plan_root_count() == 0 {
            blockers.push(SlotBlocker::new(
                SlotBlockerKind::NoPlanRootImported,
                None,
                self.config.root(),
            ));
        }
        if self.imported_plan_root_count() < self.config.min_plan_roots {
            blockers.push(SlotBlocker::new(
                SlotBlockerKind::PlanRootQuotaNotMet,
                None,
                self.config.root(),
            ));
        }
        if self.operator_signoff_count() < self.config.min_operator_signoffs {
            blockers.push(SlotBlocker::new(
                SlotBlockerKind::OperatorSignoffQuorumNotMet,
                Some(ReceiptSlotKind::OperatorSignoff),
                self.config.root(),
            ));
        }
        for slot in self.slots.values() {
            if !slot.clearable() {
                blockers.push(SlotBlocker::new(
                    SlotBlockerKind::SlotStillFailClosed,
                    Some(slot.kind),
                    slot.record_root(),
                ));
            }
            blockers.extend(slot.blockers.clone());
        }
        dedupe_blockers(blockers)
    }

    pub fn verdict(&self) -> SlotRegistryVerdict {
        let blockers = self.blocker_set();
        let clearable_slots = self.clearable_slot_count();
        let blocked_slots = self.blocked_slot_count();
        let verdict = if blockers.is_empty()
            && clearable_slots == self.slots.len()
            && self.operator_signoff_count() >= self.config.min_operator_signoffs
        {
            RegistryVerdict::Clearable
        } else if self.imported_plan_root_count() >= self.config.min_plan_roots {
            RegistryVerdict::AwaitingLiveReceipts
        } else {
            RegistryVerdict::FailClosed
        };
        let root = root_for_parts(
            "slot-registry-verdict",
            &[
                verdict.as_str(),
                &clearable_slots.to_string(),
                &blocked_slots.to_string(),
                &blockers
                    .iter()
                    .map(SlotBlocker::record_root)
                    .collect::<Vec<_>>()
                    .join("|"),
            ],
        );
        SlotRegistryVerdict {
            verdict,
            clearable_slots,
            blocked_slots,
            blockers,
            root,
        }
    }

    pub fn state_root(&self) -> String {
        let plan_roots = self
            .plan_roots
            .values()
            .map(PlanRootIntake::record_root)
            .collect::<Vec<_>>()
            .join("|");
        let import_roots = self
            .import_rules
            .values()
            .map(ImportRule::record_root)
            .collect::<Vec<_>>()
            .join("|");
        let slot_roots = self
            .slots
            .values()
            .map(ReceiptSlot::record_root)
            .collect::<Vec<_>>()
            .join("|");
        let hint_roots = self
            .operator_hints
            .iter()
            .map(OperatorCommandHint::record_root)
            .collect::<Vec<_>>()
            .join("|");
        root_for_parts(
            "state",
            &[
                self.config.root().as_str(),
                plan_roots.as_str(),
                import_roots.as_str(),
                slot_roots.as_str(),
                hint_roots.as_str(),
                self.verdict().root.as_str(),
            ],
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        let verdict = self.verdict();
        PublicRecord {
            config_root: self.config.root(),
            state_root: self.state_root(),
            verdict_root: verdict.root,
            plan_root_count: self.imported_plan_root_count(),
            slot_count: self.slots.len(),
            clearable_slot_count: self.clearable_slot_count(),
            blocked_slot_count: self.blocked_slot_count(),
            imported_plan_roots: self.imported_plan_roots(),
            slot_roots: self.slots.values().map(ReceiptSlot::record_root).collect(),
            command_hint_roots: self
                .operator_hints
                .iter()
                .map(OperatorCommandHint::record_root)
                .collect(),
        }
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

pub fn canonical_slot_kinds() -> Vec<ReceiptSlotKind> {
    ReceiptSlotKind::all().to_vec()
}

pub fn canonical_plan_root_kinds() -> Vec<PlanRootKind> {
    PlanRootKind::all().to_vec()
}

pub fn default_operator_hints() -> Vec<OperatorCommandHint> {
    vec![
        OperatorCommandHint::new(
            ReceiptSlotKind::CargoCheck,
            "attach_cargo_check_live_receipt_root",
            root_for_parts("operator-action", &["cargo_check", "attach_live_root"]),
            root_for_parts(
                "operator-reason",
                &["cargo_check", "compile_lane_clearance"],
            ),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::CargoTest,
            "attach_cargo_test_live_receipt_root",
            root_for_parts("operator-action", &["cargo_test", "attach_live_root"]),
            root_for_parts(
                "operator-reason",
                &["cargo_test", "runtime_regression_clearance"],
            ),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::Clippy,
            "attach_clippy_live_receipt_root",
            root_for_parts("operator-action", &["clippy", "attach_live_root"]),
            root_for_parts("operator-reason", &["clippy", "lint_gate_clearance"]),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::Rustfmt,
            "attach_rustfmt_live_receipt_root",
            root_for_parts("operator-action", &["rustfmt", "attach_live_root"]),
            root_for_parts("operator-reason", &["rustfmt", "format_gate_clearance"]),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::Rustc,
            "attach_rustc_live_receipt_root",
            root_for_parts("operator-action", &["rustc", "attach_live_root"]),
            root_for_parts("operator-reason", &["rustc", "compiler_gate_clearance"]),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::BuildMetadata,
            "attach_build_metadata_live_receipt_root",
            root_for_parts("operator-action", &["build_metadata", "attach_live_root"]),
            root_for_parts(
                "operator-reason",
                &["build_metadata", "build_context_clearance"],
            ),
        ),
        OperatorCommandHint::new(
            ReceiptSlotKind::OperatorSignoff,
            "attach_operator_signoff_live_receipt_root",
            root_for_parts("operator-action", &["operator_signoff", "attach_live_root"]),
            root_for_parts(
                "operator-reason",
                &["operator_signoff", "human_release_clearance"],
            ),
        ),
    ]
}

pub fn root_for_parts(domain: &str, parts: &[&str]) -> String {
    let mut acc = 0xcbf29ce484222325_u64;
    acc = fnv1a_mix(acc, domain.as_bytes());
    for part in parts {
        acc = fnv1a_mix(acc, &[0x1f]);
        acc = fnv1a_mix(acc, part.as_bytes());
    }
    format!("{}:{:016x}", domain, acc)
}

fn fnv1a_mix(mut acc: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        acc ^= u64::from(*byte);
        acc = acc.wrapping_mul(0x100000001b3);
    }
    acc
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn dedupe_blockers(blockers: Vec<SlotBlocker>) -> Vec<SlotBlocker> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for blocker in blockers {
        let slot = match blocker.slot {
            Some(slot) => slot.as_str(),
            None => "registry",
        };
        let key = root_for_parts(
            "blocker-dedupe",
            &[blocker.kind.as_str(), slot, blocker.root.as_str()],
        );
        if seen.insert(key) {
            out.push(blocker);
        }
    }
    out
}
