use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash as crate_domain_hash, merkle_root as crate_merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave91-live-heavy-gate-execution-plan-wallet-watchtower-receipt-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave91";
pub const WAVE90_DENIAL_SOURCE: &str = "wave90-denial-output";
pub const LIVE_HEAVY_GATE_LANE: &str = "force-exit-live-heavy-gate-wallet-watchtower-receipt-lane";
pub const DEFAULT_MIN_WATCHTOWER_QUORUM: u64 = 4;
pub const DEFAULT_MIN_USER_REPLAY_STEPS: u64 = 6;
pub const DEFAULT_MIN_OPERATOR_SIGNERS: u64 = 3;
pub const DEFAULT_MIN_RECOVERY_PROOF_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WALLET_VISIBLE_ROOTS: u64 = 2;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub lane: String,
    pub min_watchtower_quorum: u64,
    pub min_user_replay_steps: u64,
    pub min_operator_signers: u64,
    pub min_recovery_proof_roots: u64,
    pub min_wallet_visible_roots: u64,
    pub require_wave90_denial_intake: bool,
    pub require_wallet_escape_dry_run: bool,
    pub require_watchtower_quorum: bool,
    pub require_user_runbook_replay: bool,
    pub require_redacted_recovery_proof: bool,
    pub require_wallet_visible_receipt: bool,
    pub require_operator_signoff: bool,
    pub require_roots_only_privacy: bool,
    pub fail_closed_on_any_gap: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_label: WAVE_LABEL.to_string(),
            source_wave_label: WAVE90_DENIAL_SOURCE.to_string(),
            lane: LIVE_HEAVY_GATE_LANE.to_string(),
            min_watchtower_quorum: DEFAULT_MIN_WATCHTOWER_QUORUM,
            min_user_replay_steps: DEFAULT_MIN_USER_REPLAY_STEPS,
            min_operator_signers: DEFAULT_MIN_OPERATOR_SIGNERS,
            min_recovery_proof_roots: DEFAULT_MIN_RECOVERY_PROOF_ROOTS,
            min_wallet_visible_roots: DEFAULT_MIN_WALLET_VISIBLE_ROOTS,
            require_wave90_denial_intake: true,
            require_wallet_escape_dry_run: true,
            require_watchtower_quorum: true,
            require_user_runbook_replay: true,
            require_redacted_recovery_proof: true,
            require_wallet_visible_receipt: true,
            require_operator_signoff: true,
            require_roots_only_privacy: true,
            fail_closed_on_any_gap: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_action_count(&self) -> u64 {
        [
            self.require_wave90_denial_intake,
            self.require_wallet_escape_dry_run,
            self.require_watchtower_quorum,
            self.require_user_runbook_replay,
            self.require_redacted_recovery_proof,
            self.require_wallet_visible_receipt,
            self.require_operator_signoff,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_label": self.wave_label,
            "source_wave_label": self.source_wave_label,
            "lane": self.lane,
            "min_watchtower_quorum": self.min_watchtower_quorum,
            "min_user_replay_steps": self.min_user_replay_steps,
            "min_operator_signers": self.min_operator_signers,
            "min_recovery_proof_roots": self.min_recovery_proof_roots,
            "min_wallet_visible_roots": self.min_wallet_visible_roots,
            "require_wave90_denial_intake": self.require_wave90_denial_intake,
            "require_wallet_escape_dry_run": self.require_wallet_escape_dry_run,
            "require_watchtower_quorum": self.require_watchtower_quorum,
            "require_user_runbook_replay": self.require_user_runbook_replay,
            "require_redacted_recovery_proof": self.require_redacted_recovery_proof,
            "require_wallet_visible_receipt": self.require_wallet_visible_receipt,
            "require_operator_signoff": self.require_operator_signoff,
            "require_roots_only_privacy": self.require_roots_only_privacy,
            "fail_closed_on_any_gap": self.fail_closed_on_any_gap,
            "required_action_count": self.required_action_count(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialBlocker {
    WalletEscapeDryRunMissing,
    WatchtowerQuorumMissing,
    UserRunbookReplayMissing,
    RedactedRecoveryProofMissing,
    WalletVisibleReceiptMissing,
    OperatorSignoffMissing,
}

impl DenialBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletEscapeDryRunMissing => "wallet_escape_dry_run_missing",
            Self::WatchtowerQuorumMissing => "watchtower_quorum_missing",
            Self::UserRunbookReplayMissing => "user_runbook_replay_missing",
            Self::RedactedRecoveryProofMissing => "redacted_recovery_proof_missing",
            Self::WalletVisibleReceiptMissing => "wallet_visible_receipt_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionLane {
    DenialRootIntake,
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl ActionLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenialRootIntake => "denial_root_intake",
            Self::WalletEscapeDryRun => "wallet_escape_dry_run",
            Self::WatchtowerQuorum => "watchtower_quorum",
            Self::UserRunbookReplay => "user_runbook_replay",
            Self::RedactedRecoveryProof => "redacted_recovery_proof",
            Self::WalletVisibleReceipt => "wallet_visible_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceMode {
    RootOnly,
    RedactedRootOnly,
    QuorumRootOnly,
}

impl EvidenceMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootOnly => "root_only",
            Self::RedactedRootOnly => "redacted_root_only",
            Self::QuorumRootOnly => "quorum_root_only",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Blocked,
    Ready,
    Accepted,
}

impl ActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Ready => "ready",
            Self::Accepted => "accepted",
        }
    }

    pub fn clears(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceVerdictKind {
    Clear,
    FailClosed,
}

impl ClearanceVerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenialRootIntake {
    pub intake_id: String,
    pub denial_output_root: String,
    pub denial_manifest_root: String,
    pub blocker_root: String,
    pub source_run_root: String,
    pub blocker_count: u64,
    pub source_height: u64,
    pub roots_only: bool,
    pub blockers: Vec<DenialBlocker>,
}

impl DenialRootIntake {
    pub fn new(
        denial_output_root: &str,
        denial_manifest_root: &str,
        blocker_root: &str,
        source_run_root: &str,
        source_height: u64,
        blockers: Vec<DenialBlocker>,
    ) -> Self {
        let blocker_count = blockers.len() as u64;
        let intake_id = denial_intake_id(
            denial_output_root,
            denial_manifest_root,
            blocker_root,
            source_run_root,
            source_height,
            blocker_count,
        );
        Self {
            intake_id,
            denial_output_root: denial_output_root.to_string(),
            denial_manifest_root: denial_manifest_root.to_string(),
            blocker_root: blocker_root.to_string(),
            source_run_root: source_run_root.to_string(),
            blocker_count,
            source_height,
            roots_only: true,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        let blockers = self
            .blockers
            .iter()
            .map(|blocker| blocker.as_str())
            .collect::<Vec<_>>();
        json!({
            "intake_id": self.intake_id,
            "denial_output_root": self.denial_output_root,
            "denial_manifest_root": self.denial_manifest_root,
            "blocker_root": self.blocker_root,
            "source_run_root": self.source_run_root,
            "blocker_count": self.blocker_count,
            "source_height": self.source_height,
            "roots_only": self.roots_only,
            "blockers": blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("denial_root_intake", &self.public_record())
    }

    pub fn valid(&self) -> bool {
        self.roots_only
            && self.blocker_count == self.blockers.len() as u64
            && non_empty(&self.denial_output_root)
            && non_empty(&self.denial_manifest_root)
            && non_empty(&self.blocker_root)
            && non_empty(&self.source_run_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletReceiptPlan {
    pub plan_id: String,
    pub dry_run_root: String,
    pub wallet_state_before_root: String,
    pub wallet_state_after_root: String,
    pub escape_route_root: String,
    pub fee_bound_root: String,
    pub no_broadcast_root: String,
    pub receipt_root: String,
    pub visible_receipt_root: String,
    pub wallet_visible_root_count: u64,
    pub mode: EvidenceMode,
    pub status: ActionStatus,
}

impl WalletReceiptPlan {
    pub fn new(
        dry_run_root: &str,
        wallet_state_before_root: &str,
        wallet_state_after_root: &str,
        escape_route_root: &str,
        fee_bound_root: &str,
        no_broadcast_root: &str,
        visible_receipt_root: &str,
        wallet_visible_root_count: u64,
        status: ActionStatus,
    ) -> Self {
        let receipt_root = merkle_root(
            "wave91-wallet-receipt-plan",
            &[
                dry_run_root,
                wallet_state_before_root,
                wallet_state_after_root,
                escape_route_root,
                fee_bound_root,
                no_broadcast_root,
                visible_receipt_root,
            ],
        );
        let plan_id = action_id(
            ActionLane::WalletEscapeDryRun,
            &receipt_root,
            wallet_visible_root_count,
            status,
        );
        Self {
            plan_id,
            dry_run_root: dry_run_root.to_string(),
            wallet_state_before_root: wallet_state_before_root.to_string(),
            wallet_state_after_root: wallet_state_after_root.to_string(),
            escape_route_root: escape_route_root.to_string(),
            fee_bound_root: fee_bound_root.to_string(),
            no_broadcast_root: no_broadcast_root.to_string(),
            receipt_root,
            visible_receipt_root: visible_receipt_root.to_string(),
            wallet_visible_root_count,
            mode: EvidenceMode::RedactedRootOnly,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "plan_id": self.plan_id,
            "dry_run_root": self.dry_run_root,
            "wallet_state_before_root": self.wallet_state_before_root,
            "wallet_state_after_root": self.wallet_state_after_root,
            "escape_route_root": self.escape_route_root,
            "fee_bound_root": self.fee_bound_root,
            "no_broadcast_root": self.no_broadcast_root,
            "receipt_root": self.receipt_root,
            "visible_receipt_root": self.visible_receipt_root,
            "wallet_visible_root_count": self.wallet_visible_root_count,
            "mode": self.mode.as_str(),
            "status": self.status.as_str(),
            "clears": self.status.clears(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wallet_receipt_plan", &self.public_record())
    }

    pub fn valid_for(&self, config: &Config) -> bool {
        self.status.clears()
            && self.wallet_visible_root_count >= config.min_wallet_visible_roots
            && non_empty(&self.dry_run_root)
            && non_empty(&self.wallet_state_before_root)
            && non_empty(&self.wallet_state_after_root)
            && non_empty(&self.escape_route_root)
            && non_empty(&self.fee_bound_root)
            && non_empty(&self.no_broadcast_root)
            && non_empty(&self.visible_receipt_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerReceiptPlan {
    pub plan_id: String,
    pub quorum_root: String,
    pub observation_set_root: String,
    pub challenge_window_root: String,
    pub challenge_replay_root: String,
    pub dissent_root: String,
    pub attestation_root: String,
    pub quorum_count: u64,
    pub dissent_count: u64,
    pub mode: EvidenceMode,
    pub status: ActionStatus,
}

impl WatchtowerReceiptPlan {
    pub fn new(
        quorum_root: &str,
        observation_set_root: &str,
        challenge_window_root: &str,
        challenge_replay_root: &str,
        dissent_root: &str,
        attestation_root: &str,
        quorum_count: u64,
        dissent_count: u64,
        status: ActionStatus,
    ) -> Self {
        let joined_root = merkle_root(
            "wave91-watchtower-receipt-plan",
            &[
                quorum_root,
                observation_set_root,
                challenge_window_root,
                challenge_replay_root,
                dissent_root,
                attestation_root,
            ],
        );
        let plan_id = action_id(
            ActionLane::WatchtowerQuorum,
            &joined_root,
            quorum_count,
            status,
        );
        Self {
            plan_id,
            quorum_root: quorum_root.to_string(),
            observation_set_root: observation_set_root.to_string(),
            challenge_window_root: challenge_window_root.to_string(),
            challenge_replay_root: challenge_replay_root.to_string(),
            dissent_root: dissent_root.to_string(),
            attestation_root: attestation_root.to_string(),
            quorum_count,
            dissent_count,
            mode: EvidenceMode::QuorumRootOnly,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "plan_id": self.plan_id,
            "quorum_root": self.quorum_root,
            "observation_set_root": self.observation_set_root,
            "challenge_window_root": self.challenge_window_root,
            "challenge_replay_root": self.challenge_replay_root,
            "dissent_root": self.dissent_root,
            "attestation_root": self.attestation_root,
            "quorum_count": self.quorum_count,
            "dissent_count": self.dissent_count,
            "mode": self.mode.as_str(),
            "status": self.status.as_str(),
            "clears": self.status.clears(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("watchtower_receipt_plan", &self.public_record())
    }

    pub fn valid_for(&self, config: &Config) -> bool {
        self.status.clears()
            && self.quorum_count >= config.min_watchtower_quorum
            && self.dissent_count == 0
            && non_empty(&self.quorum_root)
            && non_empty(&self.observation_set_root)
            && non_empty(&self.challenge_window_root)
            && non_empty(&self.challenge_replay_root)
            && non_empty(&self.dissent_root)
            && non_empty(&self.attestation_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecoveryReceiptPlan {
    pub plan_id: String,
    pub recovery_bundle_root: String,
    pub redaction_policy_root: String,
    pub custody_release_root: String,
    pub recovery_proof_root: String,
    pub privacy_review_root: String,
    pub recovery_proof_root_count: u64,
    pub mode: EvidenceMode,
    pub status: ActionStatus,
}

impl RecoveryReceiptPlan {
    pub fn new(
        recovery_bundle_root: &str,
        redaction_policy_root: &str,
        custody_release_root: &str,
        recovery_proof_root: &str,
        privacy_review_root: &str,
        recovery_proof_root_count: u64,
        status: ActionStatus,
    ) -> Self {
        let joined_root = merkle_root(
            "wave91-recovery-receipt-plan",
            &[
                recovery_bundle_root,
                redaction_policy_root,
                custody_release_root,
                recovery_proof_root,
                privacy_review_root,
            ],
        );
        let plan_id = action_id(
            ActionLane::RedactedRecoveryProof,
            &joined_root,
            recovery_proof_root_count,
            status,
        );
        Self {
            plan_id,
            recovery_bundle_root: recovery_bundle_root.to_string(),
            redaction_policy_root: redaction_policy_root.to_string(),
            custody_release_root: custody_release_root.to_string(),
            recovery_proof_root: recovery_proof_root.to_string(),
            privacy_review_root: privacy_review_root.to_string(),
            recovery_proof_root_count,
            mode: EvidenceMode::RedactedRootOnly,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "plan_id": self.plan_id,
            "recovery_bundle_root": self.recovery_bundle_root,
            "redaction_policy_root": self.redaction_policy_root,
            "custody_release_root": self.custody_release_root,
            "recovery_proof_root": self.recovery_proof_root,
            "privacy_review_root": self.privacy_review_root,
            "recovery_proof_root_count": self.recovery_proof_root_count,
            "mode": self.mode.as_str(),
            "status": self.status.as_str(),
            "clears": self.status.clears(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("recovery_receipt_plan", &self.public_record())
    }

    pub fn valid_for(&self, config: &Config) -> bool {
        self.status.clears()
            && self.recovery_proof_root_count >= config.min_recovery_proof_roots
            && non_empty(&self.recovery_bundle_root)
            && non_empty(&self.redaction_policy_root)
            && non_empty(&self.custody_release_root)
            && non_empty(&self.recovery_proof_root)
            && non_empty(&self.privacy_review_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserRunbookReplayPlan {
    pub replay_id: String,
    pub runbook_root: String,
    pub replay_trace_root: String,
    pub command_transcript_root: String,
    pub operator_hint_root: String,
    pub replay_step_count: u64,
    pub deterministic: bool,
    pub status: ActionStatus,
}

impl UserRunbookReplayPlan {
    pub fn new(
        runbook_root: &str,
        replay_trace_root: &str,
        command_transcript_root: &str,
        operator_hint_root: &str,
        replay_step_count: u64,
        status: ActionStatus,
    ) -> Self {
        let joined_root = merkle_root(
            "wave91-user-runbook-replay-plan",
            &[
                runbook_root,
                replay_trace_root,
                command_transcript_root,
                operator_hint_root,
            ],
        );
        let replay_id = action_id(
            ActionLane::UserRunbookReplay,
            &joined_root,
            replay_step_count,
            status,
        );
        Self {
            replay_id,
            runbook_root: runbook_root.to_string(),
            replay_trace_root: replay_trace_root.to_string(),
            command_transcript_root: command_transcript_root.to_string(),
            operator_hint_root: operator_hint_root.to_string(),
            replay_step_count,
            deterministic: true,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_id": self.replay_id,
            "runbook_root": self.runbook_root,
            "replay_trace_root": self.replay_trace_root,
            "command_transcript_root": self.command_transcript_root,
            "operator_hint_root": self.operator_hint_root,
            "replay_step_count": self.replay_step_count,
            "deterministic": self.deterministic,
            "status": self.status.as_str(),
            "clears": self.status.clears(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("user_runbook_replay_plan", &self.public_record())
    }

    pub fn valid_for(&self, config: &Config) -> bool {
        self.status.clears()
            && self.deterministic
            && self.replay_step_count >= config.min_user_replay_steps
            && non_empty(&self.runbook_root)
            && non_empty(&self.replay_trace_root)
            && non_empty(&self.command_transcript_root)
            && non_empty(&self.operator_hint_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommandHints {
    pub hints_id: String,
    pub dry_run_command_root: String,
    pub quorum_command_root: String,
    pub replay_command_root: String,
    pub recovery_command_root: String,
    pub receipt_command_root: String,
    pub signoff_command_root: String,
    pub redaction_notice_root: String,
}

impl OperatorCommandHints {
    pub fn new(
        dry_run_command_root: &str,
        quorum_command_root: &str,
        replay_command_root: &str,
        recovery_command_root: &str,
        receipt_command_root: &str,
        signoff_command_root: &str,
        redaction_notice_root: &str,
    ) -> Self {
        let hints_id = merkle_root(
            "wave91-operator-command-hints",
            &[
                dry_run_command_root,
                quorum_command_root,
                replay_command_root,
                recovery_command_root,
                receipt_command_root,
                signoff_command_root,
                redaction_notice_root,
            ],
        );
        Self {
            hints_id,
            dry_run_command_root: dry_run_command_root.to_string(),
            quorum_command_root: quorum_command_root.to_string(),
            replay_command_root: replay_command_root.to_string(),
            recovery_command_root: recovery_command_root.to_string(),
            receipt_command_root: receipt_command_root.to_string(),
            signoff_command_root: signoff_command_root.to_string(),
            redaction_notice_root: redaction_notice_root.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hints_id": self.hints_id,
            "dry_run_command_root": self.dry_run_command_root,
            "quorum_command_root": self.quorum_command_root,
            "replay_command_root": self.replay_command_root,
            "recovery_command_root": self.recovery_command_root,
            "receipt_command_root": self.receipt_command_root,
            "signoff_command_root": self.signoff_command_root,
            "redaction_notice_root": self.redaction_notice_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator_command_hints", &self.public_record())
    }

    pub fn valid(&self) -> bool {
        non_empty(&self.dry_run_command_root)
            && non_empty(&self.quorum_command_root)
            && non_empty(&self.replay_command_root)
            && non_empty(&self.recovery_command_root)
            && non_empty(&self.receipt_command_root)
            && non_empty(&self.signoff_command_root)
            && non_empty(&self.redaction_notice_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorSignoffPlan {
    pub signoff_id: String,
    pub signer_set_root: String,
    pub authority_policy_root: String,
    pub clearance_packet_root: String,
    pub no_payload_attestation_root: String,
    pub signoff_count: u64,
    pub status: ActionStatus,
}

impl OperatorSignoffPlan {
    pub fn new(
        signer_set_root: &str,
        authority_policy_root: &str,
        clearance_packet_root: &str,
        no_payload_attestation_root: &str,
        signoff_count: u64,
        status: ActionStatus,
    ) -> Self {
        let joined_root = merkle_root(
            "wave91-operator-signoff-plan",
            &[
                signer_set_root,
                authority_policy_root,
                clearance_packet_root,
                no_payload_attestation_root,
            ],
        );
        let signoff_id = action_id(
            ActionLane::OperatorSignoff,
            &joined_root,
            signoff_count,
            status,
        );
        Self {
            signoff_id,
            signer_set_root: signer_set_root.to_string(),
            authority_policy_root: authority_policy_root.to_string(),
            clearance_packet_root: clearance_packet_root.to_string(),
            no_payload_attestation_root: no_payload_attestation_root.to_string(),
            signoff_count,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signoff_id": self.signoff_id,
            "signer_set_root": self.signer_set_root,
            "authority_policy_root": self.authority_policy_root,
            "clearance_packet_root": self.clearance_packet_root,
            "no_payload_attestation_root": self.no_payload_attestation_root,
            "signoff_count": self.signoff_count,
            "status": self.status.as_str(),
            "clears": self.status.clears(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator_signoff_plan", &self.public_record())
    }

    pub fn valid_for(&self, config: &Config) -> bool {
        self.status.clears()
            && self.signoff_count >= config.min_operator_signers
            && non_empty(&self.signer_set_root)
            && non_empty(&self.authority_policy_root)
            && non_empty(&self.clearance_packet_root)
            && non_empty(&self.no_payload_attestation_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptanceCriteria {
    pub criteria_id: String,
    pub denial_root_intake_required: bool,
    pub wallet_escape_dry_run_required: bool,
    pub watchtower_quorum_required: bool,
    pub user_runbook_replay_required: bool,
    pub redacted_recovery_proof_required: bool,
    pub wallet_visible_receipt_required: bool,
    pub operator_signoff_required: bool,
    pub roots_only_required: bool,
    pub fail_closed_required: bool,
}

impl AcceptanceCriteria {
    pub fn from_config(config: &Config) -> Self {
        let criteria_id = domain_hash(
            "wave91-acceptance-criteria",
            &[
                HashPart::Str(&config.chain_id),
                HashPart::Str(&config.protocol_version),
                HashPart::U64(config.required_action_count()),
                HashPart::Str(bool_str(config.require_roots_only_privacy)),
                HashPart::Str(bool_str(config.fail_closed_on_any_gap)),
            ],
        );
        Self {
            criteria_id,
            denial_root_intake_required: config.require_wave90_denial_intake,
            wallet_escape_dry_run_required: config.require_wallet_escape_dry_run,
            watchtower_quorum_required: config.require_watchtower_quorum,
            user_runbook_replay_required: config.require_user_runbook_replay,
            redacted_recovery_proof_required: config.require_redacted_recovery_proof,
            wallet_visible_receipt_required: config.require_wallet_visible_receipt,
            operator_signoff_required: config.require_operator_signoff,
            roots_only_required: config.require_roots_only_privacy,
            fail_closed_required: config.fail_closed_on_any_gap,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "criteria_id": self.criteria_id,
            "denial_root_intake_required": self.denial_root_intake_required,
            "wallet_escape_dry_run_required": self.wallet_escape_dry_run_required,
            "watchtower_quorum_required": self.watchtower_quorum_required,
            "user_runbook_replay_required": self.user_runbook_replay_required,
            "redacted_recovery_proof_required": self.redacted_recovery_proof_required,
            "wallet_visible_receipt_required": self.wallet_visible_receipt_required,
            "operator_signoff_required": self.operator_signoff_required,
            "roots_only_required": self.roots_only_required,
            "fail_closed_required": self.fail_closed_required,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("acceptance_criteria", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClearanceVerdict {
    pub verdict_id: String,
    pub kind: ClearanceVerdictKind,
    pub denial_intake_clear: bool,
    pub wallet_escape_clear: bool,
    pub watchtower_quorum_clear: bool,
    pub user_runbook_clear: bool,
    pub recovery_proof_clear: bool,
    pub wallet_visible_clear: bool,
    pub operator_signoff_clear: bool,
    pub roots_only_clear: bool,
    pub cleared_action_count: u64,
    pub missing_action_count: u64,
    pub release_live_heavy_gate: bool,
    pub operator_message: String,
}

impl ClearanceVerdict {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        denial_intake_clear: bool,
        wallet_escape_clear: bool,
        watchtower_quorum_clear: bool,
        user_runbook_clear: bool,
        recovery_proof_clear: bool,
        wallet_visible_clear: bool,
        operator_signoff_clear: bool,
        roots_only_clear: bool,
    ) -> Self {
        let flags = [
            denial_intake_clear,
            wallet_escape_clear,
            watchtower_quorum_clear,
            user_runbook_clear,
            recovery_proof_clear,
            wallet_visible_clear,
            operator_signoff_clear,
        ];
        let cleared_action_count = flags.iter().filter(|flag| **flag).count() as u64;
        let missing_action_count = flags.len() as u64 - cleared_action_count;
        let all_clear = missing_action_count == 0 && roots_only_clear;
        let kind = if all_clear {
            ClearanceVerdictKind::Clear
        } else {
            ClearanceVerdictKind::FailClosed
        };
        let operator_message = if all_clear {
            "live heavy gate receipt lane clear for wallet watchtower blockers".to_string()
        } else {
            "fail closed until every wallet watchtower receipt root is accepted".to_string()
        };
        let verdict_id = domain_hash(
            "wave91-clearance-verdict",
            &[
                HashPart::Str(kind.as_str()),
                HashPart::Str(bool_str(denial_intake_clear)),
                HashPart::Str(bool_str(wallet_escape_clear)),
                HashPart::Str(bool_str(watchtower_quorum_clear)),
                HashPart::Str(bool_str(user_runbook_clear)),
                HashPart::Str(bool_str(recovery_proof_clear)),
                HashPart::Str(bool_str(wallet_visible_clear)),
                HashPart::Str(bool_str(operator_signoff_clear)),
                HashPart::Str(bool_str(roots_only_clear)),
                HashPart::U64(cleared_action_count),
                HashPart::U64(missing_action_count),
            ],
        );
        Self {
            verdict_id,
            kind,
            denial_intake_clear,
            wallet_escape_clear,
            watchtower_quorum_clear,
            user_runbook_clear,
            recovery_proof_clear,
            wallet_visible_clear,
            operator_signoff_clear,
            roots_only_clear,
            cleared_action_count,
            missing_action_count,
            release_live_heavy_gate: all_clear,
            operator_message,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "kind": self.kind.as_str(),
            "denial_intake_clear": self.denial_intake_clear,
            "wallet_escape_clear": self.wallet_escape_clear,
            "watchtower_quorum_clear": self.watchtower_quorum_clear,
            "user_runbook_clear": self.user_runbook_clear,
            "recovery_proof_clear": self.recovery_proof_clear,
            "wallet_visible_clear": self.wallet_visible_clear,
            "operator_signoff_clear": self.operator_signoff_clear,
            "roots_only_clear": self.roots_only_clear,
            "cleared_action_count": self.cleared_action_count,
            "missing_action_count": self.missing_action_count,
            "release_live_heavy_gate": self.release_live_heavy_gate,
            "operator_message": self.operator_message,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("clearance_verdict", &self.public_record())
    }

    pub fn is_clear(&self) -> bool {
        self.kind == ClearanceVerdictKind::Clear && self.release_live_heavy_gate
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRecord {
    pub config_root: String,
    pub denial_intake_root: String,
    pub wallet_plan_root: String,
    pub watchtower_plan_root: String,
    pub runbook_replay_root: String,
    pub recovery_plan_root: String,
    pub command_hints_root: String,
    pub operator_signoff_root: String,
    pub acceptance_criteria_root: String,
    pub clearance_verdict_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "denial_intake_root": self.denial_intake_root,
            "wallet_plan_root": self.wallet_plan_root,
            "watchtower_plan_root": self.watchtower_plan_root,
            "runbook_replay_root": self.runbook_replay_root,
            "recovery_plan_root": self.recovery_plan_root,
            "command_hints_root": self.command_hints_root,
            "operator_signoff_root": self.operator_signoff_root,
            "acceptance_criteria_root": self.acceptance_criteria_root,
            "clearance_verdict_root": self.clearance_verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub denial_intake: DenialRootIntake,
    pub wallet_plan: WalletReceiptPlan,
    pub watchtower_plan: WatchtowerReceiptPlan,
    pub runbook_replay: UserRunbookReplayPlan,
    pub recovery_plan: RecoveryReceiptPlan,
    pub command_hints: OperatorCommandHints,
    pub operator_signoff: OperatorSignoffPlan,
    pub acceptance_criteria: AcceptanceCriteria,
    pub clearance_verdict: ClearanceVerdict,
}

impl State {
    pub fn new(
        config: Config,
        denial_intake: DenialRootIntake,
        wallet_plan: WalletReceiptPlan,
        watchtower_plan: WatchtowerReceiptPlan,
        runbook_replay: UserRunbookReplayPlan,
        recovery_plan: RecoveryReceiptPlan,
        command_hints: OperatorCommandHints,
        operator_signoff: OperatorSignoffPlan,
    ) -> Self {
        let acceptance_criteria = AcceptanceCriteria::from_config(&config);
        let denial_intake_clear = !config.require_wave90_denial_intake || denial_intake.valid();
        let wallet_escape_clear =
            !config.require_wallet_escape_dry_run || wallet_plan.valid_for(&config);
        let watchtower_quorum_clear =
            !config.require_watchtower_quorum || watchtower_plan.valid_for(&config);
        let user_runbook_clear =
            !config.require_user_runbook_replay || runbook_replay.valid_for(&config);
        let recovery_proof_clear =
            !config.require_redacted_recovery_proof || recovery_plan.valid_for(&config);
        let wallet_visible_clear =
            !config.require_wallet_visible_receipt || wallet_plan.valid_for(&config);
        let operator_signoff_clear =
            !config.require_operator_signoff || operator_signoff.valid_for(&config);
        let roots_only_clear = !config.require_roots_only_privacy
            || (denial_intake.roots_only
                && wallet_plan.mode == EvidenceMode::RedactedRootOnly
                && watchtower_plan.mode == EvidenceMode::QuorumRootOnly
                && recovery_plan.mode == EvidenceMode::RedactedRootOnly
                && command_hints.valid()
                && non_empty(&operator_signoff.no_payload_attestation_root));
        let clearance_verdict = ClearanceVerdict::new(
            denial_intake_clear,
            wallet_escape_clear,
            watchtower_quorum_clear,
            user_runbook_clear,
            recovery_proof_clear,
            wallet_visible_clear,
            operator_signoff_clear,
            roots_only_clear,
        );
        Self {
            config,
            denial_intake,
            wallet_plan,
            watchtower_plan,
            runbook_replay,
            recovery_plan,
            command_hints,
            operator_signoff,
            acceptance_criteria,
            clearance_verdict,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let config_root = self.config.state_root();
        let denial_intake_root = self.denial_intake.state_root();
        let wallet_plan_root = self.wallet_plan.state_root();
        let watchtower_plan_root = self.watchtower_plan.state_root();
        let runbook_replay_root = self.runbook_replay.state_root();
        let recovery_plan_root = self.recovery_plan.state_root();
        let command_hints_root = self.command_hints.state_root();
        let operator_signoff_root = self.operator_signoff.state_root();
        let acceptance_criteria_root = self.acceptance_criteria.state_root();
        let clearance_verdict_root = self.clearance_verdict.state_root();
        let state_root = merkle_root(
            "wave91-live-heavy-gate-state",
            &[
                &config_root,
                &denial_intake_root,
                &wallet_plan_root,
                &watchtower_plan_root,
                &runbook_replay_root,
                &recovery_plan_root,
                &command_hints_root,
                &operator_signoff_root,
                &acceptance_criteria_root,
                &clearance_verdict_root,
            ],
        );
        PublicRecord {
            config_root,
            denial_intake_root,
            wallet_plan_root,
            watchtower_plan_root,
            runbook_replay_root,
            recovery_plan_root,
            command_hints_root,
            operator_signoff_root,
            acceptance_criteria_root,
            clearance_verdict_root,
            state_root,
        }
    }

    pub fn state_root(&self) -> String {
        self.public_record().state_root
    }

    pub fn fail_closed_clearance_verdict(&self) -> ClearanceVerdictKind {
        self.clearance_verdict.kind
    }

    pub fn release_live_heavy_gate(&self) -> bool {
        self.clearance_verdict.is_clear()
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let denial_intake = DenialRootIntake::new(
        &sample_root("wave90-denial-output-root"),
        &sample_root("wave90-denial-manifest-root"),
        &sample_root("wave90-blocker-root"),
        &sample_root("wave90-source-run-root"),
        91,
        vec![
            DenialBlocker::WalletEscapeDryRunMissing,
            DenialBlocker::WatchtowerQuorumMissing,
            DenialBlocker::UserRunbookReplayMissing,
            DenialBlocker::RedactedRecoveryProofMissing,
            DenialBlocker::WalletVisibleReceiptMissing,
            DenialBlocker::OperatorSignoffMissing,
        ],
    );
    let wallet_plan = WalletReceiptPlan::new(
        &sample_root("wallet-escape-dry-run"),
        &sample_root("wallet-state-before"),
        &sample_root("wallet-state-after"),
        &sample_root("escape-route"),
        &sample_root("fee-bound"),
        &sample_root("no-broadcast"),
        &sample_root("wallet-visible-receipt"),
        DEFAULT_MIN_WALLET_VISIBLE_ROOTS,
        ActionStatus::Accepted,
    );
    let watchtower_plan = WatchtowerReceiptPlan::new(
        &sample_root("watchtower-quorum"),
        &sample_root("watchtower-observation-set"),
        &sample_root("challenge-window"),
        &sample_root("challenge-replay"),
        &sample_root("zero-dissent"),
        &sample_root("watchtower-attestation"),
        DEFAULT_MIN_WATCHTOWER_QUORUM,
        0,
        ActionStatus::Accepted,
    );
    let runbook_replay = UserRunbookReplayPlan::new(
        &sample_root("user-runbook"),
        &sample_root("runbook-replay-trace"),
        &sample_root("command-transcript"),
        &sample_root("operator-hints"),
        DEFAULT_MIN_USER_REPLAY_STEPS,
        ActionStatus::Accepted,
    );
    let recovery_plan = RecoveryReceiptPlan::new(
        &sample_root("recovery-bundle"),
        &sample_root("redaction-policy"),
        &sample_root("custody-release"),
        &sample_root("recovery-proof"),
        &sample_root("privacy-review"),
        DEFAULT_MIN_RECOVERY_PROOF_ROOTS,
        ActionStatus::Accepted,
    );
    let command_hints = OperatorCommandHints::new(
        &sample_root("hint-dry-run-command"),
        &sample_root("hint-quorum-command"),
        &sample_root("hint-replay-command"),
        &sample_root("hint-recovery-command"),
        &sample_root("hint-receipt-command"),
        &sample_root("hint-signoff-command"),
        &sample_root("hint-redaction-notice"),
    );
    let operator_signoff = OperatorSignoffPlan::new(
        &sample_root("operator-signer-set"),
        &sample_root("operator-authority-policy"),
        &sample_root("clearance-packet"),
        &sample_root("no-payload-attestation"),
        DEFAULT_MIN_OPERATOR_SIGNERS,
        ActionStatus::Accepted,
    );
    State::new(
        config,
        denial_intake,
        wallet_plan,
        watchtower_plan,
        runbook_replay,
        recovery_plan,
        command_hints,
        operator_signoff,
    )
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn denial_intake_id(
    denial_output_root: &str,
    denial_manifest_root: &str,
    blocker_root: &str,
    source_run_root: &str,
    source_height: u64,
    blocker_count: u64,
) -> String {
    domain_hash(
        "wave91-denial-root-intake-id",
        &[
            HashPart::Str(denial_output_root),
            HashPart::Str(denial_manifest_root),
            HashPart::Str(blocker_root),
            HashPart::Str(source_run_root),
            HashPart::U64(source_height),
            HashPart::U64(blocker_count),
        ],
    )
}

pub fn action_id(
    lane: ActionLane,
    action_root: &str,
    witness_count: u64,
    status: ActionStatus,
) -> String {
    domain_hash(
        "wave91-live-heavy-gate-action-id",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(action_root),
            HashPart::U64(witness_count),
            HashPart::Str(status.as_str()),
        ],
    )
}

pub fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "wave91-live-heavy-gate-record-root",
        &[HashPart::Str(label), HashPart::Str(&record.to_string())],
    )
}

pub fn sample_root(label: &str) -> String {
    domain_hash(
        "wave91-live-heavy-gate-sample-root",
        &[
            HashPart::Str(WAVE_LABEL),
            HashPart::Str(LIVE_HEAVY_GATE_LANE),
            HashPart::Str(label),
        ],
    )
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

fn domain_hash(domain: &str, parts: &[HashPart<'_>]) -> String {
    crate_domain_hash(domain, parts, 32)
}

fn merkle_root(domain: &str, leaves: &[&str]) -> String {
    let values = leaves.iter().map(|leaf| json!(leaf)).collect::<Vec<_>>();
    crate_merkle_root(domain, &values)
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
