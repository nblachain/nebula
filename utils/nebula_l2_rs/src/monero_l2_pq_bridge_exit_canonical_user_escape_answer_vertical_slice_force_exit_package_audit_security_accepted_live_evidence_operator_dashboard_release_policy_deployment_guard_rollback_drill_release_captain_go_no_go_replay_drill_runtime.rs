use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAuditSecurityAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-audit-security-release-captain-go-no-go-replay-drill-v1";
pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_EPOCH: u64 = 88;
pub const SOURCE_WAVE: u64 = 87;
pub const DEFAULT_REPLAY_HEIGHT: u64 = 880_000;
pub const DEFAULT_MIN_THREAT_FINDINGS: u16 = 5;
pub const DEFAULT_MIN_SIGNER_QUORUM_REVIEWS: u16 = 4;
pub const DEFAULT_MIN_RELEASE_CAPTAIN_SIGNOFFS: u16 = 3;
pub const DEFAULT_MAX_PRIVACY_LEAK_BUDGET_UNITS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRootKind {
    OperatorCommandChecklist,
    AuditSecurityChecklist,
    RollbackDrillEvidence,
    DeploymentGuardPolicy,
    AcceptedLiveEvidence,
    OperatorDashboardSnapshot,
}

impl AuditRootKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OperatorCommandChecklist,
            Self::AuditSecurityChecklist,
            Self::RollbackDrillEvidence,
            Self::DeploymentGuardPolicy,
            Self::AcceptedLiveEvidence,
            Self::OperatorDashboardSnapshot,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorCommandChecklist => "operator_command_checklist",
            Self::AuditSecurityChecklist => "audit_security_checklist",
            Self::RollbackDrillEvidence => "rollback_drill_evidence",
            Self::DeploymentGuardPolicy => "deployment_guard_policy",
            Self::AcceptedLiveEvidence => "accepted_live_evidence",
            Self::OperatorDashboardSnapshot => "operator_dashboard_snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFindingStatus {
    Clean,
    Mitigated,
    DeferredGate,
    PrivacyBlocked,
    QuorumBlocked,
    ReleaseBlocked,
}

impl ReplayFindingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Mitigated => "mitigated",
            Self::DeferredGate => "deferred_gate",
            Self::PrivacyBlocked => "privacy_blocked",
            Self::QuorumBlocked => "quorum_blocked",
            Self::ReleaseBlocked => "release_blocked",
        }
    }

    pub fn blocking(self) -> bool {
        matches!(
            self,
            Self::DeferredGate | Self::PrivacyBlocked | Self::QuorumBlocked | Self::ReleaseBlocked
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffDecision {
    Hold,
    GoAfterDeferredAudit,
    Go,
    NoGo,
    MoreEvidence,
}

impl SignoffDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hold => "hold",
            Self::GoAfterDeferredAudit => "go_after_deferred_audit",
            Self::Go => "go",
            Self::NoGo => "no_go",
            Self::MoreEvidence => "more_evidence",
        }
    }

    pub fn release_go(self) -> bool {
        matches!(self, Self::Go | Self::GoAfterDeferredAudit)
    }

    pub fn blocking(self) -> bool {
        !matches!(self, Self::Go)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingWave87Root,
    EmptyRoot,
    MissingThreatFinding,
    PrivacyLeakBudgetSpent,
    MissingPrivacyBudgetRoot,
    MissingSignerQuorumReview,
    SignerQuorumRejected,
    DeferredAuditGateOpen,
    MissingReleaseCaptainSignoff,
    ReleaseCaptainHold,
    ReleaseCaptainNoGo,
    MoreEvidenceRequired,
    ReplayFindingBlocksRelease,
    FailClosedVerdict,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave87Root => "missing_wave87_root",
            Self::EmptyRoot => "empty_root",
            Self::MissingThreatFinding => "missing_threat_finding",
            Self::PrivacyLeakBudgetSpent => "privacy_leak_budget_spent",
            Self::MissingPrivacyBudgetRoot => "missing_privacy_budget_root",
            Self::MissingSignerQuorumReview => "missing_signer_quorum_review",
            Self::SignerQuorumRejected => "signer_quorum_rejected",
            Self::DeferredAuditGateOpen => "deferred_audit_gate_open",
            Self::MissingReleaseCaptainSignoff => "missing_release_captain_signoff",
            Self::ReleaseCaptainHold => "release_captain_hold",
            Self::ReleaseCaptainNoGo => "release_captain_no_go",
            Self::MoreEvidenceRequired => "more_evidence_required",
            Self::ReplayFindingBlocksRelease => "replay_finding_blocks_release",
            Self::FailClosedVerdict => "fail_closed_verdict",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::EmptyRoot | Self::MissingWave87Root | Self::MissingThreatFinding => 2,
            Self::MissingPrivacyBudgetRoot | Self::MissingSignerQuorumReview => 3,
            Self::DeferredAuditGateOpen | Self::MoreEvidenceRequired => 4,
            Self::PrivacyLeakBudgetSpent
            | Self::SignerQuorumRejected
            | Self::ReleaseCaptainHold
            | Self::ReleaseCaptainNoGo
            | Self::ReplayFindingBlocksRelease
            | Self::MissingReleaseCaptainSignoff
            | Self::FailClosedVerdict => 5,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub release_epoch: u64,
    pub source_wave: u64,
    pub replay_height: u64,
    pub min_threat_findings: u16,
    pub min_signer_quorum_reviews: u16,
    pub min_release_captain_signoffs: u16,
    pub max_privacy_leak_budget_units: u64,
    pub required_wave87_roots: BTreeSet<AuditRootKind>,
    pub fail_closed_on_any_blocker: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            release_epoch: RELEASE_EPOCH,
            source_wave: SOURCE_WAVE,
            replay_height: DEFAULT_REPLAY_HEIGHT,
            min_threat_findings: DEFAULT_MIN_THREAT_FINDINGS,
            min_signer_quorum_reviews: DEFAULT_MIN_SIGNER_QUORUM_REVIEWS,
            min_release_captain_signoffs: DEFAULT_MIN_RELEASE_CAPTAIN_SIGNOFFS,
            max_privacy_leak_budget_units: DEFAULT_MAX_PRIVACY_LEAK_BUDGET_UNITS,
            required_wave87_roots: AuditRootKind::all().into_iter().collect(),
            fail_closed_on_any_blocker: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("chain_id", &self.chain_id)?;
        require(
            self.release_epoch >= self.source_wave,
            "release_epoch must follow source_wave",
        )?;
        require(self.replay_height > 0, "replay_height must be nonzero")?;
        require(
            self.min_threat_findings > 0,
            "min_threat_findings must be nonzero",
        )?;
        require(
            self.min_signer_quorum_reviews > 0,
            "min_signer_quorum_reviews must be nonzero",
        )?;
        require(
            self.min_release_captain_signoffs > 0,
            "min_release_captain_signoffs must be nonzero",
        )?;
        require(
            !self.required_wave87_roots.is_empty(),
            "required_wave87_roots must be non-empty",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "hash_suite": HASH_SUITE,
            "chain_id": self.chain_id,
            "release_epoch": self.release_epoch,
            "source_wave": self.source_wave,
            "replay_height": self.replay_height,
            "min_threat_findings": self.min_threat_findings,
            "min_signer_quorum_reviews": self.min_signer_quorum_reviews,
            "min_release_captain_signoffs": self.min_release_captain_signoffs,
            "max_privacy_leak_budget_units": self.max_privacy_leak_budget_units,
            "required_wave87_roots": self.required_wave87_roots.iter().map(|kind| kind.as_str()).collect::<Vec<_>>(),
            "fail_closed_on_any_blocker": self.fail_closed_on_any_blocker,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Wave87ChecklistRoot {
    pub kind: AuditRootKind,
    pub root: String,
    pub accepted: bool,
}

impl Wave87ChecklistRoot {
    pub fn devnet(kind: AuditRootKind, ordinal: u64) -> Self {
        Self {
            kind,
            root: sample_root("wave87-root", kind.as_str(), ordinal),
            accepted: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "root": self.root,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave87_checklist_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreatModelReplayFinding {
    pub finding_id: String,
    pub source_root_kind: AuditRootKind,
    pub evidence_root: String,
    pub status: ReplayFindingStatus,
    pub severity: u8,
    pub private_payload_redacted: bool,
}

impl ThreatModelReplayFinding {
    pub fn devnet(kind: AuditRootKind, ordinal: u64, status: ReplayFindingStatus) -> Self {
        Self {
            finding_id: stable_id("threat-finding", kind.as_str(), ordinal),
            source_root_kind: kind,
            evidence_root: sample_root("threat-finding-evidence", kind.as_str(), ordinal),
            status,
            severity: if status.blocking() { 5 } else { 1 },
            private_payload_redacted: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "finding_id": self.finding_id,
            "source_root_kind": self.source_root_kind.as_str(),
            "evidence_root": self.evidence_root,
            "status": self.status.as_str(),
            "severity": self.severity,
            "private_payload_redacted": self.private_payload_redacted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("threat_model_replay_finding", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivacyLeakBudget {
    pub budget_id: String,
    pub leak_budget_root: String,
    pub spent_units: u64,
    pub hard_cap_units: u64,
    pub blocker_active: bool,
}

impl PrivacyLeakBudget {
    pub fn devnet(config: &Config) -> Self {
        Self {
            budget_id: stable_id("privacy-leak-budget", "audit-security", 1),
            leak_budget_root: sample_root("privacy-leak-budget", "accepted-live-evidence", 1),
            spent_units: 0,
            hard_cap_units: config.max_privacy_leak_budget_units,
            blocker_active: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "budget_id": self.budget_id,
            "leak_budget_root": self.leak_budget_root,
            "spent_units": self.spent_units,
            "hard_cap_units": self.hard_cap_units,
            "blocker_active": self.blocker_active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("privacy_leak_budget", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignerQuorumReview {
    pub review_id: String,
    pub signer_policy_root: String,
    pub quorum_review_root: String,
    pub approved: bool,
    pub key_material_redacted: bool,
}

impl SignerQuorumReview {
    pub fn devnet(label: &str, ordinal: u64, approved: bool) -> Self {
        Self {
            review_id: stable_id("signer-quorum-review", label, ordinal),
            signer_policy_root: sample_root("signer-policy", label, ordinal),
            quorum_review_root: sample_root("quorum-review", label, ordinal),
            approved,
            key_material_redacted: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "review_id": self.review_id,
            "signer_policy_root": self.signer_policy_root,
            "quorum_review_root": self.quorum_review_root,
            "approved": self.approved,
            "key_material_redacted": self.key_material_redacted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("signer_quorum_review", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeferredAuditGate {
    pub gate_id: String,
    pub placeholder_root: String,
    pub closure_root: Option<String>,
    pub open: bool,
}

impl DeferredAuditGate {
    pub fn devnet(label: &str, ordinal: u64, open: bool) -> Self {
        Self {
            gate_id: stable_id("deferred-audit-gate", label, ordinal),
            placeholder_root: sample_root("deferred-audit-placeholder", label, ordinal),
            closure_root: if open {
                None
            } else {
                Some(sample_root("deferred-audit-closure", label, ordinal))
            },
            open,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate_id": self.gate_id,
            "placeholder_root": self.placeholder_root,
            "closure_root": self.closure_root,
            "open": self.open,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("deferred_audit_gate", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseCaptainSignoff {
    pub signoff_id: String,
    pub captain_role: String,
    pub signoff_root: String,
    pub decision: SignoffDecision,
    pub weight: u16,
}

impl ReleaseCaptainSignoff {
    pub fn devnet(role: &str, decision: SignoffDecision, weight: u16, ordinal: u64) -> Self {
        Self {
            signoff_id: stable_id("release-captain-signoff", role, ordinal),
            captain_role: role.to_string(),
            signoff_root: sample_root("release-captain-signoff", role, ordinal),
            decision,
            weight,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "captain_role": self.captain_role,
            "signoff_root": self.signoff_root,
            "decision": self.decision.as_str(),
            "weight": self.weight,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_captain_signoff", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GoNoGoVerdict {
    Go,
    NoGo,
}

impl GoNoGoVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Summary {
    pub verdict: GoNoGoVerdict,
    pub blocker_count: u16,
    pub max_blocker_severity: u8,
    pub threat_finding_count: u16,
    pub privacy_budget_spent_units: u64,
    pub signer_quorum_review_count: u16,
    pub release_captain_go_weight: u16,
    pub fail_closed: bool,
    pub summary_root: String,
}

impl Summary {
    pub fn build(
        config: &Config,
        threat_findings: &[ThreatModelReplayFinding],
        privacy_budget: &PrivacyLeakBudget,
        signer_reviews: &[SignerQuorumReview],
        signoffs: &[ReleaseCaptainSignoff],
        blockers: &BTreeMap<String, Vec<BlockerKind>>,
    ) -> Self {
        let blocker_count = blockers.values().map(|items| items.len()).sum::<usize>() as u16;
        let max_blocker_severity = match blockers
            .values()
            .flat_map(|items| items.iter())
            .map(|blocker| blocker.severity())
            .max()
        {
            Some(severity) => severity,
            None => 0,
        };
        let release_captain_go_weight = signoffs
            .iter()
            .filter(|signoff| signoff.decision.release_go())
            .map(|signoff| signoff.weight)
            .sum::<u16>();
        let fail_closed = config.fail_closed_on_any_blocker && blocker_count > 0;
        let verdict = if fail_closed || blocker_count > 0 {
            GoNoGoVerdict::NoGo
        } else {
            GoNoGoVerdict::Go
        };
        let root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-AUDIT-SECURITY-GO-NO-GO-SUMMARY",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(verdict.as_str()),
                HashPart::U64(blocker_count as u64),
                HashPart::U64(max_blocker_severity as u64),
                HashPart::U64(threat_findings.len() as u64),
                HashPart::U64(privacy_budget.spent_units),
                HashPart::U64(signer_reviews.len() as u64),
                HashPart::U64(release_captain_go_weight as u64),
            ],
            32,
        );
        Self {
            verdict,
            blocker_count,
            max_blocker_severity,
            threat_finding_count: threat_findings.len() as u16,
            privacy_budget_spent_units: privacy_budget.spent_units,
            signer_quorum_review_count: signer_reviews.len() as u16,
            release_captain_go_weight,
            fail_closed,
            summary_root: root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "blocker_count": self.blocker_count,
            "max_blocker_severity": self.max_blocker_severity,
            "threat_finding_count": self.threat_finding_count,
            "privacy_budget_spent_units": self.privacy_budget_spent_units,
            "signer_quorum_review_count": self.signer_quorum_review_count,
            "release_captain_go_weight": self.release_captain_go_weight,
            "fail_closed": self.fail_closed,
            "summary_root": self.summary_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub wave87_roots: Vec<Wave87ChecklistRoot>,
    pub threat_findings: Vec<ThreatModelReplayFinding>,
    pub privacy_budget: PrivacyLeakBudget,
    pub signer_quorum_reviews: Vec<SignerQuorumReview>,
    pub deferred_audit_gates: Vec<DeferredAuditGate>,
    pub release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
    pub blockers: BTreeMap<String, Vec<BlockerKind>>,
    pub wave87_root: String,
    pub threat_finding_root: String,
    pub privacy_budget_root: String,
    pub signer_quorum_root: String,
    pub deferred_audit_gate_root: String,
    pub release_captain_root: String,
    pub blocker_root: String,
    pub summary: Summary,
}

impl State {
    pub fn new(
        config: Config,
        wave87_roots: Vec<Wave87ChecklistRoot>,
        threat_findings: Vec<ThreatModelReplayFinding>,
        privacy_budget: PrivacyLeakBudget,
        signer_quorum_reviews: Vec<SignerQuorumReview>,
        deferred_audit_gates: Vec<DeferredAuditGate>,
        release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
    ) -> Result<Self> {
        config.validate()?;
        let blockers = evaluate_blockers(
            &config,
            &wave87_roots,
            &threat_findings,
            &privacy_budget,
            &signer_quorum_reviews,
            &deferred_audit_gates,
            &release_captain_signoffs,
        );
        let wave87_root = roots_root(
            "audit-security-wave87-roots",
            wave87_roots.iter().map(Wave87ChecklistRoot::state_root),
        );
        let threat_finding_root = roots_root(
            "audit-security-threat-findings",
            threat_findings
                .iter()
                .map(ThreatModelReplayFinding::state_root),
        );
        let privacy_budget_root = privacy_budget.state_root();
        let signer_quorum_root = roots_root(
            "audit-security-signer-quorum-reviews",
            signer_quorum_reviews
                .iter()
                .map(SignerQuorumReview::state_root),
        );
        let deferred_audit_gate_root = roots_root(
            "audit-security-deferred-audit-gates",
            deferred_audit_gates
                .iter()
                .map(DeferredAuditGate::state_root),
        );
        let release_captain_root = roots_root(
            "audit-security-release-captain-signoffs",
            release_captain_signoffs
                .iter()
                .map(ReleaseCaptainSignoff::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary = Summary::build(
            &config,
            &threat_findings,
            &privacy_budget,
            &signer_quorum_reviews,
            &release_captain_signoffs,
            &blockers,
        );
        Ok(Self {
            config,
            wave87_roots,
            threat_findings,
            privacy_budget,
            signer_quorum_reviews,
            deferred_audit_gates,
            release_captain_signoffs,
            blockers,
            wave87_root,
            threat_finding_root,
            privacy_budget_root,
            signer_quorum_root,
            deferred_audit_gate_root,
            release_captain_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave87_roots = AuditRootKind::all()
            .into_iter()
            .enumerate()
            .map(|(index, kind)| Wave87ChecklistRoot::devnet(kind, one_based(index)))
            .collect::<Vec<_>>();
        let threat_findings = AuditRootKind::all()
            .into_iter()
            .enumerate()
            .map(|(index, kind)| {
                ThreatModelReplayFinding::devnet(
                    kind,
                    one_based(index),
                    ReplayFindingStatus::Mitigated,
                )
            })
            .collect::<Vec<_>>();
        let privacy_budget = PrivacyLeakBudget::devnet(&config);
        let signer_quorum_reviews = vec![
            SignerQuorumReview::devnet("release-captain", 1, true),
            SignerQuorumReview::devnet("security-incident-lead", 2, true),
            SignerQuorumReview::devnet("bridge-custody-lead", 3, true),
            SignerQuorumReview::devnet("privacy-review-lead", 4, true),
        ];
        let deferred_audit_gates = vec![
            DeferredAuditGate::devnet("formal-heavy-gate", 1, false),
            DeferredAuditGate::devnet("privacy-heavy-gate", 2, false),
        ];
        let release_captain_signoffs = vec![
            ReleaseCaptainSignoff::devnet("release-captain", SignoffDecision::Go, 40, 1),
            ReleaseCaptainSignoff::devnet("security-captain", SignoffDecision::Go, 35, 2),
            ReleaseCaptainSignoff::devnet("rollback-captain", SignoffDecision::Go, 25, 3),
        ];
        match Self::new(
            config,
            wave87_roots,
            threat_findings,
            privacy_budget,
            signer_quorum_reviews,
            deferred_audit_gates,
            release_captain_signoffs,
        ) {
            Ok(state) => state,
            Err(reason) => build_fail_closed_state(reason),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        require(
            self.wave87_root
                == roots_root(
                    "audit-security-wave87-roots",
                    self.wave87_roots
                        .iter()
                        .map(Wave87ChecklistRoot::state_root),
                ),
            "wave87_root mismatch",
        )?;
        require(
            self.blocker_root == blockers_root(&self.blockers),
            "blocker_root mismatch",
        )?;
        if self.config.fail_closed_on_any_blocker {
            require(
                self.blockers.is_empty() || self.summary.verdict == GoNoGoVerdict::NoGo,
                "blockers must produce no_go verdict",
            )?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "roots": {
                "wave87_root": self.wave87_root,
                "threat_finding_root": self.threat_finding_root,
                "privacy_budget_root": self.privacy_budget_root,
                "signer_quorum_root": self.signer_quorum_root,
                "deferred_audit_gate_root": self.deferred_audit_gate_root,
                "release_captain_root": self.release_captain_root,
                "blocker_root": self.blocker_root,
            },
            "summary": self.summary.public_record(),
            "blockers": self.blockers.iter().map(|(subject, blockers)| {
                json!({
                    "subject": subject,
                    "blockers": blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                    "max_severity": blockers.iter().map(|blocker| blocker.severity()).max(),
                })
            }).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-AUDIT-SECURITY-GO-NO-GO-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.wave87_root),
                HashPart::Str(&self.threat_finding_root),
                HashPart::Str(&self.privacy_budget_root),
                HashPart::Str(&self.signer_quorum_root),
                HashPart::Str(&self.deferred_audit_gate_root),
                HashPart::Str(&self.release_captain_root),
                HashPart::Str(&self.blocker_root),
                HashPart::Json(&self.summary.public_record()),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn evaluate_blockers(
    config: &Config,
    wave87_roots: &[Wave87ChecklistRoot],
    threat_findings: &[ThreatModelReplayFinding],
    privacy_budget: &PrivacyLeakBudget,
    signer_reviews: &[SignerQuorumReview],
    deferred_gates: &[DeferredAuditGate],
    signoffs: &[ReleaseCaptainSignoff],
) -> BTreeMap<String, Vec<BlockerKind>> {
    let mut blockers = BTreeMap::<String, Vec<BlockerKind>>::new();
    for required in &config.required_wave87_roots {
        let matching = wave87_roots.iter().find(|root| root.kind == *required);
        match matching {
            Some(root) => {
                if root.root.trim().is_empty() {
                    blockers
                        .entry(required.as_str().to_string())
                        .or_default()
                        .push(BlockerKind::EmptyRoot);
                }
                if !root.accepted {
                    blockers
                        .entry(required.as_str().to_string())
                        .or_default()
                        .push(BlockerKind::MissingWave87Root);
                }
            }
            None => blockers
                .entry(required.as_str().to_string())
                .or_default()
                .push(BlockerKind::MissingWave87Root),
        }
    }
    if threat_findings.len() < config.min_threat_findings as usize {
        blockers
            .entry("threat_model_replay".to_string())
            .or_default()
            .push(BlockerKind::MissingThreatFinding);
    }
    for finding in threat_findings {
        if finding.status.blocking() {
            blockers
                .entry(finding.finding_id.clone())
                .or_default()
                .push(BlockerKind::ReplayFindingBlocksRelease);
        }
        if !finding.private_payload_redacted {
            blockers
                .entry(finding.finding_id.clone())
                .or_default()
                .push(BlockerKind::PrivacyLeakBudgetSpent);
        }
    }
    if privacy_budget.leak_budget_root.trim().is_empty() {
        blockers
            .entry("privacy_leak_budget".to_string())
            .or_default()
            .push(BlockerKind::MissingPrivacyBudgetRoot);
    }
    if privacy_budget.blocker_active
        || privacy_budget.spent_units > config.max_privacy_leak_budget_units
    {
        blockers
            .entry("privacy_leak_budget".to_string())
            .or_default()
            .push(BlockerKind::PrivacyLeakBudgetSpent);
    }
    if signer_reviews.len() < config.min_signer_quorum_reviews as usize {
        blockers
            .entry("signer_quorum".to_string())
            .or_default()
            .push(BlockerKind::MissingSignerQuorumReview);
    }
    for review in signer_reviews {
        if review.signer_policy_root.trim().is_empty()
            || review.quorum_review_root.trim().is_empty()
        {
            blockers
                .entry(review.review_id.clone())
                .or_default()
                .push(BlockerKind::EmptyRoot);
        }
        if !review.approved || !review.key_material_redacted {
            blockers
                .entry(review.review_id.clone())
                .or_default()
                .push(BlockerKind::SignerQuorumRejected);
        }
    }
    for gate in deferred_gates {
        if gate.placeholder_root.trim().is_empty() {
            blockers
                .entry(gate.gate_id.clone())
                .or_default()
                .push(BlockerKind::EmptyRoot);
        }
        if gate.open || option_is_blank(&gate.closure_root) {
            blockers
                .entry(gate.gate_id.clone())
                .or_default()
                .push(BlockerKind::DeferredAuditGateOpen);
        }
    }
    if signoffs.len() < config.min_release_captain_signoffs as usize {
        blockers
            .entry("release_captain_signoffs".to_string())
            .or_default()
            .push(BlockerKind::MissingReleaseCaptainSignoff);
    }
    for signoff in signoffs {
        if signoff.signoff_root.trim().is_empty() {
            blockers
                .entry(signoff.signoff_id.clone())
                .or_default()
                .push(BlockerKind::EmptyRoot);
        }
        if signoff.decision.blocking() {
            let blocker = match signoff.decision {
                SignoffDecision::Hold | SignoffDecision::GoAfterDeferredAudit => {
                    BlockerKind::ReleaseCaptainHold
                }
                SignoffDecision::NoGo => BlockerKind::ReleaseCaptainNoGo,
                SignoffDecision::MoreEvidence => BlockerKind::MoreEvidenceRequired,
                SignoffDecision::Go => BlockerKind::FailClosedVerdict,
            };
            blockers
                .entry(signoff.signoff_id.clone())
                .or_default()
                .push(blocker);
        }
    }
    if config.fail_closed_on_any_blocker && !blockers.is_empty() {
        blockers
            .entry("go_no_go".to_string())
            .or_default()
            .push(BlockerKind::FailClosedVerdict);
    }
    blockers
}

fn build_fail_closed_state(reason: String) -> State {
    let config = Config::devnet();
    let privacy_budget = PrivacyLeakBudget {
        blocker_active: true,
        ..PrivacyLeakBudget::devnet(&config)
    };
    let mut blockers = BTreeMap::<String, Vec<BlockerKind>>::new();
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(BlockerKind::FailClosedVerdict);
    blockers
        .entry(if reason.trim().is_empty() {
            "fallback_empty_reason".to_string()
        } else {
            "fallback_build_reason".to_string()
        })
        .or_default()
        .push(BlockerKind::MoreEvidenceRequired);
    let summary = Summary::build(&config, &[], &privacy_budget, &[], &[], &blockers);
    State {
        config,
        wave87_roots: Vec::new(),
        threat_findings: Vec::new(),
        privacy_budget,
        signer_quorum_reviews: Vec::new(),
        deferred_audit_gates: Vec::new(),
        release_captain_signoffs: Vec::new(),
        wave87_root: roots_root("audit-security-wave87-roots", Vec::<String>::new()),
        threat_finding_root: roots_root("audit-security-threat-findings", Vec::<String>::new()),
        privacy_budget_root: sample_root("fallback-privacy-budget", "fail-closed", 1),
        signer_quorum_root: roots_root(
            "audit-security-signer-quorum-reviews",
            Vec::<String>::new(),
        ),
        deferred_audit_gate_root: roots_root(
            "audit-security-deferred-audit-gates",
            Vec::<String>::new(),
        ),
        release_captain_root: roots_root(
            "audit-security-release-captain-signoffs",
            Vec::<String>::new(),
        ),
        blocker_root: blockers_root(&blockers),
        blockers,
        summary,
    }
}

fn option_is_blank(value: &Option<String>) -> bool {
    match value {
        Some(inner) => inner.trim().is_empty(),
        None => true,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<BlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            json!({
                "subject": subject,
                "blockers": blocker_list.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                "max_severity": blocker_list.iter().map(|blocker| blocker.severity()).max(),
            })
        })
        .collect::<Vec<_>>();
    merkle_root("audit-security-release-captain-blockers", &leaves)
}

fn roots_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-AUDIT-SECURITY-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn stable_id(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-AUDIT-SECURITY-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn sample_root(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-AUDIT-SECURITY-SAMPLE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn one_based(index: usize) -> u64 {
    index as u64 + 1
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<()> {
    require(
        !value.trim().is_empty(),
        &format!("{label} must be non-empty"),
    )
}
