use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqReservePrivacyAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-reserve-privacy-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const EVIDENCE_SUITE: &str =
    "monero-l2-force-exit-package-pq-reserve-privacy-accepted-live-evidence-v1";
pub const DEFAULT_CURRENT_HEIGHT: u64 = 4_287_120;
pub const DEFAULT_GOVERNANCE_HEIGHT: u64 = 4_287_168;
pub const DEFAULT_KEY_EPOCH: u64 = 81;
pub const DEFAULT_MIN_PQ_QUORUM_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_PQ_SIGNERS: u64 = 4;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_500;
pub const DEFAULT_MAX_PRIVACY_BUDGET_SPEND_BPS: u64 = 6_000;
pub const DEFAULT_MIN_NON_LINKAGE_SCORE_BPS: u64 = 9_500;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_WATCHER_CONFIRMATIONS: u64 = 3;
pub const DEFAULT_MIN_LIVE_SAMPLE_COUNT: u64 = 8;
pub const DEFAULT_MIN_RESERVE_ORACLE_COUNT: u64 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportPhase {
    Pending,
    AcceptedLiveEvidence,
    Quarantined,
    Rejected,
}

impl ImportPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::AcceptedLiveEvidence => "accepted_live_evidence",
            Self::Quarantined => "quarantined",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionKind {
    Go,
    NoGo,
}

impl GovernanceDecisionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerSeverity {
    Advisory,
    Soft,
    Hard,
    Fatal,
}

impl BlockerSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Soft => "soft",
            Self::Hard => "hard",
            Self::Fatal => "fatal",
        }
    }

    pub fn fail_closed(self) -> bool {
        matches!(self, Self::Hard | Self::Fatal)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingPqQuorum,
    PqKeyEpochMismatch,
    PqRevokedSigner,
    PqDuplicateSigner,
    PqTranscriptRootMismatch,
    ReserveCoverageShortfall,
    ReserveReceiptStale,
    ReserveOracleQuorumShortfall,
    PrivacyBudgetExceeded,
    NonLinkageInsufficient,
    LinkageComplaintAccepted,
    EvidenceRootMismatch,
    EvidenceStale,
    WatcherConfirmationsShortfall,
    LiveSampleCountShortfall,
    PackageAlreadyImported,
    PackageQuarantined,
    FinalGoNoGoMismatch,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingPqQuorum => "missing_pq_quorum",
            Self::PqKeyEpochMismatch => "pq_key_epoch_mismatch",
            Self::PqRevokedSigner => "pq_revoked_signer",
            Self::PqDuplicateSigner => "pq_duplicate_signer",
            Self::PqTranscriptRootMismatch => "pq_transcript_root_mismatch",
            Self::ReserveCoverageShortfall => "reserve_coverage_shortfall",
            Self::ReserveReceiptStale => "reserve_receipt_stale",
            Self::ReserveOracleQuorumShortfall => "reserve_oracle_quorum_shortfall",
            Self::PrivacyBudgetExceeded => "privacy_budget_exceeded",
            Self::NonLinkageInsufficient => "non_linkage_insufficient",
            Self::LinkageComplaintAccepted => "linkage_complaint_accepted",
            Self::EvidenceRootMismatch => "evidence_root_mismatch",
            Self::EvidenceStale => "evidence_stale",
            Self::WatcherConfirmationsShortfall => "watcher_confirmations_shortfall",
            Self::LiveSampleCountShortfall => "live_sample_count_shortfall",
            Self::PackageAlreadyImported => "package_already_imported",
            Self::PackageQuarantined => "package_quarantined",
            Self::FinalGoNoGoMismatch => "final_go_no_go_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Accepted,
    Pending,
    Rejected,
    Expired,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub evidence_suite: String,
    pub current_height: u64,
    pub governance_height: u64,
    pub current_key_epoch: u64,
    pub min_pq_quorum_weight: u64,
    pub min_pq_signers: u64,
    pub min_reserve_coverage_bps: u64,
    pub max_privacy_budget_spend_bps: u64,
    pub min_non_linkage_score_bps: u64,
    pub max_evidence_age_blocks: u64,
    pub min_watcher_confirmations: u64,
    pub min_live_sample_count: u64,
    pub min_reserve_oracle_count: u64,
    pub fail_closed_on_any_blocker: bool,
    pub fail_closed_on_quarantine: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            evidence_suite: EVIDENCE_SUITE.to_string(),
            current_height: DEFAULT_CURRENT_HEIGHT,
            governance_height: DEFAULT_GOVERNANCE_HEIGHT,
            current_key_epoch: DEFAULT_KEY_EPOCH,
            min_pq_quorum_weight: DEFAULT_MIN_PQ_QUORUM_WEIGHT,
            min_pq_signers: DEFAULT_MIN_PQ_SIGNERS,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            max_privacy_budget_spend_bps: DEFAULT_MAX_PRIVACY_BUDGET_SPEND_BPS,
            min_non_linkage_score_bps: DEFAULT_MIN_NON_LINKAGE_SCORE_BPS,
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            min_watcher_confirmations: DEFAULT_MIN_WATCHER_CONFIRMATIONS,
            min_live_sample_count: DEFAULT_MIN_LIVE_SAMPLE_COUNT,
            min_reserve_oracle_count: DEFAULT_MIN_RESERVE_ORACLE_COUNT,
            fail_closed_on_any_blocker: true,
            fail_closed_on_quarantine: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "evidence_suite": self.evidence_suite,
            "current_height": self.current_height,
            "governance_height": self.governance_height,
            "current_key_epoch": self.current_key_epoch,
            "min_pq_quorum_weight": self.min_pq_quorum_weight,
            "min_pq_signers": self.min_pq_signers,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "max_privacy_budget_spend_bps": self.max_privacy_budget_spend_bps,
            "min_non_linkage_score_bps": self.min_non_linkage_score_bps,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "min_watcher_confirmations": self.min_watcher_confirmations,
            "min_live_sample_count": self.min_live_sample_count,
            "min_reserve_oracle_count": self.min_reserve_oracle_count,
            "fail_closed_on_any_blocker": self.fail_closed_on_any_blocker,
            "fail_closed_on_quarantine": self.fail_closed_on_quarantine,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-RESERVE-PRIVACY-LIVE-EVIDENCE-CONFIG",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PqSignerAttestation {
    pub signer_id: String,
    pub key_epoch: u64,
    pub quorum_weight: u64,
    pub public_key_root: String,
    pub transcript_root: String,
    pub package_root: String,
    pub accepted_at_height: u64,
    pub revoked: bool,
}

impl PqSignerAttestation {
    pub fn public_record(&self) -> Value {
        json!({
            "signer_id": self.signer_id,
            "key_epoch": self.key_epoch,
            "quorum_weight": self.quorum_weight,
            "public_key_root": self.public_key_root,
            "transcript_root": self.transcript_root,
            "package_root": self.package_root,
            "accepted_at_height": self.accepted_at_height,
            "revoked": self.revoked,
        })
    }

    pub fn attestation_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-SIGNER-ATTESTATION",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PqQuorumEvidence {
    pub quorum_id: String,
    pub key_epoch: u64,
    pub expected_key_epoch_root: String,
    pub expected_transcript_root: String,
    pub package_root: String,
    pub signers: Vec<PqSignerAttestation>,
}

impl PqQuorumEvidence {
    pub fn public_record(&self) -> Value {
        let signer_records = self
            .signers
            .iter()
            .map(PqSignerAttestation::public_record)
            .collect::<Vec<_>>();
        json!({
            "quorum_id": self.quorum_id,
            "key_epoch": self.key_epoch,
            "expected_key_epoch_root": self.expected_key_epoch_root,
            "expected_transcript_root": self.expected_transcript_root,
            "package_root": self.package_root,
            "signer_root": merkle_root("MONERO-L2-PQ-SIGNER-ATTESTATION", &signer_records),
            "signer_count": self.signers.len() as u64,
            "accepted_weight": self.accepted_weight(),
        })
    }

    pub fn accepted_weight(&self) -> u64 {
        self.signers
            .iter()
            .filter(|signer| !signer.revoked)
            .map(|signer| signer.quorum_weight)
            .sum()
    }

    pub fn active_signer_count(&self) -> u64 {
        self.signers.iter().filter(|signer| !signer.revoked).count() as u64
    }

    pub fn signer_records(&self) -> Vec<Value> {
        self.signers
            .iter()
            .map(PqSignerAttestation::public_record)
            .collect()
    }

    pub fn key_epoch_root(&self) -> String {
        let mut records = self.signer_records();
        records.push(json!({
            "key_epoch": self.key_epoch,
            "quorum_id": self.quorum_id,
            "package_root": self.package_root,
        }));
        merkle_root("MONERO-L2-PQ-KEY-EPOCH", &records)
    }

    pub fn quorum_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-QUORUM-EVIDENCE",
            &[
                HashPart::Json(&self.public_record()),
                HashPart::Str(&self.key_epoch_root()),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReserveCoverageReceipt {
    pub receipt_id: String,
    pub oracle_id: String,
    pub package_root: String,
    pub asset: String,
    pub liability_amount: u64,
    pub reserve_amount: u64,
    pub coverage_bps: u64,
    pub observed_height: u64,
    pub receipt_status: ReceiptStatus,
    pub reserve_commitment_root: String,
}

impl ReserveCoverageReceipt {
    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "oracle_id": self.oracle_id,
            "package_root": self.package_root,
            "asset": self.asset,
            "liability_amount": self.liability_amount,
            "reserve_amount": self.reserve_amount,
            "coverage_bps": self.coverage_bps,
            "observed_height": self.observed_height,
            "receipt_status": self.receipt_status.as_str(),
            "reserve_commitment_root": self.reserve_commitment_root,
        })
    }

    pub fn receipt_root(&self) -> String {
        domain_hash(
            "MONERO-L2-RESERVE-COVERAGE-RECEIPT",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    pub fn age_at(&self, height: u64) -> u64 {
        height.saturating_sub(self.observed_height)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReserveCoverageEvidence {
    pub reserve_set_id: String,
    pub package_root: String,
    pub receipts: Vec<ReserveCoverageReceipt>,
}

impl ReserveCoverageEvidence {
    pub fn public_record(&self) -> Value {
        let receipt_records = self.receipt_records();
        json!({
            "reserve_set_id": self.reserve_set_id,
            "package_root": self.package_root,
            "receipt_root": merkle_root("MONERO-L2-RESERVE-COVERAGE-RECEIPT", &receipt_records),
            "receipt_count": self.receipts.len() as u64,
            "accepted_receipt_count": self.accepted_receipt_count(),
            "min_coverage_bps": self.min_accepted_coverage_bps(),
            "weighted_coverage_bps": self.weighted_coverage_bps(),
        })
    }

    pub fn receipt_records(&self) -> Vec<Value> {
        self.receipts
            .iter()
            .map(ReserveCoverageReceipt::public_record)
            .collect()
    }

    pub fn accepted_receipt_count(&self) -> u64 {
        self.receipts
            .iter()
            .filter(|receipt| receipt.receipt_status == ReceiptStatus::Accepted)
            .count() as u64
    }

    pub fn min_accepted_coverage_bps(&self) -> u64 {
        self.receipts
            .iter()
            .filter(|receipt| receipt.receipt_status == ReceiptStatus::Accepted)
            .map(|receipt| receipt.coverage_bps)
            .min()
            .unwrap_or(0)
    }

    pub fn weighted_coverage_bps(&self) -> u64 {
        let mut total_liability = 0_u64;
        let mut total_reserve = 0_u64;
        for receipt in self
            .receipts
            .iter()
            .filter(|receipt| receipt.receipt_status == ReceiptStatus::Accepted)
        {
            total_liability = total_liability.saturating_add(receipt.liability_amount);
            total_reserve = total_reserve.saturating_add(receipt.reserve_amount);
        }
        if total_liability == 0 {
            return 0;
        }
        total_reserve.saturating_mul(10_000) / total_liability
    }

    pub fn reserve_root(&self) -> String {
        domain_hash(
            "MONERO-L2-RESERVE-COVERAGE-EVIDENCE",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivacyBudgetEvidence {
    pub budget_id: String,
    pub package_root: String,
    pub nullifier_set_root: String,
    pub unlinkability_set_root: String,
    pub budget_limit_bps: u64,
    pub budget_spent_bps: u64,
    pub non_linkage_score_bps: u64,
    pub live_sample_count: u64,
    pub linkage_complaints: u64,
    pub accepted_linkage_complaints: u64,
}

impl PrivacyBudgetEvidence {
    pub fn public_record(&self) -> Value {
        json!({
            "budget_id": self.budget_id,
            "package_root": self.package_root,
            "nullifier_set_root": self.nullifier_set_root,
            "unlinkability_set_root": self.unlinkability_set_root,
            "budget_limit_bps": self.budget_limit_bps,
            "budget_spent_bps": self.budget_spent_bps,
            "non_linkage_score_bps": self.non_linkage_score_bps,
            "live_sample_count": self.live_sample_count,
            "linkage_complaints": self.linkage_complaints,
            "accepted_linkage_complaints": self.accepted_linkage_complaints,
        })
    }

    pub fn privacy_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PRIVACY-BUDGET-EVIDENCE",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    pub fn budget_remaining_bps(&self) -> u64 {
        self.budget_limit_bps.saturating_sub(self.budget_spent_bps)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveEvidenceWitness {
    pub witness_id: String,
    pub watcher_id: String,
    pub package_root: String,
    pub observed_height: u64,
    pub force_exit_queue_root: String,
    pub canonical_answer_root: String,
    pub import_payload_root: String,
    pub watcher_confirmations: u64,
    pub accepted: bool,
}

impl LiveEvidenceWitness {
    pub fn public_record(&self) -> Value {
        json!({
            "witness_id": self.witness_id,
            "watcher_id": self.watcher_id,
            "package_root": self.package_root,
            "observed_height": self.observed_height,
            "force_exit_queue_root": self.force_exit_queue_root,
            "canonical_answer_root": self.canonical_answer_root,
            "import_payload_root": self.import_payload_root,
            "watcher_confirmations": self.watcher_confirmations,
            "accepted": self.accepted,
        })
    }

    pub fn witness_root(&self) -> String {
        domain_hash(
            "MONERO-L2-LIVE-EVIDENCE-WITNESS",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EvidencePackage {
    pub package_id: String,
    pub user_escape_id: String,
    pub force_exit_id: String,
    pub answer_id: String,
    pub l2_account_commitment: String,
    pub monero_destination_commitment: String,
    pub created_height: u64,
    pub expected_evidence_root: String,
    pub pq_quorum: PqQuorumEvidence,
    pub reserve_coverage: ReserveCoverageEvidence,
    pub privacy_budget: PrivacyBudgetEvidence,
    pub live_witnesses: Vec<LiveEvidenceWitness>,
}

impl EvidencePackage {
    pub fn public_record(&self) -> Value {
        let witness_records = self.live_witness_records();
        json!({
            "package_id": self.package_id,
            "user_escape_id": self.user_escape_id,
            "force_exit_id": self.force_exit_id,
            "answer_id": self.answer_id,
            "l2_account_commitment": self.l2_account_commitment,
            "monero_destination_commitment": self.monero_destination_commitment,
            "created_height": self.created_height,
            "expected_evidence_root": self.expected_evidence_root,
            "pq_quorum_root": self.pq_quorum.quorum_root(),
            "pq_key_epoch_root": self.pq_quorum.key_epoch_root(),
            "reserve_coverage_root": self.reserve_coverage.reserve_root(),
            "privacy_budget_root": self.privacy_budget.privacy_root(),
            "live_witness_root": merkle_root("MONERO-L2-LIVE-EVIDENCE-WITNESS", &witness_records),
            "accepted_live_witness_count": self.accepted_live_witness_count(),
        })
    }

    pub fn live_witness_records(&self) -> Vec<Value> {
        self.live_witnesses
            .iter()
            .map(LiveEvidenceWitness::public_record)
            .collect()
    }

    pub fn accepted_live_witness_count(&self) -> u64 {
        self.live_witnesses
            .iter()
            .filter(|witness| witness.accepted)
            .count() as u64
    }

    pub fn max_watcher_confirmations(&self) -> u64 {
        self.live_witnesses
            .iter()
            .filter(|witness| witness.accepted)
            .map(|witness| witness.watcher_confirmations)
            .max()
            .unwrap_or(0)
    }

    pub fn observed_live_height(&self) -> u64 {
        self.live_witnesses
            .iter()
            .filter(|witness| witness.accepted)
            .map(|witness| witness.observed_height)
            .max()
            .unwrap_or(self.created_height)
    }

    pub fn computed_evidence_root(&self) -> String {
        let record = json!({
            "package_id": self.package_id,
            "user_escape_id": self.user_escape_id,
            "force_exit_id": self.force_exit_id,
            "answer_id": self.answer_id,
            "pq_quorum_root": self.pq_quorum.quorum_root(),
            "pq_key_epoch_root": self.pq_quorum.key_epoch_root(),
            "reserve_coverage_root": self.reserve_coverage.reserve_root(),
            "privacy_budget_root": self.privacy_budget.privacy_root(),
            "live_witness_root": merkle_root(
                "MONERO-L2-LIVE-EVIDENCE-WITNESS",
                &self.live_witness_records()
            ),
        });
        domain_hash(
            "MONERO-L2-ACCEPTED-LIVE-EVIDENCE-PACKAGE",
            &[HashPart::Json(&record)],
            32,
        )
    }

    pub fn package_root(&self) -> String {
        domain_hash(
            "MONERO-L2-FORCE-EXIT-PACKAGE",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    pub fn age_at(&self, height: u64) -> u64 {
        height.saturating_sub(self.created_height)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportBlocker {
    pub blocker_id: String,
    pub package_id: String,
    pub kind: BlockerKind,
    pub severity: BlockerSeverity,
    pub observed_value: String,
    pub required_value: String,
    pub evidence_root: String,
}

impl ImportBlocker {
    pub fn new(
        package_id: &str,
        kind: BlockerKind,
        severity: BlockerSeverity,
        observed_value: impl Into<String>,
        required_value: impl Into<String>,
        evidence_root: impl Into<String>,
    ) -> Self {
        let observed_value = observed_value.into();
        let required_value = required_value.into();
        let evidence_root = evidence_root.into();
        let blocker_id = domain_hash(
            "MONERO-L2-LIVE-EVIDENCE-IMPORT-BLOCKER-ID",
            &[
                HashPart::Str(package_id),
                HashPart::Str(kind.as_str()),
                HashPart::Str(&observed_value),
                HashPart::Str(&required_value),
                HashPart::Str(&evidence_root),
            ],
            32,
        );
        Self {
            blocker_id,
            package_id: package_id.to_string(),
            kind,
            severity,
            observed_value,
            required_value,
            evidence_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "package_id": self.package_id,
            "kind": self.kind.as_str(),
            "severity": self.severity.as_str(),
            "observed_value": self.observed_value,
            "required_value": self.required_value,
            "evidence_root": self.evidence_root,
            "fail_closed": self.severity.fail_closed(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportDecision {
    pub decision_id: String,
    pub package_id: String,
    pub phase: ImportPhase,
    pub governance_decision: GovernanceDecisionKind,
    pub imported_at_height: u64,
    pub evidence_root: String,
    pub package_root: String,
    pub blocker_root: String,
    pub blocker_count: u64,
}

impl ImportDecision {
    pub fn public_record(&self) -> Value {
        json!({
            "decision_id": self.decision_id,
            "package_id": self.package_id,
            "phase": self.phase.as_str(),
            "governance_decision": self.governance_decision.as_str(),
            "imported_at_height": self.imported_at_height,
            "evidence_root": self.evidence_root,
            "package_root": self.package_root,
            "blocker_root": self.blocker_root,
            "blocker_count": self.blocker_count,
        })
    }

    pub fn decision_root(&self) -> String {
        domain_hash(
            "MONERO-L2-LIVE-EVIDENCE-IMPORT-DECISION",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RuntimeCounters {
    pub import_attempts: u64,
    pub accepted_imports: u64,
    pub rejected_imports: u64,
    pub quarantined_imports: u64,
    pub fail_closed_blockers: u64,
}

impl RuntimeCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "import_attempts": self.import_attempts,
            "accepted_imports": self.accepted_imports,
            "rejected_imports": self.rejected_imports,
            "quarantined_imports": self.quarantined_imports,
            "fail_closed_blockers": self.fail_closed_blockers,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuntimeRoots {
    pub config_root: String,
    pub package_root: String,
    pub pq_quorum_root: String,
    pub pq_key_epoch_root: String,
    pub reserve_coverage_root: String,
    pub privacy_budget_root: String,
    pub live_witness_root: String,
    pub blocker_root: String,
    pub decision_root: String,
    pub governance_root: String,
}

impl RuntimeRoots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "package_root": self.package_root,
            "pq_quorum_root": self.pq_quorum_root,
            "pq_key_epoch_root": self.pq_key_epoch_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_budget_root": self.privacy_budget_root,
            "live_witness_root": self.live_witness_root,
            "blocker_root": self.blocker_root,
            "decision_root": self.decision_root,
            "governance_root": self.governance_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GovernanceEnvelope {
    pub envelope_id: String,
    pub package_id: String,
    pub final_decision: GovernanceDecisionKind,
    pub evidence_root: String,
    pub blocker_root: String,
    pub pq_key_epoch_root: String,
    pub reserve_coverage_root: String,
    pub privacy_budget_root: String,
    pub emitted_height: u64,
}

impl GovernanceEnvelope {
    pub fn public_record(&self) -> Value {
        json!({
            "envelope_id": self.envelope_id,
            "package_id": self.package_id,
            "final_decision": self.final_decision.as_str(),
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "pq_key_epoch_root": self.pq_key_epoch_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_budget_root": self.privacy_budget_root,
            "emitted_height": self.emitted_height,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub packages: BTreeMap<String, EvidencePackage>,
    pub imported_package_ids: BTreeSet<String>,
    pub blockers: Vec<ImportBlocker>,
    pub decisions: Vec<ImportDecision>,
    pub governance_envelopes: Vec<GovernanceEnvelope>,
    pub counters: RuntimeCounters,
}

impl State {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            packages: BTreeMap::new(),
            imported_package_ids: BTreeSet::new(),
            blockers: Vec::new(),
            decisions: Vec::new(),
            governance_envelopes: Vec::new(),
            counters: RuntimeCounters::default(),
        }
    }

    pub fn devnet() -> Self {
        let mut state = Self::new(Config::devnet());
        let package = devnet_package();
        let _ = state.import_accepted_live_evidence(package);
        state
    }

    pub fn import_accepted_live_evidence(
        &mut self,
        package: EvidencePackage,
    ) -> Result<ImportDecision> {
        self.counters.import_attempts = self.counters.import_attempts.saturating_add(1);
        let mut blockers = self.evaluate_package(&package);
        if self.imported_package_ids.contains(&package.package_id) {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::PackageAlreadyImported,
                BlockerSeverity::Hard,
                "already_imported",
                "not_imported",
                package.computed_evidence_root(),
            ));
        }

        let blocker_records = blockers
            .iter()
            .map(ImportBlocker::public_record)
            .collect::<Vec<_>>();
        let blocker_root = merkle_root("MONERO-L2-LIVE-EVIDENCE-IMPORT-BLOCKER", &blocker_records);
        let evidence_root = package.computed_evidence_root();
        let package_root = package.package_root();
        let has_fail_closed = blockers
            .iter()
            .any(|blocker| blocker.severity.fail_closed())
            && self.config.fail_closed_on_any_blocker;
        let phase = if has_fail_closed {
            ImportPhase::Rejected
        } else if blockers.is_empty() {
            ImportPhase::AcceptedLiveEvidence
        } else {
            ImportPhase::Quarantined
        };
        let governance_decision = if phase == ImportPhase::AcceptedLiveEvidence {
            GovernanceDecisionKind::Go
        } else {
            GovernanceDecisionKind::NoGo
        };
        let decision_id = domain_hash(
            "MONERO-L2-LIVE-EVIDENCE-IMPORT-DECISION-ID",
            &[
                HashPart::Str(&package.package_id),
                HashPart::Str(phase.as_str()),
                HashPart::Str(governance_decision.as_str()),
                HashPart::Str(&evidence_root),
                HashPart::Str(&blocker_root),
            ],
            32,
        );
        let decision = ImportDecision {
            decision_id,
            package_id: package.package_id.clone(),
            phase,
            governance_decision,
            imported_at_height: self.config.current_height,
            evidence_root: evidence_root.clone(),
            package_root: package_root.clone(),
            blocker_root: blocker_root.clone(),
            blocker_count: blockers.len() as u64,
        };
        let envelope = GovernanceEnvelope {
            envelope_id: domain_hash(
                "MONERO-L2-FINAL-GO-NO-GO-GOVERNANCE-ENVELOPE-ID",
                &[
                    HashPart::Str(&package.package_id),
                    HashPart::Str(governance_decision.as_str()),
                    HashPart::Str(&decision.decision_root()),
                ],
                32,
            ),
            package_id: package.package_id.clone(),
            final_decision: governance_decision,
            evidence_root,
            blocker_root,
            pq_key_epoch_root: package.pq_quorum.key_epoch_root(),
            reserve_coverage_root: package.reserve_coverage.reserve_root(),
            privacy_budget_root: package.privacy_budget.privacy_root(),
            emitted_height: self.config.governance_height,
        };

        if has_fail_closed {
            self.counters.rejected_imports = self.counters.rejected_imports.saturating_add(1);
        } else if phase == ImportPhase::AcceptedLiveEvidence {
            self.counters.accepted_imports = self.counters.accepted_imports.saturating_add(1);
            self.imported_package_ids.insert(package.package_id.clone());
        } else {
            self.counters.quarantined_imports = self.counters.quarantined_imports.saturating_add(1);
        }
        self.counters.fail_closed_blockers = self.counters.fail_closed_blockers.saturating_add(
            blockers
                .iter()
                .filter(|blocker| blocker.severity.fail_closed())
                .count() as u64,
        );

        self.blockers.extend(blockers);
        self.governance_envelopes.push(envelope);
        self.decisions.push(decision.clone());
        self.packages.insert(package.package_id.clone(), package);
        Ok(decision)
    }

    pub fn evaluate_package(&self, package: &EvidencePackage) -> Vec<ImportBlocker> {
        let mut blockers = Vec::new();
        self.evaluate_evidence_roots(package, &mut blockers);
        self.evaluate_pq_quorum(package, &mut blockers);
        self.evaluate_reserve_coverage(package, &mut blockers);
        self.evaluate_privacy_budget(package, &mut blockers);
        self.evaluate_live_witnesses(package, &mut blockers);
        blockers
    }

    fn evaluate_evidence_roots(
        &self,
        package: &EvidencePackage,
        blockers: &mut Vec<ImportBlocker>,
    ) {
        let computed = package.computed_evidence_root();
        if package.expected_evidence_root != computed {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::EvidenceRootMismatch,
                BlockerSeverity::Fatal,
                package.expected_evidence_root.clone(),
                computed,
                package.package_root(),
            ));
        }
        let age = package.age_at(self.config.current_height);
        if age > self.config.max_evidence_age_blocks {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::EvidenceStale,
                BlockerSeverity::Hard,
                age.to_string(),
                self.config.max_evidence_age_blocks.to_string(),
                package.computed_evidence_root(),
            ));
        }
    }

    fn evaluate_pq_quorum(&self, package: &EvidencePackage, blockers: &mut Vec<ImportBlocker>) {
        let pq = &package.pq_quorum;
        if pq.key_epoch != self.config.current_key_epoch {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::PqKeyEpochMismatch,
                BlockerSeverity::Hard,
                pq.key_epoch.to_string(),
                self.config.current_key_epoch.to_string(),
                pq.quorum_root(),
            ));
        }
        let key_epoch_root = pq.key_epoch_root();
        if pq.expected_key_epoch_root != key_epoch_root {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::PqKeyEpochMismatch,
                BlockerSeverity::Fatal,
                pq.expected_key_epoch_root.clone(),
                key_epoch_root,
                pq.quorum_root(),
            ));
        }
        if pq.accepted_weight() < self.config.min_pq_quorum_weight {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::MissingPqQuorum,
                BlockerSeverity::Hard,
                pq.accepted_weight().to_string(),
                self.config.min_pq_quorum_weight.to_string(),
                pq.quorum_root(),
            ));
        }
        if pq.active_signer_count() < self.config.min_pq_signers {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::MissingPqQuorum,
                BlockerSeverity::Hard,
                pq.active_signer_count().to_string(),
                self.config.min_pq_signers.to_string(),
                pq.quorum_root(),
            ));
        }
        let mut signer_ids = BTreeSet::new();
        for signer in &pq.signers {
            if signer.package_root != pq.package_root {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::PqTranscriptRootMismatch,
                    BlockerSeverity::Fatal,
                    signer.package_root.clone(),
                    pq.package_root.clone(),
                    signer.attestation_root(),
                ));
            }
            if signer.transcript_root != pq.expected_transcript_root {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::PqTranscriptRootMismatch,
                    BlockerSeverity::Fatal,
                    signer.transcript_root.clone(),
                    pq.expected_transcript_root.clone(),
                    signer.attestation_root(),
                ));
            }
            if signer.key_epoch != pq.key_epoch {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::PqKeyEpochMismatch,
                    BlockerSeverity::Hard,
                    signer.key_epoch.to_string(),
                    pq.key_epoch.to_string(),
                    signer.attestation_root(),
                ));
            }
            if signer.revoked {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::PqRevokedSigner,
                    BlockerSeverity::Hard,
                    signer.signer_id.clone(),
                    "active_signer",
                    signer.attestation_root(),
                ));
            }
            if !signer_ids.insert(signer.signer_id.clone()) {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::PqDuplicateSigner,
                    BlockerSeverity::Fatal,
                    signer.signer_id.clone(),
                    "unique_signer",
                    signer.attestation_root(),
                ));
            }
        }
    }

    fn evaluate_reserve_coverage(
        &self,
        package: &EvidencePackage,
        blockers: &mut Vec<ImportBlocker>,
    ) {
        let reserve = &package.reserve_coverage;
        if reserve.accepted_receipt_count() < self.config.min_reserve_oracle_count {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::ReserveOracleQuorumShortfall,
                BlockerSeverity::Hard,
                reserve.accepted_receipt_count().to_string(),
                self.config.min_reserve_oracle_count.to_string(),
                reserve.reserve_root(),
            ));
        }
        if reserve.weighted_coverage_bps() < self.config.min_reserve_coverage_bps {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::ReserveCoverageShortfall,
                BlockerSeverity::Hard,
                reserve.weighted_coverage_bps().to_string(),
                self.config.min_reserve_coverage_bps.to_string(),
                reserve.reserve_root(),
            ));
        }
        for receipt in &reserve.receipts {
            if receipt.package_root != reserve.package_root {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::EvidenceRootMismatch,
                    BlockerSeverity::Fatal,
                    receipt.package_root.clone(),
                    reserve.package_root.clone(),
                    receipt.receipt_root(),
                ));
            }
            if receipt.receipt_status == ReceiptStatus::Expired
                || receipt.age_at(self.config.current_height) > self.config.max_evidence_age_blocks
            {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::ReserveReceiptStale,
                    BlockerSeverity::Hard,
                    receipt.age_at(self.config.current_height).to_string(),
                    self.config.max_evidence_age_blocks.to_string(),
                    receipt.receipt_root(),
                ));
            }
        }
    }

    fn evaluate_privacy_budget(
        &self,
        package: &EvidencePackage,
        blockers: &mut Vec<ImportBlocker>,
    ) {
        let privacy = &package.privacy_budget;
        if privacy.budget_spent_bps > self.config.max_privacy_budget_spend_bps {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::PrivacyBudgetExceeded,
                BlockerSeverity::Hard,
                privacy.budget_spent_bps.to_string(),
                self.config.max_privacy_budget_spend_bps.to_string(),
                privacy.privacy_root(),
            ));
        }
        if privacy.non_linkage_score_bps < self.config.min_non_linkage_score_bps {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::NonLinkageInsufficient,
                BlockerSeverity::Hard,
                privacy.non_linkage_score_bps.to_string(),
                self.config.min_non_linkage_score_bps.to_string(),
                privacy.privacy_root(),
            ));
        }
        if privacy.accepted_linkage_complaints > 0 {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::LinkageComplaintAccepted,
                BlockerSeverity::Fatal,
                privacy.accepted_linkage_complaints.to_string(),
                "0",
                privacy.privacy_root(),
            ));
        }
        if privacy.live_sample_count < self.config.min_live_sample_count {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::LiveSampleCountShortfall,
                BlockerSeverity::Hard,
                privacy.live_sample_count.to_string(),
                self.config.min_live_sample_count.to_string(),
                privacy.privacy_root(),
            ));
        }
    }

    fn evaluate_live_witnesses(
        &self,
        package: &EvidencePackage,
        blockers: &mut Vec<ImportBlocker>,
    ) {
        if package.accepted_live_witness_count() < self.config.min_live_sample_count {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::LiveSampleCountShortfall,
                BlockerSeverity::Hard,
                package.accepted_live_witness_count().to_string(),
                self.config.min_live_sample_count.to_string(),
                package.computed_evidence_root(),
            ));
        }
        if package.max_watcher_confirmations() < self.config.min_watcher_confirmations {
            blockers.push(ImportBlocker::new(
                &package.package_id,
                BlockerKind::WatcherConfirmationsShortfall,
                BlockerSeverity::Hard,
                package.max_watcher_confirmations().to_string(),
                self.config.min_watcher_confirmations.to_string(),
                package.computed_evidence_root(),
            ));
        }
        for witness in &package.live_witnesses {
            if witness.package_root != package.pq_quorum.package_root {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::EvidenceRootMismatch,
                    BlockerSeverity::Fatal,
                    witness.package_root.clone(),
                    package.pq_quorum.package_root.clone(),
                    witness.witness_root(),
                ));
            }
            if self
                .config
                .current_height
                .saturating_sub(witness.observed_height)
                > self.config.max_evidence_age_blocks
            {
                blockers.push(ImportBlocker::new(
                    &package.package_id,
                    BlockerKind::EvidenceStale,
                    BlockerSeverity::Hard,
                    self.config
                        .current_height
                        .saturating_sub(witness.observed_height)
                        .to_string(),
                    self.config.max_evidence_age_blocks.to_string(),
                    witness.witness_root(),
                ));
            }
        }
    }

    pub fn roots(&self) -> RuntimeRoots {
        let package_records = self
            .packages
            .values()
            .map(EvidencePackage::public_record)
            .collect::<Vec<_>>();
        let blocker_records = self
            .blockers
            .iter()
            .map(ImportBlocker::public_record)
            .collect::<Vec<_>>();
        let decision_records = self
            .decisions
            .iter()
            .map(ImportDecision::public_record)
            .collect::<Vec<_>>();
        let governance_records = self
            .governance_envelopes
            .iter()
            .map(GovernanceEnvelope::public_record)
            .collect::<Vec<_>>();
        let pq_records = self
            .packages
            .values()
            .map(|package| package.pq_quorum.public_record())
            .collect::<Vec<_>>();
        let key_epoch_records = self
            .packages
            .values()
            .map(|package| {
                json!({
                    "package_id": package.package_id,
                    "key_epoch_root": package.pq_quorum.key_epoch_root(),
                })
            })
            .collect::<Vec<_>>();
        let reserve_records = self
            .packages
            .values()
            .map(|package| package.reserve_coverage.public_record())
            .collect::<Vec<_>>();
        let privacy_records = self
            .packages
            .values()
            .map(|package| package.privacy_budget.public_record())
            .collect::<Vec<_>>();
        let live_witness_records = self
            .packages
            .values()
            .flat_map(EvidencePackage::live_witness_records)
            .collect::<Vec<_>>();
        RuntimeRoots {
            config_root: self.config.state_root(),
            package_root: merkle_root("MONERO-L2-FORCE-EXIT-PACKAGE-STATE", &package_records),
            pq_quorum_root: merkle_root("MONERO-L2-PQ-QUORUM-STATE", &pq_records),
            pq_key_epoch_root: merkle_root("MONERO-L2-PQ-KEY-EPOCH-STATE", &key_epoch_records),
            reserve_coverage_root: merkle_root(
                "MONERO-L2-RESERVE-COVERAGE-STATE",
                &reserve_records,
            ),
            privacy_budget_root: merkle_root("MONERO-L2-PRIVACY-BUDGET-STATE", &privacy_records),
            live_witness_root: merkle_root("MONERO-L2-LIVE-WITNESS-STATE", &live_witness_records),
            blocker_root: merkle_root("MONERO-L2-LIVE-EVIDENCE-IMPORT-BLOCKER", &blocker_records),
            decision_root: merkle_root(
                "MONERO-L2-LIVE-EVIDENCE-IMPORT-DECISION",
                &decision_records,
            ),
            governance_root: merkle_root(
                "MONERO-L2-FINAL-GO-NO-GO-GOVERNANCE",
                &governance_records,
            ),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "roots": self.roots().public_record(),
            "counters": self.counters.public_record(),
            "package_count": self.packages.len() as u64,
            "blocker_count": self.blockers.len() as u64,
            "decision_count": self.decisions.len() as u64,
            "governance_envelope_count": self.governance_envelopes.len() as u64,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-RESERVE-PRIVACY-LIVE-EVIDENCE-IMPORT-RUNTIME-STATE",
            &[HashPart::Json(&self.public_record())],
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

fn devnet_package() -> EvidencePackage {
    let package_id = "force-exit-package-wave-81-user-escape-006".to_string();
    let user_escape_id = "user-escape-answer-wave-81-006".to_string();
    let force_exit_id = "force-exit-canonical-006".to_string();
    let answer_id = "canonical-answer-accepted-live-evidence-006".to_string();
    let package_seed_root = domain_hash(
        "MONERO-L2-DEVNET-FORCE-EXIT-PACKAGE-SEED",
        &[
            HashPart::Str(&package_id),
            HashPart::Str(&user_escape_id),
            HashPart::Str(&force_exit_id),
            HashPart::Str(&answer_id),
        ],
        32,
    );
    let transcript_root = domain_hash(
        "MONERO-L2-DEVNET-PQ-TRANSCRIPT",
        &[HashPart::Str(&package_seed_root)],
        32,
    );
    let signers = (0..5_u64)
        .map(|index| PqSignerAttestation {
            signer_id: format!("pq-governance-signer-{index:02}"),
            key_epoch: DEFAULT_KEY_EPOCH,
            quorum_weight: 15,
            public_key_root: domain_hash(
                "MONERO-L2-DEVNET-PQ-PUBLIC-KEY",
                &[HashPart::Str(&package_seed_root), HashPart::U64(index)],
                32,
            ),
            transcript_root: transcript_root.clone(),
            package_root: package_seed_root.clone(),
            accepted_at_height: DEFAULT_CURRENT_HEIGHT.saturating_sub(4),
            revoked: false,
        })
        .collect::<Vec<_>>();
    let mut pq = PqQuorumEvidence {
        quorum_id: "pq-quorum-wave-81-governance".to_string(),
        key_epoch: DEFAULT_KEY_EPOCH,
        expected_key_epoch_root: String::new(),
        expected_transcript_root: transcript_root,
        package_root: package_seed_root.clone(),
        signers,
    };
    pq.expected_key_epoch_root = pq.key_epoch_root();
    let reserve_receipts = (0..3_u64)
        .map(|index| ReserveCoverageReceipt {
            receipt_id: format!("reserve-coverage-receipt-{index:02}"),
            oracle_id: format!("reserve-oracle-{index:02}"),
            package_root: package_seed_root.clone(),
            asset: "xmr-reserve-note".to_string(),
            liability_amount: 1_000_000,
            reserve_amount: 1_080_000 + index.saturating_mul(5_000),
            coverage_bps: 10_800 + index.saturating_mul(50),
            observed_height: DEFAULT_CURRENT_HEIGHT.saturating_sub(8 + index),
            receipt_status: ReceiptStatus::Accepted,
            reserve_commitment_root: domain_hash(
                "MONERO-L2-DEVNET-RESERVE-COMMITMENT",
                &[HashPart::Str(&package_seed_root), HashPart::U64(index)],
                32,
            ),
        })
        .collect::<Vec<_>>();
    let reserve_coverage = ReserveCoverageEvidence {
        reserve_set_id: "reserve-set-wave-81-force-exit".to_string(),
        package_root: package_seed_root.clone(),
        receipts: reserve_receipts,
    };
    let privacy_budget = PrivacyBudgetEvidence {
        budget_id: "privacy-budget-wave-81-force-exit".to_string(),
        package_root: package_seed_root.clone(),
        nullifier_set_root: domain_hash(
            "MONERO-L2-DEVNET-NULLIFIER-SET",
            &[HashPart::Str(&package_seed_root)],
            32,
        ),
        unlinkability_set_root: domain_hash(
            "MONERO-L2-DEVNET-UNLINKABILITY-SET",
            &[HashPart::Str(&package_seed_root)],
            32,
        ),
        budget_limit_bps: 10_000,
        budget_spent_bps: 4_100,
        non_linkage_score_bps: 9_820,
        live_sample_count: DEFAULT_MIN_LIVE_SAMPLE_COUNT,
        linkage_complaints: 0,
        accepted_linkage_complaints: 0,
    };
    let live_witnesses = (0..DEFAULT_MIN_LIVE_SAMPLE_COUNT)
        .map(|index| LiveEvidenceWitness {
            witness_id: format!("live-evidence-witness-{index:02}"),
            watcher_id: format!("exit-watcher-{index:02}"),
            package_root: package_seed_root.clone(),
            observed_height: DEFAULT_CURRENT_HEIGHT.saturating_sub(3 + index),
            force_exit_queue_root: domain_hash(
                "MONERO-L2-DEVNET-FORCE-EXIT-QUEUE",
                &[HashPart::Str(&package_seed_root), HashPart::U64(index)],
                32,
            ),
            canonical_answer_root: domain_hash(
                "MONERO-L2-DEVNET-CANONICAL-ANSWER",
                &[HashPart::Str(&package_seed_root), HashPart::U64(index)],
                32,
            ),
            import_payload_root: domain_hash(
                "MONERO-L2-DEVNET-IMPORT-PAYLOAD",
                &[HashPart::Str(&package_seed_root), HashPart::U64(index)],
                32,
            ),
            watcher_confirmations: DEFAULT_MIN_WATCHER_CONFIRMATIONS + 1,
            accepted: true,
        })
        .collect::<Vec<_>>();
    let mut package = EvidencePackage {
        package_id,
        user_escape_id,
        force_exit_id,
        answer_id,
        l2_account_commitment: domain_hash(
            "MONERO-L2-DEVNET-L2-ACCOUNT-COMMITMENT",
            &[HashPart::Str(&package_seed_root)],
            32,
        ),
        monero_destination_commitment: domain_hash(
            "MONERO-L2-DEVNET-MONERO-DESTINATION-COMMITMENT",
            &[HashPart::Str(&package_seed_root)],
            32,
        ),
        created_height: DEFAULT_CURRENT_HEIGHT.saturating_sub(12),
        expected_evidence_root: String::new(),
        pq_quorum: pq,
        reserve_coverage,
        privacy_budget,
        live_witnesses,
    };
    package.expected_evidence_root = package.computed_evidence_root();
    package
}
