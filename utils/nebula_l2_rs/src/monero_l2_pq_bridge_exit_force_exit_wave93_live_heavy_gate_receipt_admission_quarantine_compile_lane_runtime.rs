use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave93-live-heavy-gate-receipt-admission-quarantine-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave93";
pub const PRIOR_WAVE_ID: &str = "wave92";
pub const LANE_ID: &str = "wave93-live-heavy-gate-receipt-admission-quarantine-compile-lane";
pub const SOURCE_SLOT_LANE_ID: &str = "wave92-live-heavy-gate-receipt-slot-registry-compile-slot";
pub const DEFAULT_MIN_SOURCE_ROOTS: usize = 1;
pub const DEFAULT_MIN_OPERATOR_APPROVAL_ROOTS: usize = 2;
pub const DEFAULT_MIN_RECEIPT_ROOT_CHARS: usize = 16;
pub const DEFAULT_MAX_RECEIPT_ROOTS: usize = 32;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompileGate {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl CompileGate {
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
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionMode {
    QuarantineOnly,
    AdmitRootsAfterOperatorApproval,
}

impl AdmissionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuarantineOnly => "quarantine_only",
            Self::AdmitRootsAfterOperatorApproval => "admit_roots_after_operator_approval",
        }
    }

    pub fn can_admit(self) -> bool {
        matches!(self, Self::AdmitRootsAfterOperatorApproval)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptRootStatus {
    Empty,
    Quarantined,
    Admitted,
    Rejected,
}

impl ReceiptRootStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Quarantined => "quarantined",
            Self::Admitted => "admitted",
            Self::Rejected => "rejected",
        }
    }

    pub fn clears_quarantine(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    DefaultFailClosedFutureReceiptRoot,
    EmptyReceiptRoot,
    ReceiptRootTooShort,
    SourceSlotRootMissing,
    AdmissionRuleClosed,
    OperatorApprovalQuorumMissing,
    DuplicateReceiptRoot,
    ReceiptRootCapacityReached,
    PrivacyBoundaryRootsOnly,
    HeavyGateNotRunInThisLane,
}

impl QuarantineReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosedFutureReceiptRoot => "default_fail_closed_future_receipt_root",
            Self::EmptyReceiptRoot => "empty_receipt_root",
            Self::ReceiptRootTooShort => "receipt_root_too_short",
            Self::SourceSlotRootMissing => "source_slot_root_missing",
            Self::AdmissionRuleClosed => "admission_rule_closed",
            Self::OperatorApprovalQuorumMissing => "operator_approval_quorum_missing",
            Self::DuplicateReceiptRoot => "duplicate_receipt_root",
            Self::ReceiptRootCapacityReached => "receipt_root_capacity_reached",
            Self::PrivacyBoundaryRootsOnly => "privacy_boundary_roots_only",
            Self::HeavyGateNotRunInThisLane => "heavy_gate_not_run_in_this_lane",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    Quarantined,
    ReadyForReview,
    Admitted,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Quarantined => "quarantined",
            Self::ReadyForReview => "ready_for_review",
            Self::Admitted => "admitted",
        }
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
    pub source_slot_lane_id: String,
    pub default_admission_mode: AdmissionMode,
    pub min_source_roots: usize,
    pub min_operator_approval_roots: usize,
    pub min_receipt_root_chars: usize,
    pub max_receipt_roots: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            source_slot_lane_id: SOURCE_SLOT_LANE_ID.to_string(),
            default_admission_mode: AdmissionMode::QuarantineOnly,
            min_source_roots: DEFAULT_MIN_SOURCE_ROOTS,
            min_operator_approval_roots: DEFAULT_MIN_OPERATOR_APPROVAL_ROOTS,
            min_receipt_root_chars: DEFAULT_MIN_RECEIPT_ROOT_CHARS,
            max_receipt_roots: DEFAULT_MAX_RECEIPT_ROOTS,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_id": self.wave_id,
            "prior_wave_id": self.prior_wave_id,
            "lane_id": self.lane_id,
            "source_slot_lane_id": self.source_slot_lane_id,
            "default_admission_mode": self.default_admission_mode.as_str(),
            "min_source_roots": self.min_source_roots,
            "min_operator_approval_roots": self.min_operator_approval_roots,
            "min_receipt_root_chars": self.min_receipt_root_chars,
            "max_receipt_roots": self.max_receipt_roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceSlotRoot {
    pub gate: CompileGate,
    pub source_wave_id: String,
    pub source_lane_id: String,
    pub slot_root: String,
    pub imported: bool,
}

impl SourceSlotRoot {
    pub fn empty(gate: CompileGate) -> Self {
        Self {
            gate,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: SOURCE_SLOT_LANE_ID.to_string(),
            slot_root: empty_root("source_slot_root"),
            imported: false,
        }
    }

    pub fn imported(gate: CompileGate, slot_root: impl Into<String>) -> Self {
        Self {
            gate,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: SOURCE_SLOT_LANE_ID.to_string(),
            slot_root: slot_root.into(),
            imported: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "source_wave_id": self.source_wave_id,
            "source_lane_id": self.source_lane_id,
            "slot_root": self.slot_root,
            "imported": self.imported,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source_slot_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionRule {
    pub gate: CompileGate,
    pub mode: AdmissionMode,
    pub requires_source_slot_root: bool,
    pub requires_operator_approval_quorum: bool,
    pub roots_only_public_record: bool,
}

impl AdmissionRule {
    pub fn quarantine_only(gate: CompileGate) -> Self {
        Self {
            gate,
            mode: AdmissionMode::QuarantineOnly,
            requires_source_slot_root: true,
            requires_operator_approval_quorum: true,
            roots_only_public_record: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "mode": self.mode.as_str(),
            "requires_source_slot_root": self.requires_source_slot_root,
            "requires_operator_approval_quorum": self.requires_operator_approval_quorum,
            "roots_only_public_record": self.roots_only_public_record,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission_rule", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuarantineEntry {
    pub gate: CompileGate,
    pub receipt_root: String,
    pub status: ReceiptRootStatus,
    pub reason: QuarantineReason,
    pub source_slot_root: String,
    pub operator_approval_root: String,
}

impl QuarantineEntry {
    pub fn empty(gate: CompileGate) -> Self {
        Self {
            gate,
            receipt_root: empty_root(gate.as_str()),
            status: ReceiptRootStatus::Quarantined,
            reason: QuarantineReason::DefaultFailClosedFutureReceiptRoot,
            source_slot_root: empty_root("source_slot_root"),
            operator_approval_root: empty_root("operator_approval"),
        }
    }

    pub fn new(
        gate: CompileGate,
        receipt_root: impl Into<String>,
        status: ReceiptRootStatus,
        reason: QuarantineReason,
        source_slot_root: impl Into<String>,
        operator_approval_root: impl Into<String>,
    ) -> Self {
        Self {
            gate,
            receipt_root: receipt_root.into(),
            status,
            reason,
            source_slot_root: source_slot_root.into(),
            operator_approval_root: operator_approval_root.into(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "receipt_root": self.receipt_root,
            "status": self.status.as_str(),
            "reason": self.reason.as_str(),
            "source_slot_root": self.source_slot_root,
            "operator_approval_root": self.operator_approval_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("quarantine_entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommand {
    pub gate: CompileGate,
    pub command_id: String,
    pub command_root: String,
    pub capture_rule_root: String,
    pub refusal_root: String,
}

impl OperatorCommand {
    pub fn new(gate: CompileGate, command_id: impl Into<String>, command_text: &str) -> Self {
        let command_id = command_id.into();
        Self {
            gate,
            command_id,
            command_root: record_root(
                "operator_command_text",
                &json!({
                    "gate": gate.as_str(),
                    "command_text": command_text,
                }),
            ),
            capture_rule_root: record_root(
                "operator_command_capture_rule",
                &json!({
                    "gate": gate.as_str(),
                    "rule": "operator_may_later_seal_status_stdout_stderr_artifact_and_note_roots",
                }),
            ),
            refusal_root: record_root(
                "operator_command_refusal",
                &json!({
                    "gate": gate.as_str(),
                    "rule": "this_runtime_records_future_roots_only_and_does_not_run_commands",
                }),
            ),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "command_id": self.command_id,
            "command_root": self.command_root,
            "capture_rule_root": self.capture_rule_root,
            "refusal_root": self.refusal_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator_command", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionVerdict {
    pub verdict: RuntimeVerdict,
    pub admitted_count: usize,
    pub quarantined_count: usize,
    pub rejected_count: usize,
    pub reason_roots: Vec<String>,
}

impl AdmissionVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "admitted_count": self.admitted_count,
            "quarantined_count": self.quarantined_count,
            "rejected_count": self.rejected_count,
            "reason_roots": self.reason_roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission_verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub source_slot_root: String,
    pub admission_rule_root: String,
    pub quarantine_root: String,
    pub admitted_receipt_root: String,
    pub operator_command_root: String,
    pub operator_approval_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "source_slot_root": self.source_slot_root,
            "admission_rule_root": self.admission_rule_root,
            "quarantine_root": self.quarantine_root,
            "admitted_receipt_root": self.admitted_receipt_root,
            "operator_command_root": self.operator_command_root,
            "operator_approval_root": self.operator_approval_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source_slot_roots: BTreeMap<CompileGate, SourceSlotRoot>,
    pub admission_rules: BTreeMap<CompileGate, AdmissionRule>,
    pub quarantined_receipts: Vec<QuarantineEntry>,
    pub admitted_receipt_roots: BTreeMap<CompileGate, Vec<String>>,
    pub operator_commands: Vec<OperatorCommand>,
    pub operator_approval_roots: Vec<String>,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut source_slot_roots = BTreeMap::new();
        let mut admission_rules = BTreeMap::new();
        let mut admitted_receipt_roots = BTreeMap::new();
        let mut quarantined_receipts = Vec::new();
        for gate in CompileGate::all() {
            source_slot_roots.insert(gate, SourceSlotRoot::empty(gate));
            admission_rules.insert(gate, AdmissionRule::quarantine_only(gate));
            admitted_receipt_roots.insert(gate, Vec::new());
            quarantined_receipts.push(QuarantineEntry::empty(gate));
        }
        Self {
            config,
            source_slot_roots,
            admission_rules,
            quarantined_receipts,
            admitted_receipt_roots,
            operator_commands: default_operator_commands(),
            operator_approval_roots: Vec::new(),
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet())
    }

    pub fn import_source_slot_root(&mut self, source: SourceSlotRoot) -> Result<()> {
        self.source_slot_roots.insert(source.gate, source);
        Ok(())
    }

    pub fn add_operator_approval_root(&mut self, approval_root: impl Into<String>) -> Result<()> {
        let approval_root = approval_root.into();
        if approval_root.len() < self.config.min_receipt_root_chars {
            return Err("operator approval root too short".to_string());
        }
        if self.operator_approval_roots.contains(&approval_root) {
            return Err("duplicate operator approval root".to_string());
        }
        self.operator_approval_roots.push(approval_root);
        Ok(())
    }

    pub fn set_admission_rule(&mut self, rule: AdmissionRule) -> Result<()> {
        self.admission_rules.insert(rule.gate, rule);
        Ok(())
    }

    pub fn submit_future_receipt_root(
        &mut self,
        gate: CompileGate,
        receipt_root: impl Into<String>,
    ) -> Result<()> {
        let receipt_root = receipt_root.into();
        let source_slot_root = self.source_slot_root_for(gate);
        let operator_approval_root = self.operator_approval_root();
        let reason = self.quarantine_reason(gate, &receipt_root);
        if reason != QuarantineReason::HeavyGateNotRunInThisLane {
            self.quarantined_receipts.push(QuarantineEntry::new(
                gate,
                receipt_root,
                ReceiptRootStatus::Quarantined,
                reason,
                source_slot_root,
                operator_approval_root,
            ));
            return Err(reason.as_str().to_string());
        }
        let admitted = self
            .admitted_receipt_roots
            .entry(gate)
            .or_insert_with(Vec::new);
        admitted.push(receipt_root);
        Ok(())
    }

    pub fn imported_source_root_count(&self) -> usize {
        self.source_slot_roots
            .values()
            .filter(|root| root.imported)
            .count()
    }

    pub fn admitted_count(&self) -> usize {
        self.admitted_receipt_roots
            .values()
            .map(Vec::len)
            .sum::<usize>()
    }

    pub fn rejected_count(&self) -> usize {
        self.quarantined_receipts
            .iter()
            .filter(|entry| entry.status == ReceiptRootStatus::Rejected)
            .count()
    }

    pub fn verdict(&self) -> AdmissionVerdict {
        let admitted_count = self.admitted_count();
        let rejected_count = self.rejected_count();
        let quarantined_count = self.quarantined_receipts.len();
        let reason_roots = self.quarantine_reason_roots();
        let verdict = if admitted_count > 0 && reason_roots.is_empty() {
            RuntimeVerdict::Admitted
        } else if self.imported_source_root_count() >= self.config.min_source_roots
            && self.operator_approval_roots.len() >= self.config.min_operator_approval_roots
        {
            RuntimeVerdict::ReadyForReview
        } else if quarantined_count > 0 {
            RuntimeVerdict::Quarantined
        } else {
            RuntimeVerdict::FailClosed
        };
        AdmissionVerdict {
            verdict,
            admitted_count,
            quarantined_count,
            rejected_count,
            reason_roots,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        public_record_from_state(self)
    }

    pub fn state_root(&self) -> String {
        self.public_record().state_root
    }

    fn source_slot_root_for(&self, gate: CompileGate) -> String {
        self.source_slot_roots
            .get(&gate)
            .map(SourceSlotRoot::state_root)
            .map_or_else(|| empty_root("source_slot_root"), |root| root)
    }

    fn operator_approval_root(&self) -> String {
        string_list_root("operator_approval_roots", &self.operator_approval_roots)
    }

    fn quarantine_reason(&self, gate: CompileGate, receipt_root: &str) -> QuarantineReason {
        let rule_closed = match self.admission_rules.get(&gate) {
            Some(rule) => !rule.mode.can_admit(),
            None => true,
        };
        if receipt_root.is_empty() {
            return QuarantineReason::EmptyReceiptRoot;
        }
        if receipt_root.len() < self.config.min_receipt_root_chars {
            return QuarantineReason::ReceiptRootTooShort;
        }
        if self.imported_source_root_count() < self.config.min_source_roots {
            return QuarantineReason::SourceSlotRootMissing;
        }
        if rule_closed {
            return QuarantineReason::AdmissionRuleClosed;
        }
        if self.operator_approval_roots.len() < self.config.min_operator_approval_roots {
            return QuarantineReason::OperatorApprovalQuorumMissing;
        }
        if self.receipt_root_exists(receipt_root) {
            return QuarantineReason::DuplicateReceiptRoot;
        }
        if self.admitted_count() >= self.config.max_receipt_roots {
            return QuarantineReason::ReceiptRootCapacityReached;
        }
        QuarantineReason::HeavyGateNotRunInThisLane
    }

    fn receipt_root_exists(&self, receipt_root: &str) -> bool {
        self.admitted_receipt_roots
            .values()
            .any(|roots| roots.iter().any(|root| root == receipt_root))
            || self
                .quarantined_receipts
                .iter()
                .any(|entry| entry.receipt_root == receipt_root)
    }

    fn quarantine_reason_roots(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut roots = Vec::new();
        for entry in &self.quarantined_receipts {
            let root = record_root(
                "quarantine_reason",
                &json!({
                    "gate": entry.gate.as_str(),
                    "reason": entry.reason.as_str(),
                    "receipt_root": entry.receipt_root,
                }),
            );
            if seen.insert(root.clone()) {
                roots.push(root);
            }
        }
        roots
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

pub fn default_operator_commands() -> Vec<OperatorCommand> {
    vec![
        OperatorCommand::new(
            CompileGate::CargoCheck,
            "cargo_check_receipt_root_capture",
            "cargo check --workspace --all-targets",
        ),
        OperatorCommand::new(
            CompileGate::CargoTest,
            "cargo_test_receipt_root_capture",
            "cargo test --workspace --all-targets",
        ),
        OperatorCommand::new(
            CompileGate::Clippy,
            "clippy_receipt_root_capture",
            "cargo clippy --workspace --all-targets -- -D warnings",
        ),
        OperatorCommand::new(
            CompileGate::Rustfmt,
            "rustfmt_receipt_root_capture",
            "cargo fmt --all --check",
        ),
        OperatorCommand::new(
            CompileGate::Rustc,
            "rustc_receipt_root_capture",
            "rustc --version --verbose",
        ),
        OperatorCommand::new(
            CompileGate::BuildMetadata,
            "build_metadata_receipt_root_capture",
            "cargo metadata --no-deps --format-version 1",
        ),
        OperatorCommand::new(
            CompileGate::OperatorSignoff,
            "operator_signoff_receipt_root_capture",
            "operator records independent approval roots",
        ),
    ]
}

pub fn public_record_from_state(state: &State) -> PublicRecord {
    let config_root = state.config.state_root();
    let source_slot_root = list_root(
        "source_slot_roots",
        state
            .source_slot_roots
            .values()
            .map(SourceSlotRoot::public_record),
    );
    let admission_rule_root = list_root(
        "admission_rules",
        state
            .admission_rules
            .values()
            .map(AdmissionRule::public_record),
    );
    let quarantine_root = list_root(
        "quarantined_receipts",
        state
            .quarantined_receipts
            .iter()
            .map(QuarantineEntry::public_record),
    );
    let admitted_receipt_root = admitted_root(&state.admitted_receipt_roots);
    let operator_command_root = list_root(
        "operator_commands",
        state
            .operator_commands
            .iter()
            .map(OperatorCommand::public_record),
    );
    let operator_approval_root =
        string_list_root("operator_approval_roots", &state.operator_approval_roots);
    let verdict = state.verdict();
    let verdict_root = verdict.state_root();
    let state_root = record_root(
        "state",
        &json!({
            "config_root": config_root,
            "source_slot_root": source_slot_root,
            "admission_rule_root": admission_rule_root,
            "quarantine_root": quarantine_root,
            "admitted_receipt_root": admitted_receipt_root,
            "operator_command_root": operator_command_root,
            "operator_approval_root": operator_approval_root,
            "verdict_root": verdict_root,
        }),
    );
    PublicRecord {
        config_root,
        source_slot_root,
        admission_rule_root,
        quarantine_root,
        admitted_receipt_root,
        operator_command_root,
        operator_approval_root,
        verdict_root,
        state_root,
    }
}

pub fn admitted_root(admitted: &BTreeMap<CompileGate, Vec<String>>) -> String {
    let leaves = admitted
        .iter()
        .map(|(gate, roots)| {
            json!({
                "gate": gate.as_str(),
                "receipt_roots": roots,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-WAVE93-RECEIPT-ADMISSION-ADMITTED",
        &leaves,
    )
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE93-RECEIPT-ADMISSION-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

pub fn empty_root(kind: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE93-RECEIPT-ADMISSION-EMPTY",
        &[HashPart::Str(kind)],
        32,
    )
}

pub fn list_root<I>(kind: &str, records: I) -> String
where
    I: IntoIterator<Item = Value>,
{
    let leaves = records.into_iter().collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE93-RECEIPT-ADMISSION-{kind}"),
        &leaves,
    )
}

pub fn string_list_root(kind: &str, values: &[String]) -> String {
    let leaves = values
        .iter()
        .map(|value| json!({ "root": value }))
        .collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE93-RECEIPT-ADMISSION-{kind}"),
        &leaves,
    )
}
