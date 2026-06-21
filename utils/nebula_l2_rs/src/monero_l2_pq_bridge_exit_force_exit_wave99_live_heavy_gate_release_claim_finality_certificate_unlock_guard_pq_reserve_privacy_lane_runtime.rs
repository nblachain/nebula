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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const GUARD_SUITE: &str =
    "monero-l2-wave99-release-claim-finality-certificate-unlock-guard-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 99;
pub const HOLDOFF_WAVE: u64 = 98;
pub const RELEASE_CLAIM_SEAL_WAVE: u64 = 97;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 99;
pub const DEFAULT_MIN_FINALITY_CONFIRMATIONS: u64 = 64;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS: u64 = 8_000;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MAX_UNLOCK_RECORDS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-pq-reserve-privacy-lane-runtime";

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
pub enum UnlockRecordKind {
    Wave98Holdoff,
    FinalityCertificate,
    ReserveAccounting,
    PrivacyBudgetGuard,
    RollbackGuard,
    CircuitBreaker,
    OperatorReviewerSignoff,
    CommandHint,
}

impl UnlockRecordKind {
    pub fn all() -> [Self; 8] {
        [
            Self::Wave98Holdoff,
            Self::FinalityCertificate,
            Self::ReserveAccounting,
            Self::PrivacyBudgetGuard,
            Self::RollbackGuard,
            Self::CircuitBreaker,
            Self::OperatorReviewerSignoff,
            Self::CommandHint,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98Holdoff => "wave98_holdoff",
            Self::FinalityCertificate => "finality_certificate",
            Self::ReserveAccounting => "reserve_accounting",
            Self::PrivacyBudgetGuard => "privacy_budget_guard",
            Self::RollbackGuard => "rollback_guard",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorReviewerSignoff => "operator_reviewer_signoff",
            Self::CommandHint => "command_hint",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockStatus {
    Locked,
    Blocked,
    CertificateHeld,
    ShadowReady,
}

impl UnlockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Locked => "locked",
            Self::Blocked => "blocked",
            Self::CertificateHeld => "certificate_held",
            Self::ShadowReady => "shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockBlocker {
    HeavyGatesNotRun,
    ProductionDenied,
    ReleaseDenied,
    UnlockCertificateMissing,
    FinalityCertificateInactive,
    FinalityConfirmationsLow,
    Wave98HoldoffActive,
    ReserveAccountingBlocked,
    ReserveCoverageLow,
    PrivacyBudgetBlocked,
    PrivacyBudgetLow,
    LinkageRiskHigh,
    RollbackGuardActive,
    CircuitBreakerOpen,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    RootsOnlyBoundary,
    DeterministicRootMissing,
    DuplicateUnlockRoot,
    UnlockCapacityReached,
}

impl UnlockBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
            Self::ProductionDenied => "production_denied",
            Self::ReleaseDenied => "release_denied",
            Self::UnlockCertificateMissing => "unlock_certificate_missing",
            Self::FinalityCertificateInactive => "finality_certificate_inactive",
            Self::FinalityConfirmationsLow => "finality_confirmations_low",
            Self::Wave98HoldoffActive => "wave98_holdoff_active",
            Self::ReserveAccountingBlocked => "reserve_accounting_blocked",
            Self::ReserveCoverageLow => "reserve_coverage_low",
            Self::PrivacyBudgetBlocked => "privacy_budget_blocked",
            Self::PrivacyBudgetLow => "privacy_budget_low",
            Self::LinkageRiskHigh => "linkage_risk_high",
            Self::RollbackGuardActive => "rollback_guard_active",
            Self::CircuitBreakerOpen => "circuit_breaker_open",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DeterministicRootMissing => "deterministic_root_missing",
            Self::DuplicateUnlockRoot => "duplicate_unlock_root",
            Self::UnlockCapacityReached => "unlock_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    UnlockGuardActive,
    ReleaseShadowReady,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::UnlockGuardActive => "unlock_guard_active",
            Self::ReleaseShadowReady => "release_shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave98HoldoffRoots,
    AttachFinalityCertificateRoots,
    AttachReserveAccountingRoots,
    AttachPrivacyBudgetGuardRoots,
    AttachRollbackGuardRoots,
    AttachCircuitBreakerRoots,
    AttachOperatorReviewerSignoffRoots,
    KeepUnlockGuardLocked,
    PublishRootsOnlyUnlockGuardRecord,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave98HoldoffRoots,
            Self::AttachFinalityCertificateRoots,
            Self::AttachReserveAccountingRoots,
            Self::AttachPrivacyBudgetGuardRoots,
            Self::AttachRollbackGuardRoots,
            Self::AttachCircuitBreakerRoots,
            Self::AttachOperatorReviewerSignoffRoots,
            Self::KeepUnlockGuardLocked,
            Self::PublishRootsOnlyUnlockGuardRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave98HoldoffRoots => "import_wave98_holdoff_roots",
            Self::AttachFinalityCertificateRoots => "attach_finality_certificate_roots",
            Self::AttachReserveAccountingRoots => "attach_reserve_accounting_roots",
            Self::AttachPrivacyBudgetGuardRoots => "attach_privacy_budget_guard_roots",
            Self::AttachRollbackGuardRoots => "attach_rollback_guard_roots",
            Self::AttachCircuitBreakerRoots => "attach_circuit_breaker_roots",
            Self::AttachOperatorReviewerSignoffRoots => "attach_operator_reviewer_signoff_roots",
            Self::KeepUnlockGuardLocked => "keep_unlock_guard_locked",
            Self::PublishRootsOnlyUnlockGuardRecord => "publish_roots_only_unlock_guard_record",
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
    pub holdoff_wave: u64,
    pub release_claim_seal_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_finality_confirmations: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_privacy_budget_remaining_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub fail_closed: bool,
    pub heavy_gates_ran: bool,
    pub production_allowed: bool,
    pub release_allowed: bool,
    pub unlock_certificate_allowed: bool,
    pub wave98_holdoff_active: bool,
    pub rollback_guard_active: bool,
    pub circuit_breaker_open: bool,
    pub roots_only_public_record: bool,
    pub max_unlock_records: usize,
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
            holdoff_wave: HOLDOFF_WAVE,
            release_claim_seal_wave: RELEASE_CLAIM_SEAL_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_finality_confirmations: DEFAULT_MIN_FINALITY_CONFIRMATIONS,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_privacy_budget_remaining_bps: DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            fail_closed: true,
            heavy_gates_ran: false,
            production_allowed: false,
            release_allowed: false,
            unlock_certificate_allowed: false,
            wave98_holdoff_active: true,
            rollback_guard_active: true,
            circuit_breaker_open: true,
            roots_only_public_record: true,
            max_unlock_records: DEFAULT_MAX_UNLOCK_RECORDS,
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
            "guard_suite": self.guard_suite,
            "wave": self.wave,
            "holdoff_wave": self.holdoff_wave,
            "release_claim_seal_wave": self.release_claim_seal_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_finality_confirmations": self.min_finality_confirmations,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_privacy_budget_remaining_bps": self.min_privacy_budget_remaining_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "fail_closed": self.fail_closed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "production_allowed": self.production_allowed,
            "release_allowed": self.release_allowed,
            "unlock_certificate_allowed": self.unlock_certificate_allowed,
            "wave98_holdoff_active": self.wave98_holdoff_active,
            "rollback_guard_active": self.rollback_guard_active,
            "circuit_breaker_open": self.circuit_breaker_open,
            "roots_only_public_record": self.roots_only_public_record,
            "max_unlock_records": self.max_unlock_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardRoots {
    pub wave98_holdoff_root: String,
    pub wave98_pq_reserve_privacy_root: String,
    pub wave98_blocker_root: String,
    pub finality_certificate_root: String,
    pub finality_certificate_set_root: String,
    pub finality_witness_root: String,
    pub reserve_accounting_root: String,
    pub reserve_liability_root: String,
    pub reserve_coverage_root: String,
    pub privacy_budget_guard_root: String,
    pub privacy_spend_root: String,
    pub non_linkage_root: String,
    pub rollback_guard_root: String,
    pub rollback_fence_root: String,
    pub circuit_breaker_root: String,
    pub circuit_breaker_reason_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub deterministic_unlock_guard_root: String,
}

impl GuardRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn devnet() -> Self {
        Self {
            wave98_holdoff_root: sample_root("wave98-holdoff-active"),
            wave98_pq_reserve_privacy_root: sample_root("wave98-pq-reserve-privacy-holdoff"),
            wave98_blocker_root: blocker_root(&default_active_blockers()),
            finality_certificate_root: sample_root("finality-certificate-held"),
            finality_certificate_set_root: sample_root("finality-certificate-set-held"),
            finality_witness_root: sample_root("finality-witness-root-held"),
            reserve_accounting_root: sample_root("reserve-accounting-blocked"),
            reserve_liability_root: sample_root("reserve-liability-root"),
            reserve_coverage_root: sample_root("reserve-coverage-low"),
            privacy_budget_guard_root: sample_root("privacy-budget-guard-blocked"),
            privacy_spend_root: sample_root("privacy-spend-root"),
            non_linkage_root: sample_root("non-linkage-root"),
            rollback_guard_root: sample_root("rollback-guard-active"),
            rollback_fence_root: sample_root("rollback-fence-root"),
            circuit_breaker_root: sample_root("circuit-breaker-open"),
            circuit_breaker_reason_root: sample_root("circuit-breaker-reason-root"),
            operator_signoff_root: sample_root("operator-signoff-missing-root"),
            reviewer_signoff_root: sample_root("reviewer-signoff-missing-root"),
            command_hint_root: operator_command_root(&OperatorCommand::sequence()),
            deterministic_unlock_guard_root: deterministic_root("devnet"),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "wave98_pq_reserve_privacy_root": self.wave98_pq_reserve_privacy_root,
            "wave98_blocker_root": self.wave98_blocker_root,
            "finality_certificate_root": self.finality_certificate_root,
            "finality_certificate_set_root": self.finality_certificate_set_root,
            "finality_witness_root": self.finality_witness_root,
            "reserve_accounting_root": self.reserve_accounting_root,
            "reserve_liability_root": self.reserve_liability_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_budget_guard_root": self.privacy_budget_guard_root,
            "privacy_spend_root": self.privacy_spend_root,
            "non_linkage_root": self.non_linkage_root,
            "rollback_guard_root": self.rollback_guard_root,
            "rollback_fence_root": self.rollback_fence_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "circuit_breaker_reason_root": self.circuit_breaker_reason_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "command_hint_root": self.command_hint_root,
            "deterministic_unlock_guard_root": self.deterministic_unlock_guard_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("guard_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardMetrics {
    pub finality_confirmations: u64,
    pub reserve_coverage_bps: u64,
    pub privacy_budget_remaining_bps: u64,
    pub linkage_risk_bps: u64,
    pub certificate_active: bool,
    pub wave98_holdoff_cleared: bool,
    pub reserve_accounting_cleared: bool,
    pub privacy_budget_cleared: bool,
    pub rollback_guard_cleared: bool,
    pub circuit_breaker_closed: bool,
    pub operator_signed: bool,
    pub reviewer_signed: bool,
    pub root_only: bool,
}

impl Default for GuardMetrics {
    fn default() -> Self {
        Self {
            finality_confirmations: 0,
            reserve_coverage_bps: 0,
            privacy_budget_remaining_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            certificate_active: false,
            wave98_holdoff_cleared: false,
            reserve_accounting_cleared: false,
            privacy_budget_cleared: false,
            rollback_guard_cleared: false,
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
pub struct UnlockGuardRecord {
    pub unlock_root: String,
    pub record_kind: UnlockRecordKind,
    pub status: UnlockStatus,
    pub roots: GuardRoots,
    pub metrics: GuardMetrics,
    pub blockers: Vec<UnlockBlocker>,
}

impl UnlockGuardRecord {
    pub fn evaluate(
        config: &Config,
        record_kind: UnlockRecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Self {
        let blockers = unlock_blockers(config, &roots, &metrics);
        let status = if blockers.is_empty() {
            UnlockStatus::ShadowReady
        } else if !metrics.certificate_active || !config.unlock_certificate_allowed {
            UnlockStatus::CertificateHeld
        } else if config.wave98_holdoff_active || config.rollback_guard_active {
            UnlockStatus::Locked
        } else {
            UnlockStatus::Blocked
        };
        let unlock_root = guard_record_root(record_kind, status, &roots, &metrics, &blockers);
        Self {
            unlock_root,
            record_kind,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "unlock_root": self.unlock_root,
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "roots": self.roots.public_record(),
            "metrics": self.metrics.public_record(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock_guard_record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecordState {
    pub record_kind: UnlockRecordKind,
    pub status: UnlockStatus,
    pub unlock_root: String,
    pub blocker_root: String,
    pub release_denied: bool,
    pub command_hint_root: String,
}

impl RecordState {
    pub fn blocked(record_kind: UnlockRecordKind) -> Self {
        let blockers = default_active_blockers();
        Self {
            record_kind,
            status: UnlockStatus::Locked,
            unlock_root: empty_root("record", record_kind.as_str()),
            blocker_root: blocker_root(&blockers),
            release_denied: true,
            command_hint_root: command_hint_root(record_kind),
        }
    }

    pub fn from_record(record: &UnlockGuardRecord) -> Self {
        Self {
            record_kind: record.record_kind,
            status: record.status,
            unlock_root: record.unlock_root.clone(),
            blocker_root: blocker_root(&record.blockers),
            release_denied: record.status != UnlockStatus::ShadowReady,
            command_hint_root: command_hint_root(record.record_kind),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "unlock_root": self.unlock_root,
            "blocker_root": self.blocker_root,
            "release_denied": self.release_denied,
            "command_hint_root": self.command_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("record_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardCounters {
    pub record_count: u64,
    pub locked_count: u64,
    pub certificate_held_count: u64,
    pub blocked_count: u64,
    pub shadow_ready_count: u64,
    pub release_denied_count: u64,
    pub blocker_count: u64,
}

impl GuardCounters {
    pub fn from_parts(
        records: &[UnlockGuardRecord],
        record_states: &BTreeMap<UnlockRecordKind, RecordState>,
    ) -> Self {
        let mut counters = Self {
            record_count: records.len() as u64,
            blocker_count: records
                .iter()
                .map(|record| record.blockers.len() as u64)
                .sum(),
            ..Self::default()
        };
        for state in record_states.values() {
            match state.status {
                UnlockStatus::Locked => {
                    counters.locked_count = counters.locked_count.saturating_add(1);
                }
                UnlockStatus::CertificateHeld => {
                    counters.certificate_held_count =
                        counters.certificate_held_count.saturating_add(1);
                }
                UnlockStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                UnlockStatus::ShadowReady => {
                    counters.shadow_ready_count = counters.shadow_ready_count.saturating_add(1);
                }
            }
        }
        counters.release_denied_count = record_states
            .values()
            .filter(|state| state.release_denied)
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
    pub unlock_records: Vec<UnlockGuardRecord>,
    pub record_states: BTreeMap<UnlockRecordKind, RecordState>,
    pub counters: GuardCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, unlock_records: Vec<UnlockGuardRecord>) -> Result<Self> {
        if unlock_records.len() > config.max_unlock_records {
            return Err(UnlockBlocker::UnlockCapacityReached.as_str().to_string());
        }
        let mut seen = BTreeSet::new();
        for record in &unlock_records {
            if !seen.insert(record.unlock_root.clone()) {
                return Err(UnlockBlocker::DuplicateUnlockRoot.as_str().to_string());
            }
        }
        let mut record_states = BTreeMap::new();
        for record_kind in UnlockRecordKind::all() {
            let maybe_record = unlock_records
                .iter()
                .rev()
                .find(|record| record.record_kind == record_kind);
            let state = match maybe_record {
                Some(record) => RecordState::from_record(record),
                None => RecordState::blocked(record_kind),
            };
            record_states.insert(record_kind, state);
        }
        let counters = GuardCounters::from_parts(&unlock_records, &record_states);
        Ok(Self {
            config,
            unlock_records,
            record_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn append_unlock_record(
        &self,
        record_kind: UnlockRecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Result<Self> {
        let mut unlock_records = self.unlock_records.clone();
        if unlock_records.len() >= self.config.max_unlock_records {
            return Err(UnlockBlocker::UnlockCapacityReached.as_str().to_string());
        }
        let record = UnlockGuardRecord::evaluate(&self.config, record_kind, roots, metrics);
        if unlock_records
            .iter()
            .any(|item| item.unlock_root == record.unlock_root)
        {
            return Err(UnlockBlocker::DuplicateUnlockRoot.as_str().to_string());
        }
        unlock_records.push(record);
        Self::new(self.config.clone(), unlock_records)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.unlock_records.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.release_denied_count == 0
            && self.counters.shadow_ready_count == UnlockRecordKind::all().len() as u64
        {
            RuntimeVerdict::ReleaseShadowReady
        } else {
            RuntimeVerdict::UnlockGuardActive
        }
    }

    pub fn public_record(&self) -> Value {
        let state_records = self
            .record_states
            .values()
            .map(RecordState::public_record)
            .collect::<Vec<_>>();
        let unlock_records = self
            .unlock_records
            .iter()
            .map(UnlockGuardRecord::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "record_state_root": list_root("record_states", state_records),
            "unlock_guard_record_root": list_root("unlock_guard_records", unlock_records),
            "wave98_holdoff_root": wave98_holdoff_root(&self.unlock_records),
            "finality_certificate_root": finality_certificate_root(&self.unlock_records),
            "reserve_accounting_root": reserve_accounting_root(&self.unlock_records),
            "privacy_budget_guard_root": privacy_budget_guard_root(&self.unlock_records),
            "rollback_guard_root": rollback_guard_root(&self.unlock_records),
            "circuit_breaker_root": circuit_breaker_root(&self.unlock_records),
            "operator_reviewer_signoff_root": operator_reviewer_signoff_root(&self.unlock_records),
            "command_hint_root": operator_command_root(&self.operator_commands),
            "deterministic_root": deterministic_state_root(&self.unlock_records, &self.record_states),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.unlock_records, &self.record_states),
            "active_unlock_blocker_root": blocker_root(&default_active_blockers()),
            "verdict": self.verdict().as_str(),
            "release_denied": self.verdict() != RuntimeVerdict::ReleaseShadowReady,
            "unlock_certificate_count": 0_u64,
            "released_claim_count": 0_u64,
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
                HashPart::Str(&records_root(&self.unlock_records)),
                HashPart::Str(&record_states_root(&self.record_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_blockers())),
                HashPart::Str(&operator_command_root(&self.operator_commands)),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let roots = GuardRoots::devnet();
    let metrics = GuardMetrics::default();
    let records = UnlockRecordKind::all()
        .into_iter()
        .map(|record_kind| {
            UnlockGuardRecord::evaluate(&config, record_kind, roots.clone(), metrics.clone())
        })
        .collect::<Vec<_>>();
    match State::new(config.clone(), records) {
        Ok(state) => state,
        Err(reason) => {
            let fallback_roots = GuardRoots {
                deterministic_unlock_guard_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..GuardRoots::empty()
            };
            let fallback_record = UnlockGuardRecord::evaluate(
                &config,
                UnlockRecordKind::CommandHint,
                fallback_roots,
                GuardMetrics::default(),
            );
            let fallback_records = vec![fallback_record];
            let record_states = UnlockRecordKind::all()
                .into_iter()
                .map(|record_kind| (record_kind, RecordState::blocked(record_kind)))
                .collect::<BTreeMap<_, _>>();
            State {
                config,
                unlock_records: Vec::new(),
                counters: GuardCounters::from_parts(&fallback_records, &record_states),
                record_states,
                operator_commands: OperatorCommand::sequence(),
            }
        }
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn unlock_blockers(
    config: &Config,
    roots: &GuardRoots,
    metrics: &GuardMetrics,
) -> Vec<UnlockBlocker> {
    let mut blockers = Vec::new();
    if !config.heavy_gates_ran {
        blockers.push(UnlockBlocker::HeavyGatesNotRun);
    }
    if !config.production_allowed {
        blockers.push(UnlockBlocker::ProductionDenied);
    }
    if !config.release_allowed {
        blockers.push(UnlockBlocker::ReleaseDenied);
    }
    if !config.unlock_certificate_allowed {
        blockers.push(UnlockBlocker::UnlockCertificateMissing);
    }
    if !metrics.certificate_active {
        blockers.push(UnlockBlocker::FinalityCertificateInactive);
    }
    if metrics.finality_confirmations < config.min_finality_confirmations {
        blockers.push(UnlockBlocker::FinalityConfirmationsLow);
    }
    if config.wave98_holdoff_active || !metrics.wave98_holdoff_cleared {
        blockers.push(UnlockBlocker::Wave98HoldoffActive);
    }
    if !metrics.reserve_accounting_cleared {
        blockers.push(UnlockBlocker::ReserveAccountingBlocked);
    }
    if metrics.reserve_coverage_bps < config.min_reserve_coverage_bps {
        blockers.push(UnlockBlocker::ReserveCoverageLow);
    }
    if !metrics.privacy_budget_cleared {
        blockers.push(UnlockBlocker::PrivacyBudgetBlocked);
    }
    if metrics.privacy_budget_remaining_bps < config.min_privacy_budget_remaining_bps {
        blockers.push(UnlockBlocker::PrivacyBudgetLow);
    }
    if metrics.linkage_risk_bps > config.max_linkage_risk_bps {
        blockers.push(UnlockBlocker::LinkageRiskHigh);
    }
    if config.rollback_guard_active || !metrics.rollback_guard_cleared {
        blockers.push(UnlockBlocker::RollbackGuardActive);
    }
    if config.circuit_breaker_open || !metrics.circuit_breaker_closed {
        blockers.push(UnlockBlocker::CircuitBreakerOpen);
    }
    if !metrics.operator_signed {
        blockers.push(UnlockBlocker::OperatorSignoffMissing);
    }
    if !metrics.reviewer_signed {
        blockers.push(UnlockBlocker::ReviewerSignoffMissing);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(UnlockBlocker::RootsOnlyBoundary);
    }
    if roots.deterministic_unlock_guard_root.is_empty() {
        blockers.push(UnlockBlocker::DeterministicRootMissing);
    }
    if roots.wave98_holdoff_root.is_empty()
        || roots.wave98_pq_reserve_privacy_root.is_empty()
        || roots.wave98_blocker_root.is_empty()
        || roots.finality_certificate_root.is_empty()
        || roots.finality_certificate_set_root.is_empty()
        || roots.finality_witness_root.is_empty()
        || roots.reserve_accounting_root.is_empty()
        || roots.reserve_liability_root.is_empty()
        || roots.reserve_coverage_root.is_empty()
        || roots.privacy_budget_guard_root.is_empty()
        || roots.privacy_spend_root.is_empty()
        || roots.non_linkage_root.is_empty()
        || roots.rollback_guard_root.is_empty()
        || roots.rollback_fence_root.is_empty()
        || roots.circuit_breaker_root.is_empty()
        || roots.circuit_breaker_reason_root.is_empty()
        || roots.operator_signoff_root.is_empty()
        || roots.reviewer_signoff_root.is_empty()
        || roots.command_hint_root.is_empty()
    {
        blockers.push(UnlockBlocker::UnlockCertificateMissing);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_blockers() -> Vec<UnlockBlocker> {
    vec![
        UnlockBlocker::HeavyGatesNotRun,
        UnlockBlocker::ProductionDenied,
        UnlockBlocker::ReleaseDenied,
        UnlockBlocker::UnlockCertificateMissing,
        UnlockBlocker::FinalityCertificateInactive,
        UnlockBlocker::FinalityConfirmationsLow,
        UnlockBlocker::Wave98HoldoffActive,
        UnlockBlocker::ReserveAccountingBlocked,
        UnlockBlocker::ReserveCoverageLow,
        UnlockBlocker::PrivacyBudgetBlocked,
        UnlockBlocker::PrivacyBudgetLow,
        UnlockBlocker::LinkageRiskHigh,
        UnlockBlocker::RollbackGuardActive,
        UnlockBlocker::CircuitBreakerOpen,
        UnlockBlocker::OperatorSignoffMissing,
        UnlockBlocker::ReviewerSignoffMissing,
    ]
}

fn guard_record_root(
    record_kind: UnlockRecordKind,
    status: UnlockStatus,
    roots: &GuardRoots,
    metrics: &GuardMetrics,
    blockers: &[UnlockBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:unlock-guard-record"),
        &[
            HashPart::Str(record_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn records_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| Value::String(record.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:records"), &leaves)
}

fn record_states_root(record_states: &BTreeMap<UnlockRecordKind, RecordState>) -> String {
    let leaves = record_states
        .values()
        .map(|state| Value::String(state.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:record-states"), &leaves)
}

fn wave98_holdoff_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "wave98_holdoff_root": record.roots.wave98_holdoff_root,
                "wave98_pq_reserve_privacy_root": record.roots.wave98_pq_reserve_privacy_root,
                "wave98_blocker_root": record.roots.wave98_blocker_root,
                "wave98_holdoff_cleared": record.metrics.wave98_holdoff_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:wave98-holdoff-roots"), &leaves)
}

fn finality_certificate_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "finality_certificate_root": record.roots.finality_certificate_root,
                "finality_certificate_set_root": record.roots.finality_certificate_set_root,
                "finality_witness_root": record.roots.finality_witness_root,
                "finality_confirmations": record.metrics.finality_confirmations,
                "certificate_active": record.metrics.certificate_active,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:finality-certificate-roots"), &leaves)
}

fn reserve_accounting_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "reserve_accounting_root": record.roots.reserve_accounting_root,
                "reserve_liability_root": record.roots.reserve_liability_root,
                "reserve_coverage_root": record.roots.reserve_coverage_root,
                "reserve_coverage_bps": record.metrics.reserve_coverage_bps,
                "reserve_accounting_cleared": record.metrics.reserve_accounting_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:reserve-accounting-roots"), &leaves)
}

fn privacy_budget_guard_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "privacy_budget_guard_root": record.roots.privacy_budget_guard_root,
                "privacy_spend_root": record.roots.privacy_spend_root,
                "non_linkage_root": record.roots.non_linkage_root,
                "privacy_budget_remaining_bps": record.metrics.privacy_budget_remaining_bps,
                "linkage_risk_bps": record.metrics.linkage_risk_bps,
                "privacy_budget_cleared": record.metrics.privacy_budget_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:privacy-budget-guard-roots"), &leaves)
}

fn rollback_guard_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "rollback_guard_root": record.roots.rollback_guard_root,
                "rollback_fence_root": record.roots.rollback_fence_root,
                "rollback_guard_cleared": record.metrics.rollback_guard_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:rollback-guard-roots"), &leaves)
}

fn circuit_breaker_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "circuit_breaker_root": record.roots.circuit_breaker_root,
                "circuit_breaker_reason_root": record.roots.circuit_breaker_reason_root,
                "circuit_breaker_closed": record.metrics.circuit_breaker_closed,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:circuit-breaker-roots"), &leaves)
}

fn operator_reviewer_signoff_root(records: &[UnlockGuardRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "operator_signoff_root": record.roots.operator_signoff_root,
                "reviewer_signoff_root": record.roots.reviewer_signoff_root,
                "operator_signed": record.metrics.operator_signed,
                "reviewer_signed": record.metrics.reviewer_signed,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        &format!("{DOMAIN}:operator-reviewer-signoff-roots"),
        &leaves,
    )
}

fn deterministic_state_root(
    records: &[UnlockGuardRecord],
    record_states: &BTreeMap<UnlockRecordKind, RecordState>,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:deterministic-state"),
        &[
            HashPart::Str(&records_root(records)),
            HashPart::Str(&record_states_root(record_states)),
            HashPart::Str(&blocker_root(&default_active_blockers())),
        ],
        32,
    )
}

fn all_blockers_root(
    records: &[UnlockGuardRecord],
    record_states: &BTreeMap<UnlockRecordKind, RecordState>,
) -> String {
    let mut leaves = records
        .iter()
        .flat_map(|record| {
            record.blockers.iter().map(|blocker| {
                json!({
                    "unlock_root": record.unlock_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(
        record_states
            .values()
            .filter(|state| state.release_denied)
            .map(|state| {
                json!({
                    "record_kind": state.record_kind.as_str(),
                    "blocker_root": state.blocker_root,
                })
            }),
    );
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn blocker_root(blockers: &[UnlockBlocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| json!({ "blocker": blocker.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn operator_command_root(commands: &[OperatorCommand]) -> String {
    let leaves = commands
        .iter()
        .map(|command| json!({ "command": command.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:operator-commands"), &leaves)
}

fn command_hint_root(record_kind: UnlockRecordKind) -> String {
    domain_hash(
        &format!("{DOMAIN}:command-hint"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(record_kind.as_str()),
            HashPart::Str(GUARD_SUITE),
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

fn list_root(kind: &str, leaves: Vec<Value>) -> String {
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn empty_root(kind: &str, record_kind: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:empty"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(record_kind),
        ],
        32,
    )
}

fn dedupe_blockers(blockers: &mut Vec<UnlockBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
