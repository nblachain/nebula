use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const GUARD_SUITE: &str =
    "monero-l2-wave101-release-execution-disbursement-liquidity-throttle-guard-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 101;
pub const WAVE100_SETTLEMENT_NOTARY_WAVE: u64 = 100;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 101;
pub const DEFAULT_MIN_NOTARY_QUORUM_BPS: u64 = 6_700;
pub const DEFAULT_MIN_LIQUIDITY_COVERAGE_BPS: u64 = 10_500;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_250;
pub const DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS: u64 = 8_000;
pub const DEFAULT_MAX_FEE_NETTING_DRIFT_BPS: u64 = 4;
pub const DEFAULT_MAX_PAYOUT_THROTTLE_BPS: u64 = 2_500;
pub const DEFAULT_MAX_RECORDS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-pq-reserve-privacy-lane-runtime";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    Wave100SettlementNotary,
    LiquidityReservation,
    PrivacyFeeNetting,
    PayoutThrottle,
    ReserveBuffer,
    PrivacyBudget,
    CircuitBreaker,
    OperatorReviewerSignoff,
    CommandHint,
    DeterministicGuard,
}

impl RecordKind {
    pub fn all() -> [Self; 10] {
        [
            Self::Wave100SettlementNotary,
            Self::LiquidityReservation,
            Self::PrivacyFeeNetting,
            Self::PayoutThrottle,
            Self::ReserveBuffer,
            Self::PrivacyBudget,
            Self::CircuitBreaker,
            Self::OperatorReviewerSignoff,
            Self::CommandHint,
            Self::DeterministicGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotary => "wave100_settlement_notary",
            Self::LiquidityReservation => "liquidity_reservation",
            Self::PrivacyFeeNetting => "privacy_fee_netting",
            Self::PayoutThrottle => "payout_throttle",
            Self::ReserveBuffer => "reserve_buffer",
            Self::PrivacyBudget => "privacy_budget",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorReviewerSignoff => "operator_reviewer_signoff",
            Self::CommandHint => "command_hint",
            Self::DeterministicGuard => "deterministic_guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Denied,
    Blocked,
    LiquidityHeld,
    ShadowReady,
}

impl RecordStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::Blocked => "blocked",
            Self::LiquidityHeld => "liquidity_held",
            Self::ShadowReady => "shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Blocker {
    HeavyGatesNotRun,
    ProductionDenied,
    ReleaseExecutionDenied,
    DisbursementDenied,
    NoDisbursementReservation,
    Wave100SettlementNotaryActive,
    Wave100SettlementNotaryRootMissing,
    LiquidityReservationMissing,
    LiquidityReservationBlocked,
    LiquidityCoverageLow,
    PrivacyFeeNettingMissing,
    PrivacyFeeNettingBlocked,
    FeeNettingDriftHigh,
    PayoutThrottleMissing,
    PayoutThrottleActive,
    PayoutRateHigh,
    ReserveBufferMissing,
    ReserveBufferLow,
    PrivacyBudgetMissing,
    PrivacyBudgetLow,
    CircuitBreakerOpen,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    RootsOnlyBoundary,
    DeterministicRootMissing,
    DuplicateRecordRoot,
    RecordCapacityReached,
}

impl Blocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
            Self::ProductionDenied => "production_denied",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::DisbursementDenied => "disbursement_denied",
            Self::NoDisbursementReservation => "no_disbursement_reservation",
            Self::Wave100SettlementNotaryActive => "wave100_settlement_notary_active",
            Self::Wave100SettlementNotaryRootMissing => "wave100_settlement_notary_root_missing",
            Self::LiquidityReservationMissing => "liquidity_reservation_missing",
            Self::LiquidityReservationBlocked => "liquidity_reservation_blocked",
            Self::LiquidityCoverageLow => "liquidity_coverage_low",
            Self::PrivacyFeeNettingMissing => "privacy_fee_netting_missing",
            Self::PrivacyFeeNettingBlocked => "privacy_fee_netting_blocked",
            Self::FeeNettingDriftHigh => "fee_netting_drift_high",
            Self::PayoutThrottleMissing => "payout_throttle_missing",
            Self::PayoutThrottleActive => "payout_throttle_active",
            Self::PayoutRateHigh => "payout_rate_high",
            Self::ReserveBufferMissing => "reserve_buffer_missing",
            Self::ReserveBufferLow => "reserve_buffer_low",
            Self::PrivacyBudgetMissing => "privacy_budget_missing",
            Self::PrivacyBudgetLow => "privacy_budget_low",
            Self::CircuitBreakerOpen => "circuit_breaker_open",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DeterministicRootMissing => "deterministic_root_missing",
            Self::DuplicateRecordRoot => "duplicate_record_root",
            Self::RecordCapacityReached => "record_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    LiquidityThrottleGuardActive,
    DisbursementShadowReady,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::LiquidityThrottleGuardActive => "liquidity_throttle_guard_active",
            Self::DisbursementShadowReady => "disbursement_shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    ImportWave100SettlementNotaryRoots,
    AttachLiquidityReservationRoots,
    AttachPrivacyFeeNettingRoots,
    AttachPayoutThrottleRoots,
    AttachReserveBufferRoots,
    AttachPrivacyBudgetRoots,
    AttachCircuitBreakerRoots,
    AttachOperatorReviewerSignoffRoots,
    KeepReleaseExecutionDenied,
    KeepDisbursementReservationsEmpty,
    PublishRootsOnlyThrottleGuard,
}

impl CommandHint {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave100SettlementNotaryRoots,
            Self::AttachLiquidityReservationRoots,
            Self::AttachPrivacyFeeNettingRoots,
            Self::AttachPayoutThrottleRoots,
            Self::AttachReserveBufferRoots,
            Self::AttachPrivacyBudgetRoots,
            Self::AttachCircuitBreakerRoots,
            Self::AttachOperatorReviewerSignoffRoots,
            Self::KeepReleaseExecutionDenied,
            Self::KeepDisbursementReservationsEmpty,
            Self::PublishRootsOnlyThrottleGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave100SettlementNotaryRoots => "import_wave100_settlement_notary_roots",
            Self::AttachLiquidityReservationRoots => "attach_liquidity_reservation_roots",
            Self::AttachPrivacyFeeNettingRoots => "attach_privacy_fee_netting_roots",
            Self::AttachPayoutThrottleRoots => "attach_payout_throttle_roots",
            Self::AttachReserveBufferRoots => "attach_reserve_buffer_roots",
            Self::AttachPrivacyBudgetRoots => "attach_privacy_budget_roots",
            Self::AttachCircuitBreakerRoots => "attach_circuit_breaker_roots",
            Self::AttachOperatorReviewerSignoffRoots => "attach_operator_reviewer_signoff_roots",
            Self::KeepReleaseExecutionDenied => "keep_release_execution_denied",
            Self::KeepDisbursementReservationsEmpty => "keep_disbursement_reservations_empty",
            Self::PublishRootsOnlyThrottleGuard => "publish_roots_only_throttle_guard",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub guard_suite: String,
    pub wave: u64,
    pub wave100_settlement_notary_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_notary_quorum_bps: u64,
    pub min_liquidity_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub min_privacy_budget_remaining_bps: u64,
    pub max_fee_netting_drift_bps: u64,
    pub max_payout_throttle_bps: u64,
    pub fail_closed: bool,
    pub heavy_gates_ran: bool,
    pub production_allowed: bool,
    pub release_execution_allowed: bool,
    pub disbursement_allowed: bool,
    pub disbursement_reservations_enabled: bool,
    pub wave100_settlement_notary_active: bool,
    pub liquidity_reservation_blocked: bool,
    pub privacy_fee_netting_blocked: bool,
    pub payout_throttle_active: bool,
    pub reserve_buffer_blocked: bool,
    pub privacy_budget_blocked: bool,
    pub circuit_breaker_open: bool,
    pub roots_only_public_record: bool,
    pub max_records: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            guard_suite: GUARD_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            wave100_settlement_notary_wave: WAVE100_SETTLEMENT_NOTARY_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_notary_quorum_bps: DEFAULT_MIN_NOTARY_QUORUM_BPS,
            min_liquidity_coverage_bps: DEFAULT_MIN_LIQUIDITY_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            min_privacy_budget_remaining_bps: DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS,
            max_fee_netting_drift_bps: DEFAULT_MAX_FEE_NETTING_DRIFT_BPS,
            max_payout_throttle_bps: DEFAULT_MAX_PAYOUT_THROTTLE_BPS,
            fail_closed: true,
            heavy_gates_ran: false,
            production_allowed: false,
            release_execution_allowed: false,
            disbursement_allowed: false,
            disbursement_reservations_enabled: false,
            wave100_settlement_notary_active: true,
            liquidity_reservation_blocked: true,
            privacy_fee_netting_blocked: true,
            payout_throttle_active: true,
            reserve_buffer_blocked: true,
            privacy_budget_blocked: true,
            circuit_breaker_open: true,
            roots_only_public_record: true,
            max_records: DEFAULT_MAX_RECORDS,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_root": field_root("chain", &self.chain_id),
            "protocol_root": field_root("protocol", &self.protocol_version),
            "schema_version": self.schema_version,
            "hash_suite_root": field_root("hash_suite", &self.hash_suite),
            "guard_suite_root": field_root("guard_suite", &self.guard_suite),
            "wave": self.wave,
            "wave100_settlement_notary_wave": self.wave100_settlement_notary_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_notary_quorum_bps": self.min_notary_quorum_bps,
            "min_liquidity_coverage_bps": self.min_liquidity_coverage_bps,
            "min_reserve_buffer_bps": self.min_reserve_buffer_bps,
            "min_privacy_budget_remaining_bps": self.min_privacy_budget_remaining_bps,
            "max_fee_netting_drift_bps": self.max_fee_netting_drift_bps,
            "max_payout_throttle_bps": self.max_payout_throttle_bps,
            "fail_closed": self.fail_closed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "production_allowed": self.production_allowed,
            "release_execution_allowed": self.release_execution_allowed,
            "disbursement_allowed": self.disbursement_allowed,
            "disbursement_reservations_enabled": self.disbursement_reservations_enabled,
            "wave100_settlement_notary_active": self.wave100_settlement_notary_active,
            "liquidity_reservation_blocked": self.liquidity_reservation_blocked,
            "privacy_fee_netting_blocked": self.privacy_fee_netting_blocked,
            "payout_throttle_active": self.payout_throttle_active,
            "reserve_buffer_blocked": self.reserve_buffer_blocked,
            "privacy_budget_blocked": self.privacy_budget_blocked,
            "circuit_breaker_open": self.circuit_breaker_open,
            "roots_only_public_record": self.roots_only_public_record,
            "max_records": self.max_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardRoots {
    pub wave100_settlement_notary_root: String,
    pub wave100_pq_reserve_privacy_root: String,
    pub wave100_notary_blocker_root: String,
    pub liquidity_reservation_root: String,
    pub liquidity_commitment_root: String,
    pub liquidity_capacity_root: String,
    pub privacy_fee_netting_root: String,
    pub privacy_fee_liability_root: String,
    pub fee_netting_balance_root: String,
    pub payout_throttle_root: String,
    pub payout_rate_root: String,
    pub payout_window_root: String,
    pub reserve_buffer_root: String,
    pub reserve_buffer_policy_root: String,
    pub reserve_buffer_liability_root: String,
    pub privacy_budget_root: String,
    pub privacy_budget_policy_root: String,
    pub privacy_budget_consumption_root: String,
    pub circuit_breaker_root: String,
    pub circuit_breaker_reason_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub deterministic_guard_root: String,
}

impl GuardRoots {
    pub fn devnet() -> Self {
        Self {
            wave100_settlement_notary_root: sample_root("wave100-settlement-notary-held"),
            wave100_pq_reserve_privacy_root: sample_root("wave100-pq-reserve-privacy-root"),
            wave100_notary_blocker_root: blocker_root(&default_active_blockers()),
            liquidity_reservation_root: sample_root("liquidity-reservation-denied"),
            liquidity_commitment_root: sample_root("liquidity-commitment-held"),
            liquidity_capacity_root: sample_root("liquidity-capacity-throttled"),
            privacy_fee_netting_root: sample_root("privacy-fee-netting-blocked"),
            privacy_fee_liability_root: sample_root("privacy-fee-liability-root"),
            fee_netting_balance_root: sample_root("fee-netting-balance-root"),
            payout_throttle_root: sample_root("payout-throttle-active"),
            payout_rate_root: sample_root("payout-rate-root"),
            payout_window_root: sample_root("payout-window-root"),
            reserve_buffer_root: sample_root("reserve-buffer-blocked"),
            reserve_buffer_policy_root: sample_root("reserve-buffer-policy-root"),
            reserve_buffer_liability_root: sample_root("reserve-buffer-liability-root"),
            privacy_budget_root: sample_root("privacy-budget-blocked"),
            privacy_budget_policy_root: sample_root("privacy-budget-policy-root"),
            privacy_budget_consumption_root: sample_root("privacy-budget-consumption-root"),
            circuit_breaker_root: sample_root("circuit-breaker-open"),
            circuit_breaker_reason_root: sample_root("circuit-breaker-reason-root"),
            operator_signoff_root: sample_root("operator-signoff-missing-root"),
            reviewer_signoff_root: sample_root("reviewer-signoff-missing-root"),
            command_hint_root: command_hint_root(&CommandHint::sequence()),
            deterministic_guard_root: deterministic_root("devnet"),
        }
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("guard_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardMetrics {
    pub notary_quorum_bps: u64,
    pub liquidity_coverage_bps: u64,
    pub reserve_buffer_bps: u64,
    pub privacy_budget_remaining_bps: u64,
    pub fee_netting_drift_bps: u64,
    pub payout_throttle_bps: u64,
    pub wave100_settlement_notary_cleared: bool,
    pub liquidity_reserved: bool,
    pub liquidity_reservation_cleared: bool,
    pub privacy_fee_netting_cleared: bool,
    pub payout_throttle_cleared: bool,
    pub reserve_buffer_cleared: bool,
    pub privacy_budget_cleared: bool,
    pub circuit_breaker_closed: bool,
    pub operator_signed: bool,
    pub reviewer_signed: bool,
    pub root_only: bool,
}

impl Default for GuardMetrics {
    fn default() -> Self {
        Self {
            notary_quorum_bps: 0,
            liquidity_coverage_bps: 0,
            reserve_buffer_bps: 0,
            privacy_budget_remaining_bps: 0,
            fee_netting_drift_bps: DEFAULT_MAX_FEE_NETTING_DRIFT_BPS.saturating_add(1),
            payout_throttle_bps: DEFAULT_MAX_PAYOUT_THROTTLE_BPS.saturating_add(1),
            wave100_settlement_notary_cleared: false,
            liquidity_reserved: false,
            liquidity_reservation_cleared: false,
            privacy_fee_netting_cleared: false,
            payout_throttle_cleared: false,
            reserve_buffer_cleared: false,
            privacy_budget_cleared: false,
            circuit_breaker_closed: false,
            operator_signed: false,
            reviewer_signed: false,
            root_only: true,
        }
    }
}

impl GuardMetrics {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("guard_metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardRecord {
    pub record_root: String,
    pub record_kind: RecordKind,
    pub status: RecordStatus,
    pub roots: GuardRoots,
    pub metrics: GuardMetrics,
    pub blockers: Vec<Blocker>,
}

impl GuardRecord {
    pub fn evaluate(
        config: &Config,
        record_kind: RecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Self {
        let blockers = guard_blockers(config, &roots, &metrics);
        let status = if blockers.is_empty() {
            RecordStatus::ShadowReady
        } else if !metrics.liquidity_reserved || config.payout_throttle_active {
            RecordStatus::LiquidityHeld
        } else if !config.release_execution_allowed || !config.disbursement_allowed {
            RecordStatus::Denied
        } else {
            RecordStatus::Blocked
        };
        let record_root = record_digest(record_kind, status, &roots, &metrics, &blockers);
        Self {
            record_root,
            record_kind,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_root": self.record_root,
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "roots_root": self.roots.state_root(),
            "metrics_root": self.metrics.state_root(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecordState {
    pub record_kind: RecordKind,
    pub status: RecordStatus,
    pub blocker_root: String,
    pub latest_record_root: String,
}

impl RecordState {
    pub fn blocked(record_kind: RecordKind) -> Self {
        Self {
            record_kind,
            status: RecordStatus::Blocked,
            blocker_root: blocker_root(&default_active_blockers()),
            latest_record_root: sample_root(record_kind.as_str()),
        }
    }

    pub fn from_record(record: &GuardRecord) -> Self {
        Self {
            record_kind: record.record_kind,
            status: record.status,
            blocker_root: blocker_root(&record.blockers),
            latest_record_root: record.record_root.clone(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "blocker_root": self.blocker_root,
            "latest_record_root": self.latest_record_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardCounters {
    pub total_records: u64,
    pub denied_count: u64,
    pub blocked_count: u64,
    pub liquidity_held_count: u64,
    pub shadow_ready_count: u64,
    pub active_blocker_count: u64,
    pub reservation_count: u64,
}

impl GuardCounters {
    pub fn from_parts(records: &[GuardRecord], states: &BTreeMap<RecordKind, RecordState>) -> Self {
        let mut counters = Self {
            total_records: records.len() as u64,
            ..Self::default()
        };
        for record in records {
            match record.status {
                RecordStatus::Denied => counters.denied_count += 1,
                RecordStatus::Blocked => counters.blocked_count += 1,
                RecordStatus::LiquidityHeld => counters.liquidity_held_count += 1,
                RecordStatus::ShadowReady => counters.shadow_ready_count += 1,
            }
            counters.active_blocker_count += record.blockers.len() as u64;
            if record.metrics.liquidity_reserved {
                counters.reservation_count += 1;
            }
        }
        counters.active_blocker_count += states
            .values()
            .filter(|state| state.status != RecordStatus::ShadowReady)
            .count() as u64;
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("guard_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub records: Vec<GuardRecord>,
    pub record_states: BTreeMap<RecordKind, RecordState>,
    pub counters: GuardCounters,
    pub command_hints: Vec<CommandHint>,
}

impl State {
    pub fn new(config: Config, records: Vec<GuardRecord>) -> Result<Self> {
        let mut seen = BTreeSet::new();
        for record in &records {
            if record.record_root.is_empty() {
                return Err(Blocker::DeterministicRootMissing.as_str().to_string());
            }
            if !seen.insert(record.record_root.clone()) {
                return Err(Blocker::DuplicateRecordRoot.as_str().to_string());
            }
            if !record.metrics.root_only || !config.roots_only_public_record {
                return Err(Blocker::RootsOnlyBoundary.as_str().to_string());
            }
        }
        let mut record_states = RecordKind::all()
            .into_iter()
            .map(|kind| (kind, RecordState::blocked(kind)))
            .collect::<BTreeMap<_, _>>();
        for record in &records {
            record_states.insert(record.record_kind, RecordState::from_record(record));
        }
        let counters = GuardCounters::from_parts(&records, &record_states);
        Ok(Self {
            config,
            records,
            record_states,
            counters,
            command_hints: CommandHint::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn append_record(
        &self,
        record_kind: RecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Result<Self> {
        let mut records = self.records.clone();
        if records.len() >= self.config.max_records {
            return Err(Blocker::RecordCapacityReached.as_str().to_string());
        }
        let record = GuardRecord::evaluate(&self.config, record_kind, roots, metrics);
        if records
            .iter()
            .any(|item| item.record_root == record.record_root)
        {
            return Err(Blocker::DuplicateRecordRoot.as_str().to_string());
        }
        records.push(record);
        Self::new(self.config.clone(), records)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.counters.reservation_count == 0 {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.denied_count == 0
            && self.counters.shadow_ready_count == RecordKind::all().len() as u64
        {
            RuntimeVerdict::DisbursementShadowReady
        } else {
            RuntimeVerdict::LiquidityThrottleGuardActive
        }
    }

    pub fn public_record(&self) -> Value {
        let states = self
            .record_states
            .values()
            .map(RecordState::public_record)
            .collect::<Vec<_>>();
        let records = self
            .records
            .iter()
            .map(GuardRecord::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "record_state_root": list_root("record_states", states),
            "record_root": list_root("records", records),
            "wave100_settlement_notary_root": wave100_settlement_notary_root(&self.records),
            "liquidity_reservation_root": liquidity_reservation_root(&self.records),
            "privacy_fee_netting_root": privacy_fee_netting_root(&self.records),
            "payout_throttle_root": payout_throttle_root(&self.records),
            "reserve_buffer_root": reserve_buffer_root(&self.records),
            "privacy_budget_root": privacy_budget_root(&self.records),
            "circuit_breaker_root": circuit_breaker_root(&self.records),
            "operator_reviewer_signoff_root": operator_reviewer_signoff_root(&self.records),
            "command_hint_root": command_hint_root(&self.command_hints),
            "deterministic_root": deterministic_state_root(&self.records, &self.record_states),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.records, &self.record_states),
            "active_guard_blocker_root": blocker_root(&default_active_blockers()),
            "verdict": self.verdict().as_str(),
            "release_execution_denied": self.verdict() != RuntimeVerdict::DisbursementShadowReady,
            "disbursement_reservation_count": 0_u64,
            "released_execution_count": 0_u64,
            "raw_payload_count": 0_u64,
            "counters": self.counters.public_record(),
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state"),
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state-core"),
            &[
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&records_root(&self.records)),
                HashPart::Str(&record_states_root(&self.record_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_blockers())),
                HashPart::Str(&command_hint_root(&self.command_hints)),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let roots = GuardRoots::devnet();
    let metrics = GuardMetrics::default();
    let records = RecordKind::all()
        .into_iter()
        .map(|record_kind| {
            GuardRecord::evaluate(&config, record_kind, roots.clone(), metrics.clone())
        })
        .collect::<Vec<_>>();
    match State::new(config.clone(), records) {
        Ok(state) => state,
        Err(reason) => {
            let record_states = RecordKind::all()
                .into_iter()
                .map(|record_kind| (record_kind, RecordState::blocked(record_kind)))
                .collect::<BTreeMap<_, _>>();
            State {
                config,
                records: Vec::new(),
                counters: GuardCounters {
                    active_blocker_count: default_active_blockers().len() as u64,
                    ..GuardCounters::default()
                },
                record_states,
                command_hints: vec![
                    CommandHint::KeepReleaseExecutionDenied,
                    CommandHint::KeepDisbursementReservationsEmpty,
                    CommandHint::PublishRootsOnlyThrottleGuard,
                ],
            }
            .with_closed_reason(reason)
        }
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

impl State {
    fn with_closed_reason(mut self, reason: String) -> Self {
        let reason_root = record_root(
            "closed_state_reason",
            &json!({ "reason_root": field_root("reason", &reason) }),
        );
        self.record_states.insert(
            RecordKind::CommandHint,
            RecordState {
                record_kind: RecordKind::CommandHint,
                status: RecordStatus::Blocked,
                blocker_root: blocker_root(&default_active_blockers()),
                latest_record_root: reason_root,
            },
        );
        self
    }
}

fn guard_blockers(config: &Config, roots: &GuardRoots, metrics: &GuardMetrics) -> Vec<Blocker> {
    let mut blockers = Vec::new();
    if !config.heavy_gates_ran {
        blockers.push(Blocker::HeavyGatesNotRun);
    }
    if !config.production_allowed {
        blockers.push(Blocker::ProductionDenied);
    }
    if !config.release_execution_allowed {
        blockers.push(Blocker::ReleaseExecutionDenied);
    }
    if !config.disbursement_allowed {
        blockers.push(Blocker::DisbursementDenied);
    }
    if !config.disbursement_reservations_enabled || !metrics.liquidity_reserved {
        blockers.push(Blocker::NoDisbursementReservation);
    }
    if config.wave100_settlement_notary_active || !metrics.wave100_settlement_notary_cleared {
        blockers.push(Blocker::Wave100SettlementNotaryActive);
    }
    if roots.wave100_settlement_notary_root.is_empty()
        || roots.wave100_pq_reserve_privacy_root.is_empty()
        || roots.wave100_notary_blocker_root.is_empty()
    {
        blockers.push(Blocker::Wave100SettlementNotaryRootMissing);
    }
    if roots.liquidity_reservation_root.is_empty()
        || roots.liquidity_commitment_root.is_empty()
        || roots.liquidity_capacity_root.is_empty()
    {
        blockers.push(Blocker::LiquidityReservationMissing);
    }
    if config.liquidity_reservation_blocked || !metrics.liquidity_reservation_cleared {
        blockers.push(Blocker::LiquidityReservationBlocked);
    }
    if metrics.liquidity_coverage_bps < config.min_liquidity_coverage_bps {
        blockers.push(Blocker::LiquidityCoverageLow);
    }
    if roots.privacy_fee_netting_root.is_empty()
        || roots.privacy_fee_liability_root.is_empty()
        || roots.fee_netting_balance_root.is_empty()
    {
        blockers.push(Blocker::PrivacyFeeNettingMissing);
    }
    if config.privacy_fee_netting_blocked || !metrics.privacy_fee_netting_cleared {
        blockers.push(Blocker::PrivacyFeeNettingBlocked);
    }
    if metrics.fee_netting_drift_bps > config.max_fee_netting_drift_bps {
        blockers.push(Blocker::FeeNettingDriftHigh);
    }
    if roots.payout_throttle_root.is_empty()
        || roots.payout_rate_root.is_empty()
        || roots.payout_window_root.is_empty()
    {
        blockers.push(Blocker::PayoutThrottleMissing);
    }
    if config.payout_throttle_active || !metrics.payout_throttle_cleared {
        blockers.push(Blocker::PayoutThrottleActive);
    }
    if metrics.payout_throttle_bps > config.max_payout_throttle_bps {
        blockers.push(Blocker::PayoutRateHigh);
    }
    if roots.reserve_buffer_root.is_empty()
        || roots.reserve_buffer_policy_root.is_empty()
        || roots.reserve_buffer_liability_root.is_empty()
    {
        blockers.push(Blocker::ReserveBufferMissing);
    }
    if config.reserve_buffer_blocked
        || !metrics.reserve_buffer_cleared
        || metrics.reserve_buffer_bps < config.min_reserve_buffer_bps
    {
        blockers.push(Blocker::ReserveBufferLow);
    }
    if roots.privacy_budget_root.is_empty()
        || roots.privacy_budget_policy_root.is_empty()
        || roots.privacy_budget_consumption_root.is_empty()
    {
        blockers.push(Blocker::PrivacyBudgetMissing);
    }
    if config.privacy_budget_blocked
        || !metrics.privacy_budget_cleared
        || metrics.privacy_budget_remaining_bps < config.min_privacy_budget_remaining_bps
    {
        blockers.push(Blocker::PrivacyBudgetLow);
    }
    if config.circuit_breaker_open
        || !metrics.circuit_breaker_closed
        || roots.circuit_breaker_root.is_empty()
        || roots.circuit_breaker_reason_root.is_empty()
    {
        blockers.push(Blocker::CircuitBreakerOpen);
    }
    if !metrics.operator_signed || roots.operator_signoff_root.is_empty() {
        blockers.push(Blocker::OperatorSignoffMissing);
    }
    if !metrics.reviewer_signed || roots.reviewer_signoff_root.is_empty() {
        blockers.push(Blocker::ReviewerSignoffMissing);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(Blocker::RootsOnlyBoundary);
    }
    if roots.command_hint_root.is_empty() || roots.deterministic_guard_root.is_empty() {
        blockers.push(Blocker::DeterministicRootMissing);
    }
    blockers
}

fn default_active_blockers() -> Vec<Blocker> {
    vec![
        Blocker::HeavyGatesNotRun,
        Blocker::ReleaseExecutionDenied,
        Blocker::DisbursementDenied,
        Blocker::NoDisbursementReservation,
        Blocker::Wave100SettlementNotaryActive,
        Blocker::LiquidityReservationBlocked,
        Blocker::PrivacyFeeNettingBlocked,
        Blocker::PayoutThrottleActive,
        Blocker::ReserveBufferLow,
        Blocker::PrivacyBudgetLow,
        Blocker::CircuitBreakerOpen,
        Blocker::OperatorSignoffMissing,
        Blocker::ReviewerSignoffMissing,
    ]
}

fn records_root(records: &[GuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| Value::String(record.record_root.clone()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:records"), &leaves)
}

fn record_states_root(states: &BTreeMap<RecordKind, RecordState>) -> String {
    let leaves = states
        .values()
        .map(|state| record_root("record_state", &state.public_record()))
        .map(Value::String)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:record-states"), &leaves)
}

fn wave100_settlement_notary_root(records: &[GuardRecord]) -> String {
    roots_by_kind("wave100-settlement-notary-roots", records, |roots| {
        vec![
            roots.wave100_settlement_notary_root.clone(),
            roots.wave100_pq_reserve_privacy_root.clone(),
            roots.wave100_notary_blocker_root.clone(),
        ]
    })
}

fn liquidity_reservation_root(records: &[GuardRecord]) -> String {
    roots_by_kind("liquidity-reservation-roots", records, |roots| {
        vec![
            roots.liquidity_reservation_root.clone(),
            roots.liquidity_commitment_root.clone(),
            roots.liquidity_capacity_root.clone(),
        ]
    })
}

fn privacy_fee_netting_root(records: &[GuardRecord]) -> String {
    roots_by_kind("privacy-fee-netting-roots", records, |roots| {
        vec![
            roots.privacy_fee_netting_root.clone(),
            roots.privacy_fee_liability_root.clone(),
            roots.fee_netting_balance_root.clone(),
        ]
    })
}

fn payout_throttle_root(records: &[GuardRecord]) -> String {
    roots_by_kind("payout-throttle-roots", records, |roots| {
        vec![
            roots.payout_throttle_root.clone(),
            roots.payout_rate_root.clone(),
            roots.payout_window_root.clone(),
        ]
    })
}

fn reserve_buffer_root(records: &[GuardRecord]) -> String {
    roots_by_kind("reserve-buffer-roots", records, |roots| {
        vec![
            roots.reserve_buffer_root.clone(),
            roots.reserve_buffer_policy_root.clone(),
            roots.reserve_buffer_liability_root.clone(),
        ]
    })
}

fn privacy_budget_root(records: &[GuardRecord]) -> String {
    roots_by_kind("privacy-budget-roots", records, |roots| {
        vec![
            roots.privacy_budget_root.clone(),
            roots.privacy_budget_policy_root.clone(),
            roots.privacy_budget_consumption_root.clone(),
        ]
    })
}

fn circuit_breaker_root(records: &[GuardRecord]) -> String {
    roots_by_kind("circuit-breaker-roots", records, |roots| {
        vec![
            roots.circuit_breaker_root.clone(),
            roots.circuit_breaker_reason_root.clone(),
        ]
    })
}

fn operator_reviewer_signoff_root(records: &[GuardRecord]) -> String {
    roots_by_kind("operator-reviewer-signoff-roots", records, |roots| {
        vec![
            roots.operator_signoff_root.clone(),
            roots.reviewer_signoff_root.clone(),
        ]
    })
}

fn roots_by_kind<F>(kind: &str, records: &[GuardRecord], collect: F) -> String
where
    F: Fn(&GuardRoots) -> Vec<String>,
{
    let leaves = records
        .iter()
        .flat_map(|record| collect(&record.roots))
        .filter(|root| !root.is_empty())
        .map(|root| {
            Value::String(domain_hash(
                &format!("{DOMAIN}:{kind}:leaf"),
                &[HashPart::Str(&root)],
                32,
            ))
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn list_root(kind: &str, values: Vec<Value>) -> String {
    let leaves = values
        .iter()
        .map(|value| Value::String(record_root(kind, value)))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn all_blockers_root(
    records: &[GuardRecord],
    states: &BTreeMap<RecordKind, RecordState>,
) -> String {
    let mut leaves = records
        .iter()
        .flat_map(|record| record.blockers.iter().map(|blocker| blocker.as_str()))
        .map(|blocker| Value::String(blocker.to_string()))
        .collect::<Vec<_>>();
    leaves.extend(
        states
            .values()
            .filter(|state| state.status != RecordStatus::ShadowReady)
            .map(|state| Value::String(state.blocker_root.clone())),
    );
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn blocker_root(blockers: &[Blocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| Value::String(blocker.as_str().to_string()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn command_hint_root(commands: &[CommandHint]) -> String {
    let leaves = commands
        .iter()
        .map(|command| {
            Value::String(domain_hash(
                &format!("{DOMAIN}:command-hint"),
                &[
                    HashPart::Str(CHAIN_ID),
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(command.as_str()),
                    HashPart::U64(DEFAULT_WAVE),
                ],
                32,
            ))
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:command-hints"), &leaves)
}

fn deterministic_state_root(
    records: &[GuardRecord],
    states: &BTreeMap<RecordKind, RecordState>,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:deterministic-state-root"),
        &[
            HashPart::Str(&records_root(records)),
            HashPart::Str(&record_states_root(states)),
            HashPart::Str(&blocker_root(&default_active_blockers())),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:deterministic-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::U64(DEFAULT_WAVE),
        ],
        32,
    )
}

fn sample_root(label: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:sample-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn field_root(field: &str, value: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:field-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(field),
            HashPart::Str(value),
        ],
        32,
    )
}

fn record_digest(
    record_kind: RecordKind,
    status: RecordStatus,
    roots: &GuardRoots,
    metrics: &GuardMetrics,
    blockers: &[Blocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:record-digest"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(record_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}
