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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const PROMOTION_SUITE: &str =
    "monero-l2-wave95-wave94-staged-fill-to-wave92-slot-occupancy-promotion-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 95;
pub const STAGING_WAVE: u64 = 94;
pub const SLOT_WAVE: u64 = 92;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 95;
pub const DEFAULT_MIN_PQ_SIGNERS: u64 = 5;
pub const DEFAULT_MIN_PQ_FAMILIES: u64 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_500;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_REDACTION_DEPTH: u64 = 4;
pub const DEFAULT_MAX_PROMOTION_ATTEMPTS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-pq-reserve-privacy-lane-runtime";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotKind {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqQuorum,
    ReserveCoverage,
    PrivacyLinkage,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl SlotKind {
    pub fn all() -> [Self; 8] {
        [
            Self::MlDsaAuthorityEpoch,
            Self::SlhDsaAuthorityEpoch,
            Self::PqQuorum,
            Self::ReserveCoverage,
            Self::PrivacyLinkage,
            Self::MetadataRedaction,
            Self::NullifierSeparation,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MlDsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqQuorum => "pq_quorum",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyLinkage => "privacy_linkage",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionStatus {
    Blocked,
    PromotableShadow,
    Rejected,
}

impl PromotionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::PromotableShadow => "promotable_shadow",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBlocker {
    NoPromotionAttempt,
    ProductionDenied,
    HeavyGateNotRun,
    StagedFillRootMissing,
    SlotRootMissing,
    SlotOccupancyRootMissing,
    StagedFillBindingMissing,
    MlDsaAuthorityEpochMissing,
    SlhDsaAuthorityEpochMissing,
    PqQuorumMissing,
    PqFamilyCoverageMissing,
    ReserveCoverageMissing,
    ReserveBufferMissing,
    PrivacyLinkageMissing,
    MetadataRedactionMissing,
    NullifierSeparationMissing,
    OperatorSignoffMissing,
    RootsOnlyBoundary,
    DuplicatePromotionRoot,
    PromotionCapacityReached,
}

impl PromotionBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoPromotionAttempt => "no_promotion_attempt",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::StagedFillRootMissing => "staged_fill_root_missing",
            Self::SlotRootMissing => "slot_root_missing",
            Self::SlotOccupancyRootMissing => "slot_occupancy_root_missing",
            Self::StagedFillBindingMissing => "staged_fill_binding_missing",
            Self::MlDsaAuthorityEpochMissing => "ml_dsa_authority_epoch_missing",
            Self::SlhDsaAuthorityEpochMissing => "slh_dsa_authority_epoch_missing",
            Self::PqQuorumMissing => "pq_quorum_missing",
            Self::PqFamilyCoverageMissing => "pq_family_coverage_missing",
            Self::ReserveCoverageMissing => "reserve_coverage_missing",
            Self::ReserveBufferMissing => "reserve_buffer_missing",
            Self::PrivacyLinkageMissing => "privacy_linkage_missing",
            Self::MetadataRedactionMissing => "metadata_redaction_missing",
            Self::NullifierSeparationMissing => "nullifier_separation_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DuplicatePromotionRoot => "duplicate_promotion_root",
            Self::PromotionCapacityReached => "promotion_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    Blocked,
    PromotableShadow,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Blocked => "blocked",
            Self::PromotableShadow => "promotable_shadow",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave94StagedFillRoot,
    ImportWave92SlotRoot,
    BindStagedFillToSlotRoot,
    AttachSlotOccupancyRootPlaceholder,
    AttachMlDsaAuthorityEpochRoot,
    AttachSlhDsaAuthorityEpochRoot,
    AttachPqQuorumRoot,
    AttachReserveCoverageRoot,
    AttachPrivacyLinkageRoot,
    AttachMetadataRedactionRoot,
    AttachNullifierSeparationRoot,
    AttachOperatorSignoffRoot,
    KeepProductionDenied,
    PublishRootsOnlyPromotionRecord,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave94StagedFillRoot,
            Self::ImportWave92SlotRoot,
            Self::BindStagedFillToSlotRoot,
            Self::AttachSlotOccupancyRootPlaceholder,
            Self::AttachMlDsaAuthorityEpochRoot,
            Self::AttachSlhDsaAuthorityEpochRoot,
            Self::AttachPqQuorumRoot,
            Self::AttachReserveCoverageRoot,
            Self::AttachPrivacyLinkageRoot,
            Self::AttachMetadataRedactionRoot,
            Self::AttachNullifierSeparationRoot,
            Self::AttachOperatorSignoffRoot,
            Self::KeepProductionDenied,
            Self::PublishRootsOnlyPromotionRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave94StagedFillRoot => "import_wave94_staged_fill_root",
            Self::ImportWave92SlotRoot => "import_wave92_slot_root",
            Self::BindStagedFillToSlotRoot => "bind_staged_fill_to_slot_root",
            Self::AttachSlotOccupancyRootPlaceholder => "attach_slot_occupancy_root_placeholder",
            Self::AttachMlDsaAuthorityEpochRoot => "attach_ml_dsa_authority_epoch_root",
            Self::AttachSlhDsaAuthorityEpochRoot => "attach_slh_dsa_authority_epoch_root",
            Self::AttachPqQuorumRoot => "attach_pq_quorum_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachPrivacyLinkageRoot => "attach_privacy_linkage_root",
            Self::AttachMetadataRedactionRoot => "attach_metadata_redaction_root",
            Self::AttachNullifierSeparationRoot => "attach_nullifier_separation_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
            Self::KeepProductionDenied => "keep_production_denied",
            Self::PublishRootsOnlyPromotionRecord => "publish_roots_only_promotion_record",
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
    pub staging_wave: u64,
    pub slot_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_pq_signers: u64,
    pub min_pq_families: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub min_redaction_depth: u64,
    pub fail_closed: bool,
    pub heavy_gate_ran: bool,
    pub production_allowed: bool,
    pub roots_only_public_record: bool,
    pub max_promotion_attempts: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            promotion_suite: PROMOTION_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            staging_wave: STAGING_WAVE,
            slot_wave: SLOT_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_pq_signers: DEFAULT_MIN_PQ_SIGNERS,
            min_pq_families: DEFAULT_MIN_PQ_FAMILIES,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            min_redaction_depth: DEFAULT_MIN_REDACTION_DEPTH,
            fail_closed: true,
            heavy_gate_ran: false,
            production_allowed: false,
            roots_only_public_record: true,
            max_promotion_attempts: DEFAULT_MAX_PROMOTION_ATTEMPTS,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "promotion_suite": self.promotion_suite,
            "wave": self.wave,
            "staging_wave": self.staging_wave,
            "slot_wave": self.slot_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_pq_signers": self.min_pq_signers,
            "min_pq_families": self.min_pq_families,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_reserve_buffer_bps": self.min_reserve_buffer_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "min_redaction_depth": self.min_redaction_depth,
            "fail_closed": self.fail_closed,
            "heavy_gate_ran": self.heavy_gate_ran,
            "production_allowed": self.production_allowed,
            "roots_only_public_record": self.roots_only_public_record,
            "max_promotion_attempts": self.max_promotion_attempts,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionRoots {
    pub wave94_staged_fill_root: String,
    pub wave92_slot_root: String,
    pub staged_fill_slot_binding_root: String,
    pub slot_occupancy_root_placeholder: String,
    pub ml_dsa_authority_epoch_root: String,
    pub slh_dsa_authority_epoch_root: String,
    pub pq_quorum_root: String,
    pub reserve_coverage_root: String,
    pub privacy_linkage_root: String,
    pub metadata_redaction_root: String,
    pub nullifier_separation_root: String,
    pub operator_signoff_root: String,
}

impl PromotionRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "wave92_slot_root": self.wave92_slot_root,
            "staged_fill_slot_binding_root": self.staged_fill_slot_binding_root,
            "slot_occupancy_root_placeholder": self.slot_occupancy_root_placeholder,
            "ml_dsa_authority_epoch_root": self.ml_dsa_authority_epoch_root,
            "slh_dsa_authority_epoch_root": self.slh_dsa_authority_epoch_root,
            "pq_quorum_root": self.pq_quorum_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_linkage_root": self.privacy_linkage_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "nullifier_separation_root": self.nullifier_separation_root,
            "operator_signoff_root": self.operator_signoff_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionMetrics {
    pub ml_dsa_authority_epoch: u64,
    pub slh_dsa_authority_epoch: u64,
    pub pq_signer_count: u64,
    pub pq_family_count: u64,
    pub reserve_coverage_bps: u64,
    pub reserve_buffer_bps: u64,
    pub linkage_risk_bps: u64,
    pub redaction_depth: u64,
    pub nullifier_domains_separated: bool,
    pub operator_signed: bool,
    pub root_only: bool,
}

impl Default for PromotionMetrics {
    fn default() -> Self {
        Self {
            ml_dsa_authority_epoch: 0,
            slh_dsa_authority_epoch: 0,
            pq_signer_count: 0,
            pq_family_count: 0,
            reserve_coverage_bps: 0,
            reserve_buffer_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            redaction_depth: 0,
            nullifier_domains_separated: false,
            operator_signed: false,
            root_only: true,
        }
    }
}

impl PromotionMetrics {
    pub fn public_record(&self) -> Value {
        json!({
            "ml_dsa_authority_epoch": self.ml_dsa_authority_epoch,
            "slh_dsa_authority_epoch": self.slh_dsa_authority_epoch,
            "pq_signer_count": self.pq_signer_count,
            "pq_family_count": self.pq_family_count,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "reserve_buffer_bps": self.reserve_buffer_bps,
            "linkage_risk_bps": self.linkage_risk_bps,
            "redaction_depth": self.redaction_depth,
            "nullifier_domains_separated": self.nullifier_domains_separated,
            "operator_signed": self.operator_signed,
            "root_only": self.root_only,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub attempt_root: String,
    pub slot: SlotKind,
    pub status: PromotionStatus,
    pub roots: PromotionRoots,
    pub metrics: PromotionMetrics,
    pub blockers: Vec<PromotionBlocker>,
}

impl PromotionAttempt {
    pub fn evaluate(
        config: &Config,
        slot: SlotKind,
        roots: PromotionRoots,
        metrics: PromotionMetrics,
    ) -> Self {
        let blockers = promotion_blockers(config, &roots, &metrics);
        let status = if blockers.is_empty() {
            PromotionStatus::PromotableShadow
        } else if !metrics.root_only {
            PromotionStatus::Rejected
        } else {
            PromotionStatus::Blocked
        };
        let attempt_root = attempt_root(slot, status, &roots, &metrics, &blockers);
        Self {
            attempt_root,
            slot,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "attempt_root": self.attempt_root,
            "slot": self.slot.as_str(),
            "status": self.status.as_str(),
            "roots_root": self.roots.state_root(),
            "metrics_root": self.metrics.state_root(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SlotPromotionState {
    pub slot: SlotKind,
    pub promoted: bool,
    pub blocker_root: String,
    pub promotion_attempt_root: String,
    pub slot_occupancy_root: String,
}

impl SlotPromotionState {
    pub fn blocked(slot: SlotKind) -> Self {
        let blockers = vec![
            PromotionBlocker::NoPromotionAttempt,
            PromotionBlocker::ProductionDenied,
            PromotionBlocker::HeavyGateNotRun,
        ];
        Self {
            slot,
            promoted: false,
            blocker_root: blocker_root(&blockers),
            promotion_attempt_root: empty_root("promotion_attempt", slot.as_str()),
            slot_occupancy_root: empty_root("slot_occupancy_placeholder", slot.as_str()),
        }
    }

    pub fn from_attempt(slot: SlotKind, attempt: &PromotionAttempt) -> Self {
        let occupancy_root = if attempt.status == PromotionStatus::PromotableShadow {
            attempt.roots.slot_occupancy_root_placeholder.clone()
        } else {
            empty_root("slot_occupancy_placeholder", slot.as_str())
        };
        Self {
            slot,
            promoted: attempt.status == PromotionStatus::PromotableShadow,
            blocker_root: blocker_root(&attempt.blockers),
            promotion_attempt_root: attempt.state_root(),
            slot_occupancy_root: occupancy_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "slot": self.slot.as_str(),
            "promoted": self.promoted,
            "blocker_root": self.blocker_root,
            "promotion_attempt_root": self.promotion_attempt_root,
            "slot_occupancy_root": self.slot_occupancy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("slot_promotion_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionCounters {
    pub promotion_attempt_count: u64,
    pub promotable_shadow_count: u64,
    pub blocked_count: u64,
    pub rejected_count: u64,
    pub promoted_slot_count: u64,
    pub blocked_slot_count: u64,
}

impl PromotionCounters {
    pub fn from_parts(
        attempts: &[PromotionAttempt],
        slots: &BTreeMap<SlotKind, SlotPromotionState>,
    ) -> Self {
        let mut counters = Self::default();
        for attempt in attempts {
            counters.promotion_attempt_count = counters.promotion_attempt_count.saturating_add(1);
            match attempt.status {
                PromotionStatus::PromotableShadow => {
                    counters.promotable_shadow_count =
                        counters.promotable_shadow_count.saturating_add(1);
                }
                PromotionStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                PromotionStatus::Rejected => {
                    counters.rejected_count = counters.rejected_count.saturating_add(1);
                }
            }
        }
        counters.promoted_slot_count = slots.values().filter(|slot| slot.promoted).count() as u64;
        counters.blocked_slot_count = slots.values().filter(|slot| !slot.promoted).count() as u64;
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("promotion_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promotion_attempts: Vec<PromotionAttempt>,
    pub slot_states: BTreeMap<SlotKind, SlotPromotionState>,
    pub counters: PromotionCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, promotion_attempts: Vec<PromotionAttempt>) -> Result<Self> {
        if promotion_attempts.len() > config.max_promotion_attempts {
            return Err(PromotionBlocker::PromotionCapacityReached
                .as_str()
                .to_string());
        }
        let mut seen = BTreeSet::new();
        for attempt in &promotion_attempts {
            if !seen.insert(attempt.attempt_root.clone()) {
                return Err(PromotionBlocker::DuplicatePromotionRoot
                    .as_str()
                    .to_string());
            }
        }
        let mut slot_states = BTreeMap::new();
        for slot in SlotKind::all() {
            let maybe_attempt = promotion_attempts
                .iter()
                .rev()
                .find(|attempt| attempt.slot == slot);
            let slot_state = match maybe_attempt {
                Some(attempt) => SlotPromotionState::from_attempt(slot, attempt),
                None => SlotPromotionState::blocked(slot),
            };
            slot_states.insert(slot, slot_state);
        }
        let counters = PromotionCounters::from_parts(&promotion_attempts, &slot_states);
        Ok(Self {
            config,
            promotion_attempts,
            slot_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn promote_staged_fill(
        &self,
        slot: SlotKind,
        roots: PromotionRoots,
        metrics: PromotionMetrics,
    ) -> Result<Self> {
        let mut promotion_attempts = self.promotion_attempts.clone();
        if promotion_attempts.len() >= self.config.max_promotion_attempts {
            return Err(PromotionBlocker::PromotionCapacityReached
                .as_str()
                .to_string());
        }
        let attempt = PromotionAttempt::evaluate(&self.config, slot, roots, metrics);
        if promotion_attempts
            .iter()
            .any(|item| item.attempt_root == attempt.attempt_root)
        {
            return Err(PromotionBlocker::DuplicatePromotionRoot
                .as_str()
                .to_string());
        }
        promotion_attempts.push(attempt);
        Self::new(self.config.clone(), promotion_attempts)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.promotion_attempts.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.promoted_slot_count == SlotKind::all().len() as u64
            && self.counters.blocked_slot_count == 0
        {
            RuntimeVerdict::PromotableShadow
        } else {
            RuntimeVerdict::Blocked
        }
    }

    pub fn public_record(&self) -> Value {
        let slot_records = self
            .slot_states
            .values()
            .map(SlotPromotionState::public_record)
            .collect::<Vec<_>>();
        let attempt_records = self
            .promotion_attempts
            .iter()
            .map(PromotionAttempt::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "slot_promotion_root": list_root("slot_promotion_states", slot_records),
            "promotion_attempt_root": list_root("promotion_attempts", attempt_records),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.promotion_attempts, &self.slot_states),
            "active_promotion_blocker_root": blocker_root(&default_active_promotion_blockers()),
            "operator_command_root": operator_command_root(&self.operator_commands),
            "verdict": self.verdict().as_str(),
            "counters": self.counters.public_record(),
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state"),
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state-core"),
            &[
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&attempts_root(&self.promotion_attempts)),
                HashPart::Str(&slots_root(&self.slot_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_promotion_blockers())),
                HashPart::Str(&operator_command_root(&self.operator_commands)),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    match State::new(config.clone(), Vec::new()) {
        Ok(state) => state,
        Err(reason) => {
            let roots = PromotionRoots {
                wave94_staged_fill_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..PromotionRoots::empty()
            };
            let attempt = PromotionAttempt::evaluate(
                &config,
                SlotKind::OperatorSignoff,
                roots,
                PromotionMetrics::default(),
            );
            State {
                config,
                promotion_attempts: Vec::new(),
                slot_states: SlotKind::all()
                    .into_iter()
                    .map(|slot| (slot, SlotPromotionState::blocked(slot)))
                    .collect(),
                counters: PromotionCounters::from_parts(&[attempt], &BTreeMap::new()),
                operator_commands: OperatorCommand::sequence(),
            }
        }
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn promotion_blockers(
    config: &Config,
    roots: &PromotionRoots,
    metrics: &PromotionMetrics,
) -> Vec<PromotionBlocker> {
    let mut blockers = Vec::new();
    if !config.production_allowed {
        blockers.push(PromotionBlocker::ProductionDenied);
    }
    if !config.heavy_gate_ran {
        blockers.push(PromotionBlocker::HeavyGateNotRun);
    }
    if roots.wave94_staged_fill_root.is_empty() {
        blockers.push(PromotionBlocker::StagedFillRootMissing);
    }
    if roots.wave92_slot_root.is_empty() {
        blockers.push(PromotionBlocker::SlotRootMissing);
    }
    if roots.staged_fill_slot_binding_root.is_empty() {
        blockers.push(PromotionBlocker::StagedFillBindingMissing);
    }
    if roots.slot_occupancy_root_placeholder.is_empty() {
        blockers.push(PromotionBlocker::SlotOccupancyRootMissing);
    }
    if roots.ml_dsa_authority_epoch_root.is_empty()
        || metrics.ml_dsa_authority_epoch < config.authority_epoch
    {
        blockers.push(PromotionBlocker::MlDsaAuthorityEpochMissing);
    }
    if roots.slh_dsa_authority_epoch_root.is_empty()
        || metrics.slh_dsa_authority_epoch < config.authority_epoch
    {
        blockers.push(PromotionBlocker::SlhDsaAuthorityEpochMissing);
    }
    if roots.pq_quorum_root.is_empty() || metrics.pq_signer_count < config.min_pq_signers {
        blockers.push(PromotionBlocker::PqQuorumMissing);
    }
    if metrics.pq_family_count < config.min_pq_families {
        blockers.push(PromotionBlocker::PqFamilyCoverageMissing);
    }
    if roots.reserve_coverage_root.is_empty()
        || metrics.reserve_coverage_bps < config.min_reserve_coverage_bps
    {
        blockers.push(PromotionBlocker::ReserveCoverageMissing);
    }
    if metrics.reserve_buffer_bps < config.min_reserve_buffer_bps {
        blockers.push(PromotionBlocker::ReserveBufferMissing);
    }
    if roots.privacy_linkage_root.is_empty()
        || metrics.linkage_risk_bps > config.max_linkage_risk_bps
    {
        blockers.push(PromotionBlocker::PrivacyLinkageMissing);
    }
    if roots.metadata_redaction_root.is_empty()
        || metrics.redaction_depth < config.min_redaction_depth
    {
        blockers.push(PromotionBlocker::MetadataRedactionMissing);
    }
    if roots.nullifier_separation_root.is_empty() || !metrics.nullifier_domains_separated {
        blockers.push(PromotionBlocker::NullifierSeparationMissing);
    }
    if roots.operator_signoff_root.is_empty() || !metrics.operator_signed {
        blockers.push(PromotionBlocker::OperatorSignoffMissing);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(PromotionBlocker::RootsOnlyBoundary);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_promotion_blockers() -> Vec<PromotionBlocker> {
    vec![
        PromotionBlocker::NoPromotionAttempt,
        PromotionBlocker::ProductionDenied,
        PromotionBlocker::HeavyGateNotRun,
        PromotionBlocker::StagedFillRootMissing,
        PromotionBlocker::SlotRootMissing,
        PromotionBlocker::SlotOccupancyRootMissing,
        PromotionBlocker::StagedFillBindingMissing,
        PromotionBlocker::MlDsaAuthorityEpochMissing,
        PromotionBlocker::SlhDsaAuthorityEpochMissing,
        PromotionBlocker::PqQuorumMissing,
        PromotionBlocker::PqFamilyCoverageMissing,
        PromotionBlocker::ReserveCoverageMissing,
        PromotionBlocker::ReserveBufferMissing,
        PromotionBlocker::PrivacyLinkageMissing,
        PromotionBlocker::MetadataRedactionMissing,
        PromotionBlocker::NullifierSeparationMissing,
        PromotionBlocker::OperatorSignoffMissing,
        PromotionBlocker::RootsOnlyBoundary,
    ]
}

fn attempt_root(
    slot: SlotKind,
    status: PromotionStatus,
    roots: &PromotionRoots,
    metrics: &PromotionMetrics,
    blockers: &[PromotionBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:attempt"),
        &[
            HashPart::Str(slot.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn attempts_root(attempts: &[PromotionAttempt]) -> String {
    let leaves = attempts
        .iter()
        .map(|attempt| Value::String(attempt.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:attempts"), &leaves)
}

fn slots_root(slots: &BTreeMap<SlotKind, SlotPromotionState>) -> String {
    let leaves = slots
        .values()
        .map(|slot| Value::String(slot.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:slots"), &leaves)
}

fn blocker_root(blockers: &[PromotionBlocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| json!({ "blocker": blocker.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn all_blockers_root(
    attempts: &[PromotionAttempt],
    slots: &BTreeMap<SlotKind, SlotPromotionState>,
) -> String {
    let mut leaves = attempts
        .iter()
        .flat_map(|attempt| {
            attempt.blockers.iter().map(|blocker| {
                json!({
                    "attempt_root": attempt.attempt_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(slots.values().filter(|slot| !slot.promoted).map(|slot| {
        json!({
            "slot": slot.slot.as_str(),
            "blocker_root": slot.blocker_root,
        })
    }));
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn operator_command_root(commands: &[OperatorCommand]) -> String {
    let leaves = commands
        .iter()
        .map(|command| json!({ "command": command.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:operator-commands"), &leaves)
}

fn list_root(kind: &str, leaves: Vec<Value>) -> String {
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn empty_root(kind: &str, slot: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:empty"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(slot),
        ],
        32,
    )
}

fn dedupe_blockers(blockers: &mut Vec<PromotionBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
