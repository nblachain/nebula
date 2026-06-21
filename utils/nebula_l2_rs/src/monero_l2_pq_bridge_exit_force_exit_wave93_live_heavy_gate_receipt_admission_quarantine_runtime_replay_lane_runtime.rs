use std::collections::{BTreeMap, BTreeSet};

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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave93-receipt-admission-quarantine-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave93-live-heavy-gate-receipt-admission-quarantine-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 93;
pub const DEFAULT_SOURCE_WAVE: u64 = 92;
pub const DEFAULT_REGISTRY_HEIGHT: u64 = 4_293_093;
pub const DEFAULT_MIN_ADMISSION_RULES: u64 = 10;
pub const DEFAULT_MIN_OPERATOR_COMMANDS: u64 = 6;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayReceiptLane {
    ReplayRun,
    RollbackDrill,
    AdversarialReplay,
    StaleArchiveReplacement,
    LiveExecutionReceipt,
    OperatorSignoff,
}

impl ReplayReceiptLane {
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptRootStatus {
    EmptyQuarantined,
    FutureRootAdmitted,
    Quarantined,
    ReplacedByFreshRoot,
}

impl ReceiptRootStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyQuarantined => "empty_quarantined",
            Self::FutureRootAdmitted => "future_root_admitted",
            Self::Quarantined => "quarantined",
            Self::ReplacedByFreshRoot => "replaced_by_fresh_root",
        }
    }

    pub fn is_admitted(self) -> bool {
        matches!(self, Self::FutureRootAdmitted | Self::ReplacedByFreshRoot)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionRuleKind {
    RootOnly,
    FutureWave,
    SourceSlotBound,
    UniqueRoot,
    NonEmptyRoot,
    Wave92SlotPresent,
    NoHeavyGateClaim,
    QuarantineByDefault,
    OperatorCommandBound,
    ArchiveReplacementFresh,
}

impl AdmissionRuleKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RootOnly,
            Self::FutureWave,
            Self::SourceSlotBound,
            Self::UniqueRoot,
            Self::NonEmptyRoot,
            Self::Wave92SlotPresent,
            Self::NoHeavyGateClaim,
            Self::QuarantineByDefault,
            Self::OperatorCommandBound,
            Self::ArchiveReplacementFresh,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootOnly => "root_only",
            Self::FutureWave => "future_wave",
            Self::SourceSlotBound => "source_slot_bound",
            Self::UniqueRoot => "unique_root",
            Self::NonEmptyRoot => "non_empty_root",
            Self::Wave92SlotPresent => "wave92_slot_present",
            Self::NoHeavyGateClaim => "no_heavy_gate_claim",
            Self::QuarantineByDefault => "quarantine_by_default",
            Self::OperatorCommandBound => "operator_command_bound",
            Self::ArchiveReplacementFresh => "archive_replacement_fresh",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReasonKind {
    EmptyFutureRoot,
    AwaitingFutureReceipt,
    DuplicateRoot,
    SlotBindingMissing,
    SourceWaveMismatch,
    NonRootMaterial,
    HeavyGateClaimPresent,
    StaleArchiveNeedsReplacement,
    OperatorCommandMissing,
    AdmissionRuleMissing,
}

impl QuarantineReasonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFutureRoot => "empty_future_root",
            Self::AwaitingFutureReceipt => "awaiting_future_receipt",
            Self::DuplicateRoot => "duplicate_root",
            Self::SlotBindingMissing => "slot_binding_missing",
            Self::SourceWaveMismatch => "source_wave_mismatch",
            Self::NonRootMaterial => "non_root_material",
            Self::HeavyGateClaimPresent => "heavy_gate_claim_present",
            Self::StaleArchiveNeedsReplacement => "stale_archive_needs_replacement",
            Self::OperatorCommandMissing => "operator_command_missing",
            Self::AdmissionRuleMissing => "admission_rule_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommandKind {
    HoldReceiptRoot,
    AdmitFutureRoot,
    QuarantineRoot,
    ReplaceStaleArchiveRoot,
    RecomputeReplayLaneRoot,
    SignOffRootsOnlyRecord,
}

impl OperatorCommandKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::HoldReceiptRoot,
            Self::AdmitFutureRoot,
            Self::QuarantineRoot,
            Self::ReplaceStaleArchiveRoot,
            Self::RecomputeReplayLaneRoot,
            Self::SignOffRootsOnlyRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldReceiptRoot => "hold_receipt_root",
            Self::AdmitFutureRoot => "admit_future_root",
            Self::QuarantineRoot => "quarantine_root",
            Self::ReplaceStaleArchiveRoot => "replace_stale_archive_root",
            Self::RecomputeReplayLaneRoot => "recompute_replay_lane_root",
            Self::SignOffRootsOnlyRecord => "sign_off_roots_only_record",
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
    pub registry_height: u64,
    pub min_admission_rules: u64,
    pub min_operator_commands: u64,
    pub max_public_raw_records: u64,
    pub wave92_future_slot_registry_root: String,
    pub wave92_runtime_replay_slot_root: String,
    pub fail_closed_armed: bool,
    pub heavy_gates_ran: bool,
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
            registry_height: DEFAULT_REGISTRY_HEIGHT,
            min_admission_rules: DEFAULT_MIN_ADMISSION_RULES,
            min_operator_commands: DEFAULT_MIN_OPERATOR_COMMANDS,
            max_public_raw_records: DEFAULT_MAX_PUBLIC_RAW_RECORDS,
            wave92_future_slot_registry_root: sample_root("wave92-future-slot-registry"),
            wave92_runtime_replay_slot_root: sample_root("wave92-runtime-replay-slot"),
            fail_closed_armed: true,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("lane_suite", &self.lane_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("registry_height", self.registry_height)?;
        ensure_root(
            "wave92_future_slot_registry_root",
            &self.wave92_future_slot_registry_root,
        )?;
        ensure_root(
            "wave92_runtime_replay_slot_root",
            &self.wave92_runtime_replay_slot_root,
        )?;
        if self.source_wave >= self.wave {
            return Err("source wave must precede admission wave".to_string());
        }
        if !self.fail_closed_armed {
            return Err("admission quarantine fail closed guard is disarmed".to_string());
        }
        if self.heavy_gates_ran {
            return Err("runtime replay lane cannot claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_receipt_admission_quarantine_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "registry_height": self.registry_height,
            "min_admission_rules": self.min_admission_rules,
            "min_operator_commands": self.min_operator_commands,
            "max_public_raw_records": self.max_public_raw_records,
            "wave92_future_slot_registry_root": self.wave92_future_slot_registry_root,
            "wave92_runtime_replay_slot_root": self.wave92_runtime_replay_slot_root,
            "fail_closed_armed": self.fail_closed_armed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE93-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionRule {
    pub rule_kind: AdmissionRuleKind,
    pub lane: Option<ReplayReceiptLane>,
    pub required: bool,
    pub quarantine_on_miss: bool,
    pub rule_root: String,
}

impl AdmissionRule {
    pub fn new(rule_kind: AdmissionRuleKind, lane: Option<ReplayReceiptLane>) -> Self {
        let lane_text = match lane {
            Some(value) => value.as_str(),
            None => "all",
        };
        let rule_root = stable_id(
            "wave93-admission-rule",
            &format!("{}:{}", rule_kind.as_str(), lane_text),
        );
        Self {
            rule_kind,
            lane,
            required: true,
            quarantine_on_miss: true,
            rule_root,
        }
    }

    pub fn canonical_rules() -> Vec<Self> {
        let mut rules = AdmissionRuleKind::all()
            .into_iter()
            .map(|kind| Self::new(kind, None))
            .collect::<Vec<_>>();
        for lane in ReplayReceiptLane::all() {
            rules.push(Self::new(AdmissionRuleKind::SourceSlotBound, Some(lane)));
            rules.push(Self::new(
                AdmissionRuleKind::OperatorCommandBound,
                Some(lane),
            ));
        }
        rules
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("rule_root", &self.rule_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_admission_rule",
            "rule_kind": self.rule_kind.as_str(),
            "lane": self.lane.map(ReplayReceiptLane::as_str),
            "required": self.required,
            "quarantine_on_miss": self.quarantine_on_miss,
            "rule_root": self.rule_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptRootEntry {
    pub lane: ReplayReceiptLane,
    pub status: ReceiptRootStatus,
    pub source_wave: u64,
    pub slot_root: String,
    pub future_receipt_root: String,
    pub admission_root: String,
    pub quarantine_root: String,
    pub replacement_root: Option<String>,
    pub roots_only: bool,
    pub heavy_gate_claimed: bool,
}

impl ReceiptRootEntry {
    pub fn empty_quarantined(lane: ReplayReceiptLane, config: &Config) -> Self {
        let slot_root = stable_id(
            "wave93-source-slot-root",
            &format!(
                "{}:{}",
                config.wave92_runtime_replay_slot_root,
                lane.as_str()
            ),
        );
        let future_receipt_root = empty_root(lane.as_str());
        let quarantine_root = stable_id(
            "wave93-quarantine-root",
            &format!("{}:{}:empty", lane.as_str(), slot_root),
        );
        let admission_root = stable_id(
            "wave93-admission-root",
            &format!("{}:{}:held", lane.as_str(), quarantine_root),
        );
        Self {
            lane,
            status: ReceiptRootStatus::EmptyQuarantined,
            source_wave: config.source_wave,
            slot_root,
            future_receipt_root,
            admission_root,
            quarantine_root,
            replacement_root: None,
            roots_only: true,
            heavy_gate_claimed: false,
        }
    }

    pub fn admit_future_root(&mut self, future_receipt_root: &str, config: &Config) -> Result<()> {
        ensure_root("future_receipt_root", future_receipt_root)?;
        if self.heavy_gate_claimed {
            return Err("receipt root entry claims heavy gates ran".to_string());
        }
        if self.source_wave != config.source_wave {
            return Err("receipt root source wave does not match config".to_string());
        }
        self.future_receipt_root = future_receipt_root.to_string();
        self.status = ReceiptRootStatus::FutureRootAdmitted;
        self.admission_root = stable_id(
            "wave93-admitted-future-root",
            &format!(
                "{}:{}:{}",
                self.lane.as_str(),
                self.slot_root,
                self.future_receipt_root
            ),
        );
        self.quarantine_root = stable_id(
            "wave93-quarantine-cleared-root",
            &format!("{}:{}", self.lane.as_str(), self.admission_root),
        );
        Ok(())
    }

    pub fn replace_stale_archive_root(&mut self, fresh_root: &str) -> Result<()> {
        ensure_root("fresh_archive_replacement_root", fresh_root)?;
        if self.lane != ReplayReceiptLane::StaleArchiveReplacement {
            return Err("archive replacement is only valid for stale archive lane".to_string());
        }
        self.replacement_root = Some(fresh_root.to_string());
        self.status = ReceiptRootStatus::ReplacedByFreshRoot;
        self.admission_root = stable_id(
            "wave93-stale-archive-replacement",
            &format!("{}:{}", self.slot_root, fresh_root),
        );
        self.quarantine_root = stable_id(
            "wave93-stale-archive-replacement-quarantine-cleared",
            &self.admission_root,
        );
        Ok(())
    }

    pub fn quarantine(&mut self, reason: QuarantineReasonKind) {
        self.status = ReceiptRootStatus::Quarantined;
        self.quarantine_root = stable_id(
            "wave93-quarantined-root",
            &format!(
                "{}:{}:{}",
                self.lane.as_str(),
                self.slot_root,
                reason.as_str()
            ),
        );
        self.admission_root = stable_id(
            "wave93-admission-held-root",
            &format!("{}:{}", self.lane.as_str(), self.quarantine_root),
        );
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_receipt_root_entry",
            "lane": self.lane.as_str(),
            "status": self.status.as_str(),
            "source_wave": self.source_wave,
            "slot_root": self.slot_root,
            "future_receipt_root": self.future_receipt_root,
            "admission_root": self.admission_root,
            "quarantine_root": self.quarantine_root,
            "replacement_root": self.replacement_root,
            "roots_only": self.roots_only,
            "heavy_gate_claimed": self.heavy_gate_claimed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE93-RECEIPT-ROOT-ENTRY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuarantineReason {
    pub lane: ReplayReceiptLane,
    pub reason_kind: QuarantineReasonKind,
    pub subject_root: String,
    pub reason_root: String,
}

impl QuarantineReason {
    pub fn new(
        lane: ReplayReceiptLane,
        reason_kind: QuarantineReasonKind,
        subject_root: &str,
    ) -> Self {
        Self {
            lane,
            reason_kind,
            subject_root: subject_root.to_string(),
            reason_root: stable_id(
                "wave93-quarantine-reason",
                &format!(
                    "{}:{}:{}",
                    lane.as_str(),
                    reason_kind.as_str(),
                    subject_root
                ),
            ),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_quarantine_reason",
            "lane": self.lane.as_str(),
            "reason_kind": self.reason_kind.as_str(),
            "subject_root": self.subject_root,
            "reason_root": self.reason_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommand {
    pub command_kind: OperatorCommandKind,
    pub lane: Option<ReplayReceiptLane>,
    pub command_root: String,
    pub allowed_while_fail_closed: bool,
    pub roots_only: bool,
}

impl OperatorCommand {
    pub fn new(command_kind: OperatorCommandKind, lane: Option<ReplayReceiptLane>) -> Self {
        let lane_text = match lane {
            Some(value) => value.as_str(),
            None => "all",
        };
        Self {
            command_kind,
            lane,
            command_root: stable_id(
                "wave93-operator-command",
                &format!("{}:{}", command_kind.as_str(), lane_text),
            ),
            allowed_while_fail_closed: true,
            roots_only: true,
        }
    }

    pub fn canonical_commands() -> Vec<Self> {
        let mut commands = OperatorCommandKind::all()
            .into_iter()
            .map(|kind| Self::new(kind, None))
            .collect::<Vec<_>>();
        for lane in ReplayReceiptLane::all() {
            commands.push(Self::new(OperatorCommandKind::HoldReceiptRoot, Some(lane)));
            commands.push(Self::new(OperatorCommandKind::QuarantineRoot, Some(lane)));
        }
        commands
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("command_root", &self.command_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_operator_command",
            "command_kind": self.command_kind.as_str(),
            "lane": self.lane.map(ReplayReceiptLane::as_str),
            "command_root": self.command_root,
            "allowed_while_fail_closed": self.allowed_while_fail_closed,
            "roots_only": self.roots_only,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeCounters {
    pub lanes_total: u64,
    pub future_roots_admitted: u64,
    pub quarantined_roots: u64,
    pub admission_rules: u64,
    pub operator_commands: u64,
    pub quarantine_reasons: u64,
    pub public_raw_records: u64,
}

impl RuntimeCounters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_runtime_replay_lane_counters",
            "lanes_total": self.lanes_total,
            "future_roots_admitted": self.future_roots_admitted,
            "quarantined_roots": self.quarantined_roots,
            "admission_rules": self.admission_rules,
            "operator_commands": self.operator_commands,
            "quarantine_reasons": self.quarantine_reasons,
            "public_raw_records": self.public_raw_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionVerdict {
    pub verdict_root: String,
    pub fail_closed: bool,
    pub lane_clearable: bool,
    pub receipt_live: bool,
    pub heavy_gates_ran: bool,
    pub quarantine_root: String,
    pub admission_root: String,
}

impl AdmissionVerdict {
    pub fn build(state: &State) -> Self {
        let quarantine_root = state.quarantine_root();
        let admission_root = state.admission_root();
        let lane_clearable = false;
        let fail_closed = true;
        Self {
            verdict_root: stable_id(
                "wave93-admission-verdict",
                &format!("{}:{}", quarantine_root, admission_root),
            ),
            fail_closed,
            lane_clearable,
            receipt_live: false,
            heavy_gates_ran: false,
            quarantine_root,
            admission_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_admission_verdict",
            "verdict_root": self.verdict_root,
            "fail_closed": self.fail_closed,
            "lane_clearable": self.lane_clearable,
            "receipt_live": self.receipt_live,
            "heavy_gates_ran": self.heavy_gates_ran,
            "quarantine_root": self.quarantine_root,
            "admission_root": self.admission_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub receipt_roots: BTreeMap<ReplayReceiptLane, ReceiptRootEntry>,
    pub admission_rules: Vec<AdmissionRule>,
    pub quarantine_reasons: Vec<QuarantineReason>,
    pub operator_commands: Vec<OperatorCommand>,
    pub counters: RuntimeCounters,
}

impl Default for State {
    fn default() -> Self {
        devnet()
    }
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let mut receipt_roots = BTreeMap::new();
        for lane in ReplayReceiptLane::all() {
            receipt_roots.insert(lane, ReceiptRootEntry::empty_quarantined(lane, &config));
        }
        let mut state = Self {
            config,
            receipt_roots,
            admission_rules: AdmissionRule::canonical_rules(),
            quarantine_reasons: Vec::new(),
            operator_commands: OperatorCommand::canonical_commands(),
            counters: RuntimeCounters::default(),
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn admit_future_receipt_root(
        &mut self,
        lane: ReplayReceiptLane,
        future_receipt_root: &str,
    ) -> Result<String> {
        self.validate_admission(lane, future_receipt_root)?;
        let entry = self
            .receipt_roots
            .get_mut(&lane)
            .ok_or_else(|| "receipt lane is not registered".to_string())?;
        entry.admit_future_root(future_receipt_root, &self.config)?;
        let root = entry.state_root();
        self.recompute();
        self.validate()?;
        Ok(root)
    }

    pub fn quarantine_lane(
        &mut self,
        lane: ReplayReceiptLane,
        reason_kind: QuarantineReasonKind,
    ) -> Result<String> {
        let entry = self
            .receipt_roots
            .get_mut(&lane)
            .ok_or_else(|| "receipt lane is not registered".to_string())?;
        entry.quarantine(reason_kind);
        let reason = QuarantineReason::new(lane, reason_kind, &entry.quarantine_root);
        let root = reason.reason_root.clone();
        self.quarantine_reasons.push(reason);
        self.recompute();
        self.validate()?;
        Ok(root)
    }

    pub fn replace_stale_archive_root(&mut self, fresh_root: &str) -> Result<String> {
        let lane = ReplayReceiptLane::StaleArchiveReplacement;
        self.validate_admission(lane, fresh_root)?;
        let entry = self
            .receipt_roots
            .get_mut(&lane)
            .ok_or_else(|| "receipt lane is not registered".to_string())?;
        entry.replace_stale_archive_root(fresh_root)?;
        let root = entry.state_root();
        self.recompute();
        self.validate()?;
        Ok(root)
    }

    pub fn validate_admission(
        &self,
        lane: ReplayReceiptLane,
        future_receipt_root: &str,
    ) -> Result<()> {
        ensure_root("future_receipt_root", future_receipt_root)?;
        if future_receipt_root == empty_root(lane.as_str()) {
            return Err("empty future receipt root remains quarantined".to_string());
        }
        if self.future_root_seen(future_receipt_root) {
            return Err("future receipt root is already present".to_string());
        }
        if self.config.heavy_gates_ran {
            return Err("admission cannot claim heavy gates ran".to_string());
        }
        let entry = self
            .receipt_roots
            .get(&lane)
            .ok_or_else(|| "receipt lane is not registered".to_string())?;
        if entry.source_wave != self.config.source_wave {
            return Err("source wave binding is not valid".to_string());
        }
        if !entry.roots_only {
            return Err("receipt entry is not roots only".to_string());
        }
        Ok(())
    }

    pub fn future_root_seen(&self, future_receipt_root: &str) -> bool {
        self.receipt_roots
            .values()
            .any(|entry| entry.future_receipt_root == future_receipt_root)
    }

    pub fn recompute(&mut self) {
        self.quarantine_reasons = self.compute_quarantine_reasons();
        self.counters = self.compute_counters();
    }

    pub fn compute_quarantine_reasons(&self) -> Vec<QuarantineReason> {
        let mut reasons = Vec::new();
        let mut seen_roots = BTreeSet::new();
        for entry in self.receipt_roots.values() {
            if !entry.roots_only {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::NonRootMaterial,
                    &entry.state_root(),
                ));
            }
            if entry.heavy_gate_claimed {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::HeavyGateClaimPresent,
                    &entry.state_root(),
                ));
            }
            if entry.source_wave != self.config.source_wave {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::SourceWaveMismatch,
                    &entry.state_root(),
                ));
            }
            if entry.slot_root.trim().is_empty() {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::SlotBindingMissing,
                    &entry.state_root(),
                ));
            }
            if entry.future_receipt_root == empty_root(entry.lane.as_str()) {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::AwaitingFutureReceipt,
                    &entry.quarantine_root,
                ));
            }
            if entry.status == ReceiptRootStatus::EmptyQuarantined {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::EmptyFutureRoot,
                    &entry.quarantine_root,
                ));
            }
            if entry.status == ReceiptRootStatus::Quarantined {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::AdmissionRuleMissing,
                    &entry.quarantine_root,
                ));
            }
            if entry.lane == ReplayReceiptLane::StaleArchiveReplacement
                && entry.status == ReceiptRootStatus::FutureRootAdmitted
                && entry.replacement_root.is_none()
            {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::StaleArchiveNeedsReplacement,
                    &entry.state_root(),
                ));
            }
            if !seen_roots.insert(entry.future_receipt_root.clone()) {
                reasons.push(QuarantineReason::new(
                    entry.lane,
                    QuarantineReasonKind::DuplicateRoot,
                    &entry.state_root(),
                ));
            }
        }
        if self.operator_commands.len() < self.config.min_operator_commands as usize {
            reasons.push(QuarantineReason::new(
                ReplayReceiptLane::OperatorSignoff,
                QuarantineReasonKind::OperatorCommandMissing,
                &self.operator_command_root(),
            ));
        }
        if self.admission_rules.len() < self.config.min_admission_rules as usize {
            reasons.push(QuarantineReason::new(
                ReplayReceiptLane::ReplayRun,
                QuarantineReasonKind::AdmissionRuleMissing,
                &self.admission_rule_root(),
            ));
        }
        reasons
    }

    pub fn compute_counters(&self) -> RuntimeCounters {
        RuntimeCounters {
            lanes_total: self.receipt_roots.len() as u64,
            future_roots_admitted: self
                .receipt_roots
                .values()
                .filter(|entry| entry.status.is_admitted())
                .count() as u64,
            quarantined_roots: self
                .receipt_roots
                .values()
                .filter(|entry| !entry.status.is_admitted())
                .count() as u64,
            admission_rules: self.admission_rules.len() as u64,
            operator_commands: self.operator_commands.len() as u64,
            quarantine_reasons: self.quarantine_reasons.len() as u64,
            public_raw_records: 0,
        }
    }

    pub fn receipt_root(&self) -> String {
        collection_root(
            "WAVE93-RECEIPT-ROOTS",
            self.receipt_roots
                .values()
                .map(ReceiptRootEntry::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn admission_rule_root(&self) -> String {
        collection_root(
            "WAVE93-ADMISSION-RULES",
            self.admission_rules
                .iter()
                .map(AdmissionRule::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn quarantine_root(&self) -> String {
        collection_root(
            "WAVE93-QUARANTINE-REASONS",
            self.quarantine_reasons
                .iter()
                .map(QuarantineReason::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn operator_command_root(&self) -> String {
        collection_root(
            "WAVE93-OPERATOR-COMMANDS",
            self.operator_commands
                .iter()
                .map(OperatorCommand::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn admission_root(&self) -> String {
        value_root(
            "WAVE93-ADMISSION-ROOT",
            &json!({
                "config_root": self.config.state_root(),
                "receipt_root": self.receipt_root(),
                "admission_rule_root": self.admission_rule_root(),
                "quarantine_root": self.quarantine_root(),
                "operator_command_root": self.operator_command_root(),
                "counters": self.counters.public_record(),
            }),
        )
    }

    pub fn verdict(&self) -> AdmissionVerdict {
        AdmissionVerdict::build(self)
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE93-STATE",
            &json!({
                "config_root": self.config.state_root(),
                "receipt_root": self.receipt_root(),
                "admission_root": self.admission_root(),
                "quarantine_root": self.quarantine_root(),
                "operator_command_root": self.operator_command_root(),
                "verdict_root": self.verdict().verdict_root,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        for rule in &self.admission_rules {
            rule.validate()?;
        }
        for entry in self.receipt_roots.values() {
            ensure_root("receipt_entry_root", &entry.state_root())?;
        }
        for command in &self.operator_commands {
            command.validate()?;
        }
        if self.counters.public_raw_records > self.config.max_public_raw_records {
            return Err("public record contains raw material".to_string());
        }
        if self.verdict().heavy_gates_ran {
            return Err("verdict cannot claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave93_receipt_admission_quarantine_runtime_replay_lane_state",
            "config": self.config.public_record(),
            "receipt_root": self.receipt_root(),
            "admission_rule_root": self.admission_rule_root(),
            "quarantine_root": self.quarantine_root(),
            "operator_command_root": self.operator_command_root(),
            "admission_root": self.admission_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict().public_record(),
            "receipt_entries": self.receipt_roots.values().map(ReceiptRootEntry::public_record).collect::<Vec<_>>(),
            "admission_rules": self.admission_rules.iter().map(AdmissionRule::public_record).collect::<Vec<_>>(),
            "quarantine_reasons": self.quarantine_reasons.iter().map(QuarantineReason::public_record).collect::<Vec<_>>(),
            "operator_commands": self.operator_commands.iter().map(OperatorCommand::public_record).collect::<Vec<_>>(),
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
    let mut receipt_roots = BTreeMap::new();
    for lane in ReplayReceiptLane::all() {
        let mut entry = ReceiptRootEntry::empty_quarantined(lane, &config);
        entry.quarantine(QuarantineReasonKind::AdmissionRuleMissing);
        receipt_roots.insert(lane, entry);
    }
    let mut state = State {
        config,
        receipt_roots,
        admission_rules: AdmissionRule::canonical_rules(),
        quarantine_reasons: vec![QuarantineReason::new(
            ReplayReceiptLane::ReplayRun,
            QuarantineReasonKind::AdmissionRuleMissing,
            &stable_id("wave93-fallback-error-root", &error),
        )],
        operator_commands: OperatorCommand::canonical_commands(),
        counters: RuntimeCounters::default(),
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

fn sample_root(label: &str) -> String {
    stable_id("wave93-sample-root", label)
}

fn empty_root(label: &str) -> String {
    stable_id("wave93-empty-root", label)
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
