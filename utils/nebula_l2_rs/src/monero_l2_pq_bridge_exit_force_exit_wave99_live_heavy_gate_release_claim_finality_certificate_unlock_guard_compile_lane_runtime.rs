use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave99";
pub const PRIOR_WAVE_ID: &str = "wave98";
pub const LANE_ID: &str =
    "wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-compile-lane";
pub const PRIOR_LANE_ID: &str =
    "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-compile-lane";
pub const DEFAULT_MIN_WAVE98_HOLDOFF_ROOTS: usize = 7;
pub const DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS: usize = 7;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompileLaneGate {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl CompileLaneGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::Clippy => "clippy",
            Self::Rustfmt => "rustfmt",
            Self::Rustc => "rustc",
            Self::BuildMetadata => "build_metadata",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> [Self; 7] {
        [
            Self::CargoCheck,
            Self::CargoTest,
            Self::Clippy,
            Self::Rustfmt,
            Self::Rustc,
            Self::BuildMetadata,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockVerdict {
    FailClosed,
    ReleaseDenied,
    FinalityBlocked,
    UnlockBlocked,
    UnlockReady,
}

impl UnlockVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReleaseDenied => "release_denied",
            Self::FinalityBlocked => "finality_blocked",
            Self::UnlockBlocked => "unlock_blocked",
            Self::UnlockReady => "unlock_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockBlockerKind {
    DefaultFailClosed,
    HeavyGateNotRun,
    ReleaseDenied,
    Wave98HoldoffActive,
    FinalityCertificateMissing,
    FinalityCertificateUnsealed,
    EscrowUnbalanced,
    AccountingUnreconciled,
    RollbackGuardActive,
    CircuitBreakerActive,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    UnlockCertificateMissing,
    RootsOnlyBoundary,
}

impl UnlockBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::ReleaseDenied => "release_denied",
            Self::Wave98HoldoffActive => "wave98_holdoff_active",
            Self::FinalityCertificateMissing => "finality_certificate_missing",
            Self::FinalityCertificateUnsealed => "finality_certificate_unsealed",
            Self::EscrowUnbalanced => "escrow_unbalanced",
            Self::AccountingUnreconciled => "accounting_unreconciled",
            Self::RollbackGuardActive => "rollback_guard_active",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::UnlockCertificateMissing => "unlock_certificate_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_id: String,
    pub prior_wave_id: String,
    pub lane_id: String,
    pub prior_lane_id: String,
    pub min_wave98_holdoff_roots: usize,
    pub min_finality_certificate_roots: usize,
    pub roots_only_public_record: bool,
    pub release_allowed: bool,
    pub unlock_certificates: usize,
    pub heavy_gates_ran: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            prior_lane_id: PRIOR_LANE_ID.to_string(),
            min_wave98_holdoff_roots: DEFAULT_MIN_WAVE98_HOLDOFF_ROOTS,
            min_finality_certificate_roots: DEFAULT_MIN_FINALITY_CERTIFICATE_ROOTS,
            roots_only_public_record: true,
            release_allowed: false,
            unlock_certificates: 0,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_id": self.wave_id,
            "prior_wave_id": self.prior_wave_id,
            "lane_id": self.lane_id,
            "prior_lane_id": self.prior_lane_id,
            "min_wave98_holdoff_roots": self.min_wave98_holdoff_roots,
            "min_finality_certificate_roots": self.min_finality_certificate_roots,
            "roots_only_public_record": self.roots_only_public_record,
            "release_allowed": self.release_allowed,
            "unlock_certificates": self.unlock_certificates,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave98HoldoffRoot {
    pub gate: CompileLaneGate,
    pub holdoff_ledger_root: String,
    pub challenge_window_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub holdoff_active: bool,
}

impl Wave98HoldoffRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            holdoff_ledger_root: placeholder_root("wave98-holdoff-ledger", gate.as_str()),
            challenge_window_root: placeholder_root("wave98-challenge-window", gate.as_str()),
            blocker_root: placeholder_root("wave98-holdoff-blocker", gate.as_str()),
            command_hint_root: placeholder_root("wave98-command-hint", gate.as_str()),
            holdoff_active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "holdoff_ledger_root": self.holdoff_ledger_root,
            "challenge_window_root": self.challenge_window_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "holdoff_active": self.holdoff_active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave98_holdoff_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FinalityCertificateRoot {
    pub gate: CompileLaneGate,
    pub certificate_root: String,
    pub witness_set_root: String,
    pub finality_height_root: String,
    pub unlock_guard_root: String,
    pub sealed: bool,
}

impl FinalityCertificateRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            certificate_root: placeholder_root("finality-certificate", gate.as_str()),
            witness_set_root: placeholder_root("finality-witness-set", gate.as_str()),
            finality_height_root: placeholder_root("finality-height", gate.as_str()),
            unlock_guard_root: placeholder_root("finality-unlock-guard", gate.as_str()),
            sealed: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "certificate_root": self.certificate_root,
            "witness_set_root": self.witness_set_root,
            "finality_height_root": self.finality_height_root,
            "unlock_guard_root": self.unlock_guard_root,
            "sealed": self.sealed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("finality_certificate_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EscrowAccountingRoot {
    pub gate: CompileLaneGate,
    pub escrow_root: String,
    pub accounting_root: String,
    pub liability_root: String,
    pub reconciliation_root: String,
    pub balanced: bool,
}

impl EscrowAccountingRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            escrow_root: placeholder_root("escrow", gate.as_str()),
            accounting_root: placeholder_root("accounting", gate.as_str()),
            liability_root: placeholder_root("liability", gate.as_str()),
            reconciliation_root: placeholder_root("reconciliation", gate.as_str()),
            balanced: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "escrow_root": self.escrow_root,
            "accounting_root": self.accounting_root,
            "liability_root": self.liability_root,
            "reconciliation_root": self.reconciliation_root,
            "balanced": self.balanced,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("escrow_accounting_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RollbackGuardRoot {
    pub gate: CompileLaneGate,
    pub rollback_guard_root: String,
    pub fork_choice_root: String,
    pub revert_window_root: String,
    pub active: bool,
}

impl RollbackGuardRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            rollback_guard_root: placeholder_root("rollback-guard", gate.as_str()),
            fork_choice_root: placeholder_root("fork-choice", gate.as_str()),
            revert_window_root: placeholder_root("revert-window", gate.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "rollback_guard_root": self.rollback_guard_root,
            "fork_choice_root": self.fork_choice_root,
            "revert_window_root": self.revert_window_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rollback_guard_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitBreakerRoot {
    pub gate: CompileLaneGate,
    pub breaker_root: String,
    pub threshold_root: String,
    pub recovery_root: String,
    pub tripped: bool,
}

impl CircuitBreakerRoot {
    pub fn tripped(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            breaker_root: placeholder_root("circuit-breaker", gate.as_str()),
            threshold_root: placeholder_root("circuit-breaker-threshold", gate.as_str()),
            recovery_root: placeholder_root("circuit-breaker-recovery", gate.as_str()),
            tripped: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "breaker_root": self.breaker_root,
            "threshold_root": self.threshold_root,
            "recovery_root": self.recovery_root,
            "tripped": self.tripped,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("circuit_breaker_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignoffRoot {
    pub gate: CompileLaneGate,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub quorum_root: String,
    pub complete: bool,
}

impl SignoffRoot {
    pub fn missing(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            operator_signoff_root: placeholder_root("operator-signoff", gate.as_str()),
            reviewer_signoff_root: placeholder_root("reviewer-signoff", gate.as_str()),
            quorum_root: placeholder_root("signoff-quorum", gate.as_str()),
            complete: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "quorum_root": self.quorum_root,
            "complete": self.complete,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("signoff_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub gate: CompileLaneGate,
    pub command_family: String,
    pub command_hint_root: String,
    pub capture_root: String,
}

impl CommandHint {
    pub fn for_gate(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            command_family: gate.as_str().to_string(),
            command_hint_root: placeholder_root("unlock-command-hint", gate.as_str()),
            capture_root: placeholder_root("unlock-capture-rule", gate.as_str()),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "command_family": self.command_family,
            "command_hint_root": self.command_hint_root,
            "capture_root": self.capture_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockBlocker {
    pub kind: UnlockBlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl UnlockBlocker {
    pub fn new(kind: UnlockBlockerKind, gate: Option<CompileLaneGate>) -> Self {
        let label = match gate {
            Some(value) => value.as_str(),
            None => "compile_lane",
        };
        Self {
            kind,
            gate,
            blocker_root: placeholder_root(kind.as_str(), label),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "gate": self.gate.map(CompileLaneGate::as_str),
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnlockCounters {
    pub wave98_holdoff_roots: usize,
    pub finality_certificate_roots: usize,
    pub escrow_accounting_roots: usize,
    pub rollback_guard_roots: usize,
    pub circuit_breaker_roots: usize,
    pub signoff_roots: usize,
    pub command_hint_roots: usize,
    pub active_blockers: usize,
    pub unlock_certificates: usize,
}

impl UnlockCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "wave98_holdoff_roots": self.wave98_holdoff_roots,
            "finality_certificate_roots": self.finality_certificate_roots,
            "escrow_accounting_roots": self.escrow_accounting_roots,
            "rollback_guard_roots": self.rollback_guard_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "signoff_roots": self.signoff_roots,
            "command_hint_roots": self.command_hint_roots,
            "active_blockers": self.active_blockers,
            "unlock_certificates": self.unlock_certificates,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("unlock_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave98_holdoff_roots: Vec<Wave98HoldoffRoot>,
    pub finality_certificate_roots: Vec<FinalityCertificateRoot>,
    pub escrow_accounting_roots: Vec<EscrowAccountingRoot>,
    pub rollback_guard_roots: Vec<RollbackGuardRoot>,
    pub circuit_breaker_roots: Vec<CircuitBreakerRoot>,
    pub signoff_roots: Vec<SignoffRoot>,
    pub command_hints: Vec<CommandHint>,
    pub blockers: Vec<UnlockBlocker>,
    pub counters: UnlockCounters,
    pub verdict: UnlockVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave98_holdoff_roots = CompileLaneGate::all()
            .into_iter()
            .map(Wave98HoldoffRoot::active)
            .collect::<Vec<_>>();
        let finality_certificate_roots = CompileLaneGate::all()
            .into_iter()
            .map(FinalityCertificateRoot::blocked)
            .collect::<Vec<_>>();
        let escrow_accounting_roots = CompileLaneGate::all()
            .into_iter()
            .map(EscrowAccountingRoot::blocked)
            .collect::<Vec<_>>();
        let rollback_guard_roots = CompileLaneGate::all()
            .into_iter()
            .map(RollbackGuardRoot::active)
            .collect::<Vec<_>>();
        let circuit_breaker_roots = CompileLaneGate::all()
            .into_iter()
            .map(CircuitBreakerRoot::tripped)
            .collect::<Vec<_>>();
        let signoff_roots = CompileLaneGate::all()
            .into_iter()
            .map(SignoffRoot::missing)
            .collect::<Vec<_>>();
        let command_hints = CompileLaneGate::all()
            .into_iter()
            .map(CommandHint::for_gate)
            .collect::<Vec<_>>();
        let mut blockers = vec![
            UnlockBlocker::new(UnlockBlockerKind::DefaultFailClosed, None),
            UnlockBlocker::new(UnlockBlockerKind::HeavyGateNotRun, None),
            UnlockBlocker::new(UnlockBlockerKind::ReleaseDenied, None),
            UnlockBlocker::new(UnlockBlockerKind::UnlockCertificateMissing, None),
            UnlockBlocker::new(UnlockBlockerKind::RootsOnlyBoundary, None),
        ];
        for gate in CompileLaneGate::all() {
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::Wave98HoldoffActive,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::FinalityCertificateMissing,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::FinalityCertificateUnsealed,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::EscrowUnbalanced,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::AccountingUnreconciled,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::RollbackGuardActive,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::CircuitBreakerActive,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::OperatorSignoffMissing,
                Some(gate),
            ));
            blockers.push(UnlockBlocker::new(
                UnlockBlockerKind::ReviewerSignoffMissing,
                Some(gate),
            ));
        }
        let counters = UnlockCounters {
            wave98_holdoff_roots: wave98_holdoff_roots.len(),
            finality_certificate_roots: finality_certificate_roots.len(),
            escrow_accounting_roots: escrow_accounting_roots.len(),
            rollback_guard_roots: rollback_guard_roots.len(),
            circuit_breaker_roots: circuit_breaker_roots.len(),
            signoff_roots: signoff_roots.len(),
            command_hint_roots: command_hints.len(),
            active_blockers: blockers.len(),
            unlock_certificates: config.unlock_certificates,
        };
        Self {
            config,
            wave98_holdoff_roots,
            finality_certificate_roots,
            escrow_accounting_roots,
            rollback_guard_roots,
            circuit_breaker_roots,
            signoff_roots,
            command_hints,
            blockers,
            counters,
            verdict: UnlockVerdict::ReleaseDenied,
        }
    }

    pub fn public_record(&self) -> Value {
        let wave98_holdoff_roots = record_roots(
            "wave98-holdoff-roots",
            self.wave98_holdoff_roots
                .iter()
                .map(Wave98HoldoffRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let finality_certificate_roots = record_roots(
            "finality-certificate-roots",
            self.finality_certificate_roots
                .iter()
                .map(FinalityCertificateRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let escrow_roots = record_roots(
            "escrow-roots",
            self.escrow_accounting_roots
                .iter()
                .map(|entry| entry.escrow_root.clone())
                .collect::<Vec<_>>(),
        );
        let accounting_roots = record_roots(
            "accounting-roots",
            self.escrow_accounting_roots
                .iter()
                .map(|entry| entry.accounting_root.clone())
                .collect::<Vec<_>>(),
        );
        let rollback_guard_roots = record_roots(
            "rollback-guard-roots",
            self.rollback_guard_roots
                .iter()
                .map(RollbackGuardRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let circuit_breaker_roots = record_roots(
            "circuit-breaker-roots",
            self.circuit_breaker_roots
                .iter()
                .map(CircuitBreakerRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let signoff_roots = record_roots(
            "operator-reviewer-signoff-roots",
            self.signoff_roots
                .iter()
                .map(SignoffRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let command_hint_roots = record_roots(
            "command-hint-roots",
            self.command_hints
                .iter()
                .map(CommandHint::state_root)
                .collect::<Vec<_>>(),
        );
        let blocker_roots = record_roots(
            "unlock-blocker-roots",
            self.blockers
                .iter()
                .map(UnlockBlocker::state_root)
                .collect::<Vec<_>>(),
        );
        let deterministic_roots = record_roots(
            "deterministic-unlock-guard-roots",
            vec![
                wave98_holdoff_roots.merkle.clone(),
                finality_certificate_roots.merkle.clone(),
                escrow_roots.merkle.clone(),
                accounting_roots.merkle.clone(),
                rollback_guard_roots.merkle.clone(),
                circuit_breaker_roots.merkle.clone(),
                signoff_roots.merkle.clone(),
                command_hint_roots.merkle.clone(),
                blocker_roots.merkle.clone(),
            ],
        );

        json!({
            "config_root": self.config.state_root(),
            "wave98_holdoff_root_count": self.wave98_holdoff_roots.len(),
            "wave98_holdoff_roots": wave98_holdoff_roots.items,
            "wave98_holdoff_merkle": wave98_holdoff_roots.merkle,
            "finality_certificate_root_count": self.finality_certificate_roots.len(),
            "finality_certificate_roots": finality_certificate_roots.items,
            "finality_certificate_merkle": finality_certificate_roots.merkle,
            "escrow_roots": escrow_roots.items,
            "escrow_merkle": escrow_roots.merkle,
            "accounting_roots": accounting_roots.items,
            "accounting_merkle": accounting_roots.merkle,
            "rollback_guard_roots": rollback_guard_roots.items,
            "rollback_guard_merkle": rollback_guard_roots.merkle,
            "circuit_breaker_roots": circuit_breaker_roots.items,
            "circuit_breaker_merkle": circuit_breaker_roots.merkle,
            "operator_reviewer_signoff_roots": signoff_roots.items,
            "operator_reviewer_signoff_merkle": signoff_roots.merkle,
            "command_hint_roots": command_hint_roots.items,
            "command_hint_merkle": command_hint_roots.merkle,
            "unlock_blocker_roots": blocker_roots.items,
            "unlock_blocker_merkle": blocker_roots.merkle,
            "deterministic_roots": deterministic_roots.items,
            "deterministic_merkle": deterministic_roots.merkle,
            "counters_root": self.counters.state_root(),
            "unlock_certificates": self.counters.unlock_certificates,
            "active_finality_blockers": self.blockers.len(),
            "holdoff_roots_active": self.wave98_holdoff_roots.iter().filter(|root| root.holdoff_active).count(),
            "rollback_guards_active": self.rollback_guard_roots.iter().filter(|root| root.active).count(),
            "circuit_breakers_tripped": self.circuit_breaker_roots.iter().filter(|root| root.tripped).count(),
            "release_allowed": self.config.release_allowed,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "roots_only_public_record": self.config.roots_only_public_record,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE99-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-COMPILE-LANE-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

struct RootList {
    items: Vec<String>,
    merkle: String,
}

fn record_roots(kind: &str, roots: Vec<String>) -> RootList {
    let leaves = roots
        .iter()
        .map(|root| Value::String(domain_hash(kind, &[HashPart::Str(root)], 32)))
        .collect::<Vec<_>>();
    RootList {
        items: roots,
        merkle: merkle_root(&format!("{DOMAIN}:{kind}"), &leaves),
    }
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "WAVE99-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-COMPILE-LANE-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn placeholder_root(kind: &str, label: &str) -> String {
    domain_hash(
        "WAVE99-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

const DOMAIN: &str = "WAVE99-RELEASE-CLAIM-FINALITY-CERTIFICATE-UNLOCK-GUARD-COMPILE-LANE";
