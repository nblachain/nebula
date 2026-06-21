use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave95";
pub const PRIOR_WAVE_ID: &str = "wave94";
pub const SLOT_WAVE_ID: &str = "wave92";
pub const LANE_ID: &str = "wave95-live-heavy-gate-receipt-slot-promotion-compile-lane";
pub const STAGING_SOURCE_LANE_ID: &str = "wave94-live-heavy-gate-receipt-fill-staging-compile-lane";
pub const SLOT_TARGET_LANE_ID: &str = "wave92-live-heavy-gate-receipt-slot-registry-compile-slot";
pub const DEFAULT_MIN_STAGED_FILL_ROOT_CHARS: usize = 16;
pub const DEFAULT_MIN_SLOT_ROOT_CHARS: usize = 16;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: usize = 2;
pub const DEFAULT_MAX_PROMOTION_ATTEMPTS: usize = 32;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompileGate {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl CompileGate {
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionStatus {
    Blocked,
    Promoted,
}

impl PromotionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Promoted => "promoted",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBlockerKind {
    DefaultFailClosed,
    NoStagedFillBinding,
    StagedFillRootTooShort,
    SlotOccupancyRootMissing,
    SlotOccupancyRootTooShort,
    OperatorSignoffQuorumMissing,
    HeavyGateNotRunInThisLane,
    ProductionDenied,
    DuplicatePromotionRoot,
    PromotionCapacityReached,
    RootsOnlyBoundary,
}

impl PromotionBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::NoStagedFillBinding => "no_staged_fill_binding",
            Self::StagedFillRootTooShort => "staged_fill_root_too_short",
            Self::SlotOccupancyRootMissing => "slot_occupancy_root_missing",
            Self::SlotOccupancyRootTooShort => "slot_occupancy_root_too_short",
            Self::OperatorSignoffQuorumMissing => "operator_signoff_quorum_missing",
            Self::HeavyGateNotRunInThisLane => "heavy_gate_not_run_in_this_lane",
            Self::ProductionDenied => "production_denied",
            Self::DuplicatePromotionRoot => "duplicate_promotion_root",
            Self::PromotionCapacityReached => "promotion_capacity_reached",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }

    pub fn all_active() -> [Self; 8] {
        [
            Self::DefaultFailClosed,
            Self::NoStagedFillBinding,
            Self::SlotOccupancyRootMissing,
            Self::OperatorSignoffQuorumMissing,
            Self::HeavyGateNotRunInThisLane,
            Self::ProductionDenied,
            Self::PromotionCapacityReached,
            Self::RootsOnlyBoundary,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    PromotionBlocked,
    ReadyForSlotClaimReview,
    ProductionDenied,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::PromotionBlocked => "promotion_blocked",
            Self::ReadyForSlotClaimReview => "ready_for_slot_claim_review",
            Self::ProductionDenied => "production_denied",
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
    pub slot_wave_id: String,
    pub lane_id: String,
    pub staging_source_lane_id: String,
    pub slot_target_lane_id: String,
    pub min_staged_fill_root_chars: usize,
    pub min_slot_root_chars: usize,
    pub min_operator_signoff_roots: usize,
    pub max_promotion_attempts: usize,
    pub production_allowed: bool,
    pub may_claim_heavy_gate_ran: bool,
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
            slot_wave_id: SLOT_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            staging_source_lane_id: STAGING_SOURCE_LANE_ID.to_string(),
            slot_target_lane_id: SLOT_TARGET_LANE_ID.to_string(),
            min_staged_fill_root_chars: DEFAULT_MIN_STAGED_FILL_ROOT_CHARS,
            min_slot_root_chars: DEFAULT_MIN_SLOT_ROOT_CHARS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            max_promotion_attempts: DEFAULT_MAX_PROMOTION_ATTEMPTS,
            production_allowed: false,
            may_claim_heavy_gate_ran: false,
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
            "slot_wave_id": self.slot_wave_id,
            "lane_id": self.lane_id,
            "staging_source_lane_id": self.staging_source_lane_id,
            "slot_target_lane_id": self.slot_target_lane_id,
            "min_staged_fill_root_chars": self.min_staged_fill_root_chars,
            "min_slot_root_chars": self.min_slot_root_chars,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "max_promotion_attempts": self.max_promotion_attempts,
            "production_allowed": self.production_allowed,
            "may_claim_heavy_gate_ran": self.may_claim_heavy_gate_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StagedFillBinding {
    pub gate: CompileGate,
    pub source_wave_id: String,
    pub source_lane_id: String,
    pub staged_fill_root: String,
    pub staging_verdict_root: String,
}

impl StagedFillBinding {
    pub fn new(
        gate: CompileGate,
        staged_fill_root: impl Into<String>,
        staging_verdict_root: impl Into<String>,
    ) -> Self {
        Self {
            gate,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: STAGING_SOURCE_LANE_ID.to_string(),
            staged_fill_root: staged_fill_root.into(),
            staging_verdict_root: staging_verdict_root.into(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "source_wave_id": self.source_wave_id,
            "source_lane_id": self.source_lane_id,
            "staged_fill_root": self.staged_fill_root,
            "staging_verdict_root": self.staging_verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("staged_fill_binding", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotOccupancyRoot {
    pub gate: CompileGate,
    pub target_wave_id: String,
    pub target_lane_id: String,
    pub occupancy_root: String,
    pub placeholder: bool,
}

impl SlotOccupancyRoot {
    pub fn placeholder(gate: CompileGate) -> Self {
        Self {
            gate,
            target_wave_id: SLOT_WAVE_ID.to_string(),
            target_lane_id: SLOT_TARGET_LANE_ID.to_string(),
            occupancy_root: empty_root("slot_occupancy"),
            placeholder: true,
        }
    }

    pub fn imported(gate: CompileGate, occupancy_root: impl Into<String>) -> Self {
        Self {
            gate,
            target_wave_id: SLOT_WAVE_ID.to_string(),
            target_lane_id: SLOT_TARGET_LANE_ID.to_string(),
            occupancy_root: occupancy_root.into(),
            placeholder: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "target_wave_id": self.target_wave_id,
            "target_lane_id": self.target_lane_id,
            "occupancy_root": self.occupancy_root,
            "placeholder": self.placeholder,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("slot_occupancy_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionBlocker {
    pub gate: CompileGate,
    pub kind: PromotionBlockerKind,
    pub blocker_root: String,
}

impl PromotionBlocker {
    pub fn new(
        gate: CompileGate,
        kind: PromotionBlockerKind,
        blocker_root: impl Into<String>,
    ) -> Self {
        Self {
            gate,
            kind,
            blocker_root: blocker_root.into(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "kind": self.kind.as_str(),
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub gate: CompileGate,
    pub status: PromotionStatus,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_root: String,
    pub promotion_attempt_root: String,
    pub blocker_roots: Vec<String>,
}

impl PromotionAttempt {
    pub fn blocked(gate: CompileGate, blocker_roots: Vec<String>) -> Self {
        let staged_fill_binding_root = empty_root("staged_fill_binding");
        let slot_occupancy_root = empty_root("slot_occupancy");
        let promotion_attempt_root = promotion_attempt_root(
            gate,
            PromotionStatus::Blocked,
            &staged_fill_binding_root,
            &slot_occupancy_root,
            &blocker_roots,
        );
        Self {
            gate,
            status: PromotionStatus::Blocked,
            staged_fill_binding_root,
            slot_occupancy_root,
            promotion_attempt_root,
            blocker_roots,
        }
    }

    pub fn from_roots(
        gate: CompileGate,
        staged_fill_binding_root: impl Into<String>,
        slot_occupancy_root: impl Into<String>,
        blocker_roots: Vec<String>,
    ) -> Self {
        let staged_fill_binding_root = staged_fill_binding_root.into();
        let slot_occupancy_root = slot_occupancy_root.into();
        let status = if blocker_roots.is_empty() {
            PromotionStatus::Promoted
        } else {
            PromotionStatus::Blocked
        };
        let promotion_attempt_root = promotion_attempt_root(
            gate,
            status,
            &staged_fill_binding_root,
            &slot_occupancy_root,
            &blocker_roots,
        );
        Self {
            gate,
            status,
            staged_fill_binding_root,
            slot_occupancy_root,
            promotion_attempt_root,
            blocker_roots,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "status": self.status.as_str(),
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_root": self.slot_occupancy_root,
            "promotion_attempt_root": self.promotion_attempt_root,
            "blocker_roots": self.blocker_roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub gate: CompileGate,
    pub command_id: String,
    pub command_root: String,
    pub capture_rule_root: String,
    pub denial_root: String,
}

impl CommandHint {
    pub fn new(gate: CompileGate, command_id: impl Into<String>, command_text: &str) -> Self {
        Self {
            gate,
            command_id: command_id.into(),
            command_root: record_root(
                "command_hint",
                &json!({
                    "gate": gate.as_str(),
                    "command_text": command_text,
                }),
            ),
            capture_rule_root: record_root(
                "capture_rule",
                &json!({
                    "gate": gate.as_str(),
                    "rule": "seal_status_stdout_stderr_artifact_metadata_and_note_roots_only",
                }),
            ),
            denial_root: record_root(
                "command_denial",
                &json!({
                    "gate": gate.as_str(),
                    "rule": "wave95_records_promotion_roots_only_and_runs_no_live_gate",
                }),
            ),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "command_id": self.command_id,
            "command_root": self.command_root,
            "capture_rule_root": self.capture_rule_root,
            "denial_root": self.denial_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionVerdict {
    pub verdict: RuntimeVerdict,
    pub attempted_count: usize,
    pub promoted_count: usize,
    pub blocked_count: usize,
    pub blocker_root: String,
}

impl PromotionVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "attempted_count": self.attempted_count,
            "promoted_count": self.promoted_count,
            "blocked_count": self.blocked_count,
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_root: String,
    pub promotion_attempt_root: String,
    pub promotion_blocker_root: String,
    pub command_hint_root: String,
    pub operator_signoff_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_root": self.slot_occupancy_root,
            "promotion_attempt_root": self.promotion_attempt_root,
            "promotion_blocker_root": self.promotion_blocker_root,
            "command_hint_root": self.command_hint_root,
            "operator_signoff_root": self.operator_signoff_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub staged_fill_bindings: BTreeMap<CompileGate, Vec<StagedFillBinding>>,
    pub slot_occupancy_roots: BTreeMap<CompileGate, SlotOccupancyRoot>,
    pub promotion_attempts: Vec<PromotionAttempt>,
    pub promotion_blockers: Vec<PromotionBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub operator_signoff_roots: Vec<String>,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut staged_fill_bindings = BTreeMap::new();
        let mut slot_occupancy_roots = BTreeMap::new();
        let mut promotion_attempts = Vec::new();
        let mut promotion_blockers = Vec::new();
        for gate in CompileGate::all() {
            staged_fill_bindings.insert(gate, Vec::new());
            slot_occupancy_roots.insert(gate, SlotOccupancyRoot::placeholder(gate));
            let blockers = default_blockers_for_gate(gate, &config);
            let blocker_roots = blockers
                .iter()
                .map(PromotionBlocker::state_root)
                .collect::<Vec<_>>();
            promotion_attempts.push(PromotionAttempt::blocked(gate, blocker_roots));
            promotion_blockers.extend(blockers);
        }
        Self {
            config,
            staged_fill_bindings,
            slot_occupancy_roots,
            promotion_attempts,
            promotion_blockers,
            command_hints: default_command_hints(),
            operator_signoff_roots: Vec::new(),
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet())
    }

    pub fn import_slot_occupancy_root(
        &mut self,
        gate: CompileGate,
        occupancy_root: impl Into<String>,
    ) -> Result<()> {
        let occupancy_root = occupancy_root.into();
        if occupancy_root.len() < self.config.min_slot_root_chars {
            self.push_blocker(
                gate,
                PromotionBlockerKind::SlotOccupancyRootTooShort,
                record_root(
                    "short_slot_occupancy_root",
                    &json!({ "root": occupancy_root }),
                ),
            );
            return Err("slot occupancy root too short".to_string());
        }
        self.slot_occupancy_roots
            .insert(gate, SlotOccupancyRoot::imported(gate, occupancy_root));
        Ok(())
    }

    pub fn add_operator_signoff_root(&mut self, signoff_root: impl Into<String>) -> Result<()> {
        let signoff_root = signoff_root.into();
        if signoff_root.len() < self.config.min_staged_fill_root_chars {
            return Err("operator signoff root too short".to_string());
        }
        if self.operator_signoff_roots.contains(&signoff_root) {
            return Err("duplicate operator signoff root".to_string());
        }
        self.operator_signoff_roots.push(signoff_root);
        Ok(())
    }

    pub fn submit_staged_fill_binding(&mut self, binding: StagedFillBinding) -> Result<()> {
        if self.promotion_attempts.len() >= self.config.max_promotion_attempts {
            self.push_blocker(
                binding.gate,
                PromotionBlockerKind::PromotionCapacityReached,
                binding.state_root(),
            );
            return Err("promotion capacity reached".to_string());
        }
        if binding.staged_fill_root.len() < self.config.min_staged_fill_root_chars {
            self.push_blocker(
                binding.gate,
                PromotionBlockerKind::StagedFillRootTooShort,
                binding.state_root(),
            );
            return Err("staged fill root too short".to_string());
        }
        if self.binding_exists(&binding.staged_fill_root) {
            self.push_blocker(
                binding.gate,
                PromotionBlockerKind::DuplicatePromotionRoot,
                binding.state_root(),
            );
            return Err("duplicate promotion root".to_string());
        }

        let gate = binding.gate;
        let binding_root = binding.state_root();
        self.staged_fill_bindings
            .entry(gate)
            .or_insert_with(Vec::new)
            .push(binding);
        let slot_root = self.slot_occupancy_state_root(gate);
        let blockers = self.blockers_for_promotion(gate, &binding_root, &slot_root);
        let blocker_roots = blockers
            .iter()
            .map(PromotionBlocker::state_root)
            .collect::<Vec<_>>();
        self.promotion_blockers.extend(blockers);
        self.promotion_attempts.push(PromotionAttempt::from_roots(
            gate,
            binding_root,
            slot_root,
            blocker_roots,
        ));
        Ok(())
    }

    pub fn attempted_count(&self) -> usize {
        self.promotion_attempts.len()
    }

    pub fn promoted_count(&self) -> usize {
        self.promotion_attempts
            .iter()
            .filter(|attempt| attempt.status == PromotionStatus::Promoted)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.promotion_attempts
            .iter()
            .filter(|attempt| attempt.status == PromotionStatus::Blocked)
            .count()
    }

    pub fn verdict(&self) -> PromotionVerdict {
        let blocker_root = list_root(
            "promotion_blockers",
            self.promotion_blockers
                .iter()
                .map(PromotionBlocker::public_record),
        );
        let verdict = if !self.config.production_allowed {
            RuntimeVerdict::ProductionDenied
        } else if self.promoted_count() > 0 && self.blocked_count() == 0 {
            RuntimeVerdict::ReadyForSlotClaimReview
        } else if self.blocked_count() > 0 {
            RuntimeVerdict::PromotionBlocked
        } else {
            RuntimeVerdict::FailClosed
        };
        PromotionVerdict {
            verdict,
            attempted_count: self.attempted_count(),
            promoted_count: self.promoted_count(),
            blocked_count: self.blocked_count(),
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        public_record_from_state(self)
    }

    pub fn state_root(&self) -> String {
        self.public_record().state_root
    }

    fn push_blocker(
        &mut self,
        gate: CompileGate,
        kind: PromotionBlockerKind,
        blocker_root: impl Into<String>,
    ) {
        self.promotion_blockers
            .push(PromotionBlocker::new(gate, kind, blocker_root));
    }

    fn binding_exists(&self, staged_fill_root: &str) -> bool {
        self.staged_fill_bindings.values().any(|bindings| {
            bindings
                .iter()
                .any(|binding| binding.staged_fill_root == staged_fill_root)
        })
    }

    fn slot_occupancy_state_root(&self, gate: CompileGate) -> String {
        self.slot_occupancy_roots
            .get(&gate)
            .map(SlotOccupancyRoot::state_root)
            .map_or_else(|| empty_root("slot_occupancy"), |root| root)
    }

    fn blockers_for_promotion(
        &self,
        gate: CompileGate,
        binding_root: &str,
        slot_root: &str,
    ) -> Vec<PromotionBlocker> {
        let mut blockers = Vec::new();
        if binding_root == empty_root("staged_fill_binding") {
            blockers.push(PromotionBlocker::new(
                gate,
                PromotionBlockerKind::NoStagedFillBinding,
                binding_root,
            ));
        }
        if slot_root == empty_root("slot_occupancy") {
            blockers.push(PromotionBlocker::new(
                gate,
                PromotionBlockerKind::SlotOccupancyRootMissing,
                slot_root,
            ));
        }
        if self.operator_signoff_roots.len() < self.config.min_operator_signoff_roots {
            blockers.push(PromotionBlocker::new(
                gate,
                PromotionBlockerKind::OperatorSignoffQuorumMissing,
                operator_signoff_root(&self.operator_signoff_roots),
            ));
        }
        blockers.push(PromotionBlocker::new(
            gate,
            PromotionBlockerKind::HeavyGateNotRunInThisLane,
            binding_root,
        ));
        if !self.config.production_allowed {
            blockers.push(PromotionBlocker::new(
                gate,
                PromotionBlockerKind::ProductionDenied,
                self.config.state_root(),
            ));
        }
        if !self.config.may_claim_heavy_gate_ran {
            blockers.push(PromotionBlocker::new(
                gate,
                PromotionBlockerKind::RootsOnlyBoundary,
                record_root("roots_only_boundary", &json!({ "gate": gate.as_str() })),
            ));
        }
        dedupe_blockers(blockers)
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

pub fn default_command_hints() -> Vec<CommandHint> {
    vec![
        CommandHint::new(
            CompileGate::CargoCheck,
            "promote_cargo_check_staged_fill_root_to_slot_claim",
            "cargo check --workspace --all-targets",
        ),
        CommandHint::new(
            CompileGate::CargoTest,
            "promote_cargo_test_staged_fill_root_to_slot_claim",
            "cargo test --workspace --all-targets",
        ),
        CommandHint::new(
            CompileGate::Clippy,
            "promote_clippy_staged_fill_root_to_slot_claim",
            "cargo clippy --workspace --all-targets -- -D warnings",
        ),
        CommandHint::new(
            CompileGate::Rustfmt,
            "promote_rustfmt_staged_fill_root_to_slot_claim",
            "cargo fmt --all --check",
        ),
        CommandHint::new(
            CompileGate::Rustc,
            "promote_rustc_staged_fill_root_to_slot_claim",
            "rustc --version --verbose",
        ),
        CommandHint::new(
            CompileGate::BuildMetadata,
            "promote_build_metadata_staged_fill_root_to_slot_claim",
            "cargo metadata --no-deps --format-version 1",
        ),
        CommandHint::new(
            CompileGate::OperatorSignoff,
            "promote_operator_signoff_staged_fill_root_to_slot_claim",
            "operator records independent signoff roots",
        ),
    ]
}

pub fn public_record_from_state(state: &State) -> PublicRecord {
    let config_root = state.config.state_root();
    let staged_fill_binding_root = staged_fill_binding_root(&state.staged_fill_bindings);
    let slot_occupancy_root = list_root(
        "slot_occupancy_roots",
        state
            .slot_occupancy_roots
            .values()
            .map(SlotOccupancyRoot::public_record),
    );
    let promotion_attempt_root = list_root(
        "promotion_attempts",
        state
            .promotion_attempts
            .iter()
            .map(PromotionAttempt::public_record),
    );
    let promotion_blocker_root = list_root(
        "promotion_blockers",
        state
            .promotion_blockers
            .iter()
            .map(PromotionBlocker::public_record),
    );
    let command_hint_root = list_root(
        "command_hints",
        state.command_hints.iter().map(CommandHint::public_record),
    );
    let operator_signoff_root = operator_signoff_root(&state.operator_signoff_roots);
    let verdict_root = state.verdict().state_root();
    let state_root = record_root(
        "state",
        &json!({
            "config_root": config_root,
            "staged_fill_binding_root": staged_fill_binding_root,
            "slot_occupancy_root": slot_occupancy_root,
            "promotion_attempt_root": promotion_attempt_root,
            "promotion_blocker_root": promotion_blocker_root,
            "command_hint_root": command_hint_root,
            "operator_signoff_root": operator_signoff_root,
            "verdict_root": verdict_root,
        }),
    );
    PublicRecord {
        config_root,
        staged_fill_binding_root,
        slot_occupancy_root,
        promotion_attempt_root,
        promotion_blocker_root,
        command_hint_root,
        operator_signoff_root,
        verdict_root,
        state_root,
    }
}

pub fn staged_fill_binding_root(
    bindings: &BTreeMap<CompileGate, Vec<StagedFillBinding>>,
) -> String {
    let leaves = bindings
        .iter()
        .map(|(gate, roots)| {
            json!({
                "gate": gate.as_str(),
                "binding_roots": roots
                    .iter()
                    .map(StagedFillBinding::state_root)
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-WAVE95-RECEIPT-SLOT-PROMOTION-STAGED-FILL-BINDINGS",
        &leaves,
    )
}

pub fn promotion_attempt_root(
    gate: CompileGate,
    status: PromotionStatus,
    staged_fill_binding_root: &str,
    slot_occupancy_root: &str,
    blocker_roots: &[String],
) -> String {
    record_root(
        "promotion_attempt_root",
        &json!({
            "gate": gate.as_str(),
            "status": status.as_str(),
            "staged_fill_binding_root": staged_fill_binding_root,
            "slot_occupancy_root": slot_occupancy_root,
            "blocker_root": string_list_root("promotion_attempt_blockers", blocker_roots),
        }),
    )
}

pub fn operator_signoff_root(values: &[String]) -> String {
    string_list_root("operator_signoff_roots", values)
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE95-RECEIPT-SLOT-PROMOTION-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

pub fn empty_root(kind: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE95-RECEIPT-SLOT-PROMOTION-EMPTY",
        &[HashPart::Str(kind)],
        32,
    )
}

pub fn list_root<I>(kind: &str, records: I) -> String
where
    I: IntoIterator<Item = Value>,
{
    let leaves = records.into_iter().collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE95-RECEIPT-SLOT-PROMOTION-{kind}"),
        &leaves,
    )
}

pub fn string_list_root(kind: &str, values: &[String]) -> String {
    let leaves = values
        .iter()
        .map(|value| json!({ "root": value }))
        .collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE95-RECEIPT-SLOT-PROMOTION-{kind}"),
        &leaves,
    )
}

pub fn default_blockers_for_gate(gate: CompileGate, config: &Config) -> Vec<PromotionBlocker> {
    let blockers = PromotionBlockerKind::all_active()
        .iter()
        .map(|kind| {
            let blocker_root = match kind {
                PromotionBlockerKind::ProductionDenied => config.state_root(),
                PromotionBlockerKind::OperatorSignoffQuorumMissing => {
                    operator_signoff_root(&Vec::new())
                }
                PromotionBlockerKind::SlotOccupancyRootMissing => empty_root("slot_occupancy"),
                PromotionBlockerKind::NoStagedFillBinding => empty_root("staged_fill_binding"),
                _ => record_root(
                    "default_promotion_blocker",
                    &json!({
                        "gate": gate.as_str(),
                        "kind": kind.as_str(),
                    }),
                ),
            };
            PromotionBlocker::new(gate, *kind, blocker_root)
        })
        .collect::<Vec<_>>();
    dedupe_blockers(blockers)
}

pub fn dedupe_blockers(blockers: Vec<PromotionBlocker>) -> Vec<PromotionBlocker> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for blocker in blockers {
        let root = blocker.state_root();
        if seen.insert(root) {
            out.push(blocker);
        }
    }
    out
}
