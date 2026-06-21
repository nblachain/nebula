use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageBridgeCustodyAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-bridge-custody-release-captain-go-no-go-replay-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_DRILL_SUITE: &str =
    "monero-l2-pq-bridge-custody-release-captain-go-no-go-replay-drill-v1";
pub const DEFAULT_WAVE: u64 = 88;
pub const DEFAULT_SOURCE_WAVE: u64 = 87;
pub const DEFAULT_REPLAY_HEIGHT: u64 = 1_445_448;
pub const DEFAULT_MAX_CHECKLIST_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_RESERVE_REPLAY_CHECKS: u64 = 3;
pub const DEFAULT_MIN_RELEASE_CAP_HOLD_ROOTS: u64 = 3;
pub const DEFAULT_MIN_SIGNER_ACKS: u64 = 4;
pub const DEFAULT_MIN_SIGNER_ACK_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_PAUSE_BLOCKERS: u64 = 2;
pub const DEFAULT_MIN_COMMAND_RECEIPTS: u64 = 3;
pub const DEFAULT_MIN_CAPTAIN_SIGNOFF_WEIGHT: u64 = 80;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Pending,
    Rejected,
    Expired,
    Blocked,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Blocked => "blocked",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPhase {
    Wave87ChecklistBound,
    ReserveLiabilityReplay,
    ReleaseCapHold,
    SignerCustodyAck,
    BridgePauseBlocked,
    CommandReceiptReplay,
    ReleaseCaptainSignoff,
    FailClosedVerdict,
}

impl ReplayPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave87ChecklistBound => "wave87_checklist_bound",
            Self::ReserveLiabilityReplay => "reserve_liability_replay",
            Self::ReleaseCapHold => "release_cap_hold",
            Self::SignerCustodyAck => "signer_custody_ack",
            Self::BridgePauseBlocked => "bridge_pause_blocked",
            Self::CommandReceiptReplay => "command_receipt_replay",
            Self::ReleaseCaptainSignoff => "release_captain_signoff",
            Self::FailClosedVerdict => "fail_closed_verdict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDecision {
    GoAfterHoldRoots,
    NoGoKeepPaused,
    MoreEvidenceRequired,
}

impl ReplayDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GoAfterHoldRoots => "go_after_hold_roots",
            Self::NoGoKeepPaused => "no_go_keep_paused",
            Self::MoreEvidenceRequired => "more_evidence_required",
        }
    }

    pub fn release_blocking(self) -> bool {
        !matches!(self, Self::GoAfterHoldRoots)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayBlockerKind {
    SourceChecklistRootMissing,
    SourceChecklistStale,
    SourceChecklistNotFailClosed,
    ReserveReplayRootMissing,
    ReserveLiabilityMismatch,
    ReleaseCapHoldRootMissing,
    ReleaseCapNotHeld,
    SignerCustodyAckQuorumLow,
    SignerCustodyAckWeightLow,
    BridgePauseBlockerCountLow,
    BridgePauseNotAsserted,
    CommandReplayReceiptMissing,
    CommandReplayReceiptRejected,
    CaptainSignoffWeightLow,
    CaptainNoGo,
    FailClosedVerdictMissing,
    ReleaseVerdictOpen,
}

impl ReplayBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceChecklistRootMissing => "source_checklist_root_missing",
            Self::SourceChecklistStale => "source_checklist_stale",
            Self::SourceChecklistNotFailClosed => "source_checklist_not_fail_closed",
            Self::ReserveReplayRootMissing => "reserve_replay_root_missing",
            Self::ReserveLiabilityMismatch => "reserve_liability_mismatch",
            Self::ReleaseCapHoldRootMissing => "release_cap_hold_root_missing",
            Self::ReleaseCapNotHeld => "release_cap_not_held",
            Self::SignerCustodyAckQuorumLow => "signer_custody_ack_quorum_low",
            Self::SignerCustodyAckWeightLow => "signer_custody_ack_weight_low",
            Self::BridgePauseBlockerCountLow => "bridge_pause_blocker_count_low",
            Self::BridgePauseNotAsserted => "bridge_pause_not_asserted",
            Self::CommandReplayReceiptMissing => "command_replay_receipt_missing",
            Self::CommandReplayReceiptRejected => "command_replay_receipt_rejected",
            Self::CaptainSignoffWeightLow => "captain_signoff_weight_low",
            Self::CaptainNoGo => "captain_no_go",
            Self::FailClosedVerdictMissing => "fail_closed_verdict_missing",
            Self::ReleaseVerdictOpen => "release_verdict_open",
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
    pub replay_drill_id: String,
    pub source_wave87_checklist_id: String,
    pub bridge_custody_lane_id: String,
    pub release_policy_id: String,
    pub wave: u64,
    pub source_wave: u64,
    pub replay_height: u64,
    pub max_checklist_age_blocks: u64,
    pub min_reserve_replay_checks: u64,
    pub min_release_cap_hold_roots: u64,
    pub min_signer_acknowledgements: u64,
    pub min_signer_acknowledgement_weight: u64,
    pub min_bridge_pause_blockers: u64,
    pub min_command_replay_receipts: u64,
    pub min_captain_signoff_weight: u64,
    pub require_source_checklist_fail_closed: bool,
    pub require_reserve_liability_match: bool,
    pub require_release_cap_holds: bool,
    pub require_signer_custody_acknowledgements: bool,
    pub require_bridge_pause_blockers: bool,
    pub require_command_replay_receipts: bool,
    pub require_release_captain_signoff: bool,
    pub require_fail_closed_verdict: bool,
    pub fail_closed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_drill_suite: REPLAY_DRILL_SUITE.to_string(),
            replay_drill_id: runtime_id("wave-88-release-captain-go-no-go-replay-drill"),
            source_wave87_checklist_id: runtime_id(
                "wave-87-bridge-custody-operator-command-checklist",
            ),
            bridge_custody_lane_id: "bridge_custody".to_string(),
            release_policy_id: runtime_id("force-exit-package-bridge-custody-release-policy"),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            replay_height: DEFAULT_REPLAY_HEIGHT,
            max_checklist_age_blocks: DEFAULT_MAX_CHECKLIST_AGE_BLOCKS,
            min_reserve_replay_checks: DEFAULT_MIN_RESERVE_REPLAY_CHECKS,
            min_release_cap_hold_roots: DEFAULT_MIN_RELEASE_CAP_HOLD_ROOTS,
            min_signer_acknowledgements: DEFAULT_MIN_SIGNER_ACKS,
            min_signer_acknowledgement_weight: DEFAULT_MIN_SIGNER_ACK_WEIGHT,
            min_bridge_pause_blockers: DEFAULT_MIN_PAUSE_BLOCKERS,
            min_command_replay_receipts: DEFAULT_MIN_COMMAND_RECEIPTS,
            min_captain_signoff_weight: DEFAULT_MIN_CAPTAIN_SIGNOFF_WEIGHT,
            require_source_checklist_fail_closed: true,
            require_reserve_liability_match: true,
            require_release_cap_holds: true,
            require_signer_custody_acknowledgements: true,
            require_bridge_pause_blockers: true,
            require_command_replay_receipts: true,
            require_release_captain_signoff: true,
            require_fail_closed_verdict: true,
            fail_closed: true,
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
        ensure_non_empty("replay_drill_id", &self.replay_drill_id)?;
        ensure_non_empty(
            "source_wave87_checklist_id",
            &self.source_wave87_checklist_id,
        )?;
        ensure_non_empty("bridge_custody_lane_id", &self.bridge_custody_lane_id)?;
        ensure_non_empty("release_policy_id", &self.release_policy_id)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(
            self.wave > self.source_wave,
            "replay drill wave must follow source wave",
        )?;
        ensure(self.replay_height > 0, "replay height must be non-zero")?;
        ensure(
            self.max_checklist_age_blocks > 0,
            "checklist age window must be non-zero",
        )?;
        ensure(
            self.min_reserve_replay_checks > 0,
            "reserve replay threshold must be non-zero",
        )?;
        ensure(
            self.min_release_cap_hold_roots > 0,
            "release cap hold threshold must be non-zero",
        )?;
        ensure(
            self.min_signer_acknowledgements > 0,
            "signer acknowledgement threshold must be non-zero",
        )?;
        ensure(
            self.min_signer_acknowledgement_weight > 0,
            "signer acknowledgement weight must be non-zero",
        )?;
        ensure(
            self.min_bridge_pause_blockers > 0,
            "bridge pause blocker threshold must be non-zero",
        )?;
        ensure(
            self.min_command_replay_receipts > 0,
            "command replay receipt threshold must be non-zero",
        )?;
        ensure(
            self.min_captain_signoff_weight > 0,
            "captain signoff weight must be non-zero",
        )?;
        ensure(
            self.fail_closed,
            "release captain replay drill must default fail-closed",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "replay_drill_suite": self.replay_drill_suite,
            "replay_drill_id": self.replay_drill_id,
            "source_wave87_checklist_id": self.source_wave87_checklist_id,
            "bridge_custody_lane_id": self.bridge_custody_lane_id,
            "release_policy_id": self.release_policy_id,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "replay_height": self.replay_height,
            "max_checklist_age_blocks": self.max_checklist_age_blocks,
            "min_reserve_replay_checks": self.min_reserve_replay_checks,
            "min_release_cap_hold_roots": self.min_release_cap_hold_roots,
            "min_signer_acknowledgements": self.min_signer_acknowledgements,
            "min_signer_acknowledgement_weight": self.min_signer_acknowledgement_weight,
            "min_bridge_pause_blockers": self.min_bridge_pause_blockers,
            "min_command_replay_receipts": self.min_command_replay_receipts,
            "min_captain_signoff_weight": self.min_captain_signoff_weight,
            "require_source_checklist_fail_closed": self.require_source_checklist_fail_closed,
            "require_reserve_liability_match": self.require_reserve_liability_match,
            "require_release_cap_holds": self.require_release_cap_holds,
            "require_signer_custody_acknowledgements": self.require_signer_custody_acknowledgements,
            "require_bridge_pause_blockers": self.require_bridge_pause_blockers,
            "require_command_replay_receipts": self.require_command_replay_receipts,
            "require_release_captain_signoff": self.require_release_captain_signoff,
            "require_fail_closed_verdict": self.require_fail_closed_verdict,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("RELEASE-CAPTAIN-REPLAY-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave87ChecklistRoots {
    pub checklist_id: String,
    pub observed_height: u64,
    pub checklist_root: String,
    pub reserve_liability_root: String,
    pub release_cap_root: String,
    pub signer_ack_root: String,
    pub command_receipt_root: String,
    pub fail_closed_root: String,
    pub fail_closed: bool,
    pub status: EvidenceStatus,
}

impl Wave87ChecklistRoots {
    pub fn devnet(config: &Config) -> Self {
        let observed_height = config.replay_height.saturating_sub(24);
        let reserve_liability_root =
            component_root(config, "wave87-reserve-liability", "source", 1);
        let release_cap_root = component_root(config, "wave87-release-cap", "source", 2);
        let signer_ack_root = component_root(config, "wave87-signer-ack", "source", 3);
        let command_receipt_root = component_root(config, "wave87-command-receipt", "source", 4);
        let fail_closed_root = component_root(config, "wave87-fail-closed", "source", 5);
        let checklist_root = merkle_root(
            "RELEASE-CAPTAIN-REPLAY-WAVE87-CHECKLIST",
            &[
                json!({"reserve_liability_root": reserve_liability_root}),
                json!({"release_cap_root": release_cap_root}),
                json!({"signer_ack_root": signer_ack_root}),
                json!({"command_receipt_root": command_receipt_root}),
                json!({"fail_closed_root": fail_closed_root}),
            ],
        );
        Self {
            checklist_id: config.source_wave87_checklist_id.clone(),
            observed_height,
            checklist_root,
            reserve_liability_root,
            release_cap_root,
            signer_ack_root,
            command_receipt_root,
            fail_closed_root,
            fail_closed: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.checklist_root.trim().is_empty()
            && !self.fail_closed_root.trim().is_empty()
            && (!config.require_source_checklist_fail_closed || self.fail_closed)
            && config.replay_height.saturating_sub(self.observed_height)
                <= config.max_checklist_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "checklist_id": self.checklist_id,
            "observed_height": self.observed_height,
            "checklist_root": self.checklist_root,
            "reserve_liability_root": self.reserve_liability_root,
            "release_cap_root": self.release_cap_root,
            "signer_ack_root": self.signer_ack_root,
            "command_receipt_root": self.command_receipt_root,
            "fail_closed_root": self.fail_closed_root,
            "fail_closed": self.fail_closed,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-WAVE87-CHECKLIST",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveLiabilityReplayCheck {
    pub check_id: String,
    pub reserve_operator_id: String,
    pub source_root: String,
    pub replay_root: String,
    pub liability_delta_atomic_units: i64,
    pub balance_matches_liability: bool,
    pub status: EvidenceStatus,
}

impl ReserveLiabilityReplayCheck {
    pub fn devnet(config: &Config, ordinal: u64, operator: &str) -> Self {
        let check_id = evidence_id(config, "reserve-replay", operator, ordinal);
        Self {
            source_root: component_root(config, "reserve-source", &check_id, ordinal),
            replay_root: component_root(config, "reserve-replay", &check_id, ordinal),
            check_id,
            reserve_operator_id: operator.to_string(),
            liability_delta_atomic_units: 0,
            balance_matches_liability: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.source_root.trim().is_empty()
            && !self.replay_root.trim().is_empty()
            && (!config.require_reserve_liability_match || self.balance_matches_liability)
            && self.liability_delta_atomic_units == 0
    }

    pub fn public_record(&self) -> Value {
        json!({
            "check_id": self.check_id,
            "reserve_operator_id": self.reserve_operator_id,
            "source_root": self.source_root,
            "replay_root": self.replay_root,
            "liability_delta_atomic_units": self.liability_delta_atomic_units,
            "balance_matches_liability": self.balance_matches_liability,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-RESERVE-CHECK",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCapHoldRoot {
    pub hold_id: String,
    pub cap_scope: String,
    pub hold_root: String,
    pub release_cap_root: String,
    pub cap_limit_atomic_units: u64,
    pub consumed_atomic_units: u64,
    pub hold_asserted: bool,
    pub status: EvidenceStatus,
}

impl ReleaseCapHoldRoot {
    pub fn devnet(config: &Config, ordinal: u64, cap_scope: &str) -> Self {
        let hold_id = evidence_id(config, "release-cap-hold", cap_scope, ordinal);
        Self {
            hold_root: component_root(config, "cap-hold", &hold_id, ordinal),
            release_cap_root: component_root(config, "cap-root", &hold_id, ordinal),
            hold_id,
            cap_scope: cap_scope.to_string(),
            cap_limit_atomic_units: 0,
            consumed_atomic_units: 0,
            hold_asserted: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.hold_root.trim().is_empty()
            && !self.release_cap_root.trim().is_empty()
            && (!config.require_release_cap_holds || self.hold_asserted)
            && self.consumed_atomic_units <= self.cap_limit_atomic_units
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hold_id": self.hold_id,
            "cap_scope": self.cap_scope,
            "hold_root": self.hold_root,
            "release_cap_root": self.release_cap_root,
            "cap_limit_atomic_units": self.cap_limit_atomic_units,
            "consumed_atomic_units": self.consumed_atomic_units,
            "hold_asserted": self.hold_asserted,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("RELEASE-CAPTAIN-REPLAY-CAP-HOLD", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignerCustodyAcknowledgement {
    pub acknowledgement_id: String,
    pub signer_id: String,
    pub custody_ack_root: String,
    pub signer_weight: u64,
    pub accepted_custody: bool,
    pub status: EvidenceStatus,
}

impl SignerCustodyAcknowledgement {
    pub fn devnet(config: &Config, ordinal: u64, signer: &str, signer_weight: u64) -> Self {
        let acknowledgement_id = evidence_id(config, "signer-custody-ack", signer, ordinal);
        Self {
            custody_ack_root: component_root(
                config,
                "signer-custody-ack",
                &acknowledgement_id,
                ordinal,
            ),
            acknowledgement_id,
            signer_id: signer.to_string(),
            signer_weight,
            accepted_custody: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.accepted_custody
            && self.signer_weight > 0
            && !self.custody_ack_root.trim().is_empty()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "acknowledgement_id": self.acknowledgement_id,
            "signer_id": self.signer_id,
            "custody_ack_root": self.custody_ack_root,
            "signer_weight": self.signer_weight,
            "accepted_custody": self.accepted_custody,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("RELEASE-CAPTAIN-REPLAY-SIGNER-ACK", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BridgePauseBlocker {
    pub blocker_id: String,
    pub subject: String,
    pub pause_root: String,
    pub bridge_pause_asserted: bool,
    pub release_blocking: bool,
    pub status: EvidenceStatus,
}

impl BridgePauseBlocker {
    pub fn devnet(config: &Config, ordinal: u64, subject: &str) -> Self {
        let blocker_id = evidence_id(config, "bridge-pause-blocker", subject, ordinal);
        Self {
            pause_root: component_root(config, "bridge-pause", &blocker_id, ordinal),
            blocker_id,
            subject: subject.to_string(),
            bridge_pause_asserted: true,
            release_blocking: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.bridge_pause_asserted
            && self.release_blocking
            && !self.pause_root.trim().is_empty()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "subject": self.subject,
            "pause_root": self.pause_root,
            "bridge_pause_asserted": self.bridge_pause_asserted,
            "release_blocking": self.release_blocking,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-PAUSE-BLOCKER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandReplayReceipt {
    pub receipt_id: String,
    pub command_subject: String,
    pub command_root: String,
    pub replay_receipt_root: String,
    pub replayed: bool,
    pub status: EvidenceStatus,
}

impl CommandReplayReceipt {
    pub fn devnet(config: &Config, ordinal: u64, command_subject: &str) -> Self {
        let receipt_id = evidence_id(config, "command-replay-receipt", command_subject, ordinal);
        Self {
            command_root: component_root(config, "command-root", &receipt_id, ordinal),
            replay_receipt_root: component_root(config, "command-replay", &receipt_id, ordinal),
            receipt_id,
            command_subject: command_subject.to_string(),
            replayed: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.replayed
            && !self.command_root.trim().is_empty()
            && !self.replay_receipt_root.trim().is_empty()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "command_subject": self.command_subject,
            "command_root": self.command_root,
            "replay_receipt_root": self.replay_receipt_root,
            "replayed": self.replayed,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-COMMAND-RECEIPT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCaptainSignoff {
    pub signoff_id: String,
    pub captain_id: String,
    pub signoff_root: String,
    pub decision: ReplayDecision,
    pub signoff_weight: u64,
    pub bridge_release_allowed: bool,
    pub status: EvidenceStatus,
}

impl ReleaseCaptainSignoff {
    pub fn devnet(config: &Config, ordinal: u64, captain_id: &str, weight: u64) -> Self {
        let signoff_id = evidence_id(config, "release-captain-signoff", captain_id, ordinal);
        Self {
            signoff_root: component_root(config, "captain-signoff", &signoff_id, ordinal),
            signoff_id,
            captain_id: captain_id.to_string(),
            decision: ReplayDecision::NoGoKeepPaused,
            signoff_weight: weight,
            bridge_release_allowed: false,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.signoff_weight > 0
            && !self.signoff_root.trim().is_empty()
            && !self.bridge_release_allowed
            && self.decision.release_blocking()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "captain_id": self.captain_id,
            "signoff_root": self.signoff_root,
            "decision": self.decision.as_str(),
            "signoff_weight": self.signoff_weight,
            "bridge_release_allowed": self.bridge_release_allowed,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-CAPTAIN-SIGNOFF",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayBlocker {
    pub blocker_id: String,
    pub kind: ReplayBlockerKind,
    pub subject: String,
    pub evidence_root: String,
}

impl ReplayBlocker {
    pub fn new(
        config: &Config,
        kind: ReplayBlockerKind,
        subject: &str,
        evidence_root: &str,
    ) -> Self {
        Self {
            blocker_id: domain_hash(
                "RELEASE-CAPTAIN-REPLAY-BLOCKER-ID",
                &[
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(&config.replay_drill_id),
                    HashPart::Str(kind.as_str()),
                    HashPart::Str(subject),
                    HashPart::Str(evidence_root),
                ],
                16,
            ),
            kind,
            subject: subject.to_string(),
            evidence_root: evidence_root.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "subject": self.subject,
            "evidence_root": self.evidence_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("RELEASE-CAPTAIN-REPLAY-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseVerdict {
    pub replay_drill_id: String,
    pub reserve_replay_count: u64,
    pub release_cap_hold_count: u64,
    pub signer_acknowledgement_count: u64,
    pub signer_acknowledgement_weight: u64,
    pub bridge_pause_blocker_count: u64,
    pub command_replay_receipt_count: u64,
    pub captain_signoff_weight: u64,
    pub evidence_root: String,
    pub blocker_root: String,
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub accepted: bool,
    pub verdict_root: String,
}

impl ReleaseVerdict {
    pub fn from_state(config: &Config, state: &State, blockers: &[ReplayBlocker]) -> Self {
        let evidence_root = state.evidence_root();
        let blocker_root = list_root(
            "RELEASE-CAPTAIN-REPLAY-BLOCKER-ROOT",
            blockers.iter().map(ReplayBlocker::state_root),
        );
        let captain_signoff_weight = state.captain_signoff_weight();
        let release_allowed =
            blockers.is_empty() && state.captain_go_weight() >= config.min_captain_signoff_weight;
        let fail_closed = config.fail_closed && !release_allowed;
        let accepted = fail_closed
            && config.require_fail_closed_verdict
            && !state.release_captain_signoffs.is_empty();
        let verdict_seed = json!({
            "replay_drill_id": config.replay_drill_id,
            "evidence_root": evidence_root,
            "blocker_root": blocker_root,
            "release_allowed": release_allowed,
            "fail_closed": fail_closed,
        });
        let verdict_root = record_root("RELEASE-CAPTAIN-REPLAY-VERDICT", &verdict_seed);
        Self {
            replay_drill_id: config.replay_drill_id.clone(),
            reserve_replay_count: state.reserve_replay_count(),
            release_cap_hold_count: state.release_cap_hold_count(),
            signer_acknowledgement_count: state.signer_acknowledgement_count(),
            signer_acknowledgement_weight: state.signer_acknowledgement_weight(),
            bridge_pause_blocker_count: state.bridge_pause_blocker_count(),
            command_replay_receipt_count: state.command_replay_receipt_count(),
            captain_signoff_weight,
            evidence_root,
            blocker_root,
            release_allowed,
            fail_closed,
            accepted,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_drill_id": self.replay_drill_id,
            "reserve_replay_count": self.reserve_replay_count,
            "release_cap_hold_count": self.release_cap_hold_count,
            "signer_acknowledgement_count": self.signer_acknowledgement_count,
            "signer_acknowledgement_weight": self.signer_acknowledgement_weight,
            "bridge_pause_blocker_count": self.bridge_pause_blocker_count,
            "command_replay_receipt_count": self.command_replay_receipt_count,
            "captain_signoff_weight": self.captain_signoff_weight,
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "accepted": self.accepted,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "RELEASE-CAPTAIN-REPLAY-VERDICT-STATE",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source_checklist_roots: Wave87ChecklistRoots,
    pub reserve_replay_checks: Vec<ReserveLiabilityReplayCheck>,
    pub release_cap_hold_roots: Vec<ReleaseCapHoldRoot>,
    pub signer_custody_acknowledgements: Vec<SignerCustodyAcknowledgement>,
    pub bridge_pause_blockers: Vec<BridgePauseBlocker>,
    pub command_replay_receipts: Vec<CommandReplayReceipt>,
    pub release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
    pub phase_roots: BTreeMap<String, String>,
    pub blockers: Vec<ReplayBlocker>,
    pub verdict: ReleaseVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let source_checklist_roots = Wave87ChecklistRoots::devnet(&config);
        let reserve_replay_checks = vec![
            ReserveLiabilityReplayCheck::devnet(&config, 1, "reserve-operator-alpha"),
            ReserveLiabilityReplayCheck::devnet(&config, 2, "reserve-operator-bravo"),
            ReserveLiabilityReplayCheck::devnet(&config, 3, "reserve-operator-charlie"),
        ];
        let release_cap_hold_roots = vec![
            ReleaseCapHoldRoot::devnet(&config, 1, "per-incident"),
            ReleaseCapHoldRoot::devnet(&config, 2, "per-epoch"),
            ReleaseCapHoldRoot::devnet(&config, 3, "manual-release"),
        ];
        let signer_custody_acknowledgements = vec![
            SignerCustodyAcknowledgement::devnet(&config, 1, "custody-signer-alpha", 18),
            SignerCustodyAcknowledgement::devnet(&config, 2, "custody-signer-bravo", 17),
            SignerCustodyAcknowledgement::devnet(&config, 3, "custody-signer-charlie", 16),
            SignerCustodyAcknowledgement::devnet(&config, 4, "custody-signer-delta", 16),
            SignerCustodyAcknowledgement::devnet(&config, 5, "custody-signer-echo", 8),
        ];
        let bridge_pause_blockers = vec![
            BridgePauseBlocker::devnet(&config, 1, "release-cap-hold"),
            BridgePauseBlocker::devnet(&config, 2, "captain-no-go"),
        ];
        let command_replay_receipts = vec![
            CommandReplayReceipt::devnet(&config, 1, "reserve-liability-replay"),
            CommandReplayReceipt::devnet(&config, 2, "release-cap-hold-replay"),
            CommandReplayReceipt::devnet(&config, 3, "bridge-pause-replay"),
        ];
        let release_captain_signoffs = vec![
            ReleaseCaptainSignoff::devnet(&config, 1, "release-captain-primary", 45),
            ReleaseCaptainSignoff::devnet(&config, 2, "release-captain-secondary", 40),
        ];
        Self::new(
            config,
            source_checklist_roots,
            reserve_replay_checks,
            release_cap_hold_roots,
            signer_custody_acknowledgements,
            bridge_pause_blockers,
            command_replay_receipts,
            release_captain_signoffs,
        )
    }

    pub fn new(
        config: Config,
        source_checklist_roots: Wave87ChecklistRoots,
        reserve_replay_checks: Vec<ReserveLiabilityReplayCheck>,
        release_cap_hold_roots: Vec<ReleaseCapHoldRoot>,
        signer_custody_acknowledgements: Vec<SignerCustodyAcknowledgement>,
        bridge_pause_blockers: Vec<BridgePauseBlocker>,
        command_replay_receipts: Vec<CommandReplayReceipt>,
        release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
    ) -> Self {
        let phase_roots = build_phase_roots(
            &config,
            &source_checklist_roots,
            &reserve_replay_checks,
            &release_cap_hold_roots,
            &signer_custody_acknowledgements,
            &bridge_pause_blockers,
            &command_replay_receipts,
            &release_captain_signoffs,
        );
        let mut state = Self {
            config,
            source_checklist_roots,
            reserve_replay_checks,
            release_cap_hold_roots,
            signer_custody_acknowledgements,
            bridge_pause_blockers,
            command_replay_receipts,
            release_captain_signoffs,
            phase_roots,
            blockers: Vec::new(),
            verdict: ReleaseVerdict {
                replay_drill_id: String::new(),
                reserve_replay_count: 0,
                release_cap_hold_count: 0,
                signer_acknowledgement_count: 0,
                signer_acknowledgement_weight: 0,
                bridge_pause_blocker_count: 0,
                command_replay_receipt_count: 0,
                captain_signoff_weight: 0,
                evidence_root: String::new(),
                blocker_root: String::new(),
                release_allowed: false,
                fail_closed: true,
                accepted: false,
                verdict_root: String::new(),
            },
        };
        let evidence_root = state.evidence_root();
        state.blockers = unique_blockers(&state.config, state.blocker_kinds(), &evidence_root);
        state.verdict = ReleaseVerdict::from_state(&state.config, &state, &state.blockers);
        state
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure(
            self.source_checklist_roots.accepted(&self.config),
            "source checklist roots are not accepted",
        )?;
        ensure(
            self.reserve_replay_count() >= self.config.min_reserve_replay_checks,
            "reserve replay check count is below threshold",
        )?;
        ensure(
            self.release_cap_hold_count() >= self.config.min_release_cap_hold_roots,
            "release cap hold root count is below threshold",
        )?;
        ensure(
            self.signer_acknowledgement_count() >= self.config.min_signer_acknowledgements,
            "signer custody acknowledgement count is below threshold",
        )?;
        ensure(
            self.signer_acknowledgement_weight() >= self.config.min_signer_acknowledgement_weight,
            "signer custody acknowledgement weight is below threshold",
        )?;
        ensure(
            self.bridge_pause_blocker_count() >= self.config.min_bridge_pause_blockers,
            "bridge pause blocker count is below threshold",
        )?;
        ensure(
            self.command_replay_receipt_count() >= self.config.min_command_replay_receipts,
            "command replay receipt count is below threshold",
        )?;
        ensure(
            self.captain_signoff_weight() >= self.config.min_captain_signoff_weight,
            "release captain signoff weight is below threshold",
        )?;
        ensure(
            !self.verdict.release_allowed,
            "bridge release verdict must stay closed during replay drill",
        )?;
        ensure(
            self.verdict.fail_closed,
            "fail-closed bridge release verdict is missing",
        )?;
        ensure(
            self.blockers.is_empty(),
            "release captain replay drill is blocked",
        )?;
        Ok(())
    }

    pub fn reserve_replay_count(&self) -> u64 {
        self.reserve_replay_checks
            .iter()
            .filter(|check| check.accepted(&self.config))
            .count() as u64
    }

    pub fn release_cap_hold_count(&self) -> u64 {
        self.release_cap_hold_roots
            .iter()
            .filter(|hold| hold.accepted(&self.config))
            .count() as u64
    }

    pub fn signer_acknowledgement_count(&self) -> u64 {
        self.signer_custody_acknowledgements
            .iter()
            .filter(|ack| ack.accepted())
            .count() as u64
    }

    pub fn signer_acknowledgement_weight(&self) -> u64 {
        self.signer_custody_acknowledgements
            .iter()
            .filter(|ack| ack.accepted())
            .map(|ack| ack.signer_weight)
            .sum()
    }

    pub fn bridge_pause_blocker_count(&self) -> u64 {
        self.bridge_pause_blockers
            .iter()
            .filter(|blocker| blocker.accepted())
            .count() as u64
    }

    pub fn command_replay_receipt_count(&self) -> u64 {
        self.command_replay_receipts
            .iter()
            .filter(|receipt| receipt.accepted())
            .count() as u64
    }

    pub fn captain_signoff_weight(&self) -> u64 {
        self.release_captain_signoffs
            .iter()
            .filter(|signoff| signoff.accepted())
            .map(|signoff| signoff.signoff_weight)
            .sum()
    }

    pub fn captain_go_weight(&self) -> u64 {
        self.release_captain_signoffs
            .iter()
            .filter(|signoff| {
                signoff.status.accepted()
                    && matches!(signoff.decision, ReplayDecision::GoAfterHoldRoots)
            })
            .map(|signoff| signoff.signoff_weight)
            .sum()
    }

    pub fn evidence_root(&self) -> String {
        merkle_root(
            "RELEASE-CAPTAIN-REPLAY-EVIDENCE-ROOT",
            &[
                json!({"config_root": self.config.state_root()}),
                json!({"source_checklist_root": self.source_checklist_roots.state_root()}),
                json!({"reserve_replay_root": list_root("RELEASE-CAPTAIN-REPLAY-RESERVE-ROOT", self.reserve_replay_checks.iter().map(ReserveLiabilityReplayCheck::state_root))}),
                json!({"release_cap_hold_root": list_root("RELEASE-CAPTAIN-REPLAY-CAP-HOLD-ROOT", self.release_cap_hold_roots.iter().map(ReleaseCapHoldRoot::state_root))}),
                json!({"signer_ack_root": list_root("RELEASE-CAPTAIN-REPLAY-SIGNER-ACK-ROOT", self.signer_custody_acknowledgements.iter().map(SignerCustodyAcknowledgement::state_root))}),
                json!({"bridge_pause_root": list_root("RELEASE-CAPTAIN-REPLAY-PAUSE-ROOT", self.bridge_pause_blockers.iter().map(BridgePauseBlocker::state_root))}),
                json!({"command_replay_root": list_root("RELEASE-CAPTAIN-REPLAY-COMMAND-ROOT", self.command_replay_receipts.iter().map(CommandReplayReceipt::state_root))}),
                json!({"captain_signoff_root": list_root("RELEASE-CAPTAIN-REPLAY-CAPTAIN-ROOT", self.release_captain_signoffs.iter().map(ReleaseCaptainSignoff::state_root))}),
            ],
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "source_checklist_roots": self.source_checklist_roots.public_record(),
            "reserve_replay_checks": self.reserve_replay_checks.iter().map(ReserveLiabilityReplayCheck::public_record).collect::<Vec<_>>(),
            "release_cap_hold_roots": self.release_cap_hold_roots.iter().map(ReleaseCapHoldRoot::public_record).collect::<Vec<_>>(),
            "signer_custody_acknowledgements": self.signer_custody_acknowledgements.iter().map(SignerCustodyAcknowledgement::public_record).collect::<Vec<_>>(),
            "bridge_pause_blockers": self.bridge_pause_blockers.iter().map(BridgePauseBlocker::public_record).collect::<Vec<_>>(),
            "command_replay_receipts": self.command_replay_receipts.iter().map(CommandReplayReceipt::public_record).collect::<Vec<_>>(),
            "release_captain_signoffs": self.release_captain_signoffs.iter().map(ReleaseCaptainSignoff::public_record).collect::<Vec<_>>(),
            "phase_roots": self.phase_roots,
            "evidence_root": self.evidence_root(),
            "blockers": self.blockers.iter().map(ReplayBlocker::public_record).collect::<Vec<_>>(),
            "verdict": self.verdict.public_record(),
            "state_root": self.state_root(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "RELEASE-CAPTAIN-REPLAY-STATE",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.replay_drill_id),
                HashPart::Str(&self.evidence_root()),
                HashPart::Str(&self.verdict.blocker_root),
                HashPart::Str(&self.verdict.verdict_root),
                HashPart::Json(&json!({
                    "config_root": self.config.state_root(),
                    "phase_roots": self.phase_roots,
                })),
            ],
            32,
        )
    }

    fn blocker_kinds(&self) -> Vec<(ReplayBlockerKind, String)> {
        let mut blockers = Vec::new();
        if !self.source_checklist_roots.accepted(&self.config) {
            if self.source_checklist_roots.checklist_root.trim().is_empty() {
                blockers.push((
                    ReplayBlockerKind::SourceChecklistRootMissing,
                    "source_checklist_roots".to_string(),
                ));
            }
            if self
                .config
                .replay_height
                .saturating_sub(self.source_checklist_roots.observed_height)
                > self.config.max_checklist_age_blocks
            {
                blockers.push((
                    ReplayBlockerKind::SourceChecklistStale,
                    "source_checklist_roots".to_string(),
                ));
            }
            if self.config.require_source_checklist_fail_closed
                && !self.source_checklist_roots.fail_closed
            {
                blockers.push((
                    ReplayBlockerKind::SourceChecklistNotFailClosed,
                    "source_checklist_roots".to_string(),
                ));
            }
        }
        if self.reserve_replay_count() < self.config.min_reserve_replay_checks
            || self
                .reserve_replay_checks
                .iter()
                .any(|check| check.replay_root.trim().is_empty())
        {
            blockers.push((
                ReplayBlockerKind::ReserveReplayRootMissing,
                "reserve_replay_checks".to_string(),
            ));
        }
        if self.reserve_replay_checks.iter().any(|check| {
            !check.balance_matches_liability || check.liability_delta_atomic_units != 0
        }) {
            blockers.push((
                ReplayBlockerKind::ReserveLiabilityMismatch,
                "reserve_replay_checks".to_string(),
            ));
        }
        if self.release_cap_hold_count() < self.config.min_release_cap_hold_roots {
            blockers.push((
                ReplayBlockerKind::ReleaseCapHoldRootMissing,
                "release_cap_hold_roots".to_string(),
            ));
        }
        if self.release_cap_hold_roots.iter().any(|hold| {
            !hold.hold_asserted || hold.consumed_atomic_units > hold.cap_limit_atomic_units
        }) {
            blockers.push((
                ReplayBlockerKind::ReleaseCapNotHeld,
                "release_cap_hold_roots".to_string(),
            ));
        }
        if self.config.require_signer_custody_acknowledgements
            && self.signer_acknowledgement_count() < self.config.min_signer_acknowledgements
        {
            blockers.push((
                ReplayBlockerKind::SignerCustodyAckQuorumLow,
                "signer_custody_acknowledgements".to_string(),
            ));
        }
        if self.config.require_signer_custody_acknowledgements
            && self.signer_acknowledgement_weight() < self.config.min_signer_acknowledgement_weight
        {
            blockers.push((
                ReplayBlockerKind::SignerCustodyAckWeightLow,
                "signer_custody_acknowledgements".to_string(),
            ));
        }
        if self.config.require_bridge_pause_blockers
            && self.bridge_pause_blocker_count() < self.config.min_bridge_pause_blockers
        {
            blockers.push((
                ReplayBlockerKind::BridgePauseBlockerCountLow,
                "bridge_pause_blockers".to_string(),
            ));
        }
        if self
            .bridge_pause_blockers
            .iter()
            .any(|blocker| !blocker.bridge_pause_asserted || !blocker.release_blocking)
        {
            blockers.push((
                ReplayBlockerKind::BridgePauseNotAsserted,
                "bridge_pause_blockers".to_string(),
            ));
        }
        if self.config.require_command_replay_receipts
            && self.command_replay_receipt_count() < self.config.min_command_replay_receipts
        {
            blockers.push((
                ReplayBlockerKind::CommandReplayReceiptMissing,
                "command_replay_receipts".to_string(),
            ));
        }
        if self
            .command_replay_receipts
            .iter()
            .any(|receipt| !receipt.accepted())
        {
            blockers.push((
                ReplayBlockerKind::CommandReplayReceiptRejected,
                "command_replay_receipts".to_string(),
            ));
        }
        if self.config.require_release_captain_signoff
            && self.captain_signoff_weight() < self.config.min_captain_signoff_weight
        {
            blockers.push((
                ReplayBlockerKind::CaptainSignoffWeightLow,
                "release_captain_signoffs".to_string(),
            ));
        }
        if self
            .release_captain_signoffs
            .iter()
            .any(|signoff| !signoff.decision.release_blocking() || signoff.bridge_release_allowed)
        {
            blockers.push((
                ReplayBlockerKind::CaptainNoGo,
                "release_captain_signoffs".to_string(),
            ));
        }
        if self.config.require_fail_closed_verdict && !self.config.fail_closed {
            blockers.push((
                ReplayBlockerKind::FailClosedVerdictMissing,
                "config".to_string(),
            ));
        }
        blockers
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

fn build_phase_roots(
    config: &Config,
    source: &Wave87ChecklistRoots,
    reserve_checks: &[ReserveLiabilityReplayCheck],
    cap_holds: &[ReleaseCapHoldRoot],
    signer_acks: &[SignerCustodyAcknowledgement],
    pause_blockers: &[BridgePauseBlocker],
    command_receipts: &[CommandReplayReceipt],
    captain_signoffs: &[ReleaseCaptainSignoff],
) -> BTreeMap<String, String> {
    let mut roots = BTreeMap::new();
    roots.insert(
        ReplayPhase::Wave87ChecklistBound.as_str().to_string(),
        source.state_root(),
    );
    roots.insert(
        ReplayPhase::ReserveLiabilityReplay.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-RESERVE",
            reserve_checks
                .iter()
                .map(ReserveLiabilityReplayCheck::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::ReleaseCapHold.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-CAP-HOLD",
            cap_holds.iter().map(ReleaseCapHoldRoot::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::SignerCustodyAck.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-SIGNER-ACK",
            signer_acks
                .iter()
                .map(SignerCustodyAcknowledgement::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::BridgePauseBlocked.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-PAUSE",
            pause_blockers.iter().map(BridgePauseBlocker::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::CommandReceiptReplay.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-COMMAND",
            command_receipts
                .iter()
                .map(CommandReplayReceipt::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::ReleaseCaptainSignoff.as_str().to_string(),
        list_root(
            "RELEASE-CAPTAIN-REPLAY-PHASE-CAPTAIN",
            captain_signoffs
                .iter()
                .map(ReleaseCaptainSignoff::state_root),
        ),
    );
    roots.insert(
        ReplayPhase::FailClosedVerdict.as_str().to_string(),
        domain_hash(
            "RELEASE-CAPTAIN-REPLAY-PHASE-FAIL-CLOSED",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.replay_drill_id),
                HashPart::Str(if config.fail_closed {
                    "fail-closed"
                } else {
                    "open"
                }),
                HashPart::U64(
                    pause_blockers
                        .iter()
                        .filter(|blocker| blocker.accepted())
                        .count() as u64,
                ),
            ],
            32,
        ),
    );
    roots
}

fn unique_blockers(
    config: &Config,
    blockers: Vec<(ReplayBlockerKind, String)>,
    evidence_root: &str,
) -> Vec<ReplayBlocker> {
    let mut seen = BTreeSet::new();
    blockers
        .into_iter()
        .filter(|(kind, subject)| seen.insert((*kind, subject.clone())))
        .map(|(kind, subject)| ReplayBlocker::new(config, kind, &subject, evidence_root))
        .collect()
}

fn runtime_id(label: &str) -> String {
    domain_hash(
        "RELEASE-CAPTAIN-REPLAY-ID",
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Str(label)],
        16,
    )
}

fn evidence_id(config: &Config, kind: &str, subject: &str, ordinal: u64) -> String {
    domain_hash(
        "RELEASE-CAPTAIN-REPLAY-EVIDENCE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_drill_id),
            HashPart::Str(kind),
            HashPart::Str(subject),
            HashPart::U64(ordinal),
        ],
        16,
    )
}

fn component_root(config: &Config, kind: &str, subject: &str, ordinal: u64) -> String {
    domain_hash(
        "RELEASE-CAPTAIN-REPLAY-COMPONENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_drill_id),
            HashPart::Str(kind),
            HashPart::Str(subject),
            HashPart::U64(config.replay_height),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn record_root(domain: &str, record: &Value) -> String {
    domain_hash(
        domain,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}

fn list_root<I>(domain: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots
        .into_iter()
        .map(|root| json!({"root": root}))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{field} must be non-empty"),
    )
}
