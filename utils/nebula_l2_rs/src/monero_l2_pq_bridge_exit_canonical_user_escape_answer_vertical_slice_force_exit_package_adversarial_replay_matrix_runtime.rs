use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageAdversarialReplayMatrixRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_ADVERSARIAL_REPLAY_MATRIX_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-adversarial-replay-matrix-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_ADVERSARIAL_REPLAY_MATRIX_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ADVERSARIAL_REPLAY_MATRIX_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-adversarial-replay-matrix-v1";
pub const DEFAULT_MIN_REPLAY_CASES: u64 = 7;
pub const DEFAULT_CURRENT_PQ_EPOCH: u64 = 88;
pub const DEFAULT_MAX_REORG_DEPTH: u64 = 6;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_RESERVE_UNIT: u64 = 1_000_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub replay_matrix_suite: String,
    pub min_replay_cases: u64,
    pub current_pq_epoch: u64,
    pub max_pq_epoch_lag: u64,
    pub max_reorg_depth: u64,
    pub challenge_window_blocks: u64,
    pub reserve_unit: u64,
    pub require_wallet_visible_blockers: bool,
    pub require_metadata_privacy: bool,
    pub require_settlement_binding: bool,
    pub require_reserve_coverage: bool,
    pub fail_closed_on_any_adversarial_replay: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_matrix_suite: ADVERSARIAL_REPLAY_MATRIX_SUITE.to_string(),
            min_replay_cases: DEFAULT_MIN_REPLAY_CASES,
            current_pq_epoch: DEFAULT_CURRENT_PQ_EPOCH,
            max_pq_epoch_lag: 1,
            max_reorg_depth: DEFAULT_MAX_REORG_DEPTH,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            reserve_unit: DEFAULT_RESERVE_UNIT,
            require_wallet_visible_blockers: true,
            require_metadata_privacy: true,
            require_settlement_binding: true,
            require_reserve_coverage: true,
            fail_closed_on_any_adversarial_replay: true,
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
            "replay_matrix_suite": self.replay_matrix_suite,
            "min_replay_cases": self.min_replay_cases,
            "current_pq_epoch": self.current_pq_epoch,
            "max_pq_epoch_lag": self.max_pq_epoch_lag,
            "max_reorg_depth": self.max_reorg_depth,
            "challenge_window_blocks": self.challenge_window_blocks,
            "reserve_unit": self.reserve_unit,
            "require_wallet_visible_blockers": self.require_wallet_visible_blockers,
            "require_metadata_privacy": self.require_metadata_privacy,
            "require_settlement_binding": self.require_settlement_binding,
            "require_reserve_coverage": self.require_reserve_coverage,
            "fail_closed_on_any_adversarial_replay": self.fail_closed_on_any_adversarial_replay,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_state_root: String,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub challenge_settlement_bundle_root: String,
    pub recovery_receipt_root: String,
    pub production_hold_root: String,
    pub package_state_root: String,
    pub execution_status: String,
    pub execution_user_escape_answer: String,
    pub execution_production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub production_blocker_count: u64,
    pub reserve_receipts_present: bool,
    pub settlement_receipts_present: bool,
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
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            package_state_root: state.source.package_state_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            execution_user_escape_answer: state.verdict.user_escape_answer.clone(),
            execution_production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            deferred_receipt_count: state.verdict.deferred_receipt_count,
            release_held_count: state.verdict.release_held_count,
            fail_closed_count: state.verdict.fail_closed_count,
            production_blocker_count: state.verdict.production_blocker_count,
            reserve_receipts_present: state.verdict.reserve_receipts_present,
            settlement_receipts_present: state.verdict.settlement_receipts_present,
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
            "recovery_receipt_root": self.recovery_receipt_root,
            "production_hold_root": self.production_hold_root,
            "package_state_root": self.package_state_root,
            "execution_status": self.execution_status,
            "execution_user_escape_answer": self.execution_user_escape_answer,
            "execution_production_answer": self.execution_production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "production_blocker_count": self.production_blocker_count,
            "reserve_receipts_present": self.reserve_receipts_present,
            "settlement_receipts_present": self.settlement_receipts_present,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdversarialReplayKind {
    StalePqEpoch,
    WatcherCollusion,
    ReorgReplay,
    ReserveShortfall,
    SettlementMismatch,
    MetadataLeak,
    BlockedChallengeWindow,
}

impl AdversarialReplayKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::StalePqEpoch,
            Self::WatcherCollusion,
            Self::ReorgReplay,
            Self::ReserveShortfall,
            Self::SettlementMismatch,
            Self::MetadataLeak,
            Self::BlockedChallengeWindow,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::StalePqEpoch => "stale_pq_epoch",
            Self::WatcherCollusion => "watcher_collusion",
            Self::ReorgReplay => "reorg_replay",
            Self::ReserveShortfall => "reserve_shortfall",
            Self::SettlementMismatch => "settlement_mismatch",
            Self::MetadataLeak => "metadata_leak",
            Self::BlockedChallengeWindow => "blocked_challenge_window",
        }
    }

    pub fn wallet_blocker(self) -> &'static str {
        match self {
            Self::StalePqEpoch => "wallet_blocker_stale_pq_authority_epoch",
            Self::WatcherCollusion => "wallet_blocker_colluding_watcher_quorum",
            Self::ReorgReplay => "wallet_blocker_reorg_replayed_force_exit_claim",
            Self::ReserveShortfall => "wallet_blocker_reserve_shortfall",
            Self::SettlementMismatch => "wallet_blocker_settlement_mismatch",
            Self::MetadataLeak => "wallet_blocker_private_metadata_leak",
            Self::BlockedChallengeWindow => "wallet_blocker_challenge_window_blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdversarialReplayStatus {
    Rejected,
    FailClosedBlocked,
    NeedsWalletEscalation,
}

impl AdversarialReplayStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rejected => "rejected",
            Self::FailClosedBlocked => "fail_closed_blocked",
            Self::NeedsWalletEscalation => "needs_wallet_escalation",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdversarialReplayCaseRecord {
    pub case_id: String,
    pub ordinal: u64,
    pub replay_kind: AdversarialReplayKind,
    pub hostile_input_root: String,
    pub expected_evidence_root: String,
    pub replay_detection_root: String,
    pub wallet_blocker_root: String,
    pub stale_pq_epoch: bool,
    pub watcher_collusion: bool,
    pub reorg_detected: bool,
    pub reserve_shortfall: bool,
    pub settlement_mismatch: bool,
    pub metadata_leak: bool,
    pub challenge_window_blocked: bool,
    pub wallet_visible: bool,
    pub production_blocked: bool,
    pub user_release_blocked: bool,
    pub status: AdversarialReplayStatus,
    pub required_outcome: String,
    pub case_root: String,
}

impl AdversarialReplayCaseRecord {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        replay_kind: AdversarialReplayKind,
    ) -> Self {
        let ordinal = case_ordinal(replay_kind);
        let stale_pq_epoch = replay_kind == AdversarialReplayKind::StalePqEpoch;
        let watcher_collusion = replay_kind == AdversarialReplayKind::WatcherCollusion;
        let reorg_detected = replay_kind == AdversarialReplayKind::ReorgReplay;
        let reserve_shortfall = replay_kind == AdversarialReplayKind::ReserveShortfall;
        let settlement_mismatch = replay_kind == AdversarialReplayKind::SettlementMismatch;
        let metadata_leak = replay_kind == AdversarialReplayKind::MetadataLeak;
        let challenge_window_blocked = replay_kind == AdversarialReplayKind::BlockedChallengeWindow;
        let hostile_input_root = hostile_input_root(config, source, replay_kind, ordinal);
        let expected_evidence_root = expected_evidence_root(config, source, replay_kind);
        let replay_detection_root = replay_detection_root(
            config,
            source,
            replay_kind,
            &hostile_input_root,
            &expected_evidence_root,
        );
        let wallet_visible = config.require_wallet_visible_blockers;
        let production_blocked = config.fail_closed_on_any_adversarial_replay
            || source.execution_production_blocked
            || source.production_blocker_count > 0;
        let user_release_blocked = wallet_visible
            && (stale_pq_epoch
                || watcher_collusion
                || reorg_detected
                || reserve_shortfall
                || settlement_mismatch
                || metadata_leak
                || challenge_window_blocked);
        let status = if !wallet_visible {
            AdversarialReplayStatus::NeedsWalletEscalation
        } else if production_blocked || user_release_blocked {
            AdversarialReplayStatus::FailClosedBlocked
        } else {
            AdversarialReplayStatus::Rejected
        };
        let wallet_blocker_root = wallet_blocker_root(
            config,
            source,
            replay_kind,
            status,
            &replay_detection_root,
            wallet_visible,
        );
        let case_root = case_root(
            config,
            source,
            replay_kind,
            ordinal,
            &hostile_input_root,
            &expected_evidence_root,
            &replay_detection_root,
            &wallet_blocker_root,
            status,
            production_blocked,
            user_release_blocked,
        );
        Self {
            case_id: case_id(replay_kind, ordinal, &case_root),
            ordinal,
            replay_kind,
            hostile_input_root,
            expected_evidence_root,
            replay_detection_root,
            wallet_blocker_root,
            stale_pq_epoch,
            watcher_collusion,
            reorg_detected,
            reserve_shortfall,
            settlement_mismatch,
            metadata_leak,
            challenge_window_blocked,
            wallet_visible,
            production_blocked,
            user_release_blocked,
            status,
            required_outcome: required_outcome(replay_kind, status).to_string(),
            case_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "case_id": self.case_id,
            "ordinal": self.ordinal,
            "replay_kind": self.replay_kind.as_str(),
            "hostile_input_root": self.hostile_input_root,
            "expected_evidence_root": self.expected_evidence_root,
            "replay_detection_root": self.replay_detection_root,
            "wallet_blocker_root": self.wallet_blocker_root,
            "stale_pq_epoch": self.stale_pq_epoch,
            "watcher_collusion": self.watcher_collusion,
            "reorg_detected": self.reorg_detected,
            "reserve_shortfall": self.reserve_shortfall,
            "settlement_mismatch": self.settlement_mismatch,
            "metadata_leak": self.metadata_leak,
            "challenge_window_blocked": self.challenge_window_blocked,
            "wallet_visible": self.wallet_visible,
            "production_blocked": self.production_blocked,
            "user_release_blocked": self.user_release_blocked,
            "status": self.status.as_str(),
            "required_outcome": self.required_outcome,
            "case_root": self.case_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdversarialReplayVerdict {
    pub verdict_id: String,
    pub matrix_root: String,
    pub wallet_blocker_bundle_root: String,
    pub case_count: u64,
    pub fail_closed_case_count: u64,
    pub wallet_visible_blocker_count: u64,
    pub stale_pq_epoch_rejected: bool,
    pub watcher_collusion_rejected: bool,
    pub reorg_replay_rejected: bool,
    pub reserve_shortfall_rejected: bool,
    pub settlement_mismatch_rejected: bool,
    pub metadata_leak_rejected: bool,
    pub blocked_challenge_window_rejected: bool,
    pub fail_closed_active: bool,
    pub production_blocked: bool,
    pub user_release_blocked: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl AdversarialReplayVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        cases: &[AdversarialReplayCaseRecord],
    ) -> Self {
        let matrix_root = replay_case_matrix_root(cases);
        let wallet_blocker_bundle_root = wallet_blocker_bundle_root(cases);
        let case_count = cases.len() as u64;
        let fail_closed_case_count = cases
            .iter()
            .filter(|case| case.status == AdversarialReplayStatus::FailClosedBlocked)
            .count() as u64;
        let wallet_visible_blocker_count = cases
            .iter()
            .filter(|case| case.wallet_visible && case.user_release_blocked)
            .count() as u64;
        let fail_closed_active = config.fail_closed_on_any_adversarial_replay
            && fail_closed_case_count > 0
            && case_count >= config.min_replay_cases;
        let production_blocked = fail_closed_active || source.execution_production_blocked;
        let user_release_blocked =
            wallet_visible_blocker_count > 0 || source.release_held_count > 0;
        let stale_pq_epoch_rejected = case_rejected(cases, AdversarialReplayKind::StalePqEpoch);
        let watcher_collusion_rejected =
            case_rejected(cases, AdversarialReplayKind::WatcherCollusion);
        let reorg_replay_rejected = case_rejected(cases, AdversarialReplayKind::ReorgReplay);
        let reserve_shortfall_rejected =
            case_rejected(cases, AdversarialReplayKind::ReserveShortfall);
        let settlement_mismatch_rejected =
            case_rejected(cases, AdversarialReplayKind::SettlementMismatch);
        let metadata_leak_rejected = case_rejected(cases, AdversarialReplayKind::MetadataLeak);
        let blocked_challenge_window_rejected =
            case_rejected(cases, AdversarialReplayKind::BlockedChallengeWindow);
        let user_escape_answer = if user_release_blocked {
            "force_exit_replay_attempt_blocked_wallet_visible".to_string()
        } else {
            "force_exit_replay_attempt_rejected_without_release_blocker".to_string()
        };
        let production_answer = if production_blocked {
            "production_blocked_until_adversarial_replay_matrix_clears".to_string()
        } else {
            "production_may_continue_after_replay_matrix_clear".to_string()
        };
        let verdict_root = verdict_root(
            config,
            source,
            &matrix_root,
            &wallet_blocker_bundle_root,
            case_count,
            fail_closed_case_count,
            wallet_visible_blocker_count,
            fail_closed_active,
            production_blocked,
            user_release_blocked,
        );
        Self {
            verdict_id: verdict_id(&verdict_root),
            matrix_root,
            wallet_blocker_bundle_root,
            case_count,
            fail_closed_case_count,
            wallet_visible_blocker_count,
            stale_pq_epoch_rejected,
            watcher_collusion_rejected,
            reorg_replay_rejected,
            reserve_shortfall_rejected,
            settlement_mismatch_rejected,
            metadata_leak_rejected,
            blocked_challenge_window_rejected,
            fail_closed_active,
            production_blocked,
            user_release_blocked,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "matrix_root": self.matrix_root,
            "wallet_blocker_bundle_root": self.wallet_blocker_bundle_root,
            "case_count": self.case_count,
            "fail_closed_case_count": self.fail_closed_case_count,
            "wallet_visible_blocker_count": self.wallet_visible_blocker_count,
            "stale_pq_epoch_rejected": self.stale_pq_epoch_rejected,
            "watcher_collusion_rejected": self.watcher_collusion_rejected,
            "reorg_replay_rejected": self.reorg_replay_rejected,
            "reserve_shortfall_rejected": self.reserve_shortfall_rejected,
            "settlement_mismatch_rejected": self.settlement_mismatch_rejected,
            "metadata_leak_rejected": self.metadata_leak_rejected,
            "blocked_challenge_window_rejected": self.blocked_challenge_window_rejected,
            "fail_closed_active": self.fail_closed_active,
            "production_blocked": self.production_blocked,
            "user_release_blocked": self.user_release_blocked,
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
    pub cases: Vec<AdversarialReplayCaseRecord>,
    pub verdict: AdversarialReplayVerdict,
    pub replay_case_matrix_root: String,
    pub wallet_blocker_bundle_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let cases = AdversarialReplayKind::ordered()
            .iter()
            .map(|kind| AdversarialReplayCaseRecord::devnet(&config, &source, *kind))
            .collect::<Vec<_>>();
        let verdict = AdversarialReplayVerdict::new(&config, &source, &cases);
        let replay_case_matrix_root = replay_case_matrix_root(&cases);
        let wallet_blocker_bundle_root = wallet_blocker_bundle_root(&cases);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &replay_case_matrix_root,
            &wallet_blocker_bundle_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            cases,
            verdict,
            replay_case_matrix_root,
            wallet_blocker_bundle_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), SourceBundle::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_adversarial_replay_matrix_runtime",
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "cases": self
                .cases
                .iter()
                .map(AdversarialReplayCaseRecord::public_record)
                .collect::<Vec<_>>(),
            "verdict": self.verdict.public_record(),
            "replay_case_matrix_root": self.replay_case_matrix_root,
            "wallet_blocker_bundle_root": self.wallet_blocker_bundle_root,
            "state_commitment_root": self.state_commitment_root,
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

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-ADVERSARIAL-REPLAY-MATRIX-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn case_ordinal(kind: AdversarialReplayKind) -> u64 {
    match kind {
        AdversarialReplayKind::StalePqEpoch => 1,
        AdversarialReplayKind::WatcherCollusion => 2,
        AdversarialReplayKind::ReorgReplay => 3,
        AdversarialReplayKind::ReserveShortfall => 4,
        AdversarialReplayKind::SettlementMismatch => 5,
        AdversarialReplayKind::MetadataLeak => 6,
        AdversarialReplayKind::BlockedChallengeWindow => 7,
    }
}

fn hostile_input_root(
    config: &Config,
    source: &SourceBundle,
    kind: AdversarialReplayKind,
    ordinal: u64,
) -> String {
    let adversarial_epoch = config
        .current_pq_epoch
        .saturating_sub(config.max_pq_epoch_lag.saturating_add(2));
    let adversarial_reorg_depth = config.max_reorg_depth.saturating_add(1);
    let reserve_required_units = source
        .execution_receipt_count
        .saturating_add(source.production_blocker_count)
        .saturating_mul(config.reserve_unit);
    let reserve_observed_units = reserve_required_units.saturating_sub(config.reserve_unit);
    record_root(
        "hostile-input",
        &json!({
            "replay_kind": kind.as_str(),
            "ordinal": ordinal,
            "package_state_root": source.package_state_root,
            "execution_receipt_root": source.execution_receipt_root,
            "adversarial_pq_epoch": adversarial_epoch,
            "current_pq_epoch": config.current_pq_epoch,
            "adversarial_reorg_depth": adversarial_reorg_depth,
            "allowed_reorg_depth": config.max_reorg_depth,
            "reserve_required_units": reserve_required_units,
            "reserve_observed_units": reserve_observed_units,
            "challenge_window_blocks": config.challenge_window_blocks,
            "hostile_vector": kind.wallet_blocker(),
        }),
    )
}

fn expected_evidence_root(
    config: &Config,
    source: &SourceBundle,
    kind: AdversarialReplayKind,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-EXPECTED-EVIDENCE",
        &[
            HashPart::Str(&config.replay_matrix_suite),
            HashPart::Str(kind.as_str()),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&source.observed_submission_bundle_root),
            HashPart::Str(&source.challenge_settlement_bundle_root),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(bool_str(source.reserve_receipts_present)),
            HashPart::Str(bool_str(source.settlement_receipts_present)),
        ],
        32,
    )
}

fn replay_detection_root(
    config: &Config,
    source: &SourceBundle,
    kind: AdversarialReplayKind,
    hostile_input_root: &str,
    expected_evidence_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-DETECTION",
        &[
            HashPart::Str(&config.replay_matrix_suite),
            HashPart::Str(&source.state_root()),
            HashPart::Str(kind.as_str()),
            HashPart::Str(hostile_input_root),
            HashPart::Str(expected_evidence_root),
            HashPart::Str(bool_str(config.require_metadata_privacy)),
            HashPart::Str(bool_str(config.require_settlement_binding)),
            HashPart::Str(bool_str(config.require_reserve_coverage)),
        ],
        32,
    )
}

fn wallet_blocker_root(
    config: &Config,
    source: &SourceBundle,
    kind: AdversarialReplayKind,
    status: AdversarialReplayStatus,
    replay_detection_root: &str,
    wallet_visible: bool,
) -> String {
    record_root(
        "wallet-visible-blocker",
        &json!({
            "replay_matrix_suite": config.replay_matrix_suite,
            "replay_kind": kind.as_str(),
            "blocker": kind.wallet_blocker(),
            "status": status.as_str(),
            "replay_detection_root": replay_detection_root,
            "execution_user_escape_answer": source.execution_user_escape_answer,
            "wallet_visible": wallet_visible,
            "fail_closed": status == AdversarialReplayStatus::FailClosedBlocked,
        }),
    )
}

fn case_root(
    config: &Config,
    source: &SourceBundle,
    kind: AdversarialReplayKind,
    ordinal: u64,
    hostile_input_root: &str,
    expected_evidence_root: &str,
    replay_detection_root: &str,
    wallet_blocker_root: &str,
    status: AdversarialReplayStatus,
    production_blocked: bool,
    user_release_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-CASE",
        &[
            HashPart::Str(&config.replay_matrix_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::Str(hostile_input_root),
            HashPart::Str(expected_evidence_root),
            HashPart::Str(replay_detection_root),
            HashPart::Str(wallet_blocker_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(bool_str(user_release_blocked)),
        ],
        32,
    )
}

fn case_id(kind: AdversarialReplayKind, ordinal: u64, case_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-CASE-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(case_root),
        ],
        16,
    )
}

fn replay_case_matrix_root(cases: &[AdversarialReplayCaseRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-MATRIX",
        &cases
            .iter()
            .map(AdversarialReplayCaseRecord::public_record)
            .collect::<Vec<_>>(),
    )
}

fn wallet_blocker_bundle_root(cases: &[AdversarialReplayCaseRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-WALLET-BLOCKERS",
        &cases
            .iter()
            .map(|case| {
                json!({
                    "case_id": case.case_id,
                    "replay_kind": case.replay_kind.as_str(),
                    "wallet_blocker_root": case.wallet_blocker_root,
                    "wallet_visible": case.wallet_visible,
                    "user_release_blocked": case.user_release_blocked,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn case_rejected(cases: &[AdversarialReplayCaseRecord], kind: AdversarialReplayKind) -> bool {
    cases.iter().any(|case| {
        case.replay_kind == kind
            && matches!(
                case.status,
                AdversarialReplayStatus::Rejected | AdversarialReplayStatus::FailClosedBlocked
            )
    })
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    matrix_root: &str,
    wallet_blocker_bundle_root: &str,
    case_count: u64,
    fail_closed_case_count: u64,
    wallet_visible_blocker_count: u64,
    fail_closed_active: bool,
    production_blocked: bool,
    user_release_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-VERDICT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(matrix_root),
            HashPart::Str(wallet_blocker_bundle_root),
            HashPart::U64(case_count),
            HashPart::U64(fail_closed_case_count),
            HashPart::U64(wallet_visible_blocker_count),
            HashPart::Str(bool_str(fail_closed_active)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(bool_str(user_release_blocked)),
        ],
        32,
    )
}

fn verdict_id(verdict_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-ADVERSARIAL-REPLAY-VERDICT-ID",
        &[HashPart::Str(verdict_root)],
        16,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    replay_case_matrix_root: &str,
    wallet_blocker_bundle_root: &str,
    verdict: &AdversarialReplayVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-ADVERSARIAL-REPLAY-MATRIX-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(replay_case_matrix_root),
            HashPart::Str(wallet_blocker_bundle_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn required_outcome(kind: AdversarialReplayKind, status: AdversarialReplayStatus) -> &'static str {
    match status {
        AdversarialReplayStatus::Rejected => "reject replay evidence and keep force-exit path live",
        AdversarialReplayStatus::FailClosedBlocked => match kind {
            AdversarialReplayKind::StalePqEpoch => {
                "fail closed until wallet sees a fresh PQ authority epoch"
            }
            AdversarialReplayKind::WatcherCollusion => {
                "fail closed until independent watcher quorum replaces colluding witnesses"
            }
            AdversarialReplayKind::ReorgReplay => {
                "fail closed until canonical chain binding defeats the replayed claim"
            }
            AdversarialReplayKind::ReserveShortfall => {
                "fail closed until reserve fallback coverage is wallet visible"
            }
            AdversarialReplayKind::SettlementMismatch => {
                "fail closed until settlement root matches the force-exit receipt"
            }
            AdversarialReplayKind::MetadataLeak => {
                "fail closed until leaked metadata is quarantined from wallet evidence"
            }
            AdversarialReplayKind::BlockedChallengeWindow => {
                "fail closed until challenge window liveness is restored"
            }
        },
        AdversarialReplayStatus::NeedsWalletEscalation => {
            "escalate because replay blocker is not wallet visible"
        }
    }
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("adversarial replay matrix chain id mismatch".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("adversarial replay matrix protocol version mismatch".to_string());
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("adversarial replay matrix schema version mismatch".to_string());
    }
    if config.min_replay_cases < DEFAULT_MIN_REPLAY_CASES {
        return Err("adversarial replay matrix requires all replay cases".to_string());
    }
    if config.challenge_window_blocks == 0 {
        return Err("adversarial replay matrix requires challenge window blocks".to_string());
    }
    if config.reserve_unit == 0 {
        return Err("adversarial replay matrix requires reserve unit".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.execution_state_root.is_empty() {
        return Err("adversarial replay matrix missing execution state root".to_string());
    }
    if source.execution_receipt_root.is_empty() {
        return Err("adversarial replay matrix missing execution receipt root".to_string());
    }
    if source.package_state_root.is_empty() {
        return Err("adversarial replay matrix missing package state root".to_string());
    }
    if source.execution_receipt_count == 0 {
        return Err("adversarial replay matrix requires execution receipts".to_string());
    }
    Ok(())
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
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        package_state_root: record_root("fallback-package-state", &json!({"reason": &reason})),
        execution_status: "fallback".to_string(),
        execution_user_escape_answer: "force_exit_replay_matrix_source_unavailable".to_string(),
        execution_production_answer: "production_blocked_until_replay_matrix_source_recovers"
            .to_string(),
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        deferred_receipt_count: 1,
        release_held_count: 1,
        fail_closed_count: 1,
        production_blocker_count: 1,
        reserve_receipts_present: false,
        settlement_receipts_present: false,
        package_execution_observed: false,
        user_escape_execution_observed: false,
        execution_production_blocked: true,
    };
    let cases = AdversarialReplayKind::ordered()
        .iter()
        .map(|kind| AdversarialReplayCaseRecord::devnet(&config, &source, *kind))
        .collect::<Vec<_>>();
    let verdict = AdversarialReplayVerdict::new(&config, &source, &cases);
    let replay_case_matrix_root = replay_case_matrix_root(&cases);
    let wallet_blocker_bundle_root = wallet_blocker_bundle_root(&cases);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &replay_case_matrix_root,
        &wallet_blocker_bundle_root,
        &verdict,
    );
    State {
        config,
        source,
        cases,
        verdict,
        replay_case_matrix_root,
        wallet_blocker_bundle_root,
        state_commitment_root,
    }
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
