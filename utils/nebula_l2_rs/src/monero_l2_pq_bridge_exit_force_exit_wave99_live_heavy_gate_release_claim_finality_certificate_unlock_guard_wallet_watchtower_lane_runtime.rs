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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave99";
pub const SOURCE_WAVE_LABEL: &str = "wave98";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-wallet-watchtower-lane";
pub const UNLOCK_GUARD_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-finality-certificate-unlock-guard-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str =
    "empty-wave99-wallet-watchtower-finality-certificate-unlock-guard-root";
pub const DEFAULT_UNLOCK_EPOCH: u64 = 99;
pub const DEFAULT_MIN_WAVE98_HOLDOFF_ROOTS: u64 = 1;
pub const DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WALLET_ACCOUNTING_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WATCHTOWER_UNLOCK_GUARD_ROOTS: u64 = 3;
pub const DEFAULT_MIN_ROLLBACK_GUARD_ROOTS: u64 = 2;
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
    pub unlock_guard_lane: String,
    pub empty_root_marker: String,
    pub unlock_epoch: u64,
    pub min_wave98_holdoff_roots: u64,
    pub min_finality_certificate_roots: u64,
    pub min_wallet_accounting_roots: u64,
    pub min_watchtower_unlock_guard_roots: u64,
    pub min_rollback_guard_roots: u64,
    pub min_circuit_breaker_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub min_reviewer_signoff_roots: u64,
    pub require_roots_only_public_record: bool,
    pub require_wave98_holdoff_roots: bool,
    pub require_finality_certificate_roots: bool,
    pub require_wallet_accounting_roots: bool,
    pub require_watchtower_unlock_guard_roots: bool,
    pub require_rollback_guard_roots: bool,
    pub require_circuit_breaker_roots: bool,
    pub require_operator_signoff_roots: bool,
    pub require_reviewer_signoff_roots: bool,
    pub wallet_finality_blocker_active: bool,
    pub watchtower_finality_blocker_active: bool,
    pub circuit_breaker_active: bool,
    pub finality_certificate_unlock_enabled: bool,
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
            unlock_guard_lane: UNLOCK_GUARD_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            unlock_epoch: DEFAULT_UNLOCK_EPOCH,
            min_wave98_holdoff_roots: DEFAULT_MIN_WAVE98_HOLDOFF_ROOTS,
            min_finality_certificate_roots: DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS,
            min_wallet_accounting_roots: DEFAULT_MIN_WALLET_ACCOUNTING_ROOTS,
            min_watchtower_unlock_guard_roots: DEFAULT_MIN_WATCHTOWER_UNLOCK_GUARD_ROOTS,
            min_rollback_guard_roots: DEFAULT_MIN_ROLLBACK_GUARD_ROOTS,
            min_circuit_breaker_roots: DEFAULT_MIN_CIRCUIT_BREAKER_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            min_reviewer_signoff_roots: DEFAULT_MIN_REVIEWER_SIGNOFF_ROOTS,
            require_roots_only_public_record: true,
            require_wave98_holdoff_roots: true,
            require_finality_certificate_roots: true,
            require_wallet_accounting_roots: true,
            require_watchtower_unlock_guard_roots: true,
            require_rollback_guard_roots: true,
            require_circuit_breaker_roots: true,
            require_operator_signoff_roots: true,
            require_reviewer_signoff_roots: true,
            wallet_finality_blocker_active: true,
            watchtower_finality_blocker_active: true,
            circuit_breaker_active: true,
            finality_certificate_unlock_enabled: false,
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
            "unlock_guard_lane": self.unlock_guard_lane,
            "empty_root_marker": self.empty_root_marker,
            "unlock_epoch": self.unlock_epoch,
            "min_wave98_holdoff_roots": self.min_wave98_holdoff_roots,
            "min_finality_certificate_roots": self.min_finality_certificate_roots,
            "min_wallet_accounting_roots": self.min_wallet_accounting_roots,
            "min_watchtower_unlock_guard_roots": self.min_watchtower_unlock_guard_roots,
            "min_rollback_guard_roots": self.min_rollback_guard_roots,
            "min_circuit_breaker_roots": self.min_circuit_breaker_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "min_reviewer_signoff_roots": self.min_reviewer_signoff_roots,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave98_holdoff_roots": self.require_wave98_holdoff_roots,
            "require_finality_certificate_roots": self.require_finality_certificate_roots,
            "require_wallet_accounting_roots": self.require_wallet_accounting_roots,
            "require_watchtower_unlock_guard_roots": self.require_watchtower_unlock_guard_roots,
            "require_rollback_guard_roots": self.require_rollback_guard_roots,
            "require_circuit_breaker_roots": self.require_circuit_breaker_roots,
            "require_operator_signoff_roots": self.require_operator_signoff_roots,
            "require_reviewer_signoff_roots": self.require_reviewer_signoff_roots,
            "wallet_finality_blocker_active": self.wallet_finality_blocker_active,
            "watchtower_finality_blocker_active": self.watchtower_finality_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
            "finality_certificate_unlock_enabled": self.finality_certificate_unlock_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockSlotKind {
    Wave98Holdoff,
    FinalityCertificate,
    WalletAccounting,
    WatchtowerUnlockGuard,
    RollbackGuard,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    OperatorCommand,
}

impl UnlockSlotKind {
    pub fn all() -> [Self; 9] {
        [
            Self::Wave98Holdoff,
            Self::FinalityCertificate,
            Self::WalletAccounting,
            Self::WatchtowerUnlockGuard,
            Self::RollbackGuard,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::OperatorCommand,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98Holdoff => "wave98_holdoff",
            Self::FinalityCertificate => "finality_certificate",
            Self::WalletAccounting => "wallet_accounting",
            Self::WatchtowerUnlockGuard => "watchtower_unlock_guard",
            Self::RollbackGuard => "rollback_guard",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorCommand => "operator_command",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockStatus {
    CertificateAbsent,
    GuardBlocked,
    Held,
    UnlockReady,
    Denied,
}

impl UnlockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CertificateAbsent => "certificate_absent",
            Self::GuardBlocked => "guard_blocked",
            Self::Held => "held",
            Self::UnlockReady => "unlock_ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_unlock(self) -> bool {
        matches!(self, Self::UnlockReady)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockBlocker {
    Wave98HoldoffRootMissing,
    FinalityCertificateRootMissing,
    WalletAccountingRootMissing,
    WatchtowerUnlockGuardRootMissing,
    RollbackGuardRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    DuplicateFinalityCertificateRoot,
    DuplicateWalletAccountingRoot,
    RollbackGuardLagging,
    CircuitBreakerActive,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    WalletFinalityBlockerActive,
    WatchtowerFinalityBlockerActive,
    UnlockDisabled,
    HeavyGatesNotRun,
}

impl UnlockBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98HoldoffRootMissing => "wave98_holdoff_root_missing",
            Self::FinalityCertificateRootMissing => "finality_certificate_root_missing",
            Self::WalletAccountingRootMissing => "wallet_accounting_root_missing",
            Self::WatchtowerUnlockGuardRootMissing => "watchtower_unlock_guard_root_missing",
            Self::RollbackGuardRootMissing => "rollback_guard_root_missing",
            Self::CircuitBreakerRootMissing => "circuit_breaker_root_missing",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::DuplicateFinalityCertificateRoot => "duplicate_finality_certificate_root",
            Self::DuplicateWalletAccountingRoot => "duplicate_wallet_accounting_root",
            Self::RollbackGuardLagging => "rollback_guard_lagging",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::WalletFinalityBlockerActive => "wallet_finality_blocker_active",
            Self::WatchtowerFinalityBlockerActive => "watchtower_finality_blocker_active",
            Self::UnlockDisabled => "unlock_disabled",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldUnlock,
    ImportWave98HoldoffRoot,
    ImportFinalityCertificateRoot,
    ImportWalletAccountingRoot,
    ImportWatchtowerUnlockGuardRoot,
    ImportRollbackGuardRoot,
    ImportCircuitBreakerRoot,
    ImportOperatorSignoffRoot,
    ImportReviewerSignoffRoot,
    ResolveDuplicateCertificateRoot,
    ResolveAccountingRoot,
    ClearCircuitBreaker,
    MaintainFinalityHold,
    UnlockAfterCertificateFinality,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldUnlock => "hold_unlock",
            Self::ImportWave98HoldoffRoot => "import_wave98_holdoff_root",
            Self::ImportFinalityCertificateRoot => "import_finality_certificate_root",
            Self::ImportWalletAccountingRoot => "import_wallet_accounting_root",
            Self::ImportWatchtowerUnlockGuardRoot => "import_watchtower_unlock_guard_root",
            Self::ImportRollbackGuardRoot => "import_rollback_guard_root",
            Self::ImportCircuitBreakerRoot => "import_circuit_breaker_root",
            Self::ImportOperatorSignoffRoot => "import_operator_signoff_root",
            Self::ImportReviewerSignoffRoot => "import_reviewer_signoff_root",
            Self::ResolveDuplicateCertificateRoot => "resolve_duplicate_certificate_root",
            Self::ResolveAccountingRoot => "resolve_accounting_root",
            Self::ClearCircuitBreaker => "clear_circuit_breaker",
            Self::MaintainFinalityHold => "maintain_finality_hold",
            Self::UnlockAfterCertificateFinality => "unlock_after_certificate_finality",
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
        slot_kind: UnlockSlotKind,
        kind: CommandHintKind,
        blockers: &[UnlockBlocker],
    ) -> Self {
        let blocker_root = blockers_root("command-next-blockers", blockers);
        let command_root = record_root(
            "command-hint",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "kind": kind.as_str(),
                "blocker_root": blocker_root,
                "raw_command_absent": true,
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
pub struct UnlockGuardEntry {
    pub slot_kind: UnlockSlotKind,
    pub wave98_holdoff_roots: Vec<String>,
    pub finality_certificate_roots: Vec<String>,
    pub wallet_accounting_roots: Vec<String>,
    pub watchtower_unlock_guard_roots: Vec<String>,
    pub rollback_guard_roots: Vec<String>,
    pub circuit_breaker_roots: Vec<String>,
    pub operator_signoff_roots: Vec<String>,
    pub reviewer_signoff_roots: Vec<String>,
    pub wave98_holdoff_root: String,
    pub finality_certificate_root: String,
    pub wallet_accounting_root: String,
    pub watchtower_unlock_guard_root: String,
    pub rollback_guard_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub finality_unlock_guard_root: String,
    pub deterministic_release_root: String,
    pub blockers: Vec<UnlockBlocker>,
    pub status: UnlockStatus,
    pub command_hint: CommandHint,
    pub unlock_allowed: bool,
}

impl UnlockGuardEntry {
    pub fn empty(slot_kind: UnlockSlotKind, config: &Config) -> Self {
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
            config,
        )
    }

    pub fn from_roots(
        slot_kind: UnlockSlotKind,
        wave98_holdoff_roots: Vec<String>,
        finality_certificate_roots: Vec<String>,
        wallet_accounting_roots: Vec<String>,
        watchtower_unlock_guard_roots: Vec<String>,
        rollback_guard_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut entry = Self {
            slot_kind,
            wave98_holdoff_roots,
            finality_certificate_roots,
            wallet_accounting_roots,
            watchtower_unlock_guard_roots,
            rollback_guard_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            wave98_holdoff_root: empty_root("wave98-holdoff"),
            finality_certificate_root: empty_root("finality-certificate"),
            wallet_accounting_root: empty_root("wallet-accounting"),
            watchtower_unlock_guard_root: empty_root("watchtower-unlock-guard"),
            rollback_guard_root: empty_root("rollback-guard"),
            circuit_breaker_root: empty_root("circuit-breaker"),
            operator_signoff_root: empty_root("operator-signoff"),
            reviewer_signoff_root: empty_root("reviewer-signoff"),
            finality_unlock_guard_root: empty_root("finality-unlock-guard"),
            deterministic_release_root: empty_root("deterministic-release"),
            blockers: Vec::new(),
            status: UnlockStatus::CertificateAbsent,
            command_hint: CommandHint::new(slot_kind, CommandHintKind::HoldUnlock, &[]),
            unlock_allowed: false,
        };
        entry.recompute(config);
        entry
    }

    pub fn recompute(&mut self, config: &Config) {
        self.wave98_holdoff_root =
            aggregate_root("wave98-holdoff-roots", &self.wave98_holdoff_roots);
        self.finality_certificate_root = aggregate_root(
            "finality-certificate-roots",
            &self.finality_certificate_roots,
        );
        self.wallet_accounting_root =
            aggregate_root("wallet-accounting-roots", &self.wallet_accounting_roots);
        self.watchtower_unlock_guard_root = aggregate_root(
            "watchtower-unlock-guard-roots",
            &self.watchtower_unlock_guard_roots,
        );
        self.rollback_guard_root =
            aggregate_root("rollback-guard-roots", &self.rollback_guard_roots);
        self.circuit_breaker_root =
            aggregate_root("circuit-breaker-roots", &self.circuit_breaker_roots);
        self.operator_signoff_root =
            aggregate_root("operator-signoff-roots", &self.operator_signoff_roots);
        self.reviewer_signoff_root =
            aggregate_root("reviewer-signoff-roots", &self.reviewer_signoff_roots);
        self.finality_unlock_guard_root = finality_unlock_guard_root(self);
        self.deterministic_release_root = deterministic_release_root(self);
        self.blockers = unlock_blockers(self, config);
        self.status = if !config.finality_certificate_unlock_enabled {
            UnlockStatus::Denied
        } else if config.wallet_finality_blocker_active
            || config.watchtower_finality_blocker_active
            || config.circuit_breaker_active
        {
            UnlockStatus::Held
        } else if self.finality_certificate_roots.is_empty() {
            UnlockStatus::CertificateAbsent
        } else if self.blockers.is_empty() {
            UnlockStatus::UnlockReady
        } else {
            UnlockStatus::GuardBlocked
        };
        self.unlock_allowed = self.status.can_unlock()
            && config.finality_certificate_unlock_enabled
            && config.heavy_gates_ran
            && !config.wallet_finality_blocker_active
            && !config.watchtower_finality_blocker_active
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
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "finality_certificate_root": self.finality_certificate_root,
            "wallet_accounting_root": self.wallet_accounting_root,
            "watchtower_unlock_guard_root": self.watchtower_unlock_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "finality_unlock_guard_root": self.finality_unlock_guard_root,
            "deterministic_release_root": self.deterministic_release_root,
            "blocker_root": self.blocker_root(),
            "status": self.status.as_str(),
            "command_hint": self.command_hint.public_record(),
            "unlock_allowed": self.unlock_allowed,
            "roots_only": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock-guard-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockSummary {
    pub fail_closed: bool,
    pub release_denied: bool,
    pub unlock_allowed_count: u64,
    pub held_count: u64,
    pub blocked_count: u64,
    pub denied_count: u64,
    pub certificate_absent_count: u64,
    pub wave98_holdoff_root: String,
    pub finality_certificate_root: String,
    pub wallet_accounting_root: String,
    pub watchtower_unlock_guard_root: String,
    pub rollback_guard_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub finality_unlock_guard_root: String,
    pub deterministic_release_root: String,
    pub blocker_root: String,
    pub command_root: String,
    pub wallet_finality_blocker_active: bool,
    pub watchtower_finality_blocker_active: bool,
    pub circuit_breaker_active: bool,
    pub heavy_gates_ran: bool,
}

impl UnlockSummary {
    pub fn from_entries(config: &Config, entries: &BTreeMap<String, UnlockGuardEntry>) -> Self {
        let unlock_allowed_count = entries
            .values()
            .filter(|entry| entry.unlock_allowed)
            .count() as u64;
        let held_count = entries
            .values()
            .filter(|entry| entry.status == UnlockStatus::Held)
            .count() as u64;
        let blocked_count = entries
            .values()
            .filter(|entry| entry.status == UnlockStatus::GuardBlocked)
            .count() as u64;
        let denied_count = entries
            .values()
            .filter(|entry| entry.status == UnlockStatus::Denied)
            .count() as u64;
        let certificate_absent_count = entries
            .values()
            .filter(|entry| entry.status == UnlockStatus::CertificateAbsent)
            .count() as u64;
        let wave98_holdoff_root = entry_field_root(
            "summary-wave98-holdoff-roots",
            entries
                .values()
                .map(|entry| entry.wave98_holdoff_root.clone()),
        );
        let finality_certificate_root = entry_field_root(
            "summary-finality-certificate-roots",
            entries
                .values()
                .map(|entry| entry.finality_certificate_root.clone()),
        );
        let wallet_accounting_root = entry_field_root(
            "summary-wallet-accounting-roots",
            entries
                .values()
                .map(|entry| entry.wallet_accounting_root.clone()),
        );
        let watchtower_unlock_guard_root = entry_field_root(
            "summary-watchtower-unlock-guard-roots",
            entries
                .values()
                .map(|entry| entry.watchtower_unlock_guard_root.clone()),
        );
        let rollback_guard_root = entry_field_root(
            "summary-rollback-guard-roots",
            entries
                .values()
                .map(|entry| entry.rollback_guard_root.clone()),
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
        let finality_unlock_guard_root = entry_field_root(
            "summary-finality-unlock-guard-roots",
            entries
                .values()
                .map(|entry| entry.finality_unlock_guard_root.clone()),
        );
        let deterministic_release_root = entry_field_root(
            "summary-deterministic-release-roots",
            entries
                .values()
                .map(|entry| entry.deterministic_release_root.clone()),
        );
        let blocker_root = entry_field_root(
            "summary-blockers",
            entries.values().map(UnlockGuardEntry::blocker_root),
        );
        let command_root = merkle_root(
            "WAVE99-WALLET-WATCHTOWER-COMMAND-HINTS",
            &entries
                .values()
                .map(|entry| entry.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let fail_closed = unlock_allowed_count == 0
            || held_count > 0
            || blocked_count > 0
            || denied_count > 0
            || certificate_absent_count > 0
            || !config.finality_certificate_unlock_enabled
            || config.wallet_finality_blocker_active
            || config.watchtower_finality_blocker_active
            || config.circuit_breaker_active
            || !config.heavy_gates_ran;
        Self {
            fail_closed,
            release_denied: fail_closed,
            unlock_allowed_count,
            held_count,
            blocked_count,
            denied_count,
            certificate_absent_count,
            wave98_holdoff_root,
            finality_certificate_root,
            wallet_accounting_root,
            watchtower_unlock_guard_root,
            rollback_guard_root,
            circuit_breaker_root,
            operator_signoff_root,
            reviewer_signoff_root,
            finality_unlock_guard_root,
            deterministic_release_root,
            blocker_root,
            command_root,
            wallet_finality_blocker_active: config.wallet_finality_blocker_active,
            watchtower_finality_blocker_active: config.watchtower_finality_blocker_active,
            circuit_breaker_active: config.circuit_breaker_active,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "release_denied": self.release_denied,
            "unlock_allowed_count": self.unlock_allowed_count,
            "held_count": self.held_count,
            "blocked_count": self.blocked_count,
            "denied_count": self.denied_count,
            "certificate_absent_count": self.certificate_absent_count,
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "finality_certificate_root": self.finality_certificate_root,
            "wallet_accounting_root": self.wallet_accounting_root,
            "watchtower_unlock_guard_root": self.watchtower_unlock_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "finality_unlock_guard_root": self.finality_unlock_guard_root,
            "deterministic_release_root": self.deterministic_release_root,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "wallet_finality_blocker_active": self.wallet_finality_blocker_active,
            "watchtower_finality_blocker_active": self.watchtower_finality_blocker_active,
            "circuit_breaker_active": self.circuit_breaker_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub unlock_entries: BTreeMap<String, UnlockGuardEntry>,
    pub summary: UnlockSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let unlock_entries = UnlockSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let entry = UnlockGuardEntry::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = UnlockSummary::from_entries(&config, &unlock_entries);
        Self {
            config,
            unlock_entries,
            summary,
        }
    }

    pub fn stage_unlock_guard_entry(
        mut self,
        slot_kind: UnlockSlotKind,
        wave98_holdoff_roots: Vec<String>,
        finality_certificate_roots: Vec<String>,
        wallet_accounting_roots: Vec<String>,
        watchtower_unlock_guard_roots: Vec<String>,
        rollback_guard_roots: Vec<String>,
        circuit_breaker_roots: Vec<String>,
        operator_signoff_roots: Vec<String>,
        reviewer_signoff_roots: Vec<String>,
    ) -> Result<Self> {
        let entry = UnlockGuardEntry::from_roots(
            slot_kind,
            wave98_holdoff_roots,
            finality_certificate_roots,
            wallet_accounting_roots,
            watchtower_unlock_guard_roots,
            rollback_guard_roots,
            circuit_breaker_roots,
            operator_signoff_roots,
            reviewer_signoff_roots,
            &self.config,
        );
        self.unlock_entries
            .insert(slot_kind.as_str().to_string(), entry);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for entry in self.unlock_entries.values_mut() {
            entry.recompute(&self.config);
        }
        self.summary = UnlockSummary::from_entries(&self.config, &self.unlock_entries);
    }

    pub fn unlock_entry_roots(&self) -> BTreeMap<String, String> {
        self.unlock_entries
            .iter()
            .map(|(key, entry)| (key.clone(), entry.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn unlock_entries_root(&self) -> String {
        merkle_root(
            "WAVE99-WALLET-WATCHTOWER-UNLOCK-GUARD-ENTRY-ROOTS",
            &self
                .unlock_entry_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "unlock_entry_roots": self.unlock_entry_roots(),
            "unlock_entries_root": self.unlock_entries_root(),
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

pub fn wallet_watchtower_finality_certificate_unlock_guard_runtime() -> Runtime {
    devnet()
}

fn unlock_blockers(entry: &UnlockGuardEntry, config: &Config) -> Vec<UnlockBlocker> {
    let mut blockers = Vec::new();
    if config.require_wave98_holdoff_roots
        && entry.wave98_holdoff_roots.len() < config.min_wave98_holdoff_roots as usize
    {
        blockers.push(UnlockBlocker::Wave98HoldoffRootMissing);
    }
    if config.require_finality_certificate_roots
        && entry.finality_certificate_roots.len() < config.min_finality_certificate_roots as usize
    {
        blockers.push(UnlockBlocker::FinalityCertificateRootMissing);
    }
    if config.require_wallet_accounting_roots
        && entry.wallet_accounting_roots.len() < config.min_wallet_accounting_roots as usize
    {
        blockers.push(UnlockBlocker::WalletAccountingRootMissing);
    }
    if config.require_watchtower_unlock_guard_roots
        && entry.watchtower_unlock_guard_roots.len()
            < config.min_watchtower_unlock_guard_roots as usize
    {
        blockers.push(UnlockBlocker::WatchtowerUnlockGuardRootMissing);
    }
    if config.require_rollback_guard_roots
        && entry.rollback_guard_roots.len() < config.min_rollback_guard_roots as usize
    {
        blockers.push(UnlockBlocker::RollbackGuardRootMissing);
    }
    if config.require_circuit_breaker_roots
        && entry.circuit_breaker_roots.len() < config.min_circuit_breaker_roots as usize
    {
        blockers.push(UnlockBlocker::CircuitBreakerRootMissing);
    }
    if config.require_operator_signoff_roots
        && entry.operator_signoff_roots.len() < config.min_operator_signoff_roots as usize
    {
        blockers.push(UnlockBlocker::OperatorSignoffRootMissing);
    }
    if config.require_reviewer_signoff_roots
        && entry.reviewer_signoff_roots.len() < config.min_reviewer_signoff_roots as usize
    {
        blockers.push(UnlockBlocker::ReviewerSignoffRootMissing);
    }
    if has_duplicate(&entry.finality_certificate_roots) {
        blockers.push(UnlockBlocker::DuplicateFinalityCertificateRoot);
    }
    if has_duplicate(&entry.wallet_accounting_roots) {
        blockers.push(UnlockBlocker::DuplicateWalletAccountingRoot);
    }
    if !entry.finality_certificate_roots.is_empty()
        && entry.rollback_guard_roots.len() < entry.finality_certificate_roots.len()
    {
        blockers.push(UnlockBlocker::RollbackGuardLagging);
    }
    if config.circuit_breaker_active {
        blockers.push(UnlockBlocker::CircuitBreakerActive);
    }
    if !roots_shape_valid(entry) {
        blockers.push(UnlockBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(entry) {
        blockers.push(UnlockBlocker::RootsOnlyRecordMissing);
    }
    if config.wallet_finality_blocker_active {
        blockers.push(UnlockBlocker::WalletFinalityBlockerActive);
    }
    if config.watchtower_finality_blocker_active {
        blockers.push(UnlockBlocker::WatchtowerFinalityBlockerActive);
    }
    if !config.finality_certificate_unlock_enabled {
        blockers.push(UnlockBlocker::UnlockDisabled);
    }
    if !config.heavy_gates_ran {
        blockers.push(UnlockBlocker::HeavyGatesNotRun);
    }
    blockers
}

fn command_for_entry(entry: &UnlockGuardEntry) -> CommandHintKind {
    if entry.blockers.is_empty() {
        return CommandHintKind::UnlockAfterCertificateFinality;
    }
    match entry.blockers[0] {
        UnlockBlocker::Wave98HoldoffRootMissing => CommandHintKind::ImportWave98HoldoffRoot,
        UnlockBlocker::FinalityCertificateRootMissing => {
            CommandHintKind::ImportFinalityCertificateRoot
        }
        UnlockBlocker::WalletAccountingRootMissing => CommandHintKind::ImportWalletAccountingRoot,
        UnlockBlocker::WatchtowerUnlockGuardRootMissing => {
            CommandHintKind::ImportWatchtowerUnlockGuardRoot
        }
        UnlockBlocker::RollbackGuardRootMissing => CommandHintKind::ImportRollbackGuardRoot,
        UnlockBlocker::CircuitBreakerRootMissing => CommandHintKind::ImportCircuitBreakerRoot,
        UnlockBlocker::OperatorSignoffRootMissing => CommandHintKind::ImportOperatorSignoffRoot,
        UnlockBlocker::ReviewerSignoffRootMissing => CommandHintKind::ImportReviewerSignoffRoot,
        UnlockBlocker::DuplicateFinalityCertificateRoot => {
            CommandHintKind::ResolveDuplicateCertificateRoot
        }
        UnlockBlocker::DuplicateWalletAccountingRoot | UnlockBlocker::RollbackGuardLagging => {
            CommandHintKind::ResolveAccountingRoot
        }
        UnlockBlocker::CircuitBreakerActive => CommandHintKind::ClearCircuitBreaker,
        UnlockBlocker::WalletFinalityBlockerActive
        | UnlockBlocker::WatchtowerFinalityBlockerActive => CommandHintKind::MaintainFinalityHold,
        UnlockBlocker::RootShapeInvalid
        | UnlockBlocker::RootsOnlyRecordMissing
        | UnlockBlocker::UnlockDisabled
        | UnlockBlocker::HeavyGatesNotRun => CommandHintKind::HoldUnlock,
    }
}

fn roots_only_record_present(entry: &UnlockGuardEntry) -> bool {
    is_root_like(&entry.wave98_holdoff_root)
        && is_root_like(&entry.finality_certificate_root)
        && is_root_like(&entry.wallet_accounting_root)
        && is_root_like(&entry.watchtower_unlock_guard_root)
        && is_root_like(&entry.rollback_guard_root)
        && is_root_like(&entry.circuit_breaker_root)
        && is_root_like(&entry.operator_signoff_root)
        && is_root_like(&entry.reviewer_signoff_root)
        && is_root_like(&entry.finality_unlock_guard_root)
        && is_root_like(&entry.deterministic_release_root)
}

fn roots_shape_valid(entry: &UnlockGuardEntry) -> bool {
    all_roots_like(&entry.wave98_holdoff_roots)
        && all_roots_like(&entry.finality_certificate_roots)
        && all_roots_like(&entry.wallet_accounting_roots)
        && all_roots_like(&entry.watchtower_unlock_guard_roots)
        && all_roots_like(&entry.rollback_guard_roots)
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

fn finality_unlock_guard_root(entry: &UnlockGuardEntry) -> String {
    record_root(
        "finality-unlock-guard-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "wave98_holdoff_root": entry.wave98_holdoff_root,
            "finality_certificate_root": entry.finality_certificate_root,
            "wallet_accounting_root": entry.wallet_accounting_root,
            "watchtower_unlock_guard_root": entry.watchtower_unlock_guard_root,
            "rollback_guard_root": entry.rollback_guard_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "wallet_material_absent": true,
            "watchtower_identity_material_absent": true,
            "certificate_payload_absent": true,
            "route_material_absent": true,
            "roots_only": true,
        }),
    )
}

fn deterministic_release_root(entry: &UnlockGuardEntry) -> String {
    record_root(
        "deterministic-release-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "finality_unlock_guard_root": entry.finality_unlock_guard_root,
            "finality_certificate_root": entry.finality_certificate_root,
            "wallet_accounting_root": entry.wallet_accounting_root,
            "rollback_guard_root": entry.rollback_guard_root,
            "circuit_breaker_root": entry.circuit_breaker_root,
            "operator_signoff_root": entry.operator_signoff_root,
            "reviewer_signoff_root": entry.reviewer_signoff_root,
            "raw_certificate_absent": true,
            "raw_release_claim_absent": true,
            "roots_only": true,
        }),
    )
}

fn aggregate_root(domain: &str, roots: &[String]) -> String {
    if roots.is_empty() {
        return empty_root(domain);
    }
    merkle_root(
        "WAVE99-WALLET-WATCHTOWER-ROOT-AGGREGATE",
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

fn blockers_root(domain: &str, blockers: &[UnlockBlocker]) -> String {
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
        "empty-unlock-guard-root",
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
        "WAVE99-WALLET-WATCHTOWER-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
