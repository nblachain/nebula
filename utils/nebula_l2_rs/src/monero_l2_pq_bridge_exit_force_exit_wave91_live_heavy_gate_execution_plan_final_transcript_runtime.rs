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
pub type MoneroL2PqBridgeExitForceExitWave91LiveHeavyGateExecutionPlanFinalTranscriptRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE91_LIVE_HEAVY_GATE_EXECUTION_PLAN_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave91-live-heavy-gate-execution-plan-final-transcript-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE91_LIVE_HEAVY_GATE_EXECUTION_PLAN_FINAL_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const PLAN_SUITE: &str = "monero-l2-pq-force-exit-live-heavy-gate-execution-plan-v1";
pub const DEFAULT_WAVE: u64 = 91;
pub const DEFAULT_SOURCE_DENIAL_WAVE: u64 = 90;
pub const DEFAULT_SOURCE_ARCHIVE_WAVE: u64 = 89;
pub const DEFAULT_SOURCE_REPLAY_WAVE: u64 = 88;
pub const DEFAULT_PLAN_HEIGHT: u64 = 910_000;
pub const DEFAULT_MIN_PLANNED_LANES: u16 = 6;
pub const DEFAULT_MIN_RECEIPT_TASKS: u16 = 48;
pub const DEFAULT_MIN_OPERATOR_STEPS: u16 = 12;
pub const DEFAULT_MIN_CLEARANCE_WEIGHT: u64 = 120;
pub const DEFAULT_MAX_PLAN_AGE_BLOCKS: u64 = 128;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanLane {
    CompileReceipt,
    RuntimeReplayReceipt,
    AuditSecurityReceipt,
    BridgeCustodyReceipt,
    WalletWatchtowerReceipt,
    PqReservePrivacyReceipt,
}

impl PlanLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileReceipt,
            Self::RuntimeReplayReceipt,
            Self::AuditSecurityReceipt,
            Self::BridgeCustodyReceipt,
            Self::WalletWatchtowerReceipt,
            Self::PqReservePrivacyReceipt,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileReceipt => "compile_receipt",
            Self::RuntimeReplayReceipt => "runtime_replay_receipt",
            Self::AuditSecurityReceipt => "audit_security_receipt",
            Self::BridgeCustodyReceipt => "bridge_custody_receipt",
            Self::WalletWatchtowerReceipt => "wallet_watchtower_receipt",
            Self::PqReservePrivacyReceipt => "pq_reserve_privacy_receipt",
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::CompileReceipt => "heavy-gate-compile-owner",
            Self::RuntimeReplayReceipt => "heavy-gate-runtime-owner",
            Self::AuditSecurityReceipt => "heavy-gate-security-owner",
            Self::BridgeCustodyReceipt => "heavy-gate-custody-owner",
            Self::WalletWatchtowerReceipt => "heavy-gate-wallet-owner",
            Self::PqReservePrivacyReceipt => "heavy-gate-pq-reserve-privacy-owner",
        }
    }

    pub fn needs_bridge_custody(self) -> bool {
        matches!(
            self,
            Self::BridgeCustodyReceipt | Self::PqReservePrivacyReceipt
        )
    }

    pub fn needs_privacy_review(self) -> bool {
        matches!(
            self,
            Self::AuditSecurityReceipt
                | Self::WalletWatchtowerReceipt
                | Self::PqReservePrivacyReceipt
        )
    }

    pub fn needs_runtime_run(self) -> bool {
        matches!(self, Self::CompileReceipt | Self::RuntimeReplayReceipt)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptKind {
    Wave90DenialRoot,
    Wave89ArchiveRoot,
    CargoCheckRun,
    CargoTestRun,
    ClippyRun,
    RustfmtRun,
    RustcRun,
    RuntimeReplayRun,
    RollbackDrillRun,
    AdversarialReplayRun,
    AuditReviewRun,
    ThreatModelReview,
    BridgeCustodyReview,
    WatchtowerQuorumRun,
    WalletEscapeRun,
    PqSignerEpochRun,
    ReserveCoverageRun,
    PrivacyRedactionReview,
    NullifierSeparationReview,
    OperatorSignoff,
    ReleaseCaptainClearance,
}

impl ReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave90DenialRoot => "wave90_denial_root",
            Self::Wave89ArchiveRoot => "wave89_archive_root",
            Self::CargoCheckRun => "cargo_check_run",
            Self::CargoTestRun => "cargo_test_run",
            Self::ClippyRun => "clippy_run",
            Self::RustfmtRun => "rustfmt_run",
            Self::RustcRun => "rustc_run",
            Self::RuntimeReplayRun => "runtime_replay_run",
            Self::RollbackDrillRun => "rollback_drill_run",
            Self::AdversarialReplayRun => "adversarial_replay_run",
            Self::AuditReviewRun => "audit_review_run",
            Self::ThreatModelReview => "threat_model_review",
            Self::BridgeCustodyReview => "bridge_custody_review",
            Self::WatchtowerQuorumRun => "watchtower_quorum_run",
            Self::WalletEscapeRun => "wallet_escape_run",
            Self::PqSignerEpochRun => "pq_signer_epoch_run",
            Self::ReserveCoverageRun => "reserve_coverage_run",
            Self::PrivacyRedactionReview => "privacy_redaction_review",
            Self::NullifierSeparationReview => "nullifier_separation_review",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReleaseCaptainClearance => "release_captain_clearance",
        }
    }

    pub fn required_for_lane(self, lane: PlanLane) -> bool {
        match self {
            Self::Wave90DenialRoot
            | Self::Wave89ArchiveRoot
            | Self::OperatorSignoff
            | Self::ReleaseCaptainClearance => true,
            Self::CargoCheckRun
            | Self::CargoTestRun
            | Self::ClippyRun
            | Self::RustfmtRun
            | Self::RustcRun => matches!(lane, PlanLane::CompileReceipt),
            Self::RuntimeReplayRun | Self::RollbackDrillRun | Self::AdversarialReplayRun => {
                matches!(lane, PlanLane::RuntimeReplayReceipt)
            }
            Self::AuditReviewRun | Self::ThreatModelReview => {
                matches!(lane, PlanLane::AuditSecurityReceipt)
            }
            Self::BridgeCustodyReview => lane.needs_bridge_custody(),
            Self::WatchtowerQuorumRun | Self::WalletEscapeRun => {
                matches!(lane, PlanLane::WalletWatchtowerReceipt)
            }
            Self::PqSignerEpochRun | Self::ReserveCoverageRun => {
                matches!(lane, PlanLane::PqReservePrivacyReceipt)
            }
            Self::PrivacyRedactionReview | Self::NullifierSeparationReview => {
                lane.needs_privacy_review()
            }
        }
    }

    pub fn clearance_weight(self) -> u64 {
        match self {
            Self::Wave90DenialRoot | Self::Wave89ArchiveRoot => 8,
            Self::ReleaseCaptainClearance => 24,
            Self::OperatorSignoff => 10,
            Self::CargoCheckRun
            | Self::CargoTestRun
            | Self::ClippyRun
            | Self::RustfmtRun
            | Self::RustcRun => 18,
            Self::RuntimeReplayRun | Self::RollbackDrillRun | Self::AdversarialReplayRun => 20,
            Self::AuditReviewRun | Self::ThreatModelReview => 22,
            Self::BridgeCustodyReview | Self::WatchtowerQuorumRun | Self::WalletEscapeRun => 21,
            Self::PqSignerEpochRun
            | Self::ReserveCoverageRun
            | Self::PrivacyRedactionReview
            | Self::NullifierSeparationReview => 23,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Planned,
    BlockedByWorkflow,
    WaitingForLiveRun,
    WaitingForReviewer,
    WaitingForOperator,
    LiveAccepted,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::BlockedByWorkflow => "blocked_by_workflow",
            Self::WaitingForLiveRun => "waiting_for_live_run",
            Self::WaitingForReviewer => "waiting_for_reviewer",
            Self::WaitingForOperator => "waiting_for_operator",
            Self::LiveAccepted => "live_accepted",
        }
    }

    pub fn clears_lane(self) -> bool {
        matches!(self, Self::LiveAccepted)
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::BlockedByWorkflow => 10,
            Self::WaitingForLiveRun => 9,
            Self::WaitingForReviewer => 8,
            Self::WaitingForOperator => 7,
            Self::Planned => 6,
            Self::LiveAccepted => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceBlockerKind {
    MissingDenialRoot,
    MissingArchiveRoot,
    LiveRunStillDeferred,
    ReviewerReceiptMissing,
    OperatorSignoffMissing,
    ReleaseCaptainClearanceMissing,
    PlanTooOld,
    ReceiptTaskCountTooLow,
    PlannedLaneCountTooLow,
    OperatorStepCountTooLow,
    ClearanceWeightTooLow,
    PrivacyReviewMissing,
    CustodyReviewMissing,
    RuntimeRunMissing,
    EmptyRoot,
}

impl ClearanceBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingDenialRoot => "missing_denial_root",
            Self::MissingArchiveRoot => "missing_archive_root",
            Self::LiveRunStillDeferred => "live_run_still_deferred",
            Self::ReviewerReceiptMissing => "reviewer_receipt_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReleaseCaptainClearanceMissing => "release_captain_clearance_missing",
            Self::PlanTooOld => "plan_too_old",
            Self::ReceiptTaskCountTooLow => "receipt_task_count_too_low",
            Self::PlannedLaneCountTooLow => "planned_lane_count_too_low",
            Self::OperatorStepCountTooLow => "operator_step_count_too_low",
            Self::ClearanceWeightTooLow => "clearance_weight_too_low",
            Self::PrivacyReviewMissing => "privacy_review_missing",
            Self::CustodyReviewMissing => "custody_review_missing",
            Self::RuntimeRunMissing => "runtime_run_missing",
            Self::EmptyRoot => "empty_root",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::MissingDenialRoot
            | Self::MissingArchiveRoot
            | Self::ReleaseCaptainClearanceMissing => 10,
            Self::LiveRunStillDeferred
            | Self::ReviewerReceiptMissing
            | Self::PrivacyReviewMissing
            | Self::CustodyReviewMissing
            | Self::RuntimeRunMissing => 9,
            Self::OperatorSignoffMissing | Self::PlanTooOld => 8,
            Self::ReceiptTaskCountTooLow
            | Self::PlannedLaneCountTooLow
            | Self::OperatorStepCountTooLow
            | Self::ClearanceWeightTooLow => 7,
            Self::EmptyRoot => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorStepKind {
    ScheduleLiveRun,
    CaptureReceiptRoot,
    AttachReviewerReceipt,
    AttachOperatorSignoff,
    ReplaceDenialRoot,
    KeepProductionHeld,
    PublishClearancePlan,
}

impl OperatorStepKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ScheduleLiveRun => "schedule_live_run",
            Self::CaptureReceiptRoot => "capture_receipt_root",
            Self::AttachReviewerReceipt => "attach_reviewer_receipt",
            Self::AttachOperatorSignoff => "attach_operator_signoff",
            Self::ReplaceDenialRoot => "replace_denial_root",
            Self::KeepProductionHeld => "keep_production_held",
            Self::PublishClearancePlan => "publish_clearance_plan",
        }
    }

    pub fn for_lane(lane: PlanLane) -> Vec<Self> {
        let mut steps = vec![Self::KeepProductionHeld, Self::PublishClearancePlan];
        if lane.needs_runtime_run() {
            steps.push(Self::ScheduleLiveRun);
            steps.push(Self::CaptureReceiptRoot);
        }
        if lane.needs_bridge_custody() || lane.needs_privacy_review() {
            steps.push(Self::AttachReviewerReceipt);
            steps.push(Self::CaptureReceiptRoot);
        }
        steps.push(Self::AttachOperatorSignoff);
        steps.push(Self::ReplaceDenialRoot);
        steps
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub wave: u64,
    pub source_denial_wave: u64,
    pub source_archive_wave: u64,
    pub source_replay_wave: u64,
    pub chain_id: String,
    pub protocol_version: String,
    pub plan_height: u64,
    pub max_plan_age_blocks: u64,
    pub min_planned_lanes: u16,
    pub min_receipt_tasks: u16,
    pub min_operator_steps: u16,
    pub min_clearance_weight: u64,
    pub require_live_receipts: bool,
    pub require_reviewer_receipts: bool,
    pub require_operator_signoff: bool,
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
            source_denial_wave: DEFAULT_SOURCE_DENIAL_WAVE,
            source_archive_wave: DEFAULT_SOURCE_ARCHIVE_WAVE,
            source_replay_wave: DEFAULT_SOURCE_REPLAY_WAVE,
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            plan_height: DEFAULT_PLAN_HEIGHT,
            max_plan_age_blocks: DEFAULT_MAX_PLAN_AGE_BLOCKS,
            min_planned_lanes: DEFAULT_MIN_PLANNED_LANES,
            min_receipt_tasks: DEFAULT_MIN_RECEIPT_TASKS,
            min_operator_steps: DEFAULT_MIN_OPERATOR_STEPS,
            min_clearance_weight: DEFAULT_MIN_CLEARANCE_WEIGHT,
            require_live_receipts: true,
            require_reviewer_receipts: true,
            require_operator_signoff: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure(
            self.wave >= self.source_denial_wave,
            "wave must cover source denial wave",
        )?;
        ensure(
            self.source_denial_wave >= self.source_archive_wave,
            "source denial wave must cover archive wave",
        )?;
        ensure(
            self.source_archive_wave >= self.source_replay_wave,
            "source archive wave must cover replay wave",
        )?;
        ensure(self.plan_height > 0, "plan height must be positive")?;
        ensure(
            self.max_plan_age_blocks > 0,
            "max plan age must be positive",
        )?;
        ensure(
            self.min_planned_lanes > 0,
            "min planned lanes must be positive",
        )?;
        ensure(
            self.min_receipt_tasks > 0,
            "min receipt tasks must be positive",
        )?;
        ensure(
            self.min_operator_steps > 0,
            "min operator steps must be positive",
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_live_heavy_gate_execution_plan_config",
            "wave": self.wave,
            "source_denial_wave": self.source_denial_wave,
            "source_archive_wave": self.source_archive_wave,
            "source_replay_wave": self.source_replay_wave,
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "plan_height": self.plan_height,
            "max_plan_age_blocks": self.max_plan_age_blocks,
            "min_planned_lanes": self.min_planned_lanes,
            "min_receipt_tasks": self.min_receipt_tasks,
            "min_operator_steps": self.min_operator_steps,
            "min_clearance_weight": self.min_clearance_weight,
            "require_live_receipts": self.require_live_receipts,
            "require_reviewer_receipts": self.require_reviewer_receipts,
            "require_operator_signoff": self.require_operator_signoff,
            "hash_suite": HASH_SUITE,
            "plan_suite": PLAN_SUITE,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptTask {
    pub task_id: String,
    pub lane: PlanLane,
    pub receipt_kind: ReceiptKind,
    pub status: ReceiptStatus,
    pub source_wave: u64,
    pub planned_height: u64,
    pub denial_root: String,
    pub archive_root: String,
    pub receipt_root: String,
    pub criteria_root: String,
    pub operator_step_root: String,
    pub clearance_weight: u64,
    pub roots_only: bool,
}

impl ReceiptTask {
    pub fn new(
        lane: PlanLane,
        receipt_kind: ReceiptKind,
        status: ReceiptStatus,
        config: &Config,
        ordinal: u64,
    ) -> Result<Self> {
        let source_wave = match receipt_kind {
            ReceiptKind::Wave90DenialRoot => config.source_denial_wave,
            ReceiptKind::Wave89ArchiveRoot => config.source_archive_wave,
            _ => config.wave,
        };
        let planned_height = config.plan_height.saturating_add(ordinal);
        let task = Self {
            task_id: stable_id(
                "receipt-task",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            lane,
            receipt_kind,
            status,
            source_wave,
            planned_height,
            denial_root: sample_root(
                "wave90-denial",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            archive_root: sample_root(
                "wave89-archive",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            receipt_root: sample_root(
                "live-receipt",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            criteria_root: sample_root(
                "clearance-criteria",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            operator_step_root: sample_root(
                "operator-step",
                lane.as_str(),
                receipt_kind.as_str(),
                ordinal,
            ),
            clearance_weight: receipt_kind.clearance_weight(),
            roots_only: true,
        };
        task.validate()?;
        Ok(task)
    }

    pub fn devnet(
        lane: PlanLane,
        receipt_kind: ReceiptKind,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        let status = status_for_receipt(receipt_kind);
        match Self::new(lane, receipt_kind, status, config, ordinal) {
            Ok(task) => task,
            Err(reason) => fallback_receipt_task(lane, receipt_kind, config, ordinal, reason),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("task_id", &self.task_id)?;
        ensure_non_empty("denial_root", &self.denial_root)?;
        ensure_non_empty("archive_root", &self.archive_root)?;
        ensure_non_empty("receipt_root", &self.receipt_root)?;
        ensure_non_empty("criteria_root", &self.criteria_root)?;
        ensure_non_empty("operator_step_root", &self.operator_step_root)?;
        ensure(
            self.receipt_kind.required_for_lane(self.lane),
            "receipt kind must belong to lane",
        )?;
        ensure(
            self.clearance_weight > 0,
            "clearance weight must be positive",
        )
    }

    pub fn clears_lane(&self) -> bool {
        self.status.clears_lane()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_receipt_task",
            "task_id": self.task_id,
            "lane": self.lane.as_str(),
            "receipt_kind": self.receipt_kind.as_str(),
            "status": self.status.as_str(),
            "source_wave": self.source_wave,
            "planned_height": self.planned_height,
            "denial_root": self.denial_root,
            "archive_root": self.archive_root,
            "receipt_root": self.receipt_root,
            "criteria_root": self.criteria_root,
            "operator_step_root": self.operator_step_root,
            "clearance_weight": self.clearance_weight,
            "roots_only": self.roots_only,
            "clears_lane": self.clears_lane(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("receipt-task", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorStep {
    pub step_id: String,
    pub lane: PlanLane,
    pub step_kind: OperatorStepKind,
    pub planned_height: u64,
    pub command_root: String,
    pub runbook_root: String,
    pub receipt_slot_root: String,
    pub publishable: bool,
}

impl OperatorStep {
    pub fn devnet(
        lane: PlanLane,
        step_kind: OperatorStepKind,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        Self {
            step_id: stable_id("operator-step", lane.as_str(), step_kind.as_str(), ordinal),
            lane,
            step_kind,
            planned_height: config.plan_height.saturating_add(ordinal),
            command_root: sample_root(
                "operator-command",
                lane.as_str(),
                step_kind.as_str(),
                ordinal,
            ),
            runbook_root: sample_root("runbook", lane.as_str(), step_kind.as_str(), ordinal),
            receipt_slot_root: sample_root(
                "receipt-slot",
                lane.as_str(),
                step_kind.as_str(),
                ordinal,
            ),
            publishable: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_operator_step",
            "step_id": self.step_id,
            "lane": self.lane.as_str(),
            "step_kind": self.step_kind.as_str(),
            "planned_height": self.planned_height,
            "command_root": self.command_root,
            "runbook_root": self.runbook_root,
            "receipt_slot_root": self.receipt_slot_root,
            "publishable": self.publishable,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator-step", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanePlan {
    pub lane: PlanLane,
    pub owner: String,
    pub receipt_task_count: u16,
    pub live_accepted_count: u16,
    pub operator_step_count: u16,
    pub clearance_weight: u64,
    pub max_severity: u8,
    pub denial_root: String,
    pub receipt_task_root: String,
    pub operator_step_root: String,
    pub blocker_root: String,
    pub clears_denial: bool,
}

impl LanePlan {
    pub fn build(
        lane: PlanLane,
        tasks: &[ReceiptTask],
        steps: &[OperatorStep],
        blockers: &[ClearanceBlockerKind],
        ordinal: u64,
    ) -> Self {
        let live_accepted_count = tasks.iter().filter(|task| task.clears_lane()).count() as u16;
        let clearance_weight = tasks.iter().map(|task| task.clearance_weight).sum::<u64>();
        Self {
            lane,
            owner: lane.owner().to_string(),
            receipt_task_count: tasks.len() as u16,
            live_accepted_count,
            operator_step_count: steps.len() as u16,
            clearance_weight,
            max_severity: max_blocker_severity(blockers),
            denial_root: sample_root("wave90-lane-denial", lane.as_str(), "lane", ordinal),
            receipt_task_root: roots_root(
                "wave91-lane-receipt-tasks",
                tasks.iter().map(ReceiptTask::state_root),
            ),
            operator_step_root: roots_root(
                "wave91-lane-operator-steps",
                steps.iter().map(OperatorStep::state_root),
            ),
            blocker_root: blocker_list_root(lane, blockers),
            clears_denial: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_lane_execution_plan",
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "receipt_task_count": self.receipt_task_count,
            "live_accepted_count": self.live_accepted_count,
            "operator_step_count": self.operator_step_count,
            "clearance_weight": self.clearance_weight,
            "max_severity": self.max_severity,
            "denial_root": self.denial_root,
            "receipt_task_root": self.receipt_task_root,
            "operator_step_root": self.operator_step_root,
            "blocker_root": self.blocker_root,
            "clears_denial": self.clears_denial,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-plan", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanSummary {
    pub planned_lane_count: u16,
    pub receipt_task_count: u16,
    pub operator_step_count: u16,
    pub total_clearance_weight: u64,
    pub max_blocker_severity: u8,
    pub live_receipts_present: bool,
    pub clears_production_denial: bool,
    pub production_ready: bool,
    pub plan_root: String,
    pub release_hold_root: String,
}

impl PlanSummary {
    pub fn build(
        config: &Config,
        lane_plans: &[LanePlan],
        tasks: &[ReceiptTask],
        steps: &[OperatorStep],
        blockers: &BTreeMap<String, Vec<ClearanceBlockerKind>>,
    ) -> Self {
        let planned_lane_count = lane_plans.len() as u16;
        let receipt_task_count = tasks.len() as u16;
        let operator_step_count = steps.len() as u16;
        let total_clearance_weight = tasks.iter().map(|task| task.clearance_weight).sum::<u64>();
        let max_blocker_severity = max_blocker_map_severity(blockers);
        let live_receipts_present = tasks.iter().all(|task| task.clears_lane());
        let plan_root = roots_root(
            "wave91-clearance-plan-root",
            lane_plans.iter().map(LanePlan::state_root),
        );
        let release_hold_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-GATE-PLAN-RELEASE-HOLD",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::U64(config.wave),
                HashPart::U64(total_clearance_weight),
                HashPart::U64(receipt_task_count as u64),
                HashPart::Str(&plan_root),
            ],
            32,
        );
        Self {
            planned_lane_count,
            receipt_task_count,
            operator_step_count,
            total_clearance_weight,
            max_blocker_severity,
            live_receipts_present,
            clears_production_denial: false,
            production_ready: false,
            plan_root,
            release_hold_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_live_heavy_gate_execution_plan_summary",
            "planned_lane_count": self.planned_lane_count,
            "receipt_task_count": self.receipt_task_count,
            "operator_step_count": self.operator_step_count,
            "total_clearance_weight": self.total_clearance_weight,
            "max_blocker_severity": self.max_blocker_severity,
            "live_receipts_present": self.live_receipts_present,
            "clears_production_denial": self.clears_production_denial,
            "production_ready": self.production_ready,
            "plan_root": self.plan_root,
            "release_hold_root": self.release_hold_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("plan-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub plan_height: u64,
    pub lane_plans: Vec<LanePlan>,
    pub receipt_tasks: Vec<ReceiptTask>,
    pub operator_steps: Vec<OperatorStep>,
    pub blockers: BTreeMap<String, Vec<ClearanceBlockerKind>>,
    pub lane_plan_root: String,
    pub receipt_task_root: String,
    pub operator_step_root: String,
    pub blocker_root: String,
    pub summary: PlanSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

impl State {
    pub fn new(
        config: Config,
        receipt_tasks: Vec<ReceiptTask>,
        operator_steps: Vec<OperatorStep>,
    ) -> Result<Self> {
        config.validate()?;
        let blockers = evaluate_blockers(&config, &receipt_tasks, &operator_steps);
        let lane_plans = build_lane_plans(&receipt_tasks, &operator_steps, &blockers);
        let lane_plan_root = roots_root(
            "wave91-lane-plan-root",
            lane_plans.iter().map(LanePlan::state_root),
        );
        let receipt_task_root = roots_root(
            "wave91-receipt-task-root",
            receipt_tasks.iter().map(ReceiptTask::state_root),
        );
        let operator_step_root = roots_root(
            "wave91-operator-step-root",
            operator_steps.iter().map(OperatorStep::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary = PlanSummary::build(
            &config,
            &lane_plans,
            &receipt_tasks,
            &operator_steps,
            &blockers,
        );
        Ok(Self {
            plan_height: config.plan_height,
            config,
            lane_plans,
            receipt_tasks,
            operator_steps,
            blockers,
            lane_plan_root,
            receipt_task_root,
            operator_step_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let mut receipt_tasks = Vec::new();
        let mut task_ordinal = 1_u64;
        for lane in PlanLane::all() {
            for receipt_kind in receipt_kinds_for_lane(lane) {
                receipt_tasks.push(ReceiptTask::devnet(
                    lane,
                    receipt_kind,
                    &config,
                    task_ordinal,
                ));
                task_ordinal = task_ordinal.saturating_add(1);
            }
        }
        let mut operator_steps = Vec::new();
        let mut step_ordinal = 1_u64;
        for lane in PlanLane::all() {
            for step_kind in OperatorStepKind::for_lane(lane) {
                operator_steps.push(OperatorStep::devnet(lane, step_kind, &config, step_ordinal));
                step_ordinal = step_ordinal.saturating_add(1);
            }
        }
        match Self::new(config, receipt_tasks, operator_steps) {
            Ok(state) => state,
            Err(reason) => fail_closed_fallback(reason),
        }
    }

    pub fn clears_production_denial(&self) -> bool {
        false
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave91_live_heavy_gate_execution_plan_final_transcript_state",
            "config": self.config.public_record(),
            "plan_height": self.plan_height,
            "lane_plan_root": self.lane_plan_root,
            "receipt_task_root": self.receipt_task_root,
            "operator_step_root": self.operator_step_root,
            "blocker_root": self.blocker_root,
            "summary": self.summary.public_record(),
            "lane_plans": self.lane_plans.iter().map(LanePlan::public_record).collect::<Vec<_>>(),
            "clears_production_denial": self.clears_production_denial(),
            "production_ready": false,
            "heavy_gates_deferred": true,
            "plan_only": true,
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

fn receipt_kinds_for_lane(lane: PlanLane) -> Vec<ReceiptKind> {
    let all = vec![
        ReceiptKind::Wave90DenialRoot,
        ReceiptKind::Wave89ArchiveRoot,
        ReceiptKind::CargoCheckRun,
        ReceiptKind::CargoTestRun,
        ReceiptKind::ClippyRun,
        ReceiptKind::RustfmtRun,
        ReceiptKind::RustcRun,
        ReceiptKind::RuntimeReplayRun,
        ReceiptKind::RollbackDrillRun,
        ReceiptKind::AdversarialReplayRun,
        ReceiptKind::AuditReviewRun,
        ReceiptKind::ThreatModelReview,
        ReceiptKind::BridgeCustodyReview,
        ReceiptKind::WatchtowerQuorumRun,
        ReceiptKind::WalletEscapeRun,
        ReceiptKind::PqSignerEpochRun,
        ReceiptKind::ReserveCoverageRun,
        ReceiptKind::PrivacyRedactionReview,
        ReceiptKind::NullifierSeparationReview,
        ReceiptKind::OperatorSignoff,
        ReceiptKind::ReleaseCaptainClearance,
    ];
    all.into_iter()
        .filter(|kind| kind.required_for_lane(lane))
        .collect()
}

fn status_for_receipt(receipt_kind: ReceiptKind) -> ReceiptStatus {
    match receipt_kind {
        ReceiptKind::Wave90DenialRoot | ReceiptKind::Wave89ArchiveRoot => ReceiptStatus::Planned,
        ReceiptKind::OperatorSignoff => ReceiptStatus::WaitingForOperator,
        ReceiptKind::ReleaseCaptainClearance => ReceiptStatus::WaitingForOperator,
        ReceiptKind::AuditReviewRun
        | ReceiptKind::ThreatModelReview
        | ReceiptKind::BridgeCustodyReview
        | ReceiptKind::PrivacyRedactionReview
        | ReceiptKind::NullifierSeparationReview => ReceiptStatus::WaitingForReviewer,
        ReceiptKind::CargoCheckRun
        | ReceiptKind::CargoTestRun
        | ReceiptKind::ClippyRun
        | ReceiptKind::RustfmtRun
        | ReceiptKind::RustcRun
        | ReceiptKind::RuntimeReplayRun
        | ReceiptKind::RollbackDrillRun
        | ReceiptKind::AdversarialReplayRun
        | ReceiptKind::WatchtowerQuorumRun
        | ReceiptKind::WalletEscapeRun
        | ReceiptKind::PqSignerEpochRun
        | ReceiptKind::ReserveCoverageRun => ReceiptStatus::WaitingForLiveRun,
    }
}

fn evaluate_blockers(
    config: &Config,
    tasks: &[ReceiptTask],
    steps: &[OperatorStep],
) -> BTreeMap<String, Vec<ClearanceBlockerKind>> {
    let mut blockers: BTreeMap<String, Vec<ClearanceBlockerKind>> = BTreeMap::new();
    let planned_lanes = tasks.iter().map(|task| task.lane).collect::<BTreeSet<_>>();
    if planned_lanes.len() < config.min_planned_lanes as usize {
        blockers
            .entry("planned_lanes".to_string())
            .or_default()
            .push(ClearanceBlockerKind::PlannedLaneCountTooLow);
    }
    if tasks.len() < config.min_receipt_tasks as usize {
        blockers
            .entry("receipt_tasks".to_string())
            .or_default()
            .push(ClearanceBlockerKind::ReceiptTaskCountTooLow);
    }
    if steps.len() < config.min_operator_steps as usize {
        blockers
            .entry("operator_steps".to_string())
            .or_default()
            .push(ClearanceBlockerKind::OperatorStepCountTooLow);
    }
    let total_weight = tasks.iter().map(|task| task.clearance_weight).sum::<u64>();
    if total_weight < config.min_clearance_weight {
        blockers
            .entry("clearance_weight".to_string())
            .or_default()
            .push(ClearanceBlockerKind::ClearanceWeightTooLow);
    }
    for lane in PlanLane::all() {
        let lane_tasks = tasks
            .iter()
            .filter(|task| task.lane == lane)
            .collect::<Vec<_>>();
        if !lane_tasks
            .iter()
            .any(|task| task.receipt_kind == ReceiptKind::Wave90DenialRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::MissingDenialRoot);
        }
        if !lane_tasks
            .iter()
            .any(|task| task.receipt_kind == ReceiptKind::Wave89ArchiveRoot)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::MissingArchiveRoot);
        }
        if !lane_tasks
            .iter()
            .any(|task| task.receipt_kind == ReceiptKind::ReleaseCaptainClearance)
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::ReleaseCaptainClearanceMissing);
        }
        if config.require_live_receipts
            && lane.needs_runtime_run()
            && !lane_tasks.iter().any(|task| task.clears_lane())
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::RuntimeRunMissing);
        }
        if config.require_reviewer_receipts
            && lane.needs_privacy_review()
            && !lane_tasks.iter().any(|task| task.clears_lane())
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::PrivacyReviewMissing);
        }
        if config.require_reviewer_receipts
            && lane.needs_bridge_custody()
            && !lane_tasks.iter().any(|task| task.clears_lane())
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::CustodyReviewMissing);
        }
        if config.require_operator_signoff
            && !steps.iter().any(|step| {
                step.lane == lane && step.step_kind == OperatorStepKind::AttachOperatorSignoff
            })
        {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::OperatorSignoffMissing);
        }
    }
    for task in tasks {
        if task.denial_root.trim().is_empty()
            || task.archive_root.trim().is_empty()
            || task.receipt_root.trim().is_empty()
            || task.criteria_root.trim().is_empty()
            || task.operator_step_root.trim().is_empty()
        {
            blockers
                .entry(task.task_id.clone())
                .or_default()
                .push(ClearanceBlockerKind::EmptyRoot);
        }
        if task.planned_height.saturating_sub(config.plan_height) > config.max_plan_age_blocks {
            blockers
                .entry(task.task_id.clone())
                .or_default()
                .push(ClearanceBlockerKind::PlanTooOld);
        }
        if !task.clears_lane() {
            blockers
                .entry(task.lane.as_str().to_string())
                .or_default()
                .push(ClearanceBlockerKind::LiveRunStillDeferred);
        }
    }
    for step in steps {
        if step.command_root.trim().is_empty()
            || step.runbook_root.trim().is_empty()
            || step.receipt_slot_root.trim().is_empty()
        {
            blockers
                .entry(step.step_id.clone())
                .or_default()
                .push(ClearanceBlockerKind::EmptyRoot);
        }
    }
    blockers
}

fn build_lane_plans(
    tasks: &[ReceiptTask],
    steps: &[OperatorStep],
    blockers: &BTreeMap<String, Vec<ClearanceBlockerKind>>,
) -> Vec<LanePlan> {
    PlanLane::all()
        .into_iter()
        .enumerate()
        .map(|(index, lane)| {
            let lane_tasks = tasks
                .iter()
                .filter(|task| task.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let lane_steps = steps
                .iter()
                .filter(|step| step.lane == lane)
                .cloned()
                .collect::<Vec<_>>();
            let lane_blockers = match blockers.get(lane.as_str()) {
                Some(found) => found.clone(),
                None => Vec::new(),
            };
            LanePlan::build(
                lane,
                &lane_tasks,
                &lane_steps,
                &lane_blockers,
                one_based(index),
            )
        })
        .collect()
}

fn fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let task = fallback_receipt_task(
        PlanLane::CompileReceipt,
        ReceiptKind::CargoCheckRun,
        &config,
        1,
        reason,
    );
    let step = OperatorStep::devnet(
        PlanLane::CompileReceipt,
        OperatorStepKind::KeepProductionHeld,
        &config,
        1,
    );
    let mut blockers = evaluate_blockers(&config, &[task.clone()], &[step.clone()]);
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(ClearanceBlockerKind::LiveRunStillDeferred);
    let lane_plans = build_lane_plans(&[task.clone()], &[step.clone()], &blockers);
    let lane_plan_root = roots_root(
        "wave91-fallback-lane-plan-root",
        lane_plans.iter().map(LanePlan::state_root),
    );
    let receipt_task_root = roots_root(
        "wave91-fallback-receipt-tasks",
        [task.state_root()].into_iter(),
    );
    let operator_step_root = roots_root(
        "wave91-fallback-operator-steps",
        [step.state_root()].into_iter(),
    );
    let blocker_root = blockers_root(&blockers);
    let summary = PlanSummary::build(
        &config,
        &lane_plans,
        &[task.clone()],
        &[step.clone()],
        &blockers,
    );
    State {
        plan_height: config.plan_height,
        config,
        lane_plans,
        receipt_tasks: vec![task],
        operator_steps: vec![step],
        blockers,
        lane_plan_root,
        receipt_task_root,
        operator_step_root,
        blocker_root,
        summary,
    }
}

fn fallback_receipt_task(
    lane: PlanLane,
    receipt_kind: ReceiptKind,
    config: &Config,
    ordinal: u64,
    reason: String,
) -> ReceiptTask {
    ReceiptTask {
        task_id: stable_id(
            "fallback-receipt-task",
            lane.as_str(),
            receipt_kind.as_str(),
            ordinal,
        ),
        lane,
        receipt_kind,
        status: ReceiptStatus::BlockedByWorkflow,
        source_wave: config.wave,
        planned_height: config.plan_height,
        denial_root: sample_root("fallback-denial", lane.as_str(), &reason, ordinal),
        archive_root: sample_root("fallback-archive", lane.as_str(), &reason, ordinal),
        receipt_root: sample_root("fallback-receipt", lane.as_str(), &reason, ordinal),
        criteria_root: sample_root("fallback-criteria", lane.as_str(), &reason, ordinal),
        operator_step_root: sample_root("fallback-operator-step", lane.as_str(), &reason, ordinal),
        clearance_weight: receipt_kind.clearance_weight(),
        roots_only: true,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<ClearanceBlockerKind>>) -> String {
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
    merkle_root("wave91-live-heavy-gate-plan-blockers", &leaves)
}

fn blocker_list_root(lane: PlanLane, blockers: &[ClearanceBlockerKind]) -> String {
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
    merkle_root("wave91-live-heavy-gate-plan-lane-blockers", &leaves)
}

fn max_blocker_severity(blockers: &[ClearanceBlockerKind]) -> u8 {
    let mut max_value = 0_u8;
    for blocker in blockers {
        let severity = blocker.severity();
        if severity > max_value {
            max_value = severity;
        }
    }
    max_value
}

fn max_blocker_map_severity(blockers: &BTreeMap<String, Vec<ClearanceBlockerKind>>) -> u8 {
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
        "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-PLAN-RECORD",
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
        "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-PLAN-ID",
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
        "MONERO-L2-PQ-BRIDGE-WAVE91-LIVE-HEAVY-GATE-PLAN-SAMPLE-ROOT",
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
