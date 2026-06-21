use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave95-live-heavy-gate-receipt-slot-promotion-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE92_SLOT_REGISTRY_ROOT: &str =
    "root:wave92:bridge-custody-receipt-slot-registry-fail-closed-placeholder";
pub const WAVE93_ADMISSION_ROOT: &str =
    "root:wave93:bridge-custody-admission-quarantine-empty-placeholder";
pub const WAVE94_STAGED_FILL_ROOT: &str =
    "root:wave94:bridge-custody-staged-fill-empty-placeholder";
pub const EMPTY_ROOT: &str =
    "root:wave95:0000000000000000000000000000000000000000000000000000000000000000";
pub const DEFAULT_MIN_WATCHER_QUORUM: u16 = 4;
pub const DEFAULT_MIN_SIGNER_QUORUM: u16 = 3;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF: u16 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u16 = 10_000;
pub const DEFAULT_CHALLENGE_HOLD_BLOCKS: u64 = 720;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave92_slot_registry_root: String,
    pub wave93_admission_root: String,
    pub wave94_staged_fill_root: String,
    pub min_watcher_quorum: u16,
    pub min_signer_quorum: u16,
    pub min_operator_signoff: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_blocks: u64,
    pub roots_only_public_records: bool,
    pub production_enabled: bool,
    pub promotion_enabled: bool,
    pub slot_occupancy_claims_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave92_slot_registry_root: WAVE92_SLOT_REGISTRY_ROOT.to_string(),
            wave93_admission_root: WAVE93_ADMISSION_ROOT.to_string(),
            wave94_staged_fill_root: WAVE94_STAGED_FILL_ROOT.to_string(),
            min_watcher_quorum: DEFAULT_MIN_WATCHER_QUORUM,
            min_signer_quorum: DEFAULT_MIN_SIGNER_QUORUM,
            min_operator_signoff: DEFAULT_MIN_OPERATOR_SIGNOFF,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            challenge_hold_blocks: DEFAULT_CHALLENGE_HOLD_BLOCKS,
            roots_only_public_records: true,
            production_enabled: false,
            promotion_enabled: false,
            slot_occupancy_claims_enabled: false,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_root_like("wave92_slot_registry_root", &self.wave92_slot_registry_root)?;
        ensure_root_like("wave93_admission_root", &self.wave93_admission_root)?;
        ensure_root_like("wave94_staged_fill_root", &self.wave94_staged_fill_root)?;
        if self.min_watcher_quorum == 0 {
            return Err("watcher quorum must be nonzero".to_string());
        }
        if self.min_signer_quorum == 0 {
            return Err("signer quorum must be nonzero".to_string());
        }
        if self.min_operator_signoff == 0 {
            return Err("operator signoff quorum must be nonzero".to_string());
        }
        if self.min_reserve_coverage_bps > 10_000 {
            return Err("reserve coverage bps exceeds full coverage".to_string());
        }
        if self.challenge_hold_blocks == 0 {
            return Err("challenge hold window must be nonzero".to_string());
        }
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.production_enabled {
            return Err("production must remain denied".to_string());
        }
        if self.promotion_enabled || self.slot_occupancy_claims_enabled {
            return Err("promotion into slot occupancy claims is disabled".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave95 promotion gate must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "lane_id": self.lane_id,
            "protocol_version": self.protocol_version,
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "wave93_admission_root": self.wave93_admission_root,
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "min_watcher_quorum": self.min_watcher_quorum,
            "min_signer_quorum": self.min_signer_quorum,
            "min_operator_signoff": self.min_operator_signoff,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "challenge_hold_blocks": self.challenge_hold_blocks,
            "roots_only_public_records": self.roots_only_public_records,
            "production_enabled": self.production_enabled,
            "promotion_enabled": self.promotion_enabled,
            "slot_occupancy_claims_enabled": self.slot_occupancy_claims_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotKind {
    MoneroWatcherQuorum,
    WithdrawalRelease,
    ReserveCoverage,
    SignerQuorum,
    ChallengeHoldReview,
    CustodyOperatorSignoff,
}

impl SlotKind {
    pub fn all() -> [Self; 6] {
        [
            Self::MoneroWatcherQuorum,
            Self::WithdrawalRelease,
            Self::ReserveCoverage,
            Self::SignerQuorum,
            Self::ChallengeHoldReview,
            Self::CustodyOperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroWatcherQuorum => "monero_watcher_quorum",
            Self::WithdrawalRelease => "withdrawal_release",
            Self::ReserveCoverage => "reserve_coverage",
            Self::SignerQuorum => "signer_quorum",
            Self::ChallengeHoldReview => "challenge_hold_review",
            Self::CustodyOperatorSignoff => "custody_operator_signoff",
        }
    }

    pub fn quorum_floor(self, config: &Config) -> u16 {
        match self {
            Self::MoneroWatcherQuorum => config.min_watcher_quorum,
            Self::SignerQuorum => config.min_signer_quorum,
            Self::CustodyOperatorSignoff => config.min_operator_signoff,
            Self::WithdrawalRelease | Self::ReserveCoverage | Self::ChallengeHoldReview => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
pub enum BlockerKind {
    NoStagedFillBinding,
    StagedFillRootMissing,
    Wave92SlotRegistryMismatch,
    Wave93AdmissionMismatch,
    QuorumBelowFloor,
    ReserveCoverageBelowFloor,
    ChallengeHoldOpen,
    ProductionDenied,
    PromotionDisabled,
    SlotOccupancyClaimsDisabled,
    HeavyGateReceiptAbsent,
    HeavyGateClaimRejected,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoStagedFillBinding => "no_staged_fill_binding",
            Self::StagedFillRootMissing => "staged_fill_root_missing",
            Self::Wave92SlotRegistryMismatch => "wave92_slot_registry_mismatch",
            Self::Wave93AdmissionMismatch => "wave93_admission_mismatch",
            Self::QuorumBelowFloor => "quorum_below_floor",
            Self::ReserveCoverageBelowFloor => "reserve_coverage_below_floor",
            Self::ChallengeHoldOpen => "challenge_hold_open",
            Self::ProductionDenied => "production_denied",
            Self::PromotionDisabled => "promotion_disabled",
            Self::SlotOccupancyClaimsDisabled => "slot_occupancy_claims_disabled",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::HeavyGateClaimRejected => "heavy_gate_claim_rejected",
        }
    }

    pub fn all_active() -> [Self; 12] {
        [
            Self::NoStagedFillBinding,
            Self::StagedFillRootMissing,
            Self::Wave92SlotRegistryMismatch,
            Self::Wave93AdmissionMismatch,
            Self::QuorumBelowFloor,
            Self::ReserveCoverageBelowFloor,
            Self::ChallengeHoldOpen,
            Self::ProductionDenied,
            Self::PromotionDisabled,
            Self::SlotOccupancyClaimsDisabled,
            Self::HeavyGateReceiptAbsent,
            Self::HeavyGateClaimRejected,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepProductionDenied,
    KeepPromotionDisabled,
    ReviewStagedFillBinding,
    ReviewMoneroWatcherQuorum,
    ReviewWithdrawalRelease,
    ReviewReserveCoverage,
    ReviewSignerQuorum,
    ReviewChallengeHold,
    ReviewCustodyOperatorSignoff,
    RecomputePromotionVerdict,
}

impl CommandHintKind {
    pub fn all() -> [Self; 10] {
        [
            Self::KeepProductionDenied,
            Self::KeepPromotionDisabled,
            Self::ReviewStagedFillBinding,
            Self::ReviewMoneroWatcherQuorum,
            Self::ReviewWithdrawalRelease,
            Self::ReviewReserveCoverage,
            Self::ReviewSignerQuorum,
            Self::ReviewChallengeHold,
            Self::ReviewCustodyOperatorSignoff,
            Self::RecomputePromotionVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepProductionDenied => "keep_production_denied",
            Self::KeepPromotionDisabled => "keep_promotion_disabled",
            Self::ReviewStagedFillBinding => "review_staged_fill_binding",
            Self::ReviewMoneroWatcherQuorum => "review_monero_watcher_quorum",
            Self::ReviewWithdrawalRelease => "review_withdrawal_release",
            Self::ReviewReserveCoverage => "review_reserve_coverage",
            Self::ReviewSignerQuorum => "review_signer_quorum",
            Self::ReviewChallengeHold => "review_challenge_hold",
            Self::ReviewCustodyOperatorSignoff => "review_custody_operator_signoff",
            Self::RecomputePromotionVerdict => "recompute_promotion_verdict",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StagedFillBinding {
    pub slot_kind: SlotKind,
    pub wave92_slot_registry_root: String,
    pub wave93_admission_root: String,
    pub wave94_staged_fill_root: String,
    pub staged_fill_root: String,
    pub binding_root: String,
}

impl StagedFillBinding {
    pub fn new(
        slot_kind: SlotKind,
        wave92_slot_registry_root: impl Into<String>,
        wave93_admission_root: impl Into<String>,
        wave94_staged_fill_root: impl Into<String>,
        staged_fill_root: impl Into<String>,
    ) -> Self {
        let wave92_slot_registry_root = wave92_slot_registry_root.into();
        let wave93_admission_root = wave93_admission_root.into();
        let wave94_staged_fill_root = wave94_staged_fill_root.into();
        let staged_fill_root = staged_fill_root.into();
        let binding_root = record_root(
            "staged-fill-binding",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "wave92_slot_registry_root": wave92_slot_registry_root,
                "wave93_admission_root": wave93_admission_root,
                "wave94_staged_fill_root": wave94_staged_fill_root,
                "staged_fill_root": staged_fill_root,
            }),
        );
        Self {
            slot_kind,
            wave92_slot_registry_root,
            wave93_admission_root,
            wave94_staged_fill_root,
            staged_fill_root,
            binding_root,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root_like("wave92_slot_registry_root", &self.wave92_slot_registry_root)?;
        ensure_root_like("wave93_admission_root", &self.wave93_admission_root)?;
        ensure_root_like("wave94_staged_fill_root", &self.wave94_staged_fill_root)?;
        ensure_root_like("staged_fill_root", &self.staged_fill_root)?;
        ensure_root_like("binding_root", &self.binding_root)?;
        if self.wave92_slot_registry_root != config.wave92_slot_registry_root {
            return Err("wave92 slot registry root mismatch".to_string());
        }
        if self.wave93_admission_root != config.wave93_admission_root {
            return Err("wave93 admission root mismatch".to_string());
        }
        if self.wave94_staged_fill_root != config.wave94_staged_fill_root {
            return Err("wave94 staged fill root mismatch".to_string());
        }
        if self.staged_fill_root == EMPTY_ROOT {
            return Err("staged fill root must be present".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "wave93_admission_root": self.wave93_admission_root,
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "staged_fill_root": self.staged_fill_root,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionAttempt {
    pub slot_kind: SlotKind,
    pub binding: Option<StagedFillBinding>,
    pub quorum_count: u16,
    pub reserve_coverage_bps: u16,
    pub challenge_hold_blocks_elapsed: u64,
    pub challenge_hold_clear: bool,
    pub heavy_gate_ran: bool,
    pub status: PromotionStatus,
    pub blocker_roots: Vec<String>,
    pub slot_occupancy_root: String,
    pub attempt_root: String,
}

impl PromotionAttempt {
    pub fn blocked_empty(slot_kind: SlotKind, config: &Config) -> Self {
        let mut attempt = Self {
            slot_kind,
            binding: None,
            quorum_count: 0,
            reserve_coverage_bps: 0,
            challenge_hold_blocks_elapsed: 0,
            challenge_hold_clear: false,
            heavy_gate_ran: false,
            status: PromotionStatus::Blocked,
            blocker_roots: Vec::new(),
            slot_occupancy_root: slot_occupancy_placeholder(slot_kind),
            attempt_root: EMPTY_ROOT.to_string(),
        };
        attempt.recompute(config);
        attempt
    }

    pub fn from_binding(
        binding: StagedFillBinding,
        quorum_count: u16,
        reserve_coverage_bps: u16,
        challenge_hold_blocks_elapsed: u64,
        challenge_hold_clear: bool,
        config: &Config,
    ) -> Self {
        let mut attempt = Self {
            slot_kind: binding.slot_kind,
            binding: Some(binding),
            quorum_count,
            reserve_coverage_bps,
            challenge_hold_blocks_elapsed,
            challenge_hold_clear,
            heavy_gate_ran: false,
            status: PromotionStatus::Blocked,
            blocker_roots: Vec::new(),
            slot_occupancy_root: EMPTY_ROOT.to_string(),
            attempt_root: EMPTY_ROOT.to_string(),
        };
        attempt.recompute(config);
        attempt
    }

    pub fn recompute(&mut self, config: &Config) {
        let blockers = self.blockers(config);
        self.blocker_roots = blockers
            .iter()
            .map(|kind| blocker_root(self.slot_kind, *kind))
            .collect::<Vec<_>>();
        self.status = if blockers.is_empty() {
            PromotionStatus::Promoted
        } else {
            PromotionStatus::Blocked
        };
        self.slot_occupancy_root = if self.status == PromotionStatus::Promoted {
            record_root("slot-occupancy-claim", &self.public_record_without_root())
        } else {
            slot_occupancy_placeholder(self.slot_kind)
        };
        self.attempt_root = record_root("promotion-attempt", &self.public_record_without_root());
    }

    pub fn blockers(&self, config: &Config) -> Vec<BlockerKind> {
        let mut blockers = Vec::new();
        match &self.binding {
            Some(binding) => {
                if binding.staged_fill_root == EMPTY_ROOT {
                    blockers.push(BlockerKind::StagedFillRootMissing);
                }
                if binding.wave92_slot_registry_root != config.wave92_slot_registry_root {
                    blockers.push(BlockerKind::Wave92SlotRegistryMismatch);
                }
                if binding.wave93_admission_root != config.wave93_admission_root {
                    blockers.push(BlockerKind::Wave93AdmissionMismatch);
                }
                if binding.validate(config).is_err() {
                    blockers.push(BlockerKind::StagedFillRootMissing);
                }
            }
            None => {
                blockers.push(BlockerKind::NoStagedFillBinding);
                blockers.push(BlockerKind::StagedFillRootMissing);
                blockers.push(BlockerKind::Wave92SlotRegistryMismatch);
                blockers.push(BlockerKind::Wave93AdmissionMismatch);
            }
        }
        if self.quorum_count < self.slot_kind.quorum_floor(config) {
            blockers.push(BlockerKind::QuorumBelowFloor);
        }
        if self.reserve_coverage_bps < config.min_reserve_coverage_bps {
            blockers.push(BlockerKind::ReserveCoverageBelowFloor);
        }
        if !self.challenge_hold_clear
            || self.challenge_hold_blocks_elapsed < config.challenge_hold_blocks
        {
            blockers.push(BlockerKind::ChallengeHoldOpen);
        }
        if !config.production_enabled {
            blockers.push(BlockerKind::ProductionDenied);
        }
        if !config.promotion_enabled {
            blockers.push(BlockerKind::PromotionDisabled);
        }
        if !config.slot_occupancy_claims_enabled {
            blockers.push(BlockerKind::SlotOccupancyClaimsDisabled);
        }
        if self.heavy_gate_ran || config.heavy_gates_ran {
            blockers.push(BlockerKind::HeavyGateClaimRejected);
        } else {
            blockers.push(BlockerKind::HeavyGateReceiptAbsent);
        }
        blockers.sort();
        blockers.dedup();
        blockers
    }

    pub fn public_record_without_root(&self) -> PublicRecord {
        let binding_root = match &self.binding {
            Some(binding) => binding.binding_root.clone(),
            None => EMPTY_ROOT.to_string(),
        };
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "binding_root": binding_root,
            "quorum_count": self.quorum_count,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "challenge_hold_blocks_elapsed": self.challenge_hold_blocks_elapsed,
            "challenge_hold_clear": self.challenge_hold_clear,
            "heavy_gate_ran": self.heavy_gate_ran,
            "status": self.status.as_str(),
            "slot_occupancy_root": self.slot_occupancy_root,
            "blocker_root": roots_root(
                "wave95-bridge-custody-promotion-blockers",
                self.blocker_roots.clone(),
            ),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_root();
        if let Some(map) = record.as_object_mut() {
            map.insert(
                "attempt_root".to_string(),
                Value::String(self.attempt_root.clone()),
            );
        }
        record
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionBlocker {
    pub slot_kind: SlotKind,
    pub blocker_kind: BlockerKind,
    pub blocker_root: String,
}

impl PromotionBlocker {
    pub fn new(slot_kind: SlotKind, blocker_kind: BlockerKind) -> Self {
        Self {
            slot_kind,
            blocker_kind,
            blocker_root: blocker_root(slot_kind, blocker_kind),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "blocker_kind": self.blocker_kind.as_str(),
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub target_slot: Option<SlotKind>,
    pub command_root: String,
    pub dry_run_only: bool,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_slot: Option<SlotKind>) -> Self {
        let slot = match target_slot {
            Some(value) => value.as_str(),
            None => "all_slots",
        };
        let command_root = record_root(
            "command-hint",
            &json!({
                "kind": kind.as_str(),
                "target_slot": slot,
                "dry_run_only": true,
            }),
        );
        Self {
            kind,
            target_slot,
            command_root,
            dry_run_only: true,
        }
    }

    pub fn canonical() -> Vec<Self> {
        CommandHintKind::all()
            .iter()
            .map(|kind| {
                let target_slot = match kind {
                    CommandHintKind::ReviewMoneroWatcherQuorum => {
                        Some(SlotKind::MoneroWatcherQuorum)
                    }
                    CommandHintKind::ReviewWithdrawalRelease => Some(SlotKind::WithdrawalRelease),
                    CommandHintKind::ReviewReserveCoverage => Some(SlotKind::ReserveCoverage),
                    CommandHintKind::ReviewSignerQuorum => Some(SlotKind::SignerQuorum),
                    CommandHintKind::ReviewChallengeHold => Some(SlotKind::ChallengeHoldReview),
                    CommandHintKind::ReviewCustodyOperatorSignoff => {
                        Some(SlotKind::CustodyOperatorSignoff)
                    }
                    _ => None,
                };
                Self::new(*kind, target_slot)
            })
            .collect()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "target_slot": self.target_slot.map(SlotKind::as_str),
            "command_root": self.command_root,
            "dry_run_only": self.dry_run_only,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotionSummary {
    pub status: PromotionStatus,
    pub fail_closed: bool,
    pub promoted_slot_count: u64,
    pub blocked_slot_count: u64,
    pub promotion_attempt_root: String,
    pub promotion_blocker_root: String,
    pub staged_fill_binding_root: String,
    pub slot_occupancy_root: String,
    pub command_hint_root: String,
    pub production_enabled: bool,
    pub promotion_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl PromotionSummary {
    pub fn from_state(
        attempts: &BTreeMap<String, PromotionAttempt>,
        command_hints: &[CommandHint],
    ) -> Self {
        let promoted_slot_count = attempts
            .values()
            .filter(|attempt| attempt.status == PromotionStatus::Promoted)
            .count() as u64;
        let blocked_slot_count = attempts.len() as u64 - promoted_slot_count;
        let blocker_records = attempts
            .values()
            .flat_map(|attempt| {
                attempt.blocker_roots.iter().map(|root| {
                    json!({
                        "slot_kind": attempt.slot_kind.as_str(),
                        "blocker_root": root,
                    })
                })
            })
            .collect::<Vec<_>>();
        let attempt_records = attempts
            .values()
            .map(|attempt| Value::String(attempt.attempt_root.clone()))
            .collect::<Vec<_>>();
        let binding_records = attempts
            .values()
            .filter_map(|attempt| attempt.binding.as_ref())
            .map(|binding| Value::String(binding.binding_root.clone()))
            .collect::<Vec<_>>();
        let occupancy_records = attempts
            .values()
            .map(|attempt| Value::String(attempt.slot_occupancy_root.clone()))
            .collect::<Vec<_>>();
        let command_records = command_hints
            .iter()
            .map(|hint| Value::String(hint.command_root.clone()))
            .collect::<Vec<_>>();
        let status = if promoted_slot_count > 0 && blocked_slot_count == 0 {
            PromotionStatus::Promoted
        } else {
            PromotionStatus::Blocked
        };
        Self {
            status,
            fail_closed: status == PromotionStatus::Blocked,
            promoted_slot_count,
            blocked_slot_count,
            promotion_attempt_root: format!(
                "root:wave95:{}",
                merkle_root("WAVE95-BRIDGE-CUSTODY-PROMOTION-ATTEMPTS", &attempt_records)
            ),
            promotion_blocker_root: format!(
                "root:wave95:{}",
                merkle_root("WAVE95-BRIDGE-CUSTODY-PROMOTION-BLOCKERS", &blocker_records)
            ),
            staged_fill_binding_root: format!(
                "root:wave95:{}",
                merkle_root(
                    "WAVE95-BRIDGE-CUSTODY-STAGED-FILL-BINDINGS",
                    &binding_records
                )
            ),
            slot_occupancy_root: format!(
                "root:wave95:{}",
                merkle_root("WAVE95-BRIDGE-CUSTODY-SLOT-OCCUPANCY", &occupancy_records)
            ),
            command_hint_root: format!(
                "root:wave95:{}",
                merkle_root("WAVE95-BRIDGE-CUSTODY-COMMAND-HINTS", &command_records)
            ),
            production_enabled: false,
            promotion_enabled: false,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "promoted_slot_count": self.promoted_slot_count,
            "blocked_slot_count": self.blocked_slot_count,
            "promotion_attempt_root": self.promotion_attempt_root,
            "promotion_blocker_root": self.promotion_blocker_root,
            "staged_fill_binding_root": self.staged_fill_binding_root,
            "slot_occupancy_root": self.slot_occupancy_root,
            "command_hint_root": self.command_hint_root,
            "production_enabled": self.production_enabled,
            "promotion_enabled": self.promotion_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promotion_attempts: BTreeMap<String, PromotionAttempt>,
    pub promotion_blockers: BTreeMap<String, PromotionBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub summary: PromotionSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let promotion_attempts = SlotKind::all()
            .iter()
            .map(|slot_kind| {
                let attempt = PromotionAttempt::blocked_empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), attempt)
            })
            .collect::<BTreeMap<_, _>>();
        let promotion_blockers = Self::blockers_from_attempts(&promotion_attempts);
        let command_hints = CommandHint::canonical();
        let summary = PromotionSummary::from_state(&promotion_attempts, &command_hints);
        Self {
            config,
            promotion_attempts,
            promotion_blockers,
            command_hints,
            summary,
        }
    }

    pub fn stage_promotion_candidate(
        mut self,
        slot_kind: SlotKind,
        staged_fill_root: impl Into<String>,
        quorum_count: u16,
        reserve_coverage_bps: u16,
        challenge_hold_blocks_elapsed: u64,
        challenge_hold_clear: bool,
    ) -> Result<Self> {
        self.config.validate()?;
        let binding = StagedFillBinding::new(
            slot_kind,
            self.config.wave92_slot_registry_root.clone(),
            self.config.wave93_admission_root.clone(),
            self.config.wave94_staged_fill_root.clone(),
            staged_fill_root,
        );
        binding.validate(&self.config)?;
        let attempt = PromotionAttempt::from_binding(
            binding,
            quorum_count,
            reserve_coverage_bps,
            challenge_hold_blocks_elapsed,
            challenge_hold_clear,
            &self.config,
        );
        self.promotion_attempts
            .insert(slot_kind.as_str().to_string(), attempt);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for attempt in self.promotion_attempts.values_mut() {
            attempt.recompute(&self.config);
        }
        self.promotion_blockers = Self::blockers_from_attempts(&self.promotion_attempts);
        self.summary = PromotionSummary::from_state(&self.promotion_attempts, &self.command_hints);
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.summary.promoted_slot_count != 0 {
            return Err("devnet must expose zero promoted slots".to_string());
        }
        if !self.summary.fail_closed {
            return Err("promotion gate must fail closed".to_string());
        }
        if self.config.production_enabled
            || self.config.promotion_enabled
            || self.config.slot_occupancy_claims_enabled
        {
            return Err("promotion lane enablement invariant is broken".to_string());
        }
        if self.config.heavy_gates_ran {
            return Err("heavy gate invariant is broken".to_string());
        }
        for attempt in self.promotion_attempts.values() {
            if attempt.heavy_gate_ran || attempt.status == PromotionStatus::Promoted {
                return Err("promotion attempt cannot claim execution or promotion".to_string());
            }
        }
        Ok(())
    }

    pub fn blockers_from_attempts(
        attempts: &BTreeMap<String, PromotionAttempt>,
    ) -> BTreeMap<String, PromotionBlocker> {
        let mut blockers = BTreeMap::new();
        for attempt in attempts.values() {
            for root in &attempt.blocker_roots {
                let kind = blocker_kind_from_root(attempt.slot_kind, root);
                blockers.insert(root.clone(), PromotionBlocker::new(attempt.slot_kind, kind));
            }
        }
        blockers
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave95_bridge_custody_receipt_slot_promotion_gate",
            "config": self.config.public_record(),
            "promotion_attempt_count": self.promotion_attempts.len() as u64,
            "promotion_attempt_root": self.summary.promotion_attempt_root,
            "promotion_blocker_count": self.promotion_blockers.len() as u64,
            "promotion_blocker_root": self.summary.promotion_blocker_root,
            "staged_fill_binding_root": self.summary.staged_fill_binding_root,
            "slot_occupancy_root": self.summary.slot_occupancy_root,
            "slot_occupancy_root_placeholder": roots_root(
                "wave95-bridge-custody-slot-occupancy-placeholders",
                SlotKind::all()
                    .iter()
                    .map(|slot| slot_occupancy_placeholder(*slot))
                    .collect(),
            ),
            "command_hint_root": self.summary.command_hint_root,
            "summary": self.summary.public_record(),
            "roots_only_public_records": true,
            "production_enabled": false,
            "promotion_enabled": false,
            "heavy_gates_ran": false,
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Some(map) = record.as_object_mut() {
            map.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    State::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn blocker_root(slot_kind: SlotKind, blocker_kind: BlockerKind) -> String {
    record_root(
        "promotion-blocker",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "blocker_kind": blocker_kind.as_str(),
        }),
    )
}

fn blocker_kind_from_root(slot_kind: SlotKind, root: &str) -> BlockerKind {
    for kind in BlockerKind::all_active() {
        if blocker_root(slot_kind, kind) == root {
            return kind;
        }
    }
    BlockerKind::NoStagedFillBinding
}

fn slot_occupancy_placeholder(slot_kind: SlotKind) -> String {
    record_root(
        "slot-occupancy-placeholder",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "occupied": false,
        }),
    )
}

fn roots_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave95:{}", merkle_root(domain, &leaves))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE95-BRIDGE-CUSTODY-RECEIPT-SLOT-PROMOTION",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave95:{hash}")
}

fn ensure_text(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{field} must be nonempty"));
    }
    Ok(())
}

fn ensure_root_like(field: &'static str, value: &str) -> Result<()> {
    ensure_text(field, value)?;
    if !(value.starts_with("root:") || value.len() >= 32) {
        return Err(format!("{field} must be a root commitment"));
    }
    for private_marker in [
        "txid",
        "address",
        "spend_key",
        "view_key",
        "signer_label",
        "payload",
        "route",
    ] {
        if value.contains(private_marker) {
            return Err(format!("{field} contains private material marker"));
        }
    }
    Ok(())
}
