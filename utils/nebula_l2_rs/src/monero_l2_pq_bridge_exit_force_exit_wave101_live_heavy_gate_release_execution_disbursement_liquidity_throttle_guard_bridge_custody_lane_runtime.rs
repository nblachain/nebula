use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave101-release-execution-disbursement-liquidity-throttle-guard-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE100_SETTLEMENT_NOTARY_ROOT: &str =
    "root:wave100:bridge-custody-release-execution-settlement-notary-guard-blocked-placeholder";
pub const DEFAULT_MIN_SETTLEMENT_NOTARY_ROOTS: u16 = 5;
pub const DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS: u16 = 4;
pub const DEFAULT_MIN_FEE_NETTING_ROOTS: u16 = 4;
pub const DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS: u16 = 4;
pub const DEFAULT_MIN_RESERVE_BUFFER_ROOTS: u16 = 3;
pub const DEFAULT_MIN_SIGNOFF_ROOTS: u16 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave100_settlement_notary_root: String,
    pub min_settlement_notary_roots: u16,
    pub min_liquidity_reservation_roots: u16,
    pub min_fee_netting_roots: u16,
    pub min_payout_throttle_roots: u16,
    pub min_reserve_buffer_roots: u16,
    pub min_signoff_roots: u16,
    pub roots_only_public_records: bool,
    pub disbursement_reservations_enabled: bool,
    pub release_execution_enabled: bool,
    pub custody_liquidity_blockers_active: bool,
    pub payout_throttle_blockers_active: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave100_settlement_notary_root: WAVE100_SETTLEMENT_NOTARY_ROOT.to_string(),
            min_settlement_notary_roots: DEFAULT_MIN_SETTLEMENT_NOTARY_ROOTS,
            min_liquidity_reservation_roots: DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS,
            min_fee_netting_roots: DEFAULT_MIN_FEE_NETTING_ROOTS,
            min_payout_throttle_roots: DEFAULT_MIN_PAYOUT_THROTTLE_ROOTS,
            min_reserve_buffer_roots: DEFAULT_MIN_RESERVE_BUFFER_ROOTS,
            min_signoff_roots: DEFAULT_MIN_SIGNOFF_ROOTS,
            roots_only_public_records: true,
            disbursement_reservations_enabled: false,
            release_execution_enabled: false,
            custody_liquidity_blockers_active: true,
            payout_throttle_blockers_active: true,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_root_like(
            "wave100_settlement_notary_root",
            &self.wave100_settlement_notary_root,
        )?;
        ensure_nonzero(
            "settlement notary root quorum",
            self.min_settlement_notary_roots,
        )?;
        ensure_nonzero(
            "liquidity reservation root quorum",
            self.min_liquidity_reservation_roots,
        )?;
        ensure_nonzero("fee netting root quorum", self.min_fee_netting_roots)?;
        ensure_nonzero(
            "payout throttle root quorum",
            self.min_payout_throttle_roots,
        )?;
        ensure_nonzero("reserve buffer root quorum", self.min_reserve_buffer_roots)?;
        ensure_nonzero("signoff root quorum", self.min_signoff_roots)?;
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.disbursement_reservations_enabled || self.release_execution_enabled {
            return Err("devnet disbursement and release execution must remain denied".to_string());
        }
        if !self.custody_liquidity_blockers_active || !self.payout_throttle_blockers_active {
            return Err("devnet liquidity and throttle blockers must remain active".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave101 must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_root": deterministic_field_root("chain", &self.chain_id),
            "lane_root": deterministic_field_root("lane", &self.lane_id),
            "protocol_root": deterministic_field_root("protocol", &self.protocol_version),
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "min_settlement_notary_roots": self.min_settlement_notary_roots,
            "min_liquidity_reservation_roots": self.min_liquidity_reservation_roots,
            "min_fee_netting_roots": self.min_fee_netting_roots,
            "min_payout_throttle_roots": self.min_payout_throttle_roots,
            "min_reserve_buffer_roots": self.min_reserve_buffer_roots,
            "min_signoff_roots": self.min_signoff_roots,
            "roots_only_public_records": self.roots_only_public_records,
            "disbursement_reservations_enabled": self.disbursement_reservations_enabled,
            "release_execution_enabled": self.release_execution_enabled,
            "custody_liquidity_blockers_active": self.custody_liquidity_blockers_active,
            "payout_throttle_blockers_active": self.payout_throttle_blockers_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RootKind {
    Wave100SettlementNotary,
    LiquidityReservation,
    CustodyFeeNetting,
    PayoutThrottle,
    ReserveBuffer,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    CommandHint,
    DeterministicDisbursementGuard,
}

impl RootKind {
    pub fn all() -> [Self; 10] {
        [
            Self::Wave100SettlementNotary,
            Self::LiquidityReservation,
            Self::CustodyFeeNetting,
            Self::PayoutThrottle,
            Self::ReserveBuffer,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::CommandHint,
            Self::DeterministicDisbursementGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotary => "wave100_settlement_notary",
            Self::LiquidityReservation => "liquidity_reservation",
            Self::CustodyFeeNetting => "custody_fee_netting",
            Self::PayoutThrottle => "payout_throttle",
            Self::ReserveBuffer => "reserve_buffer",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::CommandHint => "command_hint",
            Self::DeterministicDisbursementGuard => "deterministic_disbursement_guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave100SettlementNotaryActive,
    LiquidityReservationMissing,
    LiquidityReservationClosed,
    CustodyFeeNettingUnbalanced,
    PayoutThrottleActive,
    ReserveBufferBelowFloor,
    CircuitBreakerArmed,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    DisbursementReservationDisabled,
    ReleaseExecutionDisabled,
    HeavyGateMissing,
}

impl BlockerKind {
    pub fn all() -> [Self; 12] {
        [
            Self::Wave100SettlementNotaryActive,
            Self::LiquidityReservationMissing,
            Self::LiquidityReservationClosed,
            Self::CustodyFeeNettingUnbalanced,
            Self::PayoutThrottleActive,
            Self::ReserveBufferBelowFloor,
            Self::CircuitBreakerArmed,
            Self::OperatorSignoffMissing,
            Self::ReviewerSignoffMissing,
            Self::DisbursementReservationDisabled,
            Self::ReleaseExecutionDisabled,
            Self::HeavyGateMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotaryActive => "wave100_settlement_notary_active",
            Self::LiquidityReservationMissing => "liquidity_reservation_missing",
            Self::LiquidityReservationClosed => "liquidity_reservation_closed",
            Self::CustodyFeeNettingUnbalanced => "custody_fee_netting_unbalanced",
            Self::PayoutThrottleActive => "payout_throttle_active",
            Self::ReserveBufferBelowFloor => "reserve_buffer_below_floor",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::DisbursementReservationDisabled => "disbursement_reservation_disabled",
            Self::ReleaseExecutionDisabled => "release_execution_disabled",
            Self::HeavyGateMissing => "heavy_gate_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepReleaseExecutionDisabled,
    KeepDisbursementReservationsClosed,
    KeepCustodyLiquidityBlockersActive,
    KeepPayoutThrottleBlockersActive,
    RequireWave100SettlementNotaryRoots,
    ReconcileLiquidityReservationRoots,
    ReconcileCustodyFeeNettingRoots,
    ReviewPayoutThrottleRoots,
    HoldReserveBufferRoots,
    KeepCircuitBreakersArmed,
    ReviewOperatorSignoffs,
    ReviewReviewerSignoffs,
}

impl CommandHintKind {
    pub fn all() -> [Self; 12] {
        [
            Self::KeepReleaseExecutionDisabled,
            Self::KeepDisbursementReservationsClosed,
            Self::KeepCustodyLiquidityBlockersActive,
            Self::KeepPayoutThrottleBlockersActive,
            Self::RequireWave100SettlementNotaryRoots,
            Self::ReconcileLiquidityReservationRoots,
            Self::ReconcileCustodyFeeNettingRoots,
            Self::ReviewPayoutThrottleRoots,
            Self::HoldReserveBufferRoots,
            Self::KeepCircuitBreakersArmed,
            Self::ReviewOperatorSignoffs,
            Self::ReviewReviewerSignoffs,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseExecutionDisabled => "keep_release_execution_disabled",
            Self::KeepDisbursementReservationsClosed => "keep_disbursement_reservations_closed",
            Self::KeepCustodyLiquidityBlockersActive => "keep_custody_liquidity_blockers_active",
            Self::KeepPayoutThrottleBlockersActive => "keep_payout_throttle_blockers_active",
            Self::RequireWave100SettlementNotaryRoots => "require_wave100_settlement_notary_roots",
            Self::ReconcileLiquidityReservationRoots => "reconcile_liquidity_reservation_roots",
            Self::ReconcileCustodyFeeNettingRoots => "reconcile_custody_fee_netting_roots",
            Self::ReviewPayoutThrottleRoots => "review_payout_throttle_roots",
            Self::HoldReserveBufferRoots => "hold_reserve_buffer_roots",
            Self::KeepCircuitBreakersArmed => "keep_circuit_breakers_armed",
            Self::ReviewOperatorSignoffs => "review_operator_signoffs",
            Self::ReviewReviewerSignoffs => "review_reviewer_signoffs",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootEntry {
    pub kind: RootKind,
    pub root: String,
    pub blocker_active: bool,
    pub required_for_disbursement: bool,
}

impl RootEntry {
    pub fn devnet(kind: RootKind, config: &Config) -> Self {
        let root = match kind {
            RootKind::Wave100SettlementNotary => config.wave100_settlement_notary_root.clone(),
            _ => deterministic_root(kind.as_str()),
        };
        Self {
            kind,
            root,
            blocker_active: kind != RootKind::CommandHint
                && kind != RootKind::DeterministicDisbursementGuard,
            required_for_disbursement: kind != RootKind::CommandHint,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "root": self.root,
            "blocker_active": self.blocker_active,
            "required_for_disbursement": self.required_for_disbursement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("root-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Blocker {
    pub kind: BlockerKind,
    pub reason_root: String,
    pub active: bool,
}

impl Blocker {
    pub fn devnet(kind: BlockerKind) -> Self {
        Self {
            kind,
            reason_root: deterministic_root(kind.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "reason_root": self.reason_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub hint_root: String,
    pub active: bool,
}

impl CommandHint {
    pub fn devnet(kind: CommandHintKind) -> Self {
        Self {
            kind,
            hint_root: deterministic_root(kind.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "hint_root": self.hint_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("command-hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisbursementDecision {
    pub wave100_settlement_notary_root: String,
    pub liquidity_reservation_root: String,
    pub custody_fee_netting_root: String,
    pub payout_throttle_root: String,
    pub reserve_buffer_root: String,
    pub circuit_breaker_root: String,
    pub signoff_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub disbursement_reservations_allowed: bool,
    pub release_execution_allowed: bool,
}

impl DisbursementDecision {
    pub fn from_parts(
        entries: &BTreeMap<String, RootEntry>,
        blockers: &[Blocker],
        command_hints: &[CommandHint],
    ) -> Self {
        Self {
            wave100_settlement_notary_root: root_for_entry(
                entries,
                RootKind::Wave100SettlementNotary,
            ),
            liquidity_reservation_root: root_for_entry(entries, RootKind::LiquidityReservation),
            custody_fee_netting_root: root_for_entry(entries, RootKind::CustodyFeeNetting),
            payout_throttle_root: root_for_entry(entries, RootKind::PayoutThrottle),
            reserve_buffer_root: root_for_entry(entries, RootKind::ReserveBuffer),
            circuit_breaker_root: root_for_entry(entries, RootKind::CircuitBreaker),
            signoff_root: list_root(
                "signoff-roots",
                vec![
                    root_for_entry(entries, RootKind::OperatorSignoff),
                    root_for_entry(entries, RootKind::ReviewerSignoff),
                ],
            ),
            blocker_root: blocker_root(blockers),
            command_hint_root: command_hint_root(command_hints),
            disbursement_reservations_allowed: false,
            release_execution_allowed: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "custody_fee_netting_root": self.custody_fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "reserve_buffer_root": self.reserve_buffer_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "signoff_root": self.signoff_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "disbursement_reservations_allowed": self.disbursement_reservations_allowed,
            "release_execution_allowed": self.release_execution_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement-decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: BTreeMap<String, RootEntry>,
    pub blockers: Vec<Blocker>,
    pub command_hints: Vec<CommandHint>,
    pub decision: DisbursementDecision,
}

impl State {
    pub fn devnet() -> Self {
        Self::with_config(Config::devnet())
    }

    pub fn with_config(config: Config) -> Self {
        let roots = RootKind::all()
            .iter()
            .map(|kind| {
                let entry = RootEntry::devnet(*kind, &config);
                (kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let blockers = BlockerKind::all()
            .iter()
            .map(|kind| Blocker::devnet(*kind))
            .collect::<Vec<_>>();
        let command_hints = CommandHintKind::all()
            .iter()
            .map(|kind| CommandHint::devnet(*kind))
            .collect::<Vec<_>>();
        let decision = DisbursementDecision::from_parts(&roots, &blockers, &command_hints);
        Self {
            config,
            roots,
            blockers,
            command_hints,
            decision,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.decision.disbursement_reservations_allowed
            || self.decision.release_execution_allowed
        {
            return Err("liquidity throttle guard must deny disbursement on devnet".to_string());
        }
        if self.blockers.iter().any(|blocker| !blocker.active) {
            return Err("devnet blockers must remain active".to_string());
        }
        if self.command_hints.iter().any(|hint| !hint.active) {
            return Err("devnet command hints must remain active".to_string());
        }
        for entry in self.roots.values() {
            ensure_root_like(entry.kind.as_str(), &entry.root)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        let root_records = self
            .roots
            .iter()
            .map(|(key, entry)| (key.clone(), entry.public_record()))
            .collect::<BTreeMap<_, _>>();
        let blocker_roots = self
            .blockers
            .iter()
            .map(Blocker::state_root)
            .collect::<Vec<_>>();
        let command_hint_roots = self
            .command_hints
            .iter()
            .map(CommandHint::state_root)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.root(),
            "root_entry_records": root_records,
            "root_entries_root": self.root_entries_root(),
            "blocker_roots": blocker_roots,
            "blockers_root": self.blockers_root(),
            "command_hint_roots": command_hint_roots,
            "command_hints_root": self.command_hints_root(),
            "decision_root": self.decision.state_root(),
            "decision": self.decision.public_record(),
            "state_root": self.state_root(),
        })
    }

    pub fn root_entries_root(&self) -> String {
        list_root(
            "root-entries",
            self.roots
                .values()
                .map(RootEntry::state_root)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blockers_root(&self) -> String {
        blocker_root(&self.blockers)
    }

    pub fn command_hints_root(&self) -> String {
        command_hint_root(&self.command_hints)
    }

    pub fn state_root(&self) -> String {
        record_root(
            "state",
            &json!({
                "config_root": self.config.root(),
                "root_entries_root": self.root_entries_root(),
                "blockers_root": self.blockers_root(),
                "command_hints_root": self.command_hints_root(),
                "decision_root": self.decision.state_root(),
            }),
        )
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn root_for_entry(entries: &BTreeMap<String, RootEntry>, kind: RootKind) -> String {
    match entries.get(kind.as_str()) {
        Some(entry) if !entry.root.is_empty() => entry.root.clone(),
        _ => deterministic_root(kind.as_str()),
    }
}

fn blocker_root(blockers: &[Blocker]) -> String {
    list_root(
        "blockers",
        blockers.iter().map(Blocker::state_root).collect::<Vec<_>>(),
    )
}

fn command_hint_root(command_hints: &[CommandHint]) -> String {
    list_root(
        "command-hints",
        command_hints
            .iter()
            .map(CommandHint::state_root)
            .collect::<Vec<_>>(),
    )
}

fn record_root(label: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE101-BRIDGE-CUSTODY-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD",
        &[
            HashPart::Str(label),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave101:{hash}")
}

fn list_root(label: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave101:{}", merkle_root(label, &leaves))
}

fn deterministic_root(label: &str) -> String {
    let hash = domain_hash(
        "WAVE101-BRIDGE-CUSTODY-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-DETERMINISTIC",
        &[
            HashPart::Str(DEVNET_CHAIN_ID),
            HashPart::Str(DEVNET_LANE_ID),
            HashPart::Str(label),
        ],
        32,
    );
    format!("root:wave101:{hash}")
}

fn deterministic_field_root(field: &str, value: &str) -> String {
    let hash = domain_hash(
        "WAVE101-BRIDGE-CUSTODY-RELEASE-EXECUTION-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD-FIELD",
        &[HashPart::Str(field), HashPart::Str(value)],
        32,
    );
    format!("root:wave101:{hash}")
}

fn ensure_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{field} must be present"));
    }
    Ok(())
}

fn ensure_root_like(field: &str, value: &str) -> Result<()> {
    ensure_text(field, value)?;
    if value.len() < 16 {
        return Err(format!("{field} must be a root identifier"));
    }
    Ok(())
}

fn ensure_nonzero(label: &str, value: u16) -> Result<()> {
    if value == 0 {
        return Err(format!("{label} must be nonzero"));
    }
    Ok(())
}
