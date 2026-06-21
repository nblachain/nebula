// Wave 92 live heavy-gate receipt slot registry runtime replay lane.
//
// This module is intentionally self-contained, std-only, and roots-only.  It
// consumes the shape of Wave 91 runtime replay execution-plan output without
// importing raw operational material.  All receipt slots start empty and the
// registry remains fail-closed until every canonical future live accepted
// receipt root is attached under the import rules below.

use std::collections::BTreeMap;
use std::fmt;

pub type Result<T> = std::result::Result<T, RuntimeError>;
pub type Runtime = State;

const MODULE_ID: &str = "monero_l2_pq_bridge_exit_force_exit_wave92_live_heavy_gate_receipt_slot_registry_runtime_replay_slot_runtime";
const PROTOCOL_VERSION: &str =
    "wave92-live-heavy-gate-receipt-slot-registry-runtime-replay-slot-v1";
const HASH_DOMAIN: &str = "dinero.wave92.receipt.slot.registry.runtime.replay";
const WAVE: u16 = 92;
const SOURCE_WAVE: u16 = 91;
const SLOT_COUNT: usize = 6;
const EMPTY_ROOT: &str = "empty";

#[derive(Clone, Eq, PartialEq)]
pub struct Config {
    pub module_id: String,
    pub protocol_version: String,
    pub wave: u16,
    pub source_wave: u16,
    pub lane: ReplayLane,
    pub plan_intake: PlanRootIntake,
    pub import_rule: ImportRule,
    pub clear_rule: ClearRule,
    pub command_hints: Vec<OperatorCommandHint>,
}

impl Config {
    pub fn live() -> Self {
        Self {
            module_id: MODULE_ID.to_owned(),
            protocol_version: PROTOCOL_VERSION.to_owned(),
            wave: WAVE,
            source_wave: SOURCE_WAVE,
            lane: ReplayLane::LiveHeavyGateReceiptSlotRegistry,
            plan_intake: PlanRootIntake::wave91_runtime_replay(),
            import_rule: ImportRule::strict_future_live(),
            clear_rule: ClearRule::all_slots_with_live_roots(),
            command_hints: OperatorCommandHint::default_hints(),
        }
    }

    pub fn config_root(&self) -> String {
        root_of(&[
            "config",
            self.module_id.as_str(),
            self.protocol_version.as_str(),
            self.wave.to_string().as_str(),
            self.source_wave.to_string().as_str(),
            self.lane.as_str(),
            self.plan_intake.root().as_str(),
            self.import_rule.rule_root().as_str(),
            self.clear_rule.rule_root().as_str(),
            join_roots(self.command_hints.iter().map(OperatorCommandHint::root)).as_str(),
        ])
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::live()
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("module_id", &self.module_id)
            .field("protocol_version", &self.protocol_version)
            .field("wave", &self.wave)
            .field("source_wave", &self.source_wave)
            .field("lane", &self.lane)
            .field("plan_intake_root", &self.plan_intake.root())
            .field("import_rule_root", &self.import_rule.rule_root())
            .field("clear_rule_root", &self.clear_rule.rule_root())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub slots: BTreeMap<ReceiptSlotKind, ReceiptSlot>,
    pub blockers: Vec<ReplaySlotBlocker>,
    pub verdict: FailClosedSlotRegistryVerdict,
    pub audit: RegistryAudit,
}

impl State {
    pub fn new(config: Config) -> Self {
        let slots = ReceiptSlotKind::all()
            .into_iter()
            .map(|kind| (kind, ReceiptSlot::empty(kind)))
            .collect::<BTreeMap<_, _>>();
        let blockers = ReplaySlotBlocker::initial_blockers(&slots);
        let verdict = FailClosedSlotRegistryVerdict::blocked(&blockers);
        let audit = RegistryAudit::from_config_and_slots(&config, &slots, &blockers);

        Self {
            config,
            slots,
            blockers,
            verdict,
            audit,
        }
    }

    pub fn attach_future_live_root(
        &mut self,
        slot: ReceiptSlotKind,
        receipt: AcceptedReceiptPlaceholder,
    ) -> Result<()> {
        self.config.import_rule.check(slot, &receipt)?;
        match self.slots.get_mut(&slot) {
            Some(existing) => existing.attach(receipt)?,
            None => return Err(RuntimeError::UnknownSlot(slot)),
        }
        self.recompute();
        Ok(())
    }

    pub fn with_future_live_root(
        mut self,
        slot: ReceiptSlotKind,
        receipt: AcceptedReceiptPlaceholder,
    ) -> Result<Self> {
        self.attach_future_live_root(slot, receipt)?;
        Ok(self)
    }

    pub fn attach_canonical_future_roots(&mut self) -> Result<()> {
        for kind in ReceiptSlotKind::all() {
            self.attach_future_live_root(kind, AcceptedReceiptPlaceholder::canonical(kind))?;
        }
        Ok(())
    }

    pub fn with_canonical_future_roots(mut self) -> Result<Self> {
        self.attach_canonical_future_roots()?;
        Ok(self)
    }

    pub fn slot(&self, kind: ReceiptSlotKind) -> Option<&ReceiptSlot> {
        self.slots.get(&kind)
    }

    pub fn is_clearable(&self) -> bool {
        matches!(
            self.verdict,
            FailClosedSlotRegistryVerdict::Clearable { .. }
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        let slot_roots = self
            .slots
            .iter()
            .map(|(kind, slot)| (*kind, slot.public_root()))
            .collect::<BTreeMap<_, _>>();
        PublicRecord {
            module_root: state_root(),
            protocol_version: self.config.protocol_version.clone(),
            wave: self.config.wave,
            source_wave: self.config.source_wave,
            plan_intake_root: self.config.plan_intake.root(),
            registry_root: self.registry_root(),
            slot_roots,
            blocker_root: join_roots(self.blockers.iter().map(ReplaySlotBlocker::root)),
            verdict: self.verdict.clone(),
            command_hint_root: join_roots(
                self.config
                    .command_hints
                    .iter()
                    .map(OperatorCommandHint::root),
            ),
            audit_root: self.audit.audit_root(),
        }
    }

    pub fn registry_root(&self) -> String {
        root_of(&[
            "registry",
            self.config.config_root().as_str(),
            join_roots(self.slots.values().map(ReceiptSlot::public_root)).as_str(),
            join_roots(self.blockers.iter().map(ReplaySlotBlocker::root)).as_str(),
            self.verdict.root().as_str(),
        ])
    }

    pub fn empty_slot_count(&self) -> usize {
        self.slots.values().filter(|slot| slot.is_empty()).count()
    }

    pub fn attached_slot_count(&self) -> usize {
        self.slots
            .values()
            .filter(|slot| slot.is_attached())
            .count()
    }

    pub fn missing_slots(&self) -> Vec<ReceiptSlotKind> {
        self.slots
            .iter()
            .filter(|(_, slot)| !slot.is_attached())
            .map(|(kind, _)| *kind)
            .collect()
    }

    fn recompute(&mut self) {
        self.blockers = ReplaySlotBlocker::from_slots(&self.slots, &self.config.clear_rule);
        self.verdict = if self.blockers.is_empty() {
            FailClosedSlotRegistryVerdict::clearable(self.registry_clearance_root())
        } else {
            FailClosedSlotRegistryVerdict::blocked(&self.blockers)
        };
        self.audit =
            RegistryAudit::from_config_and_slots(&self.config, &self.slots, &self.blockers);
    }

    fn registry_clearance_root(&self) -> String {
        root_of(&[
            "clearance",
            self.config.plan_intake.root().as_str(),
            join_roots(self.slots.values().map(ReceiptSlot::attached_root)).as_str(),
            self.config.clear_rule.rule_root().as_str(),
        ])
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("config", &self.config)
            .field("slot_count", &self.slots.len())
            .field("empty_slot_count", &self.empty_slot_count())
            .field("attached_slot_count", &self.attached_slot_count())
            .field("blockers", &self.blockers)
            .field("verdict", &self.verdict)
            .field("audit_root", &self.audit.audit_root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReplayLane {
    LiveHeavyGateReceiptSlotRegistry,
}

impl ReplayLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveHeavyGateReceiptSlotRegistry => "live_heavy_gate_receipt_slot_registry",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PlanRootIntake {
    pub source_wave: u16,
    pub lane: Wave91ExecutionPlanLane,
    pub plan_root: String,
    pub replay_run_plan_root: String,
    pub rollback_drill_plan_root: String,
    pub adversarial_replay_plan_root: String,
    pub stale_archive_replacement_plan_root: String,
    pub live_execution_plan_root: String,
    pub operator_signoff_plan_root: String,
    pub transcript_root: String,
}

impl PlanRootIntake {
    pub fn wave91_runtime_replay() -> Self {
        let replay_run_plan_root = seeded_root("wave91-runtime-replay-run-plan");
        let rollback_drill_plan_root = seeded_root("wave91-rollback-drill-plan");
        let adversarial_replay_plan_root = seeded_root("wave91-adversarial-replay-plan");
        let stale_archive_replacement_plan_root =
            seeded_root("wave91-stale-archive-replacement-plan");
        let live_execution_plan_root = seeded_root("wave91-live-execution-receipt-plan");
        let operator_signoff_plan_root = seeded_root("wave91-operator-signoff-plan");
        let transcript_root =
            seeded_root("wave91-live-heavy-gate-execution-plan-runtime-replay-transcript");
        let plan_root = root_of(&[
            "wave91-plan-root-intake",
            replay_run_plan_root.as_str(),
            rollback_drill_plan_root.as_str(),
            adversarial_replay_plan_root.as_str(),
            stale_archive_replacement_plan_root.as_str(),
            live_execution_plan_root.as_str(),
            operator_signoff_plan_root.as_str(),
            transcript_root.as_str(),
        ]);

        Self {
            source_wave: SOURCE_WAVE,
            lane: Wave91ExecutionPlanLane::RuntimeReplayReceipt,
            plan_root,
            replay_run_plan_root,
            rollback_drill_plan_root,
            adversarial_replay_plan_root,
            stale_archive_replacement_plan_root,
            live_execution_plan_root,
            operator_signoff_plan_root,
            transcript_root,
        }
    }

    pub fn root(&self) -> String {
        root_of(&[
            "plan-intake",
            self.source_wave.to_string().as_str(),
            self.lane.as_str(),
            self.plan_root.as_str(),
            self.replay_run_plan_root.as_str(),
            self.rollback_drill_plan_root.as_str(),
            self.adversarial_replay_plan_root.as_str(),
            self.stale_archive_replacement_plan_root.as_str(),
            self.live_execution_plan_root.as_str(),
            self.operator_signoff_plan_root.as_str(),
            self.transcript_root.as_str(),
        ])
    }

    pub fn plan_root_for_slot(&self, kind: ReceiptSlotKind) -> &str {
        match kind {
            ReceiptSlotKind::ReplayRun => self.replay_run_plan_root.as_str(),
            ReceiptSlotKind::RollbackDrill => self.rollback_drill_plan_root.as_str(),
            ReceiptSlotKind::AdversarialReplay => self.adversarial_replay_plan_root.as_str(),
            ReceiptSlotKind::StaleArchiveReplacement => {
                self.stale_archive_replacement_plan_root.as_str()
            }
            ReceiptSlotKind::LiveExecutionReceipt => self.live_execution_plan_root.as_str(),
            ReceiptSlotKind::OperatorSignoff => self.operator_signoff_plan_root.as_str(),
        }
    }
}

impl fmt::Debug for PlanRootIntake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlanRootIntake")
            .field("source_wave", &self.source_wave)
            .field("lane", &self.lane)
            .field("root", &self.root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Wave91ExecutionPlanLane {
    RuntimeReplayReceipt,
}

impl Wave91ExecutionPlanLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplayReceipt => "runtime_replay_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReceiptSlotKind {
    ReplayRun,
    RollbackDrill,
    AdversarialReplay,
    StaleArchiveReplacement,
    LiveExecutionReceipt,
    OperatorSignoff,
}

impl ReceiptSlotKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReplayRun,
            Self::RollbackDrill,
            Self::AdversarialReplay,
            Self::StaleArchiveReplacement,
            Self::LiveExecutionReceipt,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayRun => "replay_run",
            Self::RollbackDrill => "rollback_drill",
            Self::AdversarialReplay => "adversarial_replay",
            Self::StaleArchiveReplacement => "stale_archive_replacement",
            Self::LiveExecutionReceipt => "live_execution_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn canonical_plan_seed(self) -> &'static str {
        match self {
            Self::ReplayRun => "wave91-runtime-replay-run-plan",
            Self::RollbackDrill => "wave91-rollback-drill-plan",
            Self::AdversarialReplay => "wave91-adversarial-replay-plan",
            Self::StaleArchiveReplacement => "wave91-stale-archive-replacement-plan",
            Self::LiveExecutionReceipt => "wave91-live-execution-receipt-plan",
            Self::OperatorSignoff => "wave91-operator-signoff-plan",
        }
    }

    pub fn canonical_receipt_seed(self) -> &'static str {
        match self {
            Self::ReplayRun => "wave92-future-live-accepted-replay-run-receipt-root",
            Self::RollbackDrill => "wave92-future-live-accepted-rollback-drill-receipt-root",
            Self::AdversarialReplay => {
                "wave92-future-live-accepted-adversarial-replay-receipt-root"
            }
            Self::StaleArchiveReplacement => {
                "wave92-future-live-accepted-stale-archive-replacement-receipt-root"
            }
            Self::LiveExecutionReceipt => "wave92-future-live-accepted-live-execution-receipt-root",
            Self::OperatorSignoff => "wave92-future-live-accepted-operator-signoff-receipt-root",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ReceiptSlot {
    pub kind: ReceiptSlotKind,
    pub status: ReceiptSlotStatus,
    pub plan_root: String,
    pub accepted_receipt: Option<AcceptedReceiptPlaceholder>,
    pub clearable_only_after_live_root: bool,
}

impl ReceiptSlot {
    pub fn empty(kind: ReceiptSlotKind) -> Self {
        Self {
            kind,
            status: ReceiptSlotStatus::EmptyFailClosed,
            plan_root: seeded_root(kind.canonical_plan_seed()),
            accepted_receipt: None,
            clearable_only_after_live_root: true,
        }
    }

    pub fn attach(&mut self, receipt: AcceptedReceiptPlaceholder) -> Result<()> {
        if self.is_attached() {
            return Err(RuntimeError::ReceiptRootAlreadyAttached(self.kind));
        }
        if receipt.kind != self.kind {
            return Err(RuntimeError::SlotMismatch {
                slot: self.kind,
                receipt: receipt.kind,
            });
        }
        if !receipt.is_live_accepted_root() {
            return Err(RuntimeError::ReceiptNotLiveAccepted(self.kind));
        }
        if receipt.plan_root != self.plan_root {
            return Err(RuntimeError::PlanRootMismatch(self.kind));
        }
        self.status = ReceiptSlotStatus::FutureLiveRootAttached;
        self.accepted_receipt = Some(receipt);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.status, ReceiptSlotStatus::EmptyFailClosed)
    }

    pub fn is_attached(&self) -> bool {
        matches!(self.status, ReceiptSlotStatus::FutureLiveRootAttached)
    }

    pub fn attached_root(&self) -> String {
        match &self.accepted_receipt {
            Some(receipt) => receipt.receipt_root.clone(),
            None => seeded_root(EMPTY_ROOT),
        }
    }

    pub fn public_root(&self) -> String {
        root_of(&[
            "receipt-slot",
            self.kind.as_str(),
            self.status.as_str(),
            self.plan_root.as_str(),
            self.attached_root().as_str(),
            bool_word(self.clearable_only_after_live_root),
        ])
    }
}

impl fmt::Debug for ReceiptSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceiptSlot")
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("plan_root", &self.plan_root)
            .field("attached_root", &self.attached_root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptSlotStatus {
    EmptyFailClosed,
    FutureLiveRootAttached,
}

impl ReceiptSlotStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFailClosed => "empty_fail_closed",
            Self::FutureLiveRootAttached => "future_live_root_attached",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AcceptedReceiptPlaceholder {
    pub kind: ReceiptSlotKind,
    pub source_wave: u16,
    pub target_wave: u16,
    pub status: AcceptedReceiptStatus,
    pub plan_root: String,
    pub receipt_root: String,
    pub acceptance_root: String,
    pub privacy_root: String,
}

impl AcceptedReceiptPlaceholder {
    pub fn canonical(kind: ReceiptSlotKind) -> Self {
        let plan_root = seeded_root(kind.canonical_plan_seed());
        let receipt_root = seeded_root(kind.canonical_receipt_seed());
        let acceptance_root = root_of(&[
            "accepted-receipt",
            kind.as_str(),
            plan_root.as_str(),
            receipt_root.as_str(),
            "future-live",
        ]);
        let privacy_root = root_of(&[
            "roots-only",
            kind.as_str(),
            receipt_root.as_str(),
            "no-raw-operational-material",
        ]);

        Self {
            kind,
            source_wave: SOURCE_WAVE,
            target_wave: WAVE,
            status: AcceptedReceiptStatus::FutureLiveAccepted,
            plan_root,
            receipt_root,
            acceptance_root,
            privacy_root,
        }
    }

    pub fn placeholder_root(&self) -> String {
        root_of(&[
            "accepted-receipt-placeholder",
            self.kind.as_str(),
            self.source_wave.to_string().as_str(),
            self.target_wave.to_string().as_str(),
            self.status.as_str(),
            self.plan_root.as_str(),
            self.receipt_root.as_str(),
            self.acceptance_root.as_str(),
            self.privacy_root.as_str(),
        ])
    }

    pub fn is_live_accepted_root(&self) -> bool {
        self.source_wave == SOURCE_WAVE
            && self.target_wave == WAVE
            && matches!(self.status, AcceptedReceiptStatus::FutureLiveAccepted)
            && self.receipt_root != seeded_root(EMPTY_ROOT)
            && self.privacy_root
                == root_of(&[
                    "roots-only",
                    self.kind.as_str(),
                    self.receipt_root.as_str(),
                    "no-raw-operational-material",
                ])
    }
}

impl fmt::Debug for AcceptedReceiptPlaceholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AcceptedReceiptPlaceholder")
            .field("kind", &self.kind)
            .field("source_wave", &self.source_wave)
            .field("target_wave", &self.target_wave)
            .field("status", &self.status)
            .field("placeholder_root", &self.placeholder_root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcceptedReceiptStatus {
    FutureLiveAccepted,
    Draft,
    Rejected,
    ArchiveOnly,
}

impl AcceptedReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FutureLiveAccepted => "future_live_accepted",
            Self::Draft => "draft",
            Self::Rejected => "rejected",
            Self::ArchiveOnly => "archive_only",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ImportRule {
    pub require_source_wave: u16,
    pub require_target_wave: u16,
    pub require_status: AcceptedReceiptStatus,
    pub require_roots_only: bool,
    pub forbid_empty_root: bool,
    pub forbid_replacing_attached_roots_without_new_state: bool,
}

impl ImportRule {
    pub fn strict_future_live() -> Self {
        Self {
            require_source_wave: SOURCE_WAVE,
            require_target_wave: WAVE,
            require_status: AcceptedReceiptStatus::FutureLiveAccepted,
            require_roots_only: true,
            forbid_empty_root: true,
            forbid_replacing_attached_roots_without_new_state: true,
        }
    }

    pub fn check(&self, slot: ReceiptSlotKind, receipt: &AcceptedReceiptPlaceholder) -> Result<()> {
        if receipt.kind != slot {
            return Err(RuntimeError::SlotMismatch {
                slot,
                receipt: receipt.kind,
            });
        }
        if receipt.source_wave != self.require_source_wave {
            return Err(RuntimeError::SourceWaveBlocked {
                slot,
                wave: receipt.source_wave,
            });
        }
        if receipt.target_wave != self.require_target_wave {
            return Err(RuntimeError::TargetWaveBlocked {
                slot,
                wave: receipt.target_wave,
            });
        }
        if receipt.status != self.require_status {
            return Err(RuntimeError::ReceiptStatusBlocked(slot));
        }
        if self.forbid_empty_root && receipt.receipt_root == seeded_root(EMPTY_ROOT) {
            return Err(RuntimeError::EmptyReceiptRoot(slot));
        }
        if self.require_roots_only && !receipt.is_live_accepted_root() {
            return Err(RuntimeError::RootsOnlyPrivacyBlocked(slot));
        }
        Ok(())
    }

    pub fn rule_root(&self) -> String {
        root_of(&[
            "import-rule",
            self.require_source_wave.to_string().as_str(),
            self.require_target_wave.to_string().as_str(),
            self.require_status.as_str(),
            bool_word(self.require_roots_only),
            bool_word(self.forbid_empty_root),
            bool_word(self.forbid_replacing_attached_roots_without_new_state),
        ])
    }
}

impl fmt::Debug for ImportRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportRule")
            .field("rule_root", &self.rule_root())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ClearRule {
    pub require_all_slots: bool,
    pub require_attached_live_roots: bool,
    pub require_wave91_plan_intake: bool,
    pub required_slot_count: usize,
}

impl ClearRule {
    pub fn all_slots_with_live_roots() -> Self {
        Self {
            require_all_slots: true,
            require_attached_live_roots: true,
            require_wave91_plan_intake: true,
            required_slot_count: SLOT_COUNT,
        }
    }

    pub fn rule_root(&self) -> String {
        root_of(&[
            "clear-rule",
            bool_word(self.require_all_slots),
            bool_word(self.require_attached_live_roots),
            bool_word(self.require_wave91_plan_intake),
            self.required_slot_count.to_string().as_str(),
        ])
    }
}

impl fmt::Debug for ClearRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClearRule")
            .field("rule_root", &self.rule_root())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ReplaySlotBlocker {
    pub kind: ReceiptSlotKind,
    pub reason: ReplaySlotBlockerReason,
    pub blocker_root: String,
}

impl ReplaySlotBlocker {
    pub fn initial_blockers(slots: &BTreeMap<ReceiptSlotKind, ReceiptSlot>) -> Vec<Self> {
        slots
            .keys()
            .map(|kind| Self::missing_live_root(*kind))
            .collect()
    }

    pub fn from_slots(
        slots: &BTreeMap<ReceiptSlotKind, ReceiptSlot>,
        clear_rule: &ClearRule,
    ) -> Vec<Self> {
        let mut blockers = Vec::new();
        if clear_rule.require_all_slots && slots.len() != clear_rule.required_slot_count {
            for kind in ReceiptSlotKind::all() {
                if !slots.contains_key(&kind) {
                    blockers.push(Self::missing_slot(kind));
                }
            }
        }
        for (kind, slot) in slots {
            if slot.is_empty() {
                blockers.push(Self::missing_live_root(*kind));
            } else if clear_rule.require_attached_live_roots {
                match &slot.accepted_receipt {
                    Some(receipt) if receipt.is_live_accepted_root() => {}
                    Some(_) => blockers.push(Self::non_live_root(*kind)),
                    None => blockers.push(Self::missing_live_root(*kind)),
                }
            }
        }
        blockers
    }

    pub fn missing_slot(kind: ReceiptSlotKind) -> Self {
        Self::new(kind, ReplaySlotBlockerReason::MissingCanonicalSlot)
    }

    pub fn missing_live_root(kind: ReceiptSlotKind) -> Self {
        Self::new(
            kind,
            ReplaySlotBlockerReason::MissingFutureLiveAcceptedReceiptRoot,
        )
    }

    pub fn non_live_root(kind: ReceiptSlotKind) -> Self {
        Self::new(kind, ReplaySlotBlockerReason::AttachedRootNotLiveAccepted)
    }

    pub fn new(kind: ReceiptSlotKind, reason: ReplaySlotBlockerReason) -> Self {
        let blocker_root = root_of(&["slot-blocker", kind.as_str(), reason.as_str()]);
        Self {
            kind,
            reason,
            blocker_root,
        }
    }

    pub fn root(&self) -> String {
        root_of(&[
            "replay-slot-blocker",
            self.kind.as_str(),
            self.reason.as_str(),
            self.blocker_root.as_str(),
        ])
    }
}

impl fmt::Debug for ReplaySlotBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReplaySlotBlocker")
            .field("kind", &self.kind)
            .field("reason", &self.reason)
            .field("root", &self.root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplaySlotBlockerReason {
    MissingCanonicalSlot,
    MissingFutureLiveAcceptedReceiptRoot,
    AttachedRootNotLiveAccepted,
}

impl ReplaySlotBlockerReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingCanonicalSlot => "missing_canonical_slot",
            Self::MissingFutureLiveAcceptedReceiptRoot => {
                "missing_future_live_accepted_receipt_root"
            }
            Self::AttachedRootNotLiveAccepted => "attached_root_not_live_accepted",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct OperatorCommandHint {
    pub command: OperatorCommand,
    pub slot: ReceiptSlotKind,
    pub guard: OperatorCommandGuard,
    pub hint_root: String,
}

impl OperatorCommandHint {
    pub fn default_hints() -> Vec<Self> {
        ReceiptSlotKind::all()
            .into_iter()
            .map(|slot| Self::for_slot(slot))
            .collect()
    }

    pub fn for_slot(slot: ReceiptSlotKind) -> Self {
        let command = match slot {
            ReceiptSlotKind::ReplayRun => OperatorCommand::AttachReplayRunRoot,
            ReceiptSlotKind::RollbackDrill => OperatorCommand::AttachRollbackDrillRoot,
            ReceiptSlotKind::AdversarialReplay => OperatorCommand::AttachAdversarialReplayRoot,
            ReceiptSlotKind::StaleArchiveReplacement => {
                OperatorCommand::AttachStaleArchiveReplacementRoot
            }
            ReceiptSlotKind::LiveExecutionReceipt => {
                OperatorCommand::AttachLiveExecutionReceiptRoot
            }
            ReceiptSlotKind::OperatorSignoff => OperatorCommand::AttachOperatorSignoffRoot,
        };
        let guard = OperatorCommandGuard::FutureLiveAcceptedRootOnly;
        let hint_root = root_of(&[
            "operator-command-hint",
            command.as_str(),
            slot.as_str(),
            guard.as_str(),
        ]);
        Self {
            command,
            slot,
            guard,
            hint_root,
        }
    }

    pub fn root(&self) -> String {
        root_of(&[
            "operator-command",
            self.command.as_str(),
            self.slot.as_str(),
            self.guard.as_str(),
            self.hint_root.as_str(),
        ])
    }
}

impl fmt::Debug for OperatorCommandHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OperatorCommandHint")
            .field("command", &self.command)
            .field("slot", &self.slot)
            .field("guard", &self.guard)
            .field("root", &self.root())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorCommand {
    AttachReplayRunRoot,
    AttachRollbackDrillRoot,
    AttachAdversarialReplayRoot,
    AttachStaleArchiveReplacementRoot,
    AttachLiveExecutionReceiptRoot,
    AttachOperatorSignoffRoot,
}

impl OperatorCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttachReplayRunRoot => "attach_replay_run_root",
            Self::AttachRollbackDrillRoot => "attach_rollback_drill_root",
            Self::AttachAdversarialReplayRoot => "attach_adversarial_replay_root",
            Self::AttachStaleArchiveReplacementRoot => "attach_stale_archive_replacement_root",
            Self::AttachLiveExecutionReceiptRoot => "attach_live_execution_receipt_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorCommandGuard {
    FutureLiveAcceptedRootOnly,
}

impl OperatorCommandGuard {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FutureLiveAcceptedRootOnly => "future_live_accepted_root_only",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum FailClosedSlotRegistryVerdict {
    Blocked {
        blocker_root: String,
        blockers: Vec<ReplaySlotBlocker>,
    },
    Clearable {
        clearance_root: String,
    },
}

impl FailClosedSlotRegistryVerdict {
    pub fn blocked(blockers: &[ReplaySlotBlocker]) -> Self {
        Self::Blocked {
            blocker_root: join_roots(blockers.iter().map(ReplaySlotBlocker::root)),
            blockers: blockers.to_vec(),
        }
    }

    pub fn clearable(clearance_root: String) -> Self {
        Self::Clearable { clearance_root }
    }

    pub fn root(&self) -> String {
        match self {
            Self::Blocked {
                blocker_root,
                blockers,
            } => root_of(&[
                "verdict-blocked",
                blocker_root.as_str(),
                blockers.len().to_string().as_str(),
            ]),
            Self::Clearable { clearance_root } => {
                root_of(&["verdict-clearable", clearance_root.as_str()])
            }
        }
    }
}

impl fmt::Debug for FailClosedSlotRegistryVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocked {
                blocker_root,
                blockers,
            } => f
                .debug_struct("Blocked")
                .field("blocker_root", blocker_root)
                .field("blocker_count", &blockers.len())
                .finish(),
            Self::Clearable { clearance_root } => f
                .debug_struct("Clearable")
                .field("clearance_root", clearance_root)
                .finish(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PublicRecord {
    pub module_root: String,
    pub protocol_version: String,
    pub wave: u16,
    pub source_wave: u16,
    pub plan_intake_root: String,
    pub registry_root: String,
    pub slot_roots: BTreeMap<ReceiptSlotKind, String>,
    pub blocker_root: String,
    pub verdict: FailClosedSlotRegistryVerdict,
    pub command_hint_root: String,
    pub audit_root: String,
}

impl PublicRecord {
    pub fn root(&self) -> String {
        root_of(&[
            "public-record",
            self.module_root.as_str(),
            self.protocol_version.as_str(),
            self.wave.to_string().as_str(),
            self.source_wave.to_string().as_str(),
            self.plan_intake_root.as_str(),
            self.registry_root.as_str(),
            join_roots(self.slot_roots.values().cloned()).as_str(),
            self.blocker_root.as_str(),
            self.verdict.root().as_str(),
            self.command_hint_root.as_str(),
            self.audit_root.as_str(),
        ])
    }
}

impl fmt::Debug for PublicRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PublicRecord")
            .field("module_root", &self.module_root)
            .field("wave", &self.wave)
            .field("source_wave", &self.source_wave)
            .field("registry_root", &self.registry_root)
            .field("verdict", &self.verdict)
            .field("root", &self.root())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RegistryAudit {
    pub config_root: String,
    pub slot_set_root: String,
    pub blocker_set_root: String,
    pub privacy_root: String,
}

impl RegistryAudit {
    pub fn from_config_and_slots(
        config: &Config,
        slots: &BTreeMap<ReceiptSlotKind, ReceiptSlot>,
        blockers: &[ReplaySlotBlocker],
    ) -> Self {
        Self {
            config_root: config.config_root(),
            slot_set_root: join_roots(slots.values().map(ReceiptSlot::public_root)),
            blocker_set_root: join_roots(blockers.iter().map(ReplaySlotBlocker::root)),
            privacy_root: root_of(&[
                "registry-audit-privacy",
                "roots-only",
                "no-raw-keys",
                "no-raw-addresses",
                "no-raw-transaction-identifiers",
                "no-raw-payloads",
            ]),
        }
    }

    pub fn audit_root(&self) -> String {
        root_of(&[
            "registry-audit",
            self.config_root.as_str(),
            self.slot_set_root.as_str(),
            self.blocker_set_root.as_str(),
            self.privacy_root.as_str(),
        ])
    }
}

impl fmt::Debug for RegistryAudit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistryAudit")
            .field("audit_root", &self.audit_root())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum RuntimeError {
    UnknownSlot(ReceiptSlotKind),
    SlotMismatch {
        slot: ReceiptSlotKind,
        receipt: ReceiptSlotKind,
    },
    SourceWaveBlocked {
        slot: ReceiptSlotKind,
        wave: u16,
    },
    TargetWaveBlocked {
        slot: ReceiptSlotKind,
        wave: u16,
    },
    ReceiptStatusBlocked(ReceiptSlotKind),
    ReceiptNotLiveAccepted(ReceiptSlotKind),
    RootsOnlyPrivacyBlocked(ReceiptSlotKind),
    EmptyReceiptRoot(ReceiptSlotKind),
    PlanRootMismatch(ReceiptSlotKind),
    ReceiptRootAlreadyAttached(ReceiptSlotKind),
}

impl RuntimeError {
    pub fn root(&self) -> String {
        match self {
            Self::UnknownSlot(slot) => root_of(&["error-unknown-slot", slot.as_str()]),
            Self::SlotMismatch { slot, receipt } => {
                root_of(&["error-slot-mismatch", slot.as_str(), receipt.as_str()])
            }
            Self::SourceWaveBlocked { slot, wave } => root_of(&[
                "error-source-wave-blocked",
                slot.as_str(),
                wave.to_string().as_str(),
            ]),
            Self::TargetWaveBlocked { slot, wave } => root_of(&[
                "error-target-wave-blocked",
                slot.as_str(),
                wave.to_string().as_str(),
            ]),
            Self::ReceiptStatusBlocked(slot) => {
                root_of(&["error-receipt-status-blocked", slot.as_str()])
            }
            Self::ReceiptNotLiveAccepted(slot) => {
                root_of(&["error-receipt-not-live-accepted", slot.as_str()])
            }
            Self::RootsOnlyPrivacyBlocked(slot) => {
                root_of(&["error-roots-only-privacy-blocked", slot.as_str()])
            }
            Self::EmptyReceiptRoot(slot) => root_of(&["error-empty-receipt-root", slot.as_str()]),
            Self::PlanRootMismatch(slot) => root_of(&["error-plan-root-mismatch", slot.as_str()]),
            Self::ReceiptRootAlreadyAttached(slot) => {
                root_of(&["error-receipt-root-already-attached", slot.as_str()])
            }
        }
    }
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeError")
            .field("root", &self.root())
            .finish()
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSlot(slot) => write!(f, "unknown receipt slot: {}", slot.as_str()),
            Self::SlotMismatch { slot, receipt } => write!(
                f,
                "receipt slot mismatch: slot {} received {}",
                slot.as_str(),
                receipt.as_str()
            ),
            Self::SourceWaveBlocked { slot, wave } => {
                write!(f, "source wave blocked for {}: {}", slot.as_str(), wave)
            }
            Self::TargetWaveBlocked { slot, wave } => {
                write!(f, "target wave blocked for {}: {}", slot.as_str(), wave)
            }
            Self::ReceiptStatusBlocked(slot) => {
                write!(f, "receipt status blocked for {}", slot.as_str())
            }
            Self::ReceiptNotLiveAccepted(slot) => {
                write!(f, "receipt is not live accepted for {}", slot.as_str())
            }
            Self::RootsOnlyPrivacyBlocked(slot) => {
                write!(f, "roots-only privacy rule blocked {}", slot.as_str())
            }
            Self::EmptyReceiptRoot(slot) => write!(f, "empty receipt root for {}", slot.as_str()),
            Self::PlanRootMismatch(slot) => write!(f, "plan root mismatch for {}", slot.as_str()),
            Self::ReceiptRootAlreadyAttached(slot) => {
                write!(f, "receipt root already attached for {}", slot.as_str())
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

pub fn devnet() -> Runtime {
    State::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    root_of(&[
        "state-root",
        MODULE_ID,
        PROTOCOL_VERSION,
        WAVE.to_string().as_str(),
        SOURCE_WAVE.to_string().as_str(),
        PlanRootIntake::wave91_runtime_replay().root().as_str(),
    ])
}

pub fn canonical_clearable_devnet() -> Result<Runtime> {
    devnet().with_canonical_future_roots()
}

pub fn canonical_slot_roots() -> BTreeMap<ReceiptSlotKind, String> {
    ReceiptSlotKind::all()
        .into_iter()
        .map(|kind| {
            let receipt = AcceptedReceiptPlaceholder::canonical(kind);
            (kind, receipt.placeholder_root())
        })
        .collect()
}

pub fn fail_closed_verdict() -> FailClosedSlotRegistryVerdict {
    devnet().verdict
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn seeded_root(seed: &str) -> String {
    root_of(&["seed", HASH_DOMAIN, seed])
}

fn join_roots<I>(roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let mut values = roots.into_iter().collect::<Vec<_>>();
    values.sort();
    root_of(values.iter().map(String::as_str))
}

fn root_of<I, S>(parts: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut hash = Fnv64::new();
    hash.write(HASH_DOMAIN.as_bytes());
    hash.write_u8(0xff);
    for part in parts {
        let text = part.as_ref();
        hash.write_usize(text.len());
        hash.write_u8(0);
        hash.write(text.as_bytes());
        hash.write_u8(0x1f);
    }
    format!("{}:{:016x}", HASH_DOMAIN, hash.finish())
}

#[derive(Clone, Copy)]
struct Fnv64 {
    state: u64,
}

impl Fnv64 {
    fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    fn write_u8(&mut self, byte: u8) {
        self.state ^= u64::from(byte);
        self.state = self.state.wrapping_mul(0x100000001b3);
    }

    fn write_usize(&mut self, value: usize) {
        self.write(&value.to_le_bytes());
    }

    fn finish(self) -> u64 {
        self.state
    }
}
