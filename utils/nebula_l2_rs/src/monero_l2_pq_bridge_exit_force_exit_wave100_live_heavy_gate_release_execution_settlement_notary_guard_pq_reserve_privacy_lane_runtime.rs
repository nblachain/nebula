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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave100-live-heavy-gate-release-execution-settlement-notary-guard-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const NOTARY_GUARD_SUITE: &str =
    "monero-l2-wave100-release-execution-settlement-notary-guard-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 100;
pub const WAVE99_UNLOCK_GUARD_WAVE: u64 = 99;
pub const WAVE98_HOLDOFF_WAVE: u64 = 98;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 100;
pub const DEFAULT_MIN_NOTARY_QUORUM_BPS: u64 = 6_700;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS: u64 = 8_000;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MAX_SETTLEMENT_DRIFT_BPS: u64 = 5;
pub const DEFAULT_MAX_EXECUTION_RECORDS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave100-live-heavy-gate-release-execution-settlement-notary-guard-pq-reserve-privacy-lane-runtime";

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
pub enum ExecutionRecordKind {
    Wave99UnlockGuard,
    ReserveExecutionBundle,
    PrivacySettlementAccounting,
    NotaryQuorum,
    PayoutEnvelope,
    RollbackSentinel,
    CircuitBreaker,
    OperatorReviewerSignoff,
    CommandHint,
}

impl ExecutionRecordKind {
    pub fn all() -> [Self; 9] {
        [
            Self::Wave99UnlockGuard,
            Self::ReserveExecutionBundle,
            Self::PrivacySettlementAccounting,
            Self::NotaryQuorum,
            Self::PayoutEnvelope,
            Self::RollbackSentinel,
            Self::CircuitBreaker,
            Self::OperatorReviewerSignoff,
            Self::CommandHint,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuard => "wave99_unlock_guard",
            Self::ReserveExecutionBundle => "reserve_execution_bundle",
            Self::PrivacySettlementAccounting => "privacy_settlement_accounting",
            Self::NotaryQuorum => "notary_quorum",
            Self::PayoutEnvelope => "payout_envelope",
            Self::RollbackSentinel => "rollback_sentinel",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorReviewerSignoff => "operator_reviewer_signoff",
            Self::CommandHint => "command_hint",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Denied,
    Blocked,
    NotaryHeld,
    SettlementShadowReady,
}

impl ExecutionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::Blocked => "blocked",
            Self::NotaryHeld => "notary_held",
            Self::SettlementShadowReady => "settlement_shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionBlocker {
    HeavyGatesNotRun,
    ProductionDenied,
    ReleaseExecutionDenied,
    SettlementDenied,
    Wave99UnlockGuardActive,
    Wave99UnlockGuardRootMissing,
    ReserveExecutionBundleMissing,
    ReserveExecutionBundleBlocked,
    ReserveCoverageLow,
    PrivacySettlementBlocked,
    PrivacyBudgetLow,
    LinkageRiskHigh,
    NotaryQuorumMissing,
    NotaryQuorumLow,
    PayoutEnvelopeMissing,
    PayoutEnvelopeBlocked,
    SettlementDriftHigh,
    RollbackSentinelActive,
    CircuitBreakerOpen,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    RootsOnlyBoundary,
    DeterministicRootMissing,
    DuplicateExecutionRoot,
    ExecutionCapacityReached,
}

impl ExecutionBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
            Self::ProductionDenied => "production_denied",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::SettlementDenied => "settlement_denied",
            Self::Wave99UnlockGuardActive => "wave99_unlock_guard_active",
            Self::Wave99UnlockGuardRootMissing => "wave99_unlock_guard_root_missing",
            Self::ReserveExecutionBundleMissing => "reserve_execution_bundle_missing",
            Self::ReserveExecutionBundleBlocked => "reserve_execution_bundle_blocked",
            Self::ReserveCoverageLow => "reserve_coverage_low",
            Self::PrivacySettlementBlocked => "privacy_settlement_blocked",
            Self::PrivacyBudgetLow => "privacy_budget_low",
            Self::LinkageRiskHigh => "linkage_risk_high",
            Self::NotaryQuorumMissing => "notary_quorum_missing",
            Self::NotaryQuorumLow => "notary_quorum_low",
            Self::PayoutEnvelopeMissing => "payout_envelope_missing",
            Self::PayoutEnvelopeBlocked => "payout_envelope_blocked",
            Self::SettlementDriftHigh => "settlement_drift_high",
            Self::RollbackSentinelActive => "rollback_sentinel_active",
            Self::CircuitBreakerOpen => "circuit_breaker_open",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DeterministicRootMissing => "deterministic_root_missing",
            Self::DuplicateExecutionRoot => "duplicate_execution_root",
            Self::ExecutionCapacityReached => "execution_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    NotaryGuardActive,
    ReleaseExecutionShadowReady,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::NotaryGuardActive => "notary_guard_active",
            Self::ReleaseExecutionShadowReady => "release_execution_shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave99UnlockGuardRoots,
    AttachReserveExecutionBundleRoots,
    AttachPrivacySettlementAccountingRoots,
    AttachNotaryQuorumRoots,
    AttachPayoutEnvelopeRoots,
    AttachRollbackSentinelRoots,
    AttachCircuitBreakerRoots,
    AttachOperatorReviewerSignoffRoots,
    KeepReleaseExecutionDenied,
    PublishRootsOnlySettlementNotaryGuard,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave99UnlockGuardRoots,
            Self::AttachReserveExecutionBundleRoots,
            Self::AttachPrivacySettlementAccountingRoots,
            Self::AttachNotaryQuorumRoots,
            Self::AttachPayoutEnvelopeRoots,
            Self::AttachRollbackSentinelRoots,
            Self::AttachCircuitBreakerRoots,
            Self::AttachOperatorReviewerSignoffRoots,
            Self::KeepReleaseExecutionDenied,
            Self::PublishRootsOnlySettlementNotaryGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave99UnlockGuardRoots => "import_wave99_unlock_guard_roots",
            Self::AttachReserveExecutionBundleRoots => "attach_reserve_execution_bundle_roots",
            Self::AttachPrivacySettlementAccountingRoots => {
                "attach_privacy_settlement_accounting_roots"
            }
            Self::AttachNotaryQuorumRoots => "attach_notary_quorum_roots",
            Self::AttachPayoutEnvelopeRoots => "attach_payout_envelope_roots",
            Self::AttachRollbackSentinelRoots => "attach_rollback_sentinel_roots",
            Self::AttachCircuitBreakerRoots => "attach_circuit_breaker_roots",
            Self::AttachOperatorReviewerSignoffRoots => "attach_operator_reviewer_signoff_roots",
            Self::KeepReleaseExecutionDenied => "keep_release_execution_denied",
            Self::PublishRootsOnlySettlementNotaryGuard => {
                "publish_roots_only_settlement_notary_guard"
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub notary_guard_suite: String,
    pub wave: u64,
    pub wave99_unlock_guard_wave: u64,
    pub wave98_holdoff_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_notary_quorum_bps: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_privacy_budget_remaining_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub max_settlement_drift_bps: u64,
    pub fail_closed: bool,
    pub heavy_gates_ran: bool,
    pub production_allowed: bool,
    pub release_execution_allowed: bool,
    pub settlement_allowed: bool,
    pub wave99_unlock_guard_active: bool,
    pub reserve_execution_blocked: bool,
    pub privacy_settlement_blocked: bool,
    pub rollback_sentinel_active: bool,
    pub circuit_breaker_open: bool,
    pub roots_only_public_record: bool,
    pub max_execution_records: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            notary_guard_suite: NOTARY_GUARD_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            wave99_unlock_guard_wave: WAVE99_UNLOCK_GUARD_WAVE,
            wave98_holdoff_wave: WAVE98_HOLDOFF_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_notary_quorum_bps: DEFAULT_MIN_NOTARY_QUORUM_BPS,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_privacy_budget_remaining_bps: DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            max_settlement_drift_bps: DEFAULT_MAX_SETTLEMENT_DRIFT_BPS,
            fail_closed: true,
            heavy_gates_ran: false,
            production_allowed: false,
            release_execution_allowed: false,
            settlement_allowed: false,
            wave99_unlock_guard_active: true,
            reserve_execution_blocked: true,
            privacy_settlement_blocked: true,
            rollback_sentinel_active: true,
            circuit_breaker_open: true,
            roots_only_public_record: true,
            max_execution_records: DEFAULT_MAX_EXECUTION_RECORDS,
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
            "notary_guard_suite": self.notary_guard_suite,
            "wave": self.wave,
            "wave99_unlock_guard_wave": self.wave99_unlock_guard_wave,
            "wave98_holdoff_wave": self.wave98_holdoff_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_notary_quorum_bps": self.min_notary_quorum_bps,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_privacy_budget_remaining_bps": self.min_privacy_budget_remaining_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "max_settlement_drift_bps": self.max_settlement_drift_bps,
            "fail_closed": self.fail_closed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "production_allowed": self.production_allowed,
            "release_execution_allowed": self.release_execution_allowed,
            "settlement_allowed": self.settlement_allowed,
            "wave99_unlock_guard_active": self.wave99_unlock_guard_active,
            "reserve_execution_blocked": self.reserve_execution_blocked,
            "privacy_settlement_blocked": self.privacy_settlement_blocked,
            "rollback_sentinel_active": self.rollback_sentinel_active,
            "circuit_breaker_open": self.circuit_breaker_open,
            "roots_only_public_record": self.roots_only_public_record,
            "max_execution_records": self.max_execution_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardRoots {
    pub wave99_unlock_guard_root: String,
    pub wave99_pq_reserve_privacy_root: String,
    pub wave99_unlock_blocker_root: String,
    pub reserve_execution_bundle_root: String,
    pub reserve_execution_commitment_root: String,
    pub reserve_execution_liability_root: String,
    pub privacy_settlement_accounting_root: String,
    pub privacy_budget_accounting_root: String,
    pub non_linkage_accounting_root: String,
    pub notary_quorum_root: String,
    pub notary_attestation_set_root: String,
    pub notary_epoch_root: String,
    pub payout_envelope_root: String,
    pub payout_envelope_commitment_root: String,
    pub payout_schedule_root: String,
    pub rollback_sentinel_root: String,
    pub rollback_fence_root: String,
    pub circuit_breaker_root: String,
    pub circuit_breaker_reason_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub deterministic_settlement_notary_root: String,
}

impl GuardRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn devnet() -> Self {
        Self {
            wave99_unlock_guard_root: sample_root("wave99-unlock-guard-active"),
            wave99_pq_reserve_privacy_root: sample_root("wave99-pq-reserve-privacy-guard"),
            wave99_unlock_blocker_root: blocker_root(&default_active_blockers()),
            reserve_execution_bundle_root: sample_root("reserve-execution-bundle-held"),
            reserve_execution_commitment_root: sample_root("reserve-execution-commitment-root"),
            reserve_execution_liability_root: sample_root("reserve-execution-liability-root"),
            privacy_settlement_accounting_root: sample_root(
                "privacy-settlement-accounting-blocked",
            ),
            privacy_budget_accounting_root: sample_root("privacy-budget-accounting-root"),
            non_linkage_accounting_root: sample_root("non-linkage-accounting-root"),
            notary_quorum_root: sample_root("notary-quorum-held"),
            notary_attestation_set_root: sample_root("notary-attestation-set-root"),
            notary_epoch_root: sample_root("notary-epoch-root"),
            payout_envelope_root: sample_root("payout-envelope-held"),
            payout_envelope_commitment_root: sample_root("payout-envelope-commitment-root"),
            payout_schedule_root: sample_root("payout-schedule-root"),
            rollback_sentinel_root: sample_root("rollback-sentinel-active"),
            rollback_fence_root: sample_root("rollback-fence-root"),
            circuit_breaker_root: sample_root("circuit-breaker-open"),
            circuit_breaker_reason_root: sample_root("circuit-breaker-reason-root"),
            operator_signoff_root: sample_root("operator-signoff-missing-root"),
            reviewer_signoff_root: sample_root("reviewer-signoff-missing-root"),
            command_hint_root: operator_command_root(&OperatorCommand::sequence()),
            deterministic_settlement_notary_root: deterministic_root("devnet"),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "wave99_pq_reserve_privacy_root": self.wave99_pq_reserve_privacy_root,
            "wave99_unlock_blocker_root": self.wave99_unlock_blocker_root,
            "reserve_execution_bundle_root": self.reserve_execution_bundle_root,
            "reserve_execution_commitment_root": self.reserve_execution_commitment_root,
            "reserve_execution_liability_root": self.reserve_execution_liability_root,
            "privacy_settlement_accounting_root": self.privacy_settlement_accounting_root,
            "privacy_budget_accounting_root": self.privacy_budget_accounting_root,
            "non_linkage_accounting_root": self.non_linkage_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "notary_attestation_set_root": self.notary_attestation_set_root,
            "notary_epoch_root": self.notary_epoch_root,
            "payout_envelope_root": self.payout_envelope_root,
            "payout_envelope_commitment_root": self.payout_envelope_commitment_root,
            "payout_schedule_root": self.payout_schedule_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "rollback_fence_root": self.rollback_fence_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "circuit_breaker_reason_root": self.circuit_breaker_reason_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "command_hint_root": self.command_hint_root,
            "deterministic_settlement_notary_root": self.deterministic_settlement_notary_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("guard_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardMetrics {
    pub notary_quorum_bps: u64,
    pub reserve_coverage_bps: u64,
    pub privacy_budget_remaining_bps: u64,
    pub linkage_risk_bps: u64,
    pub settlement_drift_bps: u64,
    pub wave99_unlock_guard_cleared: bool,
    pub reserve_execution_bundle_notarized: bool,
    pub reserve_execution_cleared: bool,
    pub privacy_settlement_cleared: bool,
    pub notary_quorum_cleared: bool,
    pub payout_envelope_cleared: bool,
    pub rollback_sentinel_cleared: bool,
    pub circuit_breaker_closed: bool,
    pub operator_signed: bool,
    pub reviewer_signed: bool,
    pub root_only: bool,
}

impl Default for GuardMetrics {
    fn default() -> Self {
        Self {
            notary_quorum_bps: 0,
            reserve_coverage_bps: 0,
            privacy_budget_remaining_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            settlement_drift_bps: DEFAULT_MAX_SETTLEMENT_DRIFT_BPS.saturating_add(1),
            wave99_unlock_guard_cleared: false,
            reserve_execution_bundle_notarized: false,
            reserve_execution_cleared: false,
            privacy_settlement_cleared: false,
            notary_quorum_cleared: false,
            payout_envelope_cleared: false,
            rollback_sentinel_cleared: false,
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
pub struct ExecutionRecord {
    pub execution_root: String,
    pub record_kind: ExecutionRecordKind,
    pub status: ExecutionStatus,
    pub roots: GuardRoots,
    pub metrics: GuardMetrics,
    pub blockers: Vec<ExecutionBlocker>,
}

impl ExecutionRecord {
    pub fn evaluate(
        config: &Config,
        record_kind: ExecutionRecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Self {
        let blockers = execution_blockers(config, &roots, &metrics);
        let status = if blockers.is_empty() {
            ExecutionStatus::SettlementShadowReady
        } else if !metrics.notary_quorum_cleared || metrics.notary_quorum_bps == 0 {
            ExecutionStatus::NotaryHeld
        } else if !config.release_execution_allowed || !config.settlement_allowed {
            ExecutionStatus::Denied
        } else {
            ExecutionStatus::Blocked
        };
        let execution_root =
            execution_record_root(record_kind, status, &roots, &metrics, &blockers);
        Self {
            execution_root,
            record_kind,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_root": self.execution_root,
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "roots": self.roots.public_record(),
            "metrics": self.metrics.public_record(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("execution_record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecordState {
    pub record_kind: ExecutionRecordKind,
    pub status: ExecutionStatus,
    pub execution_root: String,
    pub blocker_root: String,
    pub release_execution_denied: bool,
    pub command_hint_root: String,
}

impl RecordState {
    pub fn blocked(record_kind: ExecutionRecordKind) -> Self {
        let blockers = default_active_blockers();
        Self {
            record_kind,
            status: ExecutionStatus::Denied,
            execution_root: empty_root("record", record_kind.as_str()),
            blocker_root: blocker_root(&blockers),
            release_execution_denied: true,
            command_hint_root: command_hint_root(record_kind),
        }
    }

    pub fn from_record(record: &ExecutionRecord) -> Self {
        Self {
            record_kind: record.record_kind,
            status: record.status,
            execution_root: record.execution_root.clone(),
            blocker_root: blocker_root(&record.blockers),
            release_execution_denied: record.status != ExecutionStatus::SettlementShadowReady,
            command_hint_root: command_hint_root(record.record_kind),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_kind": self.record_kind.as_str(),
            "status": self.status.as_str(),
            "execution_root": self.execution_root,
            "blocker_root": self.blocker_root,
            "release_execution_denied": self.release_execution_denied,
            "command_hint_root": self.command_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("record_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardCounters {
    pub execution_record_count: u64,
    pub denied_count: u64,
    pub blocked_count: u64,
    pub notary_held_count: u64,
    pub settlement_shadow_ready_count: u64,
    pub release_execution_denied_count: u64,
    pub blocker_count: u64,
}

impl GuardCounters {
    pub fn from_parts(
        records: &[ExecutionRecord],
        record_states: &BTreeMap<ExecutionRecordKind, RecordState>,
    ) -> Self {
        let mut counters = Self {
            execution_record_count: records.len() as u64,
            blocker_count: records
                .iter()
                .map(|record| record.blockers.len() as u64)
                .sum(),
            ..Self::default()
        };
        for record in records {
            match record.status {
                ExecutionStatus::Denied => {
                    counters.denied_count = counters.denied_count.saturating_add(1);
                }
                ExecutionStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                ExecutionStatus::NotaryHeld => {
                    counters.notary_held_count = counters.notary_held_count.saturating_add(1);
                }
                ExecutionStatus::SettlementShadowReady => {
                    counters.settlement_shadow_ready_count =
                        counters.settlement_shadow_ready_count.saturating_add(1);
                }
            }
        }
        counters.release_execution_denied_count = record_states
            .values()
            .filter(|state| state.release_execution_denied)
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
    pub execution_records: Vec<ExecutionRecord>,
    pub record_states: BTreeMap<ExecutionRecordKind, RecordState>,
    pub counters: GuardCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, execution_records: Vec<ExecutionRecord>) -> Result<Self> {
        if execution_records.len() > config.max_execution_records {
            return Err(ExecutionBlocker::ExecutionCapacityReached
                .as_str()
                .to_string());
        }
        let mut seen = BTreeSet::new();
        for record in &execution_records {
            if !seen.insert(record.execution_root.clone()) {
                return Err(ExecutionBlocker::DuplicateExecutionRoot
                    .as_str()
                    .to_string());
            }
        }
        let mut record_states = BTreeMap::new();
        for record_kind in ExecutionRecordKind::all() {
            let maybe_record = execution_records
                .iter()
                .rev()
                .find(|record| record.record_kind == record_kind);
            let state = match maybe_record {
                Some(record) => RecordState::from_record(record),
                None => RecordState::blocked(record_kind),
            };
            record_states.insert(record_kind, state);
        }
        let counters = GuardCounters::from_parts(&execution_records, &record_states);
        Ok(Self {
            config,
            execution_records,
            record_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn append_execution_record(
        &self,
        record_kind: ExecutionRecordKind,
        roots: GuardRoots,
        metrics: GuardMetrics,
    ) -> Result<Self> {
        let mut execution_records = self.execution_records.clone();
        if execution_records.len() >= self.config.max_execution_records {
            return Err(ExecutionBlocker::ExecutionCapacityReached
                .as_str()
                .to_string());
        }
        let record = ExecutionRecord::evaluate(&self.config, record_kind, roots, metrics);
        if execution_records
            .iter()
            .any(|item| item.execution_root == record.execution_root)
        {
            return Err(ExecutionBlocker::DuplicateExecutionRoot
                .as_str()
                .to_string());
        }
        execution_records.push(record);
        Self::new(self.config.clone(), execution_records)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.execution_records.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.release_execution_denied_count == 0
            && self.counters.settlement_shadow_ready_count
                == ExecutionRecordKind::all().len() as u64
        {
            RuntimeVerdict::ReleaseExecutionShadowReady
        } else {
            RuntimeVerdict::NotaryGuardActive
        }
    }

    pub fn public_record(&self) -> Value {
        let state_records = self
            .record_states
            .values()
            .map(RecordState::public_record)
            .collect::<Vec<_>>();
        let execution_records = self
            .execution_records
            .iter()
            .map(ExecutionRecord::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "record_state_root": list_root("record_states", state_records),
            "execution_record_root": list_root("execution_records", execution_records),
            "wave99_unlock_guard_root": wave99_unlock_guard_root(&self.execution_records),
            "reserve_execution_bundle_root": reserve_execution_bundle_root(&self.execution_records),
            "privacy_settlement_accounting_root": privacy_settlement_accounting_root(&self.execution_records),
            "notary_quorum_root": notary_quorum_root(&self.execution_records),
            "payout_envelope_root": payout_envelope_root(&self.execution_records),
            "rollback_sentinel_root": rollback_sentinel_root(&self.execution_records),
            "circuit_breaker_root": circuit_breaker_root(&self.execution_records),
            "operator_reviewer_signoff_root": operator_reviewer_signoff_root(&self.execution_records),
            "command_hint_root": operator_command_root(&self.operator_commands),
            "deterministic_root": deterministic_state_root(&self.execution_records, &self.record_states),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.execution_records, &self.record_states),
            "active_execution_blocker_root": blocker_root(&default_active_blockers()),
            "verdict": self.verdict().as_str(),
            "release_execution_denied": self.verdict() != RuntimeVerdict::ReleaseExecutionShadowReady,
            "notarized_execution_bundle_count": 0_u64,
            "released_execution_count": 0_u64,
            "settlement_payload_count": 0_u64,
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
                HashPart::Str(&records_root(&self.execution_records)),
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
    let records = ExecutionRecordKind::all()
        .into_iter()
        .map(|record_kind| {
            ExecutionRecord::evaluate(&config, record_kind, roots.clone(), metrics.clone())
        })
        .collect::<Vec<_>>();
    match State::new(config.clone(), records) {
        Ok(state) => state,
        Err(reason) => {
            let fallback_roots = GuardRoots {
                deterministic_settlement_notary_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..GuardRoots::empty()
            };
            let fallback_record = ExecutionRecord::evaluate(
                &config,
                ExecutionRecordKind::CommandHint,
                fallback_roots,
                GuardMetrics::default(),
            );
            let record_states = ExecutionRecordKind::all()
                .into_iter()
                .map(|record_kind| (record_kind, RecordState::blocked(record_kind)))
                .collect::<BTreeMap<_, _>>();
            State {
                config,
                execution_records: Vec::new(),
                counters: GuardCounters::from_parts(&[fallback_record], &record_states),
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

fn execution_blockers(
    config: &Config,
    roots: &GuardRoots,
    metrics: &GuardMetrics,
) -> Vec<ExecutionBlocker> {
    let mut blockers = Vec::new();
    if !config.heavy_gates_ran {
        blockers.push(ExecutionBlocker::HeavyGatesNotRun);
    }
    if !config.production_allowed {
        blockers.push(ExecutionBlocker::ProductionDenied);
    }
    if !config.release_execution_allowed {
        blockers.push(ExecutionBlocker::ReleaseExecutionDenied);
    }
    if !config.settlement_allowed {
        blockers.push(ExecutionBlocker::SettlementDenied);
    }
    if config.wave99_unlock_guard_active || !metrics.wave99_unlock_guard_cleared {
        blockers.push(ExecutionBlocker::Wave99UnlockGuardActive);
    }
    if roots.wave99_unlock_guard_root.is_empty()
        || roots.wave99_pq_reserve_privacy_root.is_empty()
        || roots.wave99_unlock_blocker_root.is_empty()
    {
        blockers.push(ExecutionBlocker::Wave99UnlockGuardRootMissing);
    }
    if roots.reserve_execution_bundle_root.is_empty()
        || roots.reserve_execution_commitment_root.is_empty()
        || roots.reserve_execution_liability_root.is_empty()
    {
        blockers.push(ExecutionBlocker::ReserveExecutionBundleMissing);
    }
    if config.reserve_execution_blocked
        || !metrics.reserve_execution_bundle_notarized
        || !metrics.reserve_execution_cleared
    {
        blockers.push(ExecutionBlocker::ReserveExecutionBundleBlocked);
    }
    if metrics.reserve_coverage_bps < config.min_reserve_coverage_bps {
        blockers.push(ExecutionBlocker::ReserveCoverageLow);
    }
    if roots.privacy_settlement_accounting_root.is_empty()
        || roots.privacy_budget_accounting_root.is_empty()
        || roots.non_linkage_accounting_root.is_empty()
    {
        blockers.push(ExecutionBlocker::PrivacySettlementBlocked);
    }
    if config.privacy_settlement_blocked || !metrics.privacy_settlement_cleared {
        blockers.push(ExecutionBlocker::PrivacySettlementBlocked);
    }
    if metrics.privacy_budget_remaining_bps < config.min_privacy_budget_remaining_bps {
        blockers.push(ExecutionBlocker::PrivacyBudgetLow);
    }
    if metrics.linkage_risk_bps > config.max_linkage_risk_bps {
        blockers.push(ExecutionBlocker::LinkageRiskHigh);
    }
    if roots.notary_quorum_root.is_empty()
        || roots.notary_attestation_set_root.is_empty()
        || roots.notary_epoch_root.is_empty()
    {
        blockers.push(ExecutionBlocker::NotaryQuorumMissing);
    }
    if !metrics.notary_quorum_cleared || metrics.notary_quorum_bps < config.min_notary_quorum_bps {
        blockers.push(ExecutionBlocker::NotaryQuorumLow);
    }
    if roots.payout_envelope_root.is_empty()
        || roots.payout_envelope_commitment_root.is_empty()
        || roots.payout_schedule_root.is_empty()
    {
        blockers.push(ExecutionBlocker::PayoutEnvelopeMissing);
    }
    if !metrics.payout_envelope_cleared {
        blockers.push(ExecutionBlocker::PayoutEnvelopeBlocked);
    }
    if metrics.settlement_drift_bps > config.max_settlement_drift_bps {
        blockers.push(ExecutionBlocker::SettlementDriftHigh);
    }
    if config.rollback_sentinel_active || !metrics.rollback_sentinel_cleared {
        blockers.push(ExecutionBlocker::RollbackSentinelActive);
    }
    if roots.rollback_sentinel_root.is_empty() || roots.rollback_fence_root.is_empty() {
        blockers.push(ExecutionBlocker::RollbackSentinelActive);
    }
    if config.circuit_breaker_open || !metrics.circuit_breaker_closed {
        blockers.push(ExecutionBlocker::CircuitBreakerOpen);
    }
    if roots.circuit_breaker_root.is_empty() || roots.circuit_breaker_reason_root.is_empty() {
        blockers.push(ExecutionBlocker::CircuitBreakerOpen);
    }
    if !metrics.operator_signed || roots.operator_signoff_root.is_empty() {
        blockers.push(ExecutionBlocker::OperatorSignoffMissing);
    }
    if !metrics.reviewer_signed || roots.reviewer_signoff_root.is_empty() {
        blockers.push(ExecutionBlocker::ReviewerSignoffMissing);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(ExecutionBlocker::RootsOnlyBoundary);
    }
    if roots.command_hint_root.is_empty() || roots.deterministic_settlement_notary_root.is_empty() {
        blockers.push(ExecutionBlocker::DeterministicRootMissing);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_blockers() -> Vec<ExecutionBlocker> {
    vec![
        ExecutionBlocker::HeavyGatesNotRun,
        ExecutionBlocker::ProductionDenied,
        ExecutionBlocker::ReleaseExecutionDenied,
        ExecutionBlocker::SettlementDenied,
        ExecutionBlocker::Wave99UnlockGuardActive,
        ExecutionBlocker::ReserveExecutionBundleBlocked,
        ExecutionBlocker::ReserveCoverageLow,
        ExecutionBlocker::PrivacySettlementBlocked,
        ExecutionBlocker::PrivacyBudgetLow,
        ExecutionBlocker::LinkageRiskHigh,
        ExecutionBlocker::NotaryQuorumLow,
        ExecutionBlocker::PayoutEnvelopeBlocked,
        ExecutionBlocker::SettlementDriftHigh,
        ExecutionBlocker::RollbackSentinelActive,
        ExecutionBlocker::CircuitBreakerOpen,
        ExecutionBlocker::OperatorSignoffMissing,
        ExecutionBlocker::ReviewerSignoffMissing,
    ]
}

fn execution_record_root(
    record_kind: ExecutionRecordKind,
    status: ExecutionStatus,
    roots: &GuardRoots,
    metrics: &GuardMetrics,
    blockers: &[ExecutionBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:execution-record"),
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

fn records_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| Value::String(record.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:records"), &leaves)
}

fn record_states_root(record_states: &BTreeMap<ExecutionRecordKind, RecordState>) -> String {
    let leaves = record_states
        .values()
        .map(|state| Value::String(state.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:record-states"), &leaves)
}

fn wave99_unlock_guard_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "wave99_unlock_guard_root": record.roots.wave99_unlock_guard_root,
                "wave99_pq_reserve_privacy_root": record.roots.wave99_pq_reserve_privacy_root,
                "wave99_unlock_blocker_root": record.roots.wave99_unlock_blocker_root,
                "wave99_unlock_guard_cleared": record.metrics.wave99_unlock_guard_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:wave99-unlock-guard-roots"), &leaves)
}

fn reserve_execution_bundle_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "reserve_execution_bundle_root": record.roots.reserve_execution_bundle_root,
                "reserve_execution_commitment_root": record.roots.reserve_execution_commitment_root,
                "reserve_execution_liability_root": record.roots.reserve_execution_liability_root,
                "reserve_coverage_bps": record.metrics.reserve_coverage_bps,
                "reserve_execution_bundle_notarized": record.metrics.reserve_execution_bundle_notarized,
                "reserve_execution_cleared": record.metrics.reserve_execution_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:reserve-execution-bundle-roots"), &leaves)
}

fn privacy_settlement_accounting_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "privacy_settlement_accounting_root": record.roots.privacy_settlement_accounting_root,
                "privacy_budget_accounting_root": record.roots.privacy_budget_accounting_root,
                "non_linkage_accounting_root": record.roots.non_linkage_accounting_root,
                "privacy_budget_remaining_bps": record.metrics.privacy_budget_remaining_bps,
                "linkage_risk_bps": record.metrics.linkage_risk_bps,
                "privacy_settlement_cleared": record.metrics.privacy_settlement_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        &format!("{DOMAIN}:privacy-settlement-accounting-roots"),
        &leaves,
    )
}

fn notary_quorum_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "notary_quorum_root": record.roots.notary_quorum_root,
                "notary_attestation_set_root": record.roots.notary_attestation_set_root,
                "notary_epoch_root": record.roots.notary_epoch_root,
                "notary_quorum_bps": record.metrics.notary_quorum_bps,
                "notary_quorum_cleared": record.metrics.notary_quorum_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:notary-quorum-roots"), &leaves)
}

fn payout_envelope_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "payout_envelope_root": record.roots.payout_envelope_root,
                "payout_envelope_commitment_root": record.roots.payout_envelope_commitment_root,
                "payout_schedule_root": record.roots.payout_schedule_root,
                "settlement_drift_bps": record.metrics.settlement_drift_bps,
                "payout_envelope_cleared": record.metrics.payout_envelope_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:payout-envelope-roots"), &leaves)
}

fn rollback_sentinel_root(records: &[ExecutionRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| {
            json!({
                "record_kind": record.record_kind.as_str(),
                "rollback_sentinel_root": record.roots.rollback_sentinel_root,
                "rollback_fence_root": record.roots.rollback_fence_root,
                "rollback_sentinel_cleared": record.metrics.rollback_sentinel_cleared,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:rollback-sentinel-roots"), &leaves)
}

fn circuit_breaker_root(records: &[ExecutionRecord]) -> String {
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

fn operator_reviewer_signoff_root(records: &[ExecutionRecord]) -> String {
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
    records: &[ExecutionRecord],
    record_states: &BTreeMap<ExecutionRecordKind, RecordState>,
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
    records: &[ExecutionRecord],
    record_states: &BTreeMap<ExecutionRecordKind, RecordState>,
) -> String {
    let mut leaves = records
        .iter()
        .flat_map(|record| {
            record.blockers.iter().map(|blocker| {
                json!({
                    "execution_root": record.execution_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(
        record_states
            .values()
            .filter(|state| state.release_execution_denied)
            .map(|state| {
                json!({
                    "record_kind": state.record_kind.as_str(),
                    "blocker_root": state.blocker_root,
                })
            }),
    );
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn blocker_root(blockers: &[ExecutionBlocker]) -> String {
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

fn command_hint_root(record_kind: ExecutionRecordKind) -> String {
    domain_hash(
        &format!("{DOMAIN}:command-hint"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(record_kind.as_str()),
            HashPart::Str(NOTARY_GUARD_SUITE),
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

fn dedupe_blockers(blockers: &mut Vec<ExecutionBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
