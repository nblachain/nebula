use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave100-release-execution-settlement-notary-guard-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE99_UNLOCK_GUARD_ROOT: &str =
    "root:wave99:bridge-custody-finality-certificate-unlock-guard-blocked-placeholder";
pub const DEFAULT_MIN_UNLOCK_GUARD_ROOTS: u16 = 5;
pub const DEFAULT_MIN_EXECUTION_BUNDLE_ROOTS: u16 = 4;
pub const DEFAULT_MIN_SETTLEMENT_ACCOUNTING_ROOTS: u16 = 4;
pub const DEFAULT_MIN_NOTARY_QUORUM_ROOTS: u16 = 5;
pub const DEFAULT_MIN_PAYOUT_ENVELOPE_ROOTS: u16 = 3;
pub const DEFAULT_MIN_SIGNOFF_ROOTS: u16 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave99_unlock_guard_root: String,
    pub min_unlock_guard_roots: u16,
    pub min_execution_bundle_roots: u16,
    pub min_settlement_accounting_roots: u16,
    pub min_notary_quorum_roots: u16,
    pub min_payout_envelope_roots: u16,
    pub min_signoff_roots: u16,
    pub roots_only_public_records: bool,
    pub notarized_execution_bundles_enabled: bool,
    pub release_execution_enabled: bool,
    pub settlement_enabled: bool,
    pub custody_settlement_blockers_active: bool,
    pub notary_guard_blockers_active: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave99_unlock_guard_root: WAVE99_UNLOCK_GUARD_ROOT.to_string(),
            min_unlock_guard_roots: DEFAULT_MIN_UNLOCK_GUARD_ROOTS,
            min_execution_bundle_roots: DEFAULT_MIN_EXECUTION_BUNDLE_ROOTS,
            min_settlement_accounting_roots: DEFAULT_MIN_SETTLEMENT_ACCOUNTING_ROOTS,
            min_notary_quorum_roots: DEFAULT_MIN_NOTARY_QUORUM_ROOTS,
            min_payout_envelope_roots: DEFAULT_MIN_PAYOUT_ENVELOPE_ROOTS,
            min_signoff_roots: DEFAULT_MIN_SIGNOFF_ROOTS,
            roots_only_public_records: true,
            notarized_execution_bundles_enabled: false,
            release_execution_enabled: false,
            settlement_enabled: false,
            custody_settlement_blockers_active: true,
            notary_guard_blockers_active: true,
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
        ensure_root_like("wave99_unlock_guard_root", &self.wave99_unlock_guard_root)?;
        ensure_nonzero("unlock guard root quorum", self.min_unlock_guard_roots)?;
        ensure_nonzero(
            "execution bundle root quorum",
            self.min_execution_bundle_roots,
        )?;
        ensure_nonzero(
            "settlement accounting root quorum",
            self.min_settlement_accounting_roots,
        )?;
        ensure_nonzero("notary quorum root quorum", self.min_notary_quorum_roots)?;
        ensure_nonzero(
            "payout envelope root quorum",
            self.min_payout_envelope_roots,
        )?;
        ensure_nonzero("signoff root quorum", self.min_signoff_roots)?;
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.notarized_execution_bundles_enabled
            || self.release_execution_enabled
            || self.settlement_enabled
        {
            return Err("devnet release execution settlement must remain denied".to_string());
        }
        if !self.custody_settlement_blockers_active || !self.notary_guard_blockers_active {
            return Err(
                "devnet custody settlement and notary blockers must remain active".to_string(),
            );
        }
        if self.heavy_gates_ran {
            return Err("wave100 must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_root": deterministic_field_root("chain", &self.chain_id),
            "lane_root": deterministic_field_root("lane", &self.lane_id),
            "protocol_root": deterministic_field_root("protocol", &self.protocol_version),
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "min_unlock_guard_roots": self.min_unlock_guard_roots,
            "min_execution_bundle_roots": self.min_execution_bundle_roots,
            "min_settlement_accounting_roots": self.min_settlement_accounting_roots,
            "min_notary_quorum_roots": self.min_notary_quorum_roots,
            "min_payout_envelope_roots": self.min_payout_envelope_roots,
            "min_signoff_roots": self.min_signoff_roots,
            "roots_only_public_records": self.roots_only_public_records,
            "notarized_execution_bundles_enabled": self.notarized_execution_bundles_enabled,
            "release_execution_enabled": self.release_execution_enabled,
            "settlement_enabled": self.settlement_enabled,
            "custody_settlement_blockers_active": self.custody_settlement_blockers_active,
            "notary_guard_blockers_active": self.notary_guard_blockers_active,
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
    Wave99UnlockGuard,
    CustodyExecutionBundle,
    EscrowSettlementAccounting,
    NotaryQuorum,
    PayoutEnvelope,
    RollbackSentinel,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    CommandHint,
    DeterministicSettlementGuard,
}

impl RootKind {
    pub fn all() -> [Self; 11] {
        [
            Self::Wave99UnlockGuard,
            Self::CustodyExecutionBundle,
            Self::EscrowSettlementAccounting,
            Self::NotaryQuorum,
            Self::PayoutEnvelope,
            Self::RollbackSentinel,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::CommandHint,
            Self::DeterministicSettlementGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuard => "wave99_unlock_guard",
            Self::CustodyExecutionBundle => "custody_execution_bundle",
            Self::EscrowSettlementAccounting => "escrow_settlement_accounting",
            Self::NotaryQuorum => "notary_quorum",
            Self::PayoutEnvelope => "payout_envelope",
            Self::RollbackSentinel => "rollback_sentinel",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::CommandHint => "command_hint",
            Self::DeterministicSettlementGuard => "deterministic_settlement_guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave99UnlockGuardActive,
    ExecutionBundleMissing,
    ExecutionBundleNotNotarized,
    EscrowSettlementUnbalanced,
    NotaryQuorumMissing,
    PayoutEnvelopeSealed,
    RollbackSentinelActive,
    CircuitBreakerArmed,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    ReleaseExecutionDisabled,
    SettlementDisabled,
    HeavyGateMissing,
}

impl BlockerKind {
    pub fn all() -> [Self; 13] {
        [
            Self::Wave99UnlockGuardActive,
            Self::ExecutionBundleMissing,
            Self::ExecutionBundleNotNotarized,
            Self::EscrowSettlementUnbalanced,
            Self::NotaryQuorumMissing,
            Self::PayoutEnvelopeSealed,
            Self::RollbackSentinelActive,
            Self::CircuitBreakerArmed,
            Self::OperatorSignoffMissing,
            Self::ReviewerSignoffMissing,
            Self::ReleaseExecutionDisabled,
            Self::SettlementDisabled,
            Self::HeavyGateMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave99UnlockGuardActive => "wave99_unlock_guard_active",
            Self::ExecutionBundleMissing => "execution_bundle_missing",
            Self::ExecutionBundleNotNotarized => "execution_bundle_not_notarized",
            Self::EscrowSettlementUnbalanced => "escrow_settlement_unbalanced",
            Self::NotaryQuorumMissing => "notary_quorum_missing",
            Self::PayoutEnvelopeSealed => "payout_envelope_sealed",
            Self::RollbackSentinelActive => "rollback_sentinel_active",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::ReleaseExecutionDisabled => "release_execution_disabled",
            Self::SettlementDisabled => "settlement_disabled",
            Self::HeavyGateMissing => "heavy_gate_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepReleaseExecutionDisabled,
    KeepSettlementDisabled,
    KeepCustodyBlockersActive,
    RequireWave99UnlockGuardRoots,
    RequireNotaryQuorumRoots,
    ReconcileEscrowSettlementRoots,
    ReviewPayoutEnvelopeRoots,
    HoldRollbackSentinels,
    KeepCircuitBreakersArmed,
    ReviewOperatorSignoffs,
    ReviewReviewerSignoffs,
}

impl CommandHintKind {
    pub fn all() -> [Self; 11] {
        [
            Self::KeepReleaseExecutionDisabled,
            Self::KeepSettlementDisabled,
            Self::KeepCustodyBlockersActive,
            Self::RequireWave99UnlockGuardRoots,
            Self::RequireNotaryQuorumRoots,
            Self::ReconcileEscrowSettlementRoots,
            Self::ReviewPayoutEnvelopeRoots,
            Self::HoldRollbackSentinels,
            Self::KeepCircuitBreakersArmed,
            Self::ReviewOperatorSignoffs,
            Self::ReviewReviewerSignoffs,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseExecutionDisabled => "keep_release_execution_disabled",
            Self::KeepSettlementDisabled => "keep_settlement_disabled",
            Self::KeepCustodyBlockersActive => "keep_custody_blockers_active",
            Self::RequireWave99UnlockGuardRoots => "require_wave99_unlock_guard_roots",
            Self::RequireNotaryQuorumRoots => "require_notary_quorum_roots",
            Self::ReconcileEscrowSettlementRoots => "reconcile_escrow_settlement_roots",
            Self::ReviewPayoutEnvelopeRoots => "review_payout_envelope_roots",
            Self::HoldRollbackSentinels => "hold_rollback_sentinels",
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
    pub required_for_settlement: bool,
}

impl RootEntry {
    pub fn devnet(kind: RootKind, config: &Config) -> Self {
        let root = match kind {
            RootKind::Wave99UnlockGuard => config.wave99_unlock_guard_root.clone(),
            _ => deterministic_root(kind.as_str()),
        };
        Self {
            kind,
            root,
            blocker_active: kind != RootKind::CommandHint
                && kind != RootKind::DeterministicSettlementGuard,
            required_for_settlement: kind != RootKind::CommandHint,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "root": self.root,
            "blocker_active": self.blocker_active,
            "required_for_settlement": self.required_for_settlement,
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
pub struct SettlementDecision {
    pub wave99_unlock_guard_root: String,
    pub custody_execution_bundle_root: String,
    pub escrow_settlement_accounting_root: String,
    pub notary_quorum_root: String,
    pub payout_envelope_root: String,
    pub rollback_sentinel_root: String,
    pub circuit_breaker_root: String,
    pub signoff_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub notarized_execution_bundles: bool,
    pub release_execution_allowed: bool,
    pub settlement_allowed: bool,
}

impl SettlementDecision {
    pub fn from_parts(
        entries: &BTreeMap<String, RootEntry>,
        blockers: &[Blocker],
        command_hints: &[CommandHint],
    ) -> Self {
        Self {
            wave99_unlock_guard_root: root_for_entry(entries, RootKind::Wave99UnlockGuard),
            custody_execution_bundle_root: root_for_entry(
                entries,
                RootKind::CustodyExecutionBundle,
            ),
            escrow_settlement_accounting_root: root_for_entry(
                entries,
                RootKind::EscrowSettlementAccounting,
            ),
            notary_quorum_root: root_for_entry(entries, RootKind::NotaryQuorum),
            payout_envelope_root: root_for_entry(entries, RootKind::PayoutEnvelope),
            rollback_sentinel_root: root_for_entry(entries, RootKind::RollbackSentinel),
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
            notarized_execution_bundles: false,
            release_execution_allowed: false,
            settlement_allowed: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave99_unlock_guard_root": self.wave99_unlock_guard_root,
            "custody_execution_bundle_root": self.custody_execution_bundle_root,
            "escrow_settlement_accounting_root": self.escrow_settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "signoff_root": self.signoff_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "notarized_execution_bundles": self.notarized_execution_bundles,
            "release_execution_allowed": self.release_execution_allowed,
            "settlement_allowed": self.settlement_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("settlement-decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: BTreeMap<String, RootEntry>,
    pub blockers: Vec<Blocker>,
    pub command_hints: Vec<CommandHint>,
    pub decision: SettlementDecision,
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
        let decision = SettlementDecision::from_parts(&roots, &blockers, &command_hints);
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
        if self.decision.notarized_execution_bundles
            || self.decision.release_execution_allowed
            || self.decision.settlement_allowed
        {
            return Err(
                "settlement notary guard must deny release execution on devnet".to_string(),
            );
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
        "WAVE100-BRIDGE-CUSTODY-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD",
        &[
            HashPart::Str(label),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave100:{hash}")
}

fn list_root(label: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave100:{}", merkle_root(label, &leaves))
}

fn deterministic_root(label: &str) -> String {
    let hash = domain_hash(
        "WAVE100-BRIDGE-CUSTODY-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-DETERMINISTIC",
        &[
            HashPart::Str(DEVNET_CHAIN_ID),
            HashPart::Str(DEVNET_LANE_ID),
            HashPart::Str(label),
        ],
        32,
    );
    format!("root:wave100:{hash}")
}

fn deterministic_field_root(field: &str, value: &str) -> String {
    let hash = domain_hash(
        "WAVE100-BRIDGE-CUSTODY-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-FIELD",
        &[HashPart::Str(field), HashPart::Str(value)],
        32,
    );
    format!("root:wave100:{hash}")
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
