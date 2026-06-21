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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave94-live-heavy-gate-receipt-fill-staging-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave94";
pub const PRIOR_WAVE_ID: &str = "wave93";
pub const SLOT_WAVE_ID: &str = "wave92";
pub const LANE_ID: &str = "wave94-live-heavy-gate-receipt-fill-staging-compile-lane";
pub const ADMISSION_SOURCE_LANE_ID: &str =
    "wave93-live-heavy-gate-receipt-admission-quarantine-compile-lane";
pub const SLOT_TARGET_LANE_ID: &str = "wave92-live-heavy-gate-receipt-slot-registry-compile-slot";
pub const DEFAULT_MIN_ADMITTED_ROOT_CHARS: usize = 16;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: usize = 2;
pub const DEFAULT_MAX_STAGED_FILLS: usize = 32;

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
pub enum FillStatus {
    Empty,
    Staged,
    Blocked,
    Filled,
}

impl FillStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Staged => "staged",
            Self::Blocked => "blocked",
            Self::Filled => "filled",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotFillBlockerKind {
    DefaultFailClosed,
    NoWave93AdmittedRoot,
    AdmittedRootTooShort,
    Wave92SlotRootMissing,
    OperatorSignoffQuorumMissing,
    HeavyGateNotRunInThisLane,
    ProductionDenied,
    DuplicateStagedRoot,
    StagingCapacityReached,
    RootsOnlyBoundary,
}

impl SlotFillBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::NoWave93AdmittedRoot => "no_wave93_admitted_root",
            Self::AdmittedRootTooShort => "admitted_root_too_short",
            Self::Wave92SlotRootMissing => "wave92_slot_root_missing",
            Self::OperatorSignoffQuorumMissing => "operator_signoff_quorum_missing",
            Self::HeavyGateNotRunInThisLane => "heavy_gate_not_run_in_this_lane",
            Self::ProductionDenied => "production_denied",
            Self::DuplicateStagedRoot => "duplicate_staged_root",
            Self::StagingCapacityReached => "staging_capacity_reached",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    StagingOnly,
    ReadyForSlotFillReview,
    ProductionDenied,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::StagingOnly => "staging_only",
            Self::ReadyForSlotFillReview => "ready_for_slot_fill_review",
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
    pub admission_source_lane_id: String,
    pub slot_target_lane_id: String,
    pub min_admitted_root_chars: usize,
    pub min_operator_signoff_roots: usize,
    pub max_staged_fills: usize,
    pub production_allowed: bool,
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
            admission_source_lane_id: ADMISSION_SOURCE_LANE_ID.to_string(),
            slot_target_lane_id: SLOT_TARGET_LANE_ID.to_string(),
            min_admitted_root_chars: DEFAULT_MIN_ADMITTED_ROOT_CHARS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            max_staged_fills: DEFAULT_MAX_STAGED_FILLS,
            production_allowed: false,
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
            "admission_source_lane_id": self.admission_source_lane_id,
            "slot_target_lane_id": self.slot_target_lane_id,
            "min_admitted_root_chars": self.min_admitted_root_chars,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "max_staged_fills": self.max_staged_fills,
            "production_allowed": self.production_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmittedRootBinding {
    pub gate: CompileGate,
    pub source_wave_id: String,
    pub source_lane_id: String,
    pub admitted_root: String,
    pub admission_verdict_root: String,
}

impl AdmittedRootBinding {
    pub fn new(
        gate: CompileGate,
        admitted_root: impl Into<String>,
        admission_verdict_root: impl Into<String>,
    ) -> Self {
        Self {
            gate,
            source_wave_id: PRIOR_WAVE_ID.to_string(),
            source_lane_id: ADMISSION_SOURCE_LANE_ID.to_string(),
            admitted_root: admitted_root.into(),
            admission_verdict_root: admission_verdict_root.into(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "source_wave_id": self.source_wave_id,
            "source_lane_id": self.source_lane_id,
            "admitted_root": self.admitted_root,
            "admission_verdict_root": self.admission_verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admitted_root_binding", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotFillBlocker {
    pub gate: CompileGate,
    pub kind: SlotFillBlockerKind,
    pub blocker_root: String,
}

impl SlotFillBlocker {
    pub fn new(
        gate: CompileGate,
        kind: SlotFillBlockerKind,
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
        record_root("slot_fill_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FillAttempt {
    pub gate: CompileGate,
    pub status: FillStatus,
    pub admitted_binding_root: String,
    pub wave92_slot_root: String,
    pub fill_attempt_root: String,
    pub blocker_roots: Vec<String>,
}

impl FillAttempt {
    pub fn empty(gate: CompileGate) -> Self {
        let blocker = SlotFillBlocker::new(
            gate,
            SlotFillBlockerKind::DefaultFailClosed,
            empty_root("default_slot_fill_blocker"),
        );
        let blocker_roots = vec![blocker.state_root()];
        let admitted_binding_root = empty_root("admitted_binding");
        let wave92_slot_root = empty_root("wave92_slot");
        let fill_attempt_root = fill_attempt_root(
            gate,
            FillStatus::Blocked,
            &admitted_binding_root,
            &wave92_slot_root,
            &blocker_roots,
        );
        Self {
            gate,
            status: FillStatus::Blocked,
            admitted_binding_root,
            wave92_slot_root,
            fill_attempt_root,
            blocker_roots,
        }
    }

    pub fn staged(
        gate: CompileGate,
        admitted_binding_root: impl Into<String>,
        wave92_slot_root: impl Into<String>,
        blocker_roots: Vec<String>,
    ) -> Self {
        let admitted_binding_root = admitted_binding_root.into();
        let wave92_slot_root = wave92_slot_root.into();
        let status = if blocker_roots.is_empty() {
            FillStatus::Staged
        } else {
            FillStatus::Blocked
        };
        let fill_attempt_root = fill_attempt_root(
            gate,
            status,
            &admitted_binding_root,
            &wave92_slot_root,
            &blocker_roots,
        );
        Self {
            gate,
            status,
            admitted_binding_root,
            wave92_slot_root,
            fill_attempt_root,
            blocker_roots,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "status": self.status.as_str(),
            "admitted_binding_root": self.admitted_binding_root,
            "wave92_slot_root": self.wave92_slot_root,
            "fill_attempt_root": self.fill_attempt_root,
            "blocker_roots": self.blocker_roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("fill_attempt", &self.public_record())
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
                    "rule": "wave94_stages_admitted_roots_only_and_runs_no_live_gate",
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
pub struct FillVerdict {
    pub verdict: RuntimeVerdict,
    pub staged_count: usize,
    pub blocked_count: usize,
    pub filled_count: usize,
    pub blocker_root: String,
}

impl FillVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "staged_count": self.staged_count,
            "blocked_count": self.blocked_count,
            "filled_count": self.filled_count,
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("fill_verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub admitted_binding_root: String,
    pub fill_attempt_root: String,
    pub slot_fill_blocker_root: String,
    pub command_hint_root: String,
    pub operator_signoff_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "admitted_binding_root": self.admitted_binding_root,
            "fill_attempt_root": self.fill_attempt_root,
            "slot_fill_blocker_root": self.slot_fill_blocker_root,
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
    pub admitted_bindings: BTreeMap<CompileGate, Vec<AdmittedRootBinding>>,
    pub wave92_slot_roots: BTreeMap<CompileGate, String>,
    pub fill_attempts: Vec<FillAttempt>,
    pub slot_fill_blockers: Vec<SlotFillBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub operator_signoff_roots: Vec<String>,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut admitted_bindings = BTreeMap::new();
        let mut wave92_slot_roots = BTreeMap::new();
        let mut fill_attempts = Vec::new();
        let mut slot_fill_blockers = Vec::new();
        for gate in CompileGate::all() {
            admitted_bindings.insert(gate, Vec::new());
            wave92_slot_roots.insert(gate, empty_root("wave92_slot"));
            fill_attempts.push(FillAttempt::empty(gate));
            slot_fill_blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::DefaultFailClosed,
                empty_root("default_slot_fill_blocker"),
            ));
            slot_fill_blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::ProductionDenied,
                config.state_root(),
            ));
        }
        Self {
            config,
            admitted_bindings,
            wave92_slot_roots,
            fill_attempts,
            slot_fill_blockers,
            command_hints: default_command_hints(),
            operator_signoff_roots: Vec::new(),
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet())
    }

    pub fn import_wave92_slot_root(
        &mut self,
        gate: CompileGate,
        slot_root: impl Into<String>,
    ) -> Result<()> {
        self.wave92_slot_roots.insert(gate, slot_root.into());
        Ok(())
    }

    pub fn add_operator_signoff_root(&mut self, signoff_root: impl Into<String>) -> Result<()> {
        let signoff_root = signoff_root.into();
        if signoff_root.len() < self.config.min_admitted_root_chars {
            return Err("operator signoff root too short".to_string());
        }
        if self.operator_signoff_roots.contains(&signoff_root) {
            return Err("duplicate operator signoff root".to_string());
        }
        self.operator_signoff_roots.push(signoff_root);
        Ok(())
    }

    pub fn stage_admitted_root(&mut self, binding: AdmittedRootBinding) -> Result<()> {
        if self.staged_binding_count() >= self.config.max_staged_fills {
            self.push_blocker(
                binding.gate,
                SlotFillBlockerKind::StagingCapacityReached,
                binding.state_root(),
            );
            return Err("staging capacity reached".to_string());
        }
        if binding.admitted_root.len() < self.config.min_admitted_root_chars {
            self.push_blocker(
                binding.gate,
                SlotFillBlockerKind::AdmittedRootTooShort,
                binding.state_root(),
            );
            return Err("admitted root too short".to_string());
        }
        if self.binding_exists(&binding.admitted_root) {
            self.push_blocker(
                binding.gate,
                SlotFillBlockerKind::DuplicateStagedRoot,
                binding.state_root(),
            );
            return Err("duplicate staged root".to_string());
        }

        let gate = binding.gate;
        let binding_root = binding.state_root();
        self.admitted_bindings
            .entry(gate)
            .or_insert_with(Vec::new)
            .push(binding);
        let wave92_slot_root = self.wave92_slot_root(gate);
        let blockers = self.blockers_for_fill(gate, &binding_root, &wave92_slot_root);
        let blocker_roots = blockers
            .iter()
            .map(SlotFillBlocker::state_root)
            .collect::<Vec<_>>();
        self.slot_fill_blockers.extend(blockers);
        self.fill_attempts.push(FillAttempt::staged(
            gate,
            binding_root,
            wave92_slot_root,
            blocker_roots,
        ));
        Ok(())
    }

    pub fn staged_binding_count(&self) -> usize {
        self.admitted_bindings.values().map(Vec::len).sum()
    }

    pub fn staged_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| attempt.status == FillStatus::Staged)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| attempt.status == FillStatus::Blocked)
            .count()
    }

    pub fn filled_count(&self) -> usize {
        self.fill_attempts
            .iter()
            .filter(|attempt| attempt.status == FillStatus::Filled)
            .count()
    }

    pub fn verdict(&self) -> FillVerdict {
        let blocker_root = list_root(
            "slot_fill_blockers",
            self.slot_fill_blockers
                .iter()
                .map(SlotFillBlocker::public_record),
        );
        let verdict = if !self.config.production_allowed {
            RuntimeVerdict::ProductionDenied
        } else if self.staged_count() > 0 && self.blocked_count() == 0 {
            RuntimeVerdict::ReadyForSlotFillReview
        } else if self.staged_binding_count() > 0 {
            RuntimeVerdict::StagingOnly
        } else {
            RuntimeVerdict::FailClosed
        };
        FillVerdict {
            verdict,
            staged_count: self.staged_count(),
            blocked_count: self.blocked_count(),
            filled_count: self.filled_count(),
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
        kind: SlotFillBlockerKind,
        blocker_root: impl Into<String>,
    ) {
        self.slot_fill_blockers
            .push(SlotFillBlocker::new(gate, kind, blocker_root));
    }

    fn wave92_slot_root(&self, gate: CompileGate) -> String {
        match self.wave92_slot_roots.get(&gate) {
            Some(root) => root.clone(),
            None => empty_root("wave92_slot"),
        }
    }

    fn binding_exists(&self, admitted_root: &str) -> bool {
        self.admitted_bindings.values().any(|bindings| {
            bindings
                .iter()
                .any(|binding| binding.admitted_root == admitted_root)
        })
    }

    fn blockers_for_fill(
        &self,
        gate: CompileGate,
        binding_root: &str,
        wave92_slot_root: &str,
    ) -> Vec<SlotFillBlocker> {
        let mut blockers = Vec::new();
        if binding_root == empty_root("admitted_binding") {
            blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::NoWave93AdmittedRoot,
                binding_root,
            ));
        }
        if wave92_slot_root == empty_root("wave92_slot") {
            blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::Wave92SlotRootMissing,
                wave92_slot_root,
            ));
        }
        if self.operator_signoff_roots.len() < self.config.min_operator_signoff_roots {
            blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::OperatorSignoffQuorumMissing,
                operator_signoff_root(&self.operator_signoff_roots),
            ));
        }
        blockers.push(SlotFillBlocker::new(
            gate,
            SlotFillBlockerKind::HeavyGateNotRunInThisLane,
            binding_root,
        ));
        if !self.config.production_allowed {
            blockers.push(SlotFillBlocker::new(
                gate,
                SlotFillBlockerKind::ProductionDenied,
                self.config.state_root(),
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
            "stage_cargo_check_admitted_root_for_wave92_slot_fill",
            "cargo check --workspace --all-targets",
        ),
        CommandHint::new(
            CompileGate::CargoTest,
            "stage_cargo_test_admitted_root_for_wave92_slot_fill",
            "cargo test --workspace --all-targets",
        ),
        CommandHint::new(
            CompileGate::Clippy,
            "stage_clippy_admitted_root_for_wave92_slot_fill",
            "cargo clippy --workspace --all-targets -- -D warnings",
        ),
        CommandHint::new(
            CompileGate::Rustfmt,
            "stage_rustfmt_admitted_root_for_wave92_slot_fill",
            "cargo fmt --all --check",
        ),
        CommandHint::new(
            CompileGate::Rustc,
            "stage_rustc_admitted_root_for_wave92_slot_fill",
            "rustc --version --verbose",
        ),
        CommandHint::new(
            CompileGate::BuildMetadata,
            "stage_build_metadata_admitted_root_for_wave92_slot_fill",
            "cargo metadata --no-deps --format-version 1",
        ),
        CommandHint::new(
            CompileGate::OperatorSignoff,
            "stage_operator_signoff_admitted_root_for_wave92_slot_fill",
            "operator records independent approval roots",
        ),
    ]
}

pub fn public_record_from_state(state: &State) -> PublicRecord {
    let config_root = state.config.state_root();
    let admitted_binding_root = admitted_binding_root(&state.admitted_bindings);
    let fill_attempt_root = list_root(
        "fill_attempts",
        state.fill_attempts.iter().map(FillAttempt::public_record),
    );
    let slot_fill_blocker_root = list_root(
        "slot_fill_blockers",
        state
            .slot_fill_blockers
            .iter()
            .map(SlotFillBlocker::public_record),
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
            "admitted_binding_root": admitted_binding_root,
            "fill_attempt_root": fill_attempt_root,
            "slot_fill_blocker_root": slot_fill_blocker_root,
            "command_hint_root": command_hint_root,
            "operator_signoff_root": operator_signoff_root,
            "verdict_root": verdict_root,
        }),
    );
    PublicRecord {
        config_root,
        admitted_binding_root,
        fill_attempt_root,
        slot_fill_blocker_root,
        command_hint_root,
        operator_signoff_root,
        verdict_root,
        state_root,
    }
}

pub fn admitted_binding_root(bindings: &BTreeMap<CompileGate, Vec<AdmittedRootBinding>>) -> String {
    let leaves = bindings
        .iter()
        .map(|(gate, roots)| {
            json!({
                "gate": gate.as_str(),
                "binding_roots": roots
                    .iter()
                    .map(AdmittedRootBinding::state_root)
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-WAVE94-RECEIPT-FILL-STAGING-ADMITTED-BINDINGS",
        &leaves,
    )
}

pub fn fill_attempt_root(
    gate: CompileGate,
    status: FillStatus,
    admitted_binding_root: &str,
    wave92_slot_root: &str,
    blocker_roots: &[String],
) -> String {
    record_root(
        "fill_attempt_root",
        &json!({
            "gate": gate.as_str(),
            "status": status.as_str(),
            "admitted_binding_root": admitted_binding_root,
            "wave92_slot_root": wave92_slot_root,
            "blocker_root": string_list_root("fill_attempt_blockers", blocker_roots),
        }),
    )
}

pub fn operator_signoff_root(values: &[String]) -> String {
    string_list_root("operator_signoff_roots", values)
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE94-RECEIPT-FILL-STAGING-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

pub fn empty_root(kind: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE94-RECEIPT-FILL-STAGING-EMPTY",
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
        &format!("MONERO-L2-PQ-BRIDGE-WAVE94-RECEIPT-FILL-STAGING-{kind}"),
        &leaves,
    )
}

pub fn string_list_root(kind: &str, values: &[String]) -> String {
    let leaves = values
        .iter()
        .map(|value| json!({ "root": value }))
        .collect::<Vec<_>>();
    merkle_root(
        &format!("MONERO-L2-PQ-BRIDGE-WAVE94-RECEIPT-FILL-STAGING-{kind}"),
        &leaves,
    )
}

pub fn dedupe_blockers(blockers: Vec<SlotFillBlocker>) -> Vec<SlotFillBlocker> {
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
