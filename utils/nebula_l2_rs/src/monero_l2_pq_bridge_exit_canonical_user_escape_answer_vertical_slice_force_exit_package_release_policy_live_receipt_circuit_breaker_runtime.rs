use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReleasePolicyLiveReceiptCircuitBreakerRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_LIVE_RECEIPT_CIRCUIT_BREAKER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-release-policy-live-receipt-circuit-breaker-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_LIVE_RECEIPT_CIRCUIT_BREAKER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const CIRCUIT_BREAKER_SUITE: &str =
    "monero-l2-pq-force-exit-package-release-policy-live-receipt-circuit-breaker-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-devnet-v1";
pub const DEFAULT_FORCE_EXIT_PACKAGE_ID: &str =
    "force-exit-release-policy-live-receipt-circuit-breaker-devnet-0001";
pub const DEFAULT_RELEASE_EPOCH: u64 = 79;
pub const DEFAULT_L2_HEIGHT: u64 = 895_079;
pub const DEFAULT_MONERO_HEIGHT: u64 = 3_079_895;
pub const DEFAULT_MIN_ENFORCED_LANES: u64 = 7;
pub const DEFAULT_MIN_RELEASE_WEIGHT: u64 = 100;
pub const DEFAULT_MAX_ALLOWED_HOLDS: u64 = 0;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub circuit_breaker_suite: String,
    pub vertical_slice_id: String,
    pub force_exit_package_id: String,
    pub release_epoch: u64,
    pub l2_height: u64,
    pub monero_height: u64,
    pub min_enforced_lanes: u64,
    pub min_release_weight: u64,
    pub max_allowed_holds: u64,
    pub require_adjudicator_root: bool,
    pub require_release_manifest_root: bool,
    pub require_operator_ack_root: bool,
    pub require_wallet_notice_root: bool,
    pub fail_closed_on_any_hold: bool,
    pub fail_closed_on_missing_lane: bool,
    pub fail_closed_on_stale_adjudicator: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            circuit_breaker_suite: CIRCUIT_BREAKER_SUITE.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            force_exit_package_id: DEFAULT_FORCE_EXIT_PACKAGE_ID.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            l2_height: DEFAULT_L2_HEIGHT,
            monero_height: DEFAULT_MONERO_HEIGHT,
            min_enforced_lanes: DEFAULT_MIN_ENFORCED_LANES,
            min_release_weight: DEFAULT_MIN_RELEASE_WEIGHT,
            max_allowed_holds: DEFAULT_MAX_ALLOWED_HOLDS,
            require_adjudicator_root: true,
            require_release_manifest_root: true,
            require_operator_ack_root: true,
            require_wallet_notice_root: true,
            fail_closed_on_any_hold: true,
            fail_closed_on_missing_lane: true,
            fail_closed_on_stale_adjudicator: true,
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
            "circuit_breaker_suite": self.circuit_breaker_suite,
            "vertical_slice_id": self.vertical_slice_id,
            "force_exit_package_id": self.force_exit_package_id,
            "release_epoch": self.release_epoch,
            "l2_height": self.l2_height,
            "monero_height": self.monero_height,
            "min_enforced_lanes": self.min_enforced_lanes,
            "min_release_weight": self.min_release_weight,
            "max_allowed_holds": self.max_allowed_holds,
            "require_adjudicator_root": self.require_adjudicator_root,
            "require_release_manifest_root": self.require_release_manifest_root,
            "require_operator_ack_root": self.require_operator_ack_root,
            "require_wallet_notice_root": self.require_wallet_notice_root,
            "fail_closed_on_any_hold": self.fail_closed_on_any_hold,
            "fail_closed_on_missing_lane": self.fail_closed_on_missing_lane,
            "fail_closed_on_stale_adjudicator": self.fail_closed_on_stale_adjudicator,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakerLane {
    CompileRuntimeManifest,
    RuntimeReplayManifest,
    AuditSecurityManifest,
    BridgeCustodyManifest,
    WalletWatchtowerManifest,
    PqReservePrivacyManifest,
    ReleaseAdjudicator,
}

impl BreakerLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileRuntimeManifest => "compile_runtime_manifest",
            Self::RuntimeReplayManifest => "runtime_replay_manifest",
            Self::AuditSecurityManifest => "audit_security_manifest",
            Self::BridgeCustodyManifest => "bridge_custody_manifest",
            Self::WalletWatchtowerManifest => "wallet_watchtower_manifest",
            Self::PqReservePrivacyManifest => "pq_reserve_privacy_manifest",
            Self::ReleaseAdjudicator => "release_adjudicator",
        }
    }

    pub fn release_weight(self) -> u64 {
        match self {
            Self::CompileRuntimeManifest => 18,
            Self::RuntimeReplayManifest => 16,
            Self::AuditSecurityManifest => 20,
            Self::BridgeCustodyManifest => 14,
            Self::WalletWatchtowerManifest => 12,
            Self::PqReservePrivacyManifest => 15,
            Self::ReleaseAdjudicator => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementStatus {
    Enforced,
    Held,
    Missing,
    Stale,
    Mismatch,
    ReviewerPending,
}

impl EnforcementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::Held => "held",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::Mismatch => "mismatch",
            Self::ReviewerPending => "reviewer_pending",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Enforced)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakerTripReason {
    None,
    MissingLane,
    HeldLane,
    StaleLane,
    RootMismatch,
    ReviewerPending,
    ManifestPolicyGap,
    InsufficientWeight,
}

impl BreakerTripReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MissingLane => "missing_lane",
            Self::HeldLane => "held_lane",
            Self::StaleLane => "stale_lane",
            Self::RootMismatch => "root_mismatch",
            Self::ReviewerPending => "reviewer_pending",
            Self::ManifestPolicyGap => "manifest_policy_gap",
            Self::InsufficientWeight => "insufficient_weight",
        }
    }

    pub fn is_clear(self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakerDecision {
    ReleasePermitted,
    ReleaseHeld,
}

impl BreakerDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleasePermitted => "release_permitted",
            Self::ReleaseHeld => "release_held",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::ReleasePermitted)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementInput {
    pub input_id: String,
    pub lane: BreakerLane,
    pub enforcement_runtime: String,
    pub activation_root: String,
    pub adjudicator_root: String,
    pub release_manifest_root: String,
    pub replacement_manifest_root: String,
    pub operator_ack_root: String,
    pub wallet_notice_root: String,
    pub observed_l2_height: u64,
    pub observed_monero_height: u64,
    pub reviewer_count: u64,
    pub required_reviewer_count: u64,
    pub status: EnforcementStatus,
    pub hold_reason: String,
}

impl EnforcementInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lane: BreakerLane,
        enforcement_runtime: &str,
        activation_root: &str,
        adjudicator_root: &str,
        release_manifest_root: &str,
        replacement_manifest_root: &str,
        operator_ack_root: &str,
        wallet_notice_root: &str,
        observed_l2_height: u64,
        observed_monero_height: u64,
        reviewer_count: u64,
        required_reviewer_count: u64,
        status: EnforcementStatus,
        hold_reason: &str,
    ) -> Self {
        let input_id = enforcement_input_id(
            lane,
            enforcement_runtime,
            activation_root,
            adjudicator_root,
            release_manifest_root,
            replacement_manifest_root,
            operator_ack_root,
            wallet_notice_root,
            observed_l2_height,
            observed_monero_height,
            status,
            hold_reason,
        );
        Self {
            input_id,
            lane,
            enforcement_runtime: enforcement_runtime.to_string(),
            activation_root: activation_root.to_string(),
            adjudicator_root: adjudicator_root.to_string(),
            release_manifest_root: release_manifest_root.to_string(),
            replacement_manifest_root: replacement_manifest_root.to_string(),
            operator_ack_root: operator_ack_root.to_string(),
            wallet_notice_root: wallet_notice_root.to_string(),
            observed_l2_height,
            observed_monero_height,
            reviewer_count,
            required_reviewer_count,
            status,
            hold_reason: hold_reason.to_string(),
        }
    }

    pub fn reviewer_quorum_met(&self) -> bool {
        self.reviewer_count >= self.required_reviewer_count
    }

    pub fn has_policy_roots(&self) -> bool {
        !self.activation_root.is_empty()
            && !self.adjudicator_root.is_empty()
            && !self.release_manifest_root.is_empty()
            && !self.replacement_manifest_root.is_empty()
            && !self.operator_ack_root.is_empty()
            && !self.wallet_notice_root.is_empty()
    }

    pub fn is_enforced(&self) -> bool {
        self.status.permits_release() && self.reviewer_quorum_met() && self.has_policy_roots()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "input_id": self.input_id,
            "lane": self.lane.as_str(),
            "enforcement_runtime": self.enforcement_runtime,
            "activation_root": self.activation_root,
            "adjudicator_root": self.adjudicator_root,
            "release_manifest_root": self.release_manifest_root,
            "replacement_manifest_root": self.replacement_manifest_root,
            "operator_ack_root": self.operator_ack_root,
            "wallet_notice_root": self.wallet_notice_root,
            "observed_l2_height": self.observed_l2_height,
            "observed_monero_height": self.observed_monero_height,
            "reviewer_count": self.reviewer_count,
            "required_reviewer_count": self.required_reviewer_count,
            "status": self.status.as_str(),
            "hold_reason": self.hold_reason,
            "reviewer_quorum_met": self.reviewer_quorum_met(),
            "has_policy_roots": self.has_policy_roots(),
            "is_enforced": self.is_enforced(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-input", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BreakerLaneTrip {
    pub trip_id: String,
    pub lane: BreakerLane,
    pub input_root: String,
    pub trip_reason: BreakerTripReason,
    pub release_weight: u64,
    pub contributes_release_weight: bool,
    pub release_blocked: bool,
    pub public_notice_root: String,
}

impl BreakerLaneTrip {
    pub fn from_input(input: &EnforcementInput) -> Self {
        let input_root = input.state_root();
        let trip_reason = lane_trip_reason(input);
        let release_blocked = !trip_reason.is_clear();
        let contributes_release_weight = input.is_enforced() && trip_reason.is_clear();
        let release_weight = if contributes_release_weight {
            input.lane.release_weight()
        } else {
            0
        };
        let public_notice_root = public_notice_root(input.lane, trip_reason, &input.hold_reason);
        let trip_id = breaker_lane_trip_id(
            input.lane,
            &input_root,
            trip_reason,
            release_weight,
            release_blocked,
            &public_notice_root,
        );
        Self {
            trip_id,
            lane: input.lane,
            input_root,
            trip_reason,
            release_weight,
            contributes_release_weight,
            release_blocked,
            public_notice_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "trip_id": self.trip_id,
            "lane": self.lane.as_str(),
            "input_root": self.input_root,
            "trip_reason": self.trip_reason.as_str(),
            "release_weight": self.release_weight,
            "contributes_release_weight": self.contributes_release_weight,
            "release_blocked": self.release_blocked,
            "public_notice_root": self.public_notice_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("breaker-lane-trip", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lanes_seen: u64,
    pub lanes_enforced: u64,
    pub lanes_held: u64,
    pub lanes_missing: u64,
    pub lanes_stale: u64,
    pub lanes_mismatched: u64,
    pub lanes_reviewer_pending: u64,
    pub lanes_policy_gap: u64,
    pub release_weight: u64,
    pub blocked_lanes: u64,
}

impl Counters {
    pub fn from_trips(inputs: &[EnforcementInput], trips: &[BreakerLaneTrip]) -> Self {
        let mut counters = Self::default();
        counters.lanes_seen = inputs.len() as u64;
        for input in inputs {
            if input.is_enforced() {
                counters.lanes_enforced = counters.lanes_enforced.saturating_add(1);
            }
            match input.status {
                EnforcementStatus::Held => {
                    counters.lanes_held = counters.lanes_held.saturating_add(1)
                }
                EnforcementStatus::Missing => {
                    counters.lanes_missing = counters.lanes_missing.saturating_add(1)
                }
                EnforcementStatus::Stale => {
                    counters.lanes_stale = counters.lanes_stale.saturating_add(1)
                }
                EnforcementStatus::Mismatch => {
                    counters.lanes_mismatched = counters.lanes_mismatched.saturating_add(1)
                }
                EnforcementStatus::ReviewerPending => {
                    counters.lanes_reviewer_pending =
                        counters.lanes_reviewer_pending.saturating_add(1)
                }
                EnforcementStatus::Enforced => {}
            }
            if !input.has_policy_roots() {
                counters.lanes_policy_gap = counters.lanes_policy_gap.saturating_add(1);
            }
            if !input.reviewer_quorum_met() {
                counters.lanes_reviewer_pending = counters.lanes_reviewer_pending.saturating_add(1);
            }
        }
        for trip in trips {
            counters.release_weight = counters.release_weight.saturating_add(trip.release_weight);
            if trip.release_blocked {
                counters.blocked_lanes = counters.blocked_lanes.saturating_add(1);
            }
        }
        counters
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lanes_seen": self.lanes_seen,
            "lanes_enforced": self.lanes_enforced,
            "lanes_held": self.lanes_held,
            "lanes_missing": self.lanes_missing,
            "lanes_stale": self.lanes_stale,
            "lanes_mismatched": self.lanes_mismatched,
            "lanes_reviewer_pending": self.lanes_reviewer_pending,
            "lanes_policy_gap": self.lanes_policy_gap,
            "release_weight": self.release_weight,
            "blocked_lanes": self.blocked_lanes,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BreakerVerdict {
    pub verdict_id: String,
    pub decision: BreakerDecision,
    pub release_allowed: bool,
    pub trip_reason: BreakerTripReason,
    pub release_manifest_root: String,
    pub operator_action_root: String,
    pub wallet_notice_root: String,
    pub governance_hold_root: String,
    pub detail: String,
}

impl BreakerVerdict {
    pub fn new(config: &Config, counters: &Counters, trips: &[BreakerLaneTrip]) -> Self {
        let trip_reason = final_trip_reason(config, counters);
        let release_allowed = trip_reason.is_clear();
        let decision = if release_allowed {
            BreakerDecision::ReleasePermitted
        } else {
            BreakerDecision::ReleaseHeld
        };
        let release_manifest_root = final_release_manifest_root(counters, trips, decision);
        let operator_action_root =
            operator_action_root(decision, trip_reason, &release_manifest_root);
        let wallet_notice_root =
            final_wallet_notice_root(decision, trip_reason, &release_manifest_root);
        let governance_hold_root =
            governance_hold_root(decision, trip_reason, &operator_action_root);
        let detail = final_detail(config, counters, trip_reason);
        let verdict_id = breaker_verdict_id(
            decision,
            trip_reason,
            &release_manifest_root,
            &operator_action_root,
            &wallet_notice_root,
            &governance_hold_root,
            &detail,
        );
        Self {
            verdict_id,
            decision,
            release_allowed,
            trip_reason,
            release_manifest_root,
            operator_action_root,
            wallet_notice_root,
            governance_hold_root,
            detail,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "decision": self.decision.as_str(),
            "release_allowed": self.release_allowed,
            "trip_reason": self.trip_reason.as_str(),
            "release_manifest_root": self.release_manifest_root,
            "operator_action_root": self.operator_action_root,
            "wallet_notice_root": self.wallet_notice_root,
            "governance_hold_root": self.governance_hold_root,
            "detail": self.detail,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("breaker-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub input_root: String,
    pub lane_trip_root: String,
    pub counter_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "input_root": self.input_root,
            "lane_trip_root": self.lane_trip_root,
            "counter_root": self.counter_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub inputs: Vec<EnforcementInput>,
    pub lane_trips: Vec<BreakerLaneTrip>,
    pub counters: Counters,
    pub verdict: BreakerVerdict,
    pub roots: Roots,
}

impl State {
    pub fn new(config: Config, inputs: Vec<EnforcementInput>) -> Self {
        let lane_trips = inputs
            .iter()
            .map(BreakerLaneTrip::from_input)
            .collect::<Vec<_>>();
        let counters = Counters::from_trips(&inputs, &lane_trips);
        let verdict = BreakerVerdict::new(&config, &counters, &lane_trips);
        let roots = build_roots(&config, &inputs, &lane_trips, &counters, &verdict);
        Self {
            config,
            inputs,
            lane_trips,
            counters,
            verdict,
            roots,
        }
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let inputs = devnet_inputs(&config);
        Self::new(config, inputs)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "inputs": self.inputs.iter().map(EnforcementInput::public_record).collect::<Vec<_>>(),
            "lane_trips": self.lane_trips.iter().map(BreakerLaneTrip::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "roots": self.roots.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root.clone()
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

pub fn devnet_inputs(config: &Config) -> Vec<EnforcementInput> {
    [
        (
            BreakerLane::CompileRuntimeManifest,
            EnforcementStatus::ReviewerPending,
            1,
            2,
            "compile runtime lane still requires live reviewer quorum before release",
        ),
        (
            BreakerLane::RuntimeReplayManifest,
            EnforcementStatus::Held,
            2,
            2,
            "runtime replay lane still carries deferred replay fixtures",
        ),
        (
            BreakerLane::AuditSecurityManifest,
            EnforcementStatus::ReviewerPending,
            3,
            5,
            "audit security lane has not reached final review quorum",
        ),
        (
            BreakerLane::BridgeCustodyManifest,
            EnforcementStatus::Enforced,
            3,
            3,
            "bridge custody lane is enforced in the devnet packet",
        ),
        (
            BreakerLane::WalletWatchtowerManifest,
            EnforcementStatus::Enforced,
            3,
            3,
            "wallet watchtower lane is enforced in the devnet packet",
        ),
        (
            BreakerLane::PqReservePrivacyManifest,
            EnforcementStatus::Mismatch,
            2,
            2,
            "PQ reserve privacy lane contains a deliberate root mismatch",
        ),
        (
            BreakerLane::ReleaseAdjudicator,
            EnforcementStatus::Held,
            2,
            2,
            "release adjudicator remains held while any lane is not enforced",
        ),
    ]
    .iter()
    .map(
        |(lane, status, reviewer_count, required_reviewer_count, hold_reason)| {
            EnforcementInput::new(
                *lane,
                &format!("{}-release-manifest-enforcement-runtime", lane.as_str()),
                &fixture_root("activation", lane.as_str()),
                &fixture_root("adjudicator", lane.as_str()),
                &fixture_root("release-manifest", lane.as_str()),
                &fixture_root("replacement-manifest", lane.as_str()),
                &fixture_root("operator-ack", lane.as_str()),
                &fixture_root("wallet-notice", lane.as_str()),
                config
                    .l2_height
                    .saturating_sub(if matches!(status, EnforcementStatus::Stale) {
                        12
                    } else {
                        2
                    }),
                config.monero_height.saturating_sub(2),
                *reviewer_count,
                *required_reviewer_count,
                *status,
                hold_reason,
            )
        },
    )
    .collect()
}

pub fn lane_trip_reason(input: &EnforcementInput) -> BreakerTripReason {
    if !input.has_policy_roots() {
        return BreakerTripReason::ManifestPolicyGap;
    }
    if !input.reviewer_quorum_met() {
        return BreakerTripReason::ReviewerPending;
    }
    match input.status {
        EnforcementStatus::Enforced => BreakerTripReason::None,
        EnforcementStatus::Held => BreakerTripReason::HeldLane,
        EnforcementStatus::Missing => BreakerTripReason::MissingLane,
        EnforcementStatus::Stale => BreakerTripReason::StaleLane,
        EnforcementStatus::Mismatch => BreakerTripReason::RootMismatch,
        EnforcementStatus::ReviewerPending => BreakerTripReason::ReviewerPending,
    }
}

pub fn final_trip_reason(config: &Config, counters: &Counters) -> BreakerTripReason {
    if config.fail_closed_on_missing_lane && counters.lanes_seen < config.min_enforced_lanes {
        BreakerTripReason::MissingLane
    } else if counters.lanes_policy_gap > 0 {
        BreakerTripReason::ManifestPolicyGap
    } else if counters.lanes_mismatched > 0 {
        BreakerTripReason::RootMismatch
    } else if config.fail_closed_on_stale_adjudicator && counters.lanes_stale > 0 {
        BreakerTripReason::StaleLane
    } else if counters.lanes_reviewer_pending > 0 {
        BreakerTripReason::ReviewerPending
    } else if config.fail_closed_on_any_hold && counters.blocked_lanes > config.max_allowed_holds {
        BreakerTripReason::HeldLane
    } else if counters.release_weight < config.min_release_weight {
        BreakerTripReason::InsufficientWeight
    } else {
        BreakerTripReason::None
    }
}

pub fn final_detail(config: &Config, counters: &Counters, reason: BreakerTripReason) -> String {
    if reason.is_clear() {
        return format!(
            "release permitted with lanes_enforced={} and release_weight={}",
            counters.lanes_enforced, counters.release_weight
        );
    }
    format!(
        "release held by {}; lanes_seen={}/{}, lanes_enforced={}, blocked_lanes={}, holds={}, missing={}, stale={}, mismatch={}, reviewer_pending={}, policy_gap={}, release_weight={}/{}",
        reason.as_str(),
        counters.lanes_seen,
        config.min_enforced_lanes,
        counters.lanes_enforced,
        counters.blocked_lanes,
        counters.lanes_held,
        counters.lanes_missing,
        counters.lanes_stale,
        counters.lanes_mismatched,
        counters.lanes_reviewer_pending,
        counters.lanes_policy_gap,
        counters.release_weight,
        config.min_release_weight,
    )
}

pub fn build_roots(
    config: &Config,
    inputs: &[EnforcementInput],
    lane_trips: &[BreakerLaneTrip],
    counters: &Counters,
    verdict: &BreakerVerdict,
) -> Roots {
    let config_root = config.state_root();
    let input_root = merkle_root(
        "release-policy-live-receipt-circuit-breaker-inputs",
        &inputs
            .iter()
            .map(EnforcementInput::state_root)
            .collect::<Vec<_>>(),
    );
    let lane_trip_root = merkle_root(
        "release-policy-live-receipt-circuit-breaker-trips",
        &lane_trips
            .iter()
            .map(BreakerLaneTrip::state_root)
            .collect::<Vec<_>>(),
    );
    let counter_root = counters.state_root();
    let verdict_root = verdict.state_root();
    let state_root = circuit_breaker_state_root(
        &config_root,
        &input_root,
        &lane_trip_root,
        &counter_root,
        &verdict_root,
    );
    Roots {
        config_root,
        input_root,
        lane_trip_root,
        counter_root,
        verdict_root,
        state_root,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn enforcement_input_id(
    lane: BreakerLane,
    enforcement_runtime: &str,
    activation_root: &str,
    adjudicator_root: &str,
    release_manifest_root: &str,
    replacement_manifest_root: &str,
    operator_ack_root: &str,
    wallet_notice_root: &str,
    observed_l2_height: u64,
    observed_monero_height: u64,
    status: EnforcementStatus,
    hold_reason: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-INPUT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane.as_str()),
            HashPart::Str(enforcement_runtime),
            HashPart::Str(activation_root),
            HashPart::Str(adjudicator_root),
            HashPart::Str(release_manifest_root),
            HashPart::Str(replacement_manifest_root),
            HashPart::Str(operator_ack_root),
            HashPart::Str(wallet_notice_root),
            HashPart::U64(observed_l2_height),
            HashPart::U64(observed_monero_height),
            HashPart::Str(status.as_str()),
            HashPart::Str(hold_reason),
        ],
        32,
    )
}

pub fn breaker_lane_trip_id(
    lane: BreakerLane,
    input_root: &str,
    reason: BreakerTripReason,
    release_weight: u64,
    release_blocked: bool,
    public_notice_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-LANE-TRIP-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane.as_str()),
            HashPart::Str(input_root),
            HashPart::Str(reason.as_str()),
            HashPart::U64(release_weight),
            HashPart::Str(if release_blocked {
                "release-blocked"
            } else {
                "release-clear"
            }),
            HashPart::Str(public_notice_root),
        ],
        32,
    )
}

pub fn breaker_verdict_id(
    decision: BreakerDecision,
    reason: BreakerTripReason,
    release_manifest_root: &str,
    operator_action_root: &str,
    wallet_notice_root: &str,
    governance_hold_root: &str,
    detail: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-VERDICT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(decision.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(release_manifest_root),
            HashPart::Str(operator_action_root),
            HashPart::Str(wallet_notice_root),
            HashPart::Str(governance_hold_root),
            HashPart::Str(detail),
        ],
        32,
    )
}

pub fn public_notice_root(lane: BreakerLane, reason: BreakerTripReason, detail: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-PUBLIC-NOTICE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(detail),
        ],
        32,
    )
}

pub fn final_release_manifest_root(
    counters: &Counters,
    lane_trips: &[BreakerLaneTrip],
    decision: BreakerDecision,
) -> String {
    let counter_record = counters.public_record();
    let trip_root = merkle_root(
        "release-policy-live-receipt-circuit-breaker-final-trip-root",
        &lane_trips
            .iter()
            .map(BreakerLaneTrip::state_root)
            .collect::<Vec<_>>(),
    );
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-FINAL-MANIFEST",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Json(&counter_record),
            HashPart::Str(&trip_root),
            HashPart::Str(decision.as_str()),
        ],
        32,
    )
}

pub fn operator_action_root(
    decision: BreakerDecision,
    reason: BreakerTripReason,
    release_manifest_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-OPERATOR-ACTION",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(decision.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(release_manifest_root),
        ],
        32,
    )
}

pub fn final_wallet_notice_root(
    decision: BreakerDecision,
    reason: BreakerTripReason,
    release_manifest_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-WALLET-NOTICE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(decision.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(release_manifest_root),
        ],
        32,
    )
}

pub fn governance_hold_root(
    decision: BreakerDecision,
    reason: BreakerTripReason,
    operator_action_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-GOVERNANCE-HOLD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(decision.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(operator_action_root),
        ],
        32,
    )
}

pub fn circuit_breaker_state_root(
    config_root: &str,
    input_root: &str,
    lane_trip_root: &str,
    counter_root: &str,
    verdict_root: &str,
) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-STATE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(config_root),
            HashPart::Str(input_root),
            HashPart::Str(lane_trip_root),
            HashPart::Str(counter_root),
            HashPart::Str(verdict_root),
        ],
        32,
    )
}

pub fn fixture_root(kind: &str, value: &str) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-FIXTURE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(value),
        ],
        32,
    )
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "RELEASE-POLICY-LIVE-RECEIPT-CIRCUIT-BREAKER-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}
