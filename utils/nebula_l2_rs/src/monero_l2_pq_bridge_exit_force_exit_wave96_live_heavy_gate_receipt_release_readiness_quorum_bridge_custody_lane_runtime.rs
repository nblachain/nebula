use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave96-release-readiness-quorum-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE92_SLOT_REGISTRY_ROOT: &str =
    "root:wave92:bridge-custody-receipt-slot-registry-fail-closed-placeholder";
pub const WAVE93_ADMISSION_ROOT: &str =
    "root:wave93:bridge-custody-admission-quarantine-empty-placeholder";
pub const WAVE94_STAGED_FILL_ROOT: &str =
    "root:wave94:bridge-custody-staged-fill-empty-placeholder";
pub const WAVE95_PROMOTION_ROOT: &str =
    "root:wave95:bridge-custody-slot-promotion-empty-placeholder";
pub const EMPTY_ROOT: &str =
    "root:wave96:0000000000000000000000000000000000000000000000000000000000000000";
pub const DEFAULT_MIN_READY_SLOTS: u16 = 6;
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
    pub wave95_promotion_root: String,
    pub min_ready_slots: u16,
    pub min_watcher_quorum: u16,
    pub min_signer_quorum: u16,
    pub min_operator_signoff: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_blocks: u64,
    pub roots_only_public_records: bool,
    pub production_enabled: bool,
    pub release_enabled: bool,
    pub release_claims_enabled: bool,
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
            wave95_promotion_root: WAVE95_PROMOTION_ROOT.to_string(),
            min_ready_slots: DEFAULT_MIN_READY_SLOTS,
            min_watcher_quorum: DEFAULT_MIN_WATCHER_QUORUM,
            min_signer_quorum: DEFAULT_MIN_SIGNER_QUORUM,
            min_operator_signoff: DEFAULT_MIN_OPERATOR_SIGNOFF,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            challenge_hold_blocks: DEFAULT_CHALLENGE_HOLD_BLOCKS,
            roots_only_public_records: true,
            production_enabled: false,
            release_enabled: false,
            release_claims_enabled: false,
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
        ensure_root_like("wave95_promotion_root", &self.wave95_promotion_root)?;
        if self.min_ready_slots == 0 {
            return Err("ready slot quorum must be nonzero".to_string());
        }
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
        if self.production_enabled || self.release_enabled || self.release_claims_enabled {
            return Err("release lane enablement must remain denied".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave96 release readiness must not claim heavy gates ran".to_string());
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
            "wave95_promotion_root": self.wave95_promotion_root,
            "min_ready_slots": self.min_ready_slots,
            "min_watcher_quorum": self.min_watcher_quorum,
            "min_signer_quorum": self.min_signer_quorum,
            "min_operator_signoff": self.min_operator_signoff,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "challenge_hold_blocks": self.challenge_hold_blocks,
            "roots_only_public_records": self.roots_only_public_records,
            "production_enabled": self.production_enabled,
            "release_enabled": self.release_enabled,
            "release_claims_enabled": self.release_claims_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodySlotKind {
    MoneroWatcherQuorum,
    WithdrawalRelease,
    ReserveCoverage,
    SignerQuorum,
    ChallengeHoldReview,
    CustodyOperatorSignoff,
}

impl CustodySlotKind {
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
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    PromotedSlotRootMissing,
    WatcherQuorumUnmet,
    WithdrawalReleaseMissing,
    ReserveCoverageUnmet,
    SignerQuorumUnmet,
    ChallengeHoldOpen,
    CustodyOperatorSignoffMissing,
    LaneQuorumUnmet,
    ProductionDenied,
    ReleaseDisabled,
    ReleaseClaimsDisabled,
    HeavyGateClaimRejected,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PromotedSlotRootMissing => "promoted_slot_root_missing",
            Self::WatcherQuorumUnmet => "watcher_quorum_unmet",
            Self::WithdrawalReleaseMissing => "withdrawal_release_missing",
            Self::ReserveCoverageUnmet => "reserve_coverage_unmet",
            Self::SignerQuorumUnmet => "signer_quorum_unmet",
            Self::ChallengeHoldOpen => "challenge_hold_open",
            Self::CustodyOperatorSignoffMissing => "custody_operator_signoff_missing",
            Self::LaneQuorumUnmet => "lane_quorum_unmet",
            Self::ProductionDenied => "production_denied",
            Self::ReleaseDisabled => "release_disabled",
            Self::ReleaseClaimsDisabled => "release_claims_disabled",
            Self::HeavyGateClaimRejected => "heavy_gate_claim_rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStatus {
    Blocked,
    ReleaseReady,
}

impl ReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::ReleaseReady => "release_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepProductionDenied,
    KeepReleaseDisabled,
    ReviewPromotedSlotRoots,
    ReviewMoneroWatcherQuorum,
    ReviewWithdrawalRelease,
    ReviewReserveCoverage,
    ReviewSignerQuorum,
    ReviewChallengeHold,
    ReviewCustodyOperatorSignoff,
    RecomputeReadinessQuorum,
}

impl CommandHintKind {
    pub fn all() -> [Self; 10] {
        [
            Self::KeepProductionDenied,
            Self::KeepReleaseDisabled,
            Self::ReviewPromotedSlotRoots,
            Self::ReviewMoneroWatcherQuorum,
            Self::ReviewWithdrawalRelease,
            Self::ReviewReserveCoverage,
            Self::ReviewSignerQuorum,
            Self::ReviewChallengeHold,
            Self::ReviewCustodyOperatorSignoff,
            Self::RecomputeReadinessQuorum,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepProductionDenied => "keep_production_denied",
            Self::KeepReleaseDisabled => "keep_release_disabled",
            Self::ReviewPromotedSlotRoots => "review_promoted_slot_roots",
            Self::ReviewMoneroWatcherQuorum => "review_monero_watcher_quorum",
            Self::ReviewWithdrawalRelease => "review_withdrawal_release",
            Self::ReviewReserveCoverage => "review_reserve_coverage",
            Self::ReviewSignerQuorum => "review_signer_quorum",
            Self::ReviewChallengeHold => "review_challenge_hold",
            Self::ReviewCustodyOperatorSignoff => "review_custody_operator_signoff",
            Self::RecomputeReadinessQuorum => "recompute_readiness_quorum",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotedSlotReadiness {
    pub slot_kind: CustodySlotKind,
    pub promoted_slot_root: String,
    pub watcher_quorum_count: u16,
    pub withdrawal_release_root: String,
    pub reserve_coverage_bps: u16,
    pub signer_quorum_count: u16,
    pub challenge_hold_blocks_elapsed: u64,
    pub challenge_hold_clear: bool,
    pub custody_operator_signoff_count: u16,
    pub blocker_roots: Vec<String>,
    pub release_claim_root: String,
    pub readiness_root: String,
    pub status: ReadinessStatus,
}

impl PromotedSlotReadiness {
    pub fn placeholder(slot_kind: CustodySlotKind, config: &Config) -> Self {
        let mut item = Self {
            slot_kind,
            promoted_slot_root: EMPTY_ROOT.to_string(),
            watcher_quorum_count: 0,
            withdrawal_release_root: EMPTY_ROOT.to_string(),
            reserve_coverage_bps: 0,
            signer_quorum_count: 0,
            challenge_hold_blocks_elapsed: 0,
            challenge_hold_clear: false,
            custody_operator_signoff_count: 0,
            blocker_roots: Vec::new(),
            release_claim_root: release_claim_placeholder(slot_kind),
            readiness_root: EMPTY_ROOT.to_string(),
            status: ReadinessStatus::Blocked,
        };
        item.recompute(config);
        item
    }

    pub fn recompute(&mut self, config: &Config) {
        let blockers = self.blockers(config);
        self.blocker_roots = blockers
            .iter()
            .map(|kind| blocker_root(self.slot_kind, *kind))
            .collect::<Vec<_>>();
        self.status = if blockers.is_empty() {
            ReadinessStatus::ReleaseReady
        } else {
            ReadinessStatus::Blocked
        };
        self.release_claim_root = if self.status == ReadinessStatus::ReleaseReady {
            record_root("release-claim", &self.public_record_without_root())
        } else {
            release_claim_placeholder(self.slot_kind)
        };
        self.readiness_root = record_root("slot-readiness", &self.public_record_without_root());
    }

    pub fn blockers(&self, config: &Config) -> Vec<BlockerKind> {
        let mut blockers = Vec::new();
        if self.promoted_slot_root == EMPTY_ROOT {
            blockers.push(BlockerKind::PromotedSlotRootMissing);
        }
        if self.watcher_quorum_count < config.min_watcher_quorum {
            blockers.push(BlockerKind::WatcherQuorumUnmet);
        }
        if self.withdrawal_release_root == EMPTY_ROOT {
            blockers.push(BlockerKind::WithdrawalReleaseMissing);
        }
        if self.reserve_coverage_bps < config.min_reserve_coverage_bps {
            blockers.push(BlockerKind::ReserveCoverageUnmet);
        }
        if self.signer_quorum_count < config.min_signer_quorum {
            blockers.push(BlockerKind::SignerQuorumUnmet);
        }
        if !self.challenge_hold_clear
            || self.challenge_hold_blocks_elapsed < config.challenge_hold_blocks
        {
            blockers.push(BlockerKind::ChallengeHoldOpen);
        }
        if self.custody_operator_signoff_count < config.min_operator_signoff {
            blockers.push(BlockerKind::CustodyOperatorSignoffMissing);
        }
        if !config.production_enabled {
            blockers.push(BlockerKind::ProductionDenied);
        }
        if !config.release_enabled {
            blockers.push(BlockerKind::ReleaseDisabled);
        }
        if !config.release_claims_enabled {
            blockers.push(BlockerKind::ReleaseClaimsDisabled);
        }
        if config.heavy_gates_ran {
            blockers.push(BlockerKind::HeavyGateClaimRejected);
        }
        blockers.sort();
        blockers.dedup();
        blockers
    }

    pub fn public_record_without_root(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "promoted_slot_root": self.promoted_slot_root,
            "watcher_quorum_count": self.watcher_quorum_count,
            "withdrawal_release_root": self.withdrawal_release_root,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "signer_quorum_count": self.signer_quorum_count,
            "challenge_hold_blocks_elapsed": self.challenge_hold_blocks_elapsed,
            "challenge_hold_clear": self.challenge_hold_clear,
            "custody_operator_signoff_count": self.custody_operator_signoff_count,
            "release_claim_root": self.release_claim_root,
            "status": self.status.as_str(),
            "blocker_root": roots_root(
                "wave96-bridge-custody-slot-readiness-blockers",
                self.blocker_roots.clone(),
            ),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_root();
        if let Some(map) = record.as_object_mut() {
            map.insert(
                "readiness_root".to_string(),
                Value::String(self.readiness_root.clone()),
            );
        }
        record
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub target_slot: Option<CustodySlotKind>,
    pub command_root: String,
    pub dry_run_only: bool,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_slot: Option<CustodySlotKind>) -> Self {
        let target = match target_slot {
            Some(slot) => slot.as_str(),
            None => "bridge_custody_lane",
        };
        let command_root = record_root(
            "command-hint",
            &json!({
                "kind": kind.as_str(),
                "target_slot": target,
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
                        Some(CustodySlotKind::MoneroWatcherQuorum)
                    }
                    CommandHintKind::ReviewWithdrawalRelease => {
                        Some(CustodySlotKind::WithdrawalRelease)
                    }
                    CommandHintKind::ReviewReserveCoverage => {
                        Some(CustodySlotKind::ReserveCoverage)
                    }
                    CommandHintKind::ReviewSignerQuorum => Some(CustodySlotKind::SignerQuorum),
                    CommandHintKind::ReviewChallengeHold => {
                        Some(CustodySlotKind::ChallengeHoldReview)
                    }
                    CommandHintKind::ReviewCustodyOperatorSignoff => {
                        Some(CustodySlotKind::CustodyOperatorSignoff)
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
            "target_slot": self.target_slot.map(CustodySlotKind::as_str),
            "command_root": self.command_root,
            "dry_run_only": self.dry_run_only,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessSummary {
    pub status: ReadinessStatus,
    pub fail_closed: bool,
    pub release_ready_lane_count: u64,
    pub ready_slot_count: u64,
    pub blocked_slot_count: u64,
    pub promoted_slot_root: String,
    pub release_claim_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub lane_release_readiness_root: String,
    pub production_enabled: bool,
    pub release_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl ReadinessSummary {
    pub fn from_state(
        slots: &BTreeMap<String, PromotedSlotReadiness>,
        command_hints: &[CommandHint],
        config: &Config,
    ) -> Self {
        let ready_slot_count = slots
            .values()
            .filter(|slot| slot.status == ReadinessStatus::ReleaseReady)
            .count() as u64;
        let blocked_slot_count = slots.len() as u64 - ready_slot_count;
        let blocker_records = slots
            .values()
            .flat_map(|slot| {
                slot.blocker_roots.iter().map(|root| {
                    json!({
                        "slot_kind": slot.slot_kind.as_str(),
                        "blocker_root": root,
                    })
                })
            })
            .collect::<Vec<_>>();
        let promoted_slot_records = slots
            .values()
            .map(|slot| Value::String(slot.promoted_slot_root.clone()))
            .collect::<Vec<_>>();
        let release_claim_records = slots
            .values()
            .map(|slot| Value::String(slot.release_claim_root.clone()))
            .collect::<Vec<_>>();
        let command_records = command_hints
            .iter()
            .map(|hint| Value::String(hint.command_root.clone()))
            .collect::<Vec<_>>();
        let lane_ready = ready_slot_count >= config.min_ready_slots as u64
            && blocked_slot_count == 0
            && config.production_enabled
            && config.release_enabled
            && config.release_claims_enabled;
        let status = if lane_ready {
            ReadinessStatus::ReleaseReady
        } else {
            ReadinessStatus::Blocked
        };
        let release_ready_lane_count = if lane_ready { 1 } else { 0 };
        let promoted_slot_root = roots_root(
            "wave96-bridge-custody-promoted-slot-roots",
            promoted_slot_records
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect(),
        );
        let release_claim_root = roots_root(
            "wave96-bridge-custody-release-claim-placeholders",
            release_claim_records
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect(),
        );
        let blocker_root = format!(
            "root:wave96:{}",
            merkle_root("WAVE96-BRIDGE-CUSTODY-READINESS-BLOCKERS", &blocker_records)
        );
        let command_hint_root = format!(
            "root:wave96:{}",
            merkle_root("WAVE96-BRIDGE-CUSTODY-COMMAND-HINTS", &command_records)
        );
        let lane_record = json!({
            "ready_slot_count": ready_slot_count,
            "blocked_slot_count": blocked_slot_count,
            "min_ready_slots": config.min_ready_slots,
            "release_ready_lane_count": release_ready_lane_count,
            "status": status.as_str(),
        });
        Self {
            status,
            fail_closed: status == ReadinessStatus::Blocked,
            release_ready_lane_count,
            ready_slot_count,
            blocked_slot_count,
            promoted_slot_root,
            release_claim_root,
            blocker_root,
            command_hint_root,
            lane_release_readiness_root: record_root("lane-release-readiness", &lane_record),
            production_enabled: false,
            release_enabled: false,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "release_ready_lane_count": self.release_ready_lane_count,
            "ready_slot_count": self.ready_slot_count,
            "blocked_slot_count": self.blocked_slot_count,
            "promoted_slot_root": self.promoted_slot_root,
            "release_claim_root": self.release_claim_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "lane_release_readiness_root": self.lane_release_readiness_root,
            "production_enabled": self.production_enabled,
            "release_enabled": self.release_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promoted_slots: BTreeMap<String, PromotedSlotReadiness>,
    pub command_hints: Vec<CommandHint>,
    pub summary: ReadinessSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let promoted_slots = CustodySlotKind::all()
            .iter()
            .map(|slot_kind| {
                let slot = PromotedSlotReadiness::placeholder(*slot_kind, &config);
                (slot_kind.as_str().to_string(), slot)
            })
            .collect::<BTreeMap<_, _>>();
        let command_hints = CommandHint::canonical();
        let summary = ReadinessSummary::from_state(&promoted_slots, &command_hints, &config);
        Self {
            config,
            promoted_slots,
            command_hints,
            summary,
        }
    }

    pub fn recompute(&mut self) {
        for slot in self.promoted_slots.values_mut() {
            slot.recompute(&self.config);
        }
        self.summary =
            ReadinessSummary::from_state(&self.promoted_slots, &self.command_hints, &self.config);
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.summary.release_ready_lane_count != 0 {
            return Err("devnet must expose zero release-ready lanes".to_string());
        }
        if self.summary.ready_slot_count != 0 {
            return Err("devnet must expose zero release-ready slots".to_string());
        }
        if !self.summary.fail_closed {
            return Err("release readiness quorum must fail closed".to_string());
        }
        if self.config.production_enabled || self.config.release_enabled {
            return Err("release readiness enablement invariant is broken".to_string());
        }
        if self.config.heavy_gates_ran {
            return Err("heavy gate invariant is broken".to_string());
        }
        for slot in self.promoted_slots.values() {
            if slot.status == ReadinessStatus::ReleaseReady {
                return Err("devnet slot cannot be release ready".to_string());
            }
            ensure_root_like("promoted_slot_root", &slot.promoted_slot_root)?;
            ensure_root_like("release_claim_root", &slot.release_claim_root)?;
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave96_bridge_custody_receipt_release_readiness_quorum",
            "config": self.config.public_record(),
            "wave92_slot_registry_root": self.config.wave92_slot_registry_root,
            "wave93_admission_root": self.config.wave93_admission_root,
            "wave94_staged_fill_root": self.config.wave94_staged_fill_root,
            "wave95_promotion_root": self.config.wave95_promotion_root,
            "promoted_slot_count": self.promoted_slots.len() as u64,
            "promoted_slot_root": self.summary.promoted_slot_root,
            "release_claim_root": self.summary.release_claim_root,
            "quorum_blocker_root": self.summary.blocker_root,
            "command_hint_root": self.summary.command_hint_root,
            "lane_release_readiness_root": self.summary.lane_release_readiness_root,
            "summary": self.summary.public_record(),
            "roots_only_public_records": true,
            "release_claims_are_placeholders": true,
            "production_enabled": false,
            "release_enabled": false,
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

fn blocker_root(slot_kind: CustodySlotKind, blocker_kind: BlockerKind) -> String {
    record_root(
        "readiness-blocker",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "blocker_kind": blocker_kind.as_str(),
        }),
    )
}

fn release_claim_placeholder(slot_kind: CustodySlotKind) -> String {
    record_root(
        "release-claim-placeholder",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "release_ready": false,
        }),
    )
}

fn roots_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave96:{}", merkle_root(domain, &leaves))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE96-BRIDGE-CUSTODY-RELEASE-READINESS-QUORUM",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave96:{hash}")
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
