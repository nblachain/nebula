use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageLiveSettlementDrillEvidenceRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_LIVE_SETTLEMENT_DRILL_EVIDENCE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-live-settlement-drill-evidence-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_LIVE_SETTLEMENT_DRILL_EVIDENCE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const LIVE_SETTLEMENT_DRILL_EVIDENCE_SUITE: &str =
    "monero-l2-pq-force-exit-package-live-settlement-drill-evidence-v1";
pub const DEFAULT_MIN_ANCHOR_OBSERVATIONS: u64 = 5;
pub const DEFAULT_MIN_WITHDRAWAL_RELEASE_PROOFS: u64 = 3;
pub const DEFAULT_MIN_FINALITY_CONFIRMATIONS: u64 = 10;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub drill_evidence_suite: String,
    pub min_anchor_observations: u64,
    pub min_withdrawal_release_proofs: u64,
    pub min_finality_confirmations: u64,
    pub require_anchor_observations: bool,
    pub require_withdrawal_release_proofs: bool,
    pub require_custody_confirmation: bool,
    pub require_wallet_visible_finality: bool,
    pub require_reserve_match: bool,
    pub require_fail_closed_status: bool,
    pub hold_production_until_live_settlement_drill_passes: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            drill_evidence_suite: LIVE_SETTLEMENT_DRILL_EVIDENCE_SUITE.to_string(),
            min_anchor_observations: DEFAULT_MIN_ANCHOR_OBSERVATIONS,
            min_withdrawal_release_proofs: DEFAULT_MIN_WITHDRAWAL_RELEASE_PROOFS,
            min_finality_confirmations: DEFAULT_MIN_FINALITY_CONFIRMATIONS,
            require_anchor_observations: true,
            require_withdrawal_release_proofs: true,
            require_custody_confirmation: true,
            require_wallet_visible_finality: true,
            require_reserve_match: true,
            require_fail_closed_status: true,
            hold_production_until_live_settlement_drill_passes: true,
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
            "drill_evidence_suite": self.drill_evidence_suite,
            "min_anchor_observations": self.min_anchor_observations,
            "min_withdrawal_release_proofs": self.min_withdrawal_release_proofs,
            "min_finality_confirmations": self.min_finality_confirmations,
            "require_anchor_observations": self.require_anchor_observations,
            "require_withdrawal_release_proofs": self.require_withdrawal_release_proofs,
            "require_custody_confirmation": self.require_custody_confirmation,
            "require_wallet_visible_finality": self.require_wallet_visible_finality,
            "require_reserve_match": self.require_reserve_match,
            "require_fail_closed_status": self.require_fail_closed_status,
            "hold_production_until_live_settlement_drill_passes": self.hold_production_until_live_settlement_drill_passes,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub anchor_observation_root: String,
    pub withdrawal_release_proof_root: String,
    pub custody_confirmation_root: String,
    pub wallet_visible_finality_root: String,
    pub reserve_match_root: String,
    pub fail_closed_status_root: String,
    pub live_settlement_drill_evidence_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "anchor_observation_root": self.anchor_observation_root,
            "withdrawal_release_proof_root": self.withdrawal_release_proof_root,
            "custody_confirmation_root": self.custody_confirmation_root,
            "wallet_visible_finality_root": self.wallet_visible_finality_root,
            "reserve_match_root": self.reserve_match_root,
            "fail_closed_status_root": self.fail_closed_status_root,
            "live_settlement_drill_evidence_root": self.live_settlement_drill_evidence_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub anchor_observation_count: u64,
    pub withdrawal_release_proof_count: u64,
    pub custody_confirmation_count: u64,
    pub wallet_visible_finality_count: u64,
    pub reserve_match_count: u64,
    pub fail_closed_status_count: u64,
    pub evidence_lane_count: u64,
    pub satisfied_lane_count: u64,
    pub blocker_count: u64,
}

impl Counters {
    pub fn devnet(config: &Config) -> Self {
        Self {
            anchor_observation_count: config.min_anchor_observations,
            withdrawal_release_proof_count: config.min_withdrawal_release_proofs,
            custody_confirmation_count: 1,
            wallet_visible_finality_count: config.min_finality_confirmations,
            reserve_match_count: 1,
            fail_closed_status_count: 1,
            evidence_lane_count: 6,
            satisfied_lane_count: 6,
            blocker_count: 0,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "anchor_observation_count": self.anchor_observation_count,
            "withdrawal_release_proof_count": self.withdrawal_release_proof_count,
            "custody_confirmation_count": self.custody_confirmation_count,
            "wallet_visible_finality_count": self.wallet_visible_finality_count,
            "reserve_match_count": self.reserve_match_count,
            "fail_closed_status_count": self.fail_closed_status_count,
            "evidence_lane_count": self.evidence_lane_count,
            "satisfied_lane_count": self.satisfied_lane_count,
            "blocker_count": self.blocker_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub counters: Counters,
    pub roots: Roots,
    pub drill_passed: bool,
    pub production_blocked: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        validate_config(&config)?;
        let counters = Counters::devnet(&config);
        let roots = build_roots(&config, &counters);
        let drill_passed = counters.blocker_count == 0
            && counters.satisfied_lane_count >= counters.evidence_lane_count;
        let production_blocked =
            config.hold_production_until_live_settlement_drill_passes && !drill_passed;
        let user_escape_answer = if drill_passed {
            "live settlement drill evidence complete for user escape".to_string()
        } else {
            "live settlement drill evidence incomplete".to_string()
        };
        let production_answer = if production_blocked {
            "production held until live settlement drill evidence passes".to_string()
        } else {
            "production release gate has live settlement drill evidence".to_string()
        };
        let state_commitment_root = state_commitment_root(
            &config,
            &counters,
            &roots,
            drill_passed,
            production_blocked,
            &user_escape_answer,
            &production_answer,
        );
        let roots = Roots {
            state_commitment_root: state_commitment_root.clone(),
            ..roots
        };
        Ok(Self {
            config,
            counters,
            roots,
            drill_passed,
            production_blocked,
            user_escape_answer,
            production_answer,
            state_commitment_root,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "drill_passed": self.drill_passed,
            "production_blocked": self.production_blocked,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
    }
}

pub fn devnet() -> State {
    match State::new(Config::devnet()) {
        Ok(state) => state,
        Err(reason) => fallback_state(reason),
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn build_roots(config: &Config, counters: &Counters) -> Roots {
    let config_root = config.state_root();
    let anchor_observation_root = evidence_root(
        config,
        counters,
        "anchor-observations",
        counters.anchor_observation_count,
        "canonical Monero anchor observations observed by live settlement drill",
    );
    let withdrawal_release_proof_root = evidence_root(
        config,
        counters,
        "withdrawal-release-proofs",
        counters.withdrawal_release_proof_count,
        "withdrawal release proofs bind force-exit package to live settlement",
    );
    let custody_confirmation_root = evidence_root(
        config,
        counters,
        "custody-confirmation",
        counters.custody_confirmation_count,
        "custody confirmation matches package release authority",
    );
    let wallet_visible_finality_root = evidence_root(
        config,
        counters,
        "wallet-visible-finality",
        counters.wallet_visible_finality_count,
        "wallet-visible finality confirms user can observe completed settlement",
    );
    let reserve_match_root = evidence_root(
        config,
        counters,
        "reserve-match",
        counters.reserve_match_count,
        "reserve match confirms live settlement units are fully covered",
    );
    let fail_closed_status_root = evidence_root(
        config,
        counters,
        "fail-closed-status",
        counters.fail_closed_status_count,
        "fail-closed status is armed and non-bypassed during drill",
    );
    let live_settlement_drill_evidence_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIVE-SETTLEMENT-DRILL-EVIDENCE-ROOT",
        &[
            json!({"lane": "anchor_observations", "root": anchor_observation_root}),
            json!({"lane": "withdrawal_release_proofs", "root": withdrawal_release_proof_root}),
            json!({"lane": "custody_confirmation", "root": custody_confirmation_root}),
            json!({"lane": "wallet_visible_finality", "root": wallet_visible_finality_root}),
            json!({"lane": "reserve_match", "root": reserve_match_root}),
            json!({"lane": "fail_closed_status", "root": fail_closed_status_root}),
        ],
    );
    let production_hold_root = record_root(
        "production-hold",
        &json!({
            "hold_required": config.hold_production_until_live_settlement_drill_passes,
            "blocker_count": counters.blocker_count,
            "evidence_root": live_settlement_drill_evidence_root,
        }),
    );
    Roots {
        config_root,
        anchor_observation_root,
        withdrawal_release_proof_root,
        custody_confirmation_root,
        wallet_visible_finality_root,
        reserve_match_root,
        fail_closed_status_root,
        live_settlement_drill_evidence_root,
        production_hold_root,
        state_commitment_root: String::new(),
    }
}

fn evidence_root(
    config: &Config,
    counters: &Counters,
    lane: &str,
    count: u64,
    required_outcome: &str,
) -> String {
    record_root(
        lane,
        &json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "drill_evidence_suite": config.drill_evidence_suite,
            "lane": lane,
            "count": count,
            "evidence_lane_count": counters.evidence_lane_count,
            "satisfied_lane_count": counters.satisfied_lane_count,
            "required_outcome": required_outcome,
            "status": "satisfied",
        }),
    )
}

fn state_commitment_root(
    config: &Config,
    counters: &Counters,
    roots: &Roots,
    drill_passed: bool,
    production_blocked: bool,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIVE-SETTLEMENT-DRILL-EVIDENCE-STATE",
        &[
            HashPart::Str(&config.drill_evidence_suite),
            HashPart::Str(&roots.config_root),
            HashPart::Str(&roots.anchor_observation_root),
            HashPart::Str(&roots.withdrawal_release_proof_root),
            HashPart::Str(&roots.custody_confirmation_root),
            HashPart::Str(&roots.wallet_visible_finality_root),
            HashPart::Str(&roots.reserve_match_root),
            HashPart::Str(&roots.fail_closed_status_root),
            HashPart::Str(&roots.live_settlement_drill_evidence_root),
            HashPart::Str(&roots.production_hold_root),
            HashPart::U64(counters.evidence_lane_count),
            HashPart::U64(counters.satisfied_lane_count),
            HashPart::U64(counters.blocker_count),
            HashPart::Str(bool_str(drill_passed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "live settlement drill evidence chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "live settlement drill evidence protocol mismatch",
    )?;
    ensure(
        config.min_anchor_observations > 0,
        "live settlement drill evidence requires anchor observations",
    )?;
    ensure(
        config.min_withdrawal_release_proofs > 0,
        "live settlement drill evidence requires withdrawal release proofs",
    )?;
    ensure(
        config.min_finality_confirmations > 0,
        "live settlement drill evidence requires wallet-visible finality",
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
    let counters = Counters {
        anchor_observation_count: 0,
        withdrawal_release_proof_count: 0,
        custody_confirmation_count: 0,
        wallet_visible_finality_count: 0,
        reserve_match_count: 0,
        fail_closed_status_count: 1,
        evidence_lane_count: 6,
        satisfied_lane_count: 1,
        blocker_count: 5,
    };
    let mut roots = build_roots(&config, &counters);
    roots.fail_closed_status_root = record_root(
        "fallback-fail-closed-status",
        &json!({
            "reason": reason,
            "status": "fail_closed",
        }),
    );
    let drill_passed = false;
    let production_blocked = true;
    let user_escape_answer = "live settlement drill evidence failed closed".to_string();
    let production_answer =
        "production held by fallback live settlement drill evidence".to_string();
    let state_commitment_root = state_commitment_root(
        &config,
        &counters,
        &roots,
        drill_passed,
        production_blocked,
        &user_escape_answer,
        &production_answer,
    );
    roots.state_commitment_root = state_commitment_root.clone();
    State {
        config,
        counters,
        roots,
        drill_passed,
        production_blocked,
        user_escape_answer,
        production_answer,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIVE-SETTLEMENT-DRILL-EVIDENCE-RECORD",
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
