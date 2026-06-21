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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave91-live-heavy-gate-plan-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const LANE_ID: &str = "wave91-live-heavy-gate-execution-plan-compile-receipt-lane";
pub const WAVE_ID: &str = "wave91";
pub const PRIOR_WAVE_ID: &str = "wave90";
pub const DEFAULT_MIN_DENIAL_ROOTS: usize = 1;
pub const DEFAULT_MIN_OPERATOR_SIGNOFFS: usize = 2;
pub const DEFAULT_MAX_RECORDS: usize = 512;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialKind {
    ProductionReadinessDenied,
    CompileLaneBlocked,
    CargoCheckBlocked,
    CargoTestBlocked,
    ClippyBlocked,
    RustfmtBlocked,
    RustcBlocked,
    BuildMetadataBlocked,
    OperatorSignoffBlocked,
}

impl DenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProductionReadinessDenied => "production_readiness_denied",
            Self::CompileLaneBlocked => "compile_lane_blocked",
            Self::CargoCheckBlocked => "cargo_check_blocked",
            Self::CargoTestBlocked => "cargo_test_blocked",
            Self::ClippyBlocked => "clippy_blocked",
            Self::RustfmtBlocked => "rustfmt_blocked",
            Self::RustcBlocked => "rustc_blocked",
            Self::BuildMetadataBlocked => "build_metadata_blocked",
            Self::OperatorSignoffBlocked => "operator_signoff_blocked",
        }
    }

    pub fn all() -> [Self; 9] {
        [
            Self::ProductionReadinessDenied,
            Self::CompileLaneBlocked,
            Self::CargoCheckBlocked,
            Self::CargoTestBlocked,
            Self::ClippyBlocked,
            Self::RustfmtBlocked,
            Self::RustcBlocked,
            Self::BuildMetadataBlocked,
            Self::OperatorSignoffBlocked,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateKind {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl GateKind {
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

    pub fn is_machine_gate(self) -> bool {
        !matches!(self, Self::OperatorSignoff)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StepPhase {
    Intake,
    Prepare,
    Execute,
    CaptureReceipt,
    AuditReceipt,
    OperatorReview,
    Seal,
}

impl StepPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intake => "intake",
            Self::Prepare => "prepare",
            Self::Execute => "execute",
            Self::CaptureReceipt => "capture_receipt",
            Self::AuditReceipt => "audit_receipt",
            Self::OperatorReview => "operator_review",
            Self::Seal => "seal",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Missing,
    Captured,
    Cleared,
    Rejected,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Captured => "captured",
            Self::Cleared => "cleared",
            Self::Rejected => "rejected",
        }
    }

    pub fn clears(self) -> bool {
        matches!(self, Self::Cleared)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Blocked,
    Review,
    Clear,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Review => "review",
            Self::Clear => "clear",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub lane_id: String,
    pub wave_id: String,
    pub prior_wave_id: String,
    pub min_denial_roots: usize,
    pub min_operator_signoffs: usize,
    pub max_records: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            lane_id: LANE_ID.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            min_denial_roots: DEFAULT_MIN_DENIAL_ROOTS,
            min_operator_signoffs: DEFAULT_MIN_OPERATOR_SIGNOFFS,
            max_records: DEFAULT_MAX_RECORDS,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_id": self.lane_id,
            "wave_id": self.wave_id,
            "prior_wave_id": self.prior_wave_id,
            "min_denial_roots": self.min_denial_roots,
            "min_operator_signoffs": self.min_operator_signoffs,
            "max_records": self.max_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DenialRootIntake {
    pub intake_id: String,
    pub denial_kind: DenialKind,
    pub source_wave: String,
    pub denial_root: String,
    pub denial_summary_root: String,
    pub blocker_root: String,
    pub privacy_note: String,
}

impl DenialRootIntake {
    pub fn new(
        denial_kind: DenialKind,
        source_wave: impl Into<String>,
        denial_root: impl Into<String>,
        denial_summary_root: impl Into<String>,
        blocker_root: impl Into<String>,
    ) -> Self {
        let source_wave = source_wave.into();
        let denial_root = denial_root.into();
        let denial_summary_root = denial_summary_root.into();
        let blocker_root = blocker_root.into();
        let intake_id = intake_id(
            denial_kind,
            &source_wave,
            &denial_root,
            &denial_summary_root,
            &blocker_root,
        );
        Self {
            intake_id,
            denial_kind,
            source_wave,
            denial_root,
            denial_summary_root,
            blocker_root,
            privacy_note: "roots_only_no_raw_keys_txids_addresses_labels_or_payloads".to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "intake_id": self.intake_id,
            "denial_kind": self.denial_kind.as_str(),
            "source_wave": self.source_wave,
            "denial_root": self.denial_root,
            "denial_summary_root": self.denial_summary_root,
            "blocker_root": self.blocker_root,
            "privacy_note": self.privacy_note,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("denial_root_intake", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptanceCriterion {
    pub criterion_id: String,
    pub gate_kind: GateKind,
    pub title: String,
    pub required_status: ReceiptStatus,
    pub evidence_rule: String,
    pub root_rule: String,
}

impl AcceptanceCriterion {
    pub fn new(
        gate_kind: GateKind,
        title: impl Into<String>,
        evidence_rule: impl Into<String>,
        root_rule: impl Into<String>,
    ) -> Self {
        let title = title.into();
        let evidence_rule = evidence_rule.into();
        let root_rule = root_rule.into();
        let criterion_id = criterion_id(gate_kind, &title, &evidence_rule, &root_rule);
        Self {
            criterion_id,
            gate_kind,
            title,
            required_status: ReceiptStatus::Cleared,
            evidence_rule,
            root_rule,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "criterion_id": self.criterion_id,
            "gate_kind": self.gate_kind.as_str(),
            "title": self.title,
            "required_status": self.required_status.as_str(),
            "evidence_rule": self.evidence_rule,
            "root_rule": self.root_rule,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("acceptance_criterion", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperatorCommandHint {
    pub hint_id: String,
    pub gate_kind: GateKind,
    pub command_family: String,
    pub command_hint: String,
    pub capture_hint: String,
    pub refusal_note: String,
}

impl OperatorCommandHint {
    pub fn new(
        gate_kind: GateKind,
        command_family: impl Into<String>,
        command_hint: impl Into<String>,
        capture_hint: impl Into<String>,
    ) -> Self {
        let command_family = command_family.into();
        let command_hint = command_hint.into();
        let capture_hint = capture_hint.into();
        let hint_id = command_hint_id(gate_kind, &command_family, &command_hint, &capture_hint);
        Self {
            hint_id,
            gate_kind,
            command_family,
            command_hint,
            capture_hint,
            refusal_note: "agent_must_not_run_live_heavy_gate_commands_in_this_lane".to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hint_id": self.hint_id,
            "gate_kind": self.gate_kind.as_str(),
            "command_family": self.command_family,
            "command_hint": self.command_hint,
            "capture_hint": self.capture_hint,
            "refusal_note": self.refusal_note,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator_command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecutionStep {
    pub step_id: String,
    pub ordinal: u64,
    pub phase: StepPhase,
    pub gate_kind: GateKind,
    pub action: String,
    pub input_root: String,
    pub output_root: String,
    pub command_hint_root: String,
    pub criterion_root: String,
}

impl ExecutionStep {
    pub fn new(
        ordinal: u64,
        phase: StepPhase,
        gate_kind: GateKind,
        action: impl Into<String>,
        input_root: impl Into<String>,
        output_root: impl Into<String>,
        command_hint_root: impl Into<String>,
        criterion_root: impl Into<String>,
    ) -> Self {
        let action = action.into();
        let input_root = input_root.into();
        let output_root = output_root.into();
        let command_hint_root = command_hint_root.into();
        let criterion_root = criterion_root.into();
        let step_id = execution_step_id(
            ordinal,
            phase,
            gate_kind,
            &action,
            &input_root,
            &output_root,
            &command_hint_root,
            &criterion_root,
        );
        Self {
            step_id,
            ordinal,
            phase,
            gate_kind,
            action,
            input_root,
            output_root,
            command_hint_root,
            criterion_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "step_id": self.step_id,
            "ordinal": self.ordinal,
            "phase": self.phase.as_str(),
            "gate_kind": self.gate_kind.as_str(),
            "action": self.action,
            "input_root": self.input_root,
            "output_root": self.output_root,
            "command_hint_root": self.command_hint_root,
            "criterion_root": self.criterion_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("execution_step", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlannedReceipt {
    pub receipt_id: String,
    pub gate_kind: GateKind,
    pub status: ReceiptStatus,
    pub command_hint_root: String,
    pub criterion_root: String,
    pub run_root: String,
    pub stdout_root: String,
    pub stderr_root: String,
    pub artifact_root: String,
    pub operator_root: String,
    pub notes_root: String,
}

impl PlannedReceipt {
    pub fn missing(
        gate_kind: GateKind,
        command_hint_root: impl Into<String>,
        criterion_root: impl Into<String>,
    ) -> Self {
        Self::new(
            gate_kind,
            ReceiptStatus::Missing,
            command_hint_root,
            criterion_root,
            empty_root("run"),
            empty_root("stdout"),
            empty_root("stderr"),
            empty_root("artifact"),
            empty_root("operator"),
            empty_root("notes"),
        )
    }

    pub fn new(
        gate_kind: GateKind,
        status: ReceiptStatus,
        command_hint_root: impl Into<String>,
        criterion_root: impl Into<String>,
        run_root: impl Into<String>,
        stdout_root: impl Into<String>,
        stderr_root: impl Into<String>,
        artifact_root: impl Into<String>,
        operator_root: impl Into<String>,
        notes_root: impl Into<String>,
    ) -> Self {
        let command_hint_root = command_hint_root.into();
        let criterion_root = criterion_root.into();
        let run_root = run_root.into();
        let stdout_root = stdout_root.into();
        let stderr_root = stderr_root.into();
        let artifact_root = artifact_root.into();
        let operator_root = operator_root.into();
        let notes_root = notes_root.into();
        let receipt_id = receipt_id(
            gate_kind,
            status,
            &command_hint_root,
            &criterion_root,
            &run_root,
            &stdout_root,
            &stderr_root,
            &artifact_root,
            &operator_root,
            &notes_root,
        );
        Self {
            receipt_id,
            gate_kind,
            status,
            command_hint_root,
            criterion_root,
            run_root,
            stdout_root,
            stderr_root,
            artifact_root,
            operator_root,
            notes_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "gate_kind": self.gate_kind.as_str(),
            "status": self.status.as_str(),
            "command_hint_root": self.command_hint_root,
            "criterion_root": self.criterion_root,
            "run_root": self.run_root,
            "stdout_root": self.stdout_root,
            "stderr_root": self.stderr_root,
            "artifact_root": self.artifact_root,
            "operator_root": self.operator_root,
            "notes_root": self.notes_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("planned_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClearanceVerdict {
    pub verdict_id: String,
    pub verdict: Verdict,
    pub cleared_gates: Vec<String>,
    pub blocked_gates: Vec<String>,
    pub denial_intake_root: String,
    pub receipt_root: String,
    pub signoff_root: String,
    pub fail_closed_reason: String,
}

impl ClearanceVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "verdict": self.verdict.as_str(),
            "cleared_gates": self.cleared_gates,
            "blocked_gates": self.blocked_gates,
            "denial_intake_root": self.denial_intake_root,
            "receipt_root": self.receipt_root,
            "signoff_root": self.signoff_root,
            "fail_closed_reason": self.fail_closed_reason,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("clearance_verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub denial_intake_root: String,
    pub command_hint_root: String,
    pub criterion_root: String,
    pub execution_plan_root: String,
    pub receipt_root: String,
    pub signoff_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "denial_intake_root": self.denial_intake_root,
            "command_hint_root": self.command_hint_root,
            "criterion_root": self.criterion_root,
            "execution_plan_root": self.execution_plan_root,
            "receipt_root": self.receipt_root,
            "signoff_root": self.signoff_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Counters {
    pub denial_roots: usize,
    pub command_hints: usize,
    pub criteria: usize,
    pub execution_steps: usize,
    pub planned_receipts: usize,
    pub cleared_receipts: usize,
    pub operator_signoffs: usize,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "denial_roots": self.denial_roots,
            "command_hints": self.command_hints,
            "criteria": self.criteria,
            "execution_steps": self.execution_steps,
            "planned_receipts": self.planned_receipts,
            "cleared_receipts": self.cleared_receipts,
            "operator_signoffs": self.operator_signoffs,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub denial_intakes: Vec<DenialRootIntake>,
    pub command_hints: Vec<OperatorCommandHint>,
    pub criteria: Vec<AcceptanceCriterion>,
    pub execution_steps: Vec<ExecutionStep>,
    pub planned_receipts: Vec<PlannedReceipt>,
    pub operator_signoff_roots: Vec<String>,
    pub counters: Counters,
    pub verdict: ClearanceVerdict,
    pub record: PublicRecord,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let denial_intakes = default_denial_intakes();
        let command_hints = default_command_hints();
        let criteria = default_acceptance_criteria();
        let execution_steps = default_execution_steps(&denial_intakes, &command_hints, &criteria);
        let planned_receipts = default_planned_receipts(&command_hints, &criteria);
        let operator_signoff_roots = Vec::new();
        let counters = count_state(
            &denial_intakes,
            &command_hints,
            &criteria,
            &execution_steps,
            &planned_receipts,
            &operator_signoff_roots,
        );
        let denial_intake_root = list_root(
            "denial_intakes",
            denial_intakes.iter().map(DenialRootIntake::public_record),
        );
        let receipt_root = list_root(
            "planned_receipts",
            planned_receipts.iter().map(PlannedReceipt::public_record),
        );
        let signoff_root = string_list_root("operator_signoffs", &operator_signoff_roots);
        let verdict = clearance_verdict(
            &config,
            &denial_intakes,
            &planned_receipts,
            &operator_signoff_roots,
            &denial_intake_root,
            &receipt_root,
            &signoff_root,
        );
        let record = public_record_from_parts(
            &config,
            &denial_intakes,
            &command_hints,
            &criteria,
            &execution_steps,
            &planned_receipts,
            &operator_signoff_roots,
            &verdict,
            &counters,
        );
        Self {
            config,
            denial_intakes,
            command_hints,
            criteria,
            execution_steps,
            planned_receipts,
            operator_signoff_roots,
            counters,
            verdict,
            record,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        public_record_from_parts(
            &self.config,
            &self.denial_intakes,
            &self.command_hints,
            &self.criteria,
            &self.execution_steps,
            &self.planned_receipts,
            &self.operator_signoff_roots,
            &self.verdict,
            &self.counters,
        )
    }

    pub fn state_root(&self) -> String {
        self.public_record().state_root
    }

    pub fn refresh(&mut self) {
        self.counters = count_state(
            &self.denial_intakes,
            &self.command_hints,
            &self.criteria,
            &self.execution_steps,
            &self.planned_receipts,
            &self.operator_signoff_roots,
        );
        let denial_intake_root = list_root(
            "denial_intakes",
            self.denial_intakes
                .iter()
                .map(DenialRootIntake::public_record),
        );
        let receipt_root = list_root(
            "planned_receipts",
            self.planned_receipts
                .iter()
                .map(PlannedReceipt::public_record),
        );
        let signoff_root = string_list_root("operator_signoffs", &self.operator_signoff_roots);
        self.verdict = clearance_verdict(
            &self.config,
            &self.denial_intakes,
            &self.planned_receipts,
            &self.operator_signoff_roots,
            &denial_intake_root,
            &receipt_root,
            &signoff_root,
        );
        self.record = self.public_record();
    }

    pub fn add_denial_intake(&mut self, intake: DenialRootIntake) -> Result<()> {
        ensure(
            self.denial_intakes.len() < self.config.max_records,
            "denial_intake_limit",
        )?;
        ensure(!intake.denial_root.is_empty(), "denial_root_absent")?;
        ensure(!intake.blocker_root.is_empty(), "blocker_root_absent")?;
        self.denial_intakes.push(intake);
        self.refresh();
        Ok(())
    }

    pub fn record_receipt(&mut self, receipt: PlannedReceipt) -> Result<()> {
        ensure(
            self.planned_receipts.len() <= self.config.max_records,
            "receipt_limit",
        )?;
        let mut replaced = false;
        for slot in &mut self.planned_receipts {
            if slot.gate_kind == receipt.gate_kind {
                *slot = receipt.clone();
                replaced = true;
            }
        }
        if !replaced {
            self.planned_receipts.push(receipt);
        }
        self.refresh();
        Ok(())
    }

    pub fn add_operator_signoff_root(&mut self, signoff_root: impl Into<String>) -> Result<()> {
        let signoff_root = signoff_root.into();
        ensure(!signoff_root.is_empty(), "operator_signoff_root_absent")?;
        ensure(
            self.operator_signoff_roots.len() < self.config.max_records,
            "operator_signoff_limit",
        )?;
        if !self.operator_signoff_roots.contains(&signoff_root) {
            self.operator_signoff_roots.push(signoff_root);
        }
        self.refresh();
        Ok(())
    }

    pub fn receipt_for(&self, gate_kind: GateKind) -> Option<&PlannedReceipt> {
        self.planned_receipts
            .iter()
            .find(|receipt| receipt.gate_kind == gate_kind)
    }

    pub fn missing_gate_names(&self) -> Vec<String> {
        GateKind::all()
            .iter()
            .copied()
            .filter(|gate| match self.receipt_for(*gate) {
                Some(receipt) => !receipt.status.clears(),
                None => true,
            })
            .map(|gate| gate.as_str().to_string())
            .collect()
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    State::devnet().public_record()
}

pub fn state_root() -> String {
    State::devnet().state_root()
}

pub fn default_denial_intakes() -> Vec<DenialRootIntake> {
    let denial_root = record_root(
        "wave90_production_readiness_denial",
        &json!({
            "source_wave": PRIOR_WAVE_ID,
            "denial": "production_readiness_denied_until_live_heavy_gate_receipts_exist",
            "privacy": "roots_only",
        }),
    );
    let summary_root = record_root(
        "wave90_denial_summary",
        &json!({
            "compile_lane": "blocked",
            "cargo_check": "needs_live_receipt",
            "cargo_test": "needs_live_receipt",
            "clippy": "needs_live_receipt",
            "rustfmt": "needs_live_receipt",
            "rustc": "needs_live_receipt",
            "build_metadata": "needs_live_receipt",
            "operator_signoff": "needs_live_receipt",
        }),
    );
    let blocker_root = record_root(
        "wave90_blockers",
        &json!({
            "blockers": [
                "cargo_check_receipt_absent",
                "cargo_test_receipt_absent",
                "clippy_receipt_absent",
                "rustfmt_receipt_absent",
                "rustc_receipt_absent",
                "build_metadata_receipt_absent",
                "operator_signoff_receipt_absent"
            ],
        }),
    );
    vec![DenialRootIntake::new(
        DenialKind::ProductionReadinessDenied,
        PRIOR_WAVE_ID,
        denial_root,
        summary_root,
        blocker_root,
    )]
}

pub fn default_command_hints() -> Vec<OperatorCommandHint> {
    vec![
        OperatorCommandHint::new(
            GateKind::CargoCheck,
            "cargo",
            "cargo check --workspace --all-targets",
            "seal command status, toolchain root, stdout root, stderr root, and artifact root",
        ),
        OperatorCommandHint::new(
            GateKind::CargoTest,
            "cargo",
            "cargo test --workspace --all-targets",
            "seal command status, test summary root, stdout root, stderr root, and artifact root",
        ),
        OperatorCommandHint::new(
            GateKind::Clippy,
            "cargo",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "seal lint status, lint summary root, stdout root, stderr root, and artifact root",
        ),
        OperatorCommandHint::new(
            GateKind::Rustfmt,
            "cargo",
            "cargo fmt --all --check",
            "seal format status, file delta root, stdout root, stderr root, and artifact root",
        ),
        OperatorCommandHint::new(
            GateKind::Rustc,
            "rustc",
            "rustc --version --verbose",
            "seal compiler identity root and host target root",
        ),
        OperatorCommandHint::new(
            GateKind::BuildMetadata,
            "cargo",
            "cargo metadata --no-deps --format-version 1",
            "seal package graph root, target root, feature root, and lock root",
        ),
        OperatorCommandHint::new(
            GateKind::OperatorSignoff,
            "operator",
            "operator reviews sealed receipt roots and records approval roots",
            "seal at least two independent operator approval roots",
        ),
    ]
}

pub fn default_acceptance_criteria() -> Vec<AcceptanceCriterion> {
    vec![
        AcceptanceCriterion::new(
            GateKind::CargoCheck,
            "cargo check clears compile blockers",
            "receipt status clear with command family cargo and no compile error root",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::CargoTest,
            "cargo test clears runtime blockers",
            "receipt status clear with test summary root proving all selected targets clear",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::Clippy,
            "clippy clears lint blockers",
            "receipt status clear with lint root and zero warning-denial failures",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::Rustfmt,
            "rustfmt clears format blockers",
            "receipt status clear with format check root and no file mutation root",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::Rustc,
            "rustc identity clears compiler blockers",
            "receipt status clear with compiler version root and host target root",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::BuildMetadata,
            "build metadata clears package graph blockers",
            "receipt status clear with package graph root, feature root, and lock root",
            "run, stdout, stderr, artifact, and note roots must be nonempty",
        ),
        AcceptanceCriterion::new(
            GateKind::OperatorSignoff,
            "operator signoff clears release authority blockers",
            "receipt status clear only when independent operator roots meet quorum",
            "operator and note roots must be nonempty and signoff quorum must hold",
        ),
    ]
}

pub fn default_execution_steps(
    denial_intakes: &[DenialRootIntake],
    command_hints: &[OperatorCommandHint],
    criteria: &[AcceptanceCriterion],
) -> Vec<ExecutionStep> {
    let intake_root = list_root(
        "denial_intakes",
        denial_intakes.iter().map(DenialRootIntake::public_record),
    );
    let mut steps = Vec::new();
    let mut ordinal = 1_u64;
    for gate in GateKind::all() {
        let hint_root = command_hints
            .iter()
            .find(|hint| hint.gate_kind == gate)
            .map(OperatorCommandHint::state_root)
            .map_or_else(|| empty_root("command_hint"), |root| root);
        let criterion_root = criteria
            .iter()
            .find(|criterion| criterion.gate_kind == gate)
            .map(AcceptanceCriterion::state_root)
            .map_or_else(|| empty_root("criterion"), |root| root);
        steps.push(ExecutionStep::new(
            ordinal,
            StepPhase::Prepare,
            gate,
            format!("prepare_{}_receipt_capture", gate.as_str()),
            intake_root.clone(),
            empty_root("prepared_output"),
            hint_root.clone(),
            criterion_root.clone(),
        ));
        ordinal += 1;
        steps.push(ExecutionStep::new(
            ordinal,
            StepPhase::Execute,
            gate,
            format!("operator_runs_{}_outside_agent_lane", gate.as_str()),
            intake_root.clone(),
            empty_root("live_run_output"),
            hint_root.clone(),
            criterion_root.clone(),
        ));
        ordinal += 1;
        steps.push(ExecutionStep::new(
            ordinal,
            StepPhase::CaptureReceipt,
            gate,
            format!("capture_{}_roots_only_receipt", gate.as_str()),
            intake_root.clone(),
            empty_root("receipt_output"),
            hint_root.clone(),
            criterion_root.clone(),
        ));
        ordinal += 1;
        steps.push(ExecutionStep::new(
            ordinal,
            StepPhase::AuditReceipt,
            gate,
            format!(
                "audit_{}_receipt_against_acceptance_criteria",
                gate.as_str()
            ),
            intake_root.clone(),
            empty_root("audit_output"),
            hint_root.clone(),
            criterion_root,
        ));
        ordinal += 1;
    }
    steps.push(ExecutionStep::new(
        ordinal,
        StepPhase::Seal,
        GateKind::OperatorSignoff,
        "seal_clearance_verdict_after_all_receipts_clear",
        intake_root,
        empty_root("sealed_verdict"),
        empty_root("command_hint"),
        empty_root("criterion"),
    ));
    steps
}

pub fn default_planned_receipts(
    command_hints: &[OperatorCommandHint],
    criteria: &[AcceptanceCriterion],
) -> Vec<PlannedReceipt> {
    GateKind::all()
        .iter()
        .copied()
        .map(|gate| {
            let hint_root = command_hints
                .iter()
                .find(|hint| hint.gate_kind == gate)
                .map(OperatorCommandHint::state_root)
                .map_or_else(|| empty_root("command_hint"), |root| root);
            let criterion_root = criteria
                .iter()
                .find(|criterion| criterion.gate_kind == gate)
                .map(AcceptanceCriterion::state_root)
                .map_or_else(|| empty_root("criterion"), |root| root);
            PlannedReceipt::missing(gate, hint_root, criterion_root)
        })
        .collect()
}

pub fn clearance_verdict(
    config: &Config,
    denial_intakes: &[DenialRootIntake],
    receipts: &[PlannedReceipt],
    operator_signoff_roots: &[String],
    denial_intake_root: &str,
    receipt_root: &str,
    signoff_root: &str,
) -> ClearanceVerdict {
    let mut cleared = Vec::new();
    let mut blocked = Vec::new();
    for gate in GateKind::all() {
        let gate_cleared = receipts
            .iter()
            .find(|receipt| receipt.gate_kind == gate)
            .map(receipt_clears)
            .map_or(false, |clears| clears);
        if gate_cleared {
            cleared.push(gate.as_str().to_string());
        } else {
            blocked.push(gate.as_str().to_string());
        }
    }
    let denial_quorum = denial_intakes.len() >= config.min_denial_roots;
    let signoff_quorum = operator_signoff_roots.len() >= config.min_operator_signoffs;
    if !signoff_quorum && !blocked.contains(&GateKind::OperatorSignoff.as_str().to_string()) {
        blocked.push(GateKind::OperatorSignoff.as_str().to_string());
    }
    let verdict = if denial_quorum && signoff_quorum && blocked.is_empty() {
        Verdict::Clear
    } else if denial_quorum && !cleared.is_empty() {
        Verdict::Review
    } else {
        Verdict::Blocked
    };
    let fail_closed_reason = if verdict == Verdict::Clear {
        "all_live_receipts_and_operator_quorum_clear".to_string()
    } else {
        "blocked_until_denial_roots_live_gate_receipts_and_operator_quorum_clear".to_string()
    };
    let verdict_id = clearance_verdict_id(
        verdict,
        &cleared,
        &blocked,
        denial_intake_root,
        receipt_root,
        signoff_root,
        &fail_closed_reason,
    );
    ClearanceVerdict {
        verdict_id,
        verdict,
        cleared_gates: cleared,
        blocked_gates: blocked,
        denial_intake_root: denial_intake_root.to_string(),
        receipt_root: receipt_root.to_string(),
        signoff_root: signoff_root.to_string(),
        fail_closed_reason,
    }
}

pub fn receipt_clears(receipt: &PlannedReceipt) -> bool {
    receipt.status.clears()
        && receipt.run_root != empty_root("run")
        && receipt.stdout_root != empty_root("stdout")
        && receipt.stderr_root != empty_root("stderr")
        && receipt.artifact_root != empty_root("artifact")
        && receipt.notes_root != empty_root("notes")
        && if receipt.gate_kind == GateKind::OperatorSignoff {
            receipt.operator_root != empty_root("operator")
        } else {
            true
        }
}

pub fn count_state(
    denial_intakes: &[DenialRootIntake],
    command_hints: &[OperatorCommandHint],
    criteria: &[AcceptanceCriterion],
    execution_steps: &[ExecutionStep],
    receipts: &[PlannedReceipt],
    operator_signoff_roots: &[String],
) -> Counters {
    Counters {
        denial_roots: denial_intakes.len(),
        command_hints: command_hints.len(),
        criteria: criteria.len(),
        execution_steps: execution_steps.len(),
        planned_receipts: receipts.len(),
        cleared_receipts: receipts
            .iter()
            .filter(|receipt| receipt_clears(receipt))
            .count(),
        operator_signoffs: operator_signoff_roots.len(),
    }
}

pub fn public_record_from_parts(
    config: &Config,
    denial_intakes: &[DenialRootIntake],
    command_hints: &[OperatorCommandHint],
    criteria: &[AcceptanceCriterion],
    execution_steps: &[ExecutionStep],
    planned_receipts: &[PlannedReceipt],
    operator_signoff_roots: &[String],
    verdict: &ClearanceVerdict,
    counters: &Counters,
) -> PublicRecord {
    let config_root = config.state_root();
    let denial_intake_root = list_root(
        "denial_intakes",
        denial_intakes.iter().map(DenialRootIntake::public_record),
    );
    let command_hint_root = list_root(
        "command_hints",
        command_hints.iter().map(OperatorCommandHint::public_record),
    );
    let criterion_root = list_root(
        "acceptance_criteria",
        criteria.iter().map(AcceptanceCriterion::public_record),
    );
    let execution_plan_root = list_root(
        "execution_steps",
        execution_steps.iter().map(ExecutionStep::public_record),
    );
    let receipt_root = list_root(
        "planned_receipts",
        planned_receipts.iter().map(PlannedReceipt::public_record),
    );
    let signoff_root = string_list_root("operator_signoffs", operator_signoff_roots);
    let verdict_root = verdict.state_root();
    let counters_root = counters.state_root();
    let state_root = record_root(
        "state",
        &json!({
            "config_root": config_root,
            "denial_intake_root": denial_intake_root,
            "command_hint_root": command_hint_root,
            "criterion_root": criterion_root,
            "execution_plan_root": execution_plan_root,
            "receipt_root": receipt_root,
            "signoff_root": signoff_root,
            "verdict_root": verdict_root,
            "counters_root": counters_root,
        }),
    );
    PublicRecord {
        config_root,
        denial_intake_root,
        command_hint_root,
        criterion_root,
        execution_plan_root,
        receipt_root,
        signoff_root,
        verdict_root,
        state_root,
    }
}

pub fn intake_id(
    denial_kind: DenialKind,
    source_wave: &str,
    denial_root: &str,
    denial_summary_root: &str,
    blocker_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-DENIAL-INTAKE-ID",
        &[
            HashPart::Str(denial_kind.as_str()),
            HashPart::Str(source_wave),
            HashPart::Str(denial_root),
            HashPart::Str(denial_summary_root),
            HashPart::Str(blocker_root),
        ],
        32,
    )
}

pub fn criterion_id(
    gate_kind: GateKind,
    title: &str,
    evidence_rule: &str,
    root_rule: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-CRITERION-ID",
        &[
            HashPart::Str(gate_kind.as_str()),
            HashPart::Str(title),
            HashPart::Str(evidence_rule),
            HashPart::Str(root_rule),
        ],
        32,
    )
}

pub fn command_hint_id(
    gate_kind: GateKind,
    command_family: &str,
    command_hint: &str,
    capture_hint: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-COMMAND-HINT-ID",
        &[
            HashPart::Str(gate_kind.as_str()),
            HashPart::Str(command_family),
            HashPart::Str(command_hint),
            HashPart::Str(capture_hint),
        ],
        32,
    )
}

pub fn execution_step_id(
    ordinal: u64,
    phase: StepPhase,
    gate_kind: GateKind,
    action: &str,
    input_root: &str,
    output_root: &str,
    command_hint_root: &str,
    criterion_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-EXECUTION-STEP-ID",
        &[
            HashPart::U64(ordinal),
            HashPart::Str(phase.as_str()),
            HashPart::Str(gate_kind.as_str()),
            HashPart::Str(action),
            HashPart::Str(input_root),
            HashPart::Str(output_root),
            HashPart::Str(command_hint_root),
            HashPart::Str(criterion_root),
        ],
        32,
    )
}

pub fn receipt_id(
    gate_kind: GateKind,
    status: ReceiptStatus,
    command_hint_root: &str,
    criterion_root: &str,
    run_root: &str,
    stdout_root: &str,
    stderr_root: &str,
    artifact_root: &str,
    operator_root: &str,
    notes_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-RECEIPT-ID",
        &[
            HashPart::Str(gate_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(command_hint_root),
            HashPart::Str(criterion_root),
            HashPart::Str(run_root),
            HashPart::Str(stdout_root),
            HashPart::Str(stderr_root),
            HashPart::Str(artifact_root),
            HashPart::Str(operator_root),
            HashPart::Str(notes_root),
        ],
        32,
    )
}

pub fn clearance_verdict_id(
    verdict: Verdict,
    cleared_gates: &[String],
    blocked_gates: &[String],
    denial_intake_root: &str,
    receipt_root: &str,
    signoff_root: &str,
    fail_closed_reason: &str,
) -> String {
    let cleared_root = string_list_root("cleared_gates", cleared_gates);
    let blocked_root = string_list_root("blocked_gates", blocked_gates);
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-CLEARANCE-VERDICT-ID",
        &[
            HashPart::Str(verdict.as_str()),
            HashPart::Str(&cleared_root),
            HashPart::Str(&blocked_root),
            HashPart::Str(denial_intake_root),
            HashPart::Str(receipt_root),
            HashPart::Str(signoff_root),
            HashPart::Str(fail_closed_reason),
        ],
        32,
    )
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

pub fn empty_root(kind: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-EMPTY",
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
        &format!("MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-{kind}"),
        &leaves,
    )
}

pub fn string_list_root(kind: &str, values: &[String]) -> String {
    let leaves = values
        .iter()
        .map(|value| json!({ "root": value }))
        .collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-{kind}"),
        &leaves,
    )
}

pub fn gate_set(receipts: &[PlannedReceipt]) -> BTreeSet<String> {
    receipts
        .iter()
        .map(|receipt| receipt.gate_kind.as_str().to_string())
        .collect()
}

pub fn receipt_status_map(receipts: &[PlannedReceipt]) -> BTreeMap<String, String> {
    receipts
        .iter()
        .map(|receipt| {
            (
                receipt.gate_kind.as_str().to_string(),
                receipt.status.as_str().to_string(),
            )
        })
        .collect()
}

pub fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}
