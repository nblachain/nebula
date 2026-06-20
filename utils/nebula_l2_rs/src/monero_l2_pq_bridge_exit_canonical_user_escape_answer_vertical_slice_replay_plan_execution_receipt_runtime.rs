use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_heavy_gate_execution_receipt_runtime as heavy_receipt,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_replay_plan_runtime as replay_plan,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceReplayPlanExecutionReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_REPLAY_PLAN_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-replay-plan-execution-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_REPLAY_PLAN_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-replay-plan-execution-receipt";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub receipt_binding_suite: String,
    pub receipt_policy: String,
    pub min_receipt_lanes: u64,
    pub min_blocker_cases: u64,
    pub require_replay_plan: u64,
    pub require_heavy_gate_receipt: u64,
    pub require_command_root: u64,
    pub require_expected_receipt_root: u64,
    pub require_wallet_replay_root: u64,
    pub require_heavy_wallet_receipt: u64,
    pub require_force_exit_critical_receipts: u64,
    pub require_operator_independent_receipts: u64,
    pub require_pq_authority_receipts: u64,
    pub require_privacy_receipts: u64,
    pub require_release_hold: u64,
    pub production_release_allowed: u64,
    pub max_public_metadata_units: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            receipt_binding_suite:
                "monero-l2-pq-bridge-exit-user-escape-replay-plan-execution-receipt-v1".to_string(),
            receipt_policy: "bind-replay-plan-to-heavy-gate-receipts-release-held-v1".to_string(),
            min_receipt_lanes: 12,
            min_blocker_cases: 9,
            require_replay_plan: 1,
            require_heavy_gate_receipt: 1,
            require_command_root: 1,
            require_expected_receipt_root: 1,
            require_wallet_replay_root: 1,
            require_heavy_wallet_receipt: 1,
            require_force_exit_critical_receipts: 1,
            require_operator_independent_receipts: 1,
            require_pq_authority_receipts: 1,
            require_privacy_receipts: 1,
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
            "receipt_binding_suite": self.receipt_binding_suite,
            "receipt_policy": self.receipt_policy,
            "min_receipt_lanes": self.min_receipt_lanes,
            "min_blocker_cases": self.min_blocker_cases,
            "require_replay_plan": self.require_replay_plan,
            "require_heavy_gate_receipt": self.require_heavy_gate_receipt,
            "require_command_root": self.require_command_root,
            "require_expected_receipt_root": self.require_expected_receipt_root,
            "require_wallet_replay_root": self.require_wallet_replay_root,
            "require_heavy_wallet_receipt": self.require_heavy_wallet_receipt,
            "require_force_exit_critical_receipts": self.require_force_exit_critical_receipts,
            "require_operator_independent_receipts": self.require_operator_independent_receipts,
            "require_pq_authority_receipts": self.require_pq_authority_receipts,
            "require_privacy_receipts": self.require_privacy_receipts,
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
pub enum ExecutionReceiptLane {
    ReplayPlanRoot,
    CommandRoot,
    ExpectedReceiptRoot,
    HeavyReceiptRoot,
    HeavyTranscriptRoot,
    DepositLockReceipt,
    PrivateNoteReceipt,
    PrivateActionReceipt,
    SettlementReceipt,
    WalletRecoveryReceipt,
    ForceExitLivenessReceipt,
    ProductionHoldReceipt,
}

impl ExecutionReceiptLane {
    pub fn ordered() -> [Self; 12] {
        [
            Self::ReplayPlanRoot,
            Self::CommandRoot,
            Self::ExpectedReceiptRoot,
            Self::HeavyReceiptRoot,
            Self::HeavyTranscriptRoot,
            Self::DepositLockReceipt,
            Self::PrivateNoteReceipt,
            Self::PrivateActionReceipt,
            Self::SettlementReceipt,
            Self::WalletRecoveryReceipt,
            Self::ForceExitLivenessReceipt,
            Self::ProductionHoldReceipt,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayPlanRoot => "replay_plan_root",
            Self::CommandRoot => "command_root",
            Self::ExpectedReceiptRoot => "expected_receipt_root",
            Self::HeavyReceiptRoot => "heavy_receipt_root",
            Self::HeavyTranscriptRoot => "heavy_transcript_root",
            Self::DepositLockReceipt => "deposit_lock_receipt",
            Self::PrivateNoteReceipt => "private_note_receipt",
            Self::PrivateActionReceipt => "private_action_receipt",
            Self::SettlementReceipt => "settlement_receipt",
            Self::WalletRecoveryReceipt => "wallet_recovery_receipt",
            Self::ForceExitLivenessReceipt => "force_exit_liveness_receipt",
            Self::ProductionHoldReceipt => "production_hold_receipt",
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::ReplayPlanRoot => "replay_plan_owner",
            Self::CommandRoot => "runtime_command_owner",
            Self::ExpectedReceiptRoot => "receipt_expectation_owner",
            Self::HeavyReceiptRoot => "heavy_gate_owner",
            Self::HeavyTranscriptRoot => "heavy_gate_owner",
            Self::DepositLockReceipt => "monero_watcher_owner",
            Self::PrivateNoteReceipt => "private_state_owner",
            Self::PrivateActionReceipt => "private_execution_owner",
            Self::SettlementReceipt => "settlement_owner",
            Self::WalletRecoveryReceipt => "wallet_recovery_owner",
            Self::ForceExitLivenessReceipt => "liveness_owner",
            Self::ProductionHoldReceipt => "release_owner",
        }
    }

    pub fn claim(self) -> &'static str {
        match self {
            Self::ReplayPlanRoot => {
                "Replay plan state root must be bound into the execution receipt."
            }
            Self::CommandRoot => {
                "Replay command vector must be bound before any receipt is accepted."
            }
            Self::ExpectedReceiptRoot => {
                "Expected receipt vector must be compared with observed heavy-gate receipt roots."
            }
            Self::HeavyReceiptRoot => "Observed heavy-gate receipt root must be attached.",
            Self::HeavyTranscriptRoot => "Observed heavy-gate transcript root must be attached.",
            Self::DepositLockReceipt => "Deposit-lock replay must have a receipt root.",
            Self::PrivateNoteReceipt => {
                "Private note/state replay must have committed and encrypted roots."
            }
            Self::PrivateActionReceipt => {
                "Private transfer or contract-action replay must have a replay receipt root."
            }
            Self::SettlementReceipt => {
                "Settlement replay must bind withdrawal and release state roots."
            }
            Self::WalletRecoveryReceipt => {
                "Wallet recovery replay must remain reconstructable without operator cooperation."
            }
            Self::ForceExitLivenessReceipt => {
                "Forced-exit liveness replay must have force-exit-critical wallet-safe receipts."
            }
            Self::ProductionHoldReceipt => {
                "Production release must remain held while heavy gates are deferred or unaudited."
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionReceiptBlockerKind {
    CargoRuntimeDeferred,
    SecurityAuditDeferred,
    HeavyGateDeferred,
    HeavyGateWatchItems,
    WalletCriticalGap,
    OperatorIndependenceGap,
    PqReceiptGap,
    PrivacyReceiptGap,
    ProductionReleaseHeld,
}

impl ExecutionReceiptBlockerKind {
    pub fn ordered() -> [Self; 9] {
        [
            Self::CargoRuntimeDeferred,
            Self::SecurityAuditDeferred,
            Self::HeavyGateDeferred,
            Self::HeavyGateWatchItems,
            Self::WalletCriticalGap,
            Self::OperatorIndependenceGap,
            Self::PqReceiptGap,
            Self::PrivacyReceiptGap,
            Self::ProductionReleaseHeld,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "cargo_runtime_deferred",
            Self::SecurityAuditDeferred => "security_audit_deferred",
            Self::HeavyGateDeferred => "heavy_gate_deferred",
            Self::HeavyGateWatchItems => "heavy_gate_watch_items",
            Self::WalletCriticalGap => "wallet_critical_gap",
            Self::OperatorIndependenceGap => "operator_independence_gap",
            Self::PqReceiptGap => "pq_receipt_gap",
            Self::PrivacyReceiptGap => "privacy_receipt_gap",
            Self::ProductionReleaseHeld => "production_release_held",
        }
    }

    pub fn clearance(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "run cargo check/test/clippy and runtime execution gates",
            Self::SecurityAuditDeferred => "attach signed security and privacy audit receipts",
            Self::HeavyGateDeferred => "execute heavy-gate replay commands against observed roots",
            Self::HeavyGateWatchItems => "resolve heavy-gate watch receipts and mismatch evidence",
            Self::WalletCriticalGap => "make every force-exit-critical receipt wallet safe",
            Self::OperatorIndependenceGap => {
                "prove every force-exit-critical receipt is operator independent"
            }
            Self::PqReceiptGap => {
                "attach PQ authority quorum and withdrawal authorization receipts"
            }
            Self::PrivacyReceiptGap => {
                "prove roots-only export and wallet metadata privacy receipts"
            }
            Self::ProductionReleaseHeld => {
                "keep production release denied until every blocker clears"
            }
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "runtime_owner",
            Self::SecurityAuditDeferred => "audit_owner",
            Self::HeavyGateDeferred => "heavy_gate_owner",
            Self::HeavyGateWatchItems => "heavy_gate_owner",
            Self::WalletCriticalGap => "wallet_recovery_owner",
            Self::OperatorIndependenceGap => "forced_exit_contract_owner",
            Self::PqReceiptGap => "pq_authority_owner",
            Self::PrivacyReceiptGap => "privacy_owner",
            Self::ProductionReleaseHeld => "release_owner",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBundle {
    pub replay_plan_state_root: String,
    pub replay_plan_root: String,
    pub replay_plan_step_root: String,
    pub replay_plan_command_root: String,
    pub replay_plan_expected_receipt_root: String,
    pub replay_plan_wallet_replay_root: String,
    pub replay_plan_production_hold_root: String,
    pub replay_plan_verdict_root: String,
    pub replay_plan_status: String,
    pub replay_plan_user_ready_steps: u64,
    pub replay_plan_production_blocked: u64,
    pub replay_plan_cargo_deferred_count: u64,
    pub replay_plan_heavy_deferred_count: u64,
    pub replay_plan_audit_deferred_count: u64,
    pub replay_plan_live_feed_deferred_count: u64,
    pub replay_plan_release_blocker_count: u64,
    pub replay_plan_pq_authority_bound: u64,
    pub replay_plan_privacy_boundary_bound: u64,
    pub replay_plan_force_exit_liveness_bound: u64,
    pub heavy_receipt_state_root: String,
    pub heavy_receipt_root: String,
    pub heavy_transcript_root: String,
    pub heavy_step_root: String,
    pub heavy_blocker_root: String,
    pub heavy_public_root: String,
    pub heavy_committed_root: String,
    pub heavy_encrypted_root: String,
    pub heavy_wallet_recovery_root: String,
    pub heavy_verdict: String,
    pub heavy_wallet_answer: String,
    pub heavy_production_answer: String,
    pub heavy_can_wallet_force_exit: u64,
    pub heavy_production_blocked: u64,
    pub heavy_total_steps: u64,
    pub heavy_accepted_count: u64,
    pub heavy_watch_count: u64,
    pub heavy_deferred_count: u64,
    pub heavy_blocked_count: u64,
    pub heavy_rejected_count: u64,
    pub heavy_wallet_safe_count: u64,
    pub heavy_operator_independent_count: u64,
    pub heavy_production_safe_count: u64,
    pub heavy_force_exit_critical_count: u64,
    pub source_root: String,
}

impl SourceBundle {
    pub fn devnet() -> Self {
        let replay_plan = replay_plan::devnet();
        let receipt = heavy_receipt::devnet();
        Self::from_states(&replay_plan, &receipt)
    }

    pub fn from_states(replay_plan: &replay_plan::State, receipt: &heavy_receipt::State) -> Self {
        let source_root = source_bundle_root(replay_plan, receipt);
        Self {
            replay_plan_state_root: replay_plan.state_root(),
            replay_plan_root: replay_plan.replay_plan_root.clone(),
            replay_plan_step_root: replay_plan.step_root.clone(),
            replay_plan_command_root: replay_plan.command_root.clone(),
            replay_plan_expected_receipt_root: replay_plan.expected_receipt_root.clone(),
            replay_plan_wallet_replay_root: replay_plan.wallet_replay_root.clone(),
            replay_plan_production_hold_root: replay_plan.production_hold_root.clone(),
            replay_plan_verdict_root: replay_plan.verdict.verdict_root.clone(),
            replay_plan_status: replay_plan.verdict.replay_status.clone(),
            replay_plan_user_ready_steps: replay_plan.verdict.user_escape_ready_step_count,
            replay_plan_production_blocked: replay_plan.verdict.production_blocked,
            replay_plan_cargo_deferred_count: replay_plan.verdict.cargo_runtime_deferred_count,
            replay_plan_heavy_deferred_count: replay_plan.verdict.heavy_gate_deferred_count,
            replay_plan_audit_deferred_count: replay_plan.verdict.audit_deferred_count,
            replay_plan_live_feed_deferred_count: replay_plan.verdict.live_feed_deferred_count,
            replay_plan_release_blocker_count: replay_plan.verdict.release_blocker_count,
            replay_plan_pq_authority_bound: replay_plan.verdict.pq_authority_bound,
            replay_plan_privacy_boundary_bound: replay_plan.verdict.privacy_boundary_bound,
            replay_plan_force_exit_liveness_bound: replay_plan.verdict.forced_exit_liveness_bound,
            heavy_receipt_state_root: receipt.state_root(),
            heavy_receipt_root: receipt.receipt.receipt_root.clone(),
            heavy_transcript_root: receipt.receipt.transcript_root.clone(),
            heavy_step_root: receipt.receipt.step_root.clone(),
            heavy_blocker_root: receipt.receipt.blocker_root.clone(),
            heavy_public_root: receipt.receipt.public_root.clone(),
            heavy_committed_root: receipt.receipt.committed_root.clone(),
            heavy_encrypted_root: receipt.receipt.encrypted_root.clone(),
            heavy_wallet_recovery_root: receipt.receipt.wallet_recovery_root.clone(),
            heavy_verdict: receipt.receipt.verdict.as_str().to_string(),
            heavy_wallet_answer: receipt.receipt.wallet_answer.clone(),
            heavy_production_answer: receipt.receipt.production_answer.clone(),
            heavy_can_wallet_force_exit: bool_to_u64(receipt.can_wallet_force_exit()),
            heavy_production_blocked: bool_to_u64(receipt.production_blocked()),
            heavy_total_steps: receipt.receipt.counters.total(),
            heavy_accepted_count: receipt.receipt.counters.accepted,
            heavy_watch_count: receipt.receipt.counters.watch,
            heavy_deferred_count: receipt.receipt.counters.deferred,
            heavy_blocked_count: receipt.receipt.counters.blocked,
            heavy_rejected_count: receipt.receipt.counters.rejected,
            heavy_wallet_safe_count: receipt.receipt.counters.wallet_safe,
            heavy_operator_independent_count: receipt.receipt.counters.operator_independent,
            heavy_production_safe_count: receipt.receipt.counters.production_safe,
            heavy_force_exit_critical_count: receipt.receipt.counters.force_exit_critical,
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_plan_state_root": self.replay_plan_state_root,
            "replay_plan_root": self.replay_plan_root,
            "replay_plan_step_root": self.replay_plan_step_root,
            "replay_plan_command_root": self.replay_plan_command_root,
            "replay_plan_expected_receipt_root": self.replay_plan_expected_receipt_root,
            "replay_plan_wallet_replay_root": self.replay_plan_wallet_replay_root,
            "replay_plan_production_hold_root": self.replay_plan_production_hold_root,
            "replay_plan_verdict_root": self.replay_plan_verdict_root,
            "replay_plan_status": self.replay_plan_status,
            "replay_plan_user_ready_steps": self.replay_plan_user_ready_steps,
            "replay_plan_production_blocked": self.replay_plan_production_blocked,
            "replay_plan_cargo_deferred_count": self.replay_plan_cargo_deferred_count,
            "replay_plan_heavy_deferred_count": self.replay_plan_heavy_deferred_count,
            "replay_plan_audit_deferred_count": self.replay_plan_audit_deferred_count,
            "replay_plan_live_feed_deferred_count": self.replay_plan_live_feed_deferred_count,
            "replay_plan_release_blocker_count": self.replay_plan_release_blocker_count,
            "replay_plan_pq_authority_bound": self.replay_plan_pq_authority_bound,
            "replay_plan_privacy_boundary_bound": self.replay_plan_privacy_boundary_bound,
            "replay_plan_force_exit_liveness_bound": self.replay_plan_force_exit_liveness_bound,
            "heavy_receipt_state_root": self.heavy_receipt_state_root,
            "heavy_receipt_root": self.heavy_receipt_root,
            "heavy_transcript_root": self.heavy_transcript_root,
            "heavy_step_root": self.heavy_step_root,
            "heavy_blocker_root": self.heavy_blocker_root,
            "heavy_public_root": self.heavy_public_root,
            "heavy_committed_root": self.heavy_committed_root,
            "heavy_encrypted_root": self.heavy_encrypted_root,
            "heavy_wallet_recovery_root": self.heavy_wallet_recovery_root,
            "heavy_verdict": self.heavy_verdict,
            "heavy_wallet_answer": self.heavy_wallet_answer,
            "heavy_production_answer": self.heavy_production_answer,
            "heavy_can_wallet_force_exit": self.heavy_can_wallet_force_exit,
            "heavy_production_blocked": self.heavy_production_blocked,
            "heavy_total_steps": self.heavy_total_steps,
            "heavy_accepted_count": self.heavy_accepted_count,
            "heavy_watch_count": self.heavy_watch_count,
            "heavy_deferred_count": self.heavy_deferred_count,
            "heavy_blocked_count": self.heavy_blocked_count,
            "heavy_rejected_count": self.heavy_rejected_count,
            "heavy_wallet_safe_count": self.heavy_wallet_safe_count,
            "heavy_operator_independent_count": self.heavy_operator_independent_count,
            "heavy_production_safe_count": self.heavy_production_safe_count,
            "heavy_force_exit_critical_count": self.heavy_force_exit_critical_count,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceiptBinding {
    pub ordinal: u64,
    pub lane: ExecutionReceiptLane,
    pub owner: String,
    pub claim: String,
    pub replay_plan_root: String,
    pub expected_root: String,
    pub observed_root: String,
    pub binding_evidence_root: String,
    pub expected_receipt_bound: u64,
    pub observed_receipt_bound: u64,
    pub wallet_safe_receipt: u64,
    pub release_held: u64,
    pub binding_root: String,
}

impl ExecutionReceiptBinding {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        lane: ExecutionReceiptLane,
        ordinal: u64,
    ) -> Self {
        let replay_plan_root = lane_replay_plan_root(source, lane);
        let expected_root = lane_expected_root(source, lane);
        let observed_root = lane_observed_root(source, lane);
        let expected_receipt_bound = bool_to_u64(!expected_root.is_empty());
        let observed_receipt_bound = bool_to_u64(!observed_root.is_empty());
        let wallet_safe_receipt = lane_wallet_safe_receipt(config, source, lane);
        let release_held = lane_release_held(config, source, lane);
        let binding_evidence_root = binding_evidence_root(
            config,
            source,
            lane,
            &replay_plan_root,
            &expected_root,
            &observed_root,
            expected_receipt_bound,
            observed_receipt_bound,
        );
        let binding_root = binding_root(
            config,
            source,
            lane,
            ordinal,
            &binding_evidence_root,
            wallet_safe_receipt,
            release_held,
        );
        Self {
            ordinal,
            lane,
            owner: lane.owner().to_string(),
            claim: lane.claim().to_string(),
            replay_plan_root,
            expected_root,
            observed_root,
            binding_evidence_root,
            expected_receipt_bound,
            observed_receipt_bound,
            wallet_safe_receipt,
            release_held,
            binding_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "claim": self.claim,
            "replay_plan_root": self.replay_plan_root,
            "expected_root": self.expected_root,
            "observed_root": self.observed_root,
            "binding_evidence_root": self.binding_evidence_root,
            "expected_receipt_bound": self.expected_receipt_bound,
            "observed_receipt_bound": self.observed_receipt_bound,
            "wallet_safe_receipt": self.wallet_safe_receipt,
            "release_held": self.release_held,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceiptBlocker {
    pub ordinal: u64,
    pub blocker_kind: ExecutionReceiptBlockerKind,
    pub owner: String,
    pub clearance: String,
    pub source_root: String,
    pub required_receipt_root: String,
    pub still_blocks_release: u64,
    pub blocker_root: String,
}

impl ExecutionReceiptBlocker {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        blocker_kind: ExecutionReceiptBlockerKind,
        ordinal: u64,
    ) -> Self {
        let source_root = blocker_source_root(source, blocker_kind);
        let required_receipt_root =
            blocker_required_receipt_root(config, source, blocker_kind, &source_root);
        let still_blocks_release = blocker_still_blocks_release(config, source, blocker_kind);
        let blocker_root = execution_blocker_root(
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
pub struct ExecutionReceiptVerdict {
    pub receipt_lane_count: u64,
    pub blocker_case_count: u64,
    pub expected_bound_count: u64,
    pub observed_bound_count: u64,
    pub wallet_safe_binding_count: u64,
    pub release_held_count: u64,
    pub release_blocker_count: u64,
    pub replay_plan_bound: u64,
    pub heavy_receipt_bound: u64,
    pub command_root_bound: u64,
    pub expected_receipt_root_bound: u64,
    pub wallet_replay_root_bound: u64,
    pub heavy_wallet_receipt_bound: u64,
    pub force_exit_critical_receipts_bound: u64,
    pub operator_independent_receipts_bound: u64,
    pub pq_authority_receipts_bound: u64,
    pub privacy_receipts_bound: u64,
    pub cargo_deferred_count: u64,
    pub audit_deferred_count: u64,
    pub heavy_gate_deferred_count: u64,
    pub heavy_gate_watch_count: u64,
    pub production_blocked: u64,
    pub receipt_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ExecutionReceiptVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        bindings: &[ExecutionReceiptBinding],
        blockers: &[ExecutionReceiptBlocker],
    ) -> Self {
        let receipt_lane_count = bindings.len() as u64;
        let blocker_case_count = blockers.len() as u64;
        let expected_bound_count = bindings
            .iter()
            .filter(|binding| binding.expected_receipt_bound == 1)
            .count() as u64;
        let observed_bound_count = bindings
            .iter()
            .filter(|binding| binding.observed_receipt_bound == 1)
            .count() as u64;
        let wallet_safe_binding_count = bindings
            .iter()
            .filter(|binding| binding.wallet_safe_receipt == 1)
            .count() as u64;
        let release_held_count = bindings
            .iter()
            .filter(|binding| binding.release_held == 1)
            .count() as u64;
        let release_blocker_count = blockers
            .iter()
            .filter(|blocker| blocker.still_blocks_release == 1)
            .count() as u64;
        let replay_plan_bound = bool_to_u64(
            !source.replay_plan_root.is_empty() && source.replay_plan_user_ready_steps > 0,
        );
        let heavy_receipt_bound =
            bool_to_u64(!source.heavy_receipt_root.is_empty() && source.heavy_total_steps > 0);
        let command_root_bound = bool_to_u64(!source.replay_plan_command_root.is_empty());
        let expected_receipt_root_bound =
            bool_to_u64(!source.replay_plan_expected_receipt_root.is_empty());
        let wallet_replay_root_bound =
            bool_to_u64(!source.replay_plan_wallet_replay_root.is_empty());
        let heavy_wallet_receipt_bound = bool_to_u64(
            source.heavy_can_wallet_force_exit == config.require_heavy_wallet_receipt
                && source.heavy_wallet_safe_count >= source.heavy_force_exit_critical_count
                && source.heavy_force_exit_critical_count > 0,
        );
        let force_exit_critical_receipts_bound = bool_to_u64(
            source.heavy_force_exit_critical_count > 0
                && source.heavy_wallet_safe_count >= source.heavy_force_exit_critical_count,
        );
        let operator_independent_receipts_bound = bool_to_u64(
            source.heavy_force_exit_critical_count > 0
                && source.heavy_operator_independent_count
                    >= source.heavy_force_exit_critical_count,
        );
        let pq_authority_receipts_bound = source.replay_plan_pq_authority_bound;
        let privacy_receipts_bound = bool_to_u64(
            source.replay_plan_privacy_boundary_bound == config.require_privacy_receipts
                && config.max_public_metadata_units == 0,
        );
        let cargo_deferred_count =
            source.replay_plan_cargo_deferred_count + source.heavy_deferred_count;
        let audit_deferred_count = source.replay_plan_audit_deferred_count;
        let heavy_gate_deferred_count =
            source.replay_plan_heavy_deferred_count + source.heavy_blocked_count;
        let heavy_gate_watch_count = source.heavy_watch_count + source.heavy_rejected_count;
        let production_blocked = bool_to_u64(
            config.production_release_allowed == 0
                || source.replay_plan_production_blocked == 1
                || source.heavy_production_blocked == 1
                || cargo_deferred_count > 0
                || audit_deferred_count > 0
                || heavy_gate_deferred_count > 0
                || heavy_gate_watch_count > 0
                || release_blocker_count > 0,
        );
        let receipt_status = if receipt_lane_count >= config.min_receipt_lanes
            && blocker_case_count >= config.min_blocker_cases
            && replay_plan_bound == config.require_replay_plan
            && heavy_receipt_bound == config.require_heavy_gate_receipt
            && command_root_bound == config.require_command_root
            && expected_receipt_root_bound == config.require_expected_receipt_root
            && wallet_replay_root_bound == config.require_wallet_replay_root
            && heavy_wallet_receipt_bound == config.require_heavy_wallet_receipt
            && force_exit_critical_receipts_bound == config.require_force_exit_critical_receipts
            && operator_independent_receipts_bound == config.require_operator_independent_receipts
            && pq_authority_receipts_bound == config.require_pq_authority_receipts
            && privacy_receipts_bound == config.require_privacy_receipts
            && production_blocked == config.require_release_hold
        {
            "replay_plan_execution_receipt_bound_release_held"
        } else if production_blocked == 1 {
            "replay_plan_execution_receipt_watch_release_held"
        } else {
            "replay_plan_execution_receipt_gap"
        }
        .to_string();
        let user_escape_answer = if receipt_status == "replay_plan_execution_receipt_bound_release_held" {
            "the replay commands are bound to heavy-gate receipt roots and remain wallet-force-exit replayable"
        } else {
            "the replay commands do not yet have enough bound heavy-gate receipts for a full yes answer"
        }
        .to_string();
        let production_answer = if production_blocked == 1 {
            "production release remains held until cargo/runtime, heavy-gate, audit, live-feed, PQ, privacy, and governance blockers clear"
        } else {
            "this receipt binding cannot grant production release"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.receipt_policy),
                HashPart::Str(&source.replay_plan_root),
                HashPart::Str(&source.heavy_receipt_root),
                HashPart::U64(receipt_lane_count),
                HashPart::U64(blocker_case_count),
                HashPart::U64(expected_bound_count),
                HashPart::U64(observed_bound_count),
                HashPart::U64(wallet_safe_binding_count),
                HashPart::U64(release_held_count),
                HashPart::U64(release_blocker_count),
                HashPart::U64(replay_plan_bound),
                HashPart::U64(heavy_receipt_bound),
                HashPart::U64(heavy_wallet_receipt_bound),
                HashPart::U64(force_exit_critical_receipts_bound),
                HashPart::U64(operator_independent_receipts_bound),
                HashPart::U64(pq_authority_receipts_bound),
                HashPart::U64(privacy_receipts_bound),
                HashPart::U64(cargo_deferred_count),
                HashPart::U64(audit_deferred_count),
                HashPart::U64(heavy_gate_deferred_count),
                HashPart::U64(heavy_gate_watch_count),
                HashPart::U64(production_blocked),
                HashPart::Str(&receipt_status),
            ],
            32,
        );
        Self {
            receipt_lane_count,
            blocker_case_count,
            expected_bound_count,
            observed_bound_count,
            wallet_safe_binding_count,
            release_held_count,
            release_blocker_count,
            replay_plan_bound,
            heavy_receipt_bound,
            command_root_bound,
            expected_receipt_root_bound,
            wallet_replay_root_bound,
            heavy_wallet_receipt_bound,
            force_exit_critical_receipts_bound,
            operator_independent_receipts_bound,
            pq_authority_receipts_bound,
            privacy_receipts_bound,
            cargo_deferred_count,
            audit_deferred_count,
            heavy_gate_deferred_count,
            heavy_gate_watch_count,
            production_blocked,
            receipt_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_lane_count": self.receipt_lane_count,
            "blocker_case_count": self.blocker_case_count,
            "expected_bound_count": self.expected_bound_count,
            "observed_bound_count": self.observed_bound_count,
            "wallet_safe_binding_count": self.wallet_safe_binding_count,
            "release_held_count": self.release_held_count,
            "release_blocker_count": self.release_blocker_count,
            "replay_plan_bound": self.replay_plan_bound,
            "heavy_receipt_bound": self.heavy_receipt_bound,
            "command_root_bound": self.command_root_bound,
            "expected_receipt_root_bound": self.expected_receipt_root_bound,
            "wallet_replay_root_bound": self.wallet_replay_root_bound,
            "heavy_wallet_receipt_bound": self.heavy_wallet_receipt_bound,
            "force_exit_critical_receipts_bound": self.force_exit_critical_receipts_bound,
            "operator_independent_receipts_bound": self.operator_independent_receipts_bound,
            "pq_authority_receipts_bound": self.pq_authority_receipts_bound,
            "privacy_receipts_bound": self.privacy_receipts_bound,
            "cargo_deferred_count": self.cargo_deferred_count,
            "audit_deferred_count": self.audit_deferred_count,
            "heavy_gate_deferred_count": self.heavy_gate_deferred_count,
            "heavy_gate_watch_count": self.heavy_gate_watch_count,
            "production_blocked": self.production_blocked,
            "receipt_status": self.receipt_status,
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
    pub bindings: Vec<ExecutionReceiptBinding>,
    pub blockers: Vec<ExecutionReceiptBlocker>,
    pub verdict: ExecutionReceiptVerdict,
    pub binding_vector_root: String,
    pub blocker_vector_root: String,
    pub observed_receipt_root: String,
    pub wallet_receipt_root: String,
    pub production_hold_root: String,
    pub execution_receipt_binding_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let bindings = ExecutionReceiptLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| {
                ExecutionReceiptBinding::devnet(&config, &source, *lane, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let blockers = ExecutionReceiptBlockerKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, blocker_kind)| {
                ExecutionReceiptBlocker::devnet(&config, &source, *blocker_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = ExecutionReceiptVerdict::new(&config, &source, &bindings, &blockers);
        let binding_vector_root = binding_vector_root(&bindings);
        let blocker_vector_root = blocker_vector_root(&blockers);
        let observed_receipt_root =
            observed_receipt_root(&config, &source, &binding_vector_root, &verdict);
        let wallet_receipt_root =
            wallet_receipt_root(&config, &source, &observed_receipt_root, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &blocker_vector_root, &verdict);
        let execution_receipt_binding_root = execution_receipt_binding_root(
            &config,
            &source,
            &binding_vector_root,
            &blocker_vector_root,
            &observed_receipt_root,
            &wallet_receipt_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            bindings,
            blockers,
            verdict,
            binding_vector_root,
            blocker_vector_root,
            observed_receipt_root,
            wallet_receipt_root,
            production_hold_root,
            execution_receipt_binding_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_replay_plan_execution_receipt_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "binding_vector_root": self.binding_vector_root,
            "blocker_vector_root": self.blocker_vector_root,
            "observed_receipt_root": self.observed_receipt_root,
            "wallet_receipt_root": self.wallet_receipt_root,
            "production_hold_root": self.production_hold_root,
            "execution_receipt_binding_root": self.execution_receipt_binding_root,
            "verdict": self.verdict.public_record(),
            "bindings": self
                .bindings
                .iter()
                .map(ExecutionReceiptBinding::public_record)
                .collect::<Vec<_>>(),
            "blockers": self
                .blockers
                .iter()
                .map(ExecutionReceiptBlocker::public_record)
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
                "binding_vector_root": self.binding_vector_root,
                "blocker_vector_root": self.blocker_vector_root,
                "observed_receipt_root": self.observed_receipt_root,
                "wallet_receipt_root": self.wallet_receipt_root,
                "production_hold_root": self.production_hold_root,
                "execution_receipt_binding_root": self.execution_receipt_binding_root,
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
        return Err("execution receipt binding chain id mismatch".to_string());
    }
    if config.min_receipt_lanes < ExecutionReceiptLane::ordered().len() as u64 {
        return Err("execution receipt binding requires every lane".to_string());
    }
    if config.min_blocker_cases < ExecutionReceiptBlockerKind::ordered().len() as u64 {
        return Err("execution receipt binding requires every blocker case".to_string());
    }
    if config.require_replay_plan != 1
        || config.require_heavy_gate_receipt != 1
        || config.require_command_root != 1
        || config.require_expected_receipt_root != 1
        || config.require_wallet_replay_root != 1
        || config.require_heavy_wallet_receipt != 1
        || config.require_force_exit_critical_receipts != 1
        || config.require_operator_independent_receipts != 1
        || config.require_pq_authority_receipts != 1
        || config.require_privacy_receipts != 1
        || config.require_release_hold != 1
    {
        return Err("execution receipt binding requires all replay safety controls".to_string());
    }
    if config.production_release_allowed != 0 || config.max_public_metadata_units != 0 {
        return Err("execution receipt binding cannot release production or metadata".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.replay_plan_root.is_empty()
        || source.heavy_receipt_root.is_empty()
        || source.heavy_transcript_root.is_empty()
    {
        return Err("execution receipt binding missing source roots".to_string());
    }
    if source.heavy_total_steps == 0 {
        return Err("execution receipt binding missing heavy-gate receipt steps".to_string());
    }
    Ok(())
}

fn lane_replay_plan_root(source: &SourceBundle, lane: ExecutionReceiptLane) -> String {
    match lane {
        ExecutionReceiptLane::ReplayPlanRoot => source.replay_plan_root.clone(),
        ExecutionReceiptLane::CommandRoot => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::ExpectedReceiptRoot => {
            source.replay_plan_expected_receipt_root.clone()
        }
        ExecutionReceiptLane::HeavyReceiptRoot => source.replay_plan_root.clone(),
        ExecutionReceiptLane::HeavyTranscriptRoot => source.replay_plan_root.clone(),
        ExecutionReceiptLane::DepositLockReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::PrivateNoteReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::PrivateActionReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::SettlementReceipt => source.replay_plan_expected_receipt_root.clone(),
        ExecutionReceiptLane::WalletRecoveryReceipt => {
            source.replay_plan_wallet_replay_root.clone()
        }
        ExecutionReceiptLane::ForceExitLivenessReceipt => source.replay_plan_verdict_root.clone(),
        ExecutionReceiptLane::ProductionHoldReceipt => {
            source.replay_plan_production_hold_root.clone()
        }
    }
}

fn lane_expected_root(source: &SourceBundle, lane: ExecutionReceiptLane) -> String {
    match lane {
        ExecutionReceiptLane::ReplayPlanRoot => source.replay_plan_root.clone(),
        ExecutionReceiptLane::CommandRoot => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::ExpectedReceiptRoot => {
            source.replay_plan_expected_receipt_root.clone()
        }
        ExecutionReceiptLane::HeavyReceiptRoot => source.replay_plan_expected_receipt_root.clone(),
        ExecutionReceiptLane::HeavyTranscriptRoot => {
            source.replay_plan_expected_receipt_root.clone()
        }
        ExecutionReceiptLane::DepositLockReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::PrivateNoteReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::PrivateActionReceipt => source.replay_plan_command_root.clone(),
        ExecutionReceiptLane::SettlementReceipt => source.replay_plan_expected_receipt_root.clone(),
        ExecutionReceiptLane::WalletRecoveryReceipt => {
            source.replay_plan_wallet_replay_root.clone()
        }
        ExecutionReceiptLane::ForceExitLivenessReceipt => source.replay_plan_verdict_root.clone(),
        ExecutionReceiptLane::ProductionHoldReceipt => {
            source.replay_plan_production_hold_root.clone()
        }
    }
}

fn lane_observed_root(source: &SourceBundle, lane: ExecutionReceiptLane) -> String {
    match lane {
        ExecutionReceiptLane::ReplayPlanRoot => source.replay_plan_state_root.clone(),
        ExecutionReceiptLane::CommandRoot => source.heavy_step_root.clone(),
        ExecutionReceiptLane::ExpectedReceiptRoot => source.heavy_transcript_root.clone(),
        ExecutionReceiptLane::HeavyReceiptRoot => source.heavy_receipt_root.clone(),
        ExecutionReceiptLane::HeavyTranscriptRoot => source.heavy_transcript_root.clone(),
        ExecutionReceiptLane::DepositLockReceipt => source.heavy_public_root.clone(),
        ExecutionReceiptLane::PrivateNoteReceipt => source.heavy_committed_root.clone(),
        ExecutionReceiptLane::PrivateActionReceipt => source.heavy_step_root.clone(),
        ExecutionReceiptLane::SettlementReceipt => source.heavy_receipt_root.clone(),
        ExecutionReceiptLane::WalletRecoveryReceipt => source.heavy_wallet_recovery_root.clone(),
        ExecutionReceiptLane::ForceExitLivenessReceipt => source.heavy_receipt_root.clone(),
        ExecutionReceiptLane::ProductionHoldReceipt => source.heavy_blocker_root.clone(),
    }
}

fn lane_wallet_safe_receipt(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
) -> u64 {
    match lane {
        ExecutionReceiptLane::ReplayPlanRoot
        | ExecutionReceiptLane::CommandRoot
        | ExecutionReceiptLane::ExpectedReceiptRoot => {
            bool_to_u64(source.replay_plan_user_ready_steps >= config.min_receipt_lanes)
        }
        ExecutionReceiptLane::HeavyReceiptRoot | ExecutionReceiptLane::HeavyTranscriptRoot => {
            source.heavy_can_wallet_force_exit
        }
        ExecutionReceiptLane::DepositLockReceipt
        | ExecutionReceiptLane::PrivateNoteReceipt
        | ExecutionReceiptLane::SettlementReceipt
        | ExecutionReceiptLane::WalletRecoveryReceipt
        | ExecutionReceiptLane::ForceExitLivenessReceipt => bool_to_u64(
            source.heavy_force_exit_critical_count > 0
                && source.heavy_wallet_safe_count >= source.heavy_force_exit_critical_count,
        ),
        ExecutionReceiptLane::PrivateActionReceipt => {
            bool_to_u64(source.heavy_wallet_safe_count > 0 && source.heavy_blocked_count == 0)
        }
        ExecutionReceiptLane::ProductionHoldReceipt => source.heavy_production_blocked,
    }
}

fn lane_release_held(config: &Config, source: &SourceBundle, lane: ExecutionReceiptLane) -> u64 {
    let held = config.production_release_allowed == 0
        || source.replay_plan_production_blocked == 1
        || source.heavy_production_blocked == 1
        || source.heavy_deferred_count > 0
        || source.heavy_watch_count > 0
        || source.replay_plan_release_blocker_count > 0;
    match lane {
        ExecutionReceiptLane::ProductionHoldReceipt => bool_to_u64(held),
        _ => bool_to_u64(held || source.replay_plan_cargo_deferred_count > 0),
    }
}

fn blocker_source_root(source: &SourceBundle, blocker_kind: ExecutionReceiptBlockerKind) -> String {
    match blocker_kind {
        ExecutionReceiptBlockerKind::CargoRuntimeDeferred => {
            source.replay_plan_command_root.clone()
        }
        ExecutionReceiptBlockerKind::SecurityAuditDeferred => {
            source.replay_plan_production_hold_root.clone()
        }
        ExecutionReceiptBlockerKind::HeavyGateDeferred => source.heavy_receipt_root.clone(),
        ExecutionReceiptBlockerKind::HeavyGateWatchItems => source.heavy_blocker_root.clone(),
        ExecutionReceiptBlockerKind::WalletCriticalGap => source.heavy_wallet_recovery_root.clone(),
        ExecutionReceiptBlockerKind::OperatorIndependenceGap => source.heavy_step_root.clone(),
        ExecutionReceiptBlockerKind::PqReceiptGap => source.replay_plan_verdict_root.clone(),
        ExecutionReceiptBlockerKind::PrivacyReceiptGap => source.heavy_encrypted_root.clone(),
        ExecutionReceiptBlockerKind::ProductionReleaseHeld => {
            source.replay_plan_production_hold_root.clone()
        }
    }
}

fn blocker_still_blocks_release(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ExecutionReceiptBlockerKind,
) -> u64 {
    match blocker_kind {
        ExecutionReceiptBlockerKind::CargoRuntimeDeferred => bool_to_u64(
            source.replay_plan_cargo_deferred_count > 0 || source.heavy_deferred_count > 0,
        ),
        ExecutionReceiptBlockerKind::SecurityAuditDeferred => {
            bool_to_u64(source.replay_plan_audit_deferred_count > 0)
        }
        ExecutionReceiptBlockerKind::HeavyGateDeferred => bool_to_u64(
            source.replay_plan_heavy_deferred_count > 0 || source.heavy_deferred_count > 0,
        ),
        ExecutionReceiptBlockerKind::HeavyGateWatchItems => {
            bool_to_u64(source.heavy_watch_count > 0 || source.heavy_blocked_count > 0)
        }
        ExecutionReceiptBlockerKind::WalletCriticalGap => bool_to_u64(
            source.heavy_force_exit_critical_count > 0
                && source.heavy_wallet_safe_count < source.heavy_force_exit_critical_count,
        ),
        ExecutionReceiptBlockerKind::OperatorIndependenceGap => bool_to_u64(
            source.heavy_force_exit_critical_count > 0
                && source.heavy_operator_independent_count < source.heavy_force_exit_critical_count,
        ),
        ExecutionReceiptBlockerKind::PqReceiptGap => {
            bool_to_u64(source.replay_plan_pq_authority_bound == 0)
        }
        ExecutionReceiptBlockerKind::PrivacyReceiptGap => {
            bool_to_u64(source.replay_plan_privacy_boundary_bound == 0)
        }
        ExecutionReceiptBlockerKind::ProductionReleaseHeld => bool_to_u64(
            config.production_release_allowed == 0 || source.heavy_production_blocked == 1,
        ),
    }
}

fn binding_evidence_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    replay_plan_root: &str,
    expected_root: &str,
    observed_root: &str,
    expected_receipt_bound: u64,
    observed_receipt_bound: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:binding-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.receipt_binding_suite),
            HashPart::Str(lane.as_str()),
            HashPart::Str(replay_plan_root),
            HashPart::Str(expected_root),
            HashPart::Str(observed_root),
            HashPart::Str(&source.source_root),
            HashPart::U64(expected_receipt_bound),
            HashPart::U64(observed_receipt_bound),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn binding_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    ordinal: u64,
    binding_evidence_root: &str,
    wallet_safe_receipt: u64,
    release_held: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:binding"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(binding_evidence_root),
            HashPart::Str(&source.replay_plan_root),
            HashPart::Str(&source.heavy_receipt_root),
            HashPart::U64(ordinal),
            HashPart::U64(wallet_safe_receipt),
            HashPart::U64(release_held),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn blocker_required_receipt_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ExecutionReceiptBlockerKind,
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
            HashPart::Str(&source.replay_plan_production_hold_root),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn execution_blocker_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: ExecutionReceiptBlockerKind,
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

fn binding_vector_root(bindings: &[ExecutionReceiptBinding]) -> String {
    let roots = bindings
        .iter()
        .map(|binding| binding.binding_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:binding-vector"), &roots)
}

fn blocker_vector_root(blockers: &[ExecutionReceiptBlocker]) -> String {
    let roots = blockers
        .iter()
        .map(|blocker| blocker.blocker_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blocker-vector"), &roots)
}

fn observed_receipt_root(
    config: &Config,
    source: &SourceBundle,
    binding_vector_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:observed-receipt"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.receipt_policy),
            HashPart::Str(&source.heavy_receipt_root),
            HashPart::Str(&source.heavy_transcript_root),
            HashPart::Str(binding_vector_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.observed_bound_count),
            HashPart::U64(verdict.heavy_wallet_receipt_bound),
        ],
        32,
    )
}

fn wallet_receipt_root(
    config: &Config,
    source: &SourceBundle,
    observed_receipt_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:wallet-receipt"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.receipt_policy),
            HashPart::Str(&source.replay_plan_wallet_replay_root),
            HashPart::Str(&source.heavy_wallet_recovery_root),
            HashPart::Str(observed_receipt_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.wallet_safe_binding_count),
            HashPart::U64(verdict.force_exit_critical_receipts_bound),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    blocker_vector_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:production-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.receipt_policy),
            HashPart::Str(&source.replay_plan_production_hold_root),
            HashPart::Str(&source.heavy_blocker_root),
            HashPart::Str(blocker_vector_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.cargo_deferred_count),
            HashPart::U64(verdict.audit_deferred_count),
            HashPart::U64(verdict.heavy_gate_deferred_count),
            HashPart::U64(verdict.heavy_gate_watch_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn execution_receipt_binding_root(
    config: &Config,
    source: &SourceBundle,
    binding_vector_root: &str,
    blocker_vector_root: &str,
    observed_receipt_root: &str,
    wallet_receipt_root: &str,
    production_hold_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:execution-receipt-binding"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.receipt_binding_suite),
            HashPart::Str(&source.source_root),
            HashPart::Str(binding_vector_root),
            HashPart::Str(blocker_vector_root),
            HashPart::Str(observed_receipt_root),
            HashPart::Str(wallet_receipt_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.production_blocked),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn source_bundle_root(replay_plan: &replay_plan::State, receipt: &heavy_receipt::State) -> String {
    let replay_plan_state_root = replay_plan.state_root();
    let heavy_receipt_state_root = receipt.state_root();
    domain_hash(
        &format!("{DOMAIN}:source-bundle"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&replay_plan_state_root),
            HashPart::Str(&replay_plan.replay_plan_root),
            HashPart::Str(&replay_plan.expected_receipt_root),
            HashPart::Str(&replay_plan.verdict.verdict_root),
            HashPart::Str(&heavy_receipt_state_root),
            HashPart::Str(&receipt.receipt.receipt_root),
            HashPart::Str(&receipt.receipt.transcript_root),
            HashPart::Str(receipt.receipt.verdict.as_str()),
            HashPart::U64(replay_plan.verdict.production_blocked),
            HashPart::U64(bool_to_u64(receipt.can_wallet_force_exit())),
            HashPart::U64(bool_to_u64(receipt.production_blocked())),
        ],
        32,
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source_root = record_root("fallback-source", &json!({"reason": &reason}));
    let source = SourceBundle {
        replay_plan_state_root: "fallback".to_string(),
        replay_plan_root: record_root("fallback-replay-plan", &json!({"reason": &reason})),
        replay_plan_step_root: record_root("fallback-steps", &json!({"reason": &reason})),
        replay_plan_command_root: record_root("fallback-commands", &json!({"reason": &reason})),
        replay_plan_expected_receipt_root: record_root(
            "fallback-expected-receipts",
            &json!({"reason": &reason}),
        ),
        replay_plan_wallet_replay_root: record_root("fallback-wallet", &json!({"reason": &reason})),
        replay_plan_production_hold_root: record_root("fallback-hold", &json!({"reason": &reason})),
        replay_plan_verdict_root: record_root("fallback-verdict", &json!({"reason": &reason})),
        replay_plan_status: "fallback".to_string(),
        replay_plan_user_ready_steps: 0,
        replay_plan_production_blocked: 1,
        replay_plan_cargo_deferred_count: 1,
        replay_plan_heavy_deferred_count: 1,
        replay_plan_audit_deferred_count: 1,
        replay_plan_live_feed_deferred_count: 1,
        replay_plan_release_blocker_count: 1,
        replay_plan_pq_authority_bound: 0,
        replay_plan_privacy_boundary_bound: 0,
        replay_plan_force_exit_liveness_bound: 0,
        heavy_receipt_state_root: "fallback".to_string(),
        heavy_receipt_root: record_root("fallback-heavy-receipt", &json!({"reason": &reason})),
        heavy_transcript_root: record_root(
            "fallback-heavy-transcript",
            &json!({"reason": &reason}),
        ),
        heavy_step_root: record_root("fallback-heavy-steps", &json!({"reason": &reason})),
        heavy_blocker_root: record_root("fallback-heavy-blockers", &json!({"reason": &reason})),
        heavy_public_root: record_root("fallback-heavy-public", &json!({"reason": &reason})),
        heavy_committed_root: record_root("fallback-heavy-committed", &json!({"reason": &reason})),
        heavy_encrypted_root: record_root("fallback-heavy-encrypted", &json!({"reason": &reason})),
        heavy_wallet_recovery_root: record_root(
            "fallback-heavy-wallet",
            &json!({"reason": &reason}),
        ),
        heavy_verdict: "fallback".to_string(),
        heavy_wallet_answer: "fallback".to_string(),
        heavy_production_answer: "fallback".to_string(),
        heavy_can_wallet_force_exit: 0,
        heavy_production_blocked: 1,
        heavy_total_steps: 0,
        heavy_accepted_count: 0,
        heavy_watch_count: 1,
        heavy_deferred_count: 1,
        heavy_blocked_count: 1,
        heavy_rejected_count: 0,
        heavy_wallet_safe_count: 0,
        heavy_operator_independent_count: 0,
        heavy_production_safe_count: 0,
        heavy_force_exit_critical_count: 0,
        source_root,
    };
    let bindings = ExecutionReceiptLane::ordered()
        .iter()
        .enumerate()
        .map(|(index, lane)| {
            ExecutionReceiptBinding::devnet(&config, &source, *lane, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let blockers = ExecutionReceiptBlockerKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, blocker_kind)| {
            ExecutionReceiptBlocker::devnet(&config, &source, *blocker_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = ExecutionReceiptVerdict::new(&config, &source, &bindings, &blockers);
    let binding_vector_root = binding_vector_root(&bindings);
    let blocker_vector_root = blocker_vector_root(&blockers);
    let observed_receipt_root =
        observed_receipt_root(&config, &source, &binding_vector_root, &verdict);
    let wallet_receipt_root =
        wallet_receipt_root(&config, &source, &observed_receipt_root, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &blocker_vector_root, &verdict);
    let execution_receipt_binding_root = execution_receipt_binding_root(
        &config,
        &source,
        &binding_vector_root,
        &blocker_vector_root,
        &observed_receipt_root,
        &wallet_receipt_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        bindings,
        blockers,
        verdict,
        binding_vector_root,
        blocker_vector_root,
        observed_receipt_root,
        wallet_receipt_root,
        production_hold_root,
        execution_receipt_binding_root,
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
