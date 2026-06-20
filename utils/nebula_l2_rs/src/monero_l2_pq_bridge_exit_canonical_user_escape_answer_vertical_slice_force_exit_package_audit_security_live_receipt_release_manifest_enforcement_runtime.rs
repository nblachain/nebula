use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAuditSecurityLiveReceiptReleaseManifestEnforcementRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-audit-security-live-receipt-release-manifest-enforcement-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_MANIFEST_SUITE: &str =
    "monero-l2-pq-force-exit-audit-security-live-receipt-release-manifest-enforcement-v1";
pub const DEVNET_L2_NETWORK: &str = "nebula-devnet";
pub const DEVNET_MONERO_NETWORK: &str = "monero-devnet";
pub const DEFAULT_RELEASE_EPOCH: u64 = 79;
pub const DEFAULT_MIN_REVIEWER_QUORUM_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_INDEPENDENT_REVIEWERS: u64 = 3;
pub const DEFAULT_REQUIRED_SCOPE_COUNT: usize = 10;
pub const DEFAULT_CURRENT_HEIGHT: u64 = 4_260_224;
pub const DEFAULT_MAX_RECEIPT_AGE_BLOCKS: u64 = 144;
pub const DEFAULT_RELEASE_HOLD_EXPORT_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditScope {
    CustodyLockReleaseAuthority,
    UserEscapeAnswer,
    CanonicalForceExitPackage,
    PqAuthorization,
    MoneroFinality,
    PrivacyNonLinkage,
    WatcherLiveness,
    AdversarialChallenge,
    ReceiptIngestion,
    ReleaseManifestPolicy,
}

impl AuditScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustodyLockReleaseAuthority => "custody_lock_release_authority",
            Self::UserEscapeAnswer => "user_escape_answer",
            Self::CanonicalForceExitPackage => "canonical_force_exit_package",
            Self::PqAuthorization => "pq_authorization",
            Self::MoneroFinality => "monero_finality",
            Self::PrivacyNonLinkage => "privacy_non_linkage",
            Self::WatcherLiveness => "watcher_liveness",
            Self::AdversarialChallenge => "adversarial_challenge",
            Self::ReceiptIngestion => "receipt_ingestion",
            Self::ReleaseManifestPolicy => "release_manifest_policy",
        }
    }

    pub fn all_required() -> Vec<Self> {
        vec![
            Self::CustodyLockReleaseAuthority,
            Self::UserEscapeAnswer,
            Self::CanonicalForceExitPackage,
            Self::PqAuthorization,
            Self::MoneroFinality,
            Self::PrivacyNonLinkage,
            Self::WatcherLiveness,
            Self::AdversarialChallenge,
            Self::ReceiptIngestion,
            Self::ReleaseManifestPolicy,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCoverageStatus {
    Covered,
    PartiallyCovered,
    Missing,
}

impl ScopeCoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::PartiallyCovered => "partially_covered",
            Self::Missing => "missing",
        }
    }

    pub fn is_covered(self) -> bool {
        self == Self::Covered
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerRole {
    ProtocolSecurity,
    PrivacyEngineering,
    MoneroBridgeOperations,
    PqCryptography,
    ReleaseManager,
    IndependentAuditor,
}

impl ReviewerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProtocolSecurity => "protocol_security",
            Self::PrivacyEngineering => "privacy_engineering",
            Self::MoneroBridgeOperations => "monero_bridge_operations",
            Self::PqCryptography => "pq_cryptography",
            Self::ReleaseManager => "release_manager",
            Self::IndependentAuditor => "independent_auditor",
        }
    }

    pub fn is_independent(self) -> bool {
        matches!(self, Self::ProtocolSecurity | Self::IndependentAuditor)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Approved,
    ApprovedWithHold,
    Rejected,
    Abstained,
}

impl ReviewDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::ApprovedWithHold => "approved_with_hold",
            Self::Rejected => "rejected",
            Self::Abstained => "abstained",
        }
    }

    pub fn contributes_to_quorum(self) -> bool {
        matches!(self, Self::Approved | Self::ApprovedWithHold)
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::ApprovedWithHold | Self::Rejected)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyAcceptance {
    Accepted,
    AcceptedWithMitigations,
    Rejected,
    NotReviewed,
}

impl PrivacyAcceptance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::AcceptedWithMitigations => "accepted_with_mitigations",
            Self::Rejected => "rejected",
            Self::NotReviewed => "not_reviewed",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Accepted | Self::AcceptedWithMitigations)
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
        matches!(self, Self::Medium | Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    Resolved,
    Mitigated,
    AcceptedRisk,
    Open,
    Disputed,
}

impl FindingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Mitigated => "mitigated",
            Self::AcceptedRisk => "accepted_risk",
            Self::Open => "open",
            Self::Disputed => "disputed",
        }
    }

    pub fn unresolved(self) -> bool {
        matches!(self, Self::Open | Self::Disputed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptActivationStatus {
    Active,
    ShadowMode,
    Stale,
    Rejected,
}

impl ReceiptActivationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ShadowMode => "shadow_mode",
            Self::Stale => "stale",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_active(self) -> bool {
        self == Self::Active
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseManifestDecision {
    Enforce,
    HoldAuditCoverage,
    HoldReviewerQuorum,
    HoldPrivacyAcceptance,
    HoldUnresolvedFindings,
    HoldActivationRoot,
    HoldLiveReceipts,
    Deny,
}

impl ReleaseManifestDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enforce => "enforce",
            Self::HoldAuditCoverage => "hold_audit_coverage",
            Self::HoldReviewerQuorum => "hold_reviewer_quorum",
            Self::HoldPrivacyAcceptance => "hold_privacy_acceptance",
            Self::HoldUnresolvedFindings => "hold_unresolved_findings",
            Self::HoldActivationRoot => "hold_activation_root",
            Self::HoldLiveReceipts => "hold_live_receipts",
            Self::Deny => "deny",
        }
    }

    pub fn permits_activation(self) -> bool {
        self == Self::Enforce
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldExportKind {
    AuditScopeGap,
    ReviewerQuorumGap,
    PrivacyAcceptanceGap,
    UnresolvedFinding,
    ActivationAdjudicatorGap,
    LiveReceiptGap,
}

impl HoldExportKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditScopeGap => "audit_scope_gap",
            Self::ReviewerQuorumGap => "reviewer_quorum_gap",
            Self::PrivacyAcceptanceGap => "privacy_acceptance_gap",
            Self::UnresolvedFinding => "unresolved_finding",
            Self::ActivationAdjudicatorGap => "activation_adjudicator_gap",
            Self::LiveReceiptGap => "live_receipt_gap",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub release_manifest_suite: String,
    pub l2_network: String,
    pub monero_network: String,
    pub release_epoch: u64,
    pub current_height: u64,
    pub required_scope_count: usize,
    pub min_reviewer_quorum_weight: u64,
    pub min_independent_reviewers: u64,
    pub max_receipt_age_blocks: u64,
    pub release_hold_export_count: usize,
    pub require_privacy_acceptance: bool,
    pub require_activation_adjudicator_root: bool,
    pub fail_closed_on_unresolved_findings: bool,
    pub fail_closed_on_stale_receipts: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            release_manifest_suite: RELEASE_MANIFEST_SUITE.to_string(),
            l2_network: DEVNET_L2_NETWORK.to_string(),
            monero_network: DEVNET_MONERO_NETWORK.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            current_height: DEFAULT_CURRENT_HEIGHT,
            required_scope_count: DEFAULT_REQUIRED_SCOPE_COUNT,
            min_reviewer_quorum_weight: DEFAULT_MIN_REVIEWER_QUORUM_WEIGHT,
            min_independent_reviewers: DEFAULT_MIN_INDEPENDENT_REVIEWERS,
            max_receipt_age_blocks: DEFAULT_MAX_RECEIPT_AGE_BLOCKS,
            release_hold_export_count: DEFAULT_RELEASE_HOLD_EXPORT_COUNT,
            require_privacy_acceptance: true,
            require_activation_adjudicator_root: true,
            fail_closed_on_unresolved_findings: true,
            fail_closed_on_stale_receipts: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "release_manifest_suite": self.release_manifest_suite,
            "l2_network": self.l2_network,
            "monero_network": self.monero_network,
            "release_epoch": self.release_epoch,
            "current_height": self.current_height,
            "required_scope_count": self.required_scope_count,
            "min_reviewer_quorum_weight": self.min_reviewer_quorum_weight,
            "min_independent_reviewers": self.min_independent_reviewers,
            "max_receipt_age_blocks": self.max_receipt_age_blocks,
            "release_hold_export_count": self.release_hold_export_count,
            "require_privacy_acceptance": yes_no(self.require_privacy_acceptance),
            "require_activation_adjudicator_root": yes_no(self.require_activation_adjudicator_root),
            "fail_closed_on_unresolved_findings": yes_no(self.fail_closed_on_unresolved_findings),
            "fail_closed_on_stale_receipts": yes_no(self.fail_closed_on_stale_receipts),
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
pub struct AuditScopeCoverage {
    pub scope: AuditScope,
    pub status: ScopeCoverageStatus,
    pub evidence_ids: Vec<String>,
    pub evidence_root: String,
    pub coverage_note: String,
    pub release_blocker: String,
}

impl AuditScopeCoverage {
    pub fn new(
        scope: AuditScope,
        status: ScopeCoverageStatus,
        evidence_ids: Vec<String>,
        coverage_note: &str,
    ) -> Self {
        let evidence_root = merkle_root(
            "monero-l2-pq-force-exit-release-audit-scope-evidence",
            &evidence_ids.iter().map(|id| json!(id)).collect::<Vec<_>>(),
        );
        Self {
            scope,
            status,
            evidence_ids,
            evidence_root,
            coverage_note: coverage_note.to_string(),
            release_blocker: yes_no(!status.is_covered()).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "scope": self.scope.as_str(),
            "status": self.status.as_str(),
            "evidence_ids": self.evidence_ids,
            "evidence_root": self.evidence_root,
            "coverage_note": self.coverage_note,
            "release_blocker": self.release_blocker,
        })
    }

    pub fn root(&self) -> String {
        record_root("audit-scope-coverage", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerAttestation {
    pub reviewer_id: String,
    pub role: ReviewerRole,
    pub decision: ReviewDecision,
    pub quorum_weight: u64,
    pub attestation_root: String,
    pub signed_scope_root: String,
    pub note: String,
}

impl ReviewerAttestation {
    pub fn new(
        reviewer_id: &str,
        role: ReviewerRole,
        decision: ReviewDecision,
        quorum_weight: u64,
        signed_scope_root: &str,
        note: &str,
    ) -> Self {
        let attestation_root = domain_hash(
            "monero-l2-pq-force-exit-release-reviewer-attestation",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(reviewer_id),
                HashPart::Str(role.as_str()),
                HashPart::Str(decision.as_str()),
                HashPart::U64(quorum_weight),
                HashPart::Str(signed_scope_root),
            ],
            32,
        );
        Self {
            reviewer_id: reviewer_id.to_string(),
            role,
            decision,
            quorum_weight,
            attestation_root,
            signed_scope_root: signed_scope_root.to_string(),
            note: note.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_id": self.reviewer_id,
            "role": self.role.as_str(),
            "decision": self.decision.as_str(),
            "quorum_weight": self.quorum_weight,
            "attestation_root": self.attestation_root,
            "signed_scope_root": self.signed_scope_root,
            "note": self.note,
        })
    }

    pub fn root(&self) -> String {
        record_root("reviewer-attestation", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyNonLinkageAcceptance {
    pub acceptance_id: String,
    pub acceptance: PrivacyAcceptance,
    pub unlinkability_evidence_root: String,
    pub metadata_minimization_root: String,
    pub reviewer_attestation_root: String,
    pub mitigation_note: String,
}

impl PrivacyNonLinkageAcceptance {
    pub fn public_record(&self) -> Value {
        json!({
            "acceptance_id": self.acceptance_id,
            "acceptance": self.acceptance.as_str(),
            "unlinkability_evidence_root": self.unlinkability_evidence_root,
            "metadata_minimization_root": self.metadata_minimization_root,
            "reviewer_attestation_root": self.reviewer_attestation_root,
            "mitigation_note": self.mitigation_note,
        })
    }

    pub fn root(&self) -> String {
        record_root("privacy-non-linkage-acceptance", &self.public_record())
    }

    pub fn permits_release(&self) -> bool {
        self.acceptance.permits_release()
            && !self.unlinkability_evidence_root.is_empty()
            && !self.metadata_minimization_root.is_empty()
            && !self.reviewer_attestation_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityFinding {
    pub finding_id: String,
    pub scope: AuditScope,
    pub severity: FindingSeverity,
    pub status: FindingStatus,
    pub owner: String,
    pub finding_root: String,
    pub remediation_root: String,
    pub release_hold: String,
    pub summary: String,
}

impl SecurityFinding {
    pub fn new(
        finding_id: &str,
        scope: AuditScope,
        severity: FindingSeverity,
        status: FindingStatus,
        owner: &str,
        remediation_root: &str,
        summary: &str,
    ) -> Self {
        let release_hold = yes_no(severity.blocks_release() && status.unresolved()).to_string();
        let finding_root = domain_hash(
            "monero-l2-pq-force-exit-release-security-finding",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(finding_id),
                HashPart::Str(scope.as_str()),
                HashPart::Str(severity.as_str()),
                HashPart::Str(status.as_str()),
                HashPart::Str(owner),
                HashPart::Str(remediation_root),
            ],
            32,
        );
        Self {
            finding_id: finding_id.to_string(),
            scope,
            severity,
            status,
            owner: owner.to_string(),
            finding_root,
            remediation_root: remediation_root.to_string(),
            release_hold,
            summary: summary.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "finding_id": self.finding_id,
            "scope": self.scope.as_str(),
            "severity": self.severity.as_str(),
            "status": self.status.as_str(),
            "owner": self.owner,
            "finding_root": self.finding_root,
            "remediation_root": self.remediation_root,
            "release_hold": self.release_hold,
            "summary": self.summary,
        })
    }

    pub fn root(&self) -> String {
        record_root("security-finding", &self.public_record())
    }

    pub fn holds_release(&self) -> bool {
        self.release_hold == "yes"
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveReceiptActivation {
    pub receipt_id: String,
    pub force_exit_package_id: String,
    pub status: ReceiptActivationStatus,
    pub observed_height: u64,
    pub receipt_root: String,
    pub activation_feed_root: String,
    pub adjudicator_root: String,
    pub package_policy_root: String,
}

impl LiveReceiptActivation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        receipt_id: &str,
        force_exit_package_id: &str,
        status: ReceiptActivationStatus,
        observed_height: u64,
        activation_feed_root: &str,
        adjudicator_root: &str,
        package_policy_root: &str,
    ) -> Self {
        let receipt_root = domain_hash(
            "monero-l2-pq-force-exit-live-receipt-activation",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(receipt_id),
                HashPart::Str(force_exit_package_id),
                HashPart::Str(status.as_str()),
                HashPart::U64(observed_height),
                HashPart::Str(activation_feed_root),
                HashPart::Str(adjudicator_root),
                HashPart::Str(package_policy_root),
            ],
            32,
        );
        Self {
            receipt_id: receipt_id.to_string(),
            force_exit_package_id: force_exit_package_id.to_string(),
            status,
            observed_height,
            receipt_root,
            activation_feed_root: activation_feed_root.to_string(),
            adjudicator_root: adjudicator_root.to_string(),
            package_policy_root: package_policy_root.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "force_exit_package_id": self.force_exit_package_id,
            "status": self.status.as_str(),
            "observed_height": self.observed_height,
            "receipt_root": self.receipt_root,
            "activation_feed_root": self.activation_feed_root,
            "adjudicator_root": self.adjudicator_root,
            "package_policy_root": self.package_policy_root,
        })
    }

    pub fn root(&self) -> String {
        record_root("live-receipt-activation", &self.public_record())
    }

    pub fn is_fresh(&self, config: &Config) -> bool {
        config.current_height.saturating_sub(self.observed_height) <= config.max_receipt_age_blocks
    }

    pub fn satisfies_activation(&self, config: &Config) -> bool {
        self.status.is_active()
            && self.is_fresh(config)
            && !self.activation_feed_root.is_empty()
            && (!config.require_activation_adjudicator_root || !self.adjudicator_root.is_empty())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationAdjudicator {
    pub adjudicator_id: String,
    pub adjudicator_root: String,
    pub accepted_receipt_root: String,
    pub rejected_receipt_root: String,
    pub finality_note: String,
}

impl ActivationAdjudicator {
    pub fn from_receipts(receipts: &[LiveReceiptActivation], finality_note: &str) -> Self {
        let accepted = receipts
            .iter()
            .filter(|receipt| receipt.status == ReceiptActivationStatus::Active)
            .map(LiveReceiptActivation::public_record)
            .collect::<Vec<_>>();
        let rejected = receipts
            .iter()
            .filter(|receipt| receipt.status != ReceiptActivationStatus::Active)
            .map(LiveReceiptActivation::public_record)
            .collect::<Vec<_>>();
        let accepted_receipt_root =
            merkle_root("monero-l2-pq-force-exit-accepted-live-receipts", &accepted);
        let rejected_receipt_root =
            merkle_root("monero-l2-pq-force-exit-rejected-live-receipts", &rejected);
        let adjudicator_id = deterministic_id(
            "activation-adjudicator",
            &[
                HashPart::Str(&accepted_receipt_root),
                HashPart::Str(&rejected_receipt_root),
                HashPart::Str(finality_note),
            ],
        );
        let adjudicator_root = domain_hash(
            "monero-l2-pq-force-exit-activation-adjudicator",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&adjudicator_id),
                HashPart::Str(&accepted_receipt_root),
                HashPart::Str(&rejected_receipt_root),
                HashPart::Str(finality_note),
            ],
            32,
        );
        Self {
            adjudicator_id,
            adjudicator_root,
            accepted_receipt_root,
            rejected_receipt_root,
            finality_note: finality_note.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "adjudicator_id": self.adjudicator_id,
            "adjudicator_root": self.adjudicator_root,
            "accepted_receipt_root": self.accepted_receipt_root,
            "rejected_receipt_root": self.rejected_receipt_root,
            "finality_note": self.finality_note,
        })
    }

    pub fn root(&self) -> String {
        record_root("activation-adjudicator", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseHoldExport {
    pub hold_id: String,
    pub kind: HoldExportKind,
    pub source_id: String,
    pub severity: FindingSeverity,
    pub release_manifest_blocker: String,
    pub export_root: String,
    pub operator_action: String,
}

impl ReleaseHoldExport {
    pub fn new(
        kind: HoldExportKind,
        source_id: &str,
        severity: FindingSeverity,
        operator_action: &str,
    ) -> Self {
        let hold_id = deterministic_id(
            "release-hold-export",
            &[
                HashPart::Str(kind.as_str()),
                HashPart::Str(source_id),
                HashPart::Str(severity.as_str()),
                HashPart::Str(operator_action),
            ],
        );
        let export_root = domain_hash(
            "monero-l2-pq-force-exit-release-hold-export",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&hold_id),
                HashPart::Str(kind.as_str()),
                HashPart::Str(source_id),
                HashPart::Str(severity.as_str()),
                HashPart::Str(operator_action),
            ],
            32,
        );
        Self {
            hold_id,
            kind,
            source_id: source_id.to_string(),
            severity,
            release_manifest_blocker: "yes".to_string(),
            export_root,
            operator_action: operator_action.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hold_id": self.hold_id,
            "kind": self.kind.as_str(),
            "source_id": self.source_id,
            "severity": self.severity.as_str(),
            "release_manifest_blocker": self.release_manifest_blocker,
            "export_root": self.export_root,
            "operator_action": self.operator_action,
        })
    }

    pub fn root(&self) -> String {
        record_root("release-hold-export", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementCounters {
    pub required_scope_count: usize,
    pub covered_scope_count: usize,
    pub missing_scope_count: usize,
    pub reviewer_count: usize,
    pub approving_reviewer_count: usize,
    pub independent_reviewer_count: u64,
    pub reviewer_quorum_weight: u64,
    pub unresolved_finding_count: usize,
    pub critical_unresolved_finding_count: usize,
    pub active_receipt_count: usize,
    pub stale_or_rejected_receipt_count: usize,
    pub release_hold_export_count: usize,
}

impl EnforcementCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "required_scope_count": self.required_scope_count,
            "covered_scope_count": self.covered_scope_count,
            "missing_scope_count": self.missing_scope_count,
            "reviewer_count": self.reviewer_count,
            "approving_reviewer_count": self.approving_reviewer_count,
            "independent_reviewer_count": self.independent_reviewer_count,
            "reviewer_quorum_weight": self.reviewer_quorum_weight,
            "unresolved_finding_count": self.unresolved_finding_count,
            "critical_unresolved_finding_count": self.critical_unresolved_finding_count,
            "active_receipt_count": self.active_receipt_count,
            "stale_or_rejected_receipt_count": self.stale_or_rejected_receipt_count,
            "release_hold_export_count": self.release_hold_export_count,
        })
    }

    pub fn root(&self) -> String {
        record_root("enforcement-counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseManifestEnforcement {
    pub manifest_id: String,
    pub decision: ReleaseManifestDecision,
    pub decision_reason: String,
    pub audit_scope_root: String,
    pub reviewer_quorum_root: String,
    pub privacy_acceptance_root: String,
    pub unresolved_finding_root: String,
    pub activation_adjudicator_root: String,
    pub live_receipt_root: String,
    pub hold_export_root: String,
    pub counters_root: String,
    pub enforcement_root: String,
}

impl ReleaseManifestEnforcement {
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        config: &Config,
        scopes: &[AuditScopeCoverage],
        reviewers: &[ReviewerAttestation],
        privacy_acceptance: &PrivacyNonLinkageAcceptance,
        findings: &[SecurityFinding],
        receipts: &[LiveReceiptActivation],
        adjudicator: &ActivationAdjudicator,
        holds: &[ReleaseHoldExport],
        counters: &EnforcementCounters,
    ) -> Self {
        let coverage_ok = counters.covered_scope_count >= config.required_scope_count
            && counters.missing_scope_count == 0;
        let quorum_ok = counters.reviewer_quorum_weight >= config.min_reviewer_quorum_weight
            && counters.independent_reviewer_count >= config.min_independent_reviewers;
        let privacy_ok = !config.require_privacy_acceptance || privacy_acceptance.permits_release();
        let findings_ok =
            !config.fail_closed_on_unresolved_findings || counters.unresolved_finding_count == 0;
        let receipts_ok = receipts
            .iter()
            .any(|receipt| receipt.satisfies_activation(config));
        let stale_ok =
            !config.fail_closed_on_stale_receipts || counters.stale_or_rejected_receipt_count == 0;
        let adjudicator_ok = !config.require_activation_adjudicator_root
            || (!adjudicator.adjudicator_root.is_empty()
                && receipts
                    .iter()
                    .any(|receipt| receipt.adjudicator_root == adjudicator.adjudicator_root));

        let decision = if !coverage_ok {
            ReleaseManifestDecision::HoldAuditCoverage
        } else if !quorum_ok {
            ReleaseManifestDecision::HoldReviewerQuorum
        } else if !privacy_ok {
            ReleaseManifestDecision::HoldPrivacyAcceptance
        } else if !findings_ok {
            ReleaseManifestDecision::HoldUnresolvedFindings
        } else if !adjudicator_ok {
            ReleaseManifestDecision::HoldActivationRoot
        } else if !receipts_ok || !stale_ok {
            ReleaseManifestDecision::HoldLiveReceipts
        } else if holds.is_empty() {
            ReleaseManifestDecision::Enforce
        } else {
            ReleaseManifestDecision::Deny
        };
        let decision_reason =
            decision_reason(decision, coverage_ok, quorum_ok, privacy_ok, findings_ok);
        let audit_scope_root = root_for_records(
            "monero-l2-pq-force-exit-release-audit-scopes",
            &scopes
                .iter()
                .map(AuditScopeCoverage::public_record)
                .collect::<Vec<_>>(),
        );
        let reviewer_quorum_root = root_for_records(
            "monero-l2-pq-force-exit-release-reviewer-quorum",
            &reviewers
                .iter()
                .map(ReviewerAttestation::public_record)
                .collect::<Vec<_>>(),
        );
        let privacy_acceptance_root = privacy_acceptance.root();
        let unresolved_finding_root = root_for_records(
            "monero-l2-pq-force-exit-release-unresolved-findings",
            &findings
                .iter()
                .filter(|finding| finding.holds_release())
                .map(SecurityFinding::public_record)
                .collect::<Vec<_>>(),
        );
        let activation_adjudicator_root = adjudicator.root();
        let live_receipt_root = root_for_records(
            "monero-l2-pq-force-exit-release-live-receipts",
            &receipts
                .iter()
                .map(LiveReceiptActivation::public_record)
                .collect::<Vec<_>>(),
        );
        let hold_export_root = root_for_records(
            "monero-l2-pq-force-exit-release-hold-exports",
            &holds
                .iter()
                .map(ReleaseHoldExport::public_record)
                .collect::<Vec<_>>(),
        );
        let counters_root = counters.root();
        let manifest_id = deterministic_id(
            "release-manifest-enforcement",
            &[
                HashPart::Str(&audit_scope_root),
                HashPart::Str(&reviewer_quorum_root),
                HashPart::Str(&privacy_acceptance_root),
                HashPart::Str(&activation_adjudicator_root),
                HashPart::Str(&hold_export_root),
            ],
        );
        let enforcement_root = domain_hash(
            "monero-l2-pq-force-exit-release-manifest-enforcement",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&manifest_id),
                HashPart::Str(decision.as_str()),
                HashPart::Str(&decision_reason),
                HashPart::Str(&audit_scope_root),
                HashPart::Str(&reviewer_quorum_root),
                HashPart::Str(&privacy_acceptance_root),
                HashPart::Str(&unresolved_finding_root),
                HashPart::Str(&activation_adjudicator_root),
                HashPart::Str(&live_receipt_root),
                HashPart::Str(&hold_export_root),
                HashPart::Str(&counters_root),
            ],
            32,
        );
        Self {
            manifest_id,
            decision,
            decision_reason,
            audit_scope_root,
            reviewer_quorum_root,
            privacy_acceptance_root,
            unresolved_finding_root,
            activation_adjudicator_root,
            live_receipt_root,
            hold_export_root,
            counters_root,
            enforcement_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "manifest_id": self.manifest_id,
            "decision": self.decision.as_str(),
            "decision_reason": self.decision_reason,
            "audit_scope_root": self.audit_scope_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "privacy_acceptance_root": self.privacy_acceptance_root,
            "unresolved_finding_root": self.unresolved_finding_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "live_receipt_root": self.live_receipt_root,
            "hold_export_root": self.hold_export_root,
            "counters_root": self.counters_root,
            "enforcement_root": self.enforcement_root,
        })
    }

    pub fn root(&self) -> String {
        record_root("release-manifest-enforcement", &self.public_record())
    }

    pub fn permits_activation(&self) -> bool {
        self.decision.permits_activation()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub audit_scopes: Vec<AuditScopeCoverage>,
    pub reviewer_attestations: Vec<ReviewerAttestation>,
    pub privacy_acceptance: PrivacyNonLinkageAcceptance,
    pub security_findings: Vec<SecurityFinding>,
    pub live_receipts: Vec<LiveReceiptActivation>,
    pub activation_adjudicator: ActivationAdjudicator,
    pub release_hold_exports: Vec<ReleaseHoldExport>,
    pub counters: EnforcementCounters,
    pub enforcement: ReleaseManifestEnforcement,
    pub roots: BTreeMap<String, String>,
}

impl State {
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        config: Config,
        audit_scopes: Vec<AuditScopeCoverage>,
        reviewer_attestations: Vec<ReviewerAttestation>,
        privacy_acceptance: PrivacyNonLinkageAcceptance,
        security_findings: Vec<SecurityFinding>,
        live_receipts: Vec<LiveReceiptActivation>,
        activation_adjudicator: ActivationAdjudicator,
        release_hold_exports: Vec<ReleaseHoldExport>,
    ) -> Result<Self> {
        validate_required_scopes(&config, &audit_scopes)?;
        validate_reviewer_ids(&reviewer_attestations)?;
        validate_receipt_adjudicator(&config, &live_receipts, &activation_adjudicator)?;
        let counters = build_counters(
            &config,
            &audit_scopes,
            &reviewer_attestations,
            &security_findings,
            &live_receipts,
            &release_hold_exports,
        );
        let enforcement = ReleaseManifestEnforcement::evaluate(
            &config,
            &audit_scopes,
            &reviewer_attestations,
            &privacy_acceptance,
            &security_findings,
            &live_receipts,
            &activation_adjudicator,
            &release_hold_exports,
            &counters,
        );
        let roots = state_roots(
            &config,
            &audit_scopes,
            &reviewer_attestations,
            &privacy_acceptance,
            &security_findings,
            &live_receipts,
            &activation_adjudicator,
            &release_hold_exports,
            &counters,
            &enforcement,
        );
        Ok(Self {
            config,
            audit_scopes,
            reviewer_attestations,
            privacy_acceptance,
            security_findings,
            live_receipts,
            activation_adjudicator,
            release_hold_exports,
            counters,
            enforcement,
            roots,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "audit_scopes": self.audit_scopes.iter().map(AuditScopeCoverage::public_record).collect::<Vec<_>>(),
            "reviewer_attestations": self.reviewer_attestations.iter().map(ReviewerAttestation::public_record).collect::<Vec<_>>(),
            "privacy_acceptance": self.privacy_acceptance.public_record(),
            "security_findings": self.security_findings.iter().map(SecurityFinding::public_record).collect::<Vec<_>>(),
            "live_receipts": self.live_receipts.iter().map(LiveReceiptActivation::public_record).collect::<Vec<_>>(),
            "activation_adjudicator": self.activation_adjudicator.public_record(),
            "release_hold_exports": self.release_hold_exports.iter().map(ReleaseHoldExport::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "enforcement": self.enforcement.public_record(),
            "roots": self.roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }

    pub fn release_allowed(&self) -> bool {
        self.enforcement.permits_activation()
    }
}

impl Default for State {
    fn default() -> Self {
        devnet()
    }
}

pub fn devnet() -> State {
    match build_devnet() {
        Ok(state) => state,
        Err(error) => devnet_fallback(error),
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn build_devnet() -> Result<State> {
    let config = Config::devnet();
    let audit_scopes = devnet_audit_scopes();
    let audit_scope_root = root_for_records(
        "monero-l2-pq-force-exit-release-devnet-audit-scopes",
        &audit_scopes
            .iter()
            .map(AuditScopeCoverage::public_record)
            .collect::<Vec<_>>(),
    );
    let reviewer_attestations = vec![
        ReviewerAttestation::new(
            "reviewer-protocol-security-001",
            ReviewerRole::ProtocolSecurity,
            ReviewDecision::Approved,
            24,
            &audit_scope_root,
            "force-exit package policy and user escape answer are covered for release manifest enforcement",
        ),
        ReviewerAttestation::new(
            "reviewer-independent-auditor-002",
            ReviewerRole::IndependentAuditor,
            ReviewDecision::ApprovedWithHold,
            22,
            &audit_scope_root,
            "release may proceed only after unresolved high severity privacy receipt hold exports clear",
        ),
        ReviewerAttestation::new(
            "reviewer-privacy-engineering-003",
            ReviewerRole::PrivacyEngineering,
            ReviewDecision::Approved,
            18,
            &audit_scope_root,
            "non-linkage artifacts meet devnet activation criteria",
        ),
        ReviewerAttestation::new(
            "reviewer-pq-cryptography-004",
            ReviewerRole::PqCryptography,
            ReviewDecision::Approved,
            16,
            &audit_scope_root,
            "pq authorization transcript roots are deterministic and bound to the release package",
        ),
        ReviewerAttestation::new(
            "reviewer-release-manager-005",
            ReviewerRole::ReleaseManager,
            ReviewDecision::ApprovedWithHold,
            12,
            &audit_scope_root,
            "exports release holds to operator dashboard before production manifest activation",
        ),
    ];
    let reviewer_root = root_for_records(
        "monero-l2-pq-force-exit-release-devnet-reviewers",
        &reviewer_attestations
            .iter()
            .map(ReviewerAttestation::public_record)
            .collect::<Vec<_>>(),
    );
    let privacy_acceptance = PrivacyNonLinkageAcceptance {
        acceptance_id: "privacy-non-linkage-acceptance-wave-79".to_string(),
        acceptance: PrivacyAcceptance::AcceptedWithMitigations,
        unlinkability_evidence_root: domain_hash(
            "monero-l2-pq-force-exit-devnet-unlinkability-evidence",
            &[HashPart::Str(CHAIN_ID), HashPart::Str(PROTOCOL_VERSION)],
            32,
        ),
        metadata_minimization_root: domain_hash(
            "monero-l2-pq-force-exit-devnet-metadata-minimization",
            &[HashPart::Str(CHAIN_ID), HashPart::Str("metadata-minimized")],
            32,
        ),
        reviewer_attestation_root: reviewer_root,
        mitigation_note: "receipt exports remain hash-only and exclude wallet, address, timing, and operator-local linkage metadata".to_string(),
    };
    let security_findings = devnet_security_findings();
    let provisional_receipts = devnet_live_receipts("");
    let provisional_adjudicator = ActivationAdjudicator::from_receipts(
        &provisional_receipts,
        "devnet adjudicator binds active receipt set before release manifest enforcement",
    );
    let live_receipts = devnet_live_receipts(&provisional_adjudicator.adjudicator_root);
    let activation_adjudicator = ActivationAdjudicator::from_receipts(
        &live_receipts,
        "devnet adjudicator binds active receipt set before release manifest enforcement",
    );
    let release_hold_exports = release_holds_from_inputs(
        &config,
        &audit_scopes,
        &reviewer_attestations,
        &privacy_acceptance,
        &security_findings,
        &live_receipts,
        &activation_adjudicator,
    );
    State::from_parts(
        config,
        audit_scopes,
        reviewer_attestations,
        privacy_acceptance,
        security_findings,
        live_receipts,
        activation_adjudicator,
        release_hold_exports,
    )
}

fn devnet_audit_scopes() -> Vec<AuditScopeCoverage> {
    AuditScope::all_required()
        .into_iter()
        .map(|scope| {
            let evidence_ids = vec![
                format!("{}-receipt-root", scope.as_str()),
                format!("{}-adjudicator-root", scope.as_str()),
                format!("{}-release-policy-root", scope.as_str()),
            ];
            AuditScopeCoverage::new(
                scope,
                ScopeCoverageStatus::Covered,
                evidence_ids,
                "devnet evidence covers audit/security live receipt activation into release manifest policy",
            )
        })
        .collect()
}

fn devnet_security_findings() -> Vec<SecurityFinding> {
    vec![
        SecurityFinding::new(
            "finding-force-exit-package-001",
            AuditScope::CanonicalForceExitPackage,
            FindingSeverity::Low,
            FindingStatus::Mitigated,
            "bridge-security",
            "mitigation-force-exit-package-domain-separation-root",
            "canonical package hash binding needed explicit domain separation and is mitigated",
        ),
        SecurityFinding::new(
            "finding-privacy-receipt-export-002",
            AuditScope::PrivacyNonLinkage,
            FindingSeverity::High,
            FindingStatus::Open,
            "privacy-engineering",
            "mitigation-privacy-receipt-export-redaction-root",
            "operator export path needs final proof that release holds cannot link user escape answers to Monero timing",
        ),
        SecurityFinding::new(
            "finding-live-receipt-window-003",
            AuditScope::ReceiptIngestion,
            FindingSeverity::Medium,
            FindingStatus::Resolved,
            "runtime-ops",
            "mitigation-live-receipt-window-root",
            "stale receipt window was narrowed and resolved in devnet policy",
        ),
    ]
}

fn devnet_live_receipts(adjudicator_root: &str) -> Vec<LiveReceiptActivation> {
    vec![
        LiveReceiptActivation::new(
            "receipt-force-exit-package-active-001",
            "force-exit-package-user-escape-answer-wave-79",
            ReceiptActivationStatus::Active,
            DEFAULT_CURRENT_HEIGHT.saturating_sub(12),
            "activation-feed-force-exit-package-active-root",
            adjudicator_root,
            "package-policy-force-exit-release-manifest-root",
        ),
        LiveReceiptActivation::new(
            "receipt-force-exit-package-shadow-002",
            "force-exit-package-user-escape-answer-shadow-wave-79",
            ReceiptActivationStatus::ShadowMode,
            DEFAULT_CURRENT_HEIGHT.saturating_sub(18),
            "activation-feed-force-exit-package-shadow-root",
            adjudicator_root,
            "package-policy-force-exit-shadow-root",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn release_holds_from_inputs(
    config: &Config,
    scopes: &[AuditScopeCoverage],
    reviewers: &[ReviewerAttestation],
    privacy_acceptance: &PrivacyNonLinkageAcceptance,
    findings: &[SecurityFinding],
    receipts: &[LiveReceiptActivation],
    adjudicator: &ActivationAdjudicator,
) -> Vec<ReleaseHoldExport> {
    let mut holds = Vec::new();
    for scope in scopes {
        if !scope.status.is_covered() {
            holds.push(ReleaseHoldExport::new(
                HoldExportKind::AuditScopeGap,
                scope.scope.as_str(),
                FindingSeverity::High,
                "attach complete scope coverage evidence before release manifest activation",
            ));
        }
    }
    let quorum_weight = reviewers
        .iter()
        .filter(|reviewer| reviewer.decision.contributes_to_quorum())
        .map(|reviewer| reviewer.quorum_weight)
        .sum::<u64>();
    let independent_count = reviewers
        .iter()
        .filter(|reviewer| {
            reviewer.role.is_independent() && reviewer.decision.contributes_to_quorum()
        })
        .count() as u64;
    if quorum_weight < config.min_reviewer_quorum_weight
        || independent_count < config.min_independent_reviewers
    {
        holds.push(ReleaseHoldExport::new(
            HoldExportKind::ReviewerQuorumGap,
            "reviewer-quorum",
            FindingSeverity::High,
            "collect independent approval quorum before release manifest activation",
        ));
    }
    if config.require_privacy_acceptance && !privacy_acceptance.permits_release() {
        holds.push(ReleaseHoldExport::new(
            HoldExportKind::PrivacyAcceptanceGap,
            &privacy_acceptance.acceptance_id,
            FindingSeverity::Critical,
            "obtain privacy and non-linkage acceptance before exporting release manifest",
        ));
    }
    for finding in findings.iter().filter(|finding| finding.holds_release()) {
        holds.push(ReleaseHoldExport::new(
            HoldExportKind::UnresolvedFinding,
            &finding.finding_id,
            finding.severity,
            "resolve, mitigate, or explicitly adjudicate finding before release",
        ));
    }
    if config.require_activation_adjudicator_root && adjudicator.adjudicator_root.is_empty() {
        holds.push(ReleaseHoldExport::new(
            HoldExportKind::ActivationAdjudicatorGap,
            "activation-adjudicator",
            FindingSeverity::Critical,
            "publish activation adjudicator root before manifest enforcement",
        ));
    }
    for receipt in receipts
        .iter()
        .filter(|receipt| !receipt.satisfies_activation(config))
    {
        holds.push(ReleaseHoldExport::new(
            HoldExportKind::LiveReceiptGap,
            &receipt.receipt_id,
            FindingSeverity::Medium,
            "refresh or reject stale live receipt before release manifest export",
        ));
    }
    holds
        .into_iter()
        .take(config.release_hold_export_count)
        .collect()
}

fn build_counters(
    config: &Config,
    scopes: &[AuditScopeCoverage],
    reviewers: &[ReviewerAttestation],
    findings: &[SecurityFinding],
    receipts: &[LiveReceiptActivation],
    holds: &[ReleaseHoldExport],
) -> EnforcementCounters {
    let covered_scope_count = scopes
        .iter()
        .filter(|scope| scope.status.is_covered())
        .count();
    let missing_scope_count = config
        .required_scope_count
        .saturating_sub(covered_scope_count);
    let approving_reviewer_count = reviewers
        .iter()
        .filter(|reviewer| reviewer.decision.contributes_to_quorum())
        .count();
    let independent_reviewer_count = reviewers
        .iter()
        .filter(|reviewer| {
            reviewer.role.is_independent() && reviewer.decision.contributes_to_quorum()
        })
        .count() as u64;
    let reviewer_quorum_weight = reviewers
        .iter()
        .filter(|reviewer| reviewer.decision.contributes_to_quorum())
        .map(|reviewer| reviewer.quorum_weight)
        .sum();
    let unresolved_finding_count = findings
        .iter()
        .filter(|finding| finding.holds_release())
        .count();
    let critical_unresolved_finding_count = findings
        .iter()
        .filter(|finding| finding.holds_release() && finding.severity == FindingSeverity::Critical)
        .count();
    let active_receipt_count = receipts
        .iter()
        .filter(|receipt| receipt.satisfies_activation(config))
        .count();
    let stale_or_rejected_receipt_count = receipts
        .iter()
        .filter(|receipt| !receipt.satisfies_activation(config))
        .count();
    EnforcementCounters {
        required_scope_count: config.required_scope_count,
        covered_scope_count,
        missing_scope_count,
        reviewer_count: reviewers.len(),
        approving_reviewer_count,
        independent_reviewer_count,
        reviewer_quorum_weight,
        unresolved_finding_count,
        critical_unresolved_finding_count,
        active_receipt_count,
        stale_or_rejected_receipt_count,
        release_hold_export_count: holds.len(),
    }
}

#[allow(clippy::too_many_arguments)]
fn state_roots(
    config: &Config,
    scopes: &[AuditScopeCoverage],
    reviewers: &[ReviewerAttestation],
    privacy_acceptance: &PrivacyNonLinkageAcceptance,
    findings: &[SecurityFinding],
    receipts: &[LiveReceiptActivation],
    adjudicator: &ActivationAdjudicator,
    holds: &[ReleaseHoldExport],
    counters: &EnforcementCounters,
    enforcement: &ReleaseManifestEnforcement,
) -> BTreeMap<String, String> {
    let mut roots = BTreeMap::new();
    roots.insert("config".to_string(), config.state_root());
    roots.insert(
        "audit_scopes".to_string(),
        root_for_records(
            "monero-l2-pq-force-exit-release-state-audit-scopes",
            &scopes
                .iter()
                .map(AuditScopeCoverage::public_record)
                .collect::<Vec<_>>(),
        ),
    );
    roots.insert(
        "reviewer_attestations".to_string(),
        root_for_records(
            "monero-l2-pq-force-exit-release-state-reviewers",
            &reviewers
                .iter()
                .map(ReviewerAttestation::public_record)
                .collect::<Vec<_>>(),
        ),
    );
    roots.insert("privacy_acceptance".to_string(), privacy_acceptance.root());
    roots.insert(
        "security_findings".to_string(),
        root_for_records(
            "monero-l2-pq-force-exit-release-state-findings",
            &findings
                .iter()
                .map(SecurityFinding::public_record)
                .collect::<Vec<_>>(),
        ),
    );
    roots.insert(
        "live_receipts".to_string(),
        root_for_records(
            "monero-l2-pq-force-exit-release-state-live-receipts",
            &receipts
                .iter()
                .map(LiveReceiptActivation::public_record)
                .collect::<Vec<_>>(),
        ),
    );
    roots.insert("activation_adjudicator".to_string(), adjudicator.root());
    roots.insert(
        "release_hold_exports".to_string(),
        root_for_records(
            "monero-l2-pq-force-exit-release-state-hold-exports",
            &holds
                .iter()
                .map(ReleaseHoldExport::public_record)
                .collect::<Vec<_>>(),
        ),
    );
    roots.insert("counters".to_string(), counters.root());
    roots.insert("enforcement".to_string(), enforcement.root());
    roots
}

fn validate_required_scopes(config: &Config, scopes: &[AuditScopeCoverage]) -> Result<()> {
    let seen = scopes
        .iter()
        .map(|scope| scope.scope)
        .collect::<BTreeSet<_>>();
    let required = AuditScope::all_required()
        .into_iter()
        .collect::<BTreeSet<_>>();
    if seen.len() < config.required_scope_count {
        return Err("audit scope coverage count is below release manifest requirement".to_string());
    }
    for scope in required {
        if !seen.contains(&scope) {
            return Err(format!("missing required audit scope: {}", scope.as_str()));
        }
    }
    Ok(())
}

fn validate_reviewer_ids(reviewers: &[ReviewerAttestation]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for reviewer in reviewers {
        if reviewer.reviewer_id.trim().is_empty() {
            return Err("reviewer id cannot be empty".to_string());
        }
        if !seen.insert(reviewer.reviewer_id.clone()) {
            return Err(format!("duplicate reviewer id: {}", reviewer.reviewer_id));
        }
    }
    Ok(())
}

fn validate_receipt_adjudicator(
    config: &Config,
    receipts: &[LiveReceiptActivation],
    adjudicator: &ActivationAdjudicator,
) -> Result<()> {
    if config.require_activation_adjudicator_root && adjudicator.adjudicator_root.is_empty() {
        return Err("activation adjudicator root is required".to_string());
    }
    if receipts.is_empty() {
        return Err("at least one live receipt is required".to_string());
    }
    Ok(())
}

fn decision_reason(
    decision: ReleaseManifestDecision,
    coverage_ok: bool,
    quorum_ok: bool,
    privacy_ok: bool,
    findings_ok: bool,
) -> String {
    match decision {
        ReleaseManifestDecision::Enforce => {
            "all audit, quorum, privacy, finding, adjudicator, and live receipt gates enforce"
        }
        ReleaseManifestDecision::HoldAuditCoverage => "audit scope coverage is incomplete",
        ReleaseManifestDecision::HoldReviewerQuorum => "reviewer quorum is below policy",
        ReleaseManifestDecision::HoldPrivacyAcceptance => {
            "privacy and non-linkage acceptance is missing or rejected"
        }
        ReleaseManifestDecision::HoldUnresolvedFindings => {
            "unresolved security findings hold release manifest export"
        }
        ReleaseManifestDecision::HoldActivationRoot => "activation adjudicator root is missing",
        ReleaseManifestDecision::HoldLiveReceipts => "live receipt activation is stale or absent",
        ReleaseManifestDecision::Deny => {
            if !coverage_ok {
                "release denied because audit coverage failed"
            } else if !quorum_ok {
                "release denied because reviewer quorum failed"
            } else if !privacy_ok {
                "release denied because privacy acceptance failed"
            } else if !findings_ok {
                "release denied because unresolved findings remain"
            } else {
                "release denied because hold exports remain attached"
            }
        }
    }
    .to_string()
}

fn root_for_records(domain: &str, records: &[Value]) -> String {
    merkle_root(domain, records)
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "monero-l2-pq-force-exit-release-manifest-enforcement-record",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn deterministic_id(label: &str, parts: &[HashPart<'_>]) -> String {
    let digest = domain_hash(
        "monero-l2-pq-force-exit-release-manifest-enforcement-id",
        parts,
        16,
    );
    format!("{label}-{digest}")
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn devnet_fallback(_: String) -> State {
    let config = Config::devnet();
    let audit_scopes = devnet_audit_scopes();
    let privacy_acceptance = PrivacyNonLinkageAcceptance {
        acceptance_id: "fallback-privacy-acceptance".to_string(),
        acceptance: PrivacyAcceptance::Accepted,
        unlinkability_evidence_root: "fallback-unlinkability-root".to_string(),
        metadata_minimization_root: "fallback-metadata-root".to_string(),
        reviewer_attestation_root: "fallback-reviewer-root".to_string(),
        mitigation_note: "fallback acceptance keeps devnet constructor total".to_string(),
    };
    let reviewer_attestations = vec![ReviewerAttestation::new(
        "fallback-reviewer",
        ReviewerRole::IndependentAuditor,
        ReviewDecision::Approved,
        config.min_reviewer_quorum_weight,
        "fallback-scope-root",
        "fallback reviewer approval",
    )];
    let security_findings = Vec::new();
    let live_receipts = vec![LiveReceiptActivation::new(
        "fallback-live-receipt",
        "fallback-force-exit-package",
        ReceiptActivationStatus::Active,
        config.current_height,
        "fallback-activation-feed-root",
        "fallback-adjudicator-root",
        "fallback-policy-root",
    )];
    let activation_adjudicator =
        ActivationAdjudicator::from_receipts(&live_receipts, "fallback adjudicator");
    let release_hold_exports = Vec::new();
    let counters = build_counters(
        &config,
        &audit_scopes,
        &reviewer_attestations,
        &security_findings,
        &live_receipts,
        &release_hold_exports,
    );
    let enforcement = ReleaseManifestEnforcement::evaluate(
        &config,
        &audit_scopes,
        &reviewer_attestations,
        &privacy_acceptance,
        &security_findings,
        &live_receipts,
        &activation_adjudicator,
        &release_hold_exports,
        &counters,
    );
    let roots = state_roots(
        &config,
        &audit_scopes,
        &reviewer_attestations,
        &privacy_acceptance,
        &security_findings,
        &live_receipts,
        &activation_adjudicator,
        &release_hold_exports,
        &counters,
        &enforcement,
    );
    State {
        config,
        audit_scopes,
        reviewer_attestations,
        privacy_acceptance,
        security_findings,
        live_receipts,
        activation_adjudicator,
        release_hold_exports,
        counters,
        enforcement,
        roots,
    }
}
