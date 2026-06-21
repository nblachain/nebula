use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave99-release-claim-finality-certificate-unlock-guard-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE98_HOLDOFF_ROOT: &str =
    "root:wave98:bridge-custody-challenge-window-holdoff-ledger-active-placeholder";
pub const DEFAULT_FINALITY_DEPTH: u64 = 720;
pub const DEFAULT_MIN_CERTIFICATE_ROOTS: u16 = 5;
pub const DEFAULT_MIN_CUSTODY_ESCROW_ROOTS: u16 = 4;
pub const DEFAULT_MIN_ROLLBACK_GUARD_ROOTS: u16 = 4;
pub const DEFAULT_MIN_SIGNOFF_ROOTS: u16 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave98_holdoff_root: String,
    pub finality_depth_blocks: u64,
    pub min_certificate_roots: u16,
    pub min_custody_escrow_roots: u16,
    pub min_rollback_guard_roots: u16,
    pub min_signoff_roots: u16,
    pub roots_only_public_records: bool,
    pub unlock_certificates_enabled: bool,
    pub release_enabled: bool,
    pub custody_blockers_active: bool,
    pub finality_blockers_active: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave98_holdoff_root: WAVE98_HOLDOFF_ROOT.to_string(),
            finality_depth_blocks: DEFAULT_FINALITY_DEPTH,
            min_certificate_roots: DEFAULT_MIN_CERTIFICATE_ROOTS,
            min_custody_escrow_roots: DEFAULT_MIN_CUSTODY_ESCROW_ROOTS,
            min_rollback_guard_roots: DEFAULT_MIN_ROLLBACK_GUARD_ROOTS,
            min_signoff_roots: DEFAULT_MIN_SIGNOFF_ROOTS,
            roots_only_public_records: true,
            unlock_certificates_enabled: false,
            release_enabled: false,
            custody_blockers_active: true,
            finality_blockers_active: true,
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
        ensure_root_like("wave98_holdoff_root", &self.wave98_holdoff_root)?;
        if self.finality_depth_blocks == 0 {
            return Err("finality depth must be nonzero".to_string());
        }
        if self.min_certificate_roots == 0 {
            return Err("certificate root quorum must be nonzero".to_string());
        }
        if self.min_custody_escrow_roots == 0 {
            return Err("custody escrow root quorum must be nonzero".to_string());
        }
        if self.min_rollback_guard_roots == 0 {
            return Err("rollback guard root quorum must be nonzero".to_string());
        }
        if self.min_signoff_roots == 0 {
            return Err("signoff root quorum must be nonzero".to_string());
        }
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.unlock_certificates_enabled || self.release_enabled {
            return Err("devnet unlock and release must remain denied".to_string());
        }
        if !self.custody_blockers_active || !self.finality_blockers_active {
            return Err("devnet custody and finality blockers must remain active".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave99 must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_root": deterministic_field_root("chain", &self.chain_id),
            "lane_root": deterministic_field_root("lane", &self.lane_id),
            "protocol_root": deterministic_field_root("protocol", &self.protocol_version),
            "wave98_holdoff_root": self.wave98_holdoff_root,
            "finality_depth_blocks": self.finality_depth_blocks,
            "min_certificate_roots": self.min_certificate_roots,
            "min_custody_escrow_roots": self.min_custody_escrow_roots,
            "min_rollback_guard_roots": self.min_rollback_guard_roots,
            "min_signoff_roots": self.min_signoff_roots,
            "roots_only_public_records": self.roots_only_public_records,
            "unlock_certificates_enabled": self.unlock_certificates_enabled,
            "release_enabled": self.release_enabled,
            "custody_blockers_active": self.custody_blockers_active,
            "finality_blockers_active": self.finality_blockers_active,
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
    Wave98Holdoff,
    FinalityCertificate,
    CustodyEscrow,
    CustodyAccounting,
    RollbackGuard,
    CircuitBreaker,
    OperatorSignoff,
    ReviewerSignoff,
    CommandHint,
    DeterministicUnlockGuard,
}

impl RootKind {
    pub fn all() -> [Self; 10] {
        [
            Self::Wave98Holdoff,
            Self::FinalityCertificate,
            Self::CustodyEscrow,
            Self::CustodyAccounting,
            Self::RollbackGuard,
            Self::CircuitBreaker,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::CommandHint,
            Self::DeterministicUnlockGuard,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98Holdoff => "wave98_holdoff",
            Self::FinalityCertificate => "finality_certificate",
            Self::CustodyEscrow => "custody_escrow",
            Self::CustodyAccounting => "custody_accounting",
            Self::RollbackGuard => "rollback_guard",
            Self::CircuitBreaker => "circuit_breaker",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::CommandHint => "command_hint",
            Self::DeterministicUnlockGuard => "deterministic_unlock_guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave98HoldoffActive,
    FinalityCertificateMissing,
    FinalityDepthUnproven,
    CustodyEscrowLocked,
    CustodyAccountingUnbalanced,
    RollbackGuardActive,
    CircuitBreakerArmed,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    UnlockCertificateDisabled,
    ReleaseDisabled,
    HeavyGateMissing,
}

impl BlockerKind {
    pub fn all() -> [Self; 12] {
        [
            Self::Wave98HoldoffActive,
            Self::FinalityCertificateMissing,
            Self::FinalityDepthUnproven,
            Self::CustodyEscrowLocked,
            Self::CustodyAccountingUnbalanced,
            Self::RollbackGuardActive,
            Self::CircuitBreakerArmed,
            Self::OperatorSignoffMissing,
            Self::ReviewerSignoffMissing,
            Self::UnlockCertificateDisabled,
            Self::ReleaseDisabled,
            Self::HeavyGateMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave98HoldoffActive => "wave98_holdoff_active",
            Self::FinalityCertificateMissing => "finality_certificate_missing",
            Self::FinalityDepthUnproven => "finality_depth_unproven",
            Self::CustodyEscrowLocked => "custody_escrow_locked",
            Self::CustodyAccountingUnbalanced => "custody_accounting_unbalanced",
            Self::RollbackGuardActive => "rollback_guard_active",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::UnlockCertificateDisabled => "unlock_certificate_disabled",
            Self::ReleaseDisabled => "release_disabled",
            Self::HeavyGateMissing => "heavy_gate_missing",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootEntry {
    pub kind: RootKind,
    pub root: String,
    pub blocker_active: bool,
    pub required_for_unlock: bool,
}

impl RootEntry {
    pub fn devnet(kind: RootKind, config: &Config) -> Self {
        let root = match kind {
            RootKind::Wave98Holdoff => config.wave98_holdoff_root.clone(),
            _ => deterministic_root(kind.as_str()),
        };
        Self {
            kind,
            root,
            blocker_active: kind != RootKind::CommandHint
                && kind != RootKind::DeterministicUnlockGuard,
            required_for_unlock: kind != RootKind::CommandHint,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "root": self.root,
            "blocker_active": self.blocker_active,
            "required_for_unlock": self.required_for_unlock,
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
pub struct UnlockDecision {
    pub finality_certificate_root: String,
    pub custody_escrow_root: String,
    pub custody_accounting_root: String,
    pub rollback_guard_root: String,
    pub circuit_breaker_root: String,
    pub signoff_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub unlock_allowed: bool,
    pub release_allowed: bool,
}

impl UnlockDecision {
    pub fn from_parts(entries: &BTreeMap<String, RootEntry>, blockers: &[Blocker]) -> Self {
        Self {
            finality_certificate_root: root_for_entry(entries, RootKind::FinalityCertificate),
            custody_escrow_root: root_for_entry(entries, RootKind::CustodyEscrow),
            custody_accounting_root: root_for_entry(entries, RootKind::CustodyAccounting),
            rollback_guard_root: root_for_entry(entries, RootKind::RollbackGuard),
            circuit_breaker_root: root_for_entry(entries, RootKind::CircuitBreaker),
            signoff_root: list_root(
                "signoff-roots",
                vec![
                    root_for_entry(entries, RootKind::OperatorSignoff),
                    root_for_entry(entries, RootKind::ReviewerSignoff),
                ],
            ),
            blocker_root: blocker_root(blockers),
            command_hint_root: root_for_entry(entries, RootKind::CommandHint),
            unlock_allowed: false,
            release_allowed: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "finality_certificate_root": self.finality_certificate_root,
            "custody_escrow_root": self.custody_escrow_root,
            "custody_accounting_root": self.custody_accounting_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "signoff_root": self.signoff_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "unlock_allowed": self.unlock_allowed,
            "release_allowed": self.release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock-decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: BTreeMap<String, RootEntry>,
    pub blockers: Vec<Blocker>,
    pub decision: UnlockDecision,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        Self::with_config(config)
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
        let decision = UnlockDecision::from_parts(&roots, &blockers);
        Self {
            config,
            roots,
            blockers,
            decision,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.decision.unlock_allowed || self.decision.release_allowed {
            return Err("unlock guard must deny release on devnet".to_string());
        }
        if self.blockers.iter().any(|blocker| !blocker.active) {
            return Err("devnet blockers must remain active".to_string());
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
        json!({
            "config_root": self.config.root(),
            "root_entry_records": root_records,
            "root_entries_root": self.root_entries_root(),
            "blocker_roots": blocker_roots,
            "blockers_root": self.blockers_root(),
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

    pub fn state_root(&self) -> String {
        record_root(
            "state",
            &json!({
                "config_root": self.config.root(),
                "root_entries_root": self.root_entries_root(),
                "blockers_root": self.blockers_root(),
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

fn record_root(label: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE99-BRIDGE-CUSTODY-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD",
        &[
            HashPart::Str(label),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave99:{hash}")
}

fn list_root(label: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave99:{}", merkle_root(label, &leaves))
}

fn deterministic_root(label: &str) -> String {
    let hash = domain_hash(
        "WAVE99-BRIDGE-CUSTODY-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-DETERMINISTIC",
        &[
            HashPart::Str(DEVNET_CHAIN_ID),
            HashPart::Str(DEVNET_LANE_ID),
            HashPart::Str(label),
        ],
        32,
    );
    format!("root:wave99:{hash}")
}

fn deterministic_field_root(field: &str, value: &str) -> String {
    let hash = domain_hash(
        "WAVE99-BRIDGE-CUSTODY-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-FIELD",
        &[HashPart::Str(field), HashPart::Str(value)],
        32,
    );
    format!("root:wave99:{hash}")
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
