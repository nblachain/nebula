use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave100-live-heavy-gate-release-execution-settlement-notary-guard-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave100";
pub const PRIOR_WAVE_ID: &str = "wave99";
pub const LANE_ID: &str =
    "wave100-live-heavy-gate-release-execution-settlement-notary-guard-compile-lane";
pub const PRIOR_LANE_ID: &str =
    "wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-compile-lane";
pub const DEFAULT_MIN_WAVE99_UNLOCK_GUARD_ROOTS: usize = 7;
pub const DEFAULT_MIN_EXECUTION_BUNDLE_ROOTS: usize = 7;
pub const DEFAULT_MIN_NOTARY_QUORUM_ROOTS: usize = 7;

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
pub enum SettlementVerdict {
    FailClosed,
    ReleaseDenied,
    ExecutionBlocked,
    SettlementBlocked,
    NotaryBlocked,
    SettlementReady,
}

impl SettlementVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReleaseDenied => "release_denied",
            Self::ExecutionBlocked => "execution_blocked",
            Self::SettlementBlocked => "settlement_blocked",
            Self::NotaryBlocked => "notary_blocked",
            Self::SettlementReady => "settlement_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementBlockerKind {
    DefaultFailClosed,
    HeavyGateNotRun,
    ReleaseExecutionDenied,
    Wave99UnlockGuardActive,
    ExecutionBundleMissing,
    ExecutionBundleUnsealed,
    CompileArtifactSettlementMissing,
    SettlementNotNotarized,
    NotaryQuorumMissing,
    PayoutEnvelopeBlocked,
    RollbackSentinelActive,
    CircuitBreakerActive,
    OperatorSignoffMissing,
    ReviewerSignoffMissing,
    RootsOnlyBoundary,
}

impl SettlementBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::ReleaseExecutionDenied => "release_execution_denied",
            Self::Wave99UnlockGuardActive => "wave99_unlock_guard_active",
            Self::ExecutionBundleMissing => "execution_bundle_missing",
            Self::ExecutionBundleUnsealed => "execution_bundle_unsealed",
            Self::CompileArtifactSettlementMissing => "compile_artifact_settlement_missing",
            Self::SettlementNotNotarized => "settlement_not_notarized",
            Self::NotaryQuorumMissing => "notary_quorum_missing",
            Self::PayoutEnvelopeBlocked => "payout_envelope_blocked",
            Self::RollbackSentinelActive => "rollback_sentinel_active",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
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
    pub min_wave99_unlock_guard_roots: usize,
    pub min_execution_bundle_roots: usize,
    pub min_notary_quorum_roots: usize,
    pub roots_only_public_record: bool,
    pub release_execution_allowed: bool,
    pub notarized_execution_bundles: usize,
    pub settlement_blockers_active: bool,
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
            min_wave99_unlock_guard_roots: DEFAULT_MIN_WAVE99_UNLOCK_GUARD_ROOTS,
            min_execution_bundle_roots: DEFAULT_MIN_EXECUTION_BUNDLE_ROOTS,
            min_notary_quorum_roots: DEFAULT_MIN_NOTARY_QUORUM_ROOTS,
            roots_only_public_record: true,
            release_execution_allowed: false,
            notarized_execution_bundles: 0,
            settlement_blockers_active: true,
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
            "min_wave99_unlock_guard_roots": self.min_wave99_unlock_guard_roots,
            "min_execution_bundle_roots": self.min_execution_bundle_roots,
            "min_notary_quorum_roots": self.min_notary_quorum_roots,
            "roots_only_public_record": self.roots_only_public_record,
            "release_execution_allowed": self.release_execution_allowed,
            "notarized_execution_bundles": self.notarized_execution_bundles,
            "settlement_blockers_active": self.settlement_blockers_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave99UnlockGuardRoot {
    pub gate: CompileLaneGate,
    pub unlock_guard_root: String,
    pub finality_certificate_root: String,
    pub accounting_guard_root: String,
    pub command_hint_root: String,
    pub active: bool,
}

impl Wave99UnlockGuardRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            unlock_guard_root: placeholder_root("wave99-unlock-guard", gate.as_str()),
            finality_certificate_root: placeholder_root(
                "wave99-finality-certificate",
                gate.as_str(),
            ),
            accounting_guard_root: placeholder_root("wave99-accounting-guard", gate.as_str()),
            command_hint_root: placeholder_root("wave99-command-hint", gate.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "unlock_guard_root": self.unlock_guard_root,
            "finality_certificate_root": self.finality_certificate_root,
            "accounting_guard_root": self.accounting_guard_root,
            "command_hint_root": self.command_hint_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave99_unlock_guard_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecutionBundleRoot {
    pub gate: CompileLaneGate,
    pub execution_bundle_root: String,
    pub release_execution_root: String,
    pub receipt_root: String,
    pub notary_request_root: String,
    pub sealed: bool,
}

impl ExecutionBundleRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            execution_bundle_root: placeholder_root("execution-bundle", gate.as_str()),
            release_execution_root: placeholder_root("release-execution", gate.as_str()),
            receipt_root: placeholder_root("execution-receipt", gate.as_str()),
            notary_request_root: placeholder_root("execution-notary-request", gate.as_str()),
            sealed: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "execution_bundle_root": self.execution_bundle_root,
            "release_execution_root": self.release_execution_root,
            "receipt_root": self.receipt_root,
            "notary_request_root": self.notary_request_root,
            "sealed": self.sealed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("execution_bundle_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompileArtifactSettlementRoot {
    pub gate: CompileLaneGate,
    pub artifact_manifest_root: String,
    pub settlement_intent_root: String,
    pub settlement_receipt_root: String,
    pub reconciliation_root: String,
    pub settled: bool,
}

impl CompileArtifactSettlementRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            artifact_manifest_root: placeholder_root("compile-artifact-manifest", gate.as_str()),
            settlement_intent_root: placeholder_root("compile-settlement-intent", gate.as_str()),
            settlement_receipt_root: placeholder_root("compile-settlement-receipt", gate.as_str()),
            reconciliation_root: placeholder_root(
                "compile-settlement-reconciliation",
                gate.as_str(),
            ),
            settled: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "artifact_manifest_root": self.artifact_manifest_root,
            "settlement_intent_root": self.settlement_intent_root,
            "settlement_receipt_root": self.settlement_receipt_root,
            "reconciliation_root": self.reconciliation_root,
            "settled": self.settled,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("compile_artifact_settlement_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NotaryQuorumRoot {
    pub gate: CompileLaneGate,
    pub quorum_root: String,
    pub attestation_root: String,
    pub threshold_root: String,
    pub transcript_root: String,
    pub quorum_met: bool,
}

impl NotaryQuorumRoot {
    pub fn missing(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            quorum_root: placeholder_root("notary-quorum", gate.as_str()),
            attestation_root: placeholder_root("notary-attestation", gate.as_str()),
            threshold_root: placeholder_root("notary-threshold", gate.as_str()),
            transcript_root: placeholder_root("notary-transcript", gate.as_str()),
            quorum_met: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "quorum_root": self.quorum_root,
            "attestation_root": self.attestation_root,
            "threshold_root": self.threshold_root,
            "transcript_root": self.transcript_root,
            "quorum_met": self.quorum_met,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("notary_quorum_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PayoutEnvelopeRoot {
    pub gate: CompileLaneGate,
    pub payout_envelope_root: String,
    pub payout_policy_root: String,
    pub reserve_balance_root: String,
    pub release_limit_root: String,
    pub blocked: bool,
}

impl PayoutEnvelopeRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            payout_envelope_root: placeholder_root("payout-envelope", gate.as_str()),
            payout_policy_root: placeholder_root("payout-policy", gate.as_str()),
            reserve_balance_root: placeholder_root("reserve-balance", gate.as_str()),
            release_limit_root: placeholder_root("release-limit", gate.as_str()),
            blocked: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "payout_envelope_root": self.payout_envelope_root,
            "payout_policy_root": self.payout_policy_root,
            "reserve_balance_root": self.reserve_balance_root,
            "release_limit_root": self.release_limit_root,
            "blocked": self.blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("payout_envelope_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RollbackSentinelRoot {
    pub gate: CompileLaneGate,
    pub rollback_sentinel_root: String,
    pub revert_window_root: String,
    pub fork_choice_root: String,
    pub active: bool,
}

impl RollbackSentinelRoot {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            rollback_sentinel_root: placeholder_root("rollback-sentinel", gate.as_str()),
            revert_window_root: placeholder_root("revert-window", gate.as_str()),
            fork_choice_root: placeholder_root("fork-choice", gate.as_str()),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "revert_window_root": self.revert_window_root,
            "fork_choice_root": self.fork_choice_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rollback_sentinel_root", &self.public_record())
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
    pub release_captain_root: String,
    pub complete: bool,
}

impl SignoffRoot {
    pub fn missing(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            operator_signoff_root: placeholder_root("operator-signoff", gate.as_str()),
            reviewer_signoff_root: placeholder_root("reviewer-signoff", gate.as_str()),
            release_captain_root: placeholder_root("release-captain-signoff", gate.as_str()),
            complete: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "release_captain_root": self.release_captain_root,
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
            command_hint_root: placeholder_root("settlement-command-hint", gate.as_str()),
            capture_root: placeholder_root("settlement-capture-rule", gate.as_str()),
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
pub struct SettlementBlocker {
    pub kind: SettlementBlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl SettlementBlocker {
    pub fn new(kind: SettlementBlockerKind, gate: Option<CompileLaneGate>) -> Self {
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
        record_root("settlement_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementCounters {
    pub wave99_unlock_guard_roots: usize,
    pub execution_bundle_roots: usize,
    pub compile_artifact_settlement_roots: usize,
    pub notary_quorum_roots: usize,
    pub payout_envelope_roots: usize,
    pub rollback_sentinel_roots: usize,
    pub circuit_breaker_roots: usize,
    pub signoff_roots: usize,
    pub command_hint_roots: usize,
    pub active_blockers: usize,
    pub notarized_execution_bundles: usize,
}

impl SettlementCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "wave99_unlock_guard_roots": self.wave99_unlock_guard_roots,
            "execution_bundle_roots": self.execution_bundle_roots,
            "compile_artifact_settlement_roots": self.compile_artifact_settlement_roots,
            "notary_quorum_roots": self.notary_quorum_roots,
            "payout_envelope_roots": self.payout_envelope_roots,
            "rollback_sentinel_roots": self.rollback_sentinel_roots,
            "circuit_breaker_roots": self.circuit_breaker_roots,
            "signoff_roots": self.signoff_roots,
            "command_hint_roots": self.command_hint_roots,
            "active_blockers": self.active_blockers,
            "notarized_execution_bundles": self.notarized_execution_bundles,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("settlement_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave99_unlock_guard_roots: Vec<Wave99UnlockGuardRoot>,
    pub execution_bundle_roots: Vec<ExecutionBundleRoot>,
    pub compile_artifact_settlement_roots: Vec<CompileArtifactSettlementRoot>,
    pub notary_quorum_roots: Vec<NotaryQuorumRoot>,
    pub payout_envelope_roots: Vec<PayoutEnvelopeRoot>,
    pub rollback_sentinel_roots: Vec<RollbackSentinelRoot>,
    pub circuit_breaker_roots: Vec<CircuitBreakerRoot>,
    pub signoff_roots: Vec<SignoffRoot>,
    pub command_hints: Vec<CommandHint>,
    pub blockers: Vec<SettlementBlocker>,
    pub counters: SettlementCounters,
    pub verdict: SettlementVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave99_unlock_guard_roots = CompileLaneGate::all()
            .into_iter()
            .map(Wave99UnlockGuardRoot::active)
            .collect::<Vec<_>>();
        let execution_bundle_roots = CompileLaneGate::all()
            .into_iter()
            .map(ExecutionBundleRoot::blocked)
            .collect::<Vec<_>>();
        let compile_artifact_settlement_roots = CompileLaneGate::all()
            .into_iter()
            .map(CompileArtifactSettlementRoot::blocked)
            .collect::<Vec<_>>();
        let notary_quorum_roots = CompileLaneGate::all()
            .into_iter()
            .map(NotaryQuorumRoot::missing)
            .collect::<Vec<_>>();
        let payout_envelope_roots = CompileLaneGate::all()
            .into_iter()
            .map(PayoutEnvelopeRoot::blocked)
            .collect::<Vec<_>>();
        let rollback_sentinel_roots = CompileLaneGate::all()
            .into_iter()
            .map(RollbackSentinelRoot::active)
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
            SettlementBlocker::new(SettlementBlockerKind::DefaultFailClosed, None),
            SettlementBlocker::new(SettlementBlockerKind::HeavyGateNotRun, None),
            SettlementBlocker::new(SettlementBlockerKind::ReleaseExecutionDenied, None),
            SettlementBlocker::new(SettlementBlockerKind::RootsOnlyBoundary, None),
        ];
        for gate in CompileLaneGate::all() {
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::Wave99UnlockGuardActive,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::ExecutionBundleMissing,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::ExecutionBundleUnsealed,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::CompileArtifactSettlementMissing,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::SettlementNotNotarized,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::NotaryQuorumMissing,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::PayoutEnvelopeBlocked,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::RollbackSentinelActive,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::CircuitBreakerActive,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::OperatorSignoffMissing,
                Some(gate),
            ));
            blockers.push(SettlementBlocker::new(
                SettlementBlockerKind::ReviewerSignoffMissing,
                Some(gate),
            ));
        }
        let counters = SettlementCounters {
            wave99_unlock_guard_roots: wave99_unlock_guard_roots.len(),
            execution_bundle_roots: execution_bundle_roots.len(),
            compile_artifact_settlement_roots: compile_artifact_settlement_roots.len(),
            notary_quorum_roots: notary_quorum_roots.len(),
            payout_envelope_roots: payout_envelope_roots.len(),
            rollback_sentinel_roots: rollback_sentinel_roots.len(),
            circuit_breaker_roots: circuit_breaker_roots.len(),
            signoff_roots: signoff_roots.len(),
            command_hint_roots: command_hints.len(),
            active_blockers: blockers.len(),
            notarized_execution_bundles: config.notarized_execution_bundles,
        };
        Self {
            config,
            wave99_unlock_guard_roots,
            execution_bundle_roots,
            compile_artifact_settlement_roots,
            notary_quorum_roots,
            payout_envelope_roots,
            rollback_sentinel_roots,
            circuit_breaker_roots,
            signoff_roots,
            command_hints,
            blockers,
            counters,
            verdict: SettlementVerdict::ReleaseDenied,
        }
    }

    pub fn public_record(&self) -> Value {
        let wave99_unlock_guard_roots = record_roots(
            "wave99-unlock-guard-roots",
            self.wave99_unlock_guard_roots
                .iter()
                .map(Wave99UnlockGuardRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let execution_bundle_roots = record_roots(
            "execution-bundle-roots",
            self.execution_bundle_roots
                .iter()
                .map(ExecutionBundleRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let compile_artifact_settlement_roots = record_roots(
            "compile-artifact-settlement-roots",
            self.compile_artifact_settlement_roots
                .iter()
                .map(CompileArtifactSettlementRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let notary_quorum_roots = record_roots(
            "notary-quorum-roots",
            self.notary_quorum_roots
                .iter()
                .map(NotaryQuorumRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let payout_envelope_roots = record_roots(
            "payout-envelope-roots",
            self.payout_envelope_roots
                .iter()
                .map(PayoutEnvelopeRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let rollback_sentinel_roots = record_roots(
            "rollback-sentinel-roots",
            self.rollback_sentinel_roots
                .iter()
                .map(RollbackSentinelRoot::state_root)
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
            "settlement-blocker-roots",
            self.blockers
                .iter()
                .map(SettlementBlocker::state_root)
                .collect::<Vec<_>>(),
        );
        let deterministic_roots = record_roots(
            "deterministic-settlement-notary-guard-roots",
            vec![
                wave99_unlock_guard_roots.merkle.clone(),
                execution_bundle_roots.merkle.clone(),
                compile_artifact_settlement_roots.merkle.clone(),
                notary_quorum_roots.merkle.clone(),
                payout_envelope_roots.merkle.clone(),
                rollback_sentinel_roots.merkle.clone(),
                circuit_breaker_roots.merkle.clone(),
                signoff_roots.merkle.clone(),
                command_hint_roots.merkle.clone(),
                blocker_roots.merkle.clone(),
            ],
        );

        json!({
            "config_root": self.config.state_root(),
            "wave99_unlock_guard_root_count": self.wave99_unlock_guard_roots.len(),
            "wave99_unlock_guard_roots": wave99_unlock_guard_roots.items,
            "wave99_unlock_guard_merkle": wave99_unlock_guard_roots.merkle,
            "execution_bundle_root_count": self.execution_bundle_roots.len(),
            "execution_bundle_roots": execution_bundle_roots.items,
            "execution_bundle_merkle": execution_bundle_roots.merkle,
            "compile_artifact_settlement_roots": compile_artifact_settlement_roots.items,
            "compile_artifact_settlement_merkle": compile_artifact_settlement_roots.merkle,
            "notary_quorum_roots": notary_quorum_roots.items,
            "notary_quorum_merkle": notary_quorum_roots.merkle,
            "payout_envelope_roots": payout_envelope_roots.items,
            "payout_envelope_merkle": payout_envelope_roots.merkle,
            "rollback_sentinel_roots": rollback_sentinel_roots.items,
            "rollback_sentinel_merkle": rollback_sentinel_roots.merkle,
            "circuit_breaker_roots": circuit_breaker_roots.items,
            "circuit_breaker_merkle": circuit_breaker_roots.merkle,
            "operator_reviewer_signoff_roots": signoff_roots.items,
            "operator_reviewer_signoff_merkle": signoff_roots.merkle,
            "command_hint_roots": command_hint_roots.items,
            "command_hint_merkle": command_hint_roots.merkle,
            "settlement_blocker_roots": blocker_roots.items,
            "settlement_blocker_merkle": blocker_roots.merkle,
            "deterministic_roots": deterministic_roots.items,
            "deterministic_merkle": deterministic_roots.merkle,
            "counters_root": self.counters.state_root(),
            "notarized_execution_bundles": self.counters.notarized_execution_bundles,
            "active_settlement_blockers": self.blockers.len(),
            "wave99_unlock_guards_active": self.wave99_unlock_guard_roots.iter().filter(|root| root.active).count(),
            "execution_bundles_sealed": self.execution_bundle_roots.iter().filter(|root| root.sealed).count(),
            "compile_artifact_settlements_complete": self.compile_artifact_settlement_roots.iter().filter(|root| root.settled).count(),
            "notary_quorums_met": self.notary_quorum_roots.iter().filter(|root| root.quorum_met).count(),
            "payout_envelopes_blocked": self.payout_envelope_roots.iter().filter(|root| root.blocked).count(),
            "rollback_sentinels_active": self.rollback_sentinel_roots.iter().filter(|root| root.active).count(),
            "circuit_breakers_tripped": self.circuit_breaker_roots.iter().filter(|root| root.tripped).count(),
            "release_execution_allowed": self.config.release_execution_allowed,
            "settlement_blockers_active": self.config.settlement_blockers_active,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "roots_only_public_record": self.config.roots_only_public_record,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE100-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-COMPILE-LANE-STATE",
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
        "WAVE100-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-COMPILE-LANE-RECORD",
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
        "WAVE100-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

const DOMAIN: &str = "WAVE100-RELEASE-EXECUTION-SETTLEMENT-NOTARY-GUARD-COMPILE-LANE";
