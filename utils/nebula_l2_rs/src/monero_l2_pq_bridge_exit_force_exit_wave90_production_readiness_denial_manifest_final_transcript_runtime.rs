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
pub type MoneroL2PqBridgeExitForceExitWave90ProductionReadinessDenialManifestFinalTranscriptRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE90_PRODUCTION_READINESS_DENIAL_MANIFEST_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave90-production-readiness-denial-manifest-final-transcript-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE90_PRODUCTION_READINESS_DENIAL_MANIFEST_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DENIAL_SUITE: &str = "monero-l2-pq-force-exit-production-readiness-denial-manifest-v1";
pub const DEFAULT_WAVE: u64 = 90;
pub const DEFAULT_SOURCE_ARCHIVE_WAVE: u64 = 89;
pub const DEFAULT_SOURCE_REPLAY_WAVE: u64 = 88;
pub const DEFAULT_COMMAND_CHECKLIST_WAVE: u64 = 87;
pub const DEFAULT_DENIAL_HEIGHT: u64 = 900_000;
pub const DEFAULT_MAX_ARCHIVE_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_DENIED_LANES: u16 = 6;
pub const DEFAULT_MIN_DENIAL_ITEMS: u16 = 48;
pub const DEFAULT_MIN_OPERATOR_ACTIONS: u16 = 6;
pub const DEFAULT_MIN_BLOCKING_WEIGHT: u64 = 120;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialLane {
    CompileBlocker,
    RuntimeReplayBlocker,
    AuditSecurityBlocker,
    BridgeCustodyBlocker,
    WalletWatchtowerBlocker,
    PqReservePrivacyBlocker,
}

impl DenialLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileBlocker,
            Self::RuntimeReplayBlocker,
            Self::AuditSecurityBlocker,
            Self::BridgeCustodyBlocker,
            Self::WalletWatchtowerBlocker,
            Self::PqReservePrivacyBlocker,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileBlocker => "compile_blocker",
            Self::RuntimeReplayBlocker => "runtime_replay_blocker",
            Self::AuditSecurityBlocker => "audit_security_blocker",
            Self::BridgeCustodyBlocker => "bridge_custody_blocker",
            Self::WalletWatchtowerBlocker => "wallet_watchtower_blocker",
            Self::PqReservePrivacyBlocker => "pq_reserve_privacy_blocker",
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::CompileBlocker => "release-captain-compile-denial",
            Self::RuntimeReplayBlocker => "release-captain-runtime-denial",
            Self::AuditSecurityBlocker => "release-captain-security-denial",
            Self::BridgeCustodyBlocker => "release-captain-custody-denial",
            Self::WalletWatchtowerBlocker => "release-captain-wallet-denial",
            Self::PqReservePrivacyBlocker => "release-captain-pq-reserve-privacy-denial",
        }
    }

    pub fn requires_live_runtime_receipt(self) -> bool {
        matches!(self, Self::CompileBlocker | Self::RuntimeReplayBlocker)
    }

    pub fn requires_live_custody_receipt(self) -> bool {
        matches!(
            self,
            Self::BridgeCustodyBlocker | Self::PqReservePrivacyBlocker
        )
    }

    pub fn requires_live_privacy_receipt(self) -> bool {
        matches!(
            self,
            Self::AuditSecurityBlocker
                | Self::WalletWatchtowerBlocker
                | Self::PqReservePrivacyBlocker
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialCriterion {
    Wave89NoGoArchiveRoot,
    Wave88GoNoGoReplayRoot,
    CargoCheckReceipt,
    CargoTestReceipt,
    ClippyReceipt,
    RuntimeReplayReceipt,
    RollbackDrillReceipt,
    AdversarialReplayReceipt,
    AuditReviewerReceipt,
    ThreatModelReceipt,
    BridgeCustodyReceipt,
    WatchtowerQuorumReceipt,
    WalletEscapeReceipt,
    PqSignerEpochReceipt,
    ReserveCoverageReceipt,
    PrivacyRedactionReceipt,
    NullifierSeparationReceipt,
    ProductionApprovalReceipt,
}

impl DenialCriterion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave89NoGoArchiveRoot => "wave89_no_go_archive_root",
            Self::Wave88GoNoGoReplayRoot => "wave88_go_no_go_replay_root",
            Self::CargoCheckReceipt => "cargo_check_receipt",
            Self::CargoTestReceipt => "cargo_test_receipt",
            Self::ClippyReceipt => "clippy_receipt",
            Self::RuntimeReplayReceipt => "runtime_replay_receipt",
            Self::RollbackDrillReceipt => "rollback_drill_receipt",
            Self::AdversarialReplayReceipt => "adversarial_replay_receipt",
            Self::AuditReviewerReceipt => "audit_reviewer_receipt",
            Self::ThreatModelReceipt => "threat_model_receipt",
            Self::BridgeCustodyReceipt => "bridge_custody_receipt",
            Self::WatchtowerQuorumReceipt => "watchtower_quorum_receipt",
            Self::WalletEscapeReceipt => "wallet_escape_receipt",
            Self::PqSignerEpochReceipt => "pq_signer_epoch_receipt",
            Self::ReserveCoverageReceipt => "reserve_coverage_receipt",
            Self::PrivacyRedactionReceipt => "privacy_redaction_receipt",
            Self::NullifierSeparationReceipt => "nullifier_separation_receipt",
            Self::ProductionApprovalReceipt => "production_approval_receipt",
        }
    }

    pub fn required_for_lane(self, lane: DenialLane) -> bool {
        match self {
            Self::Wave89NoGoArchiveRoot
            | Self::Wave88GoNoGoReplayRoot
            | Self::ProductionApprovalReceipt => true,
            Self::CargoCheckReceipt | Self::CargoTestReceipt | Self::ClippyReceipt => {
                matches!(lane, DenialLane::CompileBlocker)
            }
            Self::RuntimeReplayReceipt
            | Self::RollbackDrillReceipt
            | Self::AdversarialReplayReceipt => matches!(lane, DenialLane::RuntimeReplayBlocker),
            Self::AuditReviewerReceipt | Self::ThreatModelReceipt => {
                matches!(lane, DenialLane::AuditSecurityBlocker)
            }
            Self::BridgeCustodyReceipt => lane.requires_live_custody_receipt(),
            Self::WatchtowerQuorumReceipt | Self::WalletEscapeReceipt => {
                matches!(lane, DenialLane::WalletWatchtowerBlocker)
            }
            Self::PqSignerEpochReceipt | Self::ReserveCoverageReceipt => {
                matches!(lane, DenialLane::PqReservePrivacyBlocker)
            }
            Self::PrivacyRedactionReceipt | Self::NullifierSeparationReceipt => {
                lane.requires_live_privacy_receipt()
            }
        }
    }

    pub fn blocking_weight(self) -> u64 {
        match self {
            Self::Wave89NoGoArchiveRoot | Self::Wave88GoNoGoReplayRoot => 10,
            Self::ProductionApprovalReceipt => 24,
            Self::CargoCheckReceipt | Self::CargoTestReceipt | Self::ClippyReceipt => 18,
            Self::RuntimeReplayReceipt
            | Self::RollbackDrillReceipt
            | Self::AdversarialReplayReceipt => 20,
            Self::AuditReviewerReceipt | Self::ThreatModelReceipt => 22,
            Self::BridgeCustodyReceipt
            | Self::WatchtowerQuorumReceipt
            | Self::WalletEscapeReceipt => 21,
            Self::PqSignerEpochReceipt
            | Self::ReserveCoverageReceipt
            | Self::PrivacyRedactionReceipt
            | Self::NullifierSeparationReceipt => 23,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Missing,
    Deferred,
    ArchivedNoGo,
    OperatorActionRequired,
    LiveReceiptNeeded,
    LiveReceiptAccepted,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Deferred => "deferred",
            Self::ArchivedNoGo => "archived_no_go",
            Self::OperatorActionRequired => "operator_action_required",
            Self::LiveReceiptNeeded => "live_receipt_needed",
            Self::LiveReceiptAccepted => "live_receipt_accepted",
        }
    }

    pub fn denies_production(self) -> bool {
        !matches!(self, Self::LiveReceiptAccepted)
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::Missing => 10,
            Self::Deferred => 9,
            Self::LiveReceiptNeeded => 8,
            Self::OperatorActionRequired => 7,
            Self::ArchivedNoGo => 6,
            Self::LiveReceiptAccepted => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialBlockerKind {
    MissingNoGoArchiveRoot,
    MissingGoNoGoReplayRoot,
    MissingLiveReceipt,
    DeferredHeavyGate,
    StaleArchive,
    LaneNotDenied,
    OperatorActionMissing,
    BlockingWeightTooLow,
    DeniedLaneCountTooLow,
    DenialItemCountTooLow,
    ProductionApprovalAbsent,
    PrivacyReceiptMissing,
    CustodyReceiptMissing,
    RuntimeReceiptMissing,
    EmptyRoot,
}

impl DenialBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingNoGoArchiveRoot => "missing_no_go_archive_root",
            Self::MissingGoNoGoReplayRoot => "missing_go_no_go_replay_root",
            Self::MissingLiveReceipt => "missing_live_receipt",
            Self::DeferredHeavyGate => "deferred_heavy_gate",
            Self::StaleArchive => "stale_archive",
            Self::LaneNotDenied => "lane_not_denied",
            Self::OperatorActionMissing => "operator_action_missing",
            Self::BlockingWeightTooLow => "blocking_weight_too_low",
            Self::DeniedLaneCountTooLow => "denied_lane_count_too_low",
            Self::DenialItemCountTooLow => "denial_item_count_too_low",
            Self::ProductionApprovalAbsent => "production_approval_absent",
            Self::PrivacyReceiptMissing => "privacy_receipt_missing",
            Self::CustodyReceiptMissing => "custody_receipt_missing",
            Self::RuntimeReceiptMissing => "runtime_receipt_missing",
            Self::EmptyRoot => "empty_root",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::MissingNoGoArchiveRoot
            | Self::MissingGoNoGoReplayRoot
            | Self::ProductionApprovalAbsent => 10,
            Self::MissingLiveReceipt
            | Self::DeferredHeavyGate
            | Self::PrivacyReceiptMissing
            | Self::CustodyReceiptMissing
            | Self::RuntimeReceiptMissing => 9,
            Self::StaleArchive | Self::LaneNotDenied | Self::OperatorActionMissing => 8,
            Self::BlockingWeightTooLow | Self::DeniedLaneCountTooLow => 7,
            Self::DenialItemCountTooLow | Self::EmptyRoot => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorActionKind {
    RunHeavyGate,
    ReplaceArchiveRoot,
    AttachLiveReceipt,
    RequestReviewerSignoff,
    KeepReleaseHeld,
    PublishDenialNotice,
}

impl OperatorActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RunHeavyGate => "run_heavy_gate",
            Self::ReplaceArchiveRoot => "replace_archive_root",
            Self::AttachLiveReceipt => "attach_live_receipt",
            Self::RequestReviewerSignoff => "request_reviewer_signoff",
            Self::KeepReleaseHeld => "keep_release_held",
            Self::PublishDenialNotice => "publish_denial_notice",
        }
    }

    pub fn for_lane(lane: DenialLane) -> Vec<Self> {
        let mut actions = vec![Self::KeepReleaseHeld, Self::PublishDenialNotice];
        if lane.requires_live_runtime_receipt() {
            actions.push(Self::RunHeavyGate);
            actions.push(Self::AttachLiveReceipt);
        }
        if lane.requires_live_custody_receipt() || lane.requires_live_privacy_receipt() {
            actions.push(Self::RequestReviewerSignoff);
            actions.push(Self::AttachLiveReceipt);
        }
        actions.push(Self::ReplaceArchiveRoot);
        actions
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub wave: u64,
    pub source_archive_wave: u64,
    pub source_replay_wave: u64,
    pub command_checklist_wave: u64,
    pub chain_id: String,
    pub protocol_version: String,
    pub denial_height: u64,
    pub max_archive_age_blocks: u64,
    pub min_denied_lanes: u16,
    pub min_denial_items: u16,
    pub min_operator_actions: u16,
    pub min_blocking_weight: u64,
    pub require_live_runtime_receipts: bool,
    pub require_live_custody_receipts: bool,
    pub require_live_privacy_receipts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            wave: DEFAULT_WAVE,
            source_archive_wave: DEFAULT_SOURCE_ARCHIVE_WAVE,
            source_replay_wave: DEFAULT_SOURCE_REPLAY_WAVE,
            command_checklist_wave: DEFAULT_COMMAND_CHECKLIST_WAVE,
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            denial_height: DEFAULT_DENIAL_HEIGHT,
            max_archive_age_blocks: DEFAULT_MAX_ARCHIVE_AGE_BLOCKS,
            min_denied_lanes: DEFAULT_MIN_DENIED_LANES,
            min_denial_items: DEFAULT_MIN_DENIAL_ITEMS,
            min_operator_actions: DEFAULT_MIN_OPERATOR_ACTIONS,
            min_blocking_weight: DEFAULT_MIN_BLOCKING_WEIGHT,
            require_live_runtime_receipts: true,
            require_live_custody_receipts: true,
            require_live_privacy_receipts: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure(
            self.wave >= self.source_archive_wave,
            "wave must cover source archive wave",
        )?;
        ensure(
            self.source_archive_wave >= self.source_replay_wave,
            "source archive wave must cover replay wave",
        )?;
        ensure(
            self.source_replay_wave >= self.command_checklist_wave,
            "source replay wave must cover command checklist wave",
        )?;
        ensure(self.denial_height > 0, "denial height must be positive")?;
        ensure(
            self.max_archive_age_blocks > 0,
            "max archive age must be positive",
        )?;
        ensure(
            self.min_denied_lanes > 0,
            "min denied lanes must be positive",
        )?;
        ensure(
            self.min_denial_items > 0,
            "min denial items must be positive",
        )?;
        ensure(
            self.min_operator_actions > 0,
            "min operator actions must be positive",
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_production_readiness_denial_config",
            "wave": self.wave,
            "source_archive_wave": self.source_archive_wave,
            "source_replay_wave": self.source_replay_wave,
            "command_checklist_wave": self.command_checklist_wave,
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "denial_height": self.denial_height,
            "max_archive_age_blocks": self.max_archive_age_blocks,
            "min_denied_lanes": self.min_denied_lanes,
            "min_denial_items": self.min_denial_items,
            "min_operator_actions": self.min_operator_actions,
            "min_blocking_weight": self.min_blocking_weight,
            "require_live_runtime_receipts": self.require_live_runtime_receipts,
            "require_live_custody_receipts": self.require_live_custody_receipts,
            "require_live_privacy_receipts": self.require_live_privacy_receipts,
            "hash_suite": HASH_SUITE,
            "denial_suite": DENIAL_SUITE,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenialItem {
    pub item_id: String,
    pub lane: DenialLane,
    pub criterion: DenialCriterion,
    pub status: EvidenceStatus,
    pub source_wave: u64,
    pub archive_height: u64,
    pub archive_root: String,
    pub required_live_receipt_root: String,
    pub denial_reason_root: String,
    pub operator_action_root: String,
    pub blocking_weight: u64,
    pub roots_only: bool,
}

impl DenialItem {
    pub fn new(
        lane: DenialLane,
        criterion: DenialCriterion,
        status: EvidenceStatus,
        config: &Config,
        ordinal: u64,
    ) -> Result<Self> {
        let source_wave = match criterion {
            DenialCriterion::Wave89NoGoArchiveRoot => config.source_archive_wave,
            DenialCriterion::Wave88GoNoGoReplayRoot => config.source_replay_wave,
            _ => config.wave,
        };
        let archive_height = config.denial_height.saturating_sub(ordinal);
        let item = Self {
            item_id: stable_id("denial-item", lane.as_str(), criterion.as_str(), ordinal),
            lane,
            criterion,
            status,
            source_wave,
            archive_height,
            archive_root: sample_root("wave89-archive", lane.as_str(), criterion.as_str(), ordinal),
            required_live_receipt_root: sample_root(
                "required-live-receipt",
                lane.as_str(),
                criterion.as_str(),
                ordinal,
            ),
            denial_reason_root: sample_root(
                "denial-reason",
                lane.as_str(),
                criterion.as_str(),
                ordinal,
            ),
            operator_action_root: sample_root(
                "operator-action",
                lane.as_str(),
                criterion.as_str(),
                ordinal,
            ),
            blocking_weight: criterion.blocking_weight(),
            roots_only: true,
        };
        item.validate()?;
        Ok(item)
    }

    pub fn devnet(
        lane: DenialLane,
        criterion: DenialCriterion,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        let status = status_for_criterion(criterion);
        match Self::new(lane, criterion, status, config, ordinal) {
            Ok(item) => item,
            Err(reason) => fallback_denial_item(lane, criterion, config, ordinal, reason),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("item_id", &self.item_id)?;
        ensure_non_empty("archive_root", &self.archive_root)?;
        ensure_non_empty(
            "required_live_receipt_root",
            &self.required_live_receipt_root,
        )?;
        ensure_non_empty("denial_reason_root", &self.denial_reason_root)?;
        ensure_non_empty("operator_action_root", &self.operator_action_root)?;
        ensure(
            self.criterion.required_for_lane(self.lane),
            "criterion must belong to lane",
        )?;
        ensure(self.blocking_weight > 0, "blocking weight must be positive")
    }

    pub fn denies_production(&self) -> bool {
        self.status.denies_production()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_production_readiness_denial_item",
            "item_id": self.item_id,
            "lane": self.lane.as_str(),
            "criterion": self.criterion.as_str(),
            "status": self.status.as_str(),
            "source_wave": self.source_wave,
            "archive_height": self.archive_height,
            "archive_root": self.archive_root,
            "required_live_receipt_root": self.required_live_receipt_root,
            "denial_reason_root": self.denial_reason_root,
            "operator_action_root": self.operator_action_root,
            "blocking_weight": self.blocking_weight,
            "roots_only": self.roots_only,
            "denies_production": self.denies_production(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("denial-item", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorActionHint {
    pub action_id: String,
    pub lane: DenialLane,
    pub action_kind: OperatorActionKind,
    pub required_before_height: u64,
    pub action_root: String,
    pub runbook_root: String,
    pub publishable: bool,
}

impl OperatorActionHint {
    pub fn devnet(
        lane: DenialLane,
        action_kind: OperatorActionKind,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        Self {
            action_id: stable_id(
                "operator-action",
                lane.as_str(),
                action_kind.as_str(),
                ordinal,
            ),
            lane,
            action_kind,
            required_before_height: config.denial_height.saturating_add(ordinal),
            action_root: sample_root(
                "operator-action",
                lane.as_str(),
                action_kind.as_str(),
                ordinal,
            ),
            runbook_root: sample_root("runbook", lane.as_str(), action_kind.as_str(), ordinal),
            publishable: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_operator_action_hint",
            "action_id": self.action_id,
            "lane": self.lane.as_str(),
            "action_kind": self.action_kind.as_str(),
            "required_before_height": self.required_before_height,
            "action_root": self.action_root,
            "runbook_root": self.runbook_root,
            "publishable": self.publishable,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator-action-hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneDenial {
    pub lane: DenialLane,
    pub owner: String,
    pub denied_item_count: u16,
    pub total_item_count: u16,
    pub action_count: u16,
    pub blocking_weight: u64,
    pub max_severity: u8,
    pub lane_archive_root: String,
    pub denial_item_root: String,
    pub action_hint_root: String,
    pub lane_blocker_root: String,
    pub production_ready: bool,
}

impl LaneDenial {
    pub fn build(
        lane: DenialLane,
        items: &[DenialItem],
        actions: &[OperatorActionHint],
        blockers: &[DenialBlockerKind],
        ordinal: u64,
    ) -> Self {
        let denied_item_count = items.iter().filter(|item| item.denies_production()).count() as u16;
        let blocking_weight = items.iter().map(|item| item.blocking_weight).sum::<u64>();
        let max_severity = max_blocker_severity(blockers);
        Self {
            lane,
            owner: lane.owner().to_string(),
            denied_item_count,
            total_item_count: items.len() as u16,
            action_count: actions.len() as u16,
            blocking_weight,
            max_severity,
            lane_archive_root: sample_root("lane-archive", lane.as_str(), "wave89", ordinal),
            denial_item_root: roots_root(
                "wave90-lane-denial-items",
                items.iter().map(DenialItem::state_root),
            ),
            action_hint_root: roots_root(
                "wave90-lane-action-hints",
                actions.iter().map(OperatorActionHint::state_root),
            ),
            lane_blocker_root: blocker_list_root(lane, blockers),
            production_ready: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_lane_denial",
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "denied_item_count": self.denied_item_count,
            "total_item_count": self.total_item_count,
            "action_count": self.action_count,
            "blocking_weight": self.blocking_weight,
            "max_severity": self.max_severity,
            "lane_archive_root": self.lane_archive_root,
            "denial_item_root": self.denial_item_root,
            "action_hint_root": self.action_hint_root,
            "lane_blocker_root": self.lane_blocker_root,
            "production_ready": self.production_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-denial", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenialSummary {
    pub denied_lane_count: u16,
    pub denial_item_count: u16,
    pub operator_action_count: u16,
    pub total_blocking_weight: u64,
    pub max_blocker_severity: u8,
    pub production_ready: bool,
    pub public_claim_allowed: bool,
    pub denial_reason_root: String,
    pub release_hold_root: String,
}

impl DenialSummary {
    pub fn build(
        config: &Config,
        lane_denials: &[LaneDenial],
        items: &[DenialItem],
        actions: &[OperatorActionHint],
        blockers: &BTreeMap<String, Vec<DenialBlockerKind>>,
    ) -> Self {
        let denied_lane_count = lane_denials
            .iter()
            .filter(|lane| !lane.production_ready)
            .count() as u16;
        let denial_item_count = items.len() as u16;
        let operator_action_count = actions.len() as u16;
        let total_blocking_weight = items.iter().map(|item| item.blocking_weight).sum::<u64>();
        let max_blocker_severity = max_blocker_map_severity(blockers);
        let denial_reason_root = roots_root(
            "wave90-production-denial-reason",
            blockers.iter().map(|(subject, reasons)| {
                let reason_string = reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                sample_root(
                    "denial-reason",
                    subject,
                    &reason_string,
                    reasons.len() as u64,
                )
            }),
        );
        let release_hold_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-WAVE90-PRODUCTION-DENIAL-RELEASE-HOLD",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::U64(config.wave),
                HashPart::U64(total_blocking_weight),
                HashPart::U64(denied_lane_count as u64),
                HashPart::Str(&denial_reason_root),
            ],
            32,
        );
        Self {
            denied_lane_count,
            denial_item_count,
            operator_action_count,
            total_blocking_weight,
            max_blocker_severity,
            production_ready: false,
            public_claim_allowed: false,
            denial_reason_root,
            release_hold_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_production_readiness_denial_summary",
            "denied_lane_count": self.denied_lane_count,
            "denial_item_count": self.denial_item_count,
            "operator_action_count": self.operator_action_count,
            "total_blocking_weight": self.total_blocking_weight,
            "max_blocker_severity": self.max_blocker_severity,
            "production_ready": self.production_ready,
            "public_claim_allowed": self.public_claim_allowed,
            "denial_reason_root": self.denial_reason_root,
            "release_hold_root": self.release_hold_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("denial-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub denial_height: u64,
    pub lane_denials: Vec<LaneDenial>,
    pub denial_items: Vec<DenialItem>,
    pub operator_actions: Vec<OperatorActionHint>,
    pub blockers: BTreeMap<String, Vec<DenialBlockerKind>>,
    pub lane_denial_root: String,
    pub denial_item_root: String,
    pub operator_action_root: String,
    pub blocker_root: String,
    pub summary: DenialSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

impl State {
    pub fn new(
        config: Config,
        denial_items: Vec<DenialItem>,
        operator_actions: Vec<OperatorActionHint>,
    ) -> Result<Self> {
        config.validate()?;
        let blockers = evaluate_blockers(&config, &denial_items, &operator_actions);
        let lane_denials = build_lane_denials(&denial_items, &operator_actions, &blockers);
        let lane_denial_root = roots_root(
            "wave90-lane-denial-root",
            lane_denials.iter().map(LaneDenial::state_root),
        );
        let denial_item_root = roots_root(
            "wave90-denial-item-root",
            denial_items.iter().map(DenialItem::state_root),
        );
        let operator_action_root = roots_root(
            "wave90-operator-action-root",
            operator_actions.iter().map(OperatorActionHint::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary = DenialSummary::build(
            &config,
            &lane_denials,
            &denial_items,
            &operator_actions,
            &blockers,
        );
        Ok(Self {
            denial_height: config.denial_height,
            config,
            lane_denials,
            denial_items,
            operator_actions,
            blockers,
            lane_denial_root,
            denial_item_root,
            operator_action_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut denial_items = Vec::new();
        let mut ordinal = 1_u64;
        for lane in DenialLane::all() {
            for criterion in criteria_for_lane(lane) {
                denial_items.push(DenialItem::devnet(lane, criterion, &config, ordinal));
                ordinal = ordinal.saturating_add(1);
            }
        }
        let mut operator_actions = Vec::new();
        let mut action_ordinal = 1_u64;
        for lane in DenialLane::all() {
            for action_kind in OperatorActionKind::for_lane(lane) {
                operator_actions.push(OperatorActionHint::devnet(
                    lane,
                    action_kind,
                    &config,
                    action_ordinal,
                ));
                action_ordinal = action_ordinal.saturating_add(1);
            }
        }
        match Self::new(config, denial_items, operator_actions) {
            Ok(state) => state,
            Err(reason) => fail_closed_fallback(reason),
        }
    }

    pub fn production_ready(&self) -> bool {
        false
    }

    pub fn public_claim_allowed(&self) -> bool {
        false
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave90_production_readiness_denial_manifest_final_transcript_state",
            "config": self.config.public_record(),
            "denial_height": self.denial_height,
            "lane_denial_root": self.lane_denial_root,
            "denial_item_root": self.denial_item_root,
            "operator_action_root": self.operator_action_root,
            "blocker_root": self.blocker_root,
            "summary": self.summary.public_record(),
            "lane_denials": self.lane_denials.iter().map(LaneDenial::public_record).collect::<Vec<_>>(),
            "production_ready": self.production_ready(),
            "public_claim_allowed": self.public_claim_allowed(),
            "heavy_gates_deferred": true,
            "readiness_claim": "denied_until_live_receipts_replace_wave89_archive_roots",
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn criteria_for_lane(lane: DenialLane) -> Vec<DenialCriterion> {
    let all = vec![
        DenialCriterion::Wave89NoGoArchiveRoot,
        DenialCriterion::Wave88GoNoGoReplayRoot,
        DenialCriterion::CargoCheckReceipt,
        DenialCriterion::CargoTestReceipt,
        DenialCriterion::ClippyReceipt,
        DenialCriterion::RuntimeReplayReceipt,
        DenialCriterion::RollbackDrillReceipt,
        DenialCriterion::AdversarialReplayReceipt,
        DenialCriterion::AuditReviewerReceipt,
        DenialCriterion::ThreatModelReceipt,
        DenialCriterion::BridgeCustodyReceipt,
        DenialCriterion::WatchtowerQuorumReceipt,
        DenialCriterion::WalletEscapeReceipt,
        DenialCriterion::PqSignerEpochReceipt,
        DenialCriterion::ReserveCoverageReceipt,
        DenialCriterion::PrivacyRedactionReceipt,
        DenialCriterion::NullifierSeparationReceipt,
        DenialCriterion::ProductionApprovalReceipt,
    ];
    all.into_iter()
        .filter(|criterion| criterion.required_for_lane(lane))
        .collect()
}

fn status_for_criterion(criterion: DenialCriterion) -> EvidenceStatus {
    match criterion {
        DenialCriterion::Wave89NoGoArchiveRoot | DenialCriterion::Wave88GoNoGoReplayRoot => {
            EvidenceStatus::ArchivedNoGo
        }
        DenialCriterion::ProductionApprovalReceipt => EvidenceStatus::Missing,
        DenialCriterion::CargoCheckReceipt
        | DenialCriterion::CargoTestReceipt
        | DenialCriterion::ClippyReceipt
        | DenialCriterion::RuntimeReplayReceipt
        | DenialCriterion::RollbackDrillReceipt
        | DenialCriterion::AdversarialReplayReceipt
        | DenialCriterion::AuditReviewerReceipt
        | DenialCriterion::ThreatModelReceipt
        | DenialCriterion::BridgeCustodyReceipt
        | DenialCriterion::WatchtowerQuorumReceipt
        | DenialCriterion::WalletEscapeReceipt
        | DenialCriterion::PqSignerEpochReceipt
        | DenialCriterion::ReserveCoverageReceipt
        | DenialCriterion::PrivacyRedactionReceipt
        | DenialCriterion::NullifierSeparationReceipt => EvidenceStatus::LiveReceiptNeeded,
    }
}

fn evaluate_blockers(
    config: &Config,
    items: &[DenialItem],
    actions: &[OperatorActionHint],
) -> BTreeMap<String, Vec<DenialBlockerKind>> {
    let mut blockers: BTreeMap<String, Vec<DenialBlockerKind>> = BTreeMap::new();
    let denied_lanes = items
        .iter()
        .filter(|item| item.denies_production())
        .map(|item| item.lane)
        .collect::<BTreeSet<_>>();
    if denied_lanes.len() < config.min_denied_lanes as usize {
        blockers
            .entry("denied_lanes".to_string())
            .or_default()
            .push(DenialBlockerKind::DeniedLaneCountTooLow);
    }
    if items.len() < config.min_denial_items as usize {
        blockers
            .entry("denial_items".to_string())
            .or_default()
            .push(DenialBlockerKind::DenialItemCountTooLow);
    }
    if actions.len() < config.min_operator_actions as usize {
        blockers
            .entry("operator_actions".to_string())
            .or_default()
            .push(DenialBlockerKind::OperatorActionMissing);
    }
    let total_weight = items.iter().map(|item| item.blocking_weight).sum::<u64>();
    if total_weight < config.min_blocking_weight {
        blockers
            .entry("blocking_weight".to_string())
            .or_default()
            .push(DenialBlockerKind::BlockingWeightTooLow);
    }
    for lane in DenialLane::all() {
        let lane_items = items
            .iter()
            .filter(|item| item.lane == lane)
            .collect::<Vec<_>>();
        if !lane_items.iter().any(|item| item.denies_production()) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::LaneNotDenied);
        }
        if !lane_items
            .iter()
            .any(|item| item.criterion == DenialCriterion::Wave89NoGoArchiveRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::MissingNoGoArchiveRoot);
        }
        if !lane_items
            .iter()
            .any(|item| item.criterion == DenialCriterion::Wave88GoNoGoReplayRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::MissingGoNoGoReplayRoot);
        }
        if !lane_items
            .iter()
            .any(|item| item.criterion == DenialCriterion::ProductionApprovalReceipt)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::ProductionApprovalAbsent);
        }
        if config.require_live_runtime_receipts
            && lane.requires_live_runtime_receipt()
            && !lane_items
                .iter()
                .any(|item| item.status == EvidenceStatus::LiveReceiptAccepted)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::RuntimeReceiptMissing);
        }
        if config.require_live_custody_receipts
            && lane.requires_live_custody_receipt()
            && !lane_items
                .iter()
                .any(|item| item.status == EvidenceStatus::LiveReceiptAccepted)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::CustodyReceiptMissing);
        }
        if config.require_live_privacy_receipts
            && lane.requires_live_privacy_receipt()
            && !lane_items
                .iter()
                .any(|item| item.status == EvidenceStatus::LiveReceiptAccepted)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::PrivacyReceiptMissing);
        }
    }
    for item in items {
        if item.archive_root.trim().is_empty()
            || item.required_live_receipt_root.trim().is_empty()
            || item.denial_reason_root.trim().is_empty()
            || item.operator_action_root.trim().is_empty()
        {
            blockers
                .entry(item.item_id.clone())
                .or_default()
                .push(DenialBlockerKind::EmptyRoot);
        }
        if config.denial_height.saturating_sub(item.archive_height) > config.max_archive_age_blocks
        {
            blockers
                .entry(item.item_id.clone())
                .or_default()
                .push(DenialBlockerKind::StaleArchive);
        }
        if item.status == EvidenceStatus::Deferred {
            blockers
                .entry(item.lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::DeferredHeavyGate);
        }
        if item.status == EvidenceStatus::LiveReceiptNeeded
            || item.status == EvidenceStatus::Missing
            || item.status == EvidenceStatus::OperatorActionRequired
        {
            blockers
                .entry(item.lane.as_str().to_string())
                .or_default()
                .push(DenialBlockerKind::MissingLiveReceipt);
        }
    }
    for action in actions {
        if action.action_root.trim().is_empty() || action.runbook_root.trim().is_empty() {
            blockers
                .entry(action.action_id.clone())
                .or_default()
                .push(DenialBlockerKind::EmptyRoot);
        }
    }
    blockers
}

fn build_lane_denials(
    denial_items: &[DenialItem],
    operator_actions: &[OperatorActionHint],
    blockers: &BTreeMap<String, Vec<DenialBlockerKind>>,
) -> Vec<LaneDenial> {
    DenialLane::all()
        .into_iter()
        .enumerate()
        .map(|(index, lane)| {
            let items = denial_items
                .iter()
                .filter(|item| item.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let actions = operator_actions
                .iter()
                .filter(|action| action.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let lane_blockers = match blockers.get(lane.as_str()) {
                Some(found) => found.clone(),
                None => Vec::new(),
            };
            LaneDenial::build(lane, &items, &actions, &lane_blockers, one_based(index))
        })
        .collect()
}

fn fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let item = fallback_denial_item(
        DenialLane::CompileBlocker,
        DenialCriterion::CargoCheckReceipt,
        &config,
        1,
        reason,
    );
    let action = OperatorActionHint::devnet(
        DenialLane::CompileBlocker,
        OperatorActionKind::KeepReleaseHeld,
        &config,
        1,
    );
    let mut blockers = evaluate_blockers(&config, &[item.clone()], &[action.clone()]);
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(DenialBlockerKind::DeferredHeavyGate);
    let lane_denials = build_lane_denials(&[item.clone()], &[action.clone()], &blockers);
    let lane_denial_root = roots_root(
        "wave90-fallback-lane-denial-root",
        lane_denials.iter().map(LaneDenial::state_root),
    );
    let denial_item_root = roots_root(
        "wave90-fallback-denial-items",
        [item.state_root()].into_iter(),
    );
    let operator_action_root =
        roots_root("wave90-fallback-actions", [action.state_root()].into_iter());
    let blocker_root = blockers_root(&blockers);
    let summary = DenialSummary::build(
        &config,
        &lane_denials,
        &[item.clone()],
        &[action.clone()],
        &blockers,
    );
    State {
        denial_height: config.denial_height,
        config,
        lane_denials,
        denial_items: vec![item],
        operator_actions: vec![action],
        blockers,
        lane_denial_root,
        denial_item_root,
        operator_action_root,
        blocker_root,
        summary,
    }
}

fn fallback_denial_item(
    lane: DenialLane,
    criterion: DenialCriterion,
    config: &Config,
    ordinal: u64,
    reason: String,
) -> DenialItem {
    DenialItem {
        item_id: stable_id(
            "fallback-denial-item",
            lane.as_str(),
            criterion.as_str(),
            ordinal,
        ),
        lane,
        criterion,
        status: EvidenceStatus::Deferred,
        source_wave: config.wave,
        archive_height: config.denial_height,
        archive_root: sample_root("fallback-archive", lane.as_str(), &reason, ordinal),
        required_live_receipt_root: sample_root(
            "fallback-live-receipt",
            lane.as_str(),
            &reason,
            ordinal,
        ),
        denial_reason_root: sample_root("fallback-denial", lane.as_str(), &reason, ordinal),
        operator_action_root: sample_root("fallback-action", lane.as_str(), &reason, ordinal),
        blocking_weight: criterion.blocking_weight(),
        roots_only: true,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<DenialBlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            json!({
                "subject": subject,
                "blockers": blocker_list.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                "max_severity": max_blocker_severity(blocker_list),
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave90-production-denial-blockers", &leaves)
}

fn blocker_list_root(lane: DenialLane, blockers: &[DenialBlockerKind]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| {
            json!({
                "lane": lane.as_str(),
                "blocker": blocker.as_str(),
                "severity": blocker.severity(),
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave90-production-denial-lane-blockers", &leaves)
}

fn max_blocker_severity(blockers: &[DenialBlockerKind]) -> u8 {
    let mut max_value = 0_u8;
    for blocker in blockers {
        let severity = blocker.severity();
        if severity > max_value {
            max_value = severity;
        }
    }
    max_value
}

fn max_blocker_map_severity(blockers: &BTreeMap<String, Vec<DenialBlockerKind>>) -> u8 {
    let mut max_value = 0_u8;
    for list in blockers.values() {
        let severity = max_blocker_severity(list);
        if severity > max_value {
            max_value = severity;
        }
    }
    max_value
}

fn roots_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(kind: &str, record: &PublicRecord) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE90-PRODUCTION-DENIAL-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn stable_id(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE90-PRODUCTION-DENIAL-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn sample_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-WAVE90-PRODUCTION-DENIAL-SAMPLE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
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
