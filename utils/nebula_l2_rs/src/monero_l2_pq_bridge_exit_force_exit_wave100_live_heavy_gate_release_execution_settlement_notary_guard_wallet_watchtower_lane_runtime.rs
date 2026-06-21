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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave100-live-heavy-gate-release-execution-settlement-notary-guard-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave100";
pub const SOURCE_WAVE_LABEL: &str = "wave99";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-finality-certificate-unlock-guard-wallet-watchtower-lane";
pub const SETTLEMENT_NOTARY_GUARD_LANE: &str =
    "force-exit-live-heavy-gate-release-execution-settlement-notary-guard-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str =
    "empty-wave100-wallet-watchtower-release-execution-settlement-notary-guard-root";
pub const DEFAULT_EXECUTION_EPOCH: u64 = 100;
pub const DEFAULT_MIN_WAVE99_UNLOCK_GUARD_ROOTS: u64 = 2;
pub const DEFAULT_MIN_RELEASE_EXECUTION_BUNDLE_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WATCHTOWER_SETTLEMENT_ACCOUNTING_ROOTS: u64 = 2;
pub const DEFAULT_MIN_NOTARY_QUORUM_ROOTS: u64 = 3;
pub const DEFAULT_MIN_PAYOUT_ENVELOPE_ROOTS: u64 = 2;
pub const DEFAULT_MIN_ROLLBACK_SENTINEL_ROOTS: u64 = 2;
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
    pub settlement_notary_guard_lane: String,
    pub empty_root_marker: String,
    pub execution_epoch: u64,
    pub min_wave99_unlock_guard_roots: u64,
    pub min_release_execution_bundle_roots: u64,
    pub min_watchtower_settlement_accounting_roots: u64,
    pub min_notary_quorum_roots: u64,
    pub min_payout_envelope_roots: u64,
    pub min_rollback_sentinel_roots: u64,
    pub min_circuit_breaker_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub min_reviewer_signoff_roots: u64,
    pub require_roots_only_public_record: bool,
    pub require_wave99_unlock_guard_roots: bool,
    pub require_release_execution_bundle_roots: bool,
    pub require_watchtower_settlement_accounting_roots: bool,
    pub require_notary_quorum_roots: bool,
    pub require_payout_envelope_roots: bool,
    pub require_rollback_sentinel_roots: bool,
    pub require_circuit_breaker_roots: bool,
    pub require_operator_signoff_roots: bool,
    pub require_reviewer_signoff_roots: bool,
    pub wallet_settlement_blocker_active: bool,
    pub watchtower_settlement_blocker_active: bool,
    pub notary_guard_blocker_active: bool,
    pub circuit_breaker_active: bool,
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
            settlement_notary_guard_lane: SETTLEMENT_NOTARY_GUARD_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            execution_epoch: DEFAULT_EXECUTION_EPOCH,
            min_wave99_unlock_guard_roots: DEFAULT_MIN_WAVE99_UNLOCK_GUARD_ROOTS,
            min_release_execution_bundle_roots: DEFAULT_MIN_RELEASE_EXECUTION_BUNDLE_ROOTS,
            min_watchtower_settlement_accounting_roots:
                DEFAULT_MIN_WATCHTOWER_SETTLEMENT_ACCOUNTING_ROOTS,
            min_notary_quorum_roots: DEFAULT_MIN_NOTARY_QUORUM_ROOTS,
            min_payout_envelope_roots: DEFAULT_MIN_PAYOUT_ENVELOPE_ROOTS,
            min_rollback_sentinel_roots: DEFAULT_MIN_ROLLBACK_SENTINEL_ROOTS,
            min_circuit_breaker_roots: DEFAULT_MIN_CIRCUIT_BREAKER_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            min_reviewer_signoff_roots: DEFAULT_MIN_REVIEWER_SIGNOFF_ROOTS,
            require_roots_only_public_record: true,
            require_wave99_unlock_guard_roots: true,
            require_release_execution_bundle_roots: true,
            require_watchtower_settlement_accounting_roots: true,
            require_notary_quorum_roots: true,
            require_payout_envelope_roots: true,
            require_rollback_sentinel_roots: true,
            require_circuit_breaker_roots: true,
            require_operator_signoff_roots: true,
            require_reviewer_signoff_roots: true,
            wallet_settlement_blocker_active: true,
            watchtower_settlement_blocker_active: true,
            notary_guard_blocker_active: true,
            circuit_breaker_active: true,
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
            "settlement_notary_guard_lane": self.settlement_notary_guard_lane,
            "empty_root_marker": self.empty_root_marker,
            "execution_epoch": self.execution_epoch,
            "min_wave99_unlock_guard_roots": self.min_wave99_unlock_guard_roots,
            "min_release_execution_bundle_roots": self.min_release_execution_bundle_roots,
            "min_watchtower_settlement_accounting_roots": self.min_watchtower_settlement_accounting_roots,
            "min_notary_quorum_roots": self.min_notary_quorum_roots,
            "min_payout_envelope_roots": self.min_payout_envelope_roots,
            "min_rollback_sentinel_roots": self.min_rollback_sentinel_roots,
            "min_circuit_breaker_roots": self.min_circuit_breaker_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "min_reviewer_signoff_roots": self.min_reviewer_signoff_roots,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave99_unlock_guard_roots": self.require_wave99_unlock_guard_roots,
            "require_release_execution_bundle_roots": self.require_release_execution_bundle_roots,
            "require_watchtower_settlement_accounting_roots": self.require_watchtower_settlement_accounting_roots,
            "require_notary_quorum_roots": self.require_notary_quorum_roots,
            "require_payout_envelope_roots": self.require_payout_envelope_roots,
            "require_rollback_sentinel_roots": self.require_rollback_sentinel_roots,
            "require_circuit_breaker_roots": self.require_circuit_breaker_roots,
            "require_operator_signoff_roots": self.require_operator_signoff_roots,
            "require_reviewer_signoff_roots": self.require_reviewer_signoff_roots,
            "wallet_settlement_blocker_active": self.wallet_settlement_blocker_active,
            "watchtower_settlement_blocker_active": self.watchtower_settlement_blocker_active,
            "notary_guard_blocker_active": self.notary_guard_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
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
pub enum SettlementSlotKind {
    Wave99UnlockGuard,
    WalletReleaseExecutionBundle,
    WatchtowerSettlementAccounting,
    NotaryQuorum,
    PayoutEnvelope,
    RollbackSentinel,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    CommandHint,
}

impl SettlementSlotKind {
    pub fn all() -> [Self; 10] {
        [
            Self::Wave99UnlockGuard,
            Self::WalletReleaseExecutionBundle,
            Self::WatchtowerSettlementAccounting,
            Self::NotaryQuorum,
            Self::PayoutEnvelope,
            Self::RollbackSentinel,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::CommandHint,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuard => "wave99_unlock_guard",
            Self::WalletReleaseExecutionBundle => "wallet_release_execution_bundle",
            Self::WatchtowerSettlementAccounting => "watchtower_settlement_accounting",
            Self::NotaryQuorum => "notary_quorum",
            Self::PayoutEnvelope => "payout_envelope",
            Self::RollbackSentinel => "rollback_sentinel",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::CommandHint => "command_hint",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatus {
    BundleAbsent,
    NotaryBlocked,
    SettlementHeld,
    ExecutionReady,
    Denied,
}

impl SettlementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BundleAbsent => "bundle_absent",
            Self::NotaryBlocked => "notary_blocked",
            Self::SettlementHeld => "settlement_held",
            Self::ExecutionReady => "execution_ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_execute(self) -> bool {
        matches!(self, Self::ExecutionReady)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementBlocker {
    Wave99UnlockGuardRootMissing,
    ReleaseExecutionBundleRootMissing,
    WatchtowerSettlementAccountingRootMissing,
    NotaryQuorumRootMissing,
    PayoutEnvelopeRootMissing,
    RollbackSentinelRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    DuplicateUnlockGuardRoot,
    DuplicateExecutionBundleRoot,
    AccountingLaggingBundleRoot,
    PayoutLaggingAccountingRoot,
    RollbackSentinelLaggingNotaryRoot,
    CircuitBreakerActive,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    WalletSettlementBlockerActive,
    WatchtowerSettlementBlockerActive,
    NotaryGuardBlockerActive,
    ExecutionDisabled,
    HeavyGatesNotRun,
}

impl SettlementBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuardRootMissing => "wave99_unlock_guard_root_missing",
            Self::ReleaseExecutionBundleRootMissing => "release_execution_bundle_root_missing",
            Self::WatchtowerSettlementAccountingRootMissing => {
                "watchtower_settlement_accounting_root_missing"
            }
            Self::NotaryQuorumRootMissing => "notary_quorum_root_missing",
            Self::PayoutEnvelopeRootMissing => "payout_envelope_root_missing",
            Self::RollbackSentinelRootMissing => "rollback_sentinel_root_missing",
            Self::CircuitBreakerRootMissing => "circuit_breaker_root_missing",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::DuplicateUnlockGuardRoot => "duplicate_unlock_guard_root",
            Self::DuplicateExecutionBundleRoot => "duplicate_execution_bundle_root",
            Self::AccountingLaggingBundleRoot => "accounting_lagging_bundle_root",
            Self::PayoutLaggingAccountingRoot => "payout_lagging_accounting_root",
            Self::RollbackSentinelLaggingNotaryRoot => "rollback_sentinel_lagging_notary_root",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::WalletSettlementBlockerActive => "wallet_settlement_blocker_active",
            Self::WatchtowerSettlementBlockerActive => "watchtower_settlement_blocker_active",
            Self::NotaryGuardBlockerActive => "notary_guard_blocker_active",
            Self::ExecutionDisabled => "execution_disabled",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldExecution,
    ImportWave99UnlockGuardRoot,
    ImportReleaseExecutionBundleRoot,
    ImportWatchtowerSettlementAccountingRoot,
    ImportNotaryQuorumRoot,
    ImportPayoutEnvelopeRoot,
    ImportRollbackSentinelRoot,
    ImportCircuitBreakerRoot,
    ImportOperatorSignoffRoot,
    ImportReviewerSignoffRoot,
    ResolveDuplicateUnlockGuardRoot,
    ResolveDuplicateExecutionBundleRoot,
    ReconcileSettlementAccountingRoot,
    ReconcilePayoutEnvelopeRoot,
    MaintainSettlementHold,
    ClearCircuitBreaker,
    ExecuteAfterNotarySettlement,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldExecution => "hold_execution",
            Self::ImportWave99UnlockGuardRoot => "import_wave99_unlock_guard_root",
            Self::ImportReleaseExecutionBundleRoot => "import_release_execution_bundle_root",
            Self::ImportWatchtowerSettlementAccountingRoot => {
                "import_watchtower_settlement_accounting_root"
            }
            Self::ImportNotaryQuorumRoot => "import_notary_quorum_root",
            Self::ImportPayoutEnvelopeRoot => "import_payout_envelope_root",
            Self::ImportRollbackSentinelRoot => "import_rollback_sentinel_root",
            Self::ImportCircuitBreakerRoot => "import_circuit_breaker_root",
            Self::ImportOperatorSignoffRoot => "import_operator_signoff_root",
            Self::ImportReviewerSignoffRoot => "import_reviewer_signoff_root",
            Self::ResolveDuplicateUnlockGuardRoot => "resolve_duplicate_unlock_guard_root",
            Self::ResolveDuplicateExecutionBundleRoot => "resolve_duplicate_execution_bundle_root",
            Self::ReconcileSettlementAccountingRoot => "reconcile_settlement_accounting_root",
            Self::ReconcilePayoutEnvelopeRoot => "reconcile_payout_envelope_root",
            Self::MaintainSettlementHold => "maintain_settlement_hold",
            Self::ClearCircuitBreaker => "clear_circuit_breaker",
            Self::ExecuteAfterNotarySettlement => "execute_after_notary_settlement",
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
        slot_kind: SettlementSlotKind,
        kind: CommandHintKind,
        blockers: &[SettlementBlocker],
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
pub struct SettlementGuardEntry {
    pub slot_kind: SettlementSlotKind,
    pub wave99_unlock_guard_roots: Vec<String>,
    pub release_execution_bundle_roots: Vec<String>,
    pub watchtower_settlement_accounting_roots: Vec<String>,
    pub notary_quorum_roots: Vec<String>,
    pub payout_envelope_roots: Vec<String>,
    pub rollback_sentinel_roots: Vec<String>,
    pub circuit_breaker_roots: Vec<String>,
    pub operator_signoff_roots: Vec<String>,
    pub reviewer_signoff_roots: Vec<String>,
    pub wave99_unlock_guard_root: String,
    pub release_execution_bundle_root: String,
    pub watchtower_settlement_accounting_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub settlement_notary_guard_root: String,
    pub deterministic_execution_root: String,
    pub blockers: Vec<SettlementBlocker>,
    pub status: SettlementStatus,
    pub command_hint: CommandHint,
    pub execution_allowed: bool,
}

impl SettlementGuardEntry {
    pub fn empty(slot_kind: SettlementSlotKind, config: &Config) -> Self {
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
        slot_kind: SettlementSlotKind,
        wave99_unlock_guard_roots: Vec<String>,
        release_execution_bundle_roots: Vec<String>,
        watchtower_settlement_accounting_roots: Vec<String>,
        notary_quorum_roots: Vec<String>,
        payout_envelope_roots: Vec<String>,
        rollback_sentinel_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut entry = Self {
            slot_kind,
            wave99_unlock_guard_roots,
            release_execution_bundle_roots,
            watchtower_settlement_accounting_roots,
            notary_quorum_roots,
            payout_envelope_roots,
            rollback_sentinel_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            wave99_unlock_guard_root: empty_root("wave99-unlock-guard"),
            release_execution_bundle_root: empty_root("release-execution-bundle"),
            watchtower_settlement_accounting_root: empty_root("watchtower-settlement-accounting"),
            notary_quorum_root: empty_root("notary-quorum"),
            payout_envelope_root: empty_root("payout-envelope"),
            rollback_sentinel_root: empty_root("rollback-sentinel"),
            circuit_breaker_root: empty_root("circuit-breaker"),
            operator_signoff_root: empty_root("operator-signoff"),
            reviewer_signoff_root: empty_root("reviewer-signoff"),
            settlement_notary_guard_root: empty_root("settlement-notary-guard"),
            deterministic_execution_root: empty_root("deterministic-execution"),
            blockers: Vec::new(),
            status: SettlementStatus::BundleAbsent,
            command_hint: CommandHint::new(slot_kind, CommandHintKind::HoldExecution, &[]),
            execution_allowed: false,
        };
        entry.recompute(config);
        entry
    }

    pub fn recompute(&mut self, config: &Config) {
        self.wave99_unlock_guard_root =
            aggregate_root("wave99-unlock-guard-roots", &self.wave99_unlock_guard_roots);
        self.release_execution_bundle_root = aggregate_root(
            "release-execution-bundle-roots",
            &self.release_execution_bundle_roots,
        );
        self.watchtower_settlement_accounting_root = aggregate_root(
            "watchtower-settlement-accounting-roots",
            &self.watchtower_settlement_accounting_roots,
        );
        self.notary_quorum_root = aggregate_root("notary-quorum-roots", &self.notary_quorum_roots);
        self.payout_envelope_root =
            aggregate_root("payout-envelope-roots", &self.payout_envelope_roots);
        self.rollback_sentinel_root =
            aggregate_root("rollback-sentinel-roots", &self.rollback_sentinel_roots);
        self.circuit_breaker_root =
            aggregate_root("circuit-breaker-roots", &self.circuit_breaker_roots);
        self.operator_signoff_root =
            aggregate_root("operator-signoff-roots", &self.operator_signoff_roots);
        self.reviewer_signoff_root =
            aggregate_root("reviewer-signoff-roots", &self.reviewer_signoff_roots);
        self.settlement_notary_guard_root = settlement_notary_guard_root(self);
        self.deterministic_execution_root = deterministic_execution_root(self);
        self.blockers = settlement_blockers(self, config);
        self.status = if !config.release_execution_enabled {
            SettlementStatus::Denied
        } else if config.wallet_settlement_blocker_active
            || config.watchtower_settlement_blocker_active
            || config.notary_guard_blocker_active
            || config.circuit_breaker_active
        {
            SettlementStatus::SettlementHeld
        } else if self.release_execution_bundle_roots.is_empty() {
            SettlementStatus::BundleAbsent
        } else if self.blockers.is_empty() {
            SettlementStatus::ExecutionReady
        } else {
            SettlementStatus::NotaryBlocked
        };
        self.execution_allowed = self.status.can_execute()
            && config.release_execution_enabled
            && config.heavy_gates_ran
            && !config.wallet_settlement_blocker_active
            && !config.watchtower_settlement_blocker_active
            && !config.notary_guard_blocker_active
            && !config.circuit_breaker_active;
        self.command_hint =
            CommandHint::new(self.slot_kind, command_for_entry(self), &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        blockers_root("entry-blockers", &self.blockers)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "release_execution_bundle_root": self.release_execution_bundle_root,
            "watchtower_settlement_accounting_root": self.watchtower_settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "settlement_notary_guard_root": self.settlement_notary_guard_root,
            "deterministic_execution_root": self.deterministic_execution_root,
            "blocker_root": self.blocker_root(),
            "status": self.status.as_str(),
            "command_hint": self.command_hint.public_record(),
            "execution_allowed": self.execution_allowed,
            "roots_only": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("settlement-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementSummary {
    pub fail_closed: bool,
    pub release_execution_denied: bool,
    pub execution_allowed_count: u64,
    pub held_count: u64,
    pub blocked_count: u64,
    pub denied_count: u64,
    pub bundle_absent_count: u64,
    pub wave99_unlock_guard_root: String,
    pub release_execution_bundle_root: String,
    pub watchtower_settlement_accounting_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub settlement_notary_guard_root: String,
    pub deterministic_execution_root: String,
    pub blocker_root: String,
    pub command_root: String,
    pub wallet_settlement_blocker_active: bool,
    pub watchtower_settlement_blocker_active: bool,
    pub notary_guard_blocker_active: bool,
    pub circuit_breaker_active: bool,
    pub heavy_gates_ran: bool,
}

impl SettlementSummary {
    pub fn from_entries(config: &Config, entries: &BTreeMap<String, SettlementGuardEntry>) -> Self {
        let execution_allowed_count = entries
            .values()
            .filter(|entry| entry.execution_allowed)
            .count() as u64;
        let held_count = entries
            .values()
            .filter(|entry| entry.status == SettlementStatus::SettlementHeld)
            .count() as u64;
        let blocked_count = entries
            .values()
            .filter(|entry| entry.status == SettlementStatus::NotaryBlocked)
            .count() as u64;
        let denied_count = entries
            .values()
            .filter(|entry| entry.status == SettlementStatus::Denied)
            .count() as u64;
        let bundle_absent_count = entries
            .values()
            .filter(|entry| entry.status == SettlementStatus::BundleAbsent)
            .count() as u64;
        let wave99_unlock_guard_root = entry_field_root(
            "summary-wave99-unlock-guard-roots",
            entries
                .values()
                .map(|entry| entry.wave99_unlock_guard_root.clone()),
        );
        let release_execution_bundle_root = entry_field_root(
            "summary-release-execution-bundle-roots",
            entries
                .values()
                .map(|entry| entry.release_execution_bundle_root.clone()),
        );
        let watchtower_settlement_accounting_root = entry_field_root(
            "summary-watchtower-settlement-accounting-roots",
            entries
                .values()
                .map(|entry| entry.watchtower_settlement_accounting_root.clone()),
        );
        let notary_quorum_root = entry_field_root(
            "summary-notary-quorum-roots",
            entries
                .values()
                .map(|entry| entry.notary_quorum_root.clone()),
        );
        let payout_envelope_root = entry_field_root(
            "summary-payout-envelope-roots",
            entries
                .values()
                .map(|entry| entry.payout_envelope_root.clone()),
        );
        let rollback_sentinel_root = entry_field_root(
            "summary-rollback-sentinel-roots",
            entries
                .values()
                .map(|entry| entry.rollback_sentinel_root.clone()),
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
        let settlement_notary_guard_root = entry_field_root(
            "summary-settlement-notary-guard-roots",
            entries
                .values()
                .map(|entry| entry.settlement_notary_guard_root.clone()),
        );
        let deterministic_execution_root = entry_field_root(
            "summary-deterministic-execution-roots",
            entries
                .values()
                .map(|entry| entry.deterministic_execution_root.clone()),
        );
        let blocker_root = entry_field_root(
            "summary-blockers",
            entries.values().map(SettlementGuardEntry::blocker_root),
        );
        let command_root = merkle_root(
            "WAVE100-WALLET-WATCHTOWER-COMMAND-HINTS",
            &entries
                .values()
                .map(|entry| entry.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let fail_closed = execution_allowed_count == 0
            || held_count > 0
            || blocked_count > 0
            || denied_count > 0
            || bundle_absent_count > 0
            || !config.release_execution_enabled
            || config.wallet_settlement_blocker_active
            || config.watchtower_settlement_blocker_active
            || config.notary_guard_blocker_active
            || config.circuit_breaker_active
            || !config.heavy_gates_ran;
        Self {
            fail_closed,
            release_execution_denied: fail_closed,
            execution_allowed_count,
            held_count,
            blocked_count,
            denied_count,
            bundle_absent_count,
            wave99_unlock_guard_root,
            release_execution_bundle_root,
            watchtower_settlement_accounting_root,
            notary_quorum_root,
            payout_envelope_root,
            rollback_sentinel_root,
            circuit_breaker_root,
            operator_signoff_root,
            reviewer_signoff_root,
            settlement_notary_guard_root,
            deterministic_execution_root,
            blocker_root,
            command_root,
            wallet_settlement_blocker_active: config.wallet_settlement_blocker_active,
            watchtower_settlement_blocker_active: config.watchtower_settlement_blocker_active,
            notary_guard_blocker_active: config.notary_guard_blocker_active,
            circuit_breaker_active: config.circuit_breaker_active,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "release_execution_denied": self.release_execution_denied,
            "execution_allowed_count": self.execution_allowed_count,
            "held_count": self.held_count,
            "blocked_count": self.blocked_count,
            "denied_count": self.denied_count,
            "bundle_absent_count": self.bundle_absent_count,
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "release_execution_bundle_root": self.release_execution_bundle_root,
            "watchtower_settlement_accounting_root": self.watchtower_settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "settlement_notary_guard_root": self.settlement_notary_guard_root,
            "deterministic_execution_root": self.deterministic_execution_root,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "wallet_settlement_blocker_active": self.wallet_settlement_blocker_active,
            "watchtower_settlement_blocker_active": self.watchtower_settlement_blocker_active,
            "notary_guard_blocker_active": self.notary_guard_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("settlement-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub settlement_entries: BTreeMap<String, SettlementGuardEntry>,
    pub summary: SettlementSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let settlement_entries = SettlementSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let entry = SettlementGuardEntry::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = SettlementSummary::from_entries(&config, &settlement_entries);
        Self {
            config,
            settlement_entries,
            summary,
        }
    }

    pub fn stage_settlement_guard_entry(
        mut self,
        slot_kind: SettlementSlotKind,
        wave99_unlock_guard_roots: Vec<String>,
        release_execution_bundle_roots: Vec<String>,
        watchtower_settlement_accounting_roots: Vec<String>,
        notary_quorum_roots: Vec<String>,
        payout_envelope_roots: Vec<String>,
        rollback_sentinel_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
    ) -> Result<Self> {
        let entry = SettlementGuardEntry::from_roots(
            slot_kind,
            wave99_unlock_guard_roots,
            release_execution_bundle_roots,
            watchtower_settlement_accounting_roots,
            notary_quorum_roots,
            payout_envelope_roots,
            rollback_sentinel_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            &self.config,
        );
        self.settlement_entries
            .insert(slot_kind.as_str().to_string(), entry);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for entry in self.settlement_entries.values_mut() {
            entry.recompute(&self.config);
        }
        self.summary = SettlementSummary::from_entries(&self.config, &self.settlement_entries);
    }

    pub fn settlement_entry_roots(&self) -> BTreeMap<String, String> {
        self.settlement_entries
            .iter()
            .map(|(slot_name, entry)| (slot_name.clone(), entry.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn settlement_entries_root(&self) -> String {
        merkle_root(
            "WAVE100-WALLET-WATCHTOWER-SETTLEMENT-GUARD-ENTRY-ROOTS",
            &self
                .settlement_entry_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "settlement_entry_roots": self.settlement_entry_roots(),
            "settlement_entries_root": self.settlement_entries_root(),
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

pub fn wallet_watchtower_release_execution_settlement_notary_guard_runtime() -> Runtime {
    devnet()
}

fn settlement_blockers(entry: &SettlementGuardEntry, config: &Config) -> Vec<SettlementBlocker> {
    let mut blockers = Vec::new();
    if config.require_wave99_unlock_guard_roots
        && entry.wave99_unlock_guard_roots.len() < config.min_wave99_unlock_guard_roots as usize
    {
        blockers.push(SettlementBlocker::Wave99UnlockGuardRootMissing);
    }
    if config.require_release_execution_bundle_roots
        && entry.release_execution_bundle_roots.len()
            < config.min_release_execution_bundle_roots as usize
    {
        blockers.push(SettlementBlocker::ReleaseExecutionBundleRootMissing);
    }
    if config.require_watchtower_settlement_accounting_roots
        && entry.watchtower_settlement_accounting_roots.len()
            < config.min_watchtower_settlement_accounting_roots as usize
    {
        blockers.push(SettlementBlocker::WatchtowerSettlementAccountingRootMissing);
    }
    if config.require_notary_quorum_roots
        && entry.notary_quorum_roots.len() < config.min_notary_quorum_roots as usize
    {
        blockers.push(SettlementBlocker::NotaryQuorumRootMissing);
    }
    if config.require_payout_envelope_roots
        && entry.payout_envelope_roots.len() < config.min_payout_envelope_roots as usize
    {
        blockers.push(SettlementBlocker::PayoutEnvelopeRootMissing);
    }
    if config.require_rollback_sentinel_roots
        && entry.rollback_sentinel_roots.len() < config.min_rollback_sentinel_roots as usize
    {
        blockers.push(SettlementBlocker::RollbackSentinelRootMissing);
    }
    if config.require_circuit_breaker_roots
        && entry.circuit_breaker_roots.len() < config.min_circuit_breaker_roots as usize
    {
        blockers.push(SettlementBlocker::CircuitBreakerRootMissing);
    }
    if config.require_operator_signoff_roots
        && entry.operator_signoff_roots.len() < config.min_operator_signoff_roots as usize
    {
        blockers.push(SettlementBlocker::OperatorSignoffRootMissing);
    }
    if config.require_reviewer_signoff_roots
        && entry.reviewer_signoff_roots.len() < config.min_reviewer_signoff_roots as usize
    {
        blockers.push(SettlementBlocker::ReviewerSignoffRootMissing);
    }
    if has_duplicate(&entry.wave99_unlock_guard_roots) {
        blockers.push(SettlementBlocker::DuplicateUnlockGuardRoot);
    }
    if has_duplicate(&entry.release_execution_bundle_roots) {
        blockers.push(SettlementBlocker::DuplicateExecutionBundleRoot);
    }
    if !entry.release_execution_bundle_roots.is_empty()
        && entry.watchtower_settlement_accounting_roots.len()
            < entry.release_execution_bundle_roots.len()
    {
        blockers.push(SettlementBlocker::AccountingLaggingBundleRoot);
    }
    if !entry.watchtower_settlement_accounting_roots.is_empty()
        && entry.payout_envelope_roots.len() < entry.watchtower_settlement_accounting_roots.len()
    {
        blockers.push(SettlementBlocker::PayoutLaggingAccountingRoot);
    }
    if !entry.notary_quorum_roots.is_empty()
        && entry.rollback_sentinel_roots.len() < entry.notary_quorum_roots.len()
    {
        blockers.push(SettlementBlocker::RollbackSentinelLaggingNotaryRoot);
    }
    if config.circuit_breaker_active {
        blockers.push(SettlementBlocker::CircuitBreakerActive);
    }
    if !roots_shape_valid(entry) {
        blockers.push(SettlementBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(entry) {
        blockers.push(SettlementBlocker::RootsOnlyRecordMissing);
    }
    if config.wallet_settlement_blocker_active {
        blockers.push(SettlementBlocker::WalletSettlementBlockerActive);
    }
    if config.watchtower_settlement_blocker_active {
        blockers.push(SettlementBlocker::WatchtowerSettlementBlockerActive);
    }
    if config.notary_guard_blocker_active {
        blockers.push(SettlementBlocker::NotaryGuardBlockerActive);
    }
    if !config.release_execution_enabled {
        blockers.push(SettlementBlocker::ExecutionDisabled);
    }
    if !config.heavy_gates_ran {
        blockers.push(SettlementBlocker::HeavyGatesNotRun);
    }
    blockers
}

fn command_for_entry(entry: &SettlementGuardEntry) -> CommandHintKind {
    if entry.blockers.is_empty() {
        return CommandHintKind::ExecuteAfterNotarySettlement;
    }
    match entry.blockers[0] {
        SettlementBlocker::Wave99UnlockGuardRootMissing => {
            CommandHintKind::ImportWave99UnlockGuardRoot
        }
        SettlementBlocker::ReleaseExecutionBundleRootMissing => {
            CommandHintKind::ImportReleaseExecutionBundleRoot
        }
        SettlementBlocker::WatchtowerSettlementAccountingRootMissing => {
            CommandHintKind::ImportWatchtowerSettlementAccountingRoot
        }
        SettlementBlocker::NotaryQuorumRootMissing => CommandHintKind::ImportNotaryQuorumRoot,
        SettlementBlocker::PayoutEnvelopeRootMissing => CommandHintKind::ImportPayoutEnvelopeRoot,
        SettlementBlocker::RollbackSentinelRootMissing => {
            CommandHintKind::ImportRollbackSentinelRoot
        }
        SettlementBlocker::CircuitBreakerRootMissing => CommandHintKind::ImportCircuitBreakerRoot,
        SettlementBlocker::OperatorSignoffRootMissing => CommandHintKind::ImportOperatorSignoffRoot,
        SettlementBlocker::ReviewerSignoffRootMissing => CommandHintKind::ImportReviewerSignoffRoot,
        SettlementBlocker::DuplicateUnlockGuardRoot => {
            CommandHintKind::ResolveDuplicateUnlockGuardRoot
        }
        SettlementBlocker::DuplicateExecutionBundleRoot => {
            CommandHintKind::ResolveDuplicateExecutionBundleRoot
        }
        SettlementBlocker::AccountingLaggingBundleRoot
        | SettlementBlocker::RollbackSentinelLaggingNotaryRoot => {
            CommandHintKind::ReconcileSettlementAccountingRoot
        }
        SettlementBlocker::PayoutLaggingAccountingRoot => {
            CommandHintKind::ReconcilePayoutEnvelopeRoot
        }
        SettlementBlocker::CircuitBreakerActive => CommandHintKind::ClearCircuitBreaker,
        SettlementBlocker::WalletSettlementBlockerActive
        | SettlementBlocker::WatchtowerSettlementBlockerActive
        | SettlementBlocker::NotaryGuardBlockerActive => CommandHintKind::MaintainSettlementHold,
        SettlementBlocker::RootShapeInvalid
        | SettlementBlocker::RootsOnlyRecordMissing
        | SettlementBlocker::ExecutionDisabled
        | SettlementBlocker::HeavyGatesNotRun => CommandHintKind::HoldExecution,
    }
}

fn roots_only_record_present(entry: &SettlementGuardEntry) -> bool {
    is_root_like(&entry.wave99_unlock_guard_root)
        && is_root_like(&entry.release_execution_bundle_root)
        && is_root_like(&entry.watchtower_settlement_accounting_root)
        && is_root_like(&entry.notary_quorum_root)
        && is_root_like(&entry.payout_envelope_root)
        && is_root_like(&entry.rollback_sentinel_root)
        && is_root_like(&entry.circuit_breaker_root)
        && is_root_like(&entry.operator_signoff_root)
        && is_root_like(&entry.reviewer_signoff_root)
        && is_root_like(&entry.settlement_notary_guard_root)
        && is_root_like(&entry.deterministic_execution_root)
}

fn roots_shape_valid(entry: &SettlementGuardEntry) -> bool {
    all_roots_like(&entry.wave99_unlock_guard_roots)
        && all_roots_like(&entry.release_execution_bundle_roots)
        && all_roots_like(&entry.watchtower_settlement_accounting_roots)
        && all_roots_like(&entry.notary_quorum_roots)
        && all_roots_like(&entry.payout_envelope_roots)
        && all_roots_like(&entry.rollback_sentinel_roots)
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

fn settlement_notary_guard_root(entry: &SettlementGuardEntry) -> String {
    record_root(
        "settlement-notary-guard-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "wave99_unlock_guard_root": entry.wave99_unlock_guard_root,
            "release_execution_bundle_root": entry.release_execution_bundle_root,
            "watchtower_settlement_accounting_root": entry.watchtower_settlement_accounting_root,
            "notary_quorum_root": entry.notary_quorum_root,
            "payout_envelope_root": entry.payout_envelope_root,
            "rollback_sentinel_root": entry.rollback_sentinel_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "wallet_material_omitted": true,
            "watchtower_material_omitted": true,
            "notary_material_omitted": true,
            "roots_only": true,
        }),
    )
}

fn deterministic_execution_root(entry: &SettlementGuardEntry) -> String {
    record_root(
        "deterministic-execution-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "settlement_notary_guard_root": entry.settlement_notary_guard_root,
            "release_execution_bundle_root": entry.release_execution_bundle_root,
            "watchtower_settlement_accounting_root": entry.watchtower_settlement_accounting_root,
            "notary_quorum_root": entry.notary_quorum_root,
            "payout_envelope_root": entry.payout_envelope_root,
            "rollback_sentinel_root": entry.rollback_sentinel_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "execution_material_omitted": true,
            "settlement_material_omitted": true,
            "payout_material_omitted": true,
            "roots_only": true,
        }),
    )
}

fn aggregate_root(domain: &str, roots: &[String]) -> String {
    if roots.is_empty() {
        return empty_root(domain);
    }
    merkle_root(
        "WAVE100-WALLET-WATCHTOWER-ROOT-AGGREGATE",
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

fn blockers_root(domain: &str, blockers: &[SettlementBlocker]) -> String {
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
        "empty-settlement-notary-guard-root",
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
        "WAVE100-WALLET-WATCHTOWER-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
