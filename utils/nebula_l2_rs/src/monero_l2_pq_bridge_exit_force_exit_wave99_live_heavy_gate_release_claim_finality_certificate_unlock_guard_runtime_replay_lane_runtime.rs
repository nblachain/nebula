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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave99-release-claim-finality-certificate-unlock-guard-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 99;
pub const DEFAULT_HOLDOFF_WAVE: u64 = 98;
pub const DEFAULT_SEAL_WAVE: u64 = 97;
pub const DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS: u64 = 6;
pub const DEFAULT_MIN_REPLAY_ACCOUNTING_ROOTS: u64 = 6;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    RuntimeReplay,
    ReplayAccounting,
    RollbackGuard,
    CircuitBreaker,
    FinalityCertificate,
    OperatorSignoff,
    ReviewerSignoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RuntimeReplay,
            Self::ReplayAccounting,
            Self::RollbackGuard,
            Self::CircuitBreaker,
            Self::FinalityCertificate,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime_replay",
            Self::ReplayAccounting => "replay_accounting",
            Self::RollbackGuard => "rollback_guard",
            Self::CircuitBreaker => "circuit_breaker",
            Self::FinalityCertificate => "finality_certificate",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockStatus {
    Blocked,
    CertificateMissing,
    HeldByWave98,
    ReleaseDenied,
}

impl UnlockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::CertificateMissing => "certificate_missing",
            Self::HeldByWave98 => "held_by_wave98",
            Self::ReleaseDenied => "release_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalityBlocker {
    FailClosedArmed,
    ReleaseDenied,
    HeavyGateReceiptAbsent,
    Wave98HoldoffRootActive,
    FinalityCertificateRootMissing,
    ReplayAccountingRootMissing,
    RollbackGuardRootActive,
    CircuitBreakerRootActive,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    UnlockCertificateAbsent,
}

impl FinalityBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ReleaseDenied,
            Self::HeavyGateReceiptAbsent,
            Self::Wave98HoldoffRootActive,
            Self::FinalityCertificateRootMissing,
            Self::ReplayAccountingRootMissing,
            Self::RollbackGuardRootActive,
            Self::CircuitBreakerRootActive,
            Self::OperatorSignoffRootMissing,
            Self::ReviewerSignoffRootMissing,
            Self::UnlockCertificateAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ReleaseDenied => "release_denied",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::Wave98HoldoffRootActive => "wave98_holdoff_root_active",
            Self::FinalityCertificateRootMissing => "finality_certificate_root_missing",
            Self::ReplayAccountingRootMissing => "replay_accounting_root_missing",
            Self::RollbackGuardRootActive => "rollback_guard_root_active",
            Self::CircuitBreakerRootActive => "circuit_breaker_root_active",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::ReviewerSignoffRootMissing => "reviewer_signoff_root_missing",
            Self::UnlockCertificateAbsent => "unlock_certificate_absent",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    CarryWave98HoldoffRoots,
    DenyUnlockUntilCertificateRootsClear,
    BindReplayAccountingRoots,
    BindRollbackGuardRoots,
    BindCircuitBreakerRoots,
    RequireOperatorSignoffRoot,
    RequireReviewerSignoffRoot,
    KeepHeavyGateReceiptAbsent,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CarryWave98HoldoffRoots,
            Self::DenyUnlockUntilCertificateRootsClear,
            Self::BindReplayAccountingRoots,
            Self::BindRollbackGuardRoots,
            Self::BindCircuitBreakerRoots,
            Self::RequireOperatorSignoffRoot,
            Self::RequireReviewerSignoffRoot,
            Self::KeepHeavyGateReceiptAbsent,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CarryWave98HoldoffRoots => "carry_wave98_holdoff_roots",
            Self::DenyUnlockUntilCertificateRootsClear => {
                "deny_unlock_until_certificate_roots_clear"
            }
            Self::BindReplayAccountingRoots => "bind_replay_accounting_roots",
            Self::BindRollbackGuardRoots => "bind_rollback_guard_roots",
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
    pub holdoff_wave: u64,
    pub seal_wave: u64,
    pub min_finality_certificate_roots: u64,
    pub min_replay_accounting_roots: u64,
    pub wave98_runtime_replay_holdoff_root: String,
    pub wave98_holdoff_ledger_root: String,
    pub wave98_command_hints_root: String,
    pub fail_closed_armed: bool,
    pub release_denied: bool,
    pub finality_blockers_active: bool,
    pub unlock_certificates_present: bool,
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
            holdoff_wave: DEFAULT_HOLDOFF_WAVE,
            seal_wave: DEFAULT_SEAL_WAVE,
            min_finality_certificate_roots: DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS,
            min_replay_accounting_roots: DEFAULT_MIN_REPLAY_ACCOUNTING_ROOTS,
            wave98_runtime_replay_holdoff_root: stable_root(
                "wave98-runtime-replay-holdoff",
                "runtime-replay",
            ),
            wave98_holdoff_ledger_root: stable_root("wave98-holdoff-ledger", "all"),
            wave98_command_hints_root: stable_root("wave98-command-hints", "runtime-replay"),
            fail_closed_armed: true,
            release_denied: true,
            finality_blockers_active: true,
            unlock_certificates_present: false,
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
        ensure_positive("holdoff_wave", self.holdoff_wave)?;
        ensure_positive("seal_wave", self.seal_wave)?;
        ensure_positive(
            "min_finality_certificate_roots",
            self.min_finality_certificate_roots,
        )?;
        ensure_positive(
            "min_replay_accounting_roots",
            self.min_replay_accounting_roots,
        )?;
        ensure_root(
            "wave98_runtime_replay_holdoff_root",
            &self.wave98_runtime_replay_holdoff_root,
        )?;
        ensure_root(
            "wave98_holdoff_ledger_root",
            &self.wave98_holdoff_ledger_root,
        )?;
        ensure_root("wave98_command_hints_root", &self.wave98_command_hints_root)?;
        if !(self.seal_wave < self.holdoff_wave && self.holdoff_wave < self.wave) {
            return Err("wave ordering must be seal, holdoff, finality".to_string());
        }
        if !self.fail_closed_armed {
            return Err("finality unlock guard is disarmed".to_string());
        }
        if !self.release_denied {
            return Err("devnet finality unlock guard must deny release".to_string());
        }
        if !self.finality_blockers_active {
            return Err("devnet finality blockers must remain active".to_string());
        }
        if self.unlock_certificates_present {
            return Err("devnet must not publish unlock certificates".to_string());
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
            "kind": "wave99_finality_certificate_unlock_guard_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "holdoff_wave": self.holdoff_wave,
            "seal_wave": self.seal_wave,
            "min_finality_certificate_roots": self.min_finality_certificate_roots,
            "min_replay_accounting_roots": self.min_replay_accounting_roots,
            "wave98_runtime_replay_holdoff_root": self.wave98_runtime_replay_holdoff_root,
            "wave98_holdoff_ledger_root": self.wave98_holdoff_ledger_root,
            "wave98_command_hints_root": self.wave98_command_hints_root,
            "fail_closed_armed": self.fail_closed_armed,
            "release_denied": self.release_denied,
            "finality_blockers_active": self.finality_blockers_active,
            "unlock_certificates_present": self.unlock_certificates_present,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE99-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockGuardEntry {
    pub lane: ReplayLane,
    pub wave98_holdoff_root: String,
    pub finality_certificate_root: String,
    pub replay_accounting_root: String,
    pub rollback_guard_root: String,
    pub circuit_breaker_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub command_hint_root: String,
    pub blocker_roots: Vec<String>,
    pub status: UnlockStatus,
    pub unlock_allowed: bool,
}

impl UnlockGuardEntry {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            wave98_holdoff_root: lane_wave98_holdoff_root(lane, config),
            finality_certificate_root: lane_root("finality-certificate", lane),
            replay_accounting_root: lane_root("replay-accounting", lane),
            rollback_guard_root: lane_root("rollback-guard", lane),
            circuit_breaker_root: lane_root("circuit-breaker", lane),
            operator_signoff_root: lane_root("operator-signoff", lane),
            reviewer_signoff_root: lane_root("reviewer-signoff", lane),
            command_hint_root: lane_root("command-hint", lane),
            blocker_roots: FinalityBlocker::all()
                .iter()
                .map(|blocker| blocker_root(lane, *blocker))
                .collect(),
            status: UnlockStatus::ReleaseDenied,
            unlock_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("wave98_holdoff_root", &self.wave98_holdoff_root)?;
        ensure_root("finality_certificate_root", &self.finality_certificate_root)?;
        ensure_root("replay_accounting_root", &self.replay_accounting_root)?;
        ensure_root("rollback_guard_root", &self.rollback_guard_root)?;
        ensure_root("circuit_breaker_root", &self.circuit_breaker_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.blocker_roots.is_empty() {
            return Err("unlock guard entry requires blocker roots".to_string());
        }
        for root in self.blocker_roots.iter() {
            ensure_root("blocker_root", root)?;
        }
        if self.unlock_allowed {
            return Err("devnet unlock guard cannot allow release".to_string());
        }
        Ok(())
    }

    pub fn blocker_root(&self) -> String {
        list_root("WAVE99-FINALITY-BLOCKER-ROOTS", self.blocker_roots.clone())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave99_unlock_guard_entry",
            "lane": self.lane.as_str(),
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "finality_certificate_root": self.finality_certificate_root,
            "replay_accounting_root": self.replay_accounting_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "command_hint_root": self.command_hint_root,
            "blocker_roots_root": self.blocker_root(),
            "blocker_count": self.blocker_roots.len(),
            "status": self.status.as_str(),
            "unlock_allowed": self.unlock_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE99-UNLOCK-GUARD-ENTRY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootLedger {
    pub finality_certificate_roots: BTreeMap<String, String>,
    pub replay_accounting_roots: BTreeMap<String, String>,
    pub rollback_guard_roots: BTreeMap<String, String>,
    pub circuit_breaker_roots: BTreeMap<String, String>,
    pub operator_signoff_roots: BTreeMap<String, String>,
    pub reviewer_signoff_roots: BTreeMap<String, String>,
    pub command_hints: BTreeMap<String, String>,
}

impl RootLedger {
    pub fn devnet(lanes: &[ReplayLane]) -> Self {
        Self {
            finality_certificate_roots: lane_map(lanes, "finality-certificate"),
            replay_accounting_roots: lane_map(lanes, "replay-accounting"),
            rollback_guard_roots: lane_map(lanes, "rollback-guard"),
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
        ensure_map_roots(
            "finality_certificate_roots",
            &self.finality_certificate_roots,
        )?;
        ensure_map_roots("replay_accounting_roots", &self.replay_accounting_roots)?;
        ensure_map_roots("rollback_guard_roots", &self.rollback_guard_roots)?;
        ensure_map_roots("circuit_breaker_roots", &self.circuit_breaker_roots)?;
        ensure_map_roots("operator_signoff_roots", &self.operator_signoff_roots)?;
        ensure_map_roots("reviewer_signoff_roots", &self.reviewer_signoff_roots)?;
        ensure_map_roots("command_hints", &self.command_hints)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave99_root_ledger",
            "finality_certificate_root": map_root("WAVE99-FINALITY-CERTIFICATE-MAP", &self.finality_certificate_roots),
            "replay_accounting_root": map_root("WAVE99-REPLAY-ACCOUNTING-MAP", &self.replay_accounting_roots),
            "rollback_guard_root": map_root("WAVE99-ROLLBACK-GUARD-MAP", &self.rollback_guard_roots),
            "circuit_breaker_root": map_root("WAVE99-CIRCUIT-BREAKER-MAP", &self.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE99-OPERATOR-SIGNOFF-MAP", &self.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE99-REVIEWER-SIGNOFF-MAP", &self.reviewer_signoff_roots),
            "command_hint_root": map_root("WAVE99-COMMAND-HINT-MAP", &self.command_hints),
            "finality_certificate_count": self.finality_certificate_roots.len(),
            "replay_accounting_count": self.replay_accounting_roots.len(),
            "rollback_guard_count": self.rollback_guard_roots.len(),
            "circuit_breaker_count": self.circuit_breaker_roots.len(),
            "operator_signoff_count": self.operator_signoff_roots.len(),
            "reviewer_signoff_count": self.reviewer_signoff_roots.len(),
            "command_hint_count": self.command_hints.len(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE99-ROOT-LEDGER", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_count: u64,
    pub unlock_guard_count: u64,
    pub blocked_unlock_guards: u64,
    pub unlock_allowed_count: u64,
    pub finality_certificate_roots: u64,
    pub replay_accounting_roots: u64,
    pub rollback_guard_roots: u64,
    pub circuit_breaker_roots: u64,
    pub operator_signoff_roots: u64,
    pub reviewer_signoff_roots: u64,
    pub raw_public_records: u64,
}

impl Counters {
    pub fn from_parts(entries: &[UnlockGuardEntry], ledger: &RootLedger) -> Self {
        Self {
            lane_count: ReplayLane::all().len() as u64,
            unlock_guard_count: entries.len() as u64,
            blocked_unlock_guards: entries.iter().filter(|entry| !entry.unlock_allowed).count()
                as u64,
            unlock_allowed_count: entries.iter().filter(|entry| entry.unlock_allowed).count()
                as u64,
            finality_certificate_roots: ledger.finality_certificate_roots.len() as u64,
            replay_accounting_roots: ledger.replay_accounting_roots.len() as u64,
            rollback_guard_roots: ledger.rollback_guard_roots.len() as u64,
            circuit_breaker_roots: ledger.circuit_breaker_roots.len() as u64,
            operator_signoff_roots: ledger.operator_signoff_roots.len() as u64,
            reviewer_signoff_roots: ledger.reviewer_signoff_roots.len() as u64,
            raw_public_records: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave99_unlock_guard_counters",
            "lane_count": self.lane_count,
            "unlock_guard_count": self.unlock_guard_count,
            "blocked_unlock_guards": self.blocked_unlock_guards,
            "unlock_allowed_count": self.unlock_allowed_count,
            "finality_certificate_roots": self.finality_certificate_roots,
            "replay_accounting_roots": self.replay_accounting_roots,
            "rollback_guard_roots": self.rollback_guard_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "operator_signoff_roots": self.operator_signoff_roots,
            "reviewer_signoff_roots": self.reviewer_signoff_roots,
            "raw_public_records": self.raw_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE99-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub unlock_guards: Vec<UnlockGuardEntry>,
    pub root_ledger: RootLedger,
    pub blocker_catalog: BTreeMap<String, String>,
    pub wave98_holdoff_catalog: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        unlock_guards: Vec<UnlockGuardEntry>,
        root_ledger: RootLedger,
        blocker_catalog: BTreeMap<String, String>,
        wave98_holdoff_catalog: BTreeMap<String, String>,
    ) -> Result<Self> {
        let counters = Counters::from_parts(&unlock_guards, &root_ledger);
        let state = Self {
            config,
            unlock_guards,
            root_ledger,
            blocker_catalog,
            wave98_holdoff_catalog,
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
            "unlock_guards",
            self.unlock_guards.iter().map(|entry| entry.lane).collect(),
        )?;
        for entry in self.unlock_guards.iter() {
            entry.validate()?;
        }
        self.root_ledger.validate()?;
        ensure_map_roots("blocker_catalog", &self.blocker_catalog)?;
        ensure_map_roots("wave98_holdoff_catalog", &self.wave98_holdoff_catalog)?;
        if self.counters.unlock_allowed_count != 0 {
            return Err("devnet must have zero allowed unlocks".to_string());
        }
        if self.counters.blocked_unlock_guards != self.counters.unlock_guard_count {
            return Err("all devnet unlock guards must remain blocked".to_string());
        }
        if self.counters.raw_public_records != 0 {
            return Err("public record counter must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave99_finality_certificate_unlock_guard_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "wave98_runtime_replay_holdoff_root": self.config.wave98_runtime_replay_holdoff_root,
            "wave98_holdoff_ledger_root": self.config.wave98_holdoff_ledger_root,
            "wave98_command_hints_root": self.config.wave98_command_hints_root,
            "unlock_guards_root": unlock_guards_root(&self.unlock_guards),
            "root_ledger_root": self.root_ledger.state_root(),
            "blocker_catalog_root": map_root("WAVE99-BLOCKER-CATALOG", &self.blocker_catalog),
            "wave98_holdoff_catalog_root": map_root("WAVE99-WAVE98-HOLDOFF-CATALOG", &self.wave98_holdoff_catalog),
            "finality_certificate_root": map_root("WAVE99-FINALITY-CERTIFICATE-ROOTS", &self.root_ledger.finality_certificate_roots),
            "replay_accounting_root": map_root("WAVE99-REPLAY-ACCOUNTING-ROOTS", &self.root_ledger.replay_accounting_roots),
            "rollback_guard_root": map_root("WAVE99-ROLLBACK-GUARD-ROOTS", &self.root_ledger.rollback_guard_roots),
            "circuit_breaker_root": map_root("WAVE99-CIRCUIT-BREAKER-ROOTS", &self.root_ledger.circuit_breaker_roots),
            "operator_signoff_root": map_root("WAVE99-OPERATOR-SIGNOFF-ROOTS", &self.root_ledger.operator_signoff_roots),
            "reviewer_signoff_root": map_root("WAVE99-REVIEWER-SIGNOFF-ROOTS", &self.root_ledger.reviewer_signoff_roots),
            "command_hints_root": map_root("WAVE99-COMMAND-HINTS", &self.root_ledger.command_hints),
            "counters_root": self.counters.state_root(),
            "release_denied": self.config.release_denied,
            "finality_blockers_active": self.config.finality_blockers_active,
            "unlock_certificates_present": self.config.unlock_certificates_present,
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
        value_root("WAVE99-STATE", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let lanes = ReplayLane::all();
    let unlock_guards = lanes
        .iter()
        .map(|lane| UnlockGuardEntry::blocked(*lane, &config))
        .collect::<Vec<_>>();
    let root_ledger = RootLedger::devnet(&lanes);
    let blocker_catalog = FinalityBlocker::all()
        .iter()
        .map(|blocker| (blocker.as_str().to_string(), blocker_kind_root(*blocker)))
        .collect::<BTreeMap<_, _>>();
    let wave98_holdoff_catalog = lanes
        .iter()
        .map(|lane| {
            (
                lane.as_str().to_string(),
                lane_wave98_holdoff_root(*lane, &config),
            )
        })
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        unlock_guards,
        root_ledger,
        blocker_catalog,
        wave98_holdoff_catalog,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            unlock_guards: Vec::new(),
            root_ledger: RootLedger::devnet(&[]),
            blocker_catalog: BTreeMap::new(),
            wave98_holdoff_catalog: BTreeMap::new(),
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

fn unlock_guards_root(entries: &[UnlockGuardEntry]) -> String {
    list_root(
        "WAVE99-UNLOCK-GUARD-STATE-ROOTS",
        entries.iter().map(UnlockGuardEntry::state_root).collect(),
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

fn lane_wave98_holdoff_root(lane: ReplayLane, config: &Config) -> String {
    domain_hash(
        "WAVE99-LANE-WAVE98-HOLDOFF-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&config.wave98_runtime_replay_holdoff_root),
            HashPart::Str(&config.wave98_holdoff_ledger_root),
        ],
        32,
    )
}

fn lane_root(kind: &str, lane: ReplayLane) -> String {
    domain_hash(
        "WAVE99-LANE-ROOT",
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
        "WAVE99-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: FinalityBlocker) -> String {
    domain_hash(
        "WAVE99-FINALITY-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn blocker_kind_root(blocker: FinalityBlocker) -> String {
    domain_hash(
        "WAVE99-FINALITY-BLOCKER-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn stable_root(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE99-STABLE-ROOT",
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
