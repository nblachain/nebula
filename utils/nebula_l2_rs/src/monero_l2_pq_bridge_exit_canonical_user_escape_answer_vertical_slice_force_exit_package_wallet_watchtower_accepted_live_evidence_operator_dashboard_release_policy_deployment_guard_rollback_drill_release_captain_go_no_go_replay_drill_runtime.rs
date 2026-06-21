use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletWatchtowerAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-watchtower-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-release-captain-go-no-go-replay-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_DRILL_SUITE: &str = "wallet-watchtower-release-captain-go-no-go-replay-drill-v1";
pub const DEFAULT_WAVE: u64 = 88;
pub const DEFAULT_SOURCE_WAVE: u64 = 87;
pub const DEFAULT_HEIGHT: u64 = 4_280_808;
pub const DEFAULT_MAX_ROOT_AGE_BLOCKS: u64 = 64;
pub const DEFAULT_MIN_WATCHTOWER_AUDIT_ROOTS: u64 = 3;
pub const DEFAULT_MIN_WALLET_WARNING_ROOTS: u64 = 3;
pub const DEFAULT_MIN_SETTLEMENT_BLOCKERS: u64 = 1;
pub const DEFAULT_MIN_DISCLOSURE_GUARDS: u64 = 4;
pub const DEFAULT_MIN_ESCAPE_NOTIFICATION_ROOTS: u64 = 3;
pub const DEFAULT_MIN_CAPTAIN_SIGNOFFS: u64 = 4;
pub const DEFAULT_MAX_OPEN_SETTLEMENTS: u64 = 0;
pub const DEFAULT_MAX_DISCLOSURE_VIOLATIONS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Pending,
    Rejected,
    Stale,
    Blocked,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Rejected => "rejected",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }

    pub fn counts(self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn blocks(self) -> bool {
        matches!(self, Self::Rejected | Self::Stale | Self::Blocked)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatus {
    Finalized,
    PendingChallengeWindow,
    Delayed,
    Disputed,
    Unknown,
}

impl SettlementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Finalized => "finalized",
            Self::PendingChallengeWindow => "pending_challenge_window",
            Self::Delayed => "delayed",
            Self::Disputed => "disputed",
            Self::Unknown => "unknown",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Finalized)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureScope {
    WatchtowerReplayAudit,
    SettlementStatus,
    WalletWarning,
    UserEscapeNotice,
    ReleaseCaptainSignoff,
}

impl DisclosureScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WatchtowerReplayAudit => "watchtower_replay_audit",
            Self::SettlementStatus => "settlement_status",
            Self::WalletWarning => "wallet_warning",
            Self::UserEscapeNotice => "user_escape_notice",
            Self::ReleaseCaptainSignoff => "release_captain_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptainRole {
    ReleaseCaptain,
    WalletLead,
    WatchtowerLead,
    SettlementLead,
    PrivacyLead,
}

impl CaptainRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCaptain => "release_captain",
            Self::WalletLead => "wallet_lead",
            Self::WatchtowerLead => "watchtower_lead",
            Self::SettlementLead => "settlement_lead",
            Self::PrivacyLead => "privacy_lead",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictKind {
    Go,
    NoGoFailClosed,
}

impl VerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGoFailClosed => "no_go_fail_closed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    SourceChecklistRootMissing,
    SourceChecklistRootStale,
    WatchtowerReplayAuditRootMissing,
    SettlementStatusOpen,
    SettlementBlockerMissing,
    WalletWarningRootMissing,
    DisclosureGuardMissing,
    DisclosureViolation,
    UserEscapeNotificationRootMissing,
    ReleaseCaptainSignoffMissing,
    ReleaseCaptainSignoffOpen,
    EvidenceRejected,
    FailClosedNotArmed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceChecklistRootMissing => "source_checklist_root_missing",
            Self::SourceChecklistRootStale => "source_checklist_root_stale",
            Self::WatchtowerReplayAuditRootMissing => "watchtower_replay_audit_root_missing",
            Self::SettlementStatusOpen => "settlement_status_open",
            Self::SettlementBlockerMissing => "settlement_blocker_missing",
            Self::WalletWarningRootMissing => "wallet_warning_root_missing",
            Self::DisclosureGuardMissing => "disclosure_guard_missing",
            Self::DisclosureViolation => "disclosure_violation",
            Self::UserEscapeNotificationRootMissing => "user_escape_notification_root_missing",
            Self::ReleaseCaptainSignoffMissing => "release_captain_signoff_missing",
            Self::ReleaseCaptainSignoffOpen => "release_captain_signoff_open",
            Self::EvidenceRejected => "evidence_rejected",
            Self::FailClosedNotArmed => "fail_closed_not_armed",
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
    pub source_wave: u64,
    pub current_height: u64,
    pub max_root_age_blocks: u64,
    pub release_drill_id: String,
    pub source_wallet_checklist_root: String,
    pub source_watchtower_checklist_root: String,
    pub source_settlement_checklist_root: String,
    pub source_disclosure_checklist_root: String,
    pub source_escape_checklist_root: String,
    pub min_watchtower_audit_roots: u64,
    pub min_wallet_warning_roots: u64,
    pub min_settlement_blockers: u64,
    pub min_disclosure_guards: u64,
    pub min_escape_notification_roots: u64,
    pub min_captain_signoffs: u64,
    pub max_open_settlements: u64,
    pub max_disclosure_violations: u64,
    pub fail_closed_armed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_drill_suite: REPLAY_DRILL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            max_root_age_blocks: DEFAULT_MAX_ROOT_AGE_BLOCKS,
            release_drill_id: stable_id("release-captain-replay-drill", "wave88-devnet"),
            source_wallet_checklist_root: source_root("wave87-wallet-checklist-root"),
            source_watchtower_checklist_root: source_root("wave87-watchtower-checklist-root"),
            source_settlement_checklist_root: source_root("wave87-settlement-checklist-root"),
            source_disclosure_checklist_root: source_root("wave87-disclosure-checklist-root"),
            source_escape_checklist_root: source_root("wave87-user-escape-checklist-root"),
            min_watchtower_audit_roots: DEFAULT_MIN_WATCHTOWER_AUDIT_ROOTS,
            min_wallet_warning_roots: DEFAULT_MIN_WALLET_WARNING_ROOTS,
            min_settlement_blockers: DEFAULT_MIN_SETTLEMENT_BLOCKERS,
            min_disclosure_guards: DEFAULT_MIN_DISCLOSURE_GUARDS,
            min_escape_notification_roots: DEFAULT_MIN_ESCAPE_NOTIFICATION_ROOTS,
            min_captain_signoffs: DEFAULT_MIN_CAPTAIN_SIGNOFFS,
            max_open_settlements: DEFAULT_MAX_OPEN_SETTLEMENTS,
            max_disclosure_violations: DEFAULT_MAX_DISCLOSURE_VIOLATIONS,
            fail_closed_armed: true,
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
        ensure(self.wave > self.source_wave, "wave must follow source wave")?;
        ensure(
            self.max_root_age_blocks > 0,
            "root age window must be positive",
        )?;
        ensure_root(
            "source_wallet_checklist_root",
            &self.source_wallet_checklist_root,
        )?;
        ensure_root(
            "source_watchtower_checklist_root",
            &self.source_watchtower_checklist_root,
        )?;
        ensure_root(
            "source_settlement_checklist_root",
            &self.source_settlement_checklist_root,
        )?;
        ensure_root(
            "source_disclosure_checklist_root",
            &self.source_disclosure_checklist_root,
        )?;
        ensure_root(
            "source_escape_checklist_root",
            &self.source_escape_checklist_root,
        )?;
        ensure(
            self.min_watchtower_audit_roots > 0,
            "watchtower audit threshold must be positive",
        )?;
        ensure(
            self.min_wallet_warning_roots > 0,
            "wallet warning threshold must be positive",
        )?;
        ensure(
            self.min_settlement_blockers > 0,
            "settlement blocker threshold must be positive",
        )?;
        ensure(
            self.min_disclosure_guards > 0,
            "disclosure guard threshold must be positive",
        )?;
        ensure(
            self.min_escape_notification_roots > 0,
            "escape notification threshold must be positive",
        )?;
        ensure(
            self.min_captain_signoffs > 0,
            "captain signoff threshold must be positive",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "replay_drill_suite": self.replay_drill_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "max_root_age_blocks": self.max_root_age_blocks,
            "release_drill_id": self.release_drill_id,
            "source_wallet_checklist_root": self.source_wallet_checklist_root,
            "source_watchtower_checklist_root": self.source_watchtower_checklist_root,
            "source_settlement_checklist_root": self.source_settlement_checklist_root,
            "source_disclosure_checklist_root": self.source_disclosure_checklist_root,
            "source_escape_checklist_root": self.source_escape_checklist_root,
            "min_watchtower_audit_roots": self.min_watchtower_audit_roots,
            "min_wallet_warning_roots": self.min_wallet_warning_roots,
            "min_settlement_blockers": self.min_settlement_blockers,
            "min_disclosure_guards": self.min_disclosure_guards,
            "min_escape_notification_roots": self.min_escape_notification_roots,
            "min_captain_signoffs": self.min_captain_signoffs,
            "max_open_settlements": self.max_open_settlements,
            "max_disclosure_violations": self.max_disclosure_violations,
            "fail_closed_armed": self.fail_closed_armed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-REPLAY-DRILL-CONFIG", &self.public_record())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChecklistRootBinding {
    pub binding_id: String,
    pub observed_height: u64,
    pub wallet_checklist_root: String,
    pub watchtower_checklist_root: String,
    pub settlement_checklist_root: String,
    pub disclosure_checklist_root: String,
    pub escape_checklist_root: String,
    pub status: EvidenceStatus,
}

impl ChecklistRootBinding {
    pub fn devnet(config: &Config) -> Self {
        Self {
            binding_id: stable_id("wave87-checklist-binding", &config.release_drill_id),
            observed_height: config.current_height.saturating_sub(10),
            wallet_checklist_root: config.source_wallet_checklist_root.clone(),
            watchtower_checklist_root: config.source_watchtower_checklist_root.clone(),
            settlement_checklist_root: config.source_settlement_checklist_root.clone(),
            disclosure_checklist_root: config.source_disclosure_checklist_root.clone(),
            escape_checklist_root: config.source_escape_checklist_root.clone(),
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.wallet_checklist_root == config.source_wallet_checklist_root
            && self.watchtower_checklist_root == config.source_watchtower_checklist_root
            && self.settlement_checklist_root == config.source_settlement_checklist_root
            && self.disclosure_checklist_root == config.source_disclosure_checklist_root
            && self.escape_checklist_root == config.source_escape_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("binding_id", &self.binding_id)?;
        ensure_root("wallet_checklist_root", &self.wallet_checklist_root)?;
        ensure_root("watchtower_checklist_root", &self.watchtower_checklist_root)?;
        ensure_root("settlement_checklist_root", &self.settlement_checklist_root)?;
        ensure_root("disclosure_checklist_root", &self.disclosure_checklist_root)?;
        ensure_root("escape_checklist_root", &self.escape_checklist_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "binding_id": self.binding_id,
            "observed_height": self.observed_height,
            "wallet_checklist_root": self.wallet_checklist_root,
            "watchtower_checklist_root": self.watchtower_checklist_root,
            "settlement_checklist_root": self.settlement_checklist_root,
            "disclosure_checklist_root": self.disclosure_checklist_root,
            "escape_checklist_root": self.escape_checklist_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-CHECKLIST-ROOT-BINDING", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerReplayAuditRoot {
    pub audit_id: String,
    pub source_watchtower_root: String,
    pub replay_audit_root: String,
    pub replay_receipt_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl WatchtowerReplayAuditRoot {
    pub fn devnet(config: &Config, ordinal: u64) -> Self {
        let audit_id = evidence_id(config, "watchtower-replay-audit", ordinal);
        Self {
            source_watchtower_root: config.source_watchtower_checklist_root.clone(),
            replay_audit_root: component_root(config, "replay-audit", &audit_id),
            replay_receipt_root: component_root(config, "replay-receipt", &audit_id),
            observed_height: config
                .current_height
                .saturating_sub(8)
                .saturating_add(ordinal),
            audit_id,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.source_watchtower_root == config.source_watchtower_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("audit_id", &self.audit_id)?;
        ensure_root("source_watchtower_root", &self.source_watchtower_root)?;
        ensure_root("replay_audit_root", &self.replay_audit_root)?;
        ensure_root("replay_receipt_root", &self.replay_receipt_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "audit_id": self.audit_id,
            "source_watchtower_root": self.source_watchtower_root,
            "replay_audit_root": self.replay_audit_root,
            "replay_receipt_root": self.replay_receipt_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-WATCHTOWER-REPLAY-AUDIT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementStatusBlocker {
    pub blocker_id: String,
    pub source_settlement_root: String,
    pub settlement_status: SettlementStatus,
    pub blocker_root: String,
    pub hold_receipt_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl SettlementStatusBlocker {
    pub fn devnet(config: &Config, ordinal: u64, settlement_status: SettlementStatus) -> Self {
        let blocker_id = evidence_id(config, "settlement-status-blocker", ordinal);
        Self {
            source_settlement_root: config.source_settlement_checklist_root.clone(),
            blocker_root: component_root(config, "settlement-blocker", &blocker_id),
            hold_receipt_root: component_root(config, "settlement-hold-receipt", &blocker_id),
            observed_height: config
                .current_height
                .saturating_sub(7)
                .saturating_add(ordinal),
            blocker_id,
            settlement_status,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.source_settlement_root == config.source_settlement_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn open(&self) -> bool {
        self.settlement_status.blocks_release()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_root("source_settlement_root", &self.source_settlement_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        ensure_root("hold_receipt_root", &self.hold_receipt_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "source_settlement_root": self.source_settlement_root,
            "settlement_status": self.settlement_status.as_str(),
            "blocker_root": self.blocker_root,
            "hold_receipt_root": self.hold_receipt_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-SETTLEMENT-STATUS-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletWarningRoot {
    pub warning_id: String,
    pub source_wallet_root: String,
    pub wallet_cohort_root: String,
    pub warning_policy_root: String,
    pub operator_notice_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl WalletWarningRoot {
    pub fn devnet(config: &Config, ordinal: u64) -> Self {
        let warning_id = evidence_id(config, "wallet-warning-root", ordinal);
        Self {
            source_wallet_root: config.source_wallet_checklist_root.clone(),
            wallet_cohort_root: component_root(config, "wallet-cohort", &warning_id),
            warning_policy_root: component_root(config, "wallet-warning-policy", &warning_id),
            operator_notice_root: component_root(config, "wallet-operator-notice", &warning_id),
            observed_height: config
                .current_height
                .saturating_sub(6)
                .saturating_add(ordinal),
            warning_id,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.source_wallet_root == config.source_wallet_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("warning_id", &self.warning_id)?;
        ensure_root("source_wallet_root", &self.source_wallet_root)?;
        ensure_root("wallet_cohort_root", &self.wallet_cohort_root)?;
        ensure_root("warning_policy_root", &self.warning_policy_root)?;
        ensure_root("operator_notice_root", &self.operator_notice_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "warning_id": self.warning_id,
            "source_wallet_root": self.source_wallet_root,
            "wallet_cohort_root": self.wallet_cohort_root,
            "warning_policy_root": self.warning_policy_root,
            "operator_notice_root": self.operator_notice_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-WALLET-WARNING-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisclosureSafeguard {
    pub safeguard_id: String,
    pub scope: DisclosureScope,
    pub source_disclosure_root: String,
    pub redaction_policy_root: String,
    pub disclosure_commitment_root: String,
    pub scoped_only: bool,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl DisclosureSafeguard {
    pub fn devnet(config: &Config, scope: DisclosureScope, ordinal: u64) -> Self {
        let safeguard_id = evidence_id(config, scope.as_str(), ordinal);
        Self {
            source_disclosure_root: config.source_disclosure_checklist_root.clone(),
            redaction_policy_root: component_root(config, "redaction-policy", &safeguard_id),
            disclosure_commitment_root: component_root(
                config,
                "scoped-disclosure-commitment",
                &safeguard_id,
            ),
            scoped_only: true,
            observed_height: config
                .current_height
                .saturating_sub(5)
                .saturating_add(ordinal),
            safeguard_id,
            scope,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.scoped_only
            && self.source_disclosure_root == config.source_disclosure_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn violates_scope(&self) -> bool {
        !self.scoped_only || self.status.blocks()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("safeguard_id", &self.safeguard_id)?;
        ensure_root("source_disclosure_root", &self.source_disclosure_root)?;
        ensure_root("redaction_policy_root", &self.redaction_policy_root)?;
        ensure_root(
            "disclosure_commitment_root",
            &self.disclosure_commitment_root,
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "safeguard_id": self.safeguard_id,
            "scope": self.scope.as_str(),
            "source_disclosure_root": self.source_disclosure_root,
            "redaction_policy_root": self.redaction_policy_root,
            "disclosure_commitment_root": self.disclosure_commitment_root,
            "scoped_only": self.scoped_only,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-DISCLOSURE-SAFEGUARD", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserEscapeNotificationRoot {
    pub notice_id: String,
    pub source_escape_root: String,
    pub notice_template_root: String,
    pub notification_receipt_root: String,
    pub cohort_commitment_root: String,
    pub observed_height: u64,
    pub status: EvidenceStatus,
}

impl UserEscapeNotificationRoot {
    pub fn devnet(config: &Config, ordinal: u64) -> Self {
        let notice_id = evidence_id(config, "user-escape-notification", ordinal);
        Self {
            source_escape_root: config.source_escape_checklist_root.clone(),
            notice_template_root: component_root(config, "escape-notice-template", &notice_id),
            notification_receipt_root: component_root(config, "escape-notice-receipt", &notice_id),
            cohort_commitment_root: component_root(config, "escape-cohort", &notice_id),
            observed_height: config
                .current_height
                .saturating_sub(4)
                .saturating_add(ordinal),
            notice_id,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts()
            && self.source_escape_root == config.source_escape_checklist_root
            && evidence_fresh(config, self.observed_height)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("notice_id", &self.notice_id)?;
        ensure_root("source_escape_root", &self.source_escape_root)?;
        ensure_root("notice_template_root", &self.notice_template_root)?;
        ensure_root("notification_receipt_root", &self.notification_receipt_root)?;
        ensure_root("cohort_commitment_root", &self.cohort_commitment_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "notice_id": self.notice_id,
            "source_escape_root": self.source_escape_root,
            "notice_template_root": self.notice_template_root,
            "notification_receipt_root": self.notification_receipt_root,
            "cohort_commitment_root": self.cohort_commitment_root,
            "observed_height": self.observed_height,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-USER-ESCAPE-NOTIFICATION", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCaptainSignoff {
    pub signoff_id: String,
    pub role: CaptainRole,
    pub checklist_binding_root: String,
    pub go_no_go_packet_root: String,
    pub hold_authority_root: String,
    pub signed_height: u64,
    pub approves_go: bool,
    pub status: EvidenceStatus,
}

impl ReleaseCaptainSignoff {
    pub fn devnet(config: &Config, role: CaptainRole, ordinal: u64, binding_root: &str) -> Self {
        let signoff_id = evidence_id(config, role.as_str(), ordinal);
        Self {
            checklist_binding_root: binding_root.to_string(),
            go_no_go_packet_root: component_root(config, "go-no-go-packet", &signoff_id),
            hold_authority_root: component_root(config, "hold-authority", &signoff_id),
            signed_height: config
                .current_height
                .saturating_sub(3)
                .saturating_add(ordinal),
            approves_go: true,
            signoff_id,
            role,
            status: EvidenceStatus::Accepted,
        }
    }

    pub fn accepted(&self, config: &Config) -> bool {
        self.status.counts() && self.approves_go && evidence_fresh(config, self.signed_height)
    }

    pub fn open(&self) -> bool {
        !self.approves_go || !self.status.counts()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("signoff_id", &self.signoff_id)?;
        ensure_root("checklist_binding_root", &self.checklist_binding_root)?;
        ensure_root("go_no_go_packet_root", &self.go_no_go_packet_root)?;
        ensure_root("hold_authority_root", &self.hold_authority_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "role": self.role.as_str(),
            "checklist_binding_root": self.checklist_binding_root,
            "go_no_go_packet_root": self.go_no_go_packet_root,
            "hold_authority_root": self.hold_authority_root,
            "signed_height": self.signed_height,
            "approves_go": self.approves_go,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-RELEASE-CAPTAIN-SIGNOFF", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayDrillBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub subject: String,
    pub evidence_root: String,
    pub fail_closed: bool,
}

impl ReplayDrillBlocker {
    pub fn new(config: &Config, kind: BlockerKind, subject: &str, evidence_root: &str) -> Self {
        Self {
            blocker_id: stable_id(kind.as_str(), subject),
            kind,
            subject: subject.to_string(),
            evidence_root: evidence_root.to_string(),
            fail_closed: config.fail_closed_armed,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_non_empty("subject", &self.subject)?;
        ensure_root("evidence_root", &self.evidence_root)
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
        record_root("WAVE88-REPLAY-DRILL-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GoNoGoVerdict {
    pub verdict_id: String,
    pub verdict: VerdictKind,
    pub replay_drill_root: String,
    pub blocker_root: String,
    pub signoff_root: String,
    pub fail_closed: bool,
}

impl GoNoGoVerdict {
    pub fn new(
        config: &Config,
        replay_drill_root: &str,
        blocker_root: &str,
        signoff_root: &str,
        has_blockers: bool,
    ) -> Self {
        let verdict = if has_blockers || !config.fail_closed_armed {
            VerdictKind::NoGoFailClosed
        } else {
            VerdictKind::Go
        };
        Self {
            verdict_id: stable_id("go-no-go-verdict", replay_drill_root),
            verdict,
            replay_drill_root: replay_drill_root.to_string(),
            blocker_root: blocker_root.to_string(),
            signoff_root: signoff_root.to_string(),
            fail_closed: !matches!(verdict, VerdictKind::Go),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("verdict_id", &self.verdict_id)?;
        ensure_root("replay_drill_root", &self.replay_drill_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        ensure_root("signoff_root", &self.signoff_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "verdict": self.verdict.as_str(),
            "replay_drill_root": self.replay_drill_root,
            "blocker_root": self.blocker_root,
            "signoff_root": self.signoff_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WAVE88-GO-NO-GO-VERDICT", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCounters {
    pub watchtower_audit_roots: u64,
    pub wallet_warning_roots: u64,
    pub settlement_blockers: u64,
    pub open_settlements: u64,
    pub disclosure_guards: u64,
    pub disclosure_violations: u64,
    pub escape_notification_roots: u64,
    pub captain_signoffs: u64,
    pub open_captain_signoffs: u64,
    pub rejected_records: u64,
}

impl ReplayCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "watchtower_audit_roots": self.watchtower_audit_roots,
            "wallet_warning_roots": self.wallet_warning_roots,
            "settlement_blockers": self.settlement_blockers,
            "open_settlements": self.open_settlements,
            "disclosure_guards": self.disclosure_guards,
            "disclosure_violations": self.disclosure_violations,
            "escape_notification_roots": self.escape_notification_roots,
            "captain_signoffs": self.captain_signoffs,
            "open_captain_signoffs": self.open_captain_signoffs,
            "rejected_records": self.rejected_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub checklist_binding: ChecklistRootBinding,
    pub watchtower_audits: Vec<WatchtowerReplayAuditRoot>,
    pub settlement_blockers: Vec<SettlementStatusBlocker>,
    pub wallet_warnings: Vec<WalletWarningRoot>,
    pub disclosure_safeguards: Vec<DisclosureSafeguard>,
    pub escape_notifications: Vec<UserEscapeNotificationRoot>,
    pub captain_signoffs: Vec<ReleaseCaptainSignoff>,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let checklist_binding = ChecklistRootBinding::devnet(&config);
        let binding_root = checklist_binding.state_root();
        Self {
            watchtower_audits: (0..config.min_watchtower_audit_roots)
                .map(|ordinal| WatchtowerReplayAuditRoot::devnet(&config, ordinal))
                .collect(),
            settlement_blockers: vec![SettlementStatusBlocker::devnet(
                &config,
                0,
                SettlementStatus::Finalized,
            )],
            wallet_warnings: (0..config.min_wallet_warning_roots)
                .map(|ordinal| WalletWarningRoot::devnet(&config, ordinal))
                .collect(),
            disclosure_safeguards: vec![
                DisclosureSafeguard::devnet(&config, DisclosureScope::WatchtowerReplayAudit, 0),
                DisclosureSafeguard::devnet(&config, DisclosureScope::SettlementStatus, 1),
                DisclosureSafeguard::devnet(&config, DisclosureScope::WalletWarning, 2),
                DisclosureSafeguard::devnet(&config, DisclosureScope::UserEscapeNotice, 3),
            ],
            escape_notifications: (0..config.min_escape_notification_roots)
                .map(|ordinal| UserEscapeNotificationRoot::devnet(&config, ordinal))
                .collect(),
            captain_signoffs: vec![
                ReleaseCaptainSignoff::devnet(
                    &config,
                    CaptainRole::ReleaseCaptain,
                    0,
                    &binding_root,
                ),
                ReleaseCaptainSignoff::devnet(&config, CaptainRole::WalletLead, 1, &binding_root),
                ReleaseCaptainSignoff::devnet(
                    &config,
                    CaptainRole::WatchtowerLead,
                    2,
                    &binding_root,
                ),
                ReleaseCaptainSignoff::devnet(
                    &config,
                    CaptainRole::SettlementLead,
                    3,
                    &binding_root,
                ),
            ],
            checklist_binding,
            config,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.checklist_binding.validate()?;
        validate_records(&self.watchtower_audits, WatchtowerReplayAuditRoot::validate)?;
        validate_records(&self.settlement_blockers, SettlementStatusBlocker::validate)?;
        validate_records(&self.wallet_warnings, WalletWarningRoot::validate)?;
        validate_records(&self.disclosure_safeguards, DisclosureSafeguard::validate)?;
        validate_records(
            &self.escape_notifications,
            UserEscapeNotificationRoot::validate,
        )?;
        validate_records(&self.captain_signoffs, ReleaseCaptainSignoff::validate)
    }

    pub fn counters(&self) -> ReplayCounters {
        let mut counters = ReplayCounters::default();
        if self.checklist_binding.status.blocks() {
            counters.rejected_records = counters.rejected_records.saturating_add(1);
        }
        counters.watchtower_audit_roots = self
            .watchtower_audits
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.wallet_warning_roots = self
            .wallet_warnings
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.settlement_blockers = self
            .settlement_blockers
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.open_settlements = self
            .settlement_blockers
            .iter()
            .filter(|record| record.accepted(&self.config) && record.open())
            .count() as u64;
        counters.disclosure_guards = self
            .disclosure_safeguards
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.disclosure_violations = self
            .disclosure_safeguards
            .iter()
            .filter(|record| record.violates_scope())
            .count() as u64;
        counters.escape_notification_roots = self
            .escape_notifications
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.captain_signoffs = self
            .captain_signoffs
            .iter()
            .filter(|record| record.accepted(&self.config))
            .count() as u64;
        counters.open_captain_signoffs = self
            .captain_signoffs
            .iter()
            .filter(|record| record.open())
            .count() as u64;
        counters.rejected_records = counters.rejected_records.saturating_add(
            self.watchtower_audits
                .iter()
                .filter(|record| record.status.blocks())
                .count() as u64,
        );
        counters.rejected_records = counters.rejected_records.saturating_add(
            self.settlement_blockers
                .iter()
                .filter(|record| record.status.blocks())
                .count() as u64,
        );
        counters.rejected_records = counters.rejected_records.saturating_add(
            self.wallet_warnings
                .iter()
                .filter(|record| record.status.blocks())
                .count() as u64,
        );
        counters
    }

    pub fn blocker_records(&self) -> Vec<ReplayDrillBlocker> {
        unique_blockers(
            &self.config,
            self.blocker_kinds(),
            &self.evidence_bundle_root(),
        )
    }

    pub fn verdict(&self) -> GoNoGoVerdict {
        let blocker_root = self.blocker_root();
        let signoff_root = list_root(
            "WAVE88-SIGNOFF-ROOT",
            self.captain_signoffs
                .iter()
                .map(ReleaseCaptainSignoff::state_root),
        );
        GoNoGoVerdict::new(
            &self.config,
            &self.replay_drill_root(),
            &blocker_root,
            &signoff_root,
            !self.blocker_kinds().is_empty(),
        )
    }

    pub fn public_record(&self) -> Value {
        let blockers = self
            .blocker_records()
            .iter()
            .map(ReplayDrillBlocker::public_record)
            .collect::<Vec<_>>();
        let verdict = self.verdict();
        json!({
            "config": self.config.public_record(),
            "checklist_binding": self.checklist_binding.public_record(),
            "watchtower_audits": self.watchtower_audits.iter().map(WatchtowerReplayAuditRoot::public_record).collect::<Vec<_>>(),
            "settlement_blockers": self.settlement_blockers.iter().map(SettlementStatusBlocker::public_record).collect::<Vec<_>>(),
            "wallet_warnings": self.wallet_warnings.iter().map(WalletWarningRoot::public_record).collect::<Vec<_>>(),
            "disclosure_safeguards": self.disclosure_safeguards.iter().map(DisclosureSafeguard::public_record).collect::<Vec<_>>(),
            "escape_notifications": self.escape_notifications.iter().map(UserEscapeNotificationRoot::public_record).collect::<Vec<_>>(),
            "captain_signoffs": self.captain_signoffs.iter().map(ReleaseCaptainSignoff::public_record).collect::<Vec<_>>(),
            "counters": self.counters().public_record(),
            "blockers": blockers,
            "verdict": verdict.public_record(),
            "state_root": self.state_root(),
        })
    }

    pub fn replay_drill_root(&self) -> String {
        list_root(
            "WAVE88-REPLAY-DRILL-EVIDENCE",
            [
                vec![
                    self.config.state_root(),
                    self.checklist_binding.state_root(),
                ],
                self.watchtower_audits
                    .iter()
                    .map(WatchtowerReplayAuditRoot::state_root)
                    .collect(),
                self.settlement_blockers
                    .iter()
                    .map(SettlementStatusBlocker::state_root)
                    .collect(),
                self.wallet_warnings
                    .iter()
                    .map(WalletWarningRoot::state_root)
                    .collect(),
                self.disclosure_safeguards
                    .iter()
                    .map(DisclosureSafeguard::state_root)
                    .collect(),
                self.escape_notifications
                    .iter()
                    .map(UserEscapeNotificationRoot::state_root)
                    .collect(),
                self.captain_signoffs
                    .iter()
                    .map(ReleaseCaptainSignoff::state_root)
                    .collect(),
            ]
            .concat(),
        )
    }

    pub fn evidence_bundle_root(&self) -> String {
        record_root(
            "WAVE88-EVIDENCE-BUNDLE",
            &json!({
                "replay_drill_root": self.replay_drill_root(),
                "counters": self.counters().public_record(),
            }),
        )
    }

    pub fn blocker_root(&self) -> String {
        list_root(
            "WAVE88-BLOCKER-ROOT",
            self.blocker_records()
                .iter()
                .map(ReplayDrillBlocker::state_root),
        )
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WAVE88-STATE",
            &json!({
                "replay_drill_root": self.replay_drill_root(),
                "blocker_root": self.blocker_root(),
                "verdict_root": self.verdict().state_root(),
            }),
        )
    }

    fn blocker_kinds(&self) -> Vec<(BlockerKind, String)> {
        let counters = self.counters();
        let mut blockers = Vec::new();
        if !self.checklist_binding.accepted(&self.config) {
            blockers.push((
                BlockerKind::SourceChecklistRootMissing,
                "checklist_binding".to_string(),
            ));
        }
        if !evidence_fresh(&self.config, self.checklist_binding.observed_height) {
            blockers.push((
                BlockerKind::SourceChecklistRootStale,
                "checklist_binding".to_string(),
            ));
        }
        if counters.watchtower_audit_roots < self.config.min_watchtower_audit_roots {
            blockers.push((
                BlockerKind::WatchtowerReplayAuditRootMissing,
                "watchtower_audits".to_string(),
            ));
        }
        if counters.wallet_warning_roots < self.config.min_wallet_warning_roots {
            blockers.push((
                BlockerKind::WalletWarningRootMissing,
                "wallet_warnings".to_string(),
            ));
        }
        if counters.settlement_blockers < self.config.min_settlement_blockers {
            blockers.push((
                BlockerKind::SettlementBlockerMissing,
                "settlement_blockers".to_string(),
            ));
        }
        if counters.open_settlements > self.config.max_open_settlements {
            blockers.push((
                BlockerKind::SettlementStatusOpen,
                "settlement_blockers".to_string(),
            ));
        }
        if counters.disclosure_guards < self.config.min_disclosure_guards {
            blockers.push((
                BlockerKind::DisclosureGuardMissing,
                "disclosure_safeguards".to_string(),
            ));
        }
        if counters.disclosure_violations > self.config.max_disclosure_violations {
            blockers.push((
                BlockerKind::DisclosureViolation,
                "disclosure_safeguards".to_string(),
            ));
        }
        if counters.escape_notification_roots < self.config.min_escape_notification_roots {
            blockers.push((
                BlockerKind::UserEscapeNotificationRootMissing,
                "escape_notifications".to_string(),
            ));
        }
        if counters.captain_signoffs < self.config.min_captain_signoffs {
            blockers.push((
                BlockerKind::ReleaseCaptainSignoffMissing,
                "captain_signoffs".to_string(),
            ));
        }
        if counters.open_captain_signoffs > 0 {
            blockers.push((
                BlockerKind::ReleaseCaptainSignoffOpen,
                "captain_signoffs".to_string(),
            ));
        }
        if counters.rejected_records > 0 {
            blockers.push((BlockerKind::EvidenceRejected, "replay_drill".to_string()));
        }
        if !self.config.fail_closed_armed {
            blockers.push((BlockerKind::FailClosedNotArmed, "config".to_string()));
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
    blockers: Vec<(BlockerKind, String)>,
    evidence_root: &str,
) -> Vec<ReplayDrillBlocker> {
    let mut seen = BTreeSet::new();
    blockers
        .into_iter()
        .filter(|(kind, subject)| seen.insert((*kind, subject.clone())))
        .map(|(kind, subject)| ReplayDrillBlocker::new(config, kind, &subject, evidence_root))
        .collect()
}

fn validate_records<T, F>(records: &[T], validate: F) -> Result<()>
where
    F: Fn(&T) -> Result<()>,
{
    for record in records {
        validate(record)?;
    }
    Ok(())
}

fn evidence_fresh(config: &Config, observed_height: u64) -> bool {
    observed_height <= config.current_height
        && config.current_height.saturating_sub(observed_height) <= config.max_root_age_blocks
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

fn source_root(label: &str) -> String {
    domain_hash(
        "WAVE88-REPLAY-DRILL-SOURCE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn component_root(config: &Config, kind: &str, record_id: &str) -> String {
    domain_hash(
        "WAVE88-REPLAY-DRILL-COMPONENT-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_drill_id),
            HashPart::Str(kind),
            HashPart::Str(record_id),
            HashPart::U64(config.current_height),
        ],
        32,
    )
}

fn stable_id(prefix: &str, label: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        domain_hash(
            "WAVE88-REPLAY-DRILL-STABLE-ID",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(prefix),
                HashPart::Str(label),
            ],
            16,
        )
    )
}

fn evidence_id(config: &Config, kind: &str, ordinal: u64) -> String {
    domain_hash(
        "WAVE88-REPLAY-DRILL-EVIDENCE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_drill_id),
            HashPart::Str(kind),
            HashPart::U64(ordinal),
        ],
        16,
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
    ensure_non_empty(label, value)?;
    ensure(value.len() >= 32, &format!("{label} must be root-like"))
}
