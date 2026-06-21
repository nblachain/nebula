use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave101-release-execution-disbursement-liquidity-throttle-guard-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 101;
pub const DEFAULT_SETTLEMENT_NOTARY_WAVE: u64 = 100;
pub const DEFAULT_UNLOCK_WAVE: u64 = 99;
pub const DEFAULT_MIN_SETTLEMENT_NOTARY_ROOTS: u64 = 7;
pub const DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS: u64 = 7;
pub const DEFAULT_MIN_FEE_NETTING_ROOTS: u64 = 5;
pub const DEFAULT_MIN_THROTTLE_ROOTS: u64 = 5;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    RuntimeReplay,
    SettlementNotaryCarry,
    LiquidityReservation,
    ReplayFeeNetting,
    PayoutThrottle,
    PrivacyBudget,
    RollbackSentinel,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RuntimeReplay,
            Self::SettlementNotaryCarry,
            Self::LiquidityReservation,
            Self::ReplayFeeNetting,
            Self::PayoutThrottle,
            Self::PrivacyBudget,
            Self::RollbackSentinel,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime_replay",
            Self::SettlementNotaryCarry => "settlement_notary_carry",
            Self::LiquidityReservation => "liquidity_reservation",
            Self::ReplayFeeNetting => "replay_fee_netting",
            Self::PayoutThrottle => "payout_throttle",
            Self::PrivacyBudget => "privacy_budget",
            Self::RollbackSentinel => "rollback_sentinel",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementStatus {
    Blocked,
    LiquidityHeld,
    Throttled,
    ReleaseExecutionDenied,
}

impl DisbursementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::LiquidityHeld => "liquidity_held",
            Self::Throttled => "throttled",
            Self::ReleaseExecutionDenied => "release_execution_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThrottleBlocker {
    FailClosedArmed,
    ReleaseExecutionDenied,
    HeavyGateReceiptAbsent,
    Wave100SettlementNotaryRootActive,
    LiquidityReservationRootMissing,
    ReplayFeeNettingRootMissing,
    PayoutThrottleRootActive,
    PrivacyBudgetRootActive,
    RollbackSentinelRootActive,
    CircuitBreakerRootActive,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    DisbursementReservationAbsent,
}

impl ThrottleBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ReleaseExecutionDenied,
            Self::HeavyGateReceiptAbsent,
            Self::Wave100SettlementNotaryRootActive,
            Self::LiquidityReservationRootMissing,
            Self::ReplayFeeNettingRootMissing,
            Self::PayoutThrottleRootActive,
            Self::PrivacyBudgetRootActive,
            Self::RollbackSentinelRootActive,
            Self::CircuitBreakerRootActive,
            Self::OperatorSignoffRootMissing,
            Self::ReviewerSignoffRootMissing,
            Self::DisbursementReservationAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::Wave100SettlementNotaryRootActive => "wave100_settlement_notary_root_active",
            Self::LiquidityReservationRootMissing => "liquidity_reservation_root_missing",
            Self::ReplayFeeNettingRootMissing => "replay_fee_netting_root_missing",
            Self::PayoutThrottleRootActive => "payout_throttle_root_active",
            Self::PrivacyBudgetRootActive => "privacy_budget_root_active",
            Self::RollbackSentinelRootActive => "rollback_sentinel_root_active",
            Self::CircuitBreakerRootActive => "circuit_breaker_root_active",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::DisbursementReservationAbsent => "disbursement_reservation_absent",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    CarryWave100SettlementNotaryRoots,
    DenyReleaseExecutionUntilLiquidityClears,
    BindLiquidityReservationRoots,
    BindReplayFeeNettingRoots,
    BindPayoutThrottleRoots,
    BindPrivacyBudgetRoots,
    BindRollbackCircuitBreakerRoots,
    RequireOperatorSignoffRoot,
    RequireReviewerSignoffRoot,
    KeepHeavyGateReceiptAbsent,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CarryWave100SettlementNotaryRoots,
            Self::DenyReleaseExecutionUntilLiquidityClears,
            Self::BindLiquidityReservationRoots,
            Self::BindReplayFeeNettingRoots,
            Self::BindPayoutThrottleRoots,
            Self::BindPrivacyBudgetRoots,
            Self::BindRollbackCircuitBreakerRoots,
            Self::RequireOperatorSignoffRoot,
            Self::RequireReviewerSignoffRoot,
            Self::KeepHeavyGateReceiptAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CarryWave100SettlementNotaryRoots => "carry_wave100_settlement_notary_roots",
            Self::DenyReleaseExecutionUntilLiquidityClears => {
                "deny_release_execution_until_liquidity_clears"
            }
            Self::BindLiquidityReservationRoots => "bind_liquidity_reservation_roots",
            Self::BindReplayFeeNettingRoots => "bind_replay_fee_netting_roots",
            Self::BindPayoutThrottleRoots => "bind_payout_throttle_roots",
            Self::BindPrivacyBudgetRoots => "bind_privacy_budget_roots",
            Self::BindRollbackCircuitBreakerRoots => "bind_rollback_circuit_breaker_roots",
            Self::RequireOperatorSignoffRoot => "require_operator_signoff_root",
            Self::RequireReviewerSignoffRoot => "require_reviewer_signoff_root",
            Self::KeepHeavyGateReceiptAbsent => "keep_heavy_gate_receipt_absent",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub lane_suite: String,
    pub wave: u64,
    pub settlement_notary_wave: u64,
    pub unlock_wave: u64,
    pub min_settlement_notary_roots: u64,
    pub min_liquidity_reservation_roots: u64,
    pub min_fee_netting_roots: u64,
    pub min_throttle_roots: u64,
    pub wave100_runtime_replay_settlement_notary_root: String,
    pub wave100_settlement_notary_ledger_root: String,
    pub wave100_command_hints_root: String,
    pub fail_closed_armed: bool,
    pub release_execution_denied: bool,
    pub liquidity_blockers_active: bool,
    pub throttle_blockers_active: bool,
    pub disbursement_reservations_present: bool,
    pub heavy_gates_ran: bool,
    pub max_public_raw_records: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            lane_suite: LANE_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            settlement_notary_wave: DEFAULT_SETTLEMENT_NOTARY_WAVE,
            unlock_wave: DEFAULT_UNLOCK_WAVE,
            min_settlement_notary_roots: DEFAULT_MIN_SETTLEMENT_NOTARY_ROOTS,
            min_liquidity_reservation_roots: DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS,
            min_fee_netting_roots: DEFAULT_MIN_FEE_NETTING_ROOTS,
            min_throttle_roots: DEFAULT_MIN_THROTTLE_ROOTS,
            wave100_runtime_replay_settlement_notary_root: stable_root(
                "wave100-runtime-replay-settlement-notary",
                "runtime-replay",
            ),
            wave100_settlement_notary_ledger_root: stable_root(
                "wave100-settlement-notary-ledger",
                "all",
            ),
            wave100_command_hints_root: stable_root("wave100-command-hints", "runtime-replay"),
            fail_closed_armed: true,
            release_execution_denied: true,
            liquidity_blockers_active: true,
            throttle_blockers_active: true,
            disbursement_reservations_present: false,
            heavy_gates_ran: false,
            max_public_raw_records: DEFAULT_MAX_PUBLIC_RAW_RECORDS,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("lane_suite", &self.lane_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("settlement_notary_wave", self.settlement_notary_wave)?;
        ensure_positive("unlock_wave", self.unlock_wave)?;
        ensure_positive(
            "min_settlement_notary_roots",
            self.min_settlement_notary_roots,
        )?;
        ensure_positive(
            "min_liquidity_reservation_roots",
            self.min_liquidity_reservation_roots,
        )?;
        ensure_positive("min_fee_netting_roots", self.min_fee_netting_roots)?;
        ensure_positive("min_throttle_roots", self.min_throttle_roots)?;
        ensure_root(
            "wave100_runtime_replay_settlement_notary_root",
            &self.wave100_runtime_replay_settlement_notary_root,
        )?;
        ensure_root(
            "wave100_settlement_notary_ledger_root",
            &self.wave100_settlement_notary_ledger_root,
        )?;
        ensure_root(
            "wave100_command_hints_root",
            &self.wave100_command_hints_root,
        )?;
        if !(self.unlock_wave < self.settlement_notary_wave
            && self.settlement_notary_wave < self.wave)
        {
            return Err(
                "wave ordering must be unlock, settlement notary, disbursement".to_string(),
            );
        }
        if !self.fail_closed_armed {
            return Err("liquidity throttle guard is disarmed".to_string());
        }
        if !self.release_execution_denied {
            return Err("devnet liquidity throttle guard must deny release execution".to_string());
        }
        if !self.liquidity_blockers_active {
            return Err("devnet liquidity blockers must remain active".to_string());
        }
        if !self.throttle_blockers_active {
            return Err("devnet throttle blockers must remain active".to_string());
        }
        if self.disbursement_reservations_present {
            return Err("devnet must not publish disbursement reservations".to_string());
        }
        if self.heavy_gates_ran {
            return Err("runtime replay lane cannot claim live heavy gate execution".to_string());
        }
        if self.max_public_raw_records != 0 {
            return Err("public records must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave101_disbursement_liquidity_throttle_guard_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "settlement_notary_wave": self.settlement_notary_wave,
            "unlock_wave": self.unlock_wave,
            "min_settlement_notary_roots": self.min_settlement_notary_roots,
            "min_liquidity_reservation_roots": self.min_liquidity_reservation_roots,
            "min_fee_netting_roots": self.min_fee_netting_roots,
            "min_throttle_roots": self.min_throttle_roots,
            "wave100_runtime_replay_settlement_notary_root": self.wave100_runtime_replay_settlement_notary_root,
            "wave100_settlement_notary_ledger_root": self.wave100_settlement_notary_ledger_root,
            "wave100_command_hints_root": self.wave100_command_hints_root,
            "fail_closed_armed": self.fail_closed_armed,
            "release_execution_denied": self.release_execution_denied,
            "liquidity_blockers_active": self.liquidity_blockers_active,
            "throttle_blockers_active": self.throttle_blockers_active,
            "disbursement_reservations_present": self.disbursement_reservations_present,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ThrottleGuardEntry {
    pub lane: ReplayLane,
    pub wave100_settlement_notary_root: String,
    pub liquidity_reservation_root: String,
    pub replay_fee_netting_root: String,
    pub payout_throttle_root: String,
    pub privacy_budget_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub blocker_roots: Vec<String>,
    pub status: DisbursementStatus,
    pub release_execution_allowed: bool,
    pub disbursement_reserved: bool,
}

impl ThrottleGuardEntry {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            wave100_settlement_notary_root: lane_wave100_settlement_notary_root(lane, config),
            liquidity_reservation_root: lane_root("liquidity-reservation", lane),
            replay_fee_netting_root: lane_root("replay-fee-netting", lane),
            payout_throttle_root: lane_root("payout-throttle", lane),
            privacy_budget_root: lane_root("privacy-budget", lane),
            rollback_sentinel_root: lane_root("rollback-sentinel", lane),
            circuit_breaker_root: lane_root("circuit-breaker", lane),
            operator_signoff_root: lane_root("operator-signoff", lane),
            reviewer_signoff_root: lane_root("reviewer-signoff", lane),
            command_hint_root: lane_root("command-hint", lane),
            blocker_roots: ThrottleBlocker::all()
                .iter()
                .map(|blocker| blocker_root(lane, *blocker))
                .collect(),
            status: DisbursementStatus::ReleaseExecutionDenied,
            release_execution_allowed: false,
            disbursement_reserved: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root(
            "wave100_settlement_notary_root",
            &self.wave100_settlement_notary_root,
        )?;
        ensure_root(
            "liquidity_reservation_root",
            &self.liquidity_reservation_root,
        )?;
        ensure_root("replay_fee_netting_root", &self.replay_fee_netting_root)?;
        ensure_root("payout_throttle_root", &self.payout_throttle_root)?;
        ensure_root("privacy_budget_root", &self.privacy_budget_root)?;
        ensure_root("rollback_sentinel_root", &self.rollback_sentinel_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.blocker_roots.is_empty() {
            return Err("throttle guard entry requires blocker roots".to_string());
        }
        for root in self.blocker_roots.iter() {
            ensure_root("blocker_root", root)?;
        }
        if self.release_execution_allowed {
            return Err("devnet throttle guard cannot allow release execution".to_string());
        }
        if self.disbursement_reserved {
            return Err("devnet throttle guard cannot reserve disbursement liquidity".to_string());
        }
        Ok(())
    }

    pub fn blocker_root(&self) -> String {
        list_root("WAVE101-THROTTLE-BLOCKER-ROOTS", self.blocker_roots.clone())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave101_throttle_guard_entry",
            "lane": self.lane.as_str(),
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "replay_fee_netting_root": self.replay_fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "privacy_budget_root": self.privacy_budget_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "command_hint_root": self.command_hint_root,
            "blocker_roots_root": self.blocker_root(),
            "blocker_count": self.blocker_roots.len(),
            "status": self.status.as_str(),
            "release_execution_allowed": self.release_execution_allowed,
            "disbursement_reserved": self.disbursement_reserved,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-THROTTLE-GUARD-ENTRY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootLedger {
    pub wave100_settlement_notary_roots: BTreeMap<String, String>,
    pub liquidity_reservation_roots: BTreeMap<String, String>,
    pub replay_fee_netting_roots: BTreeMap<String, String>,
    pub payout_throttle_roots: BTreeMap<String, String>,
    pub privacy_budget_roots: BTreeMap<String, String>,
    pub rollback_sentinel_roots: BTreeMap<String, String>,
    pub circuit_breaker_roots: BTreeMap<String, String>,
    pub operator_signoff_roots: BTreeMap<String, String>,
    pub reviewer_signoff_roots: BTreeMap<String, String>,
    pub command_hints: BTreeMap<String, String>,
}

impl RootLedger {
    pub fn devnet(lanes: &[ReplayLane], config: &Config) -> Self {
        Self {
            wave100_settlement_notary_roots: lanes
                .iter()
                .map(|lane| {
                    (
                        lane.as_str().to_string(),
                        lane_wave100_settlement_notary_root(*lane, config),
                    )
                })
                .collect(),
            liquidity_reservation_roots: lane_map(lanes, "liquidity-reservation"),
            replay_fee_netting_roots: lane_map(lanes, "replay-fee-netting"),
            payout_throttle_roots: lane_map(lanes, "payout-throttle"),
            privacy_budget_roots: lane_map(lanes, "privacy-budget"),
            rollback_sentinel_roots: lane_map(lanes, "rollback-sentinel"),
            circuit_breaker_roots: lane_map(lanes, "circuit-breaker"),
            operator_signoff_roots: lane_map(lanes, "operator-signoff"),
            reviewer_signoff_roots: lane_map(lanes, "reviewer-signoff"),
            command_hints: CommandHint::all()
                .iter()
                .map(|hint| (hint.as_str().to_string(), command_hint_kind_root(*hint)))
                .collect(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_map_roots(
            "wave100_settlement_notary_roots",
            &self.wave100_settlement_notary_roots,
        )?;
        ensure_map_roots(
            "liquidity_reservation_roots",
            &self.liquidity_reservation_roots,
        )?;
        ensure_map_roots("replay_fee_netting_roots", &self.replay_fee_netting_roots)?;
        ensure_map_roots("payout_throttle_roots", &self.payout_throttle_roots)?;
        ensure_map_roots("privacy_budget_roots", &self.privacy_budget_roots)?;
        ensure_map_roots("rollback_sentinel_roots", &self.rollback_sentinel_roots)?;
        ensure_map_roots("circuit_breaker_roots", &self.circuit_breaker_roots)?;
        ensure_map_roots("operator_signoff_roots", &self.operator_signoff_roots)?;
        ensure_map_roots("reviewer_signoff_roots", &self.reviewer_signoff_roots)?;
        ensure_map_roots("command_hints", &self.command_hints)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave101_root_ledger",
            "wave100_settlement_notary_root": map_root("WAVE101-WAVE100-SETTLEMENT-NOTARY-MAP", &self.wave100_settlement_notary_roots),
            "liquidity_reservation_root": map_root("WAVE101-LIQUIDITY-RESERVATION-MAP", &self.liquidity_reservation_roots),
            "replay_fee_netting_root": map_root("WAVE101-REPLAY-FEE-NETTING-MAP", &self.replay_fee_netting_roots),
            "payout_throttle_root": map_root("WAVE101-PAYOUT-THROTTLE-MAP", &self.payout_throttle_roots),
            "privacy_budget_root": map_root("WAVE101-PRIVACY-BUDGET-MAP", &self.privacy_budget_roots),
            "rollback_sentinel_root": map_root("WAVE101-ROLLBACK-SENTINEL-MAP", &self.rollback_sentinel_roots),
            "circuit_breaker_root": map_root("WAVE101-CIRCUIT-BREAKER-MAP", &self.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE101-OPERATOR-SIGNOFF-MAP", &self.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE101-REVIEWER-SIGNOFF-MAP", &self.reviewer_signoff_roots),
            "command_hint_root": map_root("WAVE101-COMMAND-HINT-MAP", &self.command_hints),
            "wave100_settlement_notary_count": self.wave100_settlement_notary_roots.len(),
            "liquidity_reservation_count": self.liquidity_reservation_roots.len(),
            "replay_fee_netting_count": self.replay_fee_netting_roots.len(),
            "payout_throttle_count": self.payout_throttle_roots.len(),
            "privacy_budget_count": self.privacy_budget_roots.len(),
            "rollback_sentinel_count": self.rollback_sentinel_roots.len(),
            "circuit_breaker_count": self.circuit_breaker_roots.len(),
            "operator_signoff_count": self.operator_signoff_roots.len(),
            "reviewer_signoff_count": self.reviewer_signoff_roots.len(),
            "command_hint_count": self.command_hints.len(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-ROOT-LEDGER", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_count: u64,
    pub throttle_guard_count: u64,
    pub blocked_throttle_guards: u64,
    pub release_execution_allowed_count: u64,
    pub disbursement_reservation_count: u64,
    pub wave100_settlement_notary_roots: u64,
    pub liquidity_reservation_roots: u64,
    pub replay_fee_netting_roots: u64,
    pub payout_throttle_roots: u64,
    pub privacy_budget_roots: u64,
    pub rollback_sentinel_roots: u64,
    pub circuit_breaker_roots: u64,
    pub operator_signoff_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub raw_public_records: u64,
}

impl Counters {
    pub fn from_parts(entries: &[ThrottleGuardEntry], ledger: &RootLedger) -> Self {
        Self {
            lane_count: ReplayLane::all().len() as u64,
            throttle_guard_count: entries.len() as u64,
            blocked_throttle_guards: entries
                .iter()
                .filter(|entry| !entry.release_execution_allowed)
                .count() as u64,
            release_execution_allowed_count: entries
                .iter()
                .filter(|entry| entry.release_execution_allowed)
                .count() as u64,
            disbursement_reservation_count: entries
                .iter()
                .filter(|entry| entry.disbursement_reserved)
                .count() as u64,
            wave100_settlement_notary_roots: ledger.wave100_settlement_notary_roots.len() as u64,
            liquidity_reservation_roots: ledger.liquidity_reservation_roots.len() as u64,
            replay_fee_netting_roots: ledger.replay_fee_netting_roots.len() as u64,
            payout_throttle_roots: ledger.payout_throttle_roots.len() as u64,
            privacy_budget_roots: ledger.privacy_budget_roots.len() as u64,
            rollback_sentinel_roots: ledger.rollback_sentinel_roots.len() as u64,
            circuit_breaker_roots: ledger.circuit_breaker_roots.len() as u64,
            operator_signoff_roots: ledger.operator_signoff_roots.len() as u64,
            reviewer_signoff_roots: ledger.reviewer_signoff_roots.len() as u64,
            raw_public_records: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave101_disbursement_liquidity_throttle_guard_counters",
            "lane_count": self.lane_count,
            "throttle_guard_count": self.throttle_guard_count,
            "blocked_throttle_guards": self.blocked_throttle_guards,
            "release_execution_allowed_count": self.release_execution_allowed_count,
            "disbursement_reservation_count": self.disbursement_reservation_count,
            "wave100_settlement_notary_roots": self.wave100_settlement_notary_roots,
            "liquidity_reservation_roots": self.liquidity_reservation_roots,
            "replay_fee_netting_roots": self.replay_fee_netting_roots,
            "payout_throttle_roots": self.payout_throttle_roots,
            "privacy_budget_roots": self.privacy_budget_roots,
            "rollback_sentinel_roots": self.rollback_sentinel_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "raw_public_records": self.raw_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub throttle_guards: Vec<ThrottleGuardEntry>,
    pub root_ledger: RootLedger,
    pub blocker_catalog: BTreeMap<String, String>,
    pub wave100_settlement_notary_catalog: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        throttle_guards: Vec<ThrottleGuardEntry>,
        root_ledger: RootLedger,
        blocker_catalog: BTreeMap<String, String>,
        wave100_settlement_notary_catalog: BTreeMap<String, String>,
    ) -> Result<Self> {
        let counters = Counters::from_parts(&throttle_guards, &root_ledger);
        let state = Self {
            config,
            throttle_guards,
            root_ledger,
            blocker_catalog,
            wave100_settlement_notary_catalog,
            counters,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_unique_lanes(
            "throttle_guards",
            self.throttle_guards
                .iter()
                .map(|entry| entry.lane)
                .collect(),
        )?;
        for entry in self.throttle_guards.iter() {
            entry.validate()?;
        }
        self.root_ledger.validate()?;
        ensure_map_roots("blocker_catalog", &self.blocker_catalog)?;
        ensure_map_roots(
            "wave100_settlement_notary_catalog",
            &self.wave100_settlement_notary_catalog,
        )?;
        if self.counters.release_execution_allowed_count != 0 {
            return Err("devnet must have zero allowed release executions".to_string());
        }
        if self.counters.disbursement_reservation_count != 0 {
            return Err("devnet must have zero disbursement reservations".to_string());
        }
        if self.counters.blocked_throttle_guards != self.counters.throttle_guard_count {
            return Err("all devnet throttle guards must remain blocked".to_string());
        }
        if self.counters.raw_public_records != 0 {
            return Err("public record counter must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave101_release_execution_disbursement_liquidity_throttle_guard_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "wave100_runtime_replay_settlement_notary_root": self.config.wave100_runtime_replay_settlement_notary_root,
            "wave100_settlement_notary_ledger_root": self.config.wave100_settlement_notary_ledger_root,
            "wave100_command_hints_root": self.config.wave100_command_hints_root,
            "throttle_guards_root": throttle_guards_root(&self.throttle_guards),
            "root_ledger_root": self.root_ledger.state_root(),
            "blocker_catalog_root": map_root("WAVE101-BLOCKER-CATALOG", &self.blocker_catalog),
            "wave100_settlement_notary_catalog_root": map_root("WAVE101-WAVE100-SETTLEMENT-NOTARY-CATALOG", &self.wave100_settlement_notary_catalog),
            "wave100_settlement_notary_root": map_root("WAVE101-WAVE100-SETTLEMENT-NOTARY-ROOTS", &self.root_ledger.wave100_settlement_notary_roots),
            "liquidity_reservation_root": map_root("WAVE101-LIQUIDITY-RESERVATION-ROOTS", &self.root_ledger.liquidity_reservation_roots),
            "replay_fee_netting_root": map_root("WAVE101-REPLAY-FEE-NETTING-ROOTS", &self.root_ledger.replay_fee_netting_roots),
            "payout_throttle_root": map_root("WAVE101-PAYOUT-THROTTLE-ROOTS", &self.root_ledger.payout_throttle_roots),
            "privacy_budget_root": map_root("WAVE101-PRIVACY-BUDGET-ROOTS", &self.root_ledger.privacy_budget_roots),
            "rollback_sentinel_root": map_root("WAVE101-ROLLBACK-SENTINEL-ROOTS", &self.root_ledger.rollback_sentinel_roots),
            "circuit_breaker_root": map_root("WAVE101-CIRCUIT-BREAKER-ROOTS", &self.root_ledger.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE101-OPERATOR-SIGNOFF-ROOTS", &self.root_ledger.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE101-REVIEWER-SIGNOFF-ROOTS", &self.root_ledger.reviewer_signoff_roots),
            "command_hints_root": map_root("WAVE101-COMMAND-HINTS", &self.root_ledger.command_hints),
            "counters_root": self.counters.state_root(),
            "release_execution_denied": self.config.release_execution_denied,
            "liquidity_blockers_active": self.config.liquidity_blockers_active,
            "throttle_blockers_active": self.config.throttle_blockers_active,
            "disbursement_reservations_present": self.config.disbursement_reservations_present,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "raw_public_records": self.counters.raw_public_records,
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Value::Object(fields) = &mut record {
            fields.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-STATE", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let lanes = ReplayLane::all();
    let throttle_guards = lanes
        .iter()
        .map(|lane| ThrottleGuardEntry::blocked(*lane, &config))
        .collect::<Vec<_>>();
    let root_ledger = RootLedger::devnet(&lanes, &config);
    let blocker_catalog = ThrottleBlocker::all()
        .iter()
        .map(|blocker| (blocker.as_str().to_string(), blocker_kind_root(*blocker)))
        .collect::<BTreeMap<_, _>>();
    let wave100_settlement_notary_catalog = lanes
        .iter()
        .map(|lane| {
            (
                lane.as_str().to_string(),
                lane_wave100_settlement_notary_root(*lane, &config),
            )
        })
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        throttle_guards,
        root_ledger,
        blocker_catalog,
        wave100_settlement_notary_catalog,
    ) {
        Ok(state) => state,
        Err(_) => {
            let fallback_config = Config::devnet();
            State {
                config: fallback_config.clone(),
                throttle_guards: Vec::new(),
                root_ledger: RootLedger::devnet(&[], &fallback_config),
                blocker_catalog: BTreeMap::new(),
                wave100_settlement_notary_catalog: BTreeMap::new(),
                counters: Counters::default(),
            }
        }
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn throttle_guards_root(entries: &[ThrottleGuardEntry]) -> String {
    list_root(
        "WAVE101-THROTTLE-GUARD-STATE-ROOTS",
        entries.iter().map(ThrottleGuardEntry::state_root).collect(),
    )
}

fn lane_map(lanes: &[ReplayLane], kind: &str) -> BTreeMap<String, String> {
    lanes
        .iter()
        .map(|lane| (lane.as_str().to_string(), lane_root(kind, *lane)))
        .collect()
}

fn map_root(domain: &str, roots: &BTreeMap<String, String>) -> String {
    let entries = roots
        .iter()
        .map(|(label, root)| {
            domain_hash(
                domain,
                &[
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(label),
                    HashPart::Str(root),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    list_root(domain, entries)
}

fn lane_wave100_settlement_notary_root(lane: ReplayLane, config: &Config) -> String {
    domain_hash(
        "WAVE101-LANE-WAVE100-SETTLEMENT-NOTARY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&config.wave100_runtime_replay_settlement_notary_root),
            HashPart::Str(&config.wave100_settlement_notary_ledger_root),
        ],
        32,
    )
}

fn lane_root(kind: &str, lane: ReplayLane) -> String {
    domain_hash(
        "WAVE101-LANE-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn command_hint_kind_root(hint: CommandHint) -> String {
    domain_hash(
        "WAVE101-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: ThrottleBlocker) -> String {
    domain_hash(
        "WAVE101-THROTTLE-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn blocker_kind_root(blocker: ThrottleBlocker) -> String {
    domain_hash(
        "WAVE101-THROTTLE-BLOCKER-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn stable_root(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE101-STABLE-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(domain, &[HashPart::Json(value)], 32)
}

fn list_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots
        .iter()
        .map(|root| Value::String(domain_hash(domain, &[HashPart::Str(root)], 32)))
        .collect::<Vec<_>>();
    merkle_root(domain, leaves.as_slice())
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.is_empty() {
        Err(format!("{} must be non-empty", field))
    } else {
        Ok(())
    }
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        Err(format!("{} must be positive", field))
    } else {
        Ok(())
    }
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 {
        return Err(format!("{} must be a deterministic root", field));
    }
    Ok(())
}

fn ensure_map_roots(field: &str, roots: &BTreeMap<String, String>) -> Result<()> {
    if roots.is_empty() {
        return Err(format!("{} must not be empty", field));
    }
    for (label, root) in roots.iter() {
        ensure_non_empty(field, label)?;
        ensure_root(field, root)?;
    }
    Ok(())
}

fn ensure_unique_lanes(field: &str, lanes: Vec<ReplayLane>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for lane in lanes.iter() {
        if !seen.insert(*lane) {
            return Err(format!("{} contains duplicate lane", field));
        }
    }
    Ok(())
}
