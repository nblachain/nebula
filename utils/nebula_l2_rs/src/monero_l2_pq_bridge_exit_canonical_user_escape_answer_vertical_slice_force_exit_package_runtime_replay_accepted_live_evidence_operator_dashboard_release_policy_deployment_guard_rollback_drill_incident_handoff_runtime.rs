use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeReplayAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-replay-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const INCIDENT_HANDOFF_SUITE: &str = "runtime-replay-incident-handoff-v1";
pub const DEFAULT_INCIDENT_HEIGHT: u64 = 86_086;
pub const DEFAULT_TARGET_HEIGHT: u64 = 86_060;
pub const DEFAULT_MAX_TARGET_LAG_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_REPLAY_TRANSCRIPTS: u16 = 3;
pub const DEFAULT_MIN_DEFERRED_GATE_ROOTS: u16 = 3;
pub const DEFAULT_MIN_COMMAND_ROOM_ACKS: u16 = 4;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPhase {
    Intake,
    ReplayBound,
    TargetHeightPinned,
    DeferredGateBound,
    CommandRoomTransfer,
    FailClosedHandoff,
}

impl HandoffPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intake => "intake",
            Self::ReplayBound => "replay_bound",
            Self::TargetHeightPinned => "target_height_pinned",
            Self::DeferredGateBound => "deferred_gate_bound",
            Self::CommandRoomTransfer => "command_room_transfer",
            Self::FailClosedHandoff => "fail_closed_handoff",
        }
    }

    pub fn ordinal(self) -> u8 {
        match self {
            Self::Intake => 0,
            Self::ReplayBound => 1,
            Self::TargetHeightPinned => 2,
            Self::DeferredGateBound => 3,
            Self::CommandRoomTransfer => 4,
            Self::FailClosedHandoff => 5,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Intake,
            Self::ReplayBound,
            Self::TargetHeightPinned,
            Self::DeferredGateBound,
            Self::CommandRoomTransfer,
            Self::FailClosedHandoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffVerdict {
    Ready,
    Watch,
    Blocked,
    FailClosed,
}

impl HandoffVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Watch => "watch",
            Self::Blocked => "blocked",
            Self::FailClosed => "fail_closed",
        }
    }

    pub fn fail_closed(self) -> bool {
        !matches!(self, Self::Ready)
    }

    pub fn release_allowed(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub incident_height: u64,
    pub target_height: u64,
    pub max_target_lag_blocks: u64,
    pub min_replay_transcripts: u16,
    pub min_deferred_gate_roots: u16,
    pub min_command_room_acks: u16,
    pub require_fail_closed_transfer: bool,
    pub require_privacy_safe_public_record: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            incident_height: DEFAULT_INCIDENT_HEIGHT,
            target_height: DEFAULT_TARGET_HEIGHT,
            max_target_lag_blocks: DEFAULT_MAX_TARGET_LAG_BLOCKS,
            min_replay_transcripts: DEFAULT_MIN_REPLAY_TRANSCRIPTS,
            min_deferred_gate_roots: DEFAULT_MIN_DEFERRED_GATE_ROOTS,
            min_command_room_acks: DEFAULT_MIN_COMMAND_ROOM_ACKS,
            require_fail_closed_transfer: true,
            require_privacy_safe_public_record: true,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure!(
            self.incident_height >= self.target_height,
            "incident height must be at or after target height",
        )?;
        ensure!(
            self.incident_height.saturating_sub(self.target_height) <= self.max_target_lag_blocks,
            "target height is outside replay incident handoff lag window",
        )?;
        ensure!(
            self.min_replay_transcripts > 0,
            "minimum replay transcript count must be non-zero",
        )?;
        ensure!(
            self.min_deferred_gate_roots > 0,
            "minimum deferred gate root count must be non-zero",
        )?;
        ensure!(
            self.min_command_room_acks > 0,
            "minimum command-room acknowledgement count must be non-zero",
        )
    }

    pub fn canonical(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "incident_height": self.incident_height,
            "target_height": self.target_height,
            "max_target_lag_blocks": self.max_target_lag_blocks,
            "min_replay_transcripts": self.min_replay_transcripts,
            "min_deferred_gate_roots": self.min_deferred_gate_roots,
            "min_command_room_acks": self.min_command_room_acks,
            "require_fail_closed_transfer": self.require_fail_closed_transfer,
            "require_privacy_safe_public_record": self.require_privacy_safe_public_record,
        })
    }

    pub fn root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:config",
            &[HashPart::Json(&self.canonical())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayTranscriptBinding {
    pub transcript_id: String,
    pub replay_transcript_root: String,
    pub rollback_command_root: String,
    pub accepted_receipt_root: String,
    pub target_height: u64,
    pub replayed_height: u64,
    pub accepted: bool,
    pub fail_closed_observed: bool,
}

impl ReplayTranscriptBinding {
    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("transcript_id", &self.transcript_id)?;
        ensure_root("replay_transcript_root", &self.replay_transcript_root)?;
        ensure_root("rollback_command_root", &self.rollback_command_root)?;
        ensure_root("accepted_receipt_root", &self.accepted_receipt_root)?;
        ensure!(
            self.target_height == config.target_height,
            "replay transcript target height must match handoff target height",
        )?;
        ensure!(
            self.replayed_height >= self.target_height,
            "replay transcript replayed height must be at or after target height",
        )?;
        ensure!(
            self.replayed_height <= config.incident_height,
            "replay transcript replayed height must not exceed incident height",
        )
    }

    pub fn blocks_handoff(&self) -> bool {
        !self.accepted || !self.fail_closed_observed
    }

    pub fn canonical(&self) -> Value {
        json!({
            "transcript_id": self.transcript_id,
            "replay_transcript_root": self.replay_transcript_root,
            "rollback_command_root": self.rollback_command_root,
            "accepted_receipt_root": self.accepted_receipt_root,
            "target_height": self.target_height,
            "replayed_height": self.replayed_height,
            "accepted": self.accepted,
            "fail_closed_observed": self.fail_closed_observed,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "binding_root": self.root(),
            "target_height": self.target_height,
            "replayed_height": self.replayed_height,
            "accepted": self.accepted,
            "fail_closed_observed": self.fail_closed_observed,
        })
    }

    pub fn root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:replay-transcript",
            &[HashPart::Json(&self.canonical())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TargetHeightPin {
    pub target_height: u64,
    pub finalized_height: u64,
    pub source_checkpoint_root: String,
    pub replay_window_root: String,
    pub target_locked: bool,
}

impl TargetHeightPin {
    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure!(
            self.target_height == config.target_height,
            "target-height pin must match handoff target height",
        )?;
        ensure!(
            self.finalized_height >= self.target_height,
            "target-height pin finalized height must cover target height",
        )?;
        ensure!(
            self.finalized_height <= config.incident_height,
            "target-height pin finalized height must not exceed incident height",
        )?;
        ensure_root("source_checkpoint_root", &self.source_checkpoint_root)?;
        ensure_root("replay_window_root", &self.replay_window_root)
    }

    pub fn canonical(&self) -> Value {
        json!({
            "target_height": self.target_height,
            "finalized_height": self.finalized_height,
            "source_checkpoint_root": self.source_checkpoint_root,
            "replay_window_root": self.replay_window_root,
            "target_locked": self.target_locked,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "target_height": self.target_height,
            "finalized_height": self.finalized_height,
            "target_locked": self.target_locked,
            "target_pin_root": self.root(),
        })
    }

    pub fn root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:target-height-pin",
            &[HashPart::Json(&self.canonical())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeferredGateRoot {
    pub gate_id: String,
    pub deferred_gate_root: String,
    pub gate_policy_root: String,
    pub target_height: u64,
    pub defers_release: bool,
    pub fail_closed_required: bool,
}

impl DeferredGateRoot {
    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("gate_id", &self.gate_id)?;
        ensure_root("deferred_gate_root", &self.deferred_gate_root)?;
        ensure_root("gate_policy_root", &self.gate_policy_root)?;
        ensure!(
            self.target_height == config.target_height,
            "deferred gate target height must match handoff target height",
        )
    }

    pub fn armed(&self) -> bool {
        self.defers_release && self.fail_closed_required
    }

    pub fn canonical(&self) -> Value {
        json!({
            "gate_id": self.gate_id,
            "deferred_gate_root": self.deferred_gate_root,
            "gate_policy_root": self.gate_policy_root,
            "target_height": self.target_height,
            "defers_release": self.defers_release,
            "fail_closed_required": self.fail_closed_required,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate_binding_root": self.root(),
            "target_height": self.target_height,
            "defers_release": self.defers_release,
            "fail_closed_required": self.fail_closed_required,
        })
    }

    pub fn root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:deferred-gate",
            &[HashPart::Json(&self.canonical())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandRoomTransferAck {
    pub ack_id: String,
    pub operator_commitment_root: String,
    pub command_room_root: String,
    pub transfer_ack_root: String,
    pub target_height: u64,
    pub acknowledges_replay_root: String,
    pub acknowledges_deferred_gate_root: String,
    pub accepts_handoff: bool,
    pub acknowledges_fail_closed: bool,
}

impl CommandRoomTransferAck {
    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_non_empty("ack_id", &self.ack_id)?;
        ensure_root("operator_commitment_root", &self.operator_commitment_root)?;
        ensure_root("command_room_root", &self.command_room_root)?;
        ensure_root("transfer_ack_root", &self.transfer_ack_root)?;
        ensure_root("acknowledges_replay_root", &self.acknowledges_replay_root)?;
        ensure_root(
            "acknowledges_deferred_gate_root",
            &self.acknowledges_deferred_gate_root,
        )?;
        ensure!(
            self.target_height == config.target_height,
            "command-room acknowledgement target height must match handoff target height",
        )
    }

    pub fn blocks_handoff(&self) -> bool {
        !self.accepts_handoff || !self.acknowledges_fail_closed
    }

    pub fn canonical(&self) -> Value {
        json!({
            "ack_id": self.ack_id,
            "operator_commitment_root": self.operator_commitment_root,
            "command_room_root": self.command_room_root,
            "transfer_ack_root": self.transfer_ack_root,
            "target_height": self.target_height,
            "acknowledges_replay_root": self.acknowledges_replay_root,
            "acknowledges_deferred_gate_root": self.acknowledges_deferred_gate_root,
            "accepts_handoff": self.accepts_handoff,
            "acknowledges_fail_closed": self.acknowledges_fail_closed,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ack_binding_root": self.root(),
            "target_height": self.target_height,
            "accepts_handoff": self.accepts_handoff,
            "acknowledges_fail_closed": self.acknowledges_fail_closed,
        })
    }

    pub fn root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:command-room-ack",
            &[HashPart::Json(&self.canonical())],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffBlocker {
    pub code: String,
    pub phase: HandoffPhase,
    pub blocker_root: String,
    pub fail_closed: bool,
}

impl HandoffBlocker {
    pub fn new(code: &str, phase: HandoffPhase, parts: &[HashPart<'_>]) -> Self {
        Self {
            code: code.to_string(),
            phase,
            blocker_root: domain_hash(
                "runtime-replay-incident-handoff:blocker",
                &[
                    HashPart::Str(code),
                    HashPart::Str(phase.as_str()),
                    HashPart::Str(&domain_hash(
                        "runtime-replay-incident-handoff:blocker-payload",
                        parts,
                        32,
                    )),
                ],
                32,
            ),
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker code", &self.code)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        ensure(self.fail_closed, "handoff blocker must be fail-closed")
    }

    pub fn canonical(&self) -> Value {
        json!({
            "code": self.code,
            "phase": self.phase.as_str(),
            "phase_ordinal": self.phase.ordinal(),
            "blocker_root": self.blocker_root,
            "fail_closed": self.fail_closed,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffCounters {
    pub replay_transcript_count: u64,
    pub accepted_replay_transcript_count: u64,
    pub fail_closed_replay_count: u64,
    pub deferred_gate_root_count: u64,
    pub armed_deferred_gate_count: u64,
    pub command_room_ack_count: u64,
    pub accepted_command_room_ack_count: u64,
    pub fail_closed_ack_count: u64,
    pub blocker_count: u64,
}

impl HandoffCounters {
    pub fn canonical(&self) -> Value {
        json!({
            "replay_transcript_count": self.replay_transcript_count,
            "accepted_replay_transcript_count": self.accepted_replay_transcript_count,
            "fail_closed_replay_count": self.fail_closed_replay_count,
            "deferred_gate_root_count": self.deferred_gate_root_count,
            "armed_deferred_gate_count": self.armed_deferred_gate_count,
            "command_room_ack_count": self.command_room_ack_count,
            "accepted_command_room_ack_count": self.accepted_command_room_ack_count,
            "fail_closed_ack_count": self.fail_closed_ack_count,
            "blocker_count": self.blocker_count,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffDecision {
    pub verdict: HandoffVerdict,
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub decision_root: String,
}

impl HandoffDecision {
    pub fn validate(&self) -> Result<()> {
        ensure_root("decision_root", &self.decision_root)?;
        ensure(
            self.fail_closed == !self.release_allowed,
            "handoff decision fail-closed flag must oppose release allowance",
        )?;
        ensure(
            self.verdict.release_allowed() == self.release_allowed,
            "handoff decision release allowance must match verdict",
        )
    }

    pub fn canonical(&self) -> Value {
        json!({
            "verdict": self.verdict.as_str(),
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "decision_root": self.decision_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub incident_handoff_suite: String,
    pub config: Config,
    pub replay_transcripts: Vec<ReplayTranscriptBinding>,
    pub target_height_pin: TargetHeightPin,
    pub deferred_gate_roots: Vec<DeferredGateRoot>,
    pub command_room_acks: Vec<CommandRoomTransferAck>,
    pub blockers: Vec<HandoffBlocker>,
    pub counters: HandoffCounters,
    pub replay_transcript_root: String,
    pub target_height_root: String,
    pub deferred_gate_root: String,
    pub command_room_ack_root: String,
    pub blocker_root: String,
    pub handoff_root: String,
    pub decision: HandoffDecision,
}

impl State {
    pub fn new(
        config: Config,
        replay_transcripts: Vec<ReplayTranscriptBinding>,
        target_height_pin: TargetHeightPin,
        deferred_gate_roots: Vec<DeferredGateRoot>,
        command_room_acks: Vec<CommandRoomTransferAck>,
    ) -> Result<Self> {
        config.validate()?;
        validate_replay_transcripts(&config, &replay_transcripts)?;
        target_height_pin.validate(&config)?;
        validate_deferred_gate_roots(&config, &deferred_gate_roots)?;
        validate_command_room_acks(&config, &command_room_acks)?;

        let mut blockers = collect_blockers(
            &config,
            &replay_transcripts,
            &target_height_pin,
            &deferred_gate_roots,
            &command_room_acks,
        );
        for blocker in &blockers {
            blocker.validate()?;
        }
        blockers.sort_by(|left, right| {
            left.phase
                .cmp(&right.phase)
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.blocker_root.cmp(&right.blocker_root))
        });

        let counters = count_handoff(
            &replay_transcripts,
            &deferred_gate_roots,
            &command_room_acks,
            &blockers,
        );
        let replay_transcript_root = merkle_root(
            "runtime-replay-incident-handoff:replay-transcripts",
            &replay_transcripts
                .iter()
                .map(ReplayTranscriptBinding::canonical)
                .collect::<Vec<_>>(),
        );
        let target_height_root = target_height_pin.root();
        let deferred_gate_root = merkle_root(
            "runtime-replay-incident-handoff:deferred-gates",
            &deferred_gate_roots
                .iter()
                .map(DeferredGateRoot::canonical)
                .collect::<Vec<_>>(),
        );
        let command_room_ack_root = merkle_root(
            "runtime-replay-incident-handoff:command-room-acks",
            &command_room_acks
                .iter()
                .map(CommandRoomTransferAck::canonical)
                .collect::<Vec<_>>(),
        );
        let blocker_root = merkle_root(
            "runtime-replay-incident-handoff:blockers",
            &blockers
                .iter()
                .map(HandoffBlocker::canonical)
                .collect::<Vec<_>>(),
        );
        let handoff_root = domain_hash(
            "runtime-replay-incident-handoff:state",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::U64(SCHEMA_VERSION),
                HashPart::Str(&config.root()),
                HashPart::Str(&replay_transcript_root),
                HashPart::Str(&target_height_root),
                HashPart::Str(&deferred_gate_root),
                HashPart::Str(&command_room_ack_root),
                HashPart::Str(&blocker_root),
                HashPart::Json(&counters.canonical()),
            ],
            32,
        );

        let verdict = if !blockers.is_empty() || config.require_fail_closed_transfer {
            HandoffVerdict::FailClosed
        } else {
            HandoffVerdict::Ready
        };
        let decision_root = domain_hash(
            "runtime-replay-incident-handoff:decision",
            &[
                HashPart::Str(verdict.as_str()),
                HashPart::Str(bool_str(verdict.release_allowed())),
                HashPart::Str(bool_str(verdict.fail_closed())),
                HashPart::Str(&handoff_root),
                HashPart::Str(&blocker_root),
            ],
            32,
        );
        let decision = HandoffDecision {
            verdict,
            release_allowed: verdict.release_allowed(),
            fail_closed: verdict.fail_closed(),
            decision_root,
        };
        decision.validate()?;

        let state = Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            incident_handoff_suite: INCIDENT_HANDOFF_SUITE.to_string(),
            config,
            replay_transcripts,
            target_height_pin,
            deferred_gate_roots,
            command_room_acks,
            blockers,
            counters,
            replay_transcript_root,
            target_height_root,
            deferred_gate_root,
            command_room_ack_root,
            blocker_root,
            handoff_root,
            decision,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.protocol_version == PROTOCOL_VERSION,
            "protocol version mismatch",
        )?;
        ensure(
            self.schema_version == SCHEMA_VERSION,
            "schema version mismatch",
        )?;
        ensure(self.hash_suite == HASH_SUITE, "hash suite mismatch")?;
        ensure!(
            self.incident_handoff_suite == INCIDENT_HANDOFF_SUITE,
            "incident handoff suite mismatch",
        )?;
        self.config.validate()?;
        validate_replay_transcripts(&self.config, &self.replay_transcripts)?;
        self.target_height_pin.validate(&self.config)?;
        validate_deferred_gate_roots(&self.config, &self.deferred_gate_roots)?;
        validate_command_room_acks(&self.config, &self.command_room_acks)?;
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        ensure_root("replay_transcript_root", &self.replay_transcript_root)?;
        ensure_root("target_height_root", &self.target_height_root)?;
        ensure_root("deferred_gate_root", &self.deferred_gate_root)?;
        ensure_root("command_room_ack_root", &self.command_room_ack_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        ensure_root("handoff_root", &self.handoff_root)?;
        self.decision.validate()?;
        ensure!(
            self.decision.fail_closed || self.blockers.is_empty(),
            "handoff with blockers must remain fail-closed",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "incident_handoff_suite": self.incident_handoff_suite,
            "chain_id": self.config.chain_id,
            "incident_height": self.config.incident_height,
            "target_height": self.config.target_height,
            "config_root": self.config.root(),
            "replay_transcript_root": self.replay_transcript_root,
            "target_height_root": self.target_height_root,
            "deferred_gate_root": self.deferred_gate_root,
            "command_room_ack_root": self.command_room_ack_root,
            "blocker_root": self.blocker_root,
            "handoff_root": self.handoff_root,
            "decision_root": self.decision.decision_root,
            "release_allowed": self.decision.release_allowed,
            "fail_closed": self.decision.fail_closed,
            "verdict": self.decision.verdict.as_str(),
            "counters": self.counters.canonical(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "runtime-replay-incident-handoff:public-state-root",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    pub fn fail_closed(&self) -> bool {
        self.decision.fail_closed
    }
}

pub fn devnet() -> Result<Runtime> {
    let config = Config::default();
    let transcript_a = replay_transcript("primary-replay-transcript", &config, 0, true, true);
    let transcript_b = replay_transcript("secondary-replay-transcript", &config, 1, true, true);
    let transcript_c = replay_transcript("fallback-replay-transcript", &config, 2, true, true);
    let replay_root = merkle_root(
        "runtime-replay-incident-handoff:devnet-replay-commitments",
        &[
            transcript_a.canonical(),
            transcript_b.canonical(),
            transcript_c.canonical(),
        ],
    );

    let target_height_pin = TargetHeightPin {
        target_height: config.target_height,
        finalized_height: config.incident_height,
        source_checkpoint_root: sample_root("source-checkpoint", config.target_height),
        replay_window_root: sample_root("replay-window", config.incident_height),
        target_locked: true,
    };

    let gate_a = deferred_gate("runtime-replay-gate", &config, 0, true, true);
    let gate_b = deferred_gate("operator-dashboard-gate", &config, 1, true, true);
    let gate_c = deferred_gate("release-policy-gate", &config, 2, true, true);
    let gate_root = merkle_root(
        "runtime-replay-incident-handoff:devnet-gate-commitments",
        &[gate_a.canonical(), gate_b.canonical(), gate_c.canonical()],
    );

    let command_room_acks = vec![
        command_room_ack("incident-commander", &config, 0, &replay_root, &gate_root),
        command_room_ack("release-captain", &config, 1, &replay_root, &gate_root),
        command_room_ack("bridge-ops", &config, 2, &replay_root, &gate_root),
        command_room_ack("security-ops", &config, 3, &replay_root, &gate_root),
    ];

    State::new(
        config,
        vec![transcript_a, transcript_b, transcript_c],
        target_height_pin,
        vec![gate_a, gate_b, gate_c],
        command_room_acks,
    )
}

fn validate_replay_transcripts(
    config: &Config,
    transcripts: &[ReplayTranscriptBinding],
) -> Result<()> {
    ensure!(
        transcripts.len() >= usize::from(config.min_replay_transcripts),
        "insufficient replay transcripts for incident handoff",
    )?;
    let mut ids = BTreeSet::new();
    for transcript in transcripts {
        transcript.validate(config)?;
        ensure(
            ids.insert(transcript.transcript_id.clone()),
            "duplicate replay transcript id",
        )?;
    }
    Ok(())
}

fn validate_deferred_gate_roots(config: &Config, gates: &[DeferredGateRoot]) -> Result<()> {
    ensure!(
        gates.len() >= usize::from(config.min_deferred_gate_roots),
        "insufficient deferred gate roots for incident handoff",
    )?;
    let mut ids = BTreeSet::new();
    for gate in gates {
        gate.validate(config)?;
        ensure(
            ids.insert(gate.gate_id.clone()),
            "duplicate deferred gate id",
        )?;
    }
    Ok(())
}

fn validate_command_room_acks(config: &Config, acks: &[CommandRoomTransferAck]) -> Result<()> {
    ensure!(
        acks.len() >= usize::from(config.min_command_room_acks),
        "insufficient command-room transfer acknowledgements",
    )?;
    let mut ids = BTreeSet::new();
    for ack in acks {
        ack.validate(config)?;
        ensure(
            ids.insert(ack.ack_id.clone()),
            "duplicate command-room acknowledgement id",
        )?;
    }
    Ok(())
}

fn collect_blockers(
    config: &Config,
    replay_transcripts: &[ReplayTranscriptBinding],
    target_height_pin: &TargetHeightPin,
    deferred_gate_roots: &[DeferredGateRoot],
    command_room_acks: &[CommandRoomTransferAck],
) -> Vec<HandoffBlocker> {
    let mut blockers = Vec::new();
    if !target_height_pin.target_locked {
        blockers.push(HandoffBlocker::new(
            "target_height_not_locked",
            HandoffPhase::TargetHeightPinned,
            &[HashPart::U64(target_height_pin.target_height)],
        ));
    }
    if replay_transcripts
        .iter()
        .any(ReplayTranscriptBinding::blocks_handoff)
    {
        blockers.push(HandoffBlocker::new(
            "replay_transcript_not_fail_closed",
            HandoffPhase::ReplayBound,
            &[HashPart::U64(replay_transcripts.len() as u64)],
        ));
    }
    if deferred_gate_roots.iter().any(|gate| !gate.armed()) {
        blockers.push(HandoffBlocker::new(
            "deferred_gate_not_armed",
            HandoffPhase::DeferredGateBound,
            &[HashPart::U64(deferred_gate_roots.len() as u64)],
        ));
    }
    if command_room_acks
        .iter()
        .any(CommandRoomTransferAck::blocks_handoff)
    {
        blockers.push(HandoffBlocker::new(
            "command_room_transfer_not_acknowledged",
            HandoffPhase::CommandRoomTransfer,
            &[HashPart::U64(command_room_acks.len() as u64)],
        ));
    }
    if config.require_fail_closed_transfer {
        blockers.push(HandoffBlocker::new(
            "release_policy_requires_fail_closed_handoff",
            HandoffPhase::FailClosedHandoff,
            &[HashPart::U64(config.incident_height)],
        ));
    }
    blockers
}

fn count_handoff(
    replay_transcripts: &[ReplayTranscriptBinding],
    deferred_gate_roots: &[DeferredGateRoot],
    command_room_acks: &[CommandRoomTransferAck],
    blockers: &[HandoffBlocker],
) -> HandoffCounters {
    HandoffCounters {
        replay_transcript_count: replay_transcripts.len() as u64,
        accepted_replay_transcript_count: replay_transcripts
            .iter()
            .filter(|transcript| transcript.accepted)
            .count() as u64,
        fail_closed_replay_count: replay_transcripts
            .iter()
            .filter(|transcript| transcript.fail_closed_observed)
            .count() as u64,
        deferred_gate_root_count: deferred_gate_roots.len() as u64,
        armed_deferred_gate_count: deferred_gate_roots
            .iter()
            .filter(|gate| gate.armed())
            .count() as u64,
        command_room_ack_count: command_room_acks.len() as u64,
        accepted_command_room_ack_count: command_room_acks
            .iter()
            .filter(|ack| ack.accepts_handoff)
            .count() as u64,
        fail_closed_ack_count: command_room_acks
            .iter()
            .filter(|ack| ack.acknowledges_fail_closed)
            .count() as u64,
        blocker_count: blockers.len() as u64,
    }
}

fn replay_transcript(
    label: &str,
    config: &Config,
    offset: u64,
    accepted: bool,
    fail_closed_observed: bool,
) -> ReplayTranscriptBinding {
    ReplayTranscriptBinding {
        transcript_id: label.to_string(),
        replay_transcript_root: sample_root(label, config.target_height + offset),
        rollback_command_root: sample_root("rollback-command", config.target_height + offset),
        accepted_receipt_root: sample_root("accepted-receipt", config.target_height + offset),
        target_height: config.target_height,
        replayed_height: config.target_height + offset,
        accepted,
        fail_closed_observed,
    }
}

fn deferred_gate(
    label: &str,
    config: &Config,
    offset: u64,
    defers_release: bool,
    fail_closed_required: bool,
) -> DeferredGateRoot {
    DeferredGateRoot {
        gate_id: label.to_string(),
        deferred_gate_root: sample_root(label, config.target_height + offset),
        gate_policy_root: sample_root("gate-policy", config.target_height + offset),
        target_height: config.target_height,
        defers_release,
        fail_closed_required,
    }
}

fn command_room_ack(
    label: &str,
    config: &Config,
    offset: u64,
    replay_root: &str,
    gate_root: &str,
) -> CommandRoomTransferAck {
    CommandRoomTransferAck {
        ack_id: label.to_string(),
        operator_commitment_root: sample_root("operator-commitment", offset),
        command_room_root: sample_root("command-room", config.incident_height + offset),
        transfer_ack_root: sample_root(label, config.incident_height + offset),
        target_height: config.target_height,
        acknowledges_replay_root: replay_root.to_string(),
        acknowledges_deferred_gate_root: gate_root.to_string(),
        accepts_handoff: true,
        acknowledges_fail_closed: true,
    }
}

fn sample_root(label: &str, height: u64) -> String {
    domain_hash(
        "runtime-replay-incident-handoff:sample-root",
        &[HashPart::Str(label), HashPart::U64(height)],
        32,
    )
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
    ensure(
        value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()),
        &format!("{label} must be a 32-byte lowercase hex root"),
    )?;
    ensure(
        value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        &format!("{label} must use lowercase hex"),
    )
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
