use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_heavy_gate_execution_receipt_runtime as heavy_execution,
    monero_l2_pq_bridge_exit_canonical_heavy_gate_readiness_receipt_runtime as readiness,
    monero_l2_pq_bridge_exit_canonical_user_escape_security_review_final_release_acceptance_binding_runtime as acceptance,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeSecurityReviewFinalReleaseAcceptanceExecutionReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_FINAL_RELEASE_ACCEPTANCE_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION:
    &str = "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-security-review-final-release-acceptance-execution-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_FINAL_RELEASE_ACCEPTANCE_EXECUTION_RECEIPT_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-security-review-final-release-acceptance-execution-receipt";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub execution_receipt_suite: String,
    pub execution_policy: String,
    pub min_execution_lanes: u64,
    pub min_blocker_cases: u64,
    pub require_acceptance_binding: u64,
    pub require_heavy_gate_execution_receipt: u64,
    pub require_heavy_gate_readiness_receipt: u64,
    pub require_wallet_force_exit_replay: u64,
    pub require_operator_independence: u64,
    pub require_pq_release_authority: u64,
    pub require_roots_only_privacy: u64,
    pub require_cargo_runtime_execution: u64,
    pub require_security_audit_signature: u64,
    pub release_permit_count: u64,
    pub final_acceptance_executed: u64,
    pub production_release_allowed: u64,
    pub max_public_metadata_units: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            execution_receipt_suite:
                "monero-l2-pq-bridge-exit-final-acceptance-execution-receipt-v1".to_string(),
            execution_policy: "release-held-until-live-heavy-gates-execute-v1".to_string(),
            min_execution_lanes: 10,
            min_blocker_cases: 8,
            require_acceptance_binding: 1,
            require_heavy_gate_execution_receipt: 1,
            require_heavy_gate_readiness_receipt: 1,
            require_wallet_force_exit_replay: 1,
            require_operator_independence: 1,
            require_pq_release_authority: 1,
            require_roots_only_privacy: 1,
            require_cargo_runtime_execution: 1,
            require_security_audit_signature: 1,
            release_permit_count: 0,
            final_acceptance_executed: 0,
            production_release_allowed: 0,
            max_public_metadata_units: 0,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "execution_receipt_suite": self.execution_receipt_suite,
            "execution_policy": self.execution_policy,
            "min_execution_lanes": self.min_execution_lanes,
            "min_blocker_cases": self.min_blocker_cases,
            "require_acceptance_binding": self.require_acceptance_binding,
            "require_heavy_gate_execution_receipt": self.require_heavy_gate_execution_receipt,
            "require_heavy_gate_readiness_receipt": self.require_heavy_gate_readiness_receipt,
            "require_wallet_force_exit_replay": self.require_wallet_force_exit_replay,
            "require_operator_independence": self.require_operator_independence,
            "require_pq_release_authority": self.require_pq_release_authority,
            "require_roots_only_privacy": self.require_roots_only_privacy,
            "require_cargo_runtime_execution": self.require_cargo_runtime_execution,
            "require_security_audit_signature": self.require_security_audit_signature,
            "release_permit_count": self.release_permit_count,
            "final_acceptance_executed": self.final_acceptance_executed,
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
    AcceptanceBindingRoot,
    HeavyGateExecutionReceipt,
    HeavyGateReadinessReceipt,
    WalletReplayability,
    ForcedExitCriticalCoverage,
    OperatorIndependence,
    PqReleaseAuthority,
    PrivacyBoundary,
    CargoRuntimeExecution,
    FinalExecutionSeal,
}

impl ExecutionReceiptLane {
    pub fn ordered() -> [Self; 10] {
        [
            Self::AcceptanceBindingRoot,
            Self::HeavyGateExecutionReceipt,
            Self::HeavyGateReadinessReceipt,
            Self::WalletReplayability,
            Self::ForcedExitCriticalCoverage,
            Self::OperatorIndependence,
            Self::PqReleaseAuthority,
            Self::PrivacyBoundary,
            Self::CargoRuntimeExecution,
            Self::FinalExecutionSeal,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptanceBindingRoot => "acceptance_binding_root",
            Self::HeavyGateExecutionReceipt => "heavy_gate_execution_receipt",
            Self::HeavyGateReadinessReceipt => "heavy_gate_readiness_receipt",
            Self::WalletReplayability => "wallet_replayability",
            Self::ForcedExitCriticalCoverage => "forced_exit_critical_coverage",
            Self::OperatorIndependence => "operator_independence",
            Self::PqReleaseAuthority => "pq_release_authority",
            Self::PrivacyBoundary => "privacy_boundary",
            Self::CargoRuntimeExecution => "cargo_runtime_execution",
            Self::FinalExecutionSeal => "final_execution_seal",
        }
    }

    pub fn owner_lane(self) -> &'static str {
        match self {
            Self::AcceptanceBindingRoot => "release_acceptance_owner",
            Self::HeavyGateExecutionReceipt => "heavy_gate_execution_owner",
            Self::HeavyGateReadinessReceipt => "heavy_gate_readiness_owner",
            Self::WalletReplayability => "wallet_recovery_owner",
            Self::ForcedExitCriticalCoverage => "forced_exit_owner",
            Self::OperatorIndependence => "sequencer_failure_owner",
            Self::PqReleaseAuthority => "pq_release_authority_owner",
            Self::PrivacyBoundary => "privacy_boundary_owner",
            Self::CargoRuntimeExecution => "runtime_harness_owner",
            Self::FinalExecutionSeal => "release_governance_owner",
        }
    }

    pub fn required_evidence(self) -> &'static str {
        match self {
            Self::AcceptanceBindingRoot => {
                "final acceptance binding root, blocker root, hold root, and verdict root"
            }
            Self::HeavyGateExecutionReceipt => {
                "heavy-gate execution receipt, transcript root, step root, and blocker root"
            }
            Self::HeavyGateReadinessReceipt => {
                "readiness receipt root, schedule root, production blocker root, and deferred execution root"
            }
            Self::WalletReplayability => {
                "wallet recovery root proving local reconstruction and forced-exit replayability"
            }
            Self::ForcedExitCriticalCoverage => {
                "every force-exit-critical stage covered by operator-independent receipt steps"
            }
            Self::OperatorIndependence => {
                "receipt counters showing no sequencer/operator cooperation requirement"
            }
            Self::PqReleaseAuthority => {
                "acceptance source roots for PQ authority, custody policy, and authority crosscheck"
            }
            Self::PrivacyBoundary => {
                "committed/encrypted/wallet-local roots with zero public wallet metadata release"
            }
            Self::CargoRuntimeExecution => {
                "cargo/runtime execution and signed audit evidence, not simulated release-only material"
            }
            Self::FinalExecutionSeal => {
                "release permit, final acceptance execution, and production release counters all held at zero"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionBlockerKind {
    AcceptanceBindingHeld,
    HeavyGateExecutionDeferred,
    CargoRuntimeDeferred,
    SecurityAuditDeferred,
    WalletReplayGap,
    PqAuthorityGap,
    PrivacyBoundaryGap,
    ProductionReleaseAttempted,
}

impl ExecutionBlockerKind {
    pub fn ordered() -> [Self; 8] {
        [
            Self::AcceptanceBindingHeld,
            Self::HeavyGateExecutionDeferred,
            Self::CargoRuntimeDeferred,
            Self::SecurityAuditDeferred,
            Self::WalletReplayGap,
            Self::PqAuthorityGap,
            Self::PrivacyBoundaryGap,
            Self::ProductionReleaseAttempted,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptanceBindingHeld => "acceptance_binding_held",
            Self::HeavyGateExecutionDeferred => "heavy_gate_execution_deferred",
            Self::CargoRuntimeDeferred => "cargo_runtime_deferred",
            Self::SecurityAuditDeferred => "security_audit_deferred",
            Self::WalletReplayGap => "wallet_replay_gap",
            Self::PqAuthorityGap => "pq_authority_gap",
            Self::PrivacyBoundaryGap => "privacy_boundary_gap",
            Self::ProductionReleaseAttempted => "production_release_attempted",
        }
    }

    pub fn clearance(self) -> &'static str {
        match self {
            Self::AcceptanceBindingHeld => {
                "keep acceptance held until every execution receipt lane is live-proven"
            }
            Self::HeavyGateExecutionDeferred => {
                "execute heavy gates and attach the hard execution receipt transcript"
            }
            Self::CargoRuntimeDeferred => {
                "run cargo/runtime gates and bind their result roots into this receipt"
            }
            Self::SecurityAuditDeferred => {
                "attach signed security and privacy audit closure receipts"
            }
            Self::WalletReplayGap => {
                "prove the wallet can reconstruct notes, receipts, and forced-exit claims locally"
            }
            Self::PqAuthorityGap => {
                "prove PQ signer quorum, custody policy, and authority crosscheck roots"
            }
            Self::PrivacyBoundaryGap => {
                "prove roots-only output with no public wallet metadata or linkage exports"
            }
            Self::ProductionReleaseAttempted => {
                "reject production release until release permits and final execution counters are explicitly nonzero under live evidence"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBundle {
    pub acceptance_state_root: String,
    pub acceptance_final_root: String,
    pub acceptance_hold_root: String,
    pub acceptance_blocker_root: String,
    pub acceptance_verdict_root: String,
    pub acceptance_status: String,
    pub acceptance_lane_count: u64,
    pub acceptance_rejection_count: u64,
    pub acceptance_release_hold_count: u64,
    pub acceptance_permitted_count: u64,
    pub final_acceptance_accepted_count: u64,
    pub acceptance_production_release_allowed: u64,
    pub acceptance_cargo_deferred_count: u64,
    pub acceptance_zero_linkage_count: u64,
    pub execution_receipt_state_root: String,
    pub execution_receipt_root: String,
    pub execution_transcript_root: String,
    pub execution_step_root: String,
    pub execution_blocker_root: String,
    pub execution_public_root: String,
    pub execution_committed_root: String,
    pub execution_encrypted_root: String,
    pub execution_wallet_recovery_root: String,
    pub execution_wallet_safe_count: u64,
    pub execution_operator_independent_count: u64,
    pub execution_production_safe_count: u64,
    pub execution_force_exit_critical_count: u64,
    pub execution_deferred_count: u64,
    pub execution_watch_count: u64,
    pub execution_blocked_count: u64,
    pub execution_rejected_count: u64,
    pub execution_can_wallet_force_exit: u64,
    pub execution_production_blocked: u64,
    pub readiness_state_root: String,
    pub readiness_receipt_root: String,
    pub readiness_schedule_root: String,
    pub readiness_production_blocker_root: String,
    pub readiness_deferred_execution_root: String,
    pub readiness_heavy_gates_may_be_scheduled: u64,
    pub readiness_heavy_gates_executed: u64,
    pub readiness_production_release_blocked: u64,
    pub readiness_schedule_ready_lanes: u64,
    pub readiness_deferred_lanes: u64,
    pub readiness_blocked_lanes: u64,
    pub readiness_production_blockers: u64,
    pub pq_authority_root: String,
    pub custody_authority_root: String,
    pub authority_crosscheck_root: String,
    pub forced_exit_dry_run_root: String,
    pub wallet_handoff_root: String,
    pub source_root: String,
}

impl SourceBundle {
    pub fn devnet() -> Self {
        let acceptance = acceptance::devnet();
        let execution = heavy_execution::devnet();
        let readiness = readiness::devnet();
        Self::from_states(&acceptance, &execution, &readiness)
    }

    pub fn from_states(
        acceptance: &acceptance::State,
        execution: &heavy_execution::State,
        readiness: &readiness::State,
    ) -> Self {
        let source_root = source_bundle_root(acceptance, execution, readiness);
        Self {
            acceptance_state_root: acceptance.state_root(),
            acceptance_final_root: acceptance.final_acceptance_root.clone(),
            acceptance_hold_root: acceptance.acceptance_hold_root.clone(),
            acceptance_blocker_root: acceptance.acceptance_blocker_root.clone(),
            acceptance_verdict_root: acceptance.verdict.verdict_root.clone(),
            acceptance_status: acceptance.verdict.acceptance_status.clone(),
            acceptance_lane_count: acceptance.verdict.acceptance_lane_count,
            acceptance_rejection_count: acceptance.verdict.rejection_case_count,
            acceptance_release_hold_count: acceptance.verdict.release_hold_count,
            acceptance_permitted_count: acceptance.verdict.acceptance_permitted_count,
            final_acceptance_accepted_count: acceptance.verdict.final_release_accepted_count,
            acceptance_production_release_allowed: acceptance.verdict.production_release_allowed,
            acceptance_cargo_deferred_count: acceptance.verdict.cargo_deferred_count,
            acceptance_zero_linkage_count: acceptance.verdict.zero_linkage_count,
            execution_receipt_state_root: execution.state_root(),
            execution_receipt_root: execution.receipt.receipt_root.clone(),
            execution_transcript_root: execution.receipt.transcript_root.clone(),
            execution_step_root: execution.receipt.step_root.clone(),
            execution_blocker_root: execution.receipt.blocker_root.clone(),
            execution_public_root: execution.receipt.public_root.clone(),
            execution_committed_root: execution.receipt.committed_root.clone(),
            execution_encrypted_root: execution.receipt.encrypted_root.clone(),
            execution_wallet_recovery_root: execution.receipt.wallet_recovery_root.clone(),
            execution_wallet_safe_count: execution.receipt.counters.wallet_safe,
            execution_operator_independent_count: execution.receipt.counters.operator_independent,
            execution_production_safe_count: execution.receipt.counters.production_safe,
            execution_force_exit_critical_count: execution.receipt.counters.force_exit_critical,
            execution_deferred_count: execution.receipt.counters.deferred,
            execution_watch_count: execution.receipt.counters.watch,
            execution_blocked_count: execution.receipt.counters.blocked,
            execution_rejected_count: execution.receipt.counters.rejected,
            execution_can_wallet_force_exit: bool_to_u64(execution.can_wallet_force_exit()),
            execution_production_blocked: bool_to_u64(execution.production_blocked()),
            readiness_state_root: readiness.state_root(),
            readiness_receipt_root: readiness.receipt.roots.receipt_root.clone(),
            readiness_schedule_root: readiness.receipt.roots.schedule_root.clone(),
            readiness_production_blocker_root: readiness
                .receipt
                .roots
                .production_blocker_root
                .clone(),
            readiness_deferred_execution_root: readiness
                .receipt
                .roots
                .deferred_execution_root
                .clone(),
            readiness_heavy_gates_may_be_scheduled: bool_to_u64(
                readiness.receipt.heavy_gates_may_be_scheduled,
            ),
            readiness_heavy_gates_executed: bool_to_u64(readiness.receipt.heavy_gates_executed),
            readiness_production_release_blocked: bool_to_u64(
                readiness.receipt.production_release_blocked,
            ),
            readiness_schedule_ready_lanes: readiness.receipt.schedule_ready_lanes,
            readiness_deferred_lanes: readiness.receipt.deferred_lanes,
            readiness_blocked_lanes: readiness.receipt.blocked_lanes,
            readiness_production_blockers: readiness.receipt.production_blockers,
            pq_authority_root: acceptance
                .source
                .pq_authority_verification_state_root
                .clone(),
            custody_authority_root: acceptance
                .source
                .custody_release_authority_spec_state_root
                .clone(),
            authority_crosscheck_root: acceptance.source.authority_crosscheck_state_root.clone(),
            forced_exit_dry_run_root: acceptance.source.forced_exit_dry_run_state_root.clone(),
            wallet_handoff_root: acceptance.source.wallet_handoff_state_root.clone(),
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "acceptance_state_root": self.acceptance_state_root,
            "acceptance_final_root": self.acceptance_final_root,
            "acceptance_hold_root": self.acceptance_hold_root,
            "acceptance_blocker_root": self.acceptance_blocker_root,
            "acceptance_verdict_root": self.acceptance_verdict_root,
            "acceptance_status": self.acceptance_status,
            "acceptance_lane_count": self.acceptance_lane_count,
            "acceptance_rejection_count": self.acceptance_rejection_count,
            "acceptance_release_hold_count": self.acceptance_release_hold_count,
            "acceptance_permitted_count": self.acceptance_permitted_count,
            "final_acceptance_accepted_count": self.final_acceptance_accepted_count,
            "acceptance_production_release_allowed": self.acceptance_production_release_allowed,
            "acceptance_cargo_deferred_count": self.acceptance_cargo_deferred_count,
            "acceptance_zero_linkage_count": self.acceptance_zero_linkage_count,
            "execution_receipt_state_root": self.execution_receipt_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "execution_transcript_root": self.execution_transcript_root,
            "execution_step_root": self.execution_step_root,
            "execution_blocker_root": self.execution_blocker_root,
            "execution_public_root": self.execution_public_root,
            "execution_committed_root": self.execution_committed_root,
            "execution_encrypted_root": self.execution_encrypted_root,
            "execution_wallet_recovery_root": self.execution_wallet_recovery_root,
            "execution_wallet_safe_count": self.execution_wallet_safe_count,
            "execution_operator_independent_count": self.execution_operator_independent_count,
            "execution_production_safe_count": self.execution_production_safe_count,
            "execution_force_exit_critical_count": self.execution_force_exit_critical_count,
            "execution_deferred_count": self.execution_deferred_count,
            "execution_watch_count": self.execution_watch_count,
            "execution_blocked_count": self.execution_blocked_count,
            "execution_rejected_count": self.execution_rejected_count,
            "execution_can_wallet_force_exit": self.execution_can_wallet_force_exit,
            "execution_production_blocked": self.execution_production_blocked,
            "readiness_state_root": self.readiness_state_root,
            "readiness_receipt_root": self.readiness_receipt_root,
            "readiness_schedule_root": self.readiness_schedule_root,
            "readiness_production_blocker_root": self.readiness_production_blocker_root,
            "readiness_deferred_execution_root": self.readiness_deferred_execution_root,
            "readiness_heavy_gates_may_be_scheduled": self.readiness_heavy_gates_may_be_scheduled,
            "readiness_heavy_gates_executed": self.readiness_heavy_gates_executed,
            "readiness_production_release_blocked": self.readiness_production_release_blocked,
            "readiness_schedule_ready_lanes": self.readiness_schedule_ready_lanes,
            "readiness_deferred_lanes": self.readiness_deferred_lanes,
            "readiness_blocked_lanes": self.readiness_blocked_lanes,
            "readiness_production_blockers": self.readiness_production_blockers,
            "pq_authority_root": self.pq_authority_root,
            "custody_authority_root": self.custody_authority_root,
            "authority_crosscheck_root": self.authority_crosscheck_root,
            "forced_exit_dry_run_root": self.forced_exit_dry_run_root,
            "wallet_handoff_root": self.wallet_handoff_root,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceiptCheck {
    pub ordinal: u64,
    pub lane: ExecutionReceiptLane,
    pub owner_lane: String,
    pub required_evidence: String,
    pub observed: String,
    pub source_root: String,
    pub evidence_root: String,
    pub blocker_root: String,
    pub closure_root: String,
    pub release_permitted: u64,
    pub final_acceptance_executed: u64,
    pub production_release_allowed: u64,
    pub check_root: String,
}

impl ExecutionReceiptCheck {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        lane: ExecutionReceiptLane,
        ordinal: u64,
    ) -> Self {
        let source_root = lane_source_root(source, lane);
        let observed = lane_observed(source, lane);
        let evidence_root = lane_evidence_root(config, source, lane, &source_root, &observed);
        let blocker_root = lane_blocker_root(config, source, lane, &evidence_root);
        let closure_root = lane_closure_root(config, source, lane, &blocker_root);
        let check_root = execution_check_root(
            config,
            source,
            lane,
            ordinal,
            &source_root,
            &evidence_root,
            &blocker_root,
            &closure_root,
        );
        Self {
            ordinal,
            lane,
            owner_lane: lane.owner_lane().to_string(),
            required_evidence: lane.required_evidence().to_string(),
            observed,
            source_root,
            evidence_root,
            blocker_root,
            closure_root,
            release_permitted: config.release_permit_count,
            final_acceptance_executed: config.final_acceptance_executed,
            production_release_allowed: config.production_release_allowed,
            check_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "lane": self.lane.as_str(),
            "owner_lane": self.owner_lane,
            "required_evidence": self.required_evidence,
            "observed": self.observed,
            "source_root": self.source_root,
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "closure_root": self.closure_root,
            "release_permitted": self.release_permitted,
            "final_acceptance_executed": self.final_acceptance_executed,
            "production_release_allowed": self.production_release_allowed,
            "check_root": self.check_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionBlockerCase {
    pub ordinal: u64,
    pub kind: ExecutionBlockerKind,
    pub source_root: String,
    pub blocker_root: String,
    pub required_clearance: String,
    pub recovery_root: String,
    pub blocks_release: u64,
    pub case_root: String,
}

impl ExecutionBlockerCase {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        kind: ExecutionBlockerKind,
        ordinal: u64,
    ) -> Self {
        let source_root = blocker_source_root(source, kind);
        let blocker_root = blocker_case_blocker_root(config, source, kind, &source_root);
        let recovery_root = blocker_recovery_root(config, source, kind, &blocker_root);
        let case_root = blocker_case_root(
            config,
            source,
            kind,
            ordinal,
            &source_root,
            &blocker_root,
            &recovery_root,
        );
        Self {
            ordinal,
            kind,
            source_root,
            blocker_root,
            required_clearance: kind.clearance().to_string(),
            recovery_root,
            blocks_release: 1,
            case_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "kind": self.kind.as_str(),
            "source_root": self.source_root,
            "blocker_root": self.blocker_root,
            "required_clearance": self.required_clearance,
            "recovery_root": self.recovery_root,
            "blocks_release": self.blocks_release,
            "case_root": self.case_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceiptVerdict {
    pub execution_lane_count: u64,
    pub blocker_case_count: u64,
    pub release_permitted_count: u64,
    pub final_acceptance_executed_count: u64,
    pub production_release_allowed_count: u64,
    pub release_hold_count: u64,
    pub blocker_hold_count: u64,
    pub wallet_force_exit_supported: u64,
    pub operator_independent_critical_count: u64,
    pub force_exit_critical_count: u64,
    pub heavy_gates_executed_count: u64,
    pub cargo_deferred_count: u64,
    pub audit_deferred_count: u64,
    pub production_blocker_count: u64,
    pub privacy_boundary_count: u64,
    pub verdict_status: String,
    pub verdict_root: String,
}

impl ExecutionReceiptVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        checks: &[ExecutionReceiptCheck],
        blockers: &[ExecutionBlockerCase],
    ) -> Self {
        let execution_lane_count = checks.len() as u64;
        let blocker_case_count = blockers.len() as u64;
        let release_permitted_count = checks
            .iter()
            .filter(|check| check.release_permitted == 1)
            .count() as u64;
        let final_acceptance_executed_count = checks
            .iter()
            .filter(|check| check.final_acceptance_executed == 1)
            .count() as u64;
        let production_release_allowed_count = checks
            .iter()
            .filter(|check| check.production_release_allowed == 1)
            .count() as u64;
        let release_hold_count = checks
            .iter()
            .filter(|check| check.release_permitted == 0)
            .count() as u64;
        let blocker_hold_count = blockers
            .iter()
            .filter(|case| case.blocks_release == 1)
            .count() as u64;
        let wallet_force_exit_supported = source.execution_can_wallet_force_exit;
        let operator_independent_critical_count = source.execution_operator_independent_count;
        let force_exit_critical_count = source.execution_force_exit_critical_count;
        let heavy_gates_executed_count = source.readiness_heavy_gates_executed;
        let cargo_deferred_count =
            source.execution_deferred_count + source.readiness_deferred_lanes;
        let audit_deferred_count = source.readiness_deferred_lanes;
        let production_blocker_count =
            source.readiness_production_blockers + source.execution_production_blocked;
        let privacy_boundary_count = if source.acceptance_zero_linkage_count
            >= source.acceptance_lane_count
            && config.max_public_metadata_units == 0
        {
            execution_lane_count
        } else {
            0
        };
        let verdict_status = if execution_lane_count >= config.min_execution_lanes
            && blocker_case_count >= config.min_blocker_cases
            && release_permitted_count == config.release_permit_count
            && final_acceptance_executed_count == config.final_acceptance_executed
            && production_release_allowed_count == config.production_release_allowed
            && release_hold_count == execution_lane_count
            && blocker_hold_count == blocker_case_count
            && wallet_force_exit_supported == config.require_wallet_force_exit_replay
            && force_exit_critical_count > 0
            && operator_independent_critical_count >= force_exit_critical_count
            && heavy_gates_executed_count == 0
            && cargo_deferred_count > 0
            && production_blocker_count > 0
            && privacy_boundary_count == execution_lane_count
        {
            "final_acceptance_execution_receipt_ready_release_held"
        } else {
            "final_acceptance_execution_receipt_gap_release_held"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.execution_policy),
                HashPart::Str(&source.acceptance_final_root),
                HashPart::Str(&source.execution_receipt_root),
                HashPart::Str(&source.readiness_receipt_root),
                HashPart::U64(execution_lane_count),
                HashPart::U64(blocker_case_count),
                HashPart::U64(release_permitted_count),
                HashPart::U64(final_acceptance_executed_count),
                HashPart::U64(production_release_allowed_count),
                HashPart::U64(release_hold_count),
                HashPart::U64(blocker_hold_count),
                HashPart::U64(wallet_force_exit_supported),
                HashPart::U64(operator_independent_critical_count),
                HashPart::U64(force_exit_critical_count),
                HashPart::U64(heavy_gates_executed_count),
                HashPart::U64(cargo_deferred_count),
                HashPart::U64(audit_deferred_count),
                HashPart::U64(production_blocker_count),
                HashPart::U64(privacy_boundary_count),
                HashPart::Str(&verdict_status),
            ],
            32,
        );
        Self {
            execution_lane_count,
            blocker_case_count,
            release_permitted_count,
            final_acceptance_executed_count,
            production_release_allowed_count,
            release_hold_count,
            blocker_hold_count,
            wallet_force_exit_supported,
            operator_independent_critical_count,
            force_exit_critical_count,
            heavy_gates_executed_count,
            cargo_deferred_count,
            audit_deferred_count,
            production_blocker_count,
            privacy_boundary_count,
            verdict_status,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_lane_count": self.execution_lane_count,
            "blocker_case_count": self.blocker_case_count,
            "release_permitted_count": self.release_permitted_count,
            "final_acceptance_executed_count": self.final_acceptance_executed_count,
            "production_release_allowed_count": self.production_release_allowed_count,
            "release_hold_count": self.release_hold_count,
            "blocker_hold_count": self.blocker_hold_count,
            "wallet_force_exit_supported": self.wallet_force_exit_supported,
            "operator_independent_critical_count": self.operator_independent_critical_count,
            "force_exit_critical_count": self.force_exit_critical_count,
            "heavy_gates_executed_count": self.heavy_gates_executed_count,
            "cargo_deferred_count": self.cargo_deferred_count,
            "audit_deferred_count": self.audit_deferred_count,
            "production_blocker_count": self.production_blocker_count,
            "privacy_boundary_count": self.privacy_boundary_count,
            "verdict_status": self.verdict_status,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub execution_checks: Vec<ExecutionReceiptCheck>,
    pub blocker_cases: Vec<ExecutionBlockerCase>,
    pub verdict: ExecutionReceiptVerdict,
    pub execution_check_root: String,
    pub blocker_case_root: String,
    pub execution_hold_root: String,
    pub execution_blocker_root: String,
    pub execution_receipt_binding_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&config, &source)?;
        let execution_checks = ExecutionReceiptLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| {
                ExecutionReceiptCheck::devnet(&config, &source, *lane, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let blocker_cases = ExecutionBlockerKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                ExecutionBlockerCase::devnet(&config, &source, *kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict =
            ExecutionReceiptVerdict::new(&config, &source, &execution_checks, &blocker_cases);
        let execution_check_root = execution_check_vector_root(&execution_checks);
        let blocker_case_root = blocker_case_vector_root(&blocker_cases);
        let execution_hold_root = aggregate_execution_hold_root(
            &config,
            &source,
            &execution_check_root,
            &blocker_case_root,
            &verdict,
        );
        let execution_blocker_root = aggregate_execution_blocker_root(
            &config,
            &source,
            &execution_checks,
            &blocker_cases,
            &verdict,
        );
        let execution_receipt_binding_root = execution_receipt_binding_root(
            &config,
            &source,
            &execution_check_root,
            &blocker_case_root,
            &execution_hold_root,
            &execution_blocker_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            execution_checks,
            blocker_cases,
            verdict,
            execution_check_root,
            blocker_case_root,
            execution_hold_root,
            execution_blocker_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_security_review_final_release_acceptance_execution_receipt_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "execution_check_root": self.execution_check_root,
            "blocker_case_root": self.blocker_case_root,
            "execution_hold_root": self.execution_hold_root,
            "execution_blocker_root": self.execution_blocker_root,
            "execution_receipt_binding_root": self.execution_receipt_binding_root,
            "verdict": self.verdict.public_record(),
            "execution_checks": self
                .execution_checks
                .iter()
                .map(ExecutionReceiptCheck::public_record)
                .collect::<Vec<_>>(),
            "blocker_cases": self
                .blocker_cases
                .iter()
                .map(ExecutionBlockerCase::public_record)
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
                "execution_check_root": self.execution_check_root,
                "blocker_case_root": self.blocker_case_root,
                "execution_hold_root": self.execution_hold_root,
                "execution_blocker_root": self.execution_blocker_root,
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
        return Err("final acceptance execution receipt chain id mismatch".to_string());
    }
    if config.min_execution_lanes < ExecutionReceiptLane::ordered().len() as u64 {
        return Err("final acceptance execution receipt requires every lane".to_string());
    }
    if config.min_blocker_cases < ExecutionBlockerKind::ordered().len() as u64 {
        return Err("final acceptance execution receipt requires every blocker case".to_string());
    }
    if config.require_acceptance_binding != 1 {
        return Err("final acceptance execution receipt requires acceptance binding".to_string());
    }
    if config.require_heavy_gate_execution_receipt != 1 {
        return Err("final acceptance execution receipt requires heavy-gate execution".to_string());
    }
    if config.require_heavy_gate_readiness_receipt != 1 {
        return Err("final acceptance execution receipt requires readiness receipt".to_string());
    }
    if config.require_wallet_force_exit_replay != 1 {
        return Err("final acceptance execution receipt requires wallet replay".to_string());
    }
    if config.require_operator_independence != 1 {
        return Err(
            "final acceptance execution receipt requires operator independence".to_string(),
        );
    }
    if config.require_pq_release_authority != 1 {
        return Err("final acceptance execution receipt requires pq authority".to_string());
    }
    if config.require_roots_only_privacy != 1 {
        return Err("final acceptance execution receipt requires roots only privacy".to_string());
    }
    if config.require_cargo_runtime_execution != 1 {
        return Err(
            "final acceptance execution receipt requires cargo runtime execution".to_string(),
        );
    }
    if config.require_security_audit_signature != 1 {
        return Err(
            "final acceptance execution receipt requires security audit signature".to_string(),
        );
    }
    if config.release_permit_count != 0
        || config.final_acceptance_executed != 0
        || config.production_release_allowed != 0
    {
        return Err("final acceptance execution receipt must remain release-held".to_string());
    }
    if config.max_public_metadata_units != 0 {
        return Err("final acceptance execution receipt allows public metadata".to_string());
    }
    Ok(())
}

fn validate_source(config: &Config, source: &SourceBundle) -> Result<()> {
    if source.acceptance_final_root.is_empty() {
        return Err("final acceptance execution receipt missing acceptance root".to_string());
    }
    if source.execution_receipt_root.is_empty() {
        return Err("final acceptance execution receipt missing heavy execution root".to_string());
    }
    if source.readiness_receipt_root.is_empty() {
        return Err("final acceptance execution receipt missing readiness root".to_string());
    }
    if source.acceptance_permitted_count != config.release_permit_count {
        return Err("final acceptance execution receipt saw acceptance permit".to_string());
    }
    if source.final_acceptance_accepted_count != config.final_acceptance_executed {
        return Err(
            "final acceptance execution receipt saw final acceptance execution".to_string(),
        );
    }
    if source.acceptance_production_release_allowed != config.production_release_allowed {
        return Err("final acceptance execution receipt saw production release flag".to_string());
    }
    if source.execution_can_wallet_force_exit != config.require_wallet_force_exit_replay {
        return Err(
            "final acceptance execution receipt missing wallet force-exit replay".to_string(),
        );
    }
    if source.execution_operator_independent_count < source.execution_force_exit_critical_count {
        return Err(
            "final acceptance execution receipt requires operator-independent forced exit"
                .to_string(),
        );
    }
    if source.acceptance_zero_linkage_count < source.acceptance_lane_count {
        return Err(
            "final acceptance execution receipt missing zero-linkage acceptance evidence"
                .to_string(),
        );
    }
    Ok(())
}

fn lane_source_root(source: &SourceBundle, lane: ExecutionReceiptLane) -> String {
    match lane {
        ExecutionReceiptLane::AcceptanceBindingRoot => source.acceptance_final_root.clone(),
        ExecutionReceiptLane::HeavyGateExecutionReceipt => source.execution_receipt_root.clone(),
        ExecutionReceiptLane::HeavyGateReadinessReceipt => source.readiness_receipt_root.clone(),
        ExecutionReceiptLane::WalletReplayability => source.execution_wallet_recovery_root.clone(),
        ExecutionReceiptLane::ForcedExitCriticalCoverage => source.execution_step_root.clone(),
        ExecutionReceiptLane::OperatorIndependence => source.execution_transcript_root.clone(),
        ExecutionReceiptLane::PqReleaseAuthority => source.pq_authority_root.clone(),
        ExecutionReceiptLane::PrivacyBoundary => source.execution_committed_root.clone(),
        ExecutionReceiptLane::CargoRuntimeExecution => {
            source.readiness_deferred_execution_root.clone()
        }
        ExecutionReceiptLane::FinalExecutionSeal => source.acceptance_hold_root.clone(),
    }
}

fn lane_observed(source: &SourceBundle, lane: ExecutionReceiptLane) -> String {
    match lane {
        ExecutionReceiptLane::AcceptanceBindingRoot => format!(
            "status={} lanes={} release_holds={}",
            source.acceptance_status,
            source.acceptance_lane_count,
            source.acceptance_release_hold_count
        ),
        ExecutionReceiptLane::HeavyGateExecutionReceipt => format!(
            "receipt={} deferred={} blocked={} rejected={}",
            source.execution_receipt_root,
            source.execution_deferred_count,
            source.execution_blocked_count,
            source.execution_rejected_count
        ),
        ExecutionReceiptLane::HeavyGateReadinessReceipt => format!(
            "scheduled={} executed={} blockers={}",
            source.readiness_heavy_gates_may_be_scheduled,
            source.readiness_heavy_gates_executed,
            source.readiness_production_blockers
        ),
        ExecutionReceiptLane::WalletReplayability => format!(
            "wallet_force_exit={} wallet_safe_steps={}",
            source.execution_can_wallet_force_exit, source.execution_wallet_safe_count
        ),
        ExecutionReceiptLane::ForcedExitCriticalCoverage => format!(
            "critical_steps={} operator_independent_steps={}",
            source.execution_force_exit_critical_count, source.execution_operator_independent_count
        ),
        ExecutionReceiptLane::OperatorIndependence => format!(
            "operator_independent={} production_blocked={}",
            source.execution_operator_independent_count, source.execution_production_blocked
        ),
        ExecutionReceiptLane::PqReleaseAuthority => format!(
            "pq_root={} custody_root={} crosscheck_root={}",
            source.pq_authority_root,
            source.custody_authority_root,
            source.authority_crosscheck_root
        ),
        ExecutionReceiptLane::PrivacyBoundary => format!(
            "zero_linkage={} encrypted_root={} wallet_root={}",
            source.acceptance_zero_linkage_count,
            source.execution_encrypted_root,
            source.execution_wallet_recovery_root
        ),
        ExecutionReceiptLane::CargoRuntimeExecution => format!(
            "cargo_deferred={} readiness_deferred={} heavy_executed={}",
            source.acceptance_cargo_deferred_count,
            source.readiness_deferred_lanes,
            source.readiness_heavy_gates_executed
        ),
        ExecutionReceiptLane::FinalExecutionSeal => format!(
            "release_permits={} final_acceptance={} production_allowed={}",
            source.acceptance_permitted_count,
            source.final_acceptance_accepted_count,
            source.acceptance_production_release_allowed
        ),
    }
}

fn lane_evidence_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    lane_source_root: &str,
    observed: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:lane-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(lane.as_str()),
            HashPart::Str(lane_source_root),
            HashPart::Str(observed),
            HashPart::Str(&source.source_root),
        ],
        32,
    )
}

fn lane_blocker_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:lane-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_policy),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.acceptance_blocker_root),
            HashPart::Str(&source.execution_blocker_root),
            HashPart::Str(&source.readiness_production_blocker_root),
            HashPart::U64(config.release_permit_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn lane_closure_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    blocker_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:lane-closure"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker_root),
            HashPart::Str(&source.execution_wallet_recovery_root),
            HashPart::Str(&source.pq_authority_root),
            HashPart::Str(&source.forced_exit_dry_run_root),
            HashPart::U64(config.final_acceptance_executed),
        ],
        32,
    )
}

fn execution_check_root(
    config: &Config,
    source: &SourceBundle,
    lane: ExecutionReceiptLane,
    ordinal: u64,
    lane_source_root: &str,
    evidence_root: &str,
    blocker_root: &str,
    closure_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:execution-check"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::U64(ordinal),
            HashPart::Str(lane.as_str()),
            HashPart::Str(lane.owner_lane()),
            HashPart::Str(lane_source_root),
            HashPart::Str(evidence_root),
            HashPart::Str(blocker_root),
            HashPart::Str(closure_root),
            HashPart::Str(&source.source_root),
        ],
        32,
    )
}

fn blocker_source_root(source: &SourceBundle, kind: ExecutionBlockerKind) -> String {
    match kind {
        ExecutionBlockerKind::AcceptanceBindingHeld => source.acceptance_hold_root.clone(),
        ExecutionBlockerKind::HeavyGateExecutionDeferred => {
            source.readiness_deferred_execution_root.clone()
        }
        ExecutionBlockerKind::CargoRuntimeDeferred => source.execution_blocker_root.clone(),
        ExecutionBlockerKind::SecurityAuditDeferred => {
            source.readiness_production_blocker_root.clone()
        }
        ExecutionBlockerKind::WalletReplayGap => source.execution_wallet_recovery_root.clone(),
        ExecutionBlockerKind::PqAuthorityGap => source.pq_authority_root.clone(),
        ExecutionBlockerKind::PrivacyBoundaryGap => source.execution_encrypted_root.clone(),
        ExecutionBlockerKind::ProductionReleaseAttempted => source.acceptance_verdict_root.clone(),
    }
}

fn blocker_case_blocker_root(
    config: &Config,
    source: &SourceBundle,
    kind: ExecutionBlockerKind,
    source_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker-case-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(kind.clearance()),
            HashPart::Str(&source.acceptance_blocker_root),
            HashPart::Str(&source.execution_blocker_root),
            HashPart::U64(config.release_permit_count),
        ],
        32,
    )
}

fn blocker_recovery_root(
    config: &Config,
    source: &SourceBundle,
    kind: ExecutionBlockerKind,
    blocker_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker-recovery"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(blocker_root),
            HashPart::Str(&source.forced_exit_dry_run_root),
            HashPart::Str(&source.wallet_handoff_root),
            HashPart::Str(&source.authority_crosscheck_root),
            HashPart::U64(config.require_wallet_force_exit_replay),
            HashPart::U64(config.require_pq_release_authority),
        ],
        32,
    )
}

fn blocker_case_root(
    config: &Config,
    source: &SourceBundle,
    kind: ExecutionBlockerKind,
    ordinal: u64,
    source_root: &str,
    blocker_root: &str,
    recovery_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker-case"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(blocker_root),
            HashPart::Str(recovery_root),
            HashPart::Str(&source.source_root),
        ],
        32,
    )
}

fn execution_check_vector_root(checks: &[ExecutionReceiptCheck]) -> String {
    let leaves = checks
        .iter()
        .map(ExecutionReceiptCheck::public_record)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:execution-checks"), &leaves)
}

fn blocker_case_vector_root(cases: &[ExecutionBlockerCase]) -> String {
    let leaves = cases
        .iter()
        .map(ExecutionBlockerCase::public_record)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blocker-cases"), &leaves)
}

fn aggregate_execution_hold_root(
    config: &Config,
    source: &SourceBundle,
    execution_check_root: &str,
    blocker_case_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:aggregate-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_policy),
            HashPart::Str(&source.acceptance_hold_root),
            HashPart::Str(&source.readiness_production_blocker_root),
            HashPart::Str(execution_check_root),
            HashPart::Str(blocker_case_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.release_hold_count),
            HashPart::U64(verdict.blocker_hold_count),
            HashPart::U64(config.final_acceptance_executed),
        ],
        32,
    )
}

fn aggregate_execution_blocker_root(
    config: &Config,
    source: &SourceBundle,
    checks: &[ExecutionReceiptCheck],
    blockers: &[ExecutionBlockerCase],
    verdict: &ExecutionReceiptVerdict,
) -> String {
    let check_blocker_root = merkle_root(
        &format!("{DOMAIN}:check-blockers"),
        &checks
            .iter()
            .map(|check| {
                json!({
                    "lane": check.lane.as_str(),
                    "blocker_root": check.blocker_root,
                    "release_permitted": check.release_permitted,
                })
            })
            .collect::<Vec<_>>(),
    );
    let blocker_root = merkle_root(
        &format!("{DOMAIN}:case-blockers"),
        &blockers
            .iter()
            .map(|case| {
                json!({
                    "kind": case.kind.as_str(),
                    "blocker_root": case.blocker_root,
                    "blocks_release": case.blocks_release,
                })
            })
            .collect::<Vec<_>>(),
    );
    domain_hash(
        &format!("{DOMAIN}:aggregate-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_policy),
            HashPart::Str(&source.acceptance_blocker_root),
            HashPart::Str(&source.execution_blocker_root),
            HashPart::Str(&source.readiness_deferred_execution_root),
            HashPart::Str(&check_blocker_root),
            HashPart::Str(&blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.release_permitted_count),
            HashPart::U64(verdict.production_blocker_count),
        ],
        32,
    )
}

fn execution_receipt_binding_root(
    config: &Config,
    source: &SourceBundle,
    execution_check_root: &str,
    blocker_case_root: &str,
    execution_hold_root: &str,
    execution_blocker_root: &str,
    verdict: &ExecutionReceiptVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:binding-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.execution_receipt_suite),
            HashPart::Str(&source.acceptance_state_root),
            HashPart::Str(&source.execution_receipt_state_root),
            HashPart::Str(&source.readiness_state_root),
            HashPart::Str(execution_check_root),
            HashPart::Str(blocker_case_root),
            HashPart::Str(execution_hold_root),
            HashPart::Str(execution_blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(config.release_permit_count),
            HashPart::U64(config.final_acceptance_executed),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn source_bundle_root(
    acceptance: &acceptance::State,
    execution: &heavy_execution::State,
    readiness: &readiness::State,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:source-bundle"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&acceptance.final_acceptance_root),
            HashPart::Str(&acceptance.acceptance_hold_root),
            HashPart::Str(&acceptance.acceptance_blocker_root),
            HashPart::Str(&acceptance.verdict.verdict_root),
            HashPart::Str(&execution.receipt.receipt_root),
            HashPart::Str(&execution.receipt.transcript_root),
            HashPart::Str(&execution.receipt.wallet_recovery_root),
            HashPart::Str(&readiness.receipt.roots.receipt_root),
            HashPart::Str(&readiness.receipt.roots.deferred_execution_root),
            HashPart::U64(acceptance.verdict.acceptance_permitted_count),
            HashPart::U64(bool_to_u64(execution.can_wallet_force_exit())),
            HashPart::U64(bool_to_u64(readiness.receipt.production_release_blocked)),
        ],
        32,
    )
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
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let reason_ref = reason.as_str();
    let source = SourceBundle {
        acceptance_state_root: record_root(
            "fallback-acceptance-state",
            &json!({ "reason": reason_ref }),
        ),
        acceptance_final_root: record_root(
            "fallback-acceptance-final",
            &json!({ "reason": reason_ref }),
        ),
        acceptance_hold_root: record_root(
            "fallback-acceptance-hold",
            &json!({ "reason": reason_ref }),
        ),
        acceptance_blocker_root: record_root(
            "fallback-acceptance-blocker",
            &json!({ "reason": reason_ref }),
        ),
        acceptance_verdict_root: record_root(
            "fallback-acceptance-verdict",
            &json!({ "reason": reason_ref }),
        ),
        acceptance_status: "final_acceptance_execution_receipt_fallback_release_held".to_string(),
        acceptance_lane_count: 0,
        acceptance_rejection_count: 0,
        acceptance_release_hold_count: 1,
        acceptance_permitted_count: 0,
        final_acceptance_accepted_count: 0,
        acceptance_production_release_allowed: 0,
        acceptance_cargo_deferred_count: 1,
        acceptance_zero_linkage_count: 0,
        execution_receipt_state_root: record_root(
            "fallback-execution-state",
            &json!({ "reason": reason_ref }),
        ),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({ "reason": reason_ref }),
        ),
        execution_transcript_root: record_root(
            "fallback-execution-transcript",
            &json!({ "reason": reason_ref }),
        ),
        execution_step_root: record_root(
            "fallback-execution-step",
            &json!({ "reason": reason_ref }),
        ),
        execution_blocker_root: record_root(
            "fallback-execution-blocker",
            &json!({ "reason": reason_ref }),
        ),
        execution_public_root: record_root(
            "fallback-execution-public",
            &json!({ "reason": reason_ref }),
        ),
        execution_committed_root: record_root(
            "fallback-execution-committed",
            &json!({ "reason": reason_ref }),
        ),
        execution_encrypted_root: record_root(
            "fallback-execution-encrypted",
            &json!({ "reason": reason_ref }),
        ),
        execution_wallet_recovery_root: record_root(
            "fallback-execution-wallet",
            &json!({ "reason": reason_ref }),
        ),
        execution_wallet_safe_count: 0,
        execution_operator_independent_count: 0,
        execution_production_safe_count: 0,
        execution_force_exit_critical_count: 1,
        execution_deferred_count: 1,
        execution_watch_count: 0,
        execution_blocked_count: 1,
        execution_rejected_count: 0,
        execution_can_wallet_force_exit: 0,
        execution_production_blocked: 1,
        readiness_state_root: record_root(
            "fallback-readiness-state",
            &json!({ "reason": reason_ref }),
        ),
        readiness_receipt_root: record_root(
            "fallback-readiness-receipt",
            &json!({ "reason": reason_ref }),
        ),
        readiness_schedule_root: record_root(
            "fallback-readiness-schedule",
            &json!({ "reason": reason_ref }),
        ),
        readiness_production_blocker_root: record_root(
            "fallback-readiness-production-blocker",
            &json!({ "reason": reason_ref }),
        ),
        readiness_deferred_execution_root: record_root(
            "fallback-readiness-deferred",
            &json!({ "reason": reason_ref }),
        ),
        readiness_heavy_gates_may_be_scheduled: 0,
        readiness_heavy_gates_executed: 0,
        readiness_production_release_blocked: 1,
        readiness_schedule_ready_lanes: 0,
        readiness_deferred_lanes: 1,
        readiness_blocked_lanes: 1,
        readiness_production_blockers: 1,
        pq_authority_root: record_root("fallback-pq-authority", &json!({ "reason": reason_ref })),
        custody_authority_root: record_root(
            "fallback-custody-authority",
            &json!({ "reason": reason_ref }),
        ),
        authority_crosscheck_root: record_root(
            "fallback-authority-crosscheck",
            &json!({ "reason": reason_ref }),
        ),
        forced_exit_dry_run_root: record_root(
            "fallback-forced-exit",
            &json!({ "reason": reason_ref }),
        ),
        wallet_handoff_root: record_root(
            "fallback-wallet-handoff",
            &json!({ "reason": reason_ref }),
        ),
        source_root: record_root("fallback-source", &json!({ "reason": reason_ref })),
    };
    let execution_checks = Vec::new();
    let blocker_cases = Vec::new();
    let verdict = ExecutionReceiptVerdict::new(&config, &source, &execution_checks, &blocker_cases);
    let execution_check_root = execution_check_vector_root(&execution_checks);
    let blocker_case_root = blocker_case_vector_root(&blocker_cases);
    let execution_hold_root = aggregate_execution_hold_root(
        &config,
        &source,
        &execution_check_root,
        &blocker_case_root,
        &verdict,
    );
    let execution_blocker_root = aggregate_execution_blocker_root(
        &config,
        &source,
        &execution_checks,
        &blocker_cases,
        &verdict,
    );
    let execution_receipt_binding_root = execution_receipt_binding_root(
        &config,
        &source,
        &execution_check_root,
        &blocker_case_root,
        &execution_hold_root,
        &execution_blocker_root,
        &verdict,
    );
    State {
        config,
        source,
        execution_checks,
        blocker_cases,
        verdict,
        execution_check_root,
        blocker_case_root,
        execution_hold_root,
        execution_blocker_root,
        execution_receipt_binding_root,
    }
}
