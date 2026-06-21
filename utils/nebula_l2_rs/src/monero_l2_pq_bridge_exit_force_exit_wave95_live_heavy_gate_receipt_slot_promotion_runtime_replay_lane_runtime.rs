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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave95-receipt-slot-promotion-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str = "wave95-live-heavy-gate-receipt-slot-promotion-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 95;
pub const DEFAULT_STAGING_WAVE: u64 = 94;
pub const DEFAULT_ADMISSION_WAVE: u64 = 93;
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
pub enum PromotionDecision {
    Blocked,
    Promoted,
}

impl PromotionDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Promoted => "promoted",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBlocker {
    FailClosedArmed,
    ProductionDenied,
    NoPromotedSlotsDefault,
    HeavyGateReceiptAbsent,
    SlotOccupancyRootPlaceholder,
    StagedFillBindingOnly,
    Wave94StagingRootNotFinal,
    Wave93AdmissionRootNotFinal,
    Wave92SlotRootNotFinal,
    OperatorSignoffMissing,
}

impl PromotionBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ProductionDenied,
            Self::NoPromotedSlotsDefault,
            Self::HeavyGateReceiptAbsent,
            Self::SlotOccupancyRootPlaceholder,
            Self::StagedFillBindingOnly,
            Self::Wave94StagingRootNotFinal,
            Self::Wave93AdmissionRootNotFinal,
            Self::Wave92SlotRootNotFinal,
            Self::OperatorSignoffMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ProductionDenied => "production_denied",
            Self::NoPromotedSlotsDefault => "no_promoted_slots_default",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::SlotOccupancyRootPlaceholder => "slot_occupancy_root_placeholder",
            Self::StagedFillBindingOnly => "staged_fill_binding_only",
            Self::Wave94StagingRootNotFinal => "wave94_staging_root_not_final",
            Self::Wave93AdmissionRootNotFinal => "wave93_admission_root_not_final",
            Self::Wave92SlotRootNotFinal => "wave92_slot_root_not_final",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    ReplayPromotionRoots,
    DrillRollbackToStagedOnly,
    KeepAdversarialAttemptBlocked,
    ReplaceStaleArchiveRoot,
    BindLiveReceiptRoot,
    SignOffRootsOnly,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReplayPromotionRoots,
            Self::DrillRollbackToStagedOnly,
            Self::KeepAdversarialAttemptBlocked,
            Self::ReplaceStaleArchiveRoot,
            Self::BindLiveReceiptRoot,
            Self::SignOffRootsOnly,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayPromotionRoots => "replay_promotion_roots",
            Self::DrillRollbackToStagedOnly => "drill_rollback_to_staged_only",
            Self::KeepAdversarialAttemptBlocked => "keep_adversarial_attempt_blocked",
            Self::ReplaceStaleArchiveRoot => "replace_stale_archive_root",
            Self::BindLiveReceiptRoot => "bind_live_receipt_root",
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
    pub staging_wave: u64,
    pub admission_wave: u64,
    pub slot_wave: u64,
    pub wave94_fill_staging_root: String,
    pub wave93_admission_quarantine_root: String,
    pub wave92_receipt_slot_root: String,
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
            staging_wave: DEFAULT_STAGING_WAVE,
            admission_wave: DEFAULT_ADMISSION_WAVE,
            slot_wave: DEFAULT_SLOT_WAVE,
            wave94_fill_staging_root: stable_id("wave94-fill-staging", "all"),
            wave93_admission_quarantine_root: stable_id("wave93-admission-quarantine", "all"),
            wave92_receipt_slot_root: stable_id("wave92-receipt-slot-registry", "all"),
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
        ensure_positive("staging_wave", self.staging_wave)?;
        ensure_positive("admission_wave", self.admission_wave)?;
        ensure_positive("slot_wave", self.slot_wave)?;
        ensure_root("wave94_fill_staging_root", &self.wave94_fill_staging_root)?;
        ensure_root(
            "wave93_admission_quarantine_root",
            &self.wave93_admission_quarantine_root,
        )?;
        ensure_root("wave92_receipt_slot_root", &self.wave92_receipt_slot_root)?;
        if !(self.slot_wave < self.admission_wave
            && self.admission_wave < self.staging_wave
            && self.staging_wave < self.wave)
        {
            return Err("wave ordering must be slot, admission, staging, promotion".to_string());
        }
        if !self.fail_closed_armed {
            return Err("promotion fail closed guard is disarmed".to_string());
        }
        if !self.production_denied {
            return Err("devnet promotion must deny production".to_string());
        }
        if self.heavy_gates_ran {
            return Err("promotion runtime cannot claim heavy gate execution".to_string());
        }
        if self.max_public_raw_records != 0 {
            return Err("public records must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave95_slot_promotion_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "staging_wave": self.staging_wave,
            "admission_wave": self.admission_wave,
            "slot_wave": self.slot_wave,
            "wave94_fill_staging_root": self.wave94_fill_staging_root,
            "wave93_admission_quarantine_root": self.wave93_admission_quarantine_root,
            "wave92_receipt_slot_root": self.wave92_receipt_slot_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_denied": self.production_denied,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StagedFillBinding {
    pub lane: ReplayLane,
    pub staged_fill_root: String,
    pub admission_root: String,
    pub slot_root: String,
    pub slot_occupancy_placeholder_root: String,
}

impl StagedFillBinding {
    pub fn placeholder(lane: ReplayLane) -> Self {
        Self {
            lane,
            staged_fill_root: stable_id("wave94-staged-fill", lane.as_str()),
            admission_root: stable_id("wave93-admitted-fill", lane.as_str()),
            slot_root: stable_id("wave92-slot", lane.as_str()),
            slot_occupancy_placeholder_root: stable_id(
                "wave95-slot-occupancy-placeholder",
                lane.as_str(),
            ),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "staged_fill_root": self.staged_fill_root,
            "admission_root": self.admission_root,
            "slot_root": self.slot_root,
            "slot_occupancy_placeholder_root": self.slot_occupancy_placeholder_root,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-STAGED-FILL-BINDING", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub lane: ReplayLane,
    pub decision: PromotionDecision,
    pub binding_root: String,
    pub slot_occupancy_claim_root: String,
    pub promoted_slot_count: u64,
    pub blocker_roots: Vec<String>,
}

impl PromotionAttempt {
    pub fn blocked(lane: ReplayLane, binding: &StagedFillBinding) -> Self {
        let blocker_roots = PromotionBlocker::all()
            .iter()
            .map(|blocker| blocker_root(lane, *blocker))
            .collect();
        Self {
            lane,
            decision: PromotionDecision::Blocked,
            binding_root: binding.state_root(),
            slot_occupancy_claim_root: binding.slot_occupancy_placeholder_root.clone(),
            promoted_slot_count: 0,
            blocker_roots,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("binding_root", &self.binding_root)?;
        ensure_root("slot_occupancy_claim_root", &self.slot_occupancy_claim_root)?;
        if self.decision == PromotionDecision::Promoted && self.promoted_slot_count == 0 {
            return Err("promoted decision requires promoted slot count".to_string());
        }
        if self.decision == PromotionDecision::Blocked && self.blocker_roots.is_empty() {
            return Err("blocked promotion requires blocker roots".to_string());
        }
        for root in &self.blocker_roots {
            ensure_root("blocker_root", root)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "decision": self.decision.as_str(),
            "binding_root": self.binding_root,
            "slot_occupancy_claim_root": self.slot_occupancy_claim_root,
            "promoted_slot_count": self.promoted_slot_count,
            "blocker_root": list_root("promotion-blockers", self.blocker_roots.iter().cloned().collect()),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-PROMOTION-ATTEMPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneReceipt {
    pub lane: ReplayLane,
    pub binding_root: String,
    pub promotion_attempt_root: String,
    pub command_hint_root: String,
    pub replay_root: String,
}

impl LaneReceipt {
    pub fn new(binding: &StagedFillBinding, attempt: &PromotionAttempt) -> Self {
        let command_hint_root = command_hint_root(binding.lane);
        let replay_root = domain_hash(
            "WAVE95-LANE-REPLAY-ROOT",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(binding.lane.as_str()),
                HashPart::Str(&binding.state_root()),
                HashPart::Str(&attempt.state_root()),
                HashPart::Str(&command_hint_root),
            ],
            32,
        );
        Self {
            lane: binding.lane,
            binding_root: binding.state_root(),
            promotion_attempt_root: attempt.state_root(),
            command_hint_root,
            replay_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "binding_root": self.binding_root,
            "promotion_attempt_root": self.promotion_attempt_root,
            "command_hint_root": self.command_hint_root,
            "replay_root": self.replay_root,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-LANE-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionCounters {
    pub promotion_attempt_count: u64,
    pub promoted_slot_count: u64,
    pub blocked_attempt_count: u64,
    pub active_blocker_count: u64,
    pub slot_occupancy_placeholder_count: u64,
}

impl PromotionCounters {
    pub fn from_attempts(attempts: &[PromotionAttempt]) -> Self {
        let mut counters = Self::default();
        counters.promotion_attempt_count = attempts.len() as u64;
        for attempt in attempts {
            counters.promoted_slot_count = counters
                .promoted_slot_count
                .saturating_add(attempt.promoted_slot_count);
            if attempt.decision == PromotionDecision::Blocked {
                counters.blocked_attempt_count = counters.blocked_attempt_count.saturating_add(1);
            }
            counters.active_blocker_count = counters
                .active_blocker_count
                .saturating_add(attempt.blocker_roots.len() as u64);
            if attempt.promoted_slot_count == 0 {
                counters.slot_occupancy_placeholder_count =
                    counters.slot_occupancy_placeholder_count.saturating_add(1);
            }
        }
        counters
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "promotion_attempt_count": self.promotion_attempt_count,
            "promoted_slot_count": self.promoted_slot_count,
            "blocked_attempt_count": self.blocked_attempt_count,
            "active_blocker_count": self.active_blocker_count,
            "slot_occupancy_placeholder_count": self.slot_occupancy_placeholder_count,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-PROMOTION-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub staged_fill_bindings: Vec<StagedFillBinding>,
    pub promotion_attempts: Vec<PromotionAttempt>,
    pub lane_receipts: Vec<LaneReceipt>,
    pub command_hints: BTreeMap<String, String>,
    pub counters: PromotionCounters,
}

impl State {
    pub fn new(
        config: Config,
        staged_fill_bindings: Vec<StagedFillBinding>,
        promotion_attempts: Vec<PromotionAttempt>,
        lane_receipts: Vec<LaneReceipt>,
        command_hints: BTreeMap<String, String>,
    ) -> Result<Self> {
        config.validate()?;
        if staged_fill_bindings.len() != ReplayLane::all().len() {
            return Err("promotion runtime requires one staged fill binding per lane".to_string());
        }
        if promotion_attempts.len() != staged_fill_bindings.len() {
            return Err("promotion attempts must match staged fill bindings".to_string());
        }
        if lane_receipts.len() != staged_fill_bindings.len() {
            return Err("lane receipts must match staged fill bindings".to_string());
        }
        for binding in &staged_fill_bindings {
            ensure_root("staged_fill_root", &binding.staged_fill_root)?;
            ensure_root("admission_root", &binding.admission_root)?;
            ensure_root("slot_root", &binding.slot_root)?;
            ensure_root(
                "slot_occupancy_placeholder_root",
                &binding.slot_occupancy_placeholder_root,
            )?;
        }
        for attempt in &promotion_attempts {
            attempt.validate()?;
        }
        for receipt in &lane_receipts {
            ensure_root("receipt_binding_root", &receipt.binding_root)?;
            ensure_root(
                "receipt_promotion_attempt_root",
                &receipt.promotion_attempt_root,
            )?;
            ensure_root("receipt_command_hint_root", &receipt.command_hint_root)?;
            ensure_root("receipt_replay_root", &receipt.replay_root)?;
        }
        for root in command_hints.values() {
            ensure_root("command_hint_root", root)?;
        }
        let counters = PromotionCounters::from_attempts(&promotion_attempts);
        if counters.promoted_slot_count != 0 {
            return Err("devnet promotion must keep promoted slot count at zero".to_string());
        }
        if counters.blocked_attempt_count != counters.promotion_attempt_count {
            return Err("devnet promotion must keep every attempt blocked".to_string());
        }
        Ok(Self {
            config,
            staged_fill_bindings,
            promotion_attempts,
            lane_receipts,
            command_hints,
            counters,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> PublicRecord {
        let binding_roots = self
            .staged_fill_bindings
            .iter()
            .map(StagedFillBinding::state_root)
            .collect::<Vec<_>>();
        let attempt_roots = self
            .promotion_attempts
            .iter()
            .map(PromotionAttempt::state_root)
            .collect::<Vec<_>>();
        let lane_roots = self
            .lane_receipts
            .iter()
            .map(LaneReceipt::state_root)
            .collect::<Vec<_>>();
        json!({
            "kind": "wave95_slot_promotion_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "staged_fill_binding_root": list_root("staged-fill-bindings", binding_roots),
            "promotion_attempt_root": list_root("promotion-attempts", attempt_roots),
            "slot_occupancy_root_placeholder": slot_occupancy_placeholder_root(),
            "lane_receipt_root": list_root("lane-receipts", lane_roots),
            "command_hint_root": command_hints_root(&self.command_hints),
            "counter_root": self.counters.state_root(),
            "counters": self.counters.public_record(),
            "production_denied": self.config.production_denied,
            "fail_closed_armed": self.config.fail_closed_armed,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-STATE", &self.public_record())
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            "WAVE95-STATE-INTERNAL",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&attempts_root(&self.promotion_attempts)),
                HashPart::Str(&bindings_root(&self.staged_fill_bindings)),
                HashPart::Str(&receipts_root(&self.lane_receipts)),
                HashPart::Str(&command_hints_root(&self.command_hints)),
                HashPart::Str(&self.counters.state_root()),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let staged_fill_bindings = ReplayLane::all()
        .iter()
        .map(|lane| StagedFillBinding::placeholder(*lane))
        .collect::<Vec<_>>();
    let promotion_attempts = staged_fill_bindings
        .iter()
        .map(|binding| PromotionAttempt::blocked(binding.lane, binding))
        .collect::<Vec<_>>();
    let lane_receipts = staged_fill_bindings
        .iter()
        .zip(promotion_attempts.iter())
        .map(|(binding, attempt)| LaneReceipt::new(binding, attempt))
        .collect::<Vec<_>>();
    let command_hints = CommandHint::all()
        .iter()
        .map(|hint| (hint.as_str().to_string(), command_hint_kind_root(*hint)))
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        staged_fill_bindings,
        promotion_attempts,
        lane_receipts,
        command_hints,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            staged_fill_bindings: Vec::new(),
            promotion_attempts: Vec::new(),
            lane_receipts: Vec::new(),
            command_hints: BTreeMap::new(),
            counters: PromotionCounters::default(),
        },
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn attempts_root(attempts: &[PromotionAttempt]) -> String {
    list_root(
        "promotion-attempt-state-roots",
        attempts.iter().map(PromotionAttempt::state_root).collect(),
    )
}

fn bindings_root(bindings: &[StagedFillBinding]) -> String {
    list_root(
        "staged-fill-binding-state-roots",
        bindings.iter().map(StagedFillBinding::state_root).collect(),
    )
}

fn receipts_root(receipts: &[LaneReceipt]) -> String {
    list_root(
        "lane-receipt-state-roots",
        receipts.iter().map(LaneReceipt::state_root).collect(),
    )
}

fn command_hints_root(hints: &BTreeMap<String, String>) -> String {
    let values = hints
        .iter()
        .map(|(hint, root)| {
            domain_hash(
                "WAVE95-COMMAND-HINT-ENTRY",
                &[
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(hint),
                    HashPart::Str(root),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    list_root("command-hints", values)
}

fn command_hint_root(lane: ReplayLane) -> String {
    domain_hash(
        "WAVE95-LANE-COMMAND-HINT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn command_hint_kind_root(hint: CommandHint) -> String {
    domain_hash(
        "WAVE95-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: PromotionBlocker) -> String {
    domain_hash(
        "WAVE95-PROMOTION-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn slot_occupancy_placeholder_root() -> String {
    domain_hash(
        "WAVE95-SLOT-OCCUPANCY-PLACEHOLDER-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str("zero-promoted-slots"),
        ],
        32,
    )
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE95-STABLE-ROOT",
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
