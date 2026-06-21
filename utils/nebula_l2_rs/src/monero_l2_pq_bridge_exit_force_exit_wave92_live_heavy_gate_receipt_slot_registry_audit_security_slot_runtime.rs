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

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave92-live-heavy-gate-receipt-slot-registry-audit-security-slot-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const REGISTRY_SUITE: &str = "wave92-live-heavy-gate-receipt-slot-registry-v1";
pub const DEFAULT_WAVE: u64 = 92;
pub const DEFAULT_SOURCE_WAVE: u64 = 91;
pub const DEFAULT_HEIGHT: u64 = 4_281_520;
pub const DEFAULT_MAX_PLAN_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_PLAN_ROOTS: u64 = 6;
pub const DEFAULT_MIN_ACCEPTED_RECEIPT_ROOTS_PER_SLOT: u64 = 1;
pub const DEFAULT_MIN_OPERATOR_HINTS: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptSlotKind {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
}

impl ReceiptSlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialScenario => "adversarial_scenario",
            Self::ThreatModel => "threat_model",
            Self::PrivacyReview => "privacy_review",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditReview,
            Self::AdversarialScenario,
            Self::ThreatModel,
            Self::PrivacyReview,
            Self::ReviewerSignoff,
            Self::OperatorSignoff,
        ]
    }

    pub fn review_lane(self) -> ReviewLane {
        match self {
            Self::AuditReview => ReviewLane::Audit,
            Self::AdversarialScenario => ReviewLane::Security,
            Self::ThreatModel => ReviewLane::Security,
            Self::PrivacyReview => ReviewLane::Privacy,
            Self::ReviewerSignoff => ReviewLane::Reviewer,
            Self::OperatorSignoff => ReviewLane::Operator,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewLane {
    Audit,
    Security,
    Privacy,
    Reviewer,
    Operator,
}

impl ReviewLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Audit => "audit",
            Self::Security => "security",
            Self::Privacy => "privacy",
            Self::Reviewer => "reviewer",
            Self::Operator => "operator",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Placeholder,
    LiveAccepted,
    LiveRejected,
    Revoked,
    Stale,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Placeholder => "placeholder",
            Self::LiveAccepted => "live_accepted",
            Self::LiveRejected => "live_rejected",
            Self::Revoked => "revoked",
            Self::Stale => "stale",
        }
    }

    pub fn is_clearable(self) -> bool {
        matches!(self, Self::LiveAccepted)
    }

    pub fn blocks(self) -> bool {
        !self.is_clearable()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportRuleKind {
    RootsOnly,
    FutureLiveAccepted,
    SlotKindBound,
    SourcePlanBound,
    NoRawPayload,
    ReceiptRootUnique,
}

impl ImportRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootsOnly => "roots_only",
            Self::FutureLiveAccepted => "future_live_accepted",
            Self::SlotKindBound => "slot_kind_bound",
            Self::SourcePlanBound => "source_plan_bound",
            Self::NoRawPayload => "no_raw_payload",
            Self::ReceiptRootUnique => "receipt_root_unique",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    SourcePlanRootMissing,
    SourcePlanRootStale,
    SourcePlanRootRejected,
    SlotEmpty,
    SlotReceiptMissing,
    SlotReceiptNotAccepted,
    SlotReceiptRootMissing,
    SlotReceiptRootDuplicate,
    SlotPlanBindingMissing,
    SlotRootPrivacyViolation,
    RawPayloadRejected,
    OperatorHintMissing,
    RegistryFailClosedDisarmed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourcePlanRootMissing => "source_plan_root_missing",
            Self::SourcePlanRootStale => "source_plan_root_stale",
            Self::SourcePlanRootRejected => "source_plan_root_rejected",
            Self::SlotEmpty => "slot_empty",
            Self::SlotReceiptMissing => "slot_receipt_missing",
            Self::SlotReceiptNotAccepted => "slot_receipt_not_accepted",
            Self::SlotReceiptRootMissing => "slot_receipt_root_missing",
            Self::SlotReceiptRootDuplicate => "slot_receipt_root_duplicate",
            Self::SlotPlanBindingMissing => "slot_plan_binding_missing",
            Self::SlotRootPrivacyViolation => "slot_root_privacy_violation",
            Self::RawPayloadRejected => "raw_payload_rejected",
            Self::OperatorHintMissing => "operator_hint_missing",
            Self::RegistryFailClosedDisarmed => "registry_fail_closed_disarmed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictKind {
    Clearable,
    FailClosed,
}

impl VerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clearable => "clearable",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommandKind {
    HoldForceExit,
    ImportPlanRoot,
    AttachReceiptRoot,
    ReviewBlockers,
    ClearHeavyGate,
    KeepFailClosed,
}

impl OperatorCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldForceExit => "hold_force_exit",
            Self::ImportPlanRoot => "import_plan_root",
            Self::AttachReceiptRoot => "attach_receipt_root",
            Self::ReviewBlockers => "review_blockers",
            Self::ClearHeavyGate => "clear_heavy_gate",
            Self::KeepFailClosed => "keep_fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub registry_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub max_plan_age_blocks: u64,
    pub min_plan_roots: u64,
    pub min_accepted_receipt_roots_per_slot: u64,
    pub min_operator_hints: u64,
    pub max_raw_payload_records: u64,
    pub force_exit_gate_id: String,
    pub wave91_execution_plan_root: String,
    pub wave91_audit_plan_root: String,
    pub wave91_security_plan_root: String,
    pub wave91_privacy_plan_root: String,
    pub wave91_review_plan_root: String,
    pub wave91_operator_plan_root: String,
    pub fail_closed_armed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            registry_suite: REGISTRY_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            max_plan_age_blocks: DEFAULT_MAX_PLAN_AGE_BLOCKS,
            min_plan_roots: DEFAULT_MIN_PLAN_ROOTS,
            min_accepted_receipt_roots_per_slot: DEFAULT_MIN_ACCEPTED_RECEIPT_ROOTS_PER_SLOT,
            min_operator_hints: DEFAULT_MIN_OPERATOR_HINTS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            force_exit_gate_id: stable_id("force-exit-heavy-gate", "wave92-live-registry"),
            wave91_execution_plan_root: source_root("wave91-execution-plan"),
            wave91_audit_plan_root: source_root("wave91-audit-plan"),
            wave91_security_plan_root: source_root("wave91-security-plan"),
            wave91_privacy_plan_root: source_root("wave91-privacy-plan"),
            wave91_review_plan_root: source_root("wave91-review-plan"),
            wave91_operator_plan_root: source_root("wave91-operator-plan"),
            fail_closed_armed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("registry_suite", &self.registry_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("max_plan_age_blocks", self.max_plan_age_blocks)?;
        ensure_positive("min_plan_roots", self.min_plan_roots)?;
        ensure_positive(
            "min_accepted_receipt_roots_per_slot",
            self.min_accepted_receipt_roots_per_slot,
        )?;
        ensure_positive("min_operator_hints", self.min_operator_hints)?;
        ensure_non_empty("force_exit_gate_id", &self.force_exit_gate_id)?;
        ensure_root(
            "wave91_execution_plan_root",
            &self.wave91_execution_plan_root,
        )?;
        ensure_root("wave91_audit_plan_root", &self.wave91_audit_plan_root)?;
        ensure_root("wave91_security_plan_root", &self.wave91_security_plan_root)?;
        ensure_root("wave91_privacy_plan_root", &self.wave91_privacy_plan_root)?;
        ensure_root("wave91_review_plan_root", &self.wave91_review_plan_root)?;
        ensure_root("wave91_operator_plan_root", &self.wave91_operator_plan_root)?;
        if !self.fail_closed_armed {
            return Err("registry fail closed is not armed".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_receipt_slot_registry_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "registry_suite": self.registry_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "max_plan_age_blocks": self.max_plan_age_blocks,
            "min_plan_roots": self.min_plan_roots,
            "min_accepted_receipt_roots_per_slot": self.min_accepted_receipt_roots_per_slot,
            "min_operator_hints": self.min_operator_hints,
            "max_raw_payload_records": self.max_raw_payload_records,
            "force_exit_gate_id": self.force_exit_gate_id,
            "wave91_execution_plan_root": self.wave91_execution_plan_root,
            "wave91_audit_plan_root": self.wave91_audit_plan_root,
            "wave91_security_plan_root": self.wave91_security_plan_root,
            "wave91_privacy_plan_root": self.wave91_privacy_plan_root,
            "wave91_review_plan_root": self.wave91_review_plan_root,
            "wave91_operator_plan_root": self.wave91_operator_plan_root,
            "fail_closed_armed": self.fail_closed_armed,
        })
    }

    pub fn root(&self) -> String {
        value_root("WAVE92-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanRootIntake {
    pub plan_id: String,
    pub source_wave: u64,
    pub imported_at_height: u64,
    pub execution_plan_root: String,
    pub audit_plan_root: String,
    pub security_plan_root: String,
    pub privacy_plan_root: String,
    pub review_plan_root: String,
    pub operator_plan_root: String,
    pub plan_bundle_root: String,
    pub accepted: bool,
}

impl PlanRootIntake {
    pub fn from_config(config: &Config) -> Self {
        let mut intake = Self {
            plan_id: stable_id("wave91-plan-intake", &config.force_exit_gate_id),
            source_wave: config.source_wave,
            imported_at_height: config.current_height,
            execution_plan_root: config.wave91_execution_plan_root.clone(),
            audit_plan_root: config.wave91_audit_plan_root.clone(),
            security_plan_root: config.wave91_security_plan_root.clone(),
            privacy_plan_root: config.wave91_privacy_plan_root.clone(),
            review_plan_root: config.wave91_review_plan_root.clone(),
            operator_plan_root: config.wave91_operator_plan_root.clone(),
            plan_bundle_root: String::new(),
            accepted: true,
        };
        intake.plan_bundle_root = intake.compute_bundle_root();
        intake
    }

    pub fn roots(&self) -> Vec<String> {
        vec![
            self.execution_plan_root.clone(),
            self.audit_plan_root.clone(),
            self.security_plan_root.clone(),
            self.privacy_plan_root.clone(),
            self.review_plan_root.clone(),
            self.operator_plan_root.clone(),
        ]
    }

    pub fn compute_bundle_root(&self) -> String {
        string_root(
            "WAVE92-PLAN-BUNDLE",
            &[
                self.plan_id.clone(),
                self.source_wave.to_string(),
                self.execution_plan_root.clone(),
                self.audit_plan_root.clone(),
                self.security_plan_root.clone(),
                self.privacy_plan_root.clone(),
                self.review_plan_root.clone(),
                self.operator_plan_root.clone(),
            ],
        )
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("plan_id", &self.plan_id)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("imported_at_height", self.imported_at_height)?;
        ensure_root("execution_plan_root", &self.execution_plan_root)?;
        ensure_root("audit_plan_root", &self.audit_plan_root)?;
        ensure_root("security_plan_root", &self.security_plan_root)?;
        ensure_root("privacy_plan_root", &self.privacy_plan_root)?;
        ensure_root("review_plan_root", &self.review_plan_root)?;
        ensure_root("operator_plan_root", &self.operator_plan_root)?;
        ensure_root("plan_bundle_root", &self.plan_bundle_root)?;
        ensure_min_count(
            "plan roots",
            self.roots().len() as u64,
            config.min_plan_roots,
        )?;
        if self.source_wave != config.source_wave {
            return Err("plan source wave does not match config".to_string());
        }
        if self.imported_at_height > config.current_height {
            return Err("plan import height is ahead of registry height".to_string());
        }
        let age = config
            .current_height
            .saturating_sub(self.imported_at_height);
        if age > config.max_plan_age_blocks {
            return Err("plan root intake is stale".to_string());
        }
        if self.plan_bundle_root != self.compute_bundle_root() {
            return Err("plan bundle root does not match intake".to_string());
        }
        if !self.accepted {
            return Err("plan root intake is not accepted".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_execution_plan_root_intake",
            "plan_id": self.plan_id,
            "source_wave": self.source_wave,
            "imported_at_height": self.imported_at_height,
            "execution_plan_root": self.execution_plan_root,
            "audit_plan_root": self.audit_plan_root,
            "security_plan_root": self.security_plan_root,
            "privacy_plan_root": self.privacy_plan_root,
            "review_plan_root": self.review_plan_root,
            "operator_plan_root": self.operator_plan_root,
            "plan_bundle_root": self.plan_bundle_root,
            "accepted": self.accepted,
        })
    }

    pub fn root(&self) -> String {
        value_root("WAVE92-PLAN-INTAKE", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportRule {
    pub rule_id: String,
    pub kind: ImportRuleKind,
    pub slot_kind: Option<ReceiptSlotKind>,
    pub required: bool,
    pub root_only: bool,
    pub plan_bound: bool,
    pub clearable_when_met: bool,
    pub rule_root: String,
}

impl ImportRule {
    pub fn new(kind: ImportRuleKind, slot_kind: Option<ReceiptSlotKind>) -> Self {
        let slot = match slot_kind {
            Some(kind) => kind.as_str(),
            None => "registry",
        };
        let rule_id = stable_id("wave92-import-rule", &format!("{}:{}", kind.as_str(), slot));
        let mut rule = Self {
            rule_id,
            kind,
            slot_kind,
            required: true,
            root_only: matches!(
                kind,
                ImportRuleKind::RootsOnly | ImportRuleKind::NoRawPayload
            ),
            plan_bound: matches!(kind, ImportRuleKind::SourcePlanBound),
            clearable_when_met: !matches!(kind, ImportRuleKind::NoRawPayload),
            rule_root: String::new(),
        };
        rule.rule_root = rule.compute_root();
        rule
    }

    pub fn canonical_rules() -> Vec<Self> {
        let mut rules = vec![
            Self::new(ImportRuleKind::RootsOnly, None),
            Self::new(ImportRuleKind::FutureLiveAccepted, None),
            Self::new(ImportRuleKind::SourcePlanBound, None),
            Self::new(ImportRuleKind::NoRawPayload, None),
            Self::new(ImportRuleKind::ReceiptRootUnique, None),
        ];
        for slot_kind in ReceiptSlotKind::all() {
            rules.push(Self::new(ImportRuleKind::SlotKindBound, Some(slot_kind)));
        }
        rules
    }

    pub fn compute_root(&self) -> String {
        stable_id(
            "wave92-import-rule-root",
            &format!(
                "{}:{}:{}:{}:{}",
                self.rule_id,
                self.kind.as_str(),
                match self.slot_kind {
                    Some(kind) => kind.as_str(),
                    None => "registry",
                },
                self.root_only,
                self.plan_bound
            ),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("rule_id", &self.rule_id)?;
        ensure_root("rule_root", &self.rule_root)?;
        if self.rule_root != self.compute_root() {
            return Err("import rule root does not match rule".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_import_rule",
            "rule_id": self.rule_id,
            "rule_kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.map(ReceiptSlotKind::as_str),
            "required": self.required,
            "root_only": self.root_only,
            "plan_bound": self.plan_bound,
            "clearable_when_met": self.clearable_when_met,
            "rule_root": self.rule_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedReceiptPlaceholder {
    pub receipt_id: String,
    pub slot_kind: ReceiptSlotKind,
    pub status: ReceiptStatus,
    pub receipt_root: String,
    pub plan_bundle_root: String,
    pub attached_at_height: u64,
    pub live_root_attached: bool,
    pub privacy_root: String,
}

impl AcceptedReceiptPlaceholder {
    pub fn empty(slot_kind: ReceiptSlotKind, plan_bundle_root: &str) -> Self {
        let receipt_id = stable_id("wave92-empty-receipt-placeholder", slot_kind.as_str());
        Self {
            receipt_id,
            slot_kind,
            status: ReceiptStatus::Placeholder,
            receipt_root: empty_root(slot_kind.as_str()),
            plan_bundle_root: plan_bundle_root.to_string(),
            attached_at_height: 0,
            live_root_attached: false,
            privacy_root: empty_root("roots-only-privacy"),
        }
    }

    pub fn accepted(
        slot_kind: ReceiptSlotKind,
        receipt_root: &str,
        plan_bundle_root: &str,
        attached_at_height: u64,
    ) -> Self {
        let receipt_id = stable_id(
            "wave92-live-accepted-receipt",
            &format!("{}:{}", slot_kind.as_str(), receipt_root),
        );
        Self {
            receipt_id,
            slot_kind,
            status: ReceiptStatus::LiveAccepted,
            receipt_root: receipt_root.to_string(),
            plan_bundle_root: plan_bundle_root.to_string(),
            attached_at_height,
            live_root_attached: true,
            privacy_root: stable_id("wave92-receipt-privacy-root", receipt_root),
        }
    }

    pub fn clearable(&self) -> bool {
        self.status.is_clearable() && self.live_root_attached && is_root(&self.receipt_root)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("receipt_id", &self.receipt_id)?;
        ensure_root("receipt_root", &self.receipt_root)?;
        ensure_root("plan_bundle_root", &self.plan_bundle_root)?;
        ensure_root("privacy_root", &self.privacy_root)?;
        if self.status == ReceiptStatus::LiveAccepted && !self.live_root_attached {
            return Err("live accepted receipt has no attached root".to_string());
        }
        if self.live_root_attached {
            ensure_positive("attached_at_height", self.attached_at_height)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_accepted_receipt_placeholder",
            "receipt_id": self.receipt_id,
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "receipt_root": self.receipt_root,
            "plan_bundle_root": self.plan_bundle_root,
            "attached_at_height": self.attached_at_height,
            "live_root_attached": self.live_root_attached,
            "privacy_root": self.privacy_root,
        })
    }

    pub fn root(&self) -> String {
        value_root("WAVE92-ACCEPTED-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptSlot {
    pub slot_id: String,
    pub slot_kind: ReceiptSlotKind,
    pub review_lane: ReviewLane,
    pub required: bool,
    pub clearable: bool,
    pub receipt: AcceptedReceiptPlaceholder,
    pub slot_root: String,
}

impl ReceiptSlot {
    pub fn empty(slot_kind: ReceiptSlotKind, plan_bundle_root: &str) -> Self {
        let receipt = AcceptedReceiptPlaceholder::empty(slot_kind, plan_bundle_root);
        let mut slot = Self {
            slot_id: stable_id("wave92-receipt-slot", slot_kind.as_str()),
            slot_kind,
            review_lane: slot_kind.review_lane(),
            required: true,
            clearable: false,
            receipt,
            slot_root: String::new(),
        };
        slot.slot_root = slot.compute_root();
        slot
    }

    pub fn with_receipt(
        slot_kind: ReceiptSlotKind,
        receipt_root: &str,
        plan_bundle_root: &str,
        attached_at_height: u64,
    ) -> Self {
        let receipt = AcceptedReceiptPlaceholder::accepted(
            slot_kind,
            receipt_root,
            plan_bundle_root,
            attached_at_height,
        );
        let mut slot = Self {
            slot_id: stable_id("wave92-receipt-slot", slot_kind.as_str()),
            slot_kind,
            review_lane: slot_kind.review_lane(),
            required: true,
            clearable: receipt.clearable(),
            receipt,
            slot_root: String::new(),
        };
        slot.slot_root = slot.compute_root();
        slot
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE92-RECEIPT-SLOT",
            &json!({
                "slot_id": self.slot_id,
                "slot_kind": self.slot_kind.as_str(),
                "review_lane": self.review_lane.as_str(),
                "required": self.required,
                "clearable": self.clearable,
                "receipt_root": self.receipt.receipt_root,
                "receipt_status": self.receipt.status.as_str(),
                "plan_bundle_root": self.receipt.plan_bundle_root,
            }),
        )
    }

    pub fn attach_live_accepted_root(
        &mut self,
        receipt_root: &str,
        plan_bundle_root: &str,
        height: u64,
    ) -> Result<()> {
        ensure_root("receipt_root", receipt_root)?;
        ensure_root("plan_bundle_root", plan_bundle_root)?;
        ensure_positive("height", height)?;
        self.receipt = AcceptedReceiptPlaceholder::accepted(
            self.slot_kind,
            receipt_root,
            plan_bundle_root,
            height,
        );
        self.clearable = self.receipt.clearable();
        self.slot_root = self.compute_root();
        self.validate()?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("slot_id", &self.slot_id)?;
        self.receipt.validate()?;
        ensure_root("slot_root", &self.slot_root)?;
        if self.slot_kind != self.receipt.slot_kind {
            return Err("slot kind does not match receipt kind".to_string());
        }
        if self.clearable != self.receipt.clearable() {
            return Err("slot clearable flag does not match receipt".to_string());
        }
        if self.slot_root != self.compute_root() {
            return Err("slot root does not match slot".to_string());
        }
        Ok(())
    }

    pub fn blocker(&self) -> Option<AuditSlotBlocker> {
        if self.receipt.status == ReceiptStatus::Placeholder {
            return Some(AuditSlotBlocker::new(
                BlockerKind::SlotEmpty,
                Some(self.slot_kind),
                "slot has no future live accepted receipt root",
                &self.slot_root,
            ));
        }
        if self.receipt.receipt_root.is_empty() || !is_root(&self.receipt.receipt_root) {
            return Some(AuditSlotBlocker::new(
                BlockerKind::SlotReceiptRootMissing,
                Some(self.slot_kind),
                "slot receipt root is missing",
                &self.slot_root,
            ));
        }
        if !self.receipt.status.is_clearable() {
            return Some(AuditSlotBlocker::new(
                BlockerKind::SlotReceiptNotAccepted,
                Some(self.slot_kind),
                "slot receipt root is not live accepted",
                &self.slot_root,
            ));
        }
        None
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_receipt_slot",
            "slot_id": self.slot_id,
            "slot_kind": self.slot_kind.as_str(),
            "review_lane": self.review_lane.as_str(),
            "required": self.required,
            "clearable": self.clearable,
            "receipt": self.receipt.public_record(),
            "slot_root": self.slot_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuditSlotBlocker {
    pub blocker_id: String,
    pub blocker_kind: BlockerKind,
    pub slot_kind: Option<ReceiptSlotKind>,
    pub detail_root: String,
    pub evidence_root: String,
    pub fail_closed: bool,
}

impl AuditSlotBlocker {
    pub fn new(
        blocker_kind: BlockerKind,
        slot_kind: Option<ReceiptSlotKind>,
        detail: &str,
        evidence_root: &str,
    ) -> Self {
        let slot = match slot_kind {
            Some(kind) => kind.as_str(),
            None => "registry",
        };
        let detail_root = stable_id("wave92-blocker-detail", detail);
        Self {
            blocker_id: stable_id(
                "wave92-audit-slot-blocker",
                &format!("{}:{}:{}", blocker_kind.as_str(), slot, evidence_root),
            ),
            blocker_kind,
            slot_kind,
            detail_root,
            evidence_root: evidence_root.to_string(),
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_root("detail_root", &self.detail_root)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        if !self.fail_closed {
            return Err("blocker is not fail closed".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_audit_slot_blocker",
            "blocker_id": self.blocker_id,
            "blocker_kind": self.blocker_kind.as_str(),
            "slot_kind": self.slot_kind.map(ReceiptSlotKind::as_str),
            "detail_root": self.detail_root,
            "evidence_root": self.evidence_root,
            "fail_closed": self.fail_closed,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommandHint {
    pub hint_id: String,
    pub command_kind: OperatorCommandKind,
    pub slot_kind: Option<ReceiptSlotKind>,
    pub command_root: String,
    pub allowed_while_fail_closed: bool,
    pub requires_live_root: bool,
}

impl OperatorCommandHint {
    pub fn new(command_kind: OperatorCommandKind, slot_kind: Option<ReceiptSlotKind>) -> Self {
        let slot = match slot_kind {
            Some(kind) => kind.as_str(),
            None => "registry",
        };
        let hint_id = stable_id(
            "wave92-operator-command-hint",
            &format!("{}:{}", command_kind.as_str(), slot),
        );
        let command_root = stable_id("wave92-operator-command-root", &hint_id);
        Self {
            hint_id,
            command_kind,
            slot_kind,
            command_root,
            allowed_while_fail_closed: !matches!(command_kind, OperatorCommandKind::ClearHeavyGate),
            requires_live_root: matches!(
                command_kind,
                OperatorCommandKind::AttachReceiptRoot | OperatorCommandKind::ClearHeavyGate
            ),
        }
    }

    pub fn canonical_hints() -> Vec<Self> {
        let mut hints = vec![
            Self::new(OperatorCommandKind::HoldForceExit, None),
            Self::new(OperatorCommandKind::ImportPlanRoot, None),
            Self::new(OperatorCommandKind::ReviewBlockers, None),
            Self::new(OperatorCommandKind::KeepFailClosed, None),
            Self::new(OperatorCommandKind::ClearHeavyGate, None),
        ];
        for slot_kind in ReceiptSlotKind::all() {
            hints.push(Self::new(
                OperatorCommandKind::AttachReceiptRoot,
                Some(slot_kind),
            ));
        }
        hints
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("hint_id", &self.hint_id)?;
        ensure_root("command_root", &self.command_root)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_operator_command_hint",
            "hint_id": self.hint_id,
            "command_kind": self.command_kind.as_str(),
            "slot_kind": self.slot_kind.map(ReceiptSlotKind::as_str),
            "command_root": self.command_root,
            "allowed_while_fail_closed": self.allowed_while_fail_closed,
            "requires_live_root": self.requires_live_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistryCounters {
    pub plan_roots: u64,
    pub slots_total: u64,
    pub slots_clearable: u64,
    pub slots_empty: u64,
    pub accepted_receipt_roots: u64,
    pub blockers: u64,
    pub import_rules: u64,
    pub operator_hints: u64,
    pub raw_payload_records: u64,
}

impl RegistryCounters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_registry_counters",
            "plan_roots": self.plan_roots,
            "slots_total": self.slots_total,
            "slots_clearable": self.slots_clearable,
            "slots_empty": self.slots_empty,
            "accepted_receipt_roots": self.accepted_receipt_roots,
            "blockers": self.blockers,
            "import_rules": self.import_rules,
            "operator_hints": self.operator_hints,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FailClosedSlotRegistryVerdict {
    pub verdict_id: String,
    pub verdict_kind: VerdictKind,
    pub gate_clearable: bool,
    pub fail_closed: bool,
    pub state_root: String,
    pub slot_registry_root: String,
    pub blocker_root: String,
    pub plan_intake_root: String,
    pub counters: RegistryCounters,
}

impl FailClosedSlotRegistryVerdict {
    pub fn build(
        state_root: &str,
        slot_registry_root: &str,
        blocker_root: &str,
        plan_intake_root: &str,
        counters: RegistryCounters,
        fail_closed_armed: bool,
    ) -> Self {
        let gate_clearable = fail_closed_armed
            && counters.blockers == 0
            && counters.slots_total > 0
            && counters.slots_empty == 0
            && counters.slots_clearable == counters.slots_total;
        let verdict_kind = if gate_clearable {
            VerdictKind::Clearable
        } else {
            VerdictKind::FailClosed
        };
        Self {
            verdict_id: stable_id(
                "wave92-registry-verdict",
                &format!("{}:{}:{}", state_root, slot_registry_root, blocker_root),
            ),
            verdict_kind,
            gate_clearable,
            fail_closed: !gate_clearable,
            state_root: state_root.to_string(),
            slot_registry_root: slot_registry_root.to_string(),
            blocker_root: blocker_root.to_string(),
            plan_intake_root: plan_intake_root.to_string(),
            counters,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("verdict_id", &self.verdict_id)?;
        ensure_root("state_root", &self.state_root)?;
        ensure_root("slot_registry_root", &self.slot_registry_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        ensure_root("plan_intake_root", &self.plan_intake_root)?;
        if self.gate_clearable && self.fail_closed {
            return Err("clearable verdict cannot remain fail closed".to_string());
        }
        if !self.gate_clearable && !self.fail_closed {
            return Err("blocked verdict must remain fail closed".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_fail_closed_slot_registry_verdict",
            "verdict_id": self.verdict_id,
            "verdict_kind": self.verdict_kind.as_str(),
            "gate_clearable": self.gate_clearable,
            "fail_closed": self.fail_closed,
            "state_root": self.state_root,
            "slot_registry_root": self.slot_registry_root,
            "blocker_root": self.blocker_root,
            "plan_intake_root": self.plan_intake_root,
            "counters": self.counters.public_record(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub plan_intake: PlanRootIntake,
    pub import_rules: Vec<ImportRule>,
    pub slots: BTreeMap<String, ReceiptSlot>,
    pub operator_hints: Vec<OperatorCommandHint>,
    pub blockers: Vec<AuditSlotBlocker>,
    pub counters: RegistryCounters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let plan_intake = PlanRootIntake::from_config(&config);
        plan_intake.validate(&config)?;
        let mut slots = BTreeMap::new();
        for slot_kind in ReceiptSlotKind::all() {
            let slot = ReceiptSlot::empty(slot_kind, &plan_intake.plan_bundle_root);
            slots.insert(slot.slot_id.clone(), slot);
        }
        let mut state = Self {
            config,
            plan_intake,
            import_rules: ImportRule::canonical_rules(),
            slots,
            operator_hints: OperatorCommandHint::canonical_hints(),
            blockers: Vec::new(),
            counters: RegistryCounters::default(),
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn attach_live_accepted_receipt_root(
        &mut self,
        slot_kind: ReceiptSlotKind,
        receipt_root: &str,
        height: u64,
    ) -> Result<String> {
        self.validate_import(slot_kind, receipt_root, height)?;
        let slot_id = stable_id("wave92-receipt-slot", slot_kind.as_str());
        let slot = self
            .slots
            .get_mut(&slot_id)
            .ok_or_else(|| "receipt slot is missing".to_string())?;
        slot.attach_live_accepted_root(receipt_root, &self.plan_intake.plan_bundle_root, height)?;
        let slot_root = slot.slot_root.clone();
        self.recompute();
        self.validate()?;
        Ok(slot_root)
    }

    pub fn validate_import(
        &self,
        slot_kind: ReceiptSlotKind,
        receipt_root: &str,
        height: u64,
    ) -> Result<()> {
        ensure_root("receipt_root", receipt_root)?;
        ensure_positive("height", height)?;
        if height <= self.config.current_height {
            return Err("receipt root is not from a future live acceptance height".to_string());
        }
        if self.receipt_root_seen(receipt_root) {
            return Err("receipt root is already attached".to_string());
        }
        let slot_id = stable_id("wave92-receipt-slot", slot_kind.as_str());
        if !self.slots.contains_key(&slot_id) {
            return Err("receipt slot is not registered".to_string());
        }
        Ok(())
    }

    pub fn receipt_root_seen(&self, receipt_root: &str) -> bool {
        self.slots.values().any(|slot| {
            slot.receipt.live_root_attached && slot.receipt.receipt_root == receipt_root
        })
    }

    pub fn recompute(&mut self) {
        self.blockers = self.compute_blockers();
        self.counters = self.compute_counters();
    }

    pub fn compute_blockers(&self) -> Vec<AuditSlotBlocker> {
        let mut blockers = Vec::new();
        if !self.config.fail_closed_armed {
            blockers.push(AuditSlotBlocker::new(
                BlockerKind::RegistryFailClosedDisarmed,
                None,
                "registry fail closed is not armed",
                &self.config.root(),
            ));
        }
        if self.plan_intake.roots().len() < self.config.min_plan_roots as usize {
            blockers.push(AuditSlotBlocker::new(
                BlockerKind::SourcePlanRootMissing,
                None,
                "source plan root count is below minimum",
                &self.plan_intake.root(),
            ));
        }
        if !self.plan_intake.accepted {
            blockers.push(AuditSlotBlocker::new(
                BlockerKind::SourcePlanRootRejected,
                None,
                "source plan root intake is not accepted",
                &self.plan_intake.root(),
            ));
        }
        let mut seen = BTreeSet::new();
        for slot in self.slots.values() {
            if let Some(blocker) = slot.blocker() {
                blockers.push(blocker);
            }
            if slot.receipt.live_root_attached && !seen.insert(slot.receipt.receipt_root.clone()) {
                blockers.push(AuditSlotBlocker::new(
                    BlockerKind::SlotReceiptRootDuplicate,
                    Some(slot.slot_kind),
                    "receipt root is attached more than once",
                    &slot.slot_root,
                ));
            }
            if slot.receipt.live_root_attached
                && slot.receipt.plan_bundle_root != self.plan_intake.plan_bundle_root
            {
                blockers.push(AuditSlotBlocker::new(
                    BlockerKind::SlotPlanBindingMissing,
                    Some(slot.slot_kind),
                    "receipt root is not bound to active plan bundle root",
                    &slot.slot_root,
                ));
            }
        }
        if self.operator_hints.len() < self.config.min_operator_hints as usize {
            blockers.push(AuditSlotBlocker::new(
                BlockerKind::OperatorHintMissing,
                None,
                "operator command hint count is below minimum",
                &self.operator_hint_root(),
            ));
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            blockers.push(AuditSlotBlocker::new(
                BlockerKind::RawPayloadRejected,
                None,
                "registry contains raw payload records",
                &self.registry_root(),
            ));
        }
        blockers
    }

    pub fn compute_counters(&self) -> RegistryCounters {
        let slots_total = self.slots.len() as u64;
        let slots_clearable = self.slots.values().filter(|slot| slot.clearable).count() as u64;
        let slots_empty = self
            .slots
            .values()
            .filter(|slot| !slot.receipt.live_root_attached)
            .count() as u64;
        let accepted_receipt_roots = self
            .slots
            .values()
            .filter(|slot| slot.receipt.status == ReceiptStatus::LiveAccepted)
            .count() as u64;
        RegistryCounters {
            plan_roots: self.plan_intake.roots().len() as u64,
            slots_total,
            slots_clearable,
            slots_empty,
            accepted_receipt_roots,
            blockers: self.blockers.len() as u64,
            import_rules: self.import_rules.len() as u64,
            operator_hints: self.operator_hints.len() as u64,
            raw_payload_records: 0,
        }
    }

    pub fn registry_root(&self) -> String {
        map_root(
            "WAVE92-SLOT-REGISTRY",
            self.slots
                .iter()
                .map(|(slot_id, slot)| (slot_id.clone(), slot.public_record()))
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE92-BLOCKERS",
            self.blockers
                .iter()
                .map(AuditSlotBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn import_rule_root(&self) -> String {
        collection_root(
            "WAVE92-IMPORT-RULES",
            self.import_rules
                .iter()
                .map(ImportRule::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn operator_hint_root(&self) -> String {
        collection_root(
            "WAVE92-OPERATOR-HINTS",
            self.operator_hints
                .iter()
                .map(OperatorCommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE92-STATE",
            &json!({
                "config_root": self.config.root(),
                "plan_intake_root": self.plan_intake.root(),
                "registry_root": self.registry_root(),
                "blocker_root": self.blocker_root(),
                "import_rule_root": self.import_rule_root(),
                "operator_hint_root": self.operator_hint_root(),
                "counters": self.counters.public_record(),
            }),
        )
    }

    pub fn verdict(&self) -> FailClosedSlotRegistryVerdict {
        FailClosedSlotRegistryVerdict::build(
            &self.state_root(),
            &self.registry_root(),
            &self.blocker_root(),
            &self.plan_intake.root(),
            self.counters.clone(),
            self.config.fail_closed_armed,
        )
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.plan_intake.validate(&self.config)?;
        for rule in &self.import_rules {
            rule.validate()?;
        }
        for slot in self.slots.values() {
            slot.validate()?;
        }
        for hint in &self.operator_hints {
            hint.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        ensure_min_count(
            "receipt slots",
            self.slots.len() as u64,
            ReceiptSlotKind::all().len() as u64,
        )?;
        let counters = self.compute_counters();
        if counters.plan_roots != self.counters.plan_roots
            || counters.slots_total != self.counters.slots_total
            || counters.slots_clearable != self.counters.slots_clearable
            || counters.slots_empty != self.counters.slots_empty
            || counters.accepted_receipt_roots != self.counters.accepted_receipt_roots
            || counters.import_rules != self.counters.import_rules
            || counters.operator_hints != self.counters.operator_hints
            || counters.raw_payload_records != self.counters.raw_payload_records
        {
            return Err("registry counters do not match state".to_string());
        }
        self.verdict().validate()?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave92_live_heavy_gate_receipt_slot_registry_state",
            "config": self.config.public_record(),
            "plan_intake": self.plan_intake.public_record(),
            "import_rule_root": self.import_rule_root(),
            "operator_hint_root": self.operator_hint_root(),
            "slot_registry_root": self.registry_root(),
            "blocker_root": self.blocker_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict().public_record(),
            "slots": self.slots.values().map(ReceiptSlot::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(AuditSlotBlocker::public_record).collect::<Vec<_>>(),
        })
    }
}

pub fn devnet() -> Runtime {
    match State::new(Config::devnet()) {
        Ok(runtime) => runtime,
        Err(error) => fallback_runtime(error),
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn fallback_runtime(error: String) -> Runtime {
    let config = Config::devnet();
    let plan_intake = PlanRootIntake::from_config(&config);
    let mut slots = BTreeMap::new();
    for slot_kind in ReceiptSlotKind::all() {
        let slot = ReceiptSlot::empty(slot_kind, &plan_intake.plan_bundle_root);
        slots.insert(slot.slot_id.clone(), slot);
    }
    let mut state = State {
        config,
        plan_intake,
        import_rules: ImportRule::canonical_rules(),
        slots,
        operator_hints: OperatorCommandHint::canonical_hints(),
        blockers: vec![AuditSlotBlocker::new(
            BlockerKind::RegistryFailClosedDisarmed,
            None,
            &format!(
                "fallback runtime remained fail closed after validation error: {}",
                error
            ),
            &empty_root("fallback"),
        )],
        counters: RegistryCounters::default(),
    };
    state.counters = state.compute_counters();
    state
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{} is empty", field));
    }
    Ok(())
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        return Err(format!("{} must be positive", field));
    }
    Ok(())
}

fn ensure_min_count(field: &str, actual: u64, minimum: u64) -> Result<()> {
    if actual < minimum {
        return Err(format!("{} count is below minimum", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if !is_root(value) {
        return Err(format!("{} is not a canonical root", field));
    }
    Ok(())
}

fn is_root(value: &str) -> bool {
    value.len() >= 32 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn source_root(label: &str) -> String {
    stable_id("wave92-source-root", label)
}

fn empty_root(label: &str) -> String {
    stable_id("wave92-empty-root", label)
}

fn stable_id(domain: &str, value: &str) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(REGISTRY_SUITE),
            HashPart::Str(value),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&canonical_value(value)),
        ],
        32,
    )
}

fn string_root(domain: &str, values: &[String]) -> String {
    let leaves = values
        .iter()
        .map(|value| json!(stable_id(domain, value)))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    let leaves = values
        .iter()
        .map(|value| json!(value_root(domain, value)))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn map_root(domain: &str, values: Vec<(String, Value)>) -> String {
    let leaves = values
        .iter()
        .map(|(key, value)| {
            json!(value_root(
                domain,
                &json!({
                    "key_root": stable_id("wave92-map-key", key),
                    "value_root": value_root(domain, value),
                }),
            ))
        })
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn canonical_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(item) => item.to_string(),
        Value::Number(item) => item.to_string(),
        Value::String(item) => format!("{:?}", item),
        Value::Array(items) => {
            let inner = items
                .iter()
                .map(canonical_value)
                .collect::<Vec<_>>()
                .join(",");
            format!("[{}]", inner)
        }
        Value::Object(items) => {
            let inner = items
                .iter()
                .map(|(key, item)| format!("{:?}:{}", key, canonical_value(item)))
                .collect::<Vec<_>>()
                .join(",");
            format!("{{{}}}", inner)
        }
    }
}
