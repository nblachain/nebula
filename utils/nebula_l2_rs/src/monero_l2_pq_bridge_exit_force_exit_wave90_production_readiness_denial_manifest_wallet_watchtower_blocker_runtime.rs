use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave90-production-readiness-denial-manifest-wallet-watchtower-blocker-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DENIAL_SUITE: &str = "wave90-production-readiness-denial-wallet-watchtower-force-exit-v1";
pub const DEFAULT_WAVE: u64 = 90;
pub const DEFAULT_SOURCE_WAVE: u64 = 89;
pub const DEFAULT_MANIFEST_HEIGHT: u64 = 900_000;
pub const DEFAULT_ARCHIVE_HEIGHT: u64 = 890_000;
pub const DEFAULT_MAX_ARCHIVE_AGE_BLOCKS: u64 = 16_000;
pub const DEFAULT_MIN_WALLET_CRITERIA: u16 = 6;
pub const DEFAULT_MIN_WATCHTOWER_BLOCKERS: u16 = 5;
pub const DEFAULT_MIN_RUNBOOK_BLOCKERS: u16 = 5;
pub const DEFAULT_MIN_RECOVERY_GAPS: u16 = 4;
pub const DEFAULT_MIN_OPERATOR_HINTS: u16 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub denial_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub manifest_height: u64,
    pub archive_height: u64,
    pub release_channel: String,
    pub environment: String,
    pub privacy_model: String,
    pub max_archive_age_blocks: u64,
    pub min_wallet_criteria: u16,
    pub min_watchtower_blockers: u16,
    pub min_runbook_blockers: u16,
    pub min_recovery_gaps: u16,
    pub min_operator_hints: u16,
    pub require_wave89_no_go_archive: bool,
    pub require_wallet_escape_evidence: bool,
    pub require_watchtower_quorum_evidence: bool,
    pub require_user_runbook_evidence: bool,
    pub require_redacted_recovery_evidence: bool,
    pub require_wallet_visible_receipts: bool,
    pub require_fail_closed_denial: bool,
    pub allow_production_readiness: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            denial_suite: DENIAL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            manifest_height: DEFAULT_MANIFEST_HEIGHT,
            archive_height: DEFAULT_ARCHIVE_HEIGHT,
            release_channel: "devnet-force-exit-production-readiness-hold".to_string(),
            environment: "devnet-production-shadow".to_string(),
            privacy_model: "roots-only-redacted-wallet-evidence".to_string(),
            max_archive_age_blocks: DEFAULT_MAX_ARCHIVE_AGE_BLOCKS,
            min_wallet_criteria: DEFAULT_MIN_WALLET_CRITERIA,
            min_watchtower_blockers: DEFAULT_MIN_WATCHTOWER_BLOCKERS,
            min_runbook_blockers: DEFAULT_MIN_RUNBOOK_BLOCKERS,
            min_recovery_gaps: DEFAULT_MIN_RECOVERY_GAPS,
            min_operator_hints: DEFAULT_MIN_OPERATOR_HINTS,
            require_wave89_no_go_archive: true,
            require_wallet_escape_evidence: true,
            require_watchtower_quorum_evidence: true,
            require_user_runbook_evidence: true,
            require_redacted_recovery_evidence: true,
            require_wallet_visible_receipts: true,
            require_fail_closed_denial: true,
            allow_production_readiness: false,
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
        ensure_non_empty("denial_suite", &self.denial_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure_non_empty("environment", &self.environment)?;
        ensure_non_empty("privacy_model", &self.privacy_model)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.wave > self.source_wave, "wave must follow source wave")?;
        ensure(
            self.manifest_height > self.archive_height,
            "manifest height must follow archive height",
        )?;
        ensure(
            self.manifest_height - self.archive_height <= self.max_archive_age_blocks,
            "source archive is too old for wave 90 denial binding",
        )?;
        ensure(
            self.min_wallet_criteria > 0,
            "minimum wallet criteria must be non-zero",
        )?;
        ensure(
            self.min_watchtower_blockers > 0,
            "minimum watchtower blockers must be non-zero",
        )?;
        ensure(
            self.min_runbook_blockers > 0,
            "minimum runbook blockers must be non-zero",
        )?;
        ensure(
            self.min_recovery_gaps > 0,
            "minimum recovery gaps must be non-zero",
        )?;
        ensure(
            self.min_operator_hints > 0,
            "minimum operator hints must be non-zero",
        )?;
        if self.require_fail_closed_denial {
            ensure(
                !self.allow_production_readiness,
                "fail-closed denial cannot allow production readiness",
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
            "denial_suite": self.denial_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "manifest_height": self.manifest_height,
            "archive_height": self.archive_height,
            "release_channel": self.release_channel,
            "environment": self.environment,
            "privacy_model": self.privacy_model,
            "max_archive_age_blocks": self.max_archive_age_blocks,
            "min_wallet_criteria": self.min_wallet_criteria,
            "min_watchtower_blockers": self.min_watchtower_blockers,
            "min_runbook_blockers": self.min_runbook_blockers,
            "min_recovery_gaps": self.min_recovery_gaps,
            "min_operator_hints": self.min_operator_hints,
            "require_wave89_no_go_archive": self.require_wave89_no_go_archive,
            "require_wallet_escape_evidence": self.require_wallet_escape_evidence,
            "require_watchtower_quorum_evidence": self.require_watchtower_quorum_evidence,
            "require_user_runbook_evidence": self.require_user_runbook_evidence,
            "require_redacted_recovery_evidence": self.require_redacted_recovery_evidence,
            "require_wallet_visible_receipts": self.require_wallet_visible_receipts,
            "require_fail_closed_denial": self.require_fail_closed_denial,
            "allow_production_readiness": self.allow_production_readiness,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    AcceptedRoot,
    DeferredRoot,
    MissingRoot,
    RedactedRoot,
    ContradictedRoot,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedRoot => "accepted_root",
            Self::DeferredRoot => "deferred_root",
            Self::MissingRoot => "missing_root",
            Self::RedactedRoot => "redacted_root",
            Self::ContradictedRoot => "contradicted_root",
        }
    }

    pub fn blocks_readiness(self) -> bool {
        !matches!(self, Self::AcceptedRoot)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Advisory,
    Hold,
    Deny,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Hold => "hold",
            Self::Deny => "deny",
        }
    }

    pub fn weight(self) -> u64 {
        match self {
            Self::Advisory => 1,
            Self::Hold => 4,
            Self::Deny => 9,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialDomain {
    Wave89Archive,
    WalletEscape,
    WatchtowerQuorum,
    UserRunbook,
    RedactedRecovery,
    WalletVisibleReceipt,
    OperatorAction,
}

impl DenialDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave89Archive => "wave89_archive",
            Self::WalletEscape => "wallet_escape",
            Self::WatchtowerQuorum => "watchtower_quorum",
            Self::UserRunbook => "user_runbook",
            Self::RedactedRecovery => "redacted_recovery",
            Self::WalletVisibleReceipt => "wallet_visible_receipt",
            Self::OperatorAction => "operator_action",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletDenialCriterionKind {
    EscapePathProvesSpendAuthority,
    ForceExitClaimMatchesWalletView,
    OfflineWalletCanDeriveExitPackage,
    WalletCanDetectCensorship,
    WalletCanEstimateExitFeeBounds,
    WalletCanTrackChallengeWindow,
    WalletCanVerifyWatchtowerReceipt,
    WalletCanExportSupportBundle,
}

impl WalletDenialCriterionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::EscapePathProvesSpendAuthority,
            Self::ForceExitClaimMatchesWalletView,
            Self::OfflineWalletCanDeriveExitPackage,
            Self::WalletCanDetectCensorship,
            Self::WalletCanEstimateExitFeeBounds,
            Self::WalletCanTrackChallengeWindow,
            Self::WalletCanVerifyWatchtowerReceipt,
            Self::WalletCanExportSupportBundle,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::EscapePathProvesSpendAuthority => "escape_path_proves_spend_authority",
            Self::ForceExitClaimMatchesWalletView => "force_exit_claim_matches_wallet_view",
            Self::OfflineWalletCanDeriveExitPackage => "offline_wallet_can_derive_exit_package",
            Self::WalletCanDetectCensorship => "wallet_can_detect_censorship",
            Self::WalletCanEstimateExitFeeBounds => "wallet_can_estimate_exit_fee_bounds",
            Self::WalletCanTrackChallengeWindow => "wallet_can_track_challenge_window",
            Self::WalletCanVerifyWatchtowerReceipt => "wallet_can_verify_watchtower_receipt",
            Self::WalletCanExportSupportBundle => "wallet_can_export_support_bundle",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchtowerBlockerKind {
    QuorumMembershipRootMissing,
    EquivocationPolicyDeferred,
    ForceExitObservationRootMissing,
    AlertFanoutDryRunMissing,
    ChallengeWindowClockUnbound,
    WalletReceiptAckMissing,
    SlashingEvidenceRootDeferred,
}

impl WatchtowerBlockerKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::QuorumMembershipRootMissing,
            Self::EquivocationPolicyDeferred,
            Self::ForceExitObservationRootMissing,
            Self::AlertFanoutDryRunMissing,
            Self::ChallengeWindowClockUnbound,
            Self::WalletReceiptAckMissing,
            Self::SlashingEvidenceRootDeferred,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuorumMembershipRootMissing => "quorum_membership_root_missing",
            Self::EquivocationPolicyDeferred => "equivocation_policy_deferred",
            Self::ForceExitObservationRootMissing => "force_exit_observation_root_missing",
            Self::AlertFanoutDryRunMissing => "alert_fanout_dry_run_missing",
            Self::ChallengeWindowClockUnbound => "challenge_window_clock_unbound",
            Self::WalletReceiptAckMissing => "wallet_receipt_ack_missing",
            Self::SlashingEvidenceRootDeferred => "slashing_evidence_root_deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookBlockerKind {
    UserEscapeProcedureDeferred,
    WatchtowerContactPathMissing,
    FeeTopupProcedureMissing,
    ChallengeDisputeStepsDeferred,
    SupportEscalationTreeMissing,
    RecoveryVerificationStepsRedacted,
    WalletReceiptInspectionMissing,
}

impl RunbookBlockerKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::UserEscapeProcedureDeferred,
            Self::WatchtowerContactPathMissing,
            Self::FeeTopupProcedureMissing,
            Self::ChallengeDisputeStepsDeferred,
            Self::SupportEscalationTreeMissing,
            Self::RecoveryVerificationStepsRedacted,
            Self::WalletReceiptInspectionMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserEscapeProcedureDeferred => "user_escape_procedure_deferred",
            Self::WatchtowerContactPathMissing => "watchtower_contact_path_missing",
            Self::FeeTopupProcedureMissing => "fee_topup_procedure_missing",
            Self::ChallengeDisputeStepsDeferred => "challenge_dispute_steps_deferred",
            Self::SupportEscalationTreeMissing => "support_escalation_tree_missing",
            Self::RecoveryVerificationStepsRedacted => "recovery_verification_steps_redacted",
            Self::WalletReceiptInspectionMissing => "wallet_receipt_inspection_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryInstructionKind {
    RedactedKeylessRecoveryRoot,
    RedactedMultisigRotationRoot,
    RedactedWatchtowerFailoverRoot,
    RedactedSupportBundleRoot,
    RedactedDisputeEvidenceRoot,
    RedactedOperatorEscalationRoot,
}

impl RecoveryInstructionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RedactedKeylessRecoveryRoot,
            Self::RedactedMultisigRotationRoot,
            Self::RedactedWatchtowerFailoverRoot,
            Self::RedactedSupportBundleRoot,
            Self::RedactedDisputeEvidenceRoot,
            Self::RedactedOperatorEscalationRoot,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RedactedKeylessRecoveryRoot => "redacted_keyless_recovery_root",
            Self::RedactedMultisigRotationRoot => "redacted_multisig_rotation_root",
            Self::RedactedWatchtowerFailoverRoot => "redacted_watchtower_failover_root",
            Self::RedactedSupportBundleRoot => "redacted_support_bundle_root",
            Self::RedactedDisputeEvidenceRoot => "redacted_dispute_evidence_root",
            Self::RedactedOperatorEscalationRoot => "redacted_operator_escalation_root",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptEvidenceKind {
    WalletVisibleEscapeReceipt,
    WatchtowerAckReceipt,
    FeeBoundReceipt,
    ChallengeWindowReceipt,
    RecoveryBundleReceipt,
    UserFacingDenialReceipt,
}

impl ReceiptEvidenceKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::WalletVisibleEscapeReceipt,
            Self::WatchtowerAckReceipt,
            Self::FeeBoundReceipt,
            Self::ChallengeWindowReceipt,
            Self::RecoveryBundleReceipt,
            Self::UserFacingDenialReceipt,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletVisibleEscapeReceipt => "wallet_visible_escape_receipt",
            Self::WatchtowerAckReceipt => "watchtower_ack_receipt",
            Self::FeeBoundReceipt => "fee_bound_receipt",
            Self::ChallengeWindowReceipt => "challenge_window_receipt",
            Self::RecoveryBundleReceipt => "recovery_bundle_receipt",
            Self::UserFacingDenialReceipt => "user_facing_denial_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorActionKind {
    FreezeProductionReadiness,
    PublishDenialReceipt,
    RequestWalletEscapeDryRun,
    RequestWatchtowerQuorumDrill,
    RequestUserRunbookWalkthrough,
    RequestRedactedRecoveryReview,
    RequireReceiptReplay,
    KeepHeavyGatesDeferred,
}

impl OperatorActionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FreezeProductionReadiness,
            Self::PublishDenialReceipt,
            Self::RequestWalletEscapeDryRun,
            Self::RequestWatchtowerQuorumDrill,
            Self::RequestUserRunbookWalkthrough,
            Self::RequestRedactedRecoveryReview,
            Self::RequireReceiptReplay,
            Self::KeepHeavyGatesDeferred,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreezeProductionReadiness => "freeze_production_readiness",
            Self::PublishDenialReceipt => "publish_denial_receipt",
            Self::RequestWalletEscapeDryRun => "request_wallet_escape_dry_run",
            Self::RequestWatchtowerQuorumDrill => "request_watchtower_quorum_drill",
            Self::RequestUserRunbookWalkthrough => "request_user_runbook_walkthrough",
            Self::RequestRedactedRecoveryReview => "request_redacted_recovery_review",
            Self::RequireReceiptReplay => "require_receipt_replay",
            Self::KeepHeavyGatesDeferred => "keep_heavy_gates_deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessDecision {
    DenyProductionReadiness,
    HoldProductionReadiness,
}

impl ReadinessDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyProductionReadiness => "deny_production_readiness",
            Self::HoldProductionReadiness => "hold_production_readiness",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave89NoGoArchiveEvidence {
    pub archive_epoch: u64,
    pub archive_height: u64,
    pub no_go_root: String,
    pub blocker_matrix_root: String,
    pub wallet_gap_root: String,
    pub watchtower_gap_root: String,
    pub runbook_gap_root: String,
    pub recovery_gap_root: String,
    pub receipt_gap_root: String,
    pub imported_by_wave90: bool,
    pub status: EvidenceStatus,
}

impl Wave89NoGoArchiveEvidence {
    pub fn devnet() -> Self {
        Self {
            archive_epoch: DEFAULT_SOURCE_WAVE,
            archive_height: DEFAULT_ARCHIVE_HEIGHT,
            no_go_root: synthetic_root("wave89", "no-go-archive"),
            blocker_matrix_root: synthetic_root("wave89", "blocker-matrix"),
            wallet_gap_root: synthetic_root("wave89", "wallet-gap"),
            watchtower_gap_root: synthetic_root("wave89", "watchtower-gap"),
            runbook_gap_root: synthetic_root("wave89", "runbook-gap"),
            recovery_gap_root: synthetic_root("wave89", "recovery-gap"),
            receipt_gap_root: synthetic_root("wave89", "receipt-gap"),
            imported_by_wave90: true,
            status: EvidenceStatus::AcceptedRoot,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure(self.archive_epoch > 0, "archive epoch must be non-zero")?;
        ensure(self.archive_height > 0, "archive height must be non-zero")?;
        ensure_hex_root("no_go_root", &self.no_go_root)?;
        ensure_hex_root("blocker_matrix_root", &self.blocker_matrix_root)?;
        ensure_hex_root("wallet_gap_root", &self.wallet_gap_root)?;
        ensure_hex_root("watchtower_gap_root", &self.watchtower_gap_root)?;
        ensure_hex_root("runbook_gap_root", &self.runbook_gap_root)?;
        ensure_hex_root("recovery_gap_root", &self.recovery_gap_root)?;
        ensure_hex_root("receipt_gap_root", &self.receipt_gap_root)?;
        ensure(self.imported_by_wave90, "wave 89 archive must be imported")?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "archive_epoch": self.archive_epoch,
            "archive_height": self.archive_height,
            "no_go_root": self.no_go_root,
            "blocker_matrix_root": self.blocker_matrix_root,
            "wallet_gap_root": self.wallet_gap_root,
            "watchtower_gap_root": self.watchtower_gap_root,
            "runbook_gap_root": self.runbook_gap_root,
            "recovery_gap_root": self.recovery_gap_root,
            "receipt_gap_root": self.receipt_gap_root,
            "imported_by_wave90": self.imported_by_wave90,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave89-no-go-archive", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletDenialCriterion {
    pub kind: WalletDenialCriterionKind,
    pub domain: DenialDomain,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub evidence_root: String,
    pub blocker_root: String,
    pub required_for_release: bool,
    pub wallet_visible: bool,
}

impl WalletDenialCriterion {
    pub fn new(
        kind: WalletDenialCriterionKind,
        status: EvidenceStatus,
        severity: Severity,
        required_for_release: bool,
        wallet_visible: bool,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            domain: DenialDomain::WalletEscape,
            status,
            severity,
            evidence_root: synthetic_root("wallet-evidence", tag),
            blocker_root: synthetic_root("wallet-blocker", tag),
            required_for_release,
            wallet_visible,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.required_for_release && self.status.blocks_readiness()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("wallet evidence root", &self.evidence_root)?;
        ensure_hex_root("wallet blocker root", &self.blocker_root)?;
        ensure(
            self.domain == DenialDomain::WalletEscape,
            "wallet denial criterion must use wallet domain",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": self.domain.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "required_for_release": self.required_for_release,
            "wallet_visible": self.wallet_visible,
            "blocks_readiness": self.blocks_readiness(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerBlocker {
    pub kind: WatchtowerBlockerKind,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub quorum_root: String,
    pub observation_root: String,
    pub ack_root: String,
    pub min_quorum: u16,
    pub observed_quorum: u16,
    pub fail_closed: bool,
}

impl WatchtowerBlocker {
    pub fn new(
        kind: WatchtowerBlockerKind,
        status: EvidenceStatus,
        severity: Severity,
        min_quorum: u16,
        observed_quorum: u16,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            status,
            severity,
            quorum_root: synthetic_root("watchtower-quorum", tag),
            observation_root: synthetic_root("watchtower-observation", tag),
            ack_root: synthetic_root("watchtower-ack", tag),
            min_quorum,
            observed_quorum,
            fail_closed: true,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.fail_closed
            && (self.status.blocks_readiness() || self.observed_quorum < self.min_quorum)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("watchtower quorum root", &self.quorum_root)?;
        ensure_hex_root("watchtower observation root", &self.observation_root)?;
        ensure_hex_root("watchtower ack root", &self.ack_root)?;
        ensure(
            self.min_quorum > 0,
            "minimum watchtower quorum must be non-zero",
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": DenialDomain::WatchtowerQuorum.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "quorum_root": self.quorum_root,
            "observation_root": self.observation_root,
            "ack_root": self.ack_root,
            "min_quorum": self.min_quorum,
            "observed_quorum": self.observed_quorum,
            "fail_closed": self.fail_closed,
            "blocks_readiness": self.blocks_readiness(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserEscapeReceipt {
    pub kind: ReceiptEvidenceKind,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub receipt_root: String,
    pub wallet_projection_root: String,
    pub watchtower_projection_root: String,
    pub visible_to_wallet: bool,
    pub bound_to_archive: bool,
}

impl UserEscapeReceipt {
    pub fn new(
        kind: ReceiptEvidenceKind,
        status: EvidenceStatus,
        severity: Severity,
        visible_to_wallet: bool,
        bound_to_archive: bool,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            status,
            severity,
            receipt_root: synthetic_root("receipt", tag),
            wallet_projection_root: synthetic_root("receipt-wallet-projection", tag),
            watchtower_projection_root: synthetic_root("receipt-watchtower-projection", tag),
            visible_to_wallet,
            bound_to_archive,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.status.blocks_readiness() || !self.visible_to_wallet || !self.bound_to_archive
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("receipt root", &self.receipt_root)?;
        ensure_hex_root("wallet projection root", &self.wallet_projection_root)?;
        ensure_hex_root(
            "watchtower projection root",
            &self.watchtower_projection_root,
        )?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": DenialDomain::WalletVisibleReceipt.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "receipt_root": self.receipt_root,
            "wallet_projection_root": self.wallet_projection_root,
            "watchtower_projection_root": self.watchtower_projection_root,
            "visible_to_wallet": self.visible_to_wallet,
            "bound_to_archive": self.bound_to_archive,
            "blocks_readiness": self.blocks_readiness(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RunbookBlocker {
    pub kind: RunbookBlockerKind,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub runbook_root: String,
    pub walkthrough_root: String,
    pub owner_ack_root: String,
    pub user_tested: bool,
    pub support_ready: bool,
}

impl RunbookBlocker {
    pub fn new(
        kind: RunbookBlockerKind,
        status: EvidenceStatus,
        severity: Severity,
        user_tested: bool,
        support_ready: bool,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            status,
            severity,
            runbook_root: synthetic_root("runbook", tag),
            walkthrough_root: synthetic_root("runbook-walkthrough", tag),
            owner_ack_root: synthetic_root("runbook-owner-ack", tag),
            user_tested,
            support_ready,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.status.blocks_readiness() || !self.user_tested || !self.support_ready
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("runbook root", &self.runbook_root)?;
        ensure_hex_root("runbook walkthrough root", &self.walkthrough_root)?;
        ensure_hex_root("runbook owner ack root", &self.owner_ack_root)?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": DenialDomain::UserRunbook.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "runbook_root": self.runbook_root,
            "walkthrough_root": self.walkthrough_root,
            "owner_ack_root": self.owner_ack_root,
            "user_tested": self.user_tested,
            "support_ready": self.support_ready,
            "blocks_readiness": self.blocks_readiness(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecoveryInstruction {
    pub kind: RecoveryInstructionKind,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub instruction_root: String,
    pub redaction_root: String,
    pub review_root: String,
    pub has_redaction_reason: bool,
    pub has_recovery_rehearsal: bool,
}

impl RecoveryInstruction {
    pub fn new(
        kind: RecoveryInstructionKind,
        status: EvidenceStatus,
        severity: Severity,
        has_redaction_reason: bool,
        has_recovery_rehearsal: bool,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            status,
            severity,
            instruction_root: synthetic_root("recovery-instruction", tag),
            redaction_root: synthetic_root("recovery-redaction", tag),
            review_root: synthetic_root("recovery-review", tag),
            has_redaction_reason,
            has_recovery_rehearsal,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.status.blocks_readiness() || !self.has_redaction_reason || !self.has_recovery_rehearsal
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("recovery instruction root", &self.instruction_root)?;
        ensure_hex_root("recovery redaction root", &self.redaction_root)?;
        ensure_hex_root("recovery review root", &self.review_root)?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": DenialDomain::RedactedRecovery.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "instruction_root": self.instruction_root,
            "redaction_root": self.redaction_root,
            "review_root": self.review_root,
            "has_redaction_reason": self.has_redaction_reason,
            "has_recovery_rehearsal": self.has_recovery_rehearsal,
            "blocks_readiness": self.blocks_readiness(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorActionHint {
    pub kind: OperatorActionKind,
    pub domain: DenialDomain,
    pub severity: Severity,
    pub action_root: String,
    pub depends_on_root: String,
    pub command_room_required: bool,
    pub production_hold_required: bool,
}

impl OperatorActionHint {
    pub fn new(
        kind: OperatorActionKind,
        domain: DenialDomain,
        severity: Severity,
        command_room_required: bool,
        production_hold_required: bool,
    ) -> Self {
        let tag = kind.as_str();
        Self {
            kind,
            domain,
            severity,
            action_root: synthetic_root("operator-action", tag),
            depends_on_root: synthetic_root("operator-action-dependency", tag),
            command_room_required,
            production_hold_required,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_hex_root("operator action root", &self.action_root)?;
        ensure_hex_root("operator action dependency root", &self.depends_on_root)?;
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "domain": self.domain.as_str(),
            "severity": self.severity.as_str(),
            "action_root": self.action_root,
            "depends_on_root": self.depends_on_root,
            "command_room_required": self.command_room_required,
            "production_hold_required": self.production_hold_required,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenialFinding {
    pub domain: DenialDomain,
    pub status: EvidenceStatus,
    pub severity: Severity,
    pub evidence_root: String,
    pub reason_code: String,
    pub readiness_blocking: bool,
}

impl DenialFinding {
    pub fn new(
        domain: DenialDomain,
        status: EvidenceStatus,
        severity: Severity,
        evidence_root: String,
        reason_code: String,
        readiness_blocking: bool,
    ) -> Self {
        Self {
            domain,
            status,
            severity,
            evidence_root,
            reason_code,
            readiness_blocking,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "domain": self.domain.as_str(),
            "status": self.status.as_str(),
            "severity": self.severity.as_str(),
            "evidence_root": self.evidence_root,
            "reason_code": self.reason_code,
            "readiness_blocking": self.readiness_blocking,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenialVerdict {
    pub decision: ReadinessDecision,
    pub fail_closed: bool,
    pub deny_score: u64,
    pub hold_score: u64,
    pub total_findings: u16,
    pub blocking_findings: u16,
    pub denied_domains: BTreeSet<DenialDomain>,
    pub findings_root: String,
    pub operator_hint_root: String,
}

impl DenialVerdict {
    pub fn evaluate(
        findings: &[DenialFinding],
        operator_hints: &[OperatorActionHint],
        config: &Config,
    ) -> Self {
        let mut denied_domains = BTreeSet::new();
        let mut deny_score = 0_u64;
        let mut hold_score = 0_u64;
        let mut blocking_findings = 0_u16;

        for finding in findings {
            if finding.readiness_blocking {
                denied_domains.insert(finding.domain);
                blocking_findings = blocking_findings.saturating_add(1);
                match finding.severity {
                    Severity::Deny => {
                        deny_score = deny_score.saturating_add(finding.severity.weight())
                    }
                    Severity::Hold | Severity::Advisory => {
                        hold_score = hold_score.saturating_add(finding.severity.weight())
                    }
                }
            }
        }

        let decision = if config.require_fail_closed_denial
            || !denied_domains.is_empty()
            || config.allow_production_readiness == false
        {
            ReadinessDecision::DenyProductionReadiness
        } else {
            ReadinessDecision::HoldProductionReadiness
        };

        let finding_records = findings
            .iter()
            .map(DenialFinding::public_record)
            .collect::<Vec<_>>();
        let hint_records = operator_hints
            .iter()
            .map(OperatorActionHint::public_record)
            .collect::<Vec<_>>();

        Self {
            decision,
            fail_closed: config.require_fail_closed_denial,
            deny_score,
            hold_score,
            total_findings: bounded_len(findings.len()),
            blocking_findings,
            denied_domains,
            findings_root: merkle_root("wave90-denial-findings", &finding_records),
            operator_hint_root: merkle_root("wave90-operator-hints", &hint_records),
        }
    }

    pub fn public_record(&self) -> Value {
        let domains = self
            .denied_domains
            .iter()
            .map(|domain| domain.as_str())
            .collect::<Vec<_>>();
        json!({
            "decision": self.decision.as_str(),
            "fail_closed": self.fail_closed,
            "deny_score": self.deny_score,
            "hold_score": self.hold_score,
            "total_findings": self.total_findings,
            "blocking_findings": self.blocking_findings,
            "denied_domains": domains,
            "findings_root": self.findings_root,
            "operator_hint_root": self.operator_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("denial-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceCounters {
    pub wallet_criteria: u16,
    pub wallet_blockers: u16,
    pub watchtower_blockers: u16,
    pub runbook_blockers: u16,
    pub recovery_gaps: u16,
    pub receipt_gaps: u16,
    pub operator_hints: u16,
    pub accepted_roots: u16,
    pub deferred_roots: u16,
    pub missing_roots: u16,
    pub redacted_roots: u16,
    pub contradicted_roots: u16,
}

impl EvidenceCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_criteria": self.wallet_criteria,
            "wallet_blockers": self.wallet_blockers,
            "watchtower_blockers": self.watchtower_blockers,
            "runbook_blockers": self.runbook_blockers,
            "recovery_gaps": self.recovery_gaps,
            "receipt_gaps": self.receipt_gaps,
            "operator_hints": self.operator_hints,
            "accepted_roots": self.accepted_roots,
            "deferred_roots": self.deferred_roots,
            "missing_roots": self.missing_roots,
            "redacted_roots": self.redacted_roots,
            "contradicted_roots": self.contradicted_roots,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub version: String,
    pub state_root: String,
    pub config_root: String,
    pub archive_root: String,
    pub wallet_root: String,
    pub watchtower_root: String,
    pub runbook_root: String,
    pub recovery_root: String,
    pub receipt_root: String,
    pub operator_hint_root: String,
    pub verdict_root: String,
    pub decision: ReadinessDecision,
    pub counters: EvidenceCounters,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "version": self.version,
            "state_root": self.state_root,
            "config_root": self.config_root,
            "archive_root": self.archive_root,
            "wallet_root": self.wallet_root,
            "watchtower_root": self.watchtower_root,
            "runbook_root": self.runbook_root,
            "recovery_root": self.recovery_root,
            "receipt_root": self.receipt_root,
            "operator_hint_root": self.operator_hint_root,
            "verdict_root": self.verdict_root,
            "decision": self.decision.as_str(),
            "counters": self.counters.public_record(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave89_archive: Wave89NoGoArchiveEvidence,
    pub wallet_criteria: Vec<WalletDenialCriterion>,
    pub watchtower_blockers: Vec<WatchtowerBlocker>,
    pub user_escape_receipts: Vec<UserEscapeReceipt>,
    pub runbook_blockers: Vec<RunbookBlocker>,
    pub recovery_instructions: Vec<RecoveryInstruction>,
    pub operator_hints: Vec<OperatorActionHint>,
    pub findings: Vec<DenialFinding>,
    pub verdict: DenialVerdict,
}

impl State {
    pub fn new(
        config: Config,
        wave89_archive: Wave89NoGoArchiveEvidence,
        wallet_criteria: Vec<WalletDenialCriterion>,
        watchtower_blockers: Vec<WatchtowerBlocker>,
        user_escape_receipts: Vec<UserEscapeReceipt>,
        runbook_blockers: Vec<RunbookBlocker>,
        recovery_instructions: Vec<RecoveryInstruction>,
        operator_hints: Vec<OperatorActionHint>,
    ) -> Result<Self> {
        config.validate()?;
        wave89_archive.validate()?;
        validate_items(&wallet_criteria, WalletDenialCriterion::validate)?;
        validate_items(&watchtower_blockers, WatchtowerBlocker::validate)?;
        validate_items(&user_escape_receipts, UserEscapeReceipt::validate)?;
        validate_items(&runbook_blockers, RunbookBlocker::validate)?;
        validate_items(&recovery_instructions, RecoveryInstruction::validate)?;
        validate_items(&operator_hints, OperatorActionHint::validate)?;

        ensure(
            wallet_criteria.len() >= usize::from(config.min_wallet_criteria),
            "wallet denial criteria are below configured minimum",
        )?;
        ensure(
            watchtower_blockers.len() >= usize::from(config.min_watchtower_blockers),
            "watchtower blockers are below configured minimum",
        )?;
        ensure(
            runbook_blockers.len() >= usize::from(config.min_runbook_blockers),
            "runbook blockers are below configured minimum",
        )?;
        ensure(
            recovery_instructions.len() >= usize::from(config.min_recovery_gaps),
            "recovery gaps are below configured minimum",
        )?;
        ensure(
            operator_hints.len() >= usize::from(config.min_operator_hints),
            "operator hints are below configured minimum",
        )?;

        let findings = collect_findings(
            &config,
            &wave89_archive,
            &wallet_criteria,
            &watchtower_blockers,
            &user_escape_receipts,
            &runbook_blockers,
            &recovery_instructions,
        );
        let verdict = DenialVerdict::evaluate(&findings, &operator_hints, &config);

        Ok(Self {
            config,
            wave89_archive,
            wallet_criteria,
            watchtower_blockers,
            user_escape_receipts,
            runbook_blockers,
            recovery_instructions,
            operator_hints,
            findings,
            verdict,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave89_archive = Wave89NoGoArchiveEvidence::devnet();
        let wallet_criteria = default_wallet_criteria();
        let watchtower_blockers = default_watchtower_blockers();
        let user_escape_receipts = default_user_escape_receipts();
        let runbook_blockers = default_runbook_blockers();
        let recovery_instructions = default_recovery_instructions();
        let operator_hints = default_operator_hints();
        match Self::new(
            config,
            wave89_archive,
            wallet_criteria,
            watchtower_blockers,
            user_escape_receipts,
            runbook_blockers,
            recovery_instructions,
            operator_hints,
        ) {
            Ok(state) => state,
            Err(error) => fallback_state(error),
        }
    }

    pub fn counters(&self) -> EvidenceCounters {
        let mut status_counts = BTreeMap::<EvidenceStatus, u16>::new();
        count_status(&mut status_counts, self.wave89_archive.status);
        for item in &self.wallet_criteria {
            count_status(&mut status_counts, item.status);
        }
        for item in &self.watchtower_blockers {
            count_status(&mut status_counts, item.status);
        }
        for item in &self.user_escape_receipts {
            count_status(&mut status_counts, item.status);
        }
        for item in &self.runbook_blockers {
            count_status(&mut status_counts, item.status);
        }
        for item in &self.recovery_instructions {
            count_status(&mut status_counts, item.status);
        }

        EvidenceCounters {
            wallet_criteria: bounded_len(self.wallet_criteria.len()),
            wallet_blockers: bounded_len(
                self.wallet_criteria
                    .iter()
                    .filter(|item| item.blocks_readiness())
                    .count(),
            ),
            watchtower_blockers: bounded_len(
                self.watchtower_blockers
                    .iter()
                    .filter(|item| item.blocks_readiness())
                    .count(),
            ),
            runbook_blockers: bounded_len(
                self.runbook_blockers
                    .iter()
                    .filter(|item| item.blocks_readiness())
                    .count(),
            ),
            recovery_gaps: bounded_len(
                self.recovery_instructions
                    .iter()
                    .filter(|item| item.blocks_readiness())
                    .count(),
            ),
            receipt_gaps: bounded_len(
                self.user_escape_receipts
                    .iter()
                    .filter(|item| item.blocks_readiness())
                    .count(),
            ),
            operator_hints: bounded_len(self.operator_hints.len()),
            accepted_roots: status_count(&status_counts, EvidenceStatus::AcceptedRoot),
            deferred_roots: status_count(&status_counts, EvidenceStatus::DeferredRoot),
            missing_roots: status_count(&status_counts, EvidenceStatus::MissingRoot),
            redacted_roots: status_count(&status_counts, EvidenceStatus::RedactedRoot),
            contradicted_roots: status_count(&status_counts, EvidenceStatus::ContradictedRoot),
        }
    }

    pub fn roots(&self) -> BTreeMap<String, String> {
        let mut roots = BTreeMap::new();
        roots.insert("config".to_string(), self.config.state_root());
        roots.insert(
            "wave89_archive".to_string(),
            self.wave89_archive.state_root(),
        );
        roots.insert("wallet".to_string(), self.wallet_root());
        roots.insert("watchtower".to_string(), self.watchtower_root());
        roots.insert("receipt".to_string(), self.receipt_root());
        roots.insert("runbook".to_string(), self.runbook_root());
        roots.insert("recovery".to_string(), self.recovery_root());
        roots.insert("operator_hint".to_string(), self.operator_hint_root());
        roots.insert("findings".to_string(), self.findings_root());
        roots.insert("verdict".to_string(), self.verdict.state_root());
        roots
    }

    pub fn wallet_root(&self) -> String {
        merkle_root(
            "wave90-wallet-denial-criteria",
            &self
                .wallet_criteria
                .iter()
                .map(WalletDenialCriterion::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn watchtower_root(&self) -> String {
        merkle_root(
            "wave90-watchtower-blockers",
            &self
                .watchtower_blockers
                .iter()
                .map(WatchtowerBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn receipt_root(&self) -> String {
        merkle_root(
            "wave90-wallet-visible-receipts",
            &self
                .user_escape_receipts
                .iter()
                .map(UserEscapeReceipt::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn runbook_root(&self) -> String {
        merkle_root(
            "wave90-user-runbook-blockers",
            &self
                .runbook_blockers
                .iter()
                .map(RunbookBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn recovery_root(&self) -> String {
        merkle_root(
            "wave90-redacted-recovery-instructions",
            &self
                .recovery_instructions
                .iter()
                .map(RecoveryInstruction::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn operator_hint_root(&self) -> String {
        merkle_root(
            "wave90-operator-action-hints",
            &self
                .operator_hints
                .iter()
                .map(OperatorActionHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn findings_root(&self) -> String {
        merkle_root(
            "wave90-denial-findings",
            &self
                .findings
                .iter()
                .map(DenialFinding::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            version: PROTOCOL_VERSION.to_string(),
            state_root: self.state_root(),
            config_root: self.config.state_root(),
            archive_root: self.wave89_archive.state_root(),
            wallet_root: self.wallet_root(),
            watchtower_root: self.watchtower_root(),
            runbook_root: self.runbook_root(),
            recovery_root: self.recovery_root(),
            receipt_root: self.receipt_root(),
            operator_hint_root: self.operator_hint_root(),
            verdict_root: self.verdict.state_root(),
            decision: self.verdict.decision,
            counters: self.counters(),
        }
    }

    pub fn public_record_value(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "wave89_archive": self.wave89_archive.public_record(),
            "roots": self.roots(),
            "counters": self.counters().public_record(),
            "verdict": self.verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE90-PRODUCTION-READINESS-DENIAL-STATE",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Json(&json!({
                    "config": self.config.public_record(),
                    "wave89_archive": self.wave89_archive.public_record(),
                    "wallet_root": self.wallet_root(),
                    "watchtower_root": self.watchtower_root(),
                    "receipt_root": self.receipt_root(),
                    "runbook_root": self.runbook_root(),
                    "recovery_root": self.recovery_root(),
                    "operator_hint_root": self.operator_hint_root(),
                    "findings_root": self.findings_root(),
                    "verdict": self.verdict.public_record(),
                    "counters": self.counters().public_record(),
                })),
            ],
            32,
        )
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

fn default_wallet_criteria() -> Vec<WalletDenialCriterion> {
    WalletDenialCriterionKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let status = match index % 4 {
                0 => EvidenceStatus::DeferredRoot,
                1 => EvidenceStatus::MissingRoot,
                2 => EvidenceStatus::RedactedRoot,
                _ => EvidenceStatus::ContradictedRoot,
            };
            WalletDenialCriterion::new(kind, status, Severity::Deny, true, index % 3 == 0)
        })
        .collect()
}

fn default_watchtower_blockers() -> Vec<WatchtowerBlocker> {
    WatchtowerBlockerKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let status = if index % 2 == 0 {
                EvidenceStatus::MissingRoot
            } else {
                EvidenceStatus::DeferredRoot
            };
            WatchtowerBlocker::new(kind, status, Severity::Deny, 5, (index % 3) as u16)
        })
        .collect()
}

fn default_user_escape_receipts() -> Vec<UserEscapeReceipt> {
    ReceiptEvidenceKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let status = match index % 3 {
                0 => EvidenceStatus::MissingRoot,
                1 => EvidenceStatus::DeferredRoot,
                _ => EvidenceStatus::RedactedRoot,
            };
            UserEscapeReceipt::new(kind, status, Severity::Deny, index == 5, index % 2 == 0)
        })
        .collect()
}

fn default_runbook_blockers() -> Vec<RunbookBlocker> {
    RunbookBlockerKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let status = if index % 2 == 0 {
                EvidenceStatus::DeferredRoot
            } else {
                EvidenceStatus::MissingRoot
            };
            RunbookBlocker::new(kind, status, Severity::Hold, index == 0, false)
        })
        .collect()
}

fn default_recovery_instructions() -> Vec<RecoveryInstruction> {
    RecoveryInstructionKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let status = if index % 2 == 0 {
                EvidenceStatus::RedactedRoot
            } else {
                EvidenceStatus::DeferredRoot
            };
            RecoveryInstruction::new(kind, status, Severity::Deny, true, index == 0)
        })
        .collect()
}

fn default_operator_hints() -> Vec<OperatorActionHint> {
    OperatorActionKind::all()
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let domain = match index {
                0 | 1 => DenialDomain::OperatorAction,
                2 => DenialDomain::WalletEscape,
                3 => DenialDomain::WatchtowerQuorum,
                4 => DenialDomain::UserRunbook,
                5 => DenialDomain::RedactedRecovery,
                6 => DenialDomain::WalletVisibleReceipt,
                _ => DenialDomain::Wave89Archive,
            };
            OperatorActionHint::new(kind, domain, Severity::Deny, true, true)
        })
        .collect()
}

fn collect_findings(
    config: &Config,
    archive: &Wave89NoGoArchiveEvidence,
    wallet_criteria: &[WalletDenialCriterion],
    watchtower_blockers: &[WatchtowerBlocker],
    receipts: &[UserEscapeReceipt],
    runbook_blockers: &[RunbookBlocker],
    recovery_instructions: &[RecoveryInstruction],
) -> Vec<DenialFinding> {
    let mut findings = Vec::new();

    if config.require_wave89_no_go_archive && archive.status.blocks_readiness() {
        findings.push(DenialFinding::new(
            DenialDomain::Wave89Archive,
            archive.status,
            Severity::Deny,
            archive.state_root(),
            "wave89_no_go_archive_not_accepted".to_string(),
            true,
        ));
    }

    for item in wallet_criteria {
        if config.require_wallet_escape_evidence && item.blocks_readiness() {
            findings.push(DenialFinding::new(
                DenialDomain::WalletEscape,
                item.status,
                item.severity,
                item.blocker_root.clone(),
                item.kind.as_str().to_string(),
                true,
            ));
        }
    }

    for item in watchtower_blockers {
        if config.require_watchtower_quorum_evidence && item.blocks_readiness() {
            findings.push(DenialFinding::new(
                DenialDomain::WatchtowerQuorum,
                item.status,
                item.severity,
                item.quorum_root.clone(),
                item.kind.as_str().to_string(),
                true,
            ));
        }
    }

    for item in receipts {
        if config.require_wallet_visible_receipts && item.blocks_readiness() {
            findings.push(DenialFinding::new(
                DenialDomain::WalletVisibleReceipt,
                item.status,
                item.severity,
                item.receipt_root.clone(),
                item.kind.as_str().to_string(),
                true,
            ));
        }
    }

    for item in runbook_blockers {
        if config.require_user_runbook_evidence && item.blocks_readiness() {
            findings.push(DenialFinding::new(
                DenialDomain::UserRunbook,
                item.status,
                item.severity,
                item.runbook_root.clone(),
                item.kind.as_str().to_string(),
                true,
            ));
        }
    }

    for item in recovery_instructions {
        if config.require_redacted_recovery_evidence && item.blocks_readiness() {
            findings.push(DenialFinding::new(
                DenialDomain::RedactedRecovery,
                item.status,
                item.severity,
                item.instruction_root.clone(),
                item.kind.as_str().to_string(),
                true,
            ));
        }
    }

    if config.require_fail_closed_denial && findings.is_empty() {
        findings.push(DenialFinding::new(
            DenialDomain::OperatorAction,
            EvidenceStatus::DeferredRoot,
            Severity::Hold,
            synthetic_root("fail-closed", "empty-finding-set"),
            "fail_closed_denial_requires_explicit_clearance".to_string(),
            true,
        ));
    }

    findings
}

fn fallback_state(error: String) -> State {
    let config = Config::default();
    let archive = Wave89NoGoArchiveEvidence::devnet();
    let operator_hints = default_operator_hints();
    let findings = vec![DenialFinding::new(
        DenialDomain::OperatorAction,
        EvidenceStatus::ContradictedRoot,
        Severity::Deny,
        synthetic_root("fallback-state", &error),
        "fallback_state_validation_error".to_string(),
        true,
    )];
    let verdict = DenialVerdict::evaluate(&findings, &operator_hints, &config);
    State {
        config,
        wave89_archive: archive,
        wallet_criteria: default_wallet_criteria(),
        watchtower_blockers: default_watchtower_blockers(),
        user_escape_receipts: default_user_escape_receipts(),
        runbook_blockers: default_runbook_blockers(),
        recovery_instructions: default_recovery_instructions(),
        operator_hints,
        findings,
        verdict,
    }
}

fn validate_items<T>(items: &[T], validate: fn(&T) -> Result<()>) -> Result<()> {
    for item in items {
        validate(item)?;
    }
    Ok(())
}

fn count_status(counts: &mut BTreeMap<EvidenceStatus, u16>, status: EvidenceStatus) {
    let current = status_count(counts, status);
    counts.insert(status, current.saturating_add(1));
}

fn status_count(counts: &BTreeMap<EvidenceStatus, u16>, status: EvidenceStatus) -> u16 {
    match counts.get(&status) {
        Some(count) => *count,
        None => 0,
    }
}

fn bounded_len(len: usize) -> u16 {
    if len > usize::from(u16::MAX) {
        u16::MAX
    } else {
        len as u16
    }
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(name: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{name} must be non-empty"),
    )
}

fn ensure_hex_root(name: &str, value: &str) -> Result<()> {
    ensure_non_empty(name, value)?;
    ensure(
        value.len() == 64,
        &format!("{name} must be a 32-byte hex root"),
    )?;
    ensure(
        value.bytes().all(|byte| byte.is_ascii_hexdigit()),
        &format!("{name} must be hex encoded"),
    )
}

fn record_root(tag: &str, record: &Value) -> String {
    domain_hash(
        "WAVE90-PRODUCTION-READINESS-DENIAL-RECORD",
        &[HashPart::Str(tag), HashPart::Json(record)],
        32,
    )
}

fn synthetic_root(domain: &str, tag: &str) -> String {
    domain_hash(
        "WAVE90-PRODUCTION-READINESS-DENIAL-SYNTHETIC-ROOT",
        &[HashPart::Str(domain), HashPart::Str(tag)],
        32,
    )
}
