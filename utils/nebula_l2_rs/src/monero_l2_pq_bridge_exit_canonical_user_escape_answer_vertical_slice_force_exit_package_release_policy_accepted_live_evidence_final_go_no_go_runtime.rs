use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReleasePolicyAcceptedLiveEvidenceFinalGoNoGoRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_FINAL_GO_NO_GO_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-canonical-force-exit-release-policy-accepted-live-evidence-final-go-no-go-v1";
pub const PROTOCOL_VERSION: &str = MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_FINAL_GO_NO_GO_RUNTIME_PROTOCOL_VERSION;
pub const DEFAULT_FINALITY_HEIGHT: u64 = 92_000;
pub const DEFAULT_MAX_RECEIPT_AGE_BLOCKS: u64 = 160;
pub const DEFAULT_MIN_OPERATOR_ACKS: u16 = 2;
pub const DEFAULT_MIN_GOVERNANCE_ATTESTATIONS: u16 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLane {
    CompileRuntime,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl EvidenceLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileRuntime,
            Self::RuntimeReplay,
            Self::AuditSecurity,
            Self::BridgeCustody,
            Self::WalletWatchtower,
            Self::PqReservePrivacy,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile_runtime",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn imported_module(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile_runtime_accepted_live_evidence_import",
            Self::RuntimeReplay => "runtime_replay_accepted_live_evidence_import",
            Self::AuditSecurity => "audit_security_accepted_live_evidence_import",
            Self::BridgeCustody => "bridge_custody_accepted_live_evidence_import",
            Self::WalletWatchtower => "wallet_watchtower_accepted_live_evidence_import",
            Self::PqReservePrivacy => "pq_reserve_privacy_accepted_live_evidence_import",
        }
    }

    pub fn requires_privacy_boundary(self) -> bool {
        matches!(
            self,
            Self::AuditSecurity | Self::WalletWatchtower | Self::PqReservePrivacy
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Missing,
    Deferred,
    PendingReview,
    Accepted,
    Rejected,
    Stale,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Deferred => "deferred",
            Self::PendingReview => "pending_review",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Stale => "stale",
        }
    }

    pub fn permits_governance_import(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingEvidence,
    DeferredEvidence,
    PendingReview,
    RejectedEvidence,
    StaleEvidence,
    MissingReleaseManifest,
    CircuitBreakerStillArmed,
    MissingOperatorAcknowledgement,
    MissingGovernanceAttestation,
    PrivacyBoundaryMissing,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingEvidence => "missing_evidence",
            Self::DeferredEvidence => "deferred_evidence",
            Self::PendingReview => "pending_review",
            Self::RejectedEvidence => "rejected_evidence",
            Self::StaleEvidence => "stale_evidence",
            Self::MissingReleaseManifest => "missing_release_manifest",
            Self::CircuitBreakerStillArmed => "circuit_breaker_still_armed",
            Self::MissingOperatorAcknowledgement => "missing_operator_acknowledgement",
            Self::MissingGovernanceAttestation => "missing_governance_attestation",
            Self::PrivacyBoundaryMissing => "privacy_boundary_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub import_policy_id: String,
    pub required_lanes: Vec<EvidenceLane>,
    pub finality_height: u64,
    pub max_receipt_age_blocks: u64,
    pub min_operator_acknowledgements: u16,
    pub min_governance_attestations: u16,
    pub require_release_manifest_root: bool,
    pub require_circuit_breaker_clearance: bool,
    pub require_privacy_boundaries: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            import_policy_id: policy_id("devnet-final-go-no-go-live-evidence-import"),
            required_lanes: EvidenceLane::all(),
            finality_height: DEFAULT_FINALITY_HEIGHT,
            max_receipt_age_blocks: DEFAULT_MAX_RECEIPT_AGE_BLOCKS,
            min_operator_acknowledgements: DEFAULT_MIN_OPERATOR_ACKS,
            min_governance_attestations: DEFAULT_MIN_GOVERNANCE_ATTESTATIONS,
            require_release_manifest_root: true,
            require_circuit_breaker_clearance: true,
            require_privacy_boundaries: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("import_policy_id", &self.import_policy_id)?;
        ensure(
            !self.required_lanes.is_empty(),
            "at least one evidence lane is required",
        )?;
        ensure(
            self.max_receipt_age_blocks > 0,
            "max receipt age must be non-zero",
        )?;
        ensure(
            self.min_operator_acknowledgements > 0,
            "operator acknowledgement quorum must be non-zero",
        )?;
        ensure(
            self.min_governance_attestations > 0,
            "governance attestation quorum must be non-zero",
        )?;
        let mut seen = BTreeSet::new();
        for lane in &self.required_lanes {
            ensure(seen.insert(*lane), "duplicate evidence lane")?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "import_policy_id": self.import_policy_id,
            "required_lanes": self.required_lanes.iter().map(|lane| lane.as_str()).collect::<Vec<_>>(),
            "finality_height": self.finality_height,
            "max_receipt_age_blocks": self.max_receipt_age_blocks,
            "min_operator_acknowledgements": self.min_operator_acknowledgements,
            "min_governance_attestations": self.min_governance_attestations,
            "require_release_manifest_root": self.require_release_manifest_root,
            "require_circuit_breaker_clearance": self.require_circuit_breaker_clearance,
            "require_privacy_boundaries": self.require_privacy_boundaries,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn policy_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-CONFIG",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptedLiveEvidenceReceipt {
    pub lane: EvidenceLane,
    pub receipt_id: String,
    pub source_module: String,
    pub live_receipt_root: String,
    pub enforcement_root: String,
    pub final_go_no_go_root: String,
    pub reviewer_quorum_root: String,
    pub operator_ack_root: String,
    pub privacy_boundary_root: String,
    pub imported_at_height: u64,
    pub observed_at_height: u64,
    pub expires_at_height: u64,
    pub status: EvidenceStatus,
}

impl AcceptedLiveEvidenceReceipt {
    pub fn new(
        config: &Config,
        lane: EvidenceLane,
        receipt_id: &str,
        source_module: &str,
        imported_at_height: u64,
        observed_at_height: u64,
        status: EvidenceStatus,
    ) -> Result<Self> {
        ensure_non_empty("receipt_id", receipt_id)?;
        ensure_non_empty("source_module", source_module)?;
        ensure(
            imported_at_height >= observed_at_height,
            "import height must not precede observation height",
        )?;
        let expires_at_height = imported_at_height.saturating_add(config.max_receipt_age_blocks);
        let live_receipt_root = sample_root(lane, receipt_id, "live-receipt");
        let enforcement_root = sample_root(lane, receipt_id, "manifest-enforcement");
        let final_go_no_go_root = sample_root(lane, receipt_id, "final-go-no-go");
        let reviewer_quorum_root = sample_root(lane, receipt_id, "reviewer-quorum");
        let operator_ack_root = sample_root(lane, receipt_id, "operator-acknowledgement");
        let privacy_boundary_root = if lane.requires_privacy_boundary() {
            sample_root(lane, receipt_id, "privacy-boundary")
        } else {
            sample_root(lane, receipt_id, "privacy-boundary-not-required")
        };
        let receipt = Self {
            lane,
            receipt_id: receipt_id.to_string(),
            source_module: source_module.to_string(),
            live_receipt_root,
            enforcement_root,
            final_go_no_go_root,
            reviewer_quorum_root,
            operator_ack_root,
            privacy_boundary_root,
            imported_at_height,
            observed_at_height,
            expires_at_height,
            status,
        };
        receipt.validate(config)?;
        Ok(receipt)
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("receipt_id", &self.receipt_id)?;
        ensure_non_empty("source_module", &self.source_module)?;
        ensure_root("live_receipt_root", &self.live_receipt_root)?;
        ensure_root("enforcement_root", &self.enforcement_root)?;
        ensure_root("final_go_no_go_root", &self.final_go_no_go_root)?;
        ensure_root("reviewer_quorum_root", &self.reviewer_quorum_root)?;
        ensure_root("operator_ack_root", &self.operator_ack_root)?;
        if config.require_privacy_boundaries && self.lane.requires_privacy_boundary() {
            ensure_root("privacy_boundary_root", &self.privacy_boundary_root)?;
        }
        ensure(
            self.imported_at_height >= self.observed_at_height,
            "import height must not precede observation height",
        )?;
        ensure(
            self.expires_at_height >= self.imported_at_height,
            "receipt expiry must not precede import height",
        )?;
        Ok(())
    }

    pub fn is_fresh_at(&self, height: u64) -> bool {
        self.imported_at_height <= height && height <= self.expires_at_height
    }

    pub fn accepted_and_fresh_at(&self, height: u64) -> bool {
        self.status.permits_governance_import() && self.is_fresh_at(height)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane.as_str(),
            "receipt_id": self.receipt_id,
            "source_module": self.source_module,
            "live_receipt_root": self.live_receipt_root,
            "enforcement_root": self.enforcement_root,
            "final_go_no_go_root": self.final_go_no_go_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "operator_ack_root": self.operator_ack_root,
            "privacy_boundary_root": self.privacy_boundary_root,
            "imported_at_height": self.imported_at_height,
            "observed_at_height": self.observed_at_height,
            "expires_at_height": self.expires_at_height,
            "status": self.status.as_str(),
            "accepted_and_live_at_import": self.accepted_and_fresh_at(self.imported_at_height),
        })
    }

    pub fn receipt_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-RECEIPT",
            &self.public_record(),
        )
    }

    pub fn binding_root(&self) -> String {
        domain_hash(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-BINDING",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(self.lane.as_str()),
                HashPart::Str(&self.receipt_id),
                HashPart::Str(&self.live_receipt_root),
                HashPart::Str(&self.enforcement_root),
                HashPart::Str(&self.final_go_no_go_root),
                HashPart::Int(self.imported_at_height as i128),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceAttestation {
    pub attestor_id: String,
    pub attestation_root: String,
    pub signed_import_root: String,
    pub signed_at_height: u64,
}

impl GovernanceAttestation {
    pub fn new(
        attestor_id: &str,
        attestation_root: &str,
        signed_import_root: &str,
        signed_at_height: u64,
    ) -> Result<Self> {
        ensure_non_empty("attestor_id", attestor_id)?;
        ensure_root("attestation_root", attestation_root)?;
        ensure_root("signed_import_root", signed_import_root)?;
        Ok(Self {
            attestor_id: attestor_id.to_string(),
            attestation_root: attestation_root.to_string(),
            signed_import_root: signed_import_root.to_string(),
            signed_at_height,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "attestor_id": self.attestor_id,
            "attestation_root": self.attestation_root,
            "signed_import_root": self.signed_import_root,
            "signed_at_height": self.signed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-GOVERNANCE-ATTESTATION",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorAcknowledgement {
    pub operator_id: String,
    pub acknowledgement_root: String,
    pub signed_import_root: String,
    pub acknowledged_at_height: u64,
}

impl OperatorAcknowledgement {
    pub fn new(
        operator_id: &str,
        acknowledgement_root: &str,
        signed_import_root: &str,
        acknowledged_at_height: u64,
    ) -> Result<Self> {
        ensure_non_empty("operator_id", operator_id)?;
        ensure_root("acknowledgement_root", acknowledgement_root)?;
        ensure_root("signed_import_root", signed_import_root)?;
        Ok(Self {
            operator_id: operator_id.to_string(),
            acknowledgement_root: acknowledgement_root.to_string(),
            signed_import_root: signed_import_root.to_string(),
            acknowledged_at_height,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "operator_id": self.operator_id,
            "acknowledgement_root": self.acknowledgement_root,
            "signed_import_root": self.signed_import_root,
            "acknowledged_at_height": self.acknowledged_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-OPERATOR-ACKNOWLEDGEMENT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseBlocker {
    pub blocker_id: String,
    pub lane: String,
    pub kind: BlockerKind,
    pub evidence_root: String,
    pub detail: String,
    pub observed_at_height: u64,
}

impl ReleaseBlocker {
    pub fn new(
        lane: &str,
        kind: BlockerKind,
        evidence_root: &str,
        detail: &str,
        observed_at_height: u64,
    ) -> Self {
        Self {
            blocker_id: blocker_id(lane, kind, evidence_root, observed_at_height),
            lane: lane.to_string(),
            kind,
            evidence_root: evidence_root.to_string(),
            detail: detail.to_string(),
            observed_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "lane": self.lane,
            "kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "detail": self.detail,
            "observed_at_height": self.observed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-BLOCKER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportReadiness {
    pub required_lanes: usize,
    pub accepted_lanes: usize,
    pub fresh_lanes: usize,
    pub missing_lanes: Vec<String>,
    pub stale_lanes: Vec<String>,
    pub pending_lanes: Vec<String>,
    pub rejected_lanes: Vec<String>,
    pub operator_acknowledgements: usize,
    pub governance_attestations: usize,
    pub release_manifest_bound: bool,
    pub circuit_breaker_cleared: bool,
}

impl ImportReadiness {
    pub fn public_record(&self) -> Value {
        json!({
            "required_lanes": self.required_lanes,
            "accepted_lanes": self.accepted_lanes,
            "fresh_lanes": self.fresh_lanes,
            "missing_lanes": self.missing_lanes,
            "stale_lanes": self.stale_lanes,
            "pending_lanes": self.pending_lanes,
            "rejected_lanes": self.rejected_lanes,
            "operator_acknowledgements": self.operator_acknowledgements,
            "governance_attestations": self.governance_attestations,
            "release_manifest_bound": self.release_manifest_bound,
            "circuit_breaker_cleared": self.circuit_breaker_cleared,
        })
    }

    pub fn readiness_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-READINESS",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalGoNoGoVerdict {
    pub verdict_id: String,
    pub release_allowed: bool,
    pub import_readiness_root: String,
    pub imported_evidence_root: String,
    pub blocker_root: String,
    pub operator_acknowledgement_root: String,
    pub governance_attestation_root: String,
    pub release_manifest_root: String,
    pub circuit_breaker_clearance_root: String,
    pub previous_final_go_no_go_root: String,
    pub evaluated_at_height: u64,
}

impl FinalGoNoGoVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "release_allowed": self.release_allowed,
            "import_readiness_root": self.import_readiness_root,
            "imported_evidence_root": self.imported_evidence_root,
            "blocker_root": self.blocker_root,
            "operator_acknowledgement_root": self.operator_acknowledgement_root,
            "governance_attestation_root": self.governance_attestation_root,
            "release_manifest_root": self.release_manifest_root,
            "circuit_breaker_clearance_root": self.circuit_breaker_clearance_root,
            "previous_final_go_no_go_root": self.previous_final_go_no_go_root,
            "evaluated_at_height": self.evaluated_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-FINAL-GO-NO-GO-VERDICT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub release_manifest_root: String,
    pub circuit_breaker_clearance_root: String,
    pub previous_final_go_no_go_root: String,
    pub receipts: BTreeMap<EvidenceLane, AcceptedLiveEvidenceReceipt>,
    pub operator_acknowledgements: BTreeMap<String, OperatorAcknowledgement>,
    pub governance_attestations: BTreeMap<String, GovernanceAttestation>,
}

impl State {
    pub fn new(config: Config, height: u64) -> Result<Self> {
        config.validate()?;
        ensure(
            height >= config.finality_height,
            "state height must satisfy finality height",
        )?;
        Ok(Self {
            config,
            height,
            release_manifest_root: String::new(),
            circuit_breaker_clearance_root: String::new(),
            previous_final_go_no_go_root: String::new(),
            receipts: BTreeMap::new(),
            operator_acknowledgements: BTreeMap::new(),
            governance_attestations: BTreeMap::new(),
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut state = match Self::new(config.clone(), DEFAULT_FINALITY_HEIGHT + 12) {
            Ok(state) => state,
            Err(_) => Self {
                config,
                height: DEFAULT_FINALITY_HEIGHT + 12,
                release_manifest_root: String::new(),
                circuit_breaker_clearance_root: String::new(),
                previous_final_go_no_go_root: String::new(),
                receipts: BTreeMap::new(),
                operator_acknowledgements: BTreeMap::new(),
                governance_attestations: BTreeMap::new(),
            },
        };
        state.release_manifest_root = generic_sample_root("release-manifest-import-root");
        state.circuit_breaker_clearance_root = generic_sample_root("circuit-breaker-clearance");
        state.previous_final_go_no_go_root = generic_sample_root("wave-80-final-go-no-go-root");
        for (offset, lane) in EvidenceLane::all().into_iter().enumerate() {
            let receipt_id = format!("devnet-{}-accepted-live-evidence", lane.as_str());
            if let Ok(receipt) = AcceptedLiveEvidenceReceipt::new(
                &state.config,
                lane,
                &receipt_id,
                lane.imported_module(),
                DEFAULT_FINALITY_HEIGHT + 8 + offset as u64,
                DEFAULT_FINALITY_HEIGHT + 2 + offset as u64,
                EvidenceStatus::Accepted,
            ) {
                let _ = state.import_receipt(receipt);
            }
        }
        let import_root = state.imported_evidence_root();
        for index in 0..state.config.min_operator_acknowledgements {
            if let Ok(ack) = OperatorAcknowledgement::new(
                &format!("operator-{index}"),
                &generic_sample_root(&format!("operator-{index}-ack")),
                &import_root,
                DEFAULT_FINALITY_HEIGHT + 20 + u64::from(index),
            ) {
                let _ = state.add_operator_acknowledgement(ack);
            }
        }
        for index in 0..state.config.min_governance_attestations {
            if let Ok(attestation) = GovernanceAttestation::new(
                &format!("governance-attestor-{index}"),
                &generic_sample_root(&format!("governance-attestor-{index}-attestation")),
                &import_root,
                DEFAULT_FINALITY_HEIGHT + 30 + u64::from(index),
            ) {
                let _ = state.add_governance_attestation(attestation);
            }
        }
        state
    }

    pub fn import_receipt(&mut self, receipt: AcceptedLiveEvidenceReceipt) -> Result<()> {
        receipt.validate(&self.config)?;
        ensure(
            self.config.required_lanes.contains(&receipt.lane),
            "receipt lane is not required by policy",
        )?;
        self.receipts.insert(receipt.lane, receipt);
        Ok(())
    }

    pub fn add_operator_acknowledgement(
        &mut self,
        acknowledgement: OperatorAcknowledgement,
    ) -> Result<()> {
        ensure_root("signed_import_root", &acknowledgement.signed_import_root)?;
        self.operator_acknowledgements
            .insert(acknowledgement.operator_id.clone(), acknowledgement);
        Ok(())
    }

    pub fn add_governance_attestation(&mut self, attestation: GovernanceAttestation) -> Result<()> {
        ensure_root("signed_import_root", &attestation.signed_import_root)?;
        self.governance_attestations
            .insert(attestation.attestor_id.clone(), attestation);
        Ok(())
    }

    pub fn bind_release_manifest_root(&mut self, root: &str) -> Result<()> {
        ensure_root("release_manifest_root", root)?;
        self.release_manifest_root = root.to_string();
        Ok(())
    }

    pub fn bind_circuit_breaker_clearance_root(&mut self, root: &str) -> Result<()> {
        ensure_root("circuit_breaker_clearance_root", root)?;
        self.circuit_breaker_clearance_root = root.to_string();
        Ok(())
    }

    pub fn bind_previous_final_go_no_go_root(&mut self, root: &str) -> Result<()> {
        ensure_root("previous_final_go_no_go_root", root)?;
        self.previous_final_go_no_go_root = root.to_string();
        Ok(())
    }

    pub fn imported_evidence_root(&self) -> String {
        map_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-IMPORT-ROOT",
            self.config.required_lanes.iter().map(|lane| {
                self.receipts
                    .get(lane)
                    .map(AcceptedLiveEvidenceReceipt::binding_root)
                    .unwrap_or_else(|| missing_lane_root(*lane))
            }),
        )
    }

    pub fn operator_acknowledgement_root(&self) -> String {
        map_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-OPERATOR-ACKS",
            self.operator_acknowledgements
                .values()
                .map(OperatorAcknowledgement::state_root),
        )
    }

    pub fn governance_attestation_root(&self) -> String {
        map_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-GOVERNANCE-ATTESTATIONS",
            self.governance_attestations
                .values()
                .map(GovernanceAttestation::state_root),
        )
    }

    pub fn readiness(&self) -> ImportReadiness {
        let mut accepted_lanes = 0usize;
        let mut fresh_lanes = 0usize;
        let mut missing_lanes = Vec::new();
        let mut stale_lanes = Vec::new();
        let mut pending_lanes = Vec::new();
        let mut rejected_lanes = Vec::new();
        for lane in &self.config.required_lanes {
            match self.receipts.get(lane) {
                Some(receipt) => {
                    if receipt.status.permits_governance_import() {
                        accepted_lanes = accepted_lanes.saturating_add(1);
                    }
                    if receipt.accepted_and_fresh_at(self.height) {
                        fresh_lanes = fresh_lanes.saturating_add(1);
                    } else if !receipt.is_fresh_at(self.height) {
                        stale_lanes.push(lane.as_str().to_string());
                    }
                    match receipt.status {
                        EvidenceStatus::PendingReview => {
                            pending_lanes.push(lane.as_str().to_string())
                        }
                        EvidenceStatus::Rejected => rejected_lanes.push(lane.as_str().to_string()),
                        EvidenceStatus::Deferred => pending_lanes.push(lane.as_str().to_string()),
                        EvidenceStatus::Missing
                        | EvidenceStatus::Accepted
                        | EvidenceStatus::Stale => {}
                    }
                }
                None => missing_lanes.push(lane.as_str().to_string()),
            }
        }
        ImportReadiness {
            required_lanes: self.config.required_lanes.len(),
            accepted_lanes,
            fresh_lanes,
            missing_lanes,
            stale_lanes,
            pending_lanes,
            rejected_lanes,
            operator_acknowledgements: self.operator_acknowledgements.len(),
            governance_attestations: self.governance_attestations.len(),
            release_manifest_bound: !self.release_manifest_root.is_empty(),
            circuit_breaker_cleared: !self.circuit_breaker_clearance_root.is_empty(),
        }
    }

    pub fn blockers(&self) -> Vec<ReleaseBlocker> {
        let mut blockers = Vec::new();
        for lane in &self.config.required_lanes {
            match self.receipts.get(lane) {
                Some(receipt) => {
                    if !receipt.is_fresh_at(self.height) {
                        blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::StaleEvidence,
                            &receipt.receipt_root(),
                            "accepted live evidence receipt is outside the freshness window",
                            self.height,
                        ));
                    }
                    match receipt.status {
                        EvidenceStatus::Missing => blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::MissingEvidence,
                            &receipt.receipt_root(),
                            "lane reports missing live evidence",
                            self.height,
                        )),
                        EvidenceStatus::Deferred => blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::DeferredEvidence,
                            &receipt.receipt_root(),
                            "lane still carries deferred evidence",
                            self.height,
                        )),
                        EvidenceStatus::PendingReview => blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::PendingReview,
                            &receipt.receipt_root(),
                            "lane evidence is pending reviewer acceptance",
                            self.height,
                        )),
                        EvidenceStatus::Rejected => blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::RejectedEvidence,
                            &receipt.receipt_root(),
                            "lane evidence was rejected",
                            self.height,
                        )),
                        EvidenceStatus::Stale => blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::StaleEvidence,
                            &receipt.receipt_root(),
                            "lane status is stale",
                            self.height,
                        )),
                        EvidenceStatus::Accepted => {}
                    }
                    if self.config.require_privacy_boundaries
                        && lane.requires_privacy_boundary()
                        && receipt.privacy_boundary_root.is_empty()
                    {
                        blockers.push(ReleaseBlocker::new(
                            lane.as_str(),
                            BlockerKind::PrivacyBoundaryMissing,
                            &receipt.receipt_root(),
                            "lane requires a privacy boundary root before final import",
                            self.height,
                        ));
                    }
                }
                None => blockers.push(ReleaseBlocker::new(
                    lane.as_str(),
                    BlockerKind::MissingEvidence,
                    &missing_lane_root(*lane),
                    "required lane has not imported an accepted live evidence receipt",
                    self.height,
                )),
            }
        }
        if self.config.require_release_manifest_root && self.release_manifest_root.is_empty() {
            blockers.push(ReleaseBlocker::new(
                "release_policy",
                BlockerKind::MissingReleaseManifest,
                &self.config.policy_root(),
                "release manifest root must be bound before final go/no-go import",
                self.height,
            ));
        }
        if self.config.require_circuit_breaker_clearance
            && self.circuit_breaker_clearance_root.is_empty()
        {
            blockers.push(ReleaseBlocker::new(
                "release_policy",
                BlockerKind::CircuitBreakerStillArmed,
                &self.config.policy_root(),
                "circuit breaker clearance root must be present before final go/no-go import",
                self.height,
            ));
        }
        if self.operator_acknowledgements.len()
            < usize::from(self.config.min_operator_acknowledgements)
        {
            blockers.push(ReleaseBlocker::new(
                "operator_acknowledgement",
                BlockerKind::MissingOperatorAcknowledgement,
                &self.imported_evidence_root(),
                "operator acknowledgement quorum is below policy",
                self.height,
            ));
        }
        if self.governance_attestations.len() < usize::from(self.config.min_governance_attestations)
        {
            blockers.push(ReleaseBlocker::new(
                "governance_attestation",
                BlockerKind::MissingGovernanceAttestation,
                &self.imported_evidence_root(),
                "governance attestation quorum is below policy",
                self.height,
            ));
        }
        blockers
    }

    pub fn blocker_root(&self) -> String {
        map_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-BLOCKERS",
            self.blockers().iter().map(ReleaseBlocker::state_root),
        )
    }

    pub fn final_verdict(&self) -> FinalGoNoGoVerdict {
        let readiness = self.readiness();
        let blocker_root = self.blocker_root();
        let imported_evidence_root = self.imported_evidence_root();
        let release_allowed = self.blockers().is_empty()
            && readiness.accepted_lanes == readiness.required_lanes
            && readiness.fresh_lanes == readiness.required_lanes
            && readiness.release_manifest_bound
            && readiness.circuit_breaker_cleared;
        let verdict_id = domain_hash(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-FINAL-VERDICT-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&imported_evidence_root),
                HashPart::Str(&blocker_root),
                HashPart::Int(self.height as i128),
            ],
            32,
        );
        FinalGoNoGoVerdict {
            verdict_id,
            release_allowed,
            import_readiness_root: readiness.readiness_root(),
            imported_evidence_root,
            blocker_root,
            operator_acknowledgement_root: self.operator_acknowledgement_root(),
            governance_attestation_root: self.governance_attestation_root(),
            release_manifest_root: self.release_manifest_root.clone(),
            circuit_breaker_clearance_root: self.circuit_breaker_clearance_root.clone(),
            previous_final_go_no_go_root: self.previous_final_go_no_go_root.clone(),
            evaluated_at_height: self.height,
        }
    }

    pub fn public_record(&self) -> Value {
        let readiness = self.readiness();
        let blockers = self.blockers();
        let verdict = self.final_verdict();
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "config": self.config.public_record(),
            "release_manifest_root": self.release_manifest_root,
            "circuit_breaker_clearance_root": self.circuit_breaker_clearance_root,
            "previous_final_go_no_go_root": self.previous_final_go_no_go_root,
            "imported_evidence_root": self.imported_evidence_root(),
            "operator_acknowledgement_root": self.operator_acknowledgement_root(),
            "governance_attestation_root": self.governance_attestation_root(),
            "blocker_root": self.blocker_root(),
            "readiness": readiness.public_record(),
            "final_verdict": verdict.public_record(),
            "receipts": self.receipts.values().map(AcceptedLiveEvidenceReceipt::public_record).collect::<Vec<_>>(),
            "operator_acknowledgements": self.operator_acknowledgements.values().map(OperatorAcknowledgement::public_record).collect::<Vec<_>>(),
            "governance_attestations": self.governance_attestations.values().map(GovernanceAttestation::public_record).collect::<Vec<_>>(),
            "blockers": blockers.iter().map(ReleaseBlocker::public_record).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-FINAL-GO-NO-GO-STATE",
            &self.public_record(),
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

pub fn final_verdict() -> FinalGoNoGoVerdict {
    devnet().final_verdict()
}

fn policy_id(label: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-POLICY-ID",
        &[HashPart::Str(CHAIN_ID), HashPart::Str(label)],
        32,
    )
}

fn blocker_id(
    lane: &str,
    kind: BlockerKind,
    evidence_root: &str,
    observed_at_height: u64,
) -> String {
    domain_hash(
        "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-BLOCKER-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane),
            HashPart::Str(kind.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Int(observed_at_height as i128),
        ],
        32,
    )
}

fn missing_lane_root(lane: EvidenceLane) -> String {
    domain_hash(
        "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-MISSING-LANE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn sample_root(lane: EvidenceLane, receipt_id: &str, label: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-DEVNET-SAMPLE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(receipt_id),
            HashPart::Str(label),
        ],
        32,
    )
}

fn generic_sample_root(label: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-ACCEPTED-LIVE-EVIDENCE-DEVNET-GENERIC",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
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
