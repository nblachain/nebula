use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave101";
pub const PRIOR_WAVE_ID: &str = "wave100";
pub const LANE_ID: &str =
    "wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-compile-lane";
pub const PRIOR_LANE_ID: &str =
    "wave100-live-heavy-gate-release-execution-settlement-notary-guard-compile-lane";
pub const DEFAULT_MIN_WAVE100_SETTLEMENT_NOTARY_ROOTS: usize = 7;
pub const DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS: usize = 7;
pub const DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS: usize = 7;
pub const DEFAULT_MIN_PRIVACY_BUDGET_ROOTS: usize = 7;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompileLaneGate {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl CompileLaneGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::Clippy => "clippy",
            Self::Rustfmt => "rustfmt",
            Self::Rustc => "rustc",
            Self::BuildMetadata => "build_metadata",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> [Self; 7] {
        [
            Self::CargoCheck,
            Self::CargoTest,
            Self::Clippy,
            Self::Rustfmt,
            Self::Rustc,
            Self::BuildMetadata,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementVerdict {
    FailClosed,
    ReleaseExecutionDenied,
    SettlementNotaryBlocked,
    LiquidityBlocked,
    PayoutThrottled,
    DisbursementReady,
}

impl DisbursementVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::SettlementNotaryBlocked => "settlement_notary_blocked",
            Self::LiquidityBlocked => "liquidity_blocked",
            Self::PayoutThrottled => "payout_throttled",
            Self::DisbursementReady => "disbursement_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementBlockerKind {
    DefaultFailClosed,
    HeavyGateNotRun,
    ReleaseExecutionDenied,
    Wave100SettlementNotaryActive,
    SettlementNotaryMissing,
    LiquidityReservationMissing,
    LiquidityReservationBlocked,
    FeeNettingUnreconciled,
    PayoutThrottleActive,
    PrivacyBudgetLocked,
    CircuitBreakerActive,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    DisbursementReservationMissing,
    RootsOnlyBoundary,
}

impl DisbursementBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::Wave100SettlementNotaryActive => "wave100_settlement_notary_active",
            Self::SettlementNotaryMissing => "settlement_notary_missing",
            Self::LiquidityReservationMissing => "liquidity_reservation_missing",
            Self::LiquidityReservationBlocked => "liquidity_reservation_blocked",
            Self::FeeNettingUnreconciled => "fee_netting_unreconciled",
            Self::PayoutThrottleActive => "payout_throttle_active",
            Self::PrivacyBudgetLocked => "privacy_budget_locked",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::DisbursementReservationMissing => "disbursement_reservation_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_id: String,
    pub prior_wave_id: String,
    pub lane_id: String,
    pub prior_lane_id: String,
    pub min_wave100_settlement_notary_roots: usize,
    pub min_liquidity_reservation_roots: usize,
    pub min_payout_throttle_roots: usize,
    pub min_privacy_budget_roots: usize,
    pub roots_only_public_record: bool,
    pub release_execution_allowed: bool,
    pub disbursement_reservations: usize,
    pub liquidity_blockers_active: bool,
    pub throttle_blockers_active: bool,
    pub heavy_gates_ran: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            prior_lane_id: PRIOR_LANE_ID.to_string(),
            min_wave100_settlement_notary_roots: DEFAULT_MIN_WAVE100_SETTLEMENT_NOTARY_ROOTS,
            min_liquidity_reservation_roots: DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS,
            min_payout_throttle_roots: DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS,
            min_privacy_budget_roots: DEFAULT_MIN_PRIVACY_BUDGET_ROOTS,
            roots_only_public_record: true,
            release_execution_allowed: false,
            disbursement_reservations: 0,
            liquidity_blockers_active: true,
            throttle_blockers_active: true,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_id": self.wave_id,
            "prior_wave_id": self.prior_wave_id,
            "lane_id": self.lane_id,
            "prior_lane_id": self.prior_lane_id,
            "min_wave100_settlement_notary_roots": self.min_wave100_settlement_notary_roots,
            "min_liquidity_reservation_roots": self.min_liquidity_reservation_roots,
            "min_payout_throttle_roots": self.min_payout_throttle_roots,
            "min_privacy_budget_roots": self.min_privacy_budget_roots,
            "roots_only_public_record": self.roots_only_public_record,
            "release_execution_allowed": self.release_execution_allowed,
            "disbursement_reservations": self.disbursement_reservations,
            "liquidity_blockers_active": self.liquidity_blockers_active,
            "throttle_blockers_active": self.throttle_blockers_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave100SettlementNotaryRoot {
    pub gate: CompileLaneGate,
    pub settlement_notary_root: String,
    pub execution_bundle_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub active: bool,
}

impl Wave100SettlementNotaryRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            settlement_notary_root: placeholder_root("wave100-settlement-notary", gate.as_str()),
            execution_bundle_root: placeholder_root("wave100-execution-bundle", gate.as_str()),
            notary_quorum_root: placeholder_root("wave100-notary-quorum", gate.as_str()),
            payout_envelope_root: placeholder_root("wave100-payout-envelope", gate.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "settlement_notary_root": self.settlement_notary_root,
            "execution_bundle_root": self.execution_bundle_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave100_settlement_notary_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiquidityReservationRoot {
    pub gate: CompileLaneGate,
    pub reservation_root: String,
    pub liquidity_pool_root: String,
    pub reserve_policy_root: String,
    pub disbursement_window_root: String,
    pub reserved: bool,
}

impl LiquidityReservationRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            reservation_root: placeholder_root("liquidity-reservation", gate.as_str()),
            liquidity_pool_root: placeholder_root("liquidity-pool", gate.as_str()),
            reserve_policy_root: placeholder_root("liquidity-reserve-policy", gate.as_str()),
            disbursement_window_root: placeholder_root("disbursement-window", gate.as_str()),
            reserved: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "reservation_root": self.reservation_root,
            "liquidity_pool_root": self.liquidity_pool_root,
            "reserve_policy_root": self.reserve_policy_root,
            "disbursement_window_root": self.disbursement_window_root,
            "reserved": self.reserved,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("liquidity_reservation_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompileFeeNettingRoot {
    pub gate: CompileLaneGate,
    pub fee_netting_root: String,
    pub fee_liability_root: String,
    pub rebate_pool_root: String,
    pub reconciliation_root: String,
    pub reconciled: bool,
}

impl CompileFeeNettingRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            fee_netting_root: placeholder_root("compile-fee-netting", gate.as_str()),
            fee_liability_root: placeholder_root("compile-fee-liability", gate.as_str()),
            rebate_pool_root: placeholder_root("compile-rebate-pool", gate.as_str()),
            reconciliation_root: placeholder_root("compile-fee-reconciliation", gate.as_str()),
            reconciled: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "fee_netting_root": self.fee_netting_root,
            "fee_liability_root": self.fee_liability_root,
            "rebate_pool_root": self.rebate_pool_root,
            "reconciliation_root": self.reconciliation_root,
            "reconciled": self.reconciled,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("compile_fee_netting_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PayoutThrottleRoot {
    pub gate: CompileLaneGate,
    pub throttle_root: String,
    pub rate_limit_root: String,
    pub burst_window_root: String,
    pub queue_depth_root: String,
    pub active: bool,
}

impl PayoutThrottleRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            throttle_root: placeholder_root("payout-throttle", gate.as_str()),
            rate_limit_root: placeholder_root("payout-rate-limit", gate.as_str()),
            burst_window_root: placeholder_root("payout-burst-window", gate.as_str()),
            queue_depth_root: placeholder_root("payout-queue-depth", gate.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "throttle_root": self.throttle_root,
            "rate_limit_root": self.rate_limit_root,
            "burst_window_root": self.burst_window_root,
            "queue_depth_root": self.queue_depth_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("payout_throttle_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyBudgetRoot {
    pub gate: CompileLaneGate,
    pub budget_root: String,
    pub disclosure_limit_root: String,
    pub nullifier_budget_root: String,
    pub audit_mask_root: String,
    pub locked: bool,
}

impl PrivacyBudgetRoot {
    pub fn locked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            budget_root: placeholder_root("privacy-budget", gate.as_str()),
            disclosure_limit_root: placeholder_root("privacy-disclosure-limit", gate.as_str()),
            nullifier_budget_root: placeholder_root("privacy-nullifier-budget", gate.as_str()),
            audit_mask_root: placeholder_root("privacy-audit-mask", gate.as_str()),
            locked: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "budget_root": self.budget_root,
            "disclosure_limit_root": self.disclosure_limit_root,
            "nullifier_budget_root": self.nullifier_budget_root,
            "audit_mask_root": self.audit_mask_root,
            "locked": self.locked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("privacy_budget_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitBreakerRoot {
    pub gate: CompileLaneGate,
    pub breaker_root: String,
    pub liquidity_threshold_root: String,
    pub throttle_threshold_root: String,
    pub recovery_root: String,
    pub tripped: bool,
}

impl CircuitBreakerRoot {
    pub fn tripped(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            breaker_root: placeholder_root("circuit-breaker", gate.as_str()),
            liquidity_threshold_root: placeholder_root(
                "liquidity-breaker-threshold",
                gate.as_str(),
            ),
            throttle_threshold_root: placeholder_root("throttle-breaker-threshold", gate.as_str()),
            recovery_root: placeholder_root("circuit-breaker-recovery", gate.as_str()),
            tripped: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "breaker_root": self.breaker_root,
            "liquidity_threshold_root": self.liquidity_threshold_root,
            "throttle_threshold_root": self.throttle_threshold_root,
            "recovery_root": self.recovery_root,
            "tripped": self.tripped,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("circuit_breaker_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignoffRoot {
    pub gate: CompileLaneGate,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub liquidity_controller_root: String,
    pub complete: bool,
}

impl SignoffRoot {
    pub fn missing(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            operator_signoff_root: placeholder_root("operator-signoff", gate.as_str()),
            reviewer_signoff_root: placeholder_root("reviewer-signoff", gate.as_str()),
            liquidity_controller_root: placeholder_root(
                "liquidity-controller-signoff",
                gate.as_str(),
            ),
            complete: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "liquidity_controller_root": self.liquidity_controller_root,
            "complete": self.complete,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("signoff_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub gate: CompileLaneGate,
    pub command_family: String,
    pub command_hint_root: String,
    pub capture_root: String,
}

impl CommandHint {
    pub fn for_gate(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            command_family: gate.as_str().to_string(),
            command_hint_root: placeholder_root("disbursement-command-hint", gate.as_str()),
            capture_root: placeholder_root("disbursement-capture-rule", gate.as_str()),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "command_family": self.command_family,
            "command_hint_root": self.command_hint_root,
            "capture_root": self.capture_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisbursementBlocker {
    pub kind: DisbursementBlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl DisbursementBlocker {
    pub fn new(kind: DisbursementBlockerKind, gate: Option<CompileLaneGate>) -> Self {
        let label = match gate {
            Some(value) => value.as_str(),
            None => "compile_lane",
        };
        Self {
            kind,
            gate,
            blocker_root: placeholder_root(kind.as_str(), label),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "gate": self.gate.map(CompileLaneGate::as_str),
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisbursementCounters {
    pub wave100_settlement_notary_roots: usize,
    pub liquidity_reservation_roots: usize,
    pub compile_fee_netting_roots: usize,
    pub payout_throttle_roots: usize,
    pub privacy_budget_roots: usize,
    pub circuit_breaker_roots: usize,
    pub signoff_roots: usize,
    pub command_hint_roots: usize,
    pub active_blockers: usize,
    pub disbursement_reservations: usize,
}

impl DisbursementCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "wave100_settlement_notary_roots": self.wave100_settlement_notary_roots,
            "liquidity_reservation_roots": self.liquidity_reservation_roots,
            "compile_fee_netting_roots": self.compile_fee_netting_roots,
            "payout_throttle_roots": self.payout_throttle_roots,
            "privacy_budget_roots": self.privacy_budget_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "signoff_roots": self.signoff_roots,
            "command_hint_roots": self.command_hint_roots,
            "active_blockers": self.active_blockers,
            "disbursement_reservations": self.disbursement_reservations,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave100_settlement_notary_roots: Vec<Wave100SettlementNotaryRoot>,
    pub liquidity_reservation_roots: Vec<LiquidityReservationRoot>,
    pub compile_fee_netting_roots: Vec<CompileFeeNettingRoot>,
    pub payout_throttle_roots: Vec<PayoutThrottleRoot>,
    pub privacy_budget_roots: Vec<PrivacyBudgetRoot>,
    pub circuit_breaker_roots: Vec<CircuitBreakerRoot>,
    pub signoff_roots: Vec<SignoffRoot>,
    pub command_hints: Vec<CommandHint>,
    pub blockers: Vec<DisbursementBlocker>,
    pub counters: DisbursementCounters,
    pub verdict: DisbursementVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave100_settlement_notary_roots = CompileLaneGate::all()
            .into_iter()
            .map(Wave100SettlementNotaryRoot::active)
            .collect::<Vec<_>>();
        let liquidity_reservation_roots = CompileLaneGate::all()
            .into_iter()
            .map(LiquidityReservationRoot::blocked)
            .collect::<Vec<_>>();
        let compile_fee_netting_roots = CompileLaneGate::all()
            .into_iter()
            .map(CompileFeeNettingRoot::blocked)
            .collect::<Vec<_>>();
        let payout_throttle_roots = CompileLaneGate::all()
            .into_iter()
            .map(PayoutThrottleRoot::active)
            .collect::<Vec<_>>();
        let privacy_budget_roots = CompileLaneGate::all()
            .into_iter()
            .map(PrivacyBudgetRoot::locked)
            .collect::<Vec<_>>();
        let circuit_breaker_roots = CompileLaneGate::all()
            .into_iter()
            .map(CircuitBreakerRoot::tripped)
            .collect::<Vec<_>>();
        let signoff_roots = CompileLaneGate::all()
            .into_iter()
            .map(SignoffRoot::missing)
            .collect::<Vec<_>>();
        let command_hints = CompileLaneGate::all()
            .into_iter()
            .map(CommandHint::for_gate)
            .collect::<Vec<_>>();
        let mut blockers = vec![
            DisbursementBlocker::new(DisbursementBlockerKind::DefaultFailClosed, None),
            DisbursementBlocker::new(DisbursementBlockerKind::HeavyGateNotRun, None),
            DisbursementBlocker::new(DisbursementBlockerKind::ReleaseExecutionDenied, None),
            DisbursementBlocker::new(
                DisbursementBlockerKind::DisbursementReservationMissing,
                None,
            ),
            DisbursementBlocker::new(DisbursementBlockerKind::RootsOnlyBoundary, None),
        ];
        for gate in CompileLaneGate::all() {
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::Wave100SettlementNotaryActive,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::SettlementNotaryMissing,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::LiquidityReservationMissing,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::LiquidityReservationBlocked,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::FeeNettingUnreconciled,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::PayoutThrottleActive,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::PrivacyBudgetLocked,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::CircuitBreakerActive,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::OperatorSignoffMissing,
                Some(gate),
            ));
            blockers.push(DisbursementBlocker::new(
                DisbursementBlockerKind::ReviewerSignoffMissing,
                Some(gate),
            ));
        }
        let counters = DisbursementCounters {
            wave100_settlement_notary_roots: wave100_settlement_notary_roots.len(),
            liquidity_reservation_roots: liquidity_reservation_roots.len(),
            compile_fee_netting_roots: compile_fee_netting_roots.len(),
            payout_throttle_roots: payout_throttle_roots.len(),
            privacy_budget_roots: privacy_budget_roots.len(),
            circuit_breaker_roots: circuit_breaker_roots.len(),
            signoff_roots: signoff_roots.len(),
            command_hint_roots: command_hints.len(),
            active_blockers: blockers.len(),
            disbursement_reservations: config.disbursement_reservations,
        };
        Self {
            config,
            wave100_settlement_notary_roots,
            liquidity_reservation_roots,
            compile_fee_netting_roots,
            payout_throttle_roots,
            privacy_budget_roots,
            circuit_breaker_roots,
            signoff_roots,
            command_hints,
            blockers,
            counters,
            verdict: DisbursementVerdict::ReleaseExecutionDenied,
        }
    }

    pub fn public_record(&self) -> Value {
        let wave100_settlement_notary_roots = record_roots(
            "wave100-settlement-notary-roots",
            self.wave100_settlement_notary_roots
                .iter()
                .map(Wave100SettlementNotaryRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let liquidity_reservation_roots = record_roots(
            "liquidity-reservation-roots",
            self.liquidity_reservation_roots
                .iter()
                .map(LiquidityReservationRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let compile_fee_netting_roots = record_roots(
            "compile-fee-netting-roots",
            self.compile_fee_netting_roots
                .iter()
                .map(CompileFeeNettingRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let payout_throttle_roots = record_roots(
            "payout-throttle-roots",
            self.payout_throttle_roots
                .iter()
                .map(PayoutThrottleRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let privacy_budget_roots = record_roots(
            "privacy-budget-roots",
            self.privacy_budget_roots
                .iter()
                .map(PrivacyBudgetRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let circuit_breaker_roots = record_roots(
            "circuit-breaker-roots",
            self.circuit_breaker_roots
                .iter()
                .map(CircuitBreakerRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let signoff_roots = record_roots(
            "operator-reviewer-signoff-roots",
            self.signoff_roots
                .iter()
                .map(SignoffRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let command_hint_roots = record_roots(
            "command-hint-roots",
            self.command_hints
                .iter()
                .map(CommandHint::state_root)
                .collect::<Vec<_>>(),
        );
        let blocker_roots = record_roots(
            "disbursement-blocker-roots",
            self.blockers
                .iter()
                .map(DisbursementBlocker::state_root)
                .collect::<Vec<_>>(),
        );
        let deterministic_roots = record_roots(
            "deterministic-disbursement-liquidity-throttle-guard-roots",
            vec![
                wave100_settlement_notary_roots.merkle.clone(),
                liquidity_reservation_roots.merkle.clone(),
                compile_fee_netting_roots.merkle.clone(),
                payout_throttle_roots.merkle.clone(),
                privacy_budget_roots.merkle.clone(),
                circuit_breaker_roots.merkle.clone(),
                signoff_roots.merkle.clone(),
                command_hint_roots.merkle.clone(),
                blocker_roots.merkle.clone(),
            ],
        );

        json!({
            "config_root": self.config.state_root(),
            "wave100_settlement_notary_root_count": self.wave100_settlement_notary_roots.len(),
            "wave100_settlement_notary_roots": wave100_settlement_notary_roots.items,
            "wave100_settlement_notary_merkle": wave100_settlement_notary_roots.merkle,
            "liquidity_reservation_root_count": self.liquidity_reservation_roots.len(),
            "liquidity_reservation_roots": liquidity_reservation_roots.items,
            "liquidity_reservation_merkle": liquidity_reservation_roots.merkle,
            "compile_fee_netting_roots": compile_fee_netting_roots.items,
            "compile_fee_netting_merkle": compile_fee_netting_roots.merkle,
            "payout_throttle_roots": payout_throttle_roots.items,
            "payout_throttle_merkle": payout_throttle_roots.merkle,
            "privacy_budget_roots": privacy_budget_roots.items,
            "privacy_budget_merkle": privacy_budget_roots.merkle,
            "circuit_breaker_roots": circuit_breaker_roots.items,
            "circuit_breaker_merkle": circuit_breaker_roots.merkle,
            "operator_reviewer_signoff_roots": signoff_roots.items,
            "operator_reviewer_signoff_merkle": signoff_roots.merkle,
            "command_hint_roots": command_hint_roots.items,
            "command_hint_merkle": command_hint_roots.merkle,
            "disbursement_blocker_roots": blocker_roots.items,
            "disbursement_blocker_merkle": blocker_roots.merkle,
            "deterministic_roots": deterministic_roots.items,
            "deterministic_merkle": deterministic_roots.merkle,
            "counters_root": self.counters.state_root(),
            "disbursement_reservations": self.counters.disbursement_reservations,
            "active_disbursement_blockers": self.blockers.len(),
            "wave100_settlement_notaries_active": self.wave100_settlement_notary_roots.iter().filter(|root| root.active).count(),
            "liquidity_reservations_ready": self.liquidity_reservation_roots.iter().filter(|root| root.reserved).count(),
            "compile_fee_netting_reconciled": self.compile_fee_netting_roots.iter().filter(|root| root.reconciled).count(),
            "payout_throttles_active": self.payout_throttle_roots.iter().filter(|root| root.active).count(),
            "privacy_budgets_locked": self.privacy_budget_roots.iter().filter(|root| root.locked).count(),
            "circuit_breakers_tripped": self.circuit_breaker_roots.iter().filter(|root| root.tripped).count(),
            "release_execution_allowed": self.config.release_execution_allowed,
            "liquidity_blockers_active": self.config.liquidity_blockers_active,
            "throttle_blockers_active": self.config.throttle_blockers_active,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "roots_only_public_record": self.config.roots_only_public_record,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE101-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-COMPILE-LANE-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
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

struct RootList {
    items: Vec<String>,
    merkle: String,
}

fn record_roots(kind: &str, roots: Vec<String>) -> RootList {
    let leaves = roots
        .iter()
        .map(|root| Value::String(domain_hash(kind, &[HashPart::Str(root)], 32)))
        .collect::<Vec<_>>();
    RootList {
        items: roots,
        merkle: merkle_root(&format!("{DOMAIN}:{kind}"), &leaves),
    }
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "WAVE101-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-COMPILE-LANE-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn placeholder_root(kind: &str, label: &str) -> String {
    domain_hash(
        "WAVE101-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

const DOMAIN: &str = "WAVE101-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-COMPILE-LANE";
