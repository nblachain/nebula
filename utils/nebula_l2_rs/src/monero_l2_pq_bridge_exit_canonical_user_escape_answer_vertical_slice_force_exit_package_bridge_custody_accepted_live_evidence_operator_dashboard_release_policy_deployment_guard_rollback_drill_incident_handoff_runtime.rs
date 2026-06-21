use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageBridgeCustodyAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-bridge-custody-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const INCIDENT_HANDOFF_SUITE: &str =
    "monero-l2-pq-bridge-custody-rollback-drill-incident-handoff-v1";
pub const DEFAULT_WAVE: u64 = 86;
pub const DEFAULT_SOURCE_WAVE: u64 = 85;
pub const DEFAULT_HANDOFF_HEIGHT: u64 = 1_445_280;
pub const DEFAULT_MAX_ROLLBACK_DRILL_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_CUSTODY_HOLD_COUNT: u64 = 3;
pub const DEFAULT_MIN_RELEASE_CAP_COUNT: u64 = 3;
pub const DEFAULT_MIN_RESERVE_LIABILITY_COUNT: u64 = 3;
pub const DEFAULT_MIN_SIGNER_RECEIPT_COUNT: u64 = 4;
pub const DEFAULT_MIN_SIGNER_RECEIPT_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_AUTHORITY_TRANSITIONS: u64 = 3;

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
pub enum CustodyHoldKind {
    RollbackDrillHold,
    EmergencyReleaseFreeze,
    ReserveExitHold,
    SignerSessionFreeze,
}

impl CustodyHoldKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RollbackDrillHold => "rollback_drill_hold",
            Self::EmergencyReleaseFreeze => "emergency_release_freeze",
            Self::ReserveExitHold => "reserve_exit_hold",
            Self::SignerSessionFreeze => "signer_session_freeze",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCapKind {
    PerIncidentCap,
    PerEpochCap,
    SignerOverrideCap,
    ManualReleaseCap,
}

impl ReleaseCapKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PerIncidentCap => "per_incident_cap",
            Self::PerEpochCap => "per_epoch_cap",
            Self::SignerOverrideCap => "signer_override_cap",
            Self::ManualReleaseCap => "manual_release_cap",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRole {
    CustodyLead,
    IncidentCommander,
    ReleaseCoordinator,
    ReserveOperator,
    SignerQuorum,
}

impl AuthorityRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustodyLead => "custody_lead",
            Self::IncidentCommander => "incident_commander",
            Self::ReleaseCoordinator => "release_coordinator",
            Self::ReserveOperator => "reserve_operator",
            Self::SignerQuorum => "signer_quorum",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentBlockerKind {
    RollbackDrillRootMissing,
    RollbackDrillStale,
    RollbackDrillNotFailClosed,
    CustodyHoldRootMissing,
    ReleaseCapRootMissing,
    ReleaseCapExceeded,
    ReserveLiabilityRootMissing,
    ReserveLiabilityMismatch,
    SignerReceiptQuorumLow,
    SignerReceiptWeightLow,
    CommandAuthorityTransitionMissing,
    CommandAuthorityOpen,
    HandoffVerdictRejected,
}

impl IncidentBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RollbackDrillRootMissing => "rollback_drill_root_missing",
            Self::RollbackDrillStale => "rollback_drill_stale",
            Self::RollbackDrillNotFailClosed => "rollback_drill_not_fail_closed",
            Self::CustodyHoldRootMissing => "custody_hold_root_missing",
            Self::ReleaseCapRootMissing => "release_cap_root_missing",
            Self::ReleaseCapExceeded => "release_cap_exceeded",
            Self::ReserveLiabilityRootMissing => "reserve_liability_root_missing",
            Self::ReserveLiabilityMismatch => "reserve_liability_mismatch",
            Self::SignerReceiptQuorumLow => "signer_receipt_quorum_low",
            Self::SignerReceiptWeightLow => "signer_receipt_weight_low",
            Self::CommandAuthorityTransitionMissing => "command_authority_transition_missing",
            Self::CommandAuthorityOpen => "command_authority_open",
            Self::HandoffVerdictRejected => "handoff_verdict_rejected",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub incident_handoff_suite: String,
    pub incident_handoff_id: String,
    pub source_rollback_drill_id: String,
    pub bridge_custody_lane_id: String,
    pub release_policy_id: String,
    pub wave: u64,
    pub source_wave: u64,
    pub handoff_height: u64,
    pub max_rollback_drill_age_blocks: u64,
    pub min_custody_hold_count: u64,
    pub min_release_cap_count: u64,
    pub min_reserve_liability_count: u64,
    pub min_signer_receipt_count: u64,
    pub min_signer_receipt_weight: u64,
    pub min_authority_transitions: u64,
    pub require_rollback_drill_fail_closed: bool,
    pub require_release_caps_zeroed: bool,
    pub require_reserve_liability_match: bool,
    pub require_signer_handoff_receipts: bool,
    pub require_command_authority_closed: bool,
    pub fail_closed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            incident_handoff_suite: INCIDENT_HANDOFF_SUITE.to_string(),
            incident_handoff_id: runtime_id("wave-86-bridge-custody-incident-handoff"),
            source_rollback_drill_id: runtime_id("wave-85-bridge-custody-rollback-drill"),
            bridge_custody_lane_id: "bridge_custody".to_string(),
            release_policy_id: runtime_id("force-exit-package-bridge-custody-release-policy"),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            handoff_height: DEFAULT_HANDOFF_HEIGHT,
            max_rollback_drill_age_blocks: DEFAULT_MAX_ROLLBACK_DRILL_AGE_BLOCKS,
            min_custody_hold_count: DEFAULT_MIN_CUSTODY_HOLD_COUNT,
            min_release_cap_count: DEFAULT_MIN_RELEASE_CAP_COUNT,
            min_reserve_liability_count: DEFAULT_MIN_RESERVE_LIABILITY_COUNT,
            min_signer_receipt_count: DEFAULT_MIN_SIGNER_RECEIPT_COUNT,
            min_signer_receipt_weight: DEFAULT_MIN_SIGNER_RECEIPT_WEIGHT,
            min_authority_transitions: DEFAULT_MIN_AUTHORITY_TRANSITIONS,
            require_rollback_drill_fail_closed: true,
            require_release_caps_zeroed: true,
            require_reserve_liability_match: true,
            require_signer_handoff_receipts: true,
            require_command_authority_closed: true,
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
        ensure_non_empty("incident_handoff_suite", &self.incident_handoff_suite)?;
        ensure_non_empty("incident_handoff_id", &self.incident_handoff_id)?;
        ensure_non_empty("source_rollback_drill_id", &self.source_rollback_drill_id)?;
        ensure_non_empty("bridge_custody_lane_id", &self.bridge_custody_lane_id)?;
        ensure_non_empty("release_policy_id", &self.release_policy_id)?;
        ensure(
            self.wave > self.source_wave,
            "handoff wave must follow source wave",
        )?;
        ensure(self.handoff_height > 0, "handoff height must be non-zero")?;
        ensure(
            self.max_rollback_drill_age_blocks > 0,
            "rollback drill age window must be non-zero",
        )?;
        ensure(
            self.min_custody_hold_count > 0,
            "custody hold threshold must be non-zero",
        )?;
        ensure(
            self.min_release_cap_count > 0,
            "release cap threshold must be non-zero",
        )?;
        ensure(
            self.min_reserve_liability_count > 0,
            "reserve liability threshold must be non-zero",
        )?;
        ensure(
            self.min_signer_receipt_count > 0,
            "signer receipt threshold must be non-zero",
        )?;
        ensure(
            self.min_signer_receipt_weight > 0,
            "signer receipt weight must be non-zero",
        )?;
        ensure(
            self.min_authority_transitions > 0,
            "authority transition threshold must be non-zero",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "incident_handoff_suite": self.incident_handoff_suite,
            "incident_handoff_id": self.incident_handoff_id,
            "source_rollback_drill_id": self.source_rollback_drill_id,
            "bridge_custody_lane_id": self.bridge_custody_lane_id,
            "release_policy_id": self.release_policy_id,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "handoff_height": self.handoff_height,
            "max_rollback_drill_age_blocks": self.max_rollback_drill_age_blocks,
            "min_custody_hold_count": self.min_custody_hold_count,
            "min_release_cap_count": self.min_release_cap_count,
            "min_reserve_liability_count": self.min_reserve_liability_count,
            "min_signer_receipt_count": self.min_signer_receipt_count,
            "min_signer_receipt_weight": self.min_signer_receipt_weight,
            "min_authority_transitions": self.min_authority_transitions,
            "require_rollback_drill_fail_closed": self.require_rollback_drill_fail_closed,
            "require_release_caps_zeroed": self.require_release_caps_zeroed,
            "require_reserve_liability_match": self.require_reserve_liability_match,
            "require_signer_handoff_receipts": self.require_signer_handoff_receipts,
            "require_command_authority_closed": self.require_command_authority_closed,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceRollbackDrill {
    pub drill_id: String,
    pub observed_height: u64,
    pub drill_state_root: String,
    pub custody_rollback_root: String,
    pub release_abort_root: String,
    pub reserve_rollback_root: String,
    pub command_abort_root: String,
    pub fail_closed_root: String,
    pub fail_closed: bool,
    pub status: EvidenceStatus,
}

impl SourceRollbackDrill {
    pub fn devnet(config: &Config) -> Self {
        let observed_height = config.handoff_height.saturating_sub(24);
        let custody_rollback_root = source_component_root(config, "custody-rollback");
        let release_abort_root = source_component_root(config, "release-abort");
        let reserve_rollback_root = source_component_root(config, "reserve-rollback");
        let command_abort_root = source_component_root(config, "command-abort");
        let fail_closed_root = source_component_root(config, "fail-closed");
        let drill_state_root = merkle_root(
            "INCIDENT-HANDOFF-SOURCE-ROLLBACK-DRILL",
            &[
                json!({"custody_rollback_root": custody_rollback_root}),
                json!({"release_abort_root": release_abort_root}),
                json!({"reserve_rollback_root": reserve_rollback_root}),
                json!({"command_abort_root": command_abort_root}),
                json!({"fail_closed_root": fail_closed_root}),
            ],
        );
        Self {
            drill_id: config.source_rollback_drill_id.clone(),
            observed_height,
            drill_state_root,
            custody_rollback_root,
            release_abort_root,
            reserve_rollback_root,
            command_abort_root,
            fail_closed_root,
            fail_closed: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.drill_state_root.is_empty()
            && !self.fail_closed_root.is_empty()
            && (!config.require_rollback_drill_fail_closed || self.fail_closed)
            && config.handoff_height.saturating_sub(self.observed_height)
                <= config.max_rollback_drill_age_blocks
    }

    pub fn public_record(&self) -> Value {
        json!({
            "drill_id": self.drill_id,
            "observed_height": self.observed_height,
            "drill_state_root": self.drill_state_root,
            "custody_rollback_root": self.custody_rollback_root,
            "release_abort_root": self.release_abort_root,
            "reserve_rollback_root": self.reserve_rollback_root,
            "command_abort_root": self.command_abort_root,
            "fail_closed_root": self.fail_closed_root,
            "fail_closed": self.fail_closed,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-SOURCE-DRILL-STATE", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodyHold {
    pub hold_id: String,
    pub hold_kind: CustodyHoldKind,
    pub custody_hold_root: String,
    pub rollback_drill_root: String,
    pub dashboard_ack_root: String,
    pub enforced: bool,
    pub status: EvidenceStatus,
}

impl CustodyHold {
    pub fn devnet(config: &Config, ordinal: u64, hold_kind: CustodyHoldKind) -> Self {
        let hold_id = evidence_id(config, "custody-hold", hold_kind.as_str(), ordinal);
        let custody_hold_root = component_root(config, "custody-hold", &hold_id);
        let rollback_drill_root = component_root(config, "rollback-drill", &hold_id);
        let dashboard_ack_root = component_root(config, "dashboard-ack", &hold_id);
        Self {
            hold_id,
            hold_kind,
            custody_hold_root,
            rollback_drill_root,
            dashboard_ack_root,
            enforced: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.enforced
            && !self.custody_hold_root.is_empty()
            && !self.rollback_drill_root.is_empty()
            && !self.dashboard_ack_root.is_empty()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hold_id": self.hold_id,
            "hold_kind": self.hold_kind.as_str(),
            "custody_hold_root": self.custody_hold_root,
            "rollback_drill_root": self.rollback_drill_root,
            "dashboard_ack_root": self.dashboard_ack_root,
            "enforced": self.enforced,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-CUSTODY-HOLD", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCap {
    pub cap_id: String,
    pub cap_kind: ReleaseCapKind,
    pub cap_limit_atomic_units: u64,
    pub consumed_atomic_units: u64,
    pub cap_root: String,
    pub policy_binding_root: String,
    pub status: EvidenceStatus,
}

impl ReleaseCap {
    pub fn devnet(config: &Config, ordinal: u64, cap_kind: ReleaseCapKind) -> Self {
        let cap_id = evidence_id(config, "release-cap", cap_kind.as_str(), ordinal);
        let cap_root = component_root(config, "release-cap", &cap_id);
        let policy_binding_root = component_root(config, "policy-binding", &cap_id);
        Self {
            cap_id,
            cap_kind,
            cap_limit_atomic_units: 0,
            consumed_atomic_units: 0,
            cap_root,
            policy_binding_root,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.cap_root.is_empty()
            && !self.policy_binding_root.is_empty()
            && (!config.require_release_caps_zeroed
                || self.consumed_atomic_units <= self.cap_limit_atomic_units)
            && (!config.require_release_caps_zeroed || self.cap_limit_atomic_units == 0)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "cap_id": self.cap_id,
            "cap_kind": self.cap_kind.as_str(),
            "cap_limit_atomic_units": self.cap_limit_atomic_units,
            "consumed_atomic_units": self.consumed_atomic_units,
            "cap_root": self.cap_root,
            "policy_binding_root": self.policy_binding_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-RELEASE-CAP", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveLiabilityRoot {
    pub liability_id: String,
    pub reserve_operator_id: String,
    pub reserve_root: String,
    pub liability_root: String,
    pub custody_hold_root: String,
    pub balance_matches_liability: bool,
    pub status: EvidenceStatus,
}

impl ReserveLiabilityRoot {
    pub fn devnet(config: &Config, ordinal: u64, reserve_operator_id: &str) -> Self {
        let liability_id = evidence_id(config, "reserve-liability", reserve_operator_id, ordinal);
        let reserve_root = component_root(config, "reserve-root", &liability_id);
        let liability_root = component_root(config, "liability-root", &liability_id);
        let custody_hold_root = component_root(config, "custody-hold-root", &liability_id);
        Self {
            liability_id,
            reserve_operator_id: reserve_operator_id.to_string(),
            reserve_root,
            liability_root,
            custody_hold_root,
            balance_matches_liability: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.reserve_root.is_empty()
            && !self.liability_root.is_empty()
            && !self.custody_hold_root.is_empty()
            && (!config.require_reserve_liability_match || self.balance_matches_liability)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "liability_id": self.liability_id,
            "reserve_operator_id": self.reserve_operator_id,
            "reserve_root": self.reserve_root,
            "liability_root": self.liability_root,
            "custody_hold_root": self.custody_hold_root,
            "balance_matches_liability": self.balance_matches_liability,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-RESERVE-LIABILITY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignerHandoffReceipt {
    pub receipt_id: String,
    pub signer_id: String,
    pub signer_weight: u64,
    pub handoff_receipt_root: String,
    pub custody_hold_ack_root: String,
    pub command_revoke_root: String,
    pub accepted_live_evidence_root: String,
    pub status: EvidenceStatus,
}

impl SignerHandoffReceipt {
    pub fn devnet(config: &Config, ordinal: u64, signer_id: &str, signer_weight: u64) -> Self {
        let receipt_id = evidence_id(config, "signer-handoff-receipt", signer_id, ordinal);
        let handoff_receipt_root = component_root(config, "signer-handoff", &receipt_id);
        let custody_hold_ack_root = component_root(config, "custody-hold-ack", &receipt_id);
        let command_revoke_root = component_root(config, "command-revoke", &receipt_id);
        let accepted_live_evidence_root =
            component_root(config, "accepted-live-evidence", &receipt_id);
        Self {
            receipt_id,
            signer_id: signer_id.to_string(),
            signer_weight,
            handoff_receipt_root,
            custody_hold_ack_root,
            command_revoke_root,
            accepted_live_evidence_root,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
            && self.signer_weight > 0
            && !self.handoff_receipt_root.is_empty()
            && !self.custody_hold_ack_root.is_empty()
            && !self.command_revoke_root.is_empty()
            && !self.accepted_live_evidence_root.is_empty()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "signer_id": self.signer_id,
            "signer_weight": self.signer_weight,
            "handoff_receipt_root": self.handoff_receipt_root,
            "custody_hold_ack_root": self.custody_hold_ack_root,
            "command_revoke_root": self.command_revoke_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-SIGNER-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandAuthorityTransition {
    pub transition_id: String,
    pub from_role: AuthorityRole,
    pub to_role: AuthorityRole,
    pub revocation_root: String,
    pub acceptance_root: String,
    pub dashboard_handoff_root: String,
    pub command_window_closed: bool,
    pub status: EvidenceStatus,
}

impl CommandAuthorityTransition {
    pub fn devnet(
        config: &Config,
        ordinal: u64,
        from_role: AuthorityRole,
        to_role: AuthorityRole,
    ) -> Self {
        let subject = format!("{}-{}", from_role.as_str(), to_role.as_str());
        let transition_id = evidence_id(config, "command-authority-transition", &subject, ordinal);
        let revocation_root = component_root(config, "authority-revocation", &transition_id);
        let acceptance_root = component_root(config, "authority-acceptance", &transition_id);
        let dashboard_handoff_root = component_root(config, "dashboard-handoff", &transition_id);
        Self {
            transition_id,
            from_role,
            to_role,
            revocation_root,
            acceptance_root,
            dashboard_handoff_root,
            command_window_closed: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.accepted()
            && !self.revocation_root.is_empty()
            && !self.acceptance_root.is_empty()
            && !self.dashboard_handoff_root.is_empty()
            && (!config.require_command_authority_closed || self.command_window_closed)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "transition_id": self.transition_id,
            "from_role": self.from_role.as_str(),
            "to_role": self.to_role.as_str(),
            "revocation_root": self.revocation_root,
            "acceptance_root": self.acceptance_root,
            "dashboard_handoff_root": self.dashboard_handoff_root,
            "command_window_closed": self.command_window_closed,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "INCIDENT-HANDOFF-AUTHORITY-TRANSITION",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncidentBlocker {
    pub blocker_id: String,
    pub kind: IncidentBlockerKind,
    pub subject: String,
    pub evidence_root: String,
}

impl IncidentBlocker {
    pub fn new(
        config: &Config,
        kind: IncidentBlockerKind,
        subject: &str,
        evidence_root: &str,
    ) -> Self {
        let blocker_id = domain_hash(
            "INCIDENT-HANDOFF-BLOCKER-ID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.incident_handoff_id),
                HashPart::Str(kind.as_str()),
                HashPart::Str(subject),
                HashPart::Str(evidence_root),
            ],
            16,
        );
        Self {
            blocker_id,
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
        record_root("INCIDENT-HANDOFF-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffVerdict {
    pub incident_handoff_id: String,
    pub custody_hold_count: u64,
    pub release_cap_count: u64,
    pub reserve_liability_count: u64,
    pub signer_receipt_count: u64,
    pub signer_receipt_weight: u64,
    pub authority_transition_count: u64,
    pub evidence_root: String,
    pub blocker_root: String,
    pub fail_closed: bool,
    pub accepted: bool,
}

impl HandoffVerdict {
    pub fn from_state(config: &Config, state: &State, blockers: &[IncidentBlocker]) -> Self {
        let evidence_root = state.evidence_root();
        let blocker_root = list_root(
            "INCIDENT-HANDOFF-BLOCKER-ROOT",
            blockers.iter().map(IncidentBlocker::state_root),
        );
        let accepted = config.fail_closed && blockers.is_empty();
        Self {
            incident_handoff_id: config.incident_handoff_id.clone(),
            custody_hold_count: state.custody_hold_count(),
            release_cap_count: state.release_cap_count(config),
            reserve_liability_count: state.reserve_liability_count(config),
            signer_receipt_count: state.signer_receipt_count(),
            signer_receipt_weight: state.signer_receipt_weight(),
            authority_transition_count: state.authority_transition_count(config),
            evidence_root,
            blocker_root,
            fail_closed: config.fail_closed && !accepted,
            accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "incident_handoff_id": self.incident_handoff_id,
            "custody_hold_count": self.custody_hold_count,
            "release_cap_count": self.release_cap_count,
            "reserve_liability_count": self.reserve_liability_count,
            "signer_receipt_count": self.signer_receipt_count,
            "signer_receipt_weight": self.signer_receipt_weight,
            "authority_transition_count": self.authority_transition_count,
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "fail_closed": self.fail_closed,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("INCIDENT-HANDOFF-VERDICT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source_rollback_drill: SourceRollbackDrill,
    pub custody_holds: Vec<CustodyHold>,
    pub release_caps: Vec<ReleaseCap>,
    pub reserve_liability_roots: Vec<ReserveLiabilityRoot>,
    pub signer_handoff_receipts: Vec<SignerHandoffReceipt>,
    pub command_authority_transitions: Vec<CommandAuthorityTransition>,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        Self {
            source_rollback_drill: SourceRollbackDrill::devnet(&config),
            custody_holds: vec![
                CustodyHold::devnet(&config, 1, CustodyHoldKind::RollbackDrillHold),
                CustodyHold::devnet(&config, 2, CustodyHoldKind::EmergencyReleaseFreeze),
                CustodyHold::devnet(&config, 3, CustodyHoldKind::ReserveExitHold),
                CustodyHold::devnet(&config, 4, CustodyHoldKind::SignerSessionFreeze),
            ],
            release_caps: vec![
                ReleaseCap::devnet(&config, 1, ReleaseCapKind::PerIncidentCap),
                ReleaseCap::devnet(&config, 2, ReleaseCapKind::PerEpochCap),
                ReleaseCap::devnet(&config, 3, ReleaseCapKind::SignerOverrideCap),
                ReleaseCap::devnet(&config, 4, ReleaseCapKind::ManualReleaseCap),
            ],
            reserve_liability_roots: vec![
                ReserveLiabilityRoot::devnet(&config, 1, "reserve-operator-alpha"),
                ReserveLiabilityRoot::devnet(&config, 2, "reserve-operator-bravo"),
                ReserveLiabilityRoot::devnet(&config, 3, "reserve-operator-charlie"),
            ],
            signer_handoff_receipts: vec![
                SignerHandoffReceipt::devnet(&config, 1, "custody-signer-alpha", 18),
                SignerHandoffReceipt::devnet(&config, 2, "custody-signer-bravo", 17),
                SignerHandoffReceipt::devnet(&config, 3, "custody-signer-charlie", 16),
                SignerHandoffReceipt::devnet(&config, 4, "custody-signer-delta", 16),
                SignerHandoffReceipt::devnet(&config, 5, "custody-signer-echo", 8),
            ],
            command_authority_transitions: vec![
                CommandAuthorityTransition::devnet(
                    &config,
                    1,
                    AuthorityRole::ReleaseCoordinator,
                    AuthorityRole::IncidentCommander,
                ),
                CommandAuthorityTransition::devnet(
                    &config,
                    2,
                    AuthorityRole::CustodyLead,
                    AuthorityRole::SignerQuorum,
                ),
                CommandAuthorityTransition::devnet(
                    &config,
                    3,
                    AuthorityRole::ReserveOperator,
                    AuthorityRole::IncidentCommander,
                ),
            ],
            config,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure(
            self.source_rollback_drill.accepted(&self.config),
            "source rollback drill is not accepted fail-closed evidence",
        )?;
        ensure(
            self.custody_hold_count() >= self.config.min_custody_hold_count,
            "custody hold count is below threshold",
        )?;
        ensure(
            self.release_cap_count(&self.config) >= self.config.min_release_cap_count,
            "release cap count is below threshold",
        )?;
        ensure(
            self.reserve_liability_count(&self.config) >= self.config.min_reserve_liability_count,
            "reserve liability root count is below threshold",
        )?;
        ensure(
            self.signer_receipt_count() >= self.config.min_signer_receipt_count,
            "signer handoff receipt quorum is below threshold",
        )?;
        ensure(
            self.signer_receipt_weight() >= self.config.min_signer_receipt_weight,
            "signer handoff receipt weight is below threshold",
        )?;
        ensure(
            self.authority_transition_count(&self.config) >= self.config.min_authority_transitions,
            "command authority transition count is below threshold",
        )?;
        ensure(
            self.blockers().is_empty(),
            "incident handoff is fail-closed by blockers",
        )?;
        Ok(())
    }

    pub fn custody_hold_count(&self) -> u64 {
        self.custody_holds
            .iter()
            .filter(|hold| hold.accepted())
            .count() as u64
    }

    pub fn release_cap_count(&self, config: &Config) -> u64 {
        self.release_caps
            .iter()
            .filter(|cap| cap.accepted(config))
            .count() as u64
    }

    pub fn reserve_liability_count(&self, config: &Config) -> u64 {
        self.reserve_liability_roots
            .iter()
            .filter(|root| root.accepted(config))
            .count() as u64
    }

    pub fn signer_receipt_count(&self) -> u64 {
        self.signer_handoff_receipts
            .iter()
            .filter(|receipt| receipt.accepted())
            .count() as u64
    }

    pub fn signer_receipt_weight(&self) -> u64 {
        self.signer_handoff_receipts
            .iter()
            .filter(|receipt| receipt.accepted())
            .map(|receipt| receipt.signer_weight)
            .sum()
    }

    pub fn authority_transition_count(&self, config: &Config) -> u64 {
        self.command_authority_transitions
            .iter()
            .filter(|transition| transition.accepted(config))
            .count() as u64
    }

    pub fn evidence_root(&self) -> String {
        merkle_root(
            "INCIDENT-HANDOFF-EVIDENCE-ROOT",
            &[
                json!({"config_root": self.config.state_root()}),
                json!({"source_rollback_drill_root": self.source_rollback_drill.state_root()}),
                json!({"custody_hold_root": list_root("INCIDENT-HANDOFF-CUSTODY-HOLD-ROOT", self.custody_holds.iter().map(CustodyHold::state_root))}),
                json!({"release_cap_root": list_root("INCIDENT-HANDOFF-RELEASE-CAP-ROOT", self.release_caps.iter().map(ReleaseCap::state_root))}),
                json!({"reserve_liability_root": list_root("INCIDENT-HANDOFF-RESERVE-LIABILITY-ROOT", self.reserve_liability_roots.iter().map(ReserveLiabilityRoot::state_root))}),
                json!({"signer_receipt_root": list_root("INCIDENT-HANDOFF-SIGNER-RECEIPT-ROOT", self.signer_handoff_receipts.iter().map(SignerHandoffReceipt::state_root))}),
                json!({"authority_transition_root": list_root("INCIDENT-HANDOFF-AUTHORITY-ROOT", self.command_authority_transitions.iter().map(CommandAuthorityTransition::state_root))}),
            ],
        )
    }

    pub fn blockers(&self) -> Vec<IncidentBlocker> {
        unique_blockers(&self.config, self.blocker_kinds(), &self.evidence_root())
    }

    pub fn verdict(&self) -> HandoffVerdict {
        let blockers = self.blockers();
        HandoffVerdict::from_state(&self.config, self, &blockers)
    }

    pub fn public_record(&self) -> Value {
        let blockers = self.blockers();
        let verdict = HandoffVerdict::from_state(&self.config, self, &blockers);
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "source_rollback_drill": self.source_rollback_drill.public_record(),
            "custody_holds": self.custody_holds.iter().map(CustodyHold::public_record).collect::<Vec<_>>(),
            "release_caps": self.release_caps.iter().map(ReleaseCap::public_record).collect::<Vec<_>>(),
            "reserve_liability_roots": self.reserve_liability_roots.iter().map(ReserveLiabilityRoot::public_record).collect::<Vec<_>>(),
            "signer_handoff_receipts": self.signer_handoff_receipts.iter().map(SignerHandoffReceipt::public_record).collect::<Vec<_>>(),
            "command_authority_transitions": self.command_authority_transitions.iter().map(CommandAuthorityTransition::public_record).collect::<Vec<_>>(),
            "evidence_root": self.evidence_root(),
            "blockers": blockers.iter().map(IncidentBlocker::public_record).collect::<Vec<_>>(),
            "verdict": verdict.public_record(),
            "state_root": self.state_root(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "INCIDENT-HANDOFF-STATE",
            &json!({
                "config_root": self.config.state_root(),
                "evidence_root": self.evidence_root(),
                "verdict_root": self.verdict().state_root(),
            }),
        )
    }

    fn blocker_kinds(&self) -> Vec<(IncidentBlockerKind, String)> {
        let mut blockers = Vec::new();
        if !self.source_rollback_drill.accepted(&self.config) {
            if self.source_rollback_drill.drill_state_root.is_empty() {
                blockers.push((
                    IncidentBlockerKind::RollbackDrillRootMissing,
                    "source_rollback_drill".to_string(),
                ));
            }
            if self
                .config
                .handoff_height
                .saturating_sub(self.source_rollback_drill.observed_height)
                > self.config.max_rollback_drill_age_blocks
            {
                blockers.push((
                    IncidentBlockerKind::RollbackDrillStale,
                    "source_rollback_drill".to_string(),
                ));
            }
            if self.config.require_rollback_drill_fail_closed
                && !self.source_rollback_drill.fail_closed
            {
                blockers.push((
                    IncidentBlockerKind::RollbackDrillNotFailClosed,
                    "source_rollback_drill".to_string(),
                ));
            }
        }
        if self.custody_hold_count() < self.config.min_custody_hold_count
            || self.custody_holds.iter().any(|hold| !hold.accepted())
        {
            blockers.push((
                IncidentBlockerKind::CustodyHoldRootMissing,
                "custody_holds".to_string(),
            ));
        }
        if self.release_cap_count(&self.config) < self.config.min_release_cap_count {
            blockers.push((
                IncidentBlockerKind::ReleaseCapRootMissing,
                "release_caps".to_string(),
            ));
        }
        if self
            .release_caps
            .iter()
            .any(|cap| cap.consumed_atomic_units > cap.cap_limit_atomic_units)
        {
            blockers.push((
                IncidentBlockerKind::ReleaseCapExceeded,
                "release_caps".to_string(),
            ));
        }
        if self.reserve_liability_count(&self.config) < self.config.min_reserve_liability_count {
            blockers.push((
                IncidentBlockerKind::ReserveLiabilityRootMissing,
                "reserve_liability_roots".to_string(),
            ));
        }
        if self.reserve_liability_roots.iter().any(|root| {
            self.config.require_reserve_liability_match && !root.balance_matches_liability
        }) {
            blockers.push((
                IncidentBlockerKind::ReserveLiabilityMismatch,
                "reserve_liability_roots".to_string(),
            ));
        }
        if self.config.require_signer_handoff_receipts
            && self.signer_receipt_count() < self.config.min_signer_receipt_count
        {
            blockers.push((
                IncidentBlockerKind::SignerReceiptQuorumLow,
                "signer_handoff_receipts".to_string(),
            ));
        }
        if self.config.require_signer_handoff_receipts
            && self.signer_receipt_weight() < self.config.min_signer_receipt_weight
        {
            blockers.push((
                IncidentBlockerKind::SignerReceiptWeightLow,
                "signer_handoff_receipts".to_string(),
            ));
        }
        if self.authority_transition_count(&self.config) < self.config.min_authority_transitions {
            blockers.push((
                IncidentBlockerKind::CommandAuthorityTransitionMissing,
                "command_authority_transitions".to_string(),
            ));
        }
        if self
            .command_authority_transitions
            .iter()
            .any(|transition| !transition.command_window_closed)
        {
            blockers.push((
                IncidentBlockerKind::CommandAuthorityOpen,
                "command_authority_transitions".to_string(),
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

fn unique_blockers(
    config: &Config,
    blockers: Vec<(IncidentBlockerKind, String)>,
    evidence_root: &str,
) -> Vec<IncidentBlocker> {
    let mut seen = BTreeSet::new();
    blockers
        .into_iter()
        .filter(|(kind, subject)| seen.insert((*kind, subject.clone())))
        .map(|(kind, subject)| IncidentBlocker::new(config, kind, &subject, evidence_root))
        .collect()
}

fn runtime_id(label: &str) -> String {
    domain_hash(
        "INCIDENT-HANDOFF-ID",
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Str(label)],
        16,
    )
}

fn evidence_id(config: &Config, kind: &str, subject: &str, ordinal: u64) -> String {
    domain_hash(
        "INCIDENT-HANDOFF-EVIDENCE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.incident_handoff_id),
            HashPart::Str(kind),
            HashPart::Str(subject),
            HashPart::U64(ordinal),
        ],
        16,
    )
}

fn source_component_root(config: &Config, component: &str) -> String {
    domain_hash(
        "INCIDENT-HANDOFF-SOURCE-COMPONENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.incident_handoff_id),
            HashPart::Str(&config.source_rollback_drill_id),
            HashPart::Str(component),
            HashPart::U64(config.source_wave),
        ],
        32,
    )
}

fn component_root(config: &Config, kind: &str, evidence_id: &str) -> String {
    domain_hash(
        "INCIDENT-HANDOFF-COMPONENT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.incident_handoff_id),
            HashPart::Str(kind),
            HashPart::Str(evidence_id),
            HashPart::U64(config.handoff_height),
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
        &format!("{} must not be empty", field),
    )
}
