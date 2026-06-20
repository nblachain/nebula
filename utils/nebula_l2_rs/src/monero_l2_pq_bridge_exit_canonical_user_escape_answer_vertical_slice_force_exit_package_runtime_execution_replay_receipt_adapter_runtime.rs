use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution_receipt,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_live_feed_reconciliation_runtime as live_feed,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_operator_independent_force_exit_package_runtime as force_exit_package,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeExecutionReplayReceiptAdapterRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_EXECUTION_REPLAY_RECEIPT_ADAPTER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-execution-replay-receipt-adapter-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_EXECUTION_REPLAY_RECEIPT_ADAPTER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_RECEIPT_ADAPTER_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-runtime-execution-replay-receipt-adapter-v1";
pub const DEFAULT_MIN_REPLAY_LANES: u64 = 5;
pub const DEFAULT_MIN_DETERMINISTIC_TRANSCRIPTS: u64 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub adapter_suite: String,
    pub min_replay_lanes: u64,
    pub min_deterministic_transcripts: u64,
    pub require_devnet_replay_roots: bool,
    pub require_force_exit_closure_roots: bool,
    pub require_monero_anchor_roots: bool,
    pub require_wallet_replay_roots: bool,
    pub require_deterministic_transcript_roots: bool,
    pub defer_to_live_replacement: bool,
    pub fail_closed_on_divergence: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            adapter_suite: REPLAY_RECEIPT_ADAPTER_SUITE.to_string(),
            min_replay_lanes: DEFAULT_MIN_REPLAY_LANES,
            min_deterministic_transcripts: DEFAULT_MIN_DETERMINISTIC_TRANSCRIPTS,
            require_devnet_replay_roots: true,
            require_force_exit_closure_roots: true,
            require_monero_anchor_roots: true,
            require_wallet_replay_roots: true,
            require_deterministic_transcript_roots: true,
            defer_to_live_replacement: true,
            fail_closed_on_divergence: true,
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
            "adapter_suite": self.adapter_suite,
            "min_replay_lanes": self.min_replay_lanes,
            "min_deterministic_transcripts": self.min_deterministic_transcripts,
            "require_devnet_replay_roots": self.require_devnet_replay_roots,
            "require_force_exit_closure_roots": self.require_force_exit_closure_roots,
            "require_monero_anchor_roots": self.require_monero_anchor_roots,
            "require_wallet_replay_roots": self.require_wallet_replay_roots,
            "require_deterministic_transcript_roots": self.require_deterministic_transcript_roots,
            "defer_to_live_replacement": self.defer_to_live_replacement,
            "fail_closed_on_divergence": self.fail_closed_on_divergence,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub package_state_root: String,
    pub package_action_root: String,
    pub package_closure_root: String,
    pub execution_receipt_state_root: String,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub live_feed_state_root: String,
    pub deferred_fixture_root: String,
    pub live_feed_root: String,
    pub fail_closed_hold_root: String,
    pub devnet_fixture_roots_present: bool,
    pub live_replacement_available: bool,
    pub package_action_count: u64,
    pub execution_receipt_count: u64,
    pub wallet_replay_count: u64,
    pub deterministic_transcript_count: u64,
}

impl SourceBundle {
    pub fn from_states(
        package_state: &force_exit_package::State,
        execution_state: &execution_receipt::State,
        live_feed_state: &live_feed::State,
    ) -> Self {
        Self {
            package_state_root: package_state.state_root(),
            package_action_root: package_state.package_action_root.clone(),
            package_closure_root: package_state.production_hold_root.clone(),
            execution_receipt_state_root: execution_state.state_root(),
            execution_receipt_root: execution_state.execution_receipt_root.clone(),
            observed_submission_bundle_root: execution_state
                .observed_submission_bundle_root
                .clone(),
            live_feed_state_root: live_feed_state.state_root(),
            deferred_fixture_root: live_feed_state.deferred_fixture_root.clone(),
            live_feed_root: live_feed_state.live_feed_root.clone(),
            fail_closed_hold_root: live_feed_state.fail_closed_hold_root.clone(),
            devnet_fixture_roots_present: live_feed_state.source.deferred_fixture_roots_present,
            live_replacement_available: live_feed_state.verdict.all_live_feeds_matched,
            package_action_count: package_state.verdict.package_action_count,
            execution_receipt_count: execution_state.verdict.execution_receipt_count,
            wallet_replay_count: execution_state.verdict.user_escape_receipt_count,
            deterministic_transcript_count: execution_state.verdict.observed_receipt_count,
        }
    }

    pub fn devnet() -> Self {
        Self::from_states(
            &force_exit_package::devnet(),
            &execution_receipt::devnet(),
            &live_feed::devnet(),
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "package_state_root": self.package_state_root,
            "package_action_root": self.package_action_root,
            "package_closure_root": self.package_closure_root,
            "execution_receipt_state_root": self.execution_receipt_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "live_feed_state_root": self.live_feed_state_root,
            "deferred_fixture_root": self.deferred_fixture_root,
            "live_feed_root": self.live_feed_root,
            "fail_closed_hold_root": self.fail_closed_hold_root,
            "devnet_fixture_roots_present": self.devnet_fixture_roots_present,
            "live_replacement_available": self.live_replacement_available,
            "package_action_count": self.package_action_count,
            "execution_receipt_count": self.execution_receipt_count,
            "wallet_replay_count": self.wallet_replay_count,
            "deterministic_transcript_count": self.deterministic_transcript_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLaneKind {
    DevnetReplayRoots,
    ForceExitClosureStateRoots,
    ObservedMoneroAnchorRoots,
    WalletReplayRoots,
    DeterministicTranscriptRoots,
}

impl ReplayLaneKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::DevnetReplayRoots,
            Self::ForceExitClosureStateRoots,
            Self::ObservedMoneroAnchorRoots,
            Self::WalletReplayRoots,
            Self::DeterministicTranscriptRoots,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DevnetReplayRoots => "devnet_replay_roots",
            Self::ForceExitClosureStateRoots => "force_exit_closure_state_roots",
            Self::ObservedMoneroAnchorRoots => "observed_monero_anchor_roots",
            Self::WalletReplayRoots => "wallet_replay_roots",
            Self::DeterministicTranscriptRoots => "deterministic_transcript_roots",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayAdaptationStatus {
    Adapted,
    DeferredToLiveReplacement,
    Divergent,
    MissingRoot,
    FailClosed,
}

impl ReplayAdaptationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Adapted => "adapted",
            Self::DeferredToLiveReplacement => "deferred_to_live_replacement",
            Self::Divergent => "divergent",
            Self::MissingRoot => "missing_root",
            Self::FailClosed => "fail_closed",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Adapted)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayReceiptAdaptation {
    pub adaptation_id: String,
    pub ordinal: u64,
    pub lane_kind: ReplayLaneKind,
    pub replay_root: String,
    pub closure_state_root: String,
    pub monero_anchor_root: String,
    pub wallet_replay_root: String,
    pub deterministic_transcript_root: String,
    pub expected_receipt_root: String,
    pub adapted_receipt_root: String,
    pub divergence_finding_root: String,
    pub status: ReplayAdaptationStatus,
    pub deferred_to_live_replacement: bool,
    pub fail_closed: bool,
    pub finding: String,
}

impl ReplayReceiptAdaptation {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        lane_kind: ReplayLaneKind,
        ordinal: u64,
    ) -> Self {
        let replay_root = lane_replay_root(source, lane_kind);
        let closure_state_root = lane_closure_state_root(source, lane_kind);
        let monero_anchor_root = lane_monero_anchor_root(source, lane_kind);
        let wallet_replay_root = lane_wallet_replay_root(source, lane_kind);
        let deterministic_transcript_root = lane_deterministic_transcript_root(source, lane_kind);
        let missing_root = replay_root.is_empty()
            || closure_state_root.is_empty()
            || monero_anchor_root.is_empty()
            || wallet_replay_root.is_empty()
            || deterministic_transcript_root.is_empty();
        let deferred_to_live_replacement = config.defer_to_live_replacement
            && (source.devnet_fixture_roots_present || !source.live_replacement_available);
        let divergent = !missing_root
            && (source.devnet_fixture_roots_present
                || source.deterministic_transcript_count < config.min_deterministic_transcripts);
        let fail_closed = config.fail_closed_on_divergence && (missing_root || divergent);
        let status = adaptation_status(
            missing_root,
            deferred_to_live_replacement,
            divergent,
            fail_closed,
        );
        let expected_receipt_root = expected_receipt_root(
            config,
            source,
            lane_kind,
            ordinal,
            &replay_root,
            &closure_state_root,
        );
        let adapted_receipt_root = adapted_receipt_root(
            config,
            source,
            lane_kind,
            &expected_receipt_root,
            &monero_anchor_root,
            &wallet_replay_root,
            &deterministic_transcript_root,
            status,
        );
        let finding = finding(status, lane_kind).to_string();
        let divergence_finding_root = divergence_finding_root(
            config,
            source,
            lane_kind,
            &adapted_receipt_root,
            status,
            &finding,
        );
        let adaptation_id = adaptation_id(lane_kind, ordinal, &adapted_receipt_root);
        Self {
            adaptation_id,
            ordinal,
            lane_kind,
            replay_root,
            closure_state_root,
            monero_anchor_root,
            wallet_replay_root,
            deterministic_transcript_root,
            expected_receipt_root,
            adapted_receipt_root,
            divergence_finding_root,
            status,
            deferred_to_live_replacement,
            fail_closed,
            finding,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "adaptation_id": self.adaptation_id,
            "ordinal": self.ordinal,
            "lane_kind": self.lane_kind.as_str(),
            "replay_root": self.replay_root,
            "closure_state_root": self.closure_state_root,
            "monero_anchor_root": self.monero_anchor_root,
            "wallet_replay_root": self.wallet_replay_root,
            "deterministic_transcript_root": self.deterministic_transcript_root,
            "expected_receipt_root": self.expected_receipt_root,
            "adapted_receipt_root": self.adapted_receipt_root,
            "divergence_finding_root": self.divergence_finding_root,
            "status": self.status.as_str(),
            "deferred_to_live_replacement": self.deferred_to_live_replacement,
            "fail_closed": self.fail_closed,
            "finding": self.finding,
        })
    }

    pub fn state_root(&self) -> String {
        self.adapted_receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub devnet_replay_root: String,
    pub force_exit_closure_state_root: String,
    pub observed_monero_anchor_root: String,
    pub wallet_replay_root: String,
    pub deterministic_transcript_root: String,
    pub adaptation_receipt_root: String,
    pub divergence_finding_root: String,
    pub live_replacement_verdict_root: String,
    pub fail_closed_status_root: String,
}

impl Roots {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        adaptations: &[ReplayReceiptAdaptation],
        counters: &Counters,
    ) -> Self {
        let devnet_replay_root = lane_vector_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-DEVNET-REPLAY-ROOTS",
            adaptations,
            |adaptation| adaptation.replay_root.clone(),
        );
        let force_exit_closure_state_root = lane_vector_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-CLOSURE-STATE-ROOTS",
            adaptations,
            |adaptation| adaptation.closure_state_root.clone(),
        );
        let observed_monero_anchor_root = lane_vector_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-MONERO-ANCHOR-ROOTS",
            adaptations,
            |adaptation| adaptation.monero_anchor_root.clone(),
        );
        let wallet_replay_root = lane_vector_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-WALLET-REPLAY-ROOTS",
            adaptations,
            |adaptation| adaptation.wallet_replay_root.clone(),
        );
        let deterministic_transcript_root = lane_vector_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-DETERMINISTIC-TRANSCRIPT-ROOTS",
            adaptations,
            |adaptation| adaptation.deterministic_transcript_root.clone(),
        );
        let adaptation_receipt_root = merkle_root(
            "MONERO-L2-PQ-REPLAY-ADAPTER-ADAPTED-RECEIPTS",
            &adaptations
                .iter()
                .map(ReplayReceiptAdaptation::public_record)
                .collect::<Vec<_>>(),
        );
        let divergence_finding_root = divergence_vector_root(adaptations);
        let live_replacement_verdict_root =
            live_replacement_verdict_root(config, source, counters, &adaptation_receipt_root);
        let fail_closed_status_root =
            fail_closed_status_root(config, source, counters, &divergence_finding_root);
        Self {
            devnet_replay_root,
            force_exit_closure_state_root,
            observed_monero_anchor_root,
            wallet_replay_root,
            deterministic_transcript_root,
            adaptation_receipt_root,
            divergence_finding_root,
            live_replacement_verdict_root,
            fail_closed_status_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "devnet_replay_root": self.devnet_replay_root,
            "force_exit_closure_state_root": self.force_exit_closure_state_root,
            "observed_monero_anchor_root": self.observed_monero_anchor_root,
            "wallet_replay_root": self.wallet_replay_root,
            "deterministic_transcript_root": self.deterministic_transcript_root,
            "adaptation_receipt_root": self.adaptation_receipt_root,
            "divergence_finding_root": self.divergence_finding_root,
            "live_replacement_verdict_root": self.live_replacement_verdict_root,
            "fail_closed_status_root": self.fail_closed_status_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub replay_lane_count: u64,
    pub adapted_count: u64,
    pub deferred_to_live_replacement_count: u64,
    pub divergent_count: u64,
    pub missing_root_count: u64,
    pub fail_closed_count: u64,
    pub deterministic_transcript_count: u64,
    pub wallet_replay_count: u64,
}

impl Counters {
    pub fn new(source: &SourceBundle, adaptations: &[ReplayReceiptAdaptation]) -> Self {
        Self {
            replay_lane_count: adaptations.len() as u64,
            adapted_count: count_status(adaptations, ReplayAdaptationStatus::Adapted),
            deferred_to_live_replacement_count: count_status(
                adaptations,
                ReplayAdaptationStatus::DeferredToLiveReplacement,
            ),
            divergent_count: count_status(adaptations, ReplayAdaptationStatus::Divergent),
            missing_root_count: count_status(adaptations, ReplayAdaptationStatus::MissingRoot),
            fail_closed_count: adaptations
                .iter()
                .filter(|adaptation| adaptation.fail_closed)
                .count() as u64,
            deterministic_transcript_count: source.deterministic_transcript_count,
            wallet_replay_count: source.wallet_replay_count,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_lane_count": self.replay_lane_count,
            "adapted_count": self.adapted_count,
            "deferred_to_live_replacement_count": self.deferred_to_live_replacement_count,
            "divergent_count": self.divergent_count,
            "missing_root_count": self.missing_root_count,
            "fail_closed_count": self.fail_closed_count,
            "deterministic_transcript_count": self.deterministic_transcript_count,
            "wallet_replay_count": self.wallet_replay_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayReceiptAdapterVerdict {
    pub replay_receipt_adapted: bool,
    pub deferred_to_live_replacement: bool,
    pub divergence_found: bool,
    pub fail_closed: bool,
    pub release_allowed: bool,
    pub production_blocked: bool,
    pub verdict_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ReplayReceiptAdapterVerdict {
    pub fn new(config: &Config, source: &SourceBundle, counters: &Counters, roots: &Roots) -> Self {
        let replay_receipt_adapted = counters.replay_lane_count >= config.min_replay_lanes
            && counters.adapted_count == counters.replay_lane_count
            && counters.deterministic_transcript_count >= config.min_deterministic_transcripts;
        let deferred_to_live_replacement =
            counters.deferred_to_live_replacement_count > 0 || !source.live_replacement_available;
        let divergence_found = counters.divergent_count > 0 || counters.missing_root_count > 0;
        let fail_closed = config.fail_closed_on_divergence
            && (divergence_found || counters.fail_closed_count > 0);
        let release_allowed =
            replay_receipt_adapted && !deferred_to_live_replacement && !fail_closed;
        let production_blocked =
            !release_allowed || fail_closed || config.defer_to_live_replacement;
        let verdict_status = verdict_status(
            replay_receipt_adapted,
            deferred_to_live_replacement,
            divergence_found,
            fail_closed,
        )
        .to_string();
        let user_escape_answer = user_escape_answer(release_allowed, fail_closed).to_string();
        let production_answer =
            production_answer(production_blocked, deferred_to_live_replacement).to_string();
        let verdict_root = verdict_root(
            config,
            source,
            counters,
            roots,
            replay_receipt_adapted,
            deferred_to_live_replacement,
            divergence_found,
            fail_closed,
            release_allowed,
            production_blocked,
            &verdict_status,
        );
        Self {
            replay_receipt_adapted,
            deferred_to_live_replacement,
            divergence_found,
            fail_closed,
            release_allowed,
            production_blocked,
            verdict_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_receipt_adapted": self.replay_receipt_adapted,
            "deferred_to_live_replacement": self.deferred_to_live_replacement,
            "divergence_found": self.divergence_found,
            "fail_closed": self.fail_closed,
            "release_allowed": self.release_allowed,
            "production_blocked": self.production_blocked,
            "verdict_status": self.verdict_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.verdict_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub adaptations: Vec<ReplayReceiptAdaptation>,
    pub roots: Roots,
    pub counters: Counters,
    pub verdict: ReplayReceiptAdapterVerdict,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let adaptations = ReplayLaneKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane_kind)| {
                ReplayReceiptAdaptation::new(&config, &source, *lane_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let counters = Counters::new(&source, &adaptations);
        let roots = Roots::new(&config, &source, &adaptations, &counters);
        let verdict = ReplayReceiptAdapterVerdict::new(&config, &source, &counters, &roots);
        let state_commitment_root =
            state_commitment_root(&config, &source, &roots, &counters, &verdict);
        Ok(Self {
            config,
            source,
            adaptations,
            roots,
            counters,
            verdict,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_runtime_execution_replay_receipt_adapter_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "adaptations": self.adaptations.iter().map(ReplayReceiptAdaptation::public_record).collect::<Vec<_>>(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
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
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RUNTIME-EXECUTION-REPLAY-RECEIPT-ADAPTER-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn lane_replay_root(source: &SourceBundle, lane_kind: ReplayLaneKind) -> String {
    match lane_kind {
        ReplayLaneKind::DevnetReplayRoots => source.deferred_fixture_root.clone(),
        ReplayLaneKind::ForceExitClosureStateRoots => source.package_action_root.clone(),
        ReplayLaneKind::ObservedMoneroAnchorRoots => source.observed_submission_bundle_root.clone(),
        ReplayLaneKind::WalletReplayRoots => source.execution_receipt_root.clone(),
        ReplayLaneKind::DeterministicTranscriptRoots => source.execution_receipt_state_root.clone(),
    }
}

fn lane_closure_state_root(source: &SourceBundle, lane_kind: ReplayLaneKind) -> String {
    match lane_kind {
        ReplayLaneKind::DevnetReplayRoots => source.package_state_root.clone(),
        ReplayLaneKind::ForceExitClosureStateRoots => source.package_closure_root.clone(),
        ReplayLaneKind::ObservedMoneroAnchorRoots => source.fail_closed_hold_root.clone(),
        ReplayLaneKind::WalletReplayRoots => source.execution_receipt_state_root.clone(),
        ReplayLaneKind::DeterministicTranscriptRoots => source.live_feed_state_root.clone(),
    }
}

fn lane_monero_anchor_root(source: &SourceBundle, lane_kind: ReplayLaneKind) -> String {
    match lane_kind {
        ReplayLaneKind::DevnetReplayRoots => source.observed_submission_bundle_root.clone(),
        ReplayLaneKind::ForceExitClosureStateRoots => source.live_feed_root.clone(),
        ReplayLaneKind::ObservedMoneroAnchorRoots => source.live_feed_root.clone(),
        ReplayLaneKind::WalletReplayRoots => source.observed_submission_bundle_root.clone(),
        ReplayLaneKind::DeterministicTranscriptRoots => source.deferred_fixture_root.clone(),
    }
}

fn lane_wallet_replay_root(source: &SourceBundle, lane_kind: ReplayLaneKind) -> String {
    match lane_kind {
        ReplayLaneKind::DevnetReplayRoots => source.execution_receipt_root.clone(),
        ReplayLaneKind::ForceExitClosureStateRoots => source.execution_receipt_root.clone(),
        ReplayLaneKind::ObservedMoneroAnchorRoots => source.execution_receipt_state_root.clone(),
        ReplayLaneKind::WalletReplayRoots => source.package_action_root.clone(),
        ReplayLaneKind::DeterministicTranscriptRoots => source.package_state_root.clone(),
    }
}

fn lane_deterministic_transcript_root(source: &SourceBundle, lane_kind: ReplayLaneKind) -> String {
    match lane_kind {
        ReplayLaneKind::DevnetReplayRoots => source.execution_receipt_state_root.clone(),
        ReplayLaneKind::ForceExitClosureStateRoots => {
            source.observed_submission_bundle_root.clone()
        }
        ReplayLaneKind::ObservedMoneroAnchorRoots => source.package_closure_root.clone(),
        ReplayLaneKind::WalletReplayRoots => source.deferred_fixture_root.clone(),
        ReplayLaneKind::DeterministicTranscriptRoots => source.fail_closed_hold_root.clone(),
    }
}

fn adaptation_status(
    missing_root: bool,
    deferred_to_live_replacement: bool,
    divergent: bool,
    fail_closed: bool,
) -> ReplayAdaptationStatus {
    if fail_closed {
        ReplayAdaptationStatus::FailClosed
    } else if missing_root {
        ReplayAdaptationStatus::MissingRoot
    } else if divergent {
        ReplayAdaptationStatus::Divergent
    } else if deferred_to_live_replacement {
        ReplayAdaptationStatus::DeferredToLiveReplacement
    } else {
        ReplayAdaptationStatus::Adapted
    }
}

fn expected_receipt_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ReplayLaneKind,
    ordinal: u64,
    replay_root: &str,
    closure_state_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RUNTIME-EXECUTION-REPLAY-EXPECTED-RECEIPT",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.package_state_root),
            HashPart::Str(lane_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(replay_root),
            HashPart::Str(closure_state_root),
        ],
        32,
    )
}

fn adapted_receipt_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ReplayLaneKind,
    expected_receipt_root: &str,
    monero_anchor_root: &str,
    wallet_replay_root: &str,
    deterministic_transcript_root: &str,
    status: ReplayAdaptationStatus,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RUNTIME-EXECUTION-REPLAY-ADAPTED-RECEIPT",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.execution_receipt_state_root),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(expected_receipt_root),
            HashPart::Str(monero_anchor_root),
            HashPart::Str(wallet_replay_root),
            HashPart::Str(deterministic_transcript_root),
            HashPart::Str(status.as_str()),
        ],
        32,
    )
}

fn divergence_finding_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ReplayLaneKind,
    adapted_receipt_root: &str,
    status: ReplayAdaptationStatus,
    finding: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RUNTIME-EXECUTION-REPLAY-DIVERGENCE-FINDING",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.state_root()),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(adapted_receipt_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(finding),
        ],
        32,
    )
}

fn adaptation_id(lane_kind: ReplayLaneKind, ordinal: u64, adapted_receipt_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-RUNTIME-EXECUTION-REPLAY-ADAPTATION-ID",
        &[
            HashPart::Str(lane_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(adapted_receipt_root),
        ],
        16,
    )
}

fn lane_vector_root<F>(domain: &str, adaptations: &[ReplayReceiptAdaptation], root_for: F) -> String
where
    F: Fn(&ReplayReceiptAdaptation) -> String,
{
    merkle_root(
        domain,
        &adaptations
            .iter()
            .map(|adaptation| {
                json!({
                    "lane_kind": adaptation.lane_kind.as_str(),
                    "root": root_for(adaptation),
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn divergence_vector_root(adaptations: &[ReplayReceiptAdaptation]) -> String {
    merkle_root(
        "MONERO-L2-PQ-REPLAY-ADAPTER-DIVERGENCE-FINDINGS",
        &adaptations
            .iter()
            .filter(|adaptation| adaptation.status.blocks_release())
            .map(|adaptation| {
                json!({
                    "adaptation_id": adaptation.adaptation_id,
                    "lane_kind": adaptation.lane_kind.as_str(),
                    "divergence_finding_root": adaptation.divergence_finding_root,
                    "status": adaptation.status.as_str(),
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn live_replacement_verdict_root(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    adaptation_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-REPLAY-ADAPTER-LIVE-REPLACEMENT-VERDICT",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.live_feed_state_root),
            HashPart::Str(adaptation_receipt_root),
            HashPart::U64(counters.deferred_to_live_replacement_count),
            HashPart::Str(bool_str(source.live_replacement_available)),
            HashPart::Str(bool_str(config.defer_to_live_replacement)),
        ],
        32,
    )
}

fn fail_closed_status_root(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    divergence_finding_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-REPLAY-ADAPTER-FAIL-CLOSED-STATUS",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.fail_closed_hold_root),
            HashPart::Str(divergence_finding_root),
            HashPart::U64(counters.fail_closed_count),
            HashPart::U64(counters.divergent_count),
            HashPart::U64(counters.missing_root_count),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    roots: &Roots,
    replay_receipt_adapted: bool,
    deferred_to_live_replacement: bool,
    divergence_found: bool,
    fail_closed: bool,
    release_allowed: bool,
    production_blocked: bool,
    verdict_status: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-REPLAY-ADAPTER-VERDICT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(bool_str(replay_receipt_adapted)),
            HashPart::Str(bool_str(deferred_to_live_replacement)),
            HashPart::Str(bool_str(divergence_found)),
            HashPart::Str(bool_str(fail_closed)),
            HashPart::Str(bool_str(release_allowed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(verdict_status),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    roots: &Roots,
    counters: &Counters,
    verdict: &ReplayReceiptAdapterVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-PACKAGE-RUNTIME-EXECUTION-REPLAY-RECEIPT-ADAPTER-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn count_status(adaptations: &[ReplayReceiptAdaptation], status: ReplayAdaptationStatus) -> u64 {
    adaptations
        .iter()
        .filter(|adaptation| adaptation.status == status)
        .count() as u64
}

fn finding(status: ReplayAdaptationStatus, lane_kind: ReplayLaneKind) -> &'static str {
    match status {
        ReplayAdaptationStatus::Adapted => {
            "runtime execution replay receipt adapted deterministically"
        }
        ReplayAdaptationStatus::DeferredToLiveReplacement => {
            "devnet replay receipt remains deferred until live replacement evidence arrives"
        }
        ReplayAdaptationStatus::Divergent => match lane_kind {
            ReplayLaneKind::ObservedMoneroAnchorRoots => {
                "observed Monero anchor root diverges from deterministic replay transcript"
            }
            ReplayLaneKind::WalletReplayRoots => {
                "wallet replay root diverges from force-exit closure state"
            }
            _ => "runtime execution replay receipt diverges from canonical closure roots",
        },
        ReplayAdaptationStatus::MissingRoot => {
            "runtime execution replay receipt is missing a required canonical root"
        }
        ReplayAdaptationStatus::FailClosed => {
            "runtime execution replay receipt adapter failed closed"
        }
    }
}

fn verdict_status(
    replay_receipt_adapted: bool,
    deferred_to_live_replacement: bool,
    divergence_found: bool,
    fail_closed: bool,
) -> &'static str {
    if fail_closed {
        "fail_closed"
    } else if divergence_found {
        "divergence_found"
    } else if deferred_to_live_replacement {
        "deferred_to_live_replacement"
    } else if replay_receipt_adapted {
        "runtime_execution_replay_receipt_adapted"
    } else {
        "waiting_for_replay_receipt_adaptation"
    }
}

fn user_escape_answer(release_allowed: bool, fail_closed: bool) -> &'static str {
    if release_allowed {
        "user_escape_replay_receipt_adapter_accepts_runtime_execution_receipts"
    } else if fail_closed {
        "user_escape_replay_receipt_adapter_fails_closed_until_live_roots_replace_devnet_replay"
    } else {
        "user_escape_waits_for_runtime_execution_replay_receipt_adaptation"
    }
}

fn production_answer(production_blocked: bool, deferred_to_live_replacement: bool) -> &'static str {
    if production_blocked && deferred_to_live_replacement {
        "production_held_until_live_replacement_verdict_replaces_replay_receipts"
    } else if production_blocked {
        "production_held_by_runtime_execution_replay_receipt_adapter"
    } else {
        "production_release_allowed_after_runtime_execution_replay_receipt_adaptation"
    }
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "runtime execution replay receipt adapter chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "runtime execution replay receipt adapter protocol mismatch",
    )?;
    ensure(
        config.min_replay_lanes > 0,
        "runtime execution replay receipt adapter requires replay lanes",
    )?;
    ensure(
        config.min_deterministic_transcripts > 0,
        "runtime execution replay receipt adapter requires deterministic transcripts",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.package_state_root.is_empty(),
        "runtime execution replay receipt adapter missing package state root",
    )?;
    ensure(
        !source.execution_receipt_state_root.is_empty(),
        "runtime execution replay receipt adapter missing execution receipt state root",
    )?;
    ensure(
        !source.live_feed_state_root.is_empty(),
        "runtime execution replay receipt adapter missing live feed state root",
    )?;
    ensure(
        source.package_action_count > 0,
        "runtime execution replay receipt adapter requires package actions",
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
        package_closure_root: record_root("fallback-package-closure", &json!({"reason": &reason})),
        execution_receipt_state_root: record_root(
            "fallback-execution-receipt-state",
            &json!({"reason": &reason}),
        ),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({"reason": &reason}),
        ),
        observed_submission_bundle_root: record_root(
            "fallback-observed-submission",
            &json!({"reason": &reason}),
        ),
        live_feed_state_root: record_root("fallback-live-feed-state", &json!({"reason": &reason})),
        deferred_fixture_root: record_root(
            "fallback-deferred-fixture",
            &json!({"reason": &reason}),
        ),
        live_feed_root: record_root("fallback-live-feed", &json!({"reason": &reason})),
        fail_closed_hold_root: record_root(
            "fallback-fail-closed-hold",
            &json!({"reason": &reason}),
        ),
        devnet_fixture_roots_present: true,
        live_replacement_available: false,
        package_action_count: 1,
        execution_receipt_count: 1,
        wallet_replay_count: 1,
        deterministic_transcript_count: 1,
    };
    let adaptations = ReplayLaneKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, lane_kind)| {
            ReplayReceiptAdaptation::new(&config, &source, *lane_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let counters = Counters::new(&source, &adaptations);
    let roots = Roots::new(&config, &source, &adaptations, &counters);
    let verdict = ReplayReceiptAdapterVerdict::new(&config, &source, &counters, &roots);
    let state_commitment_root =
        state_commitment_root(&config, &source, &roots, &counters, &verdict);
    State {
        config,
        source,
        adaptations,
        roots,
        counters,
        verdict,
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
