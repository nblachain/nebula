use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution_receipt,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReserveFallbackObservationRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RESERVE_FALLBACK_OBSERVATION_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-reserve-fallback-observation-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RESERVE_FALLBACK_OBSERVATION_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RESERVE_FALLBACK_OBSERVATION_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-reserve-fallback-observation-v1";
pub const DEFAULT_MIN_RESERVE_OBSERVATIONS: u64 = 3;
pub const DEFAULT_MIN_SETTLEMENT_RESERVE_OBSERVATIONS: u64 = 1;
pub const DEFAULT_MIN_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_RESERVE_UNIT: u64 = 1_000_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub reserve_fallback_suite: String,
    pub min_reserve_observations: u64,
    pub min_settlement_reserve_observations: u64,
    pub min_coverage_bps: u64,
    pub reserve_unit: u64,
    pub require_liquidity_not_exhausted: bool,
    pub require_fallback_payout_roots: bool,
    pub require_settlement_reserve_evidence: bool,
    pub hold_production_until_reserve_evidence: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            reserve_fallback_suite: RESERVE_FALLBACK_OBSERVATION_SUITE.to_string(),
            min_reserve_observations: DEFAULT_MIN_RESERVE_OBSERVATIONS,
            min_settlement_reserve_observations: DEFAULT_MIN_SETTLEMENT_RESERVE_OBSERVATIONS,
            min_coverage_bps: DEFAULT_MIN_COVERAGE_BPS,
            reserve_unit: DEFAULT_RESERVE_UNIT,
            require_liquidity_not_exhausted: true,
            require_fallback_payout_roots: true,
            require_settlement_reserve_evidence: true,
            hold_production_until_reserve_evidence: true,
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
            "reserve_fallback_suite": self.reserve_fallback_suite,
            "min_reserve_observations": self.min_reserve_observations,
            "min_settlement_reserve_observations": self.min_settlement_reserve_observations,
            "min_coverage_bps": self.min_coverage_bps,
            "reserve_unit": self.reserve_unit,
            "require_liquidity_not_exhausted": self.require_liquidity_not_exhausted,
            "require_fallback_payout_roots": self.require_fallback_payout_roots,
            "require_settlement_reserve_evidence": self.require_settlement_reserve_evidence,
            "hold_production_until_reserve_evidence": self.hold_production_until_reserve_evidence,
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
    pub challenge_settlement_bundle_root: String,
    pub recovery_receipt_root: String,
    pub execution_production_hold_root: String,
    pub source_package_state_root: String,
    pub source_reserve_receipts_present: bool,
    pub source_settlement_receipts_present: bool,
    pub source_package_observed: bool,
    pub source_production_blocked: bool,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub production_blocker_count: u64,
    pub source_execution_status: String,
    pub source_user_escape_answer: String,
    pub source_production_answer: String,
}

impl SourceBundle {
    pub fn from_execution_receipts(state: &execution_receipt::State) -> Self {
        Self {
            execution_state_root: state.state_root(),
            execution_receipt_root: state.execution_receipt_root.clone(),
            challenge_settlement_bundle_root: state.challenge_settlement_bundle_root.clone(),
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            execution_production_hold_root: state.production_hold_root.clone(),
            source_package_state_root: state.source.package_state_root.clone(),
            source_reserve_receipts_present: state.verdict.reserve_receipts_present,
            source_settlement_receipts_present: state.verdict.settlement_receipts_present,
            source_package_observed: state.verdict.package_execution_observed,
            source_production_blocked: state.verdict.production_blocked,
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            deferred_receipt_count: state.verdict.deferred_receipt_count,
            production_blocker_count: state.verdict.production_blocker_count,
            source_execution_status: state.verdict.execution_status.clone(),
            source_user_escape_answer: state.verdict.user_escape_answer.clone(),
            source_production_answer: state.verdict.production_answer.clone(),
        }
    }

    pub fn devnet() -> Self {
        Self::from_execution_receipts(&execution_receipt::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "execution_production_hold_root": self.execution_production_hold_root,
            "source_package_state_root": self.source_package_state_root,
            "source_reserve_receipts_present": self.source_reserve_receipts_present,
            "source_settlement_receipts_present": self.source_settlement_receipts_present,
            "source_package_observed": self.source_package_observed,
            "source_production_blocked": self.source_production_blocked,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "production_blocker_count": self.production_blocker_count,
            "source_execution_status": self.source_execution_status,
            "source_user_escape_answer": self.source_user_escape_answer,
            "source_production_answer": self.source_production_answer,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReserveObservationKind {
    ReserveCoverage,
    LiquidityRunway,
    FallbackPayout,
    SettlementReserveEvidence,
}

impl ReserveObservationKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::ReserveCoverage,
            Self::LiquidityRunway,
            Self::FallbackPayout,
            Self::SettlementReserveEvidence,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReserveCoverage => "reserve_coverage",
            Self::LiquidityRunway => "liquidity_runway",
            Self::FallbackPayout => "fallback_payout",
            Self::SettlementReserveEvidence => "settlement_reserve_evidence",
        }
    }

    pub fn settlement_evidence(self) -> bool {
        matches!(self, Self::SettlementReserveEvidence)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReserveObservationStatus {
    Covered,
    FallbackCovered,
    LiquidityExhausted,
    EvidenceMissing,
}

impl ReserveObservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::FallbackCovered => "fallback_covered",
            Self::LiquidityExhausted => "liquidity_exhausted",
            Self::EvidenceMissing => "evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveFallbackObservation {
    pub observation_id: String,
    pub ordinal: u64,
    pub observation_kind: ReserveObservationKind,
    pub reserve_evidence_root: String,
    pub coverage_root: String,
    pub liquidity_root: String,
    pub fallback_payout_root: String,
    pub settlement_evidence_root: String,
    pub required_reserve_units: u64,
    pub observed_reserve_units: u64,
    pub fallback_payout_units: u64,
    pub reserve_deficit_units: u64,
    pub coverage_bps: u64,
    pub settlement_evidence: bool,
    pub fallback_payout_ready: bool,
    pub liquidity_exhausted: bool,
    pub blocks_production: bool,
    pub status: ReserveObservationStatus,
    pub observation_root: String,
    pub required_outcome: String,
}

impl ReserveFallbackObservation {
    pub fn devnet(config: &Config, source: &SourceBundle, kind: ReserveObservationKind) -> Self {
        let ordinal = observation_ordinal(kind);
        let required_reserve_units = required_reserve_units(config, source, kind);
        let observed_reserve_units = observed_reserve_units(config, source, kind);
        let reserve_deficit_units = required_reserve_units.saturating_sub(observed_reserve_units);
        let fallback_payout_units = fallback_payout_units(source, kind, reserve_deficit_units);
        let coverage_bps = coverage_bps(observed_reserve_units, required_reserve_units);
        let settlement_evidence = source.source_settlement_receipts_present
            && source.source_reserve_receipts_present
            && (kind.settlement_evidence() || source.source_package_observed);
        let fallback_payout_ready = fallback_payout_units >= reserve_deficit_units
            && (!config.require_fallback_payout_roots || source.source_reserve_receipts_present);
        let liquidity_exhausted = config.require_liquidity_not_exhausted
            && reserve_deficit_units > 0
            && fallback_payout_units < reserve_deficit_units;
        let status = observation_status(
            config,
            kind,
            coverage_bps,
            settlement_evidence,
            fallback_payout_ready,
            liquidity_exhausted,
        );
        let blocks_production = status != ReserveObservationStatus::Covered
            && status != ReserveObservationStatus::FallbackCovered;
        let reserve_evidence_root = reserve_evidence_root(config, source, kind, ordinal);
        let coverage_root = coverage_root(
            config,
            source,
            kind,
            required_reserve_units,
            observed_reserve_units,
            coverage_bps,
        );
        let liquidity_root = liquidity_root(
            config,
            source,
            kind,
            reserve_deficit_units,
            liquidity_exhausted,
        );
        let fallback_payout_root = fallback_payout_root(
            config,
            source,
            kind,
            fallback_payout_units,
            reserve_deficit_units,
            fallback_payout_ready,
        );
        let settlement_evidence_root =
            settlement_evidence_root(config, source, kind, settlement_evidence);
        let observation_root = observation_root(
            config,
            source,
            kind,
            status,
            &reserve_evidence_root,
            &coverage_root,
            &liquidity_root,
            &fallback_payout_root,
            &settlement_evidence_root,
            required_reserve_units,
            observed_reserve_units,
            fallback_payout_units,
            reserve_deficit_units,
            coverage_bps,
            blocks_production,
        );
        let observation_id = observation_id(kind, ordinal, &observation_root);
        Self {
            observation_id,
            ordinal,
            observation_kind: kind,
            reserve_evidence_root,
            coverage_root,
            liquidity_root,
            fallback_payout_root,
            settlement_evidence_root,
            required_reserve_units,
            observed_reserve_units,
            fallback_payout_units,
            reserve_deficit_units,
            coverage_bps,
            settlement_evidence,
            fallback_payout_ready,
            liquidity_exhausted,
            blocks_production,
            status,
            observation_root,
            required_outcome: required_outcome(status, kind).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_id": self.observation_id,
            "ordinal": self.ordinal,
            "observation_kind": self.observation_kind.as_str(),
            "reserve_evidence_root": self.reserve_evidence_root,
            "coverage_root": self.coverage_root,
            "liquidity_root": self.liquidity_root,
            "fallback_payout_root": self.fallback_payout_root,
            "settlement_evidence_root": self.settlement_evidence_root,
            "required_reserve_units": self.required_reserve_units,
            "observed_reserve_units": self.observed_reserve_units,
            "fallback_payout_units": self.fallback_payout_units,
            "reserve_deficit_units": self.reserve_deficit_units,
            "coverage_bps": self.coverage_bps,
            "settlement_evidence": self.settlement_evidence,
            "fallback_payout_ready": self.fallback_payout_ready,
            "liquidity_exhausted": self.liquidity_exhausted,
            "blocks_production": self.blocks_production,
            "status": self.status.as_str(),
            "observation_root": self.observation_root,
            "required_outcome": self.required_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.observation_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveFallbackVerdict {
    pub observation_count: u64,
    pub covered_count: u64,
    pub fallback_covered_count: u64,
    pub liquidity_exhausted_count: u64,
    pub evidence_missing_count: u64,
    pub settlement_reserve_evidence_count: u64,
    pub fallback_payout_ready_count: u64,
    pub production_blocker_count: u64,
    pub total_required_reserve_units: u64,
    pub total_observed_reserve_units: u64,
    pub total_fallback_payout_units: u64,
    pub total_reserve_deficit_units: u64,
    pub aggregate_coverage_bps: u64,
    pub reserve_coverage_met: bool,
    pub liquidity_available: bool,
    pub fallback_payout_roots_present: bool,
    pub settlement_reserve_evidence_present: bool,
    pub reserve_observation_complete: bool,
    pub production_blocked: bool,
    pub reserve_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ReserveFallbackVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        observations: &[ReserveFallbackObservation],
    ) -> Self {
        let observation_count = observations.len() as u64;
        let covered_count = count_status(observations, ReserveObservationStatus::Covered);
        let fallback_covered_count =
            count_status(observations, ReserveObservationStatus::FallbackCovered);
        let liquidity_exhausted_count =
            count_status(observations, ReserveObservationStatus::LiquidityExhausted);
        let evidence_missing_count =
            count_status(observations, ReserveObservationStatus::EvidenceMissing);
        let settlement_reserve_evidence_count = observations
            .iter()
            .filter(|observation| observation.settlement_evidence)
            .count() as u64;
        let fallback_payout_ready_count = observations
            .iter()
            .filter(|observation| observation.fallback_payout_ready)
            .count() as u64;
        let production_blocker_count = observations
            .iter()
            .filter(|observation| observation.blocks_production)
            .count() as u64;
        let total_required_reserve_units = observations
            .iter()
            .map(|observation| observation.required_reserve_units)
            .sum();
        let total_observed_reserve_units = observations
            .iter()
            .map(|observation| observation.observed_reserve_units)
            .sum();
        let total_fallback_payout_units = observations
            .iter()
            .map(|observation| observation.fallback_payout_units)
            .sum();
        let total_reserve_deficit_units = observations
            .iter()
            .map(|observation| observation.reserve_deficit_units)
            .sum();
        let aggregate_coverage_bps =
            coverage_bps(total_observed_reserve_units, total_required_reserve_units);
        let reserve_coverage_met = aggregate_coverage_bps >= config.min_coverage_bps
            && covered_count + fallback_covered_count >= config.min_reserve_observations;
        let liquidity_available =
            !config.require_liquidity_not_exhausted || liquidity_exhausted_count == 0;
        let fallback_payout_roots_present = !config.require_fallback_payout_roots
            || fallback_payout_ready_count >= config.min_reserve_observations;
        let settlement_reserve_evidence_present = !config.require_settlement_reserve_evidence
            || settlement_reserve_evidence_count >= config.min_settlement_reserve_observations;
        let reserve_observation_complete = observation_count >= config.min_reserve_observations
            && reserve_coverage_met
            && liquidity_available
            && fallback_payout_roots_present
            && settlement_reserve_evidence_present
            && source.source_reserve_receipts_present;
        let production_blocked = source.source_production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_reserve_evidence && !reserve_observation_complete);
        let reserve_status = reserve_status(
            reserve_observation_complete,
            liquidity_available,
            settlement_reserve_evidence_present,
        )
        .to_string();
        let user_escape_answer =
            user_escape_answer(reserve_observation_complete, liquidity_available).to_string();
        let production_answer = production_answer(production_blocked).to_string();
        let verdict_root = verdict_root(
            config,
            source,
            observation_count,
            covered_count,
            fallback_covered_count,
            liquidity_exhausted_count,
            evidence_missing_count,
            settlement_reserve_evidence_count,
            total_required_reserve_units,
            total_observed_reserve_units,
            total_fallback_payout_units,
            total_reserve_deficit_units,
            aggregate_coverage_bps,
            reserve_observation_complete,
            production_blocked,
            &reserve_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            observation_count,
            covered_count,
            fallback_covered_count,
            liquidity_exhausted_count,
            evidence_missing_count,
            settlement_reserve_evidence_count,
            fallback_payout_ready_count,
            production_blocker_count,
            total_required_reserve_units,
            total_observed_reserve_units,
            total_fallback_payout_units,
            total_reserve_deficit_units,
            aggregate_coverage_bps,
            reserve_coverage_met,
            liquidity_available,
            fallback_payout_roots_present,
            settlement_reserve_evidence_present,
            reserve_observation_complete,
            production_blocked,
            reserve_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_count": self.observation_count,
            "covered_count": self.covered_count,
            "fallback_covered_count": self.fallback_covered_count,
            "liquidity_exhausted_count": self.liquidity_exhausted_count,
            "evidence_missing_count": self.evidence_missing_count,
            "settlement_reserve_evidence_count": self.settlement_reserve_evidence_count,
            "fallback_payout_ready_count": self.fallback_payout_ready_count,
            "production_blocker_count": self.production_blocker_count,
            "total_required_reserve_units": self.total_required_reserve_units,
            "total_observed_reserve_units": self.total_observed_reserve_units,
            "total_fallback_payout_units": self.total_fallback_payout_units,
            "total_reserve_deficit_units": self.total_reserve_deficit_units,
            "aggregate_coverage_bps": self.aggregate_coverage_bps,
            "reserve_coverage_met": self.reserve_coverage_met,
            "liquidity_available": self.liquidity_available,
            "fallback_payout_roots_present": self.fallback_payout_roots_present,
            "settlement_reserve_evidence_present": self.settlement_reserve_evidence_present,
            "reserve_observation_complete": self.reserve_observation_complete,
            "production_blocked": self.production_blocked,
            "reserve_status": self.reserve_status,
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
    pub observations: Vec<ReserveFallbackObservation>,
    pub verdict: ReserveFallbackVerdict,
    pub reserve_observation_root: String,
    pub reserve_coverage_root: String,
    pub fallback_payout_root: String,
    pub liquidity_exhaustion_root: String,
    pub settlement_reserve_evidence_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution_receipt::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution_receipts(&execution_state);
        validate_source(&source)?;
        let observations = ReserveObservationKind::ordered()
            .iter()
            .copied()
            .map(|kind| ReserveFallbackObservation::devnet(&config, &source, kind))
            .collect::<Vec<_>>();
        let verdict = ReserveFallbackVerdict::new(&config, &source, &observations);
        let reserve_observation_root = reserve_observation_vector_root(&observations);
        let reserve_coverage_root = reserve_coverage_bundle_root(&config, &source, &observations);
        let fallback_payout_root =
            fallback_payout_bundle_root(&config, &source, &observations, &verdict);
        let liquidity_exhaustion_root =
            liquidity_exhaustion_root(&config, &source, &observations, &verdict);
        let settlement_reserve_evidence_root =
            settlement_reserve_evidence_bundle_root(&config, &source, &observations, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &observations, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &reserve_observation_root,
            &reserve_coverage_root,
            &fallback_payout_root,
            &liquidity_exhaustion_root,
            &settlement_reserve_evidence_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            observations,
            verdict,
            reserve_observation_root,
            reserve_coverage_root,
            fallback_payout_root,
            liquidity_exhaustion_root,
            settlement_reserve_evidence_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), execution_receipt::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_reserve_fallback_observation_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "reserve_observation_root": self.reserve_observation_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "fallback_payout_root": self.fallback_payout_root,
            "liquidity_exhaustion_root": self.liquidity_exhaustion_root,
            "settlement_reserve_evidence_root": self.settlement_reserve_evidence_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "observations": self
                .observations
                .iter()
                .map(ReserveFallbackObservation::public_record)
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

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATION-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn observation_ordinal(kind: ReserveObservationKind) -> u64 {
    match kind {
        ReserveObservationKind::ReserveCoverage => 1,
        ReserveObservationKind::LiquidityRunway => 2,
        ReserveObservationKind::FallbackPayout => 3,
        ReserveObservationKind::SettlementReserveEvidence => 4,
    }
}

fn required_reserve_units(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
) -> u64 {
    let base = source.execution_receipt_count.max(1) * config.reserve_unit;
    match kind {
        ReserveObservationKind::ReserveCoverage => base,
        ReserveObservationKind::LiquidityRunway => base + config.reserve_unit,
        ReserveObservationKind::FallbackPayout => base / 2,
        ReserveObservationKind::SettlementReserveEvidence => config.reserve_unit,
    }
}

fn observed_reserve_units(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
) -> u64 {
    if !source.source_reserve_receipts_present {
        return 0;
    }
    let covered = source.observed_receipt_count.max(1) * config.reserve_unit;
    match kind {
        ReserveObservationKind::ReserveCoverage => covered,
        ReserveObservationKind::LiquidityRunway => covered.saturating_sub(config.reserve_unit / 2),
        ReserveObservationKind::FallbackPayout => covered / 2,
        ReserveObservationKind::SettlementReserveEvidence => {
            if source.source_settlement_receipts_present {
                config.reserve_unit
            } else {
                0
            }
        }
    }
}

fn fallback_payout_units(
    source: &SourceBundle,
    kind: ReserveObservationKind,
    reserve_deficit_units: u64,
) -> u64 {
    if !source.source_reserve_receipts_present {
        return 0;
    }
    match kind {
        ReserveObservationKind::FallbackPayout => reserve_deficit_units,
        ReserveObservationKind::LiquidityRunway => reserve_deficit_units / 2,
        ReserveObservationKind::ReserveCoverage
        | ReserveObservationKind::SettlementReserveEvidence => 0,
    }
}

fn coverage_bps(observed_units: u64, required_units: u64) -> u64 {
    if required_units == 0 {
        10_000
    } else {
        observed_units.saturating_mul(10_000) / required_units
    }
}

fn observation_status(
    config: &Config,
    kind: ReserveObservationKind,
    coverage_bps: u64,
    settlement_evidence: bool,
    fallback_payout_ready: bool,
    liquidity_exhausted: bool,
) -> ReserveObservationStatus {
    if config.require_settlement_reserve_evidence
        && kind == ReserveObservationKind::SettlementReserveEvidence
        && !settlement_evidence
    {
        ReserveObservationStatus::EvidenceMissing
    } else if liquidity_exhausted {
        ReserveObservationStatus::LiquidityExhausted
    } else if coverage_bps >= config.min_coverage_bps {
        ReserveObservationStatus::Covered
    } else if fallback_payout_ready {
        ReserveObservationStatus::FallbackCovered
    } else {
        ReserveObservationStatus::EvidenceMissing
    }
}

fn reserve_evidence_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    ordinal: u64,
) -> String {
    record_root(
        "reserve-evidence",
        &json!({
            "reserve_fallback_suite": &config.reserve_fallback_suite,
            "source_package_state_root": &source.source_package_state_root,
            "execution_receipt_root": &source.execution_receipt_root,
            "challenge_settlement_bundle_root": &source.challenge_settlement_bundle_root,
            "observation_kind": kind.as_str(),
            "ordinal": ordinal,
            "reserve_receipts_present": source.source_reserve_receipts_present,
            "settlement_receipts_present": source.source_settlement_receipts_present,
        }),
    )
}

fn coverage_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    required_reserve_units: u64,
    observed_reserve_units: u64,
    coverage_bps: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-COVERAGE",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(kind.as_str()),
            HashPart::U64(required_reserve_units),
            HashPart::U64(observed_reserve_units),
            HashPart::U64(coverage_bps),
        ],
        32,
    )
}

fn liquidity_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    reserve_deficit_units: u64,
    liquidity_exhausted: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIQUIDITY",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(kind.as_str()),
            HashPart::U64(reserve_deficit_units),
            HashPart::Str(bool_str(liquidity_exhausted)),
        ],
        32,
    )
}

fn fallback_payout_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    fallback_payout_units: u64,
    reserve_deficit_units: u64,
    fallback_payout_ready: bool,
) -> String {
    record_root(
        "fallback-payout",
        &json!({
            "reserve_fallback_suite": &config.reserve_fallback_suite,
            "recovery_receipt_root": &source.recovery_receipt_root,
            "observation_kind": kind.as_str(),
            "fallback_payout_units": fallback_payout_units,
            "reserve_deficit_units": reserve_deficit_units,
            "fallback_payout_ready": fallback_payout_ready,
            "policy": "fallback payout root covers reserve deficit before production release",
        }),
    )
}

fn settlement_evidence_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    settlement_evidence: bool,
) -> String {
    record_root(
        "settlement-reserve-evidence",
        &json!({
            "reserve_fallback_suite": &config.reserve_fallback_suite,
            "challenge_settlement_bundle_root": &source.challenge_settlement_bundle_root,
            "execution_state_root": &source.execution_state_root,
            "observation_kind": kind.as_str(),
            "settlement_evidence": settlement_evidence,
            "source_package_observed": source.source_package_observed,
        }),
    )
}

fn observation_root(
    config: &Config,
    source: &SourceBundle,
    kind: ReserveObservationKind,
    status: ReserveObservationStatus,
    reserve_evidence_root: &str,
    coverage_root: &str,
    liquidity_root: &str,
    fallback_payout_root: &str,
    settlement_evidence_root: &str,
    required_reserve_units: u64,
    observed_reserve_units: u64,
    fallback_payout_units: u64,
    reserve_deficit_units: u64,
    coverage_bps: u64,
    blocks_production: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATION",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(reserve_evidence_root),
            HashPart::Str(coverage_root),
            HashPart::Str(liquidity_root),
            HashPart::Str(fallback_payout_root),
            HashPart::Str(settlement_evidence_root),
            HashPart::U64(required_reserve_units),
            HashPart::U64(observed_reserve_units),
            HashPart::U64(fallback_payout_units),
            HashPart::U64(reserve_deficit_units),
            HashPart::U64(coverage_bps),
            HashPart::Str(bool_str(blocks_production)),
        ],
        32,
    )
}

fn observation_id(kind: ReserveObservationKind, ordinal: u64, observation_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATION-ID",
        &[
            HashPart::Str(kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(observation_root),
        ],
        16,
    )
}

fn required_outcome(
    status: ReserveObservationStatus,
    kind: ReserveObservationKind,
) -> &'static str {
    match status {
        ReserveObservationStatus::Covered => {
            "reserve evidence covers the force-exit settlement amount"
        }
        ReserveObservationStatus::FallbackCovered => {
            "fallback payout root covers the reserve deficit"
        }
        ReserveObservationStatus::LiquidityExhausted => {
            "production remains held until reserve liquidity is replenished"
        }
        ReserveObservationStatus::EvidenceMissing => match kind {
            ReserveObservationKind::SettlementReserveEvidence => {
                "production remains held until settlement reserve evidence is present"
            }
            ReserveObservationKind::FallbackPayout => {
                "production remains held until fallback payout roots cover the deficit"
            }
            _ => "production remains held until reserve evidence is complete",
        },
    }
}

fn reserve_observation_vector_root(observations: &[ReserveFallbackObservation]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATIONS",
        &observations
            .iter()
            .map(ReserveFallbackObservation::public_record)
            .collect::<Vec<_>>(),
    )
}

fn reserve_coverage_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[ReserveFallbackObservation],
) -> String {
    let records = observations
        .iter()
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "observation_kind": observation.observation_kind.as_str(),
                "coverage_root": &observation.coverage_root,
                "coverage_bps": observation.coverage_bps,
            })
        })
        .collect::<Vec<_>>();
    let coverage_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-COVERAGE-VECTOR",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-COVERAGE-BUNDLE",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&coverage_vector_root),
        ],
        32,
    )
}

fn fallback_payout_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[ReserveFallbackObservation],
    verdict: &ReserveFallbackVerdict,
) -> String {
    let records = observations
        .iter()
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "fallback_payout_root": &observation.fallback_payout_root,
                "fallback_payout_units": observation.fallback_payout_units,
                "fallback_payout_ready": observation.fallback_payout_ready,
            })
        })
        .collect::<Vec<_>>();
    let payout_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-FALLBACK-PAYOUTS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-FALLBACK-PAYOUT-BUNDLE",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&payout_vector_root),
            HashPart::U64(verdict.total_fallback_payout_units),
            HashPart::Str(bool_str(verdict.fallback_payout_roots_present)),
        ],
        32,
    )
}

fn liquidity_exhaustion_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[ReserveFallbackObservation],
    verdict: &ReserveFallbackVerdict,
) -> String {
    let exhausted = observations
        .iter()
        .filter(|observation| observation.liquidity_exhausted)
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "liquidity_root": &observation.liquidity_root,
                "reserve_deficit_units": observation.reserve_deficit_units,
            })
        })
        .collect::<Vec<_>>();
    let exhausted_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIQUIDITY-EXHAUSTION",
        &exhausted,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-LIQUIDITY-EXHAUSTION-BUNDLE",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(&exhausted_root),
            HashPart::U64(verdict.liquidity_exhausted_count),
            HashPart::Str(bool_str(verdict.liquidity_available)),
        ],
        32,
    )
}

fn settlement_reserve_evidence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[ReserveFallbackObservation],
    verdict: &ReserveFallbackVerdict,
) -> String {
    let records = observations
        .iter()
        .filter(|observation| observation.settlement_evidence)
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "settlement_evidence_root": &observation.settlement_evidence_root,
                "observation_kind": observation.observation_kind.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let settlement_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-RESERVE-EVIDENCE",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-RESERVE-EVIDENCE-BUNDLE",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.challenge_settlement_bundle_root),
            HashPart::Str(&settlement_vector_root),
            HashPart::U64(verdict.settlement_reserve_evidence_count),
            HashPart::Str(bool_str(verdict.settlement_reserve_evidence_present)),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[ReserveFallbackObservation],
    verdict: &ReserveFallbackVerdict,
) -> String {
    let blockers = observations
        .iter()
        .filter(|observation| observation.blocks_production)
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "observation_kind": observation.observation_kind.as_str(),
                "status": observation.status.as_str(),
                "settlement_evidence_root": &observation.settlement_evidence_root,
                "fallback_payout_root": &observation.fallback_payout_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_production_hold_root),
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
    reserve_observation_root: &str,
    reserve_coverage_root: &str,
    fallback_payout_root: &str,
    liquidity_exhaustion_root: &str,
    settlement_reserve_evidence_root: &str,
    production_hold_root: &str,
    verdict: &ReserveFallbackVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATION-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(reserve_observation_root),
            HashPart::Str(reserve_coverage_root),
            HashPart::Str(fallback_payout_root),
            HashPart::Str(liquidity_exhaustion_root),
            HashPart::Str(settlement_reserve_evidence_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    observation_count: u64,
    covered_count: u64,
    fallback_covered_count: u64,
    liquidity_exhausted_count: u64,
    evidence_missing_count: u64,
    settlement_reserve_evidence_count: u64,
    total_required_reserve_units: u64,
    total_observed_reserve_units: u64,
    total_fallback_payout_units: u64,
    total_reserve_deficit_units: u64,
    aggregate_coverage_bps: u64,
    reserve_observation_complete: bool,
    production_blocked: bool,
    reserve_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-FALLBACK-OBSERVATION-VERDICT",
        &[
            HashPart::Str(&config.reserve_fallback_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::U64(observation_count),
            HashPart::U64(covered_count),
            HashPart::U64(fallback_covered_count),
            HashPart::U64(liquidity_exhausted_count),
            HashPart::U64(evidence_missing_count),
            HashPart::U64(settlement_reserve_evidence_count),
            HashPart::U64(total_required_reserve_units),
            HashPart::U64(total_observed_reserve_units),
            HashPart::U64(total_fallback_payout_units),
            HashPart::U64(total_reserve_deficit_units),
            HashPart::U64(aggregate_coverage_bps),
            HashPart::Str(bool_str(reserve_observation_complete)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(reserve_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn reserve_status(
    reserve_observation_complete: bool,
    liquidity_available: bool,
    settlement_reserve_evidence_present: bool,
) -> &'static str {
    if reserve_observation_complete {
        "reserve_observed"
    } else if !liquidity_available {
        "liquidity_exhausted"
    } else if !settlement_reserve_evidence_present {
        "settlement_reserve_evidence_missing"
    } else {
        "reserve_evidence_incomplete"
    }
}

fn user_escape_answer(
    reserve_observation_complete: bool,
    liquidity_available: bool,
) -> &'static str {
    if reserve_observation_complete {
        "force_exit_settlement_has_reserve_or_fallback_coverage"
    } else if liquidity_available {
        "wallet_escape_waits_for_reserve_evidence"
    } else {
        "wallet_escape_uses_fallback_payout_until_reserve_refills"
    }
}

fn production_answer(production_blocked: bool) -> &'static str {
    if production_blocked {
        "hold_production_until_force_exit_settlement_reserve_evidence_is_complete"
    } else {
        "production_release_allowed_after_reserve_fallback_observation"
    }
}

fn count_status(
    observations: &[ReserveFallbackObservation],
    status: ReserveObservationStatus,
) -> u64 {
    observations
        .iter()
        .filter(|observation| observation.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "reserve fallback observation chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "reserve fallback observation protocol mismatch",
    )?;
    ensure(
        config.min_reserve_observations > 0,
        "reserve fallback observation requires observations",
    )?;
    ensure(
        config.min_settlement_reserve_observations > 0,
        "reserve fallback observation requires settlement reserve evidence",
    )?;
    ensure(
        config.reserve_unit > 0,
        "reserve fallback observation requires reserve unit",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_state_root.is_empty(),
        "reserve fallback observation missing execution state root",
    )?;
    ensure(
        !source.execution_receipt_root.is_empty(),
        "reserve fallback observation missing execution receipt root",
    )?;
    ensure(
        source.execution_receipt_count > 0,
        "reserve fallback observation missing execution receipts",
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
        challenge_settlement_bundle_root: record_root(
            "fallback-challenge-settlement",
            &json!({"reason": &reason}),
        ),
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        execution_production_hold_root: record_root(
            "fallback-production-hold",
            &json!({"reason": &reason}),
        ),
        source_package_state_root: record_root("fallback-package", &json!({"reason": &reason})),
        source_reserve_receipts_present: false,
        source_settlement_receipts_present: false,
        source_package_observed: false,
        source_production_blocked: true,
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        deferred_receipt_count: 1,
        production_blocker_count: 1,
        source_execution_status: "fallback".to_string(),
        source_user_escape_answer: reason,
        source_production_answer: "fallback".to_string(),
    };
    let observations = ReserveObservationKind::ordered()
        .iter()
        .copied()
        .map(|kind| ReserveFallbackObservation::devnet(&config, &source, kind))
        .collect::<Vec<_>>();
    let verdict = ReserveFallbackVerdict::new(&config, &source, &observations);
    let reserve_observation_root = reserve_observation_vector_root(&observations);
    let reserve_coverage_root = reserve_coverage_bundle_root(&config, &source, &observations);
    let fallback_payout_root =
        fallback_payout_bundle_root(&config, &source, &observations, &verdict);
    let liquidity_exhaustion_root =
        liquidity_exhaustion_root(&config, &source, &observations, &verdict);
    let settlement_reserve_evidence_root =
        settlement_reserve_evidence_bundle_root(&config, &source, &observations, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &observations, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &reserve_observation_root,
        &reserve_coverage_root,
        &fallback_payout_root,
        &liquidity_exhaustion_root,
        &settlement_reserve_evidence_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        observations,
        verdict,
        reserve_observation_root,
        reserve_coverage_root,
        fallback_payout_root,
        liquidity_exhaustion_root,
        settlement_reserve_evidence_root,
        production_hold_root,
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
