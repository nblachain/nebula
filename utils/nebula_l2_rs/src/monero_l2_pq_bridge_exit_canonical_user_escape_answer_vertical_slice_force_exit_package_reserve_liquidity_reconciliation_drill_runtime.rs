use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReserveLiquidityReconciliationDrillRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RESERVE_LIQUIDITY_RECONCILIATION_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-reserve-liquidity-reconciliation-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RESERVE_LIQUIDITY_RECONCILIATION_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DRILL_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-reserve-liquidity-reconciliation-drill-v1";
pub const DEFAULT_DRILL_HEIGHT: u64 = 75_000;
pub const DEFAULT_RESERVE_UNIT: u64 = 1_000_000;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_TARGET_BUFFER_BPS: u64 = 1_250;
pub const DEFAULT_MIN_SIGNER_ATTESTATIONS: u64 = 4;
pub const DEFAULT_REQUIRED_DRILL_RECORDS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub drill_suite: String,
    pub drill_height: u64,
    pub reserve_unit: u64,
    pub min_reserve_coverage_bps: u64,
    pub target_buffer_bps: u64,
    pub min_signer_attestations: u64,
    pub required_drill_records: u64,
    pub require_hold_on_shortfall: bool,
    pub require_release_on_full_coverage: bool,
    pub roots_only: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            drill_suite: DRILL_SUITE.to_string(),
            drill_height: DEFAULT_DRILL_HEIGHT,
            reserve_unit: DEFAULT_RESERVE_UNIT,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            target_buffer_bps: DEFAULT_TARGET_BUFFER_BPS,
            min_signer_attestations: DEFAULT_MIN_SIGNER_ATTESTATIONS,
            required_drill_records: DEFAULT_REQUIRED_DRILL_RECORDS,
            require_hold_on_shortfall: true,
            require_release_on_full_coverage: true,
            roots_only: true,
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
            "drill_suite": self.drill_suite,
            "drill_height": self.drill_height,
            "reserve_unit": self.reserve_unit,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "target_buffer_bps": self.target_buffer_bps,
            "min_signer_attestations": self.min_signer_attestations,
            "required_drill_records": self.required_drill_records,
            "require_hold_on_shortfall": self.require_hold_on_shortfall,
            "require_release_on_full_coverage": self.require_release_on_full_coverage,
            "roots_only": self.roots_only,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillLane {
    ReserveReport,
    BucketedLiability,
    WithdrawalQueuePressure,
    LiquidityBuffer,
    SignerAttestation,
    HoldReleaseVerdict,
}

impl DrillLane {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::ReserveReport,
            Self::BucketedLiability,
            Self::WithdrawalQueuePressure,
            Self::LiquidityBuffer,
            Self::SignerAttestation,
            Self::HoldReleaseVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReserveReport => "reserve_report",
            Self::BucketedLiability => "bucketed_liability",
            Self::WithdrawalQueuePressure => "withdrawal_queue_pressure",
            Self::LiquidityBuffer => "liquidity_buffer",
            Self::SignerAttestation => "signer_attestation",
            Self::HoldReleaseVerdict => "hold_release_verdict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillStatus {
    ReleaseReady,
    HoldRequired,
}

impl DrillStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseReady => "release_ready",
            Self::HoldRequired => "hold_required",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub drill_record_count: u64,
    pub reserve_report_count: u64,
    pub bucketed_liability_count: u64,
    pub withdrawal_pressure_count: u64,
    pub liquidity_buffer_count: u64,
    pub signer_attestation_count: u64,
    pub hold_verdict_count: u64,
    pub release_verdict_count: u64,
    pub total_reported_reserve_units: u64,
    pub total_liability_units: u64,
    pub total_queued_withdrawal_units: u64,
    pub total_buffer_units: u64,
}

impl Counters {
    pub fn from_records(records: &[DrillRecord]) -> Self {
        let mut counters = Self::default();
        counters.drill_record_count = records.len() as u64;
        for record in records {
            counters.total_reported_reserve_units = counters
                .total_reported_reserve_units
                .saturating_add(record.reported_reserve_units);
            counters.total_liability_units = counters
                .total_liability_units
                .saturating_add(record.liability_units);
            counters.total_queued_withdrawal_units = counters
                .total_queued_withdrawal_units
                .saturating_add(record.queued_withdrawal_units);
            counters.total_buffer_units = counters
                .total_buffer_units
                .saturating_add(record.buffer_units);
            if record.status == DrillStatus::ReleaseReady {
                counters.release_verdict_count += 1;
            } else {
                counters.hold_verdict_count += 1;
            }
            match record.lane {
                DrillLane::ReserveReport => counters.reserve_report_count += 1,
                DrillLane::BucketedLiability => counters.bucketed_liability_count += 1,
                DrillLane::WithdrawalQueuePressure => counters.withdrawal_pressure_count += 1,
                DrillLane::LiquidityBuffer => counters.liquidity_buffer_count += 1,
                DrillLane::SignerAttestation => counters.signer_attestation_count += 1,
                DrillLane::HoldReleaseVerdict => {}
            }
        }
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrillRecord {
    pub record_id: String,
    pub ordinal: u64,
    pub lane: DrillLane,
    pub reserve_report_root: String,
    pub bucketed_liability_root: String,
    pub withdrawal_queue_pressure_root: String,
    pub liquidity_buffer_root: String,
    pub signer_attestation_root: String,
    pub hold_release_verdict_root: String,
    pub reported_reserve_units: u64,
    pub liability_units: u64,
    pub queued_withdrawal_units: u64,
    pub buffer_units: u64,
    pub signer_attestation_count: u64,
    pub coverage_bps: u64,
    pub queue_pressure_bps: u64,
    pub hold_required: bool,
    pub release_allowed: bool,
    pub status: DrillStatus,
    pub record_root: String,
}

impl DrillRecord {
    pub fn devnet(config: &Config, lane: DrillLane, ordinal: u64) -> Self {
        let liability_units = liability_units(config, lane);
        let queued_withdrawal_units = queued_withdrawal_units(config, lane);
        let buffer_units = buffer_units(config, lane);
        let reported_reserve_units = reported_reserve_units(config, lane, liability_units);
        let signer_attestation_count = signer_attestation_count(config, lane);
        let coverage_bps = bps(reported_reserve_units, liability_units);
        let queue_pressure_bps = bps(queued_withdrawal_units, reported_reserve_units);
        let hold_required = config.require_hold_on_shortfall
            && (coverage_bps < config.min_reserve_coverage_bps
                || signer_attestation_count < config.min_signer_attestations);
        let release_allowed = config.require_release_on_full_coverage
            && !hold_required
            && buffer_units >= queued_withdrawal_units;
        let status = if release_allowed {
            DrillStatus::ReleaseReady
        } else {
            DrillStatus::HoldRequired
        };
        let reserve_report_root = lane_root(
            config,
            lane,
            "reserve-report",
            reported_reserve_units,
            liability_units,
            coverage_bps,
        );
        let bucketed_liability_root = lane_root(
            config,
            lane,
            "bucketed-liability",
            liability_units,
            queued_withdrawal_units,
            queue_pressure_bps,
        );
        let withdrawal_queue_pressure_root = lane_root(
            config,
            lane,
            "withdrawal-queue-pressure",
            queued_withdrawal_units,
            reported_reserve_units,
            queue_pressure_bps,
        );
        let liquidity_buffer_root = lane_root(
            config,
            lane,
            "liquidity-buffer",
            buffer_units,
            queued_withdrawal_units,
            config.target_buffer_bps,
        );
        let signer_attestation_root = lane_root(
            config,
            lane,
            "signer-attestation",
            signer_attestation_count,
            config.min_signer_attestations,
            config.min_reserve_coverage_bps,
        );
        let hold_release_verdict_root = verdict_lane_root(
            config,
            lane,
            status,
            hold_required,
            release_allowed,
            coverage_bps,
            queue_pressure_bps,
        );
        let record_root = drill_record_root(
            config,
            lane,
            ordinal,
            &reserve_report_root,
            &bucketed_liability_root,
            &withdrawal_queue_pressure_root,
            &liquidity_buffer_root,
            &signer_attestation_root,
            &hold_release_verdict_root,
            status,
        );
        let record_id = drill_record_id(lane, ordinal, &record_root);
        Self {
            record_id,
            ordinal,
            lane,
            reserve_report_root,
            bucketed_liability_root,
            withdrawal_queue_pressure_root,
            liquidity_buffer_root,
            signer_attestation_root,
            hold_release_verdict_root,
            reported_reserve_units,
            liability_units,
            queued_withdrawal_units,
            buffer_units,
            signer_attestation_count,
            coverage_bps,
            queue_pressure_bps,
            hold_required,
            release_allowed,
            status,
            record_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_id": self.record_id,
            "ordinal": self.ordinal,
            "lane": self.lane.as_str(),
            "reserve_report_root": self.reserve_report_root,
            "bucketed_liability_root": self.bucketed_liability_root,
            "withdrawal_queue_pressure_root": self.withdrawal_queue_pressure_root,
            "liquidity_buffer_root": self.liquidity_buffer_root,
            "signer_attestation_root": self.signer_attestation_root,
            "hold_release_verdict_root": self.hold_release_verdict_root,
            "reported_reserve_units": self.reported_reserve_units,
            "liability_units": self.liability_units,
            "queued_withdrawal_units": self.queued_withdrawal_units,
            "buffer_units": self.buffer_units,
            "signer_attestation_count": self.signer_attestation_count,
            "coverage_bps": self.coverage_bps,
            "queue_pressure_bps": self.queue_pressure_bps,
            "hold_required": self.hold_required,
            "release_allowed": self.release_allowed,
            "status": self.status.as_str(),
            "record_root": self.record_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrillVerdict {
    pub drill_complete: bool,
    pub reserve_coverage_met: bool,
    pub signer_quorum_met: bool,
    pub liquidity_buffer_sufficient: bool,
    pub release_allowed: bool,
    pub production_blocked: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl DrillVerdict {
    pub fn new(config: &Config, counters: &Counters) -> Self {
        let reserve_coverage_met = bps(
            counters.total_reported_reserve_units,
            counters.total_liability_units,
        ) >= config.min_reserve_coverage_bps;
        let signer_quorum_met = counters.signer_attestation_count > 0
            && counters
                .signer_attestation_count
                .saturating_mul(config.min_signer_attestations)
                >= config.min_signer_attestations;
        let liquidity_buffer_sufficient =
            counters.total_buffer_units >= counters.total_queued_withdrawal_units;
        let drill_complete = counters.drill_record_count >= config.required_drill_records;
        let release_allowed = drill_complete
            && reserve_coverage_met
            && signer_quorum_met
            && liquidity_buffer_sufficient
            && counters.hold_verdict_count == 0;
        let production_blocked = !release_allowed;
        let user_escape_answer = user_escape_answer(release_allowed).to_string();
        let production_answer = production_answer(production_blocked).to_string();
        let verdict_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-VERDICT",
            &[
                HashPart::Str(&config.drill_suite),
                HashPart::Str(&counters.state_root()),
                HashPart::Str(bool_str(drill_complete)),
                HashPart::Str(bool_str(reserve_coverage_met)),
                HashPart::Str(bool_str(signer_quorum_met)),
                HashPart::Str(bool_str(liquidity_buffer_sufficient)),
                HashPart::Str(bool_str(release_allowed)),
                HashPart::Str(bool_str(production_blocked)),
                HashPart::Str(&user_escape_answer),
                HashPart::Str(&production_answer),
            ],
            32,
        );
        Self {
            drill_complete,
            reserve_coverage_met,
            signer_quorum_met,
            liquidity_buffer_sufficient,
            release_allowed,
            production_blocked,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub counters_root: String,
    pub drill_record_root: String,
    pub reserve_report_root: String,
    pub bucketed_liability_root: String,
    pub withdrawal_queue_pressure_root: String,
    pub liquidity_buffer_root: String,
    pub signer_attestation_root: String,
    pub hold_release_verdict_root: String,
    pub verdict_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub counters: Counters,
    pub drill_records: Vec<DrillRecord>,
    pub roots: Roots,
    pub verdict: DrillVerdict,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        validate_config(&config)?;
        let drill_records = DrillLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| DrillRecord::devnet(&config, *lane, index as u64 + 1))
            .collect::<Vec<_>>();
        let counters = Counters::from_records(&drill_records);
        let verdict = DrillVerdict::new(&config, &counters);
        let roots = roots(&config, &counters, &drill_records, &verdict);
        let state_commitment_root = state_commitment_root(&config, &roots, &verdict);
        Ok(Self {
            config,
            counters,
            drill_records,
            roots,
            verdict,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_reserve_liquidity_reconciliation_drill_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "config": self.config.public_record(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "roots_root": self.roots.state_root(),
            "verdict": self.verdict.public_record(),
            "drill_records": self
                .drill_records
                .iter()
                .map(DrillRecord::public_record)
                .collect::<Vec<_>>(),
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
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
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn roots(
    config: &Config,
    counters: &Counters,
    records: &[DrillRecord],
    verdict: &DrillVerdict,
) -> Roots {
    Roots {
        config_root: config.state_root(),
        counters_root: counters.state_root(),
        drill_record_root: drill_record_vector_root(records),
        reserve_report_root: root_for_lane_field("RESERVE-REPORTS", records, |record| {
            record.reserve_report_root.clone()
        }),
        bucketed_liability_root: root_for_lane_field("BUCKETED-LIABILITIES", records, |record| {
            record.bucketed_liability_root.clone()
        }),
        withdrawal_queue_pressure_root: root_for_lane_field(
            "WITHDRAWAL-QUEUE-PRESSURE",
            records,
            |record| record.withdrawal_queue_pressure_root.clone(),
        ),
        liquidity_buffer_root: root_for_lane_field("LIQUIDITY-BUFFER", records, |record| {
            record.liquidity_buffer_root.clone()
        }),
        signer_attestation_root: root_for_lane_field("SIGNER-ATTESTATIONS", records, |record| {
            record.signer_attestation_root.clone()
        }),
        hold_release_verdict_root: root_for_lane_field(
            "HOLD-RELEASE-VERDICTS",
            records,
            |record| record.hold_release_verdict_root.clone(),
        ),
        verdict_root: verdict.verdict_root.clone(),
    }
}

fn drill_record_vector_root(records: &[DrillRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-RECORDS",
        &records
            .iter()
            .map(DrillRecord::public_record)
            .collect::<Vec<_>>(),
    )
}

fn root_for_lane_field<F>(label: &str, records: &[DrillRecord], field: F) -> String
where
    F: Fn(&DrillRecord) -> String,
{
    let entries = records
        .iter()
        .map(|record| {
            json!({
                "record_id": record.record_id,
                "lane": record.lane.as_str(),
                "root": field(record),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-{label}"),
        &entries,
    )
}

fn lane_root(
    config: &Config,
    lane: DrillLane,
    label: &str,
    primary_units: u64,
    secondary_units: u64,
    metric_bps: u64,
) -> String {
    record_root(
        label,
        &json!({
            "drill_suite": &config.drill_suite,
            "drill_height": config.drill_height,
            "lane": lane.as_str(),
            "primary_units": primary_units,
            "secondary_units": secondary_units,
            "metric_bps": metric_bps,
        }),
    )
}

fn verdict_lane_root(
    config: &Config,
    lane: DrillLane,
    status: DrillStatus,
    hold_required: bool,
    release_allowed: bool,
    coverage_bps: u64,
    queue_pressure_bps: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-HOLD-RELEASE-LANE",
        &[
            HashPart::Str(&config.drill_suite),
            HashPart::Str(lane.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(hold_required)),
            HashPart::Str(bool_str(release_allowed)),
            HashPart::U64(coverage_bps),
            HashPart::U64(queue_pressure_bps),
        ],
        32,
    )
}

#[allow(clippy::too_many_arguments)]
fn drill_record_root(
    config: &Config,
    lane: DrillLane,
    ordinal: u64,
    reserve_report_root: &str,
    bucketed_liability_root: &str,
    withdrawal_queue_pressure_root: &str,
    liquidity_buffer_root: &str,
    signer_attestation_root: &str,
    hold_release_verdict_root: &str,
    status: DrillStatus,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-ROW",
        &[
            HashPart::Str(&config.drill_suite),
            HashPart::U64(ordinal),
            HashPart::Str(lane.as_str()),
            HashPart::Str(reserve_report_root),
            HashPart::Str(bucketed_liability_root),
            HashPart::Str(withdrawal_queue_pressure_root),
            HashPart::Str(liquidity_buffer_root),
            HashPart::Str(signer_attestation_root),
            HashPart::Str(hold_release_verdict_root),
            HashPart::Str(status.as_str()),
        ],
        32,
    )
}

fn drill_record_id(lane: DrillLane, ordinal: u64, drill_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(drill_root),
        ],
        16,
    )
}

fn state_commitment_root(config: &Config, roots: &Roots, verdict: &DrillVerdict) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RESERVE-LIQUIDITY-RECONCILIATION-DRILL-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&roots.reserve_report_root),
            HashPart::Str(&roots.bucketed_liability_root),
            HashPart::Str(&roots.withdrawal_queue_pressure_root),
            HashPart::Str(&roots.liquidity_buffer_root),
            HashPart::Str(&roots.signer_attestation_root),
            HashPart::Str(&roots.hold_release_verdict_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn liability_units(config: &Config, lane: DrillLane) -> u64 {
    match lane {
        DrillLane::ReserveReport => config.reserve_unit.saturating_mul(12),
        DrillLane::BucketedLiability => config.reserve_unit.saturating_mul(8),
        DrillLane::WithdrawalQueuePressure => config.reserve_unit.saturating_mul(6),
        DrillLane::LiquidityBuffer => config.reserve_unit.saturating_mul(4),
        DrillLane::SignerAttestation => config.reserve_unit.saturating_mul(3),
        DrillLane::HoldReleaseVerdict => config.reserve_unit.saturating_mul(2),
    }
}

fn queued_withdrawal_units(config: &Config, lane: DrillLane) -> u64 {
    match lane {
        DrillLane::WithdrawalQueuePressure => config.reserve_unit.saturating_mul(3),
        DrillLane::LiquidityBuffer => config.reserve_unit,
        DrillLane::HoldReleaseVerdict => config.reserve_unit / 2,
        _ => config.reserve_unit,
    }
}

fn buffer_units(config: &Config, lane: DrillLane) -> u64 {
    match lane {
        DrillLane::WithdrawalQueuePressure => config.reserve_unit.saturating_mul(2),
        DrillLane::LiquidityBuffer => config.reserve_unit.saturating_mul(3),
        _ => config.reserve_unit.saturating_mul(2),
    }
}

fn reported_reserve_units(config: &Config, lane: DrillLane, liability_units: u64) -> u64 {
    match lane {
        DrillLane::WithdrawalQueuePressure => liability_units.saturating_sub(config.reserve_unit),
        _ => liability_units.saturating_add(config.reserve_unit),
    }
}

fn signer_attestation_count(config: &Config, lane: DrillLane) -> u64 {
    match lane {
        DrillLane::SignerAttestation | DrillLane::HoldReleaseVerdict => {
            config.min_signer_attestations
        }
        _ => config.min_signer_attestations.saturating_add(1),
    }
}

fn bps(numerator: u64, denominator: u64) -> u64 {
    if denominator == 0 {
        return 0;
    }
    numerator.saturating_mul(10_000) / denominator
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "reserve liquidity reconciliation drill chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "reserve liquidity reconciliation drill protocol mismatch",
    )?;
    ensure(
        config.schema_version == SCHEMA_VERSION,
        "reserve liquidity reconciliation drill schema mismatch",
    )?;
    ensure(
        config.reserve_unit > 0,
        "reserve liquidity reconciliation drill reserve unit is zero",
    )?;
    ensure(
        config.required_drill_records > 0,
        "reserve liquidity reconciliation drill requires records",
    )?;
    ensure(
        config.roots_only,
        "reserve liquidity reconciliation drill must remain roots-only",
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
    let drill_records = DrillLane::ordered()
        .iter()
        .enumerate()
        .map(|(index, lane)| DrillRecord::devnet(&config, *lane, index as u64 + 1))
        .collect::<Vec<_>>();
    let counters = Counters::from_records(&drill_records);
    let mut verdict = DrillVerdict::new(&config, &counters);
    verdict.production_blocked = true;
    verdict.release_allowed = false;
    verdict.user_escape_answer = reason;
    verdict.production_answer = "hold_production_until_drill_config_is_valid".to_string();
    verdict.verdict_root = record_root("fallback-verdict", &verdict.public_record());
    let roots = roots(&config, &counters, &drill_records, &verdict);
    let state_commitment_root = state_commitment_root(&config, &roots, &verdict);
    State {
        config,
        counters,
        drill_records,
        roots,
        verdict,
        state_commitment_root,
    }
}

fn user_escape_answer(release_allowed: bool) -> &'static str {
    if release_allowed {
        "user_escape_release_allowed_after_reserve_liquidity_reconciliation_drill"
    } else {
        "user_escape_force_exit_waits_for_reserve_liquidity_reconciliation_clearance"
    }
}

fn production_answer(production_blocked: bool) -> &'static str {
    if production_blocked {
        "hold_production_until_reserve_reports_liabilities_queue_buffer_and_signers_reconcile"
    } else {
        "production_release_allowed_after_reserve_liquidity_reconciliation_drill"
    }
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
