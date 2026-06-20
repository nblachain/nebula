use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageBridgeCustodyAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-bridge-custody-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const BINDING_SUITE: &str = "monero-l2-pq-bridge-custody-dashboard-release-policy-binding-v1";
pub const DEFAULT_BINDING_HEIGHT: u64 = 1_444_940;
pub const DEFAULT_ACCEPTANCE_EPOCH: u64 = 83;
pub const DEFAULT_PREVIOUS_WAVE: u64 = 82;
pub const DEFAULT_REQUIRED_SIGNER_COUNT: u64 = 4;
pub const DEFAULT_REQUIRED_SIGNER_WEIGHT: u64 = 67;
pub const DEFAULT_REQUIRED_REVIEWER_COUNT: u64 = 3;
pub const DEFAULT_REQUIRED_REVIEWER_WEIGHT: u64 = 75;
pub const DEFAULT_REQUIRED_DASHBOARD_APPROVALS: u64 = 3;
pub const DEFAULT_MONERO_RELEASE_HEIGHT: u64 = 2_912_880;
pub const DEFAULT_MONERO_CONFIRMATIONS: u64 = 24;
pub const DEFAULT_RESERVE_HANDOFF_CONFIRMATIONS: u64 = 12;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Pending,
    Rejected,
    Blocked,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Rejected => "rejected",
            Self::Blocked => "blocked",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Go,
    NoGo,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Go)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingCustodySignerReceipt,
    InsufficientCustodySignerWeight,
    MissingMoneroReleaseObservation,
    InsufficientMoneroConfirmations,
    MissingReserveHandoffRoot,
    InsufficientReserveConfirmations,
    MissingDashboardApproval,
    MissingReviewerQuorum,
    OpenOperatorAction,
    RejectedEvidence,
    PolicyRootMismatch,
    DashboardRootMismatch,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingCustodySignerReceipt => "missing_custody_signer_receipt",
            Self::InsufficientCustodySignerWeight => "insufficient_custody_signer_weight",
            Self::MissingMoneroReleaseObservation => "missing_monero_release_observation",
            Self::InsufficientMoneroConfirmations => "insufficient_monero_confirmations",
            Self::MissingReserveHandoffRoot => "missing_reserve_handoff_root",
            Self::InsufficientReserveConfirmations => "insufficient_reserve_confirmations",
            Self::MissingDashboardApproval => "missing_dashboard_approval",
            Self::MissingReviewerQuorum => "missing_reviewer_quorum",
            Self::OpenOperatorAction => "open_operator_action",
            Self::RejectedEvidence => "rejected_evidence",
            Self::PolicyRootMismatch => "policy_root_mismatch",
            Self::DashboardRootMismatch => "dashboard_root_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardApprovalRole {
    CustodyOperator,
    ReleaseCoordinator,
    IncidentCommander,
    ReserveOperator,
}

impl DashboardApprovalRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustodyOperator => "custody_operator",
            Self::ReleaseCoordinator => "release_coordinator",
            Self::IncidentCommander => "incident_commander",
            Self::ReserveOperator => "reserve_operator",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub binding_suite: String,
    pub binding_id: String,
    pub source_runbook_audit_id: String,
    pub source_dashboard_finalization_id: String,
    pub bridge_custody_lane_id: String,
    pub release_policy_id: String,
    pub binding_height: u64,
    pub acceptance_epoch: u64,
    pub previous_wave: u64,
    pub required_signer_count: u64,
    pub required_signer_weight: u64,
    pub required_reviewer_count: u64,
    pub required_reviewer_weight: u64,
    pub required_dashboard_approvals: u64,
    pub required_monero_confirmations: u64,
    pub required_reserve_handoff_confirmations: u64,
    pub require_dual_pq_signatures: bool,
    pub require_monero_release_observation: bool,
    pub require_reserve_handoff: bool,
    pub require_dashboard_approval_quorum: bool,
    pub require_reviewer_quorum: bool,
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
            binding_id: binding_id("bridge-custody-release-policy-binding-devnet-0001"),
            source_runbook_audit_id:
                "bridge-custody-accepted-live-evidence-operator-runbook-devnet-0001".to_string(),
            source_dashboard_finalization_id:
                "release-dashboard-force-exit-package-custody-readiness-devnet-0001".to_string(),
            bridge_custody_lane_id: "bridge_custody".to_string(),
            release_policy_id: binding_id("force-exit-package-bridge-custody-release-policy"),
            binding_height: DEFAULT_BINDING_HEIGHT,
            acceptance_epoch: DEFAULT_ACCEPTANCE_EPOCH,
            previous_wave: DEFAULT_PREVIOUS_WAVE,
            required_signer_count: DEFAULT_REQUIRED_SIGNER_COUNT,
            required_signer_weight: DEFAULT_REQUIRED_SIGNER_WEIGHT,
            required_reviewer_count: DEFAULT_REQUIRED_REVIEWER_COUNT,
            required_reviewer_weight: DEFAULT_REQUIRED_REVIEWER_WEIGHT,
            required_dashboard_approvals: DEFAULT_REQUIRED_DASHBOARD_APPROVALS,
            required_monero_confirmations: DEFAULT_MONERO_CONFIRMATIONS,
            required_reserve_handoff_confirmations: DEFAULT_RESERVE_HANDOFF_CONFIRMATIONS,
            require_dual_pq_signatures: true,
            require_monero_release_observation: true,
            require_reserve_handoff: true,
            require_dashboard_approval_quorum: true,
            require_reviewer_quorum: true,
            fail_closed: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "binding_suite": self.binding_suite,
            "binding_id": self.binding_id,
            "source_runbook_audit_id": self.source_runbook_audit_id,
            "source_dashboard_finalization_id": self.source_dashboard_finalization_id,
            "bridge_custody_lane_id": self.bridge_custody_lane_id,
            "release_policy_id": self.release_policy_id,
            "binding_height": self.binding_height,
            "acceptance_epoch": self.acceptance_epoch,
            "previous_wave": self.previous_wave,
            "required_signer_count": self.required_signer_count,
            "required_signer_weight": self.required_signer_weight,
            "required_reviewer_count": self.required_reviewer_count,
            "required_reviewer_weight": self.required_reviewer_weight,
            "required_dashboard_approvals": self.required_dashboard_approvals,
            "required_monero_confirmations": self.required_monero_confirmations,
            "required_reserve_handoff_confirmations": self.required_reserve_handoff_confirmations,
            "require_dual_pq_signatures": self.require_dual_pq_signatures,
            "require_monero_release_observation": self.require_monero_release_observation,
            "require_reserve_handoff": self.require_reserve_handoff,
            "require_dashboard_approval_quorum": self.require_dashboard_approval_quorum,
            "require_reviewer_quorum": self.require_reviewer_quorum,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-RELEASE-POLICY-BINDING-CONFIG",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodySignerReceipt {
    pub receipt_id: String,
    pub signer_id: String,
    pub signer_weight: u64,
    pub receipt_height: u64,
    pub ml_dsa_signature_root: String,
    pub slh_dsa_signature_root: String,
    pub custody_commitment_root: String,
    pub accepted_live_evidence_root: String,
    pub runbook_ack_root: String,
    pub status: EvidenceStatus,
}

impl CustodySignerReceipt {
    pub fn devnet(config: &Config, ordinal: u64, signer_id: &str, signer_weight: u64) -> Self {
        let receipt_id = evidence_id(config, "custody-signer-receipt", signer_id, ordinal);
        let ml_dsa_signature_root =
            pq_signature_root(config, signer_id, ordinal, "ml-dsa-87", signer_weight);
        let slh_dsa_signature_root = pq_signature_root(
            config,
            signer_id,
            ordinal,
            "slh-dsa-shake-256f",
            signer_weight,
        );
        let custody_commitment_root = custody_commitment_root(
            config,
            signer_id,
            &ml_dsa_signature_root,
            &slh_dsa_signature_root,
        );
        let accepted_live_evidence_root = accepted_live_evidence_root(
            config,
            "custody_signer_receipt",
            &receipt_id,
            &custody_commitment_root,
        );
        let runbook_ack_root =
            ack_root(config, "runbook", &receipt_id, &accepted_live_evidence_root);
        Self {
            receipt_id,
            signer_id: signer_id.to_string(),
            signer_weight,
            receipt_height: config
                .binding_height
                .saturating_sub(80)
                .saturating_add(ordinal),
            ml_dsa_signature_root,
            slh_dsa_signature_root,
            custody_commitment_root,
            accepted_live_evidence_root,
            runbook_ack_root,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "signer_id": self.signer_id,
            "signer_weight": self.signer_weight,
            "receipt_height": self.receipt_height,
            "ml_dsa_signature_root": self.ml_dsa_signature_root,
            "slh_dsa_signature_root": self.slh_dsa_signature_root,
            "custody_commitment_root": self.custody_commitment_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "runbook_ack_root": self.runbook_ack_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("BRIDGE-CUSTODY-SIGNER-RECEIPT", &self.public_record())
    }

    pub fn is_accepted(&self, config: &Config) -> bool {
        let dual_pq_ready = !config.require_dual_pq_signatures
            || (!self.ml_dsa_signature_root.is_empty() && !self.slh_dsa_signature_root.is_empty());
        self.status.accepted()
            && dual_pq_ready
            && !self.custody_commitment_root.is_empty()
            && !self.accepted_live_evidence_root.is_empty()
            && !self.runbook_ack_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MoneroReleaseObservation {
    pub observation_id: String,
    pub release_txid: String,
    pub observed_height: u64,
    pub confirmations: u64,
    pub view_key_scan_root: String,
    pub output_membership_root: String,
    pub amount_commitment_root: String,
    pub destination_binding_root: String,
    pub observation_witness_root: String,
    pub dashboard_cell_root: String,
    pub status: EvidenceStatus,
}

impl MoneroReleaseObservation {
    pub fn devnet(config: &Config) -> Self {
        let observation_id = evidence_id(config, "monero-release-observation", "primary", 1);
        let release_txid = domain_hash(
            "BRIDGE-CUSTODY-BINDING-MONERO-RELEASE-TXID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.binding_id),
                HashPart::U64(DEFAULT_MONERO_RELEASE_HEIGHT),
            ],
            32,
        );
        let view_key_scan_root = release_component_root(config, &release_txid, "view-key-scan");
        let output_membership_root =
            release_component_root(config, &release_txid, "output-membership");
        let amount_commitment_root =
            release_component_root(config, &release_txid, "amount-commitment");
        let destination_binding_root =
            release_component_root(config, &release_txid, "destination-binding");
        let observation_witness_root = merkle_root(
            "BRIDGE-CUSTODY-BINDING-MONERO-OBSERVATION-WITNESS",
            &[
                json!({"kind": "view_key_scan", "root": view_key_scan_root}),
                json!({"kind": "output_membership", "root": output_membership_root}),
                json!({"kind": "amount_commitment", "root": amount_commitment_root}),
                json!({"kind": "destination_binding", "root": destination_binding_root}),
                json!({"kind": "release_txid", "root": release_txid}),
            ],
        );
        let dashboard_cell_root = dashboard_cell_root(
            config,
            "monero_release_observation",
            &observation_witness_root,
        );
        Self {
            observation_id,
            release_txid,
            observed_height: DEFAULT_MONERO_RELEASE_HEIGHT,
            confirmations: config.required_monero_confirmations,
            view_key_scan_root,
            output_membership_root,
            amount_commitment_root,
            destination_binding_root,
            observation_witness_root,
            dashboard_cell_root,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_id": self.observation_id,
            "release_txid": self.release_txid,
            "observed_height": self.observed_height,
            "confirmations": self.confirmations,
            "view_key_scan_root": self.view_key_scan_root,
            "output_membership_root": self.output_membership_root,
            "amount_commitment_root": self.amount_commitment_root,
            "destination_binding_root": self.destination_binding_root,
            "observation_witness_root": self.observation_witness_root,
            "dashboard_cell_root": self.dashboard_cell_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-MONERO-RELEASE-OBSERVATION",
            &self.public_record(),
        )
    }

    pub fn is_accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && self.confirmations >= config.required_monero_confirmations
            && !self.release_txid.is_empty()
            && !self.view_key_scan_root.is_empty()
            && !self.output_membership_root.is_empty()
            && !self.amount_commitment_root.is_empty()
            && !self.destination_binding_root.is_empty()
            && !self.observation_witness_root.is_empty()
            && !self.dashboard_cell_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveHandoffRoot {
    pub handoff_id: String,
    pub reserve_operator_id: String,
    pub handoff_height: u64,
    pub confirmations: u64,
    pub reserve_balance_root: String,
    pub custody_delta_root: String,
    pub emergency_recovery_root: String,
    pub handoff_witness_root: String,
    pub dashboard_cell_root: String,
    pub status: EvidenceStatus,
}

impl ReserveHandoffRoot {
    pub fn devnet(config: &Config) -> Self {
        let reserve_operator_id = "reserve-operator-devnet-primary";
        let handoff_id = evidence_id(config, "reserve-handoff-root", reserve_operator_id, 1);
        let reserve_balance_root =
            reserve_component_root(config, reserve_operator_id, "reserve-balance");
        let custody_delta_root =
            reserve_component_root(config, reserve_operator_id, "custody-delta");
        let emergency_recovery_root =
            reserve_component_root(config, reserve_operator_id, "emergency-recovery");
        let handoff_witness_root = merkle_root(
            "BRIDGE-CUSTODY-BINDING-RESERVE-HANDOFF-WITNESS",
            &[
                json!({"kind": "reserve_balance", "root": reserve_balance_root}),
                json!({"kind": "custody_delta", "root": custody_delta_root}),
                json!({"kind": "emergency_recovery", "root": emergency_recovery_root}),
                json!({"kind": "operator", "root": reserve_operator_id}),
            ],
        );
        let dashboard_cell_root =
            dashboard_cell_root(config, "reserve_handoff", &handoff_witness_root);
        Self {
            handoff_id,
            reserve_operator_id: reserve_operator_id.to_string(),
            handoff_height: config.binding_height.saturating_sub(20),
            confirmations: config.required_reserve_handoff_confirmations,
            reserve_balance_root,
            custody_delta_root,
            emergency_recovery_root,
            handoff_witness_root,
            dashboard_cell_root,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "handoff_id": self.handoff_id,
            "reserve_operator_id": self.reserve_operator_id,
            "handoff_height": self.handoff_height,
            "confirmations": self.confirmations,
            "reserve_balance_root": self.reserve_balance_root,
            "custody_delta_root": self.custody_delta_root,
            "emergency_recovery_root": self.emergency_recovery_root,
            "handoff_witness_root": self.handoff_witness_root,
            "dashboard_cell_root": self.dashboard_cell_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("BRIDGE-CUSTODY-RESERVE-HANDOFF-ROOT", &self.public_record())
    }

    pub fn is_accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && self.confirmations >= config.required_reserve_handoff_confirmations
            && !self.reserve_balance_root.is_empty()
            && !self.custody_delta_root.is_empty()
            && !self.emergency_recovery_root.is_empty()
            && !self.handoff_witness_root.is_empty()
            && !self.dashboard_cell_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DashboardApproval {
    pub approval_id: String,
    pub approver_id: String,
    pub role: DashboardApprovalRole,
    pub approved_dashboard_root: String,
    pub approved_policy_root: String,
    pub approval_evidence_root: String,
    pub approved_at_height: u64,
    pub status: EvidenceStatus,
}

impl DashboardApproval {
    pub fn devnet(
        config: &Config,
        ordinal: u64,
        approver_id: &str,
        role: DashboardApprovalRole,
    ) -> Self {
        let approval_id = evidence_id(config, "dashboard-approval", approver_id, ordinal);
        let approved_dashboard_root = source_dashboard_root(config);
        let approved_policy_root = config.state_root();
        let approval_evidence_root = approval_evidence_root(
            config,
            approver_id,
            role,
            &approved_dashboard_root,
            &approved_policy_root,
        );
        Self {
            approval_id,
            approver_id: approver_id.to_string(),
            role,
            approved_dashboard_root,
            approved_policy_root,
            approval_evidence_root,
            approved_at_height: config.binding_height.saturating_add(ordinal),
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "approval_id": self.approval_id,
            "approver_id": self.approver_id,
            "role": self.role.as_str(),
            "approved_dashboard_root": self.approved_dashboard_root,
            "approved_policy_root": self.approved_policy_root,
            "approval_evidence_root": self.approval_evidence_root,
            "approved_at_height": self.approved_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("BRIDGE-CUSTODY-DASHBOARD-APPROVAL", &self.public_record())
    }

    pub fn is_accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && self.approved_dashboard_root == source_dashboard_root(config)
            && self.approved_policy_root == config.state_root()
            && !self.approval_evidence_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerQuorumAttestation {
    pub attestation_id: String,
    pub reviewer_id: String,
    pub reviewer_weight: u64,
    pub signed_binding_root: String,
    pub review_note_root: String,
    pub reviewed_at_height: u64,
    pub status: EvidenceStatus,
}

impl ReviewerQuorumAttestation {
    pub fn devnet(config: &Config, ordinal: u64, reviewer_id: &str, reviewer_weight: u64) -> Self {
        let attestation_id =
            evidence_id(config, "reviewer-quorum-attestation", reviewer_id, ordinal);
        let review_note_root = reviewer_note_root(config, reviewer_id, reviewer_weight);
        let signed_binding_root =
            provisional_binding_root(config, &review_note_root, reviewer_weight);
        Self {
            attestation_id,
            reviewer_id: reviewer_id.to_string(),
            reviewer_weight,
            signed_binding_root,
            review_note_root,
            reviewed_at_height: config
                .binding_height
                .saturating_add(10)
                .saturating_add(ordinal),
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "attestation_id": self.attestation_id,
            "reviewer_id": self.reviewer_id,
            "reviewer_weight": self.reviewer_weight,
            "signed_binding_root": self.signed_binding_root,
            "review_note_root": self.review_note_root,
            "reviewed_at_height": self.reviewed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-REVIEWER-QUORUM-ATTESTATION",
            &self.public_record(),
        )
    }

    pub fn is_accepted(&self) -> bool {
        self.status.accepted()
            && self.reviewer_weight > 0
            && !self.signed_binding_root.is_empty()
            && !self.review_note_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorAction {
    pub action_id: String,
    pub owner: String,
    pub action_label: String,
    pub evidence_root: String,
    pub closed_at_height: u64,
    pub status: EvidenceStatus,
}

impl OperatorAction {
    pub fn closed(config: &Config, ordinal: u64, owner: &str, action_label: &str) -> Self {
        let action_id = evidence_id(config, "operator-action", action_label, ordinal);
        let evidence_root = operator_action_root(config, owner, action_label, ordinal);
        Self {
            action_id,
            owner: owner.to_string(),
            action_label: action_label.to_string(),
            evidence_root,
            closed_at_height: config
                .binding_height
                .saturating_add(20)
                .saturating_add(ordinal),
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "action_id": self.action_id,
            "owner": self.owner,
            "action_label": self.action_label,
            "evidence_root": self.evidence_root,
            "closed_at_height": self.closed_at_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("BRIDGE-CUSTODY-OPERATOR-ACTION", &self.public_record())
    }

    pub fn is_closed(&self) -> bool {
        self.status.accepted() && !self.evidence_root.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleasePolicyBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub evidence_root: String,
    pub detail: String,
    pub observed_at_height: u64,
}

impl ReleasePolicyBlocker {
    pub fn new(config: &Config, kind: BlockerKind, evidence_root: &str, detail: &str) -> Self {
        Self {
            blocker_id: blocker_id(config, kind, evidence_root),
            kind,
            evidence_root: evidence_root.to_string(),
            detail: detail.to_string(),
            observed_at_height: config.binding_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "detail": self.detail,
            "observed_at_height": self.observed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-RELEASE-POLICY-BLOCKER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceCounters {
    pub accepted_signer_count: u64,
    pub accepted_signer_weight: u64,
    pub accepted_dashboard_approval_count: u64,
    pub accepted_reviewer_count: u64,
    pub accepted_reviewer_weight: u64,
    pub closed_operator_action_count: u64,
    pub total_operator_action_count: u64,
    pub blocker_count: u64,
}

impl EvidenceCounters {
    pub fn from_parts(
        config: &Config,
        signer_receipts: &[CustodySignerReceipt],
        dashboard_approvals: &[DashboardApproval],
        reviewer_attestations: &[ReviewerQuorumAttestation],
        operator_actions: &[OperatorAction],
        blockers: &[ReleasePolicyBlocker],
    ) -> Self {
        let accepted_signers = signer_receipts
            .iter()
            .filter(|receipt| receipt.is_accepted(config))
            .collect::<Vec<_>>();
        let accepted_reviewers = reviewer_attestations
            .iter()
            .filter(|attestation| attestation.is_accepted())
            .collect::<Vec<_>>();
        let closed_operator_action_count = operator_actions
            .iter()
            .filter(|action| action.is_closed())
            .count() as u64;
        Self {
            accepted_signer_count: accepted_signers.len() as u64,
            accepted_signer_weight: accepted_signers.iter().fold(0_u64, |total, receipt| {
                total.saturating_add(receipt.signer_weight)
            }),
            accepted_dashboard_approval_count: dashboard_approvals
                .iter()
                .filter(|approval| approval.is_accepted(config))
                .count() as u64,
            accepted_reviewer_count: accepted_reviewers.len() as u64,
            accepted_reviewer_weight: accepted_reviewers
                .iter()
                .fold(0_u64, |total, attestation| {
                    total.saturating_add(attestation.reviewer_weight)
                }),
            closed_operator_action_count,
            total_operator_action_count: operator_actions.len() as u64,
            blocker_count: blockers.len() as u64,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "accepted_signer_count": self.accepted_signer_count,
            "accepted_signer_weight": self.accepted_signer_weight,
            "accepted_dashboard_approval_count": self.accepted_dashboard_approval_count,
            "accepted_reviewer_count": self.accepted_reviewer_count,
            "accepted_reviewer_weight": self.accepted_reviewer_weight,
            "closed_operator_action_count": self.closed_operator_action_count,
            "total_operator_action_count": self.total_operator_action_count,
            "blocker_count": self.blocker_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-RELEASE-POLICY-COUNTERS",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleasePolicyBinding {
    pub binding_id: String,
    pub source_runbook_root: String,
    pub source_dashboard_root: String,
    pub custody_signer_receipt_root: String,
    pub monero_release_observation_root: String,
    pub reserve_handoff_root: String,
    pub dashboard_approval_root: String,
    pub reviewer_quorum_root: String,
    pub operator_action_root: String,
    pub blocker_root: String,
    pub counters_root: String,
    pub release_policy_root: String,
    pub go_no_go_evidence_root: String,
    pub verdict: Verdict,
}

impl ReleasePolicyBinding {
    pub fn public_record(&self) -> Value {
        json!({
            "binding_id": self.binding_id,
            "source_runbook_root": self.source_runbook_root,
            "source_dashboard_root": self.source_dashboard_root,
            "custody_signer_receipt_root": self.custody_signer_receipt_root,
            "monero_release_observation_root": self.monero_release_observation_root,
            "reserve_handoff_root": self.reserve_handoff_root,
            "dashboard_approval_root": self.dashboard_approval_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "operator_action_root": self.operator_action_root,
            "blocker_root": self.blocker_root,
            "counters_root": self.counters_root,
            "release_policy_root": self.release_policy_root,
            "go_no_go_evidence_root": self.go_no_go_evidence_root,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "BRIDGE-CUSTODY-RELEASE-POLICY-BINDING",
            &self.public_record(),
        )
    }

    pub fn permits_release(&self) -> bool {
        self.verdict.permits_release()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub custody_signer_receipts: Vec<CustodySignerReceipt>,
    pub monero_release_observation: MoneroReleaseObservation,
    pub reserve_handoff_root: ReserveHandoffRoot,
    pub dashboard_approvals: Vec<DashboardApproval>,
    pub reviewer_attestations: Vec<ReviewerQuorumAttestation>,
    pub operator_actions: Vec<OperatorAction>,
    pub blockers: Vec<ReleasePolicyBlocker>,
    pub counters: EvidenceCounters,
    pub release_policy_binding: ReleasePolicyBinding,
}

impl State {
    pub fn new(
        config: Config,
        custody_signer_receipts: Vec<CustodySignerReceipt>,
        monero_release_observation: MoneroReleaseObservation,
        reserve_handoff_root: ReserveHandoffRoot,
        dashboard_approvals: Vec<DashboardApproval>,
        reviewer_attestations: Vec<ReviewerQuorumAttestation>,
        operator_actions: Vec<OperatorAction>,
    ) -> Result<Self> {
        ensure_unique_signers(&custody_signer_receipts)?;
        ensure_unique_approvers(&dashboard_approvals)?;
        ensure_unique_reviewers(&reviewer_attestations)?;
        Ok(build_state(
            config,
            custody_signer_receipts,
            monero_release_observation,
            reserve_handoff_root,
            dashboard_approvals,
            reviewer_attestations,
            operator_actions,
        ))
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "custody_signer_receipts": self
                .custody_signer_receipts
                .iter()
                .map(CustodySignerReceipt::public_record)
                .collect::<Vec<_>>(),
            "monero_release_observation": self.monero_release_observation.public_record(),
            "reserve_handoff_root": self.reserve_handoff_root.public_record(),
            "dashboard_approvals": self
                .dashboard_approvals
                .iter()
                .map(DashboardApproval::public_record)
                .collect::<Vec<_>>(),
            "reviewer_attestations": self
                .reviewer_attestations
                .iter()
                .map(ReviewerQuorumAttestation::public_record)
                .collect::<Vec<_>>(),
            "operator_actions": self
                .operator_actions
                .iter()
                .map(OperatorAction::public_record)
                .collect::<Vec<_>>(),
            "blockers": self
                .blockers
                .iter()
                .map(ReleasePolicyBlocker::public_record)
                .collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "release_policy_binding": self.release_policy_binding.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "BRIDGE-CUSTODY-RELEASE-POLICY-BINDING-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&self.release_policy_binding.state_root()),
            ],
            32,
        )
    }

    pub fn release_ready(&self) -> bool {
        self.release_policy_binding.permits_release()
            && self.counters.blocker_count == 0
            && self.counters.accepted_signer_count >= self.config.required_signer_count
            && self.counters.accepted_signer_weight >= self.config.required_signer_weight
            && self.counters.accepted_dashboard_approval_count
                >= self.config.required_dashboard_approvals
            && self.counters.accepted_reviewer_count >= self.config.required_reviewer_count
            && self.counters.accepted_reviewer_weight >= self.config.required_reviewer_weight
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let custody_signer_receipts = vec![
        CustodySignerReceipt::devnet(&config, 1, "custody-signer-alpha", 20),
        CustodySignerReceipt::devnet(&config, 2, "custody-signer-beta", 18),
        CustodySignerReceipt::devnet(&config, 3, "custody-signer-gamma", 17),
        CustodySignerReceipt::devnet(&config, 4, "custody-signer-delta", 16),
    ];
    let monero_release_observation = MoneroReleaseObservation::devnet(&config);
    let reserve_handoff_root = ReserveHandoffRoot::devnet(&config);
    let dashboard_approvals = vec![
        DashboardApproval::devnet(
            &config,
            1,
            "custody-dashboard-operator",
            DashboardApprovalRole::CustodyOperator,
        ),
        DashboardApproval::devnet(
            &config,
            2,
            "force-exit-release-coordinator",
            DashboardApprovalRole::ReleaseCoordinator,
        ),
        DashboardApproval::devnet(
            &config,
            3,
            "incident-commander-primary",
            DashboardApprovalRole::IncidentCommander,
        ),
    ];
    let reviewer_attestations = vec![
        ReviewerQuorumAttestation::devnet(&config, 1, "release-policy-reviewer-alpha", 25),
        ReviewerQuorumAttestation::devnet(&config, 2, "release-policy-reviewer-beta", 25),
        ReviewerQuorumAttestation::devnet(&config, 3, "release-policy-reviewer-gamma", 25),
    ];
    let operator_actions = vec![
        OperatorAction::closed(&config, 1, "custody-ops", "publish-signer-receipt-panel"),
        OperatorAction::closed(&config, 2, "monero-ops", "attach-release-observation-panel"),
        OperatorAction::closed(&config, 3, "reserve-ops", "bind-reserve-handoff-panel"),
        OperatorAction::closed(&config, 4, "release-ops", "record-final-go-no-go-panel"),
    ];
    build_state(
        config,
        custody_signer_receipts,
        monero_release_observation,
        reserve_handoff_root,
        dashboard_approvals,
        reviewer_attestations,
        operator_actions,
    )
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn build_state(
    config: Config,
    custody_signer_receipts: Vec<CustodySignerReceipt>,
    monero_release_observation: MoneroReleaseObservation,
    reserve_handoff_root: ReserveHandoffRoot,
    dashboard_approvals: Vec<DashboardApproval>,
    reviewer_attestations: Vec<ReviewerQuorumAttestation>,
    operator_actions: Vec<OperatorAction>,
) -> State {
    let blockers = blockers_from_parts(
        &config,
        &custody_signer_receipts,
        &monero_release_observation,
        &reserve_handoff_root,
        &dashboard_approvals,
        &reviewer_attestations,
        &operator_actions,
    );
    let counters = EvidenceCounters::from_parts(
        &config,
        &custody_signer_receipts,
        &dashboard_approvals,
        &reviewer_attestations,
        &operator_actions,
        &blockers,
    );
    let release_policy_binding = binding_from_parts(
        &config,
        &custody_signer_receipts,
        &monero_release_observation,
        &reserve_handoff_root,
        &dashboard_approvals,
        &reviewer_attestations,
        &operator_actions,
        &blockers,
        &counters,
    );
    State {
        config,
        custody_signer_receipts,
        monero_release_observation,
        reserve_handoff_root,
        dashboard_approvals,
        reviewer_attestations,
        operator_actions,
        blockers,
        counters,
        release_policy_binding,
    }
}

fn blockers_from_parts(
    config: &Config,
    signer_receipts: &[CustodySignerReceipt],
    monero_release_observation: &MoneroReleaseObservation,
    reserve_handoff_root: &ReserveHandoffRoot,
    dashboard_approvals: &[DashboardApproval],
    reviewer_attestations: &[ReviewerQuorumAttestation],
    operator_actions: &[OperatorAction],
) -> Vec<ReleasePolicyBlocker> {
    let mut blockers = Vec::new();
    let accepted_signer_count = signer_receipts
        .iter()
        .filter(|receipt| receipt.is_accepted(config))
        .count() as u64;
    let accepted_signer_weight = signer_receipts
        .iter()
        .filter(|receipt| receipt.is_accepted(config))
        .fold(0_u64, |total, receipt| {
            total.saturating_add(receipt.signer_weight)
        });
    if accepted_signer_count < config.required_signer_count {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingCustodySignerReceipt,
            &list_root(
                "BRIDGE-CUSTODY-SIGNER-RECEIPTS",
                signer_receipts.iter().map(CustodySignerReceipt::state_root),
            ),
            "accepted custody signer receipt count is below release policy",
        ));
    }
    if accepted_signer_weight < config.required_signer_weight {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::InsufficientCustodySignerWeight,
            &config.state_root(),
            "accepted custody signer weight is below release policy",
        ));
    }
    if config.require_monero_release_observation && !monero_release_observation.is_accepted(config)
    {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingMoneroReleaseObservation,
            &monero_release_observation.state_root(),
            "Monero release transaction observation is not accepted",
        ));
    }
    if monero_release_observation.confirmations < config.required_monero_confirmations {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::InsufficientMoneroConfirmations,
            &monero_release_observation.state_root(),
            "Monero release transaction confirmation count is below policy",
        ));
    }
    if config.require_reserve_handoff && !reserve_handoff_root.is_accepted(config) {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingReserveHandoffRoot,
            &reserve_handoff_root.state_root(),
            "reserve handoff roots are not accepted",
        ));
    }
    if reserve_handoff_root.confirmations < config.required_reserve_handoff_confirmations {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::InsufficientReserveConfirmations,
            &reserve_handoff_root.state_root(),
            "reserve handoff confirmation count is below policy",
        ));
    }
    let accepted_approvals = dashboard_approvals
        .iter()
        .filter(|approval| approval.is_accepted(config))
        .count() as u64;
    if config.require_dashboard_approval_quorum
        && accepted_approvals < config.required_dashboard_approvals
    {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingDashboardApproval,
            &list_root(
                "BRIDGE-CUSTODY-DASHBOARD-APPROVALS",
                dashboard_approvals
                    .iter()
                    .map(DashboardApproval::state_root),
            ),
            "operator dashboard approval quorum is below release policy",
        ));
    }
    let accepted_reviewers = reviewer_attestations
        .iter()
        .filter(|attestation| attestation.is_accepted())
        .collect::<Vec<_>>();
    let accepted_reviewer_weight = accepted_reviewers.iter().fold(0_u64, |total, attestation| {
        total.saturating_add(attestation.reviewer_weight)
    });
    if config.require_reviewer_quorum
        && (accepted_reviewers.len() as u64) < config.required_reviewer_count
    {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingReviewerQuorum,
            &list_root(
                "BRIDGE-CUSTODY-REVIEWER-ATTESTATIONS",
                reviewer_attestations
                    .iter()
                    .map(ReviewerQuorumAttestation::state_root),
            ),
            "reviewer count quorum is below release policy",
        ));
    }
    if config.require_reviewer_quorum && accepted_reviewer_weight < config.required_reviewer_weight
    {
        blockers.push(ReleasePolicyBlocker::new(
            config,
            BlockerKind::MissingReviewerQuorum,
            &config.state_root(),
            "reviewer weight quorum is below release policy",
        ));
    }
    for action in operator_actions {
        if !action.is_closed() {
            blockers.push(ReleasePolicyBlocker::new(
                config,
                BlockerKind::OpenOperatorAction,
                &action.state_root(),
                "operator dashboard action remains open",
            ));
        }
    }
    for receipt in signer_receipts {
        if matches!(
            receipt.status,
            EvidenceStatus::Rejected | EvidenceStatus::Blocked
        ) {
            blockers.push(ReleasePolicyBlocker::new(
                config,
                BlockerKind::RejectedEvidence,
                &receipt.state_root(),
                "custody signer receipt is rejected or blocked",
            ));
        }
    }
    for approval in dashboard_approvals {
        if approval.approved_policy_root != config.state_root() {
            blockers.push(ReleasePolicyBlocker::new(
                config,
                BlockerKind::PolicyRootMismatch,
                &approval.state_root(),
                "dashboard approval references a policy root outside this binding",
            ));
        }
        if approval.approved_dashboard_root != source_dashboard_root(config) {
            blockers.push(ReleasePolicyBlocker::new(
                config,
                BlockerKind::DashboardRootMismatch,
                &approval.state_root(),
                "dashboard approval references a dashboard root outside this binding",
            ));
        }
    }
    blockers
}

fn binding_from_parts(
    config: &Config,
    custody_signer_receipts: &[CustodySignerReceipt],
    monero_release_observation: &MoneroReleaseObservation,
    reserve_handoff_root: &ReserveHandoffRoot,
    dashboard_approvals: &[DashboardApproval],
    reviewer_attestations: &[ReviewerQuorumAttestation],
    operator_actions: &[OperatorAction],
    blockers: &[ReleasePolicyBlocker],
    counters: &EvidenceCounters,
) -> ReleasePolicyBinding {
    let custody_signer_receipt_root = list_root(
        "BRIDGE-CUSTODY-BINDING-SIGNER-RECEIPT-ROOT",
        custody_signer_receipts
            .iter()
            .map(CustodySignerReceipt::state_root),
    );
    let monero_release_observation_root = monero_release_observation.state_root();
    let reserve_handoff_state_root = reserve_handoff_root.state_root();
    let dashboard_approval_root = list_root(
        "BRIDGE-CUSTODY-BINDING-DASHBOARD-APPROVAL-ROOT",
        dashboard_approvals
            .iter()
            .map(DashboardApproval::state_root),
    );
    let reviewer_quorum_root = list_root(
        "BRIDGE-CUSTODY-BINDING-REVIEWER-QUORUM-ROOT",
        reviewer_attestations
            .iter()
            .map(ReviewerQuorumAttestation::state_root),
    );
    let operator_action_root = list_root(
        "BRIDGE-CUSTODY-BINDING-OPERATOR-ACTION-ROOT",
        operator_actions.iter().map(OperatorAction::state_root),
    );
    let blocker_root = list_root(
        "BRIDGE-CUSTODY-BINDING-BLOCKER-ROOT",
        blockers.iter().map(ReleasePolicyBlocker::state_root),
    );
    let counters_root = counters.state_root();
    let source_runbook_root = source_runbook_root(config);
    let source_dashboard_root = source_dashboard_root(config);
    let release_policy_root = config.state_root();
    let go_no_go_evidence_root = merkle_root(
        "BRIDGE-CUSTODY-BINDING-GO-NO-GO-EVIDENCE",
        &[
            json!({"kind": "source_runbook", "root": source_runbook_root}),
            json!({"kind": "source_dashboard", "root": source_dashboard_root}),
            json!({"kind": "custody_signer_receipts", "root": custody_signer_receipt_root}),
            json!({"kind": "monero_release_observation", "root": monero_release_observation_root}),
            json!({"kind": "reserve_handoff", "root": reserve_handoff_state_root}),
            json!({"kind": "dashboard_approvals", "root": dashboard_approval_root}),
            json!({"kind": "reviewer_quorum", "root": reviewer_quorum_root}),
            json!({"kind": "operator_actions", "root": operator_action_root}),
            json!({"kind": "blockers", "root": blocker_root}),
            json!({"kind": "counters", "root": counters_root}),
        ],
    );
    let verdict = if config.fail_closed && blockers.is_empty() {
        Verdict::Go
    } else if !config.fail_closed {
        Verdict::Go
    } else {
        Verdict::NoGo
    };
    ReleasePolicyBinding {
        binding_id: config.binding_id.clone(),
        source_runbook_root,
        source_dashboard_root,
        custody_signer_receipt_root,
        monero_release_observation_root,
        reserve_handoff_root: reserve_handoff_state_root,
        dashboard_approval_root,
        reviewer_quorum_root,
        operator_action_root,
        blocker_root,
        counters_root,
        release_policy_root,
        go_no_go_evidence_root,
        verdict,
    }
}

fn binding_id(label: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-BINDING-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn evidence_id(config: &Config, kind: &str, subject: &str, ordinal: u64) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-EVIDENCE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(kind),
            HashPart::Str(subject),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn pq_signature_root(
    config: &Config,
    signer_id: &str,
    ordinal: u64,
    scheme: &str,
    signer_weight: u64,
) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-PQ-SIGNATURE",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_runbook_audit_id),
            HashPart::Str(signer_id),
            HashPart::U64(ordinal),
            HashPart::Str(scheme),
            HashPart::U64(signer_weight),
            HashPart::U64(config.previous_wave),
        ],
        32,
    )
}

fn custody_commitment_root(
    config: &Config,
    signer_id: &str,
    ml_dsa_signature_root: &str,
    slh_dsa_signature_root: &str,
) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-CUSTODY-COMMITMENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.bridge_custody_lane_id),
            HashPart::Str(signer_id),
            HashPart::Str(ml_dsa_signature_root),
            HashPart::Str(slh_dsa_signature_root),
        ],
        32,
    )
}

fn accepted_live_evidence_root(
    config: &Config,
    kind: &str,
    evidence_id: &str,
    evidence_root: &str,
) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_runbook_audit_id),
            HashPart::Str(kind),
            HashPart::Str(evidence_id),
            HashPart::Str(evidence_root),
        ],
        32,
    )
}

fn ack_root(config: &Config, actor: &str, subject: &str, evidence_root: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-ACK",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(actor),
            HashPart::Str(subject),
            HashPart::Str(evidence_root),
            HashPart::U64(config.acceptance_epoch),
        ],
        32,
    )
}

fn release_component_root(config: &Config, release_txid: &str, component: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-MONERO-RELEASE-COMPONENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_dashboard_finalization_id),
            HashPart::Str(release_txid),
            HashPart::Str(component),
            HashPart::U64(DEFAULT_MONERO_RELEASE_HEIGHT),
        ],
        32,
    )
}

fn reserve_component_root(config: &Config, reserve_operator_id: &str, component: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-RESERVE-COMPONENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_runbook_audit_id),
            HashPart::Str(reserve_operator_id),
            HashPart::Str(component),
            HashPart::U64(config.binding_height),
        ],
        32,
    )
}

fn dashboard_cell_root(config: &Config, cell: &str, evidence_root: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-DASHBOARD-CELL",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.bridge_custody_lane_id),
            HashPart::Str(cell),
            HashPart::Str(evidence_root),
        ],
        32,
    )
}

fn approval_evidence_root(
    config: &Config,
    approver_id: &str,
    role: DashboardApprovalRole,
    dashboard_root: &str,
    policy_root: &str,
) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-DASHBOARD-APPROVAL",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(approver_id),
            HashPart::Str(role.as_str()),
            HashPart::Str(dashboard_root),
            HashPart::Str(policy_root),
        ],
        32,
    )
}

fn reviewer_note_root(config: &Config, reviewer_id: &str, reviewer_weight: u64) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-REVIEWER-NOTE",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(reviewer_id),
            HashPart::U64(reviewer_weight),
            HashPart::U64(config.acceptance_epoch),
        ],
        32,
    )
}

fn provisional_binding_root(
    config: &Config,
    review_note_root: &str,
    reviewer_weight: u64,
) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-PROVISIONAL-BINDING",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(review_note_root),
            HashPart::U64(reviewer_weight),
        ],
        32,
    )
}

fn operator_action_root(config: &Config, owner: &str, action_label: &str, ordinal: u64) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-OPERATOR-ACTION",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_dashboard_finalization_id),
            HashPart::Str(owner),
            HashPart::Str(action_label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn source_runbook_root(config: &Config) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-SOURCE-RUNBOOK-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_runbook_audit_id),
            HashPart::Str(&config.bridge_custody_lane_id),
            HashPart::U64(config.previous_wave),
        ],
        32,
    )
}

fn source_dashboard_root(config: &Config) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-SOURCE-DASHBOARD-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.source_dashboard_finalization_id),
            HashPart::Str(&config.bridge_custody_lane_id),
            HashPart::U64(config.binding_height),
        ],
        32,
    )
}

fn blocker_id(config: &Config, kind: BlockerKind, evidence_root: &str) -> String {
    domain_hash(
        "BRIDGE-CUSTODY-RELEASE-POLICY-BLOCKER-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.binding_id),
            HashPart::Str(kind.as_str()),
            HashPart::Str(evidence_root),
            HashPart::U64(config.binding_height),
        ],
        32,
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

fn list_root<I>(domain: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots
        .into_iter()
        .map(|root| json!({"root": root}))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_unique_signers(receipts: &[CustodySignerReceipt]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for receipt in receipts {
        ensure(
            seen.insert(receipt.signer_id.clone()),
            "duplicate custody signer receipt",
        )?;
    }
    Ok(())
}

fn ensure_unique_approvers(approvals: &[DashboardApproval]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for approval in approvals {
        ensure(
            seen.insert(approval.approver_id.clone()),
            "duplicate dashboard approver",
        )?;
    }
    Ok(())
}

fn ensure_unique_reviewers(attestations: &[ReviewerQuorumAttestation]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for attestation in attestations {
        ensure(
            seen.insert(attestation.reviewer_id.clone()),
            "duplicate release policy reviewer",
        )?;
    }
    Ok(())
}
