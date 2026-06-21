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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave101";
pub const SOURCE_WAVE_LABEL: &str = "wave100";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-release-execution-settlement-notary-guard-wallet-watchtower-lane";
pub const DISBURSEMENT_LIQUIDITY_THROTTLE_GUARD_LANE: &str =
    "force-exit-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str =
    "empty-wave101-wallet-watchtower-release-execution-disbursement-liquidity-throttle-guard-root";
pub const DEFAULT_DISBURSEMENT_EPOCH: u64 = 101;
pub const DEFAULT_MIN_WAVE100_SETTLEMENT_NOTARY_ROOTS: u64 = 2;
pub const DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WALLET_FEE_NETTING_ROOTS: u64 = 2;
pub const DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS: u64 = 2;
pub const DEFAULT_MIN_PRIVACY_BUDGET_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WATCHTOWER_OBSERVATION_ROOTS: u64 = 3;
pub const DEFAULT_MIN_CIRCUIT_BREAKER_ROOTS: u64 = 2;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: u64 = 1;
pub const DEFAULT_MIN_REVIEWER_SIGNOFF_ROOTS: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub source_lane: String,
    pub disbursement_liquidity_throttle_guard_lane: String,
    pub empty_root_marker: String,
    pub disbursement_epoch: u64,
    pub min_wave100_settlement_notary_roots: u64,
    pub min_liquidity_reservation_roots: u64,
    pub min_wallet_fee_netting_roots: u64,
    pub min_payout_throttle_roots: u64,
    pub min_privacy_budget_roots: u64,
    pub min_watchtower_observation_roots: u64,
    pub min_circuit_breaker_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub min_reviewer_signoff_roots: u64,
    pub require_roots_only_public_record: bool,
    pub require_wave100_settlement_notary_roots: bool,
    pub require_liquidity_reservation_roots: bool,
    pub require_wallet_fee_netting_roots: bool,
    pub require_payout_throttle_roots: bool,
    pub require_privacy_budget_roots: bool,
    pub require_watchtower_observation_roots: bool,
    pub require_circuit_breaker_roots: bool,
    pub require_operator_signoff_roots: bool,
    pub require_reviewer_signoff_roots: bool,
    pub wallet_liquidity_blocker_active: bool,
    pub watchtower_liquidity_blocker_active: bool,
    pub payout_throttle_blocker_active: bool,
    pub privacy_budget_blocker_active: bool,
    pub circuit_breaker_active: bool,
    pub disbursement_reservations_enabled: bool,
    pub release_execution_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_label: WAVE_LABEL.to_string(),
            source_wave_label: SOURCE_WAVE_LABEL.to_string(),
            source_lane: SOURCE_LANE.to_string(),
            disbursement_liquidity_throttle_guard_lane: DISBURSEMENT_LIQUIDITY_THROTTLE_GUARD_LANE
                .to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            disbursement_epoch: DEFAULT_DISBURSEMENT_EPOCH,
            min_wave100_settlement_notary_roots: DEFAULT_MIN_WAVE100_SETTLEMENT_NOTARY_ROOTS,
            min_liquidity_reservation_roots: DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS,
            min_wallet_fee_netting_roots: DEFAULT_MIN_WALLET_FEE_NETTING_ROOTS,
            min_payout_throttle_roots: DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS,
            min_privacy_budget_roots: DEFAULT_MIN_PRIVACY_BUDGET_ROOTS,
            min_watchtower_observation_roots: DEFAULT_MIN_WATCHTOWER_OBSERVATION_ROOTS,
            min_circuit_breaker_roots: DEFAULT_MIN_CIRCUIT_BREAKER_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            min_reviewer_signoff_roots: DEFAULT_MIN_REVIEWER_SIGNOFF_ROOTS,
            require_roots_only_public_record: true,
            require_wave100_settlement_notary_roots: true,
            require_liquidity_reservation_roots: true,
            require_wallet_fee_netting_roots: true,
            require_payout_throttle_roots: true,
            require_privacy_budget_roots: true,
            require_watchtower_observation_roots: true,
            require_circuit_breaker_roots: true,
            require_operator_signoff_roots: true,
            require_reviewer_signoff_roots: true,
            wallet_liquidity_blocker_active: true,
            watchtower_liquidity_blocker_active: true,
            payout_throttle_blocker_active: true,
            privacy_budget_blocker_active: true,
            circuit_breaker_active: true,
            disbursement_reservations_enabled: false,
            release_execution_enabled: false,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_label": self.wave_label,
            "source_wave_label": self.source_wave_label,
            "source_lane": self.source_lane,
            "disbursement_liquidity_throttle_guard_lane": self.disbursement_liquidity_throttle_guard_lane,
            "empty_root_marker": self.empty_root_marker,
            "disbursement_epoch": self.disbursement_epoch,
            "min_wave100_settlement_notary_roots": self.min_wave100_settlement_notary_roots,
            "min_liquidity_reservation_roots": self.min_liquidity_reservation_roots,
            "min_wallet_fee_netting_roots": self.min_wallet_fee_netting_roots,
            "min_payout_throttle_roots": self.min_payout_throttle_roots,
            "min_privacy_budget_roots": self.min_privacy_budget_roots,
            "min_watchtower_observation_roots": self.min_watchtower_observation_roots,
            "min_circuit_breaker_roots": self.min_circuit_breaker_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "min_reviewer_signoff_roots": self.min_reviewer_signoff_roots,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave100_settlement_notary_roots": self.require_wave100_settlement_notary_roots,
            "require_liquidity_reservation_roots": self.require_liquidity_reservation_roots,
            "require_wallet_fee_netting_roots": self.require_wallet_fee_netting_roots,
            "require_payout_throttle_roots": self.require_payout_throttle_roots,
            "require_privacy_budget_roots": self.require_privacy_budget_roots,
            "require_watchtower_observation_roots": self.require_watchtower_observation_roots,
            "require_circuit_breaker_roots": self.require_circuit_breaker_roots,
            "require_operator_signoff_roots": self.require_operator_signoff_roots,
            "require_reviewer_signoff_roots": self.require_reviewer_signoff_roots,
            "wallet_liquidity_blocker_active": self.wallet_liquidity_blocker_active,
            "watchtower_liquidity_blocker_active": self.watchtower_liquidity_blocker_active,
            "payout_throttle_blocker_active": self.payout_throttle_blocker_active,
            "privacy_budget_blocker_active": self.privacy_budget_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
            "disbursement_reservations_enabled": self.disbursement_reservations_enabled,
            "release_execution_enabled": self.release_execution_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementSlotKind {
    Wave100SettlementNotary,
    LiquidityReservation,
    WalletFeeNetting,
    PayoutThrottle,
    PrivacyBudget,
    WatchtowerObservation,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    CommandHint,
}

impl DisbursementSlotKind {
    pub fn all() -> [Self; 10] {
        [
            Self::Wave100SettlementNotary,
            Self::LiquidityReservation,
            Self::WalletFeeNetting,
            Self::PayoutThrottle,
            Self::PrivacyBudget,
            Self::WatchtowerObservation,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::CommandHint,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotary => "wave100_settlement_notary",
            Self::LiquidityReservation => "liquidity_reservation",
            Self::WalletFeeNetting => "wallet_fee_netting",
            Self::PayoutThrottle => "payout_throttle",
            Self::PrivacyBudget => "privacy_budget",
            Self::WatchtowerObservation => "watchtower_observation",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::CommandHint => "command_hint",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementStatus {
    ReservationAbsent,
    LiquidityBlocked,
    Throttled,
    Ready,
    Denied,
}

impl DisbursementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReservationAbsent => "reservation_absent",
            Self::LiquidityBlocked => "liquidity_blocked",
            Self::Throttled => "throttled",
            Self::Ready => "ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_disburse(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisbursementBlocker {
    Wave100SettlementNotaryRootMissing,
    LiquidityReservationRootMissing,
    WalletFeeNettingRootMissing,
    PayoutThrottleRootMissing,
    PrivacyBudgetRootMissing,
    WatchtowerObservationRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    DuplicateSettlementNotaryRoot,
    DuplicateLiquidityReservationRoot,
    FeeNettingLaggingReservationRoot,
    ThrottleLaggingFeeNettingRoot,
    PrivacyBudgetLaggingThrottleRoot,
    ObservationLaggingReservationRoot,
    CircuitBreakerActive,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    WalletLiquidityBlockerActive,
    WatchtowerLiquidityBlockerActive,
    PayoutThrottleBlockerActive,
    PrivacyBudgetBlockerActive,
    ReservationsDisabled,
    ExecutionDisabled,
    HeavyGatesNotRun,
}

impl DisbursementBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotaryRootMissing => "wave100_settlement_notary_root_missing",
            Self::LiquidityReservationRootMissing => "liquidity_reservation_root_missing",
            Self::WalletFeeNettingRootMissing => "wallet_fee_netting_root_missing",
            Self::PayoutThrottleRootMissing => "payout_throttle_root_missing",
            Self::PrivacyBudgetRootMissing => "privacy_budget_root_missing",
            Self::WatchtowerObservationRootMissing => "watchtower_observation_root_missing",
            Self::CircuitBreakerRootMissing => "circuit_breaker_root_missing",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::DuplicateSettlementNotaryRoot => "duplicate_settlement_notary_root",
            Self::DuplicateLiquidityReservationRoot => "duplicate_liquidity_reservation_root",
            Self::FeeNettingLaggingReservationRoot => "fee_netting_lagging_reservation_root",
            Self::ThrottleLaggingFeeNettingRoot => "throttle_lagging_fee_netting_root",
            Self::PrivacyBudgetLaggingThrottleRoot => "privacy_budget_lagging_throttle_root",
            Self::ObservationLaggingReservationRoot => "observation_lagging_reservation_root",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::WalletLiquidityBlockerActive => "wallet_liquidity_blocker_active",
            Self::WatchtowerLiquidityBlockerActive => "watchtower_liquidity_blocker_active",
            Self::PayoutThrottleBlockerActive => "payout_throttle_blocker_active",
            Self::PrivacyBudgetBlockerActive => "privacy_budget_blocker_active",
            Self::ReservationsDisabled => "reservations_disabled",
            Self::ExecutionDisabled => "execution_disabled",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldDisbursement,
    ImportWave100SettlementNotaryRoot,
    ImportLiquidityReservationRoot,
    ImportWalletFeeNettingRoot,
    ImportPayoutThrottleRoot,
    ImportPrivacyBudgetRoot,
    ImportWatchtowerObservationRoot,
    ImportCircuitBreakerRoot,
    ImportOperatorSignoffRoot,
    ImportReviewerSignoffRoot,
    ResolveDuplicateSettlementNotaryRoot,
    ResolveDuplicateLiquidityReservationRoot,
    ReconcileFeeNettingRoot,
    ReconcilePayoutThrottleRoot,
    ReconcilePrivacyBudgetRoot,
    ReconcileWatchtowerObservationRoot,
    MaintainLiquidityHold,
    ClearCircuitBreaker,
    DisburseAfterLiquidityThrottleGuard,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldDisbursement => "hold_disbursement",
            Self::ImportWave100SettlementNotaryRoot => "import_wave100_settlement_notary_root",
            Self::ImportLiquidityReservationRoot => "import_liquidity_reservation_root",
            Self::ImportWalletFeeNettingRoot => "import_wallet_fee_netting_root",
            Self::ImportPayoutThrottleRoot => "import_payout_throttle_root",
            Self::ImportPrivacyBudgetRoot => "import_privacy_budget_root",
            Self::ImportWatchtowerObservationRoot => "import_watchtower_observation_root",
            Self::ImportCircuitBreakerRoot => "import_circuit_breaker_root",
            Self::ImportOperatorSignoffRoot => "import_operator_signoff_root",
            Self::ImportReviewerSignoffRoot => "import_reviewer_signoff_root",
            Self::ResolveDuplicateSettlementNotaryRoot => {
                "resolve_duplicate_settlement_notary_root"
            }
            Self::ResolveDuplicateLiquidityReservationRoot => {
                "resolve_duplicate_liquidity_reservation_root"
            }
            Self::ReconcileFeeNettingRoot => "reconcile_fee_netting_root",
            Self::ReconcilePayoutThrottleRoot => "reconcile_payout_throttle_root",
            Self::ReconcilePrivacyBudgetRoot => "reconcile_privacy_budget_root",
            Self::ReconcileWatchtowerObservationRoot => "reconcile_watchtower_observation_root",
            Self::MaintainLiquidityHold => "maintain_liquidity_hold",
            Self::ClearCircuitBreaker => "clear_circuit_breaker",
            Self::DisburseAfterLiquidityThrottleGuard => "disburse_after_liquidity_throttle_guard",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub command_root: String,
    pub blocker_root: String,
}

impl CommandHint {
    pub fn new(
        slot_kind: DisbursementSlotKind,
        kind: CommandHintKind,
        blockers: &[DisbursementBlocker],
    ) -> Self {
        let blocker_root = blockers_root("command-next-blockers", blockers);
        let command_root = record_root(
            "command-hint",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "kind": kind.as_str(),
                "blocker_root": blocker_root,
                "raw_command_material_absent": true,
                "roots_only": true,
            }),
        );
        Self {
            kind,
            command_root,
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisbursementGuardEntry {
    pub slot_kind: DisbursementSlotKind,
    pub wave100_settlement_notary_roots: Vec<String>,
    pub liquidity_reservation_roots: Vec<String>,
    pub wallet_fee_netting_roots: Vec<String>,
    pub payout_throttle_roots: Vec<String>,
    pub privacy_budget_roots: Vec<String>,
    pub watchtower_observation_roots: Vec<String>,
    pub circuit_breaker_roots: Vec<String>,
    pub operator_signoff_roots: Vec<String>,
    pub reviewer_signoff_roots: Vec<String>,
    pub wave100_settlement_notary_root: String,
    pub liquidity_reservation_root: String,
    pub wallet_fee_netting_root: String,
    pub payout_throttle_root: String,
    pub privacy_budget_root: String,
    pub watchtower_observation_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub disbursement_liquidity_throttle_guard_root: String,
    pub deterministic_disbursement_root: String,
    pub blockers: Vec<DisbursementBlocker>,
    pub status: DisbursementStatus,
    pub command_hint: CommandHint,
    pub reservation_allowed: bool,
    pub disbursement_allowed: bool,
}

impl DisbursementGuardEntry {
    pub fn empty(slot_kind: DisbursementSlotKind, config: &Config) -> Self {
        Self::from_roots(
            slot_kind,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            config,
        )
    }

    pub fn from_roots(
        slot_kind: DisbursementSlotKind,
        wave100_settlement_notary_roots: Vec<String>,
        liquidity_reservation_roots: Vec<String>,
        wallet_fee_netting_roots: Vec<String>,
        payout_throttle_roots: Vec<String>,
        privacy_budget_roots: Vec<String>,
        watchtower_observation_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut entry = Self {
            slot_kind,
            wave100_settlement_notary_roots,
            liquidity_reservation_roots,
            wallet_fee_netting_roots,
            payout_throttle_roots,
            privacy_budget_roots,
            watchtower_observation_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            wave100_settlement_notary_root: empty_root("wave100-settlement-notary"),
            liquidity_reservation_root: empty_root("liquidity-reservation"),
            wallet_fee_netting_root: empty_root("wallet-fee-netting"),
            payout_throttle_root: empty_root("payout-throttle"),
            privacy_budget_root: empty_root("privacy-budget"),
            watchtower_observation_root: empty_root("watchtower-observation"),
            circuit_breaker_root: empty_root("circuit-breaker"),
            operator_signoff_root: empty_root("operator-signoff"),
            reviewer_signoff_root: empty_root("reviewer-signoff"),
            disbursement_liquidity_throttle_guard_root: empty_root(
                "disbursement-liquidity-throttle-guard",
            ),
            deterministic_disbursement_root: empty_root("deterministic-disbursement"),
            blockers: Vec::new(),
            status: DisbursementStatus::ReservationAbsent,
            command_hint: CommandHint::new(slot_kind, CommandHintKind::HoldDisbursement, &[]),
            reservation_allowed: false,
            disbursement_allowed: false,
        };
        entry.recompute(config);
        entry
    }

    pub fn recompute(&mut self, config: &Config) {
        self.wave100_settlement_notary_root = aggregate_root(
            "wave100-settlement-notary-roots",
            &self.wave100_settlement_notary_roots,
        );
        self.liquidity_reservation_root = aggregate_root(
            "liquidity-reservation-roots",
            &self.liquidity_reservation_roots,
        );
        self.wallet_fee_netting_root =
            aggregate_root("wallet-fee-netting-roots", &self.wallet_fee_netting_roots);
        self.payout_throttle_root =
            aggregate_root("payout-throttle-roots", &self.payout_throttle_roots);
        self.privacy_budget_root =
            aggregate_root("privacy-budget-roots", &self.privacy_budget_roots);
        self.watchtower_observation_root = aggregate_root(
            "watchtower-observation-roots",
            &self.watchtower_observation_roots,
        );
        self.circuit_breaker_root =
            aggregate_root("circuit-breaker-roots", &self.circuit_breaker_roots);
        self.operator_signoff_root =
            aggregate_root("operator-signoff-roots", &self.operator_signoff_roots);
        self.reviewer_signoff_root =
            aggregate_root("reviewer-signoff-roots", &self.reviewer_signoff_roots);
        self.disbursement_liquidity_throttle_guard_root =
            disbursement_liquidity_throttle_guard_root(self);
        self.deterministic_disbursement_root = deterministic_disbursement_root(self);
        self.blockers = disbursement_blockers(self, config);
        self.status = if !config.release_execution_enabled
            || !config.disbursement_reservations_enabled
            || !config.heavy_gates_ran
        {
            DisbursementStatus::Denied
        } else if config.wallet_liquidity_blocker_active
            || config.watchtower_liquidity_blocker_active
            || config.privacy_budget_blocker_active
            || config.circuit_breaker_active
        {
            DisbursementStatus::LiquidityBlocked
        } else if config.payout_throttle_blocker_active {
            DisbursementStatus::Throttled
        } else if self.liquidity_reservation_roots.is_empty() {
            DisbursementStatus::ReservationAbsent
        } else if self.blockers.is_empty() {
            DisbursementStatus::Ready
        } else {
            DisbursementStatus::LiquidityBlocked
        };
        self.reservation_allowed = self.status.can_disburse()
            && config.disbursement_reservations_enabled
            && config.release_execution_enabled
            && config.heavy_gates_ran;
        self.disbursement_allowed = self.reservation_allowed && self.blockers.is_empty();
        self.command_hint =
            CommandHint::new(self.slot_kind, command_for_entry(self), &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        blockers_root("entry-blockers", &self.blockers)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "wallet_fee_netting_root": self.wallet_fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "privacy_budget_root": self.privacy_budget_root,
            "watchtower_observation_root": self.watchtower_observation_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "disbursement_liquidity_throttle_guard_root": self.disbursement_liquidity_throttle_guard_root,
            "deterministic_disbursement_root": self.deterministic_disbursement_root,
            "blocker_root": self.blocker_root(),
            "status": self.status.as_str(),
            "command_hint": self.command_hint.public_record(),
            "reservation_allowed": self.reservation_allowed,
            "disbursement_allowed": self.disbursement_allowed,
            "roots_only": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement-guard-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisbursementSummary {
    pub fail_closed: bool,
    pub release_execution_denied: bool,
    pub disbursement_reservations_denied: bool,
    pub reservation_allowed_count: u64,
    pub disbursement_allowed_count: u64,
    pub blocked_count: u64,
    pub throttled_count: u64,
    pub denied_count: u64,
    pub reservation_absent_count: u64,
    pub wave100_settlement_notary_root: String,
    pub liquidity_reservation_root: String,
    pub wallet_fee_netting_root: String,
    pub payout_throttle_root: String,
    pub privacy_budget_root: String,
    pub watchtower_observation_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub disbursement_liquidity_throttle_guard_root: String,
    pub deterministic_disbursement_root: String,
    pub blocker_root: String,
    pub command_root: String,
    pub wallet_liquidity_blocker_active: bool,
    pub watchtower_liquidity_blocker_active: bool,
    pub payout_throttle_blocker_active: bool,
    pub privacy_budget_blocker_active: bool,
    pub circuit_breaker_active: bool,
    pub heavy_gates_ran: bool,
}

impl DisbursementSummary {
    pub fn from_entries(
        config: &Config,
        entries: &BTreeMap<String, DisbursementGuardEntry>,
    ) -> Self {
        let reservation_allowed_count = entries
            .values()
            .filter(|entry| entry.reservation_allowed)
            .count() as u64;
        let disbursement_allowed_count = entries
            .values()
            .filter(|entry| entry.disbursement_allowed)
            .count() as u64;
        let blocked_count = entries
            .values()
            .filter(|entry| entry.status == DisbursementStatus::LiquidityBlocked)
            .count() as u64;
        let throttled_count = entries
            .values()
            .filter(|entry| entry.status == DisbursementStatus::Throttled)
            .count() as u64;
        let denied_count = entries
            .values()
            .filter(|entry| entry.status == DisbursementStatus::Denied)
            .count() as u64;
        let reservation_absent_count = entries
            .values()
            .filter(|entry| entry.status == DisbursementStatus::ReservationAbsent)
            .count() as u64;
        let wave100_settlement_notary_root = entry_field_root(
            "summary-wave100-settlement-notary-roots",
            entries
                .values()
                .map(|entry| entry.wave100_settlement_notary_root.clone()),
        );
        let liquidity_reservation_root = entry_field_root(
            "summary-liquidity-reservation-roots",
            entries
                .values()
                .map(|entry| entry.liquidity_reservation_root.clone()),
        );
        let wallet_fee_netting_root = entry_field_root(
            "summary-wallet-fee-netting-roots",
            entries
                .values()
                .map(|entry| entry.wallet_fee_netting_root.clone()),
        );
        let payout_throttle_root = entry_field_root(
            "summary-payout-throttle-roots",
            entries
                .values()
                .map(|entry| entry.payout_throttle_root.clone()),
        );
        let privacy_budget_root = entry_field_root(
            "summary-privacy-budget-roots",
            entries
                .values()
                .map(|entry| entry.privacy_budget_root.clone()),
        );
        let watchtower_observation_root = entry_field_root(
            "summary-watchtower-observation-roots",
            entries
                .values()
                .map(|entry| entry.watchtower_observation_root.clone()),
        );
        let circuit_breaker_root = entry_field_root(
            "summary-circuit-breaker-roots",
            entries
                .values()
                .map(|entry| entry.circuit_breaker_root.clone()),
        );
        let operator_signoff_root = entry_field_root(
            "summary-operator-signoff-roots",
            entries
                .values()
                .map(|entry| entry.operator_signoff_root.clone()),
        );
        let reviewer_signoff_root = entry_field_root(
            "summary-reviewer-signoff-roots",
            entries
                .values()
                .map(|entry| entry.reviewer_signoff_root.clone()),
        );
        let disbursement_liquidity_throttle_guard_root = entry_field_root(
            "summary-disbursement-liquidity-throttle-guard-roots",
            entries
                .values()
                .map(|entry| entry.disbursement_liquidity_throttle_guard_root.clone()),
        );
        let deterministic_disbursement_root = entry_field_root(
            "summary-deterministic-disbursement-roots",
            entries
                .values()
                .map(|entry| entry.deterministic_disbursement_root.clone()),
        );
        let blocker_root = entry_field_root(
            "summary-blockers",
            entries.values().map(DisbursementGuardEntry::blocker_root),
        );
        let command_root = merkle_root(
            "WAVE101-WALLET-WATCHTOWER-COMMAND-HINTS",
            &entries
                .values()
                .map(|entry| entry.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let fail_closed = disbursement_allowed_count == 0
            || reservation_allowed_count == 0
            || blocked_count > 0
            || throttled_count > 0
            || denied_count > 0
            || reservation_absent_count > 0
            || !config.release_execution_enabled
            || !config.disbursement_reservations_enabled
            || config.wallet_liquidity_blocker_active
            || config.watchtower_liquidity_blocker_active
            || config.payout_throttle_blocker_active
            || config.privacy_budget_blocker_active
            || config.circuit_breaker_active
            || !config.heavy_gates_ran;
        Self {
            fail_closed,
            release_execution_denied: fail_closed,
            disbursement_reservations_denied: fail_closed,
            reservation_allowed_count,
            disbursement_allowed_count,
            blocked_count,
            throttled_count,
            denied_count,
            reservation_absent_count,
            wave100_settlement_notary_root,
            liquidity_reservation_root,
            wallet_fee_netting_root,
            payout_throttle_root,
            privacy_budget_root,
            watchtower_observation_root,
            circuit_breaker_root,
            operator_signoff_root,
            reviewer_signoff_root,
            disbursement_liquidity_throttle_guard_root,
            deterministic_disbursement_root,
            blocker_root,
            command_root,
            wallet_liquidity_blocker_active: config.wallet_liquidity_blocker_active,
            watchtower_liquidity_blocker_active: config.watchtower_liquidity_blocker_active,
            payout_throttle_blocker_active: config.payout_throttle_blocker_active,
            privacy_budget_blocker_active: config.privacy_budget_blocker_active,
            circuit_breaker_active: config.circuit_breaker_active,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "release_execution_denied": self.release_execution_denied,
            "disbursement_reservations_denied": self.disbursement_reservations_denied,
            "reservation_allowed_count": self.reservation_allowed_count,
            "disbursement_allowed_count": self.disbursement_allowed_count,
            "blocked_count": self.blocked_count,
            "throttled_count": self.throttled_count,
            "denied_count": self.denied_count,
            "reservation_absent_count": self.reservation_absent_count,
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "wallet_fee_netting_root": self.wallet_fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "privacy_budget_root": self.privacy_budget_root,
            "watchtower_observation_root": self.watchtower_observation_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "disbursement_liquidity_throttle_guard_root": self.disbursement_liquidity_throttle_guard_root,
            "deterministic_disbursement_root": self.deterministic_disbursement_root,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "wallet_liquidity_blocker_active": self.wallet_liquidity_blocker_active,
            "watchtower_liquidity_blocker_active": self.watchtower_liquidity_blocker_active,
            "payout_throttle_blocker_active": self.payout_throttle_blocker_active,
            "privacy_budget_blocker_active": self.privacy_budget_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub disbursement_entries: BTreeMap<String, DisbursementGuardEntry>,
    pub summary: DisbursementSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let disbursement_entries = DisbursementSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let entry = DisbursementGuardEntry::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = DisbursementSummary::from_entries(&config, &disbursement_entries);
        Self {
            config,
            disbursement_entries,
            summary,
        }
    }

    pub fn stage_disbursement_guard_entry(
        mut self,
        slot_kind: DisbursementSlotKind,
        wave100_settlement_notary_roots: Vec<String>,
        liquidity_reservation_roots: Vec<String>,
        wallet_fee_netting_roots: Vec<String>,
        payout_throttle_roots: Vec<String>,
        privacy_budget_roots: Vec<String>,
        watchtower_observation_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
    ) -> Result<Self> {
        let entry = DisbursementGuardEntry::from_roots(
            slot_kind,
            wave100_settlement_notary_roots,
            liquidity_reservation_roots,
            wallet_fee_netting_roots,
            payout_throttle_roots,
            privacy_budget_roots,
            watchtower_observation_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            &self.config,
        );
        self.disbursement_entries
            .insert(slot_kind.as_str().to_string(), entry);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for entry in self.disbursement_entries.values_mut() {
            entry.recompute(&self.config);
        }
        self.summary = DisbursementSummary::from_entries(&self.config, &self.disbursement_entries);
    }

    pub fn disbursement_entry_roots(&self) -> BTreeMap<String, String> {
        self.disbursement_entries
            .iter()
            .map(|(slot_name, entry)| (slot_name.clone(), entry.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn disbursement_entries_root(&self) -> String {
        merkle_root(
            "WAVE101-WALLET-WATCHTOWER-DISBURSEMENT-GUARD-ENTRY-ROOTS",
            &self
                .disbursement_entry_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "disbursement_entry_roots": self.disbursement_entry_roots(),
            "disbursement_entries_root": self.disbursement_entries_root(),
            "summary": self.summary.public_record(),
            "roots_only": true,
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Some(map) = record.as_object_mut() {
            map.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    State::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn wallet_watchtower_release_execution_disbursement_liquidity_throttle_guard_runtime() -> Runtime
{
    devnet()
}

fn disbursement_blockers(
    entry: &DisbursementGuardEntry,
    config: &Config,
) -> Vec<DisbursementBlocker> {
    let mut blockers = Vec::new();
    if config.require_wave100_settlement_notary_roots
        && entry.wave100_settlement_notary_roots.len()
            < config.min_wave100_settlement_notary_roots as usize
    {
        blockers.push(DisbursementBlocker::Wave100SettlementNotaryRootMissing);
    }
    if config.require_liquidity_reservation_roots
        && entry.liquidity_reservation_roots.len() < config.min_liquidity_reservation_roots as usize
    {
        blockers.push(DisbursementBlocker::LiquidityReservationRootMissing);
    }
    if config.require_wallet_fee_netting_roots
        && entry.wallet_fee_netting_roots.len() < config.min_wallet_fee_netting_roots as usize
    {
        blockers.push(DisbursementBlocker::WalletFeeNettingRootMissing);
    }
    if config.require_payout_throttle_roots
        && entry.payout_throttle_roots.len() < config.min_payout_throttle_roots as usize
    {
        blockers.push(DisbursementBlocker::PayoutThrottleRootMissing);
    }
    if config.require_privacy_budget_roots
        && entry.privacy_budget_roots.len() < config.min_privacy_budget_roots as usize
    {
        blockers.push(DisbursementBlocker::PrivacyBudgetRootMissing);
    }
    if config.require_watchtower_observation_roots
        && entry.watchtower_observation_roots.len()
            < config.min_watchtower_observation_roots as usize
    {
        blockers.push(DisbursementBlocker::WatchtowerObservationRootMissing);
    }
    if config.require_circuit_breaker_roots
        && entry.circuit_breaker_roots.len() < config.min_circuit_breaker_roots as usize
    {
        blockers.push(DisbursementBlocker::CircuitBreakerRootMissing);
    }
    if config.require_operator_signoff_roots
        && entry.operator_signoff_roots.len() < config.min_operator_signoff_roots as usize
    {
        blockers.push(DisbursementBlocker::OperatorSignoffRootMissing);
    }
    if config.require_reviewer_signoff_roots
        && entry.reviewer_signoff_roots.len() < config.min_reviewer_signoff_roots as usize
    {
        blockers.push(DisbursementBlocker::ReviewerSignoffRootMissing);
    }
    if has_duplicate(&entry.wave100_settlement_notary_roots) {
        blockers.push(DisbursementBlocker::DuplicateSettlementNotaryRoot);
    }
    if has_duplicate(&entry.liquidity_reservation_roots) {
        blockers.push(DisbursementBlocker::DuplicateLiquidityReservationRoot);
    }
    if !entry.liquidity_reservation_roots.is_empty()
        && entry.wallet_fee_netting_roots.len() < entry.liquidity_reservation_roots.len()
    {
        blockers.push(DisbursementBlocker::FeeNettingLaggingReservationRoot);
    }
    if !entry.wallet_fee_netting_roots.is_empty()
        && entry.payout_throttle_roots.len() < entry.wallet_fee_netting_roots.len()
    {
        blockers.push(DisbursementBlocker::ThrottleLaggingFeeNettingRoot);
    }
    if !entry.payout_throttle_roots.is_empty()
        && entry.privacy_budget_roots.len() < entry.payout_throttle_roots.len()
    {
        blockers.push(DisbursementBlocker::PrivacyBudgetLaggingThrottleRoot);
    }
    if !entry.liquidity_reservation_roots.is_empty()
        && entry.watchtower_observation_roots.len() < entry.liquidity_reservation_roots.len()
    {
        blockers.push(DisbursementBlocker::ObservationLaggingReservationRoot);
    }
    if config.circuit_breaker_active {
        blockers.push(DisbursementBlocker::CircuitBreakerActive);
    }
    if !roots_shape_valid(entry) {
        blockers.push(DisbursementBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(entry) {
        blockers.push(DisbursementBlocker::RootsOnlyRecordMissing);
    }
    if config.wallet_liquidity_blocker_active {
        blockers.push(DisbursementBlocker::WalletLiquidityBlockerActive);
    }
    if config.watchtower_liquidity_blocker_active {
        blockers.push(DisbursementBlocker::WatchtowerLiquidityBlockerActive);
    }
    if config.payout_throttle_blocker_active {
        blockers.push(DisbursementBlocker::PayoutThrottleBlockerActive);
    }
    if config.privacy_budget_blocker_active {
        blockers.push(DisbursementBlocker::PrivacyBudgetBlockerActive);
    }
    if !config.disbursement_reservations_enabled {
        blockers.push(DisbursementBlocker::ReservationsDisabled);
    }
    if !config.release_execution_enabled {
        blockers.push(DisbursementBlocker::ExecutionDisabled);
    }
    if !config.heavy_gates_ran {
        blockers.push(DisbursementBlocker::HeavyGatesNotRun);
    }
    blockers
}

fn command_for_entry(entry: &DisbursementGuardEntry) -> CommandHintKind {
    if entry.blockers.is_empty() {
        return CommandHintKind::DisburseAfterLiquidityThrottleGuard;
    }
    match entry.blockers[0] {
        DisbursementBlocker::Wave100SettlementNotaryRootMissing => {
            CommandHintKind::ImportWave100SettlementNotaryRoot
        }
        DisbursementBlocker::LiquidityReservationRootMissing => {
            CommandHintKind::ImportLiquidityReservationRoot
        }
        DisbursementBlocker::WalletFeeNettingRootMissing => {
            CommandHintKind::ImportWalletFeeNettingRoot
        }
        DisbursementBlocker::PayoutThrottleRootMissing => CommandHintKind::ImportPayoutThrottleRoot,
        DisbursementBlocker::PrivacyBudgetRootMissing => CommandHintKind::ImportPrivacyBudgetRoot,
        DisbursementBlocker::WatchtowerObservationRootMissing => {
            CommandHintKind::ImportWatchtowerObservationRoot
        }
        DisbursementBlocker::CircuitBreakerRootMissing => CommandHintKind::ImportCircuitBreakerRoot,
        DisbursementBlocker::OperatorSignoffRootMissing => {
            CommandHintKind::ImportOperatorSignoffRoot
        }
        DisbursementBlocker::ReviewerSignoffRootMissing => {
            CommandHintKind::ImportReviewerSignoffRoot
        }
        DisbursementBlocker::DuplicateSettlementNotaryRoot => {
            CommandHintKind::ResolveDuplicateSettlementNotaryRoot
        }
        DisbursementBlocker::DuplicateLiquidityReservationRoot => {
            CommandHintKind::ResolveDuplicateLiquidityReservationRoot
        }
        DisbursementBlocker::FeeNettingLaggingReservationRoot => {
            CommandHintKind::ReconcileFeeNettingRoot
        }
        DisbursementBlocker::ThrottleLaggingFeeNettingRoot => {
            CommandHintKind::ReconcilePayoutThrottleRoot
        }
        DisbursementBlocker::PrivacyBudgetLaggingThrottleRoot => {
            CommandHintKind::ReconcilePrivacyBudgetRoot
        }
        DisbursementBlocker::ObservationLaggingReservationRoot => {
            CommandHintKind::ReconcileWatchtowerObservationRoot
        }
        DisbursementBlocker::CircuitBreakerActive => CommandHintKind::ClearCircuitBreaker,
        DisbursementBlocker::WalletLiquidityBlockerActive
        | DisbursementBlocker::WatchtowerLiquidityBlockerActive
        | DisbursementBlocker::PayoutThrottleBlockerActive
        | DisbursementBlocker::PrivacyBudgetBlockerActive => CommandHintKind::MaintainLiquidityHold,
        DisbursementBlocker::RootShapeInvalid
        | DisbursementBlocker::RootsOnlyRecordMissing
        | DisbursementBlocker::ReservationsDisabled
        | DisbursementBlocker::ExecutionDisabled
        | DisbursementBlocker::HeavyGatesNotRun => CommandHintKind::HoldDisbursement,
    }
}

fn roots_only_record_present(entry: &DisbursementGuardEntry) -> bool {
    is_root_like(&entry.wave100_settlement_notary_root)
        && is_root_like(&entry.liquidity_reservation_root)
        && is_root_like(&entry.wallet_fee_netting_root)
        && is_root_like(&entry.payout_throttle_root)
        && is_root_like(&entry.privacy_budget_root)
        && is_root_like(&entry.watchtower_observation_root)
        && is_root_like(&entry.circuit_breaker_root)
        && is_root_like(&entry.operator_signoff_root)
        && is_root_like(&entry.reviewer_signoff_root)
        && is_root_like(&entry.disbursement_liquidity_throttle_guard_root)
        && is_root_like(&entry.deterministic_disbursement_root)
}

fn roots_shape_valid(entry: &DisbursementGuardEntry) -> bool {
    all_roots_like(&entry.wave100_settlement_notary_roots)
        && all_roots_like(&entry.liquidity_reservation_roots)
        && all_roots_like(&entry.wallet_fee_netting_roots)
        && all_roots_like(&entry.payout_throttle_roots)
        && all_roots_like(&entry.privacy_budget_roots)
        && all_roots_like(&entry.watchtower_observation_roots)
        && all_roots_like(&entry.circuit_breaker_roots)
        && all_roots_like(&entry.operator_signoff_roots)
        && all_roots_like(&entry.reviewer_signoff_roots)
}

fn all_roots_like(roots: &[String]) -> bool {
    roots.iter().all(|root| is_root_like(root))
}

fn has_duplicate(roots: &[String]) -> bool {
    let mut seen = BTreeSet::new();
    roots.iter().any(|root| !seen.insert(root))
}

fn disbursement_liquidity_throttle_guard_root(entry: &DisbursementGuardEntry) -> String {
    record_root(
        "disbursement-liquidity-throttle-guard-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "wave100_settlement_notary_root": entry.wave100_settlement_notary_root,
            "liquidity_reservation_root": entry.liquidity_reservation_root,
            "wallet_fee_netting_root": entry.wallet_fee_netting_root,
            "payout_throttle_root": entry.payout_throttle_root,
            "privacy_budget_root": entry.privacy_budget_root,
            "watchtower_observation_root": entry.watchtower_observation_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "wallet_material_absent": true,
            "watchtower_material_absent": true,
            "liquidity_material_absent": true,
            "roots_only": true,
        }),
    )
}

fn deterministic_disbursement_root(entry: &DisbursementGuardEntry) -> String {
    record_root(
        "deterministic-disbursement-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "disbursement_liquidity_throttle_guard_root": entry.disbursement_liquidity_throttle_guard_root,
            "wave100_settlement_notary_root": entry.wave100_settlement_notary_root,
            "liquidity_reservation_root": entry.liquidity_reservation_root,
            "wallet_fee_netting_root": entry.wallet_fee_netting_root,
            "payout_throttle_root": entry.payout_throttle_root,
            "privacy_budget_root": entry.privacy_budget_root,
            "watchtower_observation_root": entry.watchtower_observation_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "raw_transfer_material_absent": true,
            "raw_wallet_material_absent": true,
            "raw_watchtower_material_absent": true,
            "roots_only": true,
        }),
    )
}

fn aggregate_root(domain: &str, roots: &[String]) -> String {
    if roots.is_empty() {
        return empty_root(domain);
    }
    merkle_root(
        "WAVE101-WALLET-WATCHTOWER-ROOT-AGGREGATE",
        &roots
            .iter()
            .cloned()
            .map(|root| {
                json!({
                    "domain": domain,
                    "root": root,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn entry_field_root<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    merkle_root(
        domain,
        &values.into_iter().map(Value::String).collect::<Vec<_>>(),
    )
}

fn blockers_root(domain: &str, blockers: &[DisbursementBlocker]) -> String {
    merkle_root(
        domain,
        &blockers
            .iter()
            .map(|blocker| Value::String(blocker.as_str().to_string()))
            .collect::<Vec<_>>(),
    )
}

fn empty_root(marker_name: &str) -> String {
    let root = record_root(
        "empty-disbursement-liquidity-throttle-guard-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "marker_name": marker_name,
        }),
    );
    format!("{EMPTY_ROOT_MARKER}:{root}")
}

fn is_root_like(root: &str) -> bool {
    !root.is_empty()
        && root.len() >= 16
        && root
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ':' | '-' | '_' | '.'))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    domain_hash(
        "WAVE101-WALLET-WATCHTOWER-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
