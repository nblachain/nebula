use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitForceExitWave88ReleaseCaptainGoNoGoReplayDrillTranscriptRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE88_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave88-release-captain-go-no-go-replay-drill-transcript-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE88_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_SUITE: &str = "monero-l2-pq-force-exit-release-captain-go-no-go-replay-v1";
pub const DEFAULT_RELEASE_EPOCH: u64 = 88;
pub const DEFAULT_COMMAND_CHECKLIST_EPOCH: u64 = 87;
pub const DEFAULT_REPLAY_HEIGHT: u64 = 880_000;
pub const DEFAULT_MAX_REPLAY_AGE_BLOCKS: u64 = 80;
pub const DEFAULT_MIN_RELEASE_WEIGHT: u64 = 100;
pub const DEFAULT_MIN_ACCEPTED_LANES: u16 = 6;
pub const DEFAULT_MIN_REPLAY_ITEMS_PER_LANE: u16 = 6;
pub const DEFAULT_MIN_CUSTODY_HOLD_RECEIPTS: u16 = 4;
pub const DEFAULT_MIN_PRIVACY_HOLD_RECEIPTS: u16 = 4;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    CompileRuntime,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileRuntime,
            Self::RuntimeReplay,
            Self::AuditSecurity,
            Self::BridgeCustody,
            Self::WalletWatchtower,
            Self::PqReservePrivacy,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile_runtime",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn captain_owner(self) -> &'static str {
        match self {
            Self::CompileRuntime => "release-captain-runtime",
            Self::RuntimeReplay => "release-captain-replay",
            Self::AuditSecurity => "release-captain-security",
            Self::BridgeCustody => "release-captain-custody",
            Self::WalletWatchtower => "release-captain-watchtower",
            Self::PqReservePrivacy => "release-captain-pq-reserve",
        }
    }

    pub fn requires_custody_hold(self) -> bool {
        matches!(self, Self::BridgeCustody | Self::PqReservePrivacy)
    }

    pub fn requires_privacy_hold(self) -> bool {
        matches!(
            self,
            Self::AuditSecurity | Self::WalletWatchtower | Self::PqReservePrivacy
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayItemKind {
    CommandChecklistRoot,
    CaptainPagerAck,
    GoNoGoCriterion,
    RollbackReplayRoot,
    AbortCommandRoot,
    DeferredHeavyGateRoot,
    BridgeCustodyHold,
    ReserveLiabilityHold,
    WalletWarningHold,
    PrivacyBudgetHold,
    PqSignerPolicyHold,
}

impl ReplayItemKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommandChecklistRoot => "command_checklist_root",
            Self::CaptainPagerAck => "captain_pager_ack",
            Self::GoNoGoCriterion => "go_no_go_criterion",
            Self::RollbackReplayRoot => "rollback_replay_root",
            Self::AbortCommandRoot => "abort_command_root",
            Self::DeferredHeavyGateRoot => "deferred_heavy_gate_root",
            Self::BridgeCustodyHold => "bridge_custody_hold",
            Self::ReserveLiabilityHold => "reserve_liability_hold",
            Self::WalletWarningHold => "wallet_warning_hold",
            Self::PrivacyBudgetHold => "privacy_budget_hold",
            Self::PqSignerPolicyHold => "pq_signer_policy_hold",
        }
    }

    pub fn required_for_lane(self, lane: ReplayLane) -> bool {
        match self {
            Self::CommandChecklistRoot
            | Self::CaptainPagerAck
            | Self::GoNoGoCriterion
            | Self::RollbackReplayRoot
            | Self::AbortCommandRoot
            | Self::DeferredHeavyGateRoot => true,
            Self::BridgeCustodyHold | Self::ReserveLiabilityHold => lane.requires_custody_hold(),
            Self::WalletWarningHold => matches!(lane, ReplayLane::WalletWatchtower),
            Self::PrivacyBudgetHold => lane.requires_privacy_hold(),
            Self::PqSignerPolicyHold => matches!(lane, ReplayLane::PqReservePrivacy),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStatus {
    Missing,
    Draft,
    Held,
    ReplayedWithHold,
    ReadyAfterHeavyGate,
    Rejected,
    Expired,
}

impl ReplayStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Draft => "draft",
            Self::Held => "held",
            Self::ReplayedWithHold => "replayed_with_hold",
            Self::ReadyAfterHeavyGate => "ready_after_heavy_gate",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::ReplayedWithHold | Self::ReadyAfterHeavyGate)
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::ReadyAfterHeavyGate)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptainDecision {
    AcknowledgeHold,
    ReplayAccepted,
    CustodyHoldAccepted,
    PrivacyHoldAccepted,
    MoreEvidenceNeeded,
    RejectUnhold,
    ApproveOnlyAfterHeavyGates,
}

impl CaptainDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcknowledgeHold => "acknowledge_hold",
            Self::ReplayAccepted => "replay_accepted",
            Self::CustodyHoldAccepted => "custody_hold_accepted",
            Self::PrivacyHoldAccepted => "privacy_hold_accepted",
            Self::MoreEvidenceNeeded => "more_evidence_needed",
            Self::RejectUnhold => "reject_unhold",
            Self::ApproveOnlyAfterHeavyGates => "approve_only_after_heavy_gates",
        }
    }

    pub fn contributes_weight(self) -> bool {
        matches!(
            self,
            Self::AcknowledgeHold
                | Self::ReplayAccepted
                | Self::CustodyHoldAccepted
                | Self::PrivacyHoldAccepted
                | Self::ApproveOnlyAfterHeavyGates
        )
    }

    pub fn blocks_unhold(self) -> bool {
        !matches!(self, Self::ApproveOnlyAfterHeavyGates)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayBlockerKind {
    MissingLaneReplay,
    DuplicateLaneReplay,
    EmptyRoot,
    StaleReplay,
    MissingCommandChecklistRoot,
    MissingCaptainPagerAck,
    MissingGoNoGoCriterion,
    MissingRollbackReplayRoot,
    MissingAbortCommandRoot,
    DeferredHeavyGate,
    MissingBridgeCustodyHold,
    MissingReserveLiabilityHold,
    MissingWalletWarningHold,
    MissingPrivacyBudgetHold,
    MissingPqSignerPolicyHold,
    AcceptedLaneCountTooLow,
    ReplayItemCountTooLow,
    ReleaseWeightTooLow,
    CustodyHoldReceiptsTooLow,
    PrivacyHoldReceiptsTooLow,
    CaptainNeedsMoreEvidence,
    CaptainRejectedUnhold,
    ReleaseHoldStillActive,
    FailClosedRequired,
}

impl ReplayBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingLaneReplay => "missing_lane_replay",
            Self::DuplicateLaneReplay => "duplicate_lane_replay",
            Self::EmptyRoot => "empty_root",
            Self::StaleReplay => "stale_replay",
            Self::MissingCommandChecklistRoot => "missing_command_checklist_root",
            Self::MissingCaptainPagerAck => "missing_captain_pager_ack",
            Self::MissingGoNoGoCriterion => "missing_go_no_go_criterion",
            Self::MissingRollbackReplayRoot => "missing_rollback_replay_root",
            Self::MissingAbortCommandRoot => "missing_abort_command_root",
            Self::DeferredHeavyGate => "deferred_heavy_gate",
            Self::MissingBridgeCustodyHold => "missing_bridge_custody_hold",
            Self::MissingReserveLiabilityHold => "missing_reserve_liability_hold",
            Self::MissingWalletWarningHold => "missing_wallet_warning_hold",
            Self::MissingPrivacyBudgetHold => "missing_privacy_budget_hold",
            Self::MissingPqSignerPolicyHold => "missing_pq_signer_policy_hold",
            Self::AcceptedLaneCountTooLow => "accepted_lane_count_too_low",
            Self::ReplayItemCountTooLow => "replay_item_count_too_low",
            Self::ReleaseWeightTooLow => "release_weight_too_low",
            Self::CustodyHoldReceiptsTooLow => "custody_hold_receipts_too_low",
            Self::PrivacyHoldReceiptsTooLow => "privacy_hold_receipts_too_low",
            Self::CaptainNeedsMoreEvidence => "captain_needs_more_evidence",
            Self::CaptainRejectedUnhold => "captain_rejected_unhold",
            Self::ReleaseHoldStillActive => "release_hold_still_active",
            Self::FailClosedRequired => "fail_closed_required",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::DeferredHeavyGate | Self::ReleaseHoldStillActive | Self::FailClosedRequired => 2,
            Self::ReleaseWeightTooLow
            | Self::CustodyHoldReceiptsTooLow
            | Self::PrivacyHoldReceiptsTooLow
            | Self::CaptainNeedsMoreEvidence
            | Self::CaptainRejectedUnhold => 3,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub replay_suite: String,
    pub release_epoch: u64,
    pub command_checklist_epoch: u64,
    pub replay_height: u64,
    pub release_channel: String,
    pub captain_room_id: String,
    pub replay_policy_id: String,
    pub max_replay_age_blocks: u64,
    pub min_release_weight: u64,
    pub min_accepted_lanes: u16,
    pub min_replay_items_per_lane: u16,
    pub min_custody_hold_receipts: u16,
    pub min_privacy_hold_receipts: u16,
    pub required_lanes: Vec<ReplayLane>,
    pub require_deferred_heavy_gate_root: bool,
    pub require_custody_holds: bool,
    pub require_privacy_holds: bool,
    pub require_fail_closed_default: bool,
    pub allow_go_without_heavy_gates: bool,
}

impl Default for Config {
    fn default() -> Self {
        let captain_room_id = stable_id("captain-room", "wave-88-go-no-go", 1);
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_suite: REPLAY_SUITE.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            command_checklist_epoch: DEFAULT_COMMAND_CHECKLIST_EPOCH,
            replay_height: DEFAULT_REPLAY_HEIGHT,
            release_channel: "devnet-force-exit-release-captain-go-no-go-replay".to_string(),
            captain_room_id: captain_room_id.clone(),
            replay_policy_id: stable_id("go-no-go-replay-policy", &captain_room_id, 1),
            max_replay_age_blocks: DEFAULT_MAX_REPLAY_AGE_BLOCKS,
            min_release_weight: DEFAULT_MIN_RELEASE_WEIGHT,
            min_accepted_lanes: DEFAULT_MIN_ACCEPTED_LANES,
            min_replay_items_per_lane: DEFAULT_MIN_REPLAY_ITEMS_PER_LANE,
            min_custody_hold_receipts: DEFAULT_MIN_CUSTODY_HOLD_RECEIPTS,
            min_privacy_hold_receipts: DEFAULT_MIN_PRIVACY_HOLD_RECEIPTS,
            required_lanes: ReplayLane::all(),
            require_deferred_heavy_gate_root: true,
            require_custody_holds: true,
            require_privacy_holds: true,
            require_fail_closed_default: true,
            allow_go_without_heavy_gates: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("replay_suite", &self.replay_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure_non_empty("captain_room_id", &self.captain_room_id)?;
        ensure_non_empty("replay_policy_id", &self.replay_policy_id)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.release_epoch > 0, "release epoch must be non-zero")?;
        ensure(
            self.release_epoch > self.command_checklist_epoch,
            "release epoch must follow command checklist epoch",
        )?;
        ensure(self.replay_height > 0, "replay height must be non-zero")?;
        ensure(
            self.max_replay_age_blocks > 0,
            "max replay age must be non-zero",
        )?;
        ensure(
            self.min_release_weight > 0,
            "min release weight must be non-zero",
        )?;
        ensure(
            self.min_accepted_lanes > 0,
            "min accepted lanes must be non-zero",
        )?;
        ensure(
            !self.required_lanes.is_empty(),
            "required lanes must be non-empty",
        )?;
        let mut seen = BTreeSet::new();
        for lane in &self.required_lanes {
            ensure(seen.insert(*lane), "required lanes must be unique")?;
        }
        if self.require_fail_closed_default {
            ensure(
                !self.allow_go_without_heavy_gates,
                "fail-closed mode cannot allow go without heavy gates",
            )?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "replay_suite": self.replay_suite,
            "release_epoch": self.release_epoch,
            "command_checklist_epoch": self.command_checklist_epoch,
            "replay_height": self.replay_height,
            "release_channel": self.release_channel,
            "captain_room_id": self.captain_room_id,
            "replay_policy_id": self.replay_policy_id,
            "max_replay_age_blocks": self.max_replay_age_blocks,
            "min_release_weight": self.min_release_weight,
            "min_accepted_lanes": self.min_accepted_lanes,
            "min_replay_items_per_lane": self.min_replay_items_per_lane,
            "min_custody_hold_receipts": self.min_custody_hold_receipts,
            "min_privacy_hold_receipts": self.min_privacy_hold_receipts,
            "required_lanes": self.required_lanes.iter().map(|lane| lane.as_str()).collect::<Vec<_>>(),
            "require_deferred_heavy_gate_root": self.require_deferred_heavy_gate_root,
            "require_custody_holds": self.require_custody_holds,
            "require_privacy_holds": self.require_privacy_holds,
            "require_fail_closed_default": self.require_fail_closed_default,
            "allow_go_without_heavy_gates": self.allow_go_without_heavy_gates,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayItem {
    pub item_id: String,
    pub lane: ReplayLane,
    pub kind: ReplayItemKind,
    pub evidence_root: String,
    pub owner_id: String,
    pub replayed: bool,
    pub release_blocking: bool,
    pub privacy_safe: bool,
}

impl ReplayItem {
    pub fn devnet(lane: ReplayLane, kind: ReplayItemKind, ordinal: u64) -> Self {
        let label = format!("{}-{}", lane.as_str(), kind.as_str());
        let release_blocking = matches!(kind, ReplayItemKind::DeferredHeavyGateRoot);
        Self {
            item_id: stable_id("go-no-go-replay-item", &label, ordinal),
            lane,
            kind,
            evidence_root: sample_root("go-no-go-replay-item", &label, ordinal),
            owner_id: lane.captain_owner().to_string(),
            replayed: !release_blocking,
            release_blocking,
            privacy_safe: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("item_id", &self.item_id)?;
        ensure_non_empty("evidence_root", &self.evidence_root)?;
        ensure_non_empty("owner_id", &self.owner_id)?;
        ensure(self.privacy_safe, "replay item must be privacy safe")?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "item_id": self.item_id,
            "lane": self.lane.as_str(),
            "kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "owner_id": self.owner_id,
            "replayed": self.replayed,
            "release_blocking": self.release_blocking,
            "privacy_safe": self.privacy_safe,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay-item", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneGoNoGoReplay {
    pub lane: ReplayLane,
    pub release_captain: String,
    pub command_checklist_root: String,
    pub go_no_go_criterion_root: String,
    pub rollback_replay_root: String,
    pub abort_command_root: String,
    pub custody_hold_root: Option<String>,
    pub reserve_hold_root: Option<String>,
    pub wallet_warning_root: Option<String>,
    pub privacy_hold_root: Option<String>,
    pub pq_signer_policy_root: Option<String>,
    pub deferred_heavy_gate_root: String,
    pub observed_at_height: u64,
    pub item_count: u16,
    pub accepted_item_count: u16,
    pub status: ReplayStatus,
    pub fail_closed: bool,
}

impl LaneGoNoGoReplay {
    pub fn devnet(lane: ReplayLane, config: &Config, ordinal: u64) -> Self {
        let label = lane.as_str();
        let observed_at_height = config
            .replay_height
            .saturating_sub(config.max_replay_age_blocks / 3)
            .saturating_add(ordinal);
        let item_count = config.min_replay_items_per_lane.saturating_add(
            if lane.requires_custody_hold() { 2 } else { 0 }
                + if lane.requires_privacy_hold() { 1 } else { 0 },
        );
        Self {
            lane,
            release_captain: lane.captain_owner().to_string(),
            command_checklist_root: sample_root("command-checklist-root", label, ordinal),
            go_no_go_criterion_root: sample_root("go-no-go-criterion", label, ordinal),
            rollback_replay_root: sample_root("rollback-replay-root", label, ordinal),
            abort_command_root: sample_root("abort-command-root", label, ordinal),
            custody_hold_root: lane
                .requires_custody_hold()
                .then(|| sample_root("custody-hold-root", label, ordinal)),
            reserve_hold_root: lane
                .requires_custody_hold()
                .then(|| sample_root("reserve-hold-root", label, ordinal)),
            wallet_warning_root: matches!(lane, ReplayLane::WalletWatchtower)
                .then(|| sample_root("wallet-warning-root", label, ordinal)),
            privacy_hold_root: lane
                .requires_privacy_hold()
                .then(|| sample_root("privacy-hold-root", label, ordinal)),
            pq_signer_policy_root: matches!(lane, ReplayLane::PqReservePrivacy)
                .then(|| sample_root("pq-signer-policy-root", label, ordinal)),
            deferred_heavy_gate_root: sample_root("deferred-heavy-gate-root", label, ordinal),
            observed_at_height,
            item_count,
            accepted_item_count: config.min_replay_items_per_lane,
            status: ReplayStatus::ReplayedWithHold,
            fail_closed: true,
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        ensure_non_empty("release_captain", &self.release_captain)?;
        ensure_non_empty("command_checklist_root", &self.command_checklist_root)?;
        ensure_non_empty("go_no_go_criterion_root", &self.go_no_go_criterion_root)?;
        ensure_non_empty("rollback_replay_root", &self.rollback_replay_root)?;
        ensure_non_empty("abort_command_root", &self.abort_command_root)?;
        ensure_non_empty("deferred_heavy_gate_root", &self.deferred_heavy_gate_root)?;
        ensure(
            self.observed_at_height <= height,
            "go/no-go replay cannot be observed after state height",
        )?;
        ensure(
            self.item_count >= config.min_replay_items_per_lane,
            "replay item count below minimum",
        )?;
        if self.lane.requires_custody_hold() {
            ensure(
                option_root_present(&self.custody_hold_root),
                "custody lane requires custody hold root",
            )?;
            ensure(
                option_root_present(&self.reserve_hold_root),
                "custody lane requires reserve hold root",
            )?;
        }
        if self.lane.requires_privacy_hold() {
            ensure(
                option_root_present(&self.privacy_hold_root),
                "privacy lane requires privacy hold root",
            )?;
        }
        Ok(())
    }

    pub fn blockers(&self, config: &Config, height: u64) -> Vec<ReplayBlockerKind> {
        let mut blockers = Vec::new();
        if self
            .observed_at_height
            .saturating_add(config.max_replay_age_blocks)
            < height
        {
            blockers.push(ReplayBlockerKind::StaleReplay);
        }
        if config.require_deferred_heavy_gate_root {
            blockers.push(ReplayBlockerKind::DeferredHeavyGate);
        }
        if self.lane.requires_custody_hold() {
            if !option_root_present(&self.custody_hold_root) {
                blockers.push(ReplayBlockerKind::MissingBridgeCustodyHold);
            }
            if !option_root_present(&self.reserve_hold_root) {
                blockers.push(ReplayBlockerKind::MissingReserveLiabilityHold);
            }
        }
        if matches!(self.lane, ReplayLane::WalletWatchtower)
            && !option_root_present(&self.wallet_warning_root)
        {
            blockers.push(ReplayBlockerKind::MissingWalletWarningHold);
        }
        if self.lane.requires_privacy_hold() && !option_root_present(&self.privacy_hold_root) {
            blockers.push(ReplayBlockerKind::MissingPrivacyBudgetHold);
        }
        if matches!(self.lane, ReplayLane::PqReservePrivacy)
            && !option_root_present(&self.pq_signer_policy_root)
        {
            blockers.push(ReplayBlockerKind::MissingPqSignerPolicyHold);
        }
        if self.item_count < config.min_replay_items_per_lane {
            blockers.push(ReplayBlockerKind::ReplayItemCountTooLow);
        }
        if self.status.blocks_release() {
            blockers.push(ReplayBlockerKind::ReleaseHoldStillActive);
        }
        if config.require_fail_closed_default && self.fail_closed {
            blockers.push(ReplayBlockerKind::FailClosedRequired);
        }
        blockers
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane.as_str(),
            "release_captain": self.release_captain,
            "command_checklist_root": self.command_checklist_root,
            "go_no_go_criterion_root": self.go_no_go_criterion_root,
            "rollback_replay_root": self.rollback_replay_root,
            "abort_command_root": self.abort_command_root,
            "custody_hold_root": self.custody_hold_root,
            "reserve_hold_root": self.reserve_hold_root,
            "wallet_warning_root": self.wallet_warning_root,
            "privacy_hold_root": self.privacy_hold_root,
            "pq_signer_policy_root": self.pq_signer_policy_root,
            "deferred_heavy_gate_root": self.deferred_heavy_gate_root,
            "observed_at_height": self.observed_at_height,
            "item_count": self.item_count,
            "accepted_item_count": self.accepted_item_count,
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-go-no-go-replay", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CaptainSignoff {
    pub captain_id: String,
    pub role: String,
    pub lane_scope: Option<ReplayLane>,
    pub weight: u64,
    pub decision: CaptainDecision,
    pub signed_replay_root: String,
    pub signed_at_height: u64,
}

impl CaptainSignoff {
    pub fn devnet(
        captain_id: &str,
        role: &str,
        lane_scope: Option<ReplayLane>,
        weight: u64,
        decision: CaptainDecision,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        Self {
            captain_id: captain_id.to_string(),
            role: role.to_string(),
            lane_scope,
            weight,
            decision,
            signed_replay_root: sample_root("captain-signoff", captain_id, ordinal),
            signed_at_height: config
                .replay_height
                .saturating_sub(6)
                .saturating_add(ordinal),
        }
    }

    pub fn validate(&self, height: u64) -> Result<()> {
        ensure_non_empty("captain_id", &self.captain_id)?;
        ensure_non_empty("role", &self.role)?;
        ensure_non_empty("signed_replay_root", &self.signed_replay_root)?;
        ensure(self.weight > 0, "captain signoff weight must be non-zero")?;
        ensure(
            self.signed_at_height <= height,
            "captain signoff cannot be after state height",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "captain_id": self.captain_id,
            "role": self.role,
            "lane_scope": self.lane_scope.map(|lane| lane.as_str()),
            "weight": self.weight,
            "decision": self.decision.as_str(),
            "signed_replay_root": self.signed_replay_root,
            "signed_at_height": self.signed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("captain-signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GoNoGoSummary {
    pub state: String,
    pub accepted_lane_count: u16,
    pub replay_item_count: u16,
    pub blocker_count: u16,
    pub max_blocker_severity: u8,
    pub release_weight: u64,
    pub hold_active: bool,
    pub fail_closed: bool,
    pub go_ready: bool,
    pub summary_root: String,
}

impl GoNoGoSummary {
    pub fn build(
        config: &Config,
        lane_replays: &[LaneGoNoGoReplay],
        replay_items: &[ReplayItem],
        signoffs: &[CaptainSignoff],
        blockers: &BTreeMap<String, Vec<ReplayBlockerKind>>,
    ) -> Self {
        let accepted_lane_count = lane_replays
            .iter()
            .filter(|lane| lane.status.accepted())
            .count() as u16;
        let replay_item_count = replay_items.len() as u16;
        let blocker_count = blockers.values().map(|items| items.len()).sum::<usize>() as u16;
        let max_blocker_severity = match blockers
            .values()
            .flat_map(|items| items.iter())
            .map(|blocker| blocker.severity())
            .max()
        {
            Some(severity) => severity,
            None => 0,
        };
        let release_weight = signoffs
            .iter()
            .filter(|signoff| signoff.decision.contributes_weight())
            .map(|signoff| signoff.weight)
            .sum::<u64>();
        let hold_active = lane_replays.iter().any(|lane| lane.status.blocks_release());
        let fail_closed = config.require_fail_closed_default || hold_active;
        let go_ready = blocker_count == 0
            && accepted_lane_count >= config.min_accepted_lanes
            && release_weight >= config.min_release_weight
            && !hold_active;
        let state = if go_ready {
            "go_after_heavy_gates"
        } else if fail_closed {
            "no_go_fail_closed"
        } else {
            "pending_replay_review"
        }
        .to_string();
        let summary_root = domain_hash(
            "wave88-go-no-go-summary-root",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.replay_policy_id),
                HashPart::U64(accepted_lane_count as u64),
                HashPart::U64(replay_item_count as u64),
                HashPart::U64(blocker_count as u64),
                HashPart::U64(release_weight),
                HashPart::Json(&json!({
                    "hold_active": hold_active,
                    "fail_closed": fail_closed,
                    "go_ready": go_ready,
                })),
            ],
            32,
        );
        Self {
            state,
            accepted_lane_count,
            replay_item_count,
            blocker_count,
            max_blocker_severity,
            release_weight,
            hold_active,
            fail_closed,
            go_ready,
            summary_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "state": self.state,
            "accepted_lane_count": self.accepted_lane_count,
            "replay_item_count": self.replay_item_count,
            "blocker_count": self.blocker_count,
            "max_blocker_severity": self.max_blocker_severity,
            "release_weight": self.release_weight,
            "hold_active": self.hold_active,
            "fail_closed": self.fail_closed,
            "go_ready": self.go_ready,
            "summary_root": self.summary_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub lane_replays: Vec<LaneGoNoGoReplay>,
    pub replay_items: Vec<ReplayItem>,
    pub captain_signoffs: Vec<CaptainSignoff>,
    pub blockers: BTreeMap<String, Vec<ReplayBlockerKind>>,
    pub lane_replay_root: String,
    pub replay_item_root: String,
    pub captain_root: String,
    pub blocker_root: String,
    pub summary: GoNoGoSummary,
}

impl State {
    pub fn new(
        config: Config,
        height: u64,
        lane_replays: Vec<LaneGoNoGoReplay>,
        replay_items: Vec<ReplayItem>,
        captain_signoffs: Vec<CaptainSignoff>,
    ) -> Result<Self> {
        config.validate()?;
        ensure(height > 0, "state height must be non-zero")?;
        ensure(!lane_replays.is_empty(), "state must include lane replays")?;
        ensure(!replay_items.is_empty(), "state must include replay items")?;
        ensure(
            !captain_signoffs.is_empty(),
            "state must include captain signoffs",
        )?;
        for replay in &lane_replays {
            replay.validate(&config, height)?;
        }
        for item in &replay_items {
            item.validate()?;
        }
        for signoff in &captain_signoffs {
            signoff.validate(height)?;
        }
        let blockers = evaluate_blockers(
            &config,
            height,
            &lane_replays,
            &replay_items,
            &captain_signoffs,
        );
        let lane_replay_root = roots_root(
            "wave88-go-no-go-lane-replays",
            lane_replays.iter().map(LaneGoNoGoReplay::state_root),
        );
        let replay_item_root = roots_root(
            "wave88-go-no-go-replay-items",
            replay_items.iter().map(ReplayItem::state_root),
        );
        let captain_root = roots_root(
            "wave88-go-no-go-captain-signoffs",
            captain_signoffs.iter().map(CaptainSignoff::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary = GoNoGoSummary::build(
            &config,
            &lane_replays,
            &replay_items,
            &captain_signoffs,
            &blockers,
        );
        Ok(Self {
            config,
            height,
            lane_replays,
            replay_items,
            captain_signoffs,
            blockers,
            lane_replay_root,
            replay_item_root,
            captain_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let height = config.replay_height;
        let lanes = ReplayLane::all();
        let lane_replays = lanes
            .iter()
            .enumerate()
            .map(|(index, lane)| LaneGoNoGoReplay::devnet(*lane, &config, one_based(index)))
            .collect::<Vec<_>>();
        let mut replay_items = Vec::new();
        for (lane_index, lane) in lanes.iter().enumerate() {
            let ordinal_base = one_based(lane_index) * 20;
            let kinds = [
                ReplayItemKind::CommandChecklistRoot,
                ReplayItemKind::CaptainPagerAck,
                ReplayItemKind::GoNoGoCriterion,
                ReplayItemKind::RollbackReplayRoot,
                ReplayItemKind::AbortCommandRoot,
                ReplayItemKind::DeferredHeavyGateRoot,
                ReplayItemKind::BridgeCustodyHold,
                ReplayItemKind::ReserveLiabilityHold,
                ReplayItemKind::WalletWarningHold,
                ReplayItemKind::PrivacyBudgetHold,
                ReplayItemKind::PqSignerPolicyHold,
            ];
            for (kind_index, kind) in kinds.into_iter().enumerate() {
                if kind.required_for_lane(*lane) {
                    replay_items.push(ReplayItem::devnet(
                        *lane,
                        kind,
                        ordinal_base + one_based(kind_index),
                    ));
                }
            }
        }
        let captain_signoffs = vec![
            CaptainSignoff::devnet(
                "release-captain-runtime",
                "compile-runtime-replay",
                Some(ReplayLane::CompileRuntime),
                18,
                CaptainDecision::ReplayAccepted,
                &config,
                1,
            ),
            CaptainSignoff::devnet(
                "release-captain-replay",
                "runtime-replay",
                Some(ReplayLane::RuntimeReplay),
                16,
                CaptainDecision::ReplayAccepted,
                &config,
                2,
            ),
            CaptainSignoff::devnet(
                "release-captain-security",
                "audit-security",
                Some(ReplayLane::AuditSecurity),
                20,
                CaptainDecision::PrivacyHoldAccepted,
                &config,
                3,
            ),
            CaptainSignoff::devnet(
                "release-captain-custody",
                "bridge-custody",
                Some(ReplayLane::BridgeCustody),
                22,
                CaptainDecision::CustodyHoldAccepted,
                &config,
                4,
            ),
            CaptainSignoff::devnet(
                "release-captain-watchtower",
                "wallet-watchtower",
                Some(ReplayLane::WalletWatchtower),
                16,
                CaptainDecision::AcknowledgeHold,
                &config,
                5,
            ),
            CaptainSignoff::devnet(
                "release-captain-pq-reserve",
                "pq-reserve-privacy",
                Some(ReplayLane::PqReservePrivacy),
                22,
                CaptainDecision::PrivacyHoldAccepted,
                &config,
                6,
            ),
            CaptainSignoff::devnet(
                "release-captain",
                "incident-command",
                None,
                25,
                CaptainDecision::MoreEvidenceNeeded,
                &config,
                7,
            ),
        ];
        match Self::new(config, height, lane_replays, replay_items, captain_signoffs) {
            Ok(state) => state,
            Err(reason) => build_devnet_fail_closed_fallback(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "config": self.config.public_record(),
            "lane_replay_root": self.lane_replay_root,
            "replay_item_root": self.replay_item_root,
            "captain_root": self.captain_root,
            "blocker_root": self.blocker_root,
            "summary": self.summary.public_record(),
            "lane_replays": self.lane_replays.iter().map(LaneGoNoGoReplay::public_record).collect::<Vec<_>>(),
            "replay_items": self.replay_items.iter().map(ReplayItem::public_record).collect::<Vec<_>>(),
            "captain_signoffs": self.captain_signoffs.iter().map(CaptainSignoff::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(|(subject, blockers)| {
                let max_severity = match blockers.iter().map(|blocker| blocker.severity()).max() {
                    Some(severity) => severity,
                    None => 0,
                };
                json!({
                    "subject": subject,
                    "blockers": blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                    "max_severity": max_severity,
                })
            }).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "wave88-go-no-go-state-root",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.replay_policy_id),
                HashPart::U64(self.height),
                HashPart::Str(&self.lane_replay_root),
                HashPart::Str(&self.replay_item_root),
                HashPart::Str(&self.captain_root),
                HashPart::Str(&self.blocker_root),
                HashPart::Str(&self.summary.summary_root),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn evaluate_blockers(
    config: &Config,
    height: u64,
    lane_replays: &[LaneGoNoGoReplay],
    replay_items: &[ReplayItem],
    signoffs: &[CaptainSignoff],
) -> BTreeMap<String, Vec<ReplayBlockerKind>> {
    let mut blockers = BTreeMap::<String, Vec<ReplayBlockerKind>>::new();
    let mut seen = BTreeSet::new();
    for replay in lane_replays {
        let key = replay.lane.as_str().to_string();
        if !seen.insert(replay.lane) {
            blockers
                .entry(key.clone())
                .or_default()
                .push(ReplayBlockerKind::DuplicateLaneReplay);
        }
        let lane_blockers = replay.blockers(config, height);
        if !lane_blockers.is_empty() {
            blockers.entry(key).or_default().extend(lane_blockers);
        }
    }
    for lane in &config.required_lanes {
        if !seen.contains(lane) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ReplayBlockerKind::MissingLaneReplay);
        }
    }
    let accepted_lane_count = lane_replays
        .iter()
        .filter(|replay| replay.status.accepted())
        .count() as u16;
    if accepted_lane_count < config.min_accepted_lanes {
        blockers
            .entry("lane_quorum".to_string())
            .or_default()
            .push(ReplayBlockerKind::AcceptedLaneCountTooLow);
    }
    for lane in &config.required_lanes {
        let item_count = replay_items
            .iter()
            .filter(|item| item.lane == *lane)
            .count() as u16;
        if item_count < config.min_replay_items_per_lane {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ReplayBlockerKind::ReplayItemCountTooLow);
        }
    }
    let release_weight = signoffs
        .iter()
        .filter(|signoff| signoff.decision.contributes_weight())
        .map(|signoff| signoff.weight)
        .sum::<u64>();
    if release_weight < config.min_release_weight {
        blockers
            .entry("release_weight".to_string())
            .or_default()
            .push(ReplayBlockerKind::ReleaseWeightTooLow);
    }
    let custody_hold_count = signoffs
        .iter()
        .filter(|signoff| match signoff.lane_scope {
            Some(lane) => lane.requires_custody_hold(),
            None => false,
        })
        .count() as u16;
    if config.require_custody_holds && custody_hold_count < config.min_custody_hold_receipts {
        blockers
            .entry("custody_holds".to_string())
            .or_default()
            .push(ReplayBlockerKind::CustodyHoldReceiptsTooLow);
    }
    let privacy_hold_count = signoffs
        .iter()
        .filter(|signoff| match signoff.lane_scope {
            Some(lane) => lane.requires_privacy_hold(),
            None => false,
        })
        .count() as u16;
    if config.require_privacy_holds && privacy_hold_count < config.min_privacy_hold_receipts {
        blockers
            .entry("privacy_holds".to_string())
            .or_default()
            .push(ReplayBlockerKind::PrivacyHoldReceiptsTooLow);
    }
    for signoff in signoffs {
        if signoff.decision.blocks_unhold() {
            let blocker = match signoff.decision {
                CaptainDecision::RejectUnhold => ReplayBlockerKind::CaptainRejectedUnhold,
                CaptainDecision::MoreEvidenceNeeded => ReplayBlockerKind::CaptainNeedsMoreEvidence,
                CaptainDecision::AcknowledgeHold
                | CaptainDecision::ReplayAccepted
                | CaptainDecision::CustodyHoldAccepted
                | CaptainDecision::PrivacyHoldAccepted
                | CaptainDecision::ApproveOnlyAfterHeavyGates => {
                    ReplayBlockerKind::ReleaseHoldStillActive
                }
            };
            blockers
                .entry(signoff.captain_id.clone())
                .or_default()
                .push(blocker);
        }
    }
    blockers
}

fn build_devnet_fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let height = config.replay_height;
    let lane_replays = ReplayLane::all()
        .into_iter()
        .enumerate()
        .map(|(index, lane)| LaneGoNoGoReplay {
            status: ReplayStatus::Held,
            fail_closed: true,
            item_count: 0,
            accepted_item_count: 0,
            ..LaneGoNoGoReplay::devnet(lane, &config, one_based(index))
        })
        .collect::<Vec<_>>();
    let replay_items = vec![ReplayItem::devnet(
        ReplayLane::CompileRuntime,
        ReplayItemKind::DeferredHeavyGateRoot,
        1,
    )];
    let captain_signoffs = vec![CaptainSignoff::devnet(
        "release-captain",
        "incident-command",
        None,
        1,
        CaptainDecision::MoreEvidenceNeeded,
        &config,
        1,
    )];
    let mut blockers = evaluate_blockers(
        &config,
        height,
        &lane_replays,
        &replay_items,
        &captain_signoffs,
    );
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(ReplayBlockerKind::FailClosedRequired);
    blockers
        .entry("fallback_reason".to_string())
        .or_default()
        .push(if reason.trim().is_empty() {
            ReplayBlockerKind::EmptyRoot
        } else {
            ReplayBlockerKind::ReleaseHoldStillActive
        });
    let lane_replay_root = roots_root(
        "wave88-go-no-go-lane-replays",
        lane_replays.iter().map(LaneGoNoGoReplay::state_root),
    );
    let replay_item_root = roots_root(
        "wave88-go-no-go-replay-items",
        replay_items.iter().map(ReplayItem::state_root),
    );
    let captain_root = roots_root(
        "wave88-go-no-go-captain-signoffs",
        captain_signoffs.iter().map(CaptainSignoff::state_root),
    );
    let blocker_root = blockers_root(&blockers);
    let summary = GoNoGoSummary::build(
        &config,
        &lane_replays,
        &replay_items,
        &captain_signoffs,
        &blockers,
    );
    State {
        config,
        height,
        lane_replays,
        replay_items,
        captain_signoffs,
        blockers,
        lane_replay_root,
        replay_item_root,
        captain_root,
        blocker_root,
        summary,
    }
}

fn option_root_present(root: &Option<String>) -> bool {
    match root {
        Some(value) => !value.trim().is_empty(),
        None => false,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<ReplayBlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            let max_severity = match blocker_list.iter().map(|blocker| blocker.severity()).max() {
                Some(severity) => severity,
                None => 0,
            };
            json!({
                "subject": subject,
                "blockers": blocker_list.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                "max_severity": max_severity,
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave88-go-no-go-blockers", &leaves)
}

fn roots_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE88-GO-NO-GO-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn stable_id(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE88-GO-NO-GO-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn sample_root(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE88-GO-NO-GO-SAMPLE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn one_based(index: usize) -> u64 {
    index as u64 + 1
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{label} must be non-empty"),
    )
}
