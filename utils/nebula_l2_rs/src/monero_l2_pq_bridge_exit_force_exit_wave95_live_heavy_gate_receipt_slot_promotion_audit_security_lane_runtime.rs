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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const PROMOTION_SUITE: &str =
    "wave95-wave94-staged-fill-to-wave92-slot-occupancy-promotion-gate-v1";
pub const DEFAULT_WAVE: u64 = 95;
pub const DEFAULT_SOURCE_WAVE: u64 = 94;
pub const DEFAULT_TARGET_WAVE: u64 = 92;
pub const DEFAULT_HEIGHT: u64 = 4_281_808;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;
pub const DEFAULT_MAX_PROMOTED_SLOTS: u64 = 0;

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
pub enum PromotionStatus {
    BlockedEmpty,
    BlockedStagedFill,
    Rejected,
    Promoted,
}

impl PromotionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockedEmpty => "blocked_empty",
            Self::BlockedStagedFill => "blocked_staged_fill",
            Self::Rejected => "rejected",
            Self::Promoted => "promoted",
        }
    }

    pub fn is_promoted(self) -> bool {
        self == Self::Promoted
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBlockerKind {
    StagedFillMissing,
    StagedFillNotBound,
    SlotOccupancyRootPlaceholder,
    PromotionSignoffMissing,
    OccupancyClaimDisallowed,
    PromotionBudgetZero,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl PromotionBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StagedFillMissing => "staged_fill_missing",
            Self::StagedFillNotBound => "staged_fill_not_bound",
            Self::SlotOccupancyRootPlaceholder => "slot_occupancy_root_placeholder",
            Self::PromotionSignoffMissing => "promotion_signoff_missing",
            Self::OccupancyClaimDisallowed => "occupancy_claim_disallowed",
            Self::PromotionBudgetZero => "promotion_budget_zero",
            Self::ProductionDenied => "production_denied",
            Self::FailClosedDisarmed => "fail_closed_disarmed",
            Self::RawPayloadPresent => "raw_payload_present",
            Self::HeavyGateRunClaimed => "heavy_gate_run_claimed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldPromotion,
    ImportWave94StagedFillRoot,
    BindStagedFillToSlotOccupancy,
    ReviewPromotionBlockers,
    AttachReviewerSignoffRoot,
    AttachOperatorSignoffRoot,
    KeepFailClosed,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::ImportWave94StagedFillRoot => "import_wave94_staged_fill_root",
            Self::BindStagedFillToSlotOccupancy => "bind_staged_fill_to_slot_occupancy",
            Self::ReviewPromotionBlockers => "review_promotion_blockers",
            Self::AttachReviewerSignoffRoot => "attach_reviewer_signoff_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
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
    pub promotion_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub target_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub max_raw_payload_records: u64,
    pub max_promoted_slots: u64,
    pub source_staged_fill_root: String,
    pub source_binding_root: String,
    pub source_blocker_root: String,
    pub target_slot_registry_root: String,
    pub slot_occupancy_root_placeholder: String,
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
            promotion_suite: PROMOTION_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            target_wave: DEFAULT_TARGET_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            max_promoted_slots: DEFAULT_MAX_PROMOTED_SLOTS,
            source_staged_fill_root: deterministic_root("wave94-staged-fill-root"),
            source_binding_root: deterministic_root("wave94-staged-fill-binding-root"),
            source_blocker_root: deterministic_root("wave94-staged-fill-blocker-root"),
            target_slot_registry_root: deterministic_root("wave92-slot-registry-root"),
            slot_occupancy_root_placeholder: empty_root("wave95-slot-occupancy-placeholder"),
            fail_closed_armed: true,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("promotion_suite", &self.promotion_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("target_wave", self.target_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_root("source_staged_fill_root", &self.source_staged_fill_root)?;
        ensure_root("source_binding_root", &self.source_binding_root)?;
        ensure_root("source_blocker_root", &self.source_blocker_root)?;
        ensure_root("target_slot_registry_root", &self.target_slot_registry_root)?;
        ensure_root(
            "slot_occupancy_root_placeholder",
            &self.slot_occupancy_root_placeholder,
        )?;
        if !self.fail_closed_armed {
            return Err("promotion lane fail closed is not armed".to_string());
        }
        if self.production_allowed {
            return Err("wave95 promotion lane denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave95 promotion lane cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave95_receipt_slot_promotion_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "promotion_suite": self.promotion_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "target_wave": self.target_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "max_raw_payload_records": self.max_raw_payload_records,
            "max_promoted_slots": self.max_promoted_slots,
            "source_staged_fill_root": self.source_staged_fill_root,
            "source_binding_root": self.source_binding_root,
            "source_blocker_root": self.source_blocker_root,
            "target_slot_registry_root": self.target_slot_registry_root,
            "slot_occupancy_root_placeholder": self.slot_occupancy_root_placeholder,
            "fail_closed_armed": self.fail_closed_armed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE95-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StagedFillBinding {
    pub slot_kind: LaneSlotKind,
    pub staged_fill_root: String,
    pub wave94_binding_root: String,
    pub target_slot_registry_root: String,
    pub binding_root: String,
}

impl StagedFillBinding {
    pub fn placeholder(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let mut binding = Self {
            slot_kind,
            staged_fill_root: empty_root(&format!("{}-staged-fill", slot_kind.as_str())),
            wave94_binding_root: config.source_binding_root.clone(),
            target_slot_registry_root: config.target_slot_registry_root.clone(),
            binding_root: String::new(),
        };
        binding.binding_root = binding.compute_root();
        binding
    }

    pub fn from_roots(
        slot_kind: LaneSlotKind,
        staged_fill_root: &str,
        wave94_binding_root: &str,
        target_slot_registry_root: &str,
    ) -> Result<Self> {
        ensure_root("staged_fill_root", staged_fill_root)?;
        ensure_root("wave94_binding_root", wave94_binding_root)?;
        ensure_root("target_slot_registry_root", target_slot_registry_root)?;
        let mut binding = Self {
            slot_kind,
            staged_fill_root: staged_fill_root.to_string(),
            wave94_binding_root: wave94_binding_root.to_string(),
            target_slot_registry_root: target_slot_registry_root.to_string(),
            binding_root: String::new(),
        };
        binding.binding_root = binding.compute_root();
        Ok(binding)
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE95-STAGED-FILL-BINDING",
            &json!({
                "slot_kind": self.slot_kind.as_str(),
                "staged_fill_root": self.staged_fill_root,
                "wave94_binding_root": self.wave94_binding_root,
                "target_slot_registry_root": self.target_slot_registry_root,
            }),
        )
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("staged_fill_root", &self.staged_fill_root)?;
        ensure_root("wave94_binding_root", &self.wave94_binding_root)?;
        ensure_root("target_slot_registry_root", &self.target_slot_registry_root)?;
        ensure_root("binding_root", &self.binding_root)?;
        if self.wave94_binding_root != config.source_binding_root {
            return Err("staged fill binding is not bound to configured wave94 root".to_string());
        }
        if self.target_slot_registry_root != config.target_slot_registry_root {
            return Err("staged fill binding is not bound to target slot registry".to_string());
        }
        if self.binding_root != self.compute_root() {
            return Err("binding root does not match staged fill binding".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "staged_fill_root": self.staged_fill_root,
            "wave94_binding_root": self.wave94_binding_root,
            "target_slot_registry_root": self.target_slot_registry_root,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub attempt_id: String,
    pub slot_kind: LaneSlotKind,
    pub status: PromotionStatus,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_root: String,
    pub reviewer_signoff_root: String,
    pub operator_signoff_root: String,
    pub attempt_root: String,
}

impl PromotionAttempt {
    pub fn blocked(slot_kind: LaneSlotKind, binding: &StagedFillBinding, config: &Config) -> Self {
        let mut attempt = Self {
            attempt_id: stable_id("wave95-promotion-attempt", slot_kind.as_str()),
            slot_kind,
            status: PromotionStatus::BlockedEmpty,
            staged_fill_binding_root: binding.binding_root.clone(),
            slot_occupancy_root: config.slot_occupancy_root_placeholder.clone(),
            reviewer_signoff_root: empty_root(&format!("{}-reviewer-signoff", slot_kind.as_str())),
            operator_signoff_root: empty_root(&format!("{}-operator-signoff", slot_kind.as_str())),
            attempt_root: String::new(),
        };
        attempt.attempt_root = attempt.compute_root();
        attempt
    }

    pub fn blocked_staged(
        slot_kind: LaneSlotKind,
        binding: &StagedFillBinding,
        slot_occupancy_root: &str,
    ) -> Result<Self> {
        ensure_root("slot_occupancy_root", slot_occupancy_root)?;
        let mut attempt = Self {
            attempt_id: stable_id(
                "wave95-promotion-attempt",
                &format!("{}:{}", slot_kind.as_str(), binding.binding_root),
            ),
            slot_kind,
            status: PromotionStatus::BlockedStagedFill,
            staged_fill_binding_root: binding.binding_root.clone(),
            slot_occupancy_root: slot_occupancy_root.to_string(),
            reviewer_signoff_root: empty_root(&format!("{}-reviewer-signoff", slot_kind.as_str())),
            operator_signoff_root: empty_root(&format!("{}-operator-signoff", slot_kind.as_str())),
            attempt_root: String::new(),
        };
        attempt.attempt_root = attempt.compute_root();
        Ok(attempt)
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE95-PROMOTION-ATTEMPT",
            &json!({
                "attempt_id": self.attempt_id,
                "slot_kind": self.slot_kind.as_str(),
                "status": self.status.as_str(),
                "staged_fill_binding_root": self.staged_fill_binding_root,
                "slot_occupancy_root": self.slot_occupancy_root,
                "reviewer_signoff_root": self.reviewer_signoff_root,
                "operator_signoff_root": self.operator_signoff_root,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("attempt_id", &self.attempt_id)?;
        ensure_root("staged_fill_binding_root", &self.staged_fill_binding_root)?;
        ensure_root("slot_occupancy_root", &self.slot_occupancy_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("attempt_root", &self.attempt_root)?;
        if self.attempt_root != self.compute_root() {
            return Err("promotion attempt root does not match attempt".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "attempt_id": self.attempt_id,
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_root": self.slot_occupancy_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_signoff_root": self.operator_signoff_root,
            "attempt_root": self.attempt_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionBlocker {
    pub blocker_id: String,
    pub kind: PromotionBlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub active: bool,
    pub blocker_root: String,
}

impl PromotionBlocker {
    pub fn new(kind: PromotionBlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        let blocker_id = stable_id(
            "wave95-promotion-blocker",
            &format!("{}:{}:{}", kind.as_str(), slot_kind.as_str(), evidence_root),
        );
        let mut blocker = Self {
            blocker_id,
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            active: true,
            blocker_root: String::new(),
        };
        blocker.blocker_root = blocker.compute_root();
        blocker
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE95-PROMOTION-BLOCKER",
            &json!({
                "blocker_id": self.blocker_id,
                "kind": self.kind.as_str(),
                "slot_kind": self.slot_kind.as_str(),
                "evidence_root": self.evidence_root,
                "active": self.active,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        if self.blocker_root != self.compute_root() {
            return Err("promotion blocker root does not match blocker".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "active": self.active,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub command_id: String,
    pub kind: CommandHintKind,
    pub target_root: String,
    pub fail_closed_preserving: bool,
    pub command_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_root: &str) -> Self {
        let command_id = stable_id(
            "wave95-command-hint",
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
            Self::new(CommandHintKind::HoldPromotion, &config.source_blocker_root),
            Self::new(
                CommandHintKind::ImportWave94StagedFillRoot,
                &config.source_staged_fill_root,
            ),
            Self::new(
                CommandHintKind::BindStagedFillToSlotOccupancy,
                &config.target_slot_registry_root,
            ),
            Self::new(
                CommandHintKind::ReviewPromotionBlockers,
                &config.source_blocker_root,
            ),
            Self::new(
                CommandHintKind::AttachReviewerSignoffRoot,
                &config.slot_occupancy_root_placeholder,
            ),
            Self::new(
                CommandHintKind::AttachOperatorSignoffRoot,
                &config.slot_occupancy_root_placeholder,
            ),
            Self::new(CommandHintKind::KeepFailClosed, &config.source_blocker_root),
        ]
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE95-COMMAND-HINT",
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
    pub promotion_attempts: u64,
    pub promoted_slots: u64,
    pub blocked_attempts: u64,
    pub rejected_attempts: u64,
    pub promotion_blockers: u64,
    pub command_hints: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_count": self.slot_count,
            "promotion_attempts": self.promotion_attempts,
            "promoted_slots": self.promoted_slots,
            "blocked_attempts": self.blocked_attempts,
            "rejected_attempts": self.rejected_attempts,
            "promotion_blockers": self.promotion_blockers,
            "command_hints": self.command_hints,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub staged_fill_bindings: Vec<StagedFillBinding>,
    pub promotion_attempts: Vec<PromotionAttempt>,
    pub promotion_blockers: Vec<PromotionBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let staged_fill_bindings = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| StagedFillBinding::placeholder(slot_kind, &config))
            .collect::<Vec<_>>();
        let promotion_attempts = staged_fill_bindings
            .iter()
            .map(|binding| PromotionAttempt::blocked(binding.slot_kind, binding, &config))
            .collect::<Vec<_>>();
        let mut state = Self {
            command_hints: CommandHint::canonical(&config),
            promotion_blockers: Vec::new(),
            counters: Counters::default(),
            promotion_attempts,
            staged_fill_bindings,
            config,
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn stage_promotion_attempt(
        &mut self,
        slot_kind: LaneSlotKind,
        staged_fill_root: &str,
        slot_occupancy_root: &str,
    ) -> Result<String> {
        ensure_root("staged_fill_root", staged_fill_root)?;
        ensure_root("slot_occupancy_root", slot_occupancy_root)?;
        let binding = StagedFillBinding::from_roots(
            slot_kind,
            staged_fill_root,
            &self.config.source_binding_root,
            &self.config.target_slot_registry_root,
        )?;
        for stored in &mut self.staged_fill_bindings {
            if stored.slot_kind == slot_kind {
                *stored = binding.clone();
            }
        }
        let attempt = PromotionAttempt::blocked_staged(slot_kind, &binding, slot_occupancy_root)?;
        for stored in &mut self.promotion_attempts {
            if stored.slot_kind == slot_kind {
                *stored = attempt.clone();
            }
        }
        self.recompute();
        self.validate()?;
        Ok(self.promotion_attempt_root())
    }

    pub fn recompute(&mut self) {
        self.promotion_blockers = self.compute_blockers();
        self.counters = self.compute_counters();
    }

    pub fn compute_blockers(&self) -> Vec<PromotionBlocker> {
        let mut blockers = Vec::new();
        for binding in &self.staged_fill_bindings {
            if binding.staged_fill_root
                == empty_root(&format!("{}-staged-fill", binding.slot_kind.as_str()))
            {
                blockers.push(PromotionBlocker::new(
                    PromotionBlockerKind::StagedFillMissing,
                    binding.slot_kind,
                    &binding.binding_root,
                ));
            }
            if binding.wave94_binding_root != self.config.source_binding_root {
                blockers.push(PromotionBlocker::new(
                    PromotionBlockerKind::StagedFillNotBound,
                    binding.slot_kind,
                    &binding.binding_root,
                ));
            }
        }
        for attempt in &self.promotion_attempts {
            if attempt.slot_occupancy_root == self.config.slot_occupancy_root_placeholder {
                blockers.push(PromotionBlocker::new(
                    PromotionBlockerKind::SlotOccupancyRootPlaceholder,
                    attempt.slot_kind,
                    &attempt.attempt_root,
                ));
            }
            if !attempt.status.is_promoted() {
                blockers.push(PromotionBlocker::new(
                    PromotionBlockerKind::OccupancyClaimDisallowed,
                    attempt.slot_kind,
                    &attempt.attempt_root,
                ));
            }
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::PromotionSignoffMissing,
                attempt.slot_kind,
                &attempt.attempt_root,
            ));
        }
        if self.config.max_promoted_slots == 0 {
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::PromotionBudgetZero,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if !self.config.production_allowed {
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::ProductionDenied,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if !self.config.fail_closed_armed {
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::FailClosedDisarmed,
                LaneSlotKind::AuditReview,
                &self.config.state_root(),
            ));
        }
        if self.config.heavy_gates_ran {
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::HeavyGateRunClaimed,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            blockers.push(PromotionBlocker::new(
                PromotionBlockerKind::RawPayloadPresent,
                LaneSlotKind::PrivacyReview,
                &self.state_material_root(),
            ));
        }
        blockers
    }

    pub fn compute_counters(&self) -> Counters {
        Counters {
            slot_count: self.promotion_attempts.len() as u64,
            promotion_attempts: self.promotion_attempts.len() as u64,
            promoted_slots: self
                .promotion_attempts
                .iter()
                .filter(|attempt| attempt.status == PromotionStatus::Promoted)
                .count() as u64,
            blocked_attempts: self
                .promotion_attempts
                .iter()
                .filter(|attempt| {
                    attempt.status == PromotionStatus::BlockedEmpty
                        || attempt.status == PromotionStatus::BlockedStagedFill
                })
                .count() as u64,
            rejected_attempts: self
                .promotion_attempts
                .iter()
                .filter(|attempt| attempt.status == PromotionStatus::Rejected)
                .count() as u64,
            promotion_blockers: self.promotion_blockers.len() as u64,
            command_hints: self.command_hints.len() as u64,
            raw_payload_records: 0,
        }
    }

    pub fn staged_fill_binding_root(&self) -> String {
        collection_root(
            "WAVE95-STAGED-FILL-BINDINGS",
            self.staged_fill_bindings
                .iter()
                .map(StagedFillBinding::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn promotion_attempt_root(&self) -> String {
        collection_root(
            "WAVE95-PROMOTION-ATTEMPTS",
            self.promotion_attempts
                .iter()
                .map(PromotionAttempt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn promotion_blocker_root(&self) -> String {
        collection_root(
            "WAVE95-PROMOTION-BLOCKERS",
            self.promotion_blockers
                .iter()
                .map(PromotionBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE95-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn slot_occupancy_root(&self) -> String {
        collection_root(
            "WAVE95-SLOT-OCCUPANCY-ROOT-PLACEHOLDERS",
            self.promotion_attempts
                .iter()
                .map(|attempt| {
                    json!({
                        "slot_kind": attempt.slot_kind.as_str(),
                        "slot_occupancy_root": attempt.slot_occupancy_root,
                        "promoted": false,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE95-STATE-MATERIAL",
            &json!({
                "config_root": self.config.state_root(),
                "staged_fill_binding_root": self.staged_fill_binding_root(),
                "promotion_attempt_root": self.promotion_attempt_root(),
                "slot_occupancy_root": self.slot_occupancy_root(),
                "command_hint_root": self.command_hint_root(),
                "counters": self.counters.public_record(),
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE95-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "promotion_blocker_root": self.promotion_blocker_root(),
                "fail_closed_armed": self.config.fail_closed_armed,
                "production_allowed": false,
                "promoted_slots": 0,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || !self.promotion_blockers.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.promotion_attempts.len() as u64,
            self.config.min_slot_count,
        )?;
        for binding in &self.staged_fill_bindings {
            binding.validate(&self.config)?;
        }
        for attempt in &self.promotion_attempts {
            attempt.validate()?;
        }
        for blocker in &self.promotion_blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("promotion lane contains raw payload records".to_string());
        }
        if self.counters.promoted_slots > self.config.max_promoted_slots {
            return Err("promotion lane promoted slots above configured limit".to_string());
        }
        if self.counters.promoted_slots != 0 {
            return Err("devnet promotion lane must not promote slots".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("promotion counters do not match state".to_string());
        }
        if !self.production_denied() {
            return Err("promotion lane cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave95_live_heavy_gate_receipt_slot_promotion_audit_security_lane_state",
            "config": self.config.public_record(),
            "staged_fill_binding_root": self.staged_fill_binding_root(),
            "promotion_attempt_root": self.promotion_attempt_root(),
            "slot_occupancy_root": self.slot_occupancy_root(),
            "promotion_blocker_root": self.promotion_blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "staged_fill_bindings": self.staged_fill_bindings.iter().map(StagedFillBinding::public_record).collect::<Vec<_>>(),
            "promotion_attempts": self.promotion_attempts.iter().map(PromotionAttempt::public_record).collect::<Vec<_>>(),
            "promotion_blockers": self.promotion_blockers.iter().map(PromotionBlocker::public_record).collect::<Vec<_>>(),
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
    let staged_fill_bindings = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| StagedFillBinding::placeholder(slot_kind, &config))
        .collect::<Vec<_>>();
    let promotion_attempts = staged_fill_bindings
        .iter()
        .map(|binding| PromotionAttempt::blocked(binding.slot_kind, binding, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        command_hints: CommandHint::canonical(&config),
        promotion_blockers: vec![PromotionBlocker::new(
            PromotionBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE95-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        counters: Counters::default(),
        promotion_attempts,
        staged_fill_bindings,
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
        "MONERO-L2-PQ-FORCE-EXIT-WAVE95-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(PROMOTION_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE95-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(PROMOTION_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE95-EMPTY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(PROMOTION_SUITE),
            HashPart::Str(label),
        ],
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
