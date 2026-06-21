use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave97-live-heavy-gate-release-claim-anti-equivocation-seal-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave97";
pub const PRIOR_WAVE_ID: &str = "wave96";
pub const LANE_ID: &str =
    "wave97-live-heavy-gate-release-claim-anti-equivocation-seal-compile-lane";
pub const PRIOR_LANE_ID: &str =
    "wave96-live-heavy-gate-receipt-release-readiness-quorum-compile-lane";
pub const DEFAULT_MIN_RELEASE_CLAIM_ROOTS: usize = 7;

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
pub enum SealVerdict {
    FailClosed,
    ReleaseDenied,
    AntiEquivocationBlocked,
    Sealed,
}

impl SealVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReleaseDenied => "release_denied",
            Self::AntiEquivocationBlocked => "anti_equivocation_blocked",
            Self::Sealed => "sealed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    DefaultFailClosed,
    NoSealedLane,
    ReleaseDenied,
    HeavyGateNotRun,
    AntiEquivocationActive,
    ReplayGuardActive,
    ForkGuardActive,
    DuplicateGuardActive,
    ClaimRootMissing,
    ClaimIdMissing,
    PriorQuorumUnsealed,
    RootsOnlyBoundary,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::NoSealedLane => "no_sealed_lane",
            Self::ReleaseDenied => "release_denied",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::AntiEquivocationActive => "anti_equivocation_active",
            Self::ReplayGuardActive => "replay_guard_active",
            Self::ForkGuardActive => "fork_guard_active",
            Self::DuplicateGuardActive => "duplicate_guard_active",
            Self::ClaimRootMissing => "claim_root_missing",
            Self::ClaimIdMissing => "claim_id_missing",
            Self::PriorQuorumUnsealed => "prior_quorum_unsealed",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardKind {
    Replay,
    Fork,
    Duplicate,
    ConflictingClaim,
}

impl GuardKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::Fork => "fork",
            Self::Duplicate => "duplicate",
            Self::ConflictingClaim => "conflicting_claim",
        }
    }

    pub fn all() -> [Self; 4] {
        [
            Self::Replay,
            Self::Fork,
            Self::Duplicate,
            Self::ConflictingClaim,
        ]
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
    pub min_release_claim_roots: usize,
    pub roots_only_public_record: bool,
    pub anti_equivocation_required: bool,
    pub release_allowed: bool,
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
            min_release_claim_roots: DEFAULT_MIN_RELEASE_CLAIM_ROOTS,
            roots_only_public_record: true,
            anti_equivocation_required: true,
            release_allowed: false,
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
            "min_release_claim_roots": self.min_release_claim_roots,
            "roots_only_public_record": self.roots_only_public_record,
            "anti_equivocation_required": self.anti_equivocation_required,
            "release_allowed": self.release_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimRoot {
    pub gate: CompileLaneGate,
    pub claim_id_root: String,
    pub claim_body_root: String,
    pub prior_quorum_root: String,
    pub command_hint_root: String,
    pub release_ready: bool,
}

impl ReleaseClaimRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            claim_id_root: placeholder_root("claim-id", gate.as_str()),
            claim_body_root: placeholder_root("claim-body", gate.as_str()),
            prior_quorum_root: placeholder_root("prior-quorum", gate.as_str()),
            command_hint_root: placeholder_root("claim-command", gate.as_str()),
            release_ready: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "claim_id_root": self.claim_id_root,
            "claim_body_root": self.claim_body_root,
            "prior_quorum_root": self.prior_quorum_root,
            "command_hint_root": self.command_hint_root,
            "release_ready": self.release_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AntiEquivocationBlocker {
    pub kind: BlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl AntiEquivocationBlocker {
    pub fn new(kind: BlockerKind, gate: Option<CompileLaneGate>) -> Self {
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
        record_root("anti_equivocation_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimGuard {
    pub kind: GuardKind,
    pub gate: CompileLaneGate,
    pub guard_root: String,
    pub active: bool,
}

impl ClaimGuard {
    pub fn active(kind: GuardKind, gate: CompileLaneGate) -> Self {
        let label = format!("{}:{}", kind.as_str(), gate.as_str());
        Self {
            kind,
            gate,
            guard_root: placeholder_root("claim-guard", &label),
            active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "gate": self.gate.as_str(),
            "guard_root": self.guard_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("claim_guard", &self.public_record())
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
            command_hint_root: placeholder_root("command-hint", gate.as_str()),
            capture_root: placeholder_root("capture-rule", gate.as_str()),
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
pub struct SealCounters {
    pub release_claim_roots: usize,
    pub claim_id_roots: usize,
    pub active_blockers: usize,
    pub active_guards: usize,
    pub sealed_lanes: usize,
}

impl SealCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "release_claim_roots": self.release_claim_roots,
            "claim_id_roots": self.claim_id_roots,
            "active_blockers": self.active_blockers,
            "active_guards": self.active_guards,
            "sealed_lanes": self.sealed_lanes,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("seal_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claim_roots: Vec<ReleaseClaimRoot>,
    pub anti_equivocation_blockers: Vec<AntiEquivocationBlocker>,
    pub claim_guards: Vec<ClaimGuard>,
    pub command_hints: Vec<CommandHint>,
    pub counters: SealCounters,
    pub verdict: SealVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let release_claim_roots = CompileLaneGate::all()
            .into_iter()
            .map(ReleaseClaimRoot::blocked)
            .collect::<Vec<_>>();
        let mut anti_equivocation_blockers = vec![
            AntiEquivocationBlocker::new(BlockerKind::DefaultFailClosed, None),
            AntiEquivocationBlocker::new(BlockerKind::NoSealedLane, None),
            AntiEquivocationBlocker::new(BlockerKind::ReleaseDenied, None),
            AntiEquivocationBlocker::new(BlockerKind::HeavyGateNotRun, None),
            AntiEquivocationBlocker::new(BlockerKind::AntiEquivocationActive, None),
            AntiEquivocationBlocker::new(BlockerKind::ReplayGuardActive, None),
            AntiEquivocationBlocker::new(BlockerKind::ForkGuardActive, None),
            AntiEquivocationBlocker::new(BlockerKind::DuplicateGuardActive, None),
            AntiEquivocationBlocker::new(BlockerKind::PriorQuorumUnsealed, None),
            AntiEquivocationBlocker::new(BlockerKind::RootsOnlyBoundary, None),
        ];
        for gate in CompileLaneGate::all() {
            anti_equivocation_blockers.push(AntiEquivocationBlocker::new(
                BlockerKind::ClaimRootMissing,
                Some(gate),
            ));
            anti_equivocation_blockers.push(AntiEquivocationBlocker::new(
                BlockerKind::ClaimIdMissing,
                Some(gate),
            ));
        }
        let claim_guards = CompileLaneGate::all()
            .into_iter()
            .flat_map(|gate| {
                GuardKind::all()
                    .into_iter()
                    .map(move |kind| ClaimGuard::active(kind, gate))
            })
            .collect::<Vec<_>>();
        let command_hints = CompileLaneGate::all()
            .into_iter()
            .map(CommandHint::for_gate)
            .collect::<Vec<_>>();
        let counters = SealCounters {
            release_claim_roots: release_claim_roots.len(),
            claim_id_roots: release_claim_roots.len(),
            active_blockers: anti_equivocation_blockers.len(),
            active_guards: claim_guards.len(),
            sealed_lanes: 0,
        };
        Self {
            config,
            release_claim_roots,
            anti_equivocation_blockers,
            claim_guards,
            command_hints,
            counters,
            verdict: SealVerdict::ReleaseDenied,
        }
    }

    pub fn public_record(&self) -> Value {
        let release_claim_record_roots = record_roots(
            "release-claim-record-roots",
            self.release_claim_roots
                .iter()
                .map(ReleaseClaimRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let claim_id_roots = record_roots(
            "claim-id-roots",
            self.release_claim_roots
                .iter()
                .map(|claim| claim.claim_id_root.clone())
                .collect::<Vec<_>>(),
        );
        let blocker_roots = record_roots(
            "anti-equivocation-blocker-roots",
            self.anti_equivocation_blockers
                .iter()
                .map(AntiEquivocationBlocker::state_root)
                .collect::<Vec<_>>(),
        );
        let guard_roots = record_roots(
            "claim-guard-roots",
            self.claim_guards
                .iter()
                .map(ClaimGuard::state_root)
                .collect::<Vec<_>>(),
        );
        let command_hint_roots = record_roots(
            "command-hint-roots",
            self.command_hints
                .iter()
                .map(CommandHint::state_root)
                .collect::<Vec<_>>(),
        );

        json!({
            "config_root": self.config.state_root(),
            "release_claim_root_count": self.release_claim_roots.len(),
            "release_claim_roots": release_claim_record_roots.items,
            "release_claim_merkle": release_claim_record_roots.merkle,
            "claim_id_roots": claim_id_roots.items,
            "claim_id_merkle": claim_id_roots.merkle,
            "anti_equivocation_blocker_count": self.anti_equivocation_blockers.len(),
            "anti_equivocation_blocker_roots": blocker_roots.items,
            "anti_equivocation_blocker_merkle": blocker_roots.merkle,
            "claim_guard_count": self.claim_guards.len(),
            "claim_guard_roots": guard_roots.items,
            "claim_guard_merkle": guard_roots.merkle,
            "command_hint_roots": command_hint_roots.items,
            "command_hint_merkle": command_hint_roots.merkle,
            "counters_root": self.counters.state_root(),
            "sealed_lanes": self.counters.sealed_lanes,
            "release_allowed": self.config.release_allowed,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "anti_equivocation_required": self.config.anti_equivocation_required,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE97-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL-COMPILE-LANE-STATE",
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
        "WAVE97-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL-COMPILE-LANE-RECORD",
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
        "WAVE97-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

const DOMAIN: &str = "WAVE97-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL-COMPILE-LANE";
