use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletWatchtowerAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-watchtower-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const INCIDENT_HANDOFF_SUITE: &str =
    "wallet-watchtower-deployment-guard-rollback-drill-incident-handoff-v1";
pub const DEFAULT_HEIGHT: u64 = 4_280_704;
pub const DEFAULT_INCIDENT_HEIGHT: u64 = 4_280_688;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 48;
pub const DEFAULT_MIN_USER_WARNING_ROOTS: u16 = 2;
pub const DEFAULT_MIN_SETTLEMENT_EVIDENCE: u16 = 2;
pub const DEFAULT_MIN_WATCHTOWER_ROOTS: u16 = 2;
pub const DEFAULT_MIN_DISCLOSURE_WARNINGS: u16 = 2;
pub const DEFAULT_MIN_COMMAND_ROOM_ACKS: u16 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Warning,
    Missing,
    Rejected,
    Stale,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Warning => "warning",
            Self::Missing => "missing",
            Self::Rejected => "rejected",
            Self::Stale => "stale",
        }
    }

    pub fn counts(self) -> bool {
        matches!(self, Self::Accepted | Self::Warning)
    }

    pub fn blocks(self) -> bool {
        matches!(self, Self::Missing | Self::Rejected | Self::Stale)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatus {
    Settled,
    PendingChallengeWindow,
    Delayed,
    Disputed,
    Unknown,
}

impl SettlementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Settled => "settled",
            Self::PendingChallengeWindow => "pending_challenge_window",
            Self::Delayed => "delayed",
            Self::Disputed => "disputed",
            Self::Unknown => "unknown",
        }
    }

    pub fn fail_closed_required(self) -> bool {
        matches!(
            self,
            Self::PendingChallengeWindow | Self::Delayed | Self::Disputed | Self::Unknown
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchtowerRootKind {
    Challenge,
    Audit,
}

impl WatchtowerRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Challenge => "challenge",
            Self::Audit => "audit",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureScope {
    WalletUserWarning,
    SettlementStatus,
    WatchtowerChallenge,
    WatchtowerAudit,
    OperatorDashboard,
}

impl DisclosureScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletUserWarning => "wallet_user_warning",
            Self::SettlementStatus => "settlement_status",
            Self::WatchtowerChallenge => "watchtower_challenge",
            Self::WatchtowerAudit => "watchtower_audit",
            Self::OperatorDashboard => "operator_dashboard",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AckRole {
    IncidentCommander,
    WalletLead,
    WatchtowerLead,
    SettlementLead,
    ReleaseManager,
}

impl AckRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IncidentCommander => "incident_commander",
            Self::WalletLead => "wallet_lead",
            Self::WatchtowerLead => "watchtower_lead",
            Self::SettlementLead => "settlement_lead",
            Self::ReleaseManager => "release_manager",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FailClosedState {
    Open,
    HoldRelease,
    FailClosed,
    HandoffAccepted,
}

impl FailClosedState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::HoldRelease => "hold_release",
            Self::FailClosed => "fail_closed",
            Self::HandoffAccepted => "handoff_accepted",
        }
    }

    pub fn blocks_deployment(self) -> bool {
        matches!(self, Self::HoldRelease | Self::FailClosed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    MissingUserWarningRoot,
    MissingSettlementEvidence,
    SettlementNotFinal,
    MissingWatchtowerChallengeRoot,
    MissingWatchtowerAuditRoot,
    MissingScopedDisclosureWarning,
    DisclosureScopeTooBroad,
    CommandRoomAcknowledgementMissing,
    EvidenceStale,
    FailClosedNotAsserted,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingUserWarningRoot => "missing_user_warning_root",
            Self::MissingSettlementEvidence => "missing_settlement_evidence",
            Self::SettlementNotFinal => "settlement_not_final",
            Self::MissingWatchtowerChallengeRoot => "missing_watchtower_challenge_root",
            Self::MissingWatchtowerAuditRoot => "missing_watchtower_audit_root",
            Self::MissingScopedDisclosureWarning => "missing_scoped_disclosure_warning",
            Self::DisclosureScopeTooBroad => "disclosure_scope_too_broad",
            Self::CommandRoomAcknowledgementMissing => "command_room_acknowledgement_missing",
            Self::EvidenceStale => "evidence_stale",
            Self::FailClosedNotAsserted => "fail_closed_not_asserted",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub current_height: u64,
    pub incident_height: u64,
    pub max_evidence_age_blocks: u64,
    pub min_user_warning_roots: u16,
    pub min_settlement_evidence: u16,
    pub min_watchtower_roots: u16,
    pub min_disclosure_warnings: u16,
    pub min_command_room_acks: u16,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            current_height: DEFAULT_HEIGHT,
            incident_height: DEFAULT_INCIDENT_HEIGHT,
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            min_user_warning_roots: DEFAULT_MIN_USER_WARNING_ROOTS,
            min_settlement_evidence: DEFAULT_MIN_SETTLEMENT_EVIDENCE,
            min_watchtower_roots: DEFAULT_MIN_WATCHTOWER_ROOTS,
            min_disclosure_warnings: DEFAULT_MIN_DISCLOSURE_WARNINGS,
            min_command_room_acks: DEFAULT_MIN_COMMAND_ROOM_ACKS,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure(
            self.protocol_version == PROTOCOL_VERSION,
            "protocol version mismatch",
        )?;
        ensure(
            self.schema_version == SCHEMA_VERSION,
            "schema version mismatch",
        )?;
        ensure(
            self.current_height >= self.incident_height,
            "incident height is in the future",
        )?;
        ensure(
            self.max_evidence_age_blocks > 0,
            "max evidence age must be positive",
        )?;
        ensure(
            self.min_user_warning_roots > 0,
            "user warning quorum must be positive",
        )?;
        ensure(
            self.min_settlement_evidence > 0,
            "settlement quorum must be positive",
        )?;
        ensure(
            self.min_watchtower_roots > 0,
            "watchtower quorum must be positive",
        )?;
        ensure(
            self.min_disclosure_warnings > 0,
            "disclosure quorum must be positive",
        )?;
        ensure(
            self.min_command_room_acks > 0,
            "ack quorum must be positive",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": HASH_SUITE,
            "incident_handoff_suite": INCIDENT_HANDOFF_SUITE,
            "current_height": self.current_height,
            "incident_height": self.incident_height,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "min_user_warning_roots": self.min_user_warning_roots,
            "min_settlement_evidence": self.min_settlement_evidence,
            "min_watchtower_roots": self.min_watchtower_roots,
            "min_disclosure_warnings": self.min_disclosure_warnings,
            "min_command_room_acks": self.min_command_room_acks,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-INCIDENT-HANDOFF-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserWarningRoot {
    pub warning_id: String,
    pub warning_root: String,
    pub issued_height: u64,
    pub status: EvidenceStatus,
    pub wallet_cohort_tag: String,
}

impl UserWarningRoot {
    pub fn devnet(index: u16) -> Self {
        Self {
            warning_id: format!("user-warning-root-{index:02}"),
            warning_root: sample_root(&format!("user-warning-root-{index:02}")),
            issued_height: DEFAULT_INCIDENT_HEIGHT + u64::from(index),
            status: EvidenceStatus::Accepted,
            wallet_cohort_tag: format!("cohort-{index:02}"),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("warning_id", &self.warning_id)?;
        ensure_root("warning_root", &self.warning_root)?;
        ensure_non_empty("wallet_cohort_tag", &self.wallet_cohort_tag)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "warning_id": self.warning_id,
            "warning_root": self.warning_root,
            "issued_height": self.issued_height,
            "status": self.status.as_str(),
            "wallet_cohort_tag": self.wallet_cohort_tag,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-USER-WARNING-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementEvidence {
    pub evidence_id: String,
    pub settlement_status: SettlementStatus,
    pub evidence_root: String,
    pub observed_height: u64,
    pub confirmations: u64,
    pub status: EvidenceStatus,
}

impl SettlementEvidence {
    pub fn devnet(index: u16) -> Self {
        Self {
            evidence_id: format!("settlement-evidence-{index:02}"),
            settlement_status: SettlementStatus::PendingChallengeWindow,
            evidence_root: sample_root(&format!("settlement-evidence-{index:02}")),
            observed_height: DEFAULT_INCIDENT_HEIGHT + 2 + u64::from(index),
            confirmations: 4 + u64::from(index),
            status: EvidenceStatus::Warning,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("evidence_id", &self.evidence_id)?;
        ensure_root("evidence_root", &self.evidence_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "evidence_id": self.evidence_id,
            "settlement_status": self.settlement_status.as_str(),
            "evidence_root": self.evidence_root,
            "observed_height": self.observed_height,
            "confirmations": self.confirmations,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-SETTLEMENT-EVIDENCE", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerRoot {
    pub tower_id: String,
    pub root_kind: WatchtowerRootKind,
    pub evidence_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl WatchtowerRoot {
    pub fn devnet(tower_id: &str, root_kind: WatchtowerRootKind, offset: u64) -> Self {
        Self {
            tower_id: tower_id.to_string(),
            root_kind,
            evidence_root: sample_root(&format!("watchtower-{tower_id}-{}", root_kind.as_str())),
            observed_height: DEFAULT_INCIDENT_HEIGHT + offset,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("tower_id", &self.tower_id)?;
        ensure_root("watchtower evidence root", &self.evidence_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "tower_id": self.tower_id,
            "root_kind": self.root_kind.as_str(),
            "evidence_root": self.evidence_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-WATCHTOWER-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScopedDisclosureWarning {
    pub disclosure_id: String,
    pub scope: DisclosureScope,
    pub warning_root: String,
    pub redaction_root: String,
    pub public_fields_only: bool,
    pub status: EvidenceStatus,
}

impl ScopedDisclosureWarning {
    pub fn devnet(index: u16, scope: DisclosureScope) -> Self {
        Self {
            disclosure_id: format!("scoped-disclosure-{index:02}"),
            scope,
            warning_root: sample_root(&format!("scoped-disclosure-warning-{index:02}")),
            redaction_root: sample_root(&format!("scoped-disclosure-redaction-{index:02}")),
            public_fields_only: true,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("disclosure_id", &self.disclosure_id)?;
        ensure_root("disclosure warning root", &self.warning_root)?;
        ensure_root("disclosure redaction root", &self.redaction_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "disclosure_id": self.disclosure_id,
            "scope": self.scope.as_str(),
            "warning_root": self.warning_root,
            "redaction_root": self.redaction_root,
            "public_fields_only": self.public_fields_only,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-SCOPED-DISCLOSURE-WARNING", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandRoomAcknowledgement {
    pub acknowledgement_id: String,
    pub operator_id: String,
    pub role: AckRole,
    pub acknowledgement_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl CommandRoomAcknowledgement {
    pub fn devnet(index: u16, role: AckRole) -> Self {
        Self {
            acknowledgement_id: format!("command-room-ack-{index:02}"),
            operator_id: format!("operator-{index:02}"),
            role,
            acknowledgement_root: sample_root(&format!("command-room-ack-{index:02}")),
            observed_height: DEFAULT_INCIDENT_HEIGHT + 8 + u64::from(index),
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("acknowledgement_id", &self.acknowledgement_id)?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        ensure_root("acknowledgement_root", &self.acknowledgement_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "acknowledgement_id": self.acknowledgement_id,
            "operator_id": self.operator_id,
            "role": self.role.as_str(),
            "acknowledgement_root": self.acknowledgement_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-COMMAND-ROOM-ACK", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncidentBlocker {
    pub blocker_kind: BlockerKind,
    pub lane: String,
    pub detail_code: String,
    pub evidence_root: String,
}

impl IncidentBlocker {
    pub fn new(blocker_kind: BlockerKind, lane: &str, detail_code: &str) -> Self {
        Self {
            blocker_kind,
            lane: lane.to_string(),
            detail_code: detail_code.to_string(),
            evidence_root: sample_root(&format!("blocker-{}-{detail_code}", blocker_kind.as_str())),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_kind": self.blocker_kind.as_str(),
            "lane": self.lane,
            "detail_code": self.detail_code,
            "evidence_root": self.evidence_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-INCIDENT-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffRoots {
    pub config_root: String,
    pub user_warning_root: String,
    pub settlement_evidence_root: String,
    pub watchtower_challenge_root: String,
    pub watchtower_audit_root: String,
    pub disclosure_warning_root: String,
    pub command_room_ack_root: String,
    pub blocker_root: String,
    pub incident_handoff_root: String,
}

impl HandoffRoots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "user_warning_root": self.user_warning_root,
            "settlement_evidence_root": self.settlement_evidence_root,
            "watchtower_challenge_root": self.watchtower_challenge_root,
            "watchtower_audit_root": self.watchtower_audit_root,
            "disclosure_warning_root": self.disclosure_warning_root,
            "command_room_ack_root": self.command_room_ack_root,
            "blocker_root": self.blocker_root,
            "incident_handoff_root": self.incident_handoff_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffDecision {
    pub fail_closed_state: FailClosedState,
    pub handoff_accepted: bool,
    pub deployment_blocked: bool,
    pub blocker_count: usize,
    pub accepted_user_warning_roots: usize,
    pub accepted_settlement_evidence: usize,
    pub accepted_watchtower_roots: usize,
    pub accepted_disclosure_warnings: usize,
    pub command_room_ack_count: usize,
    pub decision_root: String,
}

impl HandoffDecision {
    pub fn public_record(&self) -> Value {
        json!({
            "fail_closed_state": self.fail_closed_state.as_str(),
            "handoff_accepted": self.handoff_accepted,
            "deployment_blocked": self.deployment_blocked,
            "blocker_count": self.blocker_count,
            "accepted_user_warning_roots": self.accepted_user_warning_roots,
            "accepted_settlement_evidence": self.accepted_settlement_evidence,
            "accepted_watchtower_roots": self.accepted_watchtower_roots,
            "accepted_disclosure_warnings": self.accepted_disclosure_warnings,
            "command_room_ack_count": self.command_room_ack_count,
            "decision_root": self.decision_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub user_warning_roots: BTreeMap<String, UserWarningRoot>,
    pub settlement_evidence: BTreeMap<String, SettlementEvidence>,
    pub watchtower_roots: BTreeMap<String, WatchtowerRoot>,
    pub disclosure_warnings: BTreeMap<String, ScopedDisclosureWarning>,
    pub command_room_acknowledgements: BTreeMap<String, CommandRoomAcknowledgement>,
    pub blockers: Vec<IncidentBlocker>,
    pub roots: HandoffRoots,
    pub decision: HandoffDecision,
}

impl State {
    pub fn new(
        config: Config,
        user_warning_roots: BTreeMap<String, UserWarningRoot>,
        settlement_evidence: BTreeMap<String, SettlementEvidence>,
        watchtower_roots: BTreeMap<String, WatchtowerRoot>,
        disclosure_warnings: BTreeMap<String, ScopedDisclosureWarning>,
        command_room_acknowledgements: BTreeMap<String, CommandRoomAcknowledgement>,
    ) -> Result<Self> {
        config.validate()?;
        validate_map(&user_warning_roots, UserWarningRoot::validate)?;
        validate_map(&settlement_evidence, SettlementEvidence::validate)?;
        validate_map(&watchtower_roots, WatchtowerRoot::validate)?;
        validate_map(&disclosure_warnings, ScopedDisclosureWarning::validate)?;
        validate_map(
            &command_room_acknowledgements,
            CommandRoomAcknowledgement::validate,
        )?;
        let blockers = derive_blockers(
            &config,
            &user_warning_roots,
            &settlement_evidence,
            &watchtower_roots,
            &disclosure_warnings,
            &command_room_acknowledgements,
        );
        let roots = build_roots(
            &config,
            &user_warning_roots,
            &settlement_evidence,
            &watchtower_roots,
            &disclosure_warnings,
            &command_room_acknowledgements,
            &blockers,
        );
        let decision = build_decision(
            &config,
            &user_warning_roots,
            &settlement_evidence,
            &watchtower_roots,
            &disclosure_warnings,
            &command_room_acknowledgements,
            &blockers,
            &roots,
        );
        Ok(Self {
            config,
            user_warning_roots,
            settlement_evidence,
            watchtower_roots,
            disclosure_warnings,
            command_room_acknowledgements,
            blockers,
            roots,
            decision,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let user_warning_roots = map_by(
            vec![UserWarningRoot::devnet(1), UserWarningRoot::devnet(2)],
            |warning| warning.warning_id.clone(),
        );
        let settlement_evidence = map_by(
            vec![SettlementEvidence::devnet(1), SettlementEvidence::devnet(2)],
            |evidence| evidence.evidence_id.clone(),
        );
        let watchtower_roots = map_by(
            vec![
                WatchtowerRoot::devnet("tower-a", WatchtowerRootKind::Challenge, 4),
                WatchtowerRoot::devnet("tower-a", WatchtowerRootKind::Audit, 5),
                WatchtowerRoot::devnet("tower-b", WatchtowerRootKind::Challenge, 6),
                WatchtowerRoot::devnet("tower-b", WatchtowerRootKind::Audit, 7),
            ],
            |root| format!("{}-{}", root.tower_id, root.root_kind.as_str()),
        );
        let disclosure_warnings = map_by(
            vec![
                ScopedDisclosureWarning::devnet(1, DisclosureScope::WalletUserWarning),
                ScopedDisclosureWarning::devnet(2, DisclosureScope::WatchtowerChallenge),
            ],
            |warning| warning.disclosure_id.clone(),
        );
        let command_room_acknowledgements = map_by(
            vec![
                CommandRoomAcknowledgement::devnet(1, AckRole::IncidentCommander),
                CommandRoomAcknowledgement::devnet(2, AckRole::WalletLead),
                CommandRoomAcknowledgement::devnet(3, AckRole::WatchtowerLead),
            ],
            |ack| ack.acknowledgement_id.clone(),
        );
        match Self::new(
            config,
            user_warning_roots,
            settlement_evidence,
            watchtower_roots,
            disclosure_warnings,
            command_room_acknowledgements,
        ) {
            Ok(state) => state,
            Err(_) => State::fail_closed_fallback(),
        }
    }

    pub fn fail_closed_fallback() -> Self {
        let config = Config::devnet();
        let blockers = vec![IncidentBlocker::new(
            BlockerKind::FailClosedNotAsserted,
            "incident_handoff",
            "state-construction-failed",
        )];
        let roots = build_roots(
            &config,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &blockers,
        );
        let decision = HandoffDecision {
            fail_closed_state: FailClosedState::FailClosed,
            handoff_accepted: false,
            deployment_blocked: true,
            blocker_count: blockers.len(),
            accepted_user_warning_roots: 0,
            accepted_settlement_evidence: 0,
            accepted_watchtower_roots: 0,
            accepted_disclosure_warnings: 0,
            command_room_ack_count: 0,
            decision_root: record_root(
                "WAVE86-INCIDENT-HANDOFF-FALLBACK-DECISION",
                &json!({"deployment_blocked": true, "reason": "state_construction_failed"}),
            ),
        };
        Self {
            config,
            user_warning_roots: BTreeMap::new(),
            settlement_evidence: BTreeMap::new(),
            watchtower_roots: BTreeMap::new(),
            disclosure_warnings: BTreeMap::new(),
            command_room_acknowledgements: BTreeMap::new(),
            blockers,
            roots,
            decision,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        validate_map(&self.user_warning_roots, UserWarningRoot::validate)?;
        validate_map(&self.settlement_evidence, SettlementEvidence::validate)?;
        validate_map(&self.watchtower_roots, WatchtowerRoot::validate)?;
        validate_map(&self.disclosure_warnings, ScopedDisclosureWarning::validate)?;
        validate_map(
            &self.command_room_acknowledgements,
            CommandRoomAcknowledgement::validate,
        )?;
        ensure(
            self.decision.deployment_blocked || self.decision.handoff_accepted,
            "handoff must either block deployment or be accepted",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "user_warning_roots": self.user_warning_roots.values().map(UserWarningRoot::public_record).collect::<Vec<_>>(),
            "settlement_evidence": self.settlement_evidence.values().map(SettlementEvidence::public_record).collect::<Vec<_>>(),
            "watchtower_roots": self.watchtower_roots.values().map(WatchtowerRoot::public_record).collect::<Vec<_>>(),
            "disclosure_warnings": self.disclosure_warnings.values().map(ScopedDisclosureWarning::public_record).collect::<Vec<_>>(),
            "command_room_acknowledgements": self.command_room_acknowledgements.values().map(CommandRoomAcknowledgement::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(IncidentBlocker::public_record).collect::<Vec<_>>(),
            "roots": self.roots.public_record(),
            "decision": self.decision.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE86-INCIDENT-HANDOFF-STATE", &self.public_record())
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

fn derive_blockers(
    config: &Config,
    user_warning_roots: &BTreeMap<String, UserWarningRoot>,
    settlement_evidence: &BTreeMap<String, SettlementEvidence>,
    watchtower_roots: &BTreeMap<String, WatchtowerRoot>,
    disclosure_warnings: &BTreeMap<String, ScopedDisclosureWarning>,
    command_room_acknowledgements: &BTreeMap<String, CommandRoomAcknowledgement>,
) -> Vec<IncidentBlocker> {
    let mut blockers = Vec::new();
    let accepted_user_warnings = user_warning_roots
        .values()
        .filter(|warning| evidence_fresh(config, warning.issued_height) && warning.status.counts())
        .count();
    if accepted_user_warnings < usize::from(config.min_user_warning_roots) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::MissingUserWarningRoot,
            "wallet_user_warning",
            "warning-root-quorum-not-met",
        ));
    }
    let accepted_settlement = settlement_evidence
        .values()
        .filter(|evidence| {
            evidence_fresh(config, evidence.observed_height) && evidence.status.counts()
        })
        .count();
    if accepted_settlement < usize::from(config.min_settlement_evidence) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::MissingSettlementEvidence,
            "settlement_status",
            "settlement-evidence-quorum-not-met",
        ));
    }
    if settlement_evidence
        .values()
        .any(|evidence| evidence.settlement_status.fail_closed_required())
    {
        blockers.push(IncidentBlocker::new(
            BlockerKind::SettlementNotFinal,
            "settlement_status",
            "settlement-status-requires-fail-closed",
        ));
    }
    let accepted_challenges = watchtower_roots
        .values()
        .filter(|root| {
            root.root_kind == WatchtowerRootKind::Challenge
                && evidence_fresh(config, root.observed_height)
                && root.status.counts()
        })
        .count();
    let accepted_audits = watchtower_roots
        .values()
        .filter(|root| {
            root.root_kind == WatchtowerRootKind::Audit
                && evidence_fresh(config, root.observed_height)
                && root.status.counts()
        })
        .count();
    if accepted_challenges < usize::from(config.min_watchtower_roots) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::MissingWatchtowerChallengeRoot,
            "watchtower_challenge",
            "challenge-root-quorum-not-met",
        ));
    }
    if accepted_audits < usize::from(config.min_watchtower_roots) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::MissingWatchtowerAuditRoot,
            "watchtower_audit",
            "audit-root-quorum-not-met",
        ));
    }
    let accepted_disclosures = disclosure_warnings
        .values()
        .filter(|warning| warning.public_fields_only && warning.status.counts())
        .count();
    if accepted_disclosures < usize::from(config.min_disclosure_warnings) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::MissingScopedDisclosureWarning,
            "scoped_disclosure",
            "disclosure-warning-quorum-not-met",
        ));
    }
    if disclosure_warnings
        .values()
        .any(|warning| !warning.public_fields_only)
    {
        blockers.push(IncidentBlocker::new(
            BlockerKind::DisclosureScopeTooBroad,
            "scoped_disclosure",
            "private-fields-requested",
        ));
    }
    if command_room_ack_count(command_room_acknowledgements)
        < usize::from(config.min_command_room_acks)
    {
        blockers.push(IncidentBlocker::new(
            BlockerKind::CommandRoomAcknowledgementMissing,
            "command_room_acknowledgement",
            "ack-quorum-not-met",
        ));
    }
    if any_stale(
        config,
        user_warning_roots,
        settlement_evidence,
        watchtower_roots,
    ) {
        blockers.push(IncidentBlocker::new(
            BlockerKind::EvidenceStale,
            "incident_handoff",
            "freshness-window-exceeded",
        ));
    }
    blockers
}

fn build_roots(
    config: &Config,
    user_warning_roots: &BTreeMap<String, UserWarningRoot>,
    settlement_evidence: &BTreeMap<String, SettlementEvidence>,
    watchtower_roots: &BTreeMap<String, WatchtowerRoot>,
    disclosure_warnings: &BTreeMap<String, ScopedDisclosureWarning>,
    command_room_acknowledgements: &BTreeMap<String, CommandRoomAcknowledgement>,
    blockers: &[IncidentBlocker],
) -> HandoffRoots {
    let config_root = config.state_root();
    let user_warning_root = merkle_root(
        "WAVE86-ROOT-USER-WARNINGS",
        &user_warning_roots
            .values()
            .map(UserWarningRoot::public_record)
            .collect::<Vec<_>>(),
    );
    let settlement_evidence_root = merkle_root(
        "WAVE86-ROOT-SETTLEMENT-EVIDENCE",
        &settlement_evidence
            .values()
            .map(SettlementEvidence::public_record)
            .collect::<Vec<_>>(),
    );
    let watchtower_challenge_root = merkle_root(
        "WAVE86-ROOT-WATCHTOWER-CHALLENGES",
        &watchtower_roots
            .values()
            .filter(|root| root.root_kind == WatchtowerRootKind::Challenge)
            .map(WatchtowerRoot::public_record)
            .collect::<Vec<_>>(),
    );
    let watchtower_audit_root = merkle_root(
        "WAVE86-ROOT-WATCHTOWER-AUDITS",
        &watchtower_roots
            .values()
            .filter(|root| root.root_kind == WatchtowerRootKind::Audit)
            .map(WatchtowerRoot::public_record)
            .collect::<Vec<_>>(),
    );
    let disclosure_warning_root = merkle_root(
        "WAVE86-ROOT-SCOPED-DISCLOSURES",
        &disclosure_warnings
            .values()
            .map(ScopedDisclosureWarning::public_record)
            .collect::<Vec<_>>(),
    );
    let command_room_ack_root = merkle_root(
        "WAVE86-ROOT-COMMAND-ROOM-ACKS",
        &command_room_acknowledgements
            .values()
            .map(CommandRoomAcknowledgement::public_record)
            .collect::<Vec<_>>(),
    );
    let blocker_root = merkle_root(
        "WAVE86-ROOT-INCIDENT-BLOCKERS",
        &blockers
            .iter()
            .map(IncidentBlocker::public_record)
            .collect::<Vec<_>>(),
    );
    let incident_handoff_root = domain_hash(
        "WAVE86-INCIDENT-HANDOFF-ROOT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.protocol_version),
            HashPart::U64(config.current_height),
            HashPart::Str(&config_root),
            HashPart::Str(&user_warning_root),
            HashPart::Str(&settlement_evidence_root),
            HashPart::Str(&watchtower_challenge_root),
            HashPart::Str(&watchtower_audit_root),
            HashPart::Str(&disclosure_warning_root),
            HashPart::Str(&command_room_ack_root),
            HashPart::Str(&blocker_root),
        ],
        32,
    );
    HandoffRoots {
        config_root,
        user_warning_root,
        settlement_evidence_root,
        watchtower_challenge_root,
        watchtower_audit_root,
        disclosure_warning_root,
        command_room_ack_root,
        blocker_root,
        incident_handoff_root,
    }
}

fn build_decision(
    config: &Config,
    user_warning_roots: &BTreeMap<String, UserWarningRoot>,
    settlement_evidence: &BTreeMap<String, SettlementEvidence>,
    watchtower_roots: &BTreeMap<String, WatchtowerRoot>,
    disclosure_warnings: &BTreeMap<String, ScopedDisclosureWarning>,
    command_room_acknowledgements: &BTreeMap<String, CommandRoomAcknowledgement>,
    blockers: &[IncidentBlocker],
    roots: &HandoffRoots,
) -> HandoffDecision {
    let accepted_user_warning_roots = user_warning_roots
        .values()
        .filter(|warning| evidence_fresh(config, warning.issued_height) && warning.status.counts())
        .count();
    let accepted_settlement_evidence = settlement_evidence
        .values()
        .filter(|evidence| {
            evidence_fresh(config, evidence.observed_height) && evidence.status.counts()
        })
        .count();
    let accepted_watchtower_roots = watchtower_roots
        .values()
        .filter(|root| evidence_fresh(config, root.observed_height) && root.status.counts())
        .count();
    let accepted_disclosure_warnings = disclosure_warnings
        .values()
        .filter(|warning| warning.public_fields_only && warning.status.counts())
        .count();
    let command_room_ack_count = command_room_ack_count(command_room_acknowledgements);
    let settlement_requires_hold = settlement_evidence
        .values()
        .any(|evidence| evidence.settlement_status.fail_closed_required());
    let handoff_accepted = blockers.is_empty()
        && !settlement_requires_hold
        && command_room_ack_count >= usize::from(config.min_command_room_acks);
    let fail_closed_state = if handoff_accepted {
        FailClosedState::HandoffAccepted
    } else if settlement_requires_hold {
        FailClosedState::FailClosed
    } else {
        FailClosedState::HoldRelease
    };
    let deployment_blocked = !handoff_accepted || fail_closed_state.blocks_deployment();
    let decision_root = domain_hash(
        "WAVE86-INCIDENT-HANDOFF-DECISION",
        &[
            HashPart::Str(fail_closed_state.as_str()),
            HashPart::Str(bool_str(handoff_accepted)),
            HashPart::Str(bool_str(deployment_blocked)),
            HashPart::U64(blockers.len() as u64),
            HashPart::U64(accepted_user_warning_roots as u64),
            HashPart::U64(accepted_settlement_evidence as u64),
            HashPart::U64(accepted_watchtower_roots as u64),
            HashPart::U64(accepted_disclosure_warnings as u64),
            HashPart::U64(command_room_ack_count as u64),
            HashPart::Str(&roots.incident_handoff_root),
        ],
        32,
    );
    HandoffDecision {
        fail_closed_state,
        handoff_accepted,
        deployment_blocked,
        blocker_count: blockers.len(),
        accepted_user_warning_roots,
        accepted_settlement_evidence,
        accepted_watchtower_roots,
        accepted_disclosure_warnings,
        command_room_ack_count,
        decision_root,
    }
}

fn command_room_ack_count(
    command_room_acknowledgements: &BTreeMap<String, CommandRoomAcknowledgement>,
) -> usize {
    let mut operators = BTreeSet::new();
    for acknowledgement in command_room_acknowledgements.values() {
        if acknowledgement.status.counts() {
            operators.insert(acknowledgement.operator_id.clone());
        }
    }
    operators.len()
}

fn any_stale(
    config: &Config,
    user_warning_roots: &BTreeMap<String, UserWarningRoot>,
    settlement_evidence: &BTreeMap<String, SettlementEvidence>,
    watchtower_roots: &BTreeMap<String, WatchtowerRoot>,
) -> bool {
    user_warning_roots
        .values()
        .any(|warning| !evidence_fresh(config, warning.issued_height))
        || settlement_evidence
            .values()
            .any(|evidence| !evidence_fresh(config, evidence.observed_height))
        || watchtower_roots
            .values()
            .any(|root| !evidence_fresh(config, root.observed_height))
}

fn evidence_fresh(config: &Config, observed_height: u64) -> bool {
    observed_height <= config.current_height
        && config.current_height.saturating_sub(observed_height) <= config.max_evidence_age_blocks
}

fn validate_map<T, F>(records: &BTreeMap<String, T>, validate: F) -> Result<()>
where
    F: Fn(&T) -> Result<()>,
{
    for record in records.values() {
        validate(record)?;
    }
    Ok(())
}

fn map_by<T, F>(records: Vec<T>, key: F) -> BTreeMap<String, T>
where
    F: Fn(&T) -> String,
{
    let mut mapped = BTreeMap::new();
    for record in records {
        mapped.insert(key(&record), record);
    }
    mapped
}

fn sample_root(label: &str) -> String {
    domain_hash(
        "WAVE86-INCIDENT-HANDOFF-SAMPLE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
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

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
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
