use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_execution_replay_bundle_runtime as replay_bundle,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_evidence_dossier_runtime as dossier,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceReplayPlanRuntimeResult<T> =
    Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_REPLAY_PLAN_RUNTIME_PROTOCOL_VERSION:
    &str = "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-replay-plan-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_REPLAY_PLAN_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-replay-plan";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub replay_plan_suite: String,
    pub replay_policy: String,
    pub min_replay_steps: u64,
    pub min_blocker_cases: u64,
    pub require_answer_dossier: u64,
    pub require_replay_bundle: u64,
    pub require_deposit_lock_replay: u64,
    pub require_private_note_replay: u64,
    pub require_private_action_replay: u64,
    pub require_settlement_receipt_replay: u64,
    pub require_withdrawal_claim_replay: u64,
    pub require_force_exit_liveness_replay: u64,
    pub require_pq_authority_replay: u64,
    pub require_privacy_boundary_replay: u64,
    pub require_operator_independence: u64,
    pub require_wallet_reconstructable: u64,
    pub require_release_hold: u64,
    pub production_release_allowed: u64,
    pub max_public_metadata_units: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            replay_plan_suite: "monero-l2-pq-bridge-exit-user-escape-replay-plan-v1".to_string(),
            replay_policy: "wallet-replayable-operator-independent-release-held-v1".to_string(),
            min_replay_steps: 12,
            min_blocker_cases: 9,
            require_answer_dossier: 1,
            require_replay_bundle: 1,
            require_deposit_lock_replay: 1,
            require_private_note_replay: 1,
            require_private_action_replay: 1,
            require_settlement_receipt_replay: 1,
            require_withdrawal_claim_replay: 1,
            require_force_exit_liveness_replay: 1,
            require_pq_authority_replay: 1,
            require_privacy_boundary_replay: 1,
            require_operator_independence: 1,
            require_wallet_reconstructable: 1,
            require_release_hold: 1,
            production_release_allowed: 0,
            max_public_metadata_units: 0,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "replay_plan_suite": self.replay_plan_suite,
            "replay_policy": self.replay_policy,
            "min_replay_steps": self.min_replay_steps,
            "min_blocker_cases": self.min_blocker_cases,
            "require_answer_dossier": self.require_answer_dossier,
            "require_replay_bundle": self.require_replay_bundle,
            "require_deposit_lock_replay": self.require_deposit_lock_replay,
            "require_private_note_replay": self.require_private_note_replay,
            "require_private_action_replay": self.require_private_action_replay,
            "require_settlement_receipt_replay": self.require_settlement_receipt_replay,
            "require_withdrawal_claim_replay": self.require_withdrawal_claim_replay,
            "require_force_exit_liveness_replay": self.require_force_exit_liveness_replay,
            "require_pq_authority_replay": self.require_pq_authority_replay,
            "require_privacy_boundary_replay": self.require_privacy_boundary_replay,
            "require_operator_independence": self.require_operator_independence,
            "require_wallet_reconstructable": self.require_wallet_reconstructable,
            "require_release_hold": self.require_release_hold,
            "production_release_allowed": self.production_release_allowed,
            "max_public_metadata_units": self.max_public_metadata_units,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPlanStepKind {
    AnswerDossierRoot,
    ExecutionReplayBundleRoot,
    DepositLockReplay,
    PrivateNoteReplay,
    PrivateActionReplay,
    SettlementReceiptReplay,
    WithdrawalClaimReplay,
    ForceExitLivenessReplay,
    PqAuthorityReplay,
    PrivacyBoundaryReplay,
    ReleaseBlockerReplay,
    ProductionHoldReplay,
}

impl ReplayPlanStepKind {
    pub fn ordered() -> [Self; 12] {
        [
            Self::AnswerDossierRoot,
            Self::ExecutionReplayBundleRoot,
            Self::DepositLockReplay,
            Self::PrivateNoteReplay,
            Self::PrivateActionReplay,
            Self::SettlementReceiptReplay,
            Self::WithdrawalClaimReplay,
            Self::ForceExitLivenessReplay,
            Self::PqAuthorityReplay,
            Self::PrivacyBoundaryReplay,
            Self::ReleaseBlockerReplay,
            Self::ProductionHoldReplay,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::AnswerDossierRoot => "answer_dossier_root",
            Self::ExecutionReplayBundleRoot => "execution_replay_bundle_root",
            Self::DepositLockReplay => "deposit_lock_replay",
            Self::PrivateNoteReplay => "private_note_replay",
            Self::PrivateActionReplay => "private_action_replay",
            Self::SettlementReceiptReplay => "settlement_receipt_replay",
            Self::WithdrawalClaimReplay => "withdrawal_claim_replay",
            Self::ForceExitLivenessReplay => "force_exit_liveness_replay",
            Self::PqAuthorityReplay => "pq_authority_replay",
            Self::PrivacyBoundaryReplay => "privacy_boundary_replay",
            Self::ReleaseBlockerReplay => "release_blocker_replay",
            Self::ProductionHoldReplay => "production_hold_replay",
        }
    }

    pub fn command_name(self) -> &'static str {
        match self {
            Self::AnswerDossierRoot => "load_answer_vertical_slice_dossier",
            Self::ExecutionReplayBundleRoot => "load_canonical_execution_replay_bundle",
            Self::DepositLockReplay => "replay_monero_deposit_lock",
            Self::PrivateNoteReplay => "replay_private_note_state",
            Self::PrivateActionReplay => "replay_private_transfer_or_contract_action",
            Self::SettlementReceiptReplay => "replay_settlement_receipt",
            Self::WithdrawalClaimReplay => "replay_withdrawal_claim",
            Self::ForceExitLivenessReplay => "replay_force_exit_liveness_window",
            Self::PqAuthorityReplay => "replay_pq_authority_quorum",
            Self::PrivacyBoundaryReplay => "replay_roots_only_privacy_boundary",
            Self::ReleaseBlockerReplay => "replay_release_blocker_clearing_order",
            Self::ProductionHoldReplay => "assert_production_release_hold",
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::AnswerDossierRoot => "answer_owner",
            Self::ExecutionReplayBundleRoot => "runtime_replay_owner",
            Self::DepositLockReplay => "monero_watcher_owner",
            Self::PrivateNoteReplay => "private_state_owner",
            Self::PrivateActionReplay => "private_execution_owner",
            Self::SettlementReceiptReplay => "settlement_owner",
            Self::WithdrawalClaimReplay => "wallet_recovery_owner",
            Self::ForceExitLivenessReplay => "liveness_owner",
            Self::PqAuthorityReplay => "pq_authority_owner",
            Self::PrivacyBoundaryReplay => "privacy_owner",
            Self::ReleaseBlockerReplay => "release_governance_owner",
            Self::ProductionHoldReplay => "release_owner",
        }
    }

    pub fn wallet_critical(self) -> bool {
        matches!(
            self,
            Self::DepositLockReplay
                | Self::PrivateNoteReplay
                | Self::PrivateActionReplay
                | Self::SettlementReceiptReplay
                | Self::WithdrawalClaimReplay
                | Self::ForceExitLivenessReplay
                | Self::PqAuthorityReplay
                | Self::PrivacyBoundaryReplay
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPlanBlockerKind {
    CargoRuntimeDeferred,
    HeavyGateNotExecuted,
    SecurityAuditDeferred,
    LiveFeedSubstitutionMissing,
    MoneroVerifierMissing,
    ReserveLiquidityProofMissing,
    PqAuthorityReceiptMissing,
    PrivacyLeakReviewDeferred,
    ProductionReleaseHeld,
}

impl ReplayPlanBlockerKind {
    pub fn ordered() -> [Self; 9] {
        [
            Self::CargoRuntimeDeferred,
            Self::HeavyGateNotExecuted,
            Self::SecurityAuditDeferred,
            Self::LiveFeedSubstitutionMissing,
            Self::MoneroVerifierMissing,
            Self::ReserveLiquidityProofMissing,
            Self::PqAuthorityReceiptMissing,
            Self::PrivacyLeakReviewDeferred,
            Self::ProductionReleaseHeld,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "cargo_runtime_deferred",
            Self::HeavyGateNotExecuted => "heavy_gate_not_executed",
            Self::SecurityAuditDeferred => "security_audit_deferred",
            Self::LiveFeedSubstitutionMissing => "live_feed_substitution_missing",
            Self::MoneroVerifierMissing => "monero_verifier_missing",
            Self::ReserveLiquidityProofMissing => "reserve_liquidity_proof_missing",
            Self::PqAuthorityReceiptMissing => "pq_authority_receipt_missing",
            Self::PrivacyLeakReviewDeferred => "privacy_leak_review_deferred",
            Self::ProductionReleaseHeld => "production_release_held",
        }
    }

    pub fn clearance(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "run cargo check/test/clippy and runtime replay gates",
            Self::HeavyGateNotExecuted => {
                "execute every replay command against observed runtime roots"
            }
            Self::SecurityAuditDeferred => "attach signed security and privacy audit receipts",
            Self::LiveFeedSubstitutionMissing => {
                "replace fixture roots with live Monero watcher, reserve, and settlement feed roots"
            }
            Self::MoneroVerifierMissing => {
                "bind production Monero finality, reorg, and watcher-collusion verifier receipts"
            }
            Self::ReserveLiquidityProofMissing => {
                "prove forced-exit reserve coverage and fair queue settlement under stress"
            }
            Self::PqAuthorityReceiptMissing => {
                "attach PQ authority quorum, epoch freshness, and withdrawal authorization receipts"
            }
            Self::PrivacyLeakReviewDeferred => {
                "close roots-only export, timing, amount, and wallet-linkage privacy review"
            }
            Self::ProductionReleaseHeld => {
                "keep production release disabled until every blocker clears"
            }
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "runtime_owner",
            Self::HeavyGateNotExecuted => "heavy_gate_owner",
            Self::SecurityAuditDeferred => "audit_owner",
            Self::LiveFeedSubstitutionMissing => "live_feed_owner",
            Self::MoneroVerifierMissing => "monero_watcher_owner",
            Self::ReserveLiquidityProofMissing => "reserve_owner",
            Self::PqAuthorityReceiptMissing => "pq_authority_owner",
            Self::PrivacyLeakReviewDeferred => "privacy_owner",
            Self::ProductionReleaseHeld => "release_owner",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBundle {
    pub dossier_state_root: String,
    pub dossier_root: String,
    pub dossier_evidence_root: String,
    pub dossier_answer_binding_root: String,
    pub dossier_vertical_evidence_root: String,
    pub dossier_production_hold_root: String,
    pub dossier_verdict_root: String,
    pub dossier_status: String,
    pub dossier_user_escape_supported: u64,
    pub dossier_live_execution_deferred_count: u64,
    pub dossier_audit_deferred_count: u64,
    pub dossier_release_blocker_count: u64,
    pub dossier_pq_authority_bound: u64,
    pub dossier_privacy_boundary_bound: u64,
    pub dossier_force_exit_liveness_bound: u64,
    pub dossier_roots_only_public_export: u64,
    pub replay_state_root: String,
    pub replay_bundle_root: String,
    pub replay_item_root: String,
    pub replay_root: String,
    pub replay_public_root: String,
    pub replay_committed_root: String,
    pub replay_encrypted_root: String,
    pub replay_wallet_recovery_root: String,
    pub replay_blocker_root: String,
    pub replay_verdict: String,
    pub replay_user_escape_replayable: u64,
    pub replay_production_blocked: u64,
    pub replay_required_count: u64,
    pub replay_wallet_critical_count: u64,
    pub replay_wallet_safe_count: u64,
    pub replay_deferred_count: u64,
    pub replay_watch_count: u64,
    pub replay_blocked_count: u64,
    pub replay_rejected_count: u64,
    pub replay_production_safe_count: u64,
    pub source_root: String,
}

impl SourceBundle {
    pub fn devnet() -> Self {
        let dossier = dossier::devnet();
        let replay = replay_bundle::devnet();
        Self::from_states(&dossier, &replay)
    }

    pub fn from_states(dossier: &dossier::State, replay: &replay_bundle::State) -> Self {
        let source_root = source_bundle_root(dossier, replay);
        Self {
            dossier_state_root: dossier.state_root(),
            dossier_root: dossier.dossier_root.clone(),
            dossier_evidence_root: dossier.evidence_root.clone(),
            dossier_answer_binding_root: dossier.answer_binding_root.clone(),
            dossier_vertical_evidence_root: dossier.vertical_evidence_root.clone(),
            dossier_production_hold_root: dossier.production_hold_root.clone(),
            dossier_verdict_root: dossier.verdict.verdict_root.clone(),
            dossier_status: dossier.verdict.dossier_status.clone(),
            dossier_user_escape_supported: dossier.verdict.user_escape_answer_supported,
            dossier_live_execution_deferred_count: dossier.verdict.live_execution_deferred_count,
            dossier_audit_deferred_count: dossier.verdict.audit_deferred_count,
            dossier_release_blocker_count: dossier.verdict.release_blocker_count,
            dossier_pq_authority_bound: dossier.verdict.pq_authority_bound,
            dossier_privacy_boundary_bound: dossier.verdict.privacy_boundary_bound,
            dossier_force_exit_liveness_bound: dossier.verdict.force_exit_liveness_bound,
            dossier_roots_only_public_export: dossier.verdict.roots_only_public_export,
            replay_state_root: replay.state_root(),
            replay_bundle_root: replay.bundle.bundle_root.clone(),
            replay_item_root: replay.bundle.item_root.clone(),
            replay_root: replay.bundle.replay_root.clone(),
            replay_public_root: replay.bundle.public_root.clone(),
            replay_committed_root: replay.bundle.committed_root.clone(),
            replay_encrypted_root: replay.bundle.encrypted_root.clone(),
            replay_wallet_recovery_root: replay.bundle.wallet_recovery_root.clone(),
            replay_blocker_root: replay.bundle.blocker_root.clone(),
            replay_verdict: replay.bundle.verdict.as_str().to_string(),
            replay_user_escape_replayable: bool_to_u64(replay.user_escape_replayable()),
            replay_production_blocked: bool_to_u64(replay.production_blocked()),
            replay_required_count: replay.bundle.counters.required,
            replay_wallet_critical_count: replay.bundle.counters.wallet_critical,
            replay_wallet_safe_count: replay.bundle.counters.wallet_safe,
            replay_deferred_count: replay.bundle.counters.deferred,
            replay_watch_count: replay.bundle.counters.watch,
            replay_blocked_count: replay.bundle.counters.blocked,
            replay_rejected_count: replay.bundle.counters.rejected,
            replay_production_safe_count: replay.bundle.counters.production_safe,
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "dossier_state_root": self.dossier_state_root,
            "dossier_root": self.dossier_root,
            "dossier_evidence_root": self.dossier_evidence_root,
            "dossier_answer_binding_root": self.dossier_answer_binding_root,
            "dossier_vertical_evidence_root": self.dossier_vertical_evidence_root,
            "dossier_production_hold_root": self.dossier_production_hold_root,
            "dossier_verdict_root": self.dossier_verdict_root,
            "dossier_status": self.dossier_status,
            "dossier_user_escape_supported": self.dossier_user_escape_supported,
            "dossier_live_execution_deferred_count": self.dossier_live_execution_deferred_count,
            "dossier_audit_deferred_count": self.dossier_audit_deferred_count,
            "dossier_release_blocker_count": self.dossier_release_blocker_count,
            "dossier_pq_authority_bound": self.dossier_pq_authority_bound,
            "dossier_privacy_boundary_bound": self.dossier_privacy_boundary_bound,
            "dossier_force_exit_liveness_bound": self.dossier_force_exit_liveness_bound,
            "dossier_roots_only_public_export": self.dossier_roots_only_public_export,
            "replay_state_root": self.replay_state_root,
            "replay_bundle_root": self.replay_bundle_root,
            "replay_item_root": self.replay_item_root,
            "replay_root": self.replay_root,
            "replay_public_root": self.replay_public_root,
            "replay_committed_root": self.replay_committed_root,
            "replay_encrypted_root": self.replay_encrypted_root,
            "replay_wallet_recovery_root": self.replay_wallet_recovery_root,
            "replay_blocker_root": self.replay_blocker_root,
            "replay_verdict": self.replay_verdict,
            "replay_user_escape_replayable": self.replay_user_escape_replayable,
            "replay_production_blocked": self.replay_production_blocked,
            "replay_required_count": self.replay_required_count,
            "replay_wallet_critical_count": self.replay_wallet_critical_count,
            "replay_wallet_safe_count": self.replay_wallet_safe_count,
            "replay_deferred_count": self.replay_deferred_count,
            "replay_watch_count": self.replay_watch_count,
            "replay_blocked_count": self.replay_blocked_count,
            "replay_rejected_count": self.replay_rejected_count,
            "replay_production_safe_count": self.replay_production_safe_count,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPlanStep {
    pub ordinal: u64,
    pub step_kind: ReplayPlanStepKind,
    pub command_name: String,
    pub owner: String,
    pub input_root: String,
    pub expected_output_root: String,
    pub wallet_critical: u64,
    pub operator_independent_required: u64,
    pub wallet_reconstructable_required: u64,
    pub pq_authority_required: u64,
    pub roots_only_public_export: u64,
    pub user_escape_ready: u64,
    pub production_release_blocked: u64,
    pub replay_command_root: String,
    pub expected_receipt_root: String,
    pub replay_plan_step_root: String,
}

impl ReplayPlanStep {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        step_kind: ReplayPlanStepKind,
        ordinal: u64,
    ) -> Self {
        let input_root = step_input_root(source, step_kind);
        let expected_output_root = step_expected_output_root(source, step_kind);
        let wallet_critical = bool_to_u64(step_kind.wallet_critical());
        let operator_independent_required = config.require_operator_independence;
        let wallet_reconstructable_required = config.require_wallet_reconstructable;
        let pq_authority_required = step_pq_required(step_kind, config);
        let roots_only_public_export = config.require_privacy_boundary_replay;
        let user_escape_ready = step_user_escape_ready(config, source, step_kind);
        let production_release_blocked = step_production_blocked(config, source, step_kind);
        let replay_command_root = replay_command_root(
            config,
            source,
            step_kind,
            ordinal,
            &input_root,
            &expected_output_root,
            user_escape_ready,
        );
        let expected_receipt_root = expected_receipt_root(
            config,
            source,
            step_kind,
            &replay_command_root,
            production_release_blocked,
        );
        let replay_plan_step_root = replay_plan_step_root(
            config,
            source,
            step_kind,
            ordinal,
            &input_root,
            &expected_output_root,
            &replay_command_root,
            &expected_receipt_root,
        );
        Self {
            ordinal,
            step_kind,
            command_name: step_kind.command_name().to_string(),
            owner: step_kind.owner().to_string(),
            input_root,
            expected_output_root,
            wallet_critical,
            operator_independent_required,
            wallet_reconstructable_required,
            pq_authority_required,
            roots_only_public_export,
            user_escape_ready,
            production_release_blocked,
            replay_command_root,
            expected_receipt_root,
            replay_plan_step_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "step_kind": self.step_kind.as_str(),
            "command_name": self.command_name,
            "owner": self.owner,
            "input_root": self.input_root,
            "expected_output_root": self.expected_output_root,
            "wallet_critical": self.wallet_critical,
            "operator_independent_required": self.operator_independent_required,
            "wallet_reconstructable_required": self.wallet_reconstructable_required,
            "pq_authority_required": self.pq_authority_required,
            "roots_only_public_export": self.roots_only_public_export,
            "user_escape_ready": self.user_escape_ready,
            "production_release_blocked": self.production_release_blocked,
            "replay_command_root": self.replay_command_root,
            "expected_receipt_root": self.expected_receipt_root,
            "replay_plan_step_root": self.replay_plan_step_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPlanBlocker {
    pub ordinal: u64,
    pub blocker_kind: ReplayPlanBlockerKind,
    pub owner: String,
    pub clearance: String,
    pub source_root: String,
    pub required_receipt_root: String,
    pub still_blocks_release: u64,
    pub blocker_root: String,
}

impl ReplayPlanBlocker {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        blocker_kind: ReplayPlanBlockerKind,
        ordinal: u64,
    ) -> Self {
        let source_root = blocker_source_root(source, blocker_kind);
        let required_receipt_root =
            blocker_required_receipt_root(config, source, blocker_kind, &source_root);
        let still_blocks_release = blocker_still_blocks_release(config, source, blocker_kind);
        let blocker_root = replay_plan_blocker_root(
            config,
            source,
            blocker_kind,
            ordinal,
            &source_root,
            &required_receipt_root,
            still_blocks_release,
        );
        Self {
            ordinal,
            blocker_kind,
            owner: blocker_kind.owner().to_string(),
            clearance: blocker_kind.clearance().to_string(),
            source_root,
            required_receipt_root,
            still_blocks_release,
            blocker_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "blocker_kind": self.blocker_kind.as_str(),
            "owner": self.owner,
            "clearance": self.clearance,
            "source_root": self.source_root,
            "required_receipt_root": self.required_receipt_root,
            "still_blocks_release": self.still_blocks_release,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPlanVerdict {
    pub replay_step_count: u64,
    pub blocker_case_count: u64,
    pub wallet_critical_step_count: u64,
    pub user_escape_ready_step_count: u64,
    pub production_blocked_step_count: u64,
    pub release_blocker_count: u64,
    pub answer_dossier_bound: u64,
    pub replay_bundle_bound: u64,
    pub replay_bundle_wallet_safe: u64,
    pub operator_independent_bound: u64,
    pub wallet_reconstructable_bound: u64,
    pub pq_authority_bound: u64,
    pub privacy_boundary_bound: u64,
    pub forced_exit_liveness_bound: u64,
    pub cargo_runtime_deferred_count: u64,
    pub heavy_gate_deferred_count: u64,
    pub audit_deferred_count: u64,
    pub live_feed_deferred_count: u64,
    pub production_blocked: u64,
    pub replay_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ReplayPlanVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        steps: &[ReplayPlanStep],
        blockers: &[ReplayPlanBlocker],
    ) -> Self {
        let replay_step_count = steps.len() as u64;
        let blocker_case_count = blockers.len() as u64;
        let wallet_critical_step_count = steps
            .iter()
            .filter(|step| step.wallet_critical == 1)
            .count() as u64;
        let user_escape_ready_step_count = steps
            .iter()
            .filter(|step| step.user_escape_ready == 1)
            .count() as u64;
        let production_blocked_step_count = steps
            .iter()
            .filter(|step| step.production_release_blocked == 1)
            .count() as u64;
        let release_blocker_count = blockers
            .iter()
            .filter(|blocker| blocker.still_blocks_release == 1)
            .count() as u64;
        let answer_dossier_bound = bool_to_u64(
            !source.dossier_root.is_empty() && source.dossier_user_escape_supported == 1,
        );
        let replay_bundle_bound =
            bool_to_u64(!source.replay_bundle_root.is_empty() && source.replay_required_count > 0);
        let replay_bundle_wallet_safe = bool_to_u64(
            source.replay_wallet_critical_count > 0
                && source.replay_wallet_safe_count >= source.replay_wallet_critical_count,
        );
        let operator_independent_bound = bool_to_u64(
            replay_bundle_wallet_safe == 1
                && source.dossier_user_escape_supported == config.require_answer_dossier,
        );
        let wallet_reconstructable_bound = bool_to_u64(
            source.replay_wallet_recovery_root.len() > 8 && replay_bundle_wallet_safe == 1,
        );
        let pq_authority_bound = source.dossier_pq_authority_bound;
        let privacy_boundary_bound = bool_to_u64(
            source.dossier_privacy_boundary_bound == 1
                && source.dossier_roots_only_public_export == 1
                && config.max_public_metadata_units == 0,
        );
        let forced_exit_liveness_bound = source.dossier_force_exit_liveness_bound;
        let cargo_runtime_deferred_count =
            source.dossier_live_execution_deferred_count + source.replay_deferred_count;
        let heavy_gate_deferred_count = source.replay_watch_count + source.replay_blocked_count;
        let audit_deferred_count = source.dossier_audit_deferred_count;
        let live_feed_deferred_count = bool_to_u64(source.dossier_release_blocker_count > 0);
        let production_blocked = bool_to_u64(
            config.production_release_allowed == 0
                || source.replay_production_blocked == 1
                || source.dossier_release_blocker_count > 0
                || cargo_runtime_deferred_count > 0
                || heavy_gate_deferred_count > 0
                || audit_deferred_count > 0
                || live_feed_deferred_count > 0,
        );
        let replay_status = if replay_step_count >= config.min_replay_steps
            && blocker_case_count >= config.min_blocker_cases
            && answer_dossier_bound == config.require_answer_dossier
            && replay_bundle_bound == config.require_replay_bundle
            && replay_bundle_wallet_safe == config.require_wallet_reconstructable
            && operator_independent_bound == config.require_operator_independence
            && wallet_reconstructable_bound == config.require_wallet_reconstructable
            && pq_authority_bound == config.require_pq_authority_replay
            && privacy_boundary_bound == config.require_privacy_boundary_replay
            && forced_exit_liveness_bound == config.require_force_exit_liveness_replay
            && production_blocked == config.require_release_hold
        {
            "wallet_replay_plan_bound_release_held"
        } else if production_blocked == 1 {
            "wallet_replay_plan_watch_release_held"
        } else {
            "wallet_replay_plan_gap"
        }
        .to_string();
        let user_escape_answer = if replay_status == "wallet_replay_plan_bound_release_held" {
            "the replay plan binds the safety dossier to deterministic wallet-replayable commands for deposit, private state, action receipt, settlement, withdrawal, and forced exit"
        } else {
            "the replay plan still lacks enough executable evidence to answer yes under full misbehavior"
        }
        .to_string();
        let production_answer = if production_blocked == 1 {
            "production release stays blocked until cargo/runtime, heavy-gate execution, audit, live-feed, reserve, PQ, and privacy receipts clear"
        } else {
            "this replay plan cannot grant production release"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.replay_policy),
                HashPart::Str(&source.dossier_root),
                HashPart::Str(&source.replay_bundle_root),
                HashPart::U64(replay_step_count),
                HashPart::U64(blocker_case_count),
                HashPart::U64(wallet_critical_step_count),
                HashPart::U64(user_escape_ready_step_count),
                HashPart::U64(production_blocked_step_count),
                HashPart::U64(release_blocker_count),
                HashPart::U64(answer_dossier_bound),
                HashPart::U64(replay_bundle_bound),
                HashPart::U64(replay_bundle_wallet_safe),
                HashPart::U64(operator_independent_bound),
                HashPart::U64(wallet_reconstructable_bound),
                HashPart::U64(pq_authority_bound),
                HashPart::U64(privacy_boundary_bound),
                HashPart::U64(forced_exit_liveness_bound),
                HashPart::U64(cargo_runtime_deferred_count),
                HashPart::U64(heavy_gate_deferred_count),
                HashPart::U64(audit_deferred_count),
                HashPart::U64(live_feed_deferred_count),
                HashPart::U64(production_blocked),
                HashPart::Str(&replay_status),
            ],
            32,
        );
        Self {
            replay_step_count,
            blocker_case_count,
            wallet_critical_step_count,
            user_escape_ready_step_count,
            production_blocked_step_count,
            release_blocker_count,
            answer_dossier_bound,
            replay_bundle_bound,
            replay_bundle_wallet_safe,
            operator_independent_bound,
            wallet_reconstructable_bound,
            pq_authority_bound,
            privacy_boundary_bound,
            forced_exit_liveness_bound,
            cargo_runtime_deferred_count,
            heavy_gate_deferred_count,
            audit_deferred_count,
            live_feed_deferred_count,
            production_blocked,
            replay_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_step_count": self.replay_step_count,
            "blocker_case_count": self.blocker_case_count,
            "wallet_critical_step_count": self.wallet_critical_step_count,
            "user_escape_ready_step_count": self.user_escape_ready_step_count,
            "production_blocked_step_count": self.production_blocked_step_count,
            "release_blocker_count": self.release_blocker_count,
            "answer_dossier_bound": self.answer_dossier_bound,
            "replay_bundle_bound": self.replay_bundle_bound,
            "replay_bundle_wallet_safe": self.replay_bundle_wallet_safe,
            "operator_independent_bound": self.operator_independent_bound,
            "wallet_reconstructable_bound": self.wallet_reconstructable_bound,
            "pq_authority_bound": self.pq_authority_bound,
            "privacy_boundary_bound": self.privacy_boundary_bound,
            "forced_exit_liveness_bound": self.forced_exit_liveness_bound,
            "cargo_runtime_deferred_count": self.cargo_runtime_deferred_count,
            "heavy_gate_deferred_count": self.heavy_gate_deferred_count,
            "audit_deferred_count": self.audit_deferred_count,
            "live_feed_deferred_count": self.live_feed_deferred_count,
            "production_blocked": self.production_blocked,
            "replay_status": self.replay_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub steps: Vec<ReplayPlanStep>,
    pub blockers: Vec<ReplayPlanBlocker>,
    pub verdict: ReplayPlanVerdict,
    pub step_root: String,
    pub command_root: String,
    pub expected_receipt_root: String,
    pub blocker_root: String,
    pub wallet_replay_root: String,
    pub production_hold_root: String,
    pub replay_plan_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let steps = ReplayPlanStepKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, step_kind)| {
                ReplayPlanStep::devnet(&config, &source, *step_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let blockers = ReplayPlanBlockerKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, blocker_kind)| {
                ReplayPlanBlocker::devnet(&config, &source, *blocker_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = ReplayPlanVerdict::new(&config, &source, &steps, &blockers);
        let step_root = step_vector_root(&steps);
        let command_root = command_vector_root(&steps);
        let expected_receipt_root = expected_receipt_vector_root(&steps);
        let blocker_root = blocker_vector_root(&blockers);
        let wallet_replay_root = wallet_replay_root(&config, &source, &command_root, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &blocker_root, &verdict);
        let replay_plan_root = replay_plan_root(
            &config,
            &source,
            &step_root,
            &command_root,
            &expected_receipt_root,
            &blocker_root,
            &wallet_replay_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            steps,
            blockers,
            verdict,
            step_root,
            command_root,
            expected_receipt_root,
            blocker_root,
            wallet_replay_root,
            production_hold_root,
            replay_plan_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), SourceBundle::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_replay_plan_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "step_root": self.step_root,
            "command_root": self.command_root,
            "expected_receipt_root": self.expected_receipt_root,
            "blocker_root": self.blocker_root,
            "wallet_replay_root": self.wallet_replay_root,
            "production_hold_root": self.production_hold_root,
            "replay_plan_root": self.replay_plan_root,
            "verdict": self.verdict.public_record(),
            "steps": self
                .steps
                .iter()
                .map(ReplayPlanStep::public_record)
                .collect::<Vec<_>>(),
            "blockers": self
                .blockers
                .iter()
                .map(ReplayPlanBlocker::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "state",
            &json!({
                "chain_id": CHAIN_ID,
                "protocol_version": PROTOCOL_VERSION,
                "config_root": self.config.state_root(),
                "source_root": self.source.state_root(),
                "step_root": self.step_root,
                "command_root": self.command_root,
                "expected_receipt_root": self.expected_receipt_root,
                "blocker_root": self.blocker_root,
                "wallet_replay_root": self.wallet_replay_root,
                "production_hold_root": self.production_hold_root,
                "replay_plan_root": self.replay_plan_root,
                "verdict_root": self.verdict.verdict_root,
            }),
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

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("user escape replay plan chain id mismatch".to_string());
    }
    if config.min_replay_steps < ReplayPlanStepKind::ordered().len() as u64 {
        return Err("user escape replay plan requires every replay step".to_string());
    }
    if config.min_blocker_cases < ReplayPlanBlockerKind::ordered().len() as u64 {
        return Err("user escape replay plan requires every blocker case".to_string());
    }
    if config.require_answer_dossier != 1
        || config.require_replay_bundle != 1
        || config.require_deposit_lock_replay != 1
        || config.require_private_note_replay != 1
        || config.require_private_action_replay != 1
        || config.require_settlement_receipt_replay != 1
        || config.require_withdrawal_claim_replay != 1
        || config.require_force_exit_liveness_replay != 1
        || config.require_pq_authority_replay != 1
        || config.require_privacy_boundary_replay != 1
        || config.require_operator_independence != 1
        || config.require_wallet_reconstructable != 1
        || config.require_release_hold != 1
    {
        return Err("user escape replay plan requires all safety controls".to_string());
    }
    if config.production_release_allowed != 0 || config.max_public_metadata_units != 0 {
        return Err("user escape replay plan cannot release production or metadata".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.dossier_root.is_empty() || source.replay_bundle_root.is_empty() {
        return Err("user escape replay plan missing source roots".to_string());
    }
    if source.replay_required_count == 0 || source.replay_wallet_critical_count == 0 {
        return Err("user escape replay plan missing replay bundle domains".to_string());
    }
    Ok(())
}

fn step_input_root(source: &SourceBundle, step_kind: ReplayPlanStepKind) -> String {
    match step_kind {
        ReplayPlanStepKind::AnswerDossierRoot => source.dossier_root.clone(),
        ReplayPlanStepKind::ExecutionReplayBundleRoot => source.replay_bundle_root.clone(),
        ReplayPlanStepKind::DepositLockReplay => source.dossier_vertical_evidence_root.clone(),
        ReplayPlanStepKind::PrivateNoteReplay => source.replay_committed_root.clone(),
        ReplayPlanStepKind::PrivateActionReplay => source.replay_root.clone(),
        ReplayPlanStepKind::SettlementReceiptReplay => source.replay_item_root.clone(),
        ReplayPlanStepKind::WithdrawalClaimReplay => source.replay_wallet_recovery_root.clone(),
        ReplayPlanStepKind::ForceExitLivenessReplay => source.dossier_answer_binding_root.clone(),
        ReplayPlanStepKind::PqAuthorityReplay => source.dossier_verdict_root.clone(),
        ReplayPlanStepKind::PrivacyBoundaryReplay => source.replay_encrypted_root.clone(),
        ReplayPlanStepKind::ReleaseBlockerReplay => source.replay_blocker_root.clone(),
        ReplayPlanStepKind::ProductionHoldReplay => source.dossier_production_hold_root.clone(),
    }
}

fn step_expected_output_root(source: &SourceBundle, step_kind: ReplayPlanStepKind) -> String {
    match step_kind {
        ReplayPlanStepKind::AnswerDossierRoot => source.dossier_verdict_root.clone(),
        ReplayPlanStepKind::ExecutionReplayBundleRoot => source.replay_root.clone(),
        ReplayPlanStepKind::DepositLockReplay => source.replay_public_root.clone(),
        ReplayPlanStepKind::PrivateNoteReplay => source.replay_committed_root.clone(),
        ReplayPlanStepKind::PrivateActionReplay => source.replay_root.clone(),
        ReplayPlanStepKind::SettlementReceiptReplay => {
            source.dossier_vertical_evidence_root.clone()
        }
        ReplayPlanStepKind::WithdrawalClaimReplay => source.replay_wallet_recovery_root.clone(),
        ReplayPlanStepKind::ForceExitLivenessReplay => source.dossier_answer_binding_root.clone(),
        ReplayPlanStepKind::PqAuthorityReplay => source.dossier_verdict_root.clone(),
        ReplayPlanStepKind::PrivacyBoundaryReplay => source.replay_encrypted_root.clone(),
        ReplayPlanStepKind::ReleaseBlockerReplay => source.dossier_production_hold_root.clone(),
        ReplayPlanStepKind::ProductionHoldReplay => source.dossier_production_hold_root.clone(),
    }
}

fn step_user_escape_ready(
    config: &Config,
    source: &SourceBundle,
    step_kind: ReplayPlanStepKind,
) -> u64 {
    match step_kind {
        ReplayPlanStepKind::AnswerDossierRoot => source.dossier_user_escape_supported,
        ReplayPlanStepKind::ExecutionReplayBundleRoot => source.replay_user_escape_replayable,
        ReplayPlanStepKind::DepositLockReplay => bool_to_u64(source.dossier_root.len() > 8),
        ReplayPlanStepKind::PrivateNoteReplay => bool_to_u64(
            source.replay_committed_root.len() > 8 && source.replay_wallet_safe_count > 0,
        ),
        ReplayPlanStepKind::PrivateActionReplay => bool_to_u64(source.replay_root.len() > 8),
        ReplayPlanStepKind::SettlementReceiptReplay => {
            bool_to_u64(source.dossier_vertical_evidence_root.len() > 8)
        }
        ReplayPlanStepKind::WithdrawalClaimReplay => {
            bool_to_u64(source.replay_wallet_recovery_root.len() > 8)
        }
        ReplayPlanStepKind::ForceExitLivenessReplay => source.dossier_force_exit_liveness_bound,
        ReplayPlanStepKind::PqAuthorityReplay => source.dossier_pq_authority_bound,
        ReplayPlanStepKind::PrivacyBoundaryReplay => bool_to_u64(
            source.dossier_privacy_boundary_bound == config.require_privacy_boundary_replay
                && source.dossier_roots_only_public_export == 1,
        ),
        ReplayPlanStepKind::ReleaseBlockerReplay => {
            bool_to_u64(source.replay_blocker_root.len() > 8)
        }
        ReplayPlanStepKind::ProductionHoldReplay => {
            bool_to_u64(source.dossier_production_hold_root.len() > 8)
        }
    }
}

fn step_production_blocked(
    config: &Config,
    source: &SourceBundle,
    step_kind: ReplayPlanStepKind,
) -> u64 {
    let blocked = config.production_release_allowed == 0
        || source.replay_production_blocked == 1
        || source.dossier_release_blocker_count > 0
        || source.dossier_live_execution_deferred_count > 0
        || source.dossier_audit_deferred_count > 0;
    match step_kind {
        ReplayPlanStepKind::ProductionHoldReplay => bool_to_u64(blocked),
        _ => bool_to_u64(blocked || source.replay_deferred_count > 0),
    }
}

fn step_pq_required(step_kind: ReplayPlanStepKind, config: &Config) -> u64 {
    match step_kind {
        ReplayPlanStepKind::PqAuthorityReplay
        | ReplayPlanStepKind::SettlementReceiptReplay
        | ReplayPlanStepKind::WithdrawalClaimReplay
        | ReplayPlanStepKind::ForceExitLivenessReplay => config.require_pq_authority_replay,
        _ => 0,
    }
}

fn blocker_source_root(source: &SourceBundle, blocker_kind: ReplayPlanBlockerKind) -> String {
    match blocker_kind {
        ReplayPlanBlockerKind::CargoRuntimeDeferred => source.replay_bundle_root.clone(),
        ReplayPlanBlockerKind::HeavyGateNotExecuted => source.replay_root.clone(),
        ReplayPlanBlockerKind::SecurityAuditDeferred => source.dossier_production_hold_root.clone(),
        ReplayPlanBlockerKind::LiveFeedSubstitutionMissing => {
            source.dossier_vertical_evidence_root.clone()
        }
        ReplayPlanBlockerKind::MoneroVerifierMissing => source.replay_public_root.clone(),
        ReplayPlanBlockerKind::ReserveLiquidityProofMissing => source.dossier_evidence_root.clone(),
        ReplayPlanBlockerKind::PqAuthorityReceiptMissing => source.dossier_verdict_root.clone(),
        ReplayPlanBlockerKind::PrivacyLeakReviewDeferred => source.replay_encrypted_root.clone(),
        ReplayPlanBlockerKind::ProductionReleaseHeld => source.dossier_production_hold_root.clone(),
    }
}

fn blocker_still_blocks_release(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ReplayPlanBlockerKind,
) -> u64 {
    match blocker_kind {
        ReplayPlanBlockerKind::CargoRuntimeDeferred => bool_to_u64(
            source.replay_deferred_count > 0 || source.dossier_live_execution_deferred_count > 0,
        ),
        ReplayPlanBlockerKind::HeavyGateNotExecuted => bool_to_u64(
            source.replay_watch_count > 0
                || source.replay_blocked_count > 0
                || source.replay_production_safe_count == 0,
        ),
        ReplayPlanBlockerKind::SecurityAuditDeferred => {
            bool_to_u64(source.dossier_audit_deferred_count > 0)
        }
        ReplayPlanBlockerKind::LiveFeedSubstitutionMissing => {
            bool_to_u64(source.dossier_release_blocker_count > 0)
        }
        ReplayPlanBlockerKind::MoneroVerifierMissing => 1,
        ReplayPlanBlockerKind::ReserveLiquidityProofMissing => {
            bool_to_u64(source.dossier_release_blocker_count > 0)
        }
        ReplayPlanBlockerKind::PqAuthorityReceiptMissing => {
            bool_to_u64(source.dossier_pq_authority_bound == 0)
        }
        ReplayPlanBlockerKind::PrivacyLeakReviewDeferred => {
            bool_to_u64(source.dossier_privacy_boundary_bound == 0)
        }
        ReplayPlanBlockerKind::ProductionReleaseHeld => bool_to_u64(
            config.production_release_allowed == 0 || source.replay_production_blocked == 1,
        ),
    }
}

fn replay_command_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: ReplayPlanStepKind,
    ordinal: u64,
    input_root: &str,
    expected_output_root: &str,
    user_escape_ready: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:replay-command"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_plan_suite),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(step_kind.command_name()),
            HashPart::Str(input_root),
            HashPart::Str(expected_output_root),
            HashPart::Str(&source.source_root),
            HashPart::U64(ordinal),
            HashPart::U64(user_escape_ready),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn expected_receipt_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: ReplayPlanStepKind,
    replay_command_root: &str,
    production_release_blocked: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:expected-receipt"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_policy),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(replay_command_root),
            HashPart::Str(&source.replay_root),
            HashPart::Str(&source.dossier_root),
            HashPart::U64(production_release_blocked),
        ],
        32,
    )
}

fn replay_plan_step_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: ReplayPlanStepKind,
    ordinal: u64,
    input_root: &str,
    expected_output_root: &str,
    replay_command_root: &str,
    expected_receipt_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:step"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(&source.source_root),
            HashPart::Str(input_root),
            HashPart::Str(expected_output_root),
            HashPart::Str(replay_command_root),
            HashPart::Str(expected_receipt_root),
            HashPart::U64(ordinal),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn blocker_required_receipt_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ReplayPlanBlockerKind,
    source_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker-required-receipt"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker_kind.as_str()),
            HashPart::Str(blocker_kind.clearance()),
            HashPart::Str(source_root),
            HashPart::Str(&source.dossier_production_hold_root),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn replay_plan_blocker_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ReplayPlanBlockerKind,
    ordinal: u64,
    source_root: &str,
    required_receipt_root: &str,
    still_blocks_release: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker_kind.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(required_receipt_root),
            HashPart::Str(&source.source_root),
            HashPart::U64(ordinal),
            HashPart::U64(still_blocks_release),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn step_vector_root(steps: &[ReplayPlanStep]) -> String {
    let roots = steps
        .iter()
        .map(|step| step.replay_plan_step_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:steps"), &roots)
}

fn command_vector_root(steps: &[ReplayPlanStep]) -> String {
    let roots = steps
        .iter()
        .map(|step| step.replay_command_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:commands"), &roots)
}

fn expected_receipt_vector_root(steps: &[ReplayPlanStep]) -> String {
    let roots = steps
        .iter()
        .map(|step| step.expected_receipt_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:expected-receipts"), &roots)
}

fn blocker_vector_root(blockers: &[ReplayPlanBlocker]) -> String {
    let roots = blockers
        .iter()
        .map(|blocker| blocker.blocker_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &roots)
}

fn wallet_replay_root(
    config: &Config,
    source: &SourceBundle,
    command_root: &str,
    verdict: &ReplayPlanVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:wallet-replay"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_policy),
            HashPart::Str(&source.dossier_root),
            HashPart::Str(&source.replay_wallet_recovery_root),
            HashPart::Str(command_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.user_escape_ready_step_count),
            HashPart::U64(verdict.wallet_reconstructable_bound),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    blocker_root: &str,
    verdict: &ReplayPlanVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:production-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_policy),
            HashPart::Str(&source.dossier_production_hold_root),
            HashPart::Str(blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.cargo_runtime_deferred_count),
            HashPart::U64(verdict.heavy_gate_deferred_count),
            HashPart::U64(verdict.audit_deferred_count),
            HashPart::U64(verdict.live_feed_deferred_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn replay_plan_root(
    config: &Config,
    source: &SourceBundle,
    step_root: &str,
    command_root: &str,
    expected_receipt_root: &str,
    blocker_root: &str,
    wallet_replay_root: &str,
    production_hold_root: &str,
    verdict: &ReplayPlanVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:replay-plan"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.replay_plan_suite),
            HashPart::Str(&source.source_root),
            HashPart::Str(step_root),
            HashPart::Str(command_root),
            HashPart::Str(expected_receipt_root),
            HashPart::Str(blocker_root),
            HashPart::Str(wallet_replay_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.production_blocked),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn source_bundle_root(dossier: &dossier::State, replay: &replay_bundle::State) -> String {
    let dossier_state_root = dossier.state_root();
    let replay_state_root = replay.state_root();
    domain_hash(
        &format!("{DOMAIN}:source-bundle"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&dossier_state_root),
            HashPart::Str(&dossier.dossier_root),
            HashPart::Str(&dossier.verdict.verdict_root),
            HashPart::Str(&replay_state_root),
            HashPart::Str(&replay.bundle.bundle_root),
            HashPart::Str(&replay.bundle.replay_root),
            HashPart::Str(replay.bundle.verdict.as_str()),
            HashPart::U64(dossier.verdict.user_escape_answer_supported),
            HashPart::U64(bool_to_u64(replay.user_escape_replayable())),
            HashPart::U64(bool_to_u64(replay.production_blocked())),
        ],
        32,
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source_root = record_root("fallback-source", &json!({"reason": &reason}));
    let source = SourceBundle {
        dossier_state_root: "fallback".to_string(),
        dossier_root: record_root("fallback-dossier", &json!({"reason": &reason})),
        dossier_evidence_root: record_root("fallback-evidence", &json!({"reason": &reason})),
        dossier_answer_binding_root: record_root("fallback-answer", &json!({"reason": &reason})),
        dossier_vertical_evidence_root: record_root(
            "fallback-vertical",
            &json!({"reason": &reason}),
        ),
        dossier_production_hold_root: record_root("fallback-hold", &json!({"reason": &reason})),
        dossier_verdict_root: record_root("fallback-verdict", &json!({"reason": &reason})),
        dossier_status: "fallback".to_string(),
        dossier_user_escape_supported: 0,
        dossier_live_execution_deferred_count: 1,
        dossier_audit_deferred_count: 1,
        dossier_release_blocker_count: 1,
        dossier_pq_authority_bound: 0,
        dossier_privacy_boundary_bound: 0,
        dossier_force_exit_liveness_bound: 0,
        dossier_roots_only_public_export: 1,
        replay_state_root: "fallback".to_string(),
        replay_bundle_root: record_root("fallback-bundle", &json!({"reason": &reason})),
        replay_item_root: record_root("fallback-items", &json!({"reason": &reason})),
        replay_root: record_root("fallback-replay", &json!({"reason": &reason})),
        replay_public_root: record_root("fallback-public", &json!({"reason": &reason})),
        replay_committed_root: record_root("fallback-committed", &json!({"reason": &reason})),
        replay_encrypted_root: record_root("fallback-encrypted", &json!({"reason": &reason})),
        replay_wallet_recovery_root: record_root("fallback-wallet", &json!({"reason": &reason})),
        replay_blocker_root: record_root("fallback-blocker", &json!({"reason": &reason})),
        replay_verdict: "fallback".to_string(),
        replay_user_escape_replayable: 0,
        replay_production_blocked: 1,
        replay_required_count: 0,
        replay_wallet_critical_count: 0,
        replay_wallet_safe_count: 0,
        replay_deferred_count: 1,
        replay_watch_count: 1,
        replay_blocked_count: 1,
        replay_rejected_count: 0,
        replay_production_safe_count: 0,
        source_root,
    };
    let steps = ReplayPlanStepKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, step_kind)| {
            ReplayPlanStep::devnet(&config, &source, *step_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let blockers = ReplayPlanBlockerKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, blocker_kind)| {
            ReplayPlanBlocker::devnet(&config, &source, *blocker_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = ReplayPlanVerdict::new(&config, &source, &steps, &blockers);
    let step_root = step_vector_root(&steps);
    let command_root = command_vector_root(&steps);
    let expected_receipt_root = expected_receipt_vector_root(&steps);
    let blocker_root = blocker_vector_root(&blockers);
    let wallet_replay_root = wallet_replay_root(&config, &source, &command_root, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &blocker_root, &verdict);
    let replay_plan_root = replay_plan_root(
        &config,
        &source,
        &step_root,
        &command_root,
        &expected_receipt_root,
        &blocker_root,
        &wallet_replay_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        steps,
        blockers,
        verdict,
        step_root,
        command_root,
        expected_receipt_root,
        blocker_root,
        wallet_replay_root,
        production_hold_root,
        replay_plan_root,
    }
}

fn bool_to_u64(value: bool) -> u64 {
    if value {
        1
    } else {
        0
    }
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:{kind}"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
