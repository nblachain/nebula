use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = ReceiptSlotRuntime;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave92-live-heavy-gate-receipt-slot-registry-v1";
pub const EMPTY_ROOT: &str =
    "root:wave92:0000000000000000000000000000000000000000000000000000000000000000";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const DEVNET_PLAN_ROOT: &str = "root:wave91:bridge-custody-execution-plan-output-placeholder";
pub const DEFAULT_MIN_WATCHER_QUORUM: u16 = 4;
pub const DEFAULT_MIN_SIGNER_QUORUM: u16 = 3;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF: u16 = 2;
pub const DEFAULT_RESERVE_COVERAGE_BPS: u16 = 10_000;
pub const DEFAULT_CHALLENGE_HOLD_BLOCKS: u64 = 720;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave91_plan_root: String,
    pub min_watcher_quorum: u16,
    pub min_signer_quorum: u16,
    pub min_operator_signoff: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_blocks: u64,
    pub accepted_root_domain: String,
    pub allow_provisional_roots: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave91_plan_root: DEVNET_PLAN_ROOT.to_string(),
            min_watcher_quorum: DEFAULT_MIN_WATCHER_QUORUM,
            min_signer_quorum: DEFAULT_MIN_SIGNER_QUORUM,
            min_operator_signoff: DEFAULT_MIN_OPERATOR_SIGNOFF,
            min_reserve_coverage_bps: DEFAULT_RESERVE_COVERAGE_BPS,
            challenge_hold_blocks: DEFAULT_CHALLENGE_HOLD_BLOCKS,
            accepted_root_domain: "accepted-live-heavy-gate-receipt-root".to_string(),
            allow_provisional_roots: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_root_like("wave91_plan_root", &self.wave91_plan_root)?;
        ensure_text("accepted_root_domain", &self.accepted_root_domain)?;
        if self.min_watcher_quorum == 0 {
            return Err("watcher quorum must be nonzero".to_string());
        }
        if self.min_signer_quorum == 0 {
            return Err("signer quorum must be nonzero".to_string());
        }
        if self.min_operator_signoff == 0 {
            return Err("operator signoff quorum must be nonzero".to_string());
        }
        if self.min_reserve_coverage_bps > 10_000 {
            return Err("reserve coverage basis points exceed full coverage".to_string());
        }
        if self.challenge_hold_blocks == 0 {
            return Err("challenge hold window must be nonzero".to_string());
        }
        Ok(())
    }

    pub fn root(&self) -> String {
        root_for(
            "config",
            &[
                self.chain_id.as_str(),
                self.lane_id.as_str(),
                self.protocol_version.as_str(),
                self.wave91_plan_root.as_str(),
                &self.min_watcher_quorum.to_string(),
                &self.min_signer_quorum.to_string(),
                &self.min_operator_signoff.to_string(),
                &self.min_reserve_coverage_bps.to_string(),
                &self.challenge_hold_blocks.to_string(),
                self.accepted_root_domain.as_str(),
                bool_text(self.allow_provisional_roots),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub config: Config,
    pub plan_intake: PlanRootIntake,
    pub slots: BTreeMap<ReceiptSlotKind, ReceiptSlot>,
    pub import_rules: Vec<ImportRule>,
    pub operator_hints: Vec<OperatorCommandHint>,
    pub last_verdict: SlotRegistryVerdict,
}

impl State {
    pub fn new(config: Config) -> Self {
        let plan_intake = PlanRootIntake::new(config.wave91_plan_root.clone());
        let mut slots = BTreeMap::new();
        for kind in ReceiptSlotKind::all() {
            slots.insert(kind, ReceiptSlot::empty(kind));
        }
        let import_rules = ImportRule::canonical_rules(&config);
        let operator_hints = OperatorCommandHint::canonical_hints();
        Self {
            config,
            plan_intake,
            slots,
            import_rules,
            operator_hints,
            last_verdict: SlotRegistryVerdict::fail_closed("state starts with empty slots"),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.plan_intake.validate()?;
        if self.slots.len() != ReceiptSlotKind::all().len() {
            return Err("slot registry cardinality is invalid".to_string());
        }
        for kind in ReceiptSlotKind::all() {
            let slot = self
                .slots
                .get(&kind)
                .ok_or_else(|| format!("missing receipt slot {}", kind.as_str()))?;
            if slot.kind != kind {
                return Err("receipt slot kind mismatch".to_string());
            }
            slot.validate(&self.config)?;
        }
        for rule in &self.import_rules {
            rule.validate()?;
        }
        for hint in &self.operator_hints {
            hint.validate()?;
        }
        Ok(())
    }

    pub fn attach_receipt(&mut self, receipt: AcceptedReceiptPlaceholder) -> Result<()> {
        self.validate()?;
        receipt.validate(&self.config, &self.plan_intake)?;
        let kind = receipt.slot_kind;
        let slot = self
            .slots
            .get_mut(&kind)
            .ok_or_else(|| format!("missing receipt slot {}", kind.as_str()))?;
        slot.attach(receipt)?;
        self.last_verdict = self.evaluate();
        Ok(())
    }

    pub fn apply_plan_intake(&mut self, intake: PlanRootIntake) -> Result<()> {
        intake.validate()?;
        if intake.plan_root != self.config.wave91_plan_root {
            return Err("plan intake root does not match configured wave91 root".to_string());
        }
        self.plan_intake = intake;
        self.last_verdict = self.evaluate();
        Ok(())
    }

    pub fn evaluate(&self) -> SlotRegistryVerdict {
        if let Err(reason) = self.validate() {
            return SlotRegistryVerdict::fail_closed(reason);
        }

        let mut blockers = Vec::new();
        for kind in ReceiptSlotKind::all() {
            if let Some(slot) = self.slots.get(&kind) {
                blockers.extend(slot.blockers(&self.config, &self.plan_intake));
            } else {
                blockers.push(CustodySlotBlocker::new(
                    kind,
                    BlockerKind::SlotMissing,
                    "canonical slot is absent",
                ));
            }
        }

        if blockers.is_empty() {
            SlotRegistryVerdict {
                status: RegistryStatus::Clearable,
                clearable: true,
                blockers,
                public_root: self.root(),
                command_hint_root: self.command_hint_root(),
            }
        } else {
            SlotRegistryVerdict {
                status: RegistryStatus::FailClosed,
                clearable: false,
                blockers,
                public_root: self.root(),
                command_hint_root: self.command_hint_root(),
            }
        }
    }

    pub fn root(&self) -> String {
        let slot_roots = self
            .slots
            .values()
            .map(ReceiptSlot::root)
            .collect::<Vec<String>>();
        let rule_roots = self
            .import_rules
            .iter()
            .map(ImportRule::root)
            .collect::<Vec<String>>();
        root_for(
            "state",
            &[
                self.config.root().as_str(),
                self.plan_intake.root().as_str(),
                fold_roots("slots", &slot_roots).as_str(),
                fold_roots("rules", &rule_roots).as_str(),
                self.command_hint_root().as_str(),
                self.last_verdict.root().as_str(),
            ],
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            protocol_version: self.config.protocol_version.clone(),
            chain_id: self.config.chain_id.clone(),
            lane_id: self.config.lane_id.clone(),
            wave91_plan_root: self.config.wave91_plan_root.clone(),
            registry_root: self.root(),
            plan_intake_root: self.plan_intake.root(),
            slot_roots: self
                .slots
                .iter()
                .map(|(kind, slot)| PublicSlotRecord {
                    kind: *kind,
                    slot_root: slot.root(),
                    attached: slot.receipt.is_some(),
                    clearable: slot.is_clearable(&self.config, &self.plan_intake),
                })
                .collect(),
            verdict: self.evaluate(),
        }
    }

    pub fn command_hint_root(&self) -> String {
        let roots = self
            .operator_hints
            .iter()
            .map(OperatorCommandHint::root)
            .collect::<Vec<String>>();
        fold_roots("operator-command-hints", &roots)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptSlotRuntime {
    pub state: State,
}

impl ReceiptSlotRuntime {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let mut state = State::new(config);
        state.last_verdict = state.evaluate();
        Ok(Self { state })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::devnet()) {
            Ok(runtime) => runtime,
            Err(reason) => Self {
                state: State::new(Config {
                    chain_id: "invalid-devnet".to_string(),
                    lane_id: "invalid".to_string(),
                    protocol_version: PROTOCOL_VERSION.to_string(),
                    wave91_plan_root: EMPTY_ROOT.to_string(),
                    min_watcher_quorum: 1,
                    min_signer_quorum: 1,
                    min_operator_signoff: 1,
                    min_reserve_coverage_bps: 10_000,
                    challenge_hold_blocks: 1,
                    accepted_root_domain: "accepted-live-heavy-gate-receipt-root".to_string(),
                    allow_provisional_roots: false,
                }),
            }
            .with_failure(reason),
        }
    }

    fn with_failure(mut self, reason: String) -> Self {
        self.state.last_verdict = SlotRegistryVerdict::fail_closed(reason);
        self
    }

    pub fn attach_receipt_root(
        &mut self,
        slot_kind: ReceiptSlotKind,
        accepted_root: impl Into<String>,
        source_plan_root: impl Into<String>,
        live_gate_epoch: u64,
        quorum_count: u16,
    ) -> Result<()> {
        let receipt = AcceptedReceiptPlaceholder {
            slot_kind,
            accepted_root: accepted_root.into(),
            source_plan_root: source_plan_root.into(),
            live_gate_epoch,
            quorum_count,
            provisional: false,
            evidence_root: EMPTY_ROOT.to_string(),
            privacy_summary_root: EMPTY_ROOT.to_string(),
        };
        self.state.attach_receipt(receipt)
    }

    pub fn verdict(&self) -> SlotRegistryVerdict {
        self.state.evaluate()
    }

    pub fn public_record(&self) -> PublicRecord {
        self.state.public_record()
    }

    pub fn state_root(&self) -> String {
        self.state.root()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanRootIntake {
    pub plan_root: String,
    pub execution_plan_output_root: String,
    pub custody_route_root: String,
    pub release_policy_root: String,
    pub reserve_policy_root: String,
    pub signer_policy_root: String,
    pub challenge_policy_root: String,
    pub operator_policy_root: String,
}

impl PlanRootIntake {
    pub fn new(plan_root: String) -> Self {
        Self {
            plan_root: plan_root.clone(),
            execution_plan_output_root: root_for("wave91-execution-plan-output", &[&plan_root]),
            custody_route_root: EMPTY_ROOT.to_string(),
            release_policy_root: EMPTY_ROOT.to_string(),
            reserve_policy_root: EMPTY_ROOT.to_string(),
            signer_policy_root: EMPTY_ROOT.to_string(),
            challenge_policy_root: EMPTY_ROOT.to_string(),
            operator_policy_root: EMPTY_ROOT.to_string(),
        }
    }

    pub fn with_policy_roots(
        mut self,
        custody_route_root: impl Into<String>,
        release_policy_root: impl Into<String>,
        reserve_policy_root: impl Into<String>,
        signer_policy_root: impl Into<String>,
        challenge_policy_root: impl Into<String>,
        operator_policy_root: impl Into<String>,
    ) -> Self {
        self.custody_route_root = custody_route_root.into();
        self.release_policy_root = release_policy_root.into();
        self.reserve_policy_root = reserve_policy_root.into();
        self.signer_policy_root = signer_policy_root.into();
        self.challenge_policy_root = challenge_policy_root.into();
        self.operator_policy_root = operator_policy_root.into();
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root_like("plan_root", &self.plan_root)?;
        ensure_root_like(
            "execution_plan_output_root",
            &self.execution_plan_output_root,
        )?;
        for (name, value) in [
            ("custody_route_root", &self.custody_route_root),
            ("release_policy_root", &self.release_policy_root),
            ("reserve_policy_root", &self.reserve_policy_root),
            ("signer_policy_root", &self.signer_policy_root),
            ("challenge_policy_root", &self.challenge_policy_root),
            ("operator_policy_root", &self.operator_policy_root),
        ] {
            ensure_root_like(name, value)?;
        }
        Ok(())
    }

    pub fn is_policy_bound(&self) -> bool {
        [
            &self.custody_route_root,
            &self.release_policy_root,
            &self.reserve_policy_root,
            &self.signer_policy_root,
            &self.challenge_policy_root,
            &self.operator_policy_root,
        ]
        .iter()
        .all(|root| root.as_str() != EMPTY_ROOT)
    }

    pub fn root(&self) -> String {
        root_for(
            "plan-root-intake",
            &[
                self.plan_root.as_str(),
                self.execution_plan_output_root.as_str(),
                self.custody_route_root.as_str(),
                self.release_policy_root.as_str(),
                self.reserve_policy_root.as_str(),
                self.signer_policy_root.as_str(),
                self.challenge_policy_root.as_str(),
                self.operator_policy_root.as_str(),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReceiptSlotKind {
    MoneroWatcherQuorum,
    WithdrawalRelease,
    ReserveCoverage,
    SignerQuorum,
    ChallengeHoldReview,
    CustodyOperatorSignoff,
}

impl ReceiptSlotKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::MoneroWatcherQuorum,
            Self::WithdrawalRelease,
            Self::ReserveCoverage,
            Self::SignerQuorum,
            Self::ChallengeHoldReview,
            Self::CustodyOperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroWatcherQuorum => "monero_watcher_quorum",
            Self::WithdrawalRelease => "withdrawal_release",
            Self::ReserveCoverage => "reserve_coverage",
            Self::SignerQuorum => "signer_quorum",
            Self::ChallengeHoldReview => "challenge_hold_review",
            Self::CustodyOperatorSignoff => "custody_operator_signoff",
        }
    }

    pub fn minimum_quorum(self, config: &Config) -> u16 {
        match self {
            Self::MoneroWatcherQuorum => config.min_watcher_quorum,
            Self::SignerQuorum => config.min_signer_quorum,
            Self::CustodyOperatorSignoff => config.min_operator_signoff,
            Self::ReserveCoverage => 1,
            Self::WithdrawalRelease => 1,
            Self::ChallengeHoldReview => 1,
        }
    }

    pub fn policy_root<'a>(self, intake: &'a PlanRootIntake) -> &'a str {
        match self {
            Self::MoneroWatcherQuorum => intake.custody_route_root.as_str(),
            Self::WithdrawalRelease => intake.release_policy_root.as_str(),
            Self::ReserveCoverage => intake.reserve_policy_root.as_str(),
            Self::SignerQuorum => intake.signer_policy_root.as_str(),
            Self::ChallengeHoldReview => intake.challenge_policy_root.as_str(),
            Self::CustodyOperatorSignoff => intake.operator_policy_root.as_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptSlot {
    pub kind: ReceiptSlotKind,
    pub slot_root: String,
    pub receipt: Option<AcceptedReceiptPlaceholder>,
}

impl ReceiptSlot {
    pub fn empty(kind: ReceiptSlotKind) -> Self {
        Self {
            kind,
            slot_root: root_for("empty-receipt-slot", &[kind.as_str(), EMPTY_ROOT]),
            receipt: None,
        }
    }

    pub fn attach(&mut self, receipt: AcceptedReceiptPlaceholder) -> Result<()> {
        if receipt.slot_kind != self.kind {
            return Err("receipt kind does not match slot kind".to_string());
        }
        self.slot_root = root_for(
            "attached-receipt-slot",
            &[self.kind.as_str(), receipt.root().as_str()],
        );
        self.receipt = Some(receipt);
        Ok(())
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root_like("slot_root", &self.slot_root)?;
        if let Some(receipt) = &self.receipt {
            let intake = PlanRootIntake::new(receipt.source_plan_root.clone());
            receipt.validate(config, &intake)?;
            if receipt.slot_kind != self.kind {
                return Err("receipt slot kind mismatch".to_string());
            }
        }
        Ok(())
    }

    pub fn blockers(&self, config: &Config, intake: &PlanRootIntake) -> Vec<CustodySlotBlocker> {
        let mut blockers = Vec::new();
        let Some(receipt) = &self.receipt else {
            blockers.push(CustodySlotBlocker::new(
                self.kind,
                BlockerKind::ReceiptRootMissing,
                "accepted live receipt root is not attached",
            ));
            return blockers;
        };

        if let Err(reason) = receipt.validate(config, intake) {
            blockers.push(CustodySlotBlocker::new(
                self.kind,
                BlockerKind::ReceiptRejected,
                reason,
            ));
        }
        if receipt.quorum_count < self.kind.minimum_quorum(config) {
            blockers.push(CustodySlotBlocker::new(
                self.kind,
                BlockerKind::QuorumBelowFloor,
                "attached receipt quorum is below configured floor",
            ));
        }
        if self.kind.policy_root(intake) == EMPTY_ROOT {
            blockers.push(CustodySlotBlocker::new(
                self.kind,
                BlockerKind::PlanPolicyRootMissing,
                "wave91 policy root for this slot is empty",
            ));
        }
        if receipt.provisional && !config.allow_provisional_roots {
            blockers.push(CustodySlotBlocker::new(
                self.kind,
                BlockerKind::ProvisionalRoot,
                "provisional receipt roots are disabled",
            ));
        }
        blockers
    }

    pub fn is_clearable(&self, config: &Config, intake: &PlanRootIntake) -> bool {
        self.blockers(config, intake).is_empty()
    }

    pub fn root(&self) -> String {
        let receipt_root = match &self.receipt {
            Some(receipt) => receipt.root(),
            None => EMPTY_ROOT.to_string(),
        };
        root_for(
            "receipt-slot",
            &[
                self.kind.as_str(),
                self.slot_root.as_str(),
                receipt_root.as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedReceiptPlaceholder {
    pub slot_kind: ReceiptSlotKind,
    pub accepted_root: String,
    pub source_plan_root: String,
    pub live_gate_epoch: u64,
    pub quorum_count: u16,
    pub provisional: bool,
    pub evidence_root: String,
    pub privacy_summary_root: String,
}

impl AcceptedReceiptPlaceholder {
    pub fn validate(&self, config: &Config, intake: &PlanRootIntake) -> Result<()> {
        ensure_root_like("accepted_root", &self.accepted_root)?;
        ensure_root_like("source_plan_root", &self.source_plan_root)?;
        ensure_root_like("evidence_root", &self.evidence_root)?;
        ensure_root_like("privacy_summary_root", &self.privacy_summary_root)?;
        if self.accepted_root == EMPTY_ROOT {
            return Err("accepted receipt root must be nonempty".to_string());
        }
        if self.source_plan_root != intake.plan_root {
            return Err("receipt source plan root mismatch".to_string());
        }
        if !self.accepted_root.contains(&config.accepted_root_domain) {
            return Err("accepted receipt root domain is not approved".to_string());
        }
        if self.quorum_count == 0 {
            return Err("receipt quorum count must be nonzero".to_string());
        }
        if self.live_gate_epoch == 0 {
            return Err("live gate epoch must be nonzero".to_string());
        }
        Ok(())
    }

    pub fn root(&self) -> String {
        root_for(
            "accepted-receipt-placeholder",
            &[
                self.slot_kind.as_str(),
                self.accepted_root.as_str(),
                self.source_plan_root.as_str(),
                &self.live_gate_epoch.to_string(),
                &self.quorum_count.to_string(),
                bool_text(self.provisional),
                self.evidence_root.as_str(),
                self.privacy_summary_root.as_str(),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockerKind {
    SlotMissing,
    ReceiptRootMissing,
    ReceiptRejected,
    QuorumBelowFloor,
    PlanPolicyRootMissing,
    ProvisionalRoot,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SlotMissing => "slot_missing",
            Self::ReceiptRootMissing => "receipt_root_missing",
            Self::ReceiptRejected => "receipt_rejected",
            Self::QuorumBelowFloor => "quorum_below_floor",
            Self::PlanPolicyRootMissing => "plan_policy_root_missing",
            Self::ProvisionalRoot => "provisional_root",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustodySlotBlocker {
    pub slot_kind: ReceiptSlotKind,
    pub blocker_kind: BlockerKind,
    pub reason: String,
    pub blocker_root: String,
}

impl CustodySlotBlocker {
    pub fn new(
        slot_kind: ReceiptSlotKind,
        blocker_kind: BlockerKind,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let blocker_root = root_for(
            "custody-slot-blocker",
            &[slot_kind.as_str(), blocker_kind.as_str(), reason.as_str()],
        );
        Self {
            slot_kind,
            blocker_kind,
            reason,
            blocker_root,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportRuleKind {
    RootsOnly,
    PlanBound,
    AcceptedLiveDomain,
    QuorumFloor,
    FailClosedDefault,
    PrivacyNoPayload,
}

impl ImportRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootsOnly => "roots_only",
            Self::PlanBound => "plan_bound",
            Self::AcceptedLiveDomain => "accepted_live_domain",
            Self::QuorumFloor => "quorum_floor",
            Self::FailClosedDefault => "fail_closed_default",
            Self::PrivacyNoPayload => "privacy_no_payload",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportRule {
    pub rule_kind: ImportRuleKind,
    pub slot_kind: Option<ReceiptSlotKind>,
    pub rule_root: String,
    pub note: String,
}

impl ImportRule {
    pub fn canonical_rules(config: &Config) -> Vec<Self> {
        let mut rules = vec![
            Self::new(
                ImportRuleKind::RootsOnly,
                None,
                "registry accepts roots and counters only",
            ),
            Self::new(
                ImportRuleKind::PlanBound,
                None,
                "all receipts bind to the wave91 execution plan root",
            ),
            Self::new(
                ImportRuleKind::AcceptedLiveDomain,
                None,
                format!("receipt root must carry {}", config.accepted_root_domain),
            ),
            Self::new(
                ImportRuleKind::FailClosedDefault,
                None,
                "empty slots keep custody clearability disabled",
            ),
            Self::new(
                ImportRuleKind::PrivacyNoPayload,
                None,
                "no raw monero ids addresses labels keys or payloads are stored",
            ),
        ];
        for kind in ReceiptSlotKind::all() {
            rules.push(Self::new(
                ImportRuleKind::QuorumFloor,
                Some(kind),
                format!("slot quorum floor is {}", kind.minimum_quorum(config)),
            ));
        }
        rules
    }

    pub fn new(
        rule_kind: ImportRuleKind,
        slot_kind: Option<ReceiptSlotKind>,
        note: impl Into<String>,
    ) -> Self {
        let note = note.into();
        let slot_text = match slot_kind {
            Some(kind) => kind.as_str(),
            None => "all_slots",
        };
        let rule_root = root_for(
            "import-rule",
            &[rule_kind.as_str(), slot_text, note.as_str()],
        );
        Self {
            rule_kind,
            slot_kind,
            rule_root,
            note,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root_like("rule_root", &self.rule_root)?;
        ensure_text("rule_note", &self.note)?;
        Ok(())
    }

    pub fn root(&self) -> String {
        let slot_text = match self.slot_kind {
            Some(kind) => kind.as_str(),
            None => "all_slots",
        };
        root_for(
            "import-rule-record",
            &[
                self.rule_kind.as_str(),
                slot_text,
                self.rule_root.as_str(),
                self.note.as_str(),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorCommandKind {
    AttachWatcherQuorumRoot,
    AttachWithdrawalReleaseRoot,
    AttachReserveCoverageRoot,
    AttachSignerQuorumRoot,
    AttachChallengeHoldReviewRoot,
    AttachCustodyOperatorSignoffRoot,
    RecomputeVerdict,
}

impl OperatorCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttachWatcherQuorumRoot => "attach_watcher_quorum_root",
            Self::AttachWithdrawalReleaseRoot => "attach_withdrawal_release_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachSignerQuorumRoot => "attach_signer_quorum_root",
            Self::AttachChallengeHoldReviewRoot => "attach_challenge_hold_review_root",
            Self::AttachCustodyOperatorSignoffRoot => "attach_custody_operator_signoff_root",
            Self::RecomputeVerdict => "recompute_verdict",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorCommandHint {
    pub command_kind: OperatorCommandKind,
    pub target_slot: Option<ReceiptSlotKind>,
    pub hint_root: String,
    pub dry_run_only: bool,
}

impl OperatorCommandHint {
    pub fn canonical_hints() -> Vec<Self> {
        vec![
            Self::new(
                OperatorCommandKind::AttachWatcherQuorumRoot,
                Some(ReceiptSlotKind::MoneroWatcherQuorum),
            ),
            Self::new(
                OperatorCommandKind::AttachWithdrawalReleaseRoot,
                Some(ReceiptSlotKind::WithdrawalRelease),
            ),
            Self::new(
                OperatorCommandKind::AttachReserveCoverageRoot,
                Some(ReceiptSlotKind::ReserveCoverage),
            ),
            Self::new(
                OperatorCommandKind::AttachSignerQuorumRoot,
                Some(ReceiptSlotKind::SignerQuorum),
            ),
            Self::new(
                OperatorCommandKind::AttachChallengeHoldReviewRoot,
                Some(ReceiptSlotKind::ChallengeHoldReview),
            ),
            Self::new(
                OperatorCommandKind::AttachCustodyOperatorSignoffRoot,
                Some(ReceiptSlotKind::CustodyOperatorSignoff),
            ),
            Self::new(OperatorCommandKind::RecomputeVerdict, None),
        ]
    }

    pub fn new(command_kind: OperatorCommandKind, target_slot: Option<ReceiptSlotKind>) -> Self {
        let slot_text = match target_slot {
            Some(kind) => kind.as_str(),
            None => "registry",
        };
        let hint_root = root_for("operator-command-hint", &[command_kind.as_str(), slot_text]);
        Self {
            command_kind,
            target_slot,
            hint_root,
            dry_run_only: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root_like("hint_root", &self.hint_root)
    }

    pub fn root(&self) -> String {
        let slot_text = match self.target_slot {
            Some(kind) => kind.as_str(),
            None => "registry",
        };
        root_for(
            "operator-command-hint-record",
            &[
                self.command_kind.as_str(),
                slot_text,
                self.hint_root.as_str(),
                bool_text(self.dry_run_only),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistryStatus {
    FailClosed,
    Clearable,
}

impl RegistryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Clearable => "clearable",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotRegistryVerdict {
    pub status: RegistryStatus,
    pub clearable: bool,
    pub blockers: Vec<CustodySlotBlocker>,
    pub public_root: String,
    pub command_hint_root: String,
}

impl SlotRegistryVerdict {
    pub fn fail_closed(reason: impl Into<String>) -> Self {
        let blocker = CustodySlotBlocker::new(
            ReceiptSlotKind::CustodyOperatorSignoff,
            BlockerKind::ReceiptRootMissing,
            reason,
        );
        Self {
            status: RegistryStatus::FailClosed,
            clearable: false,
            blockers: vec![blocker],
            public_root: EMPTY_ROOT.to_string(),
            command_hint_root: EMPTY_ROOT.to_string(),
        }
    }

    pub fn root(&self) -> String {
        let blocker_roots = self
            .blockers
            .iter()
            .map(|blocker| blocker.blocker_root.clone())
            .collect::<Vec<String>>();
        root_for(
            "slot-registry-verdict",
            &[
                self.status.as_str(),
                bool_text(self.clearable),
                fold_roots("verdict-blockers", &blocker_roots).as_str(),
                self.public_root.as_str(),
                self.command_hint_root.as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicRecord {
    pub protocol_version: String,
    pub chain_id: String,
    pub lane_id: String,
    pub wave91_plan_root: String,
    pub registry_root: String,
    pub plan_intake_root: String,
    pub slot_roots: Vec<PublicSlotRecord>,
    pub verdict: SlotRegistryVerdict,
}

impl PublicRecord {
    pub fn root(&self) -> String {
        let slot_roots = self
            .slot_roots
            .iter()
            .map(PublicSlotRecord::root)
            .collect::<Vec<String>>();
        root_for(
            "public-record",
            &[
                self.protocol_version.as_str(),
                self.chain_id.as_str(),
                self.lane_id.as_str(),
                self.wave91_plan_root.as_str(),
                self.registry_root.as_str(),
                self.plan_intake_root.as_str(),
                fold_roots("public-slot-roots", &slot_roots).as_str(),
                self.verdict.root().as_str(),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicSlotRecord {
    pub kind: ReceiptSlotKind,
    pub slot_root: String,
    pub attached: bool,
    pub clearable: bool,
}

impl PublicSlotRecord {
    pub fn root(&self) -> String {
        root_for(
            "public-slot-record",
            &[
                self.kind.as_str(),
                self.slot_root.as_str(),
                bool_text(self.attached),
                bool_text(self.clearable),
            ],
        )
    }
}

pub fn devnet() -> Runtime {
    ReceiptSlotRuntime::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn ensure_text(name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(format!("{name} must be nonempty"))
    } else {
        Ok(())
    }
}

fn ensure_root_like(name: &str, value: &str) -> Result<()> {
    ensure_text(name, value)?;
    if value.contains("txid")
        || value.contains("address")
        || value.contains("spend_key")
        || value.contains("view_key")
        || value.contains("signer_label")
        || value.contains("payload")
    {
        return Err(format!(
            "{name} contains disallowed private material marker"
        ));
    }
    Ok(())
}

fn bool_text(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn root_for(domain: &str, parts: &[&str]) -> String {
    let mut state = 0xcbf2_9ce4_8422_2325_u64;
    absorb(&mut state, domain);
    for part in parts {
        absorb(&mut state, "|");
        absorb(&mut state, part);
    }
    let a = finalize(state, 0x9e37_79b9_7f4a_7c15);
    let b = finalize(state ^ a, 0xbf58_476d_1ce4_e5b9);
    let c = finalize(state ^ b, 0x94d0_49bb_1331_11eb);
    let d = finalize(state ^ c, 0xd6e8_feb8_6659_fd93);
    format!("root:{domain}:{a:016x}{b:016x}{c:016x}{d:016x}")
}

fn fold_roots(domain: &str, roots: &[String]) -> String {
    let mut parts = Vec::with_capacity(roots.len());
    for root in roots {
        parts.push(root.as_str());
    }
    root_for(domain, &parts)
}

fn absorb(state: &mut u64, text: &str) {
    for byte in text.as_bytes() {
        *state ^= u64::from(*byte);
        *state = state.wrapping_mul(0x0000_0100_0000_01b3);
        *state ^= *state >> 32;
    }
}

fn finalize(mut value: u64, salt: u64) -> u64 {
    value ^= salt;
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}
