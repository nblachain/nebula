use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAuditSecurityAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-audit-security-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const BINDING_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-audit-security-dashboard-release-policy-binding-v1";
pub const DEFAULT_HEIGHT: u64 = 93_083;
pub const DEFAULT_MIN_AUDIT_CLOSURE_RECEIPTS: u64 = 8;
pub const DEFAULT_MIN_PRIVACY_SECURITY_FINDINGS: u64 = 8;
pub const DEFAULT_MIN_REVIEWER_SIGNOFFS: u64 = 5;
pub const DEFAULT_MIN_DASHBOARD_APPROVALS: u64 = 4;
pub const DEFAULT_MAX_OPEN_BLOCKERS: u64 = 0;
pub const DEFAULT_MAX_OPEN_HIGH_OR_CRITICAL_FINDINGS: u64 = 0;
pub const DEFAULT_MAX_DASHBOARD_AGE_BLOCKS: u64 = 72;
pub const DEFAULT_MAX_BINDING_RECORDS: usize = 384;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLane {
    CanonicalUserEscapeAnswer,
    ForceExitPackage,
    AuditSecurity,
    AcceptedLiveEvidence,
    OperatorRunbook,
    OperatorDashboard,
    ReleasePolicy,
    PrivacyNonLinkage,
    SecurityClosure,
    FailClosedBlocker,
}

impl EvidenceLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalUserEscapeAnswer => "canonical_user_escape_answer",
            Self::ForceExitPackage => "force_exit_package",
            Self::AuditSecurity => "audit_security",
            Self::AcceptedLiveEvidence => "accepted_live_evidence",
            Self::OperatorRunbook => "operator_runbook",
            Self::OperatorDashboard => "operator_dashboard",
            Self::ReleasePolicy => "release_policy",
            Self::PrivacyNonLinkage => "privacy_non_linkage",
            Self::SecurityClosure => "security_closure",
            Self::FailClosedBlocker => "fail_closed_blocker",
        }
    }

    pub fn requires_release_policy_binding(self) -> bool {
        matches!(
            self,
            Self::AuditSecurity
                | Self::AcceptedLiveEvidence
                | Self::OperatorDashboard
                | Self::ReleasePolicy
                | Self::PrivacyNonLinkage
                | Self::SecurityClosure
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosureReceiptKind {
    StaticAnalysis,
    DependencyReview,
    RuntimeReplay,
    CanonicalAnswerReplay,
    ForceExitPackageReconstruction,
    MoneroPrivacyReview,
    PqReserveReview,
    DashboardEvidenceImport,
    RunbookArchive,
    ReleasePolicyBinding,
}

impl ClosureReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StaticAnalysis => "static_analysis",
            Self::DependencyReview => "dependency_review",
            Self::RuntimeReplay => "runtime_replay",
            Self::CanonicalAnswerReplay => "canonical_answer_replay",
            Self::ForceExitPackageReconstruction => "force_exit_package_reconstruction",
            Self::MoneroPrivacyReview => "monero_privacy_review",
            Self::PqReserveReview => "pq_reserve_review",
            Self::DashboardEvidenceImport => "dashboard_evidence_import",
            Self::RunbookArchive => "runbook_archive",
            Self::ReleasePolicyBinding => "release_policy_binding",
        }
    }

    pub fn is_required_for_release(self) -> bool {
        matches!(
            self,
            Self::StaticAnalysis
                | Self::DependencyReview
                | Self::RuntimeReplay
                | Self::CanonicalAnswerReplay
                | Self::ForceExitPackageReconstruction
                | Self::MoneroPrivacyReview
                | Self::DashboardEvidenceImport
                | Self::ReleasePolicyBinding
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingArea {
    CustodyBoundary,
    EscapeAnswerIntegrity,
    ForceExitSequencing,
    MoneroAddressPrivacy,
    PqKeyMaterialHandling,
    OperatorDashboardIntegrity,
    ReleasePolicyCompleteness,
    AuditArchiveIntegrity,
}

impl FindingArea {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustodyBoundary => "custody_boundary",
            Self::EscapeAnswerIntegrity => "escape_answer_integrity",
            Self::ForceExitSequencing => "force_exit_sequencing",
            Self::MoneroAddressPrivacy => "monero_address_privacy",
            Self::PqKeyMaterialHandling => "pq_key_material_handling",
            Self::OperatorDashboardIntegrity => "operator_dashboard_integrity",
            Self::ReleasePolicyCompleteness => "release_policy_completeness",
            Self::AuditArchiveIntegrity => "audit_archive_integrity",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    Closed,
    Mitigated,
    AcceptedRisk,
    Open,
    Reopened,
}

impl FindingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Mitigated => "mitigated",
            Self::AcceptedRisk => "accepted_risk",
            Self::Open => "open",
            Self::Reopened => "reopened",
        }
    }

    pub fn release_closed(self) -> bool {
        matches!(self, Self::Closed | Self::Mitigated | Self::AcceptedRisk)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerRole {
    ReleaseCaptain,
    AuditLead,
    SecurityReviewer,
    PrivacyReviewer,
    RuntimeMaintainer,
    OperatorDashboardOwner,
    IncidentCommander,
    ArchiveCustodian,
}

impl ReviewerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCaptain => "release_captain",
            Self::AuditLead => "audit_lead",
            Self::SecurityReviewer => "security_reviewer",
            Self::PrivacyReviewer => "privacy_reviewer",
            Self::RuntimeMaintainer => "runtime_maintainer",
            Self::OperatorDashboardOwner => "operator_dashboard_owner",
            Self::IncidentCommander => "incident_commander",
            Self::ArchiveCustodian => "archive_custodian",
        }
    }

    pub fn required_for_go(self) -> bool {
        matches!(
            self,
            Self::ReleaseCaptain
                | Self::AuditLead
                | Self::SecurityReviewer
                | Self::PrivacyReviewer
                | Self::RuntimeMaintainer
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffDisposition {
    Approved,
    ApprovedWithNotes,
    Watch,
    Rejected,
    Deferred,
}

impl SignoffDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::ApprovedWithNotes => "approved_with_notes",
            Self::Watch => "watch",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
        }
    }

    pub fn release_approved(self) -> bool {
        matches!(self, Self::Approved | Self::ApprovedWithNotes)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardApprovalKind {
    ImportedEvidenceRoot,
    AuditClosureRoot,
    PrivacySecurityFindingRoot,
    ReviewerSignoffRoot,
    BlockerRoot,
    ReleasePolicyBindingRoot,
}

impl DashboardApprovalKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportedEvidenceRoot => "imported_evidence_root",
            Self::AuditClosureRoot => "audit_closure_root",
            Self::PrivacySecurityFindingRoot => "privacy_security_finding_root",
            Self::ReviewerSignoffRoot => "reviewer_signoff_root",
            Self::BlockerRoot => "blocker_root",
            Self::ReleasePolicyBindingRoot => "release_policy_binding_root",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Accepted,
    AcceptedWithWatch,
    Missing,
    Rejected,
    Expired,
}

impl ApprovalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::AcceptedWithWatch => "accepted_with_watch",
            Self::Missing => "missing",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Accepted | Self::AcceptedWithWatch)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingReceipt,
    MissingRequiredSignoff,
    OpenHighFinding,
    OpenCriticalFinding,
    DashboardRejected,
    DashboardExpired,
    EvidenceRootMismatch,
    ReleasePolicyMismatch,
    PrivacyNoticeMissing,
    ManualHold,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReceipt => "missing_receipt",
            Self::MissingRequiredSignoff => "missing_required_signoff",
            Self::OpenHighFinding => "open_high_finding",
            Self::OpenCriticalFinding => "open_critical_finding",
            Self::DashboardRejected => "dashboard_rejected",
            Self::DashboardExpired => "dashboard_expired",
            Self::EvidenceRootMismatch => "evidence_root_mismatch",
            Self::ReleasePolicyMismatch => "release_policy_mismatch",
            Self::PrivacyNoticeMissing => "privacy_notice_missing",
            Self::ManualHold => "manual_hold",
        }
    }

    pub fn fail_closed(self) -> bool {
        matches!(
            self,
            Self::MissingReceipt
                | Self::MissingRequiredSignoff
                | Self::OpenHighFinding
                | Self::OpenCriticalFinding
                | Self::DashboardRejected
                | Self::DashboardExpired
                | Self::EvidenceRootMismatch
                | Self::ReleasePolicyMismatch
                | Self::PrivacyNoticeMissing
                | Self::ManualHold
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerStatus {
    Open,
    Acknowledged,
    Cleared,
    Superseded,
}

impl BlockerStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acknowledged => "acknowledged",
            Self::Cleared => "cleared",
            Self::Superseded => "superseded",
        }
    }

    pub fn active(self) -> bool {
        matches!(self, Self::Open | Self::Acknowledged)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyVerdict {
    Go,
    NoGo,
}

impl ReleasePolicyVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
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
    pub binding_suite: String,
    pub release_policy_id: String,
    pub dashboard_policy_id: String,
    pub required_lanes: Vec<EvidenceLane>,
    pub min_audit_closure_receipts: u64,
    pub min_privacy_security_findings: u64,
    pub min_reviewer_signoffs: u64,
    pub min_dashboard_approvals: u64,
    pub max_open_blockers: u64,
    pub max_open_high_or_critical_findings: u64,
    pub max_dashboard_age_blocks: u64,
    pub fail_closed: bool,
    pub require_privacy_notice: bool,
    pub require_dashboard_root_match: bool,
    pub require_release_policy_root_match: bool,
    pub max_binding_records: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            binding_suite: BINDING_SUITE.to_string(),
            release_policy_id: stable_id("release-policy", "audit-security-accepted-live-evidence"),
            dashboard_policy_id: stable_id(
                "dashboard-policy",
                "operator-dashboard-release-binding",
            ),
            required_lanes: vec![
                EvidenceLane::AuditSecurity,
                EvidenceLane::AcceptedLiveEvidence,
                EvidenceLane::OperatorRunbook,
                EvidenceLane::OperatorDashboard,
                EvidenceLane::ReleasePolicy,
                EvidenceLane::PrivacyNonLinkage,
                EvidenceLane::SecurityClosure,
            ],
            min_audit_closure_receipts: DEFAULT_MIN_AUDIT_CLOSURE_RECEIPTS,
            min_privacy_security_findings: DEFAULT_MIN_PRIVACY_SECURITY_FINDINGS,
            min_reviewer_signoffs: DEFAULT_MIN_REVIEWER_SIGNOFFS,
            min_dashboard_approvals: DEFAULT_MIN_DASHBOARD_APPROVALS,
            max_open_blockers: DEFAULT_MAX_OPEN_BLOCKERS,
            max_open_high_or_critical_findings: DEFAULT_MAX_OPEN_HIGH_OR_CRITICAL_FINDINGS,
            max_dashboard_age_blocks: DEFAULT_MAX_DASHBOARD_AGE_BLOCKS,
            fail_closed: true,
            require_privacy_notice: true,
            require_dashboard_root_match: true,
            require_release_policy_root_match: true,
            max_binding_records: DEFAULT_MAX_BINDING_RECORDS,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("binding_suite", &self.binding_suite)?;
        ensure_non_empty("release_policy_id", &self.release_policy_id)?;
        ensure_non_empty("dashboard_policy_id", &self.dashboard_policy_id)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(
            !self.required_lanes.is_empty(),
            "required lanes must be non-empty",
        )?;
        ensure(
            self.min_audit_closure_receipts > 0,
            "audit closure receipt quorum must be non-zero",
        )?;
        ensure(
            self.min_privacy_security_findings > 0,
            "privacy/security finding quorum must be non-zero",
        )?;
        ensure(
            self.min_reviewer_signoffs > 0,
            "reviewer signoff quorum must be non-zero",
        )?;
        ensure(
            self.min_dashboard_approvals > 0,
            "dashboard approval quorum must be non-zero",
        )?;
        ensure(
            self.max_dashboard_age_blocks > 0,
            "dashboard age window must be non-zero",
        )?;
        ensure(
            self.max_binding_records > 0,
            "record capacity must be non-zero",
        )?;
        let mut seen = BTreeSet::new();
        for lane in &self.required_lanes {
            ensure(seen.insert(*lane), "duplicate required lane")?;
            ensure(
                lane.requires_release_policy_binding(),
                "required lane must bind to release policy evidence",
            )?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": self.protocol_version,
            "chain_id": self.chain_id,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "binding_suite": self.binding_suite,
            "release_policy_id": self.release_policy_id,
            "dashboard_policy_id": self.dashboard_policy_id,
            "required_lanes": self.required_lanes.iter().map(|lane| lane.as_str()).collect::<Vec<_>>(),
            "min_audit_closure_receipts": self.min_audit_closure_receipts,
            "min_privacy_security_findings": self.min_privacy_security_findings,
            "min_reviewer_signoffs": self.min_reviewer_signoffs,
            "min_dashboard_approvals": self.min_dashboard_approvals,
            "max_open_blockers": self.max_open_blockers,
            "max_open_high_or_critical_findings": self.max_open_high_or_critical_findings,
            "max_dashboard_age_blocks": self.max_dashboard_age_blocks,
            "fail_closed": self.fail_closed,
            "require_privacy_notice": self.require_privacy_notice,
            "require_dashboard_root_match": self.require_dashboard_root_match,
            "require_release_policy_root_match": self.require_release_policy_root_match,
            "max_binding_records": self.max_binding_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-CONFIG",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditClosureReceipt {
    pub receipt_id: String,
    pub kind: ClosureReceiptKind,
    pub lane: EvidenceLane,
    pub source_runbook_root: String,
    pub source_dashboard_root: String,
    pub accepted_evidence_root: String,
    pub closure_root: String,
    pub reviewer_role: ReviewerRole,
    pub closed_at_height: u64,
    pub required_for_release: bool,
}

impl AuditClosureReceipt {
    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "kind": self.kind.as_str(),
            "lane": self.lane.as_str(),
            "source_runbook_root": self.source_runbook_root,
            "source_dashboard_root": self.source_dashboard_root,
            "accepted_evidence_root": self.accepted_evidence_root,
            "closure_root": self.closure_root,
            "reviewer_role": self.reviewer_role.as_str(),
            "closed_at_height": self.closed_at_height,
            "required_for_release": self.required_for_release,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("AUDIT-CLOSURE-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivacySecurityFinding {
    pub finding_id: String,
    pub area: FindingArea,
    pub severity: FindingSeverity,
    pub status: FindingStatus,
    pub evidence_root: String,
    pub mitigation_root: String,
    pub privacy_notice_root: String,
    pub owner_role: ReviewerRole,
    pub closed_at_height: u64,
    pub release_blocking: bool,
}

impl PrivacySecurityFinding {
    pub fn public_record(&self) -> Value {
        json!({
            "finding_id": self.finding_id,
            "area": self.area.as_str(),
            "severity": self.severity.as_str(),
            "status": self.status.as_str(),
            "evidence_root": self.evidence_root,
            "mitigation_root": self.mitigation_root,
            "privacy_notice_root": self.privacy_notice_root,
            "owner_role": self.owner_role.as_str(),
            "closed_at_height": self.closed_at_height,
            "release_blocking": self.release_blocking,
        })
    }

    pub fn is_open_release_blocker(&self) -> bool {
        self.release_blocking && self.severity.blocks_release() && !self.status.release_closed()
    }

    pub fn state_root(&self) -> String {
        record_root("PRIVACY-SECURITY-FINDING", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReviewerSignoff {
    pub signoff_id: String,
    pub role: ReviewerRole,
    pub reviewer_id: String,
    pub disposition: SignoffDisposition,
    pub signed_root: String,
    pub notes_root: String,
    pub signed_at_height: u64,
    pub required_for_go: bool,
}

impl ReviewerSignoff {
    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "role": self.role.as_str(),
            "reviewer_id": self.reviewer_id,
            "disposition": self.disposition.as_str(),
            "signed_root": self.signed_root,
            "notes_root": self.notes_root,
            "signed_at_height": self.signed_at_height,
            "required_for_go": self.required_for_go,
        })
    }

    pub fn accepted_for_release(&self) -> bool {
        self.disposition.release_approved()
    }

    pub fn state_root(&self) -> String {
        record_root("REVIEWER-SIGNOFF", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperatorDashboardApproval {
    pub approval_id: String,
    pub kind: DashboardApprovalKind,
    pub owner_role: ReviewerRole,
    pub status: ApprovalStatus,
    pub dashboard_root: String,
    pub release_policy_root: String,
    pub evidence_root: String,
    pub observed_at_height: u64,
    pub expires_at_height: u64,
}

impl OperatorDashboardApproval {
    pub fn public_record(&self) -> Value {
        json!({
            "approval_id": self.approval_id,
            "kind": self.kind.as_str(),
            "owner_role": self.owner_role.as_str(),
            "status": self.status.as_str(),
            "dashboard_root": self.dashboard_root,
            "release_policy_root": self.release_policy_root,
            "evidence_root": self.evidence_root,
            "observed_at_height": self.observed_at_height,
            "expires_at_height": self.expires_at_height,
        })
    }

    pub fn accepted_at(&self, height: u64) -> bool {
        self.status.accepted() && self.expires_at_height >= height
    }

    pub fn state_root(&self) -> String {
        record_root("OPERATOR-DASHBOARD-APPROVAL", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockerHandlingRecord {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub status: BlockerStatus,
    pub owner_role: ReviewerRole,
    pub evidence_root: String,
    pub clearance_root: String,
    pub opened_at_height: u64,
    pub cleared_at_height: u64,
    pub fail_closed: bool,
}

impl BlockerHandlingRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "status": self.status.as_str(),
            "owner_role": self.owner_role.as_str(),
            "evidence_root": self.evidence_root,
            "clearance_root": self.clearance_root,
            "opened_at_height": self.opened_at_height,
            "cleared_at_height": self.cleared_at_height,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn active_fail_closed(&self) -> bool {
        self.fail_closed && self.kind.fail_closed() && self.status.active()
    }

    pub fn state_root(&self) -> String {
        record_root("BLOCKER-HANDLING-RECORD", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingCounters {
    pub audit_closure_receipts: u64,
    pub required_audit_closure_receipts: u64,
    pub privacy_security_findings: u64,
    pub closed_privacy_security_findings: u64,
    pub open_high_or_critical_findings: u64,
    pub reviewer_signoffs: u64,
    pub accepted_reviewer_signoffs: u64,
    pub required_reviewer_roles_covered: u64,
    pub dashboard_approvals: u64,
    pub accepted_dashboard_approvals: u64,
    pub open_blockers: u64,
    pub active_fail_closed_blockers: u64,
}

impl BindingCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "audit_closure_receipts": self.audit_closure_receipts,
            "required_audit_closure_receipts": self.required_audit_closure_receipts,
            "privacy_security_findings": self.privacy_security_findings,
            "closed_privacy_security_findings": self.closed_privacy_security_findings,
            "open_high_or_critical_findings": self.open_high_or_critical_findings,
            "reviewer_signoffs": self.reviewer_signoffs,
            "accepted_reviewer_signoffs": self.accepted_reviewer_signoffs,
            "required_reviewer_roles_covered": self.required_reviewer_roles_covered,
            "dashboard_approvals": self.dashboard_approvals,
            "accepted_dashboard_approvals": self.accepted_dashboard_approvals,
            "open_blockers": self.open_blockers,
            "active_fail_closed_blockers": self.active_fail_closed_blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("BINDING-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleasePolicyBindingVerdict {
    pub verdict: ReleasePolicyVerdict,
    pub verdict_root: String,
    pub policy_root: String,
    pub dashboard_root: String,
    pub runbook_root: String,
    pub accepted_live_evidence_root: String,
    pub audit_closure_root: String,
    pub privacy_security_finding_root: String,
    pub reviewer_signoff_root: String,
    pub operator_dashboard_approval_root: String,
    pub blocker_root: String,
    pub counter_root: String,
    pub fail_closed: bool,
    pub reasons: Vec<String>,
}

impl ReleasePolicyBindingVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "verdict_root": self.verdict_root,
            "policy_root": self.policy_root,
            "dashboard_root": self.dashboard_root,
            "runbook_root": self.runbook_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "audit_closure_root": self.audit_closure_root,
            "privacy_security_finding_root": self.privacy_security_finding_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_dashboard_approval_root": self.operator_dashboard_approval_root,
            "blocker_root": self.blocker_root,
            "counter_root": self.counter_root,
            "fail_closed": self.fail_closed,
            "reasons": self.reasons,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("RELEASE-POLICY-BINDING-VERDICT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub release_policy_root: String,
    pub operator_dashboard_root: String,
    pub operator_runbook_root: String,
    pub accepted_live_evidence_root: String,
    pub audit_closure_receipts: BTreeMap<String, AuditClosureReceipt>,
    pub privacy_security_findings: BTreeMap<String, PrivacySecurityFinding>,
    pub reviewer_signoffs: BTreeMap<String, ReviewerSignoff>,
    pub operator_dashboard_approvals: BTreeMap<String, OperatorDashboardApproval>,
    pub blocker_handling_records: BTreeMap<String, BlockerHandlingRecord>,
    pub counters: BindingCounters,
    pub release_policy_binding: ReleasePolicyBindingVerdict,
}

impl State {
    pub fn new(
        config: Config,
        height: u64,
        release_policy_root: String,
        operator_dashboard_root: String,
        operator_runbook_root: String,
        accepted_live_evidence_root: String,
    ) -> Result<Self> {
        config.validate()?;
        ensure_non_empty("release_policy_root", &release_policy_root)?;
        ensure_non_empty("operator_dashboard_root", &operator_dashboard_root)?;
        ensure_non_empty("operator_runbook_root", &operator_runbook_root)?;
        ensure_non_empty("accepted_live_evidence_root", &accepted_live_evidence_root)?;
        let mut state = Self {
            config,
            height,
            release_policy_root,
            operator_dashboard_root,
            operator_runbook_root,
            accepted_live_evidence_root,
            audit_closure_receipts: BTreeMap::new(),
            privacy_security_findings: BTreeMap::new(),
            reviewer_signoffs: BTreeMap::new(),
            operator_dashboard_approvals: BTreeMap::new(),
            blocker_handling_records: BTreeMap::new(),
            counters: BindingCounters {
                audit_closure_receipts: 0,
                required_audit_closure_receipts: 0,
                privacy_security_findings: 0,
                closed_privacy_security_findings: 0,
                open_high_or_critical_findings: 0,
                reviewer_signoffs: 0,
                accepted_reviewer_signoffs: 0,
                required_reviewer_roles_covered: 0,
                dashboard_approvals: 0,
                accepted_dashboard_approvals: 0,
                open_blockers: 0,
                active_fail_closed_blockers: 0,
            },
            release_policy_binding: ReleasePolicyBindingVerdict {
                verdict: ReleasePolicyVerdict::NoGo,
                verdict_root: empty_root("initial-verdict"),
                policy_root: empty_root("initial-policy"),
                dashboard_root: empty_root("initial-dashboard"),
                runbook_root: empty_root("initial-runbook"),
                accepted_live_evidence_root: empty_root("initial-accepted-evidence"),
                audit_closure_root: empty_root("initial-audit-closure"),
                privacy_security_finding_root: empty_root("initial-privacy-security"),
                reviewer_signoff_root: empty_root("initial-reviewer-signoff"),
                operator_dashboard_approval_root: empty_root("initial-dashboard-approval"),
                blocker_root: empty_root("initial-blocker"),
                counter_root: empty_root("initial-counter"),
                fail_closed: true,
                reasons: vec!["not_evaluated".to_string()],
            },
        };
        state.refresh();
        Ok(state)
    }

    pub fn add_audit_closure_receipt(&mut self, receipt: AuditClosureReceipt) -> Result<()> {
        ensure_capacity(
            self.audit_closure_receipts.len(),
            self.config.max_binding_records,
        )?;
        ensure_non_empty("receipt_id", &receipt.receipt_id)?;
        ensure_non_empty("source_runbook_root", &receipt.source_runbook_root)?;
        ensure_non_empty("source_dashboard_root", &receipt.source_dashboard_root)?;
        ensure_non_empty("accepted_evidence_root", &receipt.accepted_evidence_root)?;
        ensure_non_empty("closure_root", &receipt.closure_root)?;
        ensure(
            receipt.lane.requires_release_policy_binding(),
            "receipt lane must bind release policy",
        )?;
        self.audit_closure_receipts
            .insert(receipt.receipt_id.clone(), receipt);
        self.refresh();
        Ok(())
    }

    pub fn add_privacy_security_finding(&mut self, finding: PrivacySecurityFinding) -> Result<()> {
        ensure_capacity(
            self.privacy_security_findings.len(),
            self.config.max_binding_records,
        )?;
        ensure_non_empty("finding_id", &finding.finding_id)?;
        ensure_non_empty("evidence_root", &finding.evidence_root)?;
        ensure_non_empty("mitigation_root", &finding.mitigation_root)?;
        if self.config.require_privacy_notice {
            ensure_non_empty("privacy_notice_root", &finding.privacy_notice_root)?;
        }
        self.privacy_security_findings
            .insert(finding.finding_id.clone(), finding);
        self.refresh();
        Ok(())
    }

    pub fn add_reviewer_signoff(&mut self, signoff: ReviewerSignoff) -> Result<()> {
        ensure_capacity(
            self.reviewer_signoffs.len(),
            self.config.max_binding_records,
        )?;
        ensure_non_empty("signoff_id", &signoff.signoff_id)?;
        ensure_non_empty("reviewer_id", &signoff.reviewer_id)?;
        ensure_non_empty("signed_root", &signoff.signed_root)?;
        ensure_non_empty("notes_root", &signoff.notes_root)?;
        self.reviewer_signoffs
            .insert(signoff.signoff_id.clone(), signoff);
        self.refresh();
        Ok(())
    }

    pub fn add_dashboard_approval(&mut self, approval: OperatorDashboardApproval) -> Result<()> {
        ensure_capacity(
            self.operator_dashboard_approvals.len(),
            self.config.max_binding_records,
        )?;
        ensure_non_empty("approval_id", &approval.approval_id)?;
        ensure_non_empty("dashboard_root", &approval.dashboard_root)?;
        ensure_non_empty("release_policy_root", &approval.release_policy_root)?;
        ensure_non_empty("evidence_root", &approval.evidence_root)?;
        ensure(
            approval.expires_at_height >= approval.observed_at_height,
            "dashboard approval expiry must be at or after observation height",
        )?;
        self.operator_dashboard_approvals
            .insert(approval.approval_id.clone(), approval);
        self.refresh();
        Ok(())
    }

    pub fn add_blocker(&mut self, blocker: BlockerHandlingRecord) -> Result<()> {
        ensure_capacity(
            self.blocker_handling_records.len(),
            self.config.max_binding_records,
        )?;
        ensure_non_empty("blocker_id", &blocker.blocker_id)?;
        ensure_non_empty("evidence_root", &blocker.evidence_root)?;
        ensure_non_empty("clearance_root", &blocker.clearance_root)?;
        ensure(
            blocker.cleared_at_height >= blocker.opened_at_height,
            "blocker clearance height must be at or after open height",
        )?;
        self.blocker_handling_records
            .insert(blocker.blocker_id.clone(), blocker);
        self.refresh();
        Ok(())
    }

    pub fn audit_closure_root(&self) -> String {
        map_root(
            "AUDIT-CLOSURE-RECEIPT-ROOT",
            self.audit_closure_receipts
                .values()
                .map(AuditClosureReceipt::state_root),
        )
    }

    pub fn privacy_security_finding_root(&self) -> String {
        map_root(
            "PRIVACY-SECURITY-FINDING-ROOT",
            self.privacy_security_findings
                .values()
                .map(PrivacySecurityFinding::state_root),
        )
    }

    pub fn reviewer_signoff_root(&self) -> String {
        map_root(
            "REVIEWER-SIGNOFF-ROOT",
            self.reviewer_signoffs
                .values()
                .map(ReviewerSignoff::state_root),
        )
    }

    pub fn dashboard_approval_root(&self) -> String {
        map_root(
            "OPERATOR-DASHBOARD-APPROVAL-ROOT",
            self.operator_dashboard_approvals
                .values()
                .map(OperatorDashboardApproval::state_root),
        )
    }

    pub fn blocker_root(&self) -> String {
        map_root(
            "BLOCKER-HANDLING-ROOT",
            self.blocker_handling_records
                .values()
                .map(BlockerHandlingRecord::state_root),
        )
    }

    pub fn refresh(&mut self) {
        self.counters = self.compute_counters();
        self.release_policy_binding = self.compute_release_policy_binding();
    }

    pub fn compute_counters(&self) -> BindingCounters {
        let required_roles = self.required_reviewer_roles_covered();
        BindingCounters {
            audit_closure_receipts: self.audit_closure_receipts.len() as u64,
            required_audit_closure_receipts: self
                .audit_closure_receipts
                .values()
                .filter(|receipt| {
                    receipt.required_for_release && receipt.kind.is_required_for_release()
                })
                .count() as u64,
            privacy_security_findings: self.privacy_security_findings.len() as u64,
            closed_privacy_security_findings: self
                .privacy_security_findings
                .values()
                .filter(|finding| finding.status.release_closed())
                .count() as u64,
            open_high_or_critical_findings: self
                .privacy_security_findings
                .values()
                .filter(|finding| finding.is_open_release_blocker())
                .count() as u64,
            reviewer_signoffs: self.reviewer_signoffs.len() as u64,
            accepted_reviewer_signoffs: self
                .reviewer_signoffs
                .values()
                .filter(|signoff| signoff.accepted_for_release())
                .count() as u64,
            required_reviewer_roles_covered: required_roles,
            dashboard_approvals: self.operator_dashboard_approvals.len() as u64,
            accepted_dashboard_approvals: self
                .operator_dashboard_approvals
                .values()
                .filter(|approval| approval.accepted_at(self.height))
                .count() as u64,
            open_blockers: self
                .blocker_handling_records
                .values()
                .filter(|blocker| blocker.status.active())
                .count() as u64,
            active_fail_closed_blockers: self
                .blocker_handling_records
                .values()
                .filter(|blocker| blocker.active_fail_closed())
                .count() as u64,
        }
    }

    pub fn compute_release_policy_binding(&self) -> ReleasePolicyBindingVerdict {
        let counters = self.compute_counters();
        let audit_closure_root = self.audit_closure_root();
        let privacy_security_finding_root = self.privacy_security_finding_root();
        let reviewer_signoff_root = self.reviewer_signoff_root();
        let dashboard_approval_root = self.dashboard_approval_root();
        let blocker_root = self.blocker_root();
        let counter_root = counters.state_root();
        let mut reasons = Vec::new();

        if counters.required_audit_closure_receipts < self.config.min_audit_closure_receipts {
            reasons.push("audit_closure_receipt_quorum_missing".to_string());
        }
        if counters.closed_privacy_security_findings < self.config.min_privacy_security_findings {
            reasons.push("privacy_security_finding_closure_quorum_missing".to_string());
        }
        if counters.open_high_or_critical_findings > self.config.max_open_high_or_critical_findings
        {
            reasons.push("open_high_or_critical_finding_present".to_string());
        }
        if counters.accepted_reviewer_signoffs < self.config.min_reviewer_signoffs {
            reasons.push("reviewer_signoff_quorum_missing".to_string());
        }
        if counters.required_reviewer_roles_covered < self.required_reviewer_role_count() {
            reasons.push("required_reviewer_role_missing".to_string());
        }
        if counters.accepted_dashboard_approvals < self.config.min_dashboard_approvals {
            reasons.push("operator_dashboard_approval_quorum_missing".to_string());
        }
        if counters.open_blockers > self.config.max_open_blockers {
            reasons.push("open_blocker_present".to_string());
        }
        if self.config.fail_closed && counters.active_fail_closed_blockers > 0 {
            reasons.push("active_fail_closed_blocker_present".to_string());
        }
        if self.config.require_dashboard_root_match && !self.dashboard_roots_match() {
            reasons.push("dashboard_root_mismatch".to_string());
        }
        if self.config.require_release_policy_root_match && !self.release_policy_roots_match() {
            reasons.push("release_policy_root_mismatch".to_string());
        }
        if self.config.require_privacy_notice && !self.privacy_notices_present() {
            reasons.push("privacy_notice_missing".to_string());
        }
        if !self.required_lanes_covered() {
            reasons.push("required_release_policy_lane_missing".to_string());
        }
        if reasons.is_empty() {
            reasons.push("all_release_policy_evidence_bound".to_string());
        }

        let verdict = if reasons.len() == 1 && reasons[0] == "all_release_policy_evidence_bound" {
            ReleasePolicyVerdict::Go
        } else {
            ReleasePolicyVerdict::NoGo
        };
        let verdict_root = domain_hash(
            "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-VERDICT-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(verdict.as_str()),
                HashPart::Str(&self.release_policy_root),
                HashPart::Str(&self.operator_dashboard_root),
                HashPart::Str(&self.operator_runbook_root),
                HashPart::Str(&self.accepted_live_evidence_root),
                HashPart::Str(&audit_closure_root),
                HashPart::Str(&privacy_security_finding_root),
                HashPart::Str(&reviewer_signoff_root),
                HashPart::Str(&dashboard_approval_root),
                HashPart::Str(&blocker_root),
                HashPart::Str(&counter_root),
                HashPart::Json(&json!(reasons)),
            ],
            32,
        );
        ReleasePolicyBindingVerdict {
            verdict,
            verdict_root,
            policy_root: self.release_policy_root.clone(),
            dashboard_root: self.operator_dashboard_root.clone(),
            runbook_root: self.operator_runbook_root.clone(),
            accepted_live_evidence_root: self.accepted_live_evidence_root.clone(),
            audit_closure_root,
            privacy_security_finding_root,
            reviewer_signoff_root,
            operator_dashboard_approval_root: dashboard_approval_root,
            blocker_root,
            counter_root,
            fail_closed: self.config.fail_closed,
            reasons,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "config": self.config.public_record(),
            "release_policy_root": self.release_policy_root,
            "operator_dashboard_root": self.operator_dashboard_root,
            "operator_runbook_root": self.operator_runbook_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "audit_closure_receipts": self.audit_closure_receipts.values().map(AuditClosureReceipt::public_record).collect::<Vec<_>>(),
            "privacy_security_findings": self.privacy_security_findings.values().map(PrivacySecurityFinding::public_record).collect::<Vec<_>>(),
            "reviewer_signoffs": self.reviewer_signoffs.values().map(ReviewerSignoff::public_record).collect::<Vec<_>>(),
            "operator_dashboard_approvals": self.operator_dashboard_approvals.values().map(OperatorDashboardApproval::public_record).collect::<Vec<_>>(),
            "blocker_handling_records": self.blocker_handling_records.values().map(BlockerHandlingRecord::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "release_policy_binding": self.release_policy_binding.public_record(),
            "roots": {
                "config_root": self.config.state_root(),
                "audit_closure_root": self.audit_closure_root(),
                "privacy_security_finding_root": self.privacy_security_finding_root(),
                "reviewer_signoff_root": self.reviewer_signoff_root(),
                "operator_dashboard_approval_root": self.dashboard_approval_root(),
                "blocker_root": self.blocker_root(),
                "counter_root": self.counters.state_root(),
                "release_policy_binding_root": self.release_policy_binding.state_root(),
            }
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.release_policy_root),
                HashPart::Str(&self.operator_dashboard_root),
                HashPart::Str(&self.operator_runbook_root),
                HashPart::Str(&self.accepted_live_evidence_root),
                HashPart::Str(&self.audit_closure_root()),
                HashPart::Str(&self.privacy_security_finding_root()),
                HashPart::Str(&self.reviewer_signoff_root()),
                HashPart::Str(&self.dashboard_approval_root()),
                HashPart::Str(&self.blocker_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&self.release_policy_binding.state_root()),
                HashPart::U64(self.height),
            ],
            32,
        )
    }

    fn required_reviewer_role_count(&self) -> u64 {
        [
            ReviewerRole::ReleaseCaptain,
            ReviewerRole::AuditLead,
            ReviewerRole::SecurityReviewer,
            ReviewerRole::PrivacyReviewer,
            ReviewerRole::RuntimeMaintainer,
        ]
        .iter()
        .filter(|role| role.required_for_go())
        .count() as u64
    }

    fn required_reviewer_roles_covered(&self) -> u64 {
        let mut covered = BTreeSet::new();
        for signoff in self.reviewer_signoffs.values() {
            if signoff.required_for_go
                && signoff.role.required_for_go()
                && signoff.accepted_for_release()
            {
                covered.insert(signoff.role);
            }
        }
        covered.len() as u64
    }

    fn dashboard_roots_match(&self) -> bool {
        self.operator_dashboard_approvals
            .values()
            .filter(|approval| approval.accepted_at(self.height))
            .all(|approval| approval.dashboard_root == self.operator_dashboard_root)
    }

    fn release_policy_roots_match(&self) -> bool {
        self.operator_dashboard_approvals
            .values()
            .filter(|approval| approval.accepted_at(self.height))
            .all(|approval| approval.release_policy_root == self.release_policy_root)
    }

    fn privacy_notices_present(&self) -> bool {
        self.privacy_security_findings
            .values()
            .all(|finding| !finding.privacy_notice_root.is_empty())
    }

    fn required_lanes_covered(&self) -> bool {
        let mut lanes = BTreeSet::new();
        for receipt in self.audit_closure_receipts.values() {
            if receipt.required_for_release {
                lanes.insert(receipt.lane);
            }
        }
        for lane in &self.config.required_lanes {
            if matches!(
                lane,
                EvidenceLane::OperatorDashboard | EvidenceLane::ReleasePolicy
            ) {
                continue;
            }
            if !lanes.contains(lane) {
                return false;
            }
        }
        true
    }
}

pub fn devnet() -> State {
    devnet_result().unwrap_or_else_state()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn release_policy_binding_root() -> String {
    devnet().release_policy_binding.state_root()
}

pub fn release_policy_binding_verdict() -> ReleasePolicyVerdict {
    devnet().release_policy_binding.verdict
}

fn devnet_result() -> Result<State> {
    let release_policy_root = fixture_root("release-policy-root");
    let operator_dashboard_root = fixture_root("operator-dashboard-root");
    let operator_runbook_root = fixture_root("operator-runbook-root");
    let accepted_live_evidence_root = fixture_root("accepted-live-evidence-root");
    let mut state = State::new(
        Config::devnet(),
        DEFAULT_HEIGHT,
        release_policy_root.clone(),
        operator_dashboard_root.clone(),
        operator_runbook_root.clone(),
        accepted_live_evidence_root.clone(),
    )?;
    for (index, kind, lane, role) in audit_receipt_plan() {
        let receipt = build_audit_closure_receipt(
            index,
            kind,
            lane,
            role,
            &operator_runbook_root,
            &operator_dashboard_root,
            &accepted_live_evidence_root,
        );
        state.add_audit_closure_receipt(receipt)?;
    }
    for (index, area, severity, role) in privacy_security_finding_plan() {
        let finding = build_privacy_security_finding(index, area, severity, role);
        state.add_privacy_security_finding(finding)?;
    }
    for (index, role, reviewer_id) in reviewer_signoff_plan() {
        let signed_root = domain_hash(
            "AUDIT-SECURITY-SIGNOFF-SIGNED-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(role.as_str()),
                HashPart::Str(reviewer_id),
                HashPart::Str(&state.audit_closure_root()),
                HashPart::Str(&state.privacy_security_finding_root()),
                HashPart::U64(index),
            ],
            32,
        );
        let signoff = build_reviewer_signoff(index, role, reviewer_id, &signed_root);
        state.add_reviewer_signoff(signoff)?;
    }
    for (index, kind, role) in dashboard_approval_plan() {
        let evidence_root = match kind {
            DashboardApprovalKind::ImportedEvidenceRoot => accepted_live_evidence_root.clone(),
            DashboardApprovalKind::AuditClosureRoot => state.audit_closure_root(),
            DashboardApprovalKind::PrivacySecurityFindingRoot => {
                state.privacy_security_finding_root()
            }
            DashboardApprovalKind::ReviewerSignoffRoot => state.reviewer_signoff_root(),
            DashboardApprovalKind::BlockerRoot => state.blocker_root(),
            DashboardApprovalKind::ReleasePolicyBindingRoot => release_policy_root.clone(),
        };
        let approval = build_dashboard_approval(
            index,
            kind,
            role,
            &operator_dashboard_root,
            &release_policy_root,
            &evidence_root,
        );
        state.add_dashboard_approval(approval)?;
    }
    for (index, kind, status, role) in blocker_plan() {
        let blocker = build_blocker(index, kind, status, role);
        state.add_blocker(blocker)?;
    }
    state.refresh();
    Ok(state)
}

trait DevnetFallback {
    fn unwrap_or_else_state(self) -> State;
}

impl DevnetFallback for Result<State> {
    fn unwrap_or_else_state(self) -> State {
        match self {
            Ok(state) => state,
            Err(error) => fallback_state(&error),
        }
    }
}

fn fallback_state(error: &str) -> State {
    let config = Config::devnet();
    let release_policy_root = fixture_root("fallback-release-policy-root");
    let operator_dashboard_root = fixture_root("fallback-operator-dashboard-root");
    let operator_runbook_root = fixture_root("fallback-operator-runbook-root");
    let accepted_live_evidence_root = fixture_root("fallback-accepted-live-evidence-root");
    let counters = BindingCounters {
        audit_closure_receipts: 0,
        required_audit_closure_receipts: 0,
        privacy_security_findings: 0,
        closed_privacy_security_findings: 0,
        open_high_or_critical_findings: 0,
        reviewer_signoffs: 0,
        accepted_reviewer_signoffs: 0,
        required_reviewer_roles_covered: 0,
        dashboard_approvals: 0,
        accepted_dashboard_approvals: 0,
        open_blockers: 1,
        active_fail_closed_blockers: 1,
    };
    let counter_root = counters.state_root();
    let release_policy_binding = ReleasePolicyBindingVerdict {
        verdict: ReleasePolicyVerdict::NoGo,
        verdict_root: domain_hash(
            "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-FALLBACK-VERDICT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(error),
                HashPart::Str(&counter_root),
            ],
            32,
        ),
        policy_root: release_policy_root.clone(),
        dashboard_root: operator_dashboard_root.clone(),
        runbook_root: operator_runbook_root.clone(),
        accepted_live_evidence_root: accepted_live_evidence_root.clone(),
        audit_closure_root: empty_root("fallback-audit-closure"),
        privacy_security_finding_root: empty_root("fallback-privacy-security"),
        reviewer_signoff_root: empty_root("fallback-reviewer-signoff"),
        operator_dashboard_approval_root: empty_root("fallback-dashboard-approval"),
        blocker_root: empty_root("fallback-blocker"),
        counter_root,
        fail_closed: true,
        reasons: vec![
            "devnet_construction_failed_fail_closed".to_string(),
            error.to_string(),
        ],
    };
    State {
        config,
        height: DEFAULT_HEIGHT,
        release_policy_root,
        operator_dashboard_root,
        operator_runbook_root,
        accepted_live_evidence_root,
        audit_closure_receipts: BTreeMap::new(),
        privacy_security_findings: BTreeMap::new(),
        reviewer_signoffs: BTreeMap::new(),
        operator_dashboard_approvals: BTreeMap::new(),
        blocker_handling_records: BTreeMap::new(),
        counters,
        release_policy_binding,
    }
}

fn audit_receipt_plan() -> Vec<(u64, ClosureReceiptKind, EvidenceLane, ReviewerRole)> {
    vec![
        (
            0,
            ClosureReceiptKind::StaticAnalysis,
            EvidenceLane::AuditSecurity,
            ReviewerRole::SecurityReviewer,
        ),
        (
            1,
            ClosureReceiptKind::DependencyReview,
            EvidenceLane::SecurityClosure,
            ReviewerRole::SecurityReviewer,
        ),
        (
            2,
            ClosureReceiptKind::RuntimeReplay,
            EvidenceLane::AcceptedLiveEvidence,
            ReviewerRole::RuntimeMaintainer,
        ),
        (
            3,
            ClosureReceiptKind::CanonicalAnswerReplay,
            EvidenceLane::AcceptedLiveEvidence,
            ReviewerRole::AuditLead,
        ),
        (
            4,
            ClosureReceiptKind::ForceExitPackageReconstruction,
            EvidenceLane::SecurityClosure,
            ReviewerRole::RuntimeMaintainer,
        ),
        (
            5,
            ClosureReceiptKind::MoneroPrivacyReview,
            EvidenceLane::PrivacyNonLinkage,
            ReviewerRole::PrivacyReviewer,
        ),
        (
            6,
            ClosureReceiptKind::DashboardEvidenceImport,
            EvidenceLane::OperatorDashboard,
            ReviewerRole::OperatorDashboardOwner,
        ),
        (
            7,
            ClosureReceiptKind::ReleasePolicyBinding,
            EvidenceLane::ReleasePolicy,
            ReviewerRole::ReleaseCaptain,
        ),
        (
            8,
            ClosureReceiptKind::RunbookArchive,
            EvidenceLane::OperatorRunbook,
            ReviewerRole::ArchiveCustodian,
        ),
        (
            9,
            ClosureReceiptKind::PqReserveReview,
            EvidenceLane::PrivacyNonLinkage,
            ReviewerRole::PrivacyReviewer,
        ),
    ]
}

fn privacy_security_finding_plan() -> Vec<(u64, FindingArea, FindingSeverity, ReviewerRole)> {
    vec![
        (
            0,
            FindingArea::CustodyBoundary,
            FindingSeverity::Medium,
            ReviewerRole::SecurityReviewer,
        ),
        (
            1,
            FindingArea::EscapeAnswerIntegrity,
            FindingSeverity::Medium,
            ReviewerRole::AuditLead,
        ),
        (
            2,
            FindingArea::ForceExitSequencing,
            FindingSeverity::Low,
            ReviewerRole::RuntimeMaintainer,
        ),
        (
            3,
            FindingArea::MoneroAddressPrivacy,
            FindingSeverity::High,
            ReviewerRole::PrivacyReviewer,
        ),
        (
            4,
            FindingArea::PqKeyMaterialHandling,
            FindingSeverity::High,
            ReviewerRole::PrivacyReviewer,
        ),
        (
            5,
            FindingArea::OperatorDashboardIntegrity,
            FindingSeverity::Medium,
            ReviewerRole::OperatorDashboardOwner,
        ),
        (
            6,
            FindingArea::ReleasePolicyCompleteness,
            FindingSeverity::Medium,
            ReviewerRole::ReleaseCaptain,
        ),
        (
            7,
            FindingArea::AuditArchiveIntegrity,
            FindingSeverity::Low,
            ReviewerRole::ArchiveCustodian,
        ),
    ]
}

fn reviewer_signoff_plan() -> Vec<(u64, ReviewerRole, &'static str)> {
    vec![
        (0, ReviewerRole::ReleaseCaptain, "release-captain-wave-83"),
        (1, ReviewerRole::AuditLead, "audit-lead-wave-83"),
        (
            2,
            ReviewerRole::SecurityReviewer,
            "security-reviewer-wave-83",
        ),
        (3, ReviewerRole::PrivacyReviewer, "privacy-reviewer-wave-83"),
        (
            4,
            ReviewerRole::RuntimeMaintainer,
            "runtime-maintainer-wave-83",
        ),
        (
            5,
            ReviewerRole::OperatorDashboardOwner,
            "dashboard-owner-wave-83",
        ),
    ]
}

fn dashboard_approval_plan() -> Vec<(u64, DashboardApprovalKind, ReviewerRole)> {
    vec![
        (
            0,
            DashboardApprovalKind::ImportedEvidenceRoot,
            ReviewerRole::OperatorDashboardOwner,
        ),
        (
            1,
            DashboardApprovalKind::AuditClosureRoot,
            ReviewerRole::AuditLead,
        ),
        (
            2,
            DashboardApprovalKind::PrivacySecurityFindingRoot,
            ReviewerRole::SecurityReviewer,
        ),
        (
            3,
            DashboardApprovalKind::ReviewerSignoffRoot,
            ReviewerRole::ReleaseCaptain,
        ),
        (
            4,
            DashboardApprovalKind::BlockerRoot,
            ReviewerRole::IncidentCommander,
        ),
        (
            5,
            DashboardApprovalKind::ReleasePolicyBindingRoot,
            ReviewerRole::ReleaseCaptain,
        ),
    ]
}

fn blocker_plan() -> Vec<(u64, BlockerKind, BlockerStatus, ReviewerRole)> {
    vec![
        (
            0,
            BlockerKind::MissingReceipt,
            BlockerStatus::Cleared,
            ReviewerRole::AuditLead,
        ),
        (
            1,
            BlockerKind::EvidenceRootMismatch,
            BlockerStatus::Superseded,
            ReviewerRole::OperatorDashboardOwner,
        ),
        (
            2,
            BlockerKind::PrivacyNoticeMissing,
            BlockerStatus::Cleared,
            ReviewerRole::PrivacyReviewer,
        ),
    ]
}

fn build_audit_closure_receipt(
    index: u64,
    kind: ClosureReceiptKind,
    lane: EvidenceLane,
    role: ReviewerRole,
    source_runbook_root: &str,
    source_dashboard_root: &str,
    accepted_evidence_root: &str,
) -> AuditClosureReceipt {
    let closure_root = domain_hash(
        "AUDIT-SECURITY-CLOSURE-RECEIPT-CLOSURE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(lane.as_str()),
            HashPart::Str(role.as_str()),
            HashPart::Str(source_runbook_root),
            HashPart::Str(source_dashboard_root),
            HashPart::Str(accepted_evidence_root),
            HashPart::U64(index),
        ],
        32,
    );
    let receipt_id = domain_hash(
        "AUDIT-SECURITY-CLOSURE-RECEIPT-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&closure_root),
        ],
        16,
    );
    AuditClosureReceipt {
        receipt_id,
        kind,
        lane,
        source_runbook_root: source_runbook_root.to_string(),
        source_dashboard_root: source_dashboard_root.to_string(),
        accepted_evidence_root: accepted_evidence_root.to_string(),
        closure_root,
        reviewer_role: role,
        closed_at_height: DEFAULT_HEIGHT.saturating_sub(20).saturating_add(index),
        required_for_release: kind.is_required_for_release(),
    }
}

fn build_privacy_security_finding(
    index: u64,
    area: FindingArea,
    severity: FindingSeverity,
    role: ReviewerRole,
) -> PrivacySecurityFinding {
    let evidence_root = domain_hash(
        "AUDIT-SECURITY-PRIVACY-FINDING-EVIDENCE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(area.as_str()),
            HashPart::Str(severity.as_str()),
            HashPart::Str(role.as_str()),
            HashPart::U64(index),
        ],
        32,
    );
    let mitigation_root = domain_hash(
        "AUDIT-SECURITY-PRIVACY-FINDING-MITIGATION",
        &[
            HashPart::Str(&evidence_root),
            HashPart::Str(area.as_str()),
            HashPart::Str("mitigated-before-release-policy-binding"),
        ],
        32,
    );
    let privacy_notice_root = domain_hash(
        "AUDIT-SECURITY-PRIVACY-FINDING-NOTICE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(area.as_str()),
            HashPart::Str(&mitigation_root),
        ],
        32,
    );
    let finding_id = domain_hash(
        "AUDIT-SECURITY-PRIVACY-FINDING-ID",
        &[
            HashPart::Str(area.as_str()),
            HashPart::Str(severity.as_str()),
            HashPart::Str(&evidence_root),
        ],
        16,
    );
    PrivacySecurityFinding {
        finding_id,
        area,
        severity,
        status: FindingStatus::Closed,
        evidence_root,
        mitigation_root,
        privacy_notice_root,
        owner_role: role,
        closed_at_height: DEFAULT_HEIGHT.saturating_sub(16).saturating_add(index),
        release_blocking: severity.blocks_release(),
    }
}

fn build_reviewer_signoff(
    index: u64,
    role: ReviewerRole,
    reviewer_id: &str,
    signed_root: &str,
) -> ReviewerSignoff {
    let notes_root = domain_hash(
        "AUDIT-SECURITY-REVIEWER-SIGNOFF-NOTES",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(role.as_str()),
            HashPart::Str(reviewer_id),
            HashPart::Str(signed_root),
        ],
        32,
    );
    let signoff_id = domain_hash(
        "AUDIT-SECURITY-REVIEWER-SIGNOFF-ID",
        &[
            HashPart::Str(role.as_str()),
            HashPart::Str(reviewer_id),
            HashPart::Str(&notes_root),
        ],
        16,
    );
    ReviewerSignoff {
        signoff_id,
        role,
        reviewer_id: reviewer_id.to_string(),
        disposition: SignoffDisposition::Approved,
        signed_root: signed_root.to_string(),
        notes_root,
        signed_at_height: DEFAULT_HEIGHT.saturating_sub(8).saturating_add(index),
        required_for_go: role.required_for_go(),
    }
}

fn build_dashboard_approval(
    index: u64,
    kind: DashboardApprovalKind,
    role: ReviewerRole,
    dashboard_root: &str,
    release_policy_root: &str,
    evidence_root: &str,
) -> OperatorDashboardApproval {
    let approval_id = domain_hash(
        "AUDIT-SECURITY-DASHBOARD-APPROVAL-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(role.as_str()),
            HashPart::Str(dashboard_root),
            HashPart::Str(release_policy_root),
            HashPart::Str(evidence_root),
            HashPart::U64(index),
        ],
        16,
    );
    OperatorDashboardApproval {
        approval_id,
        kind,
        owner_role: role,
        status: ApprovalStatus::Accepted,
        dashboard_root: dashboard_root.to_string(),
        release_policy_root: release_policy_root.to_string(),
        evidence_root: evidence_root.to_string(),
        observed_at_height: DEFAULT_HEIGHT.saturating_sub(4).saturating_add(index),
        expires_at_height: DEFAULT_HEIGHT.saturating_add(DEFAULT_MAX_DASHBOARD_AGE_BLOCKS),
    }
}

fn build_blocker(
    index: u64,
    kind: BlockerKind,
    status: BlockerStatus,
    role: ReviewerRole,
) -> BlockerHandlingRecord {
    let evidence_root = domain_hash(
        "AUDIT-SECURITY-BLOCKER-EVIDENCE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(role.as_str()),
            HashPart::U64(index),
        ],
        32,
    );
    let clearance_root = domain_hash(
        "AUDIT-SECURITY-BLOCKER-CLEARANCE",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&evidence_root),
        ],
        32,
    );
    let blocker_id = domain_hash(
        "AUDIT-SECURITY-BLOCKER-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(role.as_str()),
            HashPart::Str(&clearance_root),
        ],
        16,
    );
    BlockerHandlingRecord {
        blocker_id,
        kind,
        status,
        owner_role: role,
        evidence_root,
        clearance_root,
        opened_at_height: DEFAULT_HEIGHT.saturating_sub(32).saturating_add(index),
        cleared_at_height: DEFAULT_HEIGHT.saturating_sub(3).saturating_add(index),
        fail_closed: kind.fail_closed(),
    }
}

fn stable_id(label: &str, value: &str) -> String {
    domain_hash(
        "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-STABLE-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Str(value),
        ],
        16,
    )
}

fn fixture_root(label: &str) -> String {
    domain_hash(
        "AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-FIXTURE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn empty_root(label: &str) -> String {
    let leaves: Vec<Value> = Vec::new();
    merkle_root(
        &format!("AUDIT-SECURITY-DASHBOARD-RELEASE-POLICY-BINDING-EMPTY-{label}"),
        &leaves,
    )
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        label,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}

fn map_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    ensure(!value.is_empty(), &format!("{field} must be non-empty"))
}

fn ensure_capacity(current_len: usize, max_len: usize) -> Result<()> {
    ensure(current_len < max_len, "binding record capacity exceeded")
}
