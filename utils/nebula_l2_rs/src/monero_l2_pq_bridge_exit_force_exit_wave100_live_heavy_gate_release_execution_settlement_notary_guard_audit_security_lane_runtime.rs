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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave100-live-heavy-gate-release-execution-settlement-notary-guard-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const NOTARY_SUITE: &str =
    "wave100-wave99-unlock-guard-to-execution-settlement-notary-guard-v1";
pub const DEFAULT_WAVE: u64 = 100;
pub const DEFAULT_SOURCE_WAVE: u64 = 99;
pub const DEFAULT_HEIGHT: u64 = 4_282_700;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MIN_NOTARY_QUORUM: u64 = 4;
pub const DEFAULT_MAX_EXECUTION_BUNDLES: u64 = 0;
pub const DEFAULT_MAX_RELEASE_SETTLEMENTS: u64 = 0;
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
pub enum SettlementStatus {
    Wave99UnlockGuardActive,
    ExecutionBundleMissing,
    NotaryQuorumMissing,
    SettlementAccountingOpen,
    ReleaseDenied,
}

impl SettlementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuardActive => "wave99_unlock_guard_active",
            Self::ExecutionBundleMissing => "execution_bundle_missing",
            Self::NotaryQuorumMissing => "notary_quorum_missing",
            Self::SettlementAccountingOpen => "settlement_accounting_open",
            Self::ReleaseDenied => "release_denied",
        }
    }

    pub fn blocks_release(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave99UnlockGuardActive,
    ExecutionBundleMissing,
    SettlementAccountingOpen,
    NotaryQuorumMissing,
    PayoutEnvelopeSealed,
    RollbackSentinelActive,
    CircuitBreakerActive,
    ReviewerSignoffMissing,
    OperatorSignoffMissing,
    ExecutionBundleBudgetZero,
    ReleaseSettlementBudgetZero,
    ReleaseDenied,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuardActive => "wave99_unlock_guard_active",
            Self::ExecutionBundleMissing => "execution_bundle_missing",
            Self::SettlementAccountingOpen => "settlement_accounting_open",
            Self::NotaryQuorumMissing => "notary_quorum_missing",
            Self::PayoutEnvelopeSealed => "payout_envelope_sealed",
            Self::RollbackSentinelActive => "rollback_sentinel_active",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ExecutionBundleBudgetZero => "execution_bundle_budget_zero",
            Self::ReleaseSettlementBudgetZero => "release_settlement_budget_zero",
            Self::ReleaseDenied => "release_denied",
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
    ImportWave99UnlockGuardRoots,
    AttachExecutionBundleRoot,
    AttachSettlementAccountingRoot,
    RequireNotaryQuorumRoot,
    SealPayoutEnvelopeRoot,
    KeepRollbackSentinelArmed,
    KeepCircuitBreakerArmed,
    RequireReviewerSignoffRoot,
    RequireOperatorSignoffRoot,
    DenyReleaseUntilBlockersClear,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldReleaseExecution => "hold_release_execution",
            Self::KeepFailClosed => "keep_fail_closed",
            Self::ImportWave99UnlockGuardRoots => "import_wave99_unlock_guard_roots",
            Self::AttachExecutionBundleRoot => "attach_execution_bundle_root",
            Self::AttachSettlementAccountingRoot => "attach_settlement_accounting_root",
            Self::RequireNotaryQuorumRoot => "require_notary_quorum_root",
            Self::SealPayoutEnvelopeRoot => "seal_payout_envelope_root",
            Self::KeepRollbackSentinelArmed => "keep_rollback_sentinel_armed",
            Self::KeepCircuitBreakerArmed => "keep_circuit_breaker_armed",
            Self::RequireReviewerSignoffRoot => "require_reviewer_signoff_root",
            Self::RequireOperatorSignoffRoot => "require_operator_signoff_root",
            Self::DenyReleaseUntilBlockersClear => "deny_release_until_blockers_clear",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub notary_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub min_notary_quorum: u64,
    pub max_execution_bundles: u64,
    pub max_release_settlements: u64,
    pub max_raw_payload_records: u64,
    pub wave99_unlock_guard_root: String,
    pub wave99_finality_certificate_root: String,
    pub wave99_audit_accounting_root: String,
    pub wave99_rollback_guard_root: String,
    pub wave99_circuit_breaker_root: String,
    pub wave99_signoff_root: String,
    pub fail_closed_armed: bool,
    pub settlement_blockers_active: bool,
    pub circuit_breaker_armed: bool,
    pub rollback_sentinel_active: bool,
    pub release_execution_allowed: bool,
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
            notary_suite: NOTARY_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            min_notary_quorum: DEFAULT_MIN_NOTARY_QUORUM,
            max_execution_bundles: DEFAULT_MAX_EXECUTION_BUNDLES,
            max_release_settlements: DEFAULT_MAX_RELEASE_SETTLEMENTS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            wave99_unlock_guard_root: deterministic_root("wave99-unlock-guard-root"),
            wave99_finality_certificate_root: deterministic_root(
                "wave99-finality-certificate-root",
            ),
            wave99_audit_accounting_root: deterministic_root("wave99-audit-accounting-root"),
            wave99_rollback_guard_root: deterministic_root("wave99-rollback-guard-root"),
            wave99_circuit_breaker_root: deterministic_root("wave99-circuit-breaker-root"),
            wave99_signoff_root: deterministic_root("wave99-signoff-root"),
            fail_closed_armed: true,
            settlement_blockers_active: true,
            circuit_breaker_armed: true,
            rollback_sentinel_active: true,
            release_execution_allowed: false,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("notary_suite", &self.notary_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_positive("min_notary_quorum", self.min_notary_quorum)?;
        ensure_root("wave99_unlock_guard_root", &self.wave99_unlock_guard_root)?;
        ensure_root(
            "wave99_finality_certificate_root",
            &self.wave99_finality_certificate_root,
        )?;
        ensure_root(
            "wave99_audit_accounting_root",
            &self.wave99_audit_accounting_root,
        )?;
        ensure_root(
            "wave99_rollback_guard_root",
            &self.wave99_rollback_guard_root,
        )?;
        ensure_root(
            "wave99_circuit_breaker_root",
            &self.wave99_circuit_breaker_root,
        )?;
        ensure_root("wave99_signoff_root", &self.wave99_signoff_root)?;
        if !self.fail_closed_armed {
            return Err("wave100 notary guard fail closed is not armed".to_string());
        }
        if !self.settlement_blockers_active {
            return Err("wave100 settlement blockers must remain active in devnet".to_string());
        }
        if !self.circuit_breaker_armed {
            return Err("wave100 circuit breaker must remain armed in devnet".to_string());
        }
        if !self.rollback_sentinel_active {
            return Err("wave100 rollback sentinel must remain active in devnet".to_string());
        }
        if self.release_execution_allowed {
            return Err("wave100 release execution is denied by default".to_string());
        }
        if self.production_allowed {
            return Err("wave100 production is denied by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave100 notary guard cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave100_execution_settlement_notary_guard_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "notary_suite": self.notary_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "min_notary_quorum": self.min_notary_quorum,
            "max_execution_bundles": self.max_execution_bundles,
            "max_release_settlements": self.max_release_settlements,
            "max_raw_payload_records": self.max_raw_payload_records,
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "wave99_finality_certificate_root": self.wave99_finality_certificate_root,
            "wave99_audit_accounting_root": self.wave99_audit_accounting_root,
            "wave99_rollback_guard_root": self.wave99_rollback_guard_root,
            "wave99_circuit_breaker_root": self.wave99_circuit_breaker_root,
            "wave99_signoff_root": self.wave99_signoff_root,
            "fail_closed_armed": self.fail_closed_armed,
            "settlement_blockers_active": self.settlement_blockers_active,
            "circuit_breaker_armed": self.circuit_breaker_armed,
            "rollback_sentinel_active": self.rollback_sentinel_active,
            "release_execution_allowed": self.release_execution_allowed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE100-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementGuard {
    pub slot_kind: LaneSlotKind,
    pub wave99_unlock_guard_root: String,
    pub execution_bundle_root: String,
    pub settlement_accounting_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub reviewer_signoff_root: String,
    pub operator_signoff_root: String,
    pub settlement_guard_root: String,
    pub status: SettlementStatus,
}

impl SettlementGuard {
    pub fn blocked(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let wave99_unlock_guard_root = bind_root(
            "wave99-unlock-guard",
            slot,
            &config.wave99_unlock_guard_root,
        );
        let execution_bundle_root = value_root(
            "WAVE100-AUDIT-EXECUTION-BUNDLE",
            &json!({
                "slot_kind": slot,
                "source_unlock_guard_root": wave99_unlock_guard_root,
                "source_finality_certificate_root": config.wave99_finality_certificate_root,
                "execution_bundle_present": false,
                "release_execution_allowed": false,
            }),
        );
        let settlement_accounting_root = value_root(
            "WAVE100-AUDIT-SETTLEMENT-ACCOUNTING",
            &json!({
                "slot_kind": slot,
                "source_audit_accounting_root": config.wave99_audit_accounting_root,
                "execution_bundle_root": execution_bundle_root,
                "settlement_accounting_open": true,
            }),
        );
        let notary_quorum_root = value_root(
            "WAVE100-NOTARY-QUORUM",
            &json!({
                "slot_kind": slot,
                "min_notary_quorum": config.min_notary_quorum,
                "quorum_met": false,
            }),
        );
        let payout_envelope_root = value_root(
            "WAVE100-PAYOUT-ENVELOPE",
            &json!({
                "slot_kind": slot,
                "settlement_accounting_root": settlement_accounting_root,
                "payout_envelope_released": false,
            }),
        );
        let rollback_sentinel_root = value_root(
            "WAVE100-ROLLBACK-SENTINEL",
            &json!({
                "slot_kind": slot,
                "source_rollback_guard_root": config.wave99_rollback_guard_root,
                "rollback_sentinel_active": true,
            }),
        );
        let circuit_breaker_root = value_root(
            "WAVE100-CIRCUIT-BREAKER",
            &json!({
                "slot_kind": slot,
                "source_circuit_breaker_root": config.wave99_circuit_breaker_root,
                "armed": true,
            }),
        );
        let reviewer_signoff_root = value_root(
            "WAVE100-REVIEWER-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "source_signoff_root": config.wave99_signoff_root,
                "signoff_present": false,
            }),
        );
        let operator_signoff_root = value_root(
            "WAVE100-OPERATOR-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "config_root": config.state_root(),
                "signoff_present": false,
            }),
        );
        let settlement_guard_root = value_root(
            "WAVE100-SETTLEMENT-NOTARY-GUARD",
            &json!({
                "slot_kind": slot,
                "wave99_unlock_guard_root": wave99_unlock_guard_root,
                "execution_bundle_root": execution_bundle_root,
                "settlement_accounting_root": settlement_accounting_root,
                "notary_quorum_root": notary_quorum_root,
                "payout_envelope_root": payout_envelope_root,
                "rollback_sentinel_root": rollback_sentinel_root,
                "circuit_breaker_root": circuit_breaker_root,
                "reviewer_signoff_root": reviewer_signoff_root,
                "operator_signoff_root": operator_signoff_root,
                "release_execution_allowed": false,
            }),
        );
        Self {
            slot_kind,
            wave99_unlock_guard_root,
            execution_bundle_root,
            settlement_accounting_root,
            notary_quorum_root,
            payout_envelope_root,
            rollback_sentinel_root,
            circuit_breaker_root,
            reviewer_signoff_root,
            operator_signoff_root,
            settlement_guard_root,
            status: SettlementStatus::ReleaseDenied,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("wave99_unlock_guard_root", &self.wave99_unlock_guard_root)?;
        ensure_root("execution_bundle_root", &self.execution_bundle_root)?;
        ensure_root(
            "settlement_accounting_root",
            &self.settlement_accounting_root,
        )?;
        ensure_root("notary_quorum_root", &self.notary_quorum_root)?;
        ensure_root("payout_envelope_root", &self.payout_envelope_root)?;
        ensure_root("rollback_sentinel_root", &self.rollback_sentinel_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("settlement_guard_root", &self.settlement_guard_root)?;
        if !self.status.blocks_release() {
            return Err("wave100 settlement guard does not block release".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "execution_bundle_root": self.execution_bundle_root,
            "settlement_accounting_root": self.settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_signoff_root": self.operator_signoff_root,
            "settlement_guard_root": self.settlement_guard_root,
            "status": self.status.as_str(),
            "release_execution_allowed": false,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementBlocker {
    pub kind: BlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocker_root: String,
}

impl SettlementBlocker {
    pub fn new(kind: BlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        Self {
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocker_root: value_root(
                "WAVE100-SETTLEMENT-BLOCKER",
                &json!({
                    "kind": kind.as_str(),
                    "slot_kind": slot_kind.as_str(),
                    "evidence_root": evidence_root,
                    "active": true,
                }),
            ),
        }
    }

    pub fn canonical(guards: &[SettlementGuard], config: &Config) -> Vec<Self> {
        let mut blockers = Vec::new();
        for guard in guards {
            blockers.push(Self::new(
                BlockerKind::Wave99UnlockGuardActive,
                guard.slot_kind,
                &guard.wave99_unlock_guard_root,
            ));
            blockers.push(Self::new(
                BlockerKind::ExecutionBundleMissing,
                guard.slot_kind,
                &guard.execution_bundle_root,
            ));
            blockers.push(Self::new(
                BlockerKind::SettlementAccountingOpen,
                guard.slot_kind,
                &guard.settlement_accounting_root,
            ));
            blockers.push(Self::new(
                BlockerKind::NotaryQuorumMissing,
                guard.slot_kind,
                &guard.notary_quorum_root,
            ));
            blockers.push(Self::new(
                BlockerKind::PayoutEnvelopeSealed,
                guard.slot_kind,
                &guard.payout_envelope_root,
            ));
            blockers.push(Self::new(
                BlockerKind::RollbackSentinelActive,
                guard.slot_kind,
                &guard.rollback_sentinel_root,
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
            BlockerKind::ExecutionBundleBudgetZero,
            LaneSlotKind::OperatorSignoff,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            BlockerKind::ReleaseSettlementBudgetZero,
            LaneSlotKind::OperatorSignoff,
            &config.wave99_unlock_guard_root,
        ));
        blockers.push(Self::new(
            BlockerKind::ReleaseDenied,
            LaneSlotKind::OperatorSignoff,
            &config.wave99_circuit_breaker_root,
        ));
        blockers.push(Self::new(
            BlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &config.wave99_rollback_guard_root,
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
                "WAVE100-COMMAND-HINT",
                &json!({
                    "kind": kind.as_str(),
                    "target_root": target_root,
                    "wave": config.wave,
                    "source_wave": config.source_wave,
                    "fail_closed_armed": config.fail_closed_armed,
                    "release_execution_allowed": false,
                    "heavy_gates_ran": false,
                }),
            ),
        }
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandHintKind::HoldReleaseExecution,
                &config.wave99_unlock_guard_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepFailClosed,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::ImportWave99UnlockGuardRoots,
                &config.wave99_unlock_guard_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachExecutionBundleRoot,
                &config.wave99_finality_certificate_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachSettlementAccountingRoot,
                &config.wave99_audit_accounting_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireNotaryQuorumRoot,
                &config.wave99_signoff_root,
                config,
            ),
            Self::new(
                CommandHintKind::SealPayoutEnvelopeRoot,
                &config.wave99_audit_accounting_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepRollbackSentinelArmed,
                &config.wave99_rollback_guard_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepCircuitBreakerArmed,
                &config.wave99_circuit_breaker_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireReviewerSignoffRoot,
                &config.wave99_signoff_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireOperatorSignoffRoot,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::DenyReleaseUntilBlockersClear,
                &config.wave99_unlock_guard_root,
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
    pub settlement_guards: u64,
    pub wave99_unlock_guard_roots: u64,
    pub execution_bundle_roots: u64,
    pub settlement_accounting_roots: u64,
    pub notary_quorum_roots: u64,
    pub payout_envelope_roots: u64,
    pub rollback_sentinel_roots: u64,
    pub circuit_breaker_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub operator_signoff_roots: u64,
    pub blocker_count: u64,
    pub command_hint_count: u64,
    pub execution_bundles: u64,
    pub release_settlements: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "settlement_guards": self.settlement_guards,
            "wave99_unlock_guard_roots": self.wave99_unlock_guard_roots,
            "execution_bundle_roots": self.execution_bundle_roots,
            "settlement_accounting_roots": self.settlement_accounting_roots,
            "notary_quorum_roots": self.notary_quorum_roots,
            "payout_envelope_roots": self.payout_envelope_roots,
            "rollback_sentinel_roots": self.rollback_sentinel_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "blocker_count": self.blocker_count,
            "command_hint_count": self.command_hint_count,
            "execution_bundles": self.execution_bundles,
            "release_settlements": self.release_settlements,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub settlement_guards: Vec<SettlementGuard>,
    pub blockers: Vec<SettlementBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let settlement_guards = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| SettlementGuard::blocked(slot_kind, &config))
            .collect::<Vec<_>>();
        let blockers = SettlementBlocker::canonical(&settlement_guards, &config);
        let command_hints = CommandHint::canonical(&config);
        let mut state = Self {
            config,
            settlement_guards,
            blockers,
            command_hints,
            counters: Counters::default(),
        };
        state.counters = state.compute_counters();
        state.validate()?;
        Ok(state)
    }

    pub fn compute_counters(&self) -> Counters {
        let guard_count = self.settlement_guards.len() as u64;
        Counters {
            settlement_guards: guard_count,
            wave99_unlock_guard_roots: guard_count,
            execution_bundle_roots: guard_count,
            settlement_accounting_roots: guard_count,
            notary_quorum_roots: guard_count,
            payout_envelope_roots: guard_count,
            rollback_sentinel_roots: guard_count,
            circuit_breaker_roots: guard_count,
            reviewer_signoff_roots: guard_count,
            operator_signoff_roots: guard_count,
            blocker_count: self.blockers.len() as u64,
            command_hint_count: self.command_hints.len() as u64,
            execution_bundles: 0,
            release_settlements: 0,
            raw_payload_records: 0,
        }
    }

    pub fn wave99_unlock_guard_source_root(&self) -> String {
        value_root(
            "WAVE100-SOURCE-WAVE99-UNLOCK-GUARD-ROOT",
            &json!({
                "wave99_unlock_guard_root": self.config.wave99_unlock_guard_root,
                "wave99_finality_certificate_root": self.config.wave99_finality_certificate_root,
                "wave99_audit_accounting_root": self.config.wave99_audit_accounting_root,
                "wave99_rollback_guard_root": self.config.wave99_rollback_guard_root,
                "wave99_circuit_breaker_root": self.config.wave99_circuit_breaker_root,
                "wave99_signoff_root": self.config.wave99_signoff_root,
            }),
        )
    }

    pub fn execution_bundle_root(&self) -> String {
        collection_root(
            "WAVE100-AUDIT-EXECUTION-BUNDLE-ROOTS",
            self.settlement_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "execution_bundle_root": guard.execution_bundle_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn settlement_accounting_root(&self) -> String {
        collection_root(
            "WAVE100-AUDIT-SETTLEMENT-ACCOUNTING-ROOTS",
            self.settlement_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "settlement_accounting_root": guard.settlement_accounting_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn notary_quorum_root(&self) -> String {
        collection_root(
            "WAVE100-NOTARY-QUORUM-ROOTS",
            self.settlement_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "notary_quorum_root": guard.notary_quorum_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn payout_envelope_root(&self) -> String {
        collection_root(
            "WAVE100-PAYOUT-ENVELOPE-ROOTS",
            self.settlement_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "payout_envelope_root": guard.payout_envelope_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn rollback_sentinel_root(&self) -> String {
        collection_root(
            "WAVE100-ROLLBACK-SENTINEL-ROOTS",
            self.settlement_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "rollback_sentinel_root": guard.rollback_sentinel_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn circuit_breaker_root(&self) -> String {
        collection_root(
            "WAVE100-CIRCUIT-BREAKER-ROOTS",
            self.settlement_guards
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
            "WAVE100-SIGNOFF-ROOTS",
            self.settlement_guards
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

    pub fn settlement_guard_root(&self) -> String {
        collection_root(
            "WAVE100-SETTLEMENT-NOTARY-GUARDS",
            self.settlement_guards
                .iter()
                .map(SettlementGuard::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE100-SETTLEMENT-BLOCKERS",
            self.blockers
                .iter()
                .map(SettlementBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE100-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn deterministic_roots_root(&self) -> String {
        value_root(
            "WAVE100-DETERMINISTIC-ROOTS",
            &json!({
                "config_root": self.config.state_root(),
                "wave99_unlock_guard_source_root": self.wave99_unlock_guard_source_root(),
                "execution_bundle_root": self.execution_bundle_root(),
                "settlement_accounting_root": self.settlement_accounting_root(),
                "notary_quorum_root": self.notary_quorum_root(),
                "payout_envelope_root": self.payout_envelope_root(),
                "rollback_sentinel_root": self.rollback_sentinel_root(),
                "circuit_breaker_root": self.circuit_breaker_root(),
                "signoff_root": self.signoff_root(),
                "settlement_guard_root": self.settlement_guard_root(),
                "blocker_root": self.blocker_root(),
                "command_hint_root": self.command_hint_root(),
            }),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE100-STATE-MATERIAL",
            &json!({
                "deterministic_roots_root": self.deterministic_roots_root(),
                "counters": self.counters.public_record(),
                "release_execution_allowed": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE100-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "settlement_blockers_active": self.config.settlement_blockers_active,
                "circuit_breaker_armed": self.config.circuit_breaker_armed,
                "rollback_sentinel_active": self.config.rollback_sentinel_active,
                "execution_bundles": 0,
                "release_settlements": 0,
                "release_execution_denied": self.release_execution_denied(),
                "production_denied": self.production_denied(),
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn release_execution_denied(&self) -> bool {
        !self.config.release_execution_allowed
            || self.config.settlement_blockers_active
            || self.config.circuit_breaker_armed
            || self.config.rollback_sentinel_active
            || !self.blockers.is_empty()
            || self.counters.execution_bundles == 0
            || self.counters.release_settlements == 0
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || self.release_execution_denied()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.settlement_guards.len() as u64,
            self.config.min_slot_count,
        )?;
        for guard in &self.settlement_guards {
            guard.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("wave100 notary guard contains raw payload records".to_string());
        }
        if self.counters.execution_bundles > self.config.max_execution_bundles {
            return Err("wave100 execution bundles above configured limit".to_string());
        }
        if self.counters.release_settlements > self.config.max_release_settlements {
            return Err("wave100 release settlements above configured limit".to_string());
        }
        if self.counters.execution_bundles != 0 {
            return Err(
                "devnet wave100 notary guard must not notarize execution bundles".to_string(),
            );
        }
        if self.counters.release_settlements != 0 {
            return Err("devnet wave100 notary guard must not settle releases".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("wave100 notary guard counters do not match state".to_string());
        }
        if !self.release_execution_denied() {
            return Err("wave100 notary guard cannot allow release execution".to_string());
        }
        if !self.production_denied() {
            return Err("wave100 notary guard cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave100_live_heavy_gate_release_execution_settlement_notary_guard_audit_security_lane_state",
            "config": self.config.public_record(),
            "wave99_unlock_guard_source_root": self.wave99_unlock_guard_source_root(),
            "execution_bundle_root": self.execution_bundle_root(),
            "settlement_accounting_root": self.settlement_accounting_root(),
            "notary_quorum_root": self.notary_quorum_root(),
            "payout_envelope_root": self.payout_envelope_root(),
            "rollback_sentinel_root": self.rollback_sentinel_root(),
            "circuit_breaker_root": self.circuit_breaker_root(),
            "signoff_root": self.signoff_root(),
            "settlement_guard_root": self.settlement_guard_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "deterministic_roots_root": self.deterministic_roots_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "release_execution_denied": self.release_execution_denied(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "settlement_guards": self.settlement_guards.iter().map(SettlementGuard::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(SettlementBlocker::public_record).collect::<Vec<_>>(),
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
    let settlement_guards = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| SettlementGuard::blocked(slot_kind, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        blockers: vec![SettlementBlocker::new(
            BlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE100-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        command_hints: CommandHint::canonical(&config),
        counters: Counters::default(),
        settlement_guards,
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
        "MONERO-L2-PQ-FORCE-EXIT-WAVE100-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(NOTARY_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE100-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(NOTARY_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn bind_root(domain: &str, label: &str, source_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE100-BIND-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(NOTARY_SUITE),
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
