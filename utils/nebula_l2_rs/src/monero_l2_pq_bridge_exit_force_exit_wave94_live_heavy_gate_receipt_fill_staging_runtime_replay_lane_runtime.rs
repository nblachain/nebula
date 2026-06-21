use std::collections::BTreeMap;

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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave94-receipt-fill-staging-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str = "wave94-live-heavy-gate-receipt-fill-staging-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 94;
pub const DEFAULT_SOURCE_WAVE: u64 = 93;
pub const DEFAULT_SLOT_WAVE: u64 = 92;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    ReplayRun,
    RollbackDrill,
    AdversarialReplay,
    StaleArchiveReplacement,
    LiveExecutionReceipt,
    OperatorSignoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReplayRun,
            Self::RollbackDrill,
            Self::AdversarialReplay,
            Self::StaleArchiveReplacement,
            Self::LiveExecutionReceipt,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayRun => "replay_run",
            Self::RollbackDrill => "rollback_drill",
            Self::AdversarialReplay => "adversarial_replay",
            Self::StaleArchiveReplacement => "stale_archive_replacement",
            Self::LiveExecutionReceipt => "live_execution_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FillStatus {
    Blocked,
    Staged,
}

impl FillStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Staged => "staged",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    EmptyStagedFill,
    MissingWave93AdmissionRoot,
    MissingWave92SlotRoot,
    RootBindingMissing,
    HeavyGateClaimPresent,
    ProductionDenied,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStagedFill => "empty_staged_fill",
            Self::MissingWave93AdmissionRoot => "missing_wave93_admission_root",
            Self::MissingWave92SlotRoot => "missing_wave92_slot_root",
            Self::RootBindingMissing => "root_binding_missing",
            Self::HeavyGateClaimPresent => "heavy_gate_claim_present",
            Self::ProductionDenied => "production_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    StageAdmittedRoot,
    BindWave92Slot,
    RecomputeFillAttempt,
    KeepFailClosed,
    ReplaceStaleArchiveRoot,
    SignOffRootsOnly,
}

impl CommandHintKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::StageAdmittedRoot,
            Self::BindWave92Slot,
            Self::RecomputeFillAttempt,
            Self::KeepFailClosed,
            Self::ReplaceStaleArchiveRoot,
            Self::SignOffRootsOnly,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::StageAdmittedRoot => "stage_admitted_root",
            Self::BindWave92Slot => "bind_wave92_slot",
            Self::RecomputeFillAttempt => "recompute_fill_attempt",
            Self::KeepFailClosed => "keep_fail_closed",
            Self::ReplaceStaleArchiveRoot => "replace_stale_archive_root",
            Self::SignOffRootsOnly => "sign_off_roots_only",
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
    pub source_wave: u64,
    pub slot_wave: u64,
    pub wave92_future_slot_registry_root: String,
    pub wave93_admission_quarantine_root: String,
    pub fail_closed_armed: bool,
    pub production_denied: bool,
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
            source_wave: DEFAULT_SOURCE_WAVE,
            slot_wave: DEFAULT_SLOT_WAVE,
            wave92_future_slot_registry_root: stable_id("wave92-future-slot-registry", "all"),
            wave93_admission_quarantine_root: stable_id("wave93-admission-quarantine", "all"),
            fail_closed_armed: true,
            production_denied: true,
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
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("slot_wave", self.slot_wave)?;
        ensure_root(
            "wave92_future_slot_registry_root",
            &self.wave92_future_slot_registry_root,
        )?;
        ensure_root(
            "wave93_admission_quarantine_root",
            &self.wave93_admission_quarantine_root,
        )?;
        if self.slot_wave >= self.source_wave || self.source_wave >= self.wave {
            return Err("wave ordering must be slot, source, staging".to_string());
        }
        if !self.fail_closed_armed {
            return Err("fill staging fail closed guard is disarmed".to_string());
        }
        if !self.production_denied {
            return Err("devnet fill staging must deny production".to_string());
        }
        if self.heavy_gates_ran {
            return Err("fill staging cannot claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_fill_staging_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "slot_wave": self.slot_wave,
            "wave92_future_slot_registry_root": self.wave92_future_slot_registry_root,
            "wave93_admission_quarantine_root": self.wave93_admission_quarantine_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_denied": self.production_denied,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE94-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FillAttempt {
    pub lane: ReplayLane,
    pub status: FillStatus,
    pub wave92_slot_root: String,
    pub wave93_admitted_root: Option<String>,
    pub fill_attempt_root: String,
    pub admitted_root_binding: String,
    pub command_hint_root: String,
    pub roots_only: bool,
    pub heavy_gate_claimed: bool,
}

impl FillAttempt {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        let wave92_slot_root = stable_id(
            "wave92-fill-slot",
            &format!(
                "{}:{}",
                config.wave92_future_slot_registry_root,
                lane.as_str()
            ),
        );
        let admitted_root_binding = stable_id(
            "wave94-empty-admitted-root-binding",
            &format!("{}:{}", lane.as_str(), wave92_slot_root),
        );
        let command_hint_root = stable_id(
            "wave94-command-hint",
            &format!(
                "{}:{}",
                lane.as_str(),
                CommandHintKind::StageAdmittedRoot.as_str()
            ),
        );
        let fill_attempt_root = stable_id(
            "wave94-blocked-fill-attempt",
            &format!("{}:{}", wave92_slot_root, admitted_root_binding),
        );
        Self {
            lane,
            status: FillStatus::Blocked,
            wave92_slot_root,
            wave93_admitted_root: None,
            fill_attempt_root,
            admitted_root_binding,
            command_hint_root,
            roots_only: true,
            heavy_gate_claimed: false,
        }
    }

    pub fn stage(&mut self, admitted_root: &str, config: &Config) -> Result<()> {
        ensure_root("wave93_admitted_root", admitted_root)?;
        if self.heavy_gate_claimed || config.heavy_gates_ran {
            return Err("fill attempt cannot claim heavy gates ran".to_string());
        }
        self.wave93_admitted_root = Some(admitted_root.to_string());
        self.status = FillStatus::Staged;
        self.admitted_root_binding = stable_id(
            "wave94-admitted-root-binding",
            &format!(
                "{}:{}:{}",
                self.lane.as_str(),
                self.wave92_slot_root,
                admitted_root
            ),
        );
        self.fill_attempt_root = stable_id(
            "wave94-staged-fill-attempt",
            &format!(
                "{}:{}:{}",
                self.lane.as_str(),
                self.admitted_root_binding,
                config.wave93_admission_quarantine_root
            ),
        );
        self.command_hint_root = stable_id(
            "wave94-command-hint",
            &format!(
                "{}:{}",
                self.lane.as_str(),
                CommandHintKind::RecomputeFillAttempt.as_str()
            ),
        );
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_fill_attempt",
            "lane": self.lane.as_str(),
            "status": self.status.as_str(),
            "wave92_slot_root": self.wave92_slot_root,
            "wave93_admitted_root": self.wave93_admitted_root,
            "fill_attempt_root": self.fill_attempt_root,
            "admitted_root_binding": self.admitted_root_binding,
            "command_hint_root": self.command_hint_root,
            "roots_only": self.roots_only,
            "heavy_gate_claimed": self.heavy_gate_claimed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE94-FILL-ATTEMPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotFillBlocker {
    pub lane: ReplayLane,
    pub blocker_kind: BlockerKind,
    pub subject_root: String,
    pub blocker_root: String,
}

impl SlotFillBlocker {
    pub fn new(lane: ReplayLane, blocker_kind: BlockerKind, subject_root: &str) -> Self {
        Self {
            lane,
            blocker_kind,
            subject_root: subject_root.to_string(),
            blocker_root: stable_id(
                "wave94-slot-fill-blocker",
                &format!(
                    "{}:{}:{}",
                    lane.as_str(),
                    blocker_kind.as_str(),
                    subject_root
                ),
            ),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_slot_fill_blocker",
            "lane": self.lane.as_str(),
            "blocker_kind": self.blocker_kind.as_str(),
            "subject_root": self.subject_root,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub lane: ReplayLane,
    pub hint_kind: CommandHintKind,
    pub hint_root: String,
    pub allowed_while_fail_closed: bool,
}

impl CommandHint {
    pub fn new(lane: ReplayLane, hint_kind: CommandHintKind) -> Self {
        Self {
            lane,
            hint_kind,
            hint_root: stable_id(
                "wave94-command-hint-entry",
                &format!("{}:{}", lane.as_str(), hint_kind.as_str()),
            ),
            allowed_while_fail_closed: true,
        }
    }

    pub fn canonical() -> Vec<Self> {
        ReplayLane::all()
            .into_iter()
            .flat_map(|lane| {
                CommandHintKind::all()
                    .into_iter()
                    .map(move |kind| Self::new(lane, kind))
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_command_hint",
            "lane": self.lane.as_str(),
            "hint_kind": self.hint_kind.as_str(),
            "hint_root": self.hint_root,
            "allowed_while_fail_closed": self.allowed_while_fail_closed,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lanes_total: u64,
    pub staged_fills: u64,
    pub blocked_slots: u64,
    pub blocker_count: u64,
    pub command_hints: u64,
    pub public_raw_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_fill_staging_counters",
            "lanes_total": self.lanes_total,
            "staged_fills": self.staged_fills,
            "blocked_slots": self.blocked_slots,
            "blocker_count": self.blocker_count,
            "command_hints": self.command_hints,
            "public_raw_records": self.public_raw_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub fill_attempts: BTreeMap<ReplayLane, FillAttempt>,
    pub slot_fill_blockers: Vec<SlotFillBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl Default for State {
    fn default() -> Self {
        devnet()
    }
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let mut fill_attempts = BTreeMap::new();
        for lane in ReplayLane::all() {
            fill_attempts.insert(lane, FillAttempt::blocked(lane, &config));
        }
        let mut state = Self {
            config,
            fill_attempts,
            slot_fill_blockers: Vec::new(),
            command_hints: CommandHint::canonical(),
            counters: Counters::default(),
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn stage_admitted_root(&mut self, lane: ReplayLane, admitted_root: &str) -> Result<String> {
        self.validate_stage(lane, admitted_root)?;
        let attempt = self
            .fill_attempts
            .get_mut(&lane)
            .ok_or_else(|| "fill lane is not registered".to_string())?;
        attempt.stage(admitted_root, &self.config)?;
        let root = attempt.state_root();
        self.recompute();
        self.validate()?;
        Ok(root)
    }

    pub fn validate_stage(&self, lane: ReplayLane, admitted_root: &str) -> Result<()> {
        ensure_root("wave93_admitted_root", admitted_root)?;
        let attempt = self
            .fill_attempts
            .get(&lane)
            .ok_or_else(|| "fill lane is not registered".to_string())?;
        ensure_root("wave92_slot_root", &attempt.wave92_slot_root)?;
        if self.config.heavy_gates_ran || attempt.heavy_gate_claimed {
            return Err("staging cannot claim heavy gates ran".to_string());
        }
        if !attempt.roots_only {
            return Err("fill attempt is not roots only".to_string());
        }
        Ok(())
    }

    pub fn recompute(&mut self) {
        self.slot_fill_blockers = self.compute_blockers();
        self.counters = self.compute_counters();
    }

    pub fn compute_blockers(&self) -> Vec<SlotFillBlocker> {
        let mut blockers = Vec::new();
        for attempt in self.fill_attempts.values() {
            if attempt.wave93_admitted_root.is_none() {
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::EmptyStagedFill,
                    &attempt.fill_attempt_root,
                ));
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::MissingWave93AdmissionRoot,
                    &attempt.admitted_root_binding,
                ));
            }
            if attempt.wave92_slot_root.trim().is_empty() {
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::MissingWave92SlotRoot,
                    &attempt.fill_attempt_root,
                ));
            }
            if attempt.admitted_root_binding.trim().is_empty() {
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::RootBindingMissing,
                    &attempt.fill_attempt_root,
                ));
            }
            if attempt.heavy_gate_claimed || self.config.heavy_gates_ran {
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::HeavyGateClaimPresent,
                    &attempt.fill_attempt_root,
                ));
            }
            if self.config.production_denied {
                blockers.push(SlotFillBlocker::new(
                    attempt.lane,
                    BlockerKind::ProductionDenied,
                    &attempt.fill_attempt_root,
                ));
            }
        }
        blockers
    }

    pub fn compute_counters(&self) -> Counters {
        Counters {
            lanes_total: self.fill_attempts.len() as u64,
            staged_fills: self
                .fill_attempts
                .values()
                .filter(|attempt| attempt.status == FillStatus::Staged)
                .count() as u64,
            blocked_slots: self
                .fill_attempts
                .values()
                .filter(|attempt| attempt.status == FillStatus::Blocked)
                .count() as u64,
            blocker_count: self.slot_fill_blockers.len() as u64,
            command_hints: self.command_hints.len() as u64,
            public_raw_records: 0,
        }
    }

    pub fn fill_attempt_root(&self) -> String {
        collection_root(
            "WAVE94-FILL-ATTEMPTS",
            self.fill_attempts
                .values()
                .map(FillAttempt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE94-SLOT-FILL-BLOCKERS",
            self.slot_fill_blockers
                .iter()
                .map(SlotFillBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE94-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn admitted_root_binding_root(&self) -> String {
        collection_root(
            "WAVE94-ADMITTED-ROOT-BINDINGS",
            self.fill_attempts
                .values()
                .map(|attempt| {
                    json!({
                        "lane": attempt.lane.as_str(),
                        "wave92_slot_root": attempt.wave92_slot_root,
                        "wave93_admitted_root": attempt.wave93_admitted_root,
                        "admitted_root_binding": attempt.admitted_root_binding,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn readiness_root(&self) -> String {
        value_root(
            "WAVE94-READINESS",
            &json!({
                "fill_attempt_root": self.fill_attempt_root(),
                "blocker_root": self.blocker_root(),
                "admitted_root_binding_root": self.admitted_root_binding_root(),
                "command_hint_root": self.command_hint_root(),
                "fail_closed_armed": self.config.fail_closed_armed,
                "production_denied": self.config.production_denied,
                "heavy_gates_ran": self.config.heavy_gates_ran,
                "counters": self.counters.public_record(),
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE94-STATE",
            &json!({
                "config_root": self.config.state_root(),
                "fill_attempt_root": self.fill_attempt_root(),
                "blocker_root": self.blocker_root(),
                "admitted_root_binding_root": self.admitted_root_binding_root(),
                "command_hint_root": self.command_hint_root(),
                "readiness_root": self.readiness_root(),
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        for attempt in self.fill_attempts.values() {
            ensure_root("fill_attempt_root", &attempt.fill_attempt_root)?;
            ensure_root("wave92_slot_root", &attempt.wave92_slot_root)?;
            ensure_root("admitted_root_binding", &attempt.admitted_root_binding)?;
            ensure_root("command_hint_root", &attempt.command_hint_root)?;
            if !attempt.roots_only {
                return Err("fill attempt is not roots only".to_string());
            }
            if attempt.heavy_gate_claimed {
                return Err("fill attempt claims heavy gates ran".to_string());
            }
        }
        for blocker in &self.slot_fill_blockers {
            ensure_root("blocker_root", &blocker.blocker_root)?;
            ensure_root("blocker_subject_root", &blocker.subject_root)?;
        }
        for hint in &self.command_hints {
            ensure_root("hint_root", &hint.hint_root)?;
        }
        if self.counters.public_raw_records > self.config.max_public_raw_records {
            return Err("public record contains raw material".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_receipt_fill_staging_runtime_replay_lane_state",
            "config": self.config.public_record(),
            "fill_attempt_root": self.fill_attempt_root(),
            "blocker_root": self.blocker_root(),
            "admitted_root_binding_root": self.admitted_root_binding_root(),
            "command_hint_root": self.command_hint_root(),
            "readiness_root": self.readiness_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "fill_attempts": self.fill_attempts.values().map(FillAttempt::public_record).collect::<Vec<_>>(),
            "slot_fill_blockers": self.slot_fill_blockers.iter().map(SlotFillBlocker::public_record).collect::<Vec<_>>(),
            "command_hints": self.command_hints.iter().map(CommandHint::public_record).collect::<Vec<_>>(),
        })
    }
}

pub fn devnet() -> Runtime {
    match State::new(Config::devnet()) {
        Ok(runtime) => runtime,
        Err(error) => fallback_runtime(error),
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn fallback_runtime(error: String) -> Runtime {
    let config = Config::devnet();
    let mut fill_attempts = BTreeMap::new();
    for lane in ReplayLane::all() {
        fill_attempts.insert(lane, FillAttempt::blocked(lane, &config));
    }
    let error_root = stable_id("wave94-fallback-error", &error);
    let mut state = State {
        config,
        fill_attempts,
        slot_fill_blockers: vec![SlotFillBlocker::new(
            ReplayLane::ReplayRun,
            BlockerKind::ProductionDenied,
            &error_root,
        )],
        command_hints: CommandHint::canonical(),
        counters: Counters::default(),
    };
    state.counters = state.compute_counters();
    state
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{} is empty", field));
    }
    Ok(())
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        return Err(format!("{} must be positive", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if !is_root(value) {
        return Err(format!("{} is not a canonical root", field));
    }
    Ok(())
}

fn is_root(value: &str) -> bool {
    value.len() >= 32 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn stable_id(domain: &str, value: &str) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LANE_SUITE),
            HashPart::Str(value),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&canonical_value(value)),
        ],
        32,
    )
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    let leaves = values
        .iter()
        .map(|value| json!(value_root(domain, value)))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn canonical_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(item) => item.to_string(),
        Value::Number(item) => item.to_string(),
        Value::String(item) => format!("{:?}", item),
        Value::Array(items) => {
            let inner = items
                .iter()
                .map(canonical_value)
                .collect::<Vec<_>>()
                .join(",");
            format!("[{}]", inner)
        }
        Value::Object(items) => {
            let inner = items
                .iter()
                .map(|(key, item)| format!("{:?}:{}", key, canonical_value(item)))
                .collect::<Vec<_>>()
                .join(",");
            format!("{{{}}}", inner)
        }
    }
}
