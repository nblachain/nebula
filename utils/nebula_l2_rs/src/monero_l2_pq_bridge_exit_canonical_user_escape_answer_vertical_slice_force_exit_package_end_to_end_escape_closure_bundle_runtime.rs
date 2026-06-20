use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_adversarial_replay_matrix_runtime as adversarial,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_cross_receipt_consistency_runtime as consistency,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_custody_release_policy_runtime as custody,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_live_feed_reconciliation_runtime as live_feed,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_private_state_continuity_runtime as continuity,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_user_escape_verdict_bundle_runtime as user_verdict,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageEndToEndEscapeClosureBundleRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_END_TO_END_ESCAPE_CLOSURE_BUNDLE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-end-to-end-escape-closure-bundle-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_END_TO_END_ESCAPE_CLOSURE_BUNDLE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ESCAPE_CLOSURE_SUITE: &str =
    "monero-l2-pq-force-exit-package-end-to-end-escape-closure-v1";
pub const DEFAULT_MIN_CLOSURE_LANES: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub closure_suite: String,
    pub min_closure_lanes: u64,
    pub require_consistency: bool,
    pub require_user_verdict: bool,
    pub require_live_feed_reconciled: bool,
    pub require_adversarial_replay: bool,
    pub require_custody_policy: bool,
    pub require_private_state_continuity: bool,
    pub hold_production_until_live_closure: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            closure_suite: ESCAPE_CLOSURE_SUITE.to_string(),
            min_closure_lanes: DEFAULT_MIN_CLOSURE_LANES,
            require_consistency: true,
            require_user_verdict: true,
            require_live_feed_reconciled: true,
            require_adversarial_replay: true,
            require_custody_policy: true,
            require_private_state_continuity: true,
            hold_production_until_live_closure: true,
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
            "closure_suite": self.closure_suite,
            "min_closure_lanes": self.min_closure_lanes,
            "require_consistency": self.require_consistency,
            "require_user_verdict": self.require_user_verdict,
            "require_live_feed_reconciled": self.require_live_feed_reconciled,
            "require_adversarial_replay": self.require_adversarial_replay,
            "require_custody_policy": self.require_custody_policy,
            "require_private_state_continuity": self.require_private_state_continuity,
            "hold_production_until_live_closure": self.hold_production_until_live_closure,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosureLaneKind {
    CrossReceiptConsistency,
    UserEscapeVerdict,
    LiveFeedReconciliation,
    AdversarialReplay,
    CustodyReleasePolicy,
    PrivateStateContinuity,
}

impl ClosureLaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CrossReceiptConsistency => "cross_receipt_consistency",
            Self::UserEscapeVerdict => "user_escape_verdict",
            Self::LiveFeedReconciliation => "live_feed_reconciliation",
            Self::AdversarialReplay => "adversarial_replay",
            Self::CustodyReleasePolicy => "custody_release_policy",
            Self::PrivateStateContinuity => "private_state_continuity",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosureStatus {
    ClosedForUserEscape,
    WaitingForLiveEvidence,
    Blocked,
}

impl ClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClosedForUserEscape => "closed_for_user_escape",
            Self::WaitingForLiveEvidence => "waiting_for_live_evidence",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub consistency_state_root: String,
    pub consistency_record_root: String,
    pub consistency_ready: bool,
    pub user_verdict_state_root: String,
    pub user_verdict_bundle_root: String,
    pub user_verdict_ready: bool,
    pub live_feed_state_root: String,
    pub live_feed_record_root: String,
    pub live_feed_ready: bool,
    pub adversarial_state_root: String,
    pub adversarial_record_root: String,
    pub adversarial_ready: bool,
    pub custody_state_root: String,
    pub custody_record_root: String,
    pub custody_ready: bool,
    pub continuity_state_root: String,
    pub continuity_record_root: String,
    pub continuity_ready: bool,
}

impl SourceBundle {
    pub fn from_states(
        consistency_state: &consistency::State,
        user_verdict_state: &user_verdict::State,
        live_feed_state: &live_feed::State,
        adversarial_state: &adversarial::State,
        custody_state: &custody::State,
        continuity_state: &continuity::State,
    ) -> Self {
        let live_feed_record = live_feed_state.public_record();
        let adversarial_record = adversarial_state.public_record();
        let custody_record = custody_state.public_record();
        let continuity_record = continuity_state.public_record();
        Self {
            consistency_state_root: consistency_state.state_root(),
            consistency_record_root: consistency_state.consistency_record_root.clone(),
            consistency_ready: consistency_state.verdict.user_escape_consistent,
            user_verdict_state_root: user_verdict_state.state_root(),
            user_verdict_bundle_root: user_verdict_state.step_bundle_root.clone(),
            user_verdict_ready: user_verdict_state.verdict.force_exit_safe_to_attempt,
            live_feed_state_root: live_feed_state.state_root(),
            live_feed_record_root: record_root("live-feed-record", &live_feed_record),
            live_feed_ready: bool_any(
                &live_feed_record,
                &[
                    "live_feed_reconciled",
                    "live_feeds_reconciled",
                    "all_live_feeds_reconciled",
                    "all_placeholders_replaced",
                    "live_reconciliation_complete",
                ],
            ),
            adversarial_state_root: adversarial_state.state_root(),
            adversarial_record_root: record_root("adversarial-record", &adversarial_record),
            adversarial_ready: bool_any(
                &adversarial_record,
                &[
                    "adversarial_replay_passed",
                    "all_cases_fail_closed",
                    "matrix_fail_closed",
                    "hostile_cases_contained",
                ],
            ),
            custody_state_root: custody_state.state_root(),
            custody_record_root: record_root("custody-record", &custody_record),
            custody_ready: bool_any(
                &custody_record,
                &[
                    "release_policy_satisfied",
                    "custody_policy_satisfied",
                    "release_policy_ready",
                    "custody_release_authorized",
                ],
            ),
            continuity_state_root: continuity_state.state_root(),
            continuity_record_root: record_root("continuity-record", &continuity_record),
            continuity_ready: bool_any(
                &continuity_record,
                &[
                    "private_state_continuity_preserved",
                    "continuity_preserved",
                    "nullifier_continuity_preserved",
                    "privacy_continuity_verified",
                ],
            ),
        }
    }

    pub fn devnet() -> Self {
        Self::from_states(
            &consistency::devnet(),
            &user_verdict::devnet(),
            &live_feed::devnet(),
            &adversarial::devnet(),
            &custody::devnet(),
            &continuity::devnet(),
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "consistency_state_root": self.consistency_state_root,
            "consistency_record_root": self.consistency_record_root,
            "consistency_ready": self.consistency_ready,
            "user_verdict_state_root": self.user_verdict_state_root,
            "user_verdict_bundle_root": self.user_verdict_bundle_root,
            "user_verdict_ready": self.user_verdict_ready,
            "live_feed_state_root": self.live_feed_state_root,
            "live_feed_record_root": self.live_feed_record_root,
            "live_feed_ready": self.live_feed_ready,
            "adversarial_state_root": self.adversarial_state_root,
            "adversarial_record_root": self.adversarial_record_root,
            "adversarial_ready": self.adversarial_ready,
            "custody_state_root": self.custody_state_root,
            "custody_record_root": self.custody_record_root,
            "custody_ready": self.custody_ready,
            "continuity_state_root": self.continuity_state_root,
            "continuity_record_root": self.continuity_record_root,
            "continuity_ready": self.continuity_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClosureLane {
    pub lane_kind: ClosureLaneKind,
    pub state_root: String,
    pub evidence_root: String,
    pub ready: bool,
    pub required: bool,
    pub status: ClosureStatus,
    pub closure_root: String,
    pub hold_reason: String,
}

impl ClosureLane {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        lane_kind: ClosureLaneKind,
        state_root: String,
        evidence_root: String,
        ready: bool,
        required: bool,
    ) -> Self {
        let status = if ready {
            ClosureStatus::ClosedForUserEscape
        } else if required {
            ClosureStatus::WaitingForLiveEvidence
        } else {
            ClosureStatus::Blocked
        };
        let hold_reason = hold_reason(lane_kind, status).to_string();
        let closure_root = closure_root(
            config,
            source,
            lane_kind,
            &state_root,
            &evidence_root,
            ready,
            required,
            status,
            &hold_reason,
        );
        Self {
            lane_kind,
            state_root,
            evidence_root,
            ready,
            required,
            status,
            closure_root,
            hold_reason,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_kind": self.lane_kind.as_str(),
            "state_root": self.state_root,
            "evidence_root": self.evidence_root,
            "ready": self.ready,
            "required": self.required,
            "status": self.status.as_str(),
            "closure_root": self.closure_root,
            "hold_reason": self.hold_reason,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClosureVerdict {
    pub lane_count: u64,
    pub closed_count: u64,
    pub waiting_count: u64,
    pub blocked_count: u64,
    pub required_lane_count: u64,
    pub all_required_lanes_closed: bool,
    pub user_escape_closure_ready: bool,
    pub production_release_allowed: bool,
    pub closure_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ClosureVerdict {
    pub fn new(config: &Config, source: &SourceBundle, lanes: &[ClosureLane]) -> Self {
        let lane_count = lanes.len() as u64;
        let closed_count = count_status(lanes, ClosureStatus::ClosedForUserEscape);
        let waiting_count = count_status(lanes, ClosureStatus::WaitingForLiveEvidence);
        let blocked_count = count_status(lanes, ClosureStatus::Blocked);
        let required_lane_count = lanes.iter().filter(|lane| lane.required).count() as u64;
        let all_required_lanes_closed = lane_count >= config.min_closure_lanes
            && closed_count >= required_lane_count
            && waiting_count == 0
            && blocked_count == 0;
        let user_escape_closure_ready = all_required_lanes_closed
            && source.consistency_ready
            && source.user_verdict_ready
            && (!config.require_live_feed_reconciled || source.live_feed_ready)
            && (!config.require_adversarial_replay || source.adversarial_ready)
            && (!config.require_custody_policy || source.custody_ready)
            && (!config.require_private_state_continuity || source.continuity_ready);
        let production_release_allowed =
            user_escape_closure_ready && !config.hold_production_until_live_closure;
        let closure_status = if blocked_count > 0 {
            "blocked"
        } else if waiting_count > 0 {
            "waiting_for_live_evidence"
        } else if user_escape_closure_ready {
            "user_escape_closure_ready"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_closure_ready {
            "end-to-end force-exit closure bundle is ready for wallet-side escape review"
        } else {
            "end-to-end force-exit closure remains held until live evidence and all closure lanes agree"
        }
        .to_string();
        let production_answer = if production_release_allowed {
            "production release can be reviewed after live closure evidence"
        } else {
            "production release remains held; closure bundle is wallet-facing evidence only"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            lane_count,
            closed_count,
            waiting_count,
            blocked_count,
            required_lane_count,
            all_required_lanes_closed,
            user_escape_closure_ready,
            production_release_allowed,
            &closure_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            lane_count,
            closed_count,
            waiting_count,
            blocked_count,
            required_lane_count,
            all_required_lanes_closed,
            user_escape_closure_ready,
            production_release_allowed,
            closure_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_count": self.lane_count,
            "closed_count": self.closed_count,
            "waiting_count": self.waiting_count,
            "blocked_count": self.blocked_count,
            "required_lane_count": self.required_lane_count,
            "all_required_lanes_closed": self.all_required_lanes_closed,
            "user_escape_closure_ready": self.user_escape_closure_ready,
            "production_release_allowed": self.production_release_allowed,
            "closure_status": self.closure_status,
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
    pub closure_lanes: Vec<ClosureLane>,
    pub verdict: ClosureVerdict,
    pub closure_lane_root: String,
    pub wallet_closure_bundle_root: String,
    pub live_evidence_gap_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let closure_lanes = closure_lanes(&config, &source);
        let verdict = ClosureVerdict::new(&config, &source, &closure_lanes);
        let closure_lane_root = closure_lane_vector_root(&closure_lanes);
        let wallet_closure_bundle_root =
            wallet_closure_bundle_root(&config, &source, &closure_lanes, &verdict);
        let live_evidence_gap_root =
            live_evidence_gap_root(&config, &source, &closure_lanes, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &closure_lanes, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &closure_lane_root,
            &wallet_closure_bundle_root,
            &live_evidence_gap_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            closure_lanes,
            verdict,
            closure_lane_root,
            wallet_closure_bundle_root,
            live_evidence_gap_root,
            production_hold_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_end_to_end_escape_closure_bundle_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "closure_lane_root": self.closure_lane_root,
            "wallet_closure_bundle_root": self.wallet_closure_bundle_root,
            "live_evidence_gap_root": self.live_evidence_gap_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "closure_lanes": self
                .closure_lanes
                .iter()
                .map(ClosureLane::public_record)
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

fn closure_lanes(config: &Config, source: &SourceBundle) -> Vec<ClosureLane> {
    vec![
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::CrossReceiptConsistency,
            source.consistency_state_root.clone(),
            source.consistency_record_root.clone(),
            source.consistency_ready,
            config.require_consistency,
        ),
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::UserEscapeVerdict,
            source.user_verdict_state_root.clone(),
            source.user_verdict_bundle_root.clone(),
            source.user_verdict_ready,
            config.require_user_verdict,
        ),
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::LiveFeedReconciliation,
            source.live_feed_state_root.clone(),
            source.live_feed_record_root.clone(),
            source.live_feed_ready,
            config.require_live_feed_reconciled,
        ),
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::AdversarialReplay,
            source.adversarial_state_root.clone(),
            source.adversarial_record_root.clone(),
            source.adversarial_ready,
            config.require_adversarial_replay,
        ),
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::CustodyReleasePolicy,
            source.custody_state_root.clone(),
            source.custody_record_root.clone(),
            source.custody_ready,
            config.require_custody_policy,
        ),
        ClosureLane::new(
            config,
            source,
            ClosureLaneKind::PrivateStateContinuity,
            source.continuity_state_root.clone(),
            source.continuity_record_root.clone(),
            source.continuity_ready,
            config.require_private_state_continuity,
        ),
    ]
}

fn closure_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ClosureLaneKind,
    state_root: &str,
    evidence_root: &str,
    ready: bool,
    required: bool,
    status: ClosureStatus,
    hold_reason: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-CLOSURE-LANE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(state_root),
            HashPart::Str(evidence_root),
            HashPart::Str(bool_str(ready)),
            HashPart::Str(bool_str(required)),
            HashPart::Str(status.as_str()),
            HashPart::Str(hold_reason),
        ],
        32,
    )
}

fn closure_lane_vector_root(lanes: &[ClosureLane]) -> String {
    let leaves = lanes
        .iter()
        .map(ClosureLane::public_record)
        .collect::<Vec<_>>();
    merkle_root("MONERO-L2-PQ-FORCE-EXIT-END-TO-END-CLOSURE-LANES", &leaves)
}

fn wallet_closure_bundle_root(
    config: &Config,
    source: &SourceBundle,
    lanes: &[ClosureLane],
    verdict: &ClosureVerdict,
) -> String {
    let leaves = lanes
        .iter()
        .map(|lane| {
            json!({
                "lane_kind": lane.lane_kind.as_str(),
                "closure_root": lane.closure_root,
                "status": lane.status.as_str(),
                "hold_reason": lane.hold_reason,
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-WALLET-CLOSURE-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&verdict.verdict_root),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-WALLET-CLOSURE-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn live_evidence_gap_root(
    config: &Config,
    source: &SourceBundle,
    lanes: &[ClosureLane],
    verdict: &ClosureVerdict,
) -> String {
    let leaves = lanes
        .iter()
        .filter(|lane| lane.status != ClosureStatus::ClosedForUserEscape)
        .map(ClosureLane::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-LIVE-EVIDENCE-GAP",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(verdict.waiting_count),
            HashPart::U64(verdict.blocked_count),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-LIVE-EVIDENCE-GAP-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    lanes: &[ClosureLane],
    verdict: &ClosureVerdict,
) -> String {
    let leaves = lanes
        .iter()
        .filter(|lane| lane.required && !lane.ready)
        .map(ClosureLane::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(bool_str(verdict.production_release_allowed)),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-PRODUCTION-HOLD-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    closure_lane_root: &str,
    wallet_closure_bundle_root: &str,
    live_evidence_gap_root: &str,
    production_hold_root: &str,
    verdict: &ClosureVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-CLOSURE-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&source.state_root()),
            HashPart::Str(closure_lane_root),
            HashPart::Str(wallet_closure_bundle_root),
            HashPart::Str(live_evidence_gap_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    lane_count: u64,
    closed_count: u64,
    waiting_count: u64,
    blocked_count: u64,
    required_lane_count: u64,
    all_required_lanes_closed: bool,
    user_escape_closure_ready: bool,
    production_release_allowed: bool,
    closure_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-CLOSURE-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(lane_count),
            HashPart::U64(closed_count),
            HashPart::U64(waiting_count),
            HashPart::U64(blocked_count),
            HashPart::U64(required_lane_count),
            HashPart::Str(bool_str(all_required_lanes_closed)),
            HashPart::Str(bool_str(user_escape_closure_ready)),
            HashPart::Str(bool_str(production_release_allowed)),
            HashPart::Str(closure_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(lanes: &[ClosureLane], status: ClosureStatus) -> u64 {
    lanes.iter().filter(|lane| lane.status == status).count() as u64
}

fn hold_reason(lane_kind: ClosureLaneKind, status: ClosureStatus) -> &'static str {
    match status {
        ClosureStatus::ClosedForUserEscape => "lane is closed for wallet-side escape review",
        ClosureStatus::WaitingForLiveEvidence => match lane_kind {
            ClosureLaneKind::LiveFeedReconciliation => {
                "live feeds have not replaced every deferred root"
            }
            ClosureLaneKind::AdversarialReplay => "adversarial replay evidence is not complete",
            ClosureLaneKind::CustodyReleasePolicy => "custody release policy is not satisfied",
            ClosureLaneKind::PrivateStateContinuity => "private state continuity is not verified",
            _ => "required closure evidence is still waiting",
        },
        ClosureStatus::Blocked => "closure lane is blocked",
    }
}

fn bool_any(record: &Value, keys: &[&str]) -> bool {
    keys.iter().any(|key| {
        nested_bool(record, "verdict", key)
            || nested_bool(record, "source", key)
            || record
                .get(*key)
                .and_then(Value::as_bool)
                .map_or(false, |value| value)
    })
}

fn nested_bool(record: &Value, first: &str, second: &str) -> bool {
    record
        .get(first)
        .and_then(|value| value.get(second))
        .and_then(Value::as_bool)
        .map_or(false, |value| value)
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id.is_empty() {
        return Err("chain id is required".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("unexpected end-to-end closure protocol version".to_string());
    }
    if config.min_closure_lanes == 0 {
        return Err("at least one closure lane is required".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.consistency_state_root.is_empty() {
        return Err("consistency state root is required".to_string());
    }
    if source.user_verdict_state_root.is_empty() {
        return Err("user verdict state root is required".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle::devnet();
    let lane = ClosureLane {
        lane_kind: ClosureLaneKind::UserEscapeVerdict,
        state_root: source.user_verdict_state_root.clone(),
        evidence_root: source.user_verdict_bundle_root.clone(),
        ready: false,
        required: true,
        status: ClosureStatus::Blocked,
        closure_root: record_root("fallback-closure-lane", &json!({"reason": &reason})),
        hold_reason: reason,
    };
    let closure_lanes = vec![lane];
    let verdict = ClosureVerdict::new(&config, &source, &closure_lanes);
    let closure_lane_root = closure_lane_vector_root(&closure_lanes);
    let wallet_closure_bundle_root =
        wallet_closure_bundle_root(&config, &source, &closure_lanes, &verdict);
    let live_evidence_gap_root = live_evidence_gap_root(&config, &source, &closure_lanes, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &closure_lanes, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &closure_lane_root,
        &wallet_closure_bundle_root,
        &live_evidence_gap_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        closure_lanes,
        verdict,
        closure_lane_root,
        wallet_closure_bundle_root,
        live_evidence_gap_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-END-TO-END-CLOSURE-RECORD",
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
