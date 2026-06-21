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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave100-release-execution-settlement-notary-guard-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave100-live-heavy-gate-release-execution-settlement-notary-guard-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 100;
pub const DEFAULT_UNLOCK_WAVE: u64 = 99;
pub const DEFAULT_HOLDOFF_WAVE: u64 = 98;
pub const DEFAULT_MIN_NOTARY_QUORUM_ROOTS: u64 = 7;
pub const DEFAULT_MIN_SETTLEMENT_ACCOUNTING_ROOTS: u64 = 7;
pub const DEFAULT_MIN_REPLAY_BUNDLE_ROOTS: u64 = 7;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    RuntimeReplay,
    ReplayExecutionBundle,
    SettlementAccounting,
    NotaryQuorum,
    PayoutEnvelope,
    RollbackSentinel,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RuntimeReplay,
            Self::ReplayExecutionBundle,
            Self::SettlementAccounting,
            Self::NotaryQuorum,
            Self::PayoutEnvelope,
            Self::RollbackSentinel,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime_replay",
            Self::ReplayExecutionBundle => "replay_execution_bundle",
            Self::SettlementAccounting => "settlement_accounting",
            Self::NotaryQuorum => "notary_quorum",
            Self::PayoutEnvelope => "payout_envelope",
            Self::RollbackSentinel => "rollback_sentinel",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatus {
    Blocked,
    AwaitingNotaryQuorum,
    ExecutionDenied,
    Settled,
}

impl SettlementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::AwaitingNotaryQuorum => "awaiting_notary_quorum",
            Self::ExecutionDenied => "execution_denied",
            Self::Settled => "settled",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementBlocker {
    FailClosedArmed,
    ReleaseExecutionDenied,
    HeavyGateReceiptAbsent,
    Wave99UnlockGuardRootActive,
    ReplayExecutionBundleRootMissing,
    SettlementAccountingRootMissing,
    NotaryQuorumRootMissing,
    PayoutEnvelopeRootMissing,
    RollbackSentinelRootActive,
    CircuitBreakerRootActive,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    NotarizedExecutionBundleAbsent,
}

impl SettlementBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ReleaseExecutionDenied,
            Self::HeavyGateReceiptAbsent,
            Self::Wave99UnlockGuardRootActive,
            Self::ReplayExecutionBundleRootMissing,
            Self::SettlementAccountingRootMissing,
            Self::NotaryQuorumRootMissing,
            Self::PayoutEnvelopeRootMissing,
            Self::RollbackSentinelRootActive,
            Self::CircuitBreakerRootActive,
            Self::OperatorSignoffRootMissing,
            Self::ReviewerSignoffRootMissing,
            Self::NotarizedExecutionBundleAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::Wave99UnlockGuardRootActive => "wave99_unlock_guard_root_active",
            Self::ReplayExecutionBundleRootMissing => "replay_execution_bundle_root_missing",
            Self::SettlementAccountingRootMissing => "settlement_accounting_root_missing",
            Self::NotaryQuorumRootMissing => "notary_quorum_root_missing",
            Self::PayoutEnvelopeRootMissing => "payout_envelope_root_missing",
            Self::RollbackSentinelRootActive => "rollback_sentinel_root_active",
            Self::CircuitBreakerRootActive => "circuit_breaker_root_active",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::NotarizedExecutionBundleAbsent => "notarized_execution_bundle_absent",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    CarryWave99UnlockGuardRoots,
    DenyReleaseExecutionUntilNotaryRootsClear,
    BindReplayExecutionBundleRoots,
    BindSettlementAccountingRoots,
    BindNotaryQuorumRoots,
    BindPayoutEnvelopeRoots,
    BindRollbackSentinelRoots,
    BindCircuitBreakerRoots,
    RequireOperatorSignoffRoot,
    RequireReviewerSignoffRoot,
    KeepHeavyGateReceiptAbsent,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CarryWave99UnlockGuardRoots,
            Self::DenyReleaseExecutionUntilNotaryRootsClear,
            Self::BindReplayExecutionBundleRoots,
            Self::BindSettlementAccountingRoots,
            Self::BindNotaryQuorumRoots,
            Self::BindPayoutEnvelopeRoots,
            Self::BindRollbackSentinelRoots,
            Self::BindCircuitBreakerRoots,
            Self::RequireOperatorSignoffRoot,
            Self::RequireReviewerSignoffRoot,
            Self::KeepHeavyGateReceiptAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CarryWave99UnlockGuardRoots => "carry_wave99_unlock_guard_roots",
            Self::DenyReleaseExecutionUntilNotaryRootsClear => {
                "deny_release_execution_until_notary_roots_clear"
            }
            Self::BindReplayExecutionBundleRoots => "bind_replay_execution_bundle_roots",
            Self::BindSettlementAccountingRoots => "bind_settlement_accounting_roots",
            Self::BindNotaryQuorumRoots => "bind_notary_quorum_roots",
            Self::BindPayoutEnvelopeRoots => "bind_payout_envelope_roots",
            Self::BindRollbackSentinelRoots => "bind_rollback_sentinel_roots",
            Self::BindCircuitBreakerRoots => "bind_circuit_breaker_roots",
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
    pub unlock_wave: u64,
    pub holdoff_wave: u64,
    pub min_notary_quorum_roots: u64,
    pub min_settlement_accounting_roots: u64,
    pub min_replay_bundle_roots: u64,
    pub wave99_runtime_replay_unlock_guard_root: String,
    pub wave99_unlock_guard_ledger_root: String,
    pub wave99_command_hints_root: String,
    pub fail_closed_armed: bool,
    pub release_execution_denied: bool,
    pub settlement_blockers_active: bool,
    pub notarized_execution_bundles_present: bool,
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
            unlock_wave: DEFAULT_UNLOCK_WAVE,
            holdoff_wave: DEFAULT_HOLDOFF_WAVE,
            min_notary_quorum_roots: DEFAULT_MIN_NOTARY_QUORUM_ROOTS,
            min_settlement_accounting_roots: DEFAULT_MIN_SETTLEMENT_ACCOUNTING_ROOTS,
            min_replay_bundle_roots: DEFAULT_MIN_REPLAY_BUNDLE_ROOTS,
            wave99_runtime_replay_unlock_guard_root: stable_root(
                "wave99-runtime-replay-unlock-guard",
                "runtime-replay",
            ),
            wave99_unlock_guard_ledger_root: stable_root("wave99-unlock-guard-ledger", "all"),
            wave99_command_hints_root: stable_root("wave99-command-hints", "runtime-replay"),
            fail_closed_armed: true,
            release_execution_denied: true,
            settlement_blockers_active: true,
            notarized_execution_bundles_present: false,
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
        ensure_positive("unlock_wave", self.unlock_wave)?;
        ensure_positive("holdoff_wave", self.holdoff_wave)?;
        ensure_positive("min_notary_quorum_roots", self.min_notary_quorum_roots)?;
        ensure_positive(
            "min_settlement_accounting_roots",
            self.min_settlement_accounting_roots,
        )?;
        ensure_positive("min_replay_bundle_roots", self.min_replay_bundle_roots)?;
        ensure_root(
            "wave99_runtime_replay_unlock_guard_root",
            &self.wave99_runtime_replay_unlock_guard_root,
        )?;
        ensure_root(
            "wave99_unlock_guard_ledger_root",
            &self.wave99_unlock_guard_ledger_root,
        )?;
        ensure_root("wave99_command_hints_root", &self.wave99_command_hints_root)?;
        if !(self.holdoff_wave < self.unlock_wave && self.unlock_wave < self.wave) {
            return Err("wave ordering must be holdoff, unlock, settlement".to_string());
        }
        if !self.fail_closed_armed {
            return Err("settlement notary guard is disarmed".to_string());
        }
        if !self.release_execution_denied {
            return Err("devnet settlement notary guard must deny release execution".to_string());
        }
        if !self.settlement_blockers_active {
            return Err("devnet settlement blockers must remain active".to_string());
        }
        if self.notarized_execution_bundles_present {
            return Err("devnet must not publish notarized execution bundles".to_string());
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
            "kind": "wave100_settlement_notary_guard_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "unlock_wave": self.unlock_wave,
            "holdoff_wave": self.holdoff_wave,
            "min_notary_quorum_roots": self.min_notary_quorum_roots,
            "min_settlement_accounting_roots": self.min_settlement_accounting_roots,
            "min_replay_bundle_roots": self.min_replay_bundle_roots,
            "wave99_runtime_replay_unlock_guard_root": self.wave99_runtime_replay_unlock_guard_root,
            "wave99_unlock_guard_ledger_root": self.wave99_unlock_guard_ledger_root,
            "wave99_command_hints_root": self.wave99_command_hints_root,
            "fail_closed_armed": self.fail_closed_armed,
            "release_execution_denied": self.release_execution_denied,
            "settlement_blockers_active": self.settlement_blockers_active,
            "notarized_execution_bundles_present": self.notarized_execution_bundles_present,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE100-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementGuardEntry {
    pub lane: ReplayLane,
    pub wave99_unlock_guard_root: String,
    pub replay_execution_bundle_root: String,
    pub settlement_accounting_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub blocker_roots: Vec<String>,
    pub status: SettlementStatus,
    pub release_execution_allowed: bool,
}

impl SettlementGuardEntry {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            wave99_unlock_guard_root: lane_wave99_unlock_guard_root(lane, config),
            replay_execution_bundle_root: lane_root("replay-execution-bundle", lane),
            settlement_accounting_root: lane_root("settlement-accounting", lane),
            notary_quorum_root: lane_root("notary-quorum", lane),
            payout_envelope_root: lane_root("payout-envelope", lane),
            rollback_sentinel_root: lane_root("rollback-sentinel", lane),
            circuit_breaker_root: lane_root("circuit-breaker", lane),
            operator_signoff_root: lane_root("operator-signoff", lane),
            reviewer_signoff_root: lane_root("reviewer-signoff", lane),
            command_hint_root: lane_root("command-hint", lane),
            blocker_roots: SettlementBlocker::all()
                .iter()
                .map(|blocker| blocker_root(lane, *blocker))
                .collect(),
            status: SettlementStatus::ExecutionDenied,
            release_execution_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("wave99_unlock_guard_root", &self.wave99_unlock_guard_root)?;
        ensure_root(
            "replay_execution_bundle_root",
            &self.replay_execution_bundle_root,
        )?;
        ensure_root(
            "settlement_accounting_root",
            &self.settlement_accounting_root,
        )?;
        ensure_root("notary_quorum_root", &self.notary_quorum_root)?;
        ensure_root("payout_envelope_root", &self.payout_envelope_root)?;
        ensure_root("rollback_sentinel_root", &self.rollback_sentinel_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.blocker_roots.is_empty() {
            return Err("settlement guard entry requires blocker roots".to_string());
        }
        for root in self.blocker_roots.iter() {
            ensure_root("blocker_root", root)?;
        }
        if self.release_execution_allowed {
            return Err("devnet settlement guard cannot allow release execution".to_string());
        }
        if self.status == SettlementStatus::Settled {
            return Err("devnet settlement guard cannot be settled".to_string());
        }
        Ok(())
    }

    pub fn blocker_root(&self) -> String {
        list_root(
            "WAVE100-SETTLEMENT-BLOCKER-ROOTS",
            self.blocker_roots.clone(),
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave100_settlement_guard_entry",
            "lane": self.lane.as_str(),
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "replay_execution_bundle_root": self.replay_execution_bundle_root,
            "settlement_accounting_root": self.settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "command_hint_root": self.command_hint_root,
            "blocker_roots_root": self.blocker_root(),
            "blocker_count": self.blocker_roots.len(),
            "status": self.status.as_str(),
            "release_execution_allowed": self.release_execution_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE100-SETTLEMENT-GUARD-ENTRY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootLedger {
    pub wave99_unlock_guard_roots: BTreeMap<String, String>,
    pub replay_execution_bundle_roots: BTreeMap<String, String>,
    pub settlement_accounting_roots: BTreeMap<String, String>,
    pub notary_quorum_roots: BTreeMap<String, String>,
    pub payout_envelope_roots: BTreeMap<String, String>,
    pub rollback_sentinel_roots: BTreeMap<String, String>,
    pub circuit_breaker_roots: BTreeMap<String, String>,
    pub operator_signoff_roots: BTreeMap<String, String>,
    pub reviewer_signoff_roots: BTreeMap<String, String>,
    pub command_hints: BTreeMap<String, String>,
}

impl RootLedger {
    pub fn devnet(lanes: &[ReplayLane], config: &Config) -> Self {
        Self {
            wave99_unlock_guard_roots: lanes
                .iter()
                .map(|lane| {
                    (
                        lane.as_str().to_string(),
                        lane_wave99_unlock_guard_root(*lane, config),
                    )
                })
                .collect(),
            replay_execution_bundle_roots: lane_map(lanes, "replay-execution-bundle"),
            settlement_accounting_roots: lane_map(lanes, "settlement-accounting"),
            notary_quorum_roots: lane_map(lanes, "notary-quorum"),
            payout_envelope_roots: lane_map(lanes, "payout-envelope"),
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
        ensure_map_roots("wave99_unlock_guard_roots", &self.wave99_unlock_guard_roots)?;
        ensure_map_roots(
            "replay_execution_bundle_roots",
            &self.replay_execution_bundle_roots,
        )?;
        ensure_map_roots(
            "settlement_accounting_roots",
            &self.settlement_accounting_roots,
        )?;
        ensure_map_roots("notary_quorum_roots", &self.notary_quorum_roots)?;
        ensure_map_roots("payout_envelope_roots", &self.payout_envelope_roots)?;
        ensure_map_roots("rollback_sentinel_roots", &self.rollback_sentinel_roots)?;
        ensure_map_roots("circuit_breaker_roots", &self.circuit_breaker_roots)?;
        ensure_map_roots("operator_signoff_roots", &self.operator_signoff_roots)?;
        ensure_map_roots("reviewer_signoff_roots", &self.reviewer_signoff_roots)?;
        ensure_map_roots("command_hints", &self.command_hints)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave100_root_ledger",
            "wave99_unlock_guard_root": map_root("WAVE100-WAVE99-UNLOCK-GUARD-MAP", &self.wave99_unlock_guard_roots),
            "replay_execution_bundle_root": map_root("WAVE100-REPLAY-EXECUTION-BUNDLE-MAP", &self.replay_execution_bundle_roots),
            "settlement_accounting_root": map_root("WAVE100-SETTLEMENT-ACCOUNTING-MAP", &self.settlement_accounting_roots),
            "notary_quorum_root": map_root("WAVE100-NOTARY-QUORUM-MAP", &self.notary_quorum_roots),
            "payout_envelope_root": map_root("WAVE100-PAYOUT-ENVELOPE-MAP", &self.payout_envelope_roots),
            "rollback_sentinel_root": map_root("WAVE100-ROLLBACK-SENTINEL-MAP", &self.rollback_sentinel_roots),
            "circuit_breaker_root": map_root("WAVE100-CIRCUIT-BREAKER-MAP", &self.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE100-OPERATOR-SIGNOFF-MAP", &self.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE100-REVIEWER-SIGNOFF-MAP", &self.reviewer_signoff_roots),
            "command_hint_root": map_root("WAVE100-COMMAND-HINT-MAP", &self.command_hints),
            "wave99_unlock_guard_count": self.wave99_unlock_guard_roots.len(),
            "replay_execution_bundle_count": self.replay_execution_bundle_roots.len(),
            "settlement_accounting_count": self.settlement_accounting_roots.len(),
            "notary_quorum_count": self.notary_quorum_roots.len(),
            "payout_envelope_count": self.payout_envelope_roots.len(),
            "rollback_sentinel_count": self.rollback_sentinel_roots.len(),
            "circuit_breaker_count": self.circuit_breaker_roots.len(),
            "operator_signoff_count": self.operator_signoff_roots.len(),
            "reviewer_signoff_count": self.reviewer_signoff_roots.len(),
            "command_hint_count": self.command_hints.len(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE100-ROOT-LEDGER", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_count: u64,
    pub settlement_guard_count: u64,
    pub blocked_settlement_guards: u64,
    pub release_execution_allowed_count: u64,
    pub notarized_execution_bundle_count: u64,
    pub wave99_unlock_guard_roots: u64,
    pub replay_execution_bundle_roots: u64,
    pub settlement_accounting_roots: u64,
    pub notary_quorum_roots: u64,
    pub payout_envelope_roots: u64,
    pub rollback_sentinel_roots: u64,
    pub circuit_breaker_roots: u64,
    pub operator_signoff_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub raw_public_records: u64,
}

impl Counters {
    pub fn from_parts(entries: &[SettlementGuardEntry], ledger: &RootLedger) -> Self {
        Self {
            lane_count: ReplayLane::all().len() as u64,
            settlement_guard_count: entries.len() as u64,
            blocked_settlement_guards: entries
                .iter()
                .filter(|entry| !entry.release_execution_allowed)
                .count() as u64,
            release_execution_allowed_count: entries
                .iter()
                .filter(|entry| entry.release_execution_allowed)
                .count() as u64,
            notarized_execution_bundle_count: 0,
            wave99_unlock_guard_roots: ledger.wave99_unlock_guard_roots.len() as u64,
            replay_execution_bundle_roots: ledger.replay_execution_bundle_roots.len() as u64,
            settlement_accounting_roots: ledger.settlement_accounting_roots.len() as u64,
            notary_quorum_roots: ledger.notary_quorum_roots.len() as u64,
            payout_envelope_roots: ledger.payout_envelope_roots.len() as u64,
            rollback_sentinel_roots: ledger.rollback_sentinel_roots.len() as u64,
            circuit_breaker_roots: ledger.circuit_breaker_roots.len() as u64,
            operator_signoff_roots: ledger.operator_signoff_roots.len() as u64,
            reviewer_signoff_roots: ledger.reviewer_signoff_roots.len() as u64,
            raw_public_records: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave100_settlement_notary_guard_counters",
            "lane_count": self.lane_count,
            "settlement_guard_count": self.settlement_guard_count,
            "blocked_settlement_guards": self.blocked_settlement_guards,
            "release_execution_allowed_count": self.release_execution_allowed_count,
            "notarized_execution_bundle_count": self.notarized_execution_bundle_count,
            "wave99_unlock_guard_roots": self.wave99_unlock_guard_roots,
            "replay_execution_bundle_roots": self.replay_execution_bundle_roots,
            "settlement_accounting_roots": self.settlement_accounting_roots,
            "notary_quorum_roots": self.notary_quorum_roots,
            "payout_envelope_roots": self.payout_envelope_roots,
            "rollback_sentinel_roots": self.rollback_sentinel_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "raw_public_records": self.raw_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE100-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub settlement_guards: Vec<SettlementGuardEntry>,
    pub root_ledger: RootLedger,
    pub blocker_catalog: BTreeMap<String, String>,
    pub wave99_unlock_guard_catalog: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        settlement_guards: Vec<SettlementGuardEntry>,
        root_ledger: RootLedger,
        blocker_catalog: BTreeMap<String, String>,
        wave99_unlock_guard_catalog: BTreeMap<String, String>,
    ) -> Result<Self> {
        let counters = Counters::from_parts(&settlement_guards, &root_ledger);
        let state = Self {
            config,
            settlement_guards,
            root_ledger,
            blocker_catalog,
            wave99_unlock_guard_catalog,
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
            "settlement_guards",
            self.settlement_guards
                .iter()
                .map(|entry| entry.lane)
                .collect(),
        )?;
        for entry in self.settlement_guards.iter() {
            entry.validate()?;
        }
        self.root_ledger.validate()?;
        ensure_map_roots("blocker_catalog", &self.blocker_catalog)?;
        ensure_map_roots(
            "wave99_unlock_guard_catalog",
            &self.wave99_unlock_guard_catalog,
        )?;
        if self.counters.release_execution_allowed_count != 0 {
            return Err("devnet must have zero allowed release executions".to_string());
        }
        if self.counters.notarized_execution_bundle_count != 0 {
            return Err("devnet must have zero notarized execution bundles".to_string());
        }
        if self.counters.blocked_settlement_guards != self.counters.settlement_guard_count {
            return Err("all devnet settlement guards must remain blocked".to_string());
        }
        if self.counters.raw_public_records != 0 {
            return Err("public record counter must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave100_release_execution_settlement_notary_guard_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "wave99_runtime_replay_unlock_guard_root": self.config.wave99_runtime_replay_unlock_guard_root,
            "wave99_unlock_guard_ledger_root": self.config.wave99_unlock_guard_ledger_root,
            "wave99_command_hints_root": self.config.wave99_command_hints_root,
            "settlement_guards_root": settlement_guards_root(&self.settlement_guards),
            "root_ledger_root": self.root_ledger.state_root(),
            "blocker_catalog_root": map_root("WAVE100-BLOCKER-CATALOG", &self.blocker_catalog),
            "wave99_unlock_guard_catalog_root": map_root("WAVE100-WAVE99-UNLOCK-GUARD-CATALOG", &self.wave99_unlock_guard_catalog),
            "wave99_unlock_guard_root": map_root("WAVE100-WAVE99-UNLOCK-GUARD-ROOTS", &self.root_ledger.wave99_unlock_guard_roots),
            "replay_execution_bundle_root": map_root("WAVE100-REPLAY-EXECUTION-BUNDLE-ROOTS", &self.root_ledger.replay_execution_bundle_roots),
            "settlement_accounting_root": map_root("WAVE100-SETTLEMENT-ACCOUNTING-ROOTS", &self.root_ledger.settlement_accounting_roots),
            "notary_quorum_root": map_root("WAVE100-NOTARY-QUORUM-ROOTS", &self.root_ledger.notary_quorum_roots),
            "payout_envelope_root": map_root("WAVE100-PAYOUT-ENVELOPE-ROOTS", &self.root_ledger.payout_envelope_roots),
            "rollback_sentinel_root": map_root("WAVE100-ROLLBACK-SENTINEL-ROOTS", &self.root_ledger.rollback_sentinel_roots),
            "circuit_breaker_root": map_root("WAVE100-CIRCUIT-BREAKER-ROOTS", &self.root_ledger.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE100-OPERATOR-SIGNOFF-ROOTS", &self.root_ledger.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE100-REVIEWER-SIGNOFF-ROOTS", &self.root_ledger.reviewer_signoff_roots),
            "command_hints_root": map_root("WAVE100-COMMAND-HINTS", &self.root_ledger.command_hints),
            "counters_root": self.counters.state_root(),
            "release_execution_denied": self.config.release_execution_denied,
            "settlement_blockers_active": self.config.settlement_blockers_active,
            "notarized_execution_bundles_present": self.config.notarized_execution_bundles_present,
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
        value_root("WAVE100-STATE", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let lanes = ReplayLane::all();
    let settlement_guards = lanes
        .iter()
        .map(|lane| SettlementGuardEntry::blocked(*lane, &config))
        .collect::<Vec<_>>();
    let root_ledger = RootLedger::devnet(&lanes, &config);
    let blocker_catalog = SettlementBlocker::all()
        .iter()
        .map(|blocker| (blocker.as_str().to_string(), blocker_kind_root(*blocker)))
        .collect::<BTreeMap<_, _>>();
    let wave99_unlock_guard_catalog = lanes
        .iter()
        .map(|lane| {
            (
                lane.as_str().to_string(),
                lane_wave99_unlock_guard_root(*lane, &config),
            )
        })
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        settlement_guards,
        root_ledger,
        blocker_catalog,
        wave99_unlock_guard_catalog,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            settlement_guards: Vec::new(),
            root_ledger: RootLedger::devnet(&[], &Config::devnet()),
            blocker_catalog: BTreeMap::new(),
            wave99_unlock_guard_catalog: BTreeMap::new(),
            counters: Counters::default(),
        },
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn settlement_guards_root(entries: &[SettlementGuardEntry]) -> String {
    list_root(
        "WAVE100-SETTLEMENT-GUARD-STATE-ROOTS",
        entries
            .iter()
            .map(SettlementGuardEntry::state_root)
            .collect(),
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

fn lane_wave99_unlock_guard_root(lane: ReplayLane, config: &Config) -> String {
    domain_hash(
        "WAVE100-LANE-WAVE99-UNLOCK-GUARD-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&config.wave99_runtime_replay_unlock_guard_root),
            HashPart::Str(&config.wave99_unlock_guard_ledger_root),
        ],
        32,
    )
}

fn lane_root(kind: &str, lane: ReplayLane) -> String {
    domain_hash(
        "WAVE100-LANE-ROOT",
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
        "WAVE100-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: SettlementBlocker) -> String {
    domain_hash(
        "WAVE100-SETTLEMENT-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn blocker_kind_root(blocker: SettlementBlocker) -> String {
    domain_hash(
        "WAVE100-SETTLEMENT-BLOCKER-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn stable_root(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE100-STABLE-ROOT",
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
