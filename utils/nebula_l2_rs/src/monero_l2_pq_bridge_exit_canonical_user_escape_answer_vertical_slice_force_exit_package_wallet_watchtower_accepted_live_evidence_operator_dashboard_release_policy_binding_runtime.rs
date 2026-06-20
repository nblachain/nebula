use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletWatchtowerAcceptedLiveEvidenceOperatorDashboardReleasePolicyBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-watchtower-accepted-live-evidence-operator-dashboard-release-policy-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_POLICY_SUITE: &str =
    "wallet-watchtower-accepted-live-evidence-dashboard-release-policy-binding-v1";
pub const DEFAULT_HEIGHT: u64 = 4_280_576;
pub const DEFAULT_MIN_MONERO_CONFIRMATIONS: u64 = 18;
pub const DEFAULT_MIN_WATCHTOWER_REPLAYS: u16 = 3;
pub const DEFAULT_MIN_REPLAY_CONFIRMATIONS: u64 = 12;
pub const DEFAULT_MIN_DASHBOARD_APPROVALS: u16 = 2;
pub const DEFAULT_MIN_REVIEWER_QUORUM: u16 = 2;
pub const DEFAULT_MAX_DASHBOARD_AGE_BLOCKS: u64 = 12;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MAX_WALLET_SCAN_GAP_BLOCKS: u64 = 8;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletScanStatus {
    Accepted,
    Quarantined,
    Missing,
    Rejected,
}

impl WalletScanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Quarantined => "quarantined",
            Self::Missing => "missing",
            Self::Rejected => "rejected",
        }
    }

    pub fn release_ready(self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Quarantined | Self::Missing | Self::Rejected)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletFinding {
    Clean,
    GapWithinPolicy,
    ScanGapExceeded,
    MissingKeyImageProof,
    DivergentOutputIndex,
    ViewTagMismatch,
    RecoveryPathUnreviewed,
    PrivacyLeak,
}

impl WalletFinding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::GapWithinPolicy => "gap_within_policy",
            Self::ScanGapExceeded => "scan_gap_exceeded",
            Self::MissingKeyImageProof => "missing_key_image_proof",
            Self::DivergentOutputIndex => "divergent_output_index",
            Self::ViewTagMismatch => "view_tag_mismatch",
            Self::RecoveryPathUnreviewed => "recovery_path_unreviewed",
            Self::PrivacyLeak => "privacy_leak",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(
            self,
            Self::ScanGapExceeded
                | Self::MissingKeyImageProof
                | Self::DivergentOutputIndex
                | Self::ViewTagMismatch
                | Self::RecoveryPathUnreviewed
                | Self::PrivacyLeak
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStatus {
    Passed,
    Warning,
    Missing,
    Failed,
}

impl ReplayStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Missing => "missing",
            Self::Failed => "failed",
        }
    }

    pub fn counts_for_quorum(self) -> bool {
        matches!(self, Self::Passed | Self::Warning)
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Missing | Self::Failed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EscapeActionStatus {
    NotRequired,
    Published,
    Acknowledged,
    Exercised,
    Expired,
    Blocked,
    Missing,
}

impl EscapeActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Published => "published",
            Self::Acknowledged => "acknowledged",
            Self::Exercised => "exercised",
            Self::Expired => "expired",
            Self::Blocked => "blocked",
            Self::Missing => "missing",
        }
    }

    pub fn satisfies_policy(self) -> bool {
        matches!(
            self,
            Self::NotRequired | Self::Published | Self::Acknowledged | Self::Exercised
        )
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Expired | Self::Blocked | Self::Missing)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardApprovalStatus {
    Approved,
    Conditional,
    Rejected,
    Superseded,
}

impl DashboardApprovalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Conditional => "conditional",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
        }
    }

    pub fn release_ready(self) -> bool {
        matches!(self, Self::Approved)
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Conditional | Self::Rejected)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerDecision {
    Accept,
    Watch,
    Reject,
}

impl ReviewerDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Watch => "watch",
            Self::Reject => "reject",
        }
    }

    pub fn accepts(self) -> bool {
        matches!(self, Self::Accept)
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Reject)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    WalletScanMissing,
    WalletScanRejected,
    WalletFindingBlocking,
    WalletConfirmationsTooShallow,
    WalletScanGapTooLarge,
    WatchtowerReplayQuorumMissing,
    WatchtowerReplayFailed,
    EscapeActionMissing,
    EscapeActionBlocked,
    DashboardApprovalMissing,
    DashboardApprovalRejected,
    ReviewerQuorumMissing,
    ReviewerRejected,
    DashboardStale,
    EvidenceStale,
    RootMismatch,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletScanMissing => "wallet_scan_missing",
            Self::WalletScanRejected => "wallet_scan_rejected",
            Self::WalletFindingBlocking => "wallet_finding_blocking",
            Self::WalletConfirmationsTooShallow => "wallet_confirmations_too_shallow",
            Self::WalletScanGapTooLarge => "wallet_scan_gap_too_large",
            Self::WatchtowerReplayQuorumMissing => "watchtower_replay_quorum_missing",
            Self::WatchtowerReplayFailed => "watchtower_replay_failed",
            Self::EscapeActionMissing => "escape_action_missing",
            Self::EscapeActionBlocked => "escape_action_blocked",
            Self::DashboardApprovalMissing => "dashboard_approval_missing",
            Self::DashboardApprovalRejected => "dashboard_approval_rejected",
            Self::ReviewerQuorumMissing => "reviewer_quorum_missing",
            Self::ReviewerRejected => "reviewer_rejected",
            Self::DashboardStale => "dashboard_stale",
            Self::EvidenceStale => "evidence_stale",
            Self::RootMismatch => "root_mismatch",
        }
    }

    pub fn owner_lane(self) -> &'static str {
        match self {
            Self::WalletScanMissing
            | Self::WalletScanRejected
            | Self::WalletFindingBlocking
            | Self::WalletConfirmationsTooShallow
            | Self::WalletScanGapTooLarge => "wallet_scan",
            Self::WatchtowerReplayQuorumMissing | Self::WatchtowerReplayFailed => {
                "watchtower_replay"
            }
            Self::EscapeActionMissing | Self::EscapeActionBlocked => "user_escape_action",
            Self::DashboardApprovalMissing
            | Self::DashboardApprovalRejected
            | Self::DashboardStale => "operator_dashboard",
            Self::ReviewerQuorumMissing | Self::ReviewerRejected => "release_policy_review",
            Self::EvidenceStale | Self::RootMismatch => "release_policy_binding",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyVerdictKind {
    Go,
    NoGo,
}

impl ReleasePolicyVerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub release_policy_suite: String,
    pub current_height: u64,
    pub min_monero_confirmations: u64,
    pub min_watchtower_replays: u16,
    pub min_replay_confirmations: u64,
    pub min_dashboard_approvals: u16,
    pub min_reviewer_quorum: u16,
    pub max_dashboard_age_blocks: u64,
    pub max_evidence_age_blocks: u64,
    pub max_wallet_scan_gap_blocks: u64,
    pub require_user_escape_action: bool,
    pub require_dashboard_root_match: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            release_policy_suite: RELEASE_POLICY_SUITE.to_string(),
            current_height: DEFAULT_HEIGHT,
            min_monero_confirmations: DEFAULT_MIN_MONERO_CONFIRMATIONS,
            min_watchtower_replays: DEFAULT_MIN_WATCHTOWER_REPLAYS,
            min_replay_confirmations: DEFAULT_MIN_REPLAY_CONFIRMATIONS,
            min_dashboard_approvals: DEFAULT_MIN_DASHBOARD_APPROVALS,
            min_reviewer_quorum: DEFAULT_MIN_REVIEWER_QUORUM,
            max_dashboard_age_blocks: DEFAULT_MAX_DASHBOARD_AGE_BLOCKS,
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            max_wallet_scan_gap_blocks: DEFAULT_MAX_WALLET_SCAN_GAP_BLOCKS,
            require_user_escape_action: true,
            require_dashboard_root_match: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure(
            self.schema_version == SCHEMA_VERSION,
            "schema version is not supported",
        )?;
        ensure(
            self.min_monero_confirmations > 0,
            "minimum monero confirmations must be non-zero",
        )?;
        ensure(
            self.min_watchtower_replays > 0,
            "minimum watchtower replay quorum must be non-zero",
        )?;
        ensure(
            self.min_dashboard_approvals > 0,
            "minimum dashboard approvals must be non-zero",
        )?;
        ensure(
            self.min_reviewer_quorum > 0,
            "minimum reviewer quorum must be non-zero",
        )?;
        ensure(
            self.max_dashboard_age_blocks > 0,
            "dashboard freshness window must be non-zero",
        )?;
        ensure(
            self.max_evidence_age_blocks > 0,
            "evidence freshness window must be non-zero",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "release_policy_suite": self.release_policy_suite,
            "current_height": self.current_height,
            "min_monero_confirmations": self.min_monero_confirmations,
            "min_watchtower_replays": self.min_watchtower_replays,
            "min_replay_confirmations": self.min_replay_confirmations,
            "min_dashboard_approvals": self.min_dashboard_approvals,
            "min_reviewer_quorum": self.min_reviewer_quorum,
            "max_dashboard_age_blocks": self.max_dashboard_age_blocks,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "max_wallet_scan_gap_blocks": self.max_wallet_scan_gap_blocks,
            "require_user_escape_action": self.require_user_escape_action,
            "require_dashboard_root_match": self.require_dashboard_root_match,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn policy_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BINDING-CONFIG",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletScanReceipt {
    pub receipt_id: String,
    pub wallet_id: String,
    pub transcript_root: String,
    pub accepted_live_evidence_root: String,
    pub scan_start_height: u64,
    pub scan_tip_height: u64,
    pub observed_gap_blocks: u64,
    pub monero_confirmations: u64,
    pub accepted_at_height: u64,
    pub operator_id: String,
    pub status: WalletScanStatus,
    pub findings: Vec<WalletFinding>,
}

impl WalletScanReceipt {
    pub fn accepted(
        receipt_id: &str,
        wallet_id: &str,
        scan_start_height: u64,
        scan_tip_height: u64,
        monero_confirmations: u64,
        accepted_at_height: u64,
        operator_id: &str,
    ) -> Self {
        let seed = json!({
            "receipt_id": receipt_id,
            "wallet_id": wallet_id,
            "scan_start_height": scan_start_height,
            "scan_tip_height": scan_tip_height,
            "monero_confirmations": monero_confirmations,
            "accepted_at_height": accepted_at_height,
            "operator_id": operator_id,
        });
        let transcript_root = record_root("WALLET-WATCHTOWER-SCAN-TRANSCRIPT", &seed);
        let accepted_live_evidence_root =
            record_root("WALLET-WATCHTOWER-SCAN-LIVE-EVIDENCE", &seed);
        Self {
            receipt_id: receipt_id.to_string(),
            wallet_id: wallet_id.to_string(),
            transcript_root,
            accepted_live_evidence_root,
            scan_start_height,
            scan_tip_height,
            observed_gap_blocks: scan_tip_height.saturating_sub(scan_start_height),
            monero_confirmations,
            accepted_at_height,
            operator_id: operator_id.to_string(),
            status: WalletScanStatus::Accepted,
            findings: vec![WalletFinding::Clean],
        }
    }

    pub fn with_status(mut self, status: WalletScanStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_findings(mut self, findings: Vec<WalletFinding>) -> Self {
        self.findings = findings;
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("receipt_id", &self.receipt_id)?;
        ensure_non_empty("wallet_id", &self.wallet_id)?;
        ensure_root("transcript_root", &self.transcript_root)?;
        ensure_root(
            "accepted_live_evidence_root",
            &self.accepted_live_evidence_root,
        )?;
        ensure(
            self.scan_tip_height >= self.scan_start_height,
            "wallet scan tip must not precede scan start",
        )?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        Ok(())
    }

    pub fn has_blocking_findings(&self) -> bool {
        self.findings.iter().any(|finding| finding.blocks_release())
    }

    pub fn is_fresh_at(&self, height: u64, max_age: u64) -> bool {
        self.accepted_at_height <= height
            && height.saturating_sub(self.accepted_at_height) <= max_age
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "wallet_id": self.wallet_id,
            "transcript_root": self.transcript_root,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
            "scan_start_height": self.scan_start_height,
            "scan_tip_height": self.scan_tip_height,
            "observed_gap_blocks": self.observed_gap_blocks,
            "monero_confirmations": self.monero_confirmations,
            "accepted_at_height": self.accepted_at_height,
            "operator_id": self.operator_id,
            "status": self.status.as_str(),
            "findings": self.findings.iter().map(|finding| finding.as_str()).collect::<Vec<_>>(),
            "blocks_release": self.status.blocks_release() || self.has_blocking_findings(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WALLET-WATCHTOWER-SCAN-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WatchtowerReplayReceipt {
    pub replay_id: String,
    pub watchtower_id: String,
    pub wallet_id: String,
    pub expected_wallet_root: String,
    pub observed_wallet_root: String,
    pub replay_root: String,
    pub replay_confirmations: u64,
    pub replayed_at_height: u64,
    pub operator_id: String,
    pub status: ReplayStatus,
}

impl WatchtowerReplayReceipt {
    pub fn passed(
        replay_id: &str,
        watchtower_id: &str,
        wallet: &WalletScanReceipt,
        replay_confirmations: u64,
        replayed_at_height: u64,
        operator_id: &str,
    ) -> Self {
        let replay_seed = json!({
            "replay_id": replay_id,
            "watchtower_id": watchtower_id,
            "wallet_id": wallet.wallet_id,
            "expected_wallet_root": wallet.state_root(),
            "replay_confirmations": replay_confirmations,
            "replayed_at_height": replayed_at_height,
            "operator_id": operator_id,
        });
        let replay_root = record_root("WALLET-WATCHTOWER-REPLAY-RECEIPT-SEED", &replay_seed);
        let expected_wallet_root = wallet.state_root();
        Self {
            replay_id: replay_id.to_string(),
            watchtower_id: watchtower_id.to_string(),
            wallet_id: wallet.wallet_id.clone(),
            expected_wallet_root: expected_wallet_root.clone(),
            observed_wallet_root: expected_wallet_root,
            replay_root,
            replay_confirmations,
            replayed_at_height,
            operator_id: operator_id.to_string(),
            status: ReplayStatus::Passed,
        }
    }

    pub fn with_status(mut self, status: ReplayStatus) -> Self {
        self.status = status;
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("replay_id", &self.replay_id)?;
        ensure_non_empty("watchtower_id", &self.watchtower_id)?;
        ensure_non_empty("wallet_id", &self.wallet_id)?;
        ensure_root("expected_wallet_root", &self.expected_wallet_root)?;
        ensure_root("observed_wallet_root", &self.observed_wallet_root)?;
        ensure_root("replay_root", &self.replay_root)?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        Ok(())
    }

    pub fn root_matches(&self) -> bool {
        self.expected_wallet_root == self.observed_wallet_root
    }

    pub fn counts_for_quorum(&self, min_confirmations: u64) -> bool {
        self.status.counts_for_quorum()
            && self.root_matches()
            && self.replay_confirmations >= min_confirmations
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_id": self.replay_id,
            "watchtower_id": self.watchtower_id,
            "wallet_id": self.wallet_id,
            "expected_wallet_root": self.expected_wallet_root,
            "observed_wallet_root": self.observed_wallet_root,
            "replay_root": self.replay_root,
            "replay_confirmations": self.replay_confirmations,
            "replayed_at_height": self.replayed_at_height,
            "operator_id": self.operator_id,
            "status": self.status.as_str(),
            "root_matches": self.root_matches(),
            "blocks_release": self.status.blocks_release() || !self.root_matches(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("WALLET-WATCHTOWER-REPLAY-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserEscapeAction {
    pub action_id: String,
    pub wallet_id: String,
    pub force_exit_id: String,
    pub notice_root: String,
    pub action_root: String,
    pub published_at_height: u64,
    pub expires_at_height: u64,
    pub status: EscapeActionStatus,
    pub operator_id: String,
}

impl UserEscapeAction {
    pub fn published(
        action_id: &str,
        wallet_id: &str,
        force_exit_id: &str,
        published_at_height: u64,
        expires_at_height: u64,
        operator_id: &str,
    ) -> Self {
        let seed = json!({
            "action_id": action_id,
            "wallet_id": wallet_id,
            "force_exit_id": force_exit_id,
            "published_at_height": published_at_height,
            "expires_at_height": expires_at_height,
            "operator_id": operator_id,
        });
        Self {
            action_id: action_id.to_string(),
            wallet_id: wallet_id.to_string(),
            force_exit_id: force_exit_id.to_string(),
            notice_root: record_root("WALLET-WATCHTOWER-ESCAPE-NOTICE", &seed),
            action_root: record_root("WALLET-WATCHTOWER-ESCAPE-ACTION", &seed),
            published_at_height,
            expires_at_height,
            status: EscapeActionStatus::Published,
            operator_id: operator_id.to_string(),
        }
    }

    pub fn with_status(mut self, status: EscapeActionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("action_id", &self.action_id)?;
        ensure_non_empty("wallet_id", &self.wallet_id)?;
        ensure_non_empty("force_exit_id", &self.force_exit_id)?;
        ensure_root("notice_root", &self.notice_root)?;
        ensure_root("action_root", &self.action_root)?;
        ensure(
            self.expires_at_height >= self.published_at_height,
            "escape action expiry must not precede publication",
        )?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        Ok(())
    }

    pub fn blocks_release_at(&self, height: u64) -> bool {
        self.status.blocks_release()
            || !self.status.satisfies_policy()
            || (height > self.expires_at_height
                && !matches!(self.status, EscapeActionStatus::Exercised))
    }

    pub fn public_record(&self) -> Value {
        json!({
            "action_id": self.action_id,
            "wallet_id": self.wallet_id,
            "force_exit_id": self.force_exit_id,
            "notice_root": self.notice_root,
            "action_root": self.action_root,
            "published_at_height": self.published_at_height,
            "expires_at_height": self.expires_at_height,
            "status": self.status.as_str(),
            "operator_id": self.operator_id,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-USER-ESCAPE-ACTION",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperatorDashboardApproval {
    pub approval_id: String,
    pub operator_id: String,
    pub role: String,
    pub dashboard_root: String,
    pub approved_at_height: u64,
    pub expires_at_height: u64,
    pub status: DashboardApprovalStatus,
    pub notes_root: String,
}

impl OperatorDashboardApproval {
    pub fn approved(
        approval_id: &str,
        operator_id: &str,
        role: &str,
        dashboard_root: &str,
        approved_at_height: u64,
        expires_at_height: u64,
    ) -> Self {
        let seed = json!({
            "approval_id": approval_id,
            "operator_id": operator_id,
            "role": role,
            "dashboard_root": dashboard_root,
            "approved_at_height": approved_at_height,
            "expires_at_height": expires_at_height,
        });
        Self {
            approval_id: approval_id.to_string(),
            operator_id: operator_id.to_string(),
            role: role.to_string(),
            dashboard_root: dashboard_root.to_string(),
            approved_at_height,
            expires_at_height,
            status: DashboardApprovalStatus::Approved,
            notes_root: record_root("WALLET-WATCHTOWER-DASHBOARD-APPROVAL-NOTES", &seed),
        }
    }

    pub fn with_status(mut self, status: DashboardApprovalStatus) -> Self {
        self.status = status;
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("approval_id", &self.approval_id)?;
        ensure_non_empty("operator_id", &self.operator_id)?;
        ensure_non_empty("role", &self.role)?;
        ensure_root("dashboard_root", &self.dashboard_root)?;
        ensure_root("notes_root", &self.notes_root)?;
        ensure(
            self.expires_at_height >= self.approved_at_height,
            "dashboard approval expiry must not precede approval height",
        )?;
        Ok(())
    }

    pub fn fresh_at(&self, height: u64) -> bool {
        self.approved_at_height <= height && height <= self.expires_at_height
    }

    pub fn public_record(&self) -> Value {
        json!({
            "approval_id": self.approval_id,
            "operator_id": self.operator_id,
            "role": self.role,
            "dashboard_root": self.dashboard_root,
            "approved_at_height": self.approved_at_height,
            "expires_at_height": self.expires_at_height,
            "status": self.status.as_str(),
            "notes_root": self.notes_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-DASHBOARD-APPROVAL",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleasePolicyReviewer {
    pub reviewer_id: String,
    pub role: String,
    pub reviewed_dashboard_root: String,
    pub reviewed_binding_root: String,
    pub decision: ReviewerDecision,
    pub reviewed_at_height: u64,
    pub review_root: String,
}

impl ReleasePolicyReviewer {
    pub fn accept(
        reviewer_id: &str,
        role: &str,
        reviewed_dashboard_root: &str,
        reviewed_binding_root: &str,
        reviewed_at_height: u64,
    ) -> Self {
        let seed = json!({
            "reviewer_id": reviewer_id,
            "role": role,
            "reviewed_dashboard_root": reviewed_dashboard_root,
            "reviewed_binding_root": reviewed_binding_root,
            "reviewed_at_height": reviewed_at_height,
        });
        Self {
            reviewer_id: reviewer_id.to_string(),
            role: role.to_string(),
            reviewed_dashboard_root: reviewed_dashboard_root.to_string(),
            reviewed_binding_root: reviewed_binding_root.to_string(),
            decision: ReviewerDecision::Accept,
            reviewed_at_height,
            review_root: record_root("WALLET-WATCHTOWER-RELEASE-POLICY-REVIEW", &seed),
        }
    }

    pub fn with_decision(mut self, decision: ReviewerDecision) -> Self {
        self.decision = decision;
        self
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("reviewer_id", &self.reviewer_id)?;
        ensure_non_empty("role", &self.role)?;
        ensure_root("reviewed_dashboard_root", &self.reviewed_dashboard_root)?;
        ensure_root("reviewed_binding_root", &self.reviewed_binding_root)?;
        ensure_root("review_root", &self.review_root)?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_id": self.reviewer_id,
            "role": self.role,
            "reviewed_dashboard_root": self.reviewed_dashboard_root,
            "reviewed_binding_root": self.reviewed_binding_root,
            "decision": self.decision.as_str(),
            "reviewed_at_height": self.reviewed_at_height,
            "review_root": self.review_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-REVIEWER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleasePolicyBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub owner_lane: String,
    pub subject_id: String,
    pub evidence_root: String,
    pub observed_at_height: u64,
    pub detail: String,
    pub fail_closed: bool,
}

impl ReleasePolicyBlocker {
    pub fn new(
        kind: BlockerKind,
        subject_id: &str,
        evidence_root: &str,
        observed_at_height: u64,
        detail: &str,
    ) -> Self {
        let seed = json!({
            "kind": kind.as_str(),
            "owner_lane": kind.owner_lane(),
            "subject_id": subject_id,
            "evidence_root": evidence_root,
            "observed_at_height": observed_at_height,
            "detail": detail,
        });
        Self {
            blocker_id: record_root("WALLET-WATCHTOWER-RELEASE-POLICY-BLOCKER-ID", &seed),
            kind,
            owner_lane: kind.owner_lane().to_string(),
            subject_id: subject_id.to_string(),
            evidence_root: evidence_root.to_string(),
            observed_at_height,
            detail: detail.to_string(),
            fail_closed: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "owner_lane": self.owner_lane,
            "subject_id": self.subject_id,
            "evidence_root": self.evidence_root,
            "observed_at_height": self.observed_at_height,
            "detail": self.detail,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BLOCKER",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleasePolicyVerdict {
    pub verdict_id: String,
    pub release_id: String,
    pub verdict: ReleasePolicyVerdictKind,
    pub binding_root: String,
    pub wallet_scan_root: String,
    pub watchtower_replay_root: String,
    pub user_escape_action_root: String,
    pub dashboard_approval_root: String,
    pub reviewer_root: String,
    pub blocker_root: String,
    pub decided_at_height: u64,
    pub reason: String,
}

impl ReleasePolicyVerdict {
    pub fn from_state(state: &State) -> Self {
        let blockers = state.derive_blockers();
        let verdict = if state.config.fail_closed && !blockers.is_empty() {
            ReleasePolicyVerdictKind::NoGo
        } else {
            ReleasePolicyVerdictKind::Go
        };
        let roots = state.roots();
        let reason = if matches!(verdict, ReleasePolicyVerdictKind::Go) {
            "wallet/watchtower accepted-live-evidence is bound to release policy".to_string()
        } else {
            "wallet/watchtower release policy binding failed closed".to_string()
        };
        let seed = json!({
            "release_id": state.release_id,
            "verdict": verdict.as_str(),
            "binding_root": roots.binding_root,
            "blocker_root": roots.blocker_root,
            "decided_at_height": state.config.current_height,
        });
        Self {
            verdict_id: record_root("WALLET-WATCHTOWER-RELEASE-POLICY-VERDICT-ID", &seed),
            release_id: state.release_id.clone(),
            verdict,
            binding_root: roots.binding_root,
            wallet_scan_root: roots.wallet_scan_root,
            watchtower_replay_root: roots.watchtower_replay_root,
            user_escape_action_root: roots.user_escape_action_root,
            dashboard_approval_root: roots.dashboard_approval_root,
            reviewer_root: roots.reviewer_root,
            blocker_root: roots.blocker_root,
            decided_at_height: state.config.current_height,
            reason,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "release_id": self.release_id,
            "verdict": self.verdict.as_str(),
            "binding_root": self.binding_root,
            "wallet_scan_root": self.wallet_scan_root,
            "watchtower_replay_root": self.watchtower_replay_root,
            "user_escape_action_root": self.user_escape_action_root,
            "dashboard_approval_root": self.dashboard_approval_root,
            "reviewer_root": self.reviewer_root,
            "blocker_root": self.blocker_root,
            "decided_at_height": self.decided_at_height,
            "reason": self.reason,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-VERDICT",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingRoots {
    pub wallet_scan_root: String,
    pub watchtower_replay_root: String,
    pub user_escape_action_root: String,
    pub dashboard_approval_root: String,
    pub reviewer_root: String,
    pub blocker_root: String,
    pub binding_root: String,
}

impl BindingRoots {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_root": self.wallet_scan_root,
            "watchtower_replay_root": self.watchtower_replay_root,
            "user_escape_action_root": self.user_escape_action_root,
            "dashboard_approval_root": self.dashboard_approval_root,
            "reviewer_root": self.reviewer_root,
            "blocker_root": self.blocker_root,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingSummary {
    pub wallets: usize,
    pub accepted_wallets: usize,
    pub replay_receipts: usize,
    pub replay_quorum: usize,
    pub escape_actions: usize,
    pub dashboard_approvals: usize,
    pub release_policy_reviewers: usize,
    pub blockers: usize,
    pub verdict: ReleasePolicyVerdictKind,
}

impl BindingSummary {
    pub fn public_record(&self) -> Value {
        json!({
            "wallets": self.wallets,
            "accepted_wallets": self.accepted_wallets,
            "replay_receipts": self.replay_receipts,
            "replay_quorum": self.replay_quorum,
            "escape_actions": self.escape_actions,
            "dashboard_approvals": self.dashboard_approvals,
            "release_policy_reviewers": self.release_policy_reviewers,
            "blockers": self.blockers,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BINDING-SUMMARY",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub release_id: String,
    pub force_exit_id: String,
    pub dashboard_root: String,
    pub expected_dashboard_root: String,
    pub runbook_audit_root: String,
    pub dashboard_finalization_root: String,
    pub wallet_scans: BTreeMap<String, WalletScanReceipt>,
    pub watchtower_replays: BTreeMap<String, WatchtowerReplayReceipt>,
    pub user_escape_actions: BTreeMap<String, UserEscapeAction>,
    pub dashboard_approvals: BTreeMap<String, OperatorDashboardApproval>,
    pub reviewers: BTreeMap<String, ReleasePolicyReviewer>,
}

impl State {
    pub fn new(
        config: Config,
        release_id: &str,
        force_exit_id: &str,
        dashboard_root: &str,
        runbook_audit_root: &str,
        dashboard_finalization_root: &str,
    ) -> Result<Self> {
        config.validate()?;
        ensure_non_empty("release_id", release_id)?;
        ensure_non_empty("force_exit_id", force_exit_id)?;
        ensure_root("dashboard_root", dashboard_root)?;
        ensure_root("runbook_audit_root", runbook_audit_root)?;
        ensure_root("dashboard_finalization_root", dashboard_finalization_root)?;
        Ok(Self {
            config,
            release_id: release_id.to_string(),
            force_exit_id: force_exit_id.to_string(),
            dashboard_root: dashboard_root.to_string(),
            expected_dashboard_root: dashboard_root.to_string(),
            runbook_audit_root: runbook_audit_root.to_string(),
            dashboard_finalization_root: dashboard_finalization_root.to_string(),
            wallet_scans: BTreeMap::new(),
            watchtower_replays: BTreeMap::new(),
            user_escape_actions: BTreeMap::new(),
            dashboard_approvals: BTreeMap::new(),
            reviewers: BTreeMap::new(),
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let dashboard_root = sample_root("wallet-watchtower-dashboard-root");
        let runbook_root = sample_root("wallet-watchtower-runbook-audit-root");
        let finalization_root = sample_root("wallet-watchtower-dashboard-finalization-root");
        let mut state = match Self::new(
            config,
            "force-exit-wallet-watchtower-release-devnet",
            "force-exit-devnet-001",
            &dashboard_root,
            &runbook_root,
            &finalization_root,
        ) {
            Ok(state) => state,
            Err(_) => Self {
                config: Config::devnet(),
                release_id: "force-exit-wallet-watchtower-release-fallback".to_string(),
                force_exit_id: "force-exit-fallback".to_string(),
                dashboard_root: dashboard_root.clone(),
                expected_dashboard_root: dashboard_root.clone(),
                runbook_audit_root: runbook_root.clone(),
                dashboard_finalization_root: finalization_root.clone(),
                wallet_scans: BTreeMap::new(),
                watchtower_replays: BTreeMap::new(),
                user_escape_actions: BTreeMap::new(),
                dashboard_approvals: BTreeMap::new(),
                reviewers: BTreeMap::new(),
            },
        };
        let wallet = WalletScanReceipt::accepted(
            "wallet-scan-devnet-001",
            "wallet-devnet-escape-001",
            DEFAULT_HEIGHT.saturating_sub(24),
            DEFAULT_HEIGHT.saturating_sub(18),
            DEFAULT_MIN_MONERO_CONFIRMATIONS,
            DEFAULT_HEIGHT.saturating_sub(12),
            "wallet-ops-devnet",
        );
        let _ = state.record_wallet_scan(wallet.clone());
        for index in 0..DEFAULT_MIN_WATCHTOWER_REPLAYS {
            let replay = WatchtowerReplayReceipt::passed(
                &format!("watchtower-replay-devnet-{index}"),
                &format!("watchtower-devnet-{index}"),
                &wallet,
                DEFAULT_MIN_REPLAY_CONFIRMATIONS.saturating_add(u64::from(index)),
                DEFAULT_HEIGHT
                    .saturating_sub(8)
                    .saturating_add(u64::from(index)),
                &format!("watchtower-operator-{index}"),
            );
            let _ = state.record_watchtower_replay(replay);
        }
        let action = UserEscapeAction::published(
            "escape-action-devnet-001",
            "wallet-devnet-escape-001",
            "force-exit-devnet-001",
            DEFAULT_HEIGHT.saturating_sub(10),
            DEFAULT_HEIGHT.saturating_add(24),
            "release-dashboard",
        );
        let _ = state.record_user_escape_action(action);
        for index in 0..DEFAULT_MIN_DASHBOARD_APPROVALS {
            let approval = OperatorDashboardApproval::approved(
                &format!("dashboard-approval-devnet-{index}"),
                &format!("dashboard-operator-{index}"),
                if index == 0 {
                    "wallet_ops"
                } else {
                    "watchtower_ops"
                },
                &dashboard_root,
                DEFAULT_HEIGHT
                    .saturating_sub(4)
                    .saturating_add(u64::from(index)),
                DEFAULT_HEIGHT.saturating_add(DEFAULT_MAX_DASHBOARD_AGE_BLOCKS),
            );
            let _ = state.record_dashboard_approval(approval);
        }
        let binding_root = state.binding_root();
        for index in 0..DEFAULT_MIN_REVIEWER_QUORUM {
            let reviewer = ReleasePolicyReviewer::accept(
                &format!("release-policy-reviewer-{index}"),
                if index == 0 {
                    "release_policy"
                } else {
                    "security_review"
                },
                &dashboard_root,
                &binding_root,
                DEFAULT_HEIGHT
                    .saturating_sub(2)
                    .saturating_add(u64::from(index)),
            );
            let _ = state.record_reviewer(reviewer);
        }
        state
    }

    pub fn bind_expected_dashboard_root(&mut self, root: &str) -> Result<()> {
        ensure_root("expected_dashboard_root", root)?;
        self.expected_dashboard_root = root.to_string();
        Ok(())
    }

    pub fn record_wallet_scan(&mut self, receipt: WalletScanReceipt) -> Result<()> {
        receipt.validate()?;
        self.wallet_scans
            .insert(receipt.receipt_id.clone(), receipt);
        Ok(())
    }

    pub fn record_watchtower_replay(&mut self, receipt: WatchtowerReplayReceipt) -> Result<()> {
        receipt.validate()?;
        let wallet_known = self
            .wallet_scans
            .values()
            .any(|wallet| wallet.wallet_id == receipt.wallet_id);
        ensure(
            wallet_known,
            "watchtower replay references unknown wallet scan",
        )?;
        self.watchtower_replays
            .insert(receipt.replay_id.clone(), receipt);
        Ok(())
    }

    pub fn record_user_escape_action(&mut self, action: UserEscapeAction) -> Result<()> {
        action.validate()?;
        ensure(
            action.force_exit_id == self.force_exit_id,
            "user escape action force exit id does not match state",
        )?;
        self.user_escape_actions
            .insert(action.action_id.clone(), action);
        Ok(())
    }

    pub fn record_dashboard_approval(&mut self, approval: OperatorDashboardApproval) -> Result<()> {
        approval.validate()?;
        self.dashboard_approvals
            .insert(approval.approval_id.clone(), approval);
        Ok(())
    }

    pub fn record_reviewer(&mut self, reviewer: ReleasePolicyReviewer) -> Result<()> {
        reviewer.validate()?;
        self.reviewers
            .insert(reviewer.reviewer_id.clone(), reviewer);
        Ok(())
    }

    pub fn accepted_wallet_count(&self) -> usize {
        self.wallet_scans
            .values()
            .filter(|scan| scan.status.release_ready() && !scan.has_blocking_findings())
            .count()
    }

    pub fn replay_quorum_count(&self) -> usize {
        let mut watchtowers = BTreeSet::new();
        for replay in self.watchtower_replays.values() {
            if replay.counts_for_quorum(self.config.min_replay_confirmations) {
                watchtowers.insert(replay.watchtower_id.clone());
            }
        }
        watchtowers.len()
    }

    pub fn dashboard_approval_quorum_count(&self) -> usize {
        let mut operators = BTreeSet::new();
        for approval in self.dashboard_approvals.values() {
            if approval.status.release_ready()
                && approval.fresh_at(self.config.current_height)
                && approval.dashboard_root == self.dashboard_root
            {
                operators.insert(approval.operator_id.clone());
            }
        }
        operators.len()
    }

    pub fn reviewer_quorum_count(&self) -> usize {
        let mut reviewers = BTreeSet::new();
        for reviewer in self.reviewers.values() {
            if reviewer.decision.accepts()
                && reviewer.reviewed_dashboard_root == self.dashboard_root
                && reviewer.reviewed_binding_root == self.binding_root()
            {
                reviewers.insert(reviewer.reviewer_id.clone());
            }
        }
        reviewers.len()
    }

    pub fn wallet_scan_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-WALLET-SCANS",
            &self
                .wallet_scans
                .values()
                .map(WalletScanReceipt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn watchtower_replay_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-WATCHTOWER-REPLAYS",
            &self
                .watchtower_replays
                .values()
                .map(WatchtowerReplayReceipt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn user_escape_action_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-USER-ESCAPE-ACTIONS",
            &self
                .user_escape_actions
                .values()
                .map(UserEscapeAction::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn dashboard_approval_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-DASHBOARD-APPROVALS",
            &self
                .dashboard_approvals
                .values()
                .map(OperatorDashboardApproval::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn reviewer_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-REVIEWERS",
            &self
                .reviewers
                .values()
                .map(ReleasePolicyReviewer::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        merkle_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BLOCKERS",
            &self
                .derive_blockers()
                .iter()
                .map(ReleasePolicyBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn binding_root(&self) -> String {
        domain_hash(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BINDING-ROOT",
            &[
                HashPart::Str(&self.config.chain_id),
                HashPart::Str(&self.config.protocol_version),
                HashPart::Str(&self.release_id),
                HashPart::Str(&self.force_exit_id),
                HashPart::Str(&self.dashboard_root),
                HashPart::Str(&self.runbook_audit_root),
                HashPart::Str(&self.dashboard_finalization_root),
                HashPart::Str(&self.wallet_scan_root()),
                HashPart::Str(&self.watchtower_replay_root()),
                HashPart::Str(&self.user_escape_action_root()),
                HashPart::Str(&self.dashboard_approval_root()),
                HashPart::Int(self.config.current_height as i128),
            ],
            32,
        )
    }

    pub fn roots(&self) -> BindingRoots {
        BindingRoots {
            wallet_scan_root: self.wallet_scan_root(),
            watchtower_replay_root: self.watchtower_replay_root(),
            user_escape_action_root: self.user_escape_action_root(),
            dashboard_approval_root: self.dashboard_approval_root(),
            reviewer_root: self.reviewer_root(),
            blocker_root: self.blocker_root(),
            binding_root: self.binding_root(),
        }
    }

    pub fn derive_blockers(&self) -> Vec<ReleasePolicyBlocker> {
        let mut blockers = Vec::new();
        if self.wallet_scans.is_empty() {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::WalletScanMissing,
                &self.release_id,
                &self.config.policy_root(),
                self.config.current_height,
                "release policy has no accepted wallet scan receipts",
            ));
        }
        for scan in self.wallet_scans.values() {
            if scan.status.blocks_release() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WalletScanRejected,
                    &scan.receipt_id,
                    &scan.state_root(),
                    self.config.current_height,
                    "wallet scan receipt is not accepted live evidence",
                ));
            }
            if scan.has_blocking_findings() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WalletFindingBlocking,
                    &scan.receipt_id,
                    &scan.state_root(),
                    self.config.current_height,
                    "wallet scan receipt contains release-blocking findings",
                ));
            }
            if scan.monero_confirmations < self.config.min_monero_confirmations {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WalletConfirmationsTooShallow,
                    &scan.receipt_id,
                    &scan.state_root(),
                    self.config.current_height,
                    "wallet scan receipt has insufficient monero confirmations",
                ));
            }
            if scan.observed_gap_blocks > self.config.max_wallet_scan_gap_blocks {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WalletScanGapTooLarge,
                    &scan.receipt_id,
                    &scan.state_root(),
                    self.config.current_height,
                    "wallet scan receipt exceeds policy scan gap",
                ));
            }
            if !scan.is_fresh_at(
                self.config.current_height,
                self.config.max_evidence_age_blocks,
            ) {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::EvidenceStale,
                    &scan.receipt_id,
                    &scan.state_root(),
                    self.config.current_height,
                    "wallet scan receipt is outside the accepted evidence freshness window",
                ));
            }
        }
        for replay in self.watchtower_replays.values() {
            if replay.status.blocks_release() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WatchtowerReplayFailed,
                    &replay.replay_id,
                    &replay.state_root(),
                    self.config.current_height,
                    "watchtower replay receipt failed or is missing",
                ));
            }
            if !replay.root_matches() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::RootMismatch,
                    &replay.replay_id,
                    &replay.state_root(),
                    self.config.current_height,
                    "watchtower replay observed wallet root differs from expected wallet root",
                ));
            }
            if replay.replay_confirmations < self.config.min_replay_confirmations {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::WatchtowerReplayQuorumMissing,
                    &replay.replay_id,
                    &replay.state_root(),
                    self.config.current_height,
                    "watchtower replay receipt is under-confirmed",
                ));
            }
        }
        if self.replay_quorum_count() < usize::from(self.config.min_watchtower_replays) {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::WatchtowerReplayQuorumMissing,
                &self.release_id,
                &self.watchtower_replay_root(),
                self.config.current_height,
                "distinct watchtower replay quorum is below release policy",
            ));
        }
        if self.config.require_user_escape_action && self.user_escape_actions.is_empty() {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::EscapeActionMissing,
                &self.release_id,
                &self.config.policy_root(),
                self.config.current_height,
                "release policy requires at least one user escape action",
            ));
        }
        for action in self.user_escape_actions.values() {
            if action.blocks_release_at(self.config.current_height) {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::EscapeActionBlocked,
                    &action.action_id,
                    &action.state_root(),
                    self.config.current_height,
                    "user escape action is missing, expired, or blocked",
                ));
            }
        }
        if self.dashboard_approval_quorum_count() < usize::from(self.config.min_dashboard_approvals)
        {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::DashboardApprovalMissing,
                &self.release_id,
                &self.dashboard_approval_root(),
                self.config.current_height,
                "operator dashboard approval quorum is below release policy",
            ));
        }
        for approval in self.dashboard_approvals.values() {
            if approval.status.blocks_release() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::DashboardApprovalRejected,
                    &approval.approval_id,
                    &approval.state_root(),
                    self.config.current_height,
                    "operator dashboard approval is conditional or rejected",
                ));
            }
            if !approval.fresh_at(self.config.current_height) {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::DashboardStale,
                    &approval.approval_id,
                    &approval.state_root(),
                    self.config.current_height,
                    "operator dashboard approval is stale",
                ));
            }
            if self.config.require_dashboard_root_match
                && approval.dashboard_root != self.dashboard_root
            {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::RootMismatch,
                    &approval.approval_id,
                    &approval.state_root(),
                    self.config.current_height,
                    "operator dashboard approval signs a different dashboard root",
                ));
            }
        }
        if self.config.require_dashboard_root_match
            && self.dashboard_root != self.expected_dashboard_root
        {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::RootMismatch,
                &self.release_id,
                &self.dashboard_root,
                self.config.current_height,
                "bound dashboard root does not match expected dashboard root",
            ));
        }
        if self.reviewer_quorum_count() < usize::from(self.config.min_reviewer_quorum) {
            blockers.push(ReleasePolicyBlocker::new(
                BlockerKind::ReviewerQuorumMissing,
                &self.release_id,
                &self.reviewer_root(),
                self.config.current_height,
                "release policy reviewer quorum is below threshold",
            ));
        }
        for reviewer in self.reviewers.values() {
            if reviewer.decision.blocks_release() {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::ReviewerRejected,
                    &reviewer.reviewer_id,
                    &reviewer.state_root(),
                    self.config.current_height,
                    "release policy reviewer rejected the binding",
                ));
            }
            if reviewer.reviewed_dashboard_root != self.dashboard_root {
                blockers.push(ReleasePolicyBlocker::new(
                    BlockerKind::RootMismatch,
                    &reviewer.reviewer_id,
                    &reviewer.state_root(),
                    self.config.current_height,
                    "release policy reviewer signed a different dashboard root",
                ));
            }
        }
        blockers
    }

    pub fn verdict(&self) -> ReleasePolicyVerdict {
        ReleasePolicyVerdict::from_state(self)
    }

    pub fn summary(&self) -> BindingSummary {
        let blockers = self.derive_blockers();
        let verdict = if self.config.fail_closed && !blockers.is_empty() {
            ReleasePolicyVerdictKind::NoGo
        } else {
            ReleasePolicyVerdictKind::Go
        };
        BindingSummary {
            wallets: self.wallet_scans.len(),
            accepted_wallets: self.accepted_wallet_count(),
            replay_receipts: self.watchtower_replays.len(),
            replay_quorum: self.replay_quorum_count(),
            escape_actions: self.user_escape_actions.len(),
            dashboard_approvals: self.dashboard_approval_quorum_count(),
            release_policy_reviewers: self.reviewer_quorum_count(),
            blockers: blockers.len(),
            verdict,
        }
    }

    pub fn public_record(&self) -> Value {
        let blockers = self.derive_blockers();
        let roots = self.roots();
        let verdict = self.verdict();
        json!({
            "chain_id": self.config.chain_id,
            "protocol_version": self.config.protocol_version,
            "release_id": self.release_id,
            "force_exit_id": self.force_exit_id,
            "dashboard_root": self.dashboard_root,
            "expected_dashboard_root": self.expected_dashboard_root,
            "runbook_audit_root": self.runbook_audit_root,
            "dashboard_finalization_root": self.dashboard_finalization_root,
            "config": self.config.public_record(),
            "roots": roots.public_record(),
            "summary": self.summary().public_record(),
            "verdict": verdict.public_record(),
            "wallet_scans": self.wallet_scans.values().map(WalletScanReceipt::public_record).collect::<Vec<_>>(),
            "watchtower_replays": self.watchtower_replays.values().map(WatchtowerReplayReceipt::public_record).collect::<Vec<_>>(),
            "user_escape_actions": self.user_escape_actions.values().map(UserEscapeAction::public_record).collect::<Vec<_>>(),
            "dashboard_approvals": self.dashboard_approvals.values().map(OperatorDashboardApproval::public_record).collect::<Vec<_>>(),
            "reviewers": self.reviewers.values().map(ReleasePolicyReviewer::public_record).collect::<Vec<_>>(),
            "blockers": blockers.iter().map(ReleasePolicyBlocker::public_record).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "WALLET-WATCHTOWER-RELEASE-POLICY-BINDING-STATE",
            &self.public_record(),
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

fn sample_root(label: &str) -> String {
    domain_hash(
        "WALLET-WATCHTOWER-RELEASE-POLICY-BINDING-DEVNET-SAMPLE",
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
