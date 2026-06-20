use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_safety_answer_runtime as safety_answer,
    monero_l2_pq_bridge_exit_canonical_vertical_slice_runtime_exit_acceptance_evidence_manifest_runtime as exit_acceptance,
    monero_l2_pq_bridge_exit_vertical_slice_scenario_runtime as vertical_slice, CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceEvidenceDossierRuntimeResult<T> =
    Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_EVIDENCE_DOSSIER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-evidence-dossier-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_EVIDENCE_DOSSIER_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-evidence-dossier";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub dossier_suite: String,
    pub evidence_policy: String,
    pub min_dossier_lanes: u64,
    pub min_blocker_cases: u64,
    pub require_safety_answer: u64,
    pub require_vertical_slice_scenario: u64,
    pub require_exit_acceptance_manifest: u64,
    pub require_deposit_lock: u64,
    pub require_private_state_transition: u64,
    pub require_private_action_receipt: u64,
    pub require_settlement_receipt: u64,
    pub require_withdrawal_claim: u64,
    pub require_force_exit_liveness: u64,
    pub require_operator_independent_escape: u64,
    pub require_pq_authority: u64,
    pub require_roots_only_public_export: u64,
    pub production_release_allowed: u64,
    pub max_public_metadata_units: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            dossier_suite: "monero-l2-pq-bridge-exit-user-escape-vertical-slice-dossier-v1"
                .to_string(),
            evidence_policy: "answer-bound-vertical-evidence-release-held-v1".to_string(),
            min_dossier_lanes: 10,
            min_blocker_cases: 9,
            require_safety_answer: 1,
            require_vertical_slice_scenario: 1,
            require_exit_acceptance_manifest: 1,
            require_deposit_lock: 1,
            require_private_state_transition: 1,
            require_private_action_receipt: 1,
            require_settlement_receipt: 1,
            require_withdrawal_claim: 1,
            require_force_exit_liveness: 1,
            require_operator_independent_escape: 1,
            require_pq_authority: 1,
            require_roots_only_public_export: 1,
            production_release_allowed: 0,
            max_public_metadata_units: 0,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "dossier_suite": self.dossier_suite,
            "evidence_policy": self.evidence_policy,
            "min_dossier_lanes": self.min_dossier_lanes,
            "min_blocker_cases": self.min_blocker_cases,
            "require_safety_answer": self.require_safety_answer,
            "require_vertical_slice_scenario": self.require_vertical_slice_scenario,
            "require_exit_acceptance_manifest": self.require_exit_acceptance_manifest,
            "require_deposit_lock": self.require_deposit_lock,
            "require_private_state_transition": self.require_private_state_transition,
            "require_private_action_receipt": self.require_private_action_receipt,
            "require_settlement_receipt": self.require_settlement_receipt,
            "require_withdrawal_claim": self.require_withdrawal_claim,
            "require_force_exit_liveness": self.require_force_exit_liveness,
            "require_operator_independent_escape": self.require_operator_independent_escape,
            "require_pq_authority": self.require_pq_authority,
            "require_roots_only_public_export": self.require_roots_only_public_export,
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
pub enum DossierLane {
    SafetyAnswer,
    VerticalSliceScenario,
    ExitAcceptanceManifest,
    DepositLock,
    PrivateStateTransition,
    PrivateActionReceipt,
    SettlementReceipt,
    WithdrawalClaim,
    ForceExitLiveness,
    ProductionHold,
}

impl DossierLane {
    pub fn ordered() -> [Self; 10] {
        [
            Self::SafetyAnswer,
            Self::VerticalSliceScenario,
            Self::ExitAcceptanceManifest,
            Self::DepositLock,
            Self::PrivateStateTransition,
            Self::PrivateActionReceipt,
            Self::SettlementReceipt,
            Self::WithdrawalClaim,
            Self::ForceExitLiveness,
            Self::ProductionHold,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SafetyAnswer => "safety_answer",
            Self::VerticalSliceScenario => "vertical_slice_scenario",
            Self::ExitAcceptanceManifest => "exit_acceptance_manifest",
            Self::DepositLock => "deposit_lock",
            Self::PrivateStateTransition => "private_state_transition",
            Self::PrivateActionReceipt => "private_action_receipt",
            Self::SettlementReceipt => "settlement_receipt",
            Self::WithdrawalClaim => "withdrawal_claim",
            Self::ForceExitLiveness => "force_exit_liveness",
            Self::ProductionHold => "production_hold",
        }
    }

    pub fn statement(self) -> &'static str {
        match self {
            Self::SafetyAnswer => {
                "The user-escape answer must be bound to source roots and keep production blocked."
            }
            Self::VerticalSliceScenario => {
                "The dossier must point at a concrete deposit-private-action-forced-exit scenario."
            }
            Self::ExitAcceptanceManifest => {
                "Exit acceptance evidence must explain which release blockers still hold."
            }
            Self::DepositLock => {
                "The bridge path must include a Monero deposit-lock commitment and watcher evidence."
            }
            Self::PrivateStateTransition => {
                "The L2 path must include private note/state transition roots without wallet plaintext."
            }
            Self::PrivateActionReceipt => {
                "The L2 path must include transfer or contract-action receipt roots."
            }
            Self::SettlementReceipt => {
                "The path must include a settlement receipt binding withdrawal and release state."
            }
            Self::WithdrawalClaim => {
                "The wallet must retain a withdrawal/forced-exit claim root it can replay."
            }
            Self::ForceExitLiveness => {
                "The forced-exit route must become available after the liveness timeout without operator help."
            }
            Self::ProductionHold => {
                "Production release must remain held while heavy execution, cargo, audit, and live feeds are deferred."
            }
        }
    }

    pub fn owner(self) -> &'static str {
        match self {
            Self::SafetyAnswer => "answer_owner",
            Self::VerticalSliceScenario => "vertical_slice_owner",
            Self::ExitAcceptanceManifest => "acceptance_owner",
            Self::DepositLock => "monero_watcher_owner",
            Self::PrivateStateTransition => "private_state_owner",
            Self::PrivateActionReceipt => "receipt_owner",
            Self::SettlementReceipt => "settlement_owner",
            Self::WithdrawalClaim => "wallet_recovery_owner",
            Self::ForceExitLiveness => "liveness_owner",
            Self::ProductionHold => "release_governance_owner",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DossierBlockerKind {
    CargoRuntimeDeferred,
    SecurityAuditDeferred,
    LiveFeedDeferred,
    MoneroVerifierAbsent,
    LiquidityReserveNeedsProof,
    PrivacyLeakAuditDeferred,
    WatcherCollusionNeedsReplay,
    ReleaseGovernanceExecutionDeferred,
    ProductionReleaseHeld,
}

impl DossierBlockerKind {
    pub fn ordered() -> [Self; 9] {
        [
            Self::CargoRuntimeDeferred,
            Self::SecurityAuditDeferred,
            Self::LiveFeedDeferred,
            Self::MoneroVerifierAbsent,
            Self::LiquidityReserveNeedsProof,
            Self::PrivacyLeakAuditDeferred,
            Self::WatcherCollusionNeedsReplay,
            Self::ReleaseGovernanceExecutionDeferred,
            Self::ProductionReleaseHeld,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => "cargo_runtime_deferred",
            Self::SecurityAuditDeferred => "security_audit_deferred",
            Self::LiveFeedDeferred => "live_feed_deferred",
            Self::MoneroVerifierAbsent => "monero_verifier_absent",
            Self::LiquidityReserveNeedsProof => "liquidity_reserve_needs_proof",
            Self::PrivacyLeakAuditDeferred => "privacy_leak_audit_deferred",
            Self::WatcherCollusionNeedsReplay => "watcher_collusion_needs_replay",
            Self::ReleaseGovernanceExecutionDeferred => "release_governance_execution_deferred",
            Self::ProductionReleaseHeld => "production_release_held",
        }
    }

    pub fn clearance(self) -> &'static str {
        match self {
            Self::CargoRuntimeDeferred => {
                "run cargo check/test/clippy and runtime replay gates against this exact dossier"
            }
            Self::SecurityAuditDeferred => {
                "attach signed bridge, privacy, PQ authority, and forced-exit security audit receipts"
            }
            Self::LiveFeedDeferred => {
                "replace fixture roots with live Monero watcher, reserve, and settlement feed receipts"
            }
            Self::MoneroVerifierAbsent => {
                "bind a production Monero evidence verifier policy with reorg and finality handling"
            }
            Self::LiquidityReserveNeedsProof => {
                "prove reserve coverage, queue ordering, and forced-exit liquidity under stress"
            }
            Self::PrivacyLeakAuditDeferred => {
                "close wallet-linkage, timing, amount, and metadata leakage review"
            }
            Self::WatcherCollusionNeedsReplay => {
                "execute reorg and watcher-collusion replay fixtures against live adapters"
            }
            Self::ReleaseGovernanceExecutionDeferred => {
                "execute release governance, unhold drills, reviewer receipts, and emergency pause controls"
            }
            Self::ProductionReleaseHeld => {
                "keep production release denied until every blocker in the dossier is cleared"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBundle {
    pub safety_answer_state_root: String,
    pub safety_answer_root: String,
    pub user_answer_root: String,
    pub production_hold_root: String,
    pub safety_answer_status: String,
    pub safety_user_escape_supported: u64,
    pub safety_production_blocked: u64,
    pub safety_deferred_gate_count: u64,
    pub safety_production_blocker_count: u64,
    pub safety_pq_authority_bound: u64,
    pub safety_privacy_boundary_bound: u64,
    pub vertical_state_root: String,
    pub vertical_transcript_root: String,
    pub vertical_step_root: String,
    pub vertical_claim_root: String,
    pub vertical_scenario_count: u64,
    pub vertical_step_count: u64,
    pub vertical_claim_count: u64,
    pub vertical_claims_proven: u64,
    pub vertical_claims_watch: u64,
    pub vertical_claims_failed: u64,
    pub vertical_forced_exit_scenarios: u64,
    pub vertical_liveness_checks: u64,
    pub latest_scenario_id: String,
    pub latest_scenario_status: String,
    pub latest_path_id: String,
    pub latest_exit_id: String,
    pub latest_challenge_id: String,
    pub latest_settlement_id: String,
    pub latest_forced_exit_before_timeout: u64,
    pub latest_forced_exit_after_timeout: u64,
    pub latest_max_user_fee_bps: u64,
    pub latest_privacy_set_size: u64,
    pub latest_step_count: u64,
    pub latest_proven_claim_count: u64,
    pub latest_watch_claim_count: u64,
    pub latest_failed_claim_count: u64,
    pub exit_acceptance_state_root: String,
    pub exit_acceptance_root: String,
    pub exit_acceptance_verdict: String,
    pub exit_acceptance_requirement_count: u64,
    pub exit_acceptance_evidence_count: u64,
    pub exit_acceptance_accepted_count: u64,
    pub exit_acceptance_conditional_count: u64,
    pub exit_acceptance_watch_count: u64,
    pub exit_acceptance_blocked_count: u64,
    pub exit_acceptance_mismatch_count: u64,
    pub exit_acceptance_release_blocker_count: u64,
    pub exit_acceptance_critical_hold_count: u64,
    pub exit_acceptance_release_hold_count: u64,
    pub source_root: String,
}

impl SourceBundle {
    pub fn devnet() -> Self {
        let answer = safety_answer::devnet();
        let vertical = vertical_slice::devnet();
        let acceptance = exit_acceptance::devnet();
        Self::from_states(&answer, &vertical, &acceptance)
    }

    pub fn from_states(
        answer: &safety_answer::State,
        vertical: &vertical_slice::State,
        acceptance: &exit_acceptance::State,
    ) -> Self {
        let latest = vertical.transcripts.values().next_back();
        let latest_scenario_id = latest
            .map(|transcript| transcript.scenario_id.clone())
            .unwrap_or_else(|| "missing".to_string());
        let latest_scenario_status = latest
            .map(|transcript| transcript.status.as_str().to_string())
            .unwrap_or_else(|| "missing".to_string());
        let latest_path_id = latest
            .map(|transcript| transcript.path_id.clone())
            .unwrap_or_else(|| "missing".to_string());
        let latest_exit_id = latest
            .map(|transcript| transcript.exit_id.clone())
            .unwrap_or_else(|| "missing".to_string());
        let latest_challenge_id = latest
            .map(|transcript| transcript.challenge_id.clone())
            .unwrap_or_else(|| "missing".to_string());
        let latest_settlement_id = latest
            .map(|transcript| transcript.settlement_id.clone())
            .unwrap_or_else(|| "missing".to_string());
        let latest_forced_exit_before_timeout = latest
            .map(|transcript| bool_to_u64(transcript.forced_exit_available_before_timeout))
            .unwrap_or(0);
        let latest_forced_exit_after_timeout = latest
            .map(|transcript| bool_to_u64(transcript.forced_exit_available_after_timeout))
            .unwrap_or(0);
        let latest_max_user_fee_bps = latest
            .map(|transcript| transcript.max_user_fee_bps_observed)
            .unwrap_or(0);
        let latest_privacy_set_size = latest
            .map(|transcript| transcript.privacy_set_size_observed)
            .unwrap_or(0);
        let latest_step_count = latest.map(|transcript| transcript.step_count).unwrap_or(0);
        let latest_proven_claim_count = latest
            .map(|transcript| transcript.proven_claim_count)
            .unwrap_or(0);
        let latest_watch_claim_count = latest
            .map(|transcript| transcript.watch_claim_count)
            .unwrap_or(0);
        let latest_failed_claim_count = latest
            .map(|transcript| transcript.failed_claim_count)
            .unwrap_or(1);
        let source_root = source_bundle_root(answer, vertical, acceptance);

        Self {
            safety_answer_state_root: answer.state_root(),
            safety_answer_root: answer.safety_answer_root.clone(),
            user_answer_root: answer.user_answer_root.clone(),
            production_hold_root: answer.production_hold_root.clone(),
            safety_answer_status: answer.verdict.answer_status.clone(),
            safety_user_escape_supported: answer.verdict.user_escape_supported,
            safety_production_blocked: answer.verdict.production_blocked,
            safety_deferred_gate_count: answer.verdict.deferred_gate_count,
            safety_production_blocker_count: answer.verdict.production_blocker_count,
            safety_pq_authority_bound: answer.verdict.pq_authority_bound,
            safety_privacy_boundary_bound: answer.verdict.privacy_boundary_bound,
            vertical_state_root: vertical.state_root(),
            vertical_transcript_root: vertical.roots.transcript_root.clone(),
            vertical_step_root: vertical.roots.step_root.clone(),
            vertical_claim_root: vertical.roots.claim_root.clone(),
            vertical_scenario_count: vertical.transcripts.len() as u64,
            vertical_step_count: vertical.steps.len() as u64,
            vertical_claim_count: vertical.claims.len() as u64,
            vertical_claims_proven: vertical.counters.claims_proven,
            vertical_claims_watch: vertical.counters.claims_watch,
            vertical_claims_failed: vertical.counters.claims_failed,
            vertical_forced_exit_scenarios: vertical.counters.forced_exit_scenarios,
            vertical_liveness_checks: vertical.counters.liveness_checks,
            latest_scenario_id,
            latest_scenario_status,
            latest_path_id,
            latest_exit_id,
            latest_challenge_id,
            latest_settlement_id,
            latest_forced_exit_before_timeout,
            latest_forced_exit_after_timeout,
            latest_max_user_fee_bps,
            latest_privacy_set_size,
            latest_step_count,
            latest_proven_claim_count,
            latest_watch_claim_count,
            latest_failed_claim_count,
            exit_acceptance_state_root: acceptance.state_root(),
            exit_acceptance_root: acceptance.manifest.roots.acceptance_root.clone(),
            exit_acceptance_verdict: acceptance.manifest.verdict.as_str().to_string(),
            exit_acceptance_requirement_count: acceptance.manifest.counters.requirement_count,
            exit_acceptance_evidence_count: acceptance.manifest.counters.evidence_count,
            exit_acceptance_accepted_count: acceptance.manifest.counters.accepted_count,
            exit_acceptance_conditional_count: acceptance.manifest.counters.conditional_count,
            exit_acceptance_watch_count: acceptance.manifest.counters.watch_count,
            exit_acceptance_blocked_count: acceptance.manifest.counters.blocked_count,
            exit_acceptance_mismatch_count: acceptance.manifest.counters.mismatch_count,
            exit_acceptance_release_blocker_count: acceptance
                .manifest
                .counters
                .release_blocker_count,
            exit_acceptance_critical_hold_count: acceptance.manifest.counters.critical_hold_count,
            exit_acceptance_release_hold_count: acceptance.manifest.release_holds.len() as u64,
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "safety_answer_state_root": self.safety_answer_state_root,
            "safety_answer_root": self.safety_answer_root,
            "user_answer_root": self.user_answer_root,
            "production_hold_root": self.production_hold_root,
            "safety_answer_status": self.safety_answer_status,
            "safety_user_escape_supported": self.safety_user_escape_supported,
            "safety_production_blocked": self.safety_production_blocked,
            "safety_deferred_gate_count": self.safety_deferred_gate_count,
            "safety_production_blocker_count": self.safety_production_blocker_count,
            "safety_pq_authority_bound": self.safety_pq_authority_bound,
            "safety_privacy_boundary_bound": self.safety_privacy_boundary_bound,
            "vertical_state_root": self.vertical_state_root,
            "vertical_transcript_root": self.vertical_transcript_root,
            "vertical_step_root": self.vertical_step_root,
            "vertical_claim_root": self.vertical_claim_root,
            "vertical_scenario_count": self.vertical_scenario_count,
            "vertical_step_count": self.vertical_step_count,
            "vertical_claim_count": self.vertical_claim_count,
            "vertical_claims_proven": self.vertical_claims_proven,
            "vertical_claims_watch": self.vertical_claims_watch,
            "vertical_claims_failed": self.vertical_claims_failed,
            "vertical_forced_exit_scenarios": self.vertical_forced_exit_scenarios,
            "vertical_liveness_checks": self.vertical_liveness_checks,
            "latest_scenario_id": self.latest_scenario_id,
            "latest_scenario_status": self.latest_scenario_status,
            "latest_path_id": self.latest_path_id,
            "latest_exit_id": self.latest_exit_id,
            "latest_challenge_id": self.latest_challenge_id,
            "latest_settlement_id": self.latest_settlement_id,
            "latest_forced_exit_before_timeout": self.latest_forced_exit_before_timeout,
            "latest_forced_exit_after_timeout": self.latest_forced_exit_after_timeout,
            "latest_max_user_fee_bps": self.latest_max_user_fee_bps,
            "latest_privacy_set_size": self.latest_privacy_set_size,
            "latest_step_count": self.latest_step_count,
            "latest_proven_claim_count": self.latest_proven_claim_count,
            "latest_watch_claim_count": self.latest_watch_claim_count,
            "latest_failed_claim_count": self.latest_failed_claim_count,
            "exit_acceptance_state_root": self.exit_acceptance_state_root,
            "exit_acceptance_root": self.exit_acceptance_root,
            "exit_acceptance_verdict": self.exit_acceptance_verdict,
            "exit_acceptance_requirement_count": self.exit_acceptance_requirement_count,
            "exit_acceptance_evidence_count": self.exit_acceptance_evidence_count,
            "exit_acceptance_accepted_count": self.exit_acceptance_accepted_count,
            "exit_acceptance_conditional_count": self.exit_acceptance_conditional_count,
            "exit_acceptance_watch_count": self.exit_acceptance_watch_count,
            "exit_acceptance_blocked_count": self.exit_acceptance_blocked_count,
            "exit_acceptance_mismatch_count": self.exit_acceptance_mismatch_count,
            "exit_acceptance_release_blocker_count": self.exit_acceptance_release_blocker_count,
            "exit_acceptance_critical_hold_count": self.exit_acceptance_critical_hold_count,
            "exit_acceptance_release_hold_count": self.exit_acceptance_release_hold_count,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DossierEvidence {
    pub ordinal: u64,
    pub lane: DossierLane,
    pub owner: String,
    pub statement: String,
    pub observed: String,
    pub source_root: String,
    pub evidence_root: String,
    pub review_root: String,
    pub ready_for_user_escape: u64,
    pub production_release_blocked: u64,
    pub public_metadata_units: u64,
    pub evidence_weight_bps: u64,
    pub dossier_evidence_root: String,
}

impl DossierEvidence {
    pub fn devnet(config: &Config, source: &SourceBundle, lane: DossierLane, ordinal: u64) -> Self {
        let source_root = lane_source_root(source, lane);
        let observed = lane_observed(source, lane);
        let ready_for_user_escape = lane_user_escape_ready(config, source, lane);
        let production_release_blocked = lane_production_blocked(config, source, lane);
        let public_metadata_units = 0;
        let evidence_weight_bps = lane_weight_bps(source, lane, ready_for_user_escape);
        let evidence_root = lane_evidence_root(
            config,
            source,
            lane,
            &source_root,
            &observed,
            ready_for_user_escape,
            evidence_weight_bps,
        );
        let review_root = lane_review_root(config, source, lane, &evidence_root);
        let dossier_evidence_root = dossier_evidence_root(
            config,
            source,
            lane,
            ordinal,
            &source_root,
            &evidence_root,
            &review_root,
            ready_for_user_escape,
            production_release_blocked,
        );
        Self {
            ordinal,
            lane,
            owner: lane.owner().to_string(),
            statement: lane.statement().to_string(),
            observed,
            source_root,
            evidence_root,
            review_root,
            ready_for_user_escape,
            production_release_blocked,
            public_metadata_units,
            evidence_weight_bps,
            dossier_evidence_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "lane": self.lane.as_str(),
            "owner": self.owner,
            "statement": self.statement,
            "observed": self.observed,
            "source_root": self.source_root,
            "evidence_root": self.evidence_root,
            "review_root": self.review_root,
            "ready_for_user_escape": self.ready_for_user_escape,
            "production_release_blocked": self.production_release_blocked,
            "public_metadata_units": self.public_metadata_units,
            "evidence_weight_bps": self.evidence_weight_bps,
            "dossier_evidence_root": self.dossier_evidence_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DossierBlocker {
    pub ordinal: u64,
    pub blocker_kind: DossierBlockerKind,
    pub clearance: String,
    pub source_root: String,
    pub evidence_required_root: String,
    pub still_blocks_release: u64,
    pub owner: String,
    pub blocker_root: String,
}

impl DossierBlocker {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        blocker_kind: DossierBlockerKind,
        ordinal: u64,
    ) -> Self {
        let source_root = blocker_source_root(source, blocker_kind);
        let evidence_required_root =
            blocker_evidence_required_root(config, source, blocker_kind, &source_root);
        let still_blocks_release = blocker_still_blocks_release(config, source, blocker_kind);
        let owner = blocker_owner(blocker_kind).to_string();
        let blocker_root = dossier_blocker_root(
            config,
            source,
            blocker_kind,
            ordinal,
            &source_root,
            &evidence_required_root,
            still_blocks_release,
        );
        Self {
            ordinal,
            blocker_kind,
            clearance: blocker_kind.clearance().to_string(),
            source_root,
            evidence_required_root,
            still_blocks_release,
            owner,
            blocker_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "blocker_kind": self.blocker_kind.as_str(),
            "clearance": self.clearance,
            "source_root": self.source_root,
            "evidence_required_root": self.evidence_required_root,
            "still_blocks_release": self.still_blocks_release,
            "owner": self.owner,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DossierVerdict {
    pub dossier_lane_count: u64,
    pub blocker_case_count: u64,
    pub ready_lane_count: u64,
    pub production_blocked_lane_count: u64,
    pub blocker_release_hold_count: u64,
    pub answer_bound: u64,
    pub vertical_slice_bound: u64,
    pub deposit_lock_bound: u64,
    pub private_state_bound: u64,
    pub private_action_bound: u64,
    pub settlement_receipt_bound: u64,
    pub withdrawal_claim_bound: u64,
    pub force_exit_liveness_bound: u64,
    pub pq_authority_bound: u64,
    pub privacy_boundary_bound: u64,
    pub roots_only_public_export: u64,
    pub user_escape_answer_supported: u64,
    pub live_execution_deferred_count: u64,
    pub audit_deferred_count: u64,
    pub release_blocker_count: u64,
    pub dossier_status: String,
    pub user_escape_answer: String,
    pub release_answer: String,
    pub verdict_root: String,
}

impl DossierVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        evidence: &[DossierEvidence],
        blockers: &[DossierBlocker],
    ) -> Self {
        let dossier_lane_count = evidence.len() as u64;
        let blocker_case_count = blockers.len() as u64;
        let ready_lane_count = evidence
            .iter()
            .filter(|record| record.ready_for_user_escape == 1)
            .count() as u64;
        let production_blocked_lane_count = evidence
            .iter()
            .filter(|record| record.production_release_blocked == 1)
            .count() as u64;
        let blocker_release_hold_count = blockers
            .iter()
            .filter(|record| record.still_blocks_release == 1)
            .count() as u64;
        let answer_bound = bool_to_u64(!source.safety_answer_root.is_empty());
        let vertical_slice_bound = bool_to_u64(
            source.vertical_scenario_count > 0
                && source.vertical_step_count > 0
                && source.vertical_claim_count > 0
                && !source.vertical_transcript_root.is_empty(),
        );
        let deposit_lock_bound = bool_to_u64(
            source.latest_path_id != "missing"
                && source.vertical_step_root != "missing"
                && source.latest_step_count > 0,
        );
        let private_state_bound = bool_to_u64(
            source.latest_privacy_set_size > 0
                && source.vertical_claim_root != "missing"
                && source.latest_failed_claim_count == 0,
        );
        let private_action_bound =
            bool_to_u64(source.latest_proven_claim_count > 0 && source.latest_step_count >= 6);
        let settlement_receipt_bound = bool_to_u64(
            source.latest_settlement_id != "missing"
                && !source.exit_acceptance_root.is_empty()
                && source.exit_acceptance_evidence_count > 0,
        );
        let withdrawal_claim_bound =
            bool_to_u64(source.latest_exit_id != "missing" && !source.user_answer_root.is_empty());
        let force_exit_liveness_bound = bool_to_u64(
            source.latest_forced_exit_after_timeout == 1
                && source.vertical_forced_exit_scenarios > 0
                && source.vertical_liveness_checks > 0,
        );
        let pq_authority_bound = source.safety_pq_authority_bound;
        let privacy_boundary_bound = bool_to_u64(
            source.safety_privacy_boundary_bound == 1
                && config.max_public_metadata_units == 0
                && source.latest_privacy_set_size > 0,
        );
        let roots_only_public_export = bool_to_u64(config.max_public_metadata_units == 0);
        let live_execution_deferred_count =
            source.safety_deferred_gate_count + source.exit_acceptance_conditional_count;
        let audit_deferred_count =
            source.exit_acceptance_watch_count + source.exit_acceptance_mismatch_count;
        let release_blocker_count = source.safety_production_blocker_count
            + source.exit_acceptance_release_blocker_count
            + source.exit_acceptance_critical_hold_count
            + source.exit_acceptance_release_hold_count;
        let user_escape_answer_supported = bool_to_u64(
            source.safety_user_escape_supported == config.require_safety_answer
                && answer_bound == config.require_safety_answer
                && vertical_slice_bound == config.require_vertical_slice_scenario
                && deposit_lock_bound == config.require_deposit_lock
                && private_state_bound == config.require_private_state_transition
                && private_action_bound == config.require_private_action_receipt
                && settlement_receipt_bound == config.require_settlement_receipt
                && withdrawal_claim_bound == config.require_withdrawal_claim
                && force_exit_liveness_bound == config.require_force_exit_liveness
                && pq_authority_bound == config.require_pq_authority
                && privacy_boundary_bound == config.require_roots_only_public_export,
        );
        let production_blocked = config.production_release_allowed == 0
            || source.safety_production_blocked == 1
            || live_execution_deferred_count > 0
            || audit_deferred_count > 0
            || release_blocker_count > 0;
        let dossier_status = if dossier_lane_count >= config.min_dossier_lanes
            && blocker_case_count >= config.min_blocker_cases
            && ready_lane_count >= config.min_dossier_lanes.saturating_sub(1)
            && user_escape_answer_supported == 1
            && production_blocked
            && roots_only_public_export == config.require_roots_only_public_export
        {
            "vertical_slice_answer_bound_release_held"
        } else if production_blocked {
            "vertical_slice_answer_watch_release_held"
        } else {
            "vertical_slice_answer_gap"
        }
        .to_string();
        let user_escape_answer = if user_escape_answer_supported == 1 {
            "the dossier binds a concrete deposit/private-action/settlement/withdrawal/forced-exit path to the safety answer"
        } else {
            "the dossier still lacks enough bound evidence to answer yes for user escape under full misbehavior"
        }
        .to_string();
        let release_answer = if production_blocked {
            "production release remains held because live execution, cargo, audit, reserve, watcher-collusion, or governance evidence is still deferred"
        } else {
            "this dossier is not authorized to grant production release"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.evidence_policy),
                HashPart::Str(&source.safety_answer_root),
                HashPart::Str(&source.vertical_transcript_root),
                HashPart::Str(&source.exit_acceptance_root),
                HashPart::U64(dossier_lane_count),
                HashPart::U64(blocker_case_count),
                HashPart::U64(ready_lane_count),
                HashPart::U64(production_blocked_lane_count),
                HashPart::U64(blocker_release_hold_count),
                HashPart::U64(user_escape_answer_supported),
                HashPart::U64(force_exit_liveness_bound),
                HashPart::U64(pq_authority_bound),
                HashPart::U64(privacy_boundary_bound),
                HashPart::U64(live_execution_deferred_count),
                HashPart::U64(audit_deferred_count),
                HashPart::U64(release_blocker_count),
                HashPart::Str(&dossier_status),
            ],
            32,
        );
        Self {
            dossier_lane_count,
            blocker_case_count,
            ready_lane_count,
            production_blocked_lane_count,
            blocker_release_hold_count,
            answer_bound,
            vertical_slice_bound,
            deposit_lock_bound,
            private_state_bound,
            private_action_bound,
            settlement_receipt_bound,
            withdrawal_claim_bound,
            force_exit_liveness_bound,
            pq_authority_bound,
            privacy_boundary_bound,
            roots_only_public_export,
            user_escape_answer_supported,
            live_execution_deferred_count,
            audit_deferred_count,
            release_blocker_count,
            dossier_status,
            user_escape_answer,
            release_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "dossier_lane_count": self.dossier_lane_count,
            "blocker_case_count": self.blocker_case_count,
            "ready_lane_count": self.ready_lane_count,
            "production_blocked_lane_count": self.production_blocked_lane_count,
            "blocker_release_hold_count": self.blocker_release_hold_count,
            "answer_bound": self.answer_bound,
            "vertical_slice_bound": self.vertical_slice_bound,
            "deposit_lock_bound": self.deposit_lock_bound,
            "private_state_bound": self.private_state_bound,
            "private_action_bound": self.private_action_bound,
            "settlement_receipt_bound": self.settlement_receipt_bound,
            "withdrawal_claim_bound": self.withdrawal_claim_bound,
            "force_exit_liveness_bound": self.force_exit_liveness_bound,
            "pq_authority_bound": self.pq_authority_bound,
            "privacy_boundary_bound": self.privacy_boundary_bound,
            "roots_only_public_export": self.roots_only_public_export,
            "user_escape_answer_supported": self.user_escape_answer_supported,
            "live_execution_deferred_count": self.live_execution_deferred_count,
            "audit_deferred_count": self.audit_deferred_count,
            "release_blocker_count": self.release_blocker_count,
            "dossier_status": self.dossier_status,
            "user_escape_answer": self.user_escape_answer,
            "release_answer": self.release_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub evidence: Vec<DossierEvidence>,
    pub blockers: Vec<DossierBlocker>,
    pub verdict: DossierVerdict,
    pub evidence_root: String,
    pub blocker_root: String,
    pub answer_binding_root: String,
    pub vertical_evidence_root: String,
    pub production_hold_root: String,
    pub dossier_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let evidence = DossierLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| DossierEvidence::devnet(&config, &source, *lane, index as u64 + 1))
            .collect::<Vec<_>>();
        let blockers = DossierBlockerKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, kind)| DossierBlocker::devnet(&config, &source, *kind, index as u64 + 1))
            .collect::<Vec<_>>();
        let verdict = DossierVerdict::new(&config, &source, &evidence, &blockers);
        let evidence_root = evidence_vector_root(&evidence);
        let blocker_root = blocker_vector_root(&blockers);
        let answer_binding_root = answer_binding_root(&config, &source, &evidence_root, &verdict);
        let vertical_evidence_root =
            vertical_evidence_root(&config, &source, &evidence_root, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &blocker_root, &verdict);
        let dossier_root = dossier_root(
            &config,
            &source,
            &evidence_root,
            &blocker_root,
            &answer_binding_root,
            &vertical_evidence_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            evidence,
            blockers,
            verdict,
            evidence_root,
            blocker_root,
            answer_binding_root,
            vertical_evidence_root,
            production_hold_root,
            dossier_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_evidence_dossier_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "answer_binding_root": self.answer_binding_root,
            "vertical_evidence_root": self.vertical_evidence_root,
            "production_hold_root": self.production_hold_root,
            "dossier_root": self.dossier_root,
            "verdict": self.verdict.public_record(),
            "evidence": self
                .evidence
                .iter()
                .map(DossierEvidence::public_record)
                .collect::<Vec<_>>(),
            "blockers": self
                .blockers
                .iter()
                .map(DossierBlocker::public_record)
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
                "evidence_root": self.evidence_root,
                "blocker_root": self.blocker_root,
                "answer_binding_root": self.answer_binding_root,
                "vertical_evidence_root": self.vertical_evidence_root,
                "production_hold_root": self.production_hold_root,
                "dossier_root": self.dossier_root,
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
        return Err("user escape vertical dossier chain id mismatch".to_string());
    }
    if config.min_dossier_lanes < DossierLane::ordered().len() as u64 {
        return Err("user escape vertical dossier requires every evidence lane".to_string());
    }
    if config.min_blocker_cases < DossierBlockerKind::ordered().len() as u64 {
        return Err("user escape vertical dossier requires every blocker case".to_string());
    }
    if config.require_safety_answer != 1
        || config.require_vertical_slice_scenario != 1
        || config.require_exit_acceptance_manifest != 1
    {
        return Err("user escape vertical dossier requires all source artifacts".to_string());
    }
    if config.require_deposit_lock != 1
        || config.require_private_state_transition != 1
        || config.require_private_action_receipt != 1
        || config.require_settlement_receipt != 1
        || config.require_withdrawal_claim != 1
        || config.require_force_exit_liveness != 1
        || config.require_operator_independent_escape != 1
        || config.require_pq_authority != 1
        || config.require_roots_only_public_export != 1
    {
        return Err("user escape vertical dossier requires all safety invariants".to_string());
    }
    if config.production_release_allowed != 0 || config.max_public_metadata_units != 0 {
        return Err(
            "user escape vertical dossier cannot release production or metadata".to_string(),
        );
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.safety_answer_root.is_empty()
        || source.vertical_transcript_root.is_empty()
        || source.exit_acceptance_root.is_empty()
    {
        return Err("user escape vertical dossier missing source roots".to_string());
    }
    if source.vertical_scenario_count == 0 || source.exit_acceptance_evidence_count == 0 {
        return Err(
            "user escape vertical dossier missing scenario or evidence records".to_string(),
        );
    }
    Ok(())
}

fn lane_source_root(source: &SourceBundle, lane: DossierLane) -> String {
    match lane {
        DossierLane::SafetyAnswer => source.safety_answer_root.clone(),
        DossierLane::VerticalSliceScenario => source.vertical_transcript_root.clone(),
        DossierLane::ExitAcceptanceManifest => source.exit_acceptance_root.clone(),
        DossierLane::DepositLock => source.latest_path_id.clone(),
        DossierLane::PrivateStateTransition => source.vertical_claim_root.clone(),
        DossierLane::PrivateActionReceipt => source.vertical_step_root.clone(),
        DossierLane::SettlementReceipt => source.latest_settlement_id.clone(),
        DossierLane::WithdrawalClaim => source.latest_exit_id.clone(),
        DossierLane::ForceExitLiveness => source.vertical_state_root.clone(),
        DossierLane::ProductionHold => source.production_hold_root.clone(),
    }
}

fn lane_observed(source: &SourceBundle, lane: DossierLane) -> String {
    match lane {
        DossierLane::SafetyAnswer => format!(
            "status={} user_supported={} production_blocked={} deferred={} blockers={}",
            source.safety_answer_status,
            source.safety_user_escape_supported,
            source.safety_production_blocked,
            source.safety_deferred_gate_count,
            source.safety_production_blocker_count
        ),
        DossierLane::VerticalSliceScenario => format!(
            "scenario={} status={} steps={} claims={} proven={} failed={}",
            source.latest_scenario_id,
            source.latest_scenario_status,
            source.latest_step_count,
            source.vertical_claim_count,
            source.latest_proven_claim_count,
            source.latest_failed_claim_count
        ),
        DossierLane::ExitAcceptanceManifest => format!(
            "verdict={} accepted={} conditional={} watch={} blocked={} release_holds={}",
            source.exit_acceptance_verdict,
            source.exit_acceptance_accepted_count,
            source.exit_acceptance_conditional_count,
            source.exit_acceptance_watch_count,
            source.exit_acceptance_blocked_count,
            source.exit_acceptance_release_hold_count
        ),
        DossierLane::DepositLock => format!(
            "path_id={} transcript={} step_root={}",
            source.latest_path_id, source.vertical_transcript_root, source.vertical_step_root
        ),
        DossierLane::PrivateStateTransition => format!(
            "privacy_set={} claim_root={} failed_claims={}",
            source.latest_privacy_set_size,
            source.vertical_claim_root,
            source.latest_failed_claim_count
        ),
        DossierLane::PrivateActionReceipt => format!(
            "step_count={} proven_claims={} watch_claims={}",
            source.latest_step_count,
            source.latest_proven_claim_count,
            source.latest_watch_claim_count
        ),
        DossierLane::SettlementReceipt => format!(
            "settlement_id={} acceptance_root={} evidence={}",
            source.latest_settlement_id,
            source.exit_acceptance_root,
            source.exit_acceptance_evidence_count
        ),
        DossierLane::WithdrawalClaim => format!(
            "exit_id={} user_answer_root={} withdrawal_claim_bound={}",
            source.latest_exit_id, source.user_answer_root, source.latest_exit_id
        ),
        DossierLane::ForceExitLiveness => format!(
            "before_timeout={} after_timeout={} liveness_checks={} forced_exit_scenarios={}",
            source.latest_forced_exit_before_timeout,
            source.latest_forced_exit_after_timeout,
            source.vertical_liveness_checks,
            source.vertical_forced_exit_scenarios
        ),
        DossierLane::ProductionHold => format!(
            "production_hold_root={} safety_blocked={} release_blockers={} critical_holds={}",
            source.production_hold_root,
            source.safety_production_blocked,
            source.exit_acceptance_release_blocker_count,
            source.exit_acceptance_critical_hold_count
        ),
    }
}

fn lane_user_escape_ready(config: &Config, source: &SourceBundle, lane: DossierLane) -> u64 {
    match lane {
        DossierLane::SafetyAnswer => source.safety_user_escape_supported,
        DossierLane::VerticalSliceScenario => bool_to_u64(
            source.vertical_scenario_count >= config.require_vertical_slice_scenario
                && source.latest_failed_claim_count == 0,
        ),
        DossierLane::ExitAcceptanceManifest => bool_to_u64(
            source.exit_acceptance_evidence_count >= config.require_exit_acceptance_manifest
                && source.exit_acceptance_root.len() > 8,
        ),
        DossierLane::DepositLock => {
            bool_to_u64(source.latest_path_id != "missing" && source.latest_step_count > 0)
        }
        DossierLane::PrivateStateTransition => {
            bool_to_u64(source.latest_privacy_set_size > 0 && source.latest_failed_claim_count == 0)
        }
        DossierLane::PrivateActionReceipt => bool_to_u64(source.latest_proven_claim_count > 0),
        DossierLane::SettlementReceipt => bool_to_u64(source.latest_settlement_id != "missing"),
        DossierLane::WithdrawalClaim => bool_to_u64(source.latest_exit_id != "missing"),
        DossierLane::ForceExitLiveness => source.latest_forced_exit_after_timeout,
        DossierLane::ProductionHold => source.safety_production_blocked,
    }
}

fn lane_production_blocked(config: &Config, source: &SourceBundle, lane: DossierLane) -> u64 {
    let blocked = config.production_release_allowed == 0
        || source.safety_production_blocked == 1
        || source.exit_acceptance_release_blocker_count > 0
        || source.exit_acceptance_critical_hold_count > 0;
    match lane {
        DossierLane::ProductionHold => bool_to_u64(blocked),
        _ => bool_to_u64(blocked || source.safety_deferred_gate_count > 0),
    }
}

fn lane_weight_bps(source: &SourceBundle, lane: DossierLane, ready: u64) -> u64 {
    let base: u64 = match lane {
        DossierLane::SafetyAnswer => 1_400,
        DossierLane::VerticalSliceScenario => 1_300,
        DossierLane::ExitAcceptanceManifest => 1_200,
        DossierLane::DepositLock => 900,
        DossierLane::PrivateStateTransition => 900,
        DossierLane::PrivateActionReceipt => 850,
        DossierLane::SettlementReceipt => 950,
        DossierLane::WithdrawalClaim => 900,
        DossierLane::ForceExitLiveness => 1_100,
        DossierLane::ProductionHold => 500,
    };
    let bonus = if ready == 1 { 100 } else { 0 };
    let penalty = source
        .latest_failed_claim_count
        .saturating_add(source.exit_acceptance_blocked_count)
        .saturating_mul(50);
    base.saturating_add(bonus).saturating_sub(penalty)
}

fn blocker_source_root(source: &SourceBundle, blocker_kind: DossierBlockerKind) -> String {
    match blocker_kind {
        DossierBlockerKind::CargoRuntimeDeferred => source.safety_answer_root.clone(),
        DossierBlockerKind::SecurityAuditDeferred => source.production_hold_root.clone(),
        DossierBlockerKind::LiveFeedDeferred => source.exit_acceptance_root.clone(),
        DossierBlockerKind::MoneroVerifierAbsent => source.vertical_transcript_root.clone(),
        DossierBlockerKind::LiquidityReserveNeedsProof => source.exit_acceptance_root.clone(),
        DossierBlockerKind::PrivacyLeakAuditDeferred => source.vertical_claim_root.clone(),
        DossierBlockerKind::WatcherCollusionNeedsReplay => source.vertical_state_root.clone(),
        DossierBlockerKind::ReleaseGovernanceExecutionDeferred => {
            source.production_hold_root.clone()
        }
        DossierBlockerKind::ProductionReleaseHeld => source.production_hold_root.clone(),
    }
}

fn blocker_owner(blocker_kind: DossierBlockerKind) -> &'static str {
    match blocker_kind {
        DossierBlockerKind::CargoRuntimeDeferred => "runtime_owner",
        DossierBlockerKind::SecurityAuditDeferred => "audit_owner",
        DossierBlockerKind::LiveFeedDeferred => "live_feed_owner",
        DossierBlockerKind::MoneroVerifierAbsent => "monero_watcher_owner",
        DossierBlockerKind::LiquidityReserveNeedsProof => "reserve_owner",
        DossierBlockerKind::PrivacyLeakAuditDeferred => "privacy_owner",
        DossierBlockerKind::WatcherCollusionNeedsReplay => "adversarial_replay_owner",
        DossierBlockerKind::ReleaseGovernanceExecutionDeferred => "governance_owner",
        DossierBlockerKind::ProductionReleaseHeld => "release_owner",
    }
}

fn blocker_still_blocks_release(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: DossierBlockerKind,
) -> u64 {
    match blocker_kind {
        DossierBlockerKind::CargoRuntimeDeferred => {
            bool_to_u64(source.safety_deferred_gate_count > 0)
        }
        DossierBlockerKind::SecurityAuditDeferred => bool_to_u64(
            source.exit_acceptance_watch_count > 0 || source.exit_acceptance_mismatch_count > 0,
        ),
        DossierBlockerKind::LiveFeedDeferred => {
            bool_to_u64(source.exit_acceptance_conditional_count > 0)
        }
        DossierBlockerKind::MoneroVerifierAbsent => 1,
        DossierBlockerKind::LiquidityReserveNeedsProof => {
            bool_to_u64(source.exit_acceptance_release_blocker_count > 0)
        }
        DossierBlockerKind::PrivacyLeakAuditDeferred => bool_to_u64(
            source.safety_privacy_boundary_bound == 0 || source.latest_privacy_set_size == 0,
        ),
        DossierBlockerKind::WatcherCollusionNeedsReplay => bool_to_u64(
            source.vertical_forced_exit_scenarios == 0 || source.vertical_liveness_checks == 0,
        ),
        DossierBlockerKind::ReleaseGovernanceExecutionDeferred => {
            bool_to_u64(source.exit_acceptance_critical_hold_count > 0)
        }
        DossierBlockerKind::ProductionReleaseHeld => bool_to_u64(
            config.production_release_allowed == 0 || source.safety_production_blocked == 1,
        ),
    }
}

fn lane_evidence_root(
    config: &Config,
    source: &SourceBundle,
    lane: DossierLane,
    source_root: &str,
    observed: &str,
    ready: u64,
    weight_bps: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:lane-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.dossier_suite),
            HashPart::Str(lane.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(observed),
            HashPart::Str(&source.source_root),
            HashPart::U64(ready),
            HashPart::U64(weight_bps),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn lane_review_root(
    config: &Config,
    source: &SourceBundle,
    lane: DossierLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:lane-review"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.evidence_policy),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.safety_answer_root),
            HashPart::Str(&source.vertical_transcript_root),
            HashPart::Str(&source.exit_acceptance_root),
        ],
        32,
    )
}

fn dossier_evidence_root(
    config: &Config,
    source: &SourceBundle,
    lane: DossierLane,
    ordinal: u64,
    source_root: &str,
    evidence_root: &str,
    review_root: &str,
    ready: u64,
    production_blocked: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:dossier-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&source.source_root),
            HashPart::Str(source_root),
            HashPart::Str(evidence_root),
            HashPart::Str(review_root),
            HashPart::U64(ordinal),
            HashPart::U64(ready),
            HashPart::U64(production_blocked),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn blocker_evidence_required_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: DossierBlockerKind,
    source_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker-evidence-required"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker_kind.as_str()),
            HashPart::Str(blocker_kind.clearance()),
            HashPart::Str(source_root),
            HashPart::Str(&source.source_root),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn dossier_blocker_root(
    config: &Config,
    source: &SourceBundle,
    blocker_kind: DossierBlockerKind,
    ordinal: u64,
    source_root: &str,
    evidence_required_root: &str,
    still_blocks_release: u64,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:dossier-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker_kind.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(evidence_required_root),
            HashPart::Str(&source.production_hold_root),
            HashPart::U64(ordinal),
            HashPart::U64(still_blocks_release),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn evidence_vector_root(evidence: &[DossierEvidence]) -> String {
    let roots = evidence
        .iter()
        .map(|record| record.dossier_evidence_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:evidence-vector"), &roots)
}

fn blocker_vector_root(blockers: &[DossierBlocker]) -> String {
    let roots = blockers
        .iter()
        .map(|record| record.blocker_root.clone())
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blocker-vector"), &roots)
}

fn answer_binding_root(
    config: &Config,
    source: &SourceBundle,
    evidence_root: &str,
    verdict: &DossierVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:answer-binding"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.evidence_policy),
            HashPart::Str(&source.safety_answer_root),
            HashPart::Str(&source.user_answer_root),
            HashPart::Str(evidence_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.user_escape_answer_supported),
            HashPart::U64(verdict.production_blocked_lane_count),
        ],
        32,
    )
}

fn vertical_evidence_root(
    config: &Config,
    source: &SourceBundle,
    evidence_root: &str,
    verdict: &DossierVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:vertical-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.dossier_suite),
            HashPart::Str(&source.vertical_state_root),
            HashPart::Str(&source.vertical_transcript_root),
            HashPart::Str(&source.vertical_step_root),
            HashPart::Str(&source.vertical_claim_root),
            HashPart::Str(evidence_root),
            HashPart::U64(verdict.deposit_lock_bound),
            HashPart::U64(verdict.private_state_bound),
            HashPart::U64(verdict.private_action_bound),
            HashPart::U64(verdict.settlement_receipt_bound),
            HashPart::U64(verdict.withdrawal_claim_bound),
            HashPart::U64(verdict.force_exit_liveness_bound),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    blocker_root: &str,
    verdict: &DossierVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:production-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.evidence_policy),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.live_execution_deferred_count),
            HashPart::U64(verdict.audit_deferred_count),
            HashPart::U64(verdict.release_blocker_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn dossier_root(
    config: &Config,
    source: &SourceBundle,
    evidence_root: &str,
    blocker_root: &str,
    answer_binding_root: &str,
    vertical_evidence_root: &str,
    production_hold_root: &str,
    verdict: &DossierVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:dossier-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.dossier_suite),
            HashPart::Str(&source.source_root),
            HashPart::Str(evidence_root),
            HashPart::Str(blocker_root),
            HashPart::Str(answer_binding_root),
            HashPart::Str(vertical_evidence_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.user_escape_answer_supported),
            HashPart::U64(config.max_public_metadata_units),
        ],
        32,
    )
}

fn source_bundle_root(
    answer: &safety_answer::State,
    vertical: &vertical_slice::State,
    acceptance: &exit_acceptance::State,
) -> String {
    let answer_state_root = answer.state_root();
    let vertical_state_root = vertical.state_root();
    let acceptance_state_root = acceptance.state_root();
    domain_hash(
        &format!("{DOMAIN}:source-bundle"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&answer_state_root),
            HashPart::Str(&answer.safety_answer_root),
            HashPart::Str(&vertical_state_root),
            HashPart::Str(&vertical.roots.transcript_root),
            HashPart::Str(&vertical.roots.step_root),
            HashPart::Str(&vertical.roots.claim_root),
            HashPart::Str(&acceptance_state_root),
            HashPart::Str(&acceptance.manifest.roots.acceptance_root),
            HashPart::Str(acceptance.manifest.verdict.as_str()),
            HashPart::U64(answer.verdict.user_escape_supported),
            HashPart::U64(vertical.transcripts.len() as u64),
            HashPart::U64(acceptance.manifest.counters.evidence_count),
        ],
        32,
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source_root = record_root("fallback-source", &json!({"reason": &reason}));
    let source = SourceBundle {
        safety_answer_state_root: "fallback".to_string(),
        safety_answer_root: record_root("fallback-safety-answer", &json!({"reason": &reason})),
        user_answer_root: record_root("fallback-user-answer", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        safety_answer_status: "fallback".to_string(),
        safety_user_escape_supported: 0,
        safety_production_blocked: 1,
        safety_deferred_gate_count: 1,
        safety_production_blocker_count: 1,
        safety_pq_authority_bound: 0,
        safety_privacy_boundary_bound: 0,
        vertical_state_root: "fallback".to_string(),
        vertical_transcript_root: record_root("fallback-transcript", &json!({"reason": &reason})),
        vertical_step_root: record_root("fallback-steps", &json!({"reason": &reason})),
        vertical_claim_root: record_root("fallback-claims", &json!({"reason": &reason})),
        vertical_scenario_count: 0,
        vertical_step_count: 0,
        vertical_claim_count: 0,
        vertical_claims_proven: 0,
        vertical_claims_watch: 0,
        vertical_claims_failed: 1,
        vertical_forced_exit_scenarios: 0,
        vertical_liveness_checks: 0,
        latest_scenario_id: "fallback".to_string(),
        latest_scenario_status: "fallback".to_string(),
        latest_path_id: "fallback".to_string(),
        latest_exit_id: "fallback".to_string(),
        latest_challenge_id: "fallback".to_string(),
        latest_settlement_id: "fallback".to_string(),
        latest_forced_exit_before_timeout: 0,
        latest_forced_exit_after_timeout: 0,
        latest_max_user_fee_bps: 0,
        latest_privacy_set_size: 0,
        latest_step_count: 0,
        latest_proven_claim_count: 0,
        latest_watch_claim_count: 0,
        latest_failed_claim_count: 1,
        exit_acceptance_state_root: "fallback".to_string(),
        exit_acceptance_root: record_root("fallback-acceptance", &json!({"reason": &reason})),
        exit_acceptance_verdict: "fallback".to_string(),
        exit_acceptance_requirement_count: 0,
        exit_acceptance_evidence_count: 0,
        exit_acceptance_accepted_count: 0,
        exit_acceptance_conditional_count: 0,
        exit_acceptance_watch_count: 0,
        exit_acceptance_blocked_count: 1,
        exit_acceptance_mismatch_count: 1,
        exit_acceptance_release_blocker_count: 1,
        exit_acceptance_critical_hold_count: 1,
        exit_acceptance_release_hold_count: 1,
        source_root,
    };
    let evidence = DossierLane::ordered()
        .iter()
        .enumerate()
        .map(|(index, lane)| DossierEvidence::devnet(&config, &source, *lane, index as u64 + 1))
        .collect::<Vec<_>>();
    let blockers = DossierBlockerKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, kind)| DossierBlocker::devnet(&config, &source, *kind, index as u64 + 1))
        .collect::<Vec<_>>();
    let verdict = DossierVerdict::new(&config, &source, &evidence, &blockers);
    let evidence_root = evidence_vector_root(&evidence);
    let blocker_root = blocker_vector_root(&blockers);
    let answer_binding_root = answer_binding_root(&config, &source, &evidence_root, &verdict);
    let vertical_evidence_root = vertical_evidence_root(&config, &source, &evidence_root, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &blocker_root, &verdict);
    let dossier_root = dossier_root(
        &config,
        &source,
        &evidence_root,
        &blocker_root,
        &answer_binding_root,
        &vertical_evidence_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        evidence,
        blockers,
        verdict,
        evidence_root,
        blocker_root,
        answer_binding_root,
        vertical_evidence_root,
        production_hold_root,
        dossier_root,
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
