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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave94-live-heavy-gate-receipt-fill-staging-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const STAGING_SUITE: &str = "wave94-wave93-admitted-root-to-wave92-slot-fill-staging-v1";
pub const DEFAULT_WAVE: u64 = 94;
pub const DEFAULT_SOURCE_WAVE: u64 = 93;
pub const DEFAULT_TARGET_WAVE: u64 = 92;
pub const DEFAULT_HEIGHT: u64 = 4_281_712;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneSlotKind {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
}

impl LaneSlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialScenario => "adversarial_scenario",
            Self::ThreatModel => "threat_model",
            Self::PrivacyReview => "privacy_review",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditReview,
            Self::AdversarialScenario,
            Self::ThreatModel,
            Self::PrivacyReview,
            Self::ReviewerSignoff,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StagingStatus {
    BlockedEmpty,
    StagedAttempt,
    RejectedAttempt,
}

impl StagingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockedEmpty => "blocked_empty",
            Self::StagedAttempt => "staged_attempt",
            Self::RejectedAttempt => "rejected_attempt",
        }
    }

    pub fn is_staged(self) -> bool {
        self == Self::StagedAttempt
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    EmptyStagedFill,
    AdmissionRootMissing,
    AdmissionRootNotBound,
    TargetSlotRootMissing,
    FillAttemptNotStaged,
    RawPayloadPresent,
    FailClosedDisarmed,
    ProductionDenied,
    HeavyGateRunClaimed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStagedFill => "empty_staged_fill",
            Self::AdmissionRootMissing => "admission_root_missing",
            Self::AdmissionRootNotBound => "admission_root_not_bound",
            Self::TargetSlotRootMissing => "target_slot_root_missing",
            Self::FillAttemptNotStaged => "fill_attempt_not_staged",
            Self::RawPayloadPresent => "raw_payload_present",
            Self::FailClosedDisarmed => "fail_closed_disarmed",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateRunClaimed => "heavy_gate_run_claimed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandKind {
    HoldProduction,
    ImportWave93AdmissionRoot,
    BindAdmissionToWave92Slot,
    StageSlotFillAttempt,
    ReviewFillBlockers,
    KeepFailClosed,
}

impl CommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldProduction => "hold_production",
            Self::ImportWave93AdmissionRoot => "import_wave93_admission_root",
            Self::BindAdmissionToWave92Slot => "bind_admission_to_wave92_slot",
            Self::StageSlotFillAttempt => "stage_slot_fill_attempt",
            Self::ReviewFillBlockers => "review_fill_blockers",
            Self::KeepFailClosed => "keep_fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub staging_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub target_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub max_raw_payload_records: u64,
    pub source_admission_root: String,
    pub source_quarantine_root: String,
    pub source_command_root: String,
    pub target_slot_registry_root: String,
    pub target_blocker_root: String,
    pub fail_closed_armed: bool,
    pub production_allowed: bool,
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
            staging_suite: STAGING_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            target_wave: DEFAULT_TARGET_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            source_admission_root: deterministic_root("wave93-admission-root"),
            source_quarantine_root: deterministic_root("wave93-quarantine-root"),
            source_command_root: deterministic_root("wave93-command-root"),
            target_slot_registry_root: deterministic_root("wave92-slot-registry-root"),
            target_blocker_root: deterministic_root("wave92-blocker-root"),
            fail_closed_armed: true,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("staging_suite", &self.staging_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("target_wave", self.target_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_root("source_admission_root", &self.source_admission_root)?;
        ensure_root("source_quarantine_root", &self.source_quarantine_root)?;
        ensure_root("source_command_root", &self.source_command_root)?;
        ensure_root("target_slot_registry_root", &self.target_slot_registry_root)?;
        ensure_root("target_blocker_root", &self.target_blocker_root)?;
        if !self.fail_closed_armed {
            return Err("staging lane fail closed is not armed".to_string());
        }
        if self.production_allowed {
            return Err("wave94 staging lane denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave94 staging lane cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_receipt_fill_staging_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "staging_suite": self.staging_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "target_wave": self.target_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "max_raw_payload_records": self.max_raw_payload_records,
            "source_admission_root": self.source_admission_root,
            "source_quarantine_root": self.source_quarantine_root,
            "source_command_root": self.source_command_root,
            "target_slot_registry_root": self.target_slot_registry_root,
            "target_blocker_root": self.target_blocker_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE94-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmittedRootBinding {
    pub slot_kind: LaneSlotKind,
    pub wave93_admission_root: String,
    pub admitted_receipt_root: String,
    pub wave92_slot_root: String,
    pub binding_root: String,
}

impl AdmittedRootBinding {
    pub fn empty(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let mut binding = Self {
            slot_kind,
            wave93_admission_root: config.source_admission_root.clone(),
            admitted_receipt_root: empty_root(&format!("{}-admitted", slot_kind.as_str())),
            wave92_slot_root: empty_root(&format!("{}-wave92-slot", slot_kind.as_str())),
            binding_root: String::new(),
        };
        binding.binding_root = binding.compute_root();
        binding
    }

    pub fn from_roots(
        slot_kind: LaneSlotKind,
        wave93_admission_root: &str,
        admitted_receipt_root: &str,
        wave92_slot_root: &str,
    ) -> Result<Self> {
        ensure_root("wave93_admission_root", wave93_admission_root)?;
        ensure_root("admitted_receipt_root", admitted_receipt_root)?;
        ensure_root("wave92_slot_root", wave92_slot_root)?;
        let mut binding = Self {
            slot_kind,
            wave93_admission_root: wave93_admission_root.to_string(),
            admitted_receipt_root: admitted_receipt_root.to_string(),
            wave92_slot_root: wave92_slot_root.to_string(),
            binding_root: String::new(),
        };
        binding.binding_root = binding.compute_root();
        Ok(binding)
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE94-ADMITTED-ROOT-BINDING",
            &json!({
                "slot_kind": self.slot_kind.as_str(),
                "wave93_admission_root": self.wave93_admission_root,
                "admitted_receipt_root": self.admitted_receipt_root,
                "wave92_slot_root": self.wave92_slot_root,
            }),
        )
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("wave93_admission_root", &self.wave93_admission_root)?;
        ensure_root("admitted_receipt_root", &self.admitted_receipt_root)?;
        ensure_root("wave92_slot_root", &self.wave92_slot_root)?;
        ensure_root("binding_root", &self.binding_root)?;
        if self.wave93_admission_root != config.source_admission_root {
            return Err("admission root is not bound to configured source".to_string());
        }
        if self.binding_root != self.compute_root() {
            return Err("binding root does not match roots".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave93_admission_root": self.wave93_admission_root,
            "admitted_receipt_root": self.admitted_receipt_root,
            "wave92_slot_root": self.wave92_slot_root,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotFillAttempt {
    pub attempt_id: String,
    pub slot_kind: LaneSlotKind,
    pub status: StagingStatus,
    pub binding_root: String,
    pub target_slot_root: String,
    pub fill_attempt_root: String,
}

impl SlotFillAttempt {
    pub fn blocked(slot_kind: LaneSlotKind, binding: &AdmittedRootBinding) -> Self {
        let mut attempt = Self {
            attempt_id: stable_id("wave94-slot-fill-attempt", slot_kind.as_str()),
            slot_kind,
            status: StagingStatus::BlockedEmpty,
            binding_root: binding.binding_root.clone(),
            target_slot_root: binding.wave92_slot_root.clone(),
            fill_attempt_root: String::new(),
        };
        attempt.fill_attempt_root = attempt.compute_root();
        attempt
    }

    pub fn staged(slot_kind: LaneSlotKind, binding: &AdmittedRootBinding) -> Self {
        let mut attempt = Self {
            attempt_id: stable_id(
                "wave94-slot-fill-attempt",
                &format!("{}:{}", slot_kind.as_str(), binding.binding_root),
            ),
            slot_kind,
            status: StagingStatus::StagedAttempt,
            binding_root: binding.binding_root.clone(),
            target_slot_root: binding.wave92_slot_root.clone(),
            fill_attempt_root: String::new(),
        };
        attempt.fill_attempt_root = attempt.compute_root();
        attempt
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE94-SLOT-FILL-ATTEMPT",
            &json!({
                "attempt_id": self.attempt_id,
                "slot_kind": self.slot_kind.as_str(),
                "status": self.status.as_str(),
                "binding_root": self.binding_root,
                "target_slot_root": self.target_slot_root,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("attempt_id", &self.attempt_id)?;
        ensure_root("binding_root", &self.binding_root)?;
        ensure_root("target_slot_root", &self.target_slot_root)?;
        ensure_root("fill_attempt_root", &self.fill_attempt_root)?;
        if self.fill_attempt_root != self.compute_root() {
            return Err("fill attempt root does not match attempt".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "attempt_id": self.attempt_id,
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "binding_root": self.binding_root,
            "target_slot_root": self.target_slot_root,
            "fill_attempt_root": self.fill_attempt_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotFillBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocking: bool,
    pub blocker_root: String,
}

impl SlotFillBlocker {
    pub fn new(kind: BlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        let blocker_id = stable_id(
            "wave94-slot-fill-blocker",
            &format!("{}:{}:{}", kind.as_str(), slot_kind.as_str(), evidence_root),
        );
        let mut blocker = Self {
            blocker_id,
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocking: true,
            blocker_root: String::new(),
        };
        blocker.blocker_root = blocker.compute_root();
        blocker
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE94-SLOT-FILL-BLOCKER",
            &json!({
                "blocker_id": self.blocker_id,
                "kind": self.kind.as_str(),
                "slot_kind": self.slot_kind.as_str(),
                "evidence_root": self.evidence_root,
                "blocking": self.blocking,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        if self.blocker_root != self.compute_root() {
            return Err("blocker root does not match blocker".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "blocking": self.blocking,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub command_id: String,
    pub kind: CommandKind,
    pub target_root: String,
    pub fail_closed_preserving: bool,
    pub command_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandKind, target_root: &str) -> Self {
        let command_id = stable_id(
            "wave94-command-hint",
            &format!("{}:{}", kind.as_str(), target_root),
        );
        let mut command = Self {
            command_id,
            kind,
            target_root: target_root.to_string(),
            fail_closed_preserving: true,
            command_root: String::new(),
        };
        command.command_root = command.compute_root();
        command
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandKind::HoldProduction,
                &config.target_slot_registry_root,
            ),
            Self::new(
                CommandKind::ImportWave93AdmissionRoot,
                &config.source_admission_root,
            ),
            Self::new(
                CommandKind::BindAdmissionToWave92Slot,
                &config.target_slot_registry_root,
            ),
            Self::new(
                CommandKind::StageSlotFillAttempt,
                &config.target_blocker_root,
            ),
            Self::new(CommandKind::ReviewFillBlockers, &config.target_blocker_root),
            Self::new(CommandKind::KeepFailClosed, &config.source_quarantine_root),
        ]
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE94-COMMAND-HINT",
            &json!({
                "command_id": self.command_id,
                "kind": self.kind.as_str(),
                "target_root": self.target_root,
                "fail_closed_preserving": self.fail_closed_preserving,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("command_id", &self.command_id)?;
        ensure_root("target_root", &self.target_root)?;
        ensure_root("command_root", &self.command_root)?;
        if self.command_root != self.compute_root() {
            return Err("command root does not match hint".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "command_id": self.command_id,
            "kind": self.kind.as_str(),
            "target_root": self.target_root,
            "fail_closed_preserving": self.fail_closed_preserving,
            "command_root": self.command_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub slot_count: u64,
    pub staged_fills: u64,
    pub blocked_slots: u64,
    pub rejected_fills: u64,
    pub blockers: u64,
    pub command_hints: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_count": self.slot_count,
            "staged_fills": self.staged_fills,
            "blocked_slots": self.blocked_slots,
            "rejected_fills": self.rejected_fills,
            "blockers": self.blockers,
            "command_hints": self.command_hints,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub bindings: Vec<AdmittedRootBinding>,
    pub fill_attempts: Vec<SlotFillAttempt>,
    pub blockers: Vec<SlotFillBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let bindings = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| AdmittedRootBinding::empty(slot_kind, &config))
            .collect::<Vec<_>>();
        let fill_attempts = bindings
            .iter()
            .map(|binding| SlotFillAttempt::blocked(binding.slot_kind, binding))
            .collect::<Vec<_>>();
        let mut state = Self {
            command_hints: CommandHint::canonical(&config),
            blockers: Vec::new(),
            counters: Counters::default(),
            fill_attempts,
            bindings,
            config,
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn stage_admitted_root(
        &mut self,
        slot_kind: LaneSlotKind,
        admitted_receipt_root: &str,
        wave92_slot_root: &str,
    ) -> Result<String> {
        ensure_root("admitted_receipt_root", admitted_receipt_root)?;
        ensure_root("wave92_slot_root", wave92_slot_root)?;
        let binding = AdmittedRootBinding::from_roots(
            slot_kind,
            &self.config.source_admission_root,
            admitted_receipt_root,
            wave92_slot_root,
        )?;
        for stored in &mut self.bindings {
            if stored.slot_kind == slot_kind {
                *stored = binding.clone();
            }
        }
        for attempt in &mut self.fill_attempts {
            if attempt.slot_kind == slot_kind {
                *attempt = SlotFillAttempt::staged(slot_kind, &binding);
            }
        }
        self.recompute();
        self.validate()?;
        Ok(self.fill_attempt_root())
    }

    pub fn recompute(&mut self) {
        self.blockers = self.compute_blockers();
        self.counters = self.compute_counters();
    }

    pub fn compute_blockers(&self) -> Vec<SlotFillBlocker> {
        let mut blockers = Vec::new();
        for binding in &self.bindings {
            if binding.admitted_receipt_root
                == empty_root(&format!("{}-admitted", binding.slot_kind.as_str()))
            {
                blockers.push(SlotFillBlocker::new(
                    BlockerKind::AdmissionRootMissing,
                    binding.slot_kind,
                    &binding.binding_root,
                ));
            }
            if binding.wave93_admission_root != self.config.source_admission_root {
                blockers.push(SlotFillBlocker::new(
                    BlockerKind::AdmissionRootNotBound,
                    binding.slot_kind,
                    &binding.binding_root,
                ));
            }
            if binding.wave92_slot_root
                == empty_root(&format!("{}-wave92-slot", binding.slot_kind.as_str()))
            {
                blockers.push(SlotFillBlocker::new(
                    BlockerKind::TargetSlotRootMissing,
                    binding.slot_kind,
                    &binding.binding_root,
                ));
            }
        }
        for attempt in &self.fill_attempts {
            if !attempt.status.is_staged() {
                blockers.push(SlotFillBlocker::new(
                    BlockerKind::EmptyStagedFill,
                    attempt.slot_kind,
                    &attempt.fill_attempt_root,
                ));
                blockers.push(SlotFillBlocker::new(
                    BlockerKind::FillAttemptNotStaged,
                    attempt.slot_kind,
                    &attempt.fill_attempt_root,
                ));
            }
        }
        if !self.config.fail_closed_armed {
            blockers.push(SlotFillBlocker::new(
                BlockerKind::FailClosedDisarmed,
                LaneSlotKind::AuditReview,
                &self.config.state_root(),
            ));
        }
        if !self.config.production_allowed {
            blockers.push(SlotFillBlocker::new(
                BlockerKind::ProductionDenied,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if self.config.heavy_gates_ran {
            blockers.push(SlotFillBlocker::new(
                BlockerKind::HeavyGateRunClaimed,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            blockers.push(SlotFillBlocker::new(
                BlockerKind::RawPayloadPresent,
                LaneSlotKind::PrivacyReview,
                &self.state_material_root(),
            ));
        }
        blockers
    }

    pub fn compute_counters(&self) -> Counters {
        Counters {
            slot_count: self.fill_attempts.len() as u64,
            staged_fills: self
                .fill_attempts
                .iter()
                .filter(|attempt| attempt.status == StagingStatus::StagedAttempt)
                .count() as u64,
            blocked_slots: self
                .fill_attempts
                .iter()
                .filter(|attempt| attempt.status == StagingStatus::BlockedEmpty)
                .count() as u64,
            rejected_fills: self
                .fill_attempts
                .iter()
                .filter(|attempt| attempt.status == StagingStatus::RejectedAttempt)
                .count() as u64,
            blockers: self.blockers.len() as u64,
            command_hints: self.command_hints.len() as u64,
            raw_payload_records: 0,
        }
    }

    pub fn binding_root(&self) -> String {
        collection_root(
            "WAVE94-ADMITTED-ROOT-BINDINGS",
            self.bindings
                .iter()
                .map(AdmittedRootBinding::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn fill_attempt_root(&self) -> String {
        collection_root(
            "WAVE94-FILL-ATTEMPTS",
            self.fill_attempts
                .iter()
                .map(SlotFillAttempt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE94-FILL-BLOCKERS",
            self.blockers
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

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE94-STATE-MATERIAL",
            &json!({
                "config_root": self.config.state_root(),
                "binding_root": self.binding_root(),
                "fill_attempt_root": self.fill_attempt_root(),
                "command_hint_root": self.command_hint_root(),
                "counters": self.counters.public_record(),
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE94-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "blocker_root": self.blocker_root(),
                "fail_closed_armed": self.config.fail_closed_armed,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || !self.blockers.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.fill_attempts.len() as u64,
            self.config.min_slot_count,
        )?;
        for binding in &self.bindings {
            binding.validate(&self.config)?;
        }
        for attempt in &self.fill_attempts {
            attempt.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("staging lane contains raw payload records".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("staging counters do not match state".to_string());
        }
        if !self.production_denied() {
            return Err("staging lane cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave94_live_heavy_gate_receipt_fill_staging_audit_security_lane_state",
            "config": self.config.public_record(),
            "binding_root": self.binding_root(),
            "fill_attempt_root": self.fill_attempt_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "bindings": self.bindings.iter().map(AdmittedRootBinding::public_record).collect::<Vec<_>>(),
            "fill_attempts": self.fill_attempts.iter().map(SlotFillAttempt::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(SlotFillBlocker::public_record).collect::<Vec<_>>(),
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
    let bindings = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| AdmittedRootBinding::empty(slot_kind, &config))
        .collect::<Vec<_>>();
    let fill_attempts = bindings
        .iter()
        .map(|binding| SlotFillAttempt::blocked(binding.slot_kind, binding))
        .collect::<Vec<_>>();
    let mut state = State {
        command_hints: CommandHint::canonical(&config),
        blockers: vec![SlotFillBlocker::new(
            BlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root("WAVE94-FALLBACK-ERROR", &json!({"error": error})),
        )],
        counters: Counters::default(),
        fill_attempts,
        bindings,
        config,
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

fn ensure_min_count(field: &str, actual: u64, minimum: u64) -> Result<()> {
    if actual < minimum {
        return Err(format!("{} is below required minimum", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{} is not a canonical root", field));
    }
    Ok(())
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE94-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE94-DETERMINISTIC-ROOT",
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Str(label)],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE94-EMPTY-ROOT",
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Str(label)],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(
        domain,
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Json(value)],
        32,
    )
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    merkle_root(domain, &values)
}
