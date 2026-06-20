use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_monitor_runtime as challenge_window,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution_receipt,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_pq_authority_receipt_verifier_runtime as pq_authority,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_reserve_fallback_observation_runtime as reserve_fallback,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_settlement_observation_runtime as settlement_observation,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_wallet_scan_receipt_observer_runtime as wallet_scan,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageLiveFeedReconciliationRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_LIVE_FEED_RECONCILIATION_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-live-feed-reconciliation-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_LIVE_FEED_RECONCILIATION_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const LIVE_FEED_RECONCILIATION_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-live-feed-reconciliation-v1";
pub const DEFAULT_MIN_LIVE_FEEDS: u64 = 6;
pub const DEFAULT_MIN_RECONCILIATION_RECORDS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub reconciliation_suite: String,
    pub min_live_feeds: u64,
    pub min_reconciliation_records: u64,
    pub require_monero_watcher_live: bool,
    pub require_settlement_live: bool,
    pub require_reserve_live: bool,
    pub require_pq_live: bool,
    pub require_wallet_scan_live: bool,
    pub require_challenge_window_live: bool,
    pub fail_closed_on_deferred_evidence: bool,
    pub hold_production_until_all_feeds_live: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            reconciliation_suite: LIVE_FEED_RECONCILIATION_SUITE.to_string(),
            min_live_feeds: DEFAULT_MIN_LIVE_FEEDS,
            min_reconciliation_records: DEFAULT_MIN_RECONCILIATION_RECORDS,
            require_monero_watcher_live: true,
            require_settlement_live: true,
            require_reserve_live: true,
            require_pq_live: true,
            require_wallet_scan_live: true,
            require_challenge_window_live: true,
            fail_closed_on_deferred_evidence: true,
            hold_production_until_all_feeds_live: true,
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
            "reconciliation_suite": self.reconciliation_suite,
            "min_live_feeds": self.min_live_feeds,
            "min_reconciliation_records": self.min_reconciliation_records,
            "require_monero_watcher_live": self.require_monero_watcher_live,
            "require_settlement_live": self.require_settlement_live,
            "require_reserve_live": self.require_reserve_live,
            "require_pq_live": self.require_pq_live,
            "require_wallet_scan_live": self.require_wallet_scan_live,
            "require_challenge_window_live": self.require_challenge_window_live,
            "fail_closed_on_deferred_evidence": self.fail_closed_on_deferred_evidence,
            "hold_production_until_all_feeds_live": self.hold_production_until_all_feeds_live,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_receipt_state_root: String,
    pub monero_watcher_root: String,
    pub settlement_state_root: String,
    pub reserve_state_root: String,
    pub pq_authority_state_root: String,
    pub wallet_scan_state_root: String,
    pub challenge_window_state_root: String,
    pub execution_receipt_root: String,
    pub settlement_observation_root: String,
    pub reserve_observation_root: String,
    pub pq_receipt_root: String,
    pub wallet_scan_receipt_root: String,
    pub challenge_observation_root: String,
    pub live_monero_watcher: bool,
    pub live_settlement_feed: bool,
    pub live_reserve_feed: bool,
    pub live_pq_feed: bool,
    pub live_wallet_scan_feed: bool,
    pub live_challenge_window_feed: bool,
    pub deferred_fixture_roots_present: bool,
    pub placeholder_evidence_count: u64,
    pub live_feed_count: u64,
}

impl SourceBundle {
    pub fn from_states(
        execution_state: &execution_receipt::State,
        settlement_state: &settlement_observation::State,
        reserve_state: &reserve_fallback::State,
        pq_state: &pq_authority::State,
        wallet_state: &wallet_scan::State,
        challenge_state: &challenge_window::State,
    ) -> Self {
        Self {
            execution_receipt_state_root: execution_state.state_root(),
            monero_watcher_root: execution_state.observed_submission_bundle_root.clone(),
            settlement_state_root: settlement_state.state_root(),
            reserve_state_root: reserve_state.state_root(),
            pq_authority_state_root: pq_state.state_root(),
            wallet_scan_state_root: wallet_state.state_root(),
            challenge_window_state_root: challenge_state.state_root(),
            execution_receipt_root: execution_state.execution_receipt_root.clone(),
            settlement_observation_root: settlement_state.settlement_observation_root.clone(),
            reserve_observation_root: reserve_state.reserve_observation_root.clone(),
            pq_receipt_root: pq_state.authority_receipt_root.clone(),
            wallet_scan_receipt_root: wallet_state.wallet_scan_receipt_root.clone(),
            challenge_observation_root: challenge_state.watcher_objection_bundle_root.clone(),
            live_monero_watcher: false,
            live_settlement_feed: false,
            live_reserve_feed: false,
            live_pq_feed: false,
            live_wallet_scan_feed: false,
            live_challenge_window_feed: false,
            deferred_fixture_roots_present: true,
            placeholder_evidence_count: DEFAULT_MIN_LIVE_FEEDS,
            live_feed_count: 0,
        }
    }

    pub fn devnet() -> Self {
        Self::from_states(
            &execution_receipt::devnet(),
            &settlement_observation::devnet(),
            &reserve_fallback::devnet(),
            &pq_authority::devnet(),
            &wallet_scan::devnet(),
            &challenge_window::devnet(),
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_receipt_state_root": self.execution_receipt_state_root,
            "monero_watcher_root": self.monero_watcher_root,
            "settlement_state_root": self.settlement_state_root,
            "reserve_state_root": self.reserve_state_root,
            "pq_authority_state_root": self.pq_authority_state_root,
            "wallet_scan_state_root": self.wallet_scan_state_root,
            "challenge_window_state_root": self.challenge_window_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "settlement_observation_root": self.settlement_observation_root,
            "reserve_observation_root": self.reserve_observation_root,
            "pq_receipt_root": self.pq_receipt_root,
            "wallet_scan_receipt_root": self.wallet_scan_receipt_root,
            "challenge_observation_root": self.challenge_observation_root,
            "live_monero_watcher": self.live_monero_watcher,
            "live_settlement_feed": self.live_settlement_feed,
            "live_reserve_feed": self.live_reserve_feed,
            "live_pq_feed": self.live_pq_feed,
            "live_wallet_scan_feed": self.live_wallet_scan_feed,
            "live_challenge_window_feed": self.live_challenge_window_feed,
            "deferred_fixture_roots_present": self.deferred_fixture_roots_present,
            "placeholder_evidence_count": self.placeholder_evidence_count,
            "live_feed_count": self.live_feed_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveFeedKind {
    MoneroWatcher,
    SettlementObservation,
    ReserveFallback,
    PqAuthority,
    WalletScan,
    ChallengeWindow,
}

impl LiveFeedKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::MoneroWatcher,
            Self::SettlementObservation,
            Self::ReserveFallback,
            Self::PqAuthority,
            Self::WalletScan,
            Self::ChallengeWindow,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroWatcher => "monero_watcher",
            Self::SettlementObservation => "settlement_observation",
            Self::ReserveFallback => "reserve_fallback",
            Self::PqAuthority => "pq_authority",
            Self::WalletScan => "wallet_scan",
            Self::ChallengeWindow => "challenge_window",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationStatus {
    LiveMatched,
    DeferredFixture,
    MissingLiveFeed,
    PlaceholderEvidence,
    FailClosed,
}

impl ReconciliationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveMatched => "live_matched",
            Self::DeferredFixture => "deferred_fixture",
            Self::MissingLiveFeed => "missing_live_feed",
            Self::PlaceholderEvidence => "placeholder_evidence",
            Self::FailClosed => "fail_closed",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::LiveMatched)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationVerdictStatus {
    AllLiveFeedsMatched,
    WaitingLiveFeeds,
    DeferredFixtureRootsPresent,
    FailClosedHold,
}

impl ReconciliationVerdictStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllLiveFeedsMatched => "all_live_feeds_matched",
            Self::WaitingLiveFeeds => "waiting_live_feeds",
            Self::DeferredFixtureRootsPresent => "deferred_fixture_roots_present",
            Self::FailClosedHold => "fail_closed_hold",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveFeedReconciliationRecord {
    pub reconciliation_id: String,
    pub ordinal: u64,
    pub feed_kind: LiveFeedKind,
    pub deferred_root: String,
    pub live_root: String,
    pub target_root: String,
    pub observed_root: String,
    pub status: ReconciliationStatus,
    pub live_feed_present: bool,
    pub placeholder_replaced: bool,
    pub root_matches: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub required_action: String,
    pub reconciliation_root: String,
}

impl LiveFeedReconciliationRecord {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        feed_kind: LiveFeedKind,
        ordinal: u64,
    ) -> Self {
        let deferred_root = deferred_root(source, feed_kind);
        let live_root = live_root(source, feed_kind);
        let live_feed_present = live_present(source, feed_kind);
        let placeholder_replaced = live_feed_present && !source.deferred_fixture_roots_present;
        let root_matches = live_feed_present && placeholder_replaced && !live_root.is_empty();
        let status = reconciliation_status(
            config,
            source,
            live_feed_present,
            placeholder_replaced,
            root_matches,
        );
        let target_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-EXPECTED-ROOT",
            &[
                HashPart::Str(&config.reconciliation_suite),
                HashPart::Str(feed_kind.as_str()),
                HashPart::Str(&deferred_root),
                HashPart::U64(ordinal),
            ],
            32,
        );
        let observed_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-OBSERVED-ROOT",
            &[
                HashPart::Str(&config.reconciliation_suite),
                HashPart::Str(feed_kind.as_str()),
                HashPart::Str(&live_root),
                HashPart::Str(bool_str(live_feed_present)),
                HashPart::Str(bool_str(placeholder_replaced)),
            ],
            32,
        );
        let reconciliation_root = reconciliation_record_root(
            config,
            source,
            feed_kind,
            ordinal,
            &target_root,
            &observed_root,
            status,
        );
        Self {
            reconciliation_id: reconciliation_id(feed_kind, ordinal, &reconciliation_root),
            ordinal,
            feed_kind,
            deferred_root,
            live_root,
            target_root,
            observed_root,
            status,
            live_feed_present,
            placeholder_replaced,
            root_matches,
            blocks_user_release: status.blocks_release(),
            blocks_production: status.blocks_release(),
            required_action: required_action(status, feed_kind).to_string(),
            reconciliation_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reconciliation_id": self.reconciliation_id,
            "ordinal": self.ordinal,
            "feed_kind": self.feed_kind.as_str(),
            "deferred_root": self.deferred_root,
            "live_root": self.live_root,
            "target_root": self.target_root,
            "observed_root": self.observed_root,
            "status": self.status.as_str(),
            "live_feed_present": self.live_feed_present,
            "placeholder_replaced": self.placeholder_replaced,
            "root_matches": self.root_matches,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "required_action": self.required_action,
            "reconciliation_root": self.reconciliation_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("live-feed-reconciliation-record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveFeedReconciliationVerdict {
    pub status: ReconciliationVerdictStatus,
    pub live_feed_count: u64,
    pub required_live_feed_count: u64,
    pub record_count: u64,
    pub deferred_fixture_count: u64,
    pub missing_live_feed_count: u64,
    pub placeholder_evidence_count: u64,
    pub matched_live_feed_count: u64,
    pub all_placeholders_replaced: bool,
    pub all_live_feeds_matched: bool,
    pub release_allowed: bool,
    pub production_blocked: bool,
    pub fail_closed: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl LiveFeedReconciliationVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        records: &[LiveFeedReconciliationRecord],
    ) -> Self {
        let deferred_fixture_count = count_status(records, ReconciliationStatus::DeferredFixture);
        let missing_live_feed_count = count_status(records, ReconciliationStatus::MissingLiveFeed);
        let placeholder_evidence_count =
            count_status(records, ReconciliationStatus::PlaceholderEvidence)
                + source.placeholder_evidence_count;
        let matched_live_feed_count = count_status(records, ReconciliationStatus::LiveMatched);
        let all_placeholders_replaced =
            placeholder_evidence_count == 0 && !source.deferred_fixture_roots_present;
        let all_live_feeds_matched = matched_live_feed_count >= config.min_live_feeds
            && records.len() as u64 >= config.min_reconciliation_records
            && all_placeholders_replaced;
        let fail_closed = config.fail_closed_on_deferred_evidence
            && (deferred_fixture_count > 0
                || missing_live_feed_count > 0
                || placeholder_evidence_count > 0);
        let status = if all_live_feeds_matched {
            ReconciliationVerdictStatus::AllLiveFeedsMatched
        } else if fail_closed {
            ReconciliationVerdictStatus::FailClosedHold
        } else if deferred_fixture_count > 0 {
            ReconciliationVerdictStatus::DeferredFixtureRootsPresent
        } else {
            ReconciliationVerdictStatus::WaitingLiveFeeds
        };
        let release_allowed = all_live_feeds_matched && !fail_closed;
        let production_blocked =
            config.hold_production_until_all_feeds_live && (!release_allowed || fail_closed);
        let user_escape_answer = user_escape_answer(release_allowed, fail_closed).to_string();
        let production_answer = production_answer(production_blocked).to_string();
        let verdict_root = verdict_root(
            config,
            source,
            records,
            status,
            release_allowed,
            production_blocked,
            fail_closed,
        );
        Self {
            status,
            live_feed_count: source.live_feed_count,
            required_live_feed_count: config.min_live_feeds,
            record_count: records.len() as u64,
            deferred_fixture_count,
            missing_live_feed_count,
            placeholder_evidence_count,
            matched_live_feed_count,
            all_placeholders_replaced,
            all_live_feeds_matched,
            release_allowed,
            production_blocked,
            fail_closed,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "live_feed_count": self.live_feed_count,
            "required_live_feed_count": self.required_live_feed_count,
            "record_count": self.record_count,
            "deferred_fixture_count": self.deferred_fixture_count,
            "missing_live_feed_count": self.missing_live_feed_count,
            "placeholder_evidence_count": self.placeholder_evidence_count,
            "matched_live_feed_count": self.matched_live_feed_count,
            "all_placeholders_replaced": self.all_placeholders_replaced,
            "all_live_feeds_matched": self.all_live_feeds_matched,
            "release_allowed": self.release_allowed,
            "production_blocked": self.production_blocked,
            "fail_closed": self.fail_closed,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("live-feed-reconciliation-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub reconciliation_records: Vec<LiveFeedReconciliationRecord>,
    pub verdict: LiveFeedReconciliationVerdict,
    pub reconciliation_record_root: String,
    pub deferred_fixture_root: String,
    pub live_feed_root: String,
    pub fail_closed_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let reconciliation_records = LiveFeedKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, feed_kind)| {
                LiveFeedReconciliationRecord::new(&config, &source, *feed_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = LiveFeedReconciliationVerdict::new(&config, &source, &reconciliation_records);
        let reconciliation_record_root = reconciliation_record_vector_root(&reconciliation_records);
        let deferred_fixture_root = deferred_fixture_vector_root(&config, &source);
        let live_feed_root = live_feed_vector_root(&config, &source);
        let fail_closed_hold_root =
            fail_closed_hold_root(&config, &source, &reconciliation_records, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &reconciliation_record_root,
            &deferred_fixture_root,
            &live_feed_root,
            &fail_closed_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            reconciliation_records,
            verdict,
            reconciliation_record_root,
            deferred_fixture_root,
            live_feed_root,
            fail_closed_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::devnet(), SourceBundle::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_live_feed_reconciliation_runtime",
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "reconciliation_records": self.reconciliation_records.iter().map(|record| record.public_record()).collect::<Vec<_>>(),
            "verdict": self.verdict.public_record(),
            "reconciliation_record_root": self.reconciliation_record_root,
            "deferred_fixture_root": self.deferred_fixture_root,
            "live_feed_root": self.live_feed_root,
            "fail_closed_hold_root": self.fail_closed_hold_root,
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
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIVE-FEED-RECONCILIATION-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn deferred_root(source: &SourceBundle, feed_kind: LiveFeedKind) -> String {
    match feed_kind {
        LiveFeedKind::MoneroWatcher => source.execution_receipt_root.clone(),
        LiveFeedKind::SettlementObservation => source.settlement_observation_root.clone(),
        LiveFeedKind::ReserveFallback => source.reserve_observation_root.clone(),
        LiveFeedKind::PqAuthority => source.pq_receipt_root.clone(),
        LiveFeedKind::WalletScan => source.wallet_scan_receipt_root.clone(),
        LiveFeedKind::ChallengeWindow => source.challenge_observation_root.clone(),
    }
}

fn live_root(source: &SourceBundle, feed_kind: LiveFeedKind) -> String {
    match feed_kind {
        LiveFeedKind::MoneroWatcher => source.monero_watcher_root.clone(),
        LiveFeedKind::SettlementObservation => source.settlement_state_root.clone(),
        LiveFeedKind::ReserveFallback => source.reserve_state_root.clone(),
        LiveFeedKind::PqAuthority => source.pq_authority_state_root.clone(),
        LiveFeedKind::WalletScan => source.wallet_scan_state_root.clone(),
        LiveFeedKind::ChallengeWindow => source.challenge_window_state_root.clone(),
    }
}

fn live_present(source: &SourceBundle, feed_kind: LiveFeedKind) -> bool {
    match feed_kind {
        LiveFeedKind::MoneroWatcher => source.live_monero_watcher,
        LiveFeedKind::SettlementObservation => source.live_settlement_feed,
        LiveFeedKind::ReserveFallback => source.live_reserve_feed,
        LiveFeedKind::PqAuthority => source.live_pq_feed,
        LiveFeedKind::WalletScan => source.live_wallet_scan_feed,
        LiveFeedKind::ChallengeWindow => source.live_challenge_window_feed,
    }
}

fn reconciliation_status(
    config: &Config,
    source: &SourceBundle,
    live_feed_present: bool,
    placeholder_replaced: bool,
    root_matches: bool,
) -> ReconciliationStatus {
    if root_matches {
        ReconciliationStatus::LiveMatched
    } else if !live_feed_present {
        ReconciliationStatus::MissingLiveFeed
    } else if source.deferred_fixture_roots_present || !placeholder_replaced {
        ReconciliationStatus::DeferredFixture
    } else if source.placeholder_evidence_count > 0 {
        ReconciliationStatus::PlaceholderEvidence
    } else if config.fail_closed_on_deferred_evidence {
        ReconciliationStatus::FailClosed
    } else {
        ReconciliationStatus::MissingLiveFeed
    }
}

fn required_action(status: ReconciliationStatus, feed_kind: LiveFeedKind) -> &'static str {
    match status {
        ReconciliationStatus::LiveMatched => "accept live feed root",
        ReconciliationStatus::DeferredFixture => {
            "replace deferred fixture root with live feed evidence"
        }
        ReconciliationStatus::MissingLiveFeed => "wait for live feed root",
        ReconciliationStatus::PlaceholderEvidence => "replace placeholder evidence",
        ReconciliationStatus::FailClosed => match feed_kind {
            LiveFeedKind::MoneroWatcher => "hold until live Monero watcher confirms exit",
            LiveFeedKind::SettlementObservation => "hold until settlement observation is live",
            LiveFeedKind::ReserveFallback => "hold until reserve fallback observation is live",
            LiveFeedKind::PqAuthority => "hold until PQ authority receipt is live",
            LiveFeedKind::WalletScan => "hold until wallet scan receipt is live",
            LiveFeedKind::ChallengeWindow => "hold until challenge window observation is live",
        },
    }
}

fn reconciliation_id(feed_kind: LiveFeedKind, ordinal: u64, reconciliation_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-RECONCILIATION-ID",
        &[
            HashPart::Str(feed_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(reconciliation_root),
        ],
        16,
    )
}

fn reconciliation_record_root(
    config: &Config,
    source: &SourceBundle,
    feed_kind: LiveFeedKind,
    ordinal: u64,
    target_root: &str,
    observed_root: &str,
    status: ReconciliationStatus,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-RECONCILIATION-ROOT",
        &[
            HashPart::Str(&config.reconciliation_suite),
            HashPart::Str(&source.execution_receipt_state_root),
            HashPart::Str(feed_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(target_root),
            HashPart::Str(observed_root),
            HashPart::Str(status.as_str()),
        ],
        32,
    )
}

fn reconciliation_record_vector_root(records: &[LiveFeedReconciliationRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-RECONCILIATION-RECORDS",
        records
            .iter()
            .map(|record| record.reconciliation_root.clone())
            .collect::<Vec<_>>()
            .as_slice(),
    )
}

fn deferred_fixture_vector_root(config: &Config, source: &SourceBundle) -> String {
    let roots = LiveFeedKind::ordered()
        .iter()
        .map(|feed_kind| {
            domain_hash(
                "MONERO-L2-PQ-BRIDGE-EXIT-DEFERRED-FIXTURE-FEED-ROOT",
                &[
                    HashPart::Str(&config.reconciliation_suite),
                    HashPart::Str(feed_kind.as_str()),
                    HashPart::Str(&deferred_root(source, *feed_kind)),
                    HashPart::Str(bool_str(source.deferred_fixture_roots_present)),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-DEFERRED-FIXTURE-FEED-ROOTS",
        roots.as_slice(),
    )
}

fn live_feed_vector_root(config: &Config, source: &SourceBundle) -> String {
    let roots = LiveFeedKind::ordered()
        .iter()
        .map(|feed_kind| {
            domain_hash(
                "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-ROOT",
                &[
                    HashPart::Str(&config.reconciliation_suite),
                    HashPart::Str(feed_kind.as_str()),
                    HashPart::Str(&live_root(source, *feed_kind)),
                    HashPart::Str(bool_str(live_present(source, *feed_kind))),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    merkle_root("MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-ROOTS", roots.as_slice())
}

fn fail_closed_hold_root(
    config: &Config,
    source: &SourceBundle,
    records: &[LiveFeedReconciliationRecord],
    verdict: &LiveFeedReconciliationVerdict,
) -> String {
    record_root(
        "fail-closed-live-feed-hold",
        &json!({
            "reconciliation_suite": &config.reconciliation_suite,
            "source_root": source.state_root(),
            "record_count": records.len() as u64,
            "verdict_root": &verdict.verdict_root,
            "fail_closed": verdict.fail_closed,
            "production_blocked": verdict.production_blocked,
            "policy": "release remains held until every deferred fixture root is replaced by live feed evidence",
        }),
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    records: &[LiveFeedReconciliationRecord],
    status: ReconciliationVerdictStatus,
    release_allowed: bool,
    production_blocked: bool,
    fail_closed: bool,
) -> String {
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-RECONCILIATION-BLOCKERS",
        records
            .iter()
            .filter(|record| record.blocks_user_release)
            .map(|record| record.reconciliation_root.clone())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-FEED-RECONCILIATION-VERDICT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&blocker_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(release_allowed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(bool_str(fail_closed)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    reconciliation_record_root: &str,
    deferred_fixture_root: &str,
    live_feed_root: &str,
    fail_closed_hold_root: &str,
    verdict: &LiveFeedReconciliationVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIVE-FEED-RECONCILIATION-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(reconciliation_record_root),
            HashPart::Str(deferred_fixture_root),
            HashPart::Str(live_feed_root),
            HashPart::Str(fail_closed_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn user_escape_answer(release_allowed: bool, fail_closed: bool) -> &'static str {
    if release_allowed {
        "user_escape_release_allowed_after_live_feed_reconciliation"
    } else if fail_closed {
        "user_escape_force_exit_held_until_live_feeds_replace_deferred_evidence"
    } else {
        "user_escape_waits_for_live_feed_reconciliation"
    }
}

fn production_answer(production_blocked: bool) -> &'static str {
    if production_blocked {
        "hold_production_until_monero_settlement_reserve_pq_wallet_and_challenge_feeds_are_live"
    } else {
        "production_release_allowed_after_live_feed_reconciliation"
    }
}

fn count_status(records: &[LiveFeedReconciliationRecord], status: ReconciliationStatus) -> u64 {
    records
        .iter()
        .filter(|record| record.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "live feed reconciliation chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "live feed reconciliation protocol mismatch",
    )?;
    ensure(
        config.min_live_feeds > 0,
        "live feed reconciliation requires live feeds",
    )?;
    ensure(
        config.min_reconciliation_records > 0,
        "live feed reconciliation requires reconciliation records",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_receipt_state_root.is_empty(),
        "live feed reconciliation missing execution receipt state root",
    )?;
    ensure(
        !source.monero_watcher_root.is_empty(),
        "live feed reconciliation missing Monero watcher root",
    )?;
    ensure(
        !source.settlement_state_root.is_empty(),
        "live feed reconciliation missing settlement state root",
    )?;
    ensure(
        !source.reserve_state_root.is_empty(),
        "live feed reconciliation missing reserve state root",
    )?;
    ensure(
        !source.pq_authority_state_root.is_empty(),
        "live feed reconciliation missing PQ authority state root",
    )?;
    ensure(
        !source.wallet_scan_state_root.is_empty(),
        "live feed reconciliation missing wallet scan state root",
    )?;
    ensure(
        !source.challenge_window_state_root.is_empty(),
        "live feed reconciliation missing challenge window state root",
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
        execution_receipt_state_root: record_root(
            "fallback-execution-receipt-state",
            &json!({"reason": &reason}),
        ),
        monero_watcher_root: record_root("fallback-monero-watcher", &json!({"reason": &reason})),
        settlement_state_root: record_root("fallback-settlement", &json!({"reason": &reason})),
        reserve_state_root: record_root("fallback-reserve", &json!({"reason": &reason})),
        pq_authority_state_root: record_root("fallback-pq-authority", &json!({"reason": &reason})),
        wallet_scan_state_root: record_root("fallback-wallet-scan", &json!({"reason": &reason})),
        challenge_window_state_root: record_root(
            "fallback-challenge-window",
            &json!({"reason": &reason}),
        ),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({"reason": &reason}),
        ),
        settlement_observation_root: record_root(
            "fallback-settlement-observation",
            &json!({"reason": &reason}),
        ),
        reserve_observation_root: record_root(
            "fallback-reserve-observation",
            &json!({"reason": &reason}),
        ),
        pq_receipt_root: record_root("fallback-pq-receipt", &json!({"reason": &reason})),
        wallet_scan_receipt_root: record_root(
            "fallback-wallet-scan-receipt",
            &json!({"reason": &reason}),
        ),
        challenge_observation_root: record_root(
            "fallback-challenge-observation",
            &json!({"reason": &reason}),
        ),
        live_monero_watcher: false,
        live_settlement_feed: false,
        live_reserve_feed: false,
        live_pq_feed: false,
        live_wallet_scan_feed: false,
        live_challenge_window_feed: false,
        deferred_fixture_roots_present: true,
        placeholder_evidence_count: DEFAULT_MIN_LIVE_FEEDS,
        live_feed_count: 0,
    };
    let reconciliation_records = LiveFeedKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, feed_kind)| {
            LiveFeedReconciliationRecord::new(&config, &source, *feed_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = LiveFeedReconciliationVerdict::new(&config, &source, &reconciliation_records);
    let reconciliation_record_root = reconciliation_record_vector_root(&reconciliation_records);
    let deferred_fixture_root = deferred_fixture_vector_root(&config, &source);
    let live_feed_root = live_feed_vector_root(&config, &source);
    let fail_closed_hold_root =
        fail_closed_hold_root(&config, &source, &reconciliation_records, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &reconciliation_record_root,
        &deferred_fixture_root,
        &live_feed_root,
        &fail_closed_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        reconciliation_records,
        verdict,
        reconciliation_record_root,
        deferred_fixture_root,
        live_feed_root,
        fail_closed_hold_root,
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
