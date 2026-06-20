use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAuditSecurityAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-audit-security-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const IMPORT_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-audit-security-accepted-live-evidence-import-v1";
pub const DEVNET_L2_NETWORK: &str = "nebula-devnet";
pub const DEVNET_MONERO_NETWORK: &str = "monero-devnet";
pub const DEFAULT_MIN_REVIEWERS: usize = 3;
pub const DEFAULT_MIN_INDEPENDENT_SECURITY_REVIEWERS: usize = 2;
pub const DEFAULT_MIN_PRIVACY_NON_LINKAGE_CHECKS: usize = 4;
pub const DEFAULT_MIN_SECURITY_ROOTS: usize = 6;
pub const DEFAULT_MAX_OPEN_CRITICAL_FINDINGS: usize = 0;
pub const DEFAULT_MAX_OPEN_HIGH_FINDINGS: usize = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDomain {
    ForceExitPackage,
    UserEscapeAnswer,
    CanonicalTranscript,
    ReviewerEvidence,
    FindingClosure,
    PrivacyNonLinkage,
    SecurityRoot,
    GovernanceImport,
}

impl EvidenceDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForceExitPackage => "force_exit_package",
            Self::UserEscapeAnswer => "user_escape_answer",
            Self::CanonicalTranscript => "canonical_transcript",
            Self::ReviewerEvidence => "reviewer_evidence",
            Self::FindingClosure => "finding_closure",
            Self::PrivacyNonLinkage => "privacy_non_linkage",
            Self::SecurityRoot => "security_root",
            Self::GovernanceImport => "governance_import",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Rejected,
    Quarantined,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Quarantined => "quarantined",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerRole {
    SecurityAuditor,
    PrivacyReviewer,
    ProtocolReviewer,
    ReleaseGovernor,
    IncidentCommander,
}

impl ReviewerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SecurityAuditor => "security_auditor",
            Self::PrivacyReviewer => "privacy_reviewer",
            Self::ProtocolReviewer => "protocol_reviewer",
            Self::ReleaseGovernor => "release_governor",
            Self::IncidentCommander => "incident_commander",
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

    pub fn is_blocking(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    Open,
    Mitigated,
    Closed,
    RiskAccepted,
    FalsePositive,
}

impl FindingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Mitigated => "mitigated",
            Self::Closed => "closed",
            Self::RiskAccepted => "risk_accepted",
            Self::FalsePositive => "false_positive",
        }
    }

    pub fn is_closed(self) -> bool {
        matches!(
            self,
            Self::Mitigated | Self::Closed | Self::RiskAccepted | Self::FalsePositive
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyCheckKind {
    DepositToNoteUnlinkability,
    EscapeAnswerMetadata,
    NullifierSetSeparation,
    ReviewerRedaction,
    TimingCorrelation,
    ViewKeyCompartment,
}

impl PrivacyCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DepositToNoteUnlinkability => "deposit_to_note_unlinkability",
            Self::EscapeAnswerMetadata => "escape_answer_metadata",
            Self::NullifierSetSeparation => "nullifier_set_separation",
            Self::ReviewerRedaction => "reviewer_redaction",
            Self::TimingCorrelation => "timing_correlation",
            Self::ViewKeyCompartment => "view_key_compartment",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyCheckStatus {
    Passed,
    Failed,
    Inconclusive,
}

impl PrivacyCheckStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityRootKind {
    ForceExitBundle,
    UserEscapeAnswer,
    AuditReport,
    ReviewerQuorum,
    FindingClosure,
    PrivacyNonLinkage,
    ReproducibleBuild,
    GovernanceDecision,
}

impl SecurityRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForceExitBundle => "force_exit_bundle",
            Self::UserEscapeAnswer => "user_escape_answer",
            Self::AuditReport => "audit_report",
            Self::ReviewerQuorum => "reviewer_quorum",
            Self::FindingClosure => "finding_closure",
            Self::PrivacyNonLinkage => "privacy_non_linkage",
            Self::ReproducibleBuild => "reproducible_build",
            Self::GovernanceDecision => "governance_decision",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAnswer {
    Go,
    NoGo,
}

impl GovernanceAnswer {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectionCode {
    MissingEvidence,
    EvidenceRejected,
    ReviewerQuorumMissing,
    SecurityReviewerQuorumMissing,
    OpenBlockingFinding,
    PrivacyNonLinkageFailed,
    PrivacyNonLinkageInsufficient,
    SecurityRootMissing,
    SecurityRootMismatch,
    GovernanceRootMismatch,
    ProductionReleaseDisabled,
}

impl RejectionCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingEvidence => "missing_evidence",
            Self::EvidenceRejected => "evidence_rejected",
            Self::ReviewerQuorumMissing => "reviewer_quorum_missing",
            Self::SecurityReviewerQuorumMissing => "security_reviewer_quorum_missing",
            Self::OpenBlockingFinding => "open_blocking_finding",
            Self::PrivacyNonLinkageFailed => "privacy_non_linkage_failed",
            Self::PrivacyNonLinkageInsufficient => "privacy_non_linkage_insufficient",
            Self::SecurityRootMissing => "security_root_missing",
            Self::SecurityRootMismatch => "security_root_mismatch",
            Self::GovernanceRootMismatch => "governance_root_mismatch",
            Self::ProductionReleaseDisabled => "production_release_disabled",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub import_suite: String,
    pub l2_network: String,
    pub monero_network: String,
    pub min_reviewers: usize,
    pub min_independent_security_reviewers: usize,
    pub min_privacy_non_linkage_checks: usize,
    pub min_security_roots: usize,
    pub max_open_critical_findings: usize,
    pub max_open_high_findings: usize,
    pub require_privacy_non_linkage: bool,
    pub require_security_roots: bool,
    pub production_release_allowed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            import_suite: IMPORT_SUITE.to_string(),
            l2_network: DEVNET_L2_NETWORK.to_string(),
            monero_network: DEVNET_MONERO_NETWORK.to_string(),
            min_reviewers: DEFAULT_MIN_REVIEWERS,
            min_independent_security_reviewers: DEFAULT_MIN_INDEPENDENT_SECURITY_REVIEWERS,
            min_privacy_non_linkage_checks: DEFAULT_MIN_PRIVACY_NON_LINKAGE_CHECKS,
            min_security_roots: DEFAULT_MIN_SECURITY_ROOTS,
            max_open_critical_findings: DEFAULT_MAX_OPEN_CRITICAL_FINDINGS,
            max_open_high_findings: DEFAULT_MAX_OPEN_HIGH_FINDINGS,
            require_privacy_non_linkage: true,
            require_security_roots: true,
            production_release_allowed: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "import_suite": self.import_suite,
            "l2_network": self.l2_network,
            "monero_network": self.monero_network,
            "min_reviewers": self.min_reviewers,
            "min_independent_security_reviewers": self.min_independent_security_reviewers,
            "min_privacy_non_linkage_checks": self.min_privacy_non_linkage_checks,
            "min_security_roots": self.min_security_roots,
            "max_open_critical_findings": self.max_open_critical_findings,
            "max_open_high_findings": self.max_open_high_findings,
            "require_privacy_non_linkage": self.require_privacy_non_linkage,
            "require_security_roots": self.require_security_roots,
            "production_release_allowed": self.production_release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedEvidence {
    pub evidence_id: String,
    pub domain: EvidenceDomain,
    pub status: EvidenceStatus,
    pub label: String,
    pub source_runtime: String,
    pub source_record_id: String,
    pub artifact_root: String,
    pub transcript_root: String,
    pub accepted_by_reviewer_id: String,
    pub accepted_at_height: u64,
    pub redaction_root: String,
    pub public_summary: String,
}

impl AcceptedEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        domain: EvidenceDomain,
        status: EvidenceStatus,
        label: impl Into<String>,
        source_runtime: impl Into<String>,
        source_record_id: impl Into<String>,
        artifact_root: impl Into<String>,
        transcript_root: impl Into<String>,
        accepted_by_reviewer_id: impl Into<String>,
        accepted_at_height: u64,
        redaction_root: impl Into<String>,
        public_summary: impl Into<String>,
    ) -> Self {
        let label = label.into();
        let source_runtime = source_runtime.into();
        let source_record_id = source_record_id.into();
        let artifact_root = artifact_root.into();
        let transcript_root = transcript_root.into();
        let accepted_by_reviewer_id = accepted_by_reviewer_id.into();
        let redaction_root = redaction_root.into();
        let public_summary = public_summary.into();
        let evidence_id = evidence_id(
            domain,
            &source_runtime,
            &source_record_id,
            &artifact_root,
            &transcript_root,
        );
        Self {
            evidence_id,
            domain,
            status,
            label,
            source_runtime,
            source_record_id,
            artifact_root,
            transcript_root,
            accepted_by_reviewer_id,
            accepted_at_height,
            redaction_root,
            public_summary,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "evidence_id": self.evidence_id,
            "domain": self.domain.as_str(),
            "status": self.status.as_str(),
            "label": self.label,
            "source_runtime": self.source_runtime,
            "source_record_id": self.source_record_id,
            "artifact_root": self.artifact_root,
            "transcript_root": self.transcript_root,
            "accepted_by_reviewer_id": self.accepted_by_reviewer_id,
            "accepted_at_height": self.accepted_at_height,
            "redaction_root": self.redaction_root,
            "public_summary": self.public_summary,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("accepted_evidence", &self.public_record())
    }

    pub fn is_accepted(&self) -> bool {
        self.status == EvidenceStatus::Accepted
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerEvidence {
    pub reviewer_id: String,
    pub role: ReviewerRole,
    pub organization_hash: String,
    pub independence_statement_root: String,
    pub evidence_ids: Vec<String>,
    pub reviewer_attestation_root: String,
    pub signature_root: String,
    pub security_reviewer: bool,
    pub privacy_reviewer: bool,
    pub accepted: bool,
}

impl ReviewerEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        role: ReviewerRole,
        organization_hash: impl Into<String>,
        independence_statement_root: impl Into<String>,
        evidence_ids: Vec<String>,
        reviewer_attestation_root: impl Into<String>,
        signature_root: impl Into<String>,
        security_reviewer: bool,
        privacy_reviewer: bool,
        accepted: bool,
    ) -> Self {
        let organization_hash = organization_hash.into();
        let independence_statement_root = independence_statement_root.into();
        let reviewer_attestation_root = reviewer_attestation_root.into();
        let signature_root = signature_root.into();
        let reviewer_id = reviewer_id(
            role,
            &organization_hash,
            &independence_statement_root,
            &reviewer_attestation_root,
        );
        Self {
            reviewer_id,
            role,
            organization_hash,
            independence_statement_root,
            evidence_ids,
            reviewer_attestation_root,
            signature_root,
            security_reviewer,
            privacy_reviewer,
            accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_id": self.reviewer_id,
            "role": self.role.as_str(),
            "organization_hash": self.organization_hash,
            "independence_statement_root": self.independence_statement_root,
            "evidence_ids": self.evidence_ids,
            "reviewer_attestation_root": self.reviewer_attestation_root,
            "signature_root": self.signature_root,
            "security_reviewer": self.security_reviewer,
            "privacy_reviewer": self.privacy_reviewer,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("reviewer_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindingClosure {
    pub finding_id: String,
    pub severity: FindingSeverity,
    pub status: FindingStatus,
    pub title_hash: String,
    pub evidence_ids: Vec<String>,
    pub closure_root: String,
    pub mitigation_root: String,
    pub regression_proof_root: String,
    pub residual_risk_root: String,
    pub approved_by_reviewer_id: String,
}

impl FindingClosure {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        severity: FindingSeverity,
        status: FindingStatus,
        title_hash: impl Into<String>,
        evidence_ids: Vec<String>,
        closure_root: impl Into<String>,
        mitigation_root: impl Into<String>,
        regression_proof_root: impl Into<String>,
        residual_risk_root: impl Into<String>,
        approved_by_reviewer_id: impl Into<String>,
    ) -> Self {
        let title_hash = title_hash.into();
        let closure_root = closure_root.into();
        let mitigation_root = mitigation_root.into();
        let regression_proof_root = regression_proof_root.into();
        let residual_risk_root = residual_risk_root.into();
        let approved_by_reviewer_id = approved_by_reviewer_id.into();
        let finding_id = finding_id(severity, &title_hash, &closure_root);
        Self {
            finding_id,
            severity,
            status,
            title_hash,
            evidence_ids,
            closure_root,
            mitigation_root,
            regression_proof_root,
            residual_risk_root,
            approved_by_reviewer_id,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "finding_id": self.finding_id,
            "severity": self.severity.as_str(),
            "status": self.status.as_str(),
            "title_hash": self.title_hash,
            "evidence_ids": self.evidence_ids,
            "closure_root": self.closure_root,
            "mitigation_root": self.mitigation_root,
            "regression_proof_root": self.regression_proof_root,
            "residual_risk_root": self.residual_risk_root,
            "approved_by_reviewer_id": self.approved_by_reviewer_id,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("finding_closure", &self.public_record())
    }

    pub fn is_open_blocker(&self) -> bool {
        self.severity.is_blocking() && !self.status.is_closed()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyNonLinkageCheck {
    pub check_id: String,
    pub kind: PrivacyCheckKind,
    pub status: PrivacyCheckStatus,
    pub subject_commitment_root: String,
    pub nullifier_domain_root: String,
    pub redaction_root: String,
    pub leakage_bound_root: String,
    pub reviewer_id: String,
    pub evidence_ids: Vec<String>,
}

impl PrivacyNonLinkageCheck {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        kind: PrivacyCheckKind,
        status: PrivacyCheckStatus,
        subject_commitment_root: impl Into<String>,
        nullifier_domain_root: impl Into<String>,
        redaction_root: impl Into<String>,
        leakage_bound_root: impl Into<String>,
        reviewer_id: impl Into<String>,
        evidence_ids: Vec<String>,
    ) -> Self {
        let subject_commitment_root = subject_commitment_root.into();
        let nullifier_domain_root = nullifier_domain_root.into();
        let redaction_root = redaction_root.into();
        let leakage_bound_root = leakage_bound_root.into();
        let reviewer_id = reviewer_id.into();
        let check_id = privacy_check_id(
            kind,
            &subject_commitment_root,
            &nullifier_domain_root,
            &leakage_bound_root,
        );
        Self {
            check_id,
            kind,
            status,
            subject_commitment_root,
            nullifier_domain_root,
            redaction_root,
            leakage_bound_root,
            reviewer_id,
            evidence_ids,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "check_id": self.check_id,
            "kind": self.kind.as_str(),
            "status": self.status.as_str(),
            "subject_commitment_root": self.subject_commitment_root,
            "nullifier_domain_root": self.nullifier_domain_root,
            "redaction_root": self.redaction_root,
            "leakage_bound_root": self.leakage_bound_root,
            "reviewer_id": self.reviewer_id,
            "evidence_ids": self.evidence_ids,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("privacy_non_linkage_check", &self.public_record())
    }

    pub fn passed(&self) -> bool {
        self.status == PrivacyCheckStatus::Passed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityRootEvidence {
    pub root_id: String,
    pub kind: SecurityRootKind,
    pub claimed_root: String,
    pub observed_root: String,
    pub source_evidence_id: String,
    pub verifier_id: String,
    pub matches_claim: bool,
    pub imported_into_governance: bool,
}

impl SecurityRootEvidence {
    pub fn new(
        kind: SecurityRootKind,
        claimed_root: impl Into<String>,
        observed_root: impl Into<String>,
        source_evidence_id: impl Into<String>,
        verifier_id: impl Into<String>,
        imported_into_governance: bool,
    ) -> Self {
        let claimed_root = claimed_root.into();
        let observed_root = observed_root.into();
        let source_evidence_id = source_evidence_id.into();
        let verifier_id = verifier_id.into();
        let matches_claim = claimed_root == observed_root;
        let root_id = security_root_id(kind, &claimed_root, &observed_root, &source_evidence_id);
        Self {
            root_id,
            kind,
            claimed_root,
            observed_root,
            source_evidence_id,
            verifier_id,
            matches_claim,
            imported_into_governance,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "root_id": self.root_id,
            "kind": self.kind.as_str(),
            "claimed_root": self.claimed_root,
            "observed_root": self.observed_root,
            "source_evidence_id": self.source_evidence_id,
            "verifier_id": self.verifier_id,
            "matches_claim": self.matches_claim,
            "imported_into_governance": self.imported_into_governance,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("security_root_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RejectionReason {
    pub rejection_id: String,
    pub code: RejectionCode,
    pub subject_id: String,
    pub detail: String,
    pub fail_closed: bool,
}

impl RejectionReason {
    pub fn new(
        code: RejectionCode,
        subject_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let subject_id = subject_id.into();
        let detail = detail.into();
        let rejection_id = rejection_id(code, &subject_id, &detail);
        Self {
            rejection_id,
            code,
            subject_id,
            detail,
            fail_closed: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "rejection_id": self.rejection_id,
            "code": self.code.as_str(),
            "subject_id": self.subject_id,
            "detail": self.detail,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rejection_reason", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceCounters {
    pub evidence_total: usize,
    pub accepted_evidence: usize,
    pub rejected_evidence: usize,
    pub quarantined_evidence: usize,
    pub reviewer_total: usize,
    pub accepted_reviewers: usize,
    pub security_reviewers: usize,
    pub privacy_reviewers: usize,
    pub findings_total: usize,
    pub open_critical_findings: usize,
    pub open_high_findings: usize,
    pub closed_blocking_findings: usize,
    pub privacy_checks_total: usize,
    pub privacy_checks_passed: usize,
    pub privacy_checks_failed: usize,
    pub security_roots_total: usize,
    pub security_roots_matched: usize,
    pub security_roots_imported: usize,
    pub rejection_count: usize,
}

impl EvidenceCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "evidence_total": self.evidence_total,
            "accepted_evidence": self.accepted_evidence,
            "rejected_evidence": self.rejected_evidence,
            "quarantined_evidence": self.quarantined_evidence,
            "reviewer_total": self.reviewer_total,
            "accepted_reviewers": self.accepted_reviewers,
            "security_reviewers": self.security_reviewers,
            "privacy_reviewers": self.privacy_reviewers,
            "findings_total": self.findings_total,
            "open_critical_findings": self.open_critical_findings,
            "open_high_findings": self.open_high_findings,
            "closed_blocking_findings": self.closed_blocking_findings,
            "privacy_checks_total": self.privacy_checks_total,
            "privacy_checks_passed": self.privacy_checks_passed,
            "privacy_checks_failed": self.privacy_checks_failed,
            "security_roots_total": self.security_roots_total,
            "security_roots_matched": self.security_roots_matched,
            "security_roots_imported": self.security_roots_imported,
            "rejection_count": self.rejection_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("evidence_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeRoots {
    pub config_root: String,
    pub evidence_root: String,
    pub reviewer_root: String,
    pub finding_root: String,
    pub privacy_non_linkage_root: String,
    pub security_root: String,
    pub rejection_root: String,
    pub counters_root: String,
    pub governance_import_root: String,
    pub state_root: String,
}

impl RuntimeRoots {
    pub fn empty(config: &Config) -> Self {
        let config_root = config.state_root();
        let evidence_root = merkle_root("AUDIT-SECURITY-LIVE-EVIDENCE-EMPTY", &[]);
        let reviewer_root = merkle_root("AUDIT-SECURITY-LIVE-REVIEWER-EMPTY", &[]);
        let finding_root = merkle_root("AUDIT-SECURITY-LIVE-FINDING-EMPTY", &[]);
        let privacy_non_linkage_root = merkle_root("AUDIT-SECURITY-LIVE-PRIVACY-EMPTY", &[]);
        let security_root = merkle_root("AUDIT-SECURITY-LIVE-SECURITY-ROOT-EMPTY", &[]);
        let rejection_root = merkle_root("AUDIT-SECURITY-LIVE-REJECTION-EMPTY", &[]);
        let counters_root = EvidenceCounters::default().state_root();
        let governance_import_root = merkle_root("AUDIT-SECURITY-LIVE-GOVERNANCE-EMPTY", &[]);
        let state_root = state_root_from_parts(
            &config_root,
            &evidence_root,
            &reviewer_root,
            &finding_root,
            &privacy_non_linkage_root,
            &security_root,
            &rejection_root,
            &counters_root,
            &governance_import_root,
        );
        Self {
            config_root,
            evidence_root,
            reviewer_root,
            finding_root,
            privacy_non_linkage_root,
            security_root,
            rejection_root,
            counters_root,
            governance_import_root,
            state_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "evidence_root": self.evidence_root,
            "reviewer_root": self.reviewer_root,
            "finding_root": self.finding_root,
            "privacy_non_linkage_root": self.privacy_non_linkage_root,
            "security_root": self.security_root,
            "rejection_root": self.rejection_root,
            "counters_root": self.counters_root,
            "governance_import_root": self.governance_import_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GovernanceImport {
    pub import_id: String,
    pub answer: GovernanceAnswer,
    pub go_live_allowed: bool,
    pub force_exit_package_accepted: bool,
    pub user_escape_answer_accepted: bool,
    pub reviewer_quorum_met: bool,
    pub security_reviewer_quorum_met: bool,
    pub findings_closed: bool,
    pub privacy_non_linkage_accepted: bool,
    pub security_roots_accepted: bool,
    pub expected_security_root: String,
    pub observed_security_root: String,
    pub rejection_root: String,
    pub state_root: String,
}

impl GovernanceImport {
    pub fn public_record(&self) -> Value {
        json!({
            "import_id": self.import_id,
            "answer": self.answer.as_str(),
            "go_live_allowed": self.go_live_allowed,
            "force_exit_package_accepted": self.force_exit_package_accepted,
            "user_escape_answer_accepted": self.user_escape_answer_accepted,
            "reviewer_quorum_met": self.reviewer_quorum_met,
            "security_reviewer_quorum_met": self.security_reviewer_quorum_met,
            "findings_closed": self.findings_closed,
            "privacy_non_linkage_accepted": self.privacy_non_linkage_accepted,
            "security_roots_accepted": self.security_roots_accepted,
            "expected_security_root": self.expected_security_root,
            "observed_security_root": self.observed_security_root,
            "rejection_root": self.rejection_root,
            "state_root": self.state_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("governance_import", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub evidence: BTreeMap<String, AcceptedEvidence>,
    pub reviewers: BTreeMap<String, ReviewerEvidence>,
    pub findings: BTreeMap<String, FindingClosure>,
    pub privacy_checks: BTreeMap<String, PrivacyNonLinkageCheck>,
    pub security_roots: BTreeMap<String, SecurityRootEvidence>,
    pub rejections: BTreeMap<String, RejectionReason>,
    pub governance_import: GovernanceImport,
    pub counters: EvidenceCounters,
    pub roots: RuntimeRoots,
}

impl State {
    pub fn new(config: Config) -> Self {
        let roots = RuntimeRoots::empty(&config);
        let governance_import = GovernanceImport {
            import_id: domain_hash(
                "AUDIT-SECURITY-LIVE-GOVERNANCE-IMPORT-EMPTY-ID",
                &[HashPart::Str(&roots.state_root)],
                32,
            ),
            answer: GovernanceAnswer::NoGo,
            go_live_allowed: false,
            force_exit_package_accepted: false,
            user_escape_answer_accepted: false,
            reviewer_quorum_met: false,
            security_reviewer_quorum_met: false,
            findings_closed: false,
            privacy_non_linkage_accepted: false,
            security_roots_accepted: false,
            expected_security_root: roots.security_root.clone(),
            observed_security_root: roots.security_root.clone(),
            rejection_root: roots.rejection_root.clone(),
            state_root: roots.state_root.clone(),
        };
        Self {
            config,
            evidence: BTreeMap::new(),
            reviewers: BTreeMap::new(),
            findings: BTreeMap::new(),
            privacy_checks: BTreeMap::new(),
            security_roots: BTreeMap::new(),
            rejections: BTreeMap::new(),
            governance_import,
            counters: EvidenceCounters::default(),
            roots,
        }
    }

    pub fn devnet() -> Self {
        let mut state = Self::new(Config::devnet());
        state.refresh();
        state
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "evidence": self.evidence.values().map(AcceptedEvidence::public_record).collect::<Vec<_>>(),
            "reviewers": self.reviewers.values().map(ReviewerEvidence::public_record).collect::<Vec<_>>(),
            "findings": self.findings.values().map(FindingClosure::public_record).collect::<Vec<_>>(),
            "privacy_checks": self.privacy_checks.values().map(PrivacyNonLinkageCheck::public_record).collect::<Vec<_>>(),
            "security_roots": self.security_roots.values().map(SecurityRootEvidence::public_record).collect::<Vec<_>>(),
            "rejections": self.rejections.values().map(RejectionReason::public_record).collect::<Vec<_>>(),
            "governance_import": self.governance_import.public_record(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root.clone()
    }

    pub fn import_evidence(&mut self, evidence: AcceptedEvidence) -> Result<String> {
        if evidence.artifact_root.is_empty() || evidence.transcript_root.is_empty() {
            let rejection = RejectionReason::new(
                RejectionCode::MissingEvidence,
                evidence.evidence_id.clone(),
                "artifact_root and transcript_root are required for accepted live evidence import",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.refresh();
            return Err(id);
        }
        if !evidence.is_accepted() {
            let rejection = RejectionReason::new(
                RejectionCode::EvidenceRejected,
                evidence.evidence_id.clone(),
                "only accepted live evidence may be imported into final governance",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.evidence.insert(evidence.evidence_id.clone(), evidence);
            self.refresh();
            return Err(id);
        }
        let id = evidence.evidence_id.clone();
        self.evidence.insert(id.clone(), evidence);
        self.refresh();
        Ok(id)
    }

    pub fn import_reviewer_evidence(&mut self, reviewer: ReviewerEvidence) -> Result<String> {
        if reviewer.evidence_ids.is_empty() {
            let rejection = RejectionReason::new(
                RejectionCode::MissingEvidence,
                reviewer.reviewer_id.clone(),
                "reviewer evidence must bind at least one accepted evidence id",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.refresh();
            return Err(id);
        }
        let mut missing = Vec::new();
        for evidence_id in &reviewer.evidence_ids {
            if !self.evidence.contains_key(evidence_id) {
                missing.push(evidence_id.clone());
            }
        }
        if !missing.is_empty() {
            let rejection = RejectionReason::new(
                RejectionCode::MissingEvidence,
                reviewer.reviewer_id.clone(),
                format!(
                    "reviewer references evidence ids not imported: {}",
                    missing.join(",")
                ),
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.refresh();
            return Err(id);
        }
        let id = reviewer.reviewer_id.clone();
        self.reviewers.insert(id.clone(), reviewer);
        self.refresh();
        Ok(id)
    }

    pub fn import_finding_closure(&mut self, finding: FindingClosure) -> Result<String> {
        if finding.evidence_ids.is_empty() {
            let rejection = RejectionReason::new(
                RejectionCode::MissingEvidence,
                finding.finding_id.clone(),
                "finding closure must reference mitigation or closure evidence",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.refresh();
            return Err(id);
        }
        if finding.is_open_blocker() {
            let rejection = RejectionReason::new(
                RejectionCode::OpenBlockingFinding,
                finding.finding_id.clone(),
                "critical or high finding remains open and blocks final governance import",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.findings.insert(finding.finding_id.clone(), finding);
            self.refresh();
            return Err(id);
        }
        let id = finding.finding_id.clone();
        self.findings.insert(id.clone(), finding);
        self.refresh();
        Ok(id)
    }

    pub fn import_privacy_check(&mut self, check: PrivacyNonLinkageCheck) -> Result<String> {
        if !check.passed() {
            let rejection = RejectionReason::new(
                RejectionCode::PrivacyNonLinkageFailed,
                check.check_id.clone(),
                "privacy non-linkage check did not pass",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.privacy_checks.insert(check.check_id.clone(), check);
            self.refresh();
            return Err(id);
        }
        let id = check.check_id.clone();
        self.privacy_checks.insert(id.clone(), check);
        self.refresh();
        Ok(id)
    }

    pub fn import_security_root(&mut self, security_root: SecurityRootEvidence) -> Result<String> {
        if !security_root.matches_claim {
            let rejection = RejectionReason::new(
                RejectionCode::SecurityRootMismatch,
                security_root.root_id.clone(),
                "claimed security root does not match observed root",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.security_roots
                .insert(security_root.root_id.clone(), security_root);
            self.refresh();
            return Err(id);
        }
        if !security_root.imported_into_governance {
            let rejection = RejectionReason::new(
                RejectionCode::SecurityRootMissing,
                security_root.root_id.clone(),
                "security root is verified but not marked imported into governance",
            );
            let id = rejection.rejection_id.clone();
            self.rejections.insert(id.clone(), rejection);
            self.security_roots
                .insert(security_root.root_id.clone(), security_root);
            self.refresh();
            return Err(id);
        }
        let id = security_root.root_id.clone();
        self.security_roots.insert(id.clone(), security_root);
        self.refresh();
        Ok(id)
    }

    pub fn finalize_governance_import(&mut self) -> GovernanceImport {
        self.refresh();
        self.governance_import.clone()
    }

    pub fn refresh(&mut self) {
        self.counters = self.compute_counters();
        self.rebuild_policy_rejections();
        self.counters = self.compute_counters();
        self.roots = self.compute_roots();
        self.governance_import = self.compute_governance_import();
        self.roots.governance_import_root = self.governance_import.state_root();
        self.roots.state_root = state_root_from_parts(
            &self.roots.config_root,
            &self.roots.evidence_root,
            &self.roots.reviewer_root,
            &self.roots.finding_root,
            &self.roots.privacy_non_linkage_root,
            &self.roots.security_root,
            &self.roots.rejection_root,
            &self.roots.counters_root,
            &self.roots.governance_import_root,
        );
        self.governance_import.state_root = self.roots.state_root.clone();
    }

    fn compute_governance_import(&self) -> GovernanceImport {
        let force_exit_package_accepted =
            self.has_accepted_domain(EvidenceDomain::ForceExitPackage);
        let user_escape_answer_accepted =
            self.has_accepted_domain(EvidenceDomain::UserEscapeAnswer);
        let reviewer_quorum_met = self.counters.accepted_reviewers >= self.config.min_reviewers;
        let security_reviewer_quorum_met =
            self.counters.security_reviewers >= self.config.min_independent_security_reviewers;
        let findings_closed = self.counters.open_critical_findings
            <= self.config.max_open_critical_findings
            && self.counters.open_high_findings <= self.config.max_open_high_findings;
        let privacy_non_linkage_accepted = !self.config.require_privacy_non_linkage
            || self.counters.privacy_checks_passed >= self.config.min_privacy_non_linkage_checks;
        let security_roots_accepted = !self.config.require_security_roots
            || (self.counters.security_roots_imported >= self.config.min_security_roots
                && self.counters.security_roots_matched == self.counters.security_roots_total);
        let observed_security_root = self.roots.security_root.clone();
        let expected_security_root = merkle_root(
            "AUDIT-SECURITY-LIVE-GOVERNANCE-EXPECTED-SECURITY-ROOT",
            &self
                .security_roots
                .values()
                .filter(|root| root.matches_claim && root.imported_into_governance)
                .map(SecurityRootEvidence::public_record)
                .collect::<Vec<_>>(),
        );
        let security_root_match = observed_security_root == expected_security_root;
        let no_rejections = self.rejections.is_empty();
        let go_live_allowed = self.config.production_release_allowed
            && force_exit_package_accepted
            && user_escape_answer_accepted
            && reviewer_quorum_met
            && security_reviewer_quorum_met
            && findings_closed
            && privacy_non_linkage_accepted
            && security_roots_accepted
            && security_root_match
            && no_rejections;
        let answer = if go_live_allowed {
            GovernanceAnswer::Go
        } else {
            GovernanceAnswer::NoGo
        };
        let import_id = domain_hash(
            "AUDIT-SECURITY-LIVE-GOVERNANCE-IMPORT-ID",
            &[
                HashPart::Str(answer.as_str()),
                HashPart::Str(&self.roots.evidence_root),
                HashPart::Str(&self.roots.reviewer_root),
                HashPart::Str(&self.roots.finding_root),
                HashPart::Str(&self.roots.privacy_non_linkage_root),
                HashPart::Str(&observed_security_root),
                HashPart::Str(&self.roots.rejection_root),
            ],
            32,
        );
        GovernanceImport {
            import_id,
            answer,
            go_live_allowed,
            force_exit_package_accepted,
            user_escape_answer_accepted,
            reviewer_quorum_met,
            security_reviewer_quorum_met,
            findings_closed,
            privacy_non_linkage_accepted,
            security_roots_accepted: security_roots_accepted && security_root_match,
            expected_security_root,
            observed_security_root,
            rejection_root: self.roots.rejection_root.clone(),
            state_root: self.roots.state_root.clone(),
        }
    }

    fn compute_counters(&self) -> EvidenceCounters {
        let mut counters = EvidenceCounters::default();
        counters.evidence_total = self.evidence.len();
        counters.reviewer_total = self.reviewers.len();
        counters.findings_total = self.findings.len();
        counters.privacy_checks_total = self.privacy_checks.len();
        counters.security_roots_total = self.security_roots.len();
        counters.rejection_count = self.rejections.len();
        for evidence in self.evidence.values() {
            match evidence.status {
                EvidenceStatus::Accepted => counters.accepted_evidence += 1,
                EvidenceStatus::Rejected => counters.rejected_evidence += 1,
                EvidenceStatus::Quarantined => counters.quarantined_evidence += 1,
            }
        }
        for reviewer in self.reviewers.values() {
            if reviewer.accepted {
                counters.accepted_reviewers += 1;
            }
            if reviewer.accepted && reviewer.security_reviewer {
                counters.security_reviewers += 1;
            }
            if reviewer.accepted && reviewer.privacy_reviewer {
                counters.privacy_reviewers += 1;
            }
        }
        for finding in self.findings.values() {
            if finding.severity == FindingSeverity::Critical && !finding.status.is_closed() {
                counters.open_critical_findings += 1;
            }
            if finding.severity == FindingSeverity::High && !finding.status.is_closed() {
                counters.open_high_findings += 1;
            }
            if finding.severity.is_blocking() && finding.status.is_closed() {
                counters.closed_blocking_findings += 1;
            }
        }
        for check in self.privacy_checks.values() {
            if check.passed() {
                counters.privacy_checks_passed += 1;
            }
            if check.status == PrivacyCheckStatus::Failed {
                counters.privacy_checks_failed += 1;
            }
        }
        for root in self.security_roots.values() {
            if root.matches_claim {
                counters.security_roots_matched += 1;
            }
            if root.imported_into_governance {
                counters.security_roots_imported += 1;
            }
        }
        counters
    }

    fn compute_roots(&self) -> RuntimeRoots {
        let config_root = self.config.state_root();
        let evidence_root = merkle_root(
            "AUDIT-SECURITY-LIVE-EVIDENCE",
            &self
                .evidence
                .values()
                .map(AcceptedEvidence::public_record)
                .collect::<Vec<_>>(),
        );
        let reviewer_root = merkle_root(
            "AUDIT-SECURITY-LIVE-REVIEWER",
            &self
                .reviewers
                .values()
                .map(ReviewerEvidence::public_record)
                .collect::<Vec<_>>(),
        );
        let finding_root = merkle_root(
            "AUDIT-SECURITY-LIVE-FINDING",
            &self
                .findings
                .values()
                .map(FindingClosure::public_record)
                .collect::<Vec<_>>(),
        );
        let privacy_non_linkage_root = merkle_root(
            "AUDIT-SECURITY-LIVE-PRIVACY-NON-LINKAGE",
            &self
                .privacy_checks
                .values()
                .map(PrivacyNonLinkageCheck::public_record)
                .collect::<Vec<_>>(),
        );
        let accepted_security_roots = self
            .security_roots
            .values()
            .filter(|root| root.matches_claim && root.imported_into_governance)
            .map(SecurityRootEvidence::public_record)
            .collect::<Vec<_>>();
        let security_root = merkle_root(
            "AUDIT-SECURITY-LIVE-GOVERNANCE-EXPECTED-SECURITY-ROOT",
            &accepted_security_roots,
        );
        let rejection_root = merkle_root(
            "AUDIT-SECURITY-LIVE-REJECTION",
            &self
                .rejections
                .values()
                .map(RejectionReason::public_record)
                .collect::<Vec<_>>(),
        );
        let counters_root = self.counters.state_root();
        let governance_import_root = self.governance_import.state_root();
        let state_root = state_root_from_parts(
            &config_root,
            &evidence_root,
            &reviewer_root,
            &finding_root,
            &privacy_non_linkage_root,
            &security_root,
            &rejection_root,
            &counters_root,
            &governance_import_root,
        );
        RuntimeRoots {
            config_root,
            evidence_root,
            reviewer_root,
            finding_root,
            privacy_non_linkage_root,
            security_root,
            rejection_root,
            counters_root,
            governance_import_root,
            state_root,
        }
    }

    fn rebuild_policy_rejections(&mut self) {
        self.rejections
            .retain(|_, rejection| !is_policy_rejection(rejection.code));
        if !self.config.production_release_allowed {
            self.insert_policy_rejection(
                RejectionCode::ProductionReleaseDisabled,
                "config",
                "production release is disabled for this devnet import runtime",
            );
        }
        if !self.has_accepted_domain(EvidenceDomain::ForceExitPackage) {
            self.insert_policy_rejection(
                RejectionCode::MissingEvidence,
                EvidenceDomain::ForceExitPackage.as_str(),
                "accepted force exit package evidence is required",
            );
        }
        if !self.has_accepted_domain(EvidenceDomain::UserEscapeAnswer) {
            self.insert_policy_rejection(
                RejectionCode::MissingEvidence,
                EvidenceDomain::UserEscapeAnswer.as_str(),
                "accepted user escape answer evidence is required",
            );
        }
        if self.counters.accepted_reviewers < self.config.min_reviewers {
            self.insert_policy_rejection(
                RejectionCode::ReviewerQuorumMissing,
                "reviewer_quorum",
                "accepted reviewer quorum is below configured threshold",
            );
        }
        if self.counters.security_reviewers < self.config.min_independent_security_reviewers {
            self.insert_policy_rejection(
                RejectionCode::SecurityReviewerQuorumMissing,
                "security_reviewer_quorum",
                "independent security reviewer quorum is below configured threshold",
            );
        }
        if self.counters.open_critical_findings > self.config.max_open_critical_findings
            || self.counters.open_high_findings > self.config.max_open_high_findings
        {
            self.insert_policy_rejection(
                RejectionCode::OpenBlockingFinding,
                "finding_closure",
                "open critical or high finding exceeds configured tolerance",
            );
        }
        if self.config.require_privacy_non_linkage
            && self.counters.privacy_checks_passed < self.config.min_privacy_non_linkage_checks
        {
            self.insert_policy_rejection(
                RejectionCode::PrivacyNonLinkageInsufficient,
                "privacy_non_linkage",
                "passed privacy non-linkage checks are below configured threshold",
            );
        }
        if self.counters.privacy_checks_failed > 0 {
            self.insert_policy_rejection(
                RejectionCode::PrivacyNonLinkageFailed,
                "privacy_non_linkage",
                "one or more privacy non-linkage checks failed",
            );
        }
        if self.config.require_security_roots
            && self.counters.security_roots_imported < self.config.min_security_roots
        {
            self.insert_policy_rejection(
                RejectionCode::SecurityRootMissing,
                "security_roots",
                "imported security roots are below configured threshold",
            );
        }
        if self.counters.security_roots_total != self.counters.security_roots_matched {
            self.insert_policy_rejection(
                RejectionCode::SecurityRootMismatch,
                "security_roots",
                "one or more security roots did not match claimed roots",
            );
        }
    }

    fn insert_policy_rejection(
        &mut self,
        code: RejectionCode,
        subject_id: impl Into<String>,
        detail: impl Into<String>,
    ) {
        let rejection = RejectionReason::new(code, subject_id, detail);
        self.rejections
            .insert(rejection.rejection_id.clone(), rejection);
    }

    fn has_accepted_domain(&self, domain: EvidenceDomain) -> bool {
        self.evidence
            .values()
            .any(|evidence| evidence.domain == domain && evidence.is_accepted())
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record(state: &State) -> Value {
    state.public_record()
}

pub fn state_root(state: &State) -> String {
    state.state_root()
}

pub fn validate_final_governance_import(state: &State) -> Result<GovernanceImport> {
    if state.governance_import.go_live_allowed {
        Ok(state.governance_import.clone())
    } else {
        Err(state.roots.rejection_root.clone())
    }
}

pub fn accepted_evidence_root(evidence: &[AcceptedEvidence]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-EVIDENCE",
        &evidence
            .iter()
            .map(AcceptedEvidence::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn reviewer_quorum_root(reviewers: &[ReviewerEvidence]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-REVIEWER",
        &reviewers
            .iter()
            .map(ReviewerEvidence::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn finding_closure_root(findings: &[FindingClosure]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-FINDING",
        &findings
            .iter()
            .map(FindingClosure::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn privacy_non_linkage_root(checks: &[PrivacyNonLinkageCheck]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-PRIVACY-NON-LINKAGE",
        &checks
            .iter()
            .map(PrivacyNonLinkageCheck::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn security_roots_root(roots: &[SecurityRootEvidence]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-GOVERNANCE-EXPECTED-SECURITY-ROOT",
        &roots
            .iter()
            .filter(|root| root.matches_claim && root.imported_into_governance)
            .map(SecurityRootEvidence::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn rejection_root(rejections: &[RejectionReason]) -> String {
    merkle_root(
        "AUDIT-SECURITY-LIVE-REJECTION",
        &rejections
            .iter()
            .map(RejectionReason::public_record)
            .collect::<Vec<_>>(),
    )
}

pub fn evidence_domain_coverage(evidence: &BTreeMap<String, AcceptedEvidence>) -> BTreeSet<String> {
    evidence
        .values()
        .filter(|item| item.is_accepted())
        .map(|item| item.domain.as_str().to_string())
        .collect()
}

fn record_root(domain: &str, record: &Value) -> String {
    domain_hash(
        &format!("AUDIT-SECURITY-LIVE-EVIDENCE-IMPORT-{domain}"),
        &[HashPart::Json(record)],
        32,
    )
}

fn state_root_from_parts(
    config_root: &str,
    evidence_root: &str,
    reviewer_root: &str,
    finding_root: &str,
    privacy_non_linkage_root: &str,
    security_root: &str,
    rejection_root: &str,
    counters_root: &str,
    governance_import_root: &str,
) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-EVIDENCE-IMPORT-STATE",
        &[
            HashPart::Str(config_root),
            HashPart::Str(evidence_root),
            HashPart::Str(reviewer_root),
            HashPart::Str(finding_root),
            HashPart::Str(privacy_non_linkage_root),
            HashPart::Str(security_root),
            HashPart::Str(rejection_root),
            HashPart::Str(counters_root),
            HashPart::Str(governance_import_root),
        ],
        32,
    )
}

fn evidence_id(
    domain: EvidenceDomain,
    source_runtime: &str,
    source_record_id: &str,
    artifact_root: &str,
    transcript_root: &str,
) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-EVIDENCE-ID",
        &[
            HashPart::Str(domain.as_str()),
            HashPart::Str(source_runtime),
            HashPart::Str(source_record_id),
            HashPart::Str(artifact_root),
            HashPart::Str(transcript_root),
        ],
        32,
    )
}

fn reviewer_id(
    role: ReviewerRole,
    organization_hash: &str,
    independence_statement_root: &str,
    reviewer_attestation_root: &str,
) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-REVIEWER-ID",
        &[
            HashPart::Str(role.as_str()),
            HashPart::Str(organization_hash),
            HashPart::Str(independence_statement_root),
            HashPart::Str(reviewer_attestation_root),
        ],
        32,
    )
}

fn finding_id(severity: FindingSeverity, title_hash: &str, closure_root: &str) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-FINDING-ID",
        &[
            HashPart::Str(severity.as_str()),
            HashPart::Str(title_hash),
            HashPart::Str(closure_root),
        ],
        32,
    )
}

fn privacy_check_id(
    kind: PrivacyCheckKind,
    subject_commitment_root: &str,
    nullifier_domain_root: &str,
    leakage_bound_root: &str,
) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-PRIVACY-CHECK-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(subject_commitment_root),
            HashPart::Str(nullifier_domain_root),
            HashPart::Str(leakage_bound_root),
        ],
        32,
    )
}

fn security_root_id(
    kind: SecurityRootKind,
    claimed_root: &str,
    observed_root: &str,
    source_evidence_id: &str,
) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-SECURITY-ROOT-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::Str(claimed_root),
            HashPart::Str(observed_root),
            HashPart::Str(source_evidence_id),
        ],
        32,
    )
}

fn rejection_id(code: RejectionCode, subject_id: &str, detail: &str) -> String {
    domain_hash(
        "AUDIT-SECURITY-LIVE-REJECTION-ID",
        &[
            HashPart::Str(code.as_str()),
            HashPart::Str(subject_id),
            HashPart::Str(detail),
        ],
        32,
    )
}

fn is_policy_rejection(code: RejectionCode) -> bool {
    matches!(
        code,
        RejectionCode::MissingEvidence
            | RejectionCode::ReviewerQuorumMissing
            | RejectionCode::SecurityReviewerQuorumMissing
            | RejectionCode::OpenBlockingFinding
            | RejectionCode::PrivacyNonLinkageFailed
            | RejectionCode::PrivacyNonLinkageInsufficient
            | RejectionCode::SecurityRootMissing
            | RejectionCode::SecurityRootMismatch
            | RejectionCode::GovernanceRootMismatch
            | RejectionCode::ProductionReleaseDisabled
    )
}
