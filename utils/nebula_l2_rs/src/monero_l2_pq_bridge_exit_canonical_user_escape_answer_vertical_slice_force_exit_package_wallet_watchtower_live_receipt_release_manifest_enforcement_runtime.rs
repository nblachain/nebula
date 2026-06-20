use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletWatchtowerLiveReceiptReleaseManifestEnforcementRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-watchtower-live-receipt-release-manifest-enforcement-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ENFORCEMENT_SUITE: &str =
    "monero-l2-pq-force-exit-wallet-watchtower-live-receipt-release-manifest-enforcement-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-devnet-v1";
pub const DEFAULT_FORCE_EXIT_PACKAGE_ID: &str =
    "force-exit-package-wallet-watchtower-live-receipt-release-manifest-devnet-0001";
pub const DEFAULT_RELEASE_MANIFEST_ID: &str =
    "release-manifest-wallet-watchtower-live-receipt-enforcement-devnet-0001";
pub const DEFAULT_RECEIPT_EPOCH: u64 = 79;
pub const DEFAULT_SOURCE_HEIGHT: u64 = 2_791_079;
pub const DEFAULT_L2_HEIGHT: u64 = 909_079;
pub const DEFAULT_MIN_WALLET_SCAN_ROOTS: u64 = 3;
pub const DEFAULT_MIN_WALLET_RECOVERY_ROOTS: u64 = 3;
pub const DEFAULT_MIN_WATCHTOWER_REPLAY_ROOTS: u64 = 4;
pub const DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS: u64 = 3;
pub const DEFAULT_MIN_ACTIVATION_ADJUDICATOR_ROOTS: u64 = 2;
pub const DEFAULT_MIN_HOLD_EXPORT_ROOTS: u64 = 3;
pub const DEFAULT_MIN_MANIFEST_POLICY_ROOTS: u64 = 5;
pub const DEFAULT_MIN_LIVE_RECEIPTS: u64 = 6;
pub const DEFAULT_MIN_WATCHTOWER_QUORUM: u64 = 4;
pub const DEFAULT_WATCHTOWER_OBSERVER_COUNT: u64 = 5;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub enforcement_suite: String,
    pub vertical_slice_id: String,
    pub force_exit_package_id: String,
    pub release_manifest_id: String,
    pub receipt_epoch: u64,
    pub source_height: u64,
    pub l2_height: u64,
    pub min_wallet_scan_roots: u64,
    pub min_wallet_recovery_roots: u64,
    pub min_watchtower_replay_roots: u64,
    pub min_escape_transcript_roots: u64,
    pub min_activation_adjudicator_roots: u64,
    pub min_hold_export_roots: u64,
    pub min_manifest_policy_roots: u64,
    pub min_live_receipts: u64,
    pub min_watchtower_quorum: u64,
    pub watchtower_observer_count: u64,
    pub require_wallet_scan_root: bool,
    pub require_wallet_recovery_root: bool,
    pub require_watchtower_replay_quorum: bool,
    pub require_user_escape_transcript_continuity: bool,
    pub require_activation_adjudicator_root: bool,
    pub require_fail_closed_wallet_hold_exports: bool,
    pub require_manifest_policy_binding: bool,
    pub require_zero_wallet_facing_release: bool,
    pub require_zero_manifest_gaps: bool,
    pub fail_closed_on_missing_root: bool,
    pub fail_closed_on_quorum_gap: bool,
    pub fail_closed_on_transcript_gap: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            enforcement_suite: ENFORCEMENT_SUITE.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            force_exit_package_id: DEFAULT_FORCE_EXIT_PACKAGE_ID.to_string(),
            release_manifest_id: DEFAULT_RELEASE_MANIFEST_ID.to_string(),
            receipt_epoch: DEFAULT_RECEIPT_EPOCH,
            source_height: DEFAULT_SOURCE_HEIGHT,
            l2_height: DEFAULT_L2_HEIGHT,
            min_wallet_scan_roots: DEFAULT_MIN_WALLET_SCAN_ROOTS,
            min_wallet_recovery_roots: DEFAULT_MIN_WALLET_RECOVERY_ROOTS,
            min_watchtower_replay_roots: DEFAULT_MIN_WATCHTOWER_REPLAY_ROOTS,
            min_escape_transcript_roots: DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS,
            min_activation_adjudicator_roots: DEFAULT_MIN_ACTIVATION_ADJUDICATOR_ROOTS,
            min_hold_export_roots: DEFAULT_MIN_HOLD_EXPORT_ROOTS,
            min_manifest_policy_roots: DEFAULT_MIN_MANIFEST_POLICY_ROOTS,
            min_live_receipts: DEFAULT_MIN_LIVE_RECEIPTS,
            min_watchtower_quorum: DEFAULT_MIN_WATCHTOWER_QUORUM,
            watchtower_observer_count: DEFAULT_WATCHTOWER_OBSERVER_COUNT,
            require_wallet_scan_root: true,
            require_wallet_recovery_root: true,
            require_watchtower_replay_quorum: true,
            require_user_escape_transcript_continuity: true,
            require_activation_adjudicator_root: true,
            require_fail_closed_wallet_hold_exports: true,
            require_manifest_policy_binding: true,
            require_zero_wallet_facing_release: true,
            require_zero_manifest_gaps: true,
            fail_closed_on_missing_root: true,
            fail_closed_on_quorum_gap: true,
            fail_closed_on_transcript_gap: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "enforcement_suite": self.enforcement_suite,
            "vertical_slice_id": self.vertical_slice_id,
            "force_exit_package_id": self.force_exit_package_id,
            "release_manifest_id": self.release_manifest_id,
            "receipt_epoch": self.receipt_epoch,
            "source_height": self.source_height,
            "l2_height": self.l2_height,
            "min_wallet_scan_roots": self.min_wallet_scan_roots,
            "min_wallet_recovery_roots": self.min_wallet_recovery_roots,
            "min_watchtower_replay_roots": self.min_watchtower_replay_roots,
            "min_escape_transcript_roots": self.min_escape_transcript_roots,
            "min_activation_adjudicator_roots": self.min_activation_adjudicator_roots,
            "min_hold_export_roots": self.min_hold_export_roots,
            "min_manifest_policy_roots": self.min_manifest_policy_roots,
            "min_live_receipts": self.min_live_receipts,
            "min_watchtower_quorum": self.min_watchtower_quorum,
            "watchtower_observer_count": self.watchtower_observer_count,
            "require_wallet_scan_root": self.require_wallet_scan_root,
            "require_wallet_recovery_root": self.require_wallet_recovery_root,
            "require_watchtower_replay_quorum": self.require_watchtower_replay_quorum,
            "require_user_escape_transcript_continuity": self.require_user_escape_transcript_continuity,
            "require_activation_adjudicator_root": self.require_activation_adjudicator_root,
            "require_fail_closed_wallet_hold_exports": self.require_fail_closed_wallet_hold_exports,
            "require_manifest_policy_binding": self.require_manifest_policy_binding,
            "require_zero_wallet_facing_release": self.require_zero_wallet_facing_release,
            "require_zero_manifest_gaps": self.require_zero_manifest_gaps,
            "fail_closed_on_missing_root": self.fail_closed_on_missing_root,
            "fail_closed_on_quorum_gap": self.fail_closed_on_quorum_gap,
            "fail_closed_on_transcript_gap": self.fail_closed_on_transcript_gap,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementDomain {
    WalletScan,
    WalletRecovery,
    WatchtowerReplay,
    UserEscapeTranscript,
    ActivationAdjudicator,
    WalletHoldExport,
    ManifestPolicy,
    LiveReceipt,
}

impl EnforcementDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletScan => "wallet_scan",
            Self::WalletRecovery => "wallet_recovery",
            Self::WatchtowerReplay => "watchtower_replay",
            Self::UserEscapeTranscript => "user_escape_transcript",
            Self::ActivationAdjudicator => "activation_adjudicator",
            Self::WalletHoldExport => "wallet_hold_export",
            Self::ManifestPolicy => "manifest_policy",
            Self::LiveReceipt => "live_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementStatus {
    Satisfied,
    Waived,
    Missing,
    Mismatch,
    Insufficient,
}

impl RequirementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Waived => "waived",
            Self::Missing => "missing",
            Self::Mismatch => "mismatch",
            Self::Insufficient => "insufficient",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Satisfied | Self::Waived)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseManifestStatus {
    Enforced,
    HeldForWallet,
    HeldForWatchtower,
    HeldForTranscript,
    HeldForAdjudicator,
    HeldForManifestGap,
}

impl ReleaseManifestStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::HeldForWallet => "held_for_wallet",
            Self::HeldForWatchtower => "held_for_watchtower",
            Self::HeldForTranscript => "held_for_transcript",
            Self::HeldForAdjudicator => "held_for_adjudicator",
            Self::HeldForManifestGap => "held_for_manifest_gap",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletHoldExportStatus {
    FailClosed,
    Open,
    Suppressed,
}

impl WalletHoldExportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Open => "open",
            Self::Suppressed => "suppressed",
        }
    }

    pub fn is_fail_closed(self) -> bool {
        matches!(self, Self::FailClosed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationDecision {
    ActivateReleaseManifest,
    HoldWalletFacingRelease,
}

impl ActivationDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ActivateReleaseManifest => "activate_release_manifest",
            Self::HoldWalletFacingRelease => "hold_wallet_facing_release",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RequirementRecord {
    pub domain: EnforcementDomain,
    pub required: bool,
    pub observed_root: String,
    pub expected_root: String,
    pub evidence_root: String,
    pub status: RequirementStatus,
    pub record_id: String,
}

impl RequirementRecord {
    pub fn new(
        domain: EnforcementDomain,
        required: bool,
        observed_root: String,
        expected_root: String,
        evidence_root: String,
        status: RequirementStatus,
    ) -> Self {
        let record_id = deterministic_id(
            "requirement-record",
            &[
                domain.as_str(),
                status.as_str(),
                &observed_root,
                &expected_root,
            ],
        );
        Self {
            domain,
            required,
            observed_root,
            expected_root,
            evidence_root,
            status,
            record_id,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "domain": self.domain.as_str(),
            "required": self.required,
            "observed_root": self.observed_root,
            "expected_root": self.expected_root,
            "evidence_root": self.evidence_root,
            "status": self.status.as_str(),
            "record_id": self.record_id,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("requirement-record", &self.public_record())
    }

    pub fn permits_release(&self) -> bool {
        self.status.permits_release()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerReplayQuorum {
    pub replay_root: String,
    pub observer_set_root: String,
    pub quorum_certificate_root: String,
    pub accepted_observer_count: u64,
    pub required_observer_count: u64,
    pub rejected_observer_count: u64,
    pub replay_gap_count: u64,
    pub quorum_reached: bool,
}

impl WatchtowerReplayQuorum {
    pub fn public_record(&self) -> Value {
        json!({
            "replay_root": self.replay_root,
            "observer_set_root": self.observer_set_root,
            "quorum_certificate_root": self.quorum_certificate_root,
            "accepted_observer_count": self.accepted_observer_count,
            "required_observer_count": self.required_observer_count,
            "rejected_observer_count": self.rejected_observer_count,
            "replay_gap_count": self.replay_gap_count,
            "quorum_reached": self.quorum_reached,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("watchtower-replay-quorum", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserEscapeTranscriptContinuity {
    pub transcript_root: String,
    pub previous_transcript_root: String,
    pub next_transcript_root: String,
    pub continuity_proof_root: String,
    pub first_escape_event_id: String,
    pub last_escape_event_id: String,
    pub transcript_gap_count: u64,
    pub replayed_event_count: u64,
    pub continuous: bool,
}

impl UserEscapeTranscriptContinuity {
    pub fn public_record(&self) -> Value {
        json!({
            "transcript_root": self.transcript_root,
            "previous_transcript_root": self.previous_transcript_root,
            "next_transcript_root": self.next_transcript_root,
            "continuity_proof_root": self.continuity_proof_root,
            "first_escape_event_id": self.first_escape_event_id,
            "last_escape_event_id": self.last_escape_event_id,
            "transcript_gap_count": self.transcript_gap_count,
            "replayed_event_count": self.replayed_event_count,
            "continuous": self.continuous,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("user-escape-transcript-continuity", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletFacingHoldExport {
    pub export_root: String,
    pub wallet_view_root: String,
    pub hold_reason_root: String,
    pub release_suppression_root: String,
    pub status: WalletHoldExportStatus,
    pub wallet_release_visible_count: u64,
    pub fail_closed_export_count: u64,
}

impl WalletFacingHoldExport {
    pub fn public_record(&self) -> Value {
        json!({
            "export_root": self.export_root,
            "wallet_view_root": self.wallet_view_root,
            "hold_reason_root": self.hold_reason_root,
            "release_suppression_root": self.release_suppression_root,
            "status": self.status.as_str(),
            "wallet_release_visible_count": self.wallet_release_visible_count,
            "fail_closed_export_count": self.fail_closed_export_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wallet-facing-hold-export", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationAdjudicator {
    pub adjudicator_root: String,
    pub activation_receipt_root: String,
    pub dispute_resolution_root: String,
    pub policy_decision_root: String,
    pub decision: ActivationDecision,
    pub accepted_decision_count: u64,
    pub rejected_decision_count: u64,
}

impl ActivationAdjudicator {
    pub fn public_record(&self) -> Value {
        json!({
            "adjudicator_root": self.adjudicator_root,
            "activation_receipt_root": self.activation_receipt_root,
            "dispute_resolution_root": self.dispute_resolution_root,
            "policy_decision_root": self.policy_decision_root,
            "decision": self.decision.as_str(),
            "accepted_decision_count": self.accepted_decision_count,
            "rejected_decision_count": self.rejected_decision_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("activation-adjudicator", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub wallet_scan_root: String,
    pub wallet_recovery_root: String,
    pub watchtower_replay_root: String,
    pub watchtower_quorum_root: String,
    pub user_escape_transcript_root: String,
    pub transcript_continuity_root: String,
    pub activation_adjudicator_root: String,
    pub wallet_hold_export_root: String,
    pub manifest_policy_root: String,
    pub live_receipt_root: String,
    pub requirement_record_root: String,
    pub enforcement_record_root: String,
    pub release_manifest_decision_root: String,
    pub fail_closed_export_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_root": self.wallet_scan_root,
            "wallet_recovery_root": self.wallet_recovery_root,
            "watchtower_replay_root": self.watchtower_replay_root,
            "watchtower_quorum_root": self.watchtower_quorum_root,
            "user_escape_transcript_root": self.user_escape_transcript_root,
            "transcript_continuity_root": self.transcript_continuity_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "wallet_hold_export_root": self.wallet_hold_export_root,
            "manifest_policy_root": self.manifest_policy_root,
            "live_receipt_root": self.live_receipt_root,
            "requirement_record_root": self.requirement_record_root,
            "enforcement_record_root": self.enforcement_record_root,
            "release_manifest_decision_root": self.release_manifest_decision_root,
            "fail_closed_export_root": self.fail_closed_export_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub wallet_scan_root_count: u64,
    pub wallet_recovery_root_count: u64,
    pub watchtower_replay_root_count: u64,
    pub escape_transcript_root_count: u64,
    pub activation_adjudicator_root_count: u64,
    pub hold_export_root_count: u64,
    pub manifest_policy_root_count: u64,
    pub live_receipt_count: u64,
    pub accepted_requirement_count: u64,
    pub missing_root_count: u64,
    pub mismatch_root_count: u64,
    pub watchtower_quorum_gap_count: u64,
    pub transcript_gap_count: u64,
    pub wallet_facing_release_count: u64,
    pub manifest_gap_count: u64,
    pub fail_closed_export_count: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_root_count": self.wallet_scan_root_count,
            "wallet_recovery_root_count": self.wallet_recovery_root_count,
            "watchtower_replay_root_count": self.watchtower_replay_root_count,
            "escape_transcript_root_count": self.escape_transcript_root_count,
            "activation_adjudicator_root_count": self.activation_adjudicator_root_count,
            "hold_export_root_count": self.hold_export_root_count,
            "manifest_policy_root_count": self.manifest_policy_root_count,
            "live_receipt_count": self.live_receipt_count,
            "accepted_requirement_count": self.accepted_requirement_count,
            "missing_root_count": self.missing_root_count,
            "mismatch_root_count": self.mismatch_root_count,
            "watchtower_quorum_gap_count": self.watchtower_quorum_gap_count,
            "transcript_gap_count": self.transcript_gap_count,
            "wallet_facing_release_count": self.wallet_facing_release_count,
            "manifest_gap_count": self.manifest_gap_count,
            "fail_closed_export_count": self.fail_closed_export_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementVerdict {
    pub wallet_scan_enforced: bool,
    pub wallet_recovery_enforced: bool,
    pub watchtower_replay_quorum_enforced: bool,
    pub user_escape_transcript_continuous: bool,
    pub activation_adjudicator_enforced: bool,
    pub wallet_hold_exports_fail_closed: bool,
    pub manifest_policy_bound: bool,
    pub live_receipts_sufficient: bool,
    pub missing_roots_clear: bool,
    pub manifest_gaps_clear: bool,
    pub release_manifest_status: ReleaseManifestStatus,
    pub activation_decision: ActivationDecision,
    pub release_manifest_activation_allowed: bool,
    pub wallet_facing_release_allowed: bool,
    pub fail_closed_engaged: bool,
    pub production_answer: String,
    pub user_escape_answer: String,
    pub verdict_root: String,
}

impl EnforcementVerdict {
    pub fn new(
        config: &Config,
        roots: &Roots,
        counters: &Counters,
        watchtower: &WatchtowerReplayQuorum,
        transcript: &UserEscapeTranscriptContinuity,
        hold_export: &WalletFacingHoldExport,
        adjudicator: &ActivationAdjudicator,
    ) -> Self {
        let wallet_scan_enforced = !config.require_wallet_scan_root
            || counters.wallet_scan_root_count >= config.min_wallet_scan_roots;
        let wallet_recovery_enforced = !config.require_wallet_recovery_root
            || counters.wallet_recovery_root_count >= config.min_wallet_recovery_roots;
        let watchtower_replay_quorum_enforced = !config.require_watchtower_replay_quorum
            || (watchtower.quorum_reached
                && counters.watchtower_replay_root_count >= config.min_watchtower_replay_roots
                && counters.watchtower_quorum_gap_count == 0);
        let user_escape_transcript_continuous = !config.require_user_escape_transcript_continuity
            || (transcript.continuous
                && counters.escape_transcript_root_count >= config.min_escape_transcript_roots
                && counters.transcript_gap_count == 0);
        let activation_adjudicator_enforced = !config.require_activation_adjudicator_root
            || (counters.activation_adjudicator_root_count
                >= config.min_activation_adjudicator_roots
                && matches!(
                    adjudicator.decision,
                    ActivationDecision::ActivateReleaseManifest
                ));
        let wallet_hold_exports_fail_closed = !config.require_fail_closed_wallet_hold_exports
            || (hold_export.status.is_fail_closed()
                && counters.hold_export_root_count >= config.min_hold_export_roots
                && counters.fail_closed_export_count >= config.min_hold_export_roots);
        let manifest_policy_bound = !config.require_manifest_policy_binding
            || counters.manifest_policy_root_count >= config.min_manifest_policy_roots;
        let live_receipts_sufficient = counters.live_receipt_count >= config.min_live_receipts;
        let missing_roots_clear =
            counters.missing_root_count == 0 && counters.mismatch_root_count == 0;
        let manifest_gaps_clear =
            !config.require_zero_manifest_gaps || counters.manifest_gap_count == 0;
        let wallet_release_clear =
            !config.require_zero_wallet_facing_release || counters.wallet_facing_release_count == 0;

        let fail_closed_engaged = (config.fail_closed_on_missing_root && !missing_roots_clear)
            || (config.fail_closed_on_quorum_gap && !watchtower_replay_quorum_enforced)
            || (config.fail_closed_on_transcript_gap && !user_escape_transcript_continuous)
            || !wallet_hold_exports_fail_closed
            || !wallet_release_clear;

        let release_manifest_status = if wallet_scan_enforced
            && wallet_recovery_enforced
            && watchtower_replay_quorum_enforced
            && user_escape_transcript_continuous
            && activation_adjudicator_enforced
            && wallet_hold_exports_fail_closed
            && manifest_policy_bound
            && live_receipts_sufficient
            && missing_roots_clear
            && manifest_gaps_clear
            && wallet_release_clear
        {
            ReleaseManifestStatus::Enforced
        } else if !wallet_scan_enforced
            || !wallet_recovery_enforced
            || !wallet_hold_exports_fail_closed
        {
            ReleaseManifestStatus::HeldForWallet
        } else if !watchtower_replay_quorum_enforced {
            ReleaseManifestStatus::HeldForWatchtower
        } else if !user_escape_transcript_continuous {
            ReleaseManifestStatus::HeldForTranscript
        } else if !activation_adjudicator_enforced {
            ReleaseManifestStatus::HeldForAdjudicator
        } else {
            ReleaseManifestStatus::HeldForManifestGap
        };

        let release_manifest_activation_allowed =
            matches!(release_manifest_status, ReleaseManifestStatus::Enforced)
                && !fail_closed_engaged;
        let wallet_facing_release_allowed =
            release_manifest_activation_allowed && !hold_export.status.is_fail_closed();
        let activation_decision = if release_manifest_activation_allowed {
            ActivationDecision::ActivateReleaseManifest
        } else {
            ActivationDecision::HoldWalletFacingRelease
        };
        let production_answer = if release_manifest_activation_allowed {
            "release-manifest-policy-may-activate-after-wallet-watchtower-live-receipt-enforcement"
                .to_string()
        } else {
            "release-manifest-policy-held-fail-closed-for-wallet-facing-exports".to_string()
        };
        let user_escape_answer = if release_manifest_activation_allowed {
            "user-escape-continuity-watchtower-quorum-and-wallet-recovery-roots-accepted"
                .to_string()
        } else {
            "user-escape-answer-remains-held-until-wallet-watchtower-roots-and-adjudicator-clear"
                .to_string()
        };
        let verdict_root = enforcement_verdict_root(
            config,
            roots,
            counters,
            release_manifest_status,
            activation_decision,
            fail_closed_engaged,
        );

        Self {
            wallet_scan_enforced,
            wallet_recovery_enforced,
            watchtower_replay_quorum_enforced,
            user_escape_transcript_continuous,
            activation_adjudicator_enforced,
            wallet_hold_exports_fail_closed,
            manifest_policy_bound,
            live_receipts_sufficient,
            missing_roots_clear,
            manifest_gaps_clear,
            release_manifest_status,
            activation_decision,
            release_manifest_activation_allowed,
            wallet_facing_release_allowed,
            fail_closed_engaged,
            production_answer,
            user_escape_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_enforced": self.wallet_scan_enforced,
            "wallet_recovery_enforced": self.wallet_recovery_enforced,
            "watchtower_replay_quorum_enforced": self.watchtower_replay_quorum_enforced,
            "user_escape_transcript_continuous": self.user_escape_transcript_continuous,
            "activation_adjudicator_enforced": self.activation_adjudicator_enforced,
            "wallet_hold_exports_fail_closed": self.wallet_hold_exports_fail_closed,
            "manifest_policy_bound": self.manifest_policy_bound,
            "live_receipts_sufficient": self.live_receipts_sufficient,
            "missing_roots_clear": self.missing_roots_clear,
            "manifest_gaps_clear": self.manifest_gaps_clear,
            "release_manifest_status": self.release_manifest_status.as_str(),
            "activation_decision": self.activation_decision.as_str(),
            "release_manifest_activation_allowed": self.release_manifest_activation_allowed,
            "wallet_facing_release_allowed": self.wallet_facing_release_allowed,
            "fail_closed_engaged": self.fail_closed_engaged,
            "production_answer": self.production_answer,
            "user_escape_answer": self.user_escape_answer,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementRecord {
    pub enforcement_id: String,
    pub release_manifest_id: String,
    pub force_exit_package_id: String,
    pub wallet_scan_root: String,
    pub wallet_recovery_root: String,
    pub watchtower_quorum_root: String,
    pub transcript_continuity_root: String,
    pub activation_adjudicator_root: String,
    pub wallet_hold_export_root: String,
    pub verdict_root: String,
    pub status: ReleaseManifestStatus,
}

impl EnforcementRecord {
    pub fn from_parts(config: &Config, roots: &Roots, verdict: &EnforcementVerdict) -> Self {
        let enforcement_id = deterministic_id(
            "release-manifest-enforcement",
            &[
                &config.release_manifest_id,
                &roots.wallet_scan_root,
                &roots.watchtower_quorum_root,
                &verdict.verdict_root,
            ],
        );
        Self {
            enforcement_id,
            release_manifest_id: config.release_manifest_id.clone(),
            force_exit_package_id: config.force_exit_package_id.clone(),
            wallet_scan_root: roots.wallet_scan_root.clone(),
            wallet_recovery_root: roots.wallet_recovery_root.clone(),
            watchtower_quorum_root: roots.watchtower_quorum_root.clone(),
            transcript_continuity_root: roots.transcript_continuity_root.clone(),
            activation_adjudicator_root: roots.activation_adjudicator_root.clone(),
            wallet_hold_export_root: roots.wallet_hold_export_root.clone(),
            verdict_root: verdict.verdict_root.clone(),
            status: verdict.release_manifest_status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "enforcement_id": self.enforcement_id,
            "release_manifest_id": self.release_manifest_id,
            "force_exit_package_id": self.force_exit_package_id,
            "wallet_scan_root": self.wallet_scan_root,
            "wallet_recovery_root": self.wallet_recovery_root,
            "watchtower_quorum_root": self.watchtower_quorum_root,
            "transcript_continuity_root": self.transcript_continuity_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "wallet_hold_export_root": self.wallet_hold_export_root,
            "verdict_root": self.verdict_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: Roots,
    pub counters: Counters,
    pub requirements: Vec<RequirementRecord>,
    pub watchtower_quorum: WatchtowerReplayQuorum,
    pub transcript_continuity: UserEscapeTranscriptContinuity,
    pub wallet_hold_export: WalletFacingHoldExport,
    pub activation_adjudicator: ActivationAdjudicator,
    pub enforcement_record: EnforcementRecord,
    pub verdict: EnforcementVerdict,
}

impl State {
    pub fn new(
        config: Config,
        mut roots: Roots,
        counters: Counters,
        requirements: Vec<RequirementRecord>,
        watchtower_quorum: WatchtowerReplayQuorum,
        transcript_continuity: UserEscapeTranscriptContinuity,
        wallet_hold_export: WalletFacingHoldExport,
        activation_adjudicator: ActivationAdjudicator,
    ) -> Result<Self> {
        validate_config(&config)?;
        validate_counters(&config, &counters)?;
        let requirement_values = requirements
            .iter()
            .map(RequirementRecord::public_record)
            .collect::<Vec<_>>();
        roots.requirement_record_root = merkle_root(
            "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-REQUIREMENTS",
            &requirement_values,
        );
        roots.watchtower_quorum_root = watchtower_quorum.state_root();
        roots.transcript_continuity_root = transcript_continuity.state_root();
        roots.wallet_hold_export_root = wallet_hold_export.state_root();
        roots.activation_adjudicator_root = activation_adjudicator.state_root();

        let verdict = EnforcementVerdict::new(
            &config,
            &roots,
            &counters,
            &watchtower_quorum,
            &transcript_continuity,
            &wallet_hold_export,
            &activation_adjudicator,
        );
        let enforcement_record = EnforcementRecord::from_parts(&config, &roots, &verdict);
        roots.enforcement_record_root = enforcement_record.state_root();
        roots.release_manifest_decision_root = record_root(
            "release-manifest-decision",
            &json!({
                "release_manifest_id": config.release_manifest_id,
                "activation_decision": verdict.activation_decision.as_str(),
                "release_manifest_status": verdict.release_manifest_status.as_str(),
                "release_manifest_activation_allowed": verdict.release_manifest_activation_allowed,
                "wallet_facing_release_allowed": verdict.wallet_facing_release_allowed,
                "fail_closed_engaged": verdict.fail_closed_engaged,
            }),
        );
        roots.fail_closed_export_root = record_root(
            "fail-closed-wallet-facing-hold-export",
            &json!({
                "wallet_hold_export_root": roots.wallet_hold_export_root,
                "hold_export_root_count": counters.hold_export_root_count,
                "fail_closed_export_count": counters.fail_closed_export_count,
                "wallet_facing_release_count": counters.wallet_facing_release_count,
                "wallet_hold_export_status": wallet_hold_export.status.as_str(),
            }),
        );
        roots.state_commitment_root =
            state_commitment_root(&config, &roots, &counters, &verdict, &enforcement_record);

        Ok(Self {
            config,
            roots,
            counters,
            requirements,
            watchtower_quorum,
            transcript_continuity,
            wallet_hold_export,
            activation_adjudicator,
            enforcement_record,
            verdict,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        let requirements = self
            .requirements
            .iter()
            .map(RequirementRecord::public_record)
            .collect::<Vec<_>>();
        json!({
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "requirements": requirements,
            "watchtower_quorum": self.watchtower_quorum.public_record(),
            "transcript_continuity": self.transcript_continuity.public_record(),
            "wallet_hold_export": self.wallet_hold_export.public_record(),
            "activation_adjudicator": self.activation_adjudicator.public_record(),
            "enforcement_record": self.enforcement_record.public_record(),
            "verdict": self.verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.roots.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&self.enforcement_record.state_root()),
                HashPart::Str(&self.verdict.state_root()),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let counters = Counters {
        wallet_scan_root_count: DEFAULT_MIN_WALLET_SCAN_ROOTS,
        wallet_recovery_root_count: DEFAULT_MIN_WALLET_RECOVERY_ROOTS,
        watchtower_replay_root_count: DEFAULT_MIN_WATCHTOWER_REPLAY_ROOTS,
        escape_transcript_root_count: DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS,
        activation_adjudicator_root_count: DEFAULT_MIN_ACTIVATION_ADJUDICATOR_ROOTS,
        hold_export_root_count: DEFAULT_MIN_HOLD_EXPORT_ROOTS,
        manifest_policy_root_count: DEFAULT_MIN_MANIFEST_POLICY_ROOTS,
        live_receipt_count: DEFAULT_MIN_LIVE_RECEIPTS,
        accepted_requirement_count: 8,
        missing_root_count: 0,
        mismatch_root_count: 0,
        watchtower_quorum_gap_count: 0,
        transcript_gap_count: 0,
        wallet_facing_release_count: 0,
        manifest_gap_count: 0,
        fail_closed_export_count: DEFAULT_MIN_HOLD_EXPORT_ROOTS,
    };

    let wallet_scan_root = vector_root("wallet-scan-roots", DEFAULT_MIN_WALLET_SCAN_ROOTS);
    let wallet_recovery_root =
        vector_root("wallet-recovery-roots", DEFAULT_MIN_WALLET_RECOVERY_ROOTS);
    let watchtower_replay_root = vector_root(
        "watchtower-replay-roots",
        DEFAULT_MIN_WATCHTOWER_REPLAY_ROOTS,
    );
    let user_escape_transcript_root = vector_root(
        "user-escape-transcript-roots",
        DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS,
    );
    let manifest_policy_root =
        vector_root("manifest-policy-roots", DEFAULT_MIN_MANIFEST_POLICY_ROOTS);
    let live_receipt_root = vector_root("live-receipt-roots", DEFAULT_MIN_LIVE_RECEIPTS);

    let watchtower_quorum = WatchtowerReplayQuorum {
        replay_root: watchtower_replay_root.clone(),
        observer_set_root: vector_root(
            "watchtower-observer-set",
            DEFAULT_WATCHTOWER_OBSERVER_COUNT,
        ),
        quorum_certificate_root: vector_root(
            "watchtower-quorum-certificates",
            DEFAULT_MIN_WATCHTOWER_QUORUM,
        ),
        accepted_observer_count: DEFAULT_MIN_WATCHTOWER_QUORUM,
        required_observer_count: DEFAULT_MIN_WATCHTOWER_QUORUM,
        rejected_observer_count: 0,
        replay_gap_count: 0,
        quorum_reached: true,
    };

    let transcript_continuity = UserEscapeTranscriptContinuity {
        transcript_root: user_escape_transcript_root.clone(),
        previous_transcript_root: deterministic_root("previous-user-escape-transcript"),
        next_transcript_root: deterministic_root("next-user-escape-transcript"),
        continuity_proof_root: vector_root(
            "user-escape-continuity-proofs",
            DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS,
        ),
        first_escape_event_id: deterministic_id(
            "escape-event",
            &["first", DEFAULT_VERTICAL_SLICE_ID],
        ),
        last_escape_event_id: deterministic_id(
            "escape-event",
            &["last", DEFAULT_VERTICAL_SLICE_ID],
        ),
        transcript_gap_count: 0,
        replayed_event_count: DEFAULT_MIN_ESCAPE_TRANSCRIPT_ROOTS,
        continuous: true,
    };

    let wallet_hold_export = WalletFacingHoldExport {
        export_root: vector_root("wallet-facing-hold-exports", DEFAULT_MIN_HOLD_EXPORT_ROOTS),
        wallet_view_root: deterministic_root("wallet-view-fail-closed"),
        hold_reason_root: deterministic_root("wallet-hold-reason-release-manifest-policy"),
        release_suppression_root: deterministic_root("wallet-release-suppression"),
        status: WalletHoldExportStatus::FailClosed,
        wallet_release_visible_count: 0,
        fail_closed_export_count: DEFAULT_MIN_HOLD_EXPORT_ROOTS,
    };

    let activation_adjudicator = ActivationAdjudicator {
        adjudicator_root: vector_root(
            "activation-adjudicator-roots",
            DEFAULT_MIN_ACTIVATION_ADJUDICATOR_ROOTS,
        ),
        activation_receipt_root: deterministic_root("activation-receipt-release-manifest"),
        dispute_resolution_root: deterministic_root("activation-adjudicator-dispute-resolution"),
        policy_decision_root: deterministic_root("activation-adjudicator-policy-decision"),
        decision: ActivationDecision::ActivateReleaseManifest,
        accepted_decision_count: DEFAULT_MIN_ACTIVATION_ADJUDICATOR_ROOTS,
        rejected_decision_count: 0,
    };

    let requirements = vec![
        RequirementRecord::new(
            EnforcementDomain::WalletScan,
            true,
            wallet_scan_root.clone(),
            wallet_scan_root.clone(),
            deterministic_root("wallet-scan-evidence"),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::WalletRecovery,
            true,
            wallet_recovery_root.clone(),
            wallet_recovery_root.clone(),
            deterministic_root("wallet-recovery-evidence"),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::WatchtowerReplay,
            true,
            watchtower_replay_root.clone(),
            watchtower_replay_root.clone(),
            watchtower_quorum.quorum_certificate_root.clone(),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::UserEscapeTranscript,
            true,
            user_escape_transcript_root.clone(),
            user_escape_transcript_root.clone(),
            transcript_continuity.continuity_proof_root.clone(),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::ActivationAdjudicator,
            true,
            activation_adjudicator.adjudicator_root.clone(),
            activation_adjudicator.adjudicator_root.clone(),
            activation_adjudicator.policy_decision_root.clone(),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::WalletHoldExport,
            true,
            wallet_hold_export.export_root.clone(),
            wallet_hold_export.export_root.clone(),
            wallet_hold_export.release_suppression_root.clone(),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::ManifestPolicy,
            true,
            manifest_policy_root.clone(),
            manifest_policy_root.clone(),
            deterministic_root("release-manifest-policy-binding"),
            RequirementStatus::Satisfied,
        ),
        RequirementRecord::new(
            EnforcementDomain::LiveReceipt,
            true,
            live_receipt_root.clone(),
            live_receipt_root.clone(),
            deterministic_root("wallet-watchtower-live-receipt-activation"),
            RequirementStatus::Satisfied,
        ),
    ];

    let roots = Roots {
        wallet_scan_root,
        wallet_recovery_root,
        watchtower_replay_root,
        watchtower_quorum_root: String::new(),
        user_escape_transcript_root,
        transcript_continuity_root: String::new(),
        activation_adjudicator_root: String::new(),
        wallet_hold_export_root: String::new(),
        manifest_policy_root,
        live_receipt_root,
        requirement_record_root: String::new(),
        enforcement_record_root: String::new(),
        release_manifest_decision_root: String::new(),
        fail_closed_export_root: String::new(),
        state_commitment_root: String::new(),
    };

    if let Ok(state) = State::new(
        config,
        roots,
        counters,
        requirements,
        watchtower_quorum,
        transcript_continuity,
        wallet_hold_export,
        activation_adjudicator,
    ) {
        state
    } else {
        devnet_fail_closed_state()
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

pub fn deterministic_id(kind: &str, parts: &[&str]) -> String {
    let leaves = parts
        .iter()
        .enumerate()
        .map(|(ordinal, part)| {
            json!({
                "chain_id": CHAIN_ID,
                "kind": kind,
                "ordinal": ordinal as u64,
                "part": part,
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-ID",
        &[
            HashPart::Str(kind),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-ID-PARTS",
                &leaves,
            )),
        ],
        16,
    )
}

fn deterministic_root(kind: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-DETERMINISTIC-ROOT",
        &[HashPart::Str(CHAIN_ID), HashPart::Str(PROTOCOL_VERSION), HashPart::Str(kind)],
        32,
    )
}

fn vector_root(kind: &str, count: u64) -> String {
    let leaves = (0..count)
        .map(|ordinal| {
            json!({
                "chain_id": CHAIN_ID,
                "protocol_version": PROTOCOL_VERSION,
                "kind": kind,
                "ordinal": ordinal,
                "root": domain_hash(
                    "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-VECTOR-LEAF",
                    &[HashPart::Str(kind), HashPart::U64(ordinal)],
                    32,
                ),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-VECTOR",
        &leaves,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("chain_id must match crate CHAIN_ID".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err(
            "protocol_version must match wallet-watchtower enforcement runtime".to_string(),
        );
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("schema_version must be 1".to_string());
    }
    if config.min_watchtower_quorum == 0 {
        return Err("min_watchtower_quorum must be non-zero".to_string());
    }
    if config.watchtower_observer_count < config.min_watchtower_quorum {
        return Err("watchtower_observer_count must cover min_watchtower_quorum".to_string());
    }
    if config.min_live_receipts == 0 || config.min_manifest_policy_roots == 0 {
        return Err("manifest policy and live receipt thresholds must be non-zero".to_string());
    }
    Ok(())
}

fn validate_counters(config: &Config, counters: &Counters) -> Result<()> {
    if counters.live_receipt_count < config.min_live_receipts {
        return Err("live receipt count is below release manifest threshold".to_string());
    }
    if counters.accepted_requirement_count == 0 {
        return Err("accepted requirement count must be non-zero".to_string());
    }
    if counters.fail_closed_export_count > counters.hold_export_root_count {
        return Err("fail closed export count cannot exceed hold export roots".to_string());
    }
    Ok(())
}

fn enforcement_verdict_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    status: ReleaseManifestStatus,
    decision: ActivationDecision,
    fail_closed_engaged: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-VERDICT",
        &[
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(&roots.wallet_scan_root),
            HashPart::Str(&roots.wallet_recovery_root),
            HashPart::Str(&roots.watchtower_quorum_root),
            HashPart::Str(&roots.transcript_continuity_root),
            HashPart::Str(&roots.activation_adjudicator_root),
            HashPart::Str(&roots.wallet_hold_export_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(decision.as_str()),
            HashPart::U64(counters.missing_root_count),
            HashPart::U64(counters.manifest_gap_count),
            HashPart::Str(if fail_closed_engaged {
                "fail-closed"
            } else {
                "clear"
            }),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    verdict: &EnforcementVerdict,
    enforcement_record: &EnforcementRecord,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-WALLET-WATCHTOWER-LIVE-RECEIPT-RELEASE-MANIFEST-ENFORCEMENT-COMMITMENT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(&verdict.state_root()),
            HashPart::Str(&enforcement_record.state_root()),
        ],
        32,
    )
}

fn devnet_fail_closed_state() -> State {
    let config = Config::default();
    let mut roots = Roots {
        wallet_scan_root: deterministic_root("fail-closed-wallet-scan-root"),
        wallet_recovery_root: deterministic_root("fail-closed-wallet-recovery-root"),
        watchtower_replay_root: deterministic_root("fail-closed-watchtower-replay-root"),
        watchtower_quorum_root: deterministic_root("fail-closed-watchtower-quorum-root"),
        user_escape_transcript_root: deterministic_root("fail-closed-user-escape-transcript-root"),
        transcript_continuity_root: deterministic_root("fail-closed-transcript-continuity-root"),
        activation_adjudicator_root: deterministic_root("fail-closed-activation-adjudicator-root"),
        wallet_hold_export_root: deterministic_root("fail-closed-wallet-hold-export-root"),
        manifest_policy_root: deterministic_root("fail-closed-manifest-policy-root"),
        live_receipt_root: deterministic_root("fail-closed-live-receipt-root"),
        requirement_record_root: deterministic_root("fail-closed-requirement-record-root"),
        enforcement_record_root: String::new(),
        release_manifest_decision_root: deterministic_root(
            "fail-closed-release-manifest-decision-root",
        ),
        fail_closed_export_root: deterministic_root("fail-closed-export-root"),
        state_commitment_root: String::new(),
    };
    let counters = Counters {
        wallet_scan_root_count: config.min_wallet_scan_roots,
        wallet_recovery_root_count: config.min_wallet_recovery_roots,
        watchtower_replay_root_count: config.min_watchtower_replay_roots,
        escape_transcript_root_count: config.min_escape_transcript_roots,
        activation_adjudicator_root_count: config.min_activation_adjudicator_roots,
        hold_export_root_count: config.min_hold_export_roots,
        manifest_policy_root_count: config.min_manifest_policy_roots,
        live_receipt_count: config.min_live_receipts,
        accepted_requirement_count: 1,
        missing_root_count: 1,
        mismatch_root_count: 0,
        watchtower_quorum_gap_count: 1,
        transcript_gap_count: 1,
        wallet_facing_release_count: 0,
        manifest_gap_count: 1,
        fail_closed_export_count: config.min_hold_export_roots,
    };
    let watchtower_quorum = WatchtowerReplayQuorum {
        replay_root: roots.watchtower_replay_root.clone(),
        observer_set_root: deterministic_root("fail-closed-observer-set-root"),
        quorum_certificate_root: deterministic_root("fail-closed-quorum-certificate-root"),
        accepted_observer_count: config.min_watchtower_quorum.saturating_sub(1),
        required_observer_count: config.min_watchtower_quorum,
        rejected_observer_count: 1,
        replay_gap_count: 1,
        quorum_reached: false,
    };
    let transcript_continuity = UserEscapeTranscriptContinuity {
        transcript_root: roots.user_escape_transcript_root.clone(),
        previous_transcript_root: deterministic_root("fail-closed-previous-transcript-root"),
        next_transcript_root: deterministic_root("fail-closed-next-transcript-root"),
        continuity_proof_root: deterministic_root("fail-closed-continuity-proof-root"),
        first_escape_event_id: deterministic_id("fail-closed-escape-event", &["first"]),
        last_escape_event_id: deterministic_id("fail-closed-escape-event", &["last"]),
        transcript_gap_count: 1,
        replayed_event_count: 1,
        continuous: false,
    };
    let wallet_hold_export = WalletFacingHoldExport {
        export_root: roots.wallet_hold_export_root.clone(),
        wallet_view_root: deterministic_root("fail-closed-wallet-view-root"),
        hold_reason_root: deterministic_root("fail-closed-hold-reason-root"),
        release_suppression_root: deterministic_root("fail-closed-release-suppression-root"),
        status: WalletHoldExportStatus::FailClosed,
        wallet_release_visible_count: 0,
        fail_closed_export_count: config.min_hold_export_roots,
    };
    let activation_adjudicator = ActivationAdjudicator {
        adjudicator_root: roots.activation_adjudicator_root.clone(),
        activation_receipt_root: deterministic_root("fail-closed-activation-receipt-root"),
        dispute_resolution_root: deterministic_root("fail-closed-dispute-resolution-root"),
        policy_decision_root: deterministic_root("fail-closed-policy-decision-root"),
        decision: ActivationDecision::HoldWalletFacingRelease,
        accepted_decision_count: config.min_activation_adjudicator_roots,
        rejected_decision_count: 0,
    };
    let requirements = vec![RequirementRecord::new(
        EnforcementDomain::WalletHoldExport,
        true,
        roots.wallet_hold_export_root.clone(),
        roots.wallet_hold_export_root.clone(),
        deterministic_root("fail-closed-requirement-evidence-root"),
        RequirementStatus::Satisfied,
    )];
    let verdict = EnforcementVerdict::new(
        &config,
        &roots,
        &counters,
        &watchtower_quorum,
        &transcript_continuity,
        &wallet_hold_export,
        &activation_adjudicator,
    );
    let enforcement_record = EnforcementRecord::from_parts(&config, &roots, &verdict);
    roots.enforcement_record_root = enforcement_record.state_root();
    roots.state_commitment_root =
        state_commitment_root(&config, &roots, &counters, &verdict, &enforcement_record);
    State {
        config,
        roots,
        counters,
        requirements,
        watchtower_quorum,
        transcript_continuity,
        wallet_hold_export,
        activation_adjudicator,
        enforcement_record,
        verdict,
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}
