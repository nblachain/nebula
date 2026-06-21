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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const CERTIFICATE_SUITE: &str =
    "wave99-wave98-holdoff-root-to-finality-certificate-unlock-guard-v1";
pub const DEFAULT_WAVE: u64 = 99;
pub const DEFAULT_SOURCE_WAVE: u64 = 98;
pub const DEFAULT_HEIGHT: u64 = 4_282_499;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MIN_FINALITY_DEPTH: u64 = 64;
pub const DEFAULT_MAX_UNLOCK_CERTIFICATES: u64 = 0;
pub const DEFAULT_MAX_RELEASED_CLAIMS: u64 = 0;
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
pub enum CertificateStatus {
    Missing,
    BlockedByHoldoff,
    BlockedByFinality,
    BlockedByCircuitBreaker,
    BlockedBySignoff,
    UnlockDenied,
}

impl CertificateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::BlockedByHoldoff => "blocked_by_holdoff",
            Self::BlockedByFinality => "blocked_by_finality",
            Self::BlockedByCircuitBreaker => "blocked_by_circuit_breaker",
            Self::BlockedBySignoff => "blocked_by_signoff",
            Self::UnlockDenied => "unlock_denied",
        }
    }

    pub fn blocks_unlock(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockBlockerKind {
    Wave98HoldoffRootActive,
    FinalityCertificateMissing,
    FinalityDepthTooLow,
    AuditAccountingOpen,
    RollbackGuardActive,
    CircuitBreakerActive,
    ReviewerSignoffMissing,
    OperatorSignoffMissing,
    UnlockCertificateBudgetZero,
    ReleaseDenied,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl UnlockBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98HoldoffRootActive => "wave98_holdoff_root_active",
            Self::FinalityCertificateMissing => "finality_certificate_missing",
            Self::FinalityDepthTooLow => "finality_depth_too_low",
            Self::AuditAccountingOpen => "audit_accounting_open",
            Self::RollbackGuardActive => "rollback_guard_active",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::UnlockCertificateBudgetZero => "unlock_certificate_budget_zero",
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
    HoldRelease,
    KeepFailClosed,
    ImportWave98HoldoffRoots,
    AttachFinalityCertificateRoot,
    AttachAuditAccountingRoot,
    AttachRollbackGuardRoot,
    KeepCircuitBreakerArmed,
    RequireReviewerSignoffRoot,
    RequireOperatorSignoffRoot,
    DenyUnlockUntilBlockersClear,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldRelease => "hold_release",
            Self::KeepFailClosed => "keep_fail_closed",
            Self::ImportWave98HoldoffRoots => "import_wave98_holdoff_roots",
            Self::AttachFinalityCertificateRoot => "attach_finality_certificate_root",
            Self::AttachAuditAccountingRoot => "attach_audit_accounting_root",
            Self::AttachRollbackGuardRoot => "attach_rollback_guard_root",
            Self::KeepCircuitBreakerArmed => "keep_circuit_breaker_armed",
            Self::RequireReviewerSignoffRoot => "require_reviewer_signoff_root",
            Self::RequireOperatorSignoffRoot => "require_operator_signoff_root",
            Self::DenyUnlockUntilBlockersClear => "deny_unlock_until_blockers_clear",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub certificate_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub min_finality_depth: u64,
    pub max_unlock_certificates: u64,
    pub max_released_claims: u64,
    pub max_raw_payload_records: u64,
    pub wave98_holdoff_ledger_root: String,
    pub wave98_challenge_window_root: String,
    pub wave98_audit_objection_root: String,
    pub wave98_reviewer_hold_root: String,
    pub wave98_blocker_root: String,
    pub wave98_release_denial_root: String,
    pub finality_certificate_anchor_root: String,
    pub fail_closed_armed: bool,
    pub finality_blockers_active: bool,
    pub circuit_breaker_armed: bool,
    pub rollback_guard_active: bool,
    pub unlock_allowed: bool,
    pub release_allowed: bool,
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
            certificate_suite: CERTIFICATE_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            min_finality_depth: DEFAULT_MIN_FINALITY_DEPTH,
            max_unlock_certificates: DEFAULT_MAX_UNLOCK_CERTIFICATES,
            max_released_claims: DEFAULT_MAX_RELEASED_CLAIMS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            wave98_holdoff_ledger_root: deterministic_root("wave98-holdoff-ledger-root"),
            wave98_challenge_window_root: deterministic_root("wave98-challenge-window-root"),
            wave98_audit_objection_root: deterministic_root("wave98-audit-objection-root"),
            wave98_reviewer_hold_root: deterministic_root("wave98-reviewer-hold-root"),
            wave98_blocker_root: deterministic_root("wave98-blocker-root"),
            wave98_release_denial_root: deterministic_root("wave98-release-denial-root"),
            finality_certificate_anchor_root: empty_root("wave99-finality-certificate-anchor"),
            fail_closed_armed: true,
            finality_blockers_active: true,
            circuit_breaker_armed: true,
            rollback_guard_active: true,
            unlock_allowed: false,
            release_allowed: false,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("certificate_suite", &self.certificate_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_positive("min_finality_depth", self.min_finality_depth)?;
        ensure_root(
            "wave98_holdoff_ledger_root",
            &self.wave98_holdoff_ledger_root,
        )?;
        ensure_root(
            "wave98_challenge_window_root",
            &self.wave98_challenge_window_root,
        )?;
        ensure_root(
            "wave98_audit_objection_root",
            &self.wave98_audit_objection_root,
        )?;
        ensure_root("wave98_reviewer_hold_root", &self.wave98_reviewer_hold_root)?;
        ensure_root("wave98_blocker_root", &self.wave98_blocker_root)?;
        ensure_root(
            "wave98_release_denial_root",
            &self.wave98_release_denial_root,
        )?;
        ensure_root(
            "finality_certificate_anchor_root",
            &self.finality_certificate_anchor_root,
        )?;
        if !self.fail_closed_armed {
            return Err("wave99 unlock guard fail closed is not armed".to_string());
        }
        if !self.finality_blockers_active {
            return Err("wave99 finality blockers must remain active in devnet".to_string());
        }
        if !self.circuit_breaker_armed {
            return Err("wave99 circuit breaker must remain armed in devnet".to_string());
        }
        if !self.rollback_guard_active {
            return Err("wave99 rollback guard must remain active in devnet".to_string());
        }
        if self.unlock_allowed {
            return Err("wave99 unlock guard denies certificate unlock by default".to_string());
        }
        if self.release_allowed {
            return Err("wave99 unlock guard denies release by default".to_string());
        }
        if self.production_allowed {
            return Err("wave99 unlock guard denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave99 unlock guard cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave99_finality_certificate_unlock_guard_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "certificate_suite": self.certificate_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "min_finality_depth": self.min_finality_depth,
            "max_unlock_certificates": self.max_unlock_certificates,
            "max_released_claims": self.max_released_claims,
            "max_raw_payload_records": self.max_raw_payload_records,
            "wave98_holdoff_ledger_root": self.wave98_holdoff_ledger_root,
            "wave98_challenge_window_root": self.wave98_challenge_window_root,
            "wave98_audit_objection_root": self.wave98_audit_objection_root,
            "wave98_reviewer_hold_root": self.wave98_reviewer_hold_root,
            "wave98_blocker_root": self.wave98_blocker_root,
            "wave98_release_denial_root": self.wave98_release_denial_root,
            "finality_certificate_anchor_root": self.finality_certificate_anchor_root,
            "fail_closed_armed": self.fail_closed_armed,
            "finality_blockers_active": self.finality_blockers_active,
            "circuit_breaker_armed": self.circuit_breaker_armed,
            "rollback_guard_active": self.rollback_guard_active,
            "unlock_allowed": self.unlock_allowed,
            "release_allowed": self.release_allowed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE99-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockGuard {
    pub slot_kind: LaneSlotKind,
    pub wave98_holdoff_root: String,
    pub finality_certificate_root: String,
    pub audit_accounting_root: String,
    pub rollback_guard_root: String,
    pub circuit_breaker_root: String,
    pub reviewer_signoff_root: String,
    pub operator_signoff_root: String,
    pub unlock_guard_root: String,
    pub status: CertificateStatus,
}

impl UnlockGuard {
    pub fn blocked(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let wave98_holdoff_root = bind_root(
            "wave98-holdoff-ledger",
            slot,
            &config.wave98_holdoff_ledger_root,
        );
        let finality_certificate_root = value_root(
            "WAVE99-FINALITY-CERTIFICATE",
            &json!({
                "slot_kind": slot,
                "anchor_root": config.finality_certificate_anchor_root,
                "source_holdoff_root": wave98_holdoff_root,
                "finality_depth": 0,
                "unlock_certificate_present": false,
            }),
        );
        let audit_accounting_root = value_root(
            "WAVE99-AUDIT-ACCOUNTING",
            &json!({
                "slot_kind": slot,
                "source_audit_objection_root": config.wave98_audit_objection_root,
                "holdoff_root": wave98_holdoff_root,
                "accounting_open": true,
            }),
        );
        let rollback_guard_root = value_root(
            "WAVE99-ROLLBACK-GUARD",
            &json!({
                "slot_kind": slot,
                "source_blocker_root": config.wave98_blocker_root,
                "rollback_guard_active": true,
            }),
        );
        let circuit_breaker_root = value_root(
            "WAVE99-CIRCUIT-BREAKER",
            &json!({
                "slot_kind": slot,
                "source_release_denial_root": config.wave98_release_denial_root,
                "armed": true,
            }),
        );
        let reviewer_signoff_root = value_root(
            "WAVE99-REVIEWER-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "source_reviewer_hold_root": config.wave98_reviewer_hold_root,
                "signoff_present": false,
            }),
        );
        let operator_signoff_root = value_root(
            "WAVE99-OPERATOR-SIGNOFF",
            &json!({
                "slot_kind": slot,
                "config_root": config.state_root(),
                "signoff_present": false,
            }),
        );
        let unlock_guard_root = value_root(
            "WAVE99-UNLOCK-GUARD",
            &json!({
                "slot_kind": slot,
                "wave98_holdoff_root": wave98_holdoff_root,
                "finality_certificate_root": finality_certificate_root,
                "audit_accounting_root": audit_accounting_root,
                "rollback_guard_root": rollback_guard_root,
                "circuit_breaker_root": circuit_breaker_root,
                "reviewer_signoff_root": reviewer_signoff_root,
                "operator_signoff_root": operator_signoff_root,
                "unlock_allowed": false,
            }),
        );
        Self {
            slot_kind,
            wave98_holdoff_root,
            finality_certificate_root,
            audit_accounting_root,
            rollback_guard_root,
            circuit_breaker_root,
            reviewer_signoff_root,
            operator_signoff_root,
            unlock_guard_root,
            status: CertificateStatus::UnlockDenied,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("wave98_holdoff_root", &self.wave98_holdoff_root)?;
        ensure_root("finality_certificate_root", &self.finality_certificate_root)?;
        ensure_root("audit_accounting_root", &self.audit_accounting_root)?;
        ensure_root("rollback_guard_root", &self.rollback_guard_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("unlock_guard_root", &self.unlock_guard_root)?;
        if !self.status.blocks_unlock() {
            return Err("wave99 guard does not block unlock".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "finality_certificate_root": self.finality_certificate_root,
            "audit_accounting_root": self.audit_accounting_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_signoff_root": self.operator_signoff_root,
            "unlock_guard_root": self.unlock_guard_root,
            "status": self.status.as_str(),
            "unlock_allowed": false,
            "release_allowed": false,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockBlocker {
    pub kind: UnlockBlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocker_root: String,
}

impl UnlockBlocker {
    pub fn new(kind: UnlockBlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        Self {
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocker_root: value_root(
                "WAVE99-UNLOCK-BLOCKER",
                &json!({
                    "kind": kind.as_str(),
                    "slot_kind": slot_kind.as_str(),
                    "evidence_root": evidence_root,
                    "active": true,
                }),
            ),
        }
    }

    pub fn canonical(guards: &[UnlockGuard], config: &Config) -> Vec<Self> {
        let mut blockers = Vec::new();
        for guard in guards {
            blockers.push(Self::new(
                UnlockBlockerKind::Wave98HoldoffRootActive,
                guard.slot_kind,
                &guard.wave98_holdoff_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::FinalityCertificateMissing,
                guard.slot_kind,
                &guard.finality_certificate_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::FinalityDepthTooLow,
                guard.slot_kind,
                &guard.finality_certificate_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::AuditAccountingOpen,
                guard.slot_kind,
                &guard.audit_accounting_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::RollbackGuardActive,
                guard.slot_kind,
                &guard.rollback_guard_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::CircuitBreakerActive,
                guard.slot_kind,
                &guard.circuit_breaker_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::ReviewerSignoffMissing,
                guard.slot_kind,
                &guard.reviewer_signoff_root,
            ));
            blockers.push(Self::new(
                UnlockBlockerKind::OperatorSignoffMissing,
                guard.slot_kind,
                &guard.operator_signoff_root,
            ));
        }
        blockers.push(Self::new(
            UnlockBlockerKind::UnlockCertificateBudgetZero,
            LaneSlotKind::OperatorSignoff,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            UnlockBlockerKind::ReleaseDenied,
            LaneSlotKind::OperatorSignoff,
            &config.wave98_release_denial_root,
        ));
        blockers.push(Self::new(
            UnlockBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &config.wave98_blocker_root,
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
                "WAVE99-COMMAND-HINT",
                &json!({
                    "kind": kind.as_str(),
                    "target_root": target_root,
                    "wave": config.wave,
                    "source_wave": config.source_wave,
                    "fail_closed_armed": config.fail_closed_armed,
                    "unlock_allowed": false,
                    "release_allowed": false,
                }),
            ),
        }
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandHintKind::HoldRelease,
                &config.wave98_release_denial_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepFailClosed,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::ImportWave98HoldoffRoots,
                &config.wave98_holdoff_ledger_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachFinalityCertificateRoot,
                &config.finality_certificate_anchor_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachAuditAccountingRoot,
                &config.wave98_audit_objection_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachRollbackGuardRoot,
                &config.wave98_blocker_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepCircuitBreakerArmed,
                &config.wave98_release_denial_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireReviewerSignoffRoot,
                &config.wave98_reviewer_hold_root,
                config,
            ),
            Self::new(
                CommandHintKind::RequireOperatorSignoffRoot,
                &config.state_root(),
                config,
            ),
            Self::new(
                CommandHintKind::DenyUnlockUntilBlockersClear,
                &config.wave98_holdoff_ledger_root,
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
    pub unlock_guards: u64,
    pub wave98_holdoff_roots: u64,
    pub finality_certificate_roots: u64,
    pub audit_accounting_roots: u64,
    pub rollback_guard_roots: u64,
    pub circuit_breaker_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub operator_signoff_roots: u64,
    pub blocker_count: u64,
    pub command_hint_count: u64,
    pub unlock_certificates: u64,
    pub released_claims: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "unlock_guards": self.unlock_guards,
            "wave98_holdoff_roots": self.wave98_holdoff_roots,
            "finality_certificate_roots": self.finality_certificate_roots,
            "audit_accounting_roots": self.audit_accounting_roots,
            "rollback_guard_roots": self.rollback_guard_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "blocker_count": self.blocker_count,
            "command_hint_count": self.command_hint_count,
            "unlock_certificates": self.unlock_certificates,
            "released_claims": self.released_claims,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub unlock_guards: Vec<UnlockGuard>,
    pub blockers: Vec<UnlockBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let unlock_guards = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| UnlockGuard::blocked(slot_kind, &config))
            .collect::<Vec<_>>();
        let blockers = UnlockBlocker::canonical(&unlock_guards, &config);
        let command_hints = CommandHint::canonical(&config);
        let mut state = Self {
            config,
            unlock_guards,
            blockers,
            command_hints,
            counters: Counters::default(),
        };
        state.counters = state.compute_counters();
        state.validate()?;
        Ok(state)
    }

    pub fn compute_counters(&self) -> Counters {
        let guard_count = self.unlock_guards.len() as u64;
        Counters {
            unlock_guards: guard_count,
            wave98_holdoff_roots: guard_count,
            finality_certificate_roots: guard_count,
            audit_accounting_roots: guard_count,
            rollback_guard_roots: guard_count,
            circuit_breaker_roots: guard_count,
            reviewer_signoff_roots: guard_count,
            operator_signoff_roots: guard_count,
            blocker_count: self.blockers.len() as u64,
            command_hint_count: self.command_hints.len() as u64,
            unlock_certificates: 0,
            released_claims: 0,
            raw_payload_records: 0,
        }
    }

    pub fn wave98_holdoff_root(&self) -> String {
        value_root(
            "WAVE99-SOURCE-WAVE98-HOLDOFF-ROOT",
            &json!({
                "wave98_holdoff_ledger_root": self.config.wave98_holdoff_ledger_root,
                "wave98_challenge_window_root": self.config.wave98_challenge_window_root,
                "wave98_audit_objection_root": self.config.wave98_audit_objection_root,
                "wave98_reviewer_hold_root": self.config.wave98_reviewer_hold_root,
                "wave98_blocker_root": self.config.wave98_blocker_root,
                "wave98_release_denial_root": self.config.wave98_release_denial_root,
            }),
        )
    }

    pub fn finality_certificate_root(&self) -> String {
        collection_root(
            "WAVE99-FINALITY-CERTIFICATE-ROOTS",
            self.unlock_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "finality_certificate_root": guard.finality_certificate_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn audit_accounting_root(&self) -> String {
        collection_root(
            "WAVE99-AUDIT-ACCOUNTING-ROOTS",
            self.unlock_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "audit_accounting_root": guard.audit_accounting_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn rollback_guard_root(&self) -> String {
        collection_root(
            "WAVE99-ROLLBACK-GUARD-ROOTS",
            self.unlock_guards
                .iter()
                .map(|guard| {
                    json!({
                        "slot_kind": guard.slot_kind.as_str(),
                        "rollback_guard_root": guard.rollback_guard_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn circuit_breaker_root(&self) -> String {
        collection_root(
            "WAVE99-CIRCUIT-BREAKER-ROOTS",
            self.unlock_guards
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
            "WAVE99-SIGNOFF-ROOTS",
            self.unlock_guards
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

    pub fn unlock_guard_root(&self) -> String {
        collection_root(
            "WAVE99-UNLOCK-GUARDS",
            self.unlock_guards
                .iter()
                .map(UnlockGuard::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE99-UNLOCK-BLOCKERS",
            self.blockers
                .iter()
                .map(UnlockBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE99-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn deterministic_roots_root(&self) -> String {
        value_root(
            "WAVE99-DETERMINISTIC-ROOTS",
            &json!({
                "config_root": self.config.state_root(),
                "wave98_holdoff_root": self.wave98_holdoff_root(),
                "finality_certificate_root": self.finality_certificate_root(),
                "audit_accounting_root": self.audit_accounting_root(),
                "rollback_guard_root": self.rollback_guard_root(),
                "circuit_breaker_root": self.circuit_breaker_root(),
                "signoff_root": self.signoff_root(),
                "unlock_guard_root": self.unlock_guard_root(),
                "blocker_root": self.blocker_root(),
                "command_hint_root": self.command_hint_root(),
            }),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE99-STATE-MATERIAL",
            &json!({
                "deterministic_roots_root": self.deterministic_roots_root(),
                "counters": self.counters.public_record(),
                "unlock_allowed": false,
                "release_allowed": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE99-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "finality_blockers_active": self.config.finality_blockers_active,
                "circuit_breaker_armed": self.config.circuit_breaker_armed,
                "rollback_guard_active": self.config.rollback_guard_active,
                "unlock_certificates": 0,
                "released_claims": 0,
                "unlock_denied": self.unlock_denied(),
                "release_denied": self.release_denied(),
                "production_denied": self.production_denied(),
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn unlock_denied(&self) -> bool {
        !self.config.unlock_allowed
            || self.config.finality_blockers_active
            || self.config.circuit_breaker_armed
            || self.config.rollback_guard_active
            || !self.blockers.is_empty()
            || self.counters.unlock_certificates == 0
    }

    pub fn release_denied(&self) -> bool {
        !self.config.release_allowed || self.unlock_denied()
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || self.release_denied()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.unlock_guards.len() as u64,
            self.config.min_slot_count,
        )?;
        for guard in &self.unlock_guards {
            guard.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("wave99 unlock guard contains raw payload records".to_string());
        }
        if self.counters.unlock_certificates > self.config.max_unlock_certificates {
            return Err("wave99 unlock certificates above configured limit".to_string());
        }
        if self.counters.released_claims > self.config.max_released_claims {
            return Err("wave99 released claims above configured limit".to_string());
        }
        if self.counters.unlock_certificates != 0 {
            return Err("devnet wave99 unlock guard must not mint certificates".to_string());
        }
        if self.counters.released_claims != 0 {
            return Err("devnet wave99 unlock guard must not release claims".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("wave99 unlock guard counters do not match state".to_string());
        }
        if !self.unlock_denied() {
            return Err("wave99 finality certificate guard cannot allow unlock".to_string());
        }
        if !self.release_denied() {
            return Err("wave99 finality certificate guard cannot allow release".to_string());
        }
        if !self.production_denied() {
            return Err("wave99 finality certificate guard cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave99_live_heavy_gate_release_claim_finality_certificate_unlock_guard_audit_security_lane_state",
            "config": self.config.public_record(),
            "wave98_holdoff_root": self.wave98_holdoff_root(),
            "finality_certificate_root": self.finality_certificate_root(),
            "audit_accounting_root": self.audit_accounting_root(),
            "rollback_guard_root": self.rollback_guard_root(),
            "circuit_breaker_root": self.circuit_breaker_root(),
            "signoff_root": self.signoff_root(),
            "unlock_guard_root": self.unlock_guard_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "deterministic_roots_root": self.deterministic_roots_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "unlock_denied": self.unlock_denied(),
            "release_denied": self.release_denied(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "unlock_guards": self.unlock_guards.iter().map(UnlockGuard::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(UnlockBlocker::public_record).collect::<Vec<_>>(),
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
    let unlock_guards = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| UnlockGuard::blocked(slot_kind, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        blockers: vec![UnlockBlocker::new(
            UnlockBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE99-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        command_hints: CommandHint::canonical(&config),
        counters: Counters::default(),
        unlock_guards,
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
        "MONERO-L2-PQ-FORCE-EXIT-WAVE99-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(CERTIFICATE_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE99-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(CERTIFICATE_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE99-EMPTY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(CERTIFICATE_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn bind_root(domain: &str, label: &str, source_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE99-BIND-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(CERTIFICATE_SUITE),
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
