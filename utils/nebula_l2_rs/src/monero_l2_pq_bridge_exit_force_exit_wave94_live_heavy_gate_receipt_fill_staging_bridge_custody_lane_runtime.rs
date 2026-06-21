use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave94-live-heavy-gate-receipt-fill-staging-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE92_SLOT_REGISTRY_ROOT: &str =
    "root:wave92:bridge-custody-receipt-slot-registry-fail-closed-placeholder";
pub const WAVE93_ADMISSION_ROOT: &str =
    "root:wave93:bridge-custody-admission-quarantine-empty-placeholder";
pub const EMPTY_ROOT: &str =
    "root:wave94:0000000000000000000000000000000000000000000000000000000000000000";
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
    pub min_watcher_quorum: u16,
    pub min_signer_quorum: u16,
    pub min_operator_signoff: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_blocks: u64,
    pub roots_only_public_records: bool,
    pub production_enabled: bool,
    pub live_fill_enabled: bool,
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
            min_watcher_quorum: DEFAULT_MIN_WATCHER_QUORUM,
            min_signer_quorum: DEFAULT_MIN_SIGNER_QUORUM,
            min_operator_signoff: DEFAULT_MIN_OPERATOR_SIGNOFF,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            challenge_hold_blocks: DEFAULT_CHALLENGE_HOLD_BLOCKS,
            roots_only_public_records: true,
            production_enabled: false,
            live_fill_enabled: false,
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
            return Err("production must remain denied by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave94 staging must not claim heavy gates ran".to_string());
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
            "min_watcher_quorum": self.min_watcher_quorum,
            "min_signer_quorum": self.min_signer_quorum,
            "min_operator_signoff": self.min_operator_signoff,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "challenge_hold_blocks": self.challenge_hold_blocks,
            "roots_only_public_records": self.roots_only_public_records,
            "production_enabled": self.production_enabled,
            "live_fill_enabled": self.live_fill_enabled,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    NoAdmittedRoot,
    SlotRootMissing,
    AdmissionRootMismatch,
    QuorumBelowFloor,
    ReserveCoverageBelowFloor,
    ChallengeHoldOpen,
    ProductionDenied,
    LiveFillDisabled,
    HeavyGateClaimRejected,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoAdmittedRoot => "no_admitted_root",
            Self::SlotRootMissing => "slot_root_missing",
            Self::AdmissionRootMismatch => "admission_root_mismatch",
            Self::QuorumBelowFloor => "quorum_below_floor",
            Self::ReserveCoverageBelowFloor => "reserve_coverage_below_floor",
            Self::ChallengeHoldOpen => "challenge_hold_open",
            Self::ProductionDenied => "production_denied",
            Self::LiveFillDisabled => "live_fill_disabled",
            Self::HeavyGateClaimRejected => "heavy_gate_claim_rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepProductionDenied,
    ReviewWave93AdmissionRoot,
    StageMoneroWatcherQuorum,
    StageWithdrawalRelease,
    StageReserveCoverage,
    StageSignerQuorum,
    StageChallengeHoldReview,
    StageCustodyOperatorSignoff,
    RecomputeFillVerdict,
}

impl CommandHintKind {
    pub fn all() -> [Self; 9] {
        [
            Self::KeepProductionDenied,
            Self::ReviewWave93AdmissionRoot,
            Self::StageMoneroWatcherQuorum,
            Self::StageWithdrawalRelease,
            Self::StageReserveCoverage,
            Self::StageSignerQuorum,
            Self::StageChallengeHoldReview,
            Self::StageCustodyOperatorSignoff,
            Self::RecomputeFillVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepProductionDenied => "keep_production_denied",
            Self::ReviewWave93AdmissionRoot => "review_wave93_admission_root",
            Self::StageMoneroWatcherQuorum => "stage_monero_watcher_quorum",
            Self::StageWithdrawalRelease => "stage_withdrawal_release",
            Self::StageReserveCoverage => "stage_reserve_coverage",
            Self::StageSignerQuorum => "stage_signer_quorum",
            Self::StageChallengeHoldReview => "stage_challenge_hold_review",
            Self::StageCustodyOperatorSignoff => "stage_custody_operator_signoff",
            Self::RecomputeFillVerdict => "recompute_fill_verdict",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmittedRootBinding {
    pub slot_kind: SlotKind,
    pub wave93_admission_root: String,
    pub admitted_receipt_root: String,
    pub wave92_slot_root: String,
    pub binding_root: String,
}

impl AdmittedRootBinding {
    pub fn new(
        slot_kind: SlotKind,
        wave93_admission_root: impl Into<String>,
        admitted_receipt_root: impl Into<String>,
        wave92_slot_root: impl Into<String>,
    ) -> Self {
        let wave93_admission_root = wave93_admission_root.into();
        let admitted_receipt_root = admitted_receipt_root.into();
        let wave92_slot_root = wave92_slot_root.into();
        let binding_root = record_root(
            "admitted-root-binding",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "wave93_admission_root": wave93_admission_root,
                "admitted_receipt_root": admitted_receipt_root,
                "wave92_slot_root": wave92_slot_root,
            }),
        );
        Self {
            slot_kind,
            wave93_admission_root,
            admitted_receipt_root,
            wave92_slot_root,
            binding_root,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root_like("wave93_admission_root", &self.wave93_admission_root)?;
        ensure_root_like("admitted_receipt_root", &self.admitted_receipt_root)?;
        ensure_root_like("wave92_slot_root", &self.wave92_slot_root)?;
        ensure_root_like("binding_root", &self.binding_root)?;
        if self.wave93_admission_root != config.wave93_admission_root {
            return Err("admission root does not match configured wave93 root".to_string());
        }
        if self.wave92_slot_root == EMPTY_ROOT {
            return Err("wave92 slot root must be present".to_string());
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
pub struct FillAttempt {
    pub slot_kind: SlotKind,
    pub binding: Option<AdmittedRootBinding>,
    pub quorum_count: u16,
    pub reserve_coverage_bps: u16,
    pub challenge_hold_blocks_elapsed: u64,
    pub challenge_hold_clear: bool,
    pub heavy_gate_ran: bool,
    pub status: FillStatus,
    pub blocker_roots: Vec<String>,
    pub attempt_root: String,
}

impl FillAttempt {
    pub fn blocked_empty(slot_kind: SlotKind, config: &Config) -> Self {
        let mut attempt = Self {
            slot_kind,
            binding: None,
            quorum_count: 0,
            reserve_coverage_bps: 0,
            challenge_hold_blocks_elapsed: 0,
            challenge_hold_clear: false,
            heavy_gate_ran: false,
            status: FillStatus::Blocked,
            blocker_roots: Vec::new(),
            attempt_root: EMPTY_ROOT.to_string(),
        };
        attempt.recompute(config);
        attempt
    }

    pub fn from_binding(
        binding: AdmittedRootBinding,
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
            status: FillStatus::Blocked,
            blocker_roots: Vec::new(),
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
            FillStatus::Staged
        } else {
            FillStatus::Blocked
        };
        self.attempt_root = record_root("fill-attempt", &self.public_record_without_root());
    }

    pub fn blockers(&self, config: &Config) -> Vec<BlockerKind> {
        let mut blockers = Vec::new();
        match &self.binding {
            Some(binding) => {
                if binding.validate(config).is_err() {
                    blockers.push(BlockerKind::AdmissionRootMismatch);
                }
            }
            None => blockers.push(BlockerKind::NoAdmittedRoot),
        }
        if self.quorum_count < self.slot_kind.quorum_floor(config) {
            blockers.push(BlockerKind::QuorumBelowFloor);
        }
        if self.slot_kind == SlotKind::ReserveCoverage
            && self.reserve_coverage_bps < config.min_reserve_coverage_bps
        {
            blockers.push(BlockerKind::ReserveCoverageBelowFloor);
        }
        if self.slot_kind == SlotKind::ChallengeHoldReview
            && (!self.challenge_hold_clear
                || self.challenge_hold_blocks_elapsed < config.challenge_hold_blocks)
        {
            blockers.push(BlockerKind::ChallengeHoldOpen);
        }
        if !config.production_enabled {
            blockers.push(BlockerKind::ProductionDenied);
        }
        if !config.live_fill_enabled {
            blockers.push(BlockerKind::LiveFillDisabled);
        }
        if self.heavy_gate_ran || config.heavy_gates_ran {
            blockers.push(BlockerKind::HeavyGateClaimRejected);
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
            "blocker_root": roots_root("wave94-bridge-custody-fill-blockers", self.blocker_roots.clone()),
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
pub struct SlotFillBlocker {
    pub slot_kind: SlotKind,
    pub blocker_kind: BlockerKind,
    pub blocker_root: String,
}

impl SlotFillBlocker {
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
                    CommandHintKind::StageMoneroWatcherQuorum => {
                        Some(SlotKind::MoneroWatcherQuorum)
                    }
                    CommandHintKind::StageWithdrawalRelease => Some(SlotKind::WithdrawalRelease),
                    CommandHintKind::StageReserveCoverage => Some(SlotKind::ReserveCoverage),
                    CommandHintKind::StageSignerQuorum => Some(SlotKind::SignerQuorum),
                    CommandHintKind::StageChallengeHoldReview => {
                        Some(SlotKind::ChallengeHoldReview)
                    }
                    CommandHintKind::StageCustodyOperatorSignoff => {
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
pub struct FillSummary {
    pub status: FillStatus,
    pub fail_closed: bool,
    pub staged_fill_count: u64,
    pub blocked_fill_count: u64,
    pub blocker_root: String,
    pub fill_attempt_root: String,
    pub command_hint_root: String,
    pub admitted_binding_root: String,
    pub production_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl FillSummary {
    pub fn from_state(
        fill_attempts: &BTreeMap<String, FillAttempt>,
        command_hints: &[CommandHint],
    ) -> Self {
        let staged_fill_count = fill_attempts
            .values()
            .filter(|attempt| attempt.status == FillStatus::Staged)
            .count() as u64;
        let blocked_fill_count = fill_attempts.len() as u64 - staged_fill_count;
        let blocker_records = fill_attempts
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
        let fill_attempt_records = fill_attempts
            .values()
            .map(|attempt| Value::String(attempt.attempt_root.clone()))
            .collect::<Vec<_>>();
        let command_records = command_hints
            .iter()
            .map(|hint| Value::String(hint.command_root.clone()))
            .collect::<Vec<_>>();
        let binding_records = fill_attempts
            .values()
            .filter_map(|attempt| attempt.binding.as_ref())
            .map(|binding| Value::String(binding.binding_root.clone()))
            .collect::<Vec<_>>();
        let status = if blocked_fill_count == 0 && !fill_attempts.is_empty() {
            FillStatus::Staged
        } else {
            FillStatus::Blocked
        };
        Self {
            status,
            fail_closed: status == FillStatus::Blocked,
            staged_fill_count,
            blocked_fill_count,
            blocker_root: format!(
                "root:wave94:{}",
                merkle_root("WAVE94-BRIDGE-CUSTODY-FILL-BLOCKERS", &blocker_records)
            ),
            fill_attempt_root: format!(
                "root:wave94:{}",
                merkle_root("WAVE94-BRIDGE-CUSTODY-FILL-ATTEMPTS", &fill_attempt_records)
            ),
            command_hint_root: format!(
                "root:wave94:{}",
                merkle_root("WAVE94-BRIDGE-CUSTODY-COMMAND-HINTS", &command_records)
            ),
            admitted_binding_root: format!(
                "root:wave94:{}",
                merkle_root("WAVE94-BRIDGE-CUSTODY-ADMITTED-BINDINGS", &binding_records)
            ),
            production_enabled: false,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "staged_fill_count": self.staged_fill_count,
            "blocked_fill_count": self.blocked_fill_count,
            "blocker_root": self.blocker_root,
            "fill_attempt_root": self.fill_attempt_root,
            "command_hint_root": self.command_hint_root,
            "admitted_binding_root": self.admitted_binding_root,
            "production_enabled": self.production_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub fill_attempts: BTreeMap<String, FillAttempt>,
    pub slot_fill_blockers: BTreeMap<String, SlotFillBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub summary: FillSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let fill_attempts = SlotKind::all()
            .iter()
            .map(|slot_kind| {
                let attempt = FillAttempt::blocked_empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), attempt)
            })
            .collect::<BTreeMap<_, _>>();
        let slot_fill_blockers = Self::blockers_from_attempts(&fill_attempts);
        let command_hints = CommandHint::canonical();
        let summary = FillSummary::from_state(&fill_attempts, &command_hints);
        Self {
            config,
            fill_attempts,
            slot_fill_blockers,
            command_hints,
            summary,
        }
    }

    pub fn stage_admitted_root(
        mut self,
        slot_kind: SlotKind,
        admitted_receipt_root: impl Into<String>,
        wave92_slot_root: impl Into<String>,
        quorum_count: u16,
        reserve_coverage_bps: u16,
        challenge_hold_blocks_elapsed: u64,
        challenge_hold_clear: bool,
    ) -> Result<Self> {
        self.config.validate()?;
        let binding = AdmittedRootBinding::new(
            slot_kind,
            self.config.wave93_admission_root.clone(),
            admitted_receipt_root,
            wave92_slot_root,
        );
        binding.validate(&self.config)?;
        let attempt = FillAttempt::from_binding(
            binding,
            quorum_count,
            reserve_coverage_bps,
            challenge_hold_blocks_elapsed,
            challenge_hold_clear,
            &self.config,
        );
        self.fill_attempts
            .insert(slot_kind.as_str().to_string(), attempt);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for attempt in self.fill_attempts.values_mut() {
            attempt.recompute(&self.config);
        }
        self.slot_fill_blockers = Self::blockers_from_attempts(&self.fill_attempts);
        self.summary = FillSummary::from_state(&self.fill_attempts, &self.command_hints);
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.config.production_enabled {
            return Err("production denied invariant is broken".to_string());
        }
        if self.config.heavy_gates_ran {
            return Err("heavy gate invariant is broken".to_string());
        }
        for attempt in self.fill_attempts.values() {
            if attempt.heavy_gate_ran {
                return Err("fill attempt claims heavy gate execution".to_string());
            }
        }
        Ok(())
    }

    pub fn blockers_from_attempts(
        fill_attempts: &BTreeMap<String, FillAttempt>,
    ) -> BTreeMap<String, SlotFillBlocker> {
        let mut blockers = BTreeMap::new();
        for attempt in fill_attempts.values() {
            for root in &attempt.blocker_roots {
                let kind = blocker_kind_from_root(attempt.slot_kind, root);
                blockers.insert(root.clone(), SlotFillBlocker::new(attempt.slot_kind, kind));
            }
        }
        blockers
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave94_bridge_custody_receipt_fill_staging",
            "config": self.config.public_record(),
            "fill_attempt_count": self.fill_attempts.len() as u64,
            "fill_attempt_root": self.summary.fill_attempt_root,
            "slot_fill_blocker_count": self.slot_fill_blockers.len() as u64,
            "slot_fill_blocker_root": self.summary.blocker_root,
            "admitted_root_binding_root": self.summary.admitted_binding_root,
            "command_hint_root": self.summary.command_hint_root,
            "summary": self.summary.public_record(),
            "roots_only_public_records": true,
            "live_fill": false,
            "production_enabled": false,
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
        "slot-fill-blocker",
        &json!({
            "slot_kind": slot_kind.as_str(),
            "blocker_kind": blocker_kind.as_str(),
        }),
    )
}

fn blocker_kind_from_root(slot_kind: SlotKind, root: &str) -> BlockerKind {
    for kind in [
        BlockerKind::NoAdmittedRoot,
        BlockerKind::SlotRootMissing,
        BlockerKind::AdmissionRootMismatch,
        BlockerKind::QuorumBelowFloor,
        BlockerKind::ReserveCoverageBelowFloor,
        BlockerKind::ChallengeHoldOpen,
        BlockerKind::ProductionDenied,
        BlockerKind::LiveFillDisabled,
        BlockerKind::HeavyGateClaimRejected,
    ] {
        if blocker_root(slot_kind, kind) == root {
            return kind;
        }
    }
    BlockerKind::SlotRootMissing
}

fn roots_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave94:{}", merkle_root(domain, &leaves))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE94-BRIDGE-CUSTODY-RECEIPT-FILL-STAGING",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave94:{hash}")
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
        "route",
    ] {
        if value.contains(private_marker) {
            return Err(format!("{field} contains private material marker"));
        }
    }
    Ok(())
}
