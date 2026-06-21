use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave96-live-heavy-gate-receipt-release-readiness-quorum-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave96";
pub const PRIOR_WAVE_ID: &str = "wave95";
pub const LANE_ID: &str = "wave96-live-heavy-gate-receipt-release-readiness-quorum-compile-lane";
pub const PROMOTION_SOURCE_LANE_ID: &str =
    "wave95-live-heavy-gate-receipt-slot-promotion-compile-lane";
pub const DEFAULT_MIN_PROMOTED_SLOT_ROOTS: usize = 7;

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
pub enum QuorumVerdict {
    FailClosed,
    QuorumUnmet,
    ReleaseReady,
    ProductionDenied,
}

impl QuorumVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::QuorumUnmet => "quorum_unmet",
            Self::ReleaseReady => "release_ready",
            Self::ProductionDenied => "production_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumBlockerKind {
    DefaultFailClosed,
    NoPromotedSlotRoots,
    MissingCompileGateRoot,
    ReleaseClaimPlaceholderOnly,
    OperatorSignoffMissing,
    HeavyGateNotRunInThisLane,
    ProductionDenied,
    RootsOnlyBoundary,
}

impl QuorumBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::NoPromotedSlotRoots => "no_promoted_slot_roots",
            Self::MissingCompileGateRoot => "missing_compile_gate_root",
            Self::ReleaseClaimPlaceholderOnly => "release_claim_placeholder_only",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::HeavyGateNotRunInThisLane => "heavy_gate_not_run_in_this_lane",
            Self::ProductionDenied => "production_denied",
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
    pub promotion_source_lane_id: String,
    pub min_promoted_slot_roots: usize,
    pub production_allowed: bool,
    pub may_claim_heavy_gate_ran: bool,
    pub roots_only_public_record: bool,
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
            promotion_source_lane_id: PROMOTION_SOURCE_LANE_ID.to_string(),
            min_promoted_slot_roots: DEFAULT_MIN_PROMOTED_SLOT_ROOTS,
            production_allowed: false,
            may_claim_heavy_gate_ran: false,
            roots_only_public_record: true,
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
            "promotion_source_lane_id": self.promotion_source_lane_id,
            "min_promoted_slot_roots": self.min_promoted_slot_roots,
            "production_allowed": self.production_allowed,
            "may_claim_heavy_gate_ran": self.may_claim_heavy_gate_ran,
            "roots_only_public_record": self.roots_only_public_record,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotedSlotRoot {
    pub gate: CompileLaneGate,
    pub source_wave_id: String,
    pub source_lane_id: String,
    pub promotion_root: String,
}

impl PromotedSlotRoot {
    pub fn placeholder(gate: CompileLaneGate) -> Self {
        let root = placeholder_root("future-promoted-slot", gate.as_str());
        Self {
            gate,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: PROMOTION_SOURCE_LANE_ID.to_string(),
            promotion_root: root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "source_wave_id": self.source_wave_id,
            "source_lane_id": self.source_lane_id,
            "promotion_root": self.promotion_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promoted_slot_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimPlaceholder {
    pub gate: CompileLaneGate,
    pub claim_root: String,
    pub release_ready: bool,
}

impl ReleaseClaimPlaceholder {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            claim_root: placeholder_root("release-claim-placeholder", gate.as_str()),
            release_ready: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "claim_root": self.claim_root,
            "release_ready": self.release_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_placeholder", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuorumBlocker {
    pub kind: QuorumBlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl QuorumBlocker {
    pub fn new(kind: QuorumBlockerKind, gate: Option<CompileLaneGate>) -> Self {
        let gate_label = match gate {
            Some(value) => value.as_str(),
            None => "lane",
        };
        Self {
            kind,
            gate,
            blocker_root: placeholder_root(kind.as_str(), gate_label),
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
        record_root("quorum_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub gate: CompileLaneGate,
    pub hint_root: String,
    pub command_family: String,
}

impl CommandHint {
    pub fn for_gate(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            hint_root: placeholder_root("command-hint", gate.as_str()),
            command_family: gate.as_str().to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "hint_root": self.hint_root,
            "command_family": self.command_family,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promoted_slot_roots: Vec<PromotedSlotRoot>,
    pub release_claim_placeholders: Vec<ReleaseClaimPlaceholder>,
    pub quorum_blockers: Vec<QuorumBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub release_ready_lanes: usize,
    pub verdict: QuorumVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let promoted_slot_roots = Vec::new();
        let release_claim_placeholders = CompileLaneGate::all()
            .into_iter()
            .map(ReleaseClaimPlaceholder::blocked)
            .collect();
        let command_hints = CompileLaneGate::all()
            .into_iter()
            .map(CommandHint::for_gate)
            .collect();
        let mut quorum_blockers = vec![
            QuorumBlocker::new(QuorumBlockerKind::DefaultFailClosed, None),
            QuorumBlocker::new(QuorumBlockerKind::NoPromotedSlotRoots, None),
            QuorumBlocker::new(QuorumBlockerKind::ReleaseClaimPlaceholderOnly, None),
            QuorumBlocker::new(QuorumBlockerKind::HeavyGateNotRunInThisLane, None),
            QuorumBlocker::new(QuorumBlockerKind::ProductionDenied, None),
            QuorumBlocker::new(QuorumBlockerKind::RootsOnlyBoundary, None),
        ];
        quorum_blockers.push(QuorumBlocker::new(
            QuorumBlockerKind::OperatorSignoffMissing,
            Some(CompileLaneGate::OperatorSignoff),
        ));
        for gate in CompileLaneGate::all() {
            quorum_blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::MissingCompileGateRoot,
                Some(gate),
            ));
        }
        Self {
            config,
            promoted_slot_roots,
            release_claim_placeholders,
            quorum_blockers,
            command_hints,
            release_ready_lanes: 0,
            verdict: QuorumVerdict::ProductionDenied,
        }
    }

    pub fn public_record(&self) -> Value {
        let promoted_slot_roots = self
            .promoted_slot_roots
            .iter()
            .map(PromotedSlotRoot::state_root)
            .collect::<Vec<_>>();
        let release_claim_roots = self
            .release_claim_placeholders
            .iter()
            .map(ReleaseClaimPlaceholder::state_root)
            .collect::<Vec<_>>();
        let blocker_roots = self
            .quorum_blockers
            .iter()
            .map(QuorumBlocker::state_root)
            .collect::<Vec<_>>();
        let command_hint_roots = self
            .command_hints
            .iter()
            .map(CommandHint::state_root)
            .collect::<Vec<_>>();

        json!({
            "config_root": self.config.state_root(),
            "promoted_slot_roots": promoted_slot_roots,
            "promoted_slot_root_count": self.promoted_slot_roots.len(),
            "promoted_slot_root_merkle": string_root("promoted-slot-roots", &promoted_slot_roots),
            "release_claim_placeholder_roots": release_claim_roots,
            "release_claim_placeholder_merkle": string_root("release-claim-placeholders", &release_claim_roots),
            "quorum_blocker_roots": blocker_roots,
            "quorum_blocker_merkle": string_root("quorum-blockers", &blocker_roots),
            "command_hint_roots": command_hint_roots,
            "command_hint_merkle": string_root("command-hints", &command_hint_roots),
            "release_ready_lanes": self.release_ready_lanes,
            "quorum_met": false,
            "production_allowed": self.config.production_allowed,
            "may_claim_heavy_gate_ran": self.config.may_claim_heavy_gate_ran,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
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

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "WAVE96-RELEASE-READINESS-QUORUM-COMPILE-LANE-RECORD",
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
        "WAVE96-RELEASE-READINESS-QUORUM-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

fn string_root(kind: &str, roots: &[String]) -> String {
    let parts = roots
        .iter()
        .map(|root| domain_hash(kind, &[HashPart::Str(root)], 32))
        .collect::<Vec<_>>();
    merkle_root(&parts)
}
