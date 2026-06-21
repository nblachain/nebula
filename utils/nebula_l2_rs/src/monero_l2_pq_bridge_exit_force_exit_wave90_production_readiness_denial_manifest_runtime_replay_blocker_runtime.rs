use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};

pub type Result<T> = std::result::Result<T, Error>;
pub type Runtime = State;

const DOMAIN_CONFIG: &str = "wave90.production_readiness_denial.config";
const DOMAIN_STATE: &str = "wave90.production_readiness_denial.state";
const DOMAIN_RECORD: &str = "wave90.production_readiness_denial.public_record";
const DOMAIN_ARCHIVE: &str = "wave90.production_readiness_denial.wave89_archive";
const DOMAIN_RECEIPT: &str = "wave90.production_readiness_denial.live_receipt";
const DOMAIN_BLOCKER: &str = "wave90.production_readiness_denial.blocker";
const DOMAIN_ACTION: &str = "wave90.production_readiness_denial.operator_action";
const DOMAIN_AUDIT: &str = "wave90.production_readiness_denial.audit";
const DEFAULT_WAVE: u64 = 90;
const SOURCE_WAVE: u64 = 89;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub network: Network,
    pub manifest_name: String,
    pub wave: u64,
    pub source_wave: u64,
    pub fail_closed: bool,
    pub deny_on_deferred_replay: bool,
    pub deny_on_deferred_rollback: bool,
    pub deny_on_adversarial_gap: bool,
    pub deny_on_stale_archive: bool,
    pub require_live_replay_receipt: bool,
    pub require_rollback_drill_receipt: bool,
    pub require_adversarial_replay_receipt: bool,
    pub min_live_replay_receipts: usize,
    pub min_rollback_drill_receipts: usize,
    pub min_adversarial_replay_receipts: usize,
    pub max_archive_age_blocks: u64,
    pub max_audit_events: usize,
    pub no_go_archive_lanes: BTreeSet<ArchiveLane>,
}

impl Config {
    pub fn new(network: Network, manifest_name: &str) -> Self {
        Self {
            network,
            manifest_name: clean_label(manifest_name),
            wave: DEFAULT_WAVE,
            source_wave: SOURCE_WAVE,
            fail_closed: true,
            deny_on_deferred_replay: true,
            deny_on_deferred_rollback: true,
            deny_on_adversarial_gap: true,
            deny_on_stale_archive: true,
            require_live_replay_receipt: true,
            require_rollback_drill_receipt: true,
            require_adversarial_replay_receipt: true,
            min_live_replay_receipts: 1,
            min_rollback_drill_receipts: 1,
            min_adversarial_replay_receipts: 1,
            max_archive_age_blocks: 72,
            max_audit_events: 512,
            no_go_archive_lanes: ArchiveLane::all().into_iter().collect(),
        }
    }

    pub fn devnet() -> Self {
        Self::new(Network::Devnet, "wave90-production-readiness-denial")
    }

    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_CONFIG);
        h.field("network", self.network.as_str());
        h.field("manifest_name", &self.manifest_name);
        h.u64("wave", self.wave);
        h.u64("source_wave", self.source_wave);
        h.bool("fail_closed", self.fail_closed);
        h.bool("deny_on_deferred_replay", self.deny_on_deferred_replay);
        h.bool("deny_on_deferred_rollback", self.deny_on_deferred_rollback);
        h.bool("deny_on_adversarial_gap", self.deny_on_adversarial_gap);
        h.bool("deny_on_stale_archive", self.deny_on_stale_archive);
        h.bool(
            "require_live_replay_receipt",
            self.require_live_replay_receipt,
        );
        h.bool(
            "require_rollback_drill_receipt",
            self.require_rollback_drill_receipt,
        );
        h.bool(
            "require_adversarial_replay_receipt",
            self.require_adversarial_replay_receipt,
        );
        h.usize("min_live_replay_receipts", self.min_live_replay_receipts);
        h.usize(
            "min_rollback_drill_receipts",
            self.min_rollback_drill_receipts,
        );
        h.usize(
            "min_adversarial_replay_receipts",
            self.min_adversarial_replay_receipts,
        );
        h.u64("max_archive_age_blocks", self.max_archive_age_blocks);
        h.usize("max_audit_events", self.max_audit_events);
        for lane in &self.no_go_archive_lanes {
            h.field("no_go_archive_lane", lane.as_str());
        }
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    archives: BTreeMap<ArchiveEvidenceId, Wave89ArchiveEvidence>,
    live_receipts: BTreeMap<ReceiptId, LiveExecutionReceipt>,
    blockers: BTreeMap<BlockerId, ReplayBlocker>,
    actions: BTreeMap<ActionId, OperatorActionHint>,
    audits: VecDeque<AuditEvent>,
    current_height: u64,
    next_archive: u64,
    next_receipt: u64,
    next_blocker: u64,
    next_action: u64,
    next_audit: u64,
    final_verdict: ProductionReadinessVerdict,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut state = Self {
            config,
            archives: BTreeMap::new(),
            live_receipts: BTreeMap::new(),
            blockers: BTreeMap::new(),
            actions: BTreeMap::new(),
            audits: VecDeque::new(),
            current_height: 0,
            next_archive: 1,
            next_receipt: 1,
            next_blocker: 1,
            next_action: 1,
            next_audit: 1,
            final_verdict: ProductionReadinessVerdict::hold("init"),
        };
        state.recompute();
        state
    }

    pub fn devnet() -> Self {
        let mut state = Self::new(Config::devnet());
        state.current_height = 900_000;
        state.seed_wave89_no_go_archive();
        state.recompute();
        state
    }

    pub fn set_current_height(&mut self, height: u64) {
        self.current_height = height;
        self.audit("set_current_height", Root::from_u64(height));
        self.recompute();
    }

    pub fn ingest_archive_evidence(
        &mut self,
        input: Wave89ArchiveEvidenceInput,
    ) -> Result<ArchiveEvidenceId> {
        input.validate()?;
        let id = ArchiveEvidenceId(self.next_archive);
        self.next_archive = self.next_archive.saturating_add(1);
        let evidence = Wave89ArchiveEvidence {
            id,
            lane: input.lane,
            kind: input.kind,
            disposition: input.disposition,
            archive_root: input.archive_root,
            replay_root: input.replay_root,
            rollback_root: input.rollback_root,
            adversarial_root: input.adversarial_root,
            live_execution_root: input.live_execution_root,
            observed_height: input.observed_height,
            note_root: input.note_root,
        };
        let root = evidence.root();
        self.archives.insert(id, evidence);
        self.audit("ingest_archive_evidence", root);
        self.recompute();
        Ok(id)
    }

    pub fn accept_live_receipt(&mut self, input: LiveExecutionReceiptInput) -> Result<ReceiptId> {
        input.validate()?;
        let id = ReceiptId(self.next_receipt);
        self.next_receipt = self.next_receipt.saturating_add(1);
        let receipt = LiveExecutionReceipt {
            id,
            scope: input.scope,
            status: input.status,
            evidence_root: input.evidence_root,
            transcript_root: input.transcript_root,
            monotonic_run_root: input.monotonic_run_root,
            observed_height: input.observed_height,
            accepted_by_policy: input.accepted_by_policy,
        };
        let root = receipt.root();
        self.live_receipts.insert(id, receipt);
        self.audit("accept_live_receipt", root);
        self.recompute();
        Ok(id)
    }

    pub fn deny_production_readiness(&mut self, reason: DenialReason) -> BlockerId {
        let id = self.next_blocker_id();
        let blocker = ReplayBlocker::new(
            id,
            BlockerKind::ManualDenial,
            reason,
            Severity::Critical,
            Root::tagged("manual-denial"),
        );
        let root = blocker.root();
        self.blockers.insert(id, blocker);
        self.add_action(
            ActionKind::KeepProductionGateClosed,
            "manual-denial",
            root,
            Severity::Critical,
        );
        self.audit("deny_production_readiness", root);
        self.recompute();
        id
    }

    pub fn public_record(&self) -> PublicRecord {
        let archive_roots = self
            .archives
            .values()
            .map(Wave89ArchiveEvidence::public_root)
            .collect::<Vec<_>>();
        let receipt_roots = self
            .live_receipts
            .values()
            .map(LiveExecutionReceipt::public_root)
            .collect::<Vec<_>>();
        let blocker_roots = self
            .blockers
            .values()
            .map(ReplayBlocker::public_root)
            .collect::<Vec<_>>();
        let action_roots = self
            .actions
            .values()
            .map(OperatorActionHint::public_root)
            .collect::<Vec<_>>();
        let audit_root = merkle_root(self.audits.iter().map(AuditEvent::root));
        let record = PublicRecord {
            wave: self.config.wave,
            source_wave: self.config.source_wave,
            network: self.config.network.clone(),
            config_root: self.config.root(),
            state_root: self.state_root_value(),
            archive_root: merkle_root(archive_roots.iter().copied()),
            receipt_root: merkle_root(receipt_roots.iter().copied()),
            blocker_root: merkle_root(blocker_roots.iter().copied()),
            action_root: merkle_root(action_roots.iter().copied()),
            audit_root,
            verdict: self.final_verdict.clone(),
            counts: RecordCounts {
                archive_items: self.archives.len(),
                live_receipts: self.live_receipts.len(),
                blockers: self.blockers.len(),
                operator_actions: self.actions.len(),
                audit_events: self.audits.len(),
            },
        };
        PublicRecord {
            state_root: record.root(),
            ..record
        }
    }

    pub fn state_root(&self) -> String {
        self.state_root_value().to_string()
    }

    pub fn state_root_value(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_STATE);
        h.root("config", self.config.root());
        h.u64("current_height", self.current_height);
        h.root(
            "archives",
            merkle_root(self.archives.values().map(Wave89ArchiveEvidence::root)),
        );
        h.root(
            "live_receipts",
            merkle_root(self.live_receipts.values().map(LiveExecutionReceipt::root)),
        );
        h.root(
            "blockers",
            merkle_root(self.blockers.values().map(ReplayBlocker::root)),
        );
        h.root(
            "actions",
            merkle_root(self.actions.values().map(OperatorActionHint::root)),
        );
        h.root(
            "audits",
            merkle_root(self.audits.iter().map(AuditEvent::root)),
        );
        h.root("verdict", self.final_verdict.root());
        h.finish()
    }

    pub fn verdict(&self) -> &ProductionReadinessVerdict {
        &self.final_verdict
    }

    pub fn blockers(&self) -> Vec<ReplayBlocker> {
        self.blockers.values().cloned().collect()
    }

    pub fn operator_actions(&self) -> Vec<OperatorActionHint> {
        self.actions.values().cloned().collect()
    }

    pub fn missing_receipts(&self) -> Vec<MissingReceipt> {
        let mut missing = Vec::new();
        let live = self.accepted_receipt_count(ReceiptScope::RuntimeReplay);
        let rollback = self.accepted_receipt_count(ReceiptScope::RollbackDrill);
        let adversarial = self.accepted_receipt_count(ReceiptScope::AdversarialReplay);
        if self.config.require_live_replay_receipt && live < self.config.min_live_replay_receipts {
            missing.push(MissingReceipt {
                scope: ReceiptScope::RuntimeReplay,
                required: self.config.min_live_replay_receipts,
                accepted: live,
            });
        }
        if self.config.require_rollback_drill_receipt
            && rollback < self.config.min_rollback_drill_receipts
        {
            missing.push(MissingReceipt {
                scope: ReceiptScope::RollbackDrill,
                required: self.config.min_rollback_drill_receipts,
                accepted: rollback,
            });
        }
        if self.config.require_adversarial_replay_receipt
            && adversarial < self.config.min_adversarial_replay_receipts
        {
            missing.push(MissingReceipt {
                scope: ReceiptScope::AdversarialReplay,
                required: self.config.min_adversarial_replay_receipts,
                accepted: adversarial,
            });
        }
        missing
    }

    fn seed_wave89_no_go_archive(&mut self) {
        let seeds = [
            (
                ArchiveLane::RuntimeReplayGate,
                ArchiveEvidenceKind::DeferredRuntimeReplay,
                ArchiveDisposition::NoGoArchived,
                "wave89-runtime-replay-deferred",
            ),
            (
                ArchiveLane::RuntimeReplayGate,
                ArchiveEvidenceKind::MissingLiveExecution,
                ArchiveDisposition::Hold,
                "wave89-live-execution-missing",
            ),
            (
                ArchiveLane::RuntimeReplayGate,
                ArchiveEvidenceKind::AdversarialReplayDeferred,
                ArchiveDisposition::Deferred,
                "wave89-adversarial-replay-deferred",
            ),
            (
                ArchiveLane::BridgeCustodyGate,
                ArchiveEvidenceKind::RollbackDrillDeferred,
                ArchiveDisposition::Hold,
                "wave89-rollback-drill-deferred",
            ),
            (
                ArchiveLane::AuditSecurityGate,
                ArchiveEvidenceKind::ReleaseCaptainNoGo,
                ArchiveDisposition::NoGoArchived,
                "wave89-release-captain-no-go",
            ),
        ];
        for (lane, kind, disposition, tag) in seeds {
            let input = Wave89ArchiveEvidenceInput {
                lane,
                kind,
                disposition,
                archive_root: Root::tagged(tag),
                replay_root: Some(Root::tagged("replay-root-redacted")),
                rollback_root: Some(Root::tagged("rollback-root-redacted")),
                adversarial_root: Some(Root::tagged("adversarial-root-redacted")),
                live_execution_root: None,
                observed_height: 899_900,
                note_root: Root::tagged("roots-only-note"),
            };
            let _ = self.ingest_archive_evidence(input);
        }
    }

    fn recompute(&mut self) {
        self.rebuild_derived_blockers();
        self.rebuild_actions();
        let blocking_roots = self
            .blockers
            .values()
            .filter(|blocker| blocker.blocks_production())
            .map(ReplayBlocker::public_root)
            .collect::<Vec<_>>();
        self.final_verdict = if blocking_roots.is_empty() && !self.config.fail_closed {
            ProductionReadinessVerdict::allow("policy-open", Root::tagged("policy-open"))
        } else if blocking_roots.is_empty() {
            ProductionReadinessVerdict::hold("fail-closed-no-positive-open-root")
        } else {
            ProductionReadinessVerdict::deny(
                "production-readiness-denied-runtime-replay-blocked",
                merkle_root(blocking_roots.iter().copied()),
                blocking_roots.len(),
            )
        };
    }

    fn rebuild_derived_blockers(&mut self) {
        self.blockers
            .retain(|_, blocker| blocker.kind == BlockerKind::ManualDenial);
        let archives = self.archives.values().cloned().collect::<Vec<_>>();
        for evidence in archives {
            self.add_archive_blockers(&evidence);
        }
        for missing in self.missing_receipts() {
            let severity = match missing.scope {
                ReceiptScope::RuntimeReplay => Severity::Critical,
                ReceiptScope::RollbackDrill => Severity::High,
                ReceiptScope::AdversarialReplay => Severity::Critical,
            };
            let id = self.next_blocker_id();
            let blocker = ReplayBlocker::new(
                id,
                BlockerKind::MissingAcceptedLiveReceipt,
                DenialReason::MissingLiveReceipt(missing.scope),
                severity,
                missing.root(),
            );
            self.blockers.insert(id, blocker);
        }
    }

    fn add_archive_blockers(&mut self, evidence: &Wave89ArchiveEvidence) {
        if evidence.disposition.blocks_production() {
            self.insert_blocker(
                BlockerKind::Wave89NoGoArchive,
                DenialReason::NoGoArchiveEvidence(evidence.kind),
                evidence.disposition.severity(),
                evidence.public_root(),
            );
        }
        if self.config.deny_on_deferred_replay && evidence.kind.is_replay_deferred() {
            self.insert_blocker(
                BlockerKind::DeferredRuntimeReplay,
                DenialReason::RuntimeReplayDeferred,
                Severity::Critical,
                evidence.public_root(),
            );
        }
        if self.config.deny_on_deferred_rollback && evidence.kind.is_rollback_deferred() {
            self.insert_blocker(
                BlockerKind::RollbackDrillDeferred,
                DenialReason::RollbackDrillDeferred,
                Severity::High,
                evidence.public_root(),
            );
        }
        if self.config.deny_on_adversarial_gap && evidence.kind.is_adversarial_gap() {
            self.insert_blocker(
                BlockerKind::AdversarialReplayGap,
                DenialReason::AdversarialReplayDeferred,
                Severity::Critical,
                evidence.public_root(),
            );
        }
        if evidence.kind == ArchiveEvidenceKind::MissingLiveExecution {
            self.insert_blocker(
                BlockerKind::MissingAcceptedLiveReceipt,
                DenialReason::LiveExecutionDeferred,
                Severity::Critical,
                evidence.public_root(),
            );
        }
        if self.config.deny_on_stale_archive && evidence.is_stale(self.current_height, &self.config)
        {
            self.insert_blocker(
                BlockerKind::StaleArchiveEvidence,
                DenialReason::StaleWave89Archive,
                Severity::High,
                evidence.public_root(),
            );
        }
    }

    fn insert_blocker(
        &mut self,
        kind: BlockerKind,
        reason: DenialReason,
        severity: Severity,
        source_root: Root,
    ) {
        let id = self.next_blocker_id();
        let blocker = ReplayBlocker::new(id, kind, reason, severity, source_root);
        self.blockers.insert(id, blocker);
    }

    fn rebuild_actions(&mut self) {
        self.actions.clear();
        let blockers = self.blockers.values().cloned().collect::<Vec<_>>();
        for blocker in blockers {
            let action = match blocker.reason {
                DenialReason::RuntimeReplayDeferred => ActionKind::RunLiveReplayAndArchiveReceipt,
                DenialReason::RollbackDrillDeferred => ActionKind::RunRollbackDrill,
                DenialReason::AdversarialReplayDeferred => ActionKind::RunAdversarialReplay,
                DenialReason::MissingLiveReceipt(_) => ActionKind::AttachAcceptedLiveReceipt,
                DenialReason::LiveExecutionDeferred => ActionKind::AttachAcceptedLiveReceipt,
                DenialReason::StaleWave89Archive => ActionKind::RefreshWave89Archive,
                DenialReason::NoGoArchiveEvidence(_) => ActionKind::KeepProductionGateClosed,
                DenialReason::ManualOperatorDenial => ActionKind::KeepProductionGateClosed,
            };
            self.add_action(
                action,
                blocker.kind.as_str(),
                blocker.public_root(),
                blocker.severity,
            );
        }
    }

    fn add_action(
        &mut self,
        kind: ActionKind,
        reason_label: &str,
        blocker_root: Root,
        severity: Severity,
    ) -> ActionId {
        let id = ActionId(self.next_action);
        self.next_action = self.next_action.saturating_add(1);
        let action = OperatorActionHint {
            id,
            kind,
            severity,
            reason_label: clean_label(reason_label),
            blocker_root,
            command_root: Root::tagged(kind.as_str()),
        };
        self.actions.insert(id, action);
        id
    }

    fn next_blocker_id(&mut self) -> BlockerId {
        let id = BlockerId(self.next_blocker);
        self.next_blocker = self.next_blocker.saturating_add(1);
        id
    }

    fn audit(&mut self, action: &str, root: Root) {
        let id = self.next_audit;
        self.next_audit = self.next_audit.saturating_add(1);
        self.audits.push_back(AuditEvent {
            sequence: id,
            action: clean_label(action),
            root,
        });
        while self.audits.len() > self.config.max_audit_events {
            let _ = self.audits.pop_front();
        }
    }

    fn accepted_receipt_count(&self, scope: ReceiptScope) -> usize {
        self.live_receipts
            .values()
            .filter(|receipt| {
                receipt.scope == scope
                    && receipt.status == ReceiptStatus::Accepted
                    && receipt.accepted_by_policy
            })
            .count()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub wave: u64,
    pub source_wave: u64,
    pub network: Network,
    pub config_root: Root,
    pub state_root: Root,
    pub archive_root: Root,
    pub receipt_root: Root,
    pub blocker_root: Root,
    pub action_root: Root,
    pub audit_root: Root,
    pub verdict: ProductionReadinessVerdict,
    pub counts: RecordCounts,
}

impl PublicRecord {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_RECORD);
        h.u64("wave", self.wave);
        h.u64("source_wave", self.source_wave);
        h.field("network", self.network.as_str());
        h.root("config_root", self.config_root);
        h.root("state_root", self.state_root);
        h.root("archive_root", self.archive_root);
        h.root("receipt_root", self.receipt_root);
        h.root("blocker_root", self.blocker_root);
        h.root("action_root", self.action_root);
        h.root("audit_root", self.audit_root);
        h.root("verdict", self.verdict.root());
        h.root("counts", self.counts.root());
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordCounts {
    pub archive_items: usize,
    pub live_receipts: usize,
    pub blockers: usize,
    pub operator_actions: usize,
    pub audit_events: usize,
}

impl RecordCounts {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new("record-counts");
        h.usize("archive_items", self.archive_items);
        h.usize("live_receipts", self.live_receipts);
        h.usize("blockers", self.blockers);
        h.usize("operator_actions", self.operator_actions);
        h.usize("audit_events", self.audit_events);
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave89ArchiveEvidenceInput {
    pub lane: ArchiveLane,
    pub kind: ArchiveEvidenceKind,
    pub disposition: ArchiveDisposition,
    pub archive_root: Root,
    pub replay_root: Option<Root>,
    pub rollback_root: Option<Root>,
    pub adversarial_root: Option<Root>,
    pub live_execution_root: Option<Root>,
    pub observed_height: u64,
    pub note_root: Root,
}

impl Wave89ArchiveEvidenceInput {
    pub fn validate(&self) -> Result<()> {
        if self.archive_root.is_zero() {
            return Err(Error::ZeroRoot("archive_root"));
        }
        if self.note_root.is_zero() {
            return Err(Error::ZeroRoot("note_root"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave89ArchiveEvidence {
    pub id: ArchiveEvidenceId,
    pub lane: ArchiveLane,
    pub kind: ArchiveEvidenceKind,
    pub disposition: ArchiveDisposition,
    pub archive_root: Root,
    pub replay_root: Option<Root>,
    pub rollback_root: Option<Root>,
    pub adversarial_root: Option<Root>,
    pub live_execution_root: Option<Root>,
    pub observed_height: u64,
    pub note_root: Root,
}

impl Wave89ArchiveEvidence {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_ARCHIVE);
        h.u64("id", self.id.0);
        h.field("lane", self.lane.as_str());
        h.field("kind", self.kind.as_str());
        h.field("disposition", self.disposition.as_str());
        h.root("archive_root", self.archive_root);
        h.option_root("replay_root", self.replay_root);
        h.option_root("rollback_root", self.rollback_root);
        h.option_root("adversarial_root", self.adversarial_root);
        h.option_root("live_execution_root", self.live_execution_root);
        h.u64("observed_height", self.observed_height);
        h.root("note_root", self.note_root);
        h.finish()
    }

    pub fn public_root(&self) -> Root {
        let mut h = RootBuilder::new("wave89-archive-public");
        h.field("lane", self.lane.as_str());
        h.field("kind", self.kind.as_str());
        h.field("disposition", self.disposition.as_str());
        h.root("archive_root", self.archive_root);
        h.u64("observed_height", self.observed_height);
        h.root("note_root", self.note_root);
        h.finish()
    }

    pub fn is_stale(&self, current_height: u64, config: &Config) -> bool {
        current_height.saturating_sub(self.observed_height) > config.max_archive_age_blocks
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionReceiptInput {
    pub scope: ReceiptScope,
    pub status: ReceiptStatus,
    pub evidence_root: Root,
    pub transcript_root: Root,
    pub monotonic_run_root: Root,
    pub observed_height: u64,
    pub accepted_by_policy: bool,
}

impl LiveExecutionReceiptInput {
    pub fn validate(&self) -> Result<()> {
        if self.evidence_root.is_zero() {
            return Err(Error::ZeroRoot("evidence_root"));
        }
        if self.transcript_root.is_zero() {
            return Err(Error::ZeroRoot("transcript_root"));
        }
        if self.monotonic_run_root.is_zero() {
            return Err(Error::ZeroRoot("monotonic_run_root"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionReceipt {
    pub id: ReceiptId,
    pub scope: ReceiptScope,
    pub status: ReceiptStatus,
    pub evidence_root: Root,
    pub transcript_root: Root,
    pub monotonic_run_root: Root,
    pub observed_height: u64,
    pub accepted_by_policy: bool,
}

impl LiveExecutionReceipt {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_RECEIPT);
        h.u64("id", self.id.0);
        h.field("scope", self.scope.as_str());
        h.field("status", self.status.as_str());
        h.root("evidence_root", self.evidence_root);
        h.root("transcript_root", self.transcript_root);
        h.root("monotonic_run_root", self.monotonic_run_root);
        h.u64("observed_height", self.observed_height);
        h.bool("accepted_by_policy", self.accepted_by_policy);
        h.finish()
    }

    pub fn public_root(&self) -> Root {
        let mut h = RootBuilder::new("live-receipt-public");
        h.field("scope", self.scope.as_str());
        h.field("status", self.status.as_str());
        h.root("evidence_root", self.evidence_root);
        h.root("transcript_root", self.transcript_root);
        h.u64("observed_height", self.observed_height);
        h.bool("accepted_by_policy", self.accepted_by_policy);
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReceipt {
    pub scope: ReceiptScope,
    pub required: usize,
    pub accepted: usize,
}

impl MissingReceipt {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new("missing-receipt");
        h.field("scope", self.scope.as_str());
        h.usize("required", self.required);
        h.usize("accepted", self.accepted);
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayBlocker {
    pub id: BlockerId,
    pub kind: BlockerKind,
    pub reason: DenialReason,
    pub severity: Severity,
    pub source_root: Root,
    pub blocker_root: Root,
}

impl ReplayBlocker {
    pub fn new(
        id: BlockerId,
        kind: BlockerKind,
        reason: DenialReason,
        severity: Severity,
        source_root: Root,
    ) -> Self {
        let mut blocker = Self {
            id,
            kind,
            reason,
            severity,
            source_root,
            blocker_root: Root::zero(),
        };
        blocker.blocker_root = blocker.root();
        blocker
    }

    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_BLOCKER);
        h.u64("id", self.id.0);
        h.field("kind", self.kind.as_str());
        h.field("reason", self.reason.as_str());
        h.field("severity", self.severity.as_str());
        h.root("source_root", self.source_root);
        h.finish()
    }

    pub fn public_root(&self) -> Root {
        let mut h = RootBuilder::new("blocker-public");
        h.field("kind", self.kind.as_str());
        h.field("reason", self.reason.as_str());
        h.field("severity", self.severity.as_str());
        h.root("source_root", self.source_root);
        h.root("blocker_root", self.blocker_root);
        h.finish()
    }

    pub fn blocks_production(&self) -> bool {
        self.severity.blocks_production()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorActionHint {
    pub id: ActionId,
    pub kind: ActionKind,
    pub severity: Severity,
    pub reason_label: String,
    pub blocker_root: Root,
    pub command_root: Root,
}

impl OperatorActionHint {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_ACTION);
        h.u64("id", self.id.0);
        h.field("kind", self.kind.as_str());
        h.field("severity", self.severity.as_str());
        h.field("reason_label", &self.reason_label);
        h.root("blocker_root", self.blocker_root);
        h.root("command_root", self.command_root);
        h.finish()
    }

    pub fn public_root(&self) -> Root {
        let mut h = RootBuilder::new("action-public");
        h.field("kind", self.kind.as_str());
        h.field("severity", self.severity.as_str());
        h.root("blocker_root", self.blocker_root);
        h.root("command_root", self.command_root);
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionReadinessVerdict {
    pub status: VerdictStatus,
    pub reason_label: String,
    pub blocking_root: Root,
    pub blocking_count: usize,
}

impl ProductionReadinessVerdict {
    pub fn deny(reason_label: &str, blocking_root: Root, blocking_count: usize) -> Self {
        Self {
            status: VerdictStatus::Deny,
            reason_label: clean_label(reason_label),
            blocking_root,
            blocking_count,
        }
    }

    pub fn hold(reason_label: &str) -> Self {
        Self {
            status: VerdictStatus::Hold,
            reason_label: clean_label(reason_label),
            blocking_root: Root::tagged(reason_label),
            blocking_count: 0,
        }
    }

    pub fn allow(reason_label: &str, root: Root) -> Self {
        Self {
            status: VerdictStatus::Allow,
            reason_label: clean_label(reason_label),
            blocking_root: root,
            blocking_count: 0,
        }
    }

    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new("production-readiness-verdict");
        h.field("status", self.status.as_str());
        h.field("reason_label", &self.reason_label);
        h.root("blocking_root", self.blocking_root);
        h.usize("blocking_count", self.blocking_count);
        h.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEvent {
    pub sequence: u64,
    pub action: String,
    pub root: Root,
}

impl AuditEvent {
    pub fn root(&self) -> Root {
        let mut h = RootBuilder::new(DOMAIN_AUDIT);
        h.u64("sequence", self.sequence);
        h.field("action", &self.action);
        h.root("root", self.root);
        h.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArchiveEvidenceId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReceiptId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BlockerId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ActionId(pub u64);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
    Custom(String),
}

impl Network {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Devnet => "devnet",
            Self::Testnet => "testnet",
            Self::Mainnet => "mainnet",
            Self::Custom(value) => value.as_str(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArchiveLane {
    CompileGate,
    RuntimeReplayGate,
    AuditSecurityGate,
    BridgeCustodyGate,
    WalletWatchtowerGate,
    PqReservePrivacyGate,
    FinalTranscript,
}

impl ArchiveLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileGate,
            Self::RuntimeReplayGate,
            Self::AuditSecurityGate,
            Self::BridgeCustodyGate,
            Self::WalletWatchtowerGate,
            Self::PqReservePrivacyGate,
            Self::FinalTranscript,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileGate => "compile_gate",
            Self::RuntimeReplayGate => "runtime_replay_gate",
            Self::AuditSecurityGate => "audit_security_gate",
            Self::BridgeCustodyGate => "bridge_custody_gate",
            Self::WalletWatchtowerGate => "wallet_watchtower_gate",
            Self::PqReservePrivacyGate => "pq_reserve_privacy_gate",
            Self::FinalTranscript => "final_transcript",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveEvidenceKind {
    Wave88ReplayRoot,
    DeferredRuntimeReplay,
    RollbackDrillDeferred,
    AdversarialReplayDeferred,
    MissingLiveExecution,
    RejectedLiveExecution,
    StaleNoGoArchive,
    ReleaseCaptainNoGo,
    PrivacyLinkedReplayHold,
    BridgeCustodyRollbackHold,
}

impl ArchiveEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave88ReplayRoot => "wave88_replay_root",
            Self::DeferredRuntimeReplay => "deferred_runtime_replay",
            Self::RollbackDrillDeferred => "rollback_drill_deferred",
            Self::AdversarialReplayDeferred => "adversarial_replay_deferred",
            Self::MissingLiveExecution => "missing_live_execution",
            Self::RejectedLiveExecution => "rejected_live_execution",
            Self::StaleNoGoArchive => "stale_no_go_archive",
            Self::ReleaseCaptainNoGo => "release_captain_no_go",
            Self::PrivacyLinkedReplayHold => "privacy_linked_replay_hold",
            Self::BridgeCustodyRollbackHold => "bridge_custody_rollback_hold",
        }
    }

    pub fn is_replay_deferred(self) -> bool {
        matches!(
            self,
            Self::DeferredRuntimeReplay | Self::Wave88ReplayRoot | Self::PrivacyLinkedReplayHold
        )
    }

    pub fn is_rollback_deferred(self) -> bool {
        matches!(
            self,
            Self::RollbackDrillDeferred | Self::BridgeCustodyRollbackHold
        )
    }

    pub fn is_adversarial_gap(self) -> bool {
        matches!(self, Self::AdversarialReplayDeferred)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveDisposition {
    Missing,
    Deferred,
    Hold,
    NoGoArchived,
    Rejected,
    SupersededByAcceptedReceipt,
}

impl ArchiveDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Deferred => "deferred",
            Self::Hold => "hold",
            Self::NoGoArchived => "no_go_archived",
            Self::Rejected => "rejected",
            Self::SupersededByAcceptedReceipt => "superseded_by_accepted_receipt",
        }
    }

    pub fn blocks_production(self) -> bool {
        !matches!(self, Self::SupersededByAcceptedReceipt)
    }

    pub fn severity(self) -> Severity {
        match self {
            Self::Missing | Self::Rejected => Severity::Critical,
            Self::Deferred | Self::NoGoArchived => Severity::Critical,
            Self::Hold => Severity::High,
            Self::SupersededByAcceptedReceipt => Severity::Info,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptScope {
    RuntimeReplay,
    RollbackDrill,
    AdversarialReplay,
}

impl ReceiptScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime_replay",
            Self::RollbackDrill => "rollback_drill",
            Self::AdversarialReplay => "adversarial_replay",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptStatus {
    Draft,
    Deferred,
    Rejected,
    Accepted,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Deferred => "deferred",
            Self::Rejected => "rejected",
            Self::Accepted => "accepted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockerKind {
    Wave89NoGoArchive,
    DeferredRuntimeReplay,
    RollbackDrillDeferred,
    AdversarialReplayGap,
    MissingAcceptedLiveReceipt,
    StaleArchiveEvidence,
    ManualDenial,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave89NoGoArchive => "wave89_no_go_archive",
            Self::DeferredRuntimeReplay => "deferred_runtime_replay",
            Self::RollbackDrillDeferred => "rollback_drill_deferred",
            Self::AdversarialReplayGap => "adversarial_replay_gap",
            Self::MissingAcceptedLiveReceipt => "missing_accepted_live_receipt",
            Self::StaleArchiveEvidence => "stale_archive_evidence",
            Self::ManualDenial => "manual_denial",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenialReason {
    RuntimeReplayDeferred,
    RollbackDrillDeferred,
    AdversarialReplayDeferred,
    MissingLiveReceipt(ReceiptScope),
    LiveExecutionDeferred,
    StaleWave89Archive,
    NoGoArchiveEvidence(ArchiveEvidenceKind),
    ManualOperatorDenial,
}

impl DenialReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplayDeferred => "runtime_replay_deferred",
            Self::RollbackDrillDeferred => "rollback_drill_deferred",
            Self::AdversarialReplayDeferred => "adversarial_replay_deferred",
            Self::MissingLiveReceipt(scope) => scope.as_str(),
            Self::LiveExecutionDeferred => "live_execution_deferred",
            Self::StaleWave89Archive => "stale_wave89_archive",
            Self::NoGoArchiveEvidence(kind) => kind.as_str(),
            Self::ManualOperatorDenial => "manual_operator_denial",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionKind {
    KeepProductionGateClosed,
    RunLiveReplayAndArchiveReceipt,
    RunRollbackDrill,
    RunAdversarialReplay,
    AttachAcceptedLiveReceipt,
    RefreshWave89Archive,
}

impl ActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepProductionGateClosed => "keep_production_gate_closed",
            Self::RunLiveReplayAndArchiveReceipt => "run_live_replay_and_archive_receipt",
            Self::RunRollbackDrill => "run_rollback_drill",
            Self::RunAdversarialReplay => "run_adversarial_replay",
            Self::AttachAcceptedLiveReceipt => "attach_accepted_live_receipt",
            Self::RefreshWave89Archive => "refresh_wave89_archive",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Info,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn blocks_production(self) -> bool {
        matches!(self, Self::Medium | Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerdictStatus {
    Allow,
    Hold,
    Deny,
}

impl VerdictStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Hold => "hold",
            Self::Deny => "deny",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ZeroRoot(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroRoot(field) => write!(f, "zero root: {field}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Root(pub u64);

impl Root {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn from_u64(value: u64) -> Self {
        let mut h = RootBuilder::new("u64-root");
        h.u64("value", value);
        h.finish()
    }

    pub fn tagged(tag: &str) -> Self {
        let mut h = RootBuilder::new("tagged-root");
        h.field("tag", tag);
        h.finish()
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Root({self})")
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

struct RootBuilder {
    state: std::collections::hash_map::DefaultHasher,
}

impl RootBuilder {
    fn new(domain: &str) -> Self {
        let mut state = std::collections::hash_map::DefaultHasher::new();
        domain.hash(&mut state);
        Self { state }
    }

    fn field(&mut self, name: &str, value: &str) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn bool(&mut self, name: &str, value: bool) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn u64(&mut self, name: &str, value: u64) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn usize(&mut self, name: &str, value: usize) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn root(&mut self, name: &str, value: Root) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn option_root(&mut self, name: &str, value: Option<Root>) {
        name.hash(&mut self.state);
        value.hash(&mut self.state);
    }

    fn finish(self) -> Root {
        Root(self.state.finish())
    }
}

fn merkle_root<I>(roots: I) -> Root
where
    I: IntoIterator<Item = Root>,
{
    let mut h = RootBuilder::new("std-only-merkle-root");
    let mut count = 0_u64;
    for root in roots {
        h.root("leaf", root);
        count = count.saturating_add(1);
    }
    h.u64("count", count);
    h.finish()
}

fn clean_label(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        .take(96)
        .collect()
}
