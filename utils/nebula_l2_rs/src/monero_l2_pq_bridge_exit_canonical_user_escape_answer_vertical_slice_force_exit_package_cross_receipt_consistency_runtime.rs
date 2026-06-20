use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_monitor_runtime as challenge,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_pq_authority_receipt_verifier_runtime as pq,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_recovery_playbook_receipt_runtime as recovery,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_release_hold_clearance_receipt_runtime as clearance,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_reserve_fallback_observation_runtime as reserve,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_settlement_observation_runtime as settlement,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_wallet_scan_receipt_observer_runtime as wallet,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCrossReceiptConsistencyRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CROSS_RECEIPT_CONSISTENCY_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-cross-receipt-consistency-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CROSS_RECEIPT_CONSISTENCY_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const CROSS_RECEIPT_CONSISTENCY_SUITE: &str =
    "monero-l2-pq-force-exit-package-cross-receipt-consistency-v1";
pub const DEFAULT_MIN_CONSISTENT_LANES: u64 = 7;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub consistency_suite: String,
    pub min_consistent_lanes: u64,
    pub require_settlement_observed: bool,
    pub require_challenge_resolved: bool,
    pub require_reserve_complete: bool,
    pub require_pq_quorum: bool,
    pub require_wallet_ready: bool,
    pub require_recovery_answerable: bool,
    pub require_clearance_supported: bool,
    pub hold_production_until_consistent: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            consistency_suite: CROSS_RECEIPT_CONSISTENCY_SUITE.to_string(),
            min_consistent_lanes: DEFAULT_MIN_CONSISTENT_LANES,
            require_settlement_observed: true,
            require_challenge_resolved: true,
            require_reserve_complete: true,
            require_pq_quorum: true,
            require_wallet_ready: true,
            require_recovery_answerable: true,
            require_clearance_supported: true,
            hold_production_until_consistent: true,
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
            "consistency_suite": self.consistency_suite,
            "min_consistent_lanes": self.min_consistent_lanes,
            "require_settlement_observed": self.require_settlement_observed,
            "require_challenge_resolved": self.require_challenge_resolved,
            "require_reserve_complete": self.require_reserve_complete,
            "require_pq_quorum": self.require_pq_quorum,
            "require_wallet_ready": self.require_wallet_ready,
            "require_recovery_answerable": self.require_recovery_answerable,
            "require_clearance_supported": self.require_clearance_supported,
            "hold_production_until_consistent": self.hold_production_until_consistent,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsistencyLaneKind {
    Settlement,
    ChallengeWindow,
    ReserveFallback,
    PqAuthority,
    WalletScan,
    RecoveryPlaybook,
    ReleaseHoldClearance,
}

impl ConsistencyLaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Settlement => "settlement",
            Self::ChallengeWindow => "challenge_window",
            Self::ReserveFallback => "reserve_fallback",
            Self::PqAuthority => "pq_authority",
            Self::WalletScan => "wallet_scan",
            Self::RecoveryPlaybook => "recovery_playbook",
            Self::ReleaseHoldClearance => "release_hold_clearance",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsistencyStatus {
    Consistent,
    MissingEvidence,
    Contradictory,
    ReleaseHeld,
}

impl ConsistencyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::MissingEvidence => "missing_evidence",
            Self::Contradictory => "contradictory",
            Self::ReleaseHeld => "release_held",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub settlement_state_root: String,
    pub settlement_receipt_root: String,
    pub settlement_verdict_root: String,
    pub settlement_ready: bool,
    pub settlement_production_blocked: bool,
    pub challenge_state_root: String,
    pub challenge_receipt_root: String,
    pub challenge_verdict_root: String,
    pub challenge_ready: bool,
    pub challenge_production_blocked: bool,
    pub reserve_state_root: String,
    pub reserve_receipt_root: String,
    pub reserve_verdict_root: String,
    pub reserve_ready: bool,
    pub reserve_production_blocked: bool,
    pub pq_state_root: String,
    pub pq_receipt_root: String,
    pub pq_verdict_root: String,
    pub pq_ready: bool,
    pub pq_production_blocked: bool,
    pub wallet_state_root: String,
    pub wallet_receipt_root: String,
    pub wallet_verdict_root: String,
    pub wallet_ready: bool,
    pub wallet_production_blocked: bool,
    pub recovery_state_root: String,
    pub recovery_receipt_root: String,
    pub recovery_verdict_root: String,
    pub recovery_ready: bool,
    pub recovery_production_blocked: bool,
    pub clearance_state_root: String,
    pub clearance_receipt_root: String,
    pub clearance_verdict_root: String,
    pub clearance_ready: bool,
    pub clearance_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_states(
        settlement_state: &settlement::State,
        challenge_state: &challenge::State,
        reserve_state: &reserve::State,
        pq_state: &pq::State,
        wallet_state: &wallet::State,
        recovery_state: &recovery::State,
        clearance_state: &clearance::State,
    ) -> Self {
        Self {
            settlement_state_root: settlement_state.state_root(),
            settlement_receipt_root: settlement_state.settlement_observation_root.clone(),
            settlement_verdict_root: settlement_state.verdict.verdict_root.clone(),
            settlement_ready: settlement_state.verdict.user_escape_settlement_observed,
            settlement_production_blocked: settlement_state.verdict.production_blocked,
            challenge_state_root: challenge_state.state_root(),
            challenge_receipt_root: challenge_state.watcher_objection_bundle_root.clone(),
            challenge_verdict_root: challenge_state.verdict.verdict_root.clone(),
            challenge_ready: challenge_state.verdict.challenge_window_resolved
                && !challenge_state.verdict.fail_closed_hold_active,
            challenge_production_blocked: challenge_state.verdict.production_blocked,
            reserve_state_root: reserve_state.state_root(),
            reserve_receipt_root: reserve_state.reserve_observation_root.clone(),
            reserve_verdict_root: reserve_state.verdict.verdict_root.clone(),
            reserve_ready: reserve_state.verdict.reserve_observation_complete,
            reserve_production_blocked: reserve_state.verdict.production_blocked,
            pq_state_root: pq_state.state_root(),
            pq_receipt_root: pq_state.authority_receipt_root.clone(),
            pq_verdict_root: pq_state.verdict.verdict_root.clone(),
            pq_ready: pq_state.verdict.quorum_verified
                && pq_state.verdict.signature_threshold_met
                && !pq_state.verdict.fail_closed,
            pq_production_blocked: pq_state.verdict.production_blocked,
            wallet_state_root: wallet_state.state_root(),
            wallet_receipt_root: wallet_state.wallet_scan_receipt_root.clone(),
            wallet_verdict_root: wallet_state.verdict.verdict_root.clone(),
            wallet_ready: wallet_state.verdict.user_escape_ready
                && wallet_state.verdict.linkage_privacy_preserved,
            wallet_production_blocked: wallet_state.verdict.production_blocked,
            recovery_state_root: recovery_state.state_root(),
            recovery_receipt_root: recovery_state.recovery_step_root.clone(),
            recovery_verdict_root: recovery_state.verdict.verdict_root.clone(),
            recovery_ready: recovery_state.verdict.wallet_recovery_answerable,
            recovery_production_blocked: recovery_state.verdict.production_blocked,
            clearance_state_root: clearance_state.state_root(),
            clearance_receipt_root: clearance_state.clearance_receipt_root.clone(),
            clearance_verdict_root: clearance_state.verdict.verdict_root.clone(),
            clearance_ready: clearance_state.verdict.user_escape_clearance_supported,
            clearance_production_blocked: !clearance_state.verdict.production_clearance_allowed,
        }
    }

    pub fn devnet() -> Self {
        Self::from_states(
            &settlement::devnet(),
            &challenge::devnet(),
            &reserve::devnet(),
            &pq::devnet(),
            &wallet::devnet(),
            &recovery::devnet(),
            &clearance::devnet(),
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "settlement_state_root": self.settlement_state_root,
            "settlement_receipt_root": self.settlement_receipt_root,
            "settlement_verdict_root": self.settlement_verdict_root,
            "settlement_ready": self.settlement_ready,
            "settlement_production_blocked": self.settlement_production_blocked,
            "challenge_state_root": self.challenge_state_root,
            "challenge_receipt_root": self.challenge_receipt_root,
            "challenge_verdict_root": self.challenge_verdict_root,
            "challenge_ready": self.challenge_ready,
            "challenge_production_blocked": self.challenge_production_blocked,
            "reserve_state_root": self.reserve_state_root,
            "reserve_receipt_root": self.reserve_receipt_root,
            "reserve_verdict_root": self.reserve_verdict_root,
            "reserve_ready": self.reserve_ready,
            "reserve_production_blocked": self.reserve_production_blocked,
            "pq_state_root": self.pq_state_root,
            "pq_receipt_root": self.pq_receipt_root,
            "pq_verdict_root": self.pq_verdict_root,
            "pq_ready": self.pq_ready,
            "pq_production_blocked": self.pq_production_blocked,
            "wallet_state_root": self.wallet_state_root,
            "wallet_receipt_root": self.wallet_receipt_root,
            "wallet_verdict_root": self.wallet_verdict_root,
            "wallet_ready": self.wallet_ready,
            "wallet_production_blocked": self.wallet_production_blocked,
            "recovery_state_root": self.recovery_state_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "recovery_verdict_root": self.recovery_verdict_root,
            "recovery_ready": self.recovery_ready,
            "recovery_production_blocked": self.recovery_production_blocked,
            "clearance_state_root": self.clearance_state_root,
            "clearance_receipt_root": self.clearance_receipt_root,
            "clearance_verdict_root": self.clearance_verdict_root,
            "clearance_ready": self.clearance_ready,
            "clearance_production_blocked": self.clearance_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConsistencyRecord {
    pub lane_kind: ConsistencyLaneKind,
    pub lane_root: String,
    pub verdict_root: String,
    pub ready: bool,
    pub production_blocked: bool,
    pub dependency_root: String,
    pub consistency_root: String,
    pub status: ConsistencyStatus,
    pub reason: String,
}

impl ConsistencyRecord {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        lane_kind: ConsistencyLaneKind,
        lane_root: String,
        verdict_root: String,
        ready: bool,
        production_blocked: bool,
    ) -> Self {
        let missing = lane_root.is_empty() || verdict_root.is_empty();
        let status = if missing {
            ConsistencyStatus::MissingEvidence
        } else if !ready && production_blocked {
            ConsistencyStatus::ReleaseHeld
        } else if !ready {
            ConsistencyStatus::Contradictory
        } else {
            ConsistencyStatus::Consistent
        };
        let dependency_root = dependency_root(config, source, lane_kind, &lane_root, &verdict_root);
        let reason = consistency_reason(status, lane_kind).to_string();
        let consistency_root = consistency_root(
            config,
            source,
            lane_kind,
            &lane_root,
            &verdict_root,
            &dependency_root,
            ready,
            production_blocked,
            status,
            &reason,
        );
        Self {
            lane_kind,
            lane_root,
            verdict_root,
            ready,
            production_blocked,
            dependency_root,
            consistency_root,
            status,
            reason,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_kind": self.lane_kind.as_str(),
            "lane_root": self.lane_root,
            "verdict_root": self.verdict_root,
            "ready": self.ready,
            "production_blocked": self.production_blocked,
            "dependency_root": self.dependency_root,
            "consistency_root": self.consistency_root,
            "status": self.status.as_str(),
            "reason": self.reason,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConsistencyVerdict {
    pub lane_count: u64,
    pub consistent_count: u64,
    pub missing_count: u64,
    pub contradictory_count: u64,
    pub release_held_count: u64,
    pub production_blocker_count: u64,
    pub all_lanes_consistent: bool,
    pub user_escape_consistent: bool,
    pub production_release_consistent: bool,
    pub consistency_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ConsistencyVerdict {
    pub fn new(config: &Config, source: &SourceBundle, records: &[ConsistencyRecord]) -> Self {
        let lane_count = records.len() as u64;
        let consistent_count = count_status(records, ConsistencyStatus::Consistent);
        let missing_count = count_status(records, ConsistencyStatus::MissingEvidence);
        let contradictory_count = count_status(records, ConsistencyStatus::Contradictory);
        let release_held_count = count_status(records, ConsistencyStatus::ReleaseHeld);
        let production_blocker_count = records
            .iter()
            .filter(|record| record.production_blocked)
            .count() as u64;
        let all_lanes_consistent = lane_count >= config.min_consistent_lanes
            && consistent_count == lane_count
            && missing_count == 0
            && contradictory_count == 0;
        let user_escape_consistent = all_lanes_consistent
            && source.settlement_ready
            && source.challenge_ready
            && source.reserve_ready
            && source.pq_ready
            && source.wallet_ready
            && source.recovery_ready
            && source.clearance_ready;
        let production_release_consistent = user_escape_consistent
            && !config.hold_production_until_consistent
            && production_blocker_count == 0
            && release_held_count == 0;
        let consistency_status = if contradictory_count > 0 {
            "contradictory_receipts"
        } else if missing_count > 0 {
            "missing_receipt_dependency"
        } else if release_held_count > 0 {
            "release_held"
        } else if user_escape_consistent {
            "user_escape_consistent"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_consistent {
            "force-exit receipt lanes are internally consistent for wallet escape review"
        } else {
            "force-exit receipt lanes are not yet mutually consistent enough for wallet escape"
        }
        .to_string();
        let production_answer = if production_release_consistent {
            "production release consistency can be reviewed after live execution evidence"
        } else {
            "production release remains held until every receipt lane is live, consistent, and unblocked"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            lane_count,
            consistent_count,
            missing_count,
            contradictory_count,
            release_held_count,
            production_blocker_count,
            all_lanes_consistent,
            user_escape_consistent,
            production_release_consistent,
            &consistency_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            lane_count,
            consistent_count,
            missing_count,
            contradictory_count,
            release_held_count,
            production_blocker_count,
            all_lanes_consistent,
            user_escape_consistent,
            production_release_consistent,
            consistency_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_count": self.lane_count,
            "consistent_count": self.consistent_count,
            "missing_count": self.missing_count,
            "contradictory_count": self.contradictory_count,
            "release_held_count": self.release_held_count,
            "production_blocker_count": self.production_blocker_count,
            "all_lanes_consistent": self.all_lanes_consistent,
            "user_escape_consistent": self.user_escape_consistent,
            "production_release_consistent": self.production_release_consistent,
            "consistency_status": self.consistency_status,
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
    pub consistency_records: Vec<ConsistencyRecord>,
    pub verdict: ConsistencyVerdict,
    pub consistency_record_root: String,
    pub dependency_bundle_root: String,
    pub contradiction_bundle_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let consistency_records = consistency_records(&config, &source);
        let verdict = ConsistencyVerdict::new(&config, &source, &consistency_records);
        let consistency_record_root = consistency_record_vector_root(&consistency_records);
        let dependency_bundle_root =
            dependency_bundle_root(&config, &source, &consistency_records, &verdict);
        let contradiction_bundle_root =
            contradiction_bundle_root(&config, &source, &consistency_records, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &consistency_records, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &consistency_record_root,
            &dependency_bundle_root,
            &contradiction_bundle_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            consistency_records,
            verdict,
            consistency_record_root,
            dependency_bundle_root,
            contradiction_bundle_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_cross_receipt_consistency_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "consistency_record_root": self.consistency_record_root,
            "dependency_bundle_root": self.dependency_bundle_root,
            "contradiction_bundle_root": self.contradiction_bundle_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "consistency_records": self
                .consistency_records
                .iter()
                .map(ConsistencyRecord::public_record)
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

fn consistency_records(config: &Config, source: &SourceBundle) -> Vec<ConsistencyRecord> {
    vec![
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::Settlement,
            source.settlement_receipt_root.clone(),
            source.settlement_verdict_root.clone(),
            !config.require_settlement_observed || source.settlement_ready,
            source.settlement_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::ChallengeWindow,
            source.challenge_receipt_root.clone(),
            source.challenge_verdict_root.clone(),
            !config.require_challenge_resolved || source.challenge_ready,
            source.challenge_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::ReserveFallback,
            source.reserve_receipt_root.clone(),
            source.reserve_verdict_root.clone(),
            !config.require_reserve_complete || source.reserve_ready,
            source.reserve_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::PqAuthority,
            source.pq_receipt_root.clone(),
            source.pq_verdict_root.clone(),
            !config.require_pq_quorum || source.pq_ready,
            source.pq_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::WalletScan,
            source.wallet_receipt_root.clone(),
            source.wallet_verdict_root.clone(),
            !config.require_wallet_ready || source.wallet_ready,
            source.wallet_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::RecoveryPlaybook,
            source.recovery_receipt_root.clone(),
            source.recovery_verdict_root.clone(),
            !config.require_recovery_answerable || source.recovery_ready,
            source.recovery_production_blocked,
        ),
        ConsistencyRecord::new(
            config,
            source,
            ConsistencyLaneKind::ReleaseHoldClearance,
            source.clearance_receipt_root.clone(),
            source.clearance_verdict_root.clone(),
            !config.require_clearance_supported || source.clearance_ready,
            source.clearance_production_blocked,
        ),
    ]
}

fn dependency_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ConsistencyLaneKind,
    lane_root: &str,
    verdict_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-DEPENDENCY",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(lane_root),
            HashPart::Str(verdict_root),
        ],
        32,
    )
}

fn consistency_root(
    config: &Config,
    source: &SourceBundle,
    lane_kind: ConsistencyLaneKind,
    lane_root: &str,
    verdict_root: &str,
    dependency_root: &str,
    ready: bool,
    production_blocked: bool,
    status: ConsistencyStatus,
    reason: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-CONSISTENCY",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(lane_root),
            HashPart::Str(verdict_root),
            HashPart::Str(dependency_root),
            HashPart::Str(bool_str(ready)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(status.as_str()),
            HashPart::Str(reason),
        ],
        32,
    )
}

fn consistency_record_vector_root(records: &[ConsistencyRecord]) -> String {
    let leaves = records
        .iter()
        .map(ConsistencyRecord::public_record)
        .collect::<Vec<_>>();
    merkle_root("MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-RECORDS", &leaves)
}

fn dependency_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ConsistencyRecord],
    verdict: &ConsistencyVerdict,
) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "lane_kind": record.lane_kind.as_str(),
                "dependency_root": record.dependency_root,
                "consistency_root": record.consistency_root,
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-DEPENDENCY-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-DEPENDENCY-LEAVES",
                &leaves,
            )),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn contradiction_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ConsistencyRecord],
    verdict: &ConsistencyVerdict,
) -> String {
    let leaves = records
        .iter()
        .filter(|record| record.status != ConsistencyStatus::Consistent)
        .map(ConsistencyRecord::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-CONTRADICTION-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(verdict.missing_count),
            HashPart::U64(verdict.contradictory_count),
            HashPart::U64(verdict.release_held_count),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-CONTRADICTION-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ConsistencyRecord],
    verdict: &ConsistencyVerdict,
) -> String {
    let leaves = records
        .iter()
        .filter(|record| record.production_blocked)
        .map(ConsistencyRecord::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(bool_str(verdict.production_release_consistent)),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-PRODUCTION-HOLD-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    consistency_record_root: &str,
    dependency_bundle_root: &str,
    contradiction_bundle_root: &str,
    production_hold_root: &str,
    verdict: &ConsistencyVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&source.state_root()),
            HashPart::Str(consistency_record_root),
            HashPart::Str(dependency_bundle_root),
            HashPart::Str(contradiction_bundle_root),
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
    consistent_count: u64,
    missing_count: u64,
    contradictory_count: u64,
    release_held_count: u64,
    production_blocker_count: u64,
    all_lanes_consistent: bool,
    user_escape_consistent: bool,
    production_release_consistent: bool,
    consistency_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(lane_count),
            HashPart::U64(consistent_count),
            HashPart::U64(missing_count),
            HashPart::U64(contradictory_count),
            HashPart::U64(release_held_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(all_lanes_consistent)),
            HashPart::Str(bool_str(user_escape_consistent)),
            HashPart::Str(bool_str(production_release_consistent)),
            HashPart::Str(consistency_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(records: &[ConsistencyRecord], status: ConsistencyStatus) -> u64 {
    records
        .iter()
        .filter(|record| record.status == status)
        .count() as u64
}

fn consistency_reason(status: ConsistencyStatus, lane_kind: ConsistencyLaneKind) -> &'static str {
    match status {
        ConsistencyStatus::Consistent => "receipt lane is internally consistent",
        ConsistencyStatus::MissingEvidence => "receipt lane is missing a required root",
        ConsistencyStatus::Contradictory => {
            "receipt lane reports readiness that contradicts policy"
        }
        ConsistencyStatus::ReleaseHeld => {
            if lane_kind == ConsistencyLaneKind::ReleaseHoldClearance {
                "release-hold clearance lane remains held"
            } else {
                "receipt lane remains release-held until live evidence clears"
            }
        }
    }
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id.is_empty() {
        return Err("chain id is required".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("unexpected cross-receipt consistency protocol version".to_string());
    }
    if config.min_consistent_lanes == 0 {
        return Err("at least one consistent lane is required".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.settlement_state_root.is_empty() {
        return Err("settlement state root is required".to_string());
    }
    if source.clearance_state_root.is_empty() {
        return Err("clearance state root is required".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle::devnet();
    let record = ConsistencyRecord {
        lane_kind: ConsistencyLaneKind::ReleaseHoldClearance,
        lane_root: source.clearance_receipt_root.clone(),
        verdict_root: source.clearance_verdict_root.clone(),
        ready: false,
        production_blocked: true,
        dependency_root: record_root("fallback-dependency", &json!({"reason": &reason})),
        consistency_root: record_root("fallback-consistency", &json!({"reason": &reason})),
        status: ConsistencyStatus::ReleaseHeld,
        reason,
    };
    let consistency_records = vec![record];
    let verdict = ConsistencyVerdict::new(&config, &source, &consistency_records);
    let consistency_record_root = consistency_record_vector_root(&consistency_records);
    let dependency_bundle_root =
        dependency_bundle_root(&config, &source, &consistency_records, &verdict);
    let contradiction_bundle_root =
        contradiction_bundle_root(&config, &source, &consistency_records, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &consistency_records, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &consistency_record_root,
        &dependency_bundle_root,
        &contradiction_bundle_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        consistency_records,
        verdict,
        consistency_record_root,
        dependency_bundle_root,
        contradiction_bundle_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-CROSS-RECEIPT-RECORD",
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
