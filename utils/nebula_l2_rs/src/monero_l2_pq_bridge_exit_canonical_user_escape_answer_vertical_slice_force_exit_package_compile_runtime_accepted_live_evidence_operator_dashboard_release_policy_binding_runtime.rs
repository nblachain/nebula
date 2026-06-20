use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileRuntimeAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-compile-runtime-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const BINDING_SUITE: &str =
    "monero-l2-pq-force-exit-compile-runtime-dashboard-release-policy-binding-v1";
pub const DEFAULT_RELEASE_EPOCH: u64 = 83;
pub const DEFAULT_RELEASE_HEIGHT: u64 = 830_000;
pub const DEFAULT_MAX_RECEIPT_AGE_BLOCKS: u64 = 36;
pub const DEFAULT_MIN_REVIEWERS: u16 = 3;
pub const DEFAULT_MIN_OPERATOR_APPROVALS: u16 = 3;
pub const DEFAULT_MIN_COORDINATOR_APPROVALS: u16 = 2;
pub const DEFAULT_MIN_LANE_RECORDS: u16 = 2;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub binding_suite: String,
    pub release_epoch: u64,
    pub release_height: u64,
    pub release_channel: String,
    pub max_receipt_age_blocks: u64,
    pub min_reviewers: u16,
    pub min_operator_approvals: u16,
    pub min_coordinator_approvals: u16,
    pub min_lane_records: u16,
    pub require_compile_receipt: bool,
    pub require_check_receipt: bool,
    pub require_rustfmt_receipt: bool,
    pub require_test_receipt: bool,
    pub require_clippy_receipt: bool,
    pub require_dashboard_approval: bool,
    pub require_reviewer_quorum: bool,
    pub require_no_blockers: bool,
    pub fail_closed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            binding_suite: BINDING_SUITE.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            release_height: DEFAULT_RELEASE_HEIGHT,
            release_channel: "devnet-compile-runtime-release-policy".to_string(),
            max_receipt_age_blocks: DEFAULT_MAX_RECEIPT_AGE_BLOCKS,
            min_reviewers: DEFAULT_MIN_REVIEWERS,
            min_operator_approvals: DEFAULT_MIN_OPERATOR_APPROVALS,
            min_coordinator_approvals: DEFAULT_MIN_COORDINATOR_APPROVALS,
            min_lane_records: DEFAULT_MIN_LANE_RECORDS,
            require_compile_receipt: true,
            require_check_receipt: true,
            require_rustfmt_receipt: true,
            require_test_receipt: true,
            require_clippy_receipt: true,
            require_dashboard_approval: true,
            require_reviewer_quorum: true,
            require_no_blockers: true,
            fail_closed: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("binding_suite", &self.binding_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.release_epoch > 0, "release epoch must be non-zero")?;
        ensure(
            self.release_height > 0,
            "release height must be non-zero for age checks",
        )?;
        ensure(
            self.max_receipt_age_blocks > 0,
            "receipt age window must be non-zero",
        )?;
        ensure(self.min_reviewers > 0, "reviewer quorum must be non-zero")?;
        ensure(
            self.min_operator_approvals > 0,
            "operator approval quorum must be non-zero",
        )?;
        ensure(
            self.min_coordinator_approvals > 0,
            "coordinator approval quorum must be non-zero",
        )?;
        ensure(
            self.min_lane_records > 0,
            "lane record minimum must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "binding_suite": self.binding_suite,
            "release_epoch": self.release_epoch,
            "release_height": self.release_height,
            "release_channel": self.release_channel,
            "max_receipt_age_blocks": self.max_receipt_age_blocks,
            "min_reviewers": self.min_reviewers,
            "min_operator_approvals": self.min_operator_approvals,
            "min_coordinator_approvals": self.min_coordinator_approvals,
            "min_lane_records": self.min_lane_records,
            "require_compile_receipt": self.require_compile_receipt,
            "require_check_receipt": self.require_check_receipt,
            "require_rustfmt_receipt": self.require_rustfmt_receipt,
            "require_test_receipt": self.require_test_receipt,
            "require_clippy_receipt": self.require_clippy_receipt,
            "require_dashboard_approval": self.require_dashboard_approval,
            "require_reviewer_quorum": self.require_reviewer_quorum,
            "require_no_blockers": self.require_no_blockers,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    CompileAccepted,
    RuntimeAccepted,
    LiveEvidence,
    OperatorRunbook,
    OperatorDashboard,
    ReleasePolicy,
}

impl LaneKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileAccepted,
            Self::RuntimeAccepted,
            Self::LiveEvidence,
            Self::OperatorRunbook,
            Self::OperatorDashboard,
            Self::ReleasePolicy,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileAccepted => "compile_accepted",
            Self::RuntimeAccepted => "runtime_accepted",
            Self::LiveEvidence => "live_evidence",
            Self::OperatorRunbook => "operator_runbook",
            Self::OperatorDashboard => "operator_dashboard",
            Self::ReleasePolicy => "release_policy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptGateKind {
    Compile,
    Check,
    Rustfmt,
    Test,
    Clippy,
}

impl ReceiptGateKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Compile,
            Self::Check,
            Self::Rustfmt,
            Self::Test,
            Self::Clippy,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::Check => "check",
            Self::Rustfmt => "rustfmt",
            Self::Test => "test",
            Self::Clippy => "clippy",
        }
    }

    pub fn required_by(self, config: &Config) -> bool {
        match self {
            Self::Compile => config.require_compile_receipt,
            Self::Check => config.require_check_receipt,
            Self::Rustfmt => config.require_rustfmt_receipt,
            Self::Test => config.require_test_receipt,
            Self::Clippy => config.require_clippy_receipt,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Missing,
    Pending,
    Accepted,
    Failed,
    Expired,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Failed => "failed",
            Self::Expired => "expired",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRole {
    Operator,
    Reviewer,
    ReleaseCoordinator,
}

impl ApprovalRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::Reviewer => "reviewer",
            Self::ReleaseCoordinator => "release_coordinator",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Approved,
    Abstained,
    Rejected,
}

impl ApprovalDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Abstained => "abstained",
            Self::Rejected => "rejected",
        }
    }

    pub fn approved(self) -> bool {
        matches!(self, Self::Approved)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingLaneRecord,
    DuplicateLaneRecord,
    MissingReceipt,
    ReceiptPending,
    ReceiptFailed,
    ReceiptExpired,
    MissingDashboardRoot,
    MissingOperatorApproval,
    MissingReviewerQuorum,
    MissingCoordinatorApproval,
    RejectedApproval,
    OpenDashboardAction,
    ReleasePolicyMismatch,
    EvidenceRootMismatch,
    EmptyRoot,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingLaneRecord => "missing_lane_record",
            Self::DuplicateLaneRecord => "duplicate_lane_record",
            Self::MissingReceipt => "missing_receipt",
            Self::ReceiptPending => "receipt_pending",
            Self::ReceiptFailed => "receipt_failed",
            Self::ReceiptExpired => "receipt_expired",
            Self::MissingDashboardRoot => "missing_dashboard_root",
            Self::MissingOperatorApproval => "missing_operator_approval",
            Self::MissingReviewerQuorum => "missing_reviewer_quorum",
            Self::MissingCoordinatorApproval => "missing_coordinator_approval",
            Self::RejectedApproval => "rejected_approval",
            Self::OpenDashboardAction => "open_dashboard_action",
            Self::ReleasePolicyMismatch => "release_policy_mismatch",
            Self::EvidenceRootMismatch => "evidence_root_mismatch",
            Self::EmptyRoot => "empty_root",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictKind {
    Go,
    NoGo,
}

impl VerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneRecord {
    pub lane_id: String,
    pub lane: LaneKind,
    pub ordinal: u64,
    pub source_module: String,
    pub evidence_root: String,
    pub dashboard_root: String,
    pub runbook_root: String,
    pub release_policy_root: String,
    pub accepted_height: u64,
    pub imported: bool,
}

impl LaneRecord {
    pub fn devnet(config: &Config, lane: LaneKind, ordinal: u64) -> Self {
        let evidence_root = sample_root("lane-evidence", lane.as_str(), ordinal);
        let dashboard_root = sample_root("lane-dashboard", lane.as_str(), ordinal);
        let runbook_root = sample_root("lane-runbook", lane.as_str(), ordinal);
        let release_policy_root = release_policy_seed_root(config, lane, ordinal);
        Self {
            lane_id: lane_id(lane, ordinal),
            lane,
            ordinal,
            source_module: format!("wave-82-{}-accepted-live-evidence", lane.as_str()),
            evidence_root,
            dashboard_root,
            runbook_root,
            release_policy_root,
            accepted_height: config.release_height.saturating_sub(ordinal),
            imported: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("lane_id", &self.lane_id)?;
        ensure_non_empty("source_module", &self.source_module)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("dashboard_root", &self.dashboard_root)?;
        ensure_root("runbook_root", &self.runbook_root)?;
        ensure_root("release_policy_root", &self.release_policy_root)?;
        ensure(
            self.accepted_height > 0,
            "lane accepted height must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_id": self.lane_id,
            "lane": self.lane.as_str(),
            "ordinal": self.ordinal,
            "source_module": self.source_module,
            "evidence_root": self.evidence_root,
            "dashboard_root": self.dashboard_root,
            "runbook_root": self.runbook_root,
            "release_policy_root": self.release_policy_root,
            "accepted_height": self.accepted_height,
            "imported": self.imported,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptGate {
    pub gate_id: String,
    pub kind: ReceiptGateKind,
    pub command: String,
    pub receipt_root: String,
    pub artifact_root: String,
    pub log_root: String,
    pub observed_height: u64,
    pub status: ReceiptStatus,
    pub required: bool,
}

impl ReceiptGate {
    pub fn devnet(config: &Config, kind: ReceiptGateKind, ordinal: u64) -> Self {
        Self {
            gate_id: receipt_gate_id(kind, ordinal),
            kind,
            command: command_for(kind).to_string(),
            receipt_root: sample_root("receipt", kind.as_str(), ordinal),
            artifact_root: sample_root("receipt-artifact", kind.as_str(), ordinal),
            log_root: sample_root("receipt-log", kind.as_str(), ordinal),
            observed_height: config.release_height.saturating_sub(ordinal + 2),
            status: ReceiptStatus::Accepted,
            required: kind.required_by(config),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("gate_id", &self.gate_id)?;
        ensure_non_empty("command", &self.command)?;
        ensure_root("receipt_root", &self.receipt_root)?;
        ensure_root("artifact_root", &self.artifact_root)?;
        ensure_root("log_root", &self.log_root)?;
        ensure(
            self.observed_height > 0,
            "receipt observed height must be non-zero",
        )
    }

    pub fn age_blocks(&self, release_height: u64) -> u64 {
        release_height.saturating_sub(self.observed_height)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate_id": self.gate_id,
            "kind": self.kind.as_str(),
            "command": self.command,
            "receipt_root": self.receipt_root,
            "artifact_root": self.artifact_root,
            "log_root": self.log_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
            "required": self.required,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("receipt-gate", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DashboardApproval {
    pub approval_id: String,
    pub approver: String,
    pub role: ApprovalRole,
    pub lane: LaneKind,
    pub decision: ApprovalDecision,
    pub dashboard_root: String,
    pub signed_statement_root: String,
    pub approved_at_height: u64,
}

impl DashboardApproval {
    pub fn approve(
        config: &Config,
        approver: &str,
        role: ApprovalRole,
        lane: LaneKind,
        ordinal: u64,
    ) -> Self {
        let dashboard_root = sample_root("approval-dashboard", lane.as_str(), ordinal);
        let signed_statement_root =
            approval_statement_root(config, approver, role, lane, &dashboard_root, ordinal);
        Self {
            approval_id: approval_id(approver, role, lane, ordinal),
            approver: approver.to_string(),
            role,
            lane,
            decision: ApprovalDecision::Approved,
            dashboard_root,
            signed_statement_root,
            approved_at_height: config.release_height.saturating_sub(ordinal + 4),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("approval_id", &self.approval_id)?;
        ensure_non_empty("approver", &self.approver)?;
        ensure_root("dashboard_root", &self.dashboard_root)?;
        ensure_root("signed_statement_root", &self.signed_statement_root)?;
        ensure(
            self.approved_at_height > 0,
            "approval height must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "approval_id": self.approval_id,
            "approver": self.approver,
            "role": self.role.as_str(),
            "lane": self.lane.as_str(),
            "decision": self.decision.as_str(),
            "dashboard_root": self.dashboard_root,
            "signed_statement_root": self.signed_statement_root,
            "approved_at_height": self.approved_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("dashboard-approval", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerQuorum {
    pub quorum_id: String,
    pub reviewers: Vec<String>,
    pub reviewer_statement_roots: Vec<String>,
    pub required: u16,
    pub observed: u16,
    pub satisfied: bool,
}

impl ReviewerQuorum {
    pub fn from_approvals(config: &Config, approvals: &[DashboardApproval]) -> Self {
        let reviewers = approvals
            .iter()
            .filter(|approval| {
                approval.role == ApprovalRole::Reviewer && approval.decision.approved()
            })
            .map(|approval| approval.approver.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let reviewer_statement_roots = approvals
            .iter()
            .filter(|approval| {
                approval.role == ApprovalRole::Reviewer && approval.decision.approved()
            })
            .map(|approval| approval.signed_statement_root.clone())
            .collect::<Vec<_>>();
        let observed = reviewers.len() as u16;
        let required = config.min_reviewers;
        let quorum_id = quorum_id("reviewer", required, observed, &reviewer_statement_roots);
        Self {
            quorum_id,
            reviewers,
            reviewer_statement_roots,
            required,
            observed,
            satisfied: observed >= required,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "quorum_id": self.quorum_id,
            "reviewers": self.reviewers,
            "reviewer_statement_roots": self.reviewer_statement_roots,
            "required": self.required,
            "observed": self.observed,
            "satisfied": self.satisfied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("reviewer-quorum", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DashboardAction {
    pub action_id: String,
    pub lane: LaneKind,
    pub owner: String,
    pub label: String,
    pub evidence_root: String,
    pub closed: bool,
    pub due_height: u64,
    pub closed_height: u64,
}

impl DashboardAction {
    pub fn closed(config: &Config, lane: LaneKind, owner: &str, label: &str, ordinal: u64) -> Self {
        Self {
            action_id: dashboard_action_id(lane, owner, label, ordinal),
            lane,
            owner: owner.to_string(),
            label: label.to_string(),
            evidence_root: sample_root("dashboard-action", label, ordinal),
            closed: true,
            due_height: config.release_height.saturating_sub(2),
            closed_height: config.release_height.saturating_sub(3 + ordinal),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "action_id": self.action_id,
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "label": self.label,
            "evidence_root": self.evidence_root,
            "closed": self.closed,
            "due_height": self.due_height,
            "closed_height": self.closed_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("dashboard-action", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BindingBlocker {
    pub blocker_id: String,
    pub lane: String,
    pub kind: BlockerKind,
    pub evidence_root: String,
    pub message: String,
    pub observed_height: u64,
}

impl BindingBlocker {
    pub fn new(
        lane: &str,
        kind: BlockerKind,
        evidence_root: &str,
        message: &str,
        observed_height: u64,
    ) -> Self {
        Self {
            blocker_id: blocker_id(lane, kind, evidence_root, observed_height),
            lane: lane.to_string(),
            kind,
            evidence_root: evidence_root.to_string(),
            message: message.to_string(),
            observed_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "lane": self.lane,
            "kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "message": self.message,
            "observed_height": self.observed_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("binding-blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_record_count: u64,
    pub imported_lane_count: u64,
    pub receipt_gate_count: u64,
    pub accepted_receipt_count: u64,
    pub required_receipt_count: u64,
    pub operator_approval_count: u64,
    pub reviewer_approval_count: u64,
    pub coordinator_approval_count: u64,
    pub closed_action_count: u64,
    pub open_action_count: u64,
    pub blocker_count: u64,
}

impl Counters {
    pub fn build(
        lane_records: &[LaneRecord],
        receipt_gates: &[ReceiptGate],
        approvals: &[DashboardApproval],
        actions: &[DashboardAction],
        blockers: &[BindingBlocker],
    ) -> Self {
        Self {
            lane_record_count: lane_records.len() as u64,
            imported_lane_count: lane_records.iter().filter(|record| record.imported).count()
                as u64,
            receipt_gate_count: receipt_gates.len() as u64,
            accepted_receipt_count: receipt_gates
                .iter()
                .filter(|gate| gate.required && gate.status.accepted())
                .count() as u64,
            required_receipt_count: receipt_gates.iter().filter(|gate| gate.required).count()
                as u64,
            operator_approval_count: approvals
                .iter()
                .filter(|approval| {
                    approval.role == ApprovalRole::Operator && approval.decision.approved()
                })
                .count() as u64,
            reviewer_approval_count: approvals
                .iter()
                .filter(|approval| {
                    approval.role == ApprovalRole::Reviewer && approval.decision.approved()
                })
                .count() as u64,
            coordinator_approval_count: approvals
                .iter()
                .filter(|approval| {
                    approval.role == ApprovalRole::ReleaseCoordinator
                        && approval.decision.approved()
                })
                .count() as u64,
            closed_action_count: actions.iter().filter(|action| action.closed).count() as u64,
            open_action_count: actions.iter().filter(|action| !action.closed).count() as u64,
            blocker_count: blockers.len() as u64,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_record_count": self.lane_record_count,
            "imported_lane_count": self.imported_lane_count,
            "receipt_gate_count": self.receipt_gate_count,
            "accepted_receipt_count": self.accepted_receipt_count,
            "required_receipt_count": self.required_receipt_count,
            "operator_approval_count": self.operator_approval_count,
            "reviewer_approval_count": self.reviewer_approval_count,
            "coordinator_approval_count": self.coordinator_approval_count,
            "closed_action_count": self.closed_action_count,
            "open_action_count": self.open_action_count,
            "blocker_count": self.blocker_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub lane_record_root: String,
    pub receipt_gate_root: String,
    pub approval_root: String,
    pub reviewer_quorum_root: String,
    pub dashboard_action_root: String,
    pub blocker_root: String,
    pub counters_root: String,
    pub release_policy_binding_root: String,
    pub final_go_no_go_root: String,
}

impl Roots {
    pub fn build(
        lane_records: &[LaneRecord],
        receipt_gates: &[ReceiptGate],
        approvals: &[DashboardApproval],
        reviewer_quorum: &ReviewerQuorum,
        actions: &[DashboardAction],
        blockers: &[BindingBlocker],
        counters: &Counters,
        final_go_no_go_root: &str,
    ) -> Self {
        let lane_record_root = map_root(
            "release-policy-binding-lane-records",
            lane_records.iter().map(LaneRecord::state_root),
        );
        let receipt_gate_root = map_root(
            "release-policy-binding-receipt-gates",
            receipt_gates.iter().map(ReceiptGate::state_root),
        );
        let approval_root = map_root(
            "release-policy-binding-dashboard-approvals",
            approvals.iter().map(DashboardApproval::state_root),
        );
        let dashboard_action_root = map_root(
            "release-policy-binding-dashboard-actions",
            actions.iter().map(DashboardAction::state_root),
        );
        let blocker_root = map_root(
            "release-policy-binding-blockers",
            blockers.iter().map(BindingBlocker::state_root),
        );
        let counters_root = counters.state_root();
        let reviewer_quorum_root = reviewer_quorum.state_root();
        let release_policy_binding_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-ROOT",
            &[
                HashPart::Str(&lane_record_root),
                HashPart::Str(&receipt_gate_root),
                HashPart::Str(&approval_root),
                HashPart::Str(&reviewer_quorum_root),
                HashPart::Str(&dashboard_action_root),
                HashPart::Str(&blocker_root),
                HashPart::Str(&counters_root),
                HashPart::Str(final_go_no_go_root),
            ],
            32,
        );
        Self {
            lane_record_root,
            receipt_gate_root,
            approval_root,
            reviewer_quorum_root,
            dashboard_action_root,
            blocker_root,
            counters_root,
            release_policy_binding_root,
            final_go_no_go_root: final_go_no_go_root.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_record_root": self.lane_record_root,
            "receipt_gate_root": self.receipt_gate_root,
            "approval_root": self.approval_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "dashboard_action_root": self.dashboard_action_root,
            "blocker_root": self.blocker_root,
            "counters_root": self.counters_root,
            "release_policy_binding_root": self.release_policy_binding_root,
            "final_go_no_go_root": self.final_go_no_go_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleasePolicyVerdict {
    pub verdict: VerdictKind,
    pub verdict_root: String,
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub reason: String,
    pub bound_at_height: u64,
}

impl ReleasePolicyVerdict {
    pub fn build(
        config: &Config,
        counters: &Counters,
        reviewer_quorum: &ReviewerQuorum,
        blockers: &[BindingBlocker],
    ) -> Self {
        let no_blockers = blockers.is_empty();
        let receipt_satisfied = counters.accepted_receipt_count >= counters.required_receipt_count;
        let operator_satisfied =
            counters.operator_approval_count >= u64::from(config.min_operator_approvals);
        let coordinator_satisfied =
            counters.coordinator_approval_count >= u64::from(config.min_coordinator_approvals);
        let reviewer_satisfied = reviewer_quorum.satisfied || !config.require_reviewer_quorum;
        let approval_satisfied = operator_satisfied && coordinator_satisfied && reviewer_satisfied;
        let release_allowed = no_blockers && receipt_satisfied && approval_satisfied;
        let verdict = if release_allowed {
            VerdictKind::Go
        } else {
            VerdictKind::NoGo
        };
        let reason = if release_allowed {
            "compile/runtime accepted-live-evidence dashboard is bound to release policy go"
                .to_string()
        } else if config.fail_closed {
            "release policy binding is fail-closed until blockers, receipts, and quorums clear"
                .to_string()
        } else {
            "release policy binding has unresolved evidence gaps".to_string()
        };
        let verdict_root = verdict_root(
            config,
            counters,
            reviewer_quorum,
            blockers,
            verdict,
            release_allowed,
        );
        Self {
            verdict,
            verdict_root,
            release_allowed,
            fail_closed: config.fail_closed && !release_allowed,
            reason,
            bound_at_height: config.release_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "verdict_root": self.verdict_root,
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "reason": self.reason,
            "bound_at_height": self.bound_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release-policy-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub lane_records: Vec<LaneRecord>,
    pub receipt_gates: Vec<ReceiptGate>,
    pub dashboard_approvals: Vec<DashboardApproval>,
    pub reviewer_quorum: ReviewerQuorum,
    pub dashboard_actions: Vec<DashboardAction>,
    pub blockers: Vec<BindingBlocker>,
    pub counters: Counters,
    pub roots: Roots,
    pub verdict: ReleasePolicyVerdict,
    pub state_commitment_root: String,
}

impl State {
    pub fn devnet() -> Self {
        Self::build(Config::devnet()).unwrap_or_else(fallback_state)
    }

    pub fn build(config: Config) -> Result<Self> {
        config.validate()?;
        let lane_records = devnet_lane_records(&config);
        let receipt_gates = ReceiptGateKind::all()
            .iter()
            .enumerate()
            .map(|(index, kind)| ReceiptGate::devnet(&config, *kind, index as u64 + 1))
            .collect::<Vec<_>>();
        let dashboard_approvals = devnet_approvals(&config);
        let reviewer_quorum = ReviewerQuorum::from_approvals(&config, &dashboard_approvals);
        let dashboard_actions = devnet_dashboard_actions(&config);
        let blockers = collect_blockers(
            &config,
            &lane_records,
            &receipt_gates,
            &dashboard_approvals,
            &reviewer_quorum,
            &dashboard_actions,
        );
        let counters = Counters::build(
            &lane_records,
            &receipt_gates,
            &dashboard_approvals,
            &dashboard_actions,
            &blockers,
        );
        let verdict = ReleasePolicyVerdict::build(&config, &counters, &reviewer_quorum, &blockers);
        let roots = Roots::build(
            &lane_records,
            &receipt_gates,
            &dashboard_approvals,
            &reviewer_quorum,
            &dashboard_actions,
            &blockers,
            &counters,
            &verdict.verdict_root,
        );
        let state_commitment_root = state_commitment_root(&config, &roots, &verdict, &counters);
        Ok(Self {
            config,
            lane_records,
            receipt_gates,
            dashboard_approvals,
            reviewer_quorum,
            dashboard_actions,
            blockers,
            counters,
            roots,
            verdict,
            state_commitment_root,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "lane_records": self.lane_records.iter().map(LaneRecord::public_record).collect::<Vec<_>>(),
            "receipt_gates": self.receipt_gates.iter().map(ReceiptGate::public_record).collect::<Vec<_>>(),
            "dashboard_approvals": self.dashboard_approvals.iter().map(DashboardApproval::public_record).collect::<Vec<_>>(),
            "reviewer_quorum": self.reviewer_quorum.public_record(),
            "dashboard_actions": self.dashboard_actions.iter().map(DashboardAction::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(BindingBlocker::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "verdict": self.verdict.public_record(),
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
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

fn devnet_lane_records(config: &Config) -> Vec<LaneRecord> {
    LaneKind::all()
        .iter()
        .enumerate()
        .map(|(index, lane)| LaneRecord::devnet(config, *lane, index as u64 + 1))
        .collect()
}

fn devnet_approvals(config: &Config) -> Vec<DashboardApproval> {
    vec![
        DashboardApproval::approve(
            config,
            "operator-compile-runtime-01",
            ApprovalRole::Operator,
            LaneKind::CompileAccepted,
            1,
        ),
        DashboardApproval::approve(
            config,
            "operator-runtime-evidence-02",
            ApprovalRole::Operator,
            LaneKind::RuntimeAccepted,
            2,
        ),
        DashboardApproval::approve(
            config,
            "operator-dashboard-03",
            ApprovalRole::Operator,
            LaneKind::OperatorDashboard,
            3,
        ),
        DashboardApproval::approve(
            config,
            "reviewer-build-systems-01",
            ApprovalRole::Reviewer,
            LaneKind::CompileAccepted,
            4,
        ),
        DashboardApproval::approve(
            config,
            "reviewer-runtime-safety-02",
            ApprovalRole::Reviewer,
            LaneKind::RuntimeAccepted,
            5,
        ),
        DashboardApproval::approve(
            config,
            "reviewer-release-policy-03",
            ApprovalRole::Reviewer,
            LaneKind::ReleasePolicy,
            6,
        ),
        DashboardApproval::approve(
            config,
            "coordinator-release-01",
            ApprovalRole::ReleaseCoordinator,
            LaneKind::OperatorDashboard,
            7,
        ),
        DashboardApproval::approve(
            config,
            "coordinator-release-02",
            ApprovalRole::ReleaseCoordinator,
            LaneKind::ReleasePolicy,
            8,
        ),
    ]
}

fn devnet_dashboard_actions(config: &Config) -> Vec<DashboardAction> {
    vec![
        DashboardAction::closed(
            config,
            LaneKind::CompileAccepted,
            "operator-compile-runtime-01",
            "bind compile receipt root to dashboard evidence",
            1,
        ),
        DashboardAction::closed(
            config,
            LaneKind::RuntimeAccepted,
            "operator-runtime-evidence-02",
            "bind runtime accepted receipt root to dashboard evidence",
            2,
        ),
        DashboardAction::closed(
            config,
            LaneKind::OperatorRunbook,
            "reviewer-build-systems-01",
            "confirm runbook audit output imported into policy gate",
            3,
        ),
        DashboardAction::closed(
            config,
            LaneKind::ReleasePolicy,
            "coordinator-release-01",
            "publish final go no-go root for compile runtime lane",
            4,
        ),
    ]
}

fn collect_blockers(
    config: &Config,
    lane_records: &[LaneRecord],
    receipt_gates: &[ReceiptGate],
    approvals: &[DashboardApproval],
    reviewer_quorum: &ReviewerQuorum,
    actions: &[DashboardAction],
) -> Vec<BindingBlocker> {
    let mut blockers = Vec::new();
    blockers.extend(validate_lane_records(config, lane_records));
    blockers.extend(validate_receipt_gates(config, receipt_gates));
    blockers.extend(validate_approvals(config, approvals, reviewer_quorum));
    blockers.extend(validate_dashboard_actions(config, actions));
    blockers
}

fn validate_lane_records(config: &Config, lane_records: &[LaneRecord]) -> Vec<BindingBlocker> {
    let mut blockers = Vec::new();
    let mut per_lane = BTreeMap::<LaneKind, u16>::new();
    for record in lane_records {
        let count = per_lane.entry(record.lane).or_insert(0);
        *count = count.saturating_add(1);
        if record.validate().is_err() {
            blockers.push(BindingBlocker::new(
                record.lane.as_str(),
                BlockerKind::EmptyRoot,
                &record.state_root(),
                "lane record has an invalid or empty binding root",
                config.release_height,
            ));
        }
        if !record.imported {
            blockers.push(BindingBlocker::new(
                record.lane.as_str(),
                BlockerKind::MissingLaneRecord,
                &record.state_root(),
                "required lane record has not been imported",
                config.release_height,
            ));
        }
        if record.accepted_height > config.release_height {
            blockers.push(BindingBlocker::new(
                record.lane.as_str(),
                BlockerKind::EvidenceRootMismatch,
                &record.evidence_root,
                "lane evidence height is after release binding height",
                config.release_height,
            ));
        }
    }
    for lane in LaneKind::all() {
        match per_lane.get(&lane) {
            Some(count) if *count == 1 => {}
            Some(_) => blockers.push(BindingBlocker::new(
                lane.as_str(),
                BlockerKind::DuplicateLaneRecord,
                &missing_root("duplicate-lane", lane.as_str()),
                "release policy binding requires exactly one record per lane",
                config.release_height,
            )),
            None => blockers.push(BindingBlocker::new(
                lane.as_str(),
                BlockerKind::MissingLaneRecord,
                &missing_root("missing-lane", lane.as_str()),
                "release policy binding is missing a required lane record",
                config.release_height,
            )),
        }
    }
    if lane_records.len() < usize::from(config.min_lane_records) {
        blockers.push(BindingBlocker::new(
            "release_policy",
            BlockerKind::MissingLaneRecord,
            &config.state_root(),
            "imported lane record count is below policy minimum",
            config.release_height,
        ));
    }
    blockers
}

fn validate_receipt_gates(config: &Config, receipt_gates: &[ReceiptGate]) -> Vec<BindingBlocker> {
    let mut blockers = Vec::new();
    let mut seen = BTreeSet::new();
    for gate in receipt_gates {
        if gate.validate().is_err() {
            blockers.push(BindingBlocker::new(
                gate.kind.as_str(),
                BlockerKind::EmptyRoot,
                &gate.state_root(),
                "receipt gate has an invalid or empty root",
                config.release_height,
            ));
        }
        seen.insert(gate.kind);
        if gate.required {
            if gate.status == ReceiptStatus::Missing {
                blockers.push(receipt_blocker(
                    config,
                    gate,
                    BlockerKind::MissingReceipt,
                    "required receipt is missing",
                ));
            }
            if gate.status == ReceiptStatus::Pending {
                blockers.push(receipt_blocker(
                    config,
                    gate,
                    BlockerKind::ReceiptPending,
                    "required receipt is still pending",
                ));
            }
            if gate.status == ReceiptStatus::Failed {
                blockers.push(receipt_blocker(
                    config,
                    gate,
                    BlockerKind::ReceiptFailed,
                    "required receipt failed",
                ));
            }
            if gate.status == ReceiptStatus::Expired {
                blockers.push(receipt_blocker(
                    config,
                    gate,
                    BlockerKind::ReceiptExpired,
                    "required receipt is expired",
                ));
            }
            if gate.age_blocks(config.release_height) > config.max_receipt_age_blocks {
                blockers.push(receipt_blocker(
                    config,
                    gate,
                    BlockerKind::ReceiptExpired,
                    "required receipt is outside the release freshness window",
                ));
            }
        }
    }
    for kind in ReceiptGateKind::all() {
        if kind.required_by(config) && !seen.contains(&kind) {
            blockers.push(BindingBlocker::new(
                kind.as_str(),
                BlockerKind::MissingReceipt,
                &missing_root("missing-receipt", kind.as_str()),
                "release policy binding is missing a required receipt gate",
                config.release_height,
            ));
        }
    }
    blockers
}

fn validate_approvals(
    config: &Config,
    approvals: &[DashboardApproval],
    reviewer_quorum: &ReviewerQuorum,
) -> Vec<BindingBlocker> {
    let mut blockers = Vec::new();
    let operator_approvals = approvals
        .iter()
        .filter(|approval| approval.role == ApprovalRole::Operator && approval.decision.approved())
        .count() as u16;
    let coordinator_approvals = approvals
        .iter()
        .filter(|approval| {
            approval.role == ApprovalRole::ReleaseCoordinator && approval.decision.approved()
        })
        .count() as u16;
    for approval in approvals {
        if approval.validate().is_err() {
            blockers.push(BindingBlocker::new(
                approval.lane.as_str(),
                BlockerKind::EmptyRoot,
                &approval.state_root(),
                "dashboard approval has an invalid or empty root",
                config.release_height,
            ));
        }
        if approval.decision == ApprovalDecision::Rejected {
            blockers.push(BindingBlocker::new(
                approval.lane.as_str(),
                BlockerKind::RejectedApproval,
                &approval.signed_statement_root,
                "dashboard approval explicitly rejects release binding",
                config.release_height,
            ));
        }
    }
    if config.require_dashboard_approval && operator_approvals < config.min_operator_approvals {
        blockers.push(BindingBlocker::new(
            "operator_dashboard",
            BlockerKind::MissingOperatorApproval,
            &approval_set_root(approvals),
            "operator approval quorum is below release policy",
            config.release_height,
        ));
    }
    if config.require_dashboard_approval && coordinator_approvals < config.min_coordinator_approvals
    {
        blockers.push(BindingBlocker::new(
            "release_policy",
            BlockerKind::MissingCoordinatorApproval,
            &approval_set_root(approvals),
            "release coordinator approval quorum is below release policy",
            config.release_height,
        ));
    }
    if config.require_reviewer_quorum && !reviewer_quorum.satisfied {
        blockers.push(BindingBlocker::new(
            "reviewer_quorum",
            BlockerKind::MissingReviewerQuorum,
            &reviewer_quorum.state_root(),
            "reviewer quorum is below release policy",
            config.release_height,
        ));
    }
    blockers
}

fn validate_dashboard_actions(config: &Config, actions: &[DashboardAction]) -> Vec<BindingBlocker> {
    let mut blockers = Vec::new();
    for action in actions {
        if action.evidence_root.trim().is_empty() {
            blockers.push(BindingBlocker::new(
                action.lane.as_str(),
                BlockerKind::EmptyRoot,
                &missing_root("empty-action", &action.action_id),
                "dashboard action evidence root is empty",
                config.release_height,
            ));
        }
        if !action.closed {
            blockers.push(BindingBlocker::new(
                action.lane.as_str(),
                BlockerKind::OpenDashboardAction,
                &action.state_root(),
                "operator dashboard action remains open",
                config.release_height,
            ));
        }
    }
    blockers
}

fn receipt_blocker(
    config: &Config,
    gate: &ReceiptGate,
    kind: BlockerKind,
    message: &str,
) -> BindingBlocker {
    BindingBlocker::new(
        gate.kind.as_str(),
        kind,
        &gate.receipt_root,
        message,
        config.release_height,
    )
}

fn command_for(kind: ReceiptGateKind) -> &'static str {
    match kind {
        ReceiptGateKind::Compile => "cargo build --locked --all-targets",
        ReceiptGateKind::Check => "cargo check --locked --all-targets",
        ReceiptGateKind::Rustfmt => "cargo fmt --all --check",
        ReceiptGateKind::Test => "cargo test --locked --all-targets",
        ReceiptGateKind::Clippy => "cargo clippy --locked --all-targets -- -D warnings",
    }
}

fn lane_id(lane: LaneKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-LANE-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn receipt_gate_id(kind: ReceiptGateKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-RECEIPT-GATE-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn approval_id(approver: &str, role: ApprovalRole, lane: LaneKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-APPROVAL-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(approver),
            HashPart::Str(role.as_str()),
            HashPart::Str(lane.as_str()),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn dashboard_action_id(lane: LaneKind, owner: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-DASHBOARD-ACTION-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(owner),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn blocker_id(lane: &str, kind: BlockerKind, evidence_root: &str, observed_height: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-BLOCKER-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane),
            HashPart::Str(kind.as_str()),
            HashPart::Str(evidence_root),
            HashPart::U64(observed_height),
        ],
        32,
    )
}

fn quorum_id(label: &str, required: u16, observed: u16, roots: &[String]) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-QUORUM-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::U64(u64::from(required)),
            HashPart::U64(u64::from(observed)),
            HashPart::Str(&merkle_root(
                "release-policy-binding-quorum-statements",
                roots,
            )),
        ],
        32,
    )
}

fn release_policy_seed_root(config: &Config, lane: LaneKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-POLICY-SEED",
        &[
            HashPart::Str(&config.binding_suite),
            HashPart::Str(&config.release_channel),
            HashPart::Str(lane.as_str()),
            HashPart::U64(config.release_epoch),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn approval_statement_root(
    config: &Config,
    approver: &str,
    role: ApprovalRole,
    lane: LaneKind,
    dashboard_root: &str,
    ordinal: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-APPROVAL-STATEMENT",
        &[
            HashPart::Str(&config.binding_suite),
            HashPart::Str(approver),
            HashPart::Str(role.as_str()),
            HashPart::Str(lane.as_str()),
            HashPart::Str(dashboard_root),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    counters: &Counters,
    reviewer_quorum: &ReviewerQuorum,
    blockers: &[BindingBlocker],
    verdict: VerdictKind,
    release_allowed: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-VERDICT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(&reviewer_quorum.state_root()),
            HashPart::Str(&map_root(
                "release-policy-binding-verdict-blockers",
                blockers.iter().map(BindingBlocker::state_root),
            )),
            HashPart::Str(verdict.as_str()),
            HashPart::Str(bool_str(release_allowed)),
            HashPart::Str(bool_str(config.fail_closed)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    roots: &Roots,
    verdict: &ReleasePolicyVerdict,
    counters: &Counters,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&verdict.state_root()),
            HashPart::Str(&counters.state_root()),
        ],
        32,
    )
}

fn approval_set_root(approvals: &[DashboardApproval]) -> String {
    map_root(
        "release-policy-binding-approval-set",
        approvals.iter().map(DashboardApproval::state_root),
    )
}

fn sample_root(label: &str, item: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-SAMPLE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Str(item),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn missing_root(label: &str, item: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-MISSING",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Str(item),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-FORCE-EXIT-RELEASE-POLICY-BINDING-RECORD",
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

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let lane_records = vec![LaneRecord {
        lane_id: lane_id(LaneKind::ReleasePolicy, 0),
        lane: LaneKind::ReleasePolicy,
        ordinal: 0,
        source_module: "fallback-release-policy-binding".to_string(),
        evidence_root: missing_root("fallback", "evidence"),
        dashboard_root: missing_root("fallback", "dashboard"),
        runbook_root: missing_root("fallback", "runbook"),
        release_policy_root: missing_root("fallback", "release-policy"),
        accepted_height: config.release_height,
        imported: false,
    }];
    let receipt_gates = Vec::new();
    let dashboard_approvals = Vec::new();
    let reviewer_quorum = ReviewerQuorum {
        quorum_id: quorum_id("fallback", config.min_reviewers, 0, &[]),
        reviewers: Vec::new(),
        reviewer_statement_roots: Vec::new(),
        required: config.min_reviewers,
        observed: 0,
        satisfied: false,
    };
    let dashboard_actions = Vec::new();
    let blockers = vec![BindingBlocker::new(
        "release_policy",
        BlockerKind::ReleasePolicyMismatch,
        &config.state_root(),
        &reason,
        config.release_height,
    )];
    let counters = Counters::build(
        &lane_records,
        &receipt_gates,
        &dashboard_approvals,
        &dashboard_actions,
        &blockers,
    );
    let verdict = ReleasePolicyVerdict::build(&config, &counters, &reviewer_quorum, &blockers);
    let roots = Roots::build(
        &lane_records,
        &receipt_gates,
        &dashboard_approvals,
        &reviewer_quorum,
        &dashboard_actions,
        &blockers,
        &counters,
        &verdict.verdict_root,
    );
    let state_commitment_root = state_commitment_root(&config, &roots, &verdict, &counters);
    State {
        config,
        lane_records,
        receipt_gates,
        dashboard_approvals,
        reviewer_quorum,
        dashboard_actions,
        blockers,
        counters,
        roots,
        verdict,
        state_commitment_root,
    }
}
