use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, RuntimeError>;
pub type Runtime = State;

const LANE: &str = "wave91-live-heavy-gate-bridge-custody-receipt";
const DENIAL_DOMAIN: &str = "wave90-denial-output";
const WATCHER_DOMAIN: &str = "monero-watcher-quorum";
const WITHDRAWAL_DOMAIN: &str = "withdrawal-release";
const RESERVE_DOMAIN: &str = "reserve-coverage";
const SIGNER_DOMAIN: &str = "signer-quorum";
const CHALLENGE_DOMAIN: &str = "challenge-hold-review";
const OPERATOR_DOMAIN: &str = "custody-operator-signoff";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    EmptyRoot {
        field: &'static str,
    },
    NonRootMaterial {
        field: &'static str,
    },
    DuplicateItem {
        item_id: String,
    },
    MissingReceipt {
        blocker: Blocker,
    },
    ThresholdNotMet {
        blocker: Blocker,
        required: u16,
        supplied: u16,
    },
    RootMismatch {
        field: &'static str,
        supplied: String,
        required: String,
    },
    HoldStillActive {
        review_root: String,
    },
    ClearanceDenied {
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blocker {
    MoneroWatcherQuorum,
    WithdrawalRelease,
    ReserveCoverage,
    SignerQuorum,
    ChallengeHoldReview,
    CustodyOperatorSignoff,
}

impl Blocker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MoneroWatcherQuorum => "monero_watcher_quorum",
            Self::WithdrawalRelease => "withdrawal_release",
            Self::ReserveCoverage => "reserve_coverage",
            Self::SignerQuorum => "signer_quorum",
            Self::ChallengeHoldReview => "challenge_hold_review",
            Self::CustodyOperatorSignoff => "custody_operator_signoff",
        }
    }

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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GateMode {
    PlanningOnly,
    LiveReceiptIntake,
    FailClosedClearance,
}

impl GateMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PlanningOnly => "planning_only",
            Self::LiveReceiptIntake => "live_receipt_intake",
            Self::FailClosedClearance => "fail_closed_clearance",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReceiptStatus {
    Planned,
    Submitted,
    Accepted,
    Rejected,
}

impl ReceiptStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Submitted => "submitted",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerdictKind {
    Clear,
    Deny,
}

impl VerdictKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Deny => "deny",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandPriority {
    Now,
    BeforeRelease,
    AuditTrail,
}

impl CommandPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Now => "now",
            Self::BeforeRelease => "before_release",
            Self::AuditTrail => "audit_trail",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub lane_id: String,
    pub wave_id: u16,
    pub prior_wave_id: u16,
    pub mode: GateMode,
    pub min_watcher_votes: u16,
    pub min_signer_votes: u16,
    pub min_operator_signoffs: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_must_be_clear: bool,
    pub roots_only_privacy: bool,
    pub live_feed_binding_root: String,
    pub custody_policy_root: String,
    pub release_manifest_root: String,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            lane_id: LANE.to_string(),
            wave_id: 91,
            prior_wave_id: 90,
            mode: GateMode::FailClosedClearance,
            min_watcher_votes: 4,
            min_signer_votes: 3,
            min_operator_signoffs: 2,
            min_reserve_coverage_bps: 12_500,
            challenge_hold_must_be_clear: true,
            roots_only_privacy: true,
            live_feed_binding_root: root_for("LIVE-FEED-BINDING", &["wave91", "bridge-custody"]),
            custody_policy_root: root_for("CUSTODY-POLICY", &["monero", "force-exit", "pq"]),
            release_manifest_root: root_for("RELEASE-MANIFEST", &["custody", "heavy-gate"]),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("config");
        record.put_str("lane_id", &self.lane_id);
        record.put_num("wave_id", self.wave_id as u64);
        record.put_num("prior_wave_id", self.prior_wave_id as u64);
        record.put_str("mode", self.mode.as_str());
        record.put_num("min_watcher_votes", self.min_watcher_votes as u64);
        record.put_num("min_signer_votes", self.min_signer_votes as u64);
        record.put_num("min_operator_signoffs", self.min_operator_signoffs as u64);
        record.put_num(
            "min_reserve_coverage_bps",
            self.min_reserve_coverage_bps as u64,
        );
        record.put_bool(
            "challenge_hold_must_be_clear",
            self.challenge_hold_must_be_clear,
        );
        record.put_bool("roots_only_privacy", self.roots_only_privacy);
        record.put_str("live_feed_binding_root", &self.live_feed_binding_root);
        record.put_str("custody_policy_root", &self.custody_policy_root);
        record.put_str("release_manifest_root", &self.release_manifest_root);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicRecord {
    pub kind: String,
    pub fields: BTreeMap<String, String>,
    pub children: BTreeMap<String, Vec<PublicRecord>>,
}

impl PublicRecord {
    pub fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            fields: BTreeMap::new(),
            children: BTreeMap::new(),
        }
    }

    pub fn put_str(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn put_num(&mut self, key: &str, value: u64) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn put_bool(&mut self, key: &str, value: bool) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn put_child(&mut self, key: &str, child: PublicRecord) {
        self.children
            .entry(key.to_string())
            .or_default()
            .push(child);
    }

    pub fn root(&self, domain: &str) -> String {
        root_for(domain, &[&self.canonical()])
    }

    pub fn canonical(&self) -> String {
        let mut out = String::new();
        out.push_str("kind=");
        out.push_str(&self.kind);
        for (key, value) in &self.fields {
            out.push('|');
            out.push_str(key);
            out.push('=');
            out.push_str(value);
        }
        for (key, values) in &self.children {
            out.push('|');
            out.push_str(key);
            out.push('[');
            for value in values {
                out.push_str(&value.canonical());
                out.push(';');
            }
            out.push(']');
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DenialRootIntake {
    pub wave90_denial_root: String,
    pub denied_gate_root: String,
    pub denied_at_height_root: String,
    pub blocker_roots: Vec<BlockerRoot>,
    pub intake_record_root: String,
}

impl DenialRootIntake {
    pub fn devnet() -> Self {
        let blocker_roots = Blocker::all()
            .into_iter()
            .map(|blocker| BlockerRoot::devnet(blocker))
            .collect::<Vec<_>>();
        let wave90_denial_root = root_for(DENIAL_DOMAIN, &["wave90", "denied", "bridge-custody"]);
        let denied_gate_root = root_for("DENIED-GATE", &["heavy-gate", "live-custody"]);
        let denied_at_height_root = root_for("DENIED-HEIGHT", &["redacted-height-root"]);
        let intake_record_root = root_for(
            "DENIAL-INTAKE",
            &[
                &wave90_denial_root,
                &denied_gate_root,
                &denied_at_height_root,
            ],
        );
        Self {
            wave90_denial_root,
            denied_gate_root,
            denied_at_height_root,
            blocker_roots,
            intake_record_root,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_root("wave90_denial_root", &self.wave90_denial_root)?;
        require_root("denied_gate_root", &self.denied_gate_root)?;
        require_root("denied_at_height_root", &self.denied_at_height_root)?;
        require_root("intake_record_root", &self.intake_record_root)?;
        for blocker in &self.blocker_roots {
            blocker.validate()?;
        }
        Ok(())
    }

    pub fn root_for_blocker(&self, blocker: &Blocker) -> Option<String> {
        self.blocker_roots
            .iter()
            .find(|item| &item.blocker == blocker)
            .map(|item| item.blocker_root.clone())
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("denial_root_intake");
        record.put_str("wave90_denial_root", &self.wave90_denial_root);
        record.put_str("denied_gate_root", &self.denied_gate_root);
        record.put_str("denied_at_height_root", &self.denied_at_height_root);
        record.put_str("intake_record_root", &self.intake_record_root);
        for blocker in &self.blocker_roots {
            record.put_child("blocker_roots", blocker.public_record());
        }
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockerRoot {
    pub blocker: Blocker,
    pub blocker_root: String,
    pub denial_reason_root: String,
}

impl BlockerRoot {
    pub fn devnet(blocker: Blocker) -> Self {
        let name = blocker.as_str();
        Self {
            blocker,
            blocker_root: root_for("BLOCKER", &[name, "root"]),
            denial_reason_root: root_for("DENIAL-REASON", &[name, "redacted"]),
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_root("blocker_root", &self.blocker_root)?;
        require_root("denial_reason_root", &self.denial_reason_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("blocker_root");
        record.put_str("blocker", self.blocker.as_str());
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("denial_reason_root", &self.denial_reason_root);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustodyReceiptPlan {
    pub plan_id: String,
    pub denial_intake_root: String,
    pub watcher_plan: WatcherReceiptPlan,
    pub withdrawal_plan: WithdrawalReleasePlan,
    pub reserve_plan: ReserveReceiptPlan,
    pub signer_plan: SignerQuorumPlan,
    pub challenge_review: ChallengeHoldReviewPlan,
    pub operator_signoff: OperatorSignoffPlan,
    pub acceptance: AcceptanceCriteria,
    pub command_hints: Vec<OperatorCommandHint>,
}

impl CustodyReceiptPlan {
    pub fn from_config_and_denial(config: &Config, denial: &DenialRootIntake) -> Self {
        let plan_id = root_for(
            "CUSTODY-RECEIPT-PLAN-ID",
            &[&config.lane_id, &denial.intake_record_root],
        );
        Self {
            plan_id,
            denial_intake_root: denial.intake_record_root.clone(),
            watcher_plan: WatcherReceiptPlan::devnet(config, denial),
            withdrawal_plan: WithdrawalReleasePlan::devnet(denial),
            reserve_plan: ReserveReceiptPlan::devnet(config, denial),
            signer_plan: SignerQuorumPlan::devnet(config, denial),
            challenge_review: ChallengeHoldReviewPlan::devnet(denial),
            operator_signoff: OperatorSignoffPlan::devnet(config, denial),
            acceptance: AcceptanceCriteria::devnet(config),
            command_hints: OperatorCommandHint::devnet_all(),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("custody_receipt_plan");
        record.put_str("plan_id", &self.plan_id);
        record.put_str("denial_intake_root", &self.denial_intake_root);
        record.put_child("watcher_plan", self.watcher_plan.public_record());
        record.put_child("withdrawal_plan", self.withdrawal_plan.public_record());
        record.put_child("reserve_plan", self.reserve_plan.public_record());
        record.put_child("signer_plan", self.signer_plan.public_record());
        record.put_child("challenge_review", self.challenge_review.public_record());
        record.put_child("operator_signoff", self.operator_signoff.public_record());
        record.put_child("acceptance", self.acceptance.public_record());
        for hint in &self.command_hints {
            record.put_child("command_hints", hint.public_record());
        }
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WatcherReceiptPlan {
    pub blocker_root: String,
    pub quorum_domain_root: String,
    pub required_votes: u16,
    pub receipt_action: String,
    pub public_evidence_root: String,
    pub action_hint_root: String,
}

impl WatcherReceiptPlan {
    pub fn devnet(config: &Config, denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::MoneroWatcherQuorum);
        Self {
            quorum_domain_root: root_for(WATCHER_DOMAIN, &[&blocker_root, &config.live_feed_binding_root]),
            required_votes: config.min_watcher_votes,
            receipt_action: "collect independent watcher attestation roots for lock observation, reorg depth, and release lane readiness".to_string(),
            public_evidence_root: root_for("WATCHER-EVIDENCE", &[&blocker_root, "roots-only"]),
            action_hint_root: root_for("WATCHER-ACTION-HINT", &[&blocker_root]),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("watcher_receipt_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("quorum_domain_root", &self.quorum_domain_root);
        record.put_num("required_votes", self.required_votes as u64);
        record.put_str("receipt_action", &self.receipt_action);
        record.put_str("public_evidence_root", &self.public_evidence_root);
        record.put_str("action_hint_root", &self.action_hint_root);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithdrawalReleasePlan {
    pub blocker_root: String,
    pub release_queue_root: String,
    pub release_authority_root: String,
    pub release_receipt_root: String,
    pub receipt_action: String,
}

impl WithdrawalReleasePlan {
    pub fn devnet(denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::WithdrawalRelease);
        Self {
            release_queue_root: root_for(WITHDRAWAL_DOMAIN, &[&blocker_root, "queue"]),
            release_authority_root: root_for(WITHDRAWAL_DOMAIN, &[&blocker_root, "authority"]),
            release_receipt_root: root_for(WITHDRAWAL_DOMAIN, &[&blocker_root, "receipt"]),
            receipt_action: "publish withdrawal release root binding claim batch, release authority, and finalized custody exit allowance".to_string(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("withdrawal_release_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("release_queue_root", &self.release_queue_root);
        record.put_str("release_authority_root", &self.release_authority_root);
        record.put_str("release_receipt_root", &self.release_receipt_root);
        record.put_str("receipt_action", &self.receipt_action);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReserveReceiptPlan {
    pub blocker_root: String,
    pub reserve_snapshot_root: String,
    pub coverage_ratio_bps: u16,
    pub reserve_policy_root: String,
    pub coverage_receipt_root: String,
    pub receipt_action: String,
}

impl ReserveReceiptPlan {
    pub fn devnet(config: &Config, denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::ReserveCoverage);
        Self {
            reserve_snapshot_root: root_for(RESERVE_DOMAIN, &[&blocker_root, "snapshot"]),
            coverage_ratio_bps: config.min_reserve_coverage_bps + 750,
            reserve_policy_root: root_for(RESERVE_DOMAIN, &[&blocker_root, "policy"]),
            coverage_receipt_root: root_for(RESERVE_DOMAIN, &[&blocker_root, "coverage"]),
            receipt_action: "bind reserve coverage root to release queue demand root with surplus margin and no raw reserve venue material".to_string(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("reserve_receipt_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("reserve_snapshot_root", &self.reserve_snapshot_root);
        record.put_num("coverage_ratio_bps", self.coverage_ratio_bps as u64);
        record.put_str("reserve_policy_root", &self.reserve_policy_root);
        record.put_str("coverage_receipt_root", &self.coverage_receipt_root);
        record.put_str("receipt_action", &self.receipt_action);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignerQuorumPlan {
    pub blocker_root: String,
    pub quorum_root: String,
    pub required_votes: u16,
    pub signature_set_root: String,
    pub key_epoch_root: String,
    pub receipt_action: String,
}

impl SignerQuorumPlan {
    pub fn devnet(config: &Config, denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::SignerQuorum);
        Self {
            quorum_root: root_for(SIGNER_DOMAIN, &[&blocker_root, "quorum"]),
            required_votes: config.min_signer_votes,
            signature_set_root: root_for(SIGNER_DOMAIN, &[&blocker_root, "signature-set"]),
            key_epoch_root: root_for(SIGNER_DOMAIN, &[&blocker_root, "key-epoch"]),
            receipt_action: "collect signer quorum roots over release manifest, denial intake, reserve coverage, and watcher quorum roots".to_string(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("signer_quorum_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("quorum_root", &self.quorum_root);
        record.put_num("required_votes", self.required_votes as u64);
        record.put_str("signature_set_root", &self.signature_set_root);
        record.put_str("key_epoch_root", &self.key_epoch_root);
        record.put_str("receipt_action", &self.receipt_action);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeHoldReviewPlan {
    pub blocker_root: String,
    pub review_root: String,
    pub open_challenge_count: u16,
    pub hold_clearance_root: String,
    pub dispute_window_root: String,
    pub receipt_action: String,
}

impl ChallengeHoldReviewPlan {
    pub fn devnet(denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::ChallengeHoldReview);
        Self {
            review_root: root_for(CHALLENGE_DOMAIN, &[&blocker_root, "review"]),
            open_challenge_count: 0,
            hold_clearance_root: root_for(CHALLENGE_DOMAIN, &[&blocker_root, "clear"]),
            dispute_window_root: root_for(CHALLENGE_DOMAIN, &[&blocker_root, "window"]),
            receipt_action:
                "record challenge hold review root showing no active custody dispute blocks release"
                    .to_string(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("challenge_hold_review_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("review_root", &self.review_root);
        record.put_num("open_challenge_count", self.open_challenge_count as u64);
        record.put_str("hold_clearance_root", &self.hold_clearance_root);
        record.put_str("dispute_window_root", &self.dispute_window_root);
        record.put_str("receipt_action", &self.receipt_action);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorSignoffPlan {
    pub blocker_root: String,
    pub signoff_bundle_root: String,
    pub required_signoffs: u16,
    pub runbook_ack_root: String,
    pub release_captain_root: String,
    pub receipt_action: String,
}

impl OperatorSignoffPlan {
    pub fn devnet(config: &Config, denial: &DenialRootIntake) -> Self {
        let blocker_root = blocker_root_or_fallback(denial, &Blocker::CustodyOperatorSignoff);
        Self {
            signoff_bundle_root: root_for(OPERATOR_DOMAIN, &[&blocker_root, "bundle"]),
            required_signoffs: config.min_operator_signoffs,
            runbook_ack_root: root_for(OPERATOR_DOMAIN, &[&blocker_root, "runbook"]),
            release_captain_root: root_for(OPERATOR_DOMAIN, &[&blocker_root, "captain"]),
            receipt_action: "capture custody operator signoff roots for release captain approval, rollback hold, and incident handoff readiness".to_string(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("operator_signoff_plan");
        record.put_str("blocker_root", &self.blocker_root);
        record.put_str("signoff_bundle_root", &self.signoff_bundle_root);
        record.put_num("required_signoffs", self.required_signoffs as u64);
        record.put_str("runbook_ack_root", &self.runbook_ack_root);
        record.put_str("release_captain_root", &self.release_captain_root);
        record.put_str("receipt_action", &self.receipt_action);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptanceCriteria {
    pub require_roots_only: bool,
    pub require_denial_root_match: bool,
    pub require_all_blockers_cleared: bool,
    pub require_fail_closed: bool,
    pub min_watcher_votes: u16,
    pub min_signer_votes: u16,
    pub min_operator_signoffs: u16,
    pub min_reserve_coverage_bps: u16,
}

impl AcceptanceCriteria {
    pub fn devnet(config: &Config) -> Self {
        Self {
            require_roots_only: true,
            require_denial_root_match: true,
            require_all_blockers_cleared: true,
            require_fail_closed: true,
            min_watcher_votes: config.min_watcher_votes,
            min_signer_votes: config.min_signer_votes,
            min_operator_signoffs: config.min_operator_signoffs,
            min_reserve_coverage_bps: config.min_reserve_coverage_bps,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("acceptance_criteria");
        record.put_bool("require_roots_only", self.require_roots_only);
        record.put_bool("require_denial_root_match", self.require_denial_root_match);
        record.put_bool(
            "require_all_blockers_cleared",
            self.require_all_blockers_cleared,
        );
        record.put_bool("require_fail_closed", self.require_fail_closed);
        record.put_num("min_watcher_votes", self.min_watcher_votes as u64);
        record.put_num("min_signer_votes", self.min_signer_votes as u64);
        record.put_num("min_operator_signoffs", self.min_operator_signoffs as u64);
        record.put_num(
            "min_reserve_coverage_bps",
            self.min_reserve_coverage_bps as u64,
        );
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorCommandHint {
    pub command_id: String,
    pub priority: CommandPriority,
    pub blocker: Blocker,
    pub public_action: String,
    pub input_root_name: String,
    pub output_root_name: String,
}

impl OperatorCommandHint {
    pub fn devnet_all() -> Vec<Self> {
        vec![
            Self::new(
                CommandPriority::Now,
                Blocker::MoneroWatcherQuorum,
                "request watcher quorum receipt roots",
                "denial_blocker_root",
                "watcher_quorum_root",
            ),
            Self::new(
                CommandPriority::Now,
                Blocker::ReserveCoverage,
                "publish reserve coverage receipt root",
                "reserve_policy_root",
                "coverage_receipt_root",
            ),
            Self::new(
                CommandPriority::BeforeRelease,
                Blocker::WithdrawalRelease,
                "bind withdrawal release queue and authority roots",
                "claim_batch_root",
                "release_receipt_root",
            ),
            Self::new(
                CommandPriority::BeforeRelease,
                Blocker::SignerQuorum,
                "collect signer quorum root over custody release manifest",
                "release_manifest_root",
                "signature_set_root",
            ),
            Self::new(
                CommandPriority::BeforeRelease,
                Blocker::ChallengeHoldReview,
                "record challenge hold review clearance root",
                "challenge_window_root",
                "hold_clearance_root",
            ),
            Self::new(
                CommandPriority::AuditTrail,
                Blocker::CustodyOperatorSignoff,
                "publish operator signoff bundle root",
                "runbook_ack_root",
                "signoff_bundle_root",
            ),
        ]
    }

    pub fn new(
        priority: CommandPriority,
        blocker: Blocker,
        public_action: &str,
        input_root_name: &str,
        output_root_name: &str,
    ) -> Self {
        let command_id = root_for(
            "OPERATOR-COMMAND-HINT",
            &[
                priority.as_str(),
                blocker.as_str(),
                public_action,
                input_root_name,
                output_root_name,
            ],
        );
        Self {
            command_id,
            priority,
            blocker,
            public_action: public_action.to_string(),
            input_root_name: input_root_name.to_string(),
            output_root_name: output_root_name.to_string(),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("operator_command_hint");
        record.put_str("command_id", &self.command_id);
        record.put_str("priority", self.priority.as_str());
        record.put_str("blocker", self.blocker.as_str());
        record.put_str("public_action", &self.public_action);
        record.put_str("input_root_name", &self.input_root_name);
        record.put_str("output_root_name", &self.output_root_name);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveReceipt {
    pub blocker: Blocker,
    pub status: ReceiptStatus,
    pub denial_blocker_root: String,
    pub receipt_root: String,
    pub evidence_root: String,
    pub vote_count: u16,
    pub coverage_bps: u16,
    pub open_hold_count: u16,
}

impl LiveReceipt {
    pub fn accepted(
        blocker: Blocker,
        denial_blocker_root: String,
        evidence_root: String,
        vote_count: u16,
        coverage_bps: u16,
        open_hold_count: u16,
    ) -> Self {
        let receipt_root = root_for(
            "LIVE-RECEIPT",
            &[
                blocker.as_str(),
                &denial_blocker_root,
                &evidence_root,
                &vote_count.to_string(),
                &coverage_bps.to_string(),
                &open_hold_count.to_string(),
            ],
        );
        Self {
            blocker,
            status: ReceiptStatus::Accepted,
            denial_blocker_root,
            receipt_root,
            evidence_root,
            vote_count,
            coverage_bps,
            open_hold_count,
        }
    }

    pub fn validate_roots(&self) -> Result<()> {
        require_root("denial_blocker_root", &self.denial_blocker_root)?;
        require_root("receipt_root", &self.receipt_root)?;
        require_root("evidence_root", &self.evidence_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("live_receipt");
        record.put_str("blocker", self.blocker.as_str());
        record.put_str("status", self.status.as_str());
        record.put_str("denial_blocker_root", &self.denial_blocker_root);
        record.put_str("receipt_root", &self.receipt_root);
        record.put_str("evidence_root", &self.evidence_root);
        record.put_num("vote_count", self.vote_count as u64);
        record.put_num("coverage_bps", self.coverage_bps as u64);
        record.put_num("open_hold_count", self.open_hold_count as u64);
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClearanceVerdict {
    pub kind: VerdictKind,
    pub fail_closed: bool,
    pub cleared_blockers: Vec<Blocker>,
    pub denied_blockers: Vec<Blocker>,
    pub denial_reasons: Vec<String>,
    pub verdict_root: String,
}

impl ClearanceVerdict {
    pub fn deny(reasons: Vec<String>, denied_blockers: Vec<Blocker>) -> Self {
        let reason_material = reasons.join("|");
        let blocker_material = denied_blockers
            .iter()
            .map(Blocker::as_str)
            .collect::<Vec<_>>()
            .join("|");
        Self {
            kind: VerdictKind::Deny,
            fail_closed: true,
            cleared_blockers: Vec::new(),
            denied_blockers,
            denial_reasons: reasons,
            verdict_root: root_for(
                "CLEARANCE-VERDICT",
                &["deny", &reason_material, &blocker_material],
            ),
        }
    }

    pub fn clear(cleared_blockers: Vec<Blocker>) -> Self {
        let blocker_material = cleared_blockers
            .iter()
            .map(Blocker::as_str)
            .collect::<Vec<_>>()
            .join("|");
        Self {
            kind: VerdictKind::Clear,
            fail_closed: true,
            cleared_blockers,
            denied_blockers: Vec::new(),
            denial_reasons: Vec::new(),
            verdict_root: root_for("CLEARANCE-VERDICT", &["clear", &blocker_material]),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("clearance_verdict");
        record.put_str("kind", self.kind.as_str());
        record.put_bool("fail_closed", self.fail_closed);
        record.put_str("verdict_root", &self.verdict_root);
        for blocker in &self.cleared_blockers {
            let mut child = PublicRecord::new("cleared_blocker");
            child.put_str("blocker", blocker.as_str());
            record.put_child("cleared_blockers", child);
        }
        for blocker in &self.denied_blockers {
            let mut child = PublicRecord::new("denied_blocker");
            child.put_str("blocker", blocker.as_str());
            record.put_child("denied_blockers", child);
        }
        for reason in &self.denial_reasons {
            let mut child = PublicRecord::new("denial_reason");
            child.put_str("reason", reason);
            record.put_child("denial_reasons", child);
        }
        record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub config: Config,
    pub denial: DenialRootIntake,
    pub plan: CustodyReceiptPlan,
    pub receipts: Vec<LiveReceipt>,
    pub verdict: ClearanceVerdict,
}

impl State {
    pub fn new(config: Config, denial: DenialRootIntake, receipts: Vec<LiveReceipt>) -> Self {
        let plan = CustodyReceiptPlan::from_config_and_denial(&config, &denial);
        let verdict = evaluate_clearance(&config, &denial, &plan.acceptance, &receipts);
        Self {
            config,
            denial,
            plan,
            receipts,
            verdict,
        }
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let denial = DenialRootIntake::devnet();
        let plan = CustodyReceiptPlan::from_config_and_denial(&config, &denial);
        let receipts = devnet_live_receipts(&config, &denial, &plan);
        let verdict = evaluate_clearance(&config, &denial, &plan.acceptance, &receipts);
        Self {
            config,
            denial,
            plan,
            receipts,
            verdict,
        }
    }

    pub fn intake_denial(&mut self, denial: DenialRootIntake) {
        self.denial = denial;
        self.plan = CustodyReceiptPlan::from_config_and_denial(&self.config, &self.denial);
        self.verdict = evaluate_clearance(
            &self.config,
            &self.denial,
            &self.plan.acceptance,
            &self.receipts,
        );
    }

    pub fn replace_receipts(&mut self, receipts: Vec<LiveReceipt>) {
        self.receipts = receipts;
        self.verdict = evaluate_clearance(
            &self.config,
            &self.denial,
            &self.plan.acceptance,
            &self.receipts,
        );
    }

    pub fn receipt_for(&self, blocker: &Blocker) -> Option<&LiveReceipt> {
        self.receipts
            .iter()
            .find(|receipt| &receipt.blocker == blocker)
    }

    pub fn fail_closed_clearance_verdict(&self) -> ClearanceVerdict {
        evaluate_clearance(
            &self.config,
            &self.denial,
            &self.plan.acceptance,
            &self.receipts,
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = PublicRecord::new("wave91_live_heavy_gate_bridge_custody_runtime");
        record.put_str("lane", LANE);
        record.put_child("config", self.config.public_record());
        record.put_child("denial", self.denial.public_record());
        record.put_child("plan", self.plan.public_record());
        for receipt in &self.receipts {
            record.put_child("receipts", receipt.public_record());
        }
        record.put_child("verdict", self.verdict.public_record());
        record.put_str("state_root", &self.state_root());
        record
    }

    pub fn state_root(&self) -> String {
        root_for(
            "WAVE91-BRIDGE-CUSTODY-STATE",
            &[
                &self.config.public_record().root("CONFIG"),
                &self.denial.public_record().root("DENIAL"),
                &self.plan.public_record().root("PLAN"),
                &receipt_set_root(&self.receipts),
                &self.verdict.public_record().root("VERDICT"),
            ],
        )
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

pub fn evaluate_clearance(
    config: &Config,
    denial: &DenialRootIntake,
    criteria: &AcceptanceCriteria,
    receipts: &[LiveReceipt],
) -> ClearanceVerdict {
    let mut reasons = Vec::new();
    let mut denied = Vec::new();
    if denial.validate().is_err() {
        reasons.push("denial intake root validation failed".to_string());
        denied.extend(Blocker::all());
        return ClearanceVerdict::deny(reasons, denied);
    }
    if criteria.require_roots_only && !config.roots_only_privacy {
        reasons.push("roots only privacy flag is not active".to_string());
        denied.extend(Blocker::all());
        return ClearanceVerdict::deny(reasons, denied);
    }
    let mut cleared = Vec::new();
    for blocker in Blocker::all() {
        match validate_receipt_for_blocker(config, criteria, denial, receipts, &blocker) {
            Ok(()) => cleared.push(blocker),
            Err(reason) => {
                reasons.push(reason);
                denied.push(blocker);
            }
        }
    }
    if denied.is_empty() {
        ClearanceVerdict::clear(cleared)
    } else {
        ClearanceVerdict::deny(reasons, denied)
    }
}

fn validate_receipt_for_blocker(
    config: &Config,
    criteria: &AcceptanceCriteria,
    denial: &DenialRootIntake,
    receipts: &[LiveReceipt],
    blocker: &Blocker,
) -> std::result::Result<(), String> {
    let receipt = match receipts.iter().find(|item| &item.blocker == blocker) {
        Some(item) => item,
        None => return Err(format!("missing receipt for {}", blocker.as_str())),
    };
    if receipt.validate_roots().is_err() {
        return Err(format!(
            "receipt roots failed validation for {}",
            blocker.as_str()
        ));
    }
    if receipt.status != ReceiptStatus::Accepted {
        return Err(format!("receipt not accepted for {}", blocker.as_str()));
    }
    if criteria.require_denial_root_match {
        let root = match denial.root_for_blocker(blocker) {
            Some(value) => value,
            None => {
                return Err(format!(
                    "denial blocker root absent for {}",
                    blocker.as_str()
                ))
            }
        };
        if root != receipt.denial_blocker_root {
            return Err(format!(
                "denial blocker root mismatch for {}",
                blocker.as_str()
            ));
        }
    }
    match blocker {
        Blocker::MoneroWatcherQuorum if receipt.vote_count < criteria.min_watcher_votes => Err(
            format!("watcher quorum below {}", criteria.min_watcher_votes),
        ),
        Blocker::SignerQuorum if receipt.vote_count < criteria.min_signer_votes => {
            Err(format!("signer quorum below {}", criteria.min_signer_votes))
        }
        Blocker::CustodyOperatorSignoff if receipt.vote_count < criteria.min_operator_signoffs => {
            Err(format!(
                "operator signoff below {}",
                criteria.min_operator_signoffs
            ))
        }
        Blocker::ReserveCoverage if receipt.coverage_bps < criteria.min_reserve_coverage_bps => {
            Err(format!(
                "reserve coverage below {}",
                criteria.min_reserve_coverage_bps
            ))
        }
        Blocker::ChallengeHoldReview
            if config.challenge_hold_must_be_clear && receipt.open_hold_count != 0 =>
        {
            Err("challenge hold remains active".to_string())
        }
        _ => Ok(()),
    }
}

fn devnet_live_receipts(
    config: &Config,
    denial: &DenialRootIntake,
    plan: &CustodyReceiptPlan,
) -> Vec<LiveReceipt> {
    vec![
        LiveReceipt::accepted(
            Blocker::MoneroWatcherQuorum,
            plan.watcher_plan.blocker_root.clone(),
            plan.watcher_plan.public_evidence_root.clone(),
            config.min_watcher_votes,
            0,
            0,
        ),
        LiveReceipt::accepted(
            Blocker::WithdrawalRelease,
            plan.withdrawal_plan.blocker_root.clone(),
            plan.withdrawal_plan.release_receipt_root.clone(),
            0,
            0,
            0,
        ),
        LiveReceipt::accepted(
            Blocker::ReserveCoverage,
            plan.reserve_plan.blocker_root.clone(),
            plan.reserve_plan.coverage_receipt_root.clone(),
            0,
            plan.reserve_plan.coverage_ratio_bps,
            0,
        ),
        LiveReceipt::accepted(
            Blocker::SignerQuorum,
            plan.signer_plan.blocker_root.clone(),
            plan.signer_plan.signature_set_root.clone(),
            config.min_signer_votes,
            0,
            0,
        ),
        LiveReceipt::accepted(
            Blocker::ChallengeHoldReview,
            plan.challenge_review.blocker_root.clone(),
            plan.challenge_review.hold_clearance_root.clone(),
            0,
            0,
            plan.challenge_review.open_challenge_count,
        ),
        LiveReceipt::accepted(
            Blocker::CustodyOperatorSignoff,
            plan.operator_signoff.blocker_root.clone(),
            plan.operator_signoff.signoff_bundle_root.clone(),
            config.min_operator_signoffs,
            0,
            0,
        ),
    ]
    .into_iter()
    .filter(|receipt| denial.root_for_blocker(&receipt.blocker).is_some())
    .collect()
}

fn receipt_set_root(receipts: &[LiveReceipt]) -> String {
    let mut roots = receipts
        .iter()
        .map(|receipt| receipt.public_record().root("RECEIPT"))
        .collect::<Vec<_>>();
    roots.sort();
    let parts = roots.iter().map(String::as_str).collect::<Vec<_>>();
    root_for("RECEIPT-SET", &parts)
}

fn blocker_root_or_fallback(denial: &DenialRootIntake, blocker: &Blocker) -> String {
    match denial.root_for_blocker(blocker) {
        Some(value) => value,
        None => root_for("MISSING-BLOCKER", &[blocker.as_str()]),
    }
}

fn require_root(field: &'static str, value: &str) -> Result<()> {
    if value.is_empty() {
        return Err(RuntimeError::EmptyRoot { field });
    }
    if !is_public_root(value) {
        return Err(RuntimeError::NonRootMaterial { field });
    }
    Ok(())
}

fn is_public_root(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|item| item.is_ascii_hexdigit())
}

fn root_for(domain: &str, parts: &[&str]) -> String {
    let mut h0: u64 = 0x243f_6a88_85a3_08d3;
    let mut h1: u64 = 0x1319_8a2e_0370_7344;
    let mut h2: u64 = 0xa409_3822_299f_31d0;
    let mut h3: u64 = 0x082e_fa98_ec4e_6c89;
    mix_bytes(&mut h0, &mut h1, &mut h2, &mut h3, domain.as_bytes());
    for part in parts {
        mix_bytes(&mut h0, &mut h1, &mut h2, &mut h3, &[0xff]);
        mix_bytes(&mut h0, &mut h1, &mut h2, &mut h3, part.as_bytes());
    }
    format!("{:016x}{:016x}{:016x}{:016x}", h0, h1, h2, h3)
}

fn mix_bytes(h0: &mut u64, h1: &mut u64, h2: &mut u64, h3: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *h0 = h0.rotate_left(5) ^ u64::from(*byte);
        *h0 = h0.wrapping_mul(0x1000_0000_01b3);
        *h1 ^= h0.rotate_left(17).wrapping_add(0x9e37_79b9_7f4a_7c15);
        *h1 = h1.wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
        *h2 ^= h1.rotate_left(29).wrapping_add(*h0);
        *h2 = h2.wrapping_mul(0x1656_67b1_9e37_79f9);
        *h3 ^= h2.rotate_left(41).wrapping_add(*h1);
        *h3 = h3.wrapping_mul(0x85eb_ca6b_27d4_eb2f);
    }
    *h0 ^= h3.rotate_right(7);
    *h1 ^= h0.rotate_right(11);
    *h2 ^= h1.rotate_right(13);
    *h3 ^= h2.rotate_right(17);
}
