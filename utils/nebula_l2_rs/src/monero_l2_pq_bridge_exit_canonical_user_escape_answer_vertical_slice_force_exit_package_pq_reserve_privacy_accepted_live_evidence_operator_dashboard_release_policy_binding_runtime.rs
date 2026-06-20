use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqReservePrivacyAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-reserve-privacy-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const DEFAULT_HEIGHT: u64 = 94_000;
pub const DEFAULT_WAVE_NUMBER: u16 = 83;
pub const DEFAULT_SOURCE_WAVE_NUMBER: u16 = 82;
pub const DEFAULT_KEY_EPOCH: u64 = 82;
pub const DEFAULT_MIN_PQ_SECURITY_BITS: u16 = 128;
pub const DEFAULT_MIN_ROTATION_SIGNERS: u16 = 5;
pub const DEFAULT_MIN_ROTATION_WEIGHT_BPS: u16 = 6_700;
pub const DEFAULT_MIN_REVIEWER_QUORUM: u16 = 3;
pub const DEFAULT_MIN_APPROVAL_COUNT: u16 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u16 = 10_500;
pub const DEFAULT_MIN_RESERVE_PROOF_COUNT: u16 = 3;
pub const DEFAULT_MIN_PRIVACY_SET_SIZE: u64 = 128;
pub const DEFAULT_MAX_PRIVACY_BUDGET_BPS: u16 = 2_500;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_BOUND_DASHBOARD_CELLS: u16 = 6;
pub const DEFAULT_PACKAGE_ID: &str = "force-exit-package-pq-reserve-privacy";
pub const DEFAULT_OPERATOR_DASHBOARD_ID: &str =
    "wave-82-pq-reserve-privacy-accepted-live-evidence-operator-dashboard";
pub const DEFAULT_RELEASE_POLICY_ID: &str = "wave-83-pq-reserve-privacy-release-policy-binding";
pub const STATUS_ACCEPTED: &str = "accepted";
pub const STATUS_PENDING: &str = "pending";
pub const STATUS_BLOCKED: &str = "blocked";
pub const STATUS_REJECTED: &str = "rejected";
pub const VERDICT_GO: &str = "go";
pub const VERDICT_NO_GO: &str = "no_go";
pub const VERDICT_FAIL_CLOSED: &str = "fail_closed";

const REQUIRED_GATES: &[ReleaseGate] = &[
    ReleaseGate::PqRotation,
    ReleaseGate::ReserveCoverage,
    ReleaseGate::PrivacyBoundary,
    ReleaseGate::OperatorDashboard,
    ReleaseGate::ReviewerQuorum,
    ReleaseGate::BlockerHandling,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseGate {
    PqRotation,
    ReserveCoverage,
    PrivacyBoundary,
    OperatorDashboard,
    ReviewerQuorum,
    BlockerHandling,
}

impl ReleaseGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqRotation => "pq_rotation",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyBoundary => "privacy_boundary",
            Self::OperatorDashboard => "operator_dashboard",
            Self::ReviewerQuorum => "reviewer_quorum",
            Self::BlockerHandling => "blocker_handling",
        }
    }

    pub fn policy_label(self) -> &'static str {
        match self {
            Self::PqRotation => "pq_rotation_quorum_receipts",
            Self::ReserveCoverage => "reserve_coverage_proofs",
            Self::PrivacyBoundary => "privacy_boundary_receipts",
            Self::OperatorDashboard => "operator_dashboard_approval_binding",
            Self::ReviewerQuorum => "release_reviewer_quorum",
            Self::BlockerHandling => "fail_closed_blocker_handling",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingPqRotationReceipt,
    PqRotationQuorumShortfall,
    PqRotationEpochMismatch,
    PqSecurityBitsShortfall,
    ReserveCoverageShortfall,
    ReserveProofQuorumShortfall,
    PrivacyBudgetExceeded,
    PrivacySetTooSmall,
    PrivacyBoundaryRejected,
    DashboardRootMismatch,
    DashboardApprovalShortfall,
    ReviewerQuorumShortfall,
    ReviewerRootMismatch,
    OpenDashboardAction,
    UnresolvedBlocker,
    StaleEvidence,
    ReleasePolicyMismatch,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingPqRotationReceipt => "missing_pq_rotation_receipt",
            Self::PqRotationQuorumShortfall => "pq_rotation_quorum_shortfall",
            Self::PqRotationEpochMismatch => "pq_rotation_epoch_mismatch",
            Self::PqSecurityBitsShortfall => "pq_security_bits_shortfall",
            Self::ReserveCoverageShortfall => "reserve_coverage_shortfall",
            Self::ReserveProofQuorumShortfall => "reserve_proof_quorum_shortfall",
            Self::PrivacyBudgetExceeded => "privacy_budget_exceeded",
            Self::PrivacySetTooSmall => "privacy_set_too_small",
            Self::PrivacyBoundaryRejected => "privacy_boundary_rejected",
            Self::DashboardRootMismatch => "dashboard_root_mismatch",
            Self::DashboardApprovalShortfall => "dashboard_approval_shortfall",
            Self::ReviewerQuorumShortfall => "reviewer_quorum_shortfall",
            Self::ReviewerRootMismatch => "reviewer_root_mismatch",
            Self::OpenDashboardAction => "open_dashboard_action",
            Self::UnresolvedBlocker => "unresolved_blocker",
            Self::StaleEvidence => "stale_evidence",
            Self::ReleasePolicyMismatch => "release_policy_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Accepted,
    Pending,
    Blocked,
    Rejected,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => STATUS_ACCEPTED,
            Self::Pending => STATUS_PENDING,
            Self::Blocked => STATUS_BLOCKED,
            Self::Rejected => STATUS_REJECTED,
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn blocking(self) -> bool {
        matches!(self, Self::Blocked | Self::Rejected)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub release_policy_id: String,
    pub operator_dashboard_id: String,
    pub force_exit_package_id: String,
    pub expected_dashboard_root: String,
    pub expected_runbook_root: String,
    pub expected_imported_evidence_root: String,
    pub expected_key_epoch: u64,
    pub min_pq_security_bits: u16,
    pub min_rotation_signers: u16,
    pub min_rotation_weight_bps: u16,
    pub min_reviewer_quorum: u16,
    pub min_approval_count: u16,
    pub min_reserve_coverage_bps: u16,
    pub min_reserve_proof_count: u16,
    pub min_privacy_set_size: u64,
    pub max_privacy_budget_bps: u16,
    pub max_evidence_age_blocks: u64,
    pub min_bound_dashboard_cells: u16,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        let release_policy_id = runtime_id(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(DEFAULT_RELEASE_POLICY_ID),
                HashPart::Int(DEFAULT_WAVE_NUMBER as i128),
            ],
        );
        let expected_runbook_root = sample_root("wave-82-operator-runbook-audit-root");
        let expected_imported_evidence_root = sample_root("wave-82-accepted-live-evidence-root");
        let expected_dashboard_root = runtime_id(
            "PQ-RESERVE-PRIVACY-EXPECTED-DASHBOARD-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(&release_policy_id),
                HashPart::Str(&expected_runbook_root),
                HashPart::Str(&expected_imported_evidence_root),
            ],
        );
        Self {
            release_policy_id,
            operator_dashboard_id: DEFAULT_OPERATOR_DASHBOARD_ID.to_string(),
            force_exit_package_id: DEFAULT_PACKAGE_ID.to_string(),
            expected_dashboard_root,
            expected_runbook_root,
            expected_imported_evidence_root,
            expected_key_epoch: DEFAULT_KEY_EPOCH,
            min_pq_security_bits: DEFAULT_MIN_PQ_SECURITY_BITS,
            min_rotation_signers: DEFAULT_MIN_ROTATION_SIGNERS,
            min_rotation_weight_bps: DEFAULT_MIN_ROTATION_WEIGHT_BPS,
            min_reviewer_quorum: DEFAULT_MIN_REVIEWER_QUORUM,
            min_approval_count: DEFAULT_MIN_APPROVAL_COUNT,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_proof_count: DEFAULT_MIN_RESERVE_PROOF_COUNT,
            min_privacy_set_size: DEFAULT_MIN_PRIVACY_SET_SIZE,
            max_privacy_budget_bps: DEFAULT_MAX_PRIVACY_BUDGET_BPS,
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            min_bound_dashboard_cells: DEFAULT_MIN_BOUND_DASHBOARD_CELLS,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("release_policy_id", &self.release_policy_id)?;
        ensure_non_empty("operator_dashboard_id", &self.operator_dashboard_id)?;
        ensure_non_empty("force_exit_package_id", &self.force_exit_package_id)?;
        ensure_root("expected_dashboard_root", &self.expected_dashboard_root)?;
        ensure_root("expected_runbook_root", &self.expected_runbook_root)?;
        ensure_root(
            "expected_imported_evidence_root",
            &self.expected_imported_evidence_root,
        )?;
        ensure(
            self.expected_key_epoch > 0,
            "expected key epoch must be non-zero",
        )?;
        ensure(
            self.min_pq_security_bits > 0,
            "minimum pq security bits must be non-zero",
        )?;
        ensure(
            self.min_rotation_signers > 0,
            "minimum rotation signers must be non-zero",
        )?;
        ensure_bps(self.min_rotation_weight_bps, "minimum rotation weight bps")?;
        ensure(
            self.min_reviewer_quorum > 0,
            "reviewer quorum must be non-zero",
        )?;
        ensure(
            self.min_approval_count > 0,
            "approval count must be non-zero",
        )?;
        ensure(
            self.min_reserve_coverage_bps > 0,
            "minimum reserve coverage must be non-zero",
        )?;
        ensure(
            self.min_reserve_proof_count > 0,
            "minimum reserve proof count must be non-zero",
        )?;
        ensure(
            self.min_privacy_set_size > 0,
            "minimum privacy set size must be non-zero",
        )?;
        ensure_bps(self.max_privacy_budget_bps, "maximum privacy budget bps")?;
        ensure(
            self.max_evidence_age_blocks > 0,
            "maximum evidence age must be non-zero",
        )?;
        ensure(
            self.min_bound_dashboard_cells > 0,
            "minimum dashboard cells must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "pq_reserve_privacy_release_policy_binding_config",
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "release_policy_id": self.release_policy_id,
            "operator_dashboard_id": self.operator_dashboard_id,
            "force_exit_package_id": self.force_exit_package_id,
            "expected_dashboard_root": self.expected_dashboard_root,
            "expected_runbook_root": self.expected_runbook_root,
            "expected_imported_evidence_root": self.expected_imported_evidence_root,
            "expected_key_epoch": self.expected_key_epoch,
            "min_pq_security_bits": self.min_pq_security_bits,
            "min_rotation_signers": self.min_rotation_signers,
            "min_rotation_weight_bps": self.min_rotation_weight_bps,
            "min_reviewer_quorum": self.min_reviewer_quorum,
            "min_approval_count": self.min_approval_count,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_reserve_proof_count": self.min_reserve_proof_count,
            "min_privacy_set_size": self.min_privacy_set_size,
            "max_privacy_budget_bps": self.max_privacy_budget_bps,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "min_bound_dashboard_cells": self.min_bound_dashboard_cells,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn policy_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BINDING-CONFIG",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqRotationReceipt {
    pub receipt_id: String,
    pub key_epoch: u64,
    pub rotation_transcript_root: String,
    pub active_keyset_root: String,
    pub retired_keyset_root: String,
    pub quorum_receipt_root: String,
    pub signer_ids: Vec<String>,
    pub signer_weight_bps: u16,
    pub threshold_bps: u16,
    pub security_bits: u16,
    pub observed_at_height: u64,
    pub status: ReceiptStatus,
}

impl PqRotationReceipt {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        key_epoch: u64,
        rotation_transcript_root: &str,
        active_keyset_root: &str,
        retired_keyset_root: &str,
        signer_ids: Vec<String>,
        signer_weight_bps: u16,
        threshold_bps: u16,
        security_bits: u16,
        observed_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let quorum_receipt_root = runtime_id(
            "PQ-ROTATION-QUORUM-RECEIPT-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Int(key_epoch as i128),
                HashPart::Str(rotation_transcript_root),
                HashPart::Str(active_keyset_root),
                HashPart::Str(retired_keyset_root),
            ],
        );
        let receipt_id = runtime_id(
            "PQ-ROTATION-RECEIPT-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Int(key_epoch as i128),
                HashPart::Str(&quorum_receipt_root),
            ],
        );
        let receipt = Self {
            receipt_id,
            key_epoch,
            rotation_transcript_root: rotation_transcript_root.to_string(),
            active_keyset_root: active_keyset_root.to_string(),
            retired_keyset_root: retired_keyset_root.to_string(),
            quorum_receipt_root,
            signer_ids: sorted_unique(signer_ids),
            signer_weight_bps,
            threshold_bps,
            security_bits,
            observed_at_height,
            status,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("pq rotation receipt id", &self.receipt_id)?;
        ensure(self.key_epoch > 0, "pq key epoch must be non-zero")?;
        ensure_root("rotation_transcript_root", &self.rotation_transcript_root)?;
        ensure_root("active_keyset_root", &self.active_keyset_root)?;
        ensure_root("retired_keyset_root", &self.retired_keyset_root)?;
        ensure_root("quorum_receipt_root", &self.quorum_receipt_root)?;
        ensure_unique_non_empty("signer_ids", &self.signer_ids)?;
        ensure_bps(self.signer_weight_bps, "signer weight bps")?;
        ensure_bps(self.threshold_bps, "threshold bps")?;
        ensure(self.security_bits > 0, "security bits must be non-zero")?;
        ensure(
            self.observed_at_height > 0,
            "observed height must be non-zero",
        )
    }

    pub fn satisfies(&self, config: &Config, height: u64) -> bool {
        self.status.accepted()
            && self.key_epoch == config.expected_key_epoch
            && self.signer_ids.len() >= usize::from(config.min_rotation_signers)
            && self.signer_weight_bps >= config.min_rotation_weight_bps
            && self.signer_weight_bps >= self.threshold_bps
            && self.security_bits >= config.min_pq_security_bits
            && !self.is_stale(config, height)
    }

    pub fn is_stale(&self, config: &Config, height: u64) -> bool {
        height.saturating_sub(self.observed_at_height) > config.max_evidence_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "pq_rotation_receipt",
            "receipt_id": self.receipt_id,
            "key_epoch": self.key_epoch,
            "rotation_transcript_root": self.rotation_transcript_root,
            "active_keyset_root": self.active_keyset_root,
            "retired_keyset_root": self.retired_keyset_root,
            "quorum_receipt_root": self.quorum_receipt_root,
            "signer_ids": self.signer_ids,
            "signer_count": self.signer_ids.len(),
            "signer_weight_bps": self.signer_weight_bps,
            "threshold_bps": self.threshold_bps,
            "security_bits": self.security_bits,
            "observed_at_height": self.observed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-PQ-ROTATION-RECEIPT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReserveCoverageProof {
    pub proof_id: String,
    pub reserve_asset: String,
    pub liability_root: String,
    pub reserve_root: String,
    pub auditor_root: String,
    pub required_atomic_units: u128,
    pub covered_atomic_units: u128,
    pub coverage_bps: u16,
    pub signer_ids: Vec<String>,
    pub observed_at_height: u64,
    pub status: ReceiptStatus,
}

impl ReserveCoverageProof {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reserve_asset: &str,
        liability_root: &str,
        reserve_root: &str,
        auditor_root: &str,
        required_atomic_units: u128,
        covered_atomic_units: u128,
        signer_ids: Vec<String>,
        observed_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let coverage_bps = coverage_bps(covered_atomic_units, required_atomic_units);
        let proof_id = runtime_id(
            "PQ-RESERVE-PRIVACY-RESERVE-PROOF-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(reserve_asset),
                HashPart::Str(liability_root),
                HashPart::Str(reserve_root),
                HashPart::Int(clamped_i128(covered_atomic_units)),
            ],
        );
        let proof = Self {
            proof_id,
            reserve_asset: reserve_asset.to_string(),
            liability_root: liability_root.to_string(),
            reserve_root: reserve_root.to_string(),
            auditor_root: auditor_root.to_string(),
            required_atomic_units,
            covered_atomic_units,
            coverage_bps,
            signer_ids: sorted_unique(signer_ids),
            observed_at_height,
            status,
        };
        proof.validate()?;
        Ok(proof)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("reserve proof id", &self.proof_id)?;
        ensure_non_empty("reserve asset", &self.reserve_asset)?;
        ensure_root("liability_root", &self.liability_root)?;
        ensure_root("reserve_root", &self.reserve_root)?;
        ensure_root("auditor_root", &self.auditor_root)?;
        ensure(
            self.required_atomic_units > 0,
            "required atomic units must be non-zero",
        )?;
        ensure(
            self.covered_atomic_units > 0,
            "covered atomic units must be non-zero",
        )?;
        ensure_unique_non_empty("reserve proof signers", &self.signer_ids)?;
        ensure(
            self.observed_at_height > 0,
            "reserve proof observed height must be non-zero",
        )
    }

    pub fn satisfies(&self, config: &Config, height: u64) -> bool {
        self.status.accepted()
            && self.coverage_bps >= config.min_reserve_coverage_bps
            && !self.is_stale(config, height)
    }

    pub fn is_stale(&self, config: &Config, height: u64) -> bool {
        height.saturating_sub(self.observed_at_height) > config.max_evidence_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "reserve_coverage_proof",
            "proof_id": self.proof_id,
            "reserve_asset": self.reserve_asset,
            "liability_root": self.liability_root,
            "reserve_root": self.reserve_root,
            "auditor_root": self.auditor_root,
            "required_atomic_units": self.required_atomic_units.to_string(),
            "covered_atomic_units": self.covered_atomic_units.to_string(),
            "coverage_bps": self.coverage_bps,
            "signer_ids": self.signer_ids,
            "signer_count": self.signer_ids.len(),
            "observed_at_height": self.observed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-RESERVE-COVERAGE-PROOF",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyBoundaryReceipt {
    pub receipt_id: String,
    pub boundary_label: String,
    pub privacy_policy_root: String,
    pub non_linkage_root: String,
    pub disclosure_control_root: String,
    pub privacy_set_size: u64,
    pub consumed_budget_bps: u16,
    pub reviewer_ids: Vec<String>,
    pub observed_at_height: u64,
    pub status: ReceiptStatus,
}

impl PrivacyBoundaryReceipt {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        boundary_label: &str,
        privacy_policy_root: &str,
        non_linkage_root: &str,
        disclosure_control_root: &str,
        privacy_set_size: u64,
        consumed_budget_bps: u16,
        reviewer_ids: Vec<String>,
        observed_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let receipt_id = runtime_id(
            "PQ-RESERVE-PRIVACY-BOUNDARY-RECEIPT-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(boundary_label),
                HashPart::Str(privacy_policy_root),
                HashPart::Str(non_linkage_root),
            ],
        );
        let receipt = Self {
            receipt_id,
            boundary_label: boundary_label.to_string(),
            privacy_policy_root: privacy_policy_root.to_string(),
            non_linkage_root: non_linkage_root.to_string(),
            disclosure_control_root: disclosure_control_root.to_string(),
            privacy_set_size,
            consumed_budget_bps,
            reviewer_ids: sorted_unique(reviewer_ids),
            observed_at_height,
            status,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("privacy boundary receipt id", &self.receipt_id)?;
        ensure_non_empty("privacy boundary label", &self.boundary_label)?;
        ensure_root("privacy_policy_root", &self.privacy_policy_root)?;
        ensure_root("non_linkage_root", &self.non_linkage_root)?;
        ensure_root("disclosure_control_root", &self.disclosure_control_root)?;
        ensure(
            self.privacy_set_size > 0,
            "privacy set size must be non-zero",
        )?;
        ensure_bps(self.consumed_budget_bps, "consumed privacy budget bps")?;
        ensure_unique_non_empty("privacy boundary reviewers", &self.reviewer_ids)?;
        ensure(
            self.observed_at_height > 0,
            "privacy receipt observed height must be non-zero",
        )
    }

    pub fn satisfies(&self, config: &Config, height: u64) -> bool {
        self.status.accepted()
            && self.privacy_set_size >= config.min_privacy_set_size
            && self.consumed_budget_bps <= config.max_privacy_budget_bps
            && !self.is_stale(config, height)
    }

    pub fn is_stale(&self, config: &Config, height: u64) -> bool {
        height.saturating_sub(self.observed_at_height) > config.max_evidence_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "privacy_boundary_receipt",
            "receipt_id": self.receipt_id,
            "boundary_label": self.boundary_label,
            "privacy_policy_root": self.privacy_policy_root,
            "non_linkage_root": self.non_linkage_root,
            "disclosure_control_root": self.disclosure_control_root,
            "privacy_set_size": self.privacy_set_size,
            "consumed_budget_bps": self.consumed_budget_bps,
            "reviewer_ids": self.reviewer_ids,
            "reviewer_count": self.reviewer_ids.len(),
            "observed_at_height": self.observed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("PQ-RESERVE-PRIVACY-BOUNDARY-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardApproval {
    pub approval_id: String,
    pub approver_id: String,
    pub role: String,
    pub dashboard_root: String,
    pub runbook_root: String,
    pub imported_evidence_root: String,
    pub approval_root: String,
    pub approved_at_height: u64,
    pub status: ReceiptStatus,
}

impl DashboardApproval {
    pub fn new(
        approver_id: &str,
        role: &str,
        dashboard_root: &str,
        runbook_root: &str,
        imported_evidence_root: &str,
        approved_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let approval_root = runtime_id(
            "PQ-RESERVE-PRIVACY-DASHBOARD-APPROVAL-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(approver_id),
                HashPart::Str(role),
                HashPart::Str(dashboard_root),
            ],
        );
        let approval_id = runtime_id(
            "PQ-RESERVE-PRIVACY-DASHBOARD-APPROVAL-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(&approval_root),
                HashPart::Int(approved_at_height as i128),
            ],
        );
        let approval = Self {
            approval_id,
            approver_id: approver_id.to_string(),
            role: role.to_string(),
            dashboard_root: dashboard_root.to_string(),
            runbook_root: runbook_root.to_string(),
            imported_evidence_root: imported_evidence_root.to_string(),
            approval_root,
            approved_at_height,
            status,
        };
        approval.validate()?;
        Ok(approval)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("dashboard approval id", &self.approval_id)?;
        ensure_non_empty("dashboard approver id", &self.approver_id)?;
        ensure_non_empty("dashboard approval role", &self.role)?;
        ensure_root("dashboard_root", &self.dashboard_root)?;
        ensure_root("runbook_root", &self.runbook_root)?;
        ensure_root("imported_evidence_root", &self.imported_evidence_root)?;
        ensure_root("approval_root", &self.approval_root)?;
        ensure(
            self.approved_at_height > 0,
            "approval height must be non-zero",
        )
    }

    pub fn satisfies(&self, config: &Config, height: u64) -> bool {
        self.status.accepted()
            && self.dashboard_root == config.expected_dashboard_root
            && self.runbook_root == config.expected_runbook_root
            && self.imported_evidence_root == config.expected_imported_evidence_root
            && height.saturating_sub(self.approved_at_height) <= config.max_evidence_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "operator_dashboard_approval",
            "approval_id": self.approval_id,
            "approver_id": self.approver_id,
            "role": self.role,
            "dashboard_root": self.dashboard_root,
            "runbook_root": self.runbook_root,
            "imported_evidence_root": self.imported_evidence_root,
            "approval_root": self.approval_root,
            "approved_at_height": self.approved_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-DASHBOARD-APPROVAL",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseReviewer {
    pub reviewer_id: String,
    pub role: String,
    pub signed_policy_root: String,
    pub signed_dashboard_root: String,
    pub signed_evidence_root: String,
    pub review_root: String,
    pub reviewed_at_height: u64,
    pub status: ReceiptStatus,
}

impl ReleaseReviewer {
    pub fn new(
        reviewer_id: &str,
        role: &str,
        signed_policy_root: &str,
        signed_dashboard_root: &str,
        signed_evidence_root: &str,
        reviewed_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let review_root = runtime_id(
            "PQ-RESERVE-PRIVACY-RELEASE-REVIEW-ROOT",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(reviewer_id),
                HashPart::Str(role),
                HashPart::Str(signed_policy_root),
                HashPart::Str(signed_dashboard_root),
                HashPart::Str(signed_evidence_root),
            ],
        );
        let reviewer = Self {
            reviewer_id: reviewer_id.to_string(),
            role: role.to_string(),
            signed_policy_root: signed_policy_root.to_string(),
            signed_dashboard_root: signed_dashboard_root.to_string(),
            signed_evidence_root: signed_evidence_root.to_string(),
            review_root,
            reviewed_at_height,
            status,
        };
        reviewer.validate()?;
        Ok(reviewer)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("release reviewer id", &self.reviewer_id)?;
        ensure_non_empty("release reviewer role", &self.role)?;
        ensure_root("signed_policy_root", &self.signed_policy_root)?;
        ensure_root("signed_dashboard_root", &self.signed_dashboard_root)?;
        ensure_root("signed_evidence_root", &self.signed_evidence_root)?;
        ensure_root("review_root", &self.review_root)?;
        ensure(
            self.reviewed_at_height > 0,
            "release reviewer height must be non-zero",
        )
    }

    pub fn satisfies(&self, config: &Config, evidence_root: &str, height: u64) -> bool {
        self.status.accepted()
            && self.signed_policy_root == config.policy_root()
            && self.signed_dashboard_root == config.expected_dashboard_root
            && self.signed_evidence_root == evidence_root
            && height.saturating_sub(self.reviewed_at_height) <= config.max_evidence_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "release_reviewer",
            "reviewer_id": self.reviewer_id,
            "role": self.role,
            "signed_policy_root": self.signed_policy_root,
            "signed_dashboard_root": self.signed_dashboard_root,
            "signed_evidence_root": self.signed_evidence_root,
            "review_root": self.review_root,
            "reviewed_at_height": self.reviewed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("PQ-RESERVE-PRIVACY-RELEASE-REVIEWER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardCellBinding {
    pub cell_id: String,
    pub gate: ReleaseGate,
    pub dashboard_cell_root: String,
    pub evidence_root: String,
    pub policy_clause_root: String,
    pub bound_at_height: u64,
    pub status: ReceiptStatus,
}

impl DashboardCellBinding {
    pub fn new(
        gate: ReleaseGate,
        dashboard_cell_root: &str,
        evidence_root: &str,
        policy_clause_root: &str,
        bound_at_height: u64,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let cell_id = runtime_id(
            "PQ-RESERVE-PRIVACY-DASHBOARD-CELL-BINDING-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(gate.as_str()),
                HashPart::Str(dashboard_cell_root),
                HashPart::Str(evidence_root),
            ],
        );
        let binding = Self {
            cell_id,
            gate,
            dashboard_cell_root: dashboard_cell_root.to_string(),
            evidence_root: evidence_root.to_string(),
            policy_clause_root: policy_clause_root.to_string(),
            bound_at_height,
            status,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("dashboard cell binding id", &self.cell_id)?;
        ensure_root("dashboard_cell_root", &self.dashboard_cell_root)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("policy_clause_root", &self.policy_clause_root)?;
        ensure(
            self.bound_at_height > 0,
            "dashboard cell binding height must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "dashboard_cell_binding",
            "cell_id": self.cell_id,
            "gate": self.gate.as_str(),
            "policy_label": self.gate.policy_label(),
            "dashboard_cell_root": self.dashboard_cell_root,
            "evidence_root": self.evidence_root,
            "policy_clause_root": self.policy_clause_root,
            "bound_at_height": self.bound_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-DASHBOARD-CELL-BINDING",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorBlocker {
    pub blocker_id: String,
    pub gate: ReleaseGate,
    pub kind: BlockerKind,
    pub evidence_root: String,
    pub detail: String,
    pub opened_at_height: u64,
    pub resolved_at_height: Option<u64>,
    pub status: ReceiptStatus,
}

impl OperatorBlocker {
    pub fn new(
        gate: ReleaseGate,
        kind: BlockerKind,
        evidence_root: &str,
        detail: &str,
        opened_at_height: u64,
        resolved_at_height: Option<u64>,
        status: ReceiptStatus,
    ) -> Result<Self> {
        let blocker_id = runtime_id(
            "PQ-RESERVE-PRIVACY-OPERATOR-BLOCKER-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(gate.as_str()),
                HashPart::Str(kind.as_str()),
                HashPart::Str(evidence_root),
                HashPart::Int(opened_at_height as i128),
            ],
        );
        let blocker = Self {
            blocker_id,
            gate,
            kind,
            evidence_root: evidence_root.to_string(),
            detail: detail.to_string(),
            opened_at_height,
            resolved_at_height,
            status,
        };
        blocker.validate()?;
        Ok(blocker)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("operator blocker id", &self.blocker_id)?;
        ensure_root("operator blocker evidence root", &self.evidence_root)?;
        ensure_non_empty("operator blocker detail", &self.detail)?;
        ensure(
            self.opened_at_height > 0,
            "operator blocker opened height must be non-zero",
        )?;
        if let Some(resolved_at_height) = self.resolved_at_height {
            ensure(
                resolved_at_height >= self.opened_at_height,
                "operator blocker resolution must not precede opening",
            )?;
        }
        Ok(())
    }

    pub fn unresolved(&self) -> bool {
        self.status.blocking() || self.resolved_at_height.is_none()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "operator_blocker",
            "blocker_id": self.blocker_id,
            "gate": self.gate.as_str(),
            "blocker_kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "detail": self.detail,
            "opened_at_height": self.opened_at_height,
            "resolved_at_height": self.resolved_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("PQ-RESERVE-PRIVACY-OPERATOR-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleasePolicyBlocker {
    pub blocker_id: String,
    pub gate: ReleaseGate,
    pub kind: BlockerKind,
    pub evidence_root: String,
    pub detail: String,
    pub observed_at_height: u64,
}

impl ReleasePolicyBlocker {
    pub fn new(
        gate: ReleaseGate,
        kind: BlockerKind,
        evidence_root: &str,
        detail: &str,
        observed_at_height: u64,
    ) -> Self {
        Self {
            blocker_id: runtime_id(
                "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BLOCKER-ID",
                &[
                    HashPart::Str(CHAIN_ID),
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(gate.as_str()),
                    HashPart::Str(kind.as_str()),
                    HashPart::Str(evidence_root),
                    HashPart::Int(observed_at_height as i128),
                ],
            ),
            gate,
            kind,
            evidence_root: evidence_root.to_string(),
            detail: detail.to_string(),
            observed_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "release_policy_blocker",
            "blocker_id": self.blocker_id,
            "gate": self.gate.as_str(),
            "blocker_kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "detail": self.detail,
            "observed_at_height": self.observed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BLOCKER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleasePolicyVerdict {
    pub verdict_id: String,
    pub release_policy_id: String,
    pub dashboard_root: String,
    pub evidence_root: String,
    pub reviewer_root: String,
    pub blocker_root: String,
    pub go: bool,
    pub verdict: String,
    pub gate_count: u64,
    pub passing_gate_count: u64,
    pub blocker_count: u64,
    pub decided_at_height: u64,
}

impl ReleasePolicyVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "kind": "release_policy_verdict",
            "verdict_id": self.verdict_id,
            "release_policy_id": self.release_policy_id,
            "dashboard_root": self.dashboard_root,
            "evidence_root": self.evidence_root,
            "reviewer_root": self.reviewer_root,
            "blocker_root": self.blocker_root,
            "go": self.go,
            "verdict": self.verdict,
            "gate_count": self.gate_count,
            "passing_gate_count": self.passing_gate_count,
            "blocker_count": self.blocker_count,
            "decided_at_height": self.decided_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-VERDICT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub source_wave: u16,
    pub binding_wave: u16,
    pub pq_rotation_receipts: BTreeMap<String, PqRotationReceipt>,
    pub reserve_coverage_proofs: BTreeMap<String, ReserveCoverageProof>,
    pub privacy_boundary_receipts: BTreeMap<String, PrivacyBoundaryReceipt>,
    pub dashboard_approvals: BTreeMap<String, DashboardApproval>,
    pub release_reviewers: BTreeMap<String, ReleaseReviewer>,
    pub dashboard_cell_bindings: BTreeMap<String, DashboardCellBinding>,
    pub operator_blockers: BTreeMap<String, OperatorBlocker>,
}

impl State {
    pub fn new(config: Config, height: u64) -> Result<Self> {
        config.validate()?;
        ensure(height > 0, "height must be non-zero")?;
        Ok(Self {
            config,
            height,
            source_wave: DEFAULT_SOURCE_WAVE_NUMBER,
            binding_wave: DEFAULT_WAVE_NUMBER,
            pq_rotation_receipts: BTreeMap::new(),
            reserve_coverage_proofs: BTreeMap::new(),
            privacy_boundary_receipts: BTreeMap::new(),
            dashboard_approvals: BTreeMap::new(),
            release_reviewers: BTreeMap::new(),
            dashboard_cell_bindings: BTreeMap::new(),
            operator_blockers: BTreeMap::new(),
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut state = match Self::new(config, DEFAULT_HEIGHT) {
            Ok(state) => state,
            Err(_) => Self {
                config: Config::devnet(),
                height: DEFAULT_HEIGHT,
                source_wave: DEFAULT_SOURCE_WAVE_NUMBER,
                binding_wave: DEFAULT_WAVE_NUMBER,
                pq_rotation_receipts: BTreeMap::new(),
                reserve_coverage_proofs: BTreeMap::new(),
                privacy_boundary_receipts: BTreeMap::new(),
                dashboard_approvals: BTreeMap::new(),
                release_reviewers: BTreeMap::new(),
                dashboard_cell_bindings: BTreeMap::new(),
                operator_blockers: BTreeMap::new(),
            },
        };
        state.seed_devnet();
        state
    }

    pub fn add_pq_rotation_receipt(&mut self, receipt: PqRotationReceipt) -> Result<()> {
        receipt.validate()?;
        self.pq_rotation_receipts
            .insert(receipt.receipt_id.clone(), receipt);
        Ok(())
    }

    pub fn add_reserve_coverage_proof(&mut self, proof: ReserveCoverageProof) -> Result<()> {
        proof.validate()?;
        self.reserve_coverage_proofs
            .insert(proof.proof_id.clone(), proof);
        Ok(())
    }

    pub fn add_privacy_boundary_receipt(&mut self, receipt: PrivacyBoundaryReceipt) -> Result<()> {
        receipt.validate()?;
        self.privacy_boundary_receipts
            .insert(receipt.receipt_id.clone(), receipt);
        Ok(())
    }

    pub fn add_dashboard_approval(&mut self, approval: DashboardApproval) -> Result<()> {
        approval.validate()?;
        self.dashboard_approvals
            .insert(approval.approval_id.clone(), approval);
        Ok(())
    }

    pub fn add_release_reviewer(&mut self, reviewer: ReleaseReviewer) -> Result<()> {
        reviewer.validate()?;
        self.release_reviewers
            .insert(reviewer.reviewer_id.clone(), reviewer);
        Ok(())
    }

    pub fn add_dashboard_cell_binding(&mut self, binding: DashboardCellBinding) -> Result<()> {
        binding.validate()?;
        self.dashboard_cell_bindings
            .insert(binding.cell_id.clone(), binding);
        Ok(())
    }

    pub fn add_operator_blocker(&mut self, blocker: OperatorBlocker) -> Result<()> {
        blocker.validate()?;
        self.operator_blockers
            .insert(blocker.blocker_id.clone(), blocker);
        Ok(())
    }

    pub fn release_policy_verdict(&self) -> ReleasePolicyVerdict {
        let blockers = self.release_policy_blockers();
        let evidence_root = self.evidence_root();
        let reviewer_root = self.reviewer_root();
        let blocker_root = map_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BLOCKER-ROOT",
            blockers.iter().map(ReleasePolicyBlocker::state_root),
        );
        let passing_gate_count = self.passing_gate_count();
        let go = blockers.is_empty() && passing_gate_count == REQUIRED_GATES.len() as u64;
        let verdict = if go {
            VERDICT_GO
        } else if self.config.fail_closed {
            VERDICT_FAIL_CLOSED
        } else {
            VERDICT_NO_GO
        }
        .to_string();
        let verdict_id = runtime_id(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-VERDICT-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.release_policy_id),
                HashPart::Str(&self.config.expected_dashboard_root),
                HashPart::Str(&evidence_root),
                HashPart::Str(&blocker_root),
                HashPart::Int(self.height as i128),
            ],
        );
        ReleasePolicyVerdict {
            verdict_id,
            release_policy_id: self.config.release_policy_id.clone(),
            dashboard_root: self.config.expected_dashboard_root.clone(),
            evidence_root,
            reviewer_root,
            blocker_root,
            go,
            verdict,
            gate_count: REQUIRED_GATES.len() as u64,
            passing_gate_count,
            blocker_count: blockers.len() as u64,
            decided_at_height: self.height,
        }
    }

    pub fn assert_release_policy_go(&self) -> Result<String> {
        let verdict = self.release_policy_verdict();
        if verdict.go {
            Ok(verdict.verdict_id)
        } else {
            Err(format!(
                "release policy binding is {} with {} blocker(s)",
                verdict.verdict, verdict.blocker_count
            ))
        }
    }

    pub fn release_policy_blockers(&self) -> Vec<ReleasePolicyBlocker> {
        let mut blockers = Vec::new();
        self.push_pq_rotation_blockers(&mut blockers);
        self.push_reserve_blockers(&mut blockers);
        self.push_privacy_blockers(&mut blockers);
        self.push_dashboard_blockers(&mut blockers);
        self.push_reviewer_blockers(&mut blockers);
        self.push_operator_blockers(&mut blockers);
        blockers
    }

    pub fn public_record(&self) -> Value {
        let mut record = self.public_record_without_root();
        if let Value::Object(object) = &mut record {
            object.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn public_record_without_root(&self) -> Value {
        let blockers = self.release_policy_blockers();
        let verdict = self.release_policy_verdict();
        json!({
            "kind": "pq_reserve_privacy_operator_dashboard_release_policy_binding_state",
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "source_wave": self.source_wave,
            "binding_wave": self.binding_wave,
            "config": self.config.public_record(),
            "verdict": verdict.public_record(),
            "pq_rotation_receipts": self.pq_rotation_receipts.values().map(PqRotationReceipt::public_record).collect::<Vec<_>>(),
            "reserve_coverage_proofs": self.reserve_coverage_proofs.values().map(ReserveCoverageProof::public_record).collect::<Vec<_>>(),
            "privacy_boundary_receipts": self.privacy_boundary_receipts.values().map(PrivacyBoundaryReceipt::public_record).collect::<Vec<_>>(),
            "dashboard_approvals": self.dashboard_approvals.values().map(DashboardApproval::public_record).collect::<Vec<_>>(),
            "release_reviewers": self.release_reviewers.values().map(ReleaseReviewer::public_record).collect::<Vec<_>>(),
            "dashboard_cell_bindings": self.dashboard_cell_bindings.values().map(DashboardCellBinding::public_record).collect::<Vec<_>>(),
            "operator_blockers": self.operator_blockers.values().map(OperatorBlocker::public_record).collect::<Vec<_>>(),
            "release_policy_blockers": blockers.iter().map(ReleasePolicyBlocker::public_record).collect::<Vec<_>>(),
            "roots": {
                "policy_root": self.config.policy_root(),
                "pq_rotation_root": self.pq_rotation_root(),
                "reserve_root": self.reserve_root(),
                "privacy_root": self.privacy_root(),
                "dashboard_approval_root": self.dashboard_approval_root(),
                "reviewer_root": self.reviewer_root(),
                "dashboard_cell_binding_root": self.dashboard_cell_binding_root(),
                "operator_blocker_root": self.operator_blocker_root(),
                "evidence_root": self.evidence_root(),
                "release_policy_blocker_root": verdict.blocker_root,
                "verdict_root": verdict.state_root(),
            },
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BINDING-STATE",
            &self.public_record_without_root(),
        )
    }

    pub fn pq_rotation_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-PQ-ROTATION-ROOT",
            self.pq_rotation_receipts
                .values()
                .map(PqRotationReceipt::state_root),
        )
    }

    pub fn reserve_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-RESERVE-COVERAGE-ROOT",
            self.reserve_coverage_proofs
                .values()
                .map(ReserveCoverageProof::state_root),
        )
    }

    pub fn privacy_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-BOUNDARY-ROOT",
            self.privacy_boundary_receipts
                .values()
                .map(PrivacyBoundaryReceipt::state_root),
        )
    }

    pub fn dashboard_approval_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-DASHBOARD-APPROVAL-ROOT",
            self.dashboard_approvals
                .values()
                .map(DashboardApproval::state_root),
        )
    }

    pub fn reviewer_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-REVIEWER-ROOT",
            self.release_reviewers
                .values()
                .map(ReleaseReviewer::state_root),
        )
    }

    pub fn dashboard_cell_binding_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-DASHBOARD-CELL-BINDING-ROOT",
            self.dashboard_cell_bindings
                .values()
                .map(DashboardCellBinding::state_root),
        )
    }

    pub fn operator_blocker_root(&self) -> String {
        map_root(
            "PQ-RESERVE-PRIVACY-OPERATOR-BLOCKER-ROOT",
            self.operator_blockers
                .values()
                .map(OperatorBlocker::state_root),
        )
    }

    pub fn evidence_root(&self) -> String {
        merkle_root(
            "PQ-RESERVE-PRIVACY-RELEASE-POLICY-EVIDENCE-ROOT",
            &vec![
                self.pq_rotation_root(),
                self.reserve_root(),
                self.privacy_root(),
                self.dashboard_approval_root(),
                self.dashboard_cell_binding_root(),
                self.operator_blocker_root(),
            ],
        )
    }

    fn passing_gate_count(&self) -> u64 {
        let mut count = 0_u64;
        if self
            .pq_rotation_receipts
            .values()
            .any(|receipt| receipt.satisfies(&self.config, self.height))
        {
            count = count.saturating_add(1);
        }
        if self.reserve_gate_passes() {
            count = count.saturating_add(1);
        }
        if self
            .privacy_boundary_receipts
            .values()
            .any(|receipt| receipt.satisfies(&self.config, self.height))
        {
            count = count.saturating_add(1);
        }
        if self.dashboard_gate_passes() {
            count = count.saturating_add(1);
        }
        if self.reviewer_gate_passes() {
            count = count.saturating_add(1);
        }
        if self
            .operator_blockers
            .values()
            .all(|blocker| !blocker.unresolved())
        {
            count = count.saturating_add(1);
        }
        count
    }

    fn reserve_gate_passes(&self) -> bool {
        let accepted = self
            .reserve_coverage_proofs
            .values()
            .filter(|proof| proof.satisfies(&self.config, self.height))
            .collect::<Vec<_>>();
        accepted.len() >= usize::from(self.config.min_reserve_proof_count)
            && accepted
                .iter()
                .all(|proof| proof.coverage_bps >= self.config.min_reserve_coverage_bps)
    }

    fn dashboard_gate_passes(&self) -> bool {
        let approvals = self
            .dashboard_approvals
            .values()
            .filter(|approval| approval.satisfies(&self.config, self.height))
            .count();
        let cells = self
            .dashboard_cell_bindings
            .values()
            .filter(|binding| binding.status.accepted())
            .filter(|binding| REQUIRED_GATES.iter().any(|gate| *gate == binding.gate))
            .count();
        approvals >= usize::from(self.config.min_approval_count)
            && cells >= usize::from(self.config.min_bound_dashboard_cells)
    }

    fn reviewer_gate_passes(&self) -> bool {
        let evidence_root = self.evidence_root();
        self.release_reviewers
            .values()
            .filter(|reviewer| reviewer.satisfies(&self.config, &evidence_root, self.height))
            .count()
            >= usize::from(self.config.min_reviewer_quorum)
    }

    fn push_pq_rotation_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        if self.pq_rotation_receipts.is_empty() {
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::PqRotation,
                BlockerKind::MissingPqRotationReceipt,
                &self.config.policy_root(),
                "no pq rotation receipt is bound to the release policy",
                self.height,
            ));
            return;
        }
        if !self
            .pq_rotation_receipts
            .values()
            .any(|receipt| receipt.satisfies(&self.config, self.height))
        {
            let root = self.pq_rotation_root();
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::PqRotation,
                BlockerKind::PqRotationQuorumShortfall,
                &root,
                "pq rotation receipts do not satisfy epoch, quorum, weight, security, and freshness policy",
                self.height,
            ));
        }
        for receipt in self.pq_rotation_receipts.values() {
            if receipt.key_epoch != self.config.expected_key_epoch {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PqRotation,
                    BlockerKind::PqRotationEpochMismatch,
                    &receipt.state_root(),
                    "pq rotation receipt key epoch does not match release policy",
                    self.height,
                ));
            }
            if receipt.security_bits < self.config.min_pq_security_bits {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PqRotation,
                    BlockerKind::PqSecurityBitsShortfall,
                    &receipt.state_root(),
                    "pq rotation receipt security bits are below policy",
                    self.height,
                ));
            }
            if receipt.is_stale(&self.config, self.height) {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PqRotation,
                    BlockerKind::StaleEvidence,
                    &receipt.state_root(),
                    "pq rotation receipt is outside the accepted-live-evidence freshness window",
                    self.height,
                ));
            }
        }
    }

    fn push_reserve_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        let accepted_count = self
            .reserve_coverage_proofs
            .values()
            .filter(|proof| proof.satisfies(&self.config, self.height))
            .count();
        if accepted_count < usize::from(self.config.min_reserve_proof_count) {
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::ReserveCoverage,
                BlockerKind::ReserveProofQuorumShortfall,
                &self.reserve_root(),
                "reserve coverage proof quorum is below release policy",
                self.height,
            ));
        }
        for proof in self.reserve_coverage_proofs.values() {
            if proof.coverage_bps < self.config.min_reserve_coverage_bps {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::ReserveCoverage,
                    BlockerKind::ReserveCoverageShortfall,
                    &proof.state_root(),
                    "reserve coverage proof is below required basis points",
                    self.height,
                ));
            }
            if proof.is_stale(&self.config, self.height) {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::ReserveCoverage,
                    BlockerKind::StaleEvidence,
                    &proof.state_root(),
                    "reserve coverage proof is stale",
                    self.height,
                ));
            }
        }
    }

    fn push_privacy_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        if !self
            .privacy_boundary_receipts
            .values()
            .any(|receipt| receipt.satisfies(&self.config, self.height))
        {
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::PrivacyBoundary,
                BlockerKind::PrivacyBoundaryRejected,
                &self.privacy_root(),
                "no privacy boundary receipt satisfies privacy set and budget policy",
                self.height,
            ));
        }
        for receipt in self.privacy_boundary_receipts.values() {
            if receipt.privacy_set_size < self.config.min_privacy_set_size {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PrivacyBoundary,
                    BlockerKind::PrivacySetTooSmall,
                    &receipt.state_root(),
                    "privacy boundary receipt set size is below policy",
                    self.height,
                ));
            }
            if receipt.consumed_budget_bps > self.config.max_privacy_budget_bps {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PrivacyBoundary,
                    BlockerKind::PrivacyBudgetExceeded,
                    &receipt.state_root(),
                    "privacy boundary receipt consumed budget exceeds policy",
                    self.height,
                ));
            }
            if receipt.is_stale(&self.config, self.height) {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::PrivacyBoundary,
                    BlockerKind::StaleEvidence,
                    &receipt.state_root(),
                    "privacy boundary receipt is stale",
                    self.height,
                ));
            }
        }
    }

    fn push_dashboard_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        if !self.dashboard_gate_passes() {
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::OperatorDashboard,
                BlockerKind::DashboardApprovalShortfall,
                &self.dashboard_approval_root(),
                "operator dashboard approvals or bound cells are below release policy",
                self.height,
            ));
        }
        for approval in self.dashboard_approvals.values() {
            if approval.dashboard_root != self.config.expected_dashboard_root
                || approval.runbook_root != self.config.expected_runbook_root
                || approval.imported_evidence_root != self.config.expected_imported_evidence_root
            {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::OperatorDashboard,
                    BlockerKind::DashboardRootMismatch,
                    &approval.state_root(),
                    "operator dashboard approval is not bound to expected dashboard/runbook/import roots",
                    self.height,
                ));
            }
        }
    }

    fn push_reviewer_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        if !self.reviewer_gate_passes() {
            blockers.push(ReleasePolicyBlocker::new(
                ReleaseGate::ReviewerQuorum,
                BlockerKind::ReviewerQuorumShortfall,
                &self.reviewer_root(),
                "release reviewer quorum does not sign policy, dashboard, and evidence roots",
                self.height,
            ));
        }
        let evidence_root = self.evidence_root();
        for reviewer in self.release_reviewers.values() {
            if reviewer.signed_policy_root != self.config.policy_root()
                || reviewer.signed_dashboard_root != self.config.expected_dashboard_root
                || reviewer.signed_evidence_root != evidence_root
            {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::ReviewerQuorum,
                    BlockerKind::ReviewerRootMismatch,
                    &reviewer.state_root(),
                    "release reviewer signature roots do not match current binding",
                    self.height,
                ));
            }
        }
    }

    fn push_operator_blockers(&self, blockers: &mut Vec<ReleasePolicyBlocker>) {
        for blocker in self.operator_blockers.values() {
            if blocker.unresolved() {
                blockers.push(ReleasePolicyBlocker::new(
                    ReleaseGate::BlockerHandling,
                    BlockerKind::UnresolvedBlocker,
                    &blocker.state_root(),
                    "operator dashboard blocker remains unresolved",
                    self.height,
                ));
            }
        }
    }

    fn seed_devnet(&mut self) {
        let signer_ids = vec![
            "pq-signer-0".to_string(),
            "pq-signer-1".to_string(),
            "pq-signer-2".to_string(),
            "pq-signer-3".to_string(),
            "pq-signer-4".to_string(),
        ];
        if let Ok(receipt) = PqRotationReceipt::new(
            self.config.expected_key_epoch,
            &sample_root("wave-82-pq-rotation-transcript"),
            &sample_root("wave-82-active-pq-keyset"),
            &sample_root("wave-82-retired-pq-keyset"),
            signer_ids,
            7_200,
            6_700,
            192,
            self.height.saturating_sub(12),
            ReceiptStatus::Accepted,
        ) {
            let _ = self.add_pq_rotation_receipt(receipt);
        }
        for asset in ["xmr", "btc-shadow", "usd-reserve"] {
            if let Ok(proof) = ReserveCoverageProof::new(
                asset,
                &sample_root(&format!("{asset}-liability-root")),
                &sample_root(&format!("{asset}-reserve-root")),
                &sample_root(&format!("{asset}-auditor-root")),
                1_000_000_000_000,
                1_080_000_000_000,
                vec![
                    format!("{asset}-reserve-signer-0"),
                    format!("{asset}-reserve-signer-1"),
                    format!("{asset}-reserve-signer-2"),
                ],
                self.height.saturating_sub(10),
                ReceiptStatus::Accepted,
            ) {
                let _ = self.add_reserve_coverage_proof(proof);
            }
        }
        if let Ok(receipt) = PrivacyBoundaryReceipt::new(
            "force-exit-pq-reserve-privacy-boundary",
            &sample_root("wave-82-privacy-policy-root"),
            &sample_root("wave-82-non-linkage-root"),
            &sample_root("wave-82-disclosure-control-root"),
            512,
            1_700,
            vec![
                "privacy-reviewer-0".to_string(),
                "privacy-reviewer-1".to_string(),
                "privacy-reviewer-2".to_string(),
            ],
            self.height.saturating_sub(9),
            ReceiptStatus::Accepted,
        ) {
            let _ = self.add_privacy_boundary_receipt(receipt);
        }
        for index in 0..self.config.min_approval_count {
            if let Ok(approval) = DashboardApproval::new(
                &format!("dashboard-approver-{index}"),
                "operator_release_approver",
                &self.config.expected_dashboard_root,
                &self.config.expected_runbook_root,
                &self.config.expected_imported_evidence_root,
                self.height
                    .saturating_sub(6)
                    .saturating_add(u64::from(index)),
                ReceiptStatus::Accepted,
            ) {
                let _ = self.add_dashboard_approval(approval);
            }
        }
        for gate in REQUIRED_GATES {
            if let Ok(binding) = DashboardCellBinding::new(
                *gate,
                &sample_root(&format!("dashboard-cell-{}", gate.as_str())),
                &self.gate_evidence_root(*gate),
                &sample_root(&format!("policy-clause-{}", gate.policy_label())),
                self.height.saturating_sub(5),
                ReceiptStatus::Accepted,
            ) {
                let _ = self.add_dashboard_cell_binding(binding);
            }
        }
        for index in 0..self.config.min_reviewer_quorum {
            let evidence_root = self.evidence_root();
            if let Ok(reviewer) = ReleaseReviewer::new(
                &format!("release-reviewer-{index}"),
                "release_policy_quorum",
                &self.config.policy_root(),
                &self.config.expected_dashboard_root,
                &evidence_root,
                self.height
                    .saturating_sub(3)
                    .saturating_add(u64::from(index)),
                ReceiptStatus::Accepted,
            ) {
                let _ = self.add_release_reviewer(reviewer);
            }
        }
        if let Ok(blocker) = OperatorBlocker::new(
            ReleaseGate::BlockerHandling,
            BlockerKind::OpenDashboardAction,
            &sample_root("wave-82-closed-dashboard-action-root"),
            "wave 82 dashboard action closed with accepted live evidence",
            self.height.saturating_sub(18),
            Some(self.height.saturating_sub(7)),
            ReceiptStatus::Accepted,
        ) {
            let _ = self.add_operator_blocker(blocker);
        }
    }

    fn gate_evidence_root(&self, gate: ReleaseGate) -> String {
        match gate {
            ReleaseGate::PqRotation => self.pq_rotation_root(),
            ReleaseGate::ReserveCoverage => self.reserve_root(),
            ReleaseGate::PrivacyBoundary => self.privacy_root(),
            ReleaseGate::OperatorDashboard => self.dashboard_approval_root(),
            ReleaseGate::ReviewerQuorum => self.reviewer_root(),
            ReleaseGate::BlockerHandling => self.operator_blocker_root(),
        }
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> serde_json::Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn release_policy_verdict() -> ReleasePolicyVerdict {
    devnet().release_policy_verdict()
}

pub fn release_policy_blockers() -> Vec<Value> {
    devnet()
        .release_policy_blockers()
        .iter()
        .map(ReleasePolicyBlocker::public_record)
        .collect()
}

fn runtime_id(domain: &str, parts: &[HashPart<'_>]) -> String {
    domain_hash(domain, parts, 32)
}

fn sample_root(label: &str) -> String {
    runtime_id(
        "PQ-RESERVE-PRIVACY-RELEASE-POLICY-BINDING-DEVNET-SAMPLE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
    )
}

fn record_root(domain: &str, record: &Value) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
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

fn coverage_bps(covered_atomic_units: u128, required_atomic_units: u128) -> u16 {
    if required_atomic_units == 0 {
        return 0;
    }
    let bps = covered_atomic_units.saturating_mul(10_000) / required_atomic_units;
    if bps > u16::MAX as u128 {
        u16::MAX
    } else {
        bps as u16
    }
}

fn clamped_i128(value: u128) -> i128 {
    if value > i128::MAX as u128 {
        i128::MAX
    } else {
        value as i128
    }
}

fn sorted_unique(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
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

fn ensure_bps(value: u16, label: &str) -> Result<()> {
    ensure(value <= 10_000, &format!("{label} must be <= 10000"))
}

fn ensure_unique_non_empty(label: &str, values: &[String]) -> Result<()> {
    ensure(!values.is_empty(), &format!("{label} must not be empty"))?;
    let mut seen = BTreeSet::new();
    for value in values {
        ensure_non_empty(label, value)?;
        ensure(
            seen.insert(value),
            &format!("{label} contains duplicate value"),
        )?;
    }
    Ok(())
}
