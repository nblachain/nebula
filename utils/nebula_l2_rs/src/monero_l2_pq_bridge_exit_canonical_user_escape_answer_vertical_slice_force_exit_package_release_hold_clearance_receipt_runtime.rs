use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReleaseHoldClearanceReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_HOLD_CLEARANCE_RECEIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-release-hold-clearance-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_HOLD_CLEARANCE_RECEIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_HOLD_CLEARANCE_RECEIPT_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-release-hold-clearance-receipt-v1";
pub const DEFAULT_MIN_CLEARANCE_RECEIPTS: u64 = 9;
pub const DEFAULT_MIN_USER_ESCAPE_CLEARANCE_RECEIPTS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub clearance_suite: String,
    pub min_clearance_receipts: u64,
    pub min_user_escape_clearance_receipts: u64,
    pub require_wallet_submission_clearance: bool,
    pub require_challenge_window_clearance: bool,
    pub require_settlement_clearance: bool,
    pub require_reserve_clearance: bool,
    pub require_pq_authority_clearance: bool,
    pub require_privacy_clearance: bool,
    pub require_fail_closed_recovery_receipts: bool,
    pub require_release_hold_receipts: bool,
    pub require_zero_user_release_blockers: bool,
    pub require_zero_production_blockers: bool,
    pub hold_production_until_clearance_observed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            clearance_suite: RELEASE_HOLD_CLEARANCE_RECEIPT_SUITE.to_string(),
            min_clearance_receipts: DEFAULT_MIN_CLEARANCE_RECEIPTS,
            min_user_escape_clearance_receipts: DEFAULT_MIN_USER_ESCAPE_CLEARANCE_RECEIPTS,
            require_wallet_submission_clearance: true,
            require_challenge_window_clearance: true,
            require_settlement_clearance: true,
            require_reserve_clearance: true,
            require_pq_authority_clearance: true,
            require_privacy_clearance: true,
            require_fail_closed_recovery_receipts: true,
            require_release_hold_receipts: true,
            require_zero_user_release_blockers: true,
            require_zero_production_blockers: true,
            hold_production_until_clearance_observed: true,
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
            "clearance_suite": self.clearance_suite,
            "min_clearance_receipts": self.min_clearance_receipts,
            "min_user_escape_clearance_receipts": self.min_user_escape_clearance_receipts,
            "require_wallet_submission_clearance": self.require_wallet_submission_clearance,
            "require_challenge_window_clearance": self.require_challenge_window_clearance,
            "require_settlement_clearance": self.require_settlement_clearance,
            "require_reserve_clearance": self.require_reserve_clearance,
            "require_pq_authority_clearance": self.require_pq_authority_clearance,
            "require_privacy_clearance": self.require_privacy_clearance,
            "require_fail_closed_recovery_receipts": self.require_fail_closed_recovery_receipts,
            "require_release_hold_receipts": self.require_release_hold_receipts,
            "require_zero_user_release_blockers": self.require_zero_user_release_blockers,
            "require_zero_production_blockers": self.require_zero_production_blockers,
            "hold_production_until_clearance_observed": self.hold_production_until_clearance_observed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceLaneKind {
    WalletSubmission,
    EvidenceBundle,
    PqAuthority,
    ChallengeWindow,
    ReserveFallback,
    SettlementObservation,
    RecoveryReadiness,
    ReleaseHoldNotice,
}

impl ClearanceLaneKind {
    pub fn from_receipt(receipt_kind: execution::ExecutionReceiptKind) -> Self {
        match receipt_kind {
            execution::ExecutionReceiptKind::IntakeWalletTranscript => Self::WalletSubmission,
            execution::ExecutionReceiptKind::BuildEvidenceBundle => Self::EvidenceBundle,
            execution::ExecutionReceiptKind::PqAuthorizeClaim => Self::PqAuthority,
            execution::ExecutionReceiptKind::BindChallengeWindow => Self::ChallengeWindow,
            execution::ExecutionReceiptKind::AttachReserveFallback => Self::ReserveFallback,
            execution::ExecutionReceiptKind::BroadcastExitClaim => Self::SettlementObservation,
            execution::ExecutionReceiptKind::WatchSettlementReceipt => Self::SettlementObservation,
            execution::ExecutionReceiptKind::FailClosedRecovery => Self::RecoveryReadiness,
            execution::ExecutionReceiptKind::ReportReleaseHold => Self::ReleaseHoldNotice,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletSubmission => "wallet_submission",
            Self::EvidenceBundle => "evidence_bundle",
            Self::PqAuthority => "pq_authority",
            Self::ChallengeWindow => "challenge_window",
            Self::ReserveFallback => "reserve_fallback",
            Self::SettlementObservation => "settlement_observation",
            Self::RecoveryReadiness => "recovery_readiness",
            Self::ReleaseHoldNotice => "release_hold_notice",
        }
    }

    pub fn blocks_user_release_if_missing(self) -> bool {
        matches!(
            self,
            Self::WalletSubmission
                | Self::EvidenceBundle
                | Self::PqAuthority
                | Self::SettlementObservation
                | Self::RecoveryReadiness
                | Self::ReleaseHoldNotice
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceStatus {
    Clearable,
    HeldUntilLiveEvidence,
    DeferredUntilExecutionObserved,
    RecoveryRequired,
    RejectedForBlocker,
}

impl ClearanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clearable => "clearable",
            Self::HeldUntilLiveEvidence => "held_until_live_evidence",
            Self::DeferredUntilExecutionObserved => "deferred_until_execution_observed",
            Self::RecoveryRequired => "recovery_required",
            Self::RejectedForBlocker => "rejected_for_blocker",
        }
    }

    pub fn is_clearable(self) -> bool {
        self == Self::Clearable
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_state_root: String,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub challenge_settlement_bundle_root: String,
    pub pq_privacy_receipt_root: String,
    pub recovery_receipt_root: String,
    pub execution_production_hold_root: String,
    pub execution_verdict_root: String,
    pub execution_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub user_escape_receipt_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub package_action_count: u64,
    pub package_submit_ready: bool,
    pub wallet_submission_receipts_present: bool,
    pub challenge_window_receipts_present: bool,
    pub settlement_receipts_present: bool,
    pub reserve_receipts_present: bool,
    pub pq_verification_receipts_present: bool,
    pub privacy_receipts_present: bool,
    pub fail_closed_receipts_present: bool,
    pub release_hold_receipts_present: bool,
    pub package_execution_observed: bool,
    pub user_escape_execution_observed: bool,
    pub execution_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_execution(state: &execution::State) -> Self {
        Self {
            execution_state_root: state.state_root(),
            execution_receipt_root: state.execution_receipt_root.clone(),
            observed_submission_bundle_root: state.observed_submission_bundle_root.clone(),
            challenge_settlement_bundle_root: state.challenge_settlement_bundle_root.clone(),
            pq_privacy_receipt_root: state.pq_privacy_receipt_root.clone(),
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            execution_production_hold_root: state.production_hold_root.clone(),
            execution_verdict_root: state.verdict.verdict_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            user_escape_answer: state.verdict.user_escape_answer.clone(),
            production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            deferred_receipt_count: state.verdict.deferred_receipt_count,
            release_held_count: state.verdict.release_held_count,
            fail_closed_count: state.verdict.fail_closed_count,
            user_escape_receipt_count: state.verdict.user_escape_receipt_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            package_action_count: state.verdict.package_action_count,
            package_submit_ready: state.verdict.package_submit_ready,
            wallet_submission_receipts_present: state.verdict.wallet_submission_receipts_present,
            challenge_window_receipts_present: state.verdict.challenge_window_receipts_present,
            settlement_receipts_present: state.verdict.settlement_receipts_present,
            reserve_receipts_present: state.verdict.reserve_receipts_present,
            pq_verification_receipts_present: state.verdict.pq_verification_receipts_present,
            privacy_receipts_present: state.verdict.privacy_receipts_present,
            fail_closed_receipts_present: state.verdict.fail_closed_receipts_present,
            release_hold_receipts_present: state.verdict.release_hold_receipts_present,
            package_execution_observed: state.verdict.package_execution_observed,
            user_escape_execution_observed: state.verdict.user_escape_execution_observed,
            execution_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        Self::from_execution(&execution::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "pq_privacy_receipt_root": self.pq_privacy_receipt_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "execution_production_hold_root": self.execution_production_hold_root,
            "execution_verdict_root": self.execution_verdict_root,
            "execution_status": self.execution_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "user_escape_receipt_count": self.user_escape_receipt_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "package_action_count": self.package_action_count,
            "package_submit_ready": self.package_submit_ready,
            "wallet_submission_receipts_present": self.wallet_submission_receipts_present,
            "challenge_window_receipts_present": self.challenge_window_receipts_present,
            "settlement_receipts_present": self.settlement_receipts_present,
            "reserve_receipts_present": self.reserve_receipts_present,
            "pq_verification_receipts_present": self.pq_verification_receipts_present,
            "privacy_receipts_present": self.privacy_receipts_present,
            "fail_closed_receipts_present": self.fail_closed_receipts_present,
            "release_hold_receipts_present": self.release_hold_receipts_present,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseHoldClearanceReceipt {
    pub clearance_id: String,
    pub ordinal: u64,
    pub lane_kind: ClearanceLaneKind,
    pub source_receipt_id: String,
    pub source_receipt_kind: String,
    pub source_receipt_status: String,
    pub execution_receipt_root: String,
    pub required_evidence_root: String,
    pub observed_clearance_root: String,
    pub missing_evidence_root: String,
    pub wallet_notice_root: String,
    pub production_hold_root: String,
    pub clearance_receipt_root: String,
    pub status: ClearanceStatus,
    pub user_release_blocked: bool,
    pub production_blocked: bool,
    pub missing_evidence_count: u64,
    pub clearance_statement: String,
}

impl ReleaseHoldClearanceReceipt {
    pub fn from_execution_receipt(
        config: &Config,
        source: &SourceBundle,
        receipt: &execution::ForceExitExecutionReceipt,
    ) -> Self {
        let lane_kind = ClearanceLaneKind::from_receipt(receipt.receipt_kind);
        let missing_evidence = missing_evidence(config, source, receipt, lane_kind);
        let missing_evidence_count = missing_evidence.len() as u64;
        let status = clearance_status(source, receipt, missing_evidence_count);
        let required_evidence_root =
            required_evidence_root(config, source, receipt, lane_kind, &missing_evidence);
        let observed_clearance_root =
            observed_clearance_root(config, source, receipt, lane_kind, status);
        let missing_evidence_root =
            missing_evidence_root(config, source, receipt, lane_kind, &missing_evidence);
        let user_release_blocked = receipt.blocks_user_release
            || missing_evidence_count > 0 && lane_kind.blocks_user_release_if_missing();
        let production_blocked = receipt.blocks_production
            || source.execution_production_blocked
            || missing_evidence_count > 0
            || status != ClearanceStatus::Clearable;
        let wallet_notice_root = wallet_notice_root(
            config,
            source,
            receipt,
            lane_kind,
            status,
            user_release_blocked,
        );
        let production_hold_root = production_hold_root(
            config,
            source,
            receipt,
            lane_kind,
            status,
            production_blocked,
        );
        let clearance_statement = clearance_statement(status, lane_kind).to_string();
        let clearance_receipt_root = clearance_receipt_root(
            config,
            source,
            receipt,
            lane_kind,
            status,
            &required_evidence_root,
            &observed_clearance_root,
            &missing_evidence_root,
            &wallet_notice_root,
            &production_hold_root,
            user_release_blocked,
            production_blocked,
            missing_evidence_count,
        );
        let clearance_id = clearance_id(lane_kind, receipt.ordinal, &clearance_receipt_root);
        Self {
            clearance_id,
            ordinal: receipt.ordinal,
            lane_kind,
            source_receipt_id: receipt.receipt_id.clone(),
            source_receipt_kind: receipt.receipt_kind.as_str().to_string(),
            source_receipt_status: receipt.status.as_str().to_string(),
            execution_receipt_root: receipt.execution_receipt_root.clone(),
            required_evidence_root,
            observed_clearance_root,
            missing_evidence_root,
            wallet_notice_root,
            production_hold_root,
            clearance_receipt_root,
            status,
            user_release_blocked,
            production_blocked,
            missing_evidence_count,
            clearance_statement,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "clearance_id": self.clearance_id,
            "ordinal": self.ordinal,
            "lane_kind": self.lane_kind.as_str(),
            "source_receipt_id": self.source_receipt_id,
            "source_receipt_kind": self.source_receipt_kind,
            "source_receipt_status": self.source_receipt_status,
            "execution_receipt_root": self.execution_receipt_root,
            "required_evidence_root": self.required_evidence_root,
            "observed_clearance_root": self.observed_clearance_root,
            "missing_evidence_root": self.missing_evidence_root,
            "wallet_notice_root": self.wallet_notice_root,
            "production_hold_root": self.production_hold_root,
            "clearance_receipt_root": self.clearance_receipt_root,
            "status": self.status.as_str(),
            "user_release_blocked": self.user_release_blocked,
            "production_blocked": self.production_blocked,
            "missing_evidence_count": self.missing_evidence_count,
            "clearance_statement": self.clearance_statement,
        })
    }

    pub fn state_root(&self) -> String {
        self.clearance_receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClearanceVerdict {
    pub clearance_receipt_count: u64,
    pub clearable_count: u64,
    pub held_count: u64,
    pub deferred_count: u64,
    pub recovery_required_count: u64,
    pub rejected_count: u64,
    pub missing_evidence_count: u64,
    pub wallet_clearance_count: u64,
    pub settlement_clearance_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub all_required_roots_present: bool,
    pub package_execution_observed: bool,
    pub user_escape_clearance_supported: bool,
    pub production_clearance_allowed: bool,
    pub clearance_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ClearanceVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        receipts: &[ReleaseHoldClearanceReceipt],
    ) -> Self {
        let clearance_receipt_count = receipts.len() as u64;
        let clearable_count = count_status(receipts, ClearanceStatus::Clearable);
        let held_count = count_status(receipts, ClearanceStatus::HeldUntilLiveEvidence);
        let deferred_count =
            count_status(receipts, ClearanceStatus::DeferredUntilExecutionObserved);
        let recovery_required_count = count_status(receipts, ClearanceStatus::RecoveryRequired);
        let rejected_count = count_status(receipts, ClearanceStatus::RejectedForBlocker);
        let missing_evidence_count = receipts
            .iter()
            .map(|receipt| receipt.missing_evidence_count)
            .sum::<u64>();
        let wallet_clearance_count = receipts
            .iter()
            .filter(|receipt| receipt.lane_kind == ClearanceLaneKind::WalletSubmission)
            .filter(|receipt| receipt.status.is_clearable())
            .count() as u64;
        let settlement_clearance_count = receipts
            .iter()
            .filter(|receipt| receipt.lane_kind == ClearanceLaneKind::SettlementObservation)
            .filter(|receipt| receipt.status.is_clearable())
            .count() as u64;
        let user_release_blocker_count = receipts
            .iter()
            .filter(|receipt| receipt.user_release_blocked)
            .count() as u64;
        let production_blocker_count = receipts
            .iter()
            .filter(|receipt| receipt.production_blocked)
            .count() as u64;
        let all_required_roots_present = missing_evidence_count == 0
            && (!config.require_zero_user_release_blockers || user_release_blocker_count == 0)
            && (!config.require_zero_production_blockers || production_blocker_count == 0);
        let package_execution_observed = source.package_execution_observed
            && source.observed_receipt_count >= config.min_clearance_receipts;
        let user_escape_clearance_supported = package_execution_observed
            && clearable_count >= config.min_user_escape_clearance_receipts
            && all_required_roots_present
            && source.user_escape_execution_observed;
        let production_clearance_allowed = user_escape_clearance_supported
            && clearable_count == clearance_receipt_count
            && source.package_action_count == clearance_receipt_count
            && !source.execution_production_blocked
            && !config.hold_production_until_clearance_observed;
        let clearance_status = if rejected_count > 0 {
            "rejected_for_blocker"
        } else if recovery_required_count > 0 {
            "recovery_required"
        } else if deferred_count > 0 {
            "deferred_until_execution_observed"
        } else if held_count > 0 || !all_required_roots_present {
            "held_until_live_evidence"
        } else if production_clearance_allowed {
            "clearance_allowed"
        } else {
            "clearance_recorded_release_still_held"
        }
        .to_string();
        let user_escape_answer = if user_escape_clearance_supported {
            "user escape clearance receipts are complete for wallet-side forced exit review"
        } else {
            "user escape remains held until live settlement, challenge, PQ, privacy, reserve, and recovery receipts clear"
        }
        .to_string();
        let production_answer = if production_clearance_allowed {
            "production release hold can be considered clear for the bounded force-exit package"
        } else {
            "production release hold remains in place until observed execution clearance and independent review are complete"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            clearance_receipt_count,
            clearable_count,
            held_count,
            deferred_count,
            recovery_required_count,
            rejected_count,
            missing_evidence_count,
            user_release_blocker_count,
            production_blocker_count,
            all_required_roots_present,
            package_execution_observed,
            user_escape_clearance_supported,
            production_clearance_allowed,
            &clearance_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            clearance_receipt_count,
            clearable_count,
            held_count,
            deferred_count,
            recovery_required_count,
            rejected_count,
            missing_evidence_count,
            wallet_clearance_count,
            settlement_clearance_count,
            user_release_blocker_count,
            production_blocker_count,
            all_required_roots_present,
            package_execution_observed,
            user_escape_clearance_supported,
            production_clearance_allowed,
            clearance_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "clearance_receipt_count": self.clearance_receipt_count,
            "clearable_count": self.clearable_count,
            "held_count": self.held_count,
            "deferred_count": self.deferred_count,
            "recovery_required_count": self.recovery_required_count,
            "rejected_count": self.rejected_count,
            "missing_evidence_count": self.missing_evidence_count,
            "wallet_clearance_count": self.wallet_clearance_count,
            "settlement_clearance_count": self.settlement_clearance_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "all_required_roots_present": self.all_required_roots_present,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_clearance_supported": self.user_escape_clearance_supported,
            "production_clearance_allowed": self.production_clearance_allowed,
            "clearance_status": self.clearance_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub clearance_receipts: Vec<ReleaseHoldClearanceReceipt>,
    pub verdict: ClearanceVerdict,
    pub clearance_receipt_root: String,
    pub evidence_clearance_bundle_root: String,
    pub wallet_notice_bundle_root: String,
    pub unresolved_blocker_root: String,
    pub production_hold_clearance_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution(&execution_state);
        validate_source(&source)?;
        let clearance_receipts = execution_state
            .execution_receipts
            .iter()
            .map(|receipt| {
                ReleaseHoldClearanceReceipt::from_execution_receipt(&config, &source, receipt)
            })
            .collect::<Vec<_>>();
        let verdict = ClearanceVerdict::new(&config, &source, &clearance_receipts);
        let clearance_receipt_root = clearance_receipt_vector_root(&clearance_receipts);
        let evidence_clearance_bundle_root =
            evidence_clearance_bundle_root(&config, &source, &clearance_receipts, &verdict);
        let wallet_notice_bundle_root =
            wallet_notice_bundle_root(&config, &source, &clearance_receipts, &verdict);
        let unresolved_blocker_root =
            unresolved_blocker_root(&config, &source, &clearance_receipts, &verdict);
        let production_hold_clearance_root =
            production_hold_clearance_root(&config, &source, &clearance_receipts, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &clearance_receipt_root,
            &evidence_clearance_bundle_root,
            &wallet_notice_bundle_root,
            &unresolved_blocker_root,
            &production_hold_clearance_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            clearance_receipts,
            verdict,
            clearance_receipt_root,
            evidence_clearance_bundle_root,
            wallet_notice_bundle_root,
            unresolved_blocker_root,
            production_hold_clearance_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), execution::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_release_hold_clearance_receipt_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "clearance_receipt_root": self.clearance_receipt_root,
            "evidence_clearance_bundle_root": self.evidence_clearance_bundle_root,
            "wallet_notice_bundle_root": self.wallet_notice_bundle_root,
            "unresolved_blocker_root": self.unresolved_blocker_root,
            "production_hold_clearance_root": self.production_hold_clearance_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "clearance_receipts": self
                .clearance_receipts
                .iter()
                .map(ReleaseHoldClearanceReceipt::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
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

fn missing_evidence(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
) -> Vec<String> {
    let mut missing = Vec::new();
    if config.require_wallet_submission_clearance
        && (!source.wallet_submission_receipts_present
            || receipt.observed_submission_root.is_empty())
    {
        missing.push("wallet_submission_receipt".to_string());
    }
    if config.require_challenge_window_clearance
        && (!source.challenge_window_receipts_present || receipt.challenge_receipt_root.is_empty())
    {
        missing.push("challenge_window_receipt".to_string());
    }
    if config.require_settlement_clearance
        && (!source.settlement_receipts_present || receipt.settlement_receipt_root.is_empty())
    {
        missing.push("settlement_receipt".to_string());
    }
    if config.require_reserve_clearance
        && (!source.reserve_receipts_present || receipt.reserve_receipt_root.is_empty())
    {
        missing.push("reserve_receipt".to_string());
    }
    if config.require_pq_authority_clearance
        && (!source.pq_verification_receipts_present
            || receipt.pq_verification_receipt_root.is_empty())
    {
        missing.push("pq_authority_receipt".to_string());
    }
    if config.require_privacy_clearance
        && (!source.privacy_receipts_present || receipt.privacy_receipt_root.is_empty())
    {
        missing.push("privacy_boundary_receipt".to_string());
    }
    if config.require_fail_closed_recovery_receipts
        && (!source.fail_closed_receipts_present || receipt.fail_closed_receipt_root.is_empty())
    {
        missing.push("fail_closed_recovery_receipt".to_string());
    }
    if config.require_release_hold_receipts
        && (!source.release_hold_receipts_present || receipt.release_hold_receipt_root.is_empty())
    {
        missing.push("release_hold_notice_receipt".to_string());
    }
    if lane_kind == ClearanceLaneKind::SettlementObservation
        && receipt.status != execution::ExecutionReceiptStatus::Observed
    {
        missing.push("observed_settlement_execution_status".to_string());
    }
    missing
}

fn clearance_status(
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    missing_evidence_count: u64,
) -> ClearanceStatus {
    if receipt.status == execution::ExecutionReceiptStatus::FailClosed {
        ClearanceStatus::RecoveryRequired
    } else if receipt.blocks_user_release || receipt.blocks_production {
        ClearanceStatus::RejectedForBlocker
    } else if !source.package_execution_observed
        || receipt.status == execution::ExecutionReceiptStatus::DeferredUntilLiveSubmission
    {
        ClearanceStatus::DeferredUntilExecutionObserved
    } else if missing_evidence_count > 0 || source.execution_production_blocked {
        ClearanceStatus::HeldUntilLiveEvidence
    } else {
        ClearanceStatus::Clearable
    }
}

fn required_evidence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    missing_evidence: &[String],
) -> String {
    let record = json!({
        "config_root": config.state_root(),
        "source_root": source.state_root(),
        "lane_kind": lane_kind.as_str(),
        "source_receipt_id": receipt.receipt_id,
        "execution_receipt_root": receipt.execution_receipt_root,
        "observed_submission_root": receipt.observed_submission_root,
        "challenge_receipt_root": receipt.challenge_receipt_root,
        "settlement_receipt_root": receipt.settlement_receipt_root,
        "reserve_receipt_root": receipt.reserve_receipt_root,
        "pq_verification_receipt_root": receipt.pq_verification_receipt_root,
        "privacy_receipt_root": receipt.privacy_receipt_root,
        "fail_closed_receipt_root": receipt.fail_closed_receipt_root,
        "release_hold_receipt_root": receipt.release_hold_receipt_root,
        "missing_evidence": missing_evidence,
    });
    record_root("required-evidence", &record)
}

fn observed_clearance_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    status: ClearanceStatus,
) -> String {
    let record = json!({
        "config_root": config.state_root(),
        "source_root": source.state_root(),
        "lane_kind": lane_kind.as_str(),
        "source_receipt_id": receipt.receipt_id,
        "execution_receipt_root": receipt.execution_receipt_root,
        "receipt_status": receipt.status.as_str(),
        "clearance_status": status.as_str(),
        "package_execution_observed": source.package_execution_observed,
        "user_escape_execution_observed": source.user_escape_execution_observed,
        "execution_production_blocked": source.execution_production_blocked,
    });
    record_root("observed-clearance", &record)
}

fn missing_evidence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    missing_evidence: &[String],
) -> String {
    let record = json!({
        "config_root": config.state_root(),
        "source_root": source.state_root(),
        "lane_kind": lane_kind.as_str(),
        "source_receipt_id": receipt.receipt_id,
        "missing_evidence_count": missing_evidence.len() as u64,
        "missing_evidence": missing_evidence,
        "user_release_blocker_source": bool_str(receipt.blocks_user_release),
        "production_blocker_source": bool_str(receipt.blocks_production),
    });
    record_root("missing-evidence", &record)
}

fn wallet_notice_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    status: ClearanceStatus,
    user_release_blocked: bool,
) -> String {
    let record = json!({
        "config_root": config.state_root(),
        "source_root": source.state_root(),
        "lane_kind": lane_kind.as_str(),
        "source_receipt_id": receipt.receipt_id,
        "clearance_status": status.as_str(),
        "user_release_blocked": user_release_blocked,
        "wallet_instruction": wallet_instruction(status, lane_kind),
        "user_escape_answer": source.user_escape_answer,
    });
    record_root("wallet-notice", &record)
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    status: ClearanceStatus,
    production_blocked: bool,
) -> String {
    let record = json!({
        "config_root": config.state_root(),
        "source_root": source.state_root(),
        "lane_kind": lane_kind.as_str(),
        "source_receipt_id": receipt.receipt_id,
        "clearance_status": status.as_str(),
        "production_blocked": production_blocked,
        "execution_production_hold_root": source.execution_production_hold_root,
        "production_answer": source.production_answer,
    });
    record_root("production-hold", &record)
}

fn clearance_receipt_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    lane_kind: ClearanceLaneKind,
    status: ClearanceStatus,
    required_evidence_root: &str,
    observed_clearance_root: &str,
    missing_evidence_root: &str,
    wallet_notice_root: &str,
    production_hold_root: &str,
    user_release_blocked: bool,
    production_blocked: bool,
    missing_evidence_count: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-RECEIPT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(lane_kind.as_str()),
            HashPart::U64(receipt.ordinal),
            HashPart::Str(&receipt.receipt_id),
            HashPart::Str(&receipt.execution_receipt_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(required_evidence_root),
            HashPart::Str(observed_clearance_root),
            HashPart::Str(missing_evidence_root),
            HashPart::Str(wallet_notice_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(bool_str(user_release_blocked)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::U64(missing_evidence_count),
        ],
        32,
    )
}

fn clearance_id(
    lane_kind: ClearanceLaneKind,
    ordinal: u64,
    clearance_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-ID",
        &[
            HashPart::Str(lane_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(clearance_receipt_root),
        ],
        16,
    )
}

fn clearance_receipt_vector_root(receipts: &[ReleaseHoldClearanceReceipt]) -> String {
    let leaves = receipts
        .iter()
        .map(ReleaseHoldClearanceReceipt::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-RECEIPT-VECTOR",
        &leaves,
    )
}

fn evidence_clearance_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ReleaseHoldClearanceReceipt],
    verdict: &ClearanceVerdict,
) -> String {
    let leaves = receipts
        .iter()
        .map(|receipt| {
            json!({
                "clearance_id": receipt.clearance_id,
                "lane_kind": receipt.lane_kind.as_str(),
                "required_evidence_root": receipt.required_evidence_root,
                "observed_clearance_root": receipt.observed_clearance_root,
                "missing_evidence_count": receipt.missing_evidence_count,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-EVIDENCE-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-EVIDENCE-LEAVES",
                &leaves,
            )),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn wallet_notice_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ReleaseHoldClearanceReceipt],
    verdict: &ClearanceVerdict,
) -> String {
    let leaves = receipts
        .iter()
        .map(|receipt| {
            json!({
                "clearance_id": receipt.clearance_id,
                "wallet_notice_root": receipt.wallet_notice_root,
                "user_release_blocked": receipt.user_release_blocked,
                "clearance_statement": receipt.clearance_statement,
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-WALLET-NOTICES",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-WALLET-LEAVES",
                &leaves,
            )),
            HashPart::Str(&verdict.user_escape_answer),
        ],
        32,
    )
}

fn unresolved_blocker_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ReleaseHoldClearanceReceipt],
    verdict: &ClearanceVerdict,
) -> String {
    let leaves = receipts
        .iter()
        .filter(|receipt| receipt.user_release_blocked || receipt.production_blocked)
        .map(|receipt| {
            json!({
                "clearance_id": receipt.clearance_id,
                "lane_kind": receipt.lane_kind.as_str(),
                "missing_evidence_root": receipt.missing_evidence_root,
                "production_hold_root": receipt.production_hold_root,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-UNRESOLVED-BLOCKERS",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_production_hold_root),
            HashPart::U64(verdict.user_release_blocker_count),
            HashPart::U64(verdict.production_blocker_count),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-BLOCKER-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn production_hold_clearance_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ReleaseHoldClearanceReceipt],
    verdict: &ClearanceVerdict,
) -> String {
    let hold_leaves = receipts
        .iter()
        .map(|receipt| {
            json!({
                "clearance_id": receipt.clearance_id,
                "production_hold_root": receipt.production_hold_root,
                "production_blocked": receipt.production_blocked,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_production_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::Str(bool_str(verdict.production_clearance_allowed)),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-HOLD-LEAVES",
                &hold_leaves,
            )),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    clearance_receipt_root: &str,
    evidence_clearance_bundle_root: &str,
    wallet_notice_bundle_root: &str,
    unresolved_blocker_root: &str,
    production_hold_clearance_root: &str,
    verdict: &ClearanceVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(clearance_receipt_root),
            HashPart::Str(evidence_clearance_bundle_root),
            HashPart::Str(wallet_notice_bundle_root),
            HashPart::Str(unresolved_blocker_root),
            HashPart::Str(production_hold_clearance_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    clearance_receipt_count: u64,
    clearable_count: u64,
    held_count: u64,
    deferred_count: u64,
    recovery_required_count: u64,
    rejected_count: u64,
    missing_evidence_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    all_required_roots_present: bool,
    package_execution_observed: bool,
    user_escape_clearance_supported: bool,
    production_clearance_allowed: bool,
    clearance_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.execution_verdict_root),
            HashPart::U64(clearance_receipt_count),
            HashPart::U64(clearable_count),
            HashPart::U64(held_count),
            HashPart::U64(deferred_count),
            HashPart::U64(recovery_required_count),
            HashPart::U64(rejected_count),
            HashPart::U64(missing_evidence_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(all_required_roots_present)),
            HashPart::Str(bool_str(package_execution_observed)),
            HashPart::Str(bool_str(user_escape_clearance_supported)),
            HashPart::Str(bool_str(production_clearance_allowed)),
            HashPart::Str(clearance_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(receipts: &[ReleaseHoldClearanceReceipt], status: ClearanceStatus) -> u64 {
    receipts
        .iter()
        .filter(|receipt| receipt.status == status)
        .count() as u64
}

fn wallet_instruction(status: ClearanceStatus, lane_kind: ClearanceLaneKind) -> &'static str {
    match status {
        ClearanceStatus::Clearable => "retain clearance receipt with wallet escape package",
        ClearanceStatus::HeldUntilLiveEvidence => {
            "keep release hold and wait for live evidence receipt"
        }
        ClearanceStatus::DeferredUntilExecutionObserved => {
            "do not treat force-exit execution as observed yet"
        }
        ClearanceStatus::RecoveryRequired => "switch wallet flow to fail-closed recovery playbook",
        ClearanceStatus::RejectedForBlocker => {
            if lane_kind.blocks_user_release_if_missing() {
                "hold user release and surface blocker receipt"
            } else {
                "hold production review and retain blocker receipt"
            }
        }
    }
}

fn clearance_statement(status: ClearanceStatus, lane_kind: ClearanceLaneKind) -> &'static str {
    match status {
        ClearanceStatus::Clearable => "release-hold clearance lane is observed and clearable",
        ClearanceStatus::HeldUntilLiveEvidence => {
            "release-hold clearance lane remains held until live evidence arrives"
        }
        ClearanceStatus::DeferredUntilExecutionObserved => {
            "release-hold clearance lane is deferred until execution receipts are live-observed"
        }
        ClearanceStatus::RecoveryRequired => {
            "release-hold clearance lane requires fail-closed recovery before release"
        }
        ClearanceStatus::RejectedForBlocker => {
            if lane_kind.blocks_user_release_if_missing() {
                "release-hold clearance lane rejects user release until blocker clears"
            } else {
                "release-hold clearance lane rejects production clearance until blocker clears"
            }
        }
    }
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id.is_empty() {
        return Err("chain id is required".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("unexpected release-hold clearance protocol version".to_string());
    }
    if config.min_clearance_receipts == 0 {
        return Err("at least one clearance receipt is required".to_string());
    }
    if config.min_user_escape_clearance_receipts == 0 {
        return Err("at least one user escape clearance receipt is required".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.execution_state_root.is_empty() {
        return Err("execution state root is required".to_string());
    }
    if source.execution_receipt_root.is_empty() {
        return Err("execution receipt root is required".to_string());
    }
    if source.execution_receipt_count == 0 {
        return Err("execution receipts are required for release-hold clearance".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle::devnet();
    let fallback_receipt = ReleaseHoldClearanceReceipt {
        clearance_id: record_root("fallback-clearance-id", &json!({"reason": &reason})),
        ordinal: 1,
        lane_kind: ClearanceLaneKind::ReleaseHoldNotice,
        source_receipt_id: "fallback".to_string(),
        source_receipt_kind: "release_hold_notice".to_string(),
        source_receipt_status: "fallback".to_string(),
        execution_receipt_root: source.execution_receipt_root.clone(),
        required_evidence_root: record_root(
            "fallback-required-evidence",
            &json!({"reason": &reason}),
        ),
        observed_clearance_root: record_root(
            "fallback-observed-clearance",
            &json!({"reason": &reason}),
        ),
        missing_evidence_root: record_root(
            "fallback-missing-evidence",
            &json!({"reason": &reason}),
        ),
        wallet_notice_root: record_root("fallback-wallet-notice", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        clearance_receipt_root: record_root(
            "fallback-clearance-receipt",
            &json!({"reason": &reason}),
        ),
        status: ClearanceStatus::HeldUntilLiveEvidence,
        user_release_blocked: true,
        production_blocked: true,
        missing_evidence_count: 1,
        clearance_statement: reason,
    };
    let clearance_receipts = vec![fallback_receipt];
    let verdict = ClearanceVerdict::new(&config, &source, &clearance_receipts);
    let clearance_receipt_root = clearance_receipt_vector_root(&clearance_receipts);
    let evidence_clearance_bundle_root =
        evidence_clearance_bundle_root(&config, &source, &clearance_receipts, &verdict);
    let wallet_notice_bundle_root =
        wallet_notice_bundle_root(&config, &source, &clearance_receipts, &verdict);
    let unresolved_blocker_root =
        unresolved_blocker_root(&config, &source, &clearance_receipts, &verdict);
    let production_hold_clearance_root =
        production_hold_clearance_root(&config, &source, &clearance_receipts, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &clearance_receipt_root,
        &evidence_clearance_bundle_root,
        &wallet_notice_bundle_root,
        &unresolved_blocker_root,
        &production_hold_clearance_root,
        &verdict,
    );
    State {
        config,
        source,
        clearance_receipts,
        verdict,
        clearance_receipt_root,
        evidence_clearance_bundle_root,
        wallet_notice_bundle_root,
        unresolved_blocker_root,
        production_hold_clearance_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RELEASE-HOLD-CLEARANCE-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
