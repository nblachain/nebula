// Wave 91 live heavy-gate execution plan runtime replay receipt lane.
//
// This module is intentionally self-contained and roots-only.  It models the
// receipt lane needed after the Wave 90 denial: replay run, rollback drill,
// adversarial replay, stale archive replacement, live execution receipt, and
// operator signoff.  It does not carry raw keys, transaction ids, addresses,
// labels, or opaque operational data.

use std::collections::BTreeMap;
use std::fmt;

pub type Result<T> = std::result::Result<T, RuntimeError>;
pub type Runtime = State;

const MODULE_ID: &str = "monero_l2_pq_bridge_exit_force_exit_wave91_live_heavy_gate_execution_plan_runtime_replay_receipt_runtime";
const WAVE: u16 = 91;
const PRIOR_WAVE: u16 = 90;
const ROOT_PREFIX: &str = "wave91";
const DENIAL_ROOT: &str = "wave90-denial-runtime-replay-blockers";
const REPLAY_ROOT: &str = "wave91-live-replay-run-receipt";
const ROLLBACK_ROOT: &str = "wave91-live-rollback-drill-receipt";
const ADVERSARIAL_ROOT: &str = "wave91-live-adversarial-replay-receipt";
const ARCHIVE_ROOT: &str = "wave91-stale-archive-replacement-receipt";
const EXECUTION_ROOT: &str = "wave91-live-execution-receipt";
const SIGNOFF_ROOT: &str = "wave91-operator-signoff-receipt";
const CLEARANCE_ROOT: &str = "wave91-fail-closed-clearance-verdict";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub module_id: String,
    pub wave: u16,
    pub prior_wave: u16,
    pub lane: Lane,
    pub privacy: PrivacyMode,
    pub denial_intake: DenialRootIntake,
    pub acceptance: AcceptanceCriteria,
    pub command_hints: Vec<OperatorCommandHint>,
    pub required_actions: Vec<ReceiptAction>,
    pub fail_closed: FailClosedPolicy,
}

impl Config {
    pub fn live() -> Self {
        let denial_intake = DenialRootIntake::wave90();
        let acceptance = AcceptanceCriteria::strict_live();
        let command_hints = OperatorCommandHint::default_hints();
        let required_actions = ReceiptAction::required_live_actions();
        let fail_closed = FailClosedPolicy::strict();

        Self {
            module_id: MODULE_ID.to_owned(),
            wave: WAVE,
            prior_wave: PRIOR_WAVE,
            lane: Lane::LiveHeavyGateRuntimeReplayReceipt,
            privacy: PrivacyMode::RootsOnly,
            denial_intake,
            acceptance,
            command_hints,
            required_actions,
            fail_closed,
        }
    }

    pub fn config_root(&self) -> String {
        root_of(&[
            self.module_id.as_str(),
            &self.wave.to_string(),
            &self.prior_wave.to_string(),
            self.lane.as_str(),
            self.privacy.as_str(),
            self.denial_intake.root.as_str(),
            self.acceptance.criteria_root().as_str(),
            self.fail_closed.policy_root().as_str(),
            &join_roots(self.required_actions.iter().map(ReceiptAction::root)),
            &join_roots(
                self.command_hints
                    .iter()
                    .map(OperatorCommandHint::hint_root),
            ),
        ])
    }

    pub fn action_by_kind(&self, kind: ReceiptKind) -> Option<ReceiptAction> {
        self.required_actions
            .iter()
            .find(|action| action.kind == kind)
            .cloned()
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            module_root: state_root(),
            wave: self.wave,
            prior_wave: self.prior_wave,
            lane: self.lane,
            denial_root: self.denial_intake.root.clone(),
            config_root: self.config_root(),
            plan_root: root_of(&[
                self.acceptance.criteria_root().as_str(),
                &join_roots(self.required_actions.iter().map(ReceiptAction::root)),
                &join_roots(
                    self.command_hints
                        .iter()
                        .map(OperatorCommandHint::hint_root),
                ),
            ]),
            receipt_roots: BTreeMap::new(),
            clearance: ClearanceVerdict::Blocked {
                reason_root: root_of(&["initial", "no-live-receipts"]),
                missing: self
                    .required_actions
                    .iter()
                    .map(|action| action.kind)
                    .collect(),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::live()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub receipts: BTreeMap<ReceiptKind, PlannedReplayReceipt>,
    pub clearance: ClearanceVerdict,
    pub audit: RuntimeAudit,
}

impl State {
    pub fn new(config: Config) -> Self {
        let missing = config
            .required_actions
            .iter()
            .map(|action| action.kind)
            .collect::<Vec<_>>();
        let clearance = ClearanceVerdict::Blocked {
            reason_root: root_of(&["new-state", "missing-live-receipts"]),
            missing,
        };
        let audit = RuntimeAudit::from_config(&config);
        Self {
            config,
            receipts: BTreeMap::new(),
            clearance,
            audit,
        }
    }

    pub fn with_receipt(mut self, receipt: PlannedReplayReceipt) -> Result<Self> {
        self.insert_receipt(receipt)?;
        Ok(self)
    }

    pub fn insert_receipt(&mut self, receipt: PlannedReplayReceipt) -> Result<()> {
        self.config.acceptance.check_receipt(&receipt)?;
        if !self
            .config
            .required_actions
            .iter()
            .any(|action| action.kind == receipt.kind)
        {
            return Err(RuntimeError::UnknownReceiptKind(receipt.kind));
        }
        if !receipt.public_only() {
            return Err(RuntimeError::PrivacyBreach(receipt.kind));
        }
        self.receipts.insert(receipt.kind, receipt);
        self.recompute_clearance();
        Ok(())
    }

    pub fn replay_run_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::replay_run())?;
        Ok(self)
    }

    pub fn rollback_drill_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::rollback_drill())?;
        Ok(self)
    }

    pub fn adversarial_replay_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::adversarial_replay())?;
        Ok(self)
    }

    pub fn stale_archive_replacement_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::stale_archive_replacement())?;
        Ok(self)
    }

    pub fn live_execution_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::live_execution())?;
        Ok(self)
    }

    pub fn operator_signoff_receipt(mut self) -> Result<Self> {
        self.insert_receipt(PlannedReplayReceipt::operator_signoff())?;
        Ok(self)
    }

    pub fn clear_with_all_live_receipts(mut self) -> Result<Self> {
        for action in self.config.required_actions.clone() {
            self.insert_receipt(action.planned_receipt())?;
        }
        Ok(self)
    }

    pub fn missing_receipts(&self) -> Vec<ReceiptKind> {
        self.config
            .required_actions
            .iter()
            .filter(|action| !self.receipts.contains_key(&action.kind))
            .map(|action| action.kind)
            .collect()
    }

    pub fn has_all_required_receipts(&self) -> bool {
        self.missing_receipts().is_empty()
    }

    pub fn fail_closed_verdict(&self) -> ClearanceVerdict {
        let missing = self.missing_receipts();
        if !missing.is_empty() {
            return ClearanceVerdict::Blocked {
                reason_root: root_of(&["missing-receipts", &join_kinds(&missing)]),
                missing,
            };
        }

        let mut failed = Vec::new();
        for receipt in self.receipts.values() {
            if !self.config.acceptance.receipt_satisfies(receipt) {
                failed.push(receipt.kind);
            }
        }

        if !failed.is_empty() {
            return ClearanceVerdict::Rejected {
                reason_root: root_of(&["criteria-not-met", &join_kinds(&failed)]),
                failed,
            };
        }

        let receipt_root = self.receipts_root();
        ClearanceVerdict::Cleared {
            clearance_root: root_of(&[
                CLEARANCE_ROOT,
                self.config.config_root().as_str(),
                receipt_root.as_str(),
            ]),
            receipt_root,
        }
    }

    pub fn recompute_clearance(&mut self) {
        self.clearance = self.fail_closed_verdict();
        self.audit = RuntimeAudit::from_state(self);
    }

    pub fn receipts_root(&self) -> String {
        let mut roots = Vec::new();
        for action in &self.config.required_actions {
            match self.receipts.get(&action.kind) {
                Some(receipt) => roots.push(receipt.receipt_root()),
                None => roots.push(root_of(&["missing", action.kind.as_str()])),
            }
        }
        root_of(&[
            "receipts-root",
            &join_roots(roots.into_iter()),
            self.config.denial_intake.root.as_str(),
        ])
    }

    pub fn runtime_root(&self) -> String {
        root_of(&[
            self.config.config_root().as_str(),
            self.receipts_root().as_str(),
            self.clearance.verdict_root().as_str(),
            self.audit.audit_root().as_str(),
        ])
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut receipt_roots = BTreeMap::new();
        for (kind, receipt) in &self.receipts {
            receipt_roots.insert(*kind, receipt.receipt_root());
        }
        PublicRecord {
            module_root: self.runtime_root(),
            wave: self.config.wave,
            prior_wave: self.config.prior_wave,
            lane: self.config.lane,
            denial_root: self.config.denial_intake.root.clone(),
            config_root: self.config.config_root(),
            plan_root: self.config.public_record().plan_root,
            receipt_roots,
            clearance: self.clearance.clone(),
        }
    }

    pub fn operator_commands(&self) -> Vec<OperatorCommandHint> {
        self.config.command_hints.clone()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub module_root: String,
    pub wave: u16,
    pub prior_wave: u16,
    pub lane: Lane,
    pub denial_root: String,
    pub config_root: String,
    pub plan_root: String,
    pub receipt_roots: BTreeMap<ReceiptKind, String>,
    pub clearance: ClearanceVerdict,
}

impl PublicRecord {
    pub fn record_root(&self) -> String {
        let receipt_roots = self
            .receipt_roots
            .iter()
            .map(|(kind, root)| root_of(&[kind.as_str(), root.as_str()]));
        root_of(&[
            self.module_root.as_str(),
            &self.wave.to_string(),
            &self.prior_wave.to_string(),
            self.lane.as_str(),
            self.denial_root.as_str(),
            self.config_root.as_str(),
            self.plan_root.as_str(),
            &join_roots(receipt_roots),
            self.clearance.verdict_root().as_str(),
        ])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lane {
    LiveHeavyGateRuntimeReplayReceipt,
}

impl Lane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveHeavyGateRuntimeReplayReceipt => "live-heavy-gate-runtime-replay-receipt",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrivacyMode {
    RootsOnly,
}

impl PrivacyMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootsOnly => "roots-only",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialRootIntake {
    pub source_wave: u16,
    pub root: String,
    pub blockers: Vec<DenialBlocker>,
    pub intake_root: String,
}

impl DenialRootIntake {
    pub fn wave90() -> Self {
        let blockers = DenialBlocker::wave90_blockers();
        let blocker_root = join_roots(blockers.iter().map(DenialBlocker::blocker_root));
        let root = root_of(&[DENIAL_ROOT, &blocker_root]);
        let intake_root = root_of(&[
            "denial-intake",
            &PRIOR_WAVE.to_string(),
            root.as_str(),
            blocker_root.as_str(),
        ]);
        Self {
            source_wave: PRIOR_WAVE,
            root,
            blockers,
            intake_root,
        }
    }

    pub fn contains_blocker(&self, blocker: DenialBlockerKind) -> bool {
        self.blockers.iter().any(|entry| entry.kind == blocker)
    }

    pub fn blocker_roots(&self) -> Vec<String> {
        self.blockers
            .iter()
            .map(DenialBlocker::blocker_root)
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialBlocker {
    pub kind: DenialBlockerKind,
    pub severity: Severity,
    pub denial_root: String,
    pub clear_by: ReceiptKind,
}

impl DenialBlocker {
    pub fn wave90_blockers() -> Vec<Self> {
        vec![
            Self::new(
                DenialBlockerKind::ReplayRunAbsent,
                Severity::Stop,
                "wave90-denial-replay-run-absent",
                ReceiptKind::ReplayRun,
            ),
            Self::new(
                DenialBlockerKind::RollbackDrillAbsent,
                Severity::Stop,
                "wave90-denial-rollback-drill-absent",
                ReceiptKind::RollbackDrill,
            ),
            Self::new(
                DenialBlockerKind::AdversarialReplayAbsent,
                Severity::Stop,
                "wave90-denial-adversarial-replay-absent",
                ReceiptKind::AdversarialReplay,
            ),
            Self::new(
                DenialBlockerKind::ArchiveStale,
                Severity::Stop,
                "wave90-denial-stale-archive-retained",
                ReceiptKind::StaleArchiveReplacement,
            ),
            Self::new(
                DenialBlockerKind::LiveExecutionReceiptAbsent,
                Severity::Stop,
                "wave90-denial-live-execution-receipt-absent",
                ReceiptKind::LiveExecutionReceipt,
            ),
            Self::new(
                DenialBlockerKind::OperatorSignoffAbsent,
                Severity::Stop,
                "wave90-denial-operator-signoff-absent",
                ReceiptKind::OperatorSignoff,
            ),
        ]
    }

    pub fn new(
        kind: DenialBlockerKind,
        severity: Severity,
        denial_root_seed: &str,
        clear_by: ReceiptKind,
    ) -> Self {
        let denial_root = root_of(&[denial_root_seed, kind.as_str(), clear_by.as_str()]);
        Self {
            kind,
            severity,
            denial_root,
            clear_by,
        }
    }

    pub fn blocker_root(&self) -> String {
        root_of(&[
            self.kind.as_str(),
            self.severity.as_str(),
            self.denial_root.as_str(),
            self.clear_by.as_str(),
        ])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DenialBlockerKind {
    ReplayRunAbsent,
    RollbackDrillAbsent,
    AdversarialReplayAbsent,
    ArchiveStale,
    LiveExecutionReceiptAbsent,
    OperatorSignoffAbsent,
}

impl DenialBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayRunAbsent => "replay-run-absent",
            Self::RollbackDrillAbsent => "rollback-drill-absent",
            Self::AdversarialReplayAbsent => "adversarial-replay-absent",
            Self::ArchiveStale => "archive-stale",
            Self::LiveExecutionReceiptAbsent => "live-execution-receipt-absent",
            Self::OperatorSignoffAbsent => "operator-signoff-absent",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Severity {
    Stop,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stop => "stop",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptAction {
    pub kind: ReceiptKind,
    pub summary_root: String,
    pub steps: Vec<ExecutionStep>,
    pub evidence: EvidenceRequirement,
    pub criteria: Vec<Criterion>,
}

impl ReceiptAction {
    pub fn required_live_actions() -> Vec<Self> {
        vec![
            Self::replay_run(),
            Self::rollback_drill(),
            Self::adversarial_replay(),
            Self::stale_archive_replacement(),
            Self::live_execution(),
            Self::operator_signoff(),
        ]
    }

    pub fn replay_run() -> Self {
        Self {
            kind: ReceiptKind::ReplayRun,
            summary_root: root_of(&[REPLAY_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::capture_wave90_denial(),
                ExecutionStep::load_replay_plan(),
                ExecutionStep::run_runtime_replay(),
                ExecutionStep::seal_replay_root(),
            ],
            evidence: EvidenceRequirement::live_root(REPLAY_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::NoRawMaterial,
                Criterion::OperatorVisible,
            ],
        }
    }

    pub fn rollback_drill() -> Self {
        Self {
            kind: ReceiptKind::RollbackDrill,
            summary_root: root_of(&[ROLLBACK_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::freeze_entry_state(),
                ExecutionStep::apply_rollback_drill(),
                ExecutionStep::verify_restored_state(),
                ExecutionStep::seal_rollback_root(),
            ],
            evidence: EvidenceRequirement::live_root(ROLLBACK_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::ReversibleState,
                Criterion::NoRawMaterial,
            ],
        }
    }

    pub fn adversarial_replay() -> Self {
        Self {
            kind: ReceiptKind::AdversarialReplay,
            summary_root: root_of(&[ADVERSARIAL_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::select_adversarial_schedule(),
                ExecutionStep::run_conflict_replay(),
                ExecutionStep::verify_reject_path(),
                ExecutionStep::seal_adversarial_root(),
            ],
            evidence: EvidenceRequirement::live_root(ADVERSARIAL_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::RejectsConflicts,
                Criterion::FailClosed,
            ],
        }
    }

    pub fn stale_archive_replacement() -> Self {
        Self {
            kind: ReceiptKind::StaleArchiveReplacement,
            summary_root: root_of(&[ARCHIVE_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::mark_stale_archive(),
                ExecutionStep::publish_replacement_archive(),
                ExecutionStep::verify_archive_root(),
                ExecutionStep::seal_archive_root(),
            ],
            evidence: EvidenceRequirement::live_root(ARCHIVE_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::ArchiveReplaced,
                Criterion::NoRawMaterial,
            ],
        }
    }

    pub fn live_execution() -> Self {
        Self {
            kind: ReceiptKind::LiveExecutionReceipt,
            summary_root: root_of(&[EXECUTION_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::open_live_lane(),
                ExecutionStep::execute_heavy_gate(),
                ExecutionStep::record_runtime_result(),
                ExecutionStep::seal_execution_root(),
            ],
            evidence: EvidenceRequirement::live_root(EXECUTION_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::LiveRunComplete,
                Criterion::OperatorVisible,
            ],
        }
    }

    pub fn operator_signoff() -> Self {
        Self {
            kind: ReceiptKind::OperatorSignoff,
            summary_root: root_of(&[SIGNOFF_ROOT, "plan"]),
            steps: vec![
                ExecutionStep::review_all_receipts(),
                ExecutionStep::verify_fail_closed_verdict(),
                ExecutionStep::record_operator_approval(),
                ExecutionStep::seal_signoff_root(),
            ],
            evidence: EvidenceRequirement::live_root(SIGNOFF_ROOT),
            criteria: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::OperatorVisible,
                Criterion::FailClosed,
            ],
        }
    }

    pub fn planned_receipt(&self) -> PlannedReplayReceipt {
        PlannedReplayReceipt {
            kind: self.kind,
            source: ReceiptSource::Live,
            action_root: self.root(),
            step_roots: self.steps.iter().map(ExecutionStep::step_root).collect(),
            evidence_root: self.evidence.requirement_root(),
            criteria_roots: self
                .criteria
                .iter()
                .map(|criterion| root_of(&["criterion", criterion.as_str()]))
                .collect(),
            public_redaction: PublicRedaction::RootsOnly,
            status: ReceiptStatus::Sealed,
        }
    }

    pub fn root(&self) -> String {
        root_of(&[
            self.kind.as_str(),
            self.summary_root.as_str(),
            &join_roots(self.steps.iter().map(ExecutionStep::step_root)),
            self.evidence.requirement_root().as_str(),
            &join_roots(
                self.criteria
                    .iter()
                    .map(|criterion| root_of(&["criterion", criterion.as_str()])),
            ),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedReplayReceipt {
    pub kind: ReceiptKind,
    pub source: ReceiptSource,
    pub action_root: String,
    pub step_roots: Vec<String>,
    pub evidence_root: String,
    pub criteria_roots: Vec<String>,
    pub public_redaction: PublicRedaction,
    pub status: ReceiptStatus,
}

impl PlannedReplayReceipt {
    pub fn replay_run() -> Self {
        ReceiptAction::replay_run().planned_receipt()
    }

    pub fn rollback_drill() -> Self {
        ReceiptAction::rollback_drill().planned_receipt()
    }

    pub fn adversarial_replay() -> Self {
        ReceiptAction::adversarial_replay().planned_receipt()
    }

    pub fn stale_archive_replacement() -> Self {
        ReceiptAction::stale_archive_replacement().planned_receipt()
    }

    pub fn live_execution() -> Self {
        ReceiptAction::live_execution().planned_receipt()
    }

    pub fn operator_signoff() -> Self {
        ReceiptAction::operator_signoff().planned_receipt()
    }

    pub fn receipt_root(&self) -> String {
        root_of(&[
            self.kind.as_str(),
            self.source.as_str(),
            self.action_root.as_str(),
            &join_roots(self.step_roots.iter().cloned()),
            self.evidence_root.as_str(),
            &join_roots(self.criteria_roots.iter().cloned()),
            self.public_redaction.as_str(),
            self.status.as_str(),
        ])
    }

    pub fn public_only(&self) -> bool {
        self.public_redaction == PublicRedaction::RootsOnly
    }

    pub fn is_sealed(&self) -> bool {
        self.status == ReceiptStatus::Sealed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionStep {
    pub kind: ExecutionStepKind,
    pub command_hint_root: String,
    pub input_root: String,
    pub output_root: String,
    pub gate: GateMode,
}

impl ExecutionStep {
    pub fn new(kind: ExecutionStepKind, gate: GateMode) -> Self {
        Self {
            kind,
            command_hint_root: root_of(&["command-hint", kind.as_str(), gate.as_str()]),
            input_root: root_of(&["input", kind.as_str(), "roots-only"]),
            output_root: root_of(&["output", kind.as_str(), "receipt-root"]),
            gate,
        }
    }

    pub fn step_root(&self) -> String {
        root_of(&[
            self.kind.as_str(),
            self.command_hint_root.as_str(),
            self.input_root.as_str(),
            self.output_root.as_str(),
            self.gate.as_str(),
        ])
    }

    pub fn capture_wave90_denial() -> Self {
        Self::new(ExecutionStepKind::CaptureWave90Denial, GateMode::ReadOnly)
    }

    pub fn load_replay_plan() -> Self {
        Self::new(ExecutionStepKind::LoadReplayPlan, GateMode::ReadOnly)
    }

    pub fn run_runtime_replay() -> Self {
        Self::new(ExecutionStepKind::RunRuntimeReplay, GateMode::Live)
    }

    pub fn seal_replay_root() -> Self {
        Self::new(ExecutionStepKind::SealReplayRoot, GateMode::Seal)
    }

    pub fn freeze_entry_state() -> Self {
        Self::new(ExecutionStepKind::FreezeEntryState, GateMode::ReadOnly)
    }

    pub fn apply_rollback_drill() -> Self {
        Self::new(ExecutionStepKind::ApplyRollbackDrill, GateMode::Live)
    }

    pub fn verify_restored_state() -> Self {
        Self::new(ExecutionStepKind::VerifyRestoredState, GateMode::ReadOnly)
    }

    pub fn seal_rollback_root() -> Self {
        Self::new(ExecutionStepKind::SealRollbackRoot, GateMode::Seal)
    }

    pub fn select_adversarial_schedule() -> Self {
        Self::new(
            ExecutionStepKind::SelectAdversarialSchedule,
            GateMode::ReadOnly,
        )
    }

    pub fn run_conflict_replay() -> Self {
        Self::new(ExecutionStepKind::RunConflictReplay, GateMode::Live)
    }

    pub fn verify_reject_path() -> Self {
        Self::new(ExecutionStepKind::VerifyRejectPath, GateMode::ReadOnly)
    }

    pub fn seal_adversarial_root() -> Self {
        Self::new(ExecutionStepKind::SealAdversarialRoot, GateMode::Seal)
    }

    pub fn mark_stale_archive() -> Self {
        Self::new(ExecutionStepKind::MarkStaleArchive, GateMode::ReadOnly)
    }

    pub fn publish_replacement_archive() -> Self {
        Self::new(ExecutionStepKind::PublishReplacementArchive, GateMode::Live)
    }

    pub fn verify_archive_root() -> Self {
        Self::new(ExecutionStepKind::VerifyArchiveRoot, GateMode::ReadOnly)
    }

    pub fn seal_archive_root() -> Self {
        Self::new(ExecutionStepKind::SealArchiveRoot, GateMode::Seal)
    }

    pub fn open_live_lane() -> Self {
        Self::new(ExecutionStepKind::OpenLiveLane, GateMode::Live)
    }

    pub fn execute_heavy_gate() -> Self {
        Self::new(ExecutionStepKind::ExecuteHeavyGate, GateMode::Live)
    }

    pub fn record_runtime_result() -> Self {
        Self::new(ExecutionStepKind::RecordRuntimeResult, GateMode::ReadOnly)
    }

    pub fn seal_execution_root() -> Self {
        Self::new(ExecutionStepKind::SealExecutionRoot, GateMode::Seal)
    }

    pub fn review_all_receipts() -> Self {
        Self::new(ExecutionStepKind::ReviewAllReceipts, GateMode::ReadOnly)
    }

    pub fn verify_fail_closed_verdict() -> Self {
        Self::new(
            ExecutionStepKind::VerifyFailClosedVerdict,
            GateMode::ReadOnly,
        )
    }

    pub fn record_operator_approval() -> Self {
        Self::new(ExecutionStepKind::RecordOperatorApproval, GateMode::Seal)
    }

    pub fn seal_signoff_root() -> Self {
        Self::new(ExecutionStepKind::SealSignoffRoot, GateMode::Seal)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExecutionStepKind {
    CaptureWave90Denial,
    LoadReplayPlan,
    RunRuntimeReplay,
    SealReplayRoot,
    FreezeEntryState,
    ApplyRollbackDrill,
    VerifyRestoredState,
    SealRollbackRoot,
    SelectAdversarialSchedule,
    RunConflictReplay,
    VerifyRejectPath,
    SealAdversarialRoot,
    MarkStaleArchive,
    PublishReplacementArchive,
    VerifyArchiveRoot,
    SealArchiveRoot,
    OpenLiveLane,
    ExecuteHeavyGate,
    RecordRuntimeResult,
    SealExecutionRoot,
    ReviewAllReceipts,
    VerifyFailClosedVerdict,
    RecordOperatorApproval,
    SealSignoffRoot,
}

impl ExecutionStepKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CaptureWave90Denial => "capture-wave90-denial",
            Self::LoadReplayPlan => "load-replay-plan",
            Self::RunRuntimeReplay => "run-runtime-replay",
            Self::SealReplayRoot => "seal-replay-root",
            Self::FreezeEntryState => "freeze-entry-state",
            Self::ApplyRollbackDrill => "apply-rollback-drill",
            Self::VerifyRestoredState => "verify-restored-state",
            Self::SealRollbackRoot => "seal-rollback-root",
            Self::SelectAdversarialSchedule => "select-adversarial-schedule",
            Self::RunConflictReplay => "run-conflict-replay",
            Self::VerifyRejectPath => "verify-reject-path",
            Self::SealAdversarialRoot => "seal-adversarial-root",
            Self::MarkStaleArchive => "mark-stale-archive",
            Self::PublishReplacementArchive => "publish-replacement-archive",
            Self::VerifyArchiveRoot => "verify-archive-root",
            Self::SealArchiveRoot => "seal-archive-root",
            Self::OpenLiveLane => "open-live-lane",
            Self::ExecuteHeavyGate => "execute-heavy-gate",
            Self::RecordRuntimeResult => "record-runtime-result",
            Self::SealExecutionRoot => "seal-execution-root",
            Self::ReviewAllReceipts => "review-all-receipts",
            Self::VerifyFailClosedVerdict => "verify-fail-closed-verdict",
            Self::RecordOperatorApproval => "record-operator-approval",
            Self::SealSignoffRoot => "seal-signoff-root",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiptKind {
    ReplayRun,
    RollbackDrill,
    AdversarialReplay,
    StaleArchiveReplacement,
    LiveExecutionReceipt,
    OperatorSignoff,
}

impl ReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayRun => "replay-run",
            Self::RollbackDrill => "rollback-drill",
            Self::AdversarialReplay => "adversarial-replay",
            Self::StaleArchiveReplacement => "stale-archive-replacement",
            Self::LiveExecutionReceipt => "live-execution-receipt",
            Self::OperatorSignoff => "operator-signoff",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceiptSource {
    Live,
}

impl ReceiptSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceiptStatus {
    Sealed,
    Draft,
    Rejected,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sealed => "sealed",
            Self::Draft => "draft",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicRedaction {
    RootsOnly,
}

impl PublicRedaction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootsOnly => "roots-only",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GateMode {
    ReadOnly,
    Live,
    Seal,
}

impl GateMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::Live => "live",
            Self::Seal => "seal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRequirement {
    pub source_root: String,
    pub retention: RetentionMode,
    pub redaction: PublicRedaction,
    pub freshness: Freshness,
}

impl EvidenceRequirement {
    pub fn live_root(seed: &str) -> Self {
        Self {
            source_root: root_of(&[seed, "live-source-root"]),
            retention: RetentionMode::PublicRootPrivateOperationalRecord,
            redaction: PublicRedaction::RootsOnly,
            freshness: Freshness::LiveRun,
        }
    }

    pub fn requirement_root(&self) -> String {
        root_of(&[
            self.source_root.as_str(),
            self.retention.as_str(),
            self.redaction.as_str(),
            self.freshness.as_str(),
        ])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RetentionMode {
    PublicRootPrivateOperationalRecord,
}

impl RetentionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PublicRootPrivateOperationalRecord => "public-root-private-operational-record",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Freshness {
    LiveRun,
}

impl Freshness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveRun => "live-run",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceCriteria {
    pub receipt_must_be_live: bool,
    pub receipt_must_be_sealed: bool,
    pub redaction: PublicRedaction,
    pub required: Vec<Criterion>,
}

impl AcceptanceCriteria {
    pub fn strict_live() -> Self {
        Self {
            receipt_must_be_live: true,
            receipt_must_be_sealed: true,
            redaction: PublicRedaction::RootsOnly,
            required: vec![
                Criterion::ReceiptPresent,
                Criterion::RootMatchesPlan,
                Criterion::NoRawMaterial,
                Criterion::FailClosed,
            ],
        }
    }

    pub fn check_receipt(&self, receipt: &PlannedReplayReceipt) -> Result<()> {
        if self.receipt_must_be_live && receipt.source != ReceiptSource::Live {
            return Err(RuntimeError::NonLiveReceipt(receipt.kind));
        }
        if self.receipt_must_be_sealed && !receipt.is_sealed() {
            return Err(RuntimeError::UnsealedReceipt(receipt.kind));
        }
        if receipt.public_redaction != self.redaction {
            return Err(RuntimeError::PrivacyBreach(receipt.kind));
        }
        if receipt.step_roots.is_empty() || receipt.criteria_roots.is_empty() {
            return Err(RuntimeError::IncompleteReceipt(receipt.kind));
        }
        Ok(())
    }

    pub fn receipt_satisfies(&self, receipt: &PlannedReplayReceipt) -> bool {
        self.check_receipt(receipt).is_ok()
    }

    pub fn criteria_root(&self) -> String {
        root_of(&[
            bool_word(self.receipt_must_be_live),
            bool_word(self.receipt_must_be_sealed),
            self.redaction.as_str(),
            &join_roots(
                self.required
                    .iter()
                    .map(|criterion| root_of(&["required", criterion.as_str()])),
            ),
        ])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Criterion {
    ReceiptPresent,
    RootMatchesPlan,
    NoRawMaterial,
    OperatorVisible,
    ReversibleState,
    RejectsConflicts,
    FailClosed,
    ArchiveReplaced,
    LiveRunComplete,
}

impl Criterion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReceiptPresent => "receipt-present",
            Self::RootMatchesPlan => "root-matches-plan",
            Self::NoRawMaterial => "no-raw-material",
            Self::OperatorVisible => "operator-visible",
            Self::ReversibleState => "reversible-state",
            Self::RejectsConflicts => "rejects-conflicts",
            Self::FailClosed => "fail-closed",
            Self::ArchiveReplaced => "archive-replaced",
            Self::LiveRunComplete => "live-run-complete",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorCommandHint {
    pub kind: ReceiptKind,
    pub command_root: String,
    pub purpose_root: String,
    pub fail_closed_note_root: String,
}

impl OperatorCommandHint {
    pub fn new(kind: ReceiptKind, purpose: &str) -> Self {
        Self {
            kind,
            command_root: root_of(&["operator-command", kind.as_str(), "roots-only"]),
            purpose_root: root_of(&["operator-purpose", purpose, kind.as_str()]),
            fail_closed_note_root: root_of(&["fail-closed-note", kind.as_str()]),
        }
    }

    pub fn default_hints() -> Vec<Self> {
        vec![
            Self::new(ReceiptKind::ReplayRun, "run-live-runtime-replay"),
            Self::new(ReceiptKind::RollbackDrill, "run-rollback-drill"),
            Self::new(ReceiptKind::AdversarialReplay, "run-adversarial-replay"),
            Self::new(
                ReceiptKind::StaleArchiveReplacement,
                "replace-stale-archive",
            ),
            Self::new(ReceiptKind::LiveExecutionReceipt, "record-live-execution"),
            Self::new(ReceiptKind::OperatorSignoff, "record-operator-signoff"),
        ]
    }

    pub fn hint_root(&self) -> String {
        root_of(&[
            self.kind.as_str(),
            self.command_root.as_str(),
            self.purpose_root.as_str(),
            self.fail_closed_note_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailClosedPolicy {
    pub block_on_missing_receipt: bool,
    pub block_on_unsealed_receipt: bool,
    pub block_on_non_live_receipt: bool,
    pub block_on_private_material: bool,
    pub clearance_kind: ClearanceKind,
}

impl FailClosedPolicy {
    pub fn strict() -> Self {
        Self {
            block_on_missing_receipt: true,
            block_on_unsealed_receipt: true,
            block_on_non_live_receipt: true,
            block_on_private_material: true,
            clearance_kind: ClearanceKind::AllReceiptsSealed,
        }
    }

    pub fn policy_root(&self) -> String {
        root_of(&[
            bool_word(self.block_on_missing_receipt),
            bool_word(self.block_on_unsealed_receipt),
            bool_word(self.block_on_non_live_receipt),
            bool_word(self.block_on_private_material),
            self.clearance_kind.as_str(),
        ])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClearanceKind {
    AllReceiptsSealed,
}

impl ClearanceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllReceiptsSealed => "all-receipts-sealed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClearanceVerdict {
    Cleared {
        clearance_root: String,
        receipt_root: String,
    },
    Blocked {
        reason_root: String,
        missing: Vec<ReceiptKind>,
    },
    Rejected {
        reason_root: String,
        failed: Vec<ReceiptKind>,
    },
}

impl ClearanceVerdict {
    pub fn is_cleared(&self) -> bool {
        matches!(self, Self::Cleared { .. })
    }

    pub fn verdict_root(&self) -> String {
        match self {
            Self::Cleared {
                clearance_root,
                receipt_root,
            } => root_of(&["cleared", clearance_root.as_str(), receipt_root.as_str()]),
            Self::Blocked {
                reason_root,
                missing,
            } => root_of(&["blocked", reason_root.as_str(), &join_kinds(missing)]),
            Self::Rejected {
                reason_root,
                failed,
            } => root_of(&["rejected", reason_root.as_str(), &join_kinds(failed)]),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAudit {
    pub config_root: String,
    pub denial_root: String,
    pub receipt_root: String,
    pub clearance_root: String,
    pub command_root: String,
}

impl RuntimeAudit {
    pub fn from_config(config: &Config) -> Self {
        let receipt_root = root_of(&["audit", "no-receipts"]);
        let clearance_root = root_of(&["audit", "blocked"]);
        let command_root = join_roots(
            config
                .command_hints
                .iter()
                .map(OperatorCommandHint::hint_root),
        );
        Self {
            config_root: config.config_root(),
            denial_root: config.denial_intake.root.clone(),
            receipt_root,
            clearance_root,
            command_root,
        }
    }

    pub fn from_state(state: &State) -> Self {
        let command_root = join_roots(
            state
                .config
                .command_hints
                .iter()
                .map(OperatorCommandHint::hint_root),
        );
        Self {
            config_root: state.config.config_root(),
            denial_root: state.config.denial_intake.root.clone(),
            receipt_root: state.receipts_root(),
            clearance_root: state.clearance.verdict_root(),
            command_root,
        }
    }

    pub fn audit_root(&self) -> String {
        root_of(&[
            self.config_root.as_str(),
            self.denial_root.as_str(),
            self.receipt_root.as_str(),
            self.clearance_root.as_str(),
            self.command_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    UnknownReceiptKind(ReceiptKind),
    NonLiveReceipt(ReceiptKind),
    UnsealedReceipt(ReceiptKind),
    PrivacyBreach(ReceiptKind),
    IncompleteReceipt(ReceiptKind),
}

impl RuntimeError {
    pub fn error_root(&self) -> String {
        match self {
            Self::UnknownReceiptKind(kind) => root_of(&["unknown-receipt-kind", kind.as_str()]),
            Self::NonLiveReceipt(kind) => root_of(&["non-live-receipt", kind.as_str()]),
            Self::UnsealedReceipt(kind) => root_of(&["unsealed-receipt", kind.as_str()]),
            Self::PrivacyBreach(kind) => root_of(&["privacy-breach", kind.as_str()]),
            Self::IncompleteReceipt(kind) => root_of(&["incomplete-receipt", kind.as_str()]),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownReceiptKind(kind) => {
                write!(f, "unknown receipt kind: {}", kind.as_str())
            }
            Self::NonLiveReceipt(kind) => {
                write!(f, "receipt is not live: {}", kind.as_str())
            }
            Self::UnsealedReceipt(kind) => {
                write!(f, "receipt is not sealed: {}", kind.as_str())
            }
            Self::PrivacyBreach(kind) => {
                write!(f, "receipt is not roots-only: {}", kind.as_str())
            }
            Self::IncompleteReceipt(kind) => {
                write!(f, "receipt lacks required roots: {}", kind.as_str())
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

pub fn devnet() -> Runtime {
    Runtime::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    root_of(&[MODULE_ID, ROOT_PREFIX, &WAVE.to_string(), "runtime"])
}

pub fn planned_actions() -> Vec<ReceiptAction> {
    ReceiptAction::required_live_actions()
}

pub fn denial_root_intake() -> DenialRootIntake {
    DenialRootIntake::wave90()
}

pub fn fail_closed_clearance_for(receipts: Vec<PlannedReplayReceipt>) -> Result<ClearanceVerdict> {
    let mut runtime = devnet();
    for receipt in receipts {
        runtime.insert_receipt(receipt)?;
    }
    Ok(runtime.clearance)
}

pub fn live_clearance_record() -> Result<PublicRecord> {
    let runtime = devnet().clear_with_all_live_receipts()?;
    Ok(runtime.public_record())
}

fn root_of(parts: &[&str]) -> String {
    let mut state = RootState::new();
    for part in parts {
        state.mix(part.as_bytes());
        state.mix(&[0x1f]);
    }
    format!("{}-{:016x}{:016x}", ROOT_PREFIX, state.a, state.b)
}

fn join_roots<I>(roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let mut joined = String::new();
    for root in roots {
        if !joined.is_empty() {
            joined.push('|');
        }
        joined.push_str(&root);
    }
    root_of(&["joined-roots", joined.as_str()])
}

fn join_kinds(kinds: &[ReceiptKind]) -> String {
    let mut joined = String::new();
    for kind in kinds {
        if !joined.is_empty() {
            joined.push('|');
        }
        joined.push_str(kind.as_str());
    }
    root_of(&["joined-kinds", joined.as_str()])
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[derive(Clone, Debug)]
struct RootState {
    a: u64,
    b: u64,
}

impl RootState {
    fn new() -> Self {
        Self {
            a: 0xcbf2_9ce4_8422_2325,
            b: 0x9e37_79b9_7f4a_7c15,
        }
    }

    fn mix(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.a ^= u64::from(*byte);
            self.a = self.a.wrapping_mul(0x0000_0100_0000_01b3);
            self.b ^= self.a.rotate_left(13);
            self.b = self.b.wrapping_mul(0xff51_afd7_ed55_8ccd);
            self.b ^= self.b >> 33;
        }
    }
}
