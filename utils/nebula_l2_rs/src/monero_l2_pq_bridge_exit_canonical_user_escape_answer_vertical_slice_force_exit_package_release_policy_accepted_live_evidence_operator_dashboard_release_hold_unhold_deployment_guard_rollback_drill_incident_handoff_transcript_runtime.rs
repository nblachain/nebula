use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReleasePolicyAcceptedLiveEvidenceOperatorDashboardReleaseHoldUnholdDeploymentGuardRollbackDrillIncidentHandoffTranscriptRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_HOLD_UNHOLD_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-canonical-force-exit-release-policy-live-evidence-dashboard-release-hold-unhold-deployment-guard-rollback-drill-incident-handoff-transcript-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_HOLD_UNHOLD_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const HANDOFF_SUITE: &str = "monero-l2-pq-force-exit-rollback-drill-incident-handoff-v1";
pub const DEFAULT_RELEASE_EPOCH: u64 = 86;
pub const DEFAULT_ROLLBACK_DRILL_EPOCH: u64 = 85;
pub const DEFAULT_HANDOFF_HEIGHT: u64 = 860_000;
pub const DEFAULT_MAX_HANDOFF_AGE_BLOCKS: u64 = 72;
pub const DEFAULT_MIN_OPERATOR_WEIGHT: u64 = 90;
pub const DEFAULT_MIN_LANE_RECEIPTS: u16 = 6;
pub const DEFAULT_MIN_ACCEPTED_LANES: u16 = 6;
pub const DEFAULT_MIN_BRIDGE_CUSTODY_SIGNOFFS: u16 = 3;
pub const DEFAULT_MIN_PRIVACY_SIGNOFFS: u16 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffLane {
    CompileRuntime,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl HandoffLane {
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

    pub fn command_owner(self) -> &'static str {
        match self {
            Self::CompileRuntime => "runtime-release-lead",
            Self::RuntimeReplay => "replay-incident-lead",
            Self::AuditSecurity => "security-incident-lead",
            Self::BridgeCustody => "bridge-custody-lead",
            Self::WalletWatchtower => "wallet-watchtower-lead",
            Self::PqReservePrivacy => "pq-reserve-privacy-lead",
        }
    }

    pub fn source_runtime_suffix(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile-runtime-rollback-drill",
            Self::RuntimeReplay => "runtime-replay-rollback-drill",
            Self::AuditSecurity => "audit-security-rollback-drill",
            Self::BridgeCustody => "bridge-custody-rollback-drill",
            Self::WalletWatchtower => "wallet-watchtower-rollback-drill",
            Self::PqReservePrivacy => "pq-reserve-privacy-rollback-drill",
        }
    }

    pub fn requires_bridge_custody(self) -> bool {
        matches!(self, Self::BridgeCustody | Self::PqReservePrivacy)
    }

    pub fn requires_privacy_review(self) -> bool {
        matches!(
            self,
            Self::AuditSecurity | Self::WalletWatchtower | Self::PqReservePrivacy
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffStatus {
    Missing,
    Draft,
    Held,
    AcceptedWithHold,
    ReadyForUnhold,
    Rejected,
    Expired,
}

impl HandoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Draft => "draft",
            Self::Held => "held",
            Self::AcceptedWithHold => "accepted_with_hold",
            Self::ReadyForUnhold => "ready_for_unhold",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::AcceptedWithHold | Self::ReadyForUnhold)
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::ReadyForUnhold)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDecision {
    AcknowledgeHold,
    AcceptCustody,
    AcceptSecurityHold,
    RequestMoreEvidence,
    RejectUnhold,
    ApproveUnholdAfterHeavyGates,
}

impl HandoffDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcknowledgeHold => "acknowledge_hold",
            Self::AcceptCustody => "accept_custody",
            Self::AcceptSecurityHold => "accept_security_hold",
            Self::RequestMoreEvidence => "request_more_evidence",
            Self::RejectUnhold => "reject_unhold",
            Self::ApproveUnholdAfterHeavyGates => "approve_unhold_after_heavy_gates",
        }
    }

    pub fn contributes_weight(self) -> bool {
        matches!(
            self,
            Self::AcknowledgeHold
                | Self::AcceptCustody
                | Self::AcceptSecurityHold
                | Self::ApproveUnholdAfterHeavyGates
        )
    }

    pub fn blocks_unhold(self) -> bool {
        !matches!(self, Self::ApproveUnholdAfterHeavyGates)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffArtifactKind {
    RollbackDrillRoot,
    IncidentRoomTranscript,
    OperatorPagerAck,
    ReleaseHoldReceipt,
    AbortCommandBundle,
    BridgeCustodyLedgerRoot,
    WatchtowerChallengeRoot,
    PrivacyLeakBudgetRoot,
    PqSignerPolicyRoot,
    DeferredHeavyGateRoot,
}

impl HandoffArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RollbackDrillRoot => "rollback_drill_root",
            Self::IncidentRoomTranscript => "incident_room_transcript",
            Self::OperatorPagerAck => "operator_pager_ack",
            Self::ReleaseHoldReceipt => "release_hold_receipt",
            Self::AbortCommandBundle => "abort_command_bundle",
            Self::BridgeCustodyLedgerRoot => "bridge_custody_ledger_root",
            Self::WatchtowerChallengeRoot => "watchtower_challenge_root",
            Self::PrivacyLeakBudgetRoot => "privacy_leak_budget_root",
            Self::PqSignerPolicyRoot => "pq_signer_policy_root",
            Self::DeferredHeavyGateRoot => "deferred_heavy_gate_root",
        }
    }

    pub fn required_for_lane(self, lane: HandoffLane) -> bool {
        match self {
            Self::RollbackDrillRoot
            | Self::IncidentRoomTranscript
            | Self::OperatorPagerAck
            | Self::ReleaseHoldReceipt
            | Self::AbortCommandBundle => true,
            Self::BridgeCustodyLedgerRoot => lane.requires_bridge_custody(),
            Self::WatchtowerChallengeRoot => matches!(lane, HandoffLane::WalletWatchtower),
            Self::PrivacyLeakBudgetRoot => lane.requires_privacy_review(),
            Self::PqSignerPolicyRoot => matches!(lane, HandoffLane::PqReservePrivacy),
            Self::DeferredHeavyGateRoot => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBlockerKind {
    MissingLane,
    DuplicateLane,
    EmptyRoot,
    StaleHandoff,
    MissingRollbackDrillRoot,
    MissingIncidentRoomTranscript,
    MissingOperatorPagerAck,
    MissingReleaseHoldReceipt,
    MissingAbortCommandBundle,
    MissingBridgeCustodyLedgerRoot,
    MissingWatchtowerChallengeRoot,
    MissingPrivacyLeakBudgetRoot,
    MissingPqSignerPolicyRoot,
    DeferredHeavyGateRoot,
    LaneReceiptCountTooLow,
    AcceptedLaneCountTooLow,
    OperatorWeightTooLow,
    BridgeCustodySignoffTooLow,
    PrivacySignoffTooLow,
    OperatorRejectedUnhold,
    OperatorRequestedMoreEvidence,
    ReleaseHoldStillActive,
    FailClosedRequired,
}

impl HandoffBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingLane => "missing_lane",
            Self::DuplicateLane => "duplicate_lane",
            Self::EmptyRoot => "empty_root",
            Self::StaleHandoff => "stale_handoff",
            Self::MissingRollbackDrillRoot => "missing_rollback_drill_root",
            Self::MissingIncidentRoomTranscript => "missing_incident_room_transcript",
            Self::MissingOperatorPagerAck => "missing_operator_pager_ack",
            Self::MissingReleaseHoldReceipt => "missing_release_hold_receipt",
            Self::MissingAbortCommandBundle => "missing_abort_command_bundle",
            Self::MissingBridgeCustodyLedgerRoot => "missing_bridge_custody_ledger_root",
            Self::MissingWatchtowerChallengeRoot => "missing_watchtower_challenge_root",
            Self::MissingPrivacyLeakBudgetRoot => "missing_privacy_leak_budget_root",
            Self::MissingPqSignerPolicyRoot => "missing_pq_signer_policy_root",
            Self::DeferredHeavyGateRoot => "deferred_heavy_gate_root",
            Self::LaneReceiptCountTooLow => "lane_receipt_count_too_low",
            Self::AcceptedLaneCountTooLow => "accepted_lane_count_too_low",
            Self::OperatorWeightTooLow => "operator_weight_too_low",
            Self::BridgeCustodySignoffTooLow => "bridge_custody_signoff_too_low",
            Self::PrivacySignoffTooLow => "privacy_signoff_too_low",
            Self::OperatorRejectedUnhold => "operator_rejected_unhold",
            Self::OperatorRequestedMoreEvidence => "operator_requested_more_evidence",
            Self::ReleaseHoldStillActive => "release_hold_still_active",
            Self::FailClosedRequired => "fail_closed_required",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::DeferredHeavyGateRoot
            | Self::ReleaseHoldStillActive
            | Self::FailClosedRequired => 2,
            Self::StaleHandoff
            | Self::OperatorWeightTooLow
            | Self::BridgeCustodySignoffTooLow
            | Self::PrivacySignoffTooLow
            | Self::OperatorRejectedUnhold
            | Self::OperatorRequestedMoreEvidence => 3,
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
    pub handoff_suite: String,
    pub release_epoch: u64,
    pub rollback_drill_epoch: u64,
    pub handoff_height: u64,
    pub release_channel: String,
    pub incident_room_id: String,
    pub command_policy_id: String,
    pub max_handoff_age_blocks: u64,
    pub min_operator_weight: u64,
    pub min_lane_receipts: u16,
    pub min_accepted_lanes: u16,
    pub min_bridge_custody_signoffs: u16,
    pub min_privacy_signoffs: u16,
    pub required_lanes: Vec<HandoffLane>,
    pub require_release_hold_root: bool,
    pub require_abort_command_root: bool,
    pub require_bridge_custody_handoff: bool,
    pub require_privacy_handoff: bool,
    pub require_pq_signer_policy_root: bool,
    pub require_deferred_heavy_gate_root: bool,
    pub require_fail_closed_default: bool,
    pub allow_unhold_without_heavy_gates: bool,
}

impl Default for Config {
    fn default() -> Self {
        let incident_room_id = stable_id("incident-room", "wave-86-handoff", 1);
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            handoff_suite: HANDOFF_SUITE.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            rollback_drill_epoch: DEFAULT_ROLLBACK_DRILL_EPOCH,
            handoff_height: DEFAULT_HANDOFF_HEIGHT,
            release_channel: "devnet-force-exit-rollback-incident-handoff".to_string(),
            incident_room_id: incident_room_id.clone(),
            command_policy_id: stable_id("command-policy", &incident_room_id, 1),
            max_handoff_age_blocks: DEFAULT_MAX_HANDOFF_AGE_BLOCKS,
            min_operator_weight: DEFAULT_MIN_OPERATOR_WEIGHT,
            min_lane_receipts: DEFAULT_MIN_LANE_RECEIPTS,
            min_accepted_lanes: DEFAULT_MIN_ACCEPTED_LANES,
            min_bridge_custody_signoffs: DEFAULT_MIN_BRIDGE_CUSTODY_SIGNOFFS,
            min_privacy_signoffs: DEFAULT_MIN_PRIVACY_SIGNOFFS,
            required_lanes: HandoffLane::all(),
            require_release_hold_root: true,
            require_abort_command_root: true,
            require_bridge_custody_handoff: true,
            require_privacy_handoff: true,
            require_pq_signer_policy_root: true,
            require_deferred_heavy_gate_root: true,
            require_fail_closed_default: true,
            allow_unhold_without_heavy_gates: false,
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
        ensure_non_empty("handoff_suite", &self.handoff_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure_non_empty("incident_room_id", &self.incident_room_id)?;
        ensure_non_empty("command_policy_id", &self.command_policy_id)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.release_epoch > 0, "release epoch must be non-zero")?;
        ensure(
            self.rollback_drill_epoch > 0,
            "rollback drill epoch must be non-zero",
        )?;
        ensure(
            self.release_epoch > self.rollback_drill_epoch,
            "handoff epoch must follow rollback drill epoch",
        )?;
        ensure(self.handoff_height > 0, "handoff height must be non-zero")?;
        ensure(
            self.max_handoff_age_blocks > 0,
            "max handoff age must be non-zero",
        )?;
        ensure(
            self.min_operator_weight > 0,
            "min operator weight must be non-zero",
        )?;
        ensure(
            self.min_lane_receipts > 0,
            "min lane receipts must be non-zero",
        )?;
        ensure(
            self.min_accepted_lanes > 0,
            "min accepted lanes must be non-zero",
        )?;
        ensure(
            !self.required_lanes.is_empty(),
            "required lane list must be non-empty",
        )?;
        let mut seen = BTreeSet::new();
        for lane in &self.required_lanes {
            ensure(seen.insert(*lane), "required lanes must be unique")?;
        }
        if self.require_fail_closed_default {
            ensure(
                !self.allow_unhold_without_heavy_gates,
                "fail-closed mode cannot allow unhold without heavy gates",
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
            "handoff_suite": self.handoff_suite,
            "release_epoch": self.release_epoch,
            "rollback_drill_epoch": self.rollback_drill_epoch,
            "handoff_height": self.handoff_height,
            "release_channel": self.release_channel,
            "incident_room_id": self.incident_room_id,
            "command_policy_id": self.command_policy_id,
            "max_handoff_age_blocks": self.max_handoff_age_blocks,
            "min_operator_weight": self.min_operator_weight,
            "min_lane_receipts": self.min_lane_receipts,
            "min_accepted_lanes": self.min_accepted_lanes,
            "min_bridge_custody_signoffs": self.min_bridge_custody_signoffs,
            "min_privacy_signoffs": self.min_privacy_signoffs,
            "required_lanes": self.required_lanes.iter().map(|lane| lane.as_str()).collect::<Vec<_>>(),
            "require_release_hold_root": self.require_release_hold_root,
            "require_abort_command_root": self.require_abort_command_root,
            "require_bridge_custody_handoff": self.require_bridge_custody_handoff,
            "require_privacy_handoff": self.require_privacy_handoff,
            "require_pq_signer_policy_root": self.require_pq_signer_policy_root,
            "require_deferred_heavy_gate_root": self.require_deferred_heavy_gate_root,
            "require_fail_closed_default": self.require_fail_closed_default,
            "allow_unhold_without_heavy_gates": self.allow_unhold_without_heavy_gates,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneIncidentHandoff {
    pub lane: HandoffLane,
    pub source_runtime: String,
    pub command_owner: String,
    pub rollback_drill_root: String,
    pub handoff_bundle_root: String,
    pub incident_room_root: String,
    pub operator_checkpoint_root: String,
    pub release_hold_root: String,
    pub abort_command_root: String,
    pub bridge_custody_root: Option<String>,
    pub watchtower_challenge_root: Option<String>,
    pub privacy_leak_budget_root: Option<String>,
    pub pq_signer_policy_root: Option<String>,
    pub deferred_heavy_gate_root: String,
    pub public_redaction_root: String,
    pub observed_at_height: u64,
    pub receipt_count: u16,
    pub accepted_receipt_count: u16,
    pub status: HandoffStatus,
    pub fail_closed: bool,
}

impl LaneIncidentHandoff {
    pub fn devnet(lane: HandoffLane, config: &Config, ordinal: u64) -> Self {
        let label = lane.as_str();
        let observed_at_height = config
            .handoff_height
            .saturating_sub(config.max_handoff_age_blocks / 3)
            .saturating_add(ordinal);
        Self {
            lane,
            source_runtime: lane.source_runtime_suffix().to_string(),
            command_owner: lane.command_owner().to_string(),
            rollback_drill_root: sample_root("rollback-drill-root", label, ordinal),
            handoff_bundle_root: sample_root("incident-handoff-bundle", label, ordinal),
            incident_room_root: sample_root("incident-room-transcript", label, ordinal),
            operator_checkpoint_root: sample_root("operator-checkpoint", label, ordinal),
            release_hold_root: sample_root("release-hold-receipt", label, ordinal),
            abort_command_root: sample_root("abort-command-bundle", label, ordinal),
            bridge_custody_root: lane
                .requires_bridge_custody()
                .then(|| sample_root("bridge-custody-ledger", label, ordinal)),
            watchtower_challenge_root: matches!(lane, HandoffLane::WalletWatchtower)
                .then(|| sample_root("watchtower-challenge-root", label, ordinal)),
            privacy_leak_budget_root: lane
                .requires_privacy_review()
                .then(|| sample_root("privacy-leak-budget", label, ordinal)),
            pq_signer_policy_root: matches!(lane, HandoffLane::PqReservePrivacy)
                .then(|| sample_root("pq-signer-policy", label, ordinal)),
            deferred_heavy_gate_root: sample_root("deferred-heavy-gate", label, ordinal),
            public_redaction_root: sample_root("public-redaction-policy", label, ordinal),
            observed_at_height,
            receipt_count: config.min_lane_receipts.saturating_add(ordinal as u16),
            accepted_receipt_count: config.min_lane_receipts,
            status: HandoffStatus::AcceptedWithHold,
            fail_closed: true,
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        ensure_non_empty("source_runtime", &self.source_runtime)?;
        ensure_non_empty("command_owner", &self.command_owner)?;
        ensure_non_empty("rollback_drill_root", &self.rollback_drill_root)?;
        ensure_non_empty("handoff_bundle_root", &self.handoff_bundle_root)?;
        ensure_non_empty("incident_room_root", &self.incident_room_root)?;
        ensure_non_empty("operator_checkpoint_root", &self.operator_checkpoint_root)?;
        ensure_non_empty("release_hold_root", &self.release_hold_root)?;
        ensure_non_empty("abort_command_root", &self.abort_command_root)?;
        ensure_non_empty("deferred_heavy_gate_root", &self.deferred_heavy_gate_root)?;
        ensure_non_empty("public_redaction_root", &self.public_redaction_root)?;
        ensure(
            self.observed_at_height <= height,
            "handoff cannot be observed after state height",
        )?;
        if self.lane.requires_bridge_custody() {
            let has_bridge_custody_root = match &self.bridge_custody_root {
                Some(root) => !root.trim().is_empty(),
                None => false,
            };
            ensure(
                has_bridge_custody_root,
                "bridge custody lane requires custody root",
            )?;
        }
        if self.lane.requires_privacy_review() {
            let has_privacy_leak_budget_root = match &self.privacy_leak_budget_root {
                Some(root) => !root.trim().is_empty(),
                None => false,
            };
            ensure(
                has_privacy_leak_budget_root,
                "privacy lane requires privacy leak budget root",
            )?;
        }
        if matches!(self.lane, HandoffLane::PqReservePrivacy) {
            let has_pq_signer_policy_root = match &self.pq_signer_policy_root {
                Some(root) => !root.trim().is_empty(),
                None => false,
            };
            ensure(
                has_pq_signer_policy_root,
                "pq reserve privacy lane requires pq signer policy root",
            )?;
        }
        ensure(
            self.receipt_count >= config.min_lane_receipts,
            "handoff receipt count below config minimum",
        )?;
        Ok(())
    }

    pub fn blockers(&self, config: &Config, height: u64) -> Vec<HandoffBlockerKind> {
        let mut blockers = Vec::new();
        if self
            .observed_at_height
            .saturating_add(config.max_handoff_age_blocks)
            < height
        {
            blockers.push(HandoffBlockerKind::StaleHandoff);
        }
        if self.rollback_drill_root.trim().is_empty() {
            blockers.push(HandoffBlockerKind::MissingRollbackDrillRoot);
        }
        if self.incident_room_root.trim().is_empty() {
            blockers.push(HandoffBlockerKind::MissingIncidentRoomTranscript);
        }
        if self.operator_checkpoint_root.trim().is_empty() {
            blockers.push(HandoffBlockerKind::MissingOperatorPagerAck);
        }
        if config.require_release_hold_root && self.release_hold_root.trim().is_empty() {
            blockers.push(HandoffBlockerKind::MissingReleaseHoldReceipt);
        }
        if config.require_abort_command_root && self.abort_command_root.trim().is_empty() {
            blockers.push(HandoffBlockerKind::MissingAbortCommandBundle);
        }
        if config.require_bridge_custody_handoff
            && self.lane.requires_bridge_custody()
            && self.bridge_custody_root.is_none()
        {
            blockers.push(HandoffBlockerKind::MissingBridgeCustodyLedgerRoot);
        }
        if matches!(self.lane, HandoffLane::WalletWatchtower)
            && self.watchtower_challenge_root.is_none()
        {
            blockers.push(HandoffBlockerKind::MissingWatchtowerChallengeRoot);
        }
        if config.require_privacy_handoff
            && self.lane.requires_privacy_review()
            && self.privacy_leak_budget_root.is_none()
        {
            blockers.push(HandoffBlockerKind::MissingPrivacyLeakBudgetRoot);
        }
        if config.require_pq_signer_policy_root
            && matches!(self.lane, HandoffLane::PqReservePrivacy)
            && self.pq_signer_policy_root.is_none()
        {
            blockers.push(HandoffBlockerKind::MissingPqSignerPolicyRoot);
        }
        if config.require_deferred_heavy_gate_root {
            blockers.push(HandoffBlockerKind::DeferredHeavyGateRoot);
        }
        if self.receipt_count < config.min_lane_receipts {
            blockers.push(HandoffBlockerKind::LaneReceiptCountTooLow);
        }
        if self.status.blocks_release() {
            blockers.push(HandoffBlockerKind::ReleaseHoldStillActive);
        }
        if config.require_fail_closed_default && self.fail_closed {
            blockers.push(HandoffBlockerKind::FailClosedRequired);
        }
        blockers
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane.as_str(),
            "source_runtime": self.source_runtime,
            "command_owner": self.command_owner,
            "rollback_drill_root": self.rollback_drill_root,
            "handoff_bundle_root": self.handoff_bundle_root,
            "incident_room_root": self.incident_room_root,
            "operator_checkpoint_root": self.operator_checkpoint_root,
            "release_hold_root": self.release_hold_root,
            "abort_command_root": self.abort_command_root,
            "bridge_custody_root": self.bridge_custody_root,
            "watchtower_challenge_root": self.watchtower_challenge_root,
            "privacy_leak_budget_root": self.privacy_leak_budget_root,
            "pq_signer_policy_root": self.pq_signer_policy_root,
            "deferred_heavy_gate_root": self.deferred_heavy_gate_root,
            "public_redaction_root": self.public_redaction_root,
            "observed_at_height": self.observed_at_height,
            "receipt_count": self.receipt_count,
            "accepted_receipt_count": self.accepted_receipt_count,
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-incident-handoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncidentHandoffArtifact {
    pub artifact_id: String,
    pub lane: HandoffLane,
    pub kind: HandoffArtifactKind,
    pub evidence_root: String,
    pub operator_id: String,
    pub observed_at_height: u64,
    pub redaction_policy_root: String,
    pub privacy_safe: bool,
    pub accepted: bool,
}

impl IncidentHandoffArtifact {
    pub fn devnet(
        lane: HandoffLane,
        kind: HandoffArtifactKind,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        let label = format!("{}-{}", lane.as_str(), kind.as_str());
        Self {
            artifact_id: stable_id("incident-handoff-artifact", &label, ordinal),
            lane,
            kind,
            evidence_root: sample_root("incident-handoff-artifact-root", &label, ordinal),
            operator_id: lane.command_owner().to_string(),
            observed_at_height: config
                .handoff_height
                .saturating_sub(16)
                .saturating_add(ordinal),
            redaction_policy_root: sample_root("artifact-redaction-policy", &label, ordinal),
            privacy_safe: true,
            accepted: true,
        }
    }

    pub fn validate(&self, height: u64) -> Result<()> {
        ensure_non_empty("artifact_id", &self.artifact_id)?;
        ensure_non_empty("evidence_root", &self.evidence_root)?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        ensure_non_empty("redaction_policy_root", &self.redaction_policy_root)?;
        ensure(
            self.observed_at_height <= height,
            "artifact cannot be observed after state height",
        )?;
        ensure(self.privacy_safe, "handoff artifact must be privacy safe")?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "artifact_id": self.artifact_id,
            "lane": self.lane.as_str(),
            "kind": self.kind.as_str(),
            "evidence_root": self.evidence_root,
            "operator_id": self.operator_id,
            "observed_at_height": self.observed_at_height,
            "redaction_policy_root": self.redaction_policy_root,
            "privacy_safe": self.privacy_safe,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("incident-handoff-artifact", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorHandoffSignoff {
    pub operator_id: String,
    pub role: String,
    pub lane_scope: Option<HandoffLane>,
    pub weight: u64,
    pub decision: HandoffDecision,
    pub signed_handoff_root: String,
    pub signed_at_height: u64,
}

impl OperatorHandoffSignoff {
    pub fn devnet(
        operator_id: &str,
        role: &str,
        lane_scope: Option<HandoffLane>,
        weight: u64,
        decision: HandoffDecision,
        config: &Config,
        ordinal: u64,
    ) -> Self {
        Self {
            operator_id: operator_id.to_string(),
            role: role.to_string(),
            lane_scope,
            weight,
            decision,
            signed_handoff_root: sample_root("operator-handoff-signoff", operator_id, ordinal),
            signed_at_height: config
                .handoff_height
                .saturating_sub(8)
                .saturating_add(ordinal),
        }
    }

    pub fn validate(&self, height: u64) -> Result<()> {
        ensure_non_empty("operator_id", &self.operator_id)?;
        ensure_non_empty("role", &self.role)?;
        ensure_non_empty("signed_handoff_root", &self.signed_handoff_root)?;
        ensure(self.weight > 0, "operator signoff weight must be non-zero")?;
        ensure(
            self.signed_at_height <= height,
            "operator signoff cannot be after state height",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "operator_id": self.operator_id,
            "role": self.role,
            "lane_scope": self.lane_scope.map(|lane| lane.as_str()),
            "weight": self.weight,
            "decision": self.decision.as_str(),
            "signed_handoff_root": self.signed_handoff_root,
            "signed_at_height": self.signed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator-handoff-signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncidentCommandSummary {
    pub state: String,
    pub accepted_lane_count: u16,
    pub deferred_heavy_gate_count: u16,
    pub blocker_count: u16,
    pub max_blocker_severity: u8,
    pub operator_weight: u64,
    pub bridge_custody_signoff_count: u16,
    pub privacy_signoff_count: u16,
    pub release_hold_active: bool,
    pub fail_closed: bool,
    pub handoff_ready: bool,
    pub summary_root: String,
}

impl IncidentCommandSummary {
    pub fn build(
        config: &Config,
        lane_handoffs: &[LaneIncidentHandoff],
        operator_signoffs: &[OperatorHandoffSignoff],
        blockers: &BTreeMap<String, Vec<HandoffBlockerKind>>,
    ) -> Self {
        let accepted_lane_count = lane_handoffs
            .iter()
            .filter(|lane| lane.status.accepted())
            .count() as u16;
        let deferred_heavy_gate_count = blockers
            .values()
            .flat_map(|items| items.iter())
            .filter(|blocker| matches!(blocker, HandoffBlockerKind::DeferredHeavyGateRoot))
            .count() as u16;
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
        let operator_weight = operator_signoffs
            .iter()
            .filter(|signoff| signoff.decision.contributes_weight())
            .map(|signoff| signoff.weight)
            .sum::<u64>();
        let bridge_custody_signoff_count = operator_signoffs
            .iter()
            .filter(|signoff| match signoff.lane_scope {
                Some(lane) => lane.requires_bridge_custody(),
                None => false,
            })
            .count() as u16;
        let privacy_signoff_count = operator_signoffs
            .iter()
            .filter(|signoff| match signoff.lane_scope {
                Some(lane) => lane.requires_privacy_review(),
                None => false,
            })
            .count() as u16;
        let release_hold_active = lane_handoffs
            .iter()
            .any(|lane| lane.status.blocks_release());
        let fail_closed = config.require_fail_closed_default || release_hold_active;
        let handoff_ready = blocker_count == 0
            && accepted_lane_count >= config.min_accepted_lanes
            && operator_weight >= config.min_operator_weight
            && !release_hold_active;
        let state = if handoff_ready {
            "ready_for_unhold"
        } else if fail_closed {
            "held_fail_closed"
        } else {
            "pending_review"
        }
        .to_string();
        let root = domain_hash(
            "incident-handoff-summary-root",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.command_policy_id),
                HashPart::U64(accepted_lane_count as u64),
                HashPart::U64(deferred_heavy_gate_count as u64),
                HashPart::U64(blocker_count as u64),
                HashPart::U64(operator_weight),
                HashPart::Json(&json!({
                    "release_hold_active": release_hold_active,
                    "fail_closed": fail_closed,
                    "handoff_ready": handoff_ready,
                })),
            ],
            32,
        );
        Self {
            state,
            accepted_lane_count,
            deferred_heavy_gate_count,
            blocker_count,
            max_blocker_severity,
            operator_weight,
            bridge_custody_signoff_count,
            privacy_signoff_count,
            release_hold_active,
            fail_closed,
            handoff_ready,
            summary_root: root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "state": self.state,
            "accepted_lane_count": self.accepted_lane_count,
            "deferred_heavy_gate_count": self.deferred_heavy_gate_count,
            "blocker_count": self.blocker_count,
            "max_blocker_severity": self.max_blocker_severity,
            "operator_weight": self.operator_weight,
            "bridge_custody_signoff_count": self.bridge_custody_signoff_count,
            "privacy_signoff_count": self.privacy_signoff_count,
            "release_hold_active": self.release_hold_active,
            "fail_closed": self.fail_closed,
            "handoff_ready": self.handoff_ready,
            "summary_root": self.summary_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub lane_handoffs: Vec<LaneIncidentHandoff>,
    pub artifacts: Vec<IncidentHandoffArtifact>,
    pub operator_signoffs: Vec<OperatorHandoffSignoff>,
    pub blockers: BTreeMap<String, Vec<HandoffBlockerKind>>,
    pub lane_handoff_root: String,
    pub artifact_root: String,
    pub operator_root: String,
    pub blocker_root: String,
    pub summary: IncidentCommandSummary,
}

impl State {
    pub fn new(
        config: Config,
        height: u64,
        lane_handoffs: Vec<LaneIncidentHandoff>,
        artifacts: Vec<IncidentHandoffArtifact>,
        operator_signoffs: Vec<OperatorHandoffSignoff>,
    ) -> Result<Self> {
        config.validate()?;
        ensure(height > 0, "state height must be non-zero")?;
        ensure(
            !lane_handoffs.is_empty(),
            "state must include lane handoffs",
        )?;
        ensure(!artifacts.is_empty(), "state must include artifacts")?;
        ensure(
            !operator_signoffs.is_empty(),
            "state must include operator signoffs",
        )?;
        for handoff in &lane_handoffs {
            handoff.validate(&config, height)?;
        }
        for artifact in &artifacts {
            artifact.validate(height)?;
        }
        for signoff in &operator_signoffs {
            signoff.validate(height)?;
        }
        let blockers = evaluate_blockers(&config, height, &lane_handoffs, &operator_signoffs);
        let lane_handoff_root = roots_root(
            "incident-handoff-lanes",
            lane_handoffs.iter().map(LaneIncidentHandoff::state_root),
        );
        let artifact_root = roots_root(
            "incident-handoff-artifacts",
            artifacts.iter().map(IncidentHandoffArtifact::state_root),
        );
        let operator_root = roots_root(
            "incident-handoff-operator-signoffs",
            operator_signoffs
                .iter()
                .map(OperatorHandoffSignoff::state_root),
        );
        let blocker_root = blockers_root(&blockers);
        let summary =
            IncidentCommandSummary::build(&config, &lane_handoffs, &operator_signoffs, &blockers);
        Ok(Self {
            config,
            height,
            lane_handoffs,
            artifacts,
            operator_signoffs,
            blockers,
            lane_handoff_root,
            artifact_root,
            operator_root,
            blocker_root,
            summary,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let height = config.handoff_height;
        let lane_handoffs = HandoffLane::all()
            .into_iter()
            .enumerate()
            .map(|(index, lane)| LaneIncidentHandoff::devnet(lane, &config, one_based(index)))
            .collect::<Vec<_>>();
        let mut artifacts = Vec::new();
        for (lane_index, lane) in HandoffLane::all().into_iter().enumerate() {
            let ordinal_base = one_based(lane_index) * 10;
            let kinds = [
                HandoffArtifactKind::RollbackDrillRoot,
                HandoffArtifactKind::IncidentRoomTranscript,
                HandoffArtifactKind::OperatorPagerAck,
                HandoffArtifactKind::ReleaseHoldReceipt,
                HandoffArtifactKind::AbortCommandBundle,
                HandoffArtifactKind::DeferredHeavyGateRoot,
                HandoffArtifactKind::BridgeCustodyLedgerRoot,
                HandoffArtifactKind::WatchtowerChallengeRoot,
                HandoffArtifactKind::PrivacyLeakBudgetRoot,
                HandoffArtifactKind::PqSignerPolicyRoot,
            ];
            for (kind_index, kind) in kinds.into_iter().enumerate() {
                if kind.required_for_lane(lane) {
                    artifacts.push(IncidentHandoffArtifact::devnet(
                        lane,
                        kind,
                        &config,
                        ordinal_base + one_based(kind_index),
                    ));
                }
            }
        }
        let operator_signoffs = vec![
            OperatorHandoffSignoff::devnet(
                "runtime-release-lead",
                "compile-runtime-owner",
                Some(HandoffLane::CompileRuntime),
                20,
                HandoffDecision::AcknowledgeHold,
                &config,
                1,
            ),
            OperatorHandoffSignoff::devnet(
                "replay-incident-lead",
                "runtime-replay-owner",
                Some(HandoffLane::RuntimeReplay),
                15,
                HandoffDecision::AcknowledgeHold,
                &config,
                2,
            ),
            OperatorHandoffSignoff::devnet(
                "security-incident-lead",
                "audit-security-owner",
                Some(HandoffLane::AuditSecurity),
                20,
                HandoffDecision::AcceptSecurityHold,
                &config,
                3,
            ),
            OperatorHandoffSignoff::devnet(
                "bridge-custody-lead",
                "bridge-custody-owner",
                Some(HandoffLane::BridgeCustody),
                20,
                HandoffDecision::AcceptCustody,
                &config,
                4,
            ),
            OperatorHandoffSignoff::devnet(
                "wallet-watchtower-lead",
                "wallet-watchtower-owner",
                Some(HandoffLane::WalletWatchtower),
                15,
                HandoffDecision::AcknowledgeHold,
                &config,
                5,
            ),
            OperatorHandoffSignoff::devnet(
                "pq-reserve-privacy-lead",
                "pq-reserve-privacy-owner",
                Some(HandoffLane::PqReservePrivacy),
                20,
                HandoffDecision::AcceptSecurityHold,
                &config,
                6,
            ),
            OperatorHandoffSignoff::devnet(
                "release-captain",
                "incident-command",
                None,
                25,
                HandoffDecision::RequestMoreEvidence,
                &config,
                7,
            ),
        ];
        match Self::new(config, height, lane_handoffs, artifacts, operator_signoffs) {
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
            "lane_handoff_root": self.lane_handoff_root,
            "artifact_root": self.artifact_root,
            "operator_root": self.operator_root,
            "blocker_root": self.blocker_root,
            "summary": self.summary.public_record(),
            "lane_handoffs": self.lane_handoffs.iter().map(LaneIncidentHandoff::public_record).collect::<Vec<_>>(),
            "artifacts": self.artifacts.iter().map(IncidentHandoffArtifact::public_record).collect::<Vec<_>>(),
            "operator_signoffs": self.operator_signoffs.iter().map(OperatorHandoffSignoff::public_record).collect::<Vec<_>>(),
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
            "incident-handoff-state-root",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.command_policy_id),
                HashPart::U64(self.height),
                HashPart::Str(&self.lane_handoff_root),
                HashPart::Str(&self.artifact_root),
                HashPart::Str(&self.operator_root),
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
    lane_handoffs: &[LaneIncidentHandoff],
    operator_signoffs: &[OperatorHandoffSignoff],
) -> BTreeMap<String, Vec<HandoffBlockerKind>> {
    let mut blockers = BTreeMap::<String, Vec<HandoffBlockerKind>>::new();
    let mut seen = BTreeSet::new();
    for handoff in lane_handoffs {
        let key = handoff.lane.as_str().to_string();
        if !seen.insert(handoff.lane) {
            blockers
                .entry(key.clone())
                .or_default()
                .push(HandoffBlockerKind::DuplicateLane);
        }
        let lane_blockers = handoff.blockers(config, height);
        if !lane_blockers.is_empty() {
            blockers.entry(key).or_default().extend(lane_blockers);
        }
    }
    for lane in &config.required_lanes {
        if !seen.contains(lane) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(HandoffBlockerKind::MissingLane);
        }
    }
    let accepted_lane_count = lane_handoffs
        .iter()
        .filter(|handoff| handoff.status.accepted())
        .count() as u16;
    if accepted_lane_count < config.min_accepted_lanes {
        blockers
            .entry("lane_quorum".to_string())
            .or_default()
            .push(HandoffBlockerKind::AcceptedLaneCountTooLow);
    }
    let operator_weight = operator_signoffs
        .iter()
        .filter(|signoff| signoff.decision.contributes_weight())
        .map(|signoff| signoff.weight)
        .sum::<u64>();
    if operator_weight < config.min_operator_weight {
        blockers
            .entry("operator_quorum".to_string())
            .or_default()
            .push(HandoffBlockerKind::OperatorWeightTooLow);
    }
    let bridge_custody_signoff_count = operator_signoffs
        .iter()
        .filter(|signoff| match signoff.lane_scope {
            Some(lane) => lane.requires_bridge_custody(),
            None => false,
        })
        .count() as u16;
    if bridge_custody_signoff_count < config.min_bridge_custody_signoffs {
        blockers
            .entry("bridge_custody_quorum".to_string())
            .or_default()
            .push(HandoffBlockerKind::BridgeCustodySignoffTooLow);
    }
    let privacy_signoff_count = operator_signoffs
        .iter()
        .filter(|signoff| match signoff.lane_scope {
            Some(lane) => lane.requires_privacy_review(),
            None => false,
        })
        .count() as u16;
    if privacy_signoff_count < config.min_privacy_signoffs {
        blockers
            .entry("privacy_quorum".to_string())
            .or_default()
            .push(HandoffBlockerKind::PrivacySignoffTooLow);
    }
    for signoff in operator_signoffs {
        if signoff.decision.blocks_unhold() {
            let blocker = match signoff.decision {
                HandoffDecision::RejectUnhold => HandoffBlockerKind::OperatorRejectedUnhold,
                HandoffDecision::RequestMoreEvidence => {
                    HandoffBlockerKind::OperatorRequestedMoreEvidence
                }
                HandoffDecision::AcknowledgeHold
                | HandoffDecision::AcceptCustody
                | HandoffDecision::AcceptSecurityHold
                | HandoffDecision::ApproveUnholdAfterHeavyGates => {
                    HandoffBlockerKind::ReleaseHoldStillActive
                }
            };
            blockers
                .entry(signoff.operator_id.clone())
                .or_default()
                .push(blocker);
        }
    }
    blockers
}

fn build_devnet_fail_closed_fallback(reason: String) -> State {
    let config = Config::devnet();
    let height = config.handoff_height;
    let lane_handoffs = HandoffLane::all()
        .into_iter()
        .enumerate()
        .map(|(index, lane)| LaneIncidentHandoff {
            status: HandoffStatus::Held,
            fail_closed: true,
            receipt_count: 0,
            accepted_receipt_count: 0,
            ..LaneIncidentHandoff::devnet(lane, &config, one_based(index))
        })
        .collect::<Vec<_>>();
    let artifacts = vec![IncidentHandoffArtifact::devnet(
        HandoffLane::CompileRuntime,
        HandoffArtifactKind::ReleaseHoldReceipt,
        &config,
        1,
    )];
    let operator_signoffs = vec![OperatorHandoffSignoff::devnet(
        "release-captain",
        "incident-command",
        None,
        1,
        HandoffDecision::RequestMoreEvidence,
        &config,
        1,
    )];
    let mut blockers = evaluate_blockers(&config, height, &lane_handoffs, &operator_signoffs);
    blockers
        .entry("fallback".to_string())
        .or_default()
        .push(HandoffBlockerKind::FailClosedRequired);
    blockers
        .entry("fallback_reason".to_string())
        .or_default()
        .push(if reason.trim().is_empty() {
            HandoffBlockerKind::EmptyRoot
        } else {
            HandoffBlockerKind::ReleaseHoldStillActive
        });
    let lane_handoff_root = roots_root(
        "incident-handoff-lanes",
        lane_handoffs.iter().map(LaneIncidentHandoff::state_root),
    );
    let artifact_root = roots_root(
        "incident-handoff-artifacts",
        artifacts.iter().map(IncidentHandoffArtifact::state_root),
    );
    let operator_root = roots_root(
        "incident-handoff-operator-signoffs",
        operator_signoffs
            .iter()
            .map(OperatorHandoffSignoff::state_root),
    );
    let blocker_root = blockers_root(&blockers);
    let summary =
        IncidentCommandSummary::build(&config, &lane_handoffs, &operator_signoffs, &blockers);
    State {
        config,
        height,
        lane_handoffs,
        artifacts,
        operator_signoffs,
        blockers,
        lane_handoff_root,
        artifact_root,
        operator_root,
        blocker_root,
        summary,
    }
}

fn blockers_root(blockers: &BTreeMap<String, Vec<HandoffBlockerKind>>) -> String {
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
    merkle_root("incident-handoff-blockers", &leaves)
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
        "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-RECORD",
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
        "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-ID",
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
        "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-SAMPLE-ROOT",
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
