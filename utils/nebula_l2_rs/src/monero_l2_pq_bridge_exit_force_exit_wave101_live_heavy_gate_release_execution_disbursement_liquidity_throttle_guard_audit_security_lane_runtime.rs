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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const THROTTLE_SUITE: &str =
    "wave101-wave100-settlement-notary-to-disbursement-liquidity-throttle-guard-v1";
pub const DEFAULT_WAVE: u64 = 101;
pub const DEFAULT_SOURCE_WAVE: u64 = 100;
pub const DEFAULT_HEIGHT: u64 = 4_282_901;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS: u64 = 6;
pub const DEFAULT_MAX_DISBURSEMENT_RESERVATIONS: u64 = 0;
pub const DEFAULT_MAX_RELEASE_EXECUTIONS: u64 = 0;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneSlotKind {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
}

impl LaneSlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialScenario => "adversarial_scenario",
            Self::ThreatModel => "threat_model",
            Self::PrivacyReview => "privacy_review",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditReview,
            Self::AdversarialScenario,
            Self::ThreatModel,
            Self::PrivacyReview,
            Self::ReviewerSignoff,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThrottleStatus {
    Wave100SettlementNotaryActive,
    LiquidityReservationMissing,
    AuditFeeNettingOpen,
    PayoutThrottleArmed,
    PrivacyBudgetBlocked,
    DisbursementDenied,
}

impl ThrottleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotaryActive => "wave100_settlement_notary_active",
            Self::LiquidityReservationMissing => "liquidity_reservation_missing",
            Self::AuditFeeNettingOpen => "audit_fee_netting_open",
            Self::PayoutThrottleArmed => "payout_throttle_armed",
            Self::PrivacyBudgetBlocked => "privacy_budget_blocked",
            Self::DisbursementDenied => "disbursement_denied",
        }
    }

    pub fn blocks_disbursement(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave100SettlementNotaryActive,
    SettlementExecutionRootMissing,
    LiquidityReservationMissing,
    AuditFeeNettingOpen,
    PayoutThrottleArmed,
    PrivacyBudgetBlocked,
    CircuitBreakerActive,
    ReviewerSignoffMissing,
    OperatorSignoffMissing,
    DisbursementReservationBudgetZero,
    ReleaseExecutionBudgetZero,
    ReleaseExecutionDenied,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave100SettlementNotaryActive => "wave100_settlement_notary_active",
            Self::SettlementExecutionRootMissing => "settlement_execution_root_missing",
            Self::LiquidityReservationMissing => "liquidity_reservation_missing",
            Self::AuditFeeNettingOpen => "audit_fee_netting_open",
            Self::PayoutThrottleArmed => "payout_throttle_armed",
            Self::PrivacyBudgetBlocked => "privacy_budget_blocked",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::DisbursementReservationBudgetZero => "disbursement_reservation_budget_zero",
            Self::ReleaseExecutionBudgetZero => "release_execution_budget_zero",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::ProductionDenied => "production_denied",
            Self::FailClosedDisarmed => "fail_closed_disarmed",
            Self::RawPayloadPresent => "raw_payload_present",
            Self::HeavyGateRunClaimed => "heavy_gate_run_claimed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldReleaseExecution,
    KeepFailClosed,
    ImportWave100SettlementNotaryRoots,
    AttachLiquidityReservationRoot,
    AttachAuditFeeNettingRoot,
    KeepPayoutThrottleArmed,
    KeepPrivacyBudgetBlocked,
    KeepCircuitBreakerArmed,
    RequireReviewerSignoffRoot,
    RequireOperatorSignoffRoot,
    DenyDisbursementUntilBlockersClear,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldReleaseExecution => "hold_release_execution",
            Self::KeepFailClosed => "keep_fail_closed",
            Self::ImportWave100SettlementNotaryRoots => "import_wave100_settlement_notary_roots",
            Self::AttachLiquidityReservationRoot => "attach_liquidity_reservation_root",
            Self::AttachAuditFeeNettingRoot => "attach_audit_fee_netting_root",
            Self::KeepPayoutThrottleArmed => "keep_payout_throttle_armed",
            Self::KeepPrivacyBudgetBlocked => "keep_privacy_budget_blocked",
            Self::KeepCircuitBreakerArmed => "keep_circuit_breaker_armed",
            Self::RequireReviewerSignoffRoot => "require_reviewer_signoff_root",
            Self::RequireOperatorSignoffRoot => "require_operator_signoff_root",
            Self::DenyDisbursementUntilBlockersClear => "deny_disbursement_until_blockers_clear",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub throttle_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub min_liquidity_reservation_roots: u64,
    pub max_disbursement_reservations: u64,
    pub max_release_executions: u64,
    pub max_raw_payload_records: u64,
    pub wave100_settlement_notary_root: String,
    pub wave100_execution_bundle_root: String,
    pub wave100_settlement_accounting_root: String,
    pub wave100_notary_quorum_root: String,
    pub wave100_payout_envelope_root: String,
    pub wave100_circuit_breaker_root: String,
    pub wave100_signoff_root: String,
    pub fail_closed_armed: bool,
    pub liquidity_blockers_active: bool,
    pub throttle_blockers_active: bool,
    pub circuit_breaker_armed: bool,
    pub release_execution_allowed: bool,
    pub disbursement_allowed: bool,
    pub production_allowed: bool,
    pub heavy_gates_ran: bool,
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
            throttle_suite: THROTTLE_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            min_liquidity_reservation_roots: DEFAULT_MIN_LIQUIDITY_RESERVATION_ROOTS,
            max_disbursement_reservations: DEFAULT_MAX_DISBURSEMENT_RESERVATIONS,
            max_release_executions: DEFAULT_MAX_RELEASE_EXECUTIONS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            wave100_settlement_notary_root: deterministic_root("wave100-settlement-notary-root"),
            wave100_execution_bundle_root: deterministic_root("wave100-execution-bundle-root"),
            wave100_settlement_accounting_root: deterministic_root(
                "wave100-settlement-accounting-root",
            ),
            wave100_notary_quorum_root: deterministic_root("wave100-notary-quorum-root"),
            wave100_payout_envelope_root: deterministic_root("wave100-payout-envelope-root"),
            wave100_circuit_breaker_root: deterministic_root("wave100-circuit-breaker-root"),
            wave100_signoff_root: deterministic_root("wave100-signoff-root"),
            fail_closed_armed: true,
            liquidity_blockers_active: true,
            throttle_blockers_active: true,
            circuit_breaker_armed: true,
            release_execution_allowed: false,
            disbursement_allowed: false,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("throttle_suite", &self.throttle_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_positive(
            "min_liquidity_reservation_roots",
            self.min_liquidity_reservation_roots,
        )?;
        ensure_root(
            "wave100_settlement_notary_root",
            &self.wave100_settlement_notary_root,
        )?;
        ensure_root(
            "wave100_execution_bundle_root",
            &self.wave100_execution_bundle_root,
        )?;
        ensure_root(
            "wave100_settlement_accounting_root",
            &self.wave100_settlement_accounting_root,
        )?;
        ensure_root(
            "wave100_notary_quorum_root",
            &self.wave100_notary_quorum_root,
        )?;
        ensure_root(
            "wave100_payout_envelope_root",
            &self.wave100_payout_envelope_root,
        )?;
        ensure_root(
            "wave100_circuit_breaker_root",
            &self.wave100_circuit_breaker_root,
        )?;
        ensure_root("wave100_signoff_root", &self.wave100_signoff_root)?;
        if !self.fail_closed_armed {
            return Err("wave101 liquidity throttle guard fail closed is not armed".to_string());
        }
        if !self.liquidity_blockers_active {
            return Err("wave101 liquidity blockers must remain active in devnet".to_string());
        }
        if !self.throttle_blockers_active {
            return Err(
                "wave101 payout throttle blockers must remain active in devnet".to_string(),
            );
        }
        if !self.circuit_breaker_armed {
            return Err("wave101 circuit breaker must remain armed in devnet".to_string());
        }
        if self.release_execution_allowed {
            return Err("wave101 release execution is denied by default".to_string());
        }
        if self.disbursement_allowed {
            return Err("wave101 disbursement is denied by default".to_string());
        }
        if self.production_allowed {
            return Err("wave101 production is denied by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave101 throttle guard cannot claim gate execution".to_string());
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
            "throttle_suite": self.throttle_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "min_liquidity_reservation_roots": self.min_liquidity_reservation_roots,
            "max_disbursement_reservations": self.max_disbursement_reservations,
            "max_release_executions": self.max_release_executions,
            "max_raw_payload_records": self.max_raw_payload_records,
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "wave100_execution_bundle_root": self.wave100_execution_bundle_root,
            "wave100_settlement_accounting_root": self.wave100_settlement_accounting_root,
            "wave100_notary_quorum_root": self.wave100_notary_quorum_root,
            "wave100_payout_envelope_root": self.wave100_payout_envelope_root,
            "wave100_circuit_breaker_root": self.wave100_circuit_breaker_root,
            "wave100_signoff_root": self.wave100_signoff_root,
            "fail_closed_armed": self.fail_closed_armed,
            "liquidity_blockers_active": self.liquidity_blockers_active,
            "throttle_blockers_active": self.throttle_blockers_active,
            "circuit_breaker_armed": self.circuit_breaker_armed,
            "release_execution_allowed": self.release_execution_allowed,
            "disbursement_allowed": self.disbursement_allowed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE101-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ThrottleGuard {
    pub slot_kind: LaneSlotKind,
    pub wave100_settlement_notary_root: String,
    pub liquidity_reservation_root: String,
    pub audit_fee_netting_root: String,
    pub payout_throttle_root: String,
    pub privacy_budget_root: String,
    pub circuit_breaker_root: String,
    pub reviewer_signoff_root: String,
    pub operator_signoff_root: String,
    pub throttle_guard_root: String,
    pub status: ThrottleStatus,
}

impl ThrottleGuard {
    pub fn blocked(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let wave100_settlement_notary_root = bind_root(
            "wave100-settlement-notary",
            slot,
            &config.wave100_settlement_notary_root,
        );
        let liquidity_reservation_root = value_root(
            "WAVE101-LIQUIDITY-RESERVATION",
            &json!({
                "slot_kind": slot,
                "source_execution_bundle_root": config.wave100_execution_bundle_root,
                "source_settlement_accounting_root": config.wave100_settlement_accounting_root,
                "reservation_present": false,
                "reserved_disbursement_units": 0,
            }),
        );
        let audit_fee_netting_root = value_root(
            "WAVE101-AUDIT-FEE-NETTING",
            &json!({
                "slot_kind": slot,
                "source_notary_quorum_root": config.wave100_notary_quorum_root,
                "liquidity_reservation_root": liquidity_reservation_root,
                "fee_netting_open": true,
            }),
        );
        let payout_throttle_root = value_root(
            "WAVE101-PAYOUT-THROTTLE",
            &json!({
                "slot_kind": slot,
                "source_payout_envelope_root": config.wave100_payout_envelope_root,
                "audit_fee_netting_root": audit_fee_netting_root,
                "throttle_armed": true,
                "release_execution_allowed": false,
            }),
        );
        let privacy_budget_root = value_root(
            "WAVE101-PRIVACY-BUDGET",
            &json!({
                "slot_kind": slot,
                "liquidity_reservation_root": liquidity_reservation_root,
                "privacy_budget_open": false,
                "budget_blocked": true,
            }),
        );
        let circuit_breaker_root = value_root(
            "WAVE101-CIRCUIT-BREAKER",
            &json!({
                "slot_kind": slot,
                "source_circuit_breaker_root": config.wave100_circuit_breaker_root,
                "armed": true,
            }),
        );
        let reviewer_signoff_root = value_root(
            "WAVE101-REVIEWER-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "source_signoff_root": config.wave100_signoff_root,
                "signoff_present": false,
            }),
        );
        let operator_signoff_root = value_root(
            "WAVE101-OPERATOR-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "config_root": config.state_root(),
                "signoff_present": false,
            }),
        );
        let throttle_guard_root = value_root(
            "WAVE101-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARD",
            &json!({
                "slot_kind": slot,
                "wave100_settlement_notary_root": wave100_settlement_notary_root,
                "liquidity_reservation_root": liquidity_reservation_root,
                "audit_fee_netting_root": audit_fee_netting_root,
                "payout_throttle_root": payout_throttle_root,
                "privacy_budget_root": privacy_budget_root,
                "circuit_breaker_root": circuit_breaker_root,
                "reviewer_signoff_root": reviewer_signoff_root,
                "operator_signoff_root": operator_signoff_root,
                "disbursement_allowed": false,
            }),
        );
        Self {
            slot_kind,
            wave100_settlement_notary_root,
            liquidity_reservation_root,
            audit_fee_netting_root,
            payout_throttle_root,
            privacy_budget_root,
            circuit_breaker_root,
            reviewer_signoff_root,
            operator_signoff_root,
            throttle_guard_root,
            status: ThrottleStatus::DisbursementDenied,
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
        ensure_root("audit_fee_netting_root", &self.audit_fee_netting_root)?;
        ensure_root("payout_throttle_root", &self.payout_throttle_root)?;
        ensure_root("privacy_budget_root", &self.privacy_budget_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("throttle_guard_root", &self.throttle_guard_root)?;
        if !self.status.blocks_disbursement() {
            return Err("wave101 throttle guard does not block disbursement".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "audit_fee_netting_root": self.audit_fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "privacy_budget_root": self.privacy_budget_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_signoff_root": self.operator_signoff_root,
            "throttle_guard_root": self.throttle_guard_root,
            "status": self.status.as_str(),
            "disbursement_allowed": false,
            "release_execution_allowed": false,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ThrottleBlocker {
    pub kind: BlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocker_root: String,
}

impl ThrottleBlocker {
    pub fn new(kind: BlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        Self {
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocker_root: value_root(
                "WAVE101-THROTTLE-BLOCKER",
                &json!({
                    "kind": kind.as_str(),
                    "slot_kind": slot_kind.as_str(),
                    "evidence_root": evidence_root,
                    "active": true,
                }),
            ),
        }
    }

    pub fn canonical(guards: &[ThrottleGuard], config: &Config) -> Vec<Self> {
        let mut blockers = Vec::new();
        for guard in guards {
            blockers.push(Self::new(
                BlockerKind::Wave100SettlementNotaryActive,
                guard.slot_kind,
                &guard.wave100_settlement_notary_root,
            ));
            blockers.push(Self::new(
                BlockerKind::LiquidityReservationMissing,
                guard.slot_kind,
                &guard.liquidity_reservation_root,
            ));
            blockers.push(Self::new(
                BlockerKind::AuditFeeNettingOpen,
                guard.slot_kind,
                &guard.audit_fee_netting_root,
            ));
            blockers.push(Self::new(
                BlockerKind::PayoutThrottleArmed,
                guard.slot_kind,
                &guard.payout_throttle_root,
            ));
            blockers.push(Self::new(
                BlockerKind::PrivacyBudgetBlocked,
                guard.slot_kind,
                &guard.privacy_budget_root,
            ));
            blockers.push(Self::new(
                BlockerKind::CircuitBreakerActive,
                guard.slot_kind,
                &guard.circuit_breaker_root,
            ));
            blockers.push(Self::new(
                BlockerKind::ReviewerSignoffMissing,
                guard.slot_kind,
                &guard.reviewer_signoff_root,
            ));
            blockers.push(Self::new(
                BlockerKind::OperatorSignoffMissing,
                guard.slot_kind,
                &guard.operator_signoff_root,
            ));
        }
        blockers.push(Self::new(
            BlockerKind::SettlementExecutionRootMissing,
            LaneSlotKind::AuditReview,
            &config.wave100_execution_bundle_root,
        ));
        blockers.push(Self::new(
            BlockerKind::DisbursementReservationBudgetZero,
            LaneSlotKind::PrivacyReview,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            BlockerKind::ReleaseExecutionBudgetZero,
            LaneSlotKind::AdversarialScenario,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            BlockerKind::ReleaseExecutionDenied,
            LaneSlotKind::ThreatModel,
            &config.wave100_payout_envelope_root,
        ));
        blockers.push(Self::new(
            BlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            BlockerKind::FailClosedDisarmed,
            LaneSlotKind::ReviewerSignoff,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            BlockerKind::RawPayloadPresent,
            LaneSlotKind::AuditReview,
            &deterministic_root("raw-payload-absence-root"),
        ));
        blockers.push(Self::new(
            BlockerKind::HeavyGateRunClaimed,
            LaneSlotKind::OperatorSignoff,
            &deterministic_root("heavy-gate-not-run-root"),
        ));
        blockers
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("blocker_root", &self.blocker_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "active": true,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub target_root: String,
    pub command_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_root: &str, config: &Config) -> Self {
        Self {
            kind,
            target_root: target_root.to_string(),
            command_root: value_root(
                "WAVE101-COMMAND-HINT",
                &json!({
                    "kind": kind.as_str(),
                    "target_root": target_root,
                    "wave": config.wave,
                    "source_wave": config.source_wave,
                    "fail_closed_armed": config.fail_closed_armed,
                    "release_execution_allowed": false,
                    "disbursement_allowed": false,
                }),
            ),
        }
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandHintKind::HoldReleaseExecution,
                &config.wave100_payout_envelope_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepFailClosed,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::ImportWave100SettlementNotaryRoots,
                &config.wave100_settlement_notary_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachLiquidityReservationRoot,
                &config.wave100_execution_bundle_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachAuditFeeNettingRoot,
                &config.wave100_settlement_accounting_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepPayoutThrottleArmed,
                &config.wave100_payout_envelope_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepPrivacyBudgetBlocked,
                &config.wave100_notary_quorum_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepCircuitBreakerArmed,
                &config.wave100_circuit_breaker_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireReviewerSignoffRoot,
                &config.wave100_signoff_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireOperatorSignoffRoot,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::DenyDisbursementUntilBlockersClear,
                &config.wave100_settlement_notary_root,
                config,
            ),
        ]
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("target_root", &self.target_root)?;
        ensure_root("command_root", &self.command_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "target_root": self.target_root,
            "command_root": self.command_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub throttle_guards: u64,
    pub wave100_settlement_notary_roots: u64,
    pub liquidity_reservation_roots: u64,
    pub audit_fee_netting_roots: u64,
    pub payout_throttle_roots: u64,
    pub privacy_budget_roots: u64,
    pub circuit_breaker_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub operator_signoff_roots: u64,
    pub blocker_count: u64,
    pub command_hint_count: u64,
    pub disbursement_reservations: u64,
    pub release_executions: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "throttle_guards": self.throttle_guards,
            "wave100_settlement_notary_roots": self.wave100_settlement_notary_roots,
            "liquidity_reservation_roots": self.liquidity_reservation_roots,
            "audit_fee_netting_roots": self.audit_fee_netting_roots,
            "payout_throttle_roots": self.payout_throttle_roots,
            "privacy_budget_roots": self.privacy_budget_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "blocker_count": self.blocker_count,
            "command_hint_count": self.command_hint_count,
            "disbursement_reservations": self.disbursement_reservations,
            "release_executions": self.release_executions,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub throttle_guards: Vec<ThrottleGuard>,
    pub blockers: Vec<ThrottleBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let throttle_guards = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| ThrottleGuard::blocked(slot_kind, &config))
            .collect::<Vec<_>>();
        let blockers = ThrottleBlocker::canonical(&throttle_guards, &config);
        let command_hints = CommandHint::canonical(&config);
        let mut state = Self {
            config,
            throttle_guards,
            blockers,
            command_hints,
            counters: Counters::default(),
        };
        state.counters = state.compute_counters();
        state.validate()?;
        Ok(state)
    }

    pub fn compute_counters(&self) -> Counters {
        let guard_count = self.throttle_guards.len() as u64;
        Counters {
            throttle_guards: guard_count,
            wave100_settlement_notary_roots: guard_count,
            liquidity_reservation_roots: guard_count,
            audit_fee_netting_roots: guard_count,
            payout_throttle_roots: guard_count,
            privacy_budget_roots: guard_count,
            circuit_breaker_roots: guard_count,
            reviewer_signoff_roots: guard_count,
            operator_signoff_roots: guard_count,
            blocker_count: self.blockers.len() as u64,
            command_hint_count: self.command_hints.len() as u64,
            disbursement_reservations: 0,
            release_executions: 0,
            raw_payload_records: 0,
        }
    }

    pub fn wave100_settlement_notary_source_root(&self) -> String {
        value_root(
            "WAVE101-SOURCE-WAVE100-SETTLEMENT-NOTARY-ROOT",
            &json!({
                "wave100_settlement_notary_root": self.config.wave100_settlement_notary_root,
                "wave100_execution_bundle_root": self.config.wave100_execution_bundle_root,
                "wave100_settlement_accounting_root": self.config.wave100_settlement_accounting_root,
                "wave100_notary_quorum_root": self.config.wave100_notary_quorum_root,
                "wave100_payout_envelope_root": self.config.wave100_payout_envelope_root,
                "wave100_circuit_breaker_root": self.config.wave100_circuit_breaker_root,
                "wave100_signoff_root": self.config.wave100_signoff_root,
            }),
        )
    }

    pub fn liquidity_reservation_root(&self) -> String {
        collection_root(
            "WAVE101-LIQUIDITY-RESERVATION-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "liquidity_reservation_root": guard.liquidity_reservation_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn audit_fee_netting_root(&self) -> String {
        collection_root(
            "WAVE101-AUDIT-FEE-NETTING-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "audit_fee_netting_root": guard.audit_fee_netting_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn payout_throttle_root(&self) -> String {
        collection_root(
            "WAVE101-PAYOUT-THROTTLE-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "payout_throttle_root": guard.payout_throttle_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn privacy_budget_root(&self) -> String {
        collection_root(
            "WAVE101-PRIVACY-BUDGET-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "privacy_budget_root": guard.privacy_budget_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn circuit_breaker_root(&self) -> String {
        collection_root(
            "WAVE101-CIRCUIT-BREAKER-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "circuit_breaker_root": guard.circuit_breaker_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn signoff_root(&self) -> String {
        collection_root(
            "WAVE101-SIGNOFF-ROOTS",
            self.throttle_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "reviewer_signoff_root": guard.reviewer_signoff_root,
                        "operator_signoff_root": guard.operator_signoff_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn throttle_guard_root(&self) -> String {
        collection_root(
            "WAVE101-DISBURSEMENT-LIQUIDITY-THROTTLE-GUARDS",
            self.throttle_guards
                .iter()
                .map(ThrottleGuard::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE101-THROTTLE-BLOCKERS",
            self.blockers
                .iter()
                .map(ThrottleBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE101-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn deterministic_roots_root(&self) -> String {
        value_root(
            "WAVE101-DETERMINISTIC-ROOTS",
            &json!({
                "config_root": self.config.state_root(),
                "wave100_settlement_notary_source_root": self.wave100_settlement_notary_source_root(),
                "liquidity_reservation_root": self.liquidity_reservation_root(),
                "audit_fee_netting_root": self.audit_fee_netting_root(),
                "payout_throttle_root": self.payout_throttle_root(),
                "privacy_budget_root": self.privacy_budget_root(),
                "circuit_breaker_root": self.circuit_breaker_root(),
                "signoff_root": self.signoff_root(),
                "throttle_guard_root": self.throttle_guard_root(),
                "blocker_root": self.blocker_root(),
                "command_hint_root": self.command_hint_root(),
            }),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE101-STATE-MATERIAL",
            &json!({
                "deterministic_roots_root": self.deterministic_roots_root(),
                "counters": self.counters.public_record(),
                "release_execution_allowed": false,
                "disbursement_allowed": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE101-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "liquidity_blockers_active": self.config.liquidity_blockers_active,
                "throttle_blockers_active": self.config.throttle_blockers_active,
                "circuit_breaker_armed": self.config.circuit_breaker_armed,
                "disbursement_reservations": 0,
                "release_executions": 0,
                "disbursement_denied": self.disbursement_denied(),
                "release_execution_denied": self.release_execution_denied(),
                "production_denied": self.production_denied(),
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn disbursement_denied(&self) -> bool {
        !self.config.disbursement_allowed
            || self.config.liquidity_blockers_active
            || self.config.throttle_blockers_active
            || self.config.circuit_breaker_armed
            || !self.blockers.is_empty()
            || self.counters.disbursement_reservations == 0
    }

    pub fn release_execution_denied(&self) -> bool {
        !self.config.release_execution_allowed
            || self.disbursement_denied()
            || self.counters.release_executions == 0
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || self.release_execution_denied()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.throttle_guards.len() as u64,
            self.config.min_slot_count,
        )?;
        ensure_min_count(
            "liquidity reservation roots",
            self.counters.liquidity_reservation_roots,
            self.config.min_liquidity_reservation_roots,
        )?;
        for guard in &self.throttle_guards {
            guard.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("wave101 throttle guard contains raw payload records".to_string());
        }
        if self.counters.disbursement_reservations > self.config.max_disbursement_reservations {
            return Err("wave101 disbursement reservations above configured limit".to_string());
        }
        if self.counters.release_executions > self.config.max_release_executions {
            return Err("wave101 release executions above configured limit".to_string());
        }
        if self.counters.disbursement_reservations != 0 {
            return Err("devnet wave101 guard must not reserve disbursement liquidity".to_string());
        }
        if self.counters.release_executions != 0 {
            return Err("devnet wave101 guard must not execute releases".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("wave101 throttle guard counters do not match state".to_string());
        }
        if !self.disbursement_denied() {
            return Err("wave101 liquidity throttle guard cannot allow disbursement".to_string());
        }
        if !self.release_execution_denied() {
            return Err(
                "wave101 liquidity throttle guard cannot allow release execution".to_string(),
            );
        }
        if !self.production_denied() {
            return Err("wave101 liquidity throttle guard cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave101_live_heavy_gate_release_execution_disbursement_liquidity_throttle_guard_audit_security_lane_state",
            "config": self.config.public_record(),
            "wave100_settlement_notary_source_root": self.wave100_settlement_notary_source_root(),
            "liquidity_reservation_root": self.liquidity_reservation_root(),
            "audit_fee_netting_root": self.audit_fee_netting_root(),
            "payout_throttle_root": self.payout_throttle_root(),
            "privacy_budget_root": self.privacy_budget_root(),
            "circuit_breaker_root": self.circuit_breaker_root(),
            "signoff_root": self.signoff_root(),
            "throttle_guard_root": self.throttle_guard_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "deterministic_roots_root": self.deterministic_roots_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "disbursement_denied": self.disbursement_denied(),
            "release_execution_denied": self.release_execution_denied(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "throttle_guards": self.throttle_guards.iter().map(ThrottleGuard::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(ThrottleBlocker::public_record).collect::<Vec<_>>(),
            "command_hints": self.command_hints.iter().map(CommandHint::public_record).collect::<Vec<_>>(),
        })
    }
}

pub fn devnet() -> Runtime {
    match State::new(Config::devnet()) {
        Ok(runtime) => runtime,
        Err(error) => fallback_runtime(error),
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn fallback_runtime(error: String) -> Runtime {
    let config = Config::devnet();
    let throttle_guards = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| ThrottleGuard::blocked(slot_kind, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        blockers: vec![ThrottleBlocker::new(
            BlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE101-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        command_hints: CommandHint::canonical(&config),
        counters: Counters::default(),
        throttle_guards,
        config,
    };
    state.counters = state.compute_counters();
    state
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{} is empty", field));
    }
    Ok(())
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        return Err(format!("{} must be positive", field));
    }
    Ok(())
}

fn ensure_min_count(field: &str, actual: u64, minimum: u64) -> Result<()> {
    if actual < minimum {
        return Err(format!("{} is below required minimum", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{} is not a canonical root", field));
    }
    Ok(())
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE101-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(THROTTLE_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE101-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(THROTTLE_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn bind_root(domain: &str, label: &str, source_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE101-BIND-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(THROTTLE_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
            HashPart::Str(source_root),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(
        domain,
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Json(value)],
        32,
    )
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    merkle_root(domain, &values)
}
