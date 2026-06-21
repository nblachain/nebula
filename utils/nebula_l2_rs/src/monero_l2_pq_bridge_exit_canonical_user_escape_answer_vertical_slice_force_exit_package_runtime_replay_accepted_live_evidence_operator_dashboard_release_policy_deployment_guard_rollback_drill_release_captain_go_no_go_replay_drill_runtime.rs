use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeReplayAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-replay-release-captain-go-no-go-replay-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_DRILL_SUITE: &str = "wave88-release-captain-go-no-go-replay-drill-v1";
pub const DEFAULT_WAVE: u64 = 88;
pub const DEFAULT_WAVE87_ROOT_HEIGHT: u64 = 870_000;
pub const DEFAULT_TARGET_HEIGHT: u64 = 880_128;
pub const DEFAULT_WINDOW_WIDTH_BLOCKS: u64 = 96;
pub const DEFAULT_MAX_ROOT_AGE_BLOCKS: u64 = 10_240;
pub const DEFAULT_MIN_REPLAY_WINDOWS: u16 = 4;
pub const DEFAULT_MIN_COMMAND_RECEIPTS: u16 = 6;
pub const DEFAULT_MIN_CAPTAIN_SIGNOFFS: u16 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    ChecklistRootBinding,
    TargetHeightWindow,
    RuntimeCommandReceipt,
    DeferredRuntimeGate,
    CaptainSignoff,
    FailClosedVerdict,
}

impl ReplayLane {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::ChecklistRootBinding,
            Self::TargetHeightWindow,
            Self::RuntimeCommandReceipt,
            Self::DeferredRuntimeGate,
            Self::CaptainSignoff,
            Self::FailClosedVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChecklistRootBinding => "checklist_root_binding",
            Self::TargetHeightWindow => "target_height_window",
            Self::RuntimeCommandReceipt => "runtime_command_receipt",
            Self::DeferredRuntimeGate => "deferred_runtime_gate",
            Self::CaptainSignoff => "captain_signoff",
            Self::FailClosedVerdict => "fail_closed_verdict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayWindowStatus {
    Bound,
    RootMissing,
    HeightOutOfRange,
    Mismatch,
    DeferredGateOpen,
}

impl ReplayWindowStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::RootMissing => "root_missing",
            Self::HeightOutOfRange => "height_out_of_range",
            Self::Mismatch => "mismatch",
            Self::DeferredGateOpen => "deferred_gate_open",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Bound)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Accepted,
    Missing,
    Mismatched,
    ReplayedLate,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Missing => "missing",
            Self::Mismatched => "mismatched",
            Self::ReplayedLate => "replayed_late",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptainDecision {
    Go,
    Hold,
    MoreEvidence,
    NoGo,
}

impl CaptainDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::Hold => "hold",
            Self::MoreEvidence => "more_evidence",
            Self::NoGo => "no_go",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Go)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalVerdict {
    Go,
    NoGoReplayMismatch,
    NoGoDeferredRuntimeGate,
    NoGoMissingReceipt,
    NoGoCaptainHold,
    NoGoFailClosed,
}

impl FinalVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGoReplayMismatch => "no_go_replay_mismatch",
            Self::NoGoDeferredRuntimeGate => "no_go_deferred_runtime_gate",
            Self::NoGoMissingReceipt => "no_go_missing_receipt",
            Self::NoGoCaptainHold => "no_go_captain_hold",
            Self::NoGoFailClosed => "no_go_fail_closed",
        }
    }

    pub fn production_go(self) -> bool {
        matches!(self, Self::Go)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave87ChecklistRootMissing,
    DuplicateChecklistLane,
    ReplayWindowMissing,
    ReplayHeightOutOfRange,
    ReplayRootMismatch,
    DeferredRuntimeGateRootMissing,
    DeferredRuntimeGateOpen,
    CommandReceiptMissing,
    CommandReceiptMismatch,
    CommandReceiptLate,
    CaptainSignoffMissing,
    CaptainHeldRelease,
    FailClosedVerdictRequired,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave87ChecklistRootMissing => "wave87_checklist_root_missing",
            Self::DuplicateChecklistLane => "duplicate_checklist_lane",
            Self::ReplayWindowMissing => "replay_window_missing",
            Self::ReplayHeightOutOfRange => "replay_height_out_of_range",
            Self::ReplayRootMismatch => "replay_root_mismatch",
            Self::DeferredRuntimeGateRootMissing => "deferred_runtime_gate_root_missing",
            Self::DeferredRuntimeGateOpen => "deferred_runtime_gate_open",
            Self::CommandReceiptMissing => "command_receipt_missing",
            Self::CommandReceiptMismatch => "command_receipt_mismatch",
            Self::CommandReceiptLate => "command_receipt_late",
            Self::CaptainSignoffMissing => "captain_signoff_missing",
            Self::CaptainHeldRelease => "captain_held_release",
            Self::FailClosedVerdictRequired => "fail_closed_verdict_required",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub replay_drill_suite: String,
    pub wave: u64,
    pub wave87_root_height: u64,
    pub target_height: u64,
    pub window_width_blocks: u64,
    pub max_root_age_blocks: u64,
    pub min_replay_windows: u16,
    pub min_command_receipts: u16,
    pub min_captain_signoffs: u16,
    pub require_wave87_roots: bool,
    pub require_replay_window_binding: bool,
    pub require_deferred_runtime_gate_roots: bool,
    pub require_command_receipts: bool,
    pub require_release_captain_signoff: bool,
    pub fail_closed_on_any_blocker: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_drill_suite: REPLAY_DRILL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            wave87_root_height: DEFAULT_WAVE87_ROOT_HEIGHT,
            target_height: DEFAULT_TARGET_HEIGHT,
            window_width_blocks: DEFAULT_WINDOW_WIDTH_BLOCKS,
            max_root_age_blocks: DEFAULT_MAX_ROOT_AGE_BLOCKS,
            min_replay_windows: DEFAULT_MIN_REPLAY_WINDOWS,
            min_command_receipts: DEFAULT_MIN_COMMAND_RECEIPTS,
            min_captain_signoffs: DEFAULT_MIN_CAPTAIN_SIGNOFFS,
            require_wave87_roots: true,
            require_replay_window_binding: true,
            require_deferred_runtime_gate_roots: true,
            require_command_receipts: true,
            require_release_captain_signoff: true,
            fail_closed_on_any_blocker: true,
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
        ensure_non_empty("replay_drill_suite", &self.replay_drill_suite)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.wave > 87, "wave must follow wave 87")?;
        ensure(
            self.wave87_root_height > 0,
            "wave 87 root height must be non-zero",
        )?;
        ensure(self.target_height > 0, "target height must be non-zero")?;
        ensure(
            self.target_height > self.wave87_root_height,
            "target height must follow wave 87 root height",
        )?;
        ensure(
            self.window_width_blocks > 0,
            "replay window width must be non-zero",
        )?;
        ensure(
            self.max_root_age_blocks >= self.window_width_blocks,
            "root age bound must cover replay window",
        )?;
        ensure(
            self.min_replay_windows > 0,
            "minimum replay windows must be non-zero",
        )?;
        ensure(
            self.min_command_receipts > 0,
            "minimum command receipts must be non-zero",
        )?;
        ensure(
            self.min_captain_signoffs > 0,
            "minimum captain signoffs must be non-zero",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave87ChecklistRoot {
    pub checklist_id: String,
    pub lane: ReplayLane,
    pub checklist_root: String,
    pub accepted_root: String,
    pub root_height: u64,
    pub binding_root: String,
}

impl Wave87ChecklistRoot {
    pub fn devnet(lane: ReplayLane, config: &Config, ordinal: u64) -> Self {
        let checklist_root = deterministic_root("wave87-checklist", lane.as_str(), ordinal);
        let accepted_root = deterministic_root("wave87-accepted", lane.as_str(), ordinal);
        let binding_root = checklist_binding_root(config, lane, &checklist_root, &accepted_root);
        Self {
            checklist_id: stable_id("wave87-checklist", lane.as_str(), ordinal),
            lane,
            checklist_root,
            accepted_root,
            root_height: config.wave87_root_height,
            binding_root,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("checklist_id", &self.checklist_id)?;
        ensure_root("checklist_root", &self.checklist_root)?;
        ensure_root("accepted_root", &self.accepted_root)?;
        ensure_root("binding_root", &self.binding_root)?;
        ensure(
            self.root_height <= config.target_height,
            "wave 87 root height must not exceed target height",
        )?;
        ensure(
            config.target_height.saturating_sub(self.root_height) <= config.max_root_age_blocks,
            "wave 87 checklist root is too old for replay drill",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "checklist_id": self.checklist_id,
            "lane": self.lane.as_str(),
            "checklist_root": self.checklist_root,
            "accepted_root": self.accepted_root,
            "root_height": self.root_height,
            "binding_root": self.binding_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave87_checklist_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayWindow {
    pub window_id: String,
    pub lane: ReplayLane,
    pub start_height: u64,
    pub target_height: u64,
    pub end_height: u64,
    pub wave87_binding_root: String,
    pub replay_input_root: String,
    pub replay_output_root: String,
    pub verified_output_root: String,
    pub deferred_gate_root: String,
    pub status: ReplayWindowStatus,
}

impl ReplayWindow {
    pub fn devnet(lane: ReplayLane, config: &Config, ordinal: u64) -> Self {
        let start_height = config
            .target_height
            .saturating_sub(config.window_width_blocks / 2);
        let end_height = config.target_height + config.window_width_blocks / 2;
        let replay_input_root = deterministic_root("replay-input", lane.as_str(), ordinal);
        let replay_output_root = deterministic_root("replay-output", lane.as_str(), ordinal);
        let verified_output_root = replay_output_root.clone();
        let wave87_binding_root =
            checklist_binding_root(config, lane, &replay_input_root, &verified_output_root);
        let deferred_gate_root =
            deterministic_root("deferred-runtime-gate", lane.as_str(), ordinal);
        Self {
            window_id: stable_id("replay-window", lane.as_str(), ordinal),
            lane,
            start_height,
            target_height: config.target_height,
            end_height,
            wave87_binding_root,
            replay_input_root,
            replay_output_root,
            verified_output_root,
            deferred_gate_root,
            status: ReplayWindowStatus::Bound,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("window_id", &self.window_id)?;
        ensure_root("wave87_binding_root", &self.wave87_binding_root)?;
        ensure_root("replay_input_root", &self.replay_input_root)?;
        ensure_root("replay_output_root", &self.replay_output_root)?;
        ensure_root("verified_output_root", &self.verified_output_root)?;
        ensure_root("deferred_gate_root", &self.deferred_gate_root)?;
        ensure(
            self.start_height <= self.target_height && self.target_height <= self.end_height,
            "target height must be inside replay window",
        )?;
        ensure(
            self.target_height == config.target_height,
            "replay window target height must match config",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "window_id": self.window_id,
            "lane": self.lane.as_str(),
            "start_height": self.start_height,
            "target_height": self.target_height,
            "end_height": self.end_height,
            "wave87_binding_root": self.wave87_binding_root,
            "replay_input_root": self.replay_input_root,
            "replay_output_root": self.replay_output_root,
            "verified_output_root": self.verified_output_root,
            "deferred_gate_root": self.deferred_gate_root,
            "status": self.status.as_str(),
            "blocks_release": self.status.blocks_release(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay_window", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCommandReceipt {
    pub receipt_id: String,
    pub command_label: String,
    pub lane: ReplayLane,
    pub command_root: String,
    pub receipt_root: String,
    pub observed_receipt_root: String,
    pub replay_window_root: String,
    pub executed_at_height: u64,
    pub status: ReceiptStatus,
}

impl ReplayCommandReceipt {
    pub fn devnet(lane: ReplayLane, config: &Config, ordinal: u64) -> Self {
        let command_label = format!("{}_release_captain_replay_drill", lane.as_str());
        let command_root = deterministic_root("replay-command", &command_label, ordinal);
        let receipt_root = deterministic_root("replay-command-receipt", &command_label, ordinal);
        Self {
            receipt_id: stable_id("replay-command-receipt", &command_label, ordinal),
            command_label,
            lane,
            command_root,
            receipt_root: receipt_root.clone(),
            observed_receipt_root: receipt_root,
            replay_window_root: deterministic_root("replay-window-root", lane.as_str(), ordinal),
            executed_at_height: config.target_height,
            status: ReceiptStatus::Accepted,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("receipt_id", &self.receipt_id)?;
        ensure_non_empty("command_label", &self.command_label)?;
        ensure_root("command_root", &self.command_root)?;
        ensure_root("receipt_root", &self.receipt_root)?;
        ensure_root("observed_receipt_root", &self.observed_receipt_root)?;
        ensure_root("replay_window_root", &self.replay_window_root)?;
        ensure(
            self.executed_at_height
                >= config
                    .target_height
                    .saturating_sub(config.window_width_blocks)
                && self.executed_at_height <= config.target_height + config.window_width_blocks,
            "command receipt height must be inside replay drill bounds",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "command_label": self.command_label,
            "lane": self.lane.as_str(),
            "command_root": self.command_root,
            "receipt_root": self.receipt_root,
            "observed_receipt_root": self.observed_receipt_root,
            "replay_window_root": self.replay_window_root,
            "executed_at_height": self.executed_at_height,
            "status": self.status.as_str(),
            "blocks_release": self.status.blocks_release(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay_command_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCaptainSignoff {
    pub signoff_id: String,
    pub captain_id: String,
    pub signed_root: String,
    pub replay_window_root: String,
    pub command_receipt_root: String,
    pub decided_at_height: u64,
    pub decision: CaptainDecision,
}

impl ReleaseCaptainSignoff {
    pub fn devnet(captain_id: &str, config: &Config, ordinal: u64) -> Self {
        let signed_root = deterministic_root("captain-signed", captain_id, ordinal);
        let replay_window_root = deterministic_root("captain-window", captain_id, ordinal);
        let command_receipt_root =
            deterministic_root("captain-command-receipt", captain_id, ordinal);
        Self {
            signoff_id: stable_id("captain-signoff", captain_id, ordinal),
            captain_id: captain_id.to_string(),
            signed_root,
            replay_window_root,
            command_receipt_root,
            decided_at_height: config.target_height,
            decision: CaptainDecision::Go,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("signoff_id", &self.signoff_id)?;
        ensure_non_empty("captain_id", &self.captain_id)?;
        ensure_root("signed_root", &self.signed_root)?;
        ensure_root("replay_window_root", &self.replay_window_root)?;
        ensure_root("command_receipt_root", &self.command_receipt_root)?;
        ensure(
            self.decided_at_height >= config.target_height,
            "captain signoff must be at or after target height",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "captain_id": self.captain_id,
            "signed_root": self.signed_root,
            "replay_window_root": self.replay_window_root,
            "command_receipt_root": self.command_receipt_root,
            "decided_at_height": self.decided_at_height,
            "decision": self.decision.as_str(),
            "permits_release": self.decision.permits_release(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_captain_signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Blocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub subject: String,
    pub evidence_root: String,
    pub fail_closed: bool,
}

impl Blocker {
    pub fn new(kind: BlockerKind, subject: &str, evidence_root: &str) -> Self {
        Self {
            blocker_id: domain_hash(
                "WAVE88-REPLAY-DRILL-BLOCKER-ID",
                &[
                    HashPart::Str(CHAIN_ID),
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(kind.as_str()),
                    HashPart::Str(subject),
                    HashPart::Str(evidence_root),
                ],
                24,
            ),
            kind,
            subject: subject.to_string(),
            evidence_root: evidence_root.to_string(),
            fail_closed: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "subject": self.subject,
            "evidence_root": self.evidence_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub checklist_roots: u64,
    pub replay_windows: u64,
    pub bound_windows: u64,
    pub command_receipts: u64,
    pub accepted_receipts: u64,
    pub captain_signoffs: u64,
    pub captain_go_signoffs: u64,
    pub blockers: u64,
}

impl Counters {
    pub fn new(
        checklist_roots: &[Wave87ChecklistRoot],
        replay_windows: &[ReplayWindow],
        command_receipts: &[ReplayCommandReceipt],
        captain_signoffs: &[ReleaseCaptainSignoff],
        blockers: &[Blocker],
    ) -> Self {
        Self {
            checklist_roots: checklist_roots.len() as u64,
            replay_windows: replay_windows.len() as u64,
            bound_windows: replay_windows
                .iter()
                .filter(|window| !window.status.blocks_release())
                .count() as u64,
            command_receipts: command_receipts.len() as u64,
            accepted_receipts: command_receipts
                .iter()
                .filter(|receipt| !receipt.status.blocks_release())
                .count() as u64,
            captain_signoffs: captain_signoffs.len() as u64,
            captain_go_signoffs: captain_signoffs
                .iter()
                .filter(|signoff| signoff.decision.permits_release())
                .count() as u64,
            blockers: blockers.len() as u64,
        }
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GoNoGoDecision {
    pub decision_id: String,
    pub verdict: FinalVerdict,
    pub production_go: bool,
    pub production_hold: bool,
    pub fail_closed: bool,
    pub reason_root: String,
    pub blocker_root: String,
    pub counters_root: String,
}

impl GoNoGoDecision {
    pub fn evaluate(config: &Config, counters: &Counters, blockers: &[Blocker]) -> Self {
        let has_blockers = !blockers.is_empty();
        let fail_closed = config.fail_closed_on_any_blocker && has_blockers;
        let verdict = if !has_blockers {
            FinalVerdict::Go
        } else if blockers
            .iter()
            .any(|blocker| matches!(blocker.kind, BlockerKind::ReplayRootMismatch))
        {
            FinalVerdict::NoGoReplayMismatch
        } else if blockers.iter().any(|blocker| {
            matches!(
                blocker.kind,
                BlockerKind::DeferredRuntimeGateRootMissing | BlockerKind::DeferredRuntimeGateOpen
            )
        }) {
            FinalVerdict::NoGoDeferredRuntimeGate
        } else if blockers.iter().any(|blocker| {
            matches!(
                blocker.kind,
                BlockerKind::CommandReceiptMissing
                    | BlockerKind::CommandReceiptMismatch
                    | BlockerKind::CommandReceiptLate
            )
        }) {
            FinalVerdict::NoGoMissingReceipt
        } else if blockers.iter().any(|blocker| {
            matches!(
                blocker.kind,
                BlockerKind::CaptainSignoffMissing | BlockerKind::CaptainHeldRelease
            )
        }) {
            FinalVerdict::NoGoCaptainHold
        } else {
            FinalVerdict::NoGoFailClosed
        };
        let production_go = verdict.production_go() && !fail_closed;
        let blocker_root = roots_merkle("blockers", blockers.iter().map(Blocker::state_root));
        let counters_root = counters.state_root();
        let reason_root =
            decision_reason_root(config, counters, &blocker_root, verdict, fail_closed);
        let decision_id = domain_hash(
            "WAVE88-REPLAY-DRILL-GO-NO-GO-DECISION-ID",
            &[
                HashPart::Str(&reason_root),
                HashPart::Str(&blocker_root),
                HashPart::Str(&counters_root),
                HashPart::Str(verdict.as_str()),
                HashPart::Str(if production_go { "go" } else { "hold" }),
            ],
            32,
        );
        Self {
            decision_id,
            verdict,
            production_go,
            production_hold: !production_go,
            fail_closed,
            reason_root,
            blocker_root,
            counters_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "decision_id": self.decision_id,
            "verdict": self.verdict.as_str(),
            "production_go": self.production_go,
            "production_hold": self.production_hold,
            "fail_closed": self.fail_closed,
            "reason_root": self.reason_root,
            "blocker_root": self.blocker_root,
            "counters_root": self.counters_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("go_no_go_decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub checklist_root: String,
    pub replay_window_root: String,
    pub command_receipt_root: String,
    pub captain_signoff_root: String,
    pub blocker_root: String,
    pub counters_root: String,
    pub decision_root: String,
    pub state_root: String,
}

impl Roots {
    pub fn compute(
        config: &Config,
        checklist_roots: &[Wave87ChecklistRoot],
        replay_windows: &[ReplayWindow],
        command_receipts: &[ReplayCommandReceipt],
        captain_signoffs: &[ReleaseCaptainSignoff],
        blockers: &[Blocker],
        counters: &Counters,
        decision: &GoNoGoDecision,
    ) -> Self {
        let config_root = config.state_root();
        let checklist_root = roots_merkle(
            "wave87-checklist-roots",
            checklist_roots.iter().map(Wave87ChecklistRoot::state_root),
        );
        let replay_window_root = roots_merkle(
            "target-height-replay-windows",
            replay_windows.iter().map(ReplayWindow::state_root),
        );
        let command_receipt_root = roots_merkle(
            "replay-command-receipts",
            command_receipts
                .iter()
                .map(ReplayCommandReceipt::state_root),
        );
        let captain_signoff_root = roots_merkle(
            "release-captain-signoffs",
            captain_signoffs
                .iter()
                .map(ReleaseCaptainSignoff::state_root),
        );
        let blocker_root = roots_merkle(
            "replay-drill-blockers",
            blockers.iter().map(Blocker::state_root),
        );
        let counters_root = counters.state_root();
        let decision_root = decision.state_root();
        let state_root = domain_hash(
            "WAVE88-RELEASE-CAPTAIN-GO-NO-GO-REPLAY-DRILL-STATE",
            &[
                HashPart::Str(&config_root),
                HashPart::Str(&checklist_root),
                HashPart::Str(&replay_window_root),
                HashPart::Str(&command_receipt_root),
                HashPart::Str(&captain_signoff_root),
                HashPart::Str(&blocker_root),
                HashPart::Str(&counters_root),
                HashPart::Str(&decision_root),
            ],
            32,
        );
        Self {
            config_root,
            checklist_root,
            replay_window_root,
            command_receipt_root,
            captain_signoff_root,
            blocker_root,
            counters_root,
            decision_root,
            state_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub checklist_roots: Vec<Wave87ChecklistRoot>,
    pub replay_windows: Vec<ReplayWindow>,
    pub command_receipts: Vec<ReplayCommandReceipt>,
    pub captain_signoffs: Vec<ReleaseCaptainSignoff>,
    pub blockers: Vec<Blocker>,
    pub counters: Counters,
    pub decision: GoNoGoDecision,
    pub roots: Roots,
}

impl State {
    pub fn new(
        config: Config,
        checklist_roots: Vec<Wave87ChecklistRoot>,
        replay_windows: Vec<ReplayWindow>,
        command_receipts: Vec<ReplayCommandReceipt>,
        captain_signoffs: Vec<ReleaseCaptainSignoff>,
    ) -> Result<Self> {
        config.validate()?;
        for checklist_root in &checklist_roots {
            checklist_root.validate(&config)?;
        }
        for replay_window in &replay_windows {
            replay_window.validate(&config)?;
        }
        for receipt in &command_receipts {
            receipt.validate(&config)?;
        }
        for signoff in &captain_signoffs {
            signoff.validate(&config)?;
        }
        let blockers = evaluate_blockers(
            &config,
            &checklist_roots,
            &replay_windows,
            &command_receipts,
            &captain_signoffs,
        );
        let counters = Counters::new(
            &checklist_roots,
            &replay_windows,
            &command_receipts,
            &captain_signoffs,
            &blockers,
        );
        let decision = GoNoGoDecision::evaluate(&config, &counters, &blockers);
        let roots = Roots::compute(
            &config,
            &checklist_roots,
            &replay_windows,
            &command_receipts,
            &captain_signoffs,
            &blockers,
            &counters,
            &decision,
        );
        Ok(Self {
            config,
            checklist_roots,
            replay_windows,
            command_receipts,
            captain_signoffs,
            blockers,
            counters,
            decision,
            roots,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let checklist_roots = ReplayLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| Wave87ChecklistRoot::devnet(*lane, &config, one_based(index)))
            .collect::<Vec<_>>();
        let replay_windows = ReplayLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| ReplayWindow::devnet(*lane, &config, one_based(index)))
            .collect::<Vec<_>>();
        let command_receipts = ReplayLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| ReplayCommandReceipt::devnet(*lane, &config, one_based(index)))
            .collect::<Vec<_>>();
        let captain_signoffs = vec![
            ReleaseCaptainSignoff::devnet("release-captain-runtime", &config, 1),
            ReleaseCaptainSignoff::devnet("release-captain-sre", &config, 2),
            ReleaseCaptainSignoff::devnet("release-captain-security", &config, 3),
        ];
        match Self::new(
            config,
            checklist_roots,
            replay_windows,
            command_receipts,
            captain_signoffs,
        ) {
            Ok(state) => state,
            Err(reason) => build_fail_closed_fallback(reason),
        }
    }

    pub fn go(&self) -> bool {
        self.decision.production_go
    }

    pub fn no_go(&self) -> bool {
        self.decision.production_hold
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "decision": self.decision.public_record(),
            "checklist_roots": self.checklist_roots.iter().map(Wave87ChecklistRoot::public_record).collect::<Vec<_>>(),
            "replay_windows": self.replay_windows.iter().map(ReplayWindow::public_record).collect::<Vec<_>>(),
            "command_receipts": self.command_receipts.iter().map(ReplayCommandReceipt::public_record).collect::<Vec<_>>(),
            "captain_signoffs": self.captain_signoffs.iter().map(ReleaseCaptainSignoff::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(Blocker::public_record).collect::<Vec<_>>(),
            "go": self.go(),
            "no_go": self.no_go(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root.clone()
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
    checklist_roots: &[Wave87ChecklistRoot],
    replay_windows: &[ReplayWindow],
    command_receipts: &[ReplayCommandReceipt],
    captain_signoffs: &[ReleaseCaptainSignoff],
) -> Vec<Blocker> {
    let mut blockers = Vec::new();
    let mut checklist_lanes = BTreeSet::new();
    for root in checklist_roots {
        if !checklist_lanes.insert(root.lane) {
            blockers.push(Blocker::new(
                BlockerKind::DuplicateChecklistLane,
                root.lane.as_str(),
                &root.binding_root,
            ));
        }
        if config.require_wave87_roots
            && (root.checklist_root.trim().is_empty() || root.accepted_root.trim().is_empty())
        {
            blockers.push(Blocker::new(
                BlockerKind::Wave87ChecklistRootMissing,
                root.lane.as_str(),
                &root.binding_root,
            ));
        }
    }
    for lane in ReplayLane::ordered() {
        if !checklist_lanes.contains(lane) {
            blockers.push(Blocker::new(
                BlockerKind::Wave87ChecklistRootMissing,
                lane.as_str(),
                &deterministic_root("missing-checklist", lane.as_str(), 0),
            ));
        }
    }

    if config.require_replay_window_binding
        && replay_windows.len() < usize::from(config.min_replay_windows)
    {
        blockers.push(Blocker::new(
            BlockerKind::ReplayWindowMissing,
            "replay_window_quorum",
            &deterministic_root("missing-window-quorum", "replay-window", 0),
        ));
    }
    for window in replay_windows {
        if window.start_height > config.target_height || window.end_height < config.target_height {
            blockers.push(Blocker::new(
                BlockerKind::ReplayHeightOutOfRange,
                window.lane.as_str(),
                &window.replay_output_root,
            ));
        }
        if window.replay_output_root != window.verified_output_root {
            blockers.push(Blocker::new(
                BlockerKind::ReplayRootMismatch,
                window.lane.as_str(),
                &window.verified_output_root,
            ));
        }
        if window.status.blocks_release() {
            let kind = match window.status {
                ReplayWindowStatus::RootMissing => BlockerKind::Wave87ChecklistRootMissing,
                ReplayWindowStatus::HeightOutOfRange => BlockerKind::ReplayHeightOutOfRange,
                ReplayWindowStatus::Mismatch => BlockerKind::ReplayRootMismatch,
                ReplayWindowStatus::DeferredGateOpen => BlockerKind::DeferredRuntimeGateOpen,
                ReplayWindowStatus::Bound => BlockerKind::ReplayWindowMissing,
            };
            blockers.push(Blocker::new(
                kind,
                window.lane.as_str(),
                &window.deferred_gate_root,
            ));
        }
        if config.require_deferred_runtime_gate_roots && window.deferred_gate_root.trim().is_empty()
        {
            blockers.push(Blocker::new(
                BlockerKind::DeferredRuntimeGateRootMissing,
                window.lane.as_str(),
                &window.replay_output_root,
            ));
        }
    }

    if config.require_command_receipts
        && command_receipts.len() < usize::from(config.min_command_receipts)
    {
        blockers.push(Blocker::new(
            BlockerKind::CommandReceiptMissing,
            "command_receipt_quorum",
            &deterministic_root("missing-command-receipt-quorum", "receipt", 0),
        ));
    }
    for receipt in command_receipts {
        if receipt.status.blocks_release() {
            let kind = match receipt.status {
                ReceiptStatus::Missing => BlockerKind::CommandReceiptMissing,
                ReceiptStatus::Mismatched => BlockerKind::CommandReceiptMismatch,
                ReceiptStatus::ReplayedLate => BlockerKind::CommandReceiptLate,
                ReceiptStatus::Accepted => BlockerKind::CommandReceiptMissing,
            };
            blockers.push(Blocker::new(
                kind,
                &receipt.command_label,
                &receipt.receipt_root,
            ));
        }
        if receipt.receipt_root != receipt.observed_receipt_root {
            blockers.push(Blocker::new(
                BlockerKind::CommandReceiptMismatch,
                &receipt.command_label,
                &receipt.observed_receipt_root,
            ));
        }
    }

    if config.require_release_captain_signoff
        && captain_signoffs.len() < usize::from(config.min_captain_signoffs)
    {
        blockers.push(Blocker::new(
            BlockerKind::CaptainSignoffMissing,
            "release_captain_quorum",
            &deterministic_root("missing-captain-quorum", "signoff", 0),
        ));
    }
    for signoff in captain_signoffs {
        if !signoff.decision.permits_release() {
            blockers.push(Blocker::new(
                BlockerKind::CaptainHeldRelease,
                &signoff.captain_id,
                &signoff.signed_root,
            ));
        }
    }
    if config.fail_closed_on_any_blocker && !blockers.is_empty() {
        blockers.push(Blocker::new(
            BlockerKind::FailClosedVerdictRequired,
            "go_no_go_verdict",
            &deterministic_root("fail-closed-verdict", "required", blockers.len() as u64),
        ));
    }
    blockers
}

fn build_fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let checklist_roots = vec![Wave87ChecklistRoot::devnet(
        ReplayLane::ChecklistRootBinding,
        &config,
        1,
    )];
    let replay_windows = vec![ReplayWindow {
        status: ReplayWindowStatus::DeferredGateOpen,
        ..ReplayWindow::devnet(ReplayLane::FailClosedVerdict, &config, 1)
    }];
    let command_receipts = vec![ReplayCommandReceipt {
        status: ReceiptStatus::Missing,
        ..ReplayCommandReceipt::devnet(ReplayLane::FailClosedVerdict, &config, 1)
    }];
    let captain_signoffs = vec![ReleaseCaptainSignoff {
        decision: CaptainDecision::Hold,
        ..ReleaseCaptainSignoff::devnet("release-captain-fail-closed", &config, 1)
    }];
    let mut blockers = evaluate_blockers(
        &config,
        &checklist_roots,
        &replay_windows,
        &command_receipts,
        &captain_signoffs,
    );
    let reason_root = deterministic_root("fallback-reason", &reason, 1);
    blockers.push(Blocker::new(
        BlockerKind::FailClosedVerdictRequired,
        "fallback",
        &reason_root,
    ));
    let counters = Counters::new(
        &checklist_roots,
        &replay_windows,
        &command_receipts,
        &captain_signoffs,
        &blockers,
    );
    let decision = GoNoGoDecision::evaluate(&config, &counters, &blockers);
    let roots = Roots::compute(
        &config,
        &checklist_roots,
        &replay_windows,
        &command_receipts,
        &captain_signoffs,
        &blockers,
        &counters,
        &decision,
    );
    State {
        config,
        checklist_roots,
        replay_windows,
        command_receipts,
        captain_signoffs,
        blockers,
        counters,
        decision,
        roots,
    }
}

fn checklist_binding_root(
    config: &Config,
    lane: ReplayLane,
    checklist_root: &str,
    accepted_root: &str,
) -> String {
    domain_hash(
        "WAVE88-WAVE87-CHECKLIST-BINDING-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::U64(config.wave),
            HashPart::U64(config.target_height),
            HashPart::Str(lane.as_str()),
            HashPart::Str(checklist_root),
            HashPart::Str(accepted_root),
        ],
        32,
    )
}

fn decision_reason_root(
    config: &Config,
    counters: &Counters,
    blocker_root: &str,
    verdict: FinalVerdict,
    fail_closed: bool,
) -> String {
    domain_hash(
        "WAVE88-REPLAY-DRILL-DECISION-REASON-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::U64(config.target_height),
            HashPart::Json(&counters.public_record()),
            HashPart::Str(blocker_root),
            HashPart::Str(verdict.as_str()),
            HashPart::Str(if fail_closed { "fail_closed" } else { "open" }),
        ],
        32,
    )
}

fn roots_merkle<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "WAVE88-RELEASE-CAPTAIN-GO-NO-GO-REPLAY-DRILL-RECORD-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn deterministic_root(label: &str, seed: &str, ordinal: u64) -> String {
    domain_hash(
        "WAVE88-RELEASE-CAPTAIN-GO-NO-GO-REPLAY-DRILL-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Str(seed),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn stable_id(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "WAVE88-RELEASE-CAPTAIN-GO-NO-GO-REPLAY-DRILL-STABLE-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        24,
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

fn ensure_root(label: &str, value: &str) -> Result<()> {
    ensure_non_empty(label, value)?;
    ensure(value.len() >= 32, &format!("{label} must be root-like"))
}
