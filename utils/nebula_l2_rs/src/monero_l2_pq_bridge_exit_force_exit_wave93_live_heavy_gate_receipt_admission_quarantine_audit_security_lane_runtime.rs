use std::collections::BTreeSet;

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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave93-live-heavy-gate-receipt-admission-quarantine-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const ADMISSION_SUITE: &str = "wave93-future-receipt-root-admission-quarantine-v1";
pub const DEFAULT_WAVE: u64 = 93;
pub const DEFAULT_SOURCE_WAVE: u64 = 92;
pub const DEFAULT_HEIGHT: u64 = 4_281_616;
pub const DEFAULT_MIN_LANE_SLOTS: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneSlotKind {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
}

impl LaneSlotKind {
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
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionStatus {
    QuarantinedEmpty,
    QuarantinedRoot,
    AdmittedFutureRoot,
    RejectedRoot,
}

impl AdmissionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuarantinedEmpty => "quarantined_empty",
            Self::QuarantinedRoot => "quarantined_root",
            Self::AdmittedFutureRoot => "admitted_future_root",
            Self::RejectedRoot => "rejected_root",
        }
    }

    pub fn is_admitted(self) -> bool {
        self == Self::AdmittedFutureRoot
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionRuleKind {
    RootsOnly,
    FutureHeightOnly,
    SlotKindBound,
    SourceRegistryBound,
    UniqueReceiptRoot,
    NoRawPayload,
    ManualReviewRequired,
    FailClosedDefault,
}

impl AdmissionRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootsOnly => "roots_only",
            Self::FutureHeightOnly => "future_height_only",
            Self::SlotKindBound => "slot_kind_bound",
            Self::SourceRegistryBound => "source_registry_bound",
            Self::UniqueReceiptRoot => "unique_receipt_root",
            Self::NoRawPayload => "no_raw_payload",
            Self::ManualReviewRequired => "manual_review_required",
            Self::FailClosedDefault => "fail_closed_default",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReasonKind {
    EmptyFutureReceiptRoot,
    NotFutureHeight,
    SlotKindMismatch,
    SourceRegistryMismatch,
    DuplicateReceiptRoot,
    RawPayloadPresent,
    ReviewerSignoffMissing,
    OperatorSignoffMissing,
    FailClosedDefault,
}

impl QuarantineReasonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFutureReceiptRoot => "empty_future_receipt_root",
            Self::NotFutureHeight => "not_future_height",
            Self::SlotKindMismatch => "slot_kind_mismatch",
            Self::SourceRegistryMismatch => "source_registry_mismatch",
            Self::DuplicateReceiptRoot => "duplicate_receipt_root",
            Self::RawPayloadPresent => "raw_payload_present",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::FailClosedDefault => "fail_closed_default",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommandKind {
    HoldAdmission,
    ImportFutureReceiptRoot,
    QuarantineRoot,
    ReviewQuarantineReasons,
    AttachReviewerSignoffRoot,
    AttachOperatorSignoffRoot,
    KeepFailClosed,
}

impl OperatorCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldAdmission => "hold_admission",
            Self::ImportFutureReceiptRoot => "import_future_receipt_root",
            Self::QuarantineRoot => "quarantine_root",
            Self::ReviewQuarantineReasons => "review_quarantine_reasons",
            Self::AttachReviewerSignoffRoot => "attach_reviewer_signoff_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
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
    pub admission_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_lane_slots: u64,
    pub max_raw_payload_records: u64,
    pub source_slot_registry_root: String,
    pub source_blocker_root: String,
    pub source_operator_hint_root: String,
    pub source_fail_closed_verdict_root: String,
    pub fail_closed_armed: bool,
    pub live_receipt_claimed: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            admission_suite: ADMISSION_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_lane_slots: DEFAULT_MIN_LANE_SLOTS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            source_slot_registry_root: deterministic_root("wave92-slot-registry"),
            source_blocker_root: deterministic_root("wave92-blockers"),
            source_operator_hint_root: deterministic_root("wave92-operator-hints"),
            source_fail_closed_verdict_root: deterministic_root("wave92-fail-closed-verdict"),
            fail_closed_armed: true,
            live_receipt_claimed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("admission_suite", &self.admission_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_lane_slots", self.min_lane_slots)?;
        ensure_root("source_slot_registry_root", &self.source_slot_registry_root)?;
        ensure_root("source_blocker_root", &self.source_blocker_root)?;
        ensure_root("source_operator_hint_root", &self.source_operator_hint_root)?;
        ensure_root(
            "source_fail_closed_verdict_root",
            &self.source_fail_closed_verdict_root,
        )?;
        if !self.fail_closed_armed {
            return Err("admission lane fail closed is not armed".to_string());
        }
        if self.live_receipt_claimed {
            return Err("wave93 admission lane cannot claim a live receipt".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave93 admission lane cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_receipt_root_admission_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "admission_suite": self.admission_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_lane_slots": self.min_lane_slots,
            "max_raw_payload_records": self.max_raw_payload_records,
            "source_slot_registry_root": self.source_slot_registry_root,
            "source_blocker_root": self.source_blocker_root,
            "source_operator_hint_root": self.source_operator_hint_root,
            "source_fail_closed_verdict_root": self.source_fail_closed_verdict_root,
            "fail_closed_armed": self.fail_closed_armed,
            "live_receipt_claimed": self.live_receipt_claimed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE93-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionRule {
    pub rule_id: String,
    pub kind: AdmissionRuleKind,
    pub slot_kind: Option<LaneSlotKind>,
    pub required: bool,
    pub rule_root: String,
}

impl AdmissionRule {
    pub fn new(kind: AdmissionRuleKind, slot_kind: Option<LaneSlotKind>) -> Self {
        let slot = match slot_kind {
            Some(kind) => kind.as_str(),
            None => "lane",
        };
        let rule_id = stable_id(
            "wave93-admission-rule",
            &format!("{}:{}", kind.as_str(), slot),
        );
        let mut rule = Self {
            rule_id,
            kind,
            slot_kind,
            required: true,
            rule_root: String::new(),
        };
        rule.rule_root = rule.compute_root();
        rule
    }

    pub fn canonical_rules() -> Vec<Self> {
        let mut rules = vec![
            Self::new(AdmissionRuleKind::RootsOnly, None),
            Self::new(AdmissionRuleKind::FutureHeightOnly, None),
            Self::new(AdmissionRuleKind::SourceRegistryBound, None),
            Self::new(AdmissionRuleKind::UniqueReceiptRoot, None),
            Self::new(AdmissionRuleKind::NoRawPayload, None),
            Self::new(AdmissionRuleKind::ManualReviewRequired, None),
            Self::new(AdmissionRuleKind::FailClosedDefault, None),
        ];
        for slot_kind in LaneSlotKind::all() {
            rules.push(Self::new(AdmissionRuleKind::SlotKindBound, Some(slot_kind)));
        }
        rules
    }

    pub fn compute_root(&self) -> String {
        stable_id(
            "wave93-admission-rule-root",
            &format!(
                "{}:{}:{}:{}",
                self.rule_id,
                self.kind.as_str(),
                match self.slot_kind {
                    Some(kind) => kind.as_str(),
                    None => "lane",
                },
                self.required
            ),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("rule_id", &self.rule_id)?;
        ensure_root("rule_root", &self.rule_root)?;
        if self.rule_root != self.compute_root() {
            return Err("admission rule root does not match rule".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "rule_id": self.rule_id,
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.map(LaneSlotKind::as_str),
            "required": self.required,
            "rule_root": self.rule_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneReceiptRoot {
    pub slot_kind: LaneSlotKind,
    pub receipt_root: String,
    pub source_registry_root: String,
    pub imported_at_height: u64,
    pub status: AdmissionStatus,
    pub review_root: String,
    pub quarantine_root: String,
}

impl LaneReceiptRoot {
    pub fn empty(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let quarantine_root = quarantine_root(
            slot_kind,
            QuarantineReasonKind::EmptyFutureReceiptRoot,
            &config.source_slot_registry_root,
        );
        Self {
            slot_kind,
            receipt_root: empty_root(slot_kind.as_str()),
            source_registry_root: config.source_slot_registry_root.clone(),
            imported_at_height: config.current_height,
            status: AdmissionStatus::QuarantinedEmpty,
            review_root: deterministic_root(&format!("wave93-review-{}", slot_kind.as_str())),
            quarantine_root,
        }
    }

    pub fn with_future_root(
        slot_kind: LaneSlotKind,
        receipt_root: &str,
        imported_at_height: u64,
        source_registry_root: &str,
        review_root: &str,
    ) -> Self {
        Self {
            slot_kind,
            receipt_root: receipt_root.to_string(),
            source_registry_root: source_registry_root.to_string(),
            imported_at_height,
            status: AdmissionStatus::QuarantinedRoot,
            review_root: review_root.to_string(),
            quarantine_root: quarantine_root(
                slot_kind,
                QuarantineReasonKind::ReviewerSignoffMissing,
                receipt_root,
            ),
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("receipt_root", &self.receipt_root)?;
        ensure_root("source_registry_root", &self.source_registry_root)?;
        ensure_positive("imported_at_height", self.imported_at_height)?;
        ensure_root("review_root", &self.review_root)?;
        ensure_root("quarantine_root", &self.quarantine_root)?;
        if self.status.is_admitted() && self.imported_at_height <= config.current_height {
            return Err("admitted receipt root is not from a future height".to_string());
        }
        if self.status.is_admitted()
            && self.source_registry_root != config.source_slot_registry_root
        {
            return Err("admitted receipt root is not bound to source registry".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "receipt_root": self.receipt_root,
            "source_registry_root": self.source_registry_root,
            "imported_at_height": self.imported_at_height,
            "status": self.status.as_str(),
            "review_root": self.review_root,
            "quarantine_root": self.quarantine_root,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE93-LANE-RECEIPT-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuarantineReason {
    pub reason_id: String,
    pub kind: QuarantineReasonKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocking: bool,
    pub reason_root: String,
}

impl QuarantineReason {
    pub fn new(kind: QuarantineReasonKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        let reason_id = stable_id(
            "wave93-quarantine-reason",
            &format!("{}:{}:{}", kind.as_str(), slot_kind.as_str(), evidence_root),
        );
        let mut reason = Self {
            reason_id,
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocking: true,
            reason_root: String::new(),
        };
        reason.reason_root = reason.compute_root();
        reason
    }

    pub fn compute_root(&self) -> String {
        stable_id(
            "wave93-quarantine-reason-root",
            &format!(
                "{}:{}:{}:{}:{}",
                self.reason_id,
                self.kind.as_str(),
                self.slot_kind.as_str(),
                self.evidence_root,
                self.blocking
            ),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("reason_id", &self.reason_id)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("reason_root", &self.reason_root)?;
        if self.reason_root != self.compute_root() {
            return Err("quarantine reason root does not match reason".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "reason_id": self.reason_id,
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "blocking": self.blocking,
            "reason_root": self.reason_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommand {
    pub command_id: String,
    pub kind: OperatorCommandKind,
    pub target_root: String,
    pub requires_reviewer_signoff: bool,
    pub requires_operator_signoff: bool,
    pub command_root: String,
}

impl OperatorCommand {
    pub fn new(kind: OperatorCommandKind, target_root: &str) -> Self {
        let command_id = stable_id(
            "wave93-operator-command",
            &format!("{}:{}", kind.as_str(), target_root),
        );
        let mut command = Self {
            command_id,
            kind,
            target_root: target_root.to_string(),
            requires_reviewer_signoff: matches!(
                kind,
                OperatorCommandKind::ImportFutureReceiptRoot
                    | OperatorCommandKind::QuarantineRoot
                    | OperatorCommandKind::AttachReviewerSignoffRoot
            ),
            requires_operator_signoff: matches!(
                kind,
                OperatorCommandKind::ImportFutureReceiptRoot
                    | OperatorCommandKind::AttachOperatorSignoffRoot
            ),
            command_root: String::new(),
        };
        command.command_root = command.compute_root();
        command
    }

    pub fn canonical_commands(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                OperatorCommandKind::HoldAdmission,
                &config.source_slot_registry_root,
            ),
            Self::new(
                OperatorCommandKind::ImportFutureReceiptRoot,
                &config.source_slot_registry_root,
            ),
            Self::new(
                OperatorCommandKind::QuarantineRoot,
                &config.source_blocker_root,
            ),
            Self::new(
                OperatorCommandKind::ReviewQuarantineReasons,
                &config.source_blocker_root,
            ),
            Self::new(
                OperatorCommandKind::AttachReviewerSignoffRoot,
                &config.source_operator_hint_root,
            ),
            Self::new(
                OperatorCommandKind::AttachOperatorSignoffRoot,
                &config.source_operator_hint_root,
            ),
            Self::new(
                OperatorCommandKind::KeepFailClosed,
                &config.source_fail_closed_verdict_root,
            ),
        ]
    }

    pub fn compute_root(&self) -> String {
        stable_id(
            "wave93-operator-command-root",
            &format!(
                "{}:{}:{}:{}:{}",
                self.command_id,
                self.kind.as_str(),
                self.target_root,
                self.requires_reviewer_signoff,
                self.requires_operator_signoff
            ),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("command_id", &self.command_id)?;
        ensure_root("target_root", &self.target_root)?;
        ensure_root("command_root", &self.command_root)?;
        if self.command_root != self.compute_root() {
            return Err("operator command root does not match command".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "command_id": self.command_id,
            "kind": self.kind.as_str(),
            "target_root": self.target_root,
            "requires_reviewer_signoff": self.requires_reviewer_signoff,
            "requires_operator_signoff": self.requires_operator_signoff,
            "command_root": self.command_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneCounters {
    pub lane_slots: u64,
    pub empty_future_receipt_roots: u64,
    pub quarantined_roots: u64,
    pub admitted_roots: u64,
    pub rejected_roots: u64,
    pub quarantine_reasons: u64,
    pub admission_rules: u64,
    pub operator_commands: u64,
    pub raw_payload_records: u64,
}

impl LaneCounters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane_slots": self.lane_slots,
            "empty_future_receipt_roots": self.empty_future_receipt_roots,
            "quarantined_roots": self.quarantined_roots,
            "admitted_roots": self.admitted_roots,
            "rejected_roots": self.rejected_roots,
            "quarantine_reasons": self.quarantine_reasons,
            "admission_rules": self.admission_rules,
            "operator_commands": self.operator_commands,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub admission_rules: Vec<AdmissionRule>,
    pub lane_roots: Vec<LaneReceiptRoot>,
    pub quarantine_reasons: Vec<QuarantineReason>,
    pub operator_commands: Vec<OperatorCommand>,
    pub counters: LaneCounters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let lane_roots = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| LaneReceiptRoot::empty(slot_kind, &config))
            .collect::<Vec<_>>();
        let mut state = Self {
            admission_rules: AdmissionRule::canonical_rules(),
            operator_commands: OperatorCommand::canonical_commands(&config),
            quarantine_reasons: Vec::new(),
            counters: LaneCounters::default(),
            lane_roots,
            config,
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn admit_future_root(
        &mut self,
        slot_kind: LaneSlotKind,
        receipt_root: &str,
        imported_at_height: u64,
        review_root: &str,
        reviewer_signoff_root: &str,
        operator_signoff_root: &str,
    ) -> Result<String> {
        self.validate_future_root(
            slot_kind,
            receipt_root,
            imported_at_height,
            review_root,
            reviewer_signoff_root,
            operator_signoff_root,
        )?;
        let source_root = self.config.source_slot_registry_root.clone();
        for lane_root in &mut self.lane_roots {
            if lane_root.slot_kind == slot_kind {
                *lane_root = LaneReceiptRoot::with_future_root(
                    slot_kind,
                    receipt_root,
                    imported_at_height,
                    &source_root,
                    review_root,
                );
                lane_root.status = AdmissionStatus::AdmittedFutureRoot;
                lane_root.quarantine_root = empty_root("wave93-admitted-no-quarantine");
            }
        }
        self.recompute();
        self.validate()?;
        Ok(self.admission_root())
    }

    pub fn quarantine_future_root(
        &mut self,
        slot_kind: LaneSlotKind,
        receipt_root: &str,
        imported_at_height: u64,
        reason: QuarantineReasonKind,
    ) -> Result<String> {
        ensure_root("receipt_root", receipt_root)?;
        ensure_positive("imported_at_height", imported_at_height)?;
        let source_root = self.config.source_slot_registry_root.clone();
        for lane_root in &mut self.lane_roots {
            if lane_root.slot_kind == slot_kind {
                *lane_root = LaneReceiptRoot::with_future_root(
                    slot_kind,
                    receipt_root,
                    imported_at_height,
                    &source_root,
                    &deterministic_root(&format!("wave93-review-{}", slot_kind.as_str())),
                );
                lane_root.status = AdmissionStatus::QuarantinedRoot;
                lane_root.quarantine_root = quarantine_root(slot_kind, reason, receipt_root);
            }
        }
        self.recompute();
        self.validate()?;
        Ok(self.quarantine_root())
    }

    pub fn validate_future_root(
        &self,
        slot_kind: LaneSlotKind,
        receipt_root: &str,
        imported_at_height: u64,
        review_root: &str,
        reviewer_signoff_root: &str,
        operator_signoff_root: &str,
    ) -> Result<()> {
        ensure_root("receipt_root", receipt_root)?;
        ensure_root("review_root", review_root)?;
        ensure_root("reviewer_signoff_root", reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", operator_signoff_root)?;
        ensure_positive("imported_at_height", imported_at_height)?;
        if imported_at_height <= self.config.current_height {
            return Err("receipt root import height is not future bound".to_string());
        }
        if self.receipt_root_seen(receipt_root) {
            return Err("receipt root already exists in admission lane".to_string());
        }
        if !LaneSlotKind::all().contains(&slot_kind) {
            return Err("slot kind is not admitted by lane".to_string());
        }
        Ok(())
    }

    pub fn receipt_root_seen(&self, receipt_root: &str) -> bool {
        self.lane_roots.iter().any(|lane_root| {
            lane_root.receipt_root == receipt_root && lane_root.status.is_admitted()
        })
    }

    pub fn recompute(&mut self) {
        self.quarantine_reasons = self.compute_quarantine_reasons();
        self.counters = self.compute_counters();
    }

    pub fn compute_quarantine_reasons(&self) -> Vec<QuarantineReason> {
        let mut reasons = Vec::new();
        let mut seen = BTreeSet::new();
        for lane_root in &self.lane_roots {
            if lane_root.status == AdmissionStatus::QuarantinedEmpty {
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::EmptyFutureReceiptRoot,
                    lane_root.slot_kind,
                    &lane_root.quarantine_root,
                ));
            }
            if lane_root.status == AdmissionStatus::QuarantinedRoot {
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::ReviewerSignoffMissing,
                    lane_root.slot_kind,
                    &lane_root.quarantine_root,
                ));
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::OperatorSignoffMissing,
                    lane_root.slot_kind,
                    &lane_root.quarantine_root,
                ));
            }
            if lane_root.imported_at_height <= self.config.current_height
                && lane_root.status != AdmissionStatus::QuarantinedEmpty
            {
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::NotFutureHeight,
                    lane_root.slot_kind,
                    &lane_root.state_root(),
                ));
            }
            if lane_root.source_registry_root != self.config.source_slot_registry_root {
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::SourceRegistryMismatch,
                    lane_root.slot_kind,
                    &lane_root.state_root(),
                ));
            }
            if !seen.insert(lane_root.receipt_root.clone())
                && lane_root.status != AdmissionStatus::QuarantinedEmpty
            {
                reasons.push(QuarantineReason::new(
                    QuarantineReasonKind::DuplicateReceiptRoot,
                    lane_root.slot_kind,
                    &lane_root.state_root(),
                ));
            }
        }
        if !self.config.fail_closed_armed {
            reasons.push(QuarantineReason::new(
                QuarantineReasonKind::FailClosedDefault,
                LaneSlotKind::AuditReview,
                &self.config.state_root(),
            ));
        }
        reasons
    }

    pub fn compute_counters(&self) -> LaneCounters {
        LaneCounters {
            lane_slots: self.lane_roots.len() as u64,
            empty_future_receipt_roots: self
                .lane_roots
                .iter()
                .filter(|root| root.status == AdmissionStatus::QuarantinedEmpty)
                .count() as u64,
            quarantined_roots: self
                .lane_roots
                .iter()
                .filter(|root| root.status == AdmissionStatus::QuarantinedRoot)
                .count() as u64,
            admitted_roots: self
                .lane_roots
                .iter()
                .filter(|root| root.status == AdmissionStatus::AdmittedFutureRoot)
                .count() as u64,
            rejected_roots: self
                .lane_roots
                .iter()
                .filter(|root| root.status == AdmissionStatus::RejectedRoot)
                .count() as u64,
            quarantine_reasons: self.quarantine_reasons.len() as u64,
            admission_rules: self.admission_rules.len() as u64,
            operator_commands: self.operator_commands.len() as u64,
            raw_payload_records: 0,
        }
    }

    pub fn admission_root(&self) -> String {
        collection_root(
            "WAVE93-ADMISSION-LANE-ROOTS",
            self.lane_roots
                .iter()
                .map(LaneReceiptRoot::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn quarantine_root(&self) -> String {
        collection_root(
            "WAVE93-QUARANTINE-REASONS",
            self.quarantine_reasons
                .iter()
                .map(QuarantineReason::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn admission_rule_root(&self) -> String {
        collection_root(
            "WAVE93-ADMISSION-RULES",
            self.admission_rules
                .iter()
                .map(AdmissionRule::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn operator_command_root(&self) -> String {
        collection_root(
            "WAVE93-OPERATOR-COMMANDS",
            self.operator_commands
                .iter()
                .map(OperatorCommand::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE93-STATE",
            &json!({
                "config_root": self.config.state_root(),
                "admission_rule_root": self.admission_rule_root(),
                "admission_root": self.admission_root(),
                "quarantine_root": self.quarantine_root(),
                "operator_command_root": self.operator_command_root(),
                "counters": self.counters.public_record(),
                "live_receipt_claimed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        for rule in &self.admission_rules {
            rule.validate()?;
        }
        for lane_root in &self.lane_roots {
            lane_root.validate(&self.config)?;
        }
        for reason in &self.quarantine_reasons {
            reason.validate()?;
        }
        for command in &self.operator_commands {
            command.validate()?;
        }
        ensure_min_count(
            "lane slots",
            self.lane_roots.len() as u64,
            self.config.min_lane_slots,
        )?;
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("admission lane contains raw payload records".to_string());
        }
        let counters = self.compute_counters();
        if counters != self.counters {
            return Err("admission lane counters do not match state".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_live_heavy_gate_receipt_admission_quarantine_audit_security_lane_state",
            "config": self.config.public_record(),
            "admission_rule_root": self.admission_rule_root(),
            "admission_root": self.admission_root(),
            "quarantine_root": self.quarantine_root(),
            "operator_command_root": self.operator_command_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "live_receipt_claimed": false,
            "heavy_gates_ran": false,
            "lane_roots": self.lane_roots.iter().map(LaneReceiptRoot::public_record).collect::<Vec<_>>(),
            "quarantine_reasons": self.quarantine_reasons.iter().map(QuarantineReason::public_record).collect::<Vec<_>>(),
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
    let lane_roots = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| LaneReceiptRoot::empty(slot_kind, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        admission_rules: AdmissionRule::canonical_rules(),
        operator_commands: OperatorCommand::canonical_commands(&config),
        quarantine_reasons: vec![QuarantineReason::new(
            QuarantineReasonKind::FailClosedDefault,
            LaneSlotKind::AuditReview,
            &value_root("WAVE93-FALLBACK-ERROR", &json!({"error": error})),
        )],
        counters: LaneCounters::default(),
        lane_roots,
        config,
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
        return Err(format!("{} is below required minimum", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 {
        return Err(format!("{} is not a root", field));
    }
    Ok(())
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE93-STABLE-ID",
        &[HashPart::Str(domain), HashPart::Str(label)],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE93-DETERMINISTIC-ROOT",
        &[HashPart::Str(label)],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE93-EMPTY-ROOT",
        &[HashPart::Str(label)],
        32,
    )
}

fn quarantine_root(
    slot_kind: LaneSlotKind,
    reason: QuarantineReasonKind,
    evidence_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE93-QUARANTINE-ROOT",
        &[
            HashPart::Str(slot_kind.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(evidence_root),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(domain, &[HashPart::Json(value)], 32)
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    merkle_root(domain, &values)
}
