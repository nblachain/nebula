use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_operator_independent_force_exit_package_runtime as package,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageExecutionReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-execution-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const FORCE_EXIT_PACKAGE_EXECUTION_RECEIPT_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-execution-receipt-v1";
pub const DEFAULT_MIN_EXECUTION_RECEIPTS: u64 = 9;
pub const DEFAULT_MIN_USER_ESCAPE_RECEIPTS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub execution_receipt_suite: String,
    pub min_execution_receipts: u64,
    pub min_user_escape_receipts: u64,
    pub require_wallet_submission_receipts: bool,
    pub require_challenge_window_receipts: bool,
    pub require_settlement_receipts: bool,
    pub require_reserve_receipts: bool,
    pub require_pq_verification_receipts: bool,
    pub require_privacy_receipts: bool,
    pub hold_production_until_execution_observed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            execution_receipt_suite: FORCE_EXIT_PACKAGE_EXECUTION_RECEIPT_SUITE.to_string(),
            min_execution_receipts: DEFAULT_MIN_EXECUTION_RECEIPTS,
            min_user_escape_receipts: DEFAULT_MIN_USER_ESCAPE_RECEIPTS,
            require_wallet_submission_receipts: true,
            require_challenge_window_receipts: true,
            require_settlement_receipts: true,
            require_reserve_receipts: true,
            require_pq_verification_receipts: true,
            require_privacy_receipts: true,
            hold_production_until_execution_observed: true,
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
            "execution_receipt_suite": self.execution_receipt_suite,
            "min_execution_receipts": self.min_execution_receipts,
            "min_user_escape_receipts": self.min_user_escape_receipts,
            "require_wallet_submission_receipts": self.require_wallet_submission_receipts,
            "require_challenge_window_receipts": self.require_challenge_window_receipts,
            "require_settlement_receipts": self.require_settlement_receipts,
            "require_reserve_receipts": self.require_reserve_receipts,
            "require_pq_verification_receipts": self.require_pq_verification_receipts,
            "require_privacy_receipts": self.require_privacy_receipts,
            "hold_production_until_execution_observed": self.hold_production_until_execution_observed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionReceiptKind {
    IntakeWalletTranscript,
    BuildEvidenceBundle,
    PqAuthorizeClaim,
    BindChallengeWindow,
    AttachReserveFallback,
    BroadcastExitClaim,
    WatchSettlementReceipt,
    FailClosedRecovery,
    ReportReleaseHold,
}

impl ExecutionReceiptKind {
    pub fn from_action(kind: package::ForceExitActionKind) -> Self {
        match kind {
            package::ForceExitActionKind::IntakeWalletTranscript => Self::IntakeWalletTranscript,
            package::ForceExitActionKind::BuildEvidenceBundle => Self::BuildEvidenceBundle,
            package::ForceExitActionKind::PqAuthorizeClaim => Self::PqAuthorizeClaim,
            package::ForceExitActionKind::BindChallengeWindow => Self::BindChallengeWindow,
            package::ForceExitActionKind::AttachReserveFallback => Self::AttachReserveFallback,
            package::ForceExitActionKind::BroadcastExitClaim => Self::BroadcastExitClaim,
            package::ForceExitActionKind::WatchSettlementReceipt => Self::WatchSettlementReceipt,
            package::ForceExitActionKind::FailClosedRecovery => Self::FailClosedRecovery,
            package::ForceExitActionKind::ReportReleaseHold => Self::ReportReleaseHold,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntakeWalletTranscript => "intake_wallet_transcript",
            Self::BuildEvidenceBundle => "build_evidence_bundle",
            Self::PqAuthorizeClaim => "pq_authorize_claim",
            Self::BindChallengeWindow => "bind_challenge_window",
            Self::AttachReserveFallback => "attach_reserve_fallback",
            Self::BroadcastExitClaim => "broadcast_exit_claim",
            Self::WatchSettlementReceipt => "watch_settlement_receipt",
            Self::FailClosedRecovery => "fail_closed_recovery",
            Self::ReportReleaseHold => "report_release_hold",
        }
    }

    pub fn observed_receipt(self) -> &'static str {
        match self {
            Self::IntakeWalletTranscript => "wallet_transcript_ingested_receipt",
            Self::BuildEvidenceBundle => "operator_independent_evidence_bundle_receipt",
            Self::PqAuthorizeClaim => "pq_withdrawal_authorization_receipt",
            Self::BindChallengeWindow => "challenge_window_bound_receipt",
            Self::AttachReserveFallback => "reserve_liquidity_fallback_receipt",
            Self::BroadcastExitClaim => "force_exit_claim_broadcast_receipt",
            Self::WatchSettlementReceipt => "settlement_watch_receipt",
            Self::FailClosedRecovery => "fail_closed_recovery_receipt",
            Self::ReportReleaseHold => "release_hold_report_receipt",
        }
    }

    pub fn user_escape_receipt(self) -> bool {
        matches!(
            self,
            Self::IntakeWalletTranscript
                | Self::BuildEvidenceBundle
                | Self::PqAuthorizeClaim
                | Self::BroadcastExitClaim
                | Self::WatchSettlementReceipt
                | Self::FailClosedRecovery
                | Self::ReportReleaseHold
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionReceiptStatus {
    Observed,
    DeferredUntilLiveSubmission,
    ReleaseHeld,
    FailClosed,
}

impl ExecutionReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::DeferredUntilLiveSubmission => "deferred_until_live_submission",
            Self::ReleaseHeld => "release_held",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub package_state_root: String,
    pub package_action_root: String,
    pub command_bundle_root: String,
    pub wallet_submission_bundle_root: String,
    pub challenge_window_bundle_root: String,
    pub recovery_bundle_root: String,
    pub package_production_hold_root: String,
    pub package_status: String,
    pub package_user_escape_answer: String,
    pub package_production_answer: String,
    pub package_action_count: u64,
    pub ready_action_count: u64,
    pub deferred_action_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub wallet_visible_count: u64,
    pub operator_independent_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub package_submit_ready: bool,
    pub package_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_package(state: &package::State) -> Self {
        Self {
            package_state_root: state.state_root(),
            package_action_root: state.package_action_root.clone(),
            command_bundle_root: state.command_bundle_root.clone(),
            wallet_submission_bundle_root: state.wallet_submission_bundle_root.clone(),
            challenge_window_bundle_root: state.challenge_window_bundle_root.clone(),
            recovery_bundle_root: state.recovery_bundle_root.clone(),
            package_production_hold_root: state.production_hold_root.clone(),
            package_status: state.verdict.package_status.clone(),
            package_user_escape_answer: state.verdict.user_escape_answer.clone(),
            package_production_answer: state.verdict.production_answer.clone(),
            package_action_count: state.verdict.package_action_count,
            ready_action_count: state.verdict.ready_action_count,
            deferred_action_count: state.verdict.deferred_action_count,
            release_held_count: state.verdict.release_held_count,
            fail_closed_count: state.verdict.fail_closed_count,
            wallet_visible_count: state.verdict.wallet_visible_count,
            operator_independent_count: state.verdict.operator_independent_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            package_submit_ready: state.verdict.package_submit_ready,
            package_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let state = package::devnet();
        Self::from_package(&state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "package_state_root": self.package_state_root,
            "package_action_root": self.package_action_root,
            "command_bundle_root": self.command_bundle_root,
            "wallet_submission_bundle_root": self.wallet_submission_bundle_root,
            "challenge_window_bundle_root": self.challenge_window_bundle_root,
            "recovery_bundle_root": self.recovery_bundle_root,
            "package_production_hold_root": self.package_production_hold_root,
            "package_status": self.package_status,
            "package_user_escape_answer": self.package_user_escape_answer,
            "package_production_answer": self.package_production_answer,
            "package_action_count": self.package_action_count,
            "ready_action_count": self.ready_action_count,
            "deferred_action_count": self.deferred_action_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "wallet_visible_count": self.wallet_visible_count,
            "operator_independent_count": self.operator_independent_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "package_submit_ready": self.package_submit_ready,
            "package_production_blocked": self.package_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForceExitExecutionReceipt {
    pub receipt_id: String,
    pub ordinal: u64,
    pub receipt_kind: ExecutionReceiptKind,
    pub package_action_id: String,
    pub package_action_status: String,
    pub command_root: String,
    pub expected_submission_root: String,
    pub observed_submission_root: String,
    pub challenge_receipt_root: String,
    pub settlement_receipt_root: String,
    pub reserve_receipt_root: String,
    pub pq_verification_receipt_root: String,
    pub privacy_receipt_root: String,
    pub fail_closed_receipt_root: String,
    pub release_hold_receipt_root: String,
    pub execution_receipt_root: String,
    pub status: ExecutionReceiptStatus,
    pub user_escape_receipt: bool,
    pub operator_independent: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub observed_receipt: String,
    pub required_outcome: String,
}

impl ForceExitExecutionReceipt {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        action: &package::ForceExitPackageAction,
    ) -> Self {
        let receipt_kind = ExecutionReceiptKind::from_action(action.action_kind);
        let status = execution_status(source, action);
        let expected_submission_root =
            expected_submission_root(config, source, action, receipt_kind);
        let observed_submission_root =
            observed_submission_root(config, source, action, status, &expected_submission_root);
        let challenge_receipt_root = challenge_receipt_root(config, source, action, receipt_kind);
        let settlement_receipt_root = settlement_receipt_root(config, source, action, receipt_kind);
        let reserve_receipt_root = reserve_receipt_root(config, source, action, receipt_kind);
        let pq_verification_receipt_root =
            pq_verification_receipt_root(config, source, action, receipt_kind);
        let privacy_receipt_root = privacy_receipt_root(config, source, action, receipt_kind);
        let fail_closed_receipt_root =
            fail_closed_receipt_root(config, source, action, receipt_kind, status);
        let release_hold_receipt_root =
            release_hold_receipt_root(config, source, action, receipt_kind, status);
        let operator_independent = action.operator_independent;
        let user_escape_receipt = receipt_kind.user_escape_receipt();
        let blocks_user_release =
            action.blocks_user_release || status == ExecutionReceiptStatus::FailClosed;
        let blocks_production =
            action.blocks_production || status != ExecutionReceiptStatus::Observed;
        let execution_receipt_root = execution_receipt_root(
            config,
            source,
            action,
            receipt_kind,
            status,
            &expected_submission_root,
            &observed_submission_root,
            &challenge_receipt_root,
            &settlement_receipt_root,
            &reserve_receipt_root,
            &pq_verification_receipt_root,
            &privacy_receipt_root,
            &fail_closed_receipt_root,
            &release_hold_receipt_root,
            blocks_user_release,
            blocks_production,
        );
        let receipt_id = receipt_id(receipt_kind, action.ordinal, &execution_receipt_root);
        Self {
            receipt_id,
            ordinal: action.ordinal,
            receipt_kind,
            package_action_id: action.action_id.clone(),
            package_action_status: action.status.as_str().to_string(),
            command_root: action.command_root.clone(),
            expected_submission_root,
            observed_submission_root,
            challenge_receipt_root,
            settlement_receipt_root,
            reserve_receipt_root,
            pq_verification_receipt_root,
            privacy_receipt_root,
            fail_closed_receipt_root,
            release_hold_receipt_root,
            execution_receipt_root,
            status,
            user_escape_receipt,
            operator_independent,
            blocks_user_release,
            blocks_production,
            observed_receipt: receipt_kind.observed_receipt().to_string(),
            required_outcome: required_outcome(status, receipt_kind).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "ordinal": self.ordinal,
            "receipt_kind": self.receipt_kind.as_str(),
            "package_action_id": self.package_action_id,
            "package_action_status": self.package_action_status,
            "command_root": self.command_root,
            "expected_submission_root": self.expected_submission_root,
            "observed_submission_root": self.observed_submission_root,
            "challenge_receipt_root": self.challenge_receipt_root,
            "settlement_receipt_root": self.settlement_receipt_root,
            "reserve_receipt_root": self.reserve_receipt_root,
            "pq_verification_receipt_root": self.pq_verification_receipt_root,
            "privacy_receipt_root": self.privacy_receipt_root,
            "fail_closed_receipt_root": self.fail_closed_receipt_root,
            "release_hold_receipt_root": self.release_hold_receipt_root,
            "execution_receipt_root": self.execution_receipt_root,
            "status": self.status.as_str(),
            "user_escape_receipt": self.user_escape_receipt,
            "operator_independent": self.operator_independent,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "observed_receipt": self.observed_receipt,
            "required_outcome": self.required_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.execution_receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecutionReceiptVerdict {
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub user_escape_receipt_count: u64,
    pub operator_independent_count: u64,
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
    pub production_blocked: bool,
    pub execution_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ExecutionReceiptVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        receipts: &[ForceExitExecutionReceipt],
    ) -> Self {
        let execution_receipt_count = receipts.len() as u64;
        let observed_receipt_count = count_status(receipts, ExecutionReceiptStatus::Observed);
        let deferred_receipt_count = count_status(
            receipts,
            ExecutionReceiptStatus::DeferredUntilLiveSubmission,
        );
        let release_held_count = count_status(receipts, ExecutionReceiptStatus::ReleaseHeld);
        let fail_closed_count = count_status(receipts, ExecutionReceiptStatus::FailClosed);
        let user_escape_receipt_count = receipts
            .iter()
            .filter(|receipt| receipt.user_escape_receipt)
            .count() as u64;
        let operator_independent_count = receipts
            .iter()
            .filter(|receipt| receipt.operator_independent)
            .count() as u64;
        let user_release_blocker_count = receipts
            .iter()
            .filter(|receipt| receipt.blocks_user_release)
            .count() as u64;
        let production_blocker_count = receipts
            .iter()
            .filter(|receipt| receipt.blocks_production)
            .count() as u64;
        let package_action_count = source.package_action_count;
        let package_submit_ready = source.package_submit_ready;
        let wallet_submission_receipts_present = !config.require_wallet_submission_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.observed_submission_root.is_empty());
        let challenge_window_receipts_present = !config.require_challenge_window_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.challenge_receipt_root.is_empty());
        let settlement_receipts_present = !config.require_settlement_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.settlement_receipt_root.is_empty());
        let reserve_receipts_present = !config.require_reserve_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.reserve_receipt_root.is_empty());
        let pq_verification_receipts_present = !config.require_pq_verification_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.pq_verification_receipt_root.is_empty());
        let privacy_receipts_present = !config.require_privacy_receipts
            || receipts
                .iter()
                .all(|receipt| !receipt.privacy_receipt_root.is_empty());
        let fail_closed_receipts_present = receipts
            .iter()
            .all(|receipt| !receipt.fail_closed_receipt_root.is_empty());
        let release_hold_receipts_present = receipts
            .iter()
            .all(|receipt| !receipt.release_hold_receipt_root.is_empty());
        let package_execution_observed = execution_receipt_count >= config.min_execution_receipts
            && execution_receipt_count == package_action_count
            && observed_receipt_count >= config.min_execution_receipts
            && deferred_receipt_count == 0
            && release_held_count == 0
            && fail_closed_count == 0
            && package_submit_ready
            && operator_independent_count == execution_receipt_count
            && wallet_submission_receipts_present
            && challenge_window_receipts_present
            && settlement_receipts_present
            && reserve_receipts_present
            && pq_verification_receipts_present
            && privacy_receipts_present
            && fail_closed_receipts_present
            && release_hold_receipts_present;
        let user_escape_execution_observed = package_execution_observed
            && user_escape_receipt_count >= config.min_user_escape_receipts
            && user_release_blocker_count == 0;
        let production_blocked = source.package_production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_execution_observed && !package_execution_observed);
        let execution_status = if fail_closed_count > 0 {
            "fail_closed"
        } else if release_held_count > 0 {
            "release_held"
        } else if deferred_receipt_count > 0 {
            "execution_deferred_until_live_submission"
        } else if package_execution_observed {
            "execution_observed"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_execution_observed {
            "user can force exit with observed operator-independent execution receipts"
        } else {
            "force-exit execution receipts are structured but remain deferred until live package submission, challenge, settlement, reserve, PQ, and privacy receipts are observed"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains blocked until operator-independent force-exit execution receipts are live-observed"
        } else {
            "bounded bridge/exit force-exit package execution is observed for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            execution_receipt_count,
            observed_receipt_count,
            deferred_receipt_count,
            release_held_count,
            fail_closed_count,
            user_release_blocker_count,
            production_blocker_count,
            package_execution_observed,
            user_escape_execution_observed,
            production_blocked,
            &execution_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            execution_receipt_count,
            observed_receipt_count,
            deferred_receipt_count,
            release_held_count,
            fail_closed_count,
            user_escape_receipt_count,
            operator_independent_count,
            user_release_blocker_count,
            production_blocker_count,
            package_action_count,
            package_submit_ready,
            wallet_submission_receipts_present,
            challenge_window_receipts_present,
            settlement_receipts_present,
            reserve_receipts_present,
            pq_verification_receipts_present,
            privacy_receipts_present,
            fail_closed_receipts_present,
            release_hold_receipts_present,
            package_execution_observed,
            user_escape_execution_observed,
            production_blocked,
            execution_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "user_escape_receipt_count": self.user_escape_receipt_count,
            "operator_independent_count": self.operator_independent_count,
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
            "production_blocked": self.production_blocked,
            "execution_status": self.execution_status,
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
    pub execution_receipts: Vec<ForceExitExecutionReceipt>,
    pub verdict: ExecutionReceiptVerdict,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub challenge_settlement_bundle_root: String,
    pub pq_privacy_receipt_root: String,
    pub recovery_receipt_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, package_state: package::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_package(&package_state);
        validate_source(&source)?;
        let execution_receipts = package_state
            .package_actions
            .iter()
            .map(|action| ForceExitExecutionReceipt::devnet(&config, &source, action))
            .collect::<Vec<_>>();
        let verdict = ExecutionReceiptVerdict::new(&config, &source, &execution_receipts);
        let execution_receipt_root = execution_receipt_vector_root(&execution_receipts);
        let observed_submission_bundle_root =
            observed_submission_bundle_root(&config, &source, &execution_receipts, &verdict);
        let challenge_settlement_bundle_root =
            challenge_settlement_bundle_root(&config, &source, &execution_receipts, &verdict);
        let pq_privacy_receipt_root =
            pq_privacy_receipt_root(&config, &source, &execution_receipts, &verdict);
        let recovery_receipt_root =
            recovery_receipt_root(&config, &source, &execution_receipts, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &execution_receipts, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &execution_receipt_root,
            &observed_submission_bundle_root,
            &challenge_settlement_bundle_root,
            &pq_privacy_receipt_root,
            &recovery_receipt_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            execution_receipts,
            verdict,
            execution_receipt_root,
            observed_submission_bundle_root,
            challenge_settlement_bundle_root,
            pq_privacy_receipt_root,
            recovery_receipt_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), package::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "pq_privacy_receipt_root": self.pq_privacy_receipt_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "execution_receipts": self
                .execution_receipts
                .iter()
                .map(ForceExitExecutionReceipt::public_record)
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

fn execution_status(
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
) -> ExecutionReceiptStatus {
    match action.status {
        package::ForceExitPackageStatus::ReadyToSubmit => {
            if source.package_submit_ready {
                ExecutionReceiptStatus::Observed
            } else {
                ExecutionReceiptStatus::DeferredUntilLiveSubmission
            }
        }
        package::ForceExitPackageStatus::DeferredUntilWalletTranscriptLive => {
            ExecutionReceiptStatus::DeferredUntilLiveSubmission
        }
        package::ForceExitPackageStatus::ReleaseHeld => ExecutionReceiptStatus::ReleaseHeld,
        package::ForceExitPackageStatus::FailClosed => ExecutionReceiptStatus::FailClosed,
    }
}

fn expected_submission_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "expected-submission",
        &json!({
            "execution_receipt_suite": &config.execution_receipt_suite,
            "receipt_kind": receipt_kind.as_str(),
            "observed_receipt": receipt_kind.observed_receipt(),
            "package_action_id": &action.action_id,
            "command_root": &action.command_root,
            "wallet_submission_bundle_root": &source.wallet_submission_bundle_root,
            "operator_independent": action.operator_independent,
        }),
    )
}

fn observed_submission_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    status: ExecutionReceiptStatus,
    expected_submission_root: &str,
) -> String {
    if status == ExecutionReceiptStatus::Observed {
        expected_submission_root.to_string()
    } else {
        record_root(
            "deferred-observed-submission",
            &json!({
                "execution_receipt_suite": &config.execution_receipt_suite,
                "package_action_id": &action.action_id,
                "status": status.as_str(),
                "package_status": &source.package_status,
                "wallet_submission_bundle_root": &source.wallet_submission_bundle_root,
                "production_hold_root": &source.package_production_hold_root,
                "reason": "live force-exit package submission receipt is not yet observed",
            }),
        )
    }
}

fn challenge_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "challenge-receipt",
        &json!({
            "required": config.require_challenge_window_receipts,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "action_challenge_window_root": &action.challenge_window_root,
            "challenge_window_bundle_root": &source.challenge_window_bundle_root,
            "policy": "challenge_window_must_be_wallet_watchable_without_operator",
        }),
    )
}

fn settlement_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "settlement-receipt",
        &json!({
            "required": config.require_settlement_receipts,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "command_bundle_root": &source.command_bundle_root,
            "wallet_submission_bundle_root": &source.wallet_submission_bundle_root,
            "policy": "settlement_receipt_or_release_hold_must_be_wallet_visible",
        }),
    )
}

fn reserve_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "reserve-receipt",
        &json!({
            "required": config.require_reserve_receipts,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "reserve_fallback_root": &action.reserve_fallback_root,
            "recovery_bundle_root": &source.recovery_bundle_root,
            "policy": "reserve_or_liquidity_shortfall_keeps_wallet_recoverable",
        }),
    )
}

fn pq_verification_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "pq-verification-receipt",
        &json!({
            "required": config.require_pq_verification_receipts,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "pq_authorization_root": &action.pq_authorization_root,
            "wallet_submission_bundle_root": &source.wallet_submission_bundle_root,
            "authority_scope": "withdrawal_authorization_bridge_release_watcher_quorum",
        }),
    )
}

fn privacy_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
) -> String {
    record_root(
        "privacy-receipt",
        &json!({
            "required": config.require_privacy_receipts,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "privacy_redaction_root": &action.privacy_redaction_root,
            "wallet_submission_bundle_root": &source.wallet_submission_bundle_root,
            "metadata_policy": "roots_only_no_force_exit_linkage_export",
        }),
    )
}

fn fail_closed_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
    status: ExecutionReceiptStatus,
) -> String {
    record_root(
        "fail-closed-receipt",
        &json!({
            "execution_receipt_suite": &config.execution_receipt_suite,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "status": status.as_str(),
            "fail_closed_recovery_root": &action.fail_closed_recovery_root,
            "recovery_bundle_root": &source.recovery_bundle_root,
            "policy": "missing_or_mismatched_execution_receipt_keeps_wallet_escape_package_recoverable",
        }),
    )
}

fn release_hold_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
    status: ExecutionReceiptStatus,
) -> String {
    record_root(
        "release-hold-receipt",
        &json!({
            "execution_receipt_suite": &config.execution_receipt_suite,
            "receipt_kind": receipt_kind.as_str(),
            "package_action_id": &action.action_id,
            "status": status.as_str(),
            "action_release_hold_root": &action.release_hold_root,
            "package_production_hold_root": &source.package_production_hold_root,
            "blocks_user_release": action.blocks_user_release,
            "blocks_production": action.blocks_production,
        }),
    )
}

fn execution_receipt_root(
    config: &Config,
    source: &SourceBundle,
    action: &package::ForceExitPackageAction,
    receipt_kind: ExecutionReceiptKind,
    status: ExecutionReceiptStatus,
    expected_submission_root: &str,
    observed_submission_root: &str,
    challenge_receipt_root: &str,
    settlement_receipt_root: &str,
    reserve_receipt_root: &str,
    pq_verification_receipt_root: &str,
    privacy_receipt_root: &str,
    fail_closed_receipt_root: &str,
    release_hold_receipt_root: &str,
    blocks_user_release: bool,
    blocks_production: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPT",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.package_state_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&action.action_id),
            HashPart::Str(&action.action_root),
            HashPart::Str(expected_submission_root),
            HashPart::Str(observed_submission_root),
            HashPart::Str(challenge_receipt_root),
            HashPart::Str(settlement_receipt_root),
            HashPart::Str(reserve_receipt_root),
            HashPart::Str(pq_verification_receipt_root),
            HashPart::Str(privacy_receipt_root),
            HashPart::Str(fail_closed_receipt_root),
            HashPart::Str(release_hold_receipt_root),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
        ],
        32,
    )
}

fn receipt_id(
    receipt_kind: ExecutionReceiptKind,
    ordinal: u64,
    execution_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPT-ID",
        &[
            HashPart::Str(receipt_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(execution_receipt_root),
        ],
        16,
    )
}

fn required_outcome(
    status: ExecutionReceiptStatus,
    receipt_kind: ExecutionReceiptKind,
) -> &'static str {
    match status {
        ExecutionReceiptStatus::Observed => {
            "execution receipt is observed and can satisfy the force-exit package action"
        }
        ExecutionReceiptStatus::DeferredUntilLiveSubmission => match receipt_kind {
            ExecutionReceiptKind::BroadcastExitClaim => {
                "wait for live force-exit claim broadcast receipt"
            }
            ExecutionReceiptKind::WatchSettlementReceipt => {
                "wait for live settlement watch receipt"
            }
            ExecutionReceiptKind::PqAuthorizeClaim => {
                "wait for live PQ withdrawal authorization verification receipt"
            }
            _ => "wait for live package execution receipt",
        },
        ExecutionReceiptStatus::ReleaseHeld => {
            "release remains held and the wallet must keep monitoring or recover fail-closed"
        }
        ExecutionReceiptStatus::FailClosed => {
            "execution fails closed and the wallet preserves the recovery package"
        }
    }
}

fn execution_receipt_vector_root(receipts: &[ForceExitExecutionReceipt]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPTS",
        &receipts
            .iter()
            .map(ForceExitExecutionReceipt::public_record)
            .collect::<Vec<_>>(),
    )
}

fn observed_submission_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ForceExitExecutionReceipt],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let observed = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "observed_submission_root": &receipt.observed_submission_root,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let observed_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-OBSERVED-SUBMISSIONS",
        &observed,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-OBSERVED-SUBMISSION-BUNDLE",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.wallet_submission_bundle_root),
            HashPart::Str(&observed_root),
            HashPart::U64(verdict.observed_receipt_count),
            HashPart::U64(verdict.deferred_receipt_count),
        ],
        32,
    )
}

fn challenge_settlement_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ForceExitExecutionReceipt],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "challenge_receipt_root": &receipt.challenge_receipt_root,
                "settlement_receipt_root": &receipt.settlement_receipt_root,
                "reserve_receipt_root": &receipt.reserve_receipt_root,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CHALLENGE-SETTLEMENT",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CHALLENGE-SETTLEMENT-BUNDLE",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.challenge_window_bundle_root),
            HashPart::Str(&record_root),
            HashPart::Str(bool_str(verdict.challenge_window_receipts_present)),
            HashPart::Str(bool_str(verdict.settlement_receipts_present)),
            HashPart::Str(bool_str(verdict.reserve_receipts_present)),
        ],
        32,
    )
}

fn pq_privacy_receipt_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ForceExitExecutionReceipt],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "pq_verification_receipt_root": &receipt.pq_verification_receipt_root,
                "privacy_receipt_root": &receipt.privacy_receipt_root,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-PRIVACY-RECEIPTS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-PRIVACY-BUNDLE",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.command_bundle_root),
            HashPart::Str(&record_root),
            HashPart::Str(bool_str(verdict.pq_verification_receipts_present)),
            HashPart::Str(bool_str(verdict.privacy_receipts_present)),
        ],
        32,
    )
}

fn recovery_receipt_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ForceExitExecutionReceipt],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "fail_closed_receipt_root": &receipt.fail_closed_receipt_root,
                "release_hold_receipt_root": &receipt.release_hold_receipt_root,
                "blocks_user_release": receipt.blocks_user_release,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RECOVERY-RECEIPTS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RECOVERY-RECEIPT-BUNDLE",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.recovery_bundle_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.fail_closed_count),
            HashPart::U64(verdict.user_release_blocker_count),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[ForceExitExecutionReceipt],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let blockers = receipts
        .iter()
        .filter(|receipt| receipt.blocks_production)
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "status": receipt.status.as_str(),
                "release_hold_receipt_root": &receipt.release_hold_receipt_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.package_production_hold_root),
            HashPart::Str(&blocker_root),
            HashPart::U64(verdict.production_blocker_count),
            HashPart::Str(bool_str(verdict.production_blocked)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    execution_receipt_root: &str,
    observed_submission_bundle_root: &str,
    challenge_settlement_bundle_root: &str,
    pq_privacy_receipt_root: &str,
    recovery_receipt_root: &str,
    production_hold_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPT-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(execution_receipt_root),
            HashPart::Str(observed_submission_bundle_root),
            HashPart::Str(challenge_settlement_bundle_root),
            HashPart::Str(pq_privacy_receipt_root),
            HashPart::Str(recovery_receipt_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    execution_receipt_count: u64,
    observed_receipt_count: u64,
    deferred_receipt_count: u64,
    release_held_count: u64,
    fail_closed_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    package_execution_observed: bool,
    user_escape_execution_observed: bool,
    production_blocked: bool,
    execution_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPT-VERDICT",
        &[
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.package_state_root),
            HashPart::Str(&source.command_bundle_root),
            HashPart::U64(execution_receipt_count),
            HashPart::U64(observed_receipt_count),
            HashPart::U64(deferred_receipt_count),
            HashPart::U64(release_held_count),
            HashPart::U64(fail_closed_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(package_execution_observed)),
            HashPart::Str(bool_str(user_escape_execution_observed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(execution_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(receipts: &[ForceExitExecutionReceipt], status: ExecutionReceiptStatus) -> u64 {
    receipts
        .iter()
        .filter(|receipt| receipt.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "force-exit execution receipt chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "force-exit execution receipt protocol mismatch",
    )?;
    ensure(
        config.min_execution_receipts > 0,
        "force-exit execution receipt requires receipts",
    )?;
    ensure(
        config.min_user_escape_receipts > 0,
        "force-exit execution receipt requires user escape receipts",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.package_state_root.is_empty(),
        "force-exit execution receipt missing package state root",
    )?;
    ensure(
        !source.wallet_submission_bundle_root.is_empty(),
        "force-exit execution receipt missing wallet submission bundle root",
    )?;
    ensure(
        source.package_action_count > 0,
        "force-exit execution receipt missing package actions",
    )?;
    Ok(())
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle {
        package_state_root: record_root("fallback-package-state", &json!({"reason": &reason})),
        package_action_root: record_root("fallback-package-action", &json!({"reason": &reason})),
        command_bundle_root: record_root("fallback-command", &json!({"reason": &reason})),
        wallet_submission_bundle_root: record_root(
            "fallback-wallet-submission",
            &json!({"reason": &reason}),
        ),
        challenge_window_bundle_root: record_root(
            "fallback-challenge-window",
            &json!({"reason": &reason}),
        ),
        recovery_bundle_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        package_production_hold_root: record_root(
            "fallback-production-hold",
            &json!({"reason": &reason}),
        ),
        package_status: "fallback".to_string(),
        package_user_escape_answer: reason.clone(),
        package_production_answer: "fallback".to_string(),
        package_action_count: 1,
        ready_action_count: 0,
        deferred_action_count: 0,
        release_held_count: 0,
        fail_closed_count: 1,
        wallet_visible_count: 1,
        operator_independent_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        package_submit_ready: false,
        package_production_blocked: true,
    };
    let fallback_action = package::ForceExitPackageAction::devnet(
        &package::Config::default(),
        &package::SourceBundle {
            transcript_state_root: source.package_state_root.clone(),
            transcript_entry_root: source.package_action_root.clone(),
            wallet_evidence_root: source.wallet_submission_bundle_root.clone(),
            private_scan_bundle_root: source.command_bundle_root.clone(),
            user_escape_package_root: source.command_bundle_root.clone(),
            transcript_production_hold_root: source.package_production_hold_root.clone(),
            transcript_status: "fallback".to_string(),
            transcript_user_escape_answer: reason.clone(),
            transcript_production_answer: "fallback".to_string(),
            transcript_entry_count: 1,
            wallet_ready_count: 0,
            deferred_entry_count: 0,
            fail_closed_count: 1,
            user_release_blocker_count: 1,
            production_blocker_count: 1,
            wallet_escape_transcript_complete: false,
            user_escape_answerable: false,
            transcript_production_blocked: true,
        },
        package::ForceExitActionKind::FailClosedRecovery,
        1,
    );
    let execution_receipts = vec![ForceExitExecutionReceipt::devnet(
        &config,
        &source,
        &fallback_action,
    )];
    let verdict = ExecutionReceiptVerdict::new(&config, &source, &execution_receipts);
    let execution_receipt_root = execution_receipt_vector_root(&execution_receipts);
    let observed_submission_bundle_root =
        observed_submission_bundle_root(&config, &source, &execution_receipts, &verdict);
    let challenge_settlement_bundle_root =
        challenge_settlement_bundle_root(&config, &source, &execution_receipts, &verdict);
    let pq_privacy_receipt_root =
        pq_privacy_receipt_root(&config, &source, &execution_receipts, &verdict);
    let recovery_receipt_root =
        recovery_receipt_root(&config, &source, &execution_receipts, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &execution_receipts, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &execution_receipt_root,
        &observed_submission_bundle_root,
        &challenge_settlement_bundle_root,
        &pq_privacy_receipt_root,
        &recovery_receipt_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        execution_receipts,
        verdict,
        execution_receipt_root,
        observed_submission_bundle_root,
        challenge_settlement_bundle_root,
        pq_privacy_receipt_root,
        recovery_receipt_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-EXECUTION-RECEIPT-RECORD",
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
