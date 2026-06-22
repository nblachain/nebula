use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAuditSecurityAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-audit-security-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_AUDIT_SECURITY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const INCIDENT_HANDOFF_SUITE: &str =
    "monero-l2-pq-bridge-force-exit-audit-security-incident-handoff-v1";
pub const DEFAULT_HEIGHT: u64 = 96_086;
pub const DEFAULT_MIN_ROLLBACK_REVIEWS: u64 = 4;
pub const DEFAULT_MIN_THREAT_MODEL_REVIEWS: u64 = 5;
pub const DEFAULT_MIN_PRIVACY_CHECKS: u64 = 6;
pub const DEFAULT_MIN_SIGNER_QUORUMS: u64 = 4;
pub const DEFAULT_MIN_DEFERRED_AUDIT_ROOTS: u64 = 3;
pub const DEFAULT_MIN_FAIL_CLOSED_TRANSFERS: u64 = 4;
pub const DEFAULT_MAX_OPEN_BLOCKERS: u64 = 0;
pub const DEFAULT_MAX_PRIVACY_LEAKS: u64 = 0;
pub const DEFAULT_MAX_UNSIGNED_COMMANDS: u64 = 0;
pub const DEFAULT_MAX_RECORDS: usize = 512;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffLane {
    RollbackEvidenceReview,
    ThreatModelBlocker,
    PrivacyLeakCheck,
    SignerQuorumCheck,
    DeferredAuditRoot,
    FailClosedCommandTransfer,
    IncidentVerdict,
}

impl HandoffLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RollbackEvidenceReview,
            Self::ThreatModelBlocker,
            Self::PrivacyLeakCheck,
            Self::SignerQuorumCheck,
            Self::DeferredAuditRoot,
            Self::FailClosedCommandTransfer,
            Self::IncidentVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RollbackEvidenceReview => "rollback_evidence_review",
            Self::ThreatModelBlocker => "threat_model_blocker",
            Self::PrivacyLeakCheck => "privacy_leak_check",
            Self::SignerQuorumCheck => "signer_quorum_check",
            Self::DeferredAuditRoot => "deferred_audit_root",
            Self::FailClosedCommandTransfer => "fail_closed_command_transfer",
            Self::IncidentVerdict => "incident_verdict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentBlockerKind {
    MissingRollbackEvidenceReview,
    RollbackEvidenceRejected,
    OpenThreatModelBlocker,
    PrivacyLeakObserved,
    MissingPrivacyLeakCheck,
    MissingSignerQuorum,
    SignerQuorumRejected,
    MissingDeferredAuditRoot,
    DeferredAuditRootUnsealed,
    MissingFailClosedTransfer,
    CommandTransferNotFailClosed,
    UnsignedCommandTransfer,
    RawPrivatePayloadReferenced,
    HandoffRootMismatch,
}

impl IncidentBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRollbackEvidenceReview => "missing_rollback_evidence_review",
            Self::RollbackEvidenceRejected => "rollback_evidence_rejected",
            Self::OpenThreatModelBlocker => "open_threat_model_blocker",
            Self::PrivacyLeakObserved => "privacy_leak_observed",
            Self::MissingPrivacyLeakCheck => "missing_privacy_leak_check",
            Self::MissingSignerQuorum => "missing_signer_quorum",
            Self::SignerQuorumRejected => "signer_quorum_rejected",
            Self::MissingDeferredAuditRoot => "missing_deferred_audit_root",
            Self::DeferredAuditRootUnsealed => "deferred_audit_root_unsealed",
            Self::MissingFailClosedTransfer => "missing_fail_closed_transfer",
            Self::CommandTransferNotFailClosed => "command_transfer_not_fail_closed",
            Self::UnsignedCommandTransfer => "unsigned_command_transfer",
            Self::RawPrivatePayloadReferenced => "raw_private_payload_referenced",
            Self::HandoffRootMismatch => "handoff_root_mismatch",
        }
    }

    pub fn fail_closed(self) -> bool {
        matches!(
            self,
            Self::MissingRollbackEvidenceReview
                | Self::RollbackEvidenceRejected
                | Self::OpenThreatModelBlocker
                | Self::PrivacyLeakObserved
                | Self::MissingPrivacyLeakCheck
                | Self::MissingSignerQuorum
                | Self::SignerQuorumRejected
                | Self::MissingDeferredAuditRoot
                | Self::DeferredAuditRootUnsealed
                | Self::MissingFailClosedTransfer
                | Self::CommandTransferNotFailClosed
                | Self::UnsignedCommandTransfer
                | Self::RawPrivatePayloadReferenced
                | Self::HandoffRootMismatch
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    Accepted,
    AcceptedWithDeferredAudit,
    Held,
    Rejected,
}

impl ReviewDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::AcceptedWithDeferredAudit => "accepted_with_deferred_audit",
            Self::Held => "held",
            Self::Rejected => "rejected",
        }
    }

    pub fn passes(self) -> bool {
        matches!(self, Self::Accepted | Self::AcceptedWithDeferredAudit)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatModelStatus {
    Closed,
    Mitigated,
    DeferredWithOwner,
    Open,
}

impl ThreatModelStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Mitigated => "mitigated",
            Self::DeferredWithOwner => "deferred_with_owner",
            Self::Open => "open",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Open)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyBoundary {
    MoneroAddress,
    PrivatePayload,
    RouteMetadata,
    WatchtowerObservation,
    OperatorDashboardExport,
    AuditArchive,
}

impl PrivacyBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroAddress => "monero_address",
            Self::PrivatePayload => "private_payload",
            Self::RouteMetadata => "route_metadata",
            Self::WatchtowerObservation => "watchtower_observation",
            Self::OperatorDashboardExport => "operator_dashboard_export",
            Self::AuditArchive => "audit_archive",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandTransferKind {
    FreezeReleaseWindow,
    DisableExitPublish,
    RevokeOperatorUnhold,
    RequireIncidentCommanderAck,
    RequireSecurityReacceptance,
    SealAuditArchive,
}

impl CommandTransferKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreezeReleaseWindow => "freeze_release_window",
            Self::DisableExitPublish => "disable_exit_publish",
            Self::RevokeOperatorUnhold => "revoke_operator_unhold",
            Self::RequireIncidentCommanderAck => "require_incident_commander_ack",
            Self::RequireSecurityReacceptance => "require_security_reacceptance",
            Self::SealAuditArchive => "seal_audit_archive",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffVerdict {
    TransferAccepted,
    Hold,
    Abort,
}

impl HandoffVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TransferAccepted => "transfer_accepted",
            Self::Hold => "hold",
            Self::Abort => "abort",
        }
    }

    pub fn allows_release(self) -> bool {
        matches!(self, Self::TransferAccepted)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub incident_handoff_suite: String,
    pub handoff_id: String,
    pub rollback_drill_root: String,
    pub deployment_guard_root: String,
    pub release_policy_root: String,
    pub operator_dashboard_root: String,
    pub height: u64,
    pub min_rollback_reviews: u64,
    pub min_threat_model_reviews: u64,
    pub min_privacy_checks: u64,
    pub min_signer_quorums: u64,
    pub min_deferred_audit_roots: u64,
    pub min_fail_closed_transfers: u64,
    pub max_open_blockers: u64,
    pub max_privacy_leaks: u64,
    pub max_unsigned_commands: u64,
    pub max_records: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            incident_handoff_suite: INCIDENT_HANDOFF_SUITE.to_string(),
            handoff_id: stable_id("incident-handoff", "wave86-devnet"),
            rollback_drill_root: fixture_root("wave85-rollback-drill-root"),
            deployment_guard_root: fixture_root("wave85-deployment-guard-root"),
            release_policy_root: fixture_root("wave84-release-policy-root"),
            operator_dashboard_root: fixture_root("wave84-operator-dashboard-root"),
            height: DEFAULT_HEIGHT,
            min_rollback_reviews: DEFAULT_MIN_ROLLBACK_REVIEWS,
            min_threat_model_reviews: DEFAULT_MIN_THREAT_MODEL_REVIEWS,
            min_privacy_checks: DEFAULT_MIN_PRIVACY_CHECKS,
            min_signer_quorums: DEFAULT_MIN_SIGNER_QUORUMS,
            min_deferred_audit_roots: DEFAULT_MIN_DEFERRED_AUDIT_ROOTS,
            min_fail_closed_transfers: DEFAULT_MIN_FAIL_CLOSED_TRANSFERS,
            max_open_blockers: DEFAULT_MAX_OPEN_BLOCKERS,
            max_privacy_leaks: DEFAULT_MAX_PRIVACY_LEAKS,
            max_unsigned_commands: DEFAULT_MAX_UNSIGNED_COMMANDS,
            max_records: DEFAULT_MAX_RECORDS,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "incident_handoff_suite": self.incident_handoff_suite,
            "handoff_id": self.handoff_id,
            "rollback_drill_root": self.rollback_drill_root,
            "deployment_guard_root": self.deployment_guard_root,
            "release_policy_root": self.release_policy_root,
            "operator_dashboard_root": self.operator_dashboard_root,
            "height": self.height,
            "min_rollback_reviews": self.min_rollback_reviews,
            "min_threat_model_reviews": self.min_threat_model_reviews,
            "min_privacy_checks": self.min_privacy_checks,
            "min_signer_quorums": self.min_signer_quorums,
            "min_deferred_audit_roots": self.min_deferred_audit_roots,
            "min_fail_closed_transfers": self.min_fail_closed_transfers,
            "max_open_blockers": self.max_open_blockers,
            "max_privacy_leaks": self.max_privacy_leaks,
            "max_unsigned_commands": self.max_unsigned_commands,
            "max_records": self.max_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-CONFIG", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_root("rollback_drill_root", &self.rollback_drill_root)?;
        ensure_root("deployment_guard_root", &self.deployment_guard_root)?;
        ensure_root("release_policy_root", &self.release_policy_root)?;
        ensure_root("operator_dashboard_root", &self.operator_dashboard_root)?;
        if self.schema_version != SCHEMA_VERSION {
            Err("unsupported incident handoff schema version".to_string())
        } else if self.max_records == 0 {
            Err("max_records must be greater than zero".to_string())
        } else {
            Ok(())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollbackEvidenceReview {
    pub review_id: String,
    pub rollback_root: String,
    pub reviewer_role: String,
    pub disposition: ReviewDisposition,
    pub evidence_root: String,
    pub redaction_root: String,
    pub reviewed_at_height: u64,
}

impl RollbackEvidenceReview {
    pub fn public_record(&self) -> Value {
        json!({
            "review_id": self.review_id,
            "rollback_root": self.rollback_root,
            "reviewer_role": self.reviewer_role,
            "disposition": self.disposition.as_str(),
            "evidence_root": self.evidence_root,
            "redaction_root": self.redaction_root,
            "reviewed_at_height": self.reviewed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-ROLLBACK-REVIEW", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("review_id", &self.review_id)?;
        ensure_non_empty("reviewer_role", &self.reviewer_role)?;
        ensure_root("rollback_root", &self.rollback_root)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("redaction_root", &self.redaction_root)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreatModelBlocker {
    pub blocker_id: String,
    pub area: String,
    pub status: ThreatModelStatus,
    pub mitigation_root: String,
    pub owner_commitment_root: String,
    pub blocks_release: bool,
}

impl ThreatModelBlocker {
    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "area": self.area,
            "status": self.status.as_str(),
            "mitigation_root": self.mitigation_root,
            "owner_commitment_root": self.owner_commitment_root,
            "blocks_release": self.blocks_release,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "INCIDENT-HANDOFF-THREAT-MODEL-BLOCKER",
            &self.public_record(),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_non_empty("area", &self.area)?;
        ensure_root("mitigation_root", &self.mitigation_root)?;
        ensure_root("owner_commitment_root", &self.owner_commitment_root)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivacyLeakCheck {
    pub check_id: String,
    pub boundary: PrivacyBoundary,
    pub sanitized_subject_root: String,
    pub leak_detected: bool,
    pub redaction_root: String,
    pub reviewer_root: String,
}

impl PrivacyLeakCheck {
    pub fn public_record(&self) -> Value {
        json!({
            "check_id": self.check_id,
            "boundary": self.boundary.as_str(),
            "sanitized_subject_root": self.sanitized_subject_root,
            "leak_detected": self.leak_detected,
            "redaction_root": self.redaction_root,
            "reviewer_root": self.reviewer_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-PRIVACY-LEAK-CHECK", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("check_id", &self.check_id)?;
        ensure_root("sanitized_subject_root", &self.sanitized_subject_root)?;
        ensure_root("redaction_root", &self.redaction_root)?;
        ensure_root("reviewer_root", &self.reviewer_root)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignerQuorumCheck {
    pub quorum_id: String,
    pub command_root: String,
    pub signer_set_root: String,
    pub required_signers: u64,
    pub observed_signers: u64,
    pub accepted: bool,
}

impl SignerQuorumCheck {
    pub fn public_record(&self) -> Value {
        json!({
            "quorum_id": self.quorum_id,
            "command_root": self.command_root,
            "signer_set_root": self.signer_set_root,
            "required_signers": self.required_signers,
            "observed_signers": self.observed_signers,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-SIGNER-QUORUM", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("quorum_id", &self.quorum_id)?;
        ensure_root("command_root", &self.command_root)?;
        ensure_root("signer_set_root", &self.signer_set_root)?;
        if self.required_signers == 0 {
            Err("required_signers must be greater than zero".to_string())
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeferredAuditRoot {
    pub audit_id: String,
    pub scope_root: String,
    pub deferral_reason_root: String,
    pub owner_root: String,
    pub due_height: u64,
    pub sealed: bool,
}

impl DeferredAuditRoot {
    pub fn public_record(&self) -> Value {
        json!({
            "audit_id": self.audit_id,
            "scope_root": self.scope_root,
            "deferral_reason_root": self.deferral_reason_root,
            "owner_root": self.owner_root,
            "due_height": self.due_height,
            "sealed": self.sealed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "INCIDENT-HANDOFF-DEFERRED-AUDIT-ROOT",
            &self.public_record(),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("audit_id", &self.audit_id)?;
        ensure_root("scope_root", &self.scope_root)?;
        ensure_root("deferral_reason_root", &self.deferral_reason_root)?;
        ensure_root("owner_root", &self.owner_root)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailClosedCommandTransfer {
    pub transfer_id: String,
    pub command: CommandTransferKind,
    pub command_root: String,
    pub previous_operator_root: String,
    pub incident_commander_root: String,
    pub signer_quorum_root: String,
    pub fail_closed: bool,
    pub signed: bool,
}

impl FailClosedCommandTransfer {
    pub fn public_record(&self) -> Value {
        json!({
            "transfer_id": self.transfer_id,
            "command": self.command.as_str(),
            "command_root": self.command_root,
            "previous_operator_root": self.previous_operator_root,
            "incident_commander_root": self.incident_commander_root,
            "signer_quorum_root": self.signer_quorum_root,
            "fail_closed": self.fail_closed,
            "signed": self.signed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "INCIDENT-HANDOFF-FAIL-CLOSED-COMMAND",
            &self.public_record(),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("transfer_id", &self.transfer_id)?;
        ensure_root("command_root", &self.command_root)?;
        ensure_root("previous_operator_root", &self.previous_operator_root)?;
        ensure_root("incident_commander_root", &self.incident_commander_root)?;
        ensure_root("signer_quorum_root", &self.signer_quorum_root)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HandoffCounters {
    pub rollback_reviews: u64,
    pub accepted_rollback_reviews: u64,
    pub threat_model_reviews: u64,
    pub open_threat_model_blockers: u64,
    pub privacy_checks: u64,
    pub privacy_leaks: u64,
    pub signer_quorums: u64,
    pub accepted_signer_quorums: u64,
    pub deferred_audit_roots: u64,
    pub sealed_deferred_audit_roots: u64,
    pub fail_closed_transfers: u64,
    pub unsigned_commands: u64,
    pub fail_closed_blockers: u64,
}

impl HandoffCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "rollback_reviews": self.rollback_reviews,
            "accepted_rollback_reviews": self.accepted_rollback_reviews,
            "threat_model_reviews": self.threat_model_reviews,
            "open_threat_model_blockers": self.open_threat_model_blockers,
            "privacy_checks": self.privacy_checks,
            "privacy_leaks": self.privacy_leaks,
            "signer_quorums": self.signer_quorums,
            "accepted_signer_quorums": self.accepted_signer_quorums,
            "deferred_audit_roots": self.deferred_audit_roots,
            "sealed_deferred_audit_roots": self.sealed_deferred_audit_roots,
            "fail_closed_transfers": self.fail_closed_transfers,
            "unsigned_commands": self.unsigned_commands,
            "fail_closed_blockers": self.fail_closed_blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncidentHandoffVerdict {
    pub verdict: HandoffVerdict,
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub blocker_root: String,
    pub lane_root: String,
    pub command_transfer_root: String,
    pub deferred_audit_root: String,
}

impl IncidentHandoffVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "blocker_root": self.blocker_root,
            "lane_root": self.lane_root,
            "command_transfer_root": self.command_transfer_root,
            "deferred_audit_root": self.deferred_audit_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-VERDICT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub rollback_reviews: BTreeMap<String, RollbackEvidenceReview>,
    pub threat_model_blockers: BTreeMap<String, ThreatModelBlocker>,
    pub privacy_leak_checks: BTreeMap<String, PrivacyLeakCheck>,
    pub signer_quorum_checks: BTreeMap<String, SignerQuorumCheck>,
    pub deferred_audit_roots: BTreeMap<String, DeferredAuditRoot>,
    pub fail_closed_transfers: BTreeMap<String, FailClosedCommandTransfer>,
    pub lane_roots: BTreeMap<String, String>,
    pub blockers: BTreeMap<String, Vec<IncidentBlockerKind>>,
    pub counters: HandoffCounters,
    pub verdict: IncidentHandoffVerdict,
}

impl State {
    pub fn new(
        config: Config,
        rollback_reviews: BTreeMap<String, RollbackEvidenceReview>,
        threat_model_blockers: BTreeMap<String, ThreatModelBlocker>,
        privacy_leak_checks: BTreeMap<String, PrivacyLeakCheck>,
        signer_quorum_checks: BTreeMap<String, SignerQuorumCheck>,
        deferred_audit_roots: BTreeMap<String, DeferredAuditRoot>,
        fail_closed_transfers: BTreeMap<String, FailClosedCommandTransfer>,
    ) -> Result<Self> {
        config.validate()?;
        ensure_capacity(rollback_reviews.len(), config.max_records)?;
        ensure_capacity(threat_model_blockers.len(), config.max_records)?;
        ensure_capacity(privacy_leak_checks.len(), config.max_records)?;
        ensure_capacity(signer_quorum_checks.len(), config.max_records)?;
        ensure_capacity(deferred_audit_roots.len(), config.max_records)?;
        ensure_capacity(fail_closed_transfers.len(), config.max_records)?;
        validate_records(&rollback_reviews, RollbackEvidenceReview::validate)?;
        validate_records(&threat_model_blockers, ThreatModelBlocker::validate)?;
        validate_records(&privacy_leak_checks, PrivacyLeakCheck::validate)?;
        validate_records(&signer_quorum_checks, SignerQuorumCheck::validate)?;
        validate_records(&deferred_audit_roots, DeferredAuditRoot::validate)?;
        validate_records(&fail_closed_transfers, FailClosedCommandTransfer::validate)?;

        let counters = build_counters(
            &rollback_reviews,
            &threat_model_blockers,
            &privacy_leak_checks,
            &signer_quorum_checks,
            &deferred_audit_roots,
            &fail_closed_transfers,
        );
        let blockers = build_blockers(&config, &counters);
        let lane_roots = build_lane_roots(
            &config,
            &rollback_reviews,
            &threat_model_blockers,
            &privacy_leak_checks,
            &signer_quorum_checks,
            &deferred_audit_roots,
            &fail_closed_transfers,
            &counters,
        );
        let verdict = build_verdict(&blockers, &lane_roots, &counters);

        Ok(Self {
            config,
            rollback_reviews,
            threat_model_blockers,
            privacy_leak_checks,
            signer_quorum_checks,
            deferred_audit_roots,
            fail_closed_transfers,
            lane_roots,
            blockers,
            counters,
            verdict,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "rollback_reviews": records_map(&self.rollback_reviews, RollbackEvidenceReview::public_record),
            "threat_model_blockers": records_map(&self.threat_model_blockers, ThreatModelBlocker::public_record),
            "privacy_leak_checks": records_map(&self.privacy_leak_checks, PrivacyLeakCheck::public_record),
            "signer_quorum_checks": records_map(&self.signer_quorum_checks, SignerQuorumCheck::public_record),
            "deferred_audit_roots": records_map(&self.deferred_audit_roots, DeferredAuditRoot::public_record),
            "fail_closed_transfers": records_map(&self.fail_closed_transfers, FailClosedCommandTransfer::public_record),
            "lane_roots": self.lane_roots,
            "blockers": blockers_record(&self.blockers),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-STATE", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.verdict.release_allowed && !self.blockers.is_empty() {
            Err("release cannot be allowed while incident handoff blockers remain".to_string())
        } else if self.verdict.release_allowed && !self.verdict.fail_closed {
            Err(
                "release cannot be allowed without fail-closed command transfer evidence"
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let rollback_reviews = devnet_rollback_reviews(&config);
    let threat_model_blockers = devnet_threat_model_blockers();
    let privacy_leak_checks = devnet_privacy_checks();
    let signer_quorum_checks = devnet_signer_quorums();
    let deferred_audit_roots = devnet_deferred_audit_roots();
    let fail_closed_transfers = devnet_fail_closed_transfers();
    match State::new(
        config,
        rollback_reviews,
        threat_model_blockers,
        privacy_leak_checks,
        signer_quorum_checks,
        deferred_audit_roots,
        fail_closed_transfers,
    ) {
        Ok(state) => state,
        Err(error) => fail_closed_state(error),
    }
}

fn devnet_rollback_reviews(config: &Config) -> BTreeMap<String, RollbackEvidenceReview> {
    (0..DEFAULT_MIN_ROLLBACK_REVIEWS)
        .map(|index| {
            let review_id = stable_id("rollback-review", &format!("wave86-{index}"));
            (
                review_id.clone(),
                RollbackEvidenceReview {
                    review_id,
                    rollback_root: config.rollback_drill_root.clone(),
                    reviewer_role: format!("security_reviewer_{index}"),
                    disposition: ReviewDisposition::AcceptedWithDeferredAudit,
                    evidence_root: fixture_root(&format!("rollback-evidence-review-{index}")),
                    redaction_root: fixture_root(&format!("rollback-redaction-review-{index}")),
                    reviewed_at_height: config.height + index,
                },
            )
        })
        .collect()
}

fn devnet_threat_model_blockers() -> BTreeMap<String, ThreatModelBlocker> {
    (0..DEFAULT_MIN_THREAT_MODEL_REVIEWS)
        .map(|index| {
            let blocker_id = stable_id("threat-model", &format!("wave86-{index}"));
            (
                blocker_id.clone(),
                ThreatModelBlocker {
                    blocker_id,
                    area: format!("force_exit_handoff_area_{index}"),
                    status: ThreatModelStatus::Mitigated,
                    mitigation_root: fixture_root(&format!("threat-model-mitigation-{index}")),
                    owner_commitment_root: fixture_root(&format!("threat-model-owner-{index}")),
                    blocks_release: false,
                },
            )
        })
        .collect()
}

fn devnet_privacy_checks() -> BTreeMap<String, PrivacyLeakCheck> {
    let boundaries = [
        PrivacyBoundary::MoneroAddress,
        PrivacyBoundary::PrivatePayload,
        PrivacyBoundary::RouteMetadata,
        PrivacyBoundary::WatchtowerObservation,
        PrivacyBoundary::OperatorDashboardExport,
        PrivacyBoundary::AuditArchive,
    ];
    boundaries
        .iter()
        .enumerate()
        .map(|(index, boundary)| {
            let check_id = stable_id("privacy-check", &format!("wave86-{index}"));
            (
                check_id.clone(),
                PrivacyLeakCheck {
                    check_id,
                    boundary: *boundary,
                    sanitized_subject_root: fixture_root(&format!("privacy-subject-{index}")),
                    leak_detected: false,
                    redaction_root: fixture_root(&format!("privacy-redaction-{index}")),
                    reviewer_root: fixture_root(&format!("privacy-reviewer-{index}")),
                },
            )
        })
        .collect()
}

fn devnet_signer_quorums() -> BTreeMap<String, SignerQuorumCheck> {
    (0..DEFAULT_MIN_SIGNER_QUORUMS)
        .map(|index| {
            let quorum_id = stable_id("signer-quorum", &format!("wave86-{index}"));
            (
                quorum_id.clone(),
                SignerQuorumCheck {
                    quorum_id,
                    command_root: fixture_root(&format!("quorum-command-{index}")),
                    signer_set_root: fixture_root(&format!("quorum-signer-set-{index}")),
                    required_signers: 3,
                    observed_signers: 4,
                    accepted: true,
                },
            )
        })
        .collect()
}

fn devnet_deferred_audit_roots() -> BTreeMap<String, DeferredAuditRoot> {
    (0..DEFAULT_MIN_DEFERRED_AUDIT_ROOTS)
        .map(|index| {
            let audit_id = stable_id("deferred-audit", &format!("wave86-{index}"));
            (
                audit_id.clone(),
                DeferredAuditRoot {
                    audit_id,
                    scope_root: fixture_root(&format!("deferred-audit-scope-{index}")),
                    deferral_reason_root: fixture_root(&format!("deferred-audit-reason-{index}")),
                    owner_root: fixture_root(&format!("deferred-audit-owner-{index}")),
                    due_height: DEFAULT_HEIGHT + 144 + index,
                    sealed: true,
                },
            )
        })
        .collect()
}

fn devnet_fail_closed_transfers() -> BTreeMap<String, FailClosedCommandTransfer> {
    let commands = [
        CommandTransferKind::FreezeReleaseWindow,
        CommandTransferKind::DisableExitPublish,
        CommandTransferKind::RevokeOperatorUnhold,
        CommandTransferKind::RequireIncidentCommanderAck,
    ];
    commands
        .iter()
        .enumerate()
        .map(|(index, command)| {
            let transfer_id = stable_id("command-transfer", &format!("wave86-{index}"));
            (
                transfer_id.clone(),
                FailClosedCommandTransfer {
                    transfer_id,
                    command: *command,
                    command_root: fixture_root(&format!("fail-closed-command-{index}")),
                    previous_operator_root: fixture_root(&format!("previous-operator-{index}")),
                    incident_commander_root: fixture_root(&format!("incident-commander-{index}")),
                    signer_quorum_root: fixture_root(&format!("command-quorum-{index}")),
                    fail_closed: true,
                    signed: true,
                },
            )
        })
        .collect()
}

fn fail_closed_state(error: String) -> State {
    let config = Config::devnet();
    let mut blockers = BTreeMap::new();
    blockers.insert(
        HandoffLane::IncidentVerdict.as_str().to_string(),
        vec![IncidentBlockerKind::HandoffRootMismatch],
    );
    let counters = HandoffCounters {
        rollback_reviews: 0,
        accepted_rollback_reviews: 0,
        threat_model_reviews: 0,
        open_threat_model_blockers: 1,
        privacy_checks: 0,
        privacy_leaks: 0,
        signer_quorums: 0,
        accepted_signer_quorums: 0,
        deferred_audit_roots: 0,
        sealed_deferred_audit_roots: 0,
        fail_closed_transfers: 0,
        unsigned_commands: 0,
        fail_closed_blockers: 1,
    };
    let mut lane_roots = BTreeMap::new();
    lane_roots.insert(
        HandoffLane::IncidentVerdict.as_str().to_string(),
        fixture_root(&format!("fail-closed-{error}")),
    );
    let verdict = build_verdict(&blockers, &lane_roots, &counters);
    State {
        config,
        rollback_reviews: BTreeMap::new(),
        threat_model_blockers: BTreeMap::new(),
        privacy_leak_checks: BTreeMap::new(),
        signer_quorum_checks: BTreeMap::new(),
        deferred_audit_roots: BTreeMap::new(),
        fail_closed_transfers: BTreeMap::new(),
        lane_roots,
        blockers,
        counters,
        verdict,
    }
}

fn build_counters(
    rollback_reviews: &BTreeMap<String, RollbackEvidenceReview>,
    threat_model_blockers: &BTreeMap<String, ThreatModelBlocker>,
    privacy_leak_checks: &BTreeMap<String, PrivacyLeakCheck>,
    signer_quorum_checks: &BTreeMap<String, SignerQuorumCheck>,
    deferred_audit_roots: &BTreeMap<String, DeferredAuditRoot>,
    fail_closed_transfers: &BTreeMap<String, FailClosedCommandTransfer>,
) -> HandoffCounters {
    let open_threat_model_blockers = threat_model_blockers
        .values()
        .filter(|item| item.status.blocks_release() || item.blocks_release)
        .count() as u64;
    let privacy_leaks = privacy_leak_checks
        .values()
        .filter(|item| item.leak_detected)
        .count() as u64;
    let unsigned_commands = fail_closed_transfers
        .values()
        .filter(|item| !item.signed)
        .count() as u64;
    let fail_closed_blockers = fail_closed_transfers
        .values()
        .filter(|item| !item.fail_closed)
        .count() as u64;

    HandoffCounters {
        rollback_reviews: rollback_reviews.len() as u64,
        accepted_rollback_reviews: rollback_reviews
            .values()
            .filter(|item| item.disposition.passes())
            .count() as u64,
        threat_model_reviews: threat_model_blockers.len() as u64,
        open_threat_model_blockers,
        privacy_checks: privacy_leak_checks.len() as u64,
        privacy_leaks,
        signer_quorums: signer_quorum_checks.len() as u64,
        accepted_signer_quorums: signer_quorum_checks
            .values()
            .filter(|item| item.accepted && item.observed_signers >= item.required_signers)
            .count() as u64,
        deferred_audit_roots: deferred_audit_roots.len() as u64,
        sealed_deferred_audit_roots: deferred_audit_roots
            .values()
            .filter(|item| item.sealed)
            .count() as u64,
        fail_closed_transfers: fail_closed_transfers
            .values()
            .filter(|item| item.fail_closed && item.signed)
            .count() as u64,
        unsigned_commands,
        fail_closed_blockers,
    }
}

fn build_blockers(
    config: &Config,
    counters: &HandoffCounters,
) -> BTreeMap<String, Vec<IncidentBlockerKind>> {
    let mut blockers = BTreeMap::new();
    insert_if_any(
        &mut blockers,
        HandoffLane::RollbackEvidenceReview,
        threshold_blockers(
            counters.accepted_rollback_reviews,
            config.min_rollback_reviews,
            IncidentBlockerKind::MissingRollbackEvidenceReview,
        ),
    );
    insert_if_any(
        &mut blockers,
        HandoffLane::ThreatModelBlocker,
        threshold_blockers(
            counters.threat_model_reviews,
            config.min_threat_model_reviews,
            IncidentBlockerKind::OpenThreatModelBlocker,
        )
        .into_iter()
        .chain(limit_blockers(
            counters.open_threat_model_blockers,
            config.max_open_blockers,
            IncidentBlockerKind::OpenThreatModelBlocker,
        ))
        .collect(),
    );
    insert_if_any(
        &mut blockers,
        HandoffLane::PrivacyLeakCheck,
        threshold_blockers(
            counters.privacy_checks,
            config.min_privacy_checks,
            IncidentBlockerKind::MissingPrivacyLeakCheck,
        )
        .into_iter()
        .chain(limit_blockers(
            counters.privacy_leaks,
            config.max_privacy_leaks,
            IncidentBlockerKind::PrivacyLeakObserved,
        ))
        .collect(),
    );
    insert_if_any(
        &mut blockers,
        HandoffLane::SignerQuorumCheck,
        threshold_blockers(
            counters.accepted_signer_quorums,
            config.min_signer_quorums,
            IncidentBlockerKind::MissingSignerQuorum,
        )
        .into_iter()
        .chain(limit_blockers(
            counters.signer_quorums - counters.accepted_signer_quorums,
            config.max_unsigned_commands,
            IncidentBlockerKind::SignerQuorumRejected,
        ))
        .collect(),
    );
    insert_if_any(
        &mut blockers,
        HandoffLane::DeferredAuditRoot,
        threshold_blockers(
            counters.sealed_deferred_audit_roots,
            config.min_deferred_audit_roots,
            IncidentBlockerKind::MissingDeferredAuditRoot,
        )
        .into_iter()
        .chain(limit_blockers(
            counters.deferred_audit_roots - counters.sealed_deferred_audit_roots,
            0,
            IncidentBlockerKind::DeferredAuditRootUnsealed,
        ))
        .collect(),
    );
    insert_if_any(
        &mut blockers,
        HandoffLane::FailClosedCommandTransfer,
        threshold_blockers(
            counters.fail_closed_transfers,
            config.min_fail_closed_transfers,
            IncidentBlockerKind::MissingFailClosedTransfer,
        )
        .into_iter()
        .chain(limit_blockers(
            counters.fail_closed_blockers,
            0,
            IncidentBlockerKind::CommandTransferNotFailClosed,
        ))
        .chain(limit_blockers(
            counters.unsigned_commands,
            config.max_unsigned_commands,
            IncidentBlockerKind::UnsignedCommandTransfer,
        ))
        .collect(),
    );
    blockers
}

fn build_lane_roots(
    config: &Config,
    rollback_reviews: &BTreeMap<String, RollbackEvidenceReview>,
    threat_model_blockers: &BTreeMap<String, ThreatModelBlocker>,
    privacy_leak_checks: &BTreeMap<String, PrivacyLeakCheck>,
    signer_quorum_checks: &BTreeMap<String, SignerQuorumCheck>,
    deferred_audit_roots: &BTreeMap<String, DeferredAuditRoot>,
    fail_closed_transfers: &BTreeMap<String, FailClosedCommandTransfer>,
    counters: &HandoffCounters,
) -> BTreeMap<String, String> {
    let mut lane_roots = BTreeMap::new();
    lane_roots.insert(
        HandoffLane::RollbackEvidenceReview.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-ROLLBACK-REVIEWS",
            rollback_reviews,
            RollbackEvidenceReview::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::ThreatModelBlocker.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-THREAT-MODEL",
            threat_model_blockers,
            ThreatModelBlocker::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::PrivacyLeakCheck.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-PRIVACY-CHECKS",
            privacy_leak_checks,
            PrivacyLeakCheck::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::SignerQuorumCheck.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-SIGNER-QUORUMS",
            signer_quorum_checks,
            SignerQuorumCheck::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::DeferredAuditRoot.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-DEFERRED-AUDIT",
            deferred_audit_roots,
            DeferredAuditRoot::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::FailClosedCommandTransfer.as_str().to_string(),
        map_root(
            "INCIDENT-HANDOFF-LANE-FAIL-CLOSED-COMMANDS",
            fail_closed_transfers,
            FailClosedCommandTransfer::state_root,
        ),
    );
    lane_roots.insert(
        HandoffLane::IncidentVerdict.as_str().to_string(),
        domain_hash(
            "INCIDENT-HANDOFF-LANE-VERDICT",
            &[
                HashPart::Str(&config.state_root()),
                HashPart::Str(&counters.state_root()),
            ],
        ),
    );
    lane_roots
}

fn build_verdict(
    blockers: &BTreeMap<String, Vec<IncidentBlockerKind>>,
    lane_roots: &BTreeMap<String, String>,
    counters: &HandoffCounters,
) -> IncidentHandoffVerdict {
    let has_blockers = blockers.values().any(|items| !items.is_empty());
    let verdict = if has_blockers {
        HandoffVerdict::Hold
    } else {
        HandoffVerdict::TransferAccepted
    };
    let lane_root = map_string_root("INCIDENT-HANDOFF-LANE-ROOTS", lane_roots);
    let command_transfer_root =
        match lane_roots.get(HandoffLane::FailClosedCommandTransfer.as_str()) {
            Some(root) => root.clone(),
            None => fixture_root("missing-command-transfer-root"),
        };
    let deferred_audit_root = match lane_roots.get(HandoffLane::DeferredAuditRoot.as_str()) {
        Some(root) => root.clone(),
        None => fixture_root("missing-deferred-audit-root"),
    };
    IncidentHandoffVerdict {
        verdict,
        release_allowed: verdict.allows_release() && !has_blockers,
        fail_closed: counters.fail_closed_transfers > 0 && counters.fail_closed_blockers == 0,
        blocker_root: blockers_root(blockers),
        lane_root,
        command_transfer_root,
        deferred_audit_root,
    }
}

fn insert_if_any(
    blockers: &mut BTreeMap<String, Vec<IncidentBlockerKind>>,
    lane: HandoffLane,
    lane_blockers: Vec<IncidentBlockerKind>,
) {
    if !lane_blockers.is_empty() {
        blockers.insert(lane.as_str().to_string(), dedup_blockers(lane_blockers));
    }
}

fn threshold_blockers(
    observed: u64,
    required: u64,
    blocker: IncidentBlockerKind,
) -> Vec<IncidentBlockerKind> {
    if observed < required {
        vec![blocker]
    } else {
        Vec::new()
    }
}

fn limit_blockers(
    observed: u64,
    allowed: u64,
    blocker: IncidentBlockerKind,
) -> Vec<IncidentBlockerKind> {
    if observed > allowed {
        vec![blocker]
    } else {
        Vec::new()
    }
}

fn dedup_blockers(blockers: Vec<IncidentBlockerKind>) -> Vec<IncidentBlockerKind> {
    let mut seen = BTreeSet::new();
    blockers
        .into_iter()
        .filter(|blocker| seen.insert(*blocker))
        .collect()
}

fn validate_records<T, F>(records: &BTreeMap<String, T>, validate: F) -> Result<()>
where
    F: Fn(&T) -> Result<()>,
{
    for record in records.values() {
        validate(record)?;
    }
    Ok(())
}

fn records_map<T, F>(records: &BTreeMap<String, T>, public_record: F) -> BTreeMap<String, Value>
where
    F: Fn(&T) -> Value,
{
    records
        .iter()
        .map(|(key, record)| (key.clone(), public_record(record)))
        .collect()
}

fn blockers_record(
    blockers: &BTreeMap<String, Vec<IncidentBlockerKind>>,
) -> BTreeMap<String, Vec<&'static str>> {
    blockers
        .iter()
        .map(|(lane, lane_blockers)| {
            (
                lane.clone(),
                lane_blockers
                    .iter()
                    .map(|blocker| blocker.as_str())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn blockers_root(blockers: &BTreeMap<String, Vec<IncidentBlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .flat_map(|(lane, lane_blockers)| {
            lane_blockers.iter().map(move |blocker| {
                domain_hash(
                    "INCIDENT-HANDOFF-BLOCKER",
                    &[
                        HashPart::Str(lane),
                        HashPart::Str(blocker.as_str()),
                        HashPart::Str(if blocker.fail_closed() {
                            "fail_closed"
                        } else {
                            "open"
                        }),
                    ],
                )
            })
        })
        .collect::<Vec<_>>();
    merkle_or_empty("INCIDENT-HANDOFF-BLOCKERS", leaves)
}

fn map_root<T, F>(label: &str, records: &BTreeMap<String, T>, state_root: F) -> String
where
    F: Fn(&T) -> String,
{
    let leaves = records
        .iter()
        .map(|(key, record)| {
            domain_hash(
                "INCIDENT-HANDOFF-MAP-LEAF",
                &[HashPart::Str(key), HashPart::Str(&state_root(record))],
            )
        })
        .collect::<Vec<_>>();
    merkle_or_empty(label, leaves)
}

fn map_string_root(label: &str, records: &BTreeMap<String, String>) -> String {
    let leaves = records
        .iter()
        .map(|(key, value)| {
            domain_hash(
                "INCIDENT-HANDOFF-STRING-MAP-LEAF",
                &[HashPart::Str(key), HashPart::Str(value)],
            )
        })
        .collect::<Vec<_>>();
    merkle_or_empty(label, leaves)
}

fn merkle_or_empty(label: &str, leaves: Vec<String>) -> String {
    if leaves.is_empty() {
        domain_hash(label, &[HashPart::Str("empty")])
    } else {
        merkle_root(label, &leaves)
    }
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        label,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
    )
}

fn fixture_root(label: &str) -> String {
    domain_hash(
        "INCIDENT-HANDOFF-FIXTURE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
    )
}

fn stable_id(prefix: &str, label: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        domain_hash(
            "INCIDENT-HANDOFF-STABLE-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(prefix),
                HashPart::Str(label)
            ]
        )
    )
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(format!("{label} must not be empty"))
    } else {
        Ok(())
    }
}

fn ensure_root(label: &str, value: &str) -> Result<()> {
    ensure_non_empty(label, value)?;
    if value.len() < 16 {
        Err(format!("{label} must look like a deterministic root"))
    } else {
        Ok(())
    }
}

fn ensure_capacity(current: usize, max: usize) -> Result<()> {
    if current > max {
        Err(format!(
            "incident handoff record capacity exceeded: {current} > {max}"
        ))
    } else {
        Ok(())
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}
