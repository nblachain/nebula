use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletScanReceiptObserverRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_SCAN_RECEIPT_OBSERVER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-scan-receipt-observer-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_SCAN_RECEIPT_OBSERVER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WALLET_SCAN_RECEIPT_OBSERVER_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-wallet-scan-receipt-observer-v1";
pub const DEFAULT_MIN_SCAN_RECEIPTS: u64 = 7;
pub const DEFAULT_MIN_READY_OBSERVATIONS: u64 = 5;
pub const DEFAULT_NULLIFIER_FENCE_COUNT: u64 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub scan_receipt_suite: String,
    pub min_scan_receipts: u64,
    pub min_ready_observations: u64,
    pub nullifier_fence_count: u64,
    pub require_encrypted_scan_bundle: bool,
    pub require_nullifier_fence: bool,
    pub require_metadata_redaction: bool,
    pub require_user_escape_readiness: bool,
    pub hold_production_until_wallet_observed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            scan_receipt_suite: WALLET_SCAN_RECEIPT_OBSERVER_SUITE.to_string(),
            min_scan_receipts: DEFAULT_MIN_SCAN_RECEIPTS,
            min_ready_observations: DEFAULT_MIN_READY_OBSERVATIONS,
            nullifier_fence_count: DEFAULT_NULLIFIER_FENCE_COUNT,
            require_encrypted_scan_bundle: true,
            require_nullifier_fence: true,
            require_metadata_redaction: true,
            require_user_escape_readiness: true,
            hold_production_until_wallet_observed: true,
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
            "scan_receipt_suite": self.scan_receipt_suite,
            "min_scan_receipts": self.min_scan_receipts,
            "min_ready_observations": self.min_ready_observations,
            "nullifier_fence_count": self.nullifier_fence_count,
            "require_encrypted_scan_bundle": self.require_encrypted_scan_bundle,
            "require_nullifier_fence": self.require_nullifier_fence,
            "require_metadata_redaction": self.require_metadata_redaction,
            "require_user_escape_readiness": self.require_user_escape_readiness,
            "hold_production_until_wallet_observed": self.hold_production_until_wallet_observed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletScanReceiptKind {
    WalletTranscriptObserved,
    EvidenceBundleScanned,
    PqAuthorizationScanned,
    ClaimBroadcastObserved,
    SettlementWatchObserved,
    FailClosedRecoveryScanned,
    ReleaseHoldScanned,
}

impl WalletScanReceiptKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::WalletTranscriptObserved,
            Self::EvidenceBundleScanned,
            Self::PqAuthorizationScanned,
            Self::ClaimBroadcastObserved,
            Self::SettlementWatchObserved,
            Self::FailClosedRecoveryScanned,
            Self::ReleaseHoldScanned,
        ]
    }

    pub fn from_execution(kind: execution::ExecutionReceiptKind) -> Option<Self> {
        match kind {
            execution::ExecutionReceiptKind::IntakeWalletTranscript => {
                Some(Self::WalletTranscriptObserved)
            }
            execution::ExecutionReceiptKind::BuildEvidenceBundle => {
                Some(Self::EvidenceBundleScanned)
            }
            execution::ExecutionReceiptKind::PqAuthorizeClaim => Some(Self::PqAuthorizationScanned),
            execution::ExecutionReceiptKind::BroadcastExitClaim => {
                Some(Self::ClaimBroadcastObserved)
            }
            execution::ExecutionReceiptKind::WatchSettlementReceipt => {
                Some(Self::SettlementWatchObserved)
            }
            execution::ExecutionReceiptKind::FailClosedRecovery => {
                Some(Self::FailClosedRecoveryScanned)
            }
            execution::ExecutionReceiptKind::ReportReleaseHold => Some(Self::ReleaseHoldScanned),
            execution::ExecutionReceiptKind::BindChallengeWindow
            | execution::ExecutionReceiptKind::AttachReserveFallback => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletTranscriptObserved => "wallet_transcript_observed",
            Self::EvidenceBundleScanned => "evidence_bundle_scanned",
            Self::PqAuthorizationScanned => "pq_authorization_scanned",
            Self::ClaimBroadcastObserved => "claim_broadcast_observed",
            Self::SettlementWatchObserved => "settlement_watch_observed",
            Self::FailClosedRecoveryScanned => "fail_closed_recovery_scanned",
            Self::ReleaseHoldScanned => "release_hold_scanned",
        }
    }

    pub fn escape_critical(self) -> bool {
        matches!(
            self,
            Self::WalletTranscriptObserved
                | Self::EvidenceBundleScanned
                | Self::PqAuthorizationScanned
                | Self::ClaimBroadcastObserved
                | Self::SettlementWatchObserved
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletScanReceiptStatus {
    Ready,
    PendingLiveObservation,
    ReleaseHeld,
    FailClosedRecoverable,
}

impl WalletScanReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PendingLiveObservation => "pending_live_observation",
            Self::ReleaseHeld => "release_held",
            Self::FailClosedRecoverable => "fail_closed_recoverable",
        }
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
    pub production_hold_root: String,
    pub execution_status: String,
    pub execution_user_escape_answer: String,
    pub execution_production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub user_escape_receipt_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
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
            production_hold_root: state.production_hold_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            execution_user_escape_answer: state.verdict.user_escape_answer.clone(),
            execution_production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            user_escape_receipt_count: state.verdict.user_escape_receipt_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            package_execution_observed: state.verdict.package_execution_observed,
            user_escape_execution_observed: state.verdict.user_escape_execution_observed,
            execution_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let state = execution::devnet();
        Self::from_execution(&state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "pq_privacy_receipt_root": self.pq_privacy_receipt_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "production_hold_root": self.production_hold_root,
            "execution_status": self.execution_status,
            "execution_user_escape_answer": self.execution_user_escape_answer,
            "execution_production_answer": self.execution_production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "user_escape_receipt_count": self.user_escape_receipt_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletScanReceipt {
    pub receipt_id: String,
    pub ordinal: u64,
    pub receipt_kind: WalletScanReceiptKind,
    pub execution_receipt_id: String,
    pub execution_receipt_root: String,
    pub encrypted_scan_bundle_root: String,
    pub nullifier_fence_root: String,
    pub metadata_redaction_root: String,
    pub observer_commitment_root: String,
    pub linkage_blind_root: String,
    pub readiness_signal_root: String,
    pub scan_receipt_root: String,
    pub status: WalletScanReceiptStatus,
    pub wallet_visible: bool,
    pub escape_critical: bool,
    pub nullifier_fenced: bool,
    pub metadata_redacted: bool,
    pub linkage_hidden: bool,
    pub user_escape_ready: bool,
    pub observer_note: String,
}

impl WalletScanReceipt {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        receipt: &execution::ForceExitExecutionReceipt,
    ) -> Option<Self> {
        let receipt_kind = WalletScanReceiptKind::from_execution(receipt.receipt_kind)?;
        let status = scan_status(receipt);
        let encrypted_scan_bundle_root =
            encrypted_scan_bundle_root(config, source, receipt, receipt_kind);
        let nullifier_fence_root = nullifier_fence_root(
            config,
            source,
            receipt,
            receipt_kind,
            &encrypted_scan_bundle_root,
        );
        let metadata_redaction_root =
            metadata_redaction_root(config, source, receipt, receipt_kind);
        let observer_commitment_root = observer_commitment_root(
            config,
            source,
            receipt,
            receipt_kind,
            &encrypted_scan_bundle_root,
            &nullifier_fence_root,
            &metadata_redaction_root,
        );
        let linkage_blind_root = linkage_blind_root(
            config,
            source,
            receipt,
            receipt_kind,
            &observer_commitment_root,
        );
        let escape_critical = receipt_kind.escape_critical();
        let user_escape_ready = status == WalletScanReceiptStatus::Ready
            && receipt.user_escape_receipt
            && !receipt.blocks_user_release;
        let readiness_signal_root = readiness_signal_root(
            config,
            source,
            receipt,
            receipt_kind,
            status,
            &linkage_blind_root,
            user_escape_ready,
        );
        let scan_receipt_root = scan_receipt_root(
            config,
            source,
            receipt,
            receipt_kind,
            status,
            &encrypted_scan_bundle_root,
            &nullifier_fence_root,
            &metadata_redaction_root,
            &observer_commitment_root,
            &linkage_blind_root,
            &readiness_signal_root,
            user_escape_ready,
        );
        let receipt_id = receipt_id(receipt_kind, receipt.ordinal, &scan_receipt_root);
        Some(Self {
            receipt_id,
            ordinal: receipt.ordinal,
            receipt_kind,
            execution_receipt_id: receipt.receipt_id.clone(),
            execution_receipt_root: receipt.execution_receipt_root.clone(),
            encrypted_scan_bundle_root,
            nullifier_fence_root,
            metadata_redaction_root,
            observer_commitment_root,
            linkage_blind_root,
            readiness_signal_root,
            scan_receipt_root,
            status,
            wallet_visible: true,
            escape_critical,
            nullifier_fenced: true,
            metadata_redacted: true,
            linkage_hidden: true,
            user_escape_ready,
            observer_note: observer_note(status, receipt_kind).to_string(),
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "ordinal": self.ordinal,
            "receipt_kind": self.receipt_kind.as_str(),
            "execution_receipt_id": self.execution_receipt_id,
            "execution_receipt_root": self.execution_receipt_root,
            "encrypted_scan_bundle_root": self.encrypted_scan_bundle_root,
            "nullifier_fence_root": self.nullifier_fence_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "observer_commitment_root": self.observer_commitment_root,
            "linkage_blind_root": self.linkage_blind_root,
            "readiness_signal_root": self.readiness_signal_root,
            "scan_receipt_root": self.scan_receipt_root,
            "status": self.status.as_str(),
            "wallet_visible": self.wallet_visible,
            "escape_critical": self.escape_critical,
            "nullifier_fenced": self.nullifier_fenced,
            "metadata_redacted": self.metadata_redacted,
            "linkage_hidden": self.linkage_hidden,
            "user_escape_ready": self.user_escape_ready,
            "observer_note": self.observer_note,
        })
    }

    pub fn state_root(&self) -> String {
        self.scan_receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletScanVerdict {
    pub scan_receipt_count: u64,
    pub ready_scan_count: u64,
    pub pending_scan_count: u64,
    pub release_held_count: u64,
    pub fail_closed_recoverable_count: u64,
    pub escape_critical_count: u64,
    pub user_escape_ready_count: u64,
    pub nullifier_fenced_count: u64,
    pub metadata_redacted_count: u64,
    pub linkage_hidden_count: u64,
    pub encrypted_scan_bundle_present: bool,
    pub nullifier_fence_complete: bool,
    pub metadata_redaction_complete: bool,
    pub linkage_privacy_preserved: bool,
    pub wallet_observer_complete: bool,
    pub user_escape_ready: bool,
    pub production_blocked: bool,
    pub scan_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl WalletScanVerdict {
    pub fn new(config: &Config, source: &SourceBundle, receipts: &[WalletScanReceipt]) -> Self {
        let scan_receipt_count = receipts.len() as u64;
        let ready_scan_count = count_status(receipts, WalletScanReceiptStatus::Ready);
        let pending_scan_count =
            count_status(receipts, WalletScanReceiptStatus::PendingLiveObservation);
        let release_held_count = count_status(receipts, WalletScanReceiptStatus::ReleaseHeld);
        let fail_closed_recoverable_count =
            count_status(receipts, WalletScanReceiptStatus::FailClosedRecoverable);
        let escape_critical_count = receipts
            .iter()
            .filter(|receipt| receipt.escape_critical)
            .count() as u64;
        let user_escape_ready_count = receipts
            .iter()
            .filter(|receipt| receipt.user_escape_ready)
            .count() as u64;
        let nullifier_fenced_count = receipts
            .iter()
            .filter(|receipt| receipt.nullifier_fenced)
            .count() as u64;
        let metadata_redacted_count = receipts
            .iter()
            .filter(|receipt| receipt.metadata_redacted)
            .count() as u64;
        let linkage_hidden_count = receipts
            .iter()
            .filter(|receipt| receipt.linkage_hidden)
            .count() as u64;
        let encrypted_scan_bundle_present = scan_receipt_count >= config.min_scan_receipts
            || (!config.require_encrypted_scan_bundle && scan_receipt_count > 0);
        let nullifier_fence_complete = nullifier_fenced_count >= config.nullifier_fence_count
            || !config.require_nullifier_fence;
        let metadata_redaction_complete =
            metadata_redacted_count == scan_receipt_count || !config.require_metadata_redaction;
        let linkage_privacy_preserved = linkage_hidden_count == scan_receipt_count
            && metadata_redaction_complete
            && nullifier_fence_complete;
        let wallet_observer_complete = encrypted_scan_bundle_present
            && nullifier_fence_complete
            && metadata_redaction_complete
            && linkage_privacy_preserved;
        let user_escape_ready = wallet_observer_complete
            && user_escape_ready_count >= config.min_ready_observations
            && source.user_escape_execution_observed;
        let production_blocked = source.execution_production_blocked
            || release_held_count > 0
            || fail_closed_recoverable_count > 0
            || (config.hold_production_until_wallet_observed && !wallet_observer_complete);
        let scan_status = if fail_closed_recoverable_count > 0 {
            "fail_closed_recoverable"
        } else if release_held_count > 0 {
            "release_held"
        } else if pending_scan_count > 0 {
            "pending_live_observation"
        } else if wallet_observer_complete {
            "wallet_scan_observed"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_ready {
            "wallet can prepare force-exit escape from encrypted scan receipts without revealing linkage"
        } else {
            "wallet scan receipts are not yet sufficient for linkage-safe force-exit readiness"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains blocked until wallet-visible force-exit scan observations are complete"
        } else {
            "wallet scan observer has redacted metadata and fenced nullifiers for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            scan_receipt_count,
            ready_scan_count,
            pending_scan_count,
            release_held_count,
            fail_closed_recoverable_count,
            user_escape_ready_count,
            nullifier_fenced_count,
            metadata_redacted_count,
            linkage_hidden_count,
            wallet_observer_complete,
            user_escape_ready,
            production_blocked,
            &scan_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            scan_receipt_count,
            ready_scan_count,
            pending_scan_count,
            release_held_count,
            fail_closed_recoverable_count,
            escape_critical_count,
            user_escape_ready_count,
            nullifier_fenced_count,
            metadata_redacted_count,
            linkage_hidden_count,
            encrypted_scan_bundle_present,
            nullifier_fence_complete,
            metadata_redaction_complete,
            linkage_privacy_preserved,
            wallet_observer_complete,
            user_escape_ready,
            production_blocked,
            scan_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "scan_receipt_count": self.scan_receipt_count,
            "ready_scan_count": self.ready_scan_count,
            "pending_scan_count": self.pending_scan_count,
            "release_held_count": self.release_held_count,
            "fail_closed_recoverable_count": self.fail_closed_recoverable_count,
            "escape_critical_count": self.escape_critical_count,
            "user_escape_ready_count": self.user_escape_ready_count,
            "nullifier_fenced_count": self.nullifier_fenced_count,
            "metadata_redacted_count": self.metadata_redacted_count,
            "linkage_hidden_count": self.linkage_hidden_count,
            "encrypted_scan_bundle_present": self.encrypted_scan_bundle_present,
            "nullifier_fence_complete": self.nullifier_fence_complete,
            "metadata_redaction_complete": self.metadata_redaction_complete,
            "linkage_privacy_preserved": self.linkage_privacy_preserved,
            "wallet_observer_complete": self.wallet_observer_complete,
            "user_escape_ready": self.user_escape_ready,
            "production_blocked": self.production_blocked,
            "scan_status": self.scan_status,
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
    pub wallet_scan_receipts: Vec<WalletScanReceipt>,
    pub verdict: WalletScanVerdict,
    pub wallet_scan_receipt_root: String,
    pub encrypted_scan_bundle_root: String,
    pub nullifier_fence_bundle_root: String,
    pub metadata_redaction_bundle_root: String,
    pub linkage_privacy_bundle_root: String,
    pub user_escape_readiness_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution(&execution_state);
        validate_source(&source)?;
        let wallet_scan_receipts = execution_state
            .execution_receipts
            .iter()
            .filter_map(|receipt| WalletScanReceipt::devnet(&config, &source, receipt))
            .collect::<Vec<_>>();
        let verdict = WalletScanVerdict::new(&config, &source, &wallet_scan_receipts);
        let wallet_scan_receipt_root = wallet_scan_receipt_vector_root(&wallet_scan_receipts);
        let encrypted_scan_bundle_root =
            encrypted_scan_bundle_vector_root(&config, &source, &wallet_scan_receipts, &verdict);
        let nullifier_fence_bundle_root =
            nullifier_fence_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
        let metadata_redaction_bundle_root =
            metadata_redaction_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
        let linkage_privacy_bundle_root =
            linkage_privacy_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
        let user_escape_readiness_root =
            user_escape_readiness_root(&config, &source, &wallet_scan_receipts, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &wallet_scan_receipts, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &wallet_scan_receipt_root,
            &encrypted_scan_bundle_root,
            &nullifier_fence_bundle_root,
            &metadata_redaction_bundle_root,
            &linkage_privacy_bundle_root,
            &user_escape_readiness_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            wallet_scan_receipts,
            verdict,
            wallet_scan_receipt_root,
            encrypted_scan_bundle_root,
            nullifier_fence_bundle_root,
            metadata_redaction_bundle_root,
            linkage_privacy_bundle_root,
            user_escape_readiness_root,
            production_hold_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_wallet_scan_receipt_observer_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "wallet_scan_receipt_root": self.wallet_scan_receipt_root,
            "encrypted_scan_bundle_root": self.encrypted_scan_bundle_root,
            "nullifier_fence_bundle_root": self.nullifier_fence_bundle_root,
            "metadata_redaction_bundle_root": self.metadata_redaction_bundle_root,
            "linkage_privacy_bundle_root": self.linkage_privacy_bundle_root,
            "user_escape_readiness_root": self.user_escape_readiness_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "wallet_scan_receipts": self
                .wallet_scan_receipts
                .iter()
                .map(WalletScanReceipt::public_record)
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

fn scan_status(receipt: &execution::ForceExitExecutionReceipt) -> WalletScanReceiptStatus {
    match receipt.status {
        execution::ExecutionReceiptStatus::Observed => {
            if receipt.blocks_user_release {
                WalletScanReceiptStatus::ReleaseHeld
            } else {
                WalletScanReceiptStatus::Ready
            }
        }
        execution::ExecutionReceiptStatus::DeferredUntilLiveSubmission => {
            WalletScanReceiptStatus::PendingLiveObservation
        }
        execution::ExecutionReceiptStatus::ReleaseHeld => WalletScanReceiptStatus::ReleaseHeld,
        execution::ExecutionReceiptStatus::FailClosed => {
            WalletScanReceiptStatus::FailClosedRecoverable
        }
    }
}

fn encrypted_scan_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
) -> String {
    record_root(
        "encrypted-scan-bundle",
        &json!({
            "scan_receipt_suite": &config.scan_receipt_suite,
            "execution_state_root": &source.execution_state_root,
            "receipt_kind": receipt_kind.as_str(),
            "execution_receipt_root": &receipt.execution_receipt_root,
            "observed_submission_root": &receipt.observed_submission_root,
            "pq_privacy_receipt_root": &source.pq_privacy_receipt_root,
            "payload": "wallet-local encrypted scan result root only",
        }),
    )
}

fn nullifier_fence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
    encrypted_scan_bundle_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-NULLIFIER-FENCE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(&receipt.execution_receipt_id),
            HashPart::Str(encrypted_scan_bundle_root),
            HashPart::Str(&receipt.privacy_receipt_root),
            HashPart::U64(config.nullifier_fence_count),
        ],
        32,
    )
}

fn metadata_redaction_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
) -> String {
    record_root(
        "metadata-redaction",
        &json!({
            "scan_receipt_suite": &config.scan_receipt_suite,
            "receipt_kind": receipt_kind.as_str(),
            "execution_receipt_root": &receipt.execution_receipt_root,
            "source_execution_root": &source.execution_state_root,
            "redacted_fields": [
                "wallet_address",
                "output_index",
                "amount",
                "view_tag",
                "scan_height",
                "counterparty_linkage"
            ],
            "public_fields": [
                "receipt_id",
                "receipt_kind",
                "roots",
                "status",
                "readiness"
            ],
        }),
    )
}

fn observer_commitment_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
    encrypted_scan_bundle_root: &str,
    nullifier_fence_root: &str,
    metadata_redaction_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-OBSERVER-COMMITMENT",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(&receipt.execution_receipt_root),
            HashPart::Str(encrypted_scan_bundle_root),
            HashPart::Str(nullifier_fence_root),
            HashPart::Str(metadata_redaction_root),
        ],
        32,
    )
}

fn linkage_blind_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
    observer_commitment_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-LINKAGE-BLIND",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.pq_privacy_receipt_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(&receipt.privacy_receipt_root),
            HashPart::Str(observer_commitment_root),
            HashPart::Str("unlinkable-wallet-visible-observation"),
        ],
        32,
    )
}

fn readiness_signal_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
    status: WalletScanReceiptStatus,
    linkage_blind_root: &str,
    user_escape_ready: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-READINESS-SIGNAL",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&receipt.execution_receipt_root),
            HashPart::Str(linkage_blind_root),
            HashPart::Str(bool_str(user_escape_ready)),
        ],
        32,
    )
}

fn scan_receipt_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    receipt_kind: WalletScanReceiptKind,
    status: WalletScanReceiptStatus,
    encrypted_scan_bundle_root: &str,
    nullifier_fence_root: &str,
    metadata_redaction_root: &str,
    observer_commitment_root: &str,
    linkage_blind_root: &str,
    readiness_signal_root: &str,
    user_escape_ready: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-RECEIPT",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&receipt.execution_receipt_id),
            HashPart::Str(&receipt.execution_receipt_root),
            HashPart::Str(encrypted_scan_bundle_root),
            HashPart::Str(nullifier_fence_root),
            HashPart::Str(metadata_redaction_root),
            HashPart::Str(observer_commitment_root),
            HashPart::Str(linkage_blind_root),
            HashPart::Str(readiness_signal_root),
            HashPart::Str(bool_str(user_escape_ready)),
        ],
        32,
    )
}

fn receipt_id(
    receipt_kind: WalletScanReceiptKind,
    ordinal: u64,
    scan_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-RECEIPT-ID",
        &[
            HashPart::Str(receipt_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(scan_receipt_root),
        ],
        16,
    )
}

fn observer_note(
    status: WalletScanReceiptStatus,
    receipt_kind: WalletScanReceiptKind,
) -> &'static str {
    match status {
        WalletScanReceiptStatus::Ready => match receipt_kind {
            WalletScanReceiptKind::SettlementWatchObserved => {
                "wallet observed settlement readiness without publishing linkage metadata"
            }
            WalletScanReceiptKind::ClaimBroadcastObserved => {
                "wallet observed force-exit claim broadcast through encrypted scan bundle roots"
            }
            _ => "wallet-visible force-exit observation is ready under redacted metadata roots",
        },
        WalletScanReceiptStatus::PendingLiveObservation => {
            "wallet must keep scanning until live receipt roots match the encrypted observation"
        }
        WalletScanReceiptStatus::ReleaseHeld => {
            "wallet keeps nullifier fences active while release remains held"
        }
        WalletScanReceiptStatus::FailClosedRecoverable => {
            "wallet preserves fail-closed recovery path without leaking linkage"
        }
    }
}

fn wallet_scan_receipt_vector_root(receipts: &[WalletScanReceipt]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-RECEIPTS",
        &receipts
            .iter()
            .map(WalletScanReceipt::public_record)
            .collect::<Vec<_>>(),
    )
}

fn encrypted_scan_bundle_vector_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "encrypted_scan_bundle_root": &receipt.encrypted_scan_bundle_root,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-ENCRYPTED-SCAN-BUNDLES",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-ENCRYPTED-SCAN-BUNDLE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.scan_receipt_count),
            HashPart::Str(bool_str(verdict.encrypted_scan_bundle_present)),
        ],
        32,
    )
}

fn nullifier_fence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "nullifier_fence_root": &receipt.nullifier_fence_root,
                "nullifier_fenced": receipt.nullifier_fenced,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-NULLIFIER-FENCES",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-NULLIFIER-FENCE-BUNDLE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.nullifier_fenced_count),
            HashPart::Str(bool_str(verdict.nullifier_fence_complete)),
        ],
        32,
    )
}

fn metadata_redaction_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "metadata_redaction_root": &receipt.metadata_redaction_root,
                "metadata_redacted": receipt.metadata_redacted,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-METADATA-REDACTIONS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-METADATA-REDACTION-BUNDLE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.pq_privacy_receipt_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.metadata_redacted_count),
            HashPart::Str(bool_str(verdict.metadata_redaction_complete)),
        ],
        32,
    )
}

fn linkage_privacy_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "linkage_blind_root": &receipt.linkage_blind_root,
                "linkage_hidden": receipt.linkage_hidden,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-LINKAGE-PRIVACY",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-LINKAGE-PRIVACY-BUNDLE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.linkage_hidden_count),
            HashPart::Str(bool_str(verdict.linkage_privacy_preserved)),
        ],
        32,
    )
}

fn user_escape_readiness_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let records = receipts
        .iter()
        .filter(|receipt| receipt.escape_critical)
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "readiness_signal_root": &receipt.readiness_signal_root,
                "user_escape_ready": receipt.user_escape_ready,
            })
        })
        .collect::<Vec<_>>();
    let record_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-USER-ESCAPE-READINESS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-USER-ESCAPE-READINESS-BUNDLE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&record_root),
            HashPart::U64(verdict.user_escape_ready_count),
            HashPart::Str(bool_str(verdict.user_escape_ready)),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[WalletScanReceipt],
    verdict: &WalletScanVerdict,
) -> String {
    let blockers = receipts
        .iter()
        .filter(|receipt| receipt.status != WalletScanReceiptStatus::Ready)
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "receipt_kind": receipt.receipt_kind.as_str(),
                "status": receipt.status.as_str(),
                "readiness_signal_root": &receipt.readiness_signal_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(&blocker_root),
            HashPart::U64(verdict.pending_scan_count),
            HashPart::U64(verdict.release_held_count),
            HashPart::U64(verdict.fail_closed_recoverable_count),
            HashPart::Str(bool_str(verdict.production_blocked)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    wallet_scan_receipt_root: &str,
    encrypted_scan_bundle_root: &str,
    nullifier_fence_bundle_root: &str,
    metadata_redaction_bundle_root: &str,
    linkage_privacy_bundle_root: &str,
    user_escape_readiness_root: &str,
    production_hold_root: &str,
    verdict: &WalletScanVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-OBSERVER-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(wallet_scan_receipt_root),
            HashPart::Str(encrypted_scan_bundle_root),
            HashPart::Str(nullifier_fence_bundle_root),
            HashPart::Str(metadata_redaction_bundle_root),
            HashPart::Str(linkage_privacy_bundle_root),
            HashPart::Str(user_escape_readiness_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    scan_receipt_count: u64,
    ready_scan_count: u64,
    pending_scan_count: u64,
    release_held_count: u64,
    fail_closed_recoverable_count: u64,
    user_escape_ready_count: u64,
    nullifier_fenced_count: u64,
    metadata_redacted_count: u64,
    linkage_hidden_count: u64,
    wallet_observer_complete: bool,
    user_escape_ready: bool,
    production_blocked: bool,
    scan_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-OBSERVER-VERDICT",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::U64(scan_receipt_count),
            HashPart::U64(ready_scan_count),
            HashPart::U64(pending_scan_count),
            HashPart::U64(release_held_count),
            HashPart::U64(fail_closed_recoverable_count),
            HashPart::U64(user_escape_ready_count),
            HashPart::U64(nullifier_fenced_count),
            HashPart::U64(metadata_redacted_count),
            HashPart::U64(linkage_hidden_count),
            HashPart::Str(bool_str(wallet_observer_complete)),
            HashPart::Str(bool_str(user_escape_ready)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(scan_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(receipts: &[WalletScanReceipt], status: WalletScanReceiptStatus) -> u64 {
    receipts
        .iter()
        .filter(|receipt| receipt.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "wallet scan receipt observer chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "wallet scan receipt observer protocol mismatch",
    )?;
    ensure(
        config.min_scan_receipts > 0,
        "wallet scan receipt observer requires scan receipts",
    )?;
    ensure(
        config.min_ready_observations > 0,
        "wallet scan receipt observer requires readiness observations",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_state_root.is_empty(),
        "wallet scan receipt observer missing execution state root",
    )?;
    ensure(
        !source.execution_receipt_root.is_empty(),
        "wallet scan receipt observer missing execution receipt root",
    )?;
    ensure(
        source.execution_receipt_count > 0,
        "wallet scan receipt observer missing execution receipts",
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
        execution_state_root: record_root("fallback-execution-state", &json!({"reason": &reason})),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({"reason": &reason}),
        ),
        observed_submission_bundle_root: record_root(
            "fallback-observed-submission",
            &json!({"reason": &reason}),
        ),
        challenge_settlement_bundle_root: record_root(
            "fallback-challenge-settlement",
            &json!({"reason": &reason}),
        ),
        pq_privacy_receipt_root: record_root("fallback-pq-privacy", &json!({"reason": &reason})),
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        execution_status: "fallback".to_string(),
        execution_user_escape_answer: reason.clone(),
        execution_production_answer: "fallback".to_string(),
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        user_escape_receipt_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        package_execution_observed: false,
        user_escape_execution_observed: false,
        execution_production_blocked: true,
    };
    let wallet_scan_receipts = WalletScanReceiptKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, receipt_kind)| {
            fallback_receipt(&config, &source, *receipt_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = WalletScanVerdict::new(&config, &source, &wallet_scan_receipts);
    let wallet_scan_receipt_root = wallet_scan_receipt_vector_root(&wallet_scan_receipts);
    let encrypted_scan_bundle_root =
        encrypted_scan_bundle_vector_root(&config, &source, &wallet_scan_receipts, &verdict);
    let nullifier_fence_bundle_root =
        nullifier_fence_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
    let metadata_redaction_bundle_root =
        metadata_redaction_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
    let linkage_privacy_bundle_root =
        linkage_privacy_bundle_root(&config, &source, &wallet_scan_receipts, &verdict);
    let user_escape_readiness_root =
        user_escape_readiness_root(&config, &source, &wallet_scan_receipts, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &wallet_scan_receipts, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &wallet_scan_receipt_root,
        &encrypted_scan_bundle_root,
        &nullifier_fence_bundle_root,
        &metadata_redaction_bundle_root,
        &linkage_privacy_bundle_root,
        &user_escape_readiness_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        wallet_scan_receipts,
        verdict,
        wallet_scan_receipt_root,
        encrypted_scan_bundle_root,
        nullifier_fence_bundle_root,
        metadata_redaction_bundle_root,
        linkage_privacy_bundle_root,
        user_escape_readiness_root,
        production_hold_root,
        state_commitment_root,
    }
}

fn fallback_receipt(
    config: &Config,
    source: &SourceBundle,
    receipt_kind: WalletScanReceiptKind,
    ordinal: u64,
) -> WalletScanReceipt {
    let execution_receipt_id = record_root(
        "fallback-execution-receipt-id",
        &json!({"receipt_kind": receipt_kind.as_str(), "ordinal": ordinal}),
    );
    let execution_receipt_root = record_root(
        "fallback-execution-receipt-root",
        &json!({"receipt_kind": receipt_kind.as_str(), "ordinal": ordinal}),
    );
    let encrypted_scan_bundle_root = record_root(
        "fallback-encrypted-scan-bundle",
        &json!({"receipt_kind": receipt_kind.as_str(), "ordinal": ordinal}),
    );
    let nullifier_fence_root = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-FALLBACK-NULLIFIER-FENCE",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::Str(&encrypted_scan_bundle_root),
            HashPart::U64(ordinal),
        ],
        32,
    );
    let metadata_redaction_root = record_root(
        "fallback-metadata-redaction",
        &json!({"receipt_kind": receipt_kind.as_str(), "ordinal": ordinal}),
    );
    let observer_commitment_root = record_root(
        "fallback-observer-commitment",
        &json!({
            "receipt_kind": receipt_kind.as_str(),
            "encrypted_scan_bundle_root": &encrypted_scan_bundle_root,
            "nullifier_fence_root": &nullifier_fence_root,
            "metadata_redaction_root": &metadata_redaction_root,
        }),
    );
    let linkage_blind_root = record_root(
        "fallback-linkage-blind",
        &json!({
            "receipt_kind": receipt_kind.as_str(),
            "observer_commitment_root": &observer_commitment_root,
        }),
    );
    let readiness_signal_root = record_root(
        "fallback-readiness-signal",
        &json!({
            "receipt_kind": receipt_kind.as_str(),
            "linkage_blind_root": &linkage_blind_root,
            "user_escape_ready": false,
        }),
    );
    let scan_receipt_root = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-FALLBACK-RECEIPT",
        &[
            HashPart::Str(&config.scan_receipt_suite),
            HashPart::Str(receipt_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(&execution_receipt_root),
            HashPart::Str(&encrypted_scan_bundle_root),
            HashPart::Str(&nullifier_fence_root),
            HashPart::Str(&metadata_redaction_root),
            HashPart::Str(&linkage_blind_root),
        ],
        32,
    );
    let receipt_id = receipt_id(receipt_kind, ordinal, &scan_receipt_root);
    WalletScanReceipt {
        receipt_id,
        ordinal,
        receipt_kind,
        execution_receipt_id,
        execution_receipt_root,
        encrypted_scan_bundle_root,
        nullifier_fence_root,
        metadata_redaction_root,
        observer_commitment_root,
        linkage_blind_root,
        readiness_signal_root,
        scan_receipt_root,
        status: WalletScanReceiptStatus::FailClosedRecoverable,
        wallet_visible: true,
        escape_critical: receipt_kind.escape_critical(),
        nullifier_fenced: true,
        metadata_redacted: true,
        linkage_hidden: true,
        user_escape_ready: false,
        observer_note: "fallback scan receipt preserves recovery without revealing linkage"
            .to_string(),
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-SCAN-OBSERVER-RECORD",
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
