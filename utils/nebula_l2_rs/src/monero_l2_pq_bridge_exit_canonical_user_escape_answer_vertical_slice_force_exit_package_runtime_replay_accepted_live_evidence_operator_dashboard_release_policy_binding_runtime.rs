use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeReplayAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-replay-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_POLICY_BINDING_SUITE: &str =
    "runtime-replay-accepted-live-evidence-operator-dashboard-release-policy-binding-v1";
pub const DEFAULT_HEIGHT: u64 = 83_082;
pub const DEFAULT_REPLAY_ROOT_COUNT: u16 = 5;
pub const DEFAULT_MIN_COMMAND_RECEIPTS: u16 = 5;
pub const DEFAULT_MIN_DASHBOARD_APPROVALS: u16 = 4;
pub const DEFAULT_MIN_REVIEWER_QUORUM: u16 = 3;
pub const DEFAULT_MAX_DASHBOARD_AGE_BLOCKS: u64 = 72;
pub const DEFAULT_MAX_WATCH_BLOCKERS: u16 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayCommandKind {
    ImportAcceptedLiveEvidence,
    LoadCanonicalUserEscapeAnswer,
    LoadForceExitPackage,
    ExecuteRuntimeReplay,
    CompareReplayRoot,
    PublishOperatorDashboard,
    BindReleasePolicy,
}

impl ReplayCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportAcceptedLiveEvidence => "import_accepted_live_evidence",
            Self::LoadCanonicalUserEscapeAnswer => "load_canonical_user_escape_answer",
            Self::LoadForceExitPackage => "load_force_exit_package",
            Self::ExecuteRuntimeReplay => "execute_runtime_replay",
            Self::CompareReplayRoot => "compare_replay_root",
            Self::PublishOperatorDashboard => "publish_operator_dashboard",
            Self::BindReleasePolicy => "bind_release_policy",
        }
    }

    pub fn required() -> [Self; 7] {
        [
            Self::ImportAcceptedLiveEvidence,
            Self::LoadCanonicalUserEscapeAnswer,
            Self::LoadForceExitPackage,
            Self::ExecuteRuntimeReplay,
            Self::CompareReplayRoot,
            Self::PublishOperatorDashboard,
            Self::BindReleasePolicy,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Accepted,
    Replayed,
    Rejected,
    Superseded,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Replayed => "replayed",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
        }
    }

    pub fn release_usable(self) -> bool {
        matches!(self, Self::Accepted | Self::Replayed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayRootStatus {
    Matched,
    Watch,
    Mismatched,
    Missing,
}

impl ReplayRootStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Watch => "watch",
            Self::Mismatched => "mismatched",
            Self::Missing => "missing",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Mismatched | Self::Missing)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRole {
    ReplayOperator,
    DashboardOwner,
    ReleaseCoordinator,
    SecurityReviewer,
    GovernanceObserver,
}

impl ApprovalRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayOperator => "replay_operator",
            Self::DashboardOwner => "dashboard_owner",
            Self::ReleaseCoordinator => "release_coordinator",
            Self::SecurityReviewer => "security_reviewer",
            Self::GovernanceObserver => "governance_observer",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Approved,
    Watch,
    Rejected,
    Revoked,
}

impl ApprovalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Watch => "watch",
            Self::Rejected => "rejected",
            Self::Revoked => "revoked",
        }
    }

    pub fn counts_for_quorum(self) -> bool {
        matches!(self, Self::Approved)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingReceipt,
    RejectedReceipt,
    ReplayRootMismatch,
    ReplayRootMissing,
    MissingDashboardApproval,
    MissingReviewerQuorum,
    DashboardExpired,
    MissingRunbookRoot,
    MissingReleasePolicyRoot,
    OpenManualBlocker,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReceipt => "missing_receipt",
            Self::RejectedReceipt => "rejected_receipt",
            Self::ReplayRootMismatch => "replay_root_mismatch",
            Self::ReplayRootMissing => "replay_root_missing",
            Self::MissingDashboardApproval => "missing_dashboard_approval",
            Self::MissingReviewerQuorum => "missing_reviewer_quorum",
            Self::DashboardExpired => "dashboard_expired",
            Self::MissingRunbookRoot => "missing_runbook_root",
            Self::MissingReleasePolicyRoot => "missing_release_policy_root",
            Self::OpenManualBlocker => "open_manual_blocker",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerSeverity {
    Watch,
    Blocking,
}

impl BlockerSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Watch => "watch",
            Self::Blocking => "blocking",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Blocking)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyVerdict {
    Go,
    Watch,
    NoGo,
}

impl ReleasePolicyVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::Watch => "watch",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub release_policy_binding_suite: String,
    pub min_command_receipts: u16,
    pub min_dashboard_approvals: u16,
    pub min_reviewer_quorum: u16,
    pub max_dashboard_age_blocks: u64,
    pub max_watch_blockers: u16,
    pub require_all_commands: bool,
    pub require_root_match: bool,
    pub require_runbook_root: bool,
    pub require_release_policy_root: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            release_policy_binding_suite: RELEASE_POLICY_BINDING_SUITE.to_string(),
            min_command_receipts: DEFAULT_MIN_COMMAND_RECEIPTS,
            min_dashboard_approvals: DEFAULT_MIN_DASHBOARD_APPROVALS,
            min_reviewer_quorum: DEFAULT_MIN_REVIEWER_QUORUM,
            max_dashboard_age_blocks: DEFAULT_MAX_DASHBOARD_AGE_BLOCKS,
            max_watch_blockers: DEFAULT_MAX_WATCH_BLOCKERS,
            require_all_commands: true,
            require_root_match: true,
            require_runbook_root: true,
            require_release_policy_root: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure(
            self.min_command_receipts > 0,
            "minimum command receipts must be non-zero",
        )?;
        ensure(
            self.min_dashboard_approvals > 0,
            "minimum dashboard approvals must be non-zero",
        )?;
        ensure(
            self.min_reviewer_quorum > 0,
            "minimum reviewer quorum must be non-zero",
        )?;
        ensure(
            self.max_dashboard_age_blocks > 0,
            "dashboard age window must be non-zero",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "release_policy_binding_suite": self.release_policy_binding_suite,
            "min_command_receipts": self.min_command_receipts,
            "min_dashboard_approvals": self.min_dashboard_approvals,
            "min_reviewer_quorum": self.min_reviewer_quorum,
            "max_dashboard_age_blocks": self.max_dashboard_age_blocks,
            "max_watch_blockers": self.max_watch_blockers,
            "require_all_commands": self.require_all_commands,
            "require_root_match": self.require_root_match,
            "require_runbook_root": self.require_runbook_root,
            "require_release_policy_root": self.require_release_policy_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_policy_binding_config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayCommandReceipt {
    pub receipt_id: String,
    pub command: ReplayCommandKind,
    pub status: ReceiptStatus,
    pub lane: String,
    pub operator_id: String,
    pub command_root: String,
    pub input_root: String,
    pub output_root: String,
    pub accepted_live_evidence_root: String,
    pub runbook_step_root: String,
    pub executed_at_height: u64,
    pub dashboard_cell: String,
}

impl ReplayCommandReceipt {
    pub fn new(input: ReplayCommandReceiptInput) -> Result<Self> {
        ensure_non_empty("lane", &input.lane)?;
        ensure_non_empty("operator_id", &input.operator_id)?;
        ensure_root("input_root", &input.input_root)?;
        ensure_root("output_root", &input.output_root)?;
        ensure_root(
            "accepted_live_evidence_root",
            &input.accepted_live_evidence_root,
        )?;
        ensure_root("runbook_step_root", &input.runbook_step_root)?;
        ensure_non_empty("dashboard_cell", &input.dashboard_cell)?;
        let command_root = command_root(
            input.command,
            &input.lane,
            &input.operator_id,
            &input.input_root,
            &input.output_root,
            &input.accepted_live_evidence_root,
            input.executed_at_height,
        );
        let receipt_id = receipt_id(
            input.command,
            input.status,
            &input.lane,
            &command_root,
            input.executed_at_height,
        );
        Ok(Self {
            receipt_id,
            command: input.command,
            status: input.status,
            lane: input.lane,
            operator_id: input.operator_id,
            command_root,
            input_root: input.input_root,
            output_root: input.output_root,
            accepted_live_evidence_root: input.accepted_live_evidence_root,
            runbook_step_root: input.runbook_step_root,
            executed_at_height: input.executed_at_height,
            dashboard_cell: input.dashboard_cell,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "command": self.command.as_str(),
            "status": self.status.as_str(),
            "lane": self.lane,
            "operator_id": self.operator_id,
            "command_root": self.command_root,
            "input_root": self.input_root,
            "output_root": self.output_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "runbook_step_root": self.runbook_step_root,
            "executed_at_height": self.executed_at_height,
            "dashboard_cell": self.dashboard_cell,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay_command_receipt", &self.public_record())
    }

    pub fn release_usable(&self) -> bool {
        self.status.release_usable()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayCommandReceiptInput {
    pub command: ReplayCommandKind,
    pub status: ReceiptStatus,
    pub lane: String,
    pub operator_id: String,
    pub input_root: String,
    pub output_root: String,
    pub accepted_live_evidence_root: String,
    pub runbook_step_root: String,
    pub executed_at_height: u64,
    pub dashboard_cell: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayRootComparison {
    pub comparison_id: String,
    pub segment: String,
    pub expected_root: String,
    pub observed_root: String,
    pub command_receipt_root: String,
    pub status: ReplayRootStatus,
    pub compared_at_height: u64,
}

impl ReplayRootComparison {
    pub fn new(
        segment: impl Into<String>,
        expected_root: impl Into<String>,
        observed_root: impl Into<String>,
        command_receipt_root: impl Into<String>,
        compared_at_height: u64,
    ) -> Result<Self> {
        let segment = segment.into();
        let expected_root = expected_root.into();
        let observed_root = observed_root.into();
        let command_receipt_root = command_receipt_root.into();
        ensure_non_empty("segment", &segment)?;
        ensure_root("expected_root", &expected_root)?;
        ensure_root("observed_root", &observed_root)?;
        ensure_root("command_receipt_root", &command_receipt_root)?;
        let status = if expected_root == observed_root {
            ReplayRootStatus::Matched
        } else if observed_root.is_empty() {
            ReplayRootStatus::Missing
        } else {
            ReplayRootStatus::Mismatched
        };
        let comparison_id = replay_root_comparison_id(
            &segment,
            &expected_root,
            &observed_root,
            &command_receipt_root,
            compared_at_height,
        );
        Ok(Self {
            comparison_id,
            segment,
            expected_root,
            observed_root,
            command_receipt_root,
            status,
            compared_at_height,
        })
    }

    pub fn watch(
        segment: impl Into<String>,
        expected_root: impl Into<String>,
        observed_root: impl Into<String>,
        command_receipt_root: impl Into<String>,
        compared_at_height: u64,
    ) -> Result<Self> {
        let mut comparison = Self::new(
            segment,
            expected_root,
            observed_root,
            command_receipt_root,
            compared_at_height,
        )?;
        comparison.status = ReplayRootStatus::Watch;
        comparison.comparison_id = replay_root_comparison_id(
            &comparison.segment,
            &comparison.expected_root,
            &comparison.observed_root,
            &comparison.command_receipt_root,
            comparison.compared_at_height,
        );
        Ok(comparison)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "comparison_id": self.comparison_id,
            "segment": self.segment,
            "expected_root": self.expected_root,
            "observed_root": self.observed_root,
            "command_receipt_root": self.command_receipt_root,
            "status": self.status.as_str(),
            "compared_at_height": self.compared_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay_root_comparison", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardApproval {
    pub approval_id: String,
    pub approver_id: String,
    pub role: ApprovalRole,
    pub status: ApprovalStatus,
    pub dashboard_root: String,
    pub runbook_audit_root: String,
    pub release_policy_root: String,
    pub signed_statement_root: String,
    pub approved_at_height: u64,
}

impl DashboardApproval {
    pub fn new(
        approver_id: impl Into<String>,
        role: ApprovalRole,
        status: ApprovalStatus,
        dashboard_root: impl Into<String>,
        runbook_audit_root: impl Into<String>,
        release_policy_root: impl Into<String>,
        approved_at_height: u64,
    ) -> Result<Self> {
        let approver_id = approver_id.into();
        let dashboard_root = dashboard_root.into();
        let runbook_audit_root = runbook_audit_root.into();
        let release_policy_root = release_policy_root.into();
        ensure_non_empty("approver_id", &approver_id)?;
        ensure_root("dashboard_root", &dashboard_root)?;
        ensure_root("runbook_audit_root", &runbook_audit_root)?;
        ensure_root("release_policy_root", &release_policy_root)?;
        let signed_statement_root = signed_statement_root(
            &approver_id,
            role,
            status,
            &dashboard_root,
            &runbook_audit_root,
            &release_policy_root,
            approved_at_height,
        );
        let approval_id = dashboard_approval_id(
            &approver_id,
            role,
            status,
            &signed_statement_root,
            approved_at_height,
        );
        Ok(Self {
            approval_id,
            approver_id,
            role,
            status,
            dashboard_root,
            runbook_audit_root,
            release_policy_root,
            signed_statement_root,
            approved_at_height,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "approval_id": self.approval_id,
            "approver_id": self.approver_id,
            "role": self.role.as_str(),
            "status": self.status.as_str(),
            "dashboard_root": self.dashboard_root,
            "runbook_audit_root": self.runbook_audit_root,
            "release_policy_root": self.release_policy_root,
            "signed_statement_root": self.signed_statement_root,
            "approved_at_height": self.approved_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("dashboard_approval", &self.public_record())
    }

    pub fn counts_for_quorum(&self) -> bool {
        self.status.counts_for_quorum()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReviewerQuorum {
    pub quorum_id: String,
    pub required: u16,
    pub observed: u16,
    pub reviewer_root: String,
    pub role_root: String,
    pub satisfied: bool,
}

impl ReviewerQuorum {
    pub fn new(required: u16, approvals: &BTreeMap<String, DashboardApproval>) -> Self {
        let approved = approvals
            .values()
            .filter(|approval| approval.counts_for_quorum())
            .collect::<Vec<_>>();
        let observed = approved.len() as u16;
        let reviewer_root = merkle_root(
            "RELEASE-POLICY-BINDING-REVIEWER-QUORUM-REVIEWERS",
            &approved
                .iter()
                .map(|approval| approval.state_root())
                .collect::<Vec<_>>(),
        );
        let role_root = merkle_root(
            "RELEASE-POLICY-BINDING-REVIEWER-QUORUM-ROLES",
            &approved
                .iter()
                .map(|approval| role_leaf(approval.role, &approval.approver_id))
                .collect::<Vec<_>>(),
        );
        let quorum_id = reviewer_quorum_id(required, observed, &reviewer_root, &role_root);
        Self {
            quorum_id,
            required,
            observed,
            reviewer_root,
            role_root,
            satisfied: observed >= required,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "quorum_id": self.quorum_id,
            "required": self.required,
            "observed": self.observed,
            "reviewer_root": self.reviewer_root,
            "role_root": self.role_root,
            "satisfied": self.satisfied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("reviewer_quorum", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub severity: BlockerSeverity,
    pub lane: String,
    pub evidence_root: String,
    pub detail: String,
    pub observed_at_height: u64,
}

impl ReleaseBlocker {
    pub fn new(
        kind: BlockerKind,
        severity: BlockerSeverity,
        lane: impl Into<String>,
        evidence_root: impl Into<String>,
        detail: impl Into<String>,
        observed_at_height: u64,
    ) -> Result<Self> {
        let lane = lane.into();
        let evidence_root = evidence_root.into();
        let detail = detail.into();
        ensure_non_empty("lane", &lane)?;
        ensure_root("evidence_root", &evidence_root)?;
        ensure_non_empty("detail", &detail)?;
        let blocker_id = blocker_id(kind, severity, &lane, &evidence_root, observed_at_height);
        Ok(Self {
            blocker_id,
            kind,
            severity,
            lane,
            evidence_root,
            detail,
            observed_at_height,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "severity": self.severity.as_str(),
            "lane": self.lane,
            "evidence_root": self.evidence_root,
            "detail": self.detail,
            "observed_at_height": self.observed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_blocker", &self.public_record())
    }

    pub fn blocks_release(&self) -> bool {
        self.severity.blocks_release()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleasePolicyBinding {
    pub binding_id: String,
    pub verdict: ReleasePolicyVerdict,
    pub fail_closed: bool,
    pub command_receipt_root: String,
    pub replay_root_comparison_root: String,
    pub dashboard_approval_root: String,
    pub reviewer_quorum_root: String,
    pub blocker_root: String,
    pub go_no_go_evidence_root: String,
    pub bound_at_height: u64,
}

impl ReleasePolicyBinding {
    pub fn public_record(&self) -> Value {
        json!({
            "binding_id": self.binding_id,
            "verdict": self.verdict.as_str(),
            "fail_closed": self.fail_closed,
            "command_receipt_root": self.command_receipt_root,
            "replay_root_comparison_root": self.replay_root_comparison_root,
            "dashboard_approval_root": self.dashboard_approval_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "blocker_root": self.blocker_root,
            "go_no_go_evidence_root": self.go_no_go_evidence_root,
            "bound_at_height": self.bound_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_policy_binding", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Counters {
    pub command_receipts: u16,
    pub usable_command_receipts: u16,
    pub replay_root_comparisons: u16,
    pub matched_replay_roots: u16,
    pub dashboard_approvals: u16,
    pub approved_dashboard_approvals: u16,
    pub reviewer_quorum_observed: u16,
    pub watch_blockers: u16,
    pub blocking_blockers: u16,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "command_receipts": self.command_receipts,
            "usable_command_receipts": self.usable_command_receipts,
            "replay_root_comparisons": self.replay_root_comparisons,
            "matched_replay_roots": self.matched_replay_roots,
            "dashboard_approvals": self.dashboard_approvals,
            "approved_dashboard_approvals": self.approved_dashboard_approvals,
            "reviewer_quorum_observed": self.reviewer_quorum_observed,
            "watch_blockers": self.watch_blockers,
            "blocking_blockers": self.blocking_blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_policy_binding_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub accepted_live_evidence_root: String,
    pub operator_runbook_audit_root: String,
    pub operator_dashboard_root: String,
    pub release_policy_root: String,
    pub command_receipts: BTreeMap<String, ReplayCommandReceipt>,
    pub replay_root_comparisons: BTreeMap<String, ReplayRootComparison>,
    pub dashboard_approvals: BTreeMap<String, DashboardApproval>,
    pub manual_blockers: BTreeMap<String, ReleaseBlocker>,
}

impl State {
    pub fn new(config: Config, height: u64) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config,
            height,
            accepted_live_evidence_root: String::new(),
            operator_runbook_audit_root: String::new(),
            operator_dashboard_root: String::new(),
            release_policy_root: String::new(),
            command_receipts: BTreeMap::new(),
            replay_root_comparisons: BTreeMap::new(),
            dashboard_approvals: BTreeMap::new(),
            manual_blockers: BTreeMap::new(),
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut state = match Self::new(config, DEFAULT_HEIGHT) {
            Ok(state) => state,
            Err(_) => Self {
                config: Config::devnet(),
                height: DEFAULT_HEIGHT,
                accepted_live_evidence_root: String::new(),
                operator_runbook_audit_root: String::new(),
                operator_dashboard_root: String::new(),
                release_policy_root: String::new(),
                command_receipts: BTreeMap::new(),
                replay_root_comparisons: BTreeMap::new(),
                dashboard_approvals: BTreeMap::new(),
                manual_blockers: BTreeMap::new(),
            },
        };
        state.accepted_live_evidence_root = sample_root("wave-82-accepted-live-evidence");
        state.operator_runbook_audit_root = sample_root("wave-82-operator-runbook-audit");
        state.operator_dashboard_root = sample_root("wave-82-operator-dashboard");
        state.release_policy_root = sample_root("wave-83-release-policy-root");
        for (index, command) in ReplayCommandKind::required().iter().enumerate() {
            let command_label = command.as_str();
            let input_root = sample_root(&format!("{command_label}-input"));
            let output_root = sample_root(&format!("{command_label}-output"));
            let runbook_step_root = sample_root(&format!("{command_label}-runbook-step"));
            if let Ok(receipt) = ReplayCommandReceipt::new(ReplayCommandReceiptInput {
                command: *command,
                status: ReceiptStatus::Replayed,
                lane: "runtime_replay".to_string(),
                operator_id: format!("runtime-replay-operator-{}", index + 1),
                input_root,
                output_root,
                accepted_live_evidence_root: state.accepted_live_evidence_root.clone(),
                runbook_step_root,
                executed_at_height: DEFAULT_HEIGHT
                    .saturating_sub(16)
                    .saturating_add(index as u64),
                dashboard_cell: format!("runtime_replay:{command_label}"),
            }) {
                let _ = state.add_command_receipt(receipt);
            }
        }
        let receipt_root = state.command_receipt_root();
        for index in 0..DEFAULT_REPLAY_ROOT_COUNT {
            let expected_root = sample_root(&format!("expected-replay-root-{index}"));
            if let Ok(comparison) = ReplayRootComparison::new(
                format!("runtime-replay-segment-{index}"),
                expected_root.clone(),
                expected_root,
                receipt_root.clone(),
                DEFAULT_HEIGHT
                    .saturating_sub(4)
                    .saturating_add(u64::from(index)),
            ) {
                let _ = state.add_replay_root_comparison(comparison);
            }
        }
        let approval_specs = [
            ("runtime-replay-lead", ApprovalRole::ReplayOperator),
            ("dashboard-owner", ApprovalRole::DashboardOwner),
            ("release-coordinator-a", ApprovalRole::ReleaseCoordinator),
            ("security-reviewer-a", ApprovalRole::SecurityReviewer),
            ("governance-observer-a", ApprovalRole::GovernanceObserver),
        ];
        for (index, (approver, role)) in approval_specs.iter().enumerate() {
            if let Ok(approval) = DashboardApproval::new(
                *approver,
                *role,
                ApprovalStatus::Approved,
                state.operator_dashboard_root.clone(),
                state.operator_runbook_audit_root.clone(),
                state.release_policy_root.clone(),
                DEFAULT_HEIGHT
                    .saturating_sub(2)
                    .saturating_add(index as u64),
            ) {
                let _ = state.add_dashboard_approval(approval);
            }
        }
        state
    }

    pub fn bind_accepted_live_evidence_root(&mut self, root: impl Into<String>) -> Result<()> {
        let root = root.into();
        ensure_root("accepted_live_evidence_root", &root)?;
        self.accepted_live_evidence_root = root;
        Ok(())
    }

    pub fn bind_operator_runbook_audit_root(&mut self, root: impl Into<String>) -> Result<()> {
        let root = root.into();
        ensure_root("operator_runbook_audit_root", &root)?;
        self.operator_runbook_audit_root = root;
        Ok(())
    }

    pub fn bind_operator_dashboard_root(&mut self, root: impl Into<String>) -> Result<()> {
        let root = root.into();
        ensure_root("operator_dashboard_root", &root)?;
        self.operator_dashboard_root = root;
        Ok(())
    }

    pub fn bind_release_policy_root(&mut self, root: impl Into<String>) -> Result<()> {
        let root = root.into();
        ensure_root("release_policy_root", &root)?;
        self.release_policy_root = root;
        Ok(())
    }

    pub fn add_command_receipt(&mut self, receipt: ReplayCommandReceipt) -> Result<()> {
        ensure_non_empty("receipt_id", &receipt.receipt_id)?;
        ensure(
            receipt.accepted_live_evidence_root == self.accepted_live_evidence_root,
            "command receipt must bind the accepted live evidence root",
        )?;
        self.command_receipts
            .insert(receipt.receipt_id.clone(), receipt);
        Ok(())
    }

    pub fn add_replay_root_comparison(&mut self, comparison: ReplayRootComparison) -> Result<()> {
        ensure_non_empty("comparison_id", &comparison.comparison_id)?;
        ensure_root("expected_root", &comparison.expected_root)?;
        ensure_root("observed_root", &comparison.observed_root)?;
        self.replay_root_comparisons
            .insert(comparison.comparison_id.clone(), comparison);
        Ok(())
    }

    pub fn add_dashboard_approval(&mut self, approval: DashboardApproval) -> Result<()> {
        ensure_non_empty("approval_id", &approval.approval_id)?;
        ensure(
            approval.dashboard_root == self.operator_dashboard_root,
            "approval dashboard root must match the bound operator dashboard root",
        )?;
        ensure(
            approval.runbook_audit_root == self.operator_runbook_audit_root,
            "approval runbook root must match the bound operator runbook audit root",
        )?;
        ensure(
            approval.release_policy_root == self.release_policy_root,
            "approval release policy root must match the bound release policy root",
        )?;
        self.dashboard_approvals
            .insert(approval.approval_id.clone(), approval);
        Ok(())
    }

    pub fn add_manual_blocker(&mut self, blocker: ReleaseBlocker) -> Result<()> {
        ensure_non_empty("blocker_id", &blocker.blocker_id)?;
        self.manual_blockers
            .insert(blocker.blocker_id.clone(), blocker);
        Ok(())
    }

    pub fn command_receipt_root(&self) -> String {
        map_root(
            "release_policy_binding_command_receipts",
            self.command_receipts
                .values()
                .map(ReplayCommandReceipt::state_root),
        )
    }

    pub fn replay_root_comparison_root(&self) -> String {
        map_root(
            "release_policy_binding_replay_root_comparisons",
            self.replay_root_comparisons
                .values()
                .map(ReplayRootComparison::state_root),
        )
    }

    pub fn dashboard_approval_root(&self) -> String {
        map_root(
            "release_policy_binding_dashboard_approvals",
            self.dashboard_approvals
                .values()
                .map(DashboardApproval::state_root),
        )
    }

    pub fn manual_blocker_root(&self) -> String {
        map_root(
            "release_policy_binding_manual_blockers",
            self.manual_blockers
                .values()
                .map(ReleaseBlocker::state_root),
        )
    }

    pub fn reviewer_quorum(&self) -> ReviewerQuorum {
        ReviewerQuorum::new(self.config.min_reviewer_quorum, &self.dashboard_approvals)
    }

    pub fn counters(&self) -> Counters {
        let blockers = self.blockers();
        Counters {
            command_receipts: self.command_receipts.len() as u16,
            usable_command_receipts: self
                .command_receipts
                .values()
                .filter(|receipt| receipt.release_usable())
                .count() as u16,
            replay_root_comparisons: self.replay_root_comparisons.len() as u16,
            matched_replay_roots: self
                .replay_root_comparisons
                .values()
                .filter(|comparison| comparison.status == ReplayRootStatus::Matched)
                .count() as u16,
            dashboard_approvals: self.dashboard_approvals.len() as u16,
            approved_dashboard_approvals: self
                .dashboard_approvals
                .values()
                .filter(|approval| approval.counts_for_quorum())
                .count() as u16,
            reviewer_quorum_observed: self.reviewer_quorum().observed,
            watch_blockers: blockers
                .iter()
                .filter(|blocker| blocker.severity == BlockerSeverity::Watch)
                .count() as u16,
            blocking_blockers: blockers
                .iter()
                .filter(|blocker| blocker.blocks_release())
                .count() as u16,
        }
    }

    pub fn blockers(&self) -> Vec<ReleaseBlocker> {
        let mut blockers = Vec::new();
        self.push_receipt_blockers(&mut blockers);
        self.push_replay_root_blockers(&mut blockers);
        self.push_dashboard_blockers(&mut blockers);
        for blocker in self.manual_blockers.values() {
            blockers.push(blocker.clone());
        }
        blockers
    }

    pub fn blocker_root(&self) -> String {
        map_root(
            "release_policy_binding_all_blockers",
            self.blockers().iter().map(ReleaseBlocker::state_root),
        )
    }

    pub fn go_no_go_evidence_root(&self) -> String {
        domain_hash(
            "RELEASE-POLICY-BINDING-GO-NO-GO-EVIDENCE-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.accepted_live_evidence_root),
                HashPart::Str(&self.operator_runbook_audit_root),
                HashPart::Str(&self.operator_dashboard_root),
                HashPart::Str(&self.release_policy_root),
                HashPart::Str(&self.command_receipt_root()),
                HashPart::Str(&self.replay_root_comparison_root()),
                HashPart::Str(&self.dashboard_approval_root()),
                HashPart::Str(&self.reviewer_quorum().state_root()),
                HashPart::Str(&self.blocker_root()),
                HashPart::Int(self.height as i128),
            ],
            32,
        )
    }

    pub fn release_policy_binding(&self) -> ReleasePolicyBinding {
        let counters = self.counters();
        let quorum = self.reviewer_quorum();
        let blocker_root = self.blocker_root();
        let verdict = self.verdict(&counters, &quorum);
        let command_receipt_root = self.command_receipt_root();
        let replay_root_comparison_root = self.replay_root_comparison_root();
        let dashboard_approval_root = self.dashboard_approval_root();
        let reviewer_quorum_root = quorum.state_root();
        let go_no_go_evidence_root = self.go_no_go_evidence_root();
        let binding_id = release_policy_binding_id(
            verdict,
            &command_receipt_root,
            &replay_root_comparison_root,
            &dashboard_approval_root,
            &reviewer_quorum_root,
            &blocker_root,
            &go_no_go_evidence_root,
            self.height,
        );
        ReleasePolicyBinding {
            binding_id,
            verdict,
            fail_closed: self.config.fail_closed,
            command_receipt_root,
            replay_root_comparison_root,
            dashboard_approval_root,
            reviewer_quorum_root,
            blocker_root,
            go_no_go_evidence_root,
            bound_at_height: self.height,
        }
    }

    pub fn public_record(&self) -> Value {
        let blockers = self.blockers();
        let counters = self.counters();
        let quorum = self.reviewer_quorum();
        let binding = self.release_policy_binding();
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "config": self.config.public_record(),
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "operator_runbook_audit_root": self.operator_runbook_audit_root,
            "operator_dashboard_root": self.operator_dashboard_root,
            "release_policy_root": self.release_policy_root,
            "command_receipt_root": self.command_receipt_root(),
            "replay_root_comparison_root": self.replay_root_comparison_root(),
            "dashboard_approval_root": self.dashboard_approval_root(),
            "reviewer_quorum": quorum.public_record(),
            "manual_blocker_root": self.manual_blocker_root(),
            "blocker_root": binding.blocker_root,
            "go_no_go_evidence_root": binding.go_no_go_evidence_root,
            "release_policy_binding": binding.public_record(),
            "counters": counters.public_record(),
            "command_receipts": self.command_receipts.values().map(ReplayCommandReceipt::public_record).collect::<Vec<_>>(),
            "replay_root_comparisons": self.replay_root_comparisons.values().map(ReplayRootComparison::public_record).collect::<Vec<_>>(),
            "dashboard_approvals": self.dashboard_approvals.values().map(DashboardApproval::public_record).collect::<Vec<_>>(),
            "blockers": blockers.iter().map(ReleaseBlocker::public_record).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_policy_binding_state", &self.public_record())
    }

    fn verdict(&self, counters: &Counters, quorum: &ReviewerQuorum) -> ReleasePolicyVerdict {
        if self.config.fail_closed && counters.blocking_blockers > 0 {
            return ReleasePolicyVerdict::NoGo;
        }
        if self.config.fail_closed && !quorum.satisfied {
            return ReleasePolicyVerdict::NoGo;
        }
        if self.config.fail_closed
            && counters.usable_command_receipts < self.config.min_command_receipts
        {
            return ReleasePolicyVerdict::NoGo;
        }
        if self.config.require_root_match
            && counters.matched_replay_roots < counters.replay_root_comparisons
        {
            return ReleasePolicyVerdict::NoGo;
        }
        if counters.watch_blockers > self.config.max_watch_blockers {
            return ReleasePolicyVerdict::Watch;
        }
        if counters.approved_dashboard_approvals < self.config.min_dashboard_approvals {
            return ReleasePolicyVerdict::Watch;
        }
        ReleasePolicyVerdict::Go
    }

    fn push_receipt_blockers(&self, blockers: &mut Vec<ReleaseBlocker>) {
        let seen: BTreeSet<ReplayCommandKind> = self
            .command_receipts
            .values()
            .filter(|receipt| receipt.release_usable())
            .map(|receipt| receipt.command)
            .collect();
        if self.config.require_all_commands {
            for command in ReplayCommandKind::required() {
                if !seen.contains(&command) {
                    push_blocker(
                        blockers,
                        BlockerKind::MissingReceipt,
                        BlockerSeverity::Blocking,
                        "runtime_replay",
                        &missing_command_root(command),
                        format!(
                            "required replay command receipt is absent: {}",
                            command.as_str()
                        ),
                        self.height,
                    );
                }
            }
        }
        for receipt in self.command_receipts.values() {
            if !receipt.release_usable() {
                push_blocker(
                    blockers,
                    BlockerKind::RejectedReceipt,
                    BlockerSeverity::Blocking,
                    &receipt.lane,
                    &receipt.state_root(),
                    format!(
                        "replay command receipt is not release usable: {}",
                        receipt.status.as_str()
                    ),
                    self.height,
                );
            }
        }
        let usable = self
            .command_receipts
            .values()
            .filter(|receipt| receipt.release_usable())
            .count() as u16;
        if usable < self.config.min_command_receipts {
            push_blocker(
                blockers,
                BlockerKind::MissingReceipt,
                BlockerSeverity::Blocking,
                "runtime_replay",
                &self.command_receipt_root(),
                format!(
                    "usable command receipt count {} is below required {}",
                    usable, self.config.min_command_receipts
                ),
                self.height,
            );
        }
    }

    fn push_replay_root_blockers(&self, blockers: &mut Vec<ReleaseBlocker>) {
        if self.replay_root_comparisons.is_empty() {
            push_blocker(
                blockers,
                BlockerKind::ReplayRootMissing,
                BlockerSeverity::Blocking,
                "runtime_replay",
                &self.command_receipt_root(),
                "no observed-vs-expected replay root comparisons are bound",
                self.height,
            );
        }
        for comparison in self.replay_root_comparisons.values() {
            if comparison.status == ReplayRootStatus::Watch {
                push_blocker(
                    blockers,
                    BlockerKind::ReplayRootMismatch,
                    BlockerSeverity::Watch,
                    "runtime_replay",
                    &comparison.state_root(),
                    format!(
                        "replay root comparison is watch-listed: {}",
                        comparison.segment
                    ),
                    self.height,
                );
            }
            if comparison.status.blocks_release() {
                let kind = if comparison.status == ReplayRootStatus::Missing {
                    BlockerKind::ReplayRootMissing
                } else {
                    BlockerKind::ReplayRootMismatch
                };
                push_blocker(
                    blockers,
                    kind,
                    BlockerSeverity::Blocking,
                    "runtime_replay",
                    &comparison.state_root(),
                    format!(
                        "expected root does not match observed root for segment {}",
                        comparison.segment
                    ),
                    self.height,
                );
            }
        }
    }

    fn push_dashboard_blockers(&self, blockers: &mut Vec<ReleaseBlocker>) {
        if self.config.require_runbook_root && self.operator_runbook_audit_root.is_empty() {
            push_blocker(
                blockers,
                BlockerKind::MissingRunbookRoot,
                BlockerSeverity::Blocking,
                "operator_dashboard",
                &self.config.state_root(),
                "operator runbook audit root is not bound",
                self.height,
            );
        }
        if self.config.require_release_policy_root && self.release_policy_root.is_empty() {
            push_blocker(
                blockers,
                BlockerKind::MissingReleasePolicyRoot,
                BlockerSeverity::Blocking,
                "release_policy",
                &self.config.state_root(),
                "release policy root is not bound",
                self.height,
            );
        }
        let approved = self
            .dashboard_approvals
            .values()
            .filter(|approval| approval.counts_for_quorum())
            .count() as u16;
        if approved < self.config.min_dashboard_approvals {
            push_blocker(
                blockers,
                BlockerKind::MissingDashboardApproval,
                BlockerSeverity::Blocking,
                "operator_dashboard",
                &self.dashboard_approval_root(),
                format!(
                    "approved dashboard approvals {} below required {}",
                    approved, self.config.min_dashboard_approvals
                ),
                self.height,
            );
        }
        let quorum = self.reviewer_quorum();
        if !quorum.satisfied {
            push_blocker(
                blockers,
                BlockerKind::MissingReviewerQuorum,
                BlockerSeverity::Blocking,
                "operator_dashboard",
                &quorum.state_root(),
                format!(
                    "reviewer quorum observed {} below required {}",
                    quorum.observed, quorum.required
                ),
                self.height,
            );
        }
        for approval in self.dashboard_approvals.values() {
            if approval
                .approved_at_height
                .saturating_add(self.config.max_dashboard_age_blocks)
                < self.height
            {
                push_blocker(
                    blockers,
                    BlockerKind::DashboardExpired,
                    BlockerSeverity::Blocking,
                    "operator_dashboard",
                    &approval.state_root(),
                    format!(
                        "dashboard approval from {} is outside the policy age window",
                        approval.approver_id
                    ),
                    self.height,
                );
            }
        }
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

pub fn release_policy_binding() -> ReleasePolicyBinding {
    devnet().release_policy_binding()
}

fn command_root(
    command: ReplayCommandKind,
    lane: &str,
    operator_id: &str,
    input_root: &str,
    output_root: &str,
    accepted_live_evidence_root: &str,
    executed_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-REPLAY-COMMAND-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(command.as_str()),
            HashPart::Str(lane),
            HashPart::Str(operator_id),
            HashPart::Str(input_root),
            HashPart::Str(output_root),
            HashPart::Str(accepted_live_evidence_root),
            HashPart::Int(executed_at_height as i128),
        ],
        32,
    )
}

fn receipt_id(
    command: ReplayCommandKind,
    status: ReceiptStatus,
    lane: &str,
    command_root: &str,
    executed_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-REPLAY-COMMAND-RECEIPT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(command.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(lane),
            HashPart::Str(command_root),
            HashPart::Int(executed_at_height as i128),
        ],
        32,
    )
}

fn replay_root_comparison_id(
    segment: &str,
    expected_root: &str,
    observed_root: &str,
    command_receipt_root: &str,
    compared_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-REPLAY-ROOT-COMPARISON-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(segment),
            HashPart::Str(expected_root),
            HashPart::Str(observed_root),
            HashPart::Str(command_receipt_root),
            HashPart::Int(compared_at_height as i128),
        ],
        32,
    )
}

fn signed_statement_root(
    approver_id: &str,
    role: ApprovalRole,
    status: ApprovalStatus,
    dashboard_root: &str,
    runbook_audit_root: &str,
    release_policy_root: &str,
    approved_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-DASHBOARD-APPROVAL-SIGNED-STATEMENT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(approver_id),
            HashPart::Str(role.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(dashboard_root),
            HashPart::Str(runbook_audit_root),
            HashPart::Str(release_policy_root),
            HashPart::Int(approved_at_height as i128),
        ],
        32,
    )
}

fn dashboard_approval_id(
    approver_id: &str,
    role: ApprovalRole,
    status: ApprovalStatus,
    signed_statement_root: &str,
    approved_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-DASHBOARD-APPROVAL-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(approver_id),
            HashPart::Str(role.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(signed_statement_root),
            HashPart::Int(approved_at_height as i128),
        ],
        32,
    )
}

fn reviewer_quorum_id(
    required: u16,
    observed: u16,
    reviewer_root: &str,
    role_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-REVIEWER-QUORUM-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Int(i128::from(required)),
            HashPart::Int(i128::from(observed)),
            HashPart::Str(reviewer_root),
            HashPart::Str(role_root),
        ],
        32,
    )
}

fn role_leaf(role: ApprovalRole, approver_id: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-REVIEWER-ROLE-LEAF",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(role.as_str()),
            HashPart::Str(approver_id),
        ],
        32,
    )
}

fn blocker_id(
    kind: BlockerKind,
    severity: BlockerSeverity,
    lane: &str,
    evidence_root: &str,
    observed_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-BLOCKER-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(severity.as_str()),
            HashPart::Str(lane),
            HashPart::Str(evidence_root),
            HashPart::Int(observed_at_height as i128),
        ],
        32,
    )
}

fn release_policy_binding_id(
    verdict: ReleasePolicyVerdict,
    command_receipt_root: &str,
    replay_root_comparison_root: &str,
    dashboard_approval_root: &str,
    reviewer_quorum_root: &str,
    blocker_root: &str,
    go_no_go_evidence_root: &str,
    bound_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(verdict.as_str()),
            HashPart::Str(command_receipt_root),
            HashPart::Str(replay_root_comparison_root),
            HashPart::Str(dashboard_approval_root),
            HashPart::Str(reviewer_quorum_root),
            HashPart::Str(blocker_root),
            HashPart::Str(go_no_go_evidence_root),
            HashPart::Int(bound_at_height as i128),
        ],
        32,
    )
}

fn missing_command_root(command: ReplayCommandKind) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-MISSING-REPLAY-COMMAND",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(command.as_str()),
        ],
        32,
    )
}

fn sample_root(label: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-DEVNET-SAMPLE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "RELEASE-POLICY-BINDING-RECORD-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn map_root<I>(domain: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn push_blocker(
    blockers: &mut Vec<ReleaseBlocker>,
    kind: BlockerKind,
    severity: BlockerSeverity,
    lane: impl Into<String>,
    evidence_root: &str,
    detail: impl Into<String>,
    observed_at_height: u64,
) {
    if let Ok(blocker) = ReleaseBlocker::new(
        kind,
        severity,
        lane,
        evidence_root.to_string(),
        detail,
        observed_at_height,
    ) {
        blockers.push(blocker);
    }
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{label} must be non-empty"),
    )
}

fn ensure_root(label: &str, value: &str) -> Result<()> {
    ensure_non_empty(label, value)?;
    ensure(value.len() >= 32, &format!("{label} must be root-like"))
}
