use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_security_review_release_governance_execution_binding_runtime as execution_binding,
    monero_l2_pq_bridge_exit_execution_release_gate_plan_runtime as release_plan,
    monero_l2_pq_bridge_exit_final_release_gate_runtime as final_gate, CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeSecurityReviewFinalReleaseAcceptanceBindingRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_FINAL_RELEASE_ACCEPTANCE_BINDING_RUNTIME_PROTOCOL_VERSION:
    &str = "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-security-review-final-release-acceptance-binding-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_FINAL_RELEASE_ACCEPTANCE_BINDING_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-security-review-final-release-acceptance-binding";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub acceptance_suite: String,
    pub acceptance_policy: String,
    pub min_acceptance_lanes: u64,
    pub min_rejection_cases: u64,
    pub require_execution_binding: u64,
    pub require_final_release_gate: u64,
    pub require_execution_release_plan: u64,
    pub require_reviewer_receipts: u64,
    pub require_pq_custody_authority: u64,
    pub require_wallet_escape_priority: u64,
    pub require_roots_only_outputs: u64,
    pub require_zero_linkage_exports: u64,
    pub require_runtime_cargo_evidence: u64,
    pub acceptance_permit_count: u64,
    pub final_release_accepted: u64,
    pub production_release_allowed: u64,
    pub max_linkage_exports: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            acceptance_suite:
                "monero-l2-pq-bridge-exit-canonical-user-escape-final-release-acceptance-binding-v1"
                    .to_string(),
            acceptance_policy: "execution-binding-final-gate-release-held-acceptance-v1"
                .to_string(),
            min_acceptance_lanes: 10,
            min_rejection_cases: 7,
            require_execution_binding: 1,
            require_final_release_gate: 1,
            require_execution_release_plan: 1,
            require_reviewer_receipts: 1,
            require_pq_custody_authority: 1,
            require_wallet_escape_priority: 1,
            require_roots_only_outputs: 1,
            require_zero_linkage_exports: 1,
            require_runtime_cargo_evidence: 1,
            acceptance_permit_count: 0,
            final_release_accepted: 0,
            production_release_allowed: 0,
            max_linkage_exports: 0,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "acceptance_suite": self.acceptance_suite,
            "acceptance_policy": self.acceptance_policy,
            "min_acceptance_lanes": self.min_acceptance_lanes,
            "min_rejection_cases": self.min_rejection_cases,
            "require_execution_binding": self.require_execution_binding,
            "require_final_release_gate": self.require_final_release_gate,
            "require_execution_release_plan": self.require_execution_release_plan,
            "require_reviewer_receipts": self.require_reviewer_receipts,
            "require_pq_custody_authority": self.require_pq_custody_authority,
            "require_wallet_escape_priority": self.require_wallet_escape_priority,
            "require_roots_only_outputs": self.require_roots_only_outputs,
            "require_zero_linkage_exports": self.require_zero_linkage_exports,
            "require_runtime_cargo_evidence": self.require_runtime_cargo_evidence,
            "acceptance_permit_count": self.acceptance_permit_count,
            "final_release_accepted": self.final_release_accepted,
            "production_release_allowed": self.production_release_allowed,
            "max_linkage_exports": self.max_linkage_exports,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptanceLane {
    ExecutionBindingVerdict,
    ExecutionHoldRoot,
    FinalReleaseGateReport,
    ExecutionReleasePlan,
    ReviewerReceiptClosure,
    PqCustodyAuthority,
    WalletEscapePriority,
    PrivacyBoundary,
    CargoRuntimeEvidence,
    AcceptanceDecisionSeal,
}

impl AcceptanceLane {
    pub fn ordered() -> [Self; 10] {
        [
            Self::ExecutionBindingVerdict,
            Self::ExecutionHoldRoot,
            Self::FinalReleaseGateReport,
            Self::ExecutionReleasePlan,
            Self::ReviewerReceiptClosure,
            Self::PqCustodyAuthority,
            Self::WalletEscapePriority,
            Self::PrivacyBoundary,
            Self::CargoRuntimeEvidence,
            Self::AcceptanceDecisionSeal,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionBindingVerdict => "execution_binding_verdict",
            Self::ExecutionHoldRoot => "execution_hold_root",
            Self::FinalReleaseGateReport => "final_release_gate_report",
            Self::ExecutionReleasePlan => "execution_release_plan",
            Self::ReviewerReceiptClosure => "reviewer_receipt_closure",
            Self::PqCustodyAuthority => "pq_custody_authority",
            Self::WalletEscapePriority => "wallet_escape_priority",
            Self::PrivacyBoundary => "privacy_boundary",
            Self::CargoRuntimeEvidence => "cargo_runtime_evidence",
            Self::AcceptanceDecisionSeal => "acceptance_decision_seal",
        }
    }

    pub fn owner_lane(self) -> &'static str {
        match self {
            Self::ExecutionBindingVerdict => "release_execution_binding_owner",
            Self::ExecutionHoldRoot => "release_hold_owner",
            Self::FinalReleaseGateReport => "final_release_gate_owner",
            Self::ExecutionReleasePlan => "execution_release_plan_owner",
            Self::ReviewerReceiptClosure => "reviewer_receipt_owner",
            Self::PqCustodyAuthority => "pq_custody_authority_owner",
            Self::WalletEscapePriority => "wallet_escape_owner",
            Self::PrivacyBoundary => "privacy_boundary_owner",
            Self::CargoRuntimeEvidence => "cargo_runtime_evidence_owner",
            Self::AcceptanceDecisionSeal => "acceptance_decision_owner",
        }
    }

    pub fn question(self) -> &'static str {
        match self {
            Self::ExecutionBindingVerdict => {
                "Does final acceptance consume the release governance execution binding verdict?"
            }
            Self::ExecutionHoldRoot => {
                "Does final acceptance preserve the execution hold and blocker roots?"
            }
            Self::FinalReleaseGateReport => {
                "Does final acceptance bind the final release gate state and report roots?"
            }
            Self::ExecutionReleasePlan => {
                "Does final acceptance bind the execution release plan and blocker roots?"
            }
            Self::ReviewerReceiptClosure => {
                "Does final acceptance require reviewer receipt replay before any release?"
            }
            Self::PqCustodyAuthority => {
                "Does final acceptance require PQ authority, custody policy, and crosscheck roots?"
            }
            Self::WalletEscapePriority => {
                "Does final acceptance keep user escape and wallet handoff ahead of release?"
            }
            Self::PrivacyBoundary => {
                "Does final acceptance keep outputs roots-only with zero linkage exports?"
            }
            Self::CargoRuntimeEvidence => {
                "Does final acceptance require deferred cargo and runtime evidence?"
            }
            Self::AcceptanceDecisionSeal => {
                "Does final acceptance seal the release as held until every lane is live-proven?"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectionCaseKind {
    ExecutionBindingMissing,
    FinalGateWatchOnly,
    ReleasePlanCargoDeferred,
    ReviewerReceiptGap,
    PqCustodyGap,
    WalletEscapeGap,
    MetadataLeakProbe,
}

impl RejectionCaseKind {
    pub fn ordered() -> [Self; 7] {
        [
            Self::ExecutionBindingMissing,
            Self::FinalGateWatchOnly,
            Self::ReleasePlanCargoDeferred,
            Self::ReviewerReceiptGap,
            Self::PqCustodyGap,
            Self::WalletEscapeGap,
            Self::MetadataLeakProbe,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionBindingMissing => "execution_binding_missing",
            Self::FinalGateWatchOnly => "final_gate_watch_only",
            Self::ReleasePlanCargoDeferred => "release_plan_cargo_deferred",
            Self::ReviewerReceiptGap => "reviewer_receipt_gap",
            Self::PqCustodyGap => "pq_custody_gap",
            Self::WalletEscapeGap => "wallet_escape_gap",
            Self::MetadataLeakProbe => "metadata_leak_probe",
        }
    }

    pub fn expected_response(self) -> &'static str {
        match self {
            Self::ExecutionBindingMissing => "reject_acceptance_and_rebuild_execution_binding",
            Self::FinalGateWatchOnly => "reject_acceptance_and_keep_final_gate_held",
            Self::ReleasePlanCargoDeferred => "reject_acceptance_and_run_deferred_runtime_gates",
            Self::ReviewerReceiptGap => "reject_acceptance_and_replay_reviewer_receipts",
            Self::PqCustodyGap => "reject_acceptance_and_recheck_pq_custody_authority",
            Self::WalletEscapeGap => "reject_acceptance_and_prioritize_wallet_escape",
            Self::MetadataLeakProbe => "reject_acceptance_and_publish_roots_only",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBundle {
    pub execution_binding_state_root: String,
    pub execution_binding_root: String,
    pub execution_hold_root: String,
    pub execution_blocker_root: String,
    pub execution_verdict_root: String,
    pub execution_permitted_count: u64,
    pub execution_release_hold_count: u64,
    pub execution_guard_block_count: u64,
    pub execution_zero_linkage_count: u64,
    pub execution_production_release_allowed: u64,
    pub final_release_gate_state_root: String,
    pub final_release_gate_report_root: String,
    pub final_release_gate_blocker_root: String,
    pub final_release_gate_status_root: String,
    pub final_gate_decisions_allowed: u64,
    pub final_gate_decisions_blocked: u64,
    pub final_gate_deferred_gates: u64,
    pub final_gate_production_blockers: u64,
    pub execution_release_plan_state_root: String,
    pub execution_release_plan_root: String,
    pub execution_release_plan_gate_root: String,
    pub execution_release_plan_blocker_root: String,
    pub execution_release_plan_production_allowed: u64,
    pub execution_release_plan_cargo_deferred: u64,
    pub reviewer_receipt_gate_state_root: String,
    pub pq_authority_verification_state_root: String,
    pub custody_release_authority_spec_state_root: String,
    pub authority_crosscheck_state_root: String,
    pub forced_exit_dry_run_state_root: String,
    pub wallet_handoff_state_root: String,
    pub source_root: String,
}

impl SourceBundle {
    pub fn devnet() -> Self {
        let execution = execution_binding::devnet();
        let final_release = final_gate::devnet();
        let plan = release_plan::devnet();
        Self::from_states(&execution, &final_release, &plan)
    }

    pub fn from_states(
        execution: &execution_binding::State,
        final_release: &final_gate::State,
        plan: &release_plan::State,
    ) -> Self {
        let final_report = final_release.latest_report.as_ref();
        let final_release_gate_report_root = match final_report {
            Some(report) => report.roots.report_root.clone(),
            None => record_root("missing-final-release-report", &json!({"present": 0_u64})),
        };
        let final_release_gate_blocker_root = match final_report {
            Some(report) => report.roots.blocker_root.clone(),
            None => record_root("missing-final-release-blockers", &json!({"present": 0_u64})),
        };
        let final_release_gate_status_root = match final_report {
            Some(report) => record_root(
                "final-release-status",
                &json!({
                    "status": report.status.as_str(),
                    "readiness_label": &report.readiness_label,
                    "user_answer": &report.user_answer,
                }),
            ),
            None => record_root("missing-final-release-status", &json!({"present": 0_u64})),
        };
        let final_gate_decisions_allowed = match final_report {
            Some(report) => report.decisions_allowed,
            None => 0,
        };
        let final_gate_decisions_blocked = match final_report {
            Some(report) => report.decisions_blocked,
            None => 1,
        };
        let final_gate_deferred_gates = match final_report {
            Some(report) => report.deferred_gates,
            None => 1,
        };
        let final_gate_production_blockers = match final_report {
            Some(report) => report.production_blockers,
            None => 1,
        };
        let source_root = source_bundle_root(execution, final_release, plan);

        Self {
            execution_binding_state_root: execution.state_root(),
            execution_binding_root: execution.execution_binding_root.clone(),
            execution_hold_root: execution.execution_hold_root.clone(),
            execution_blocker_root: execution.execution_blocker_root.clone(),
            execution_verdict_root: execution.verdict.verdict_root.clone(),
            execution_permitted_count: execution.verdict.execution_permitted_count,
            execution_release_hold_count: execution.verdict.release_hold_count,
            execution_guard_block_count: execution.verdict.guard_block_count,
            execution_zero_linkage_count: execution.verdict.zero_linkage_count,
            execution_production_release_allowed: execution.verdict.production_release_allowed,
            final_release_gate_state_root: final_release.state_root(),
            final_release_gate_report_root,
            final_release_gate_blocker_root,
            final_release_gate_status_root,
            final_gate_decisions_allowed,
            final_gate_decisions_blocked,
            final_gate_deferred_gates,
            final_gate_production_blockers,
            execution_release_plan_state_root: plan.state_root(),
            execution_release_plan_root: plan.latest_plan.plan_root.clone(),
            execution_release_plan_gate_root: plan.latest_plan.gate_root.clone(),
            execution_release_plan_blocker_root: plan.latest_plan.blocker_root.clone(),
            execution_release_plan_production_allowed: bool_to_u64(
                plan.latest_plan.production_release_allowed,
            ),
            execution_release_plan_cargo_deferred: bool_to_u64(
                plan.latest_plan.cargo_checks_deferred,
            ),
            reviewer_receipt_gate_state_root: execution
                .source
                .reviewer_receipt_gate_state_root
                .clone(),
            pq_authority_verification_state_root: execution
                .source
                .pq_authority_verification_state_root
                .clone(),
            custody_release_authority_spec_state_root: execution
                .source
                .custody_release_authority_spec_state_root
                .clone(),
            authority_crosscheck_state_root: execution
                .source
                .authority_crosscheck_state_root
                .clone(),
            forced_exit_dry_run_state_root: execution.source.forced_exit_dry_run_state_root.clone(),
            wallet_handoff_state_root: execution.source.wallet_handoff_state_root.clone(),
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_binding_state_root": self.execution_binding_state_root,
            "execution_binding_root": self.execution_binding_root,
            "execution_hold_root": self.execution_hold_root,
            "execution_blocker_root": self.execution_blocker_root,
            "execution_verdict_root": self.execution_verdict_root,
            "execution_permitted_count": self.execution_permitted_count,
            "execution_release_hold_count": self.execution_release_hold_count,
            "execution_guard_block_count": self.execution_guard_block_count,
            "execution_zero_linkage_count": self.execution_zero_linkage_count,
            "execution_production_release_allowed": self.execution_production_release_allowed,
            "final_release_gate_state_root": self.final_release_gate_state_root,
            "final_release_gate_report_root": self.final_release_gate_report_root,
            "final_release_gate_blocker_root": self.final_release_gate_blocker_root,
            "final_release_gate_status_root": self.final_release_gate_status_root,
            "final_gate_decisions_allowed": self.final_gate_decisions_allowed,
            "final_gate_decisions_blocked": self.final_gate_decisions_blocked,
            "final_gate_deferred_gates": self.final_gate_deferred_gates,
            "final_gate_production_blockers": self.final_gate_production_blockers,
            "execution_release_plan_state_root": self.execution_release_plan_state_root,
            "execution_release_plan_root": self.execution_release_plan_root,
            "execution_release_plan_gate_root": self.execution_release_plan_gate_root,
            "execution_release_plan_blocker_root": self.execution_release_plan_blocker_root,
            "execution_release_plan_production_allowed": self.execution_release_plan_production_allowed,
            "execution_release_plan_cargo_deferred": self.execution_release_plan_cargo_deferred,
            "reviewer_receipt_gate_state_root": self.reviewer_receipt_gate_state_root,
            "pq_authority_verification_state_root": self.pq_authority_verification_state_root,
            "custody_release_authority_spec_state_root": self.custody_release_authority_spec_state_root,
            "authority_crosscheck_state_root": self.authority_crosscheck_state_root,
            "forced_exit_dry_run_state_root": self.forced_exit_dry_run_state_root,
            "wallet_handoff_state_root": self.wallet_handoff_state_root,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceCheck {
    pub ordinal: u64,
    pub lane: AcceptanceLane,
    pub owner_lane: String,
    pub question: String,
    pub source_root: String,
    pub evidence_root: String,
    pub acceptance_blocker_root: String,
    pub release_hold_root: String,
    pub pq_authority_root: String,
    pub wallet_escape_root: String,
    pub privacy_boundary_root: String,
    pub decision_root: String,
    pub acceptance_permitted: u64,
    pub final_release_accepted: u64,
    pub linkage_exports_allowed: u64,
    pub check_root: String,
}

impl AcceptanceCheck {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        lane: AcceptanceLane,
        ordinal: u64,
    ) -> Self {
        let source_root = acceptance_lane_source_root(source, lane);
        let evidence_root = acceptance_evidence_root(config, source, lane, &source_root);
        let acceptance_blocker_root = acceptance_blocker_root(config, source, lane, &evidence_root);
        let release_hold_root =
            acceptance_release_hold_root(config, source, lane, &acceptance_blocker_root);
        let pq_authority_root = acceptance_pq_authority_root(config, source, lane, &evidence_root);
        let wallet_escape_root =
            acceptance_wallet_escape_root(config, source, lane, &evidence_root);
        let privacy_boundary_root =
            acceptance_privacy_boundary_root(config, source, lane, &evidence_root);
        let decision_root = acceptance_decision_root(
            config,
            source,
            lane,
            &acceptance_blocker_root,
            &release_hold_root,
            &pq_authority_root,
            &wallet_escape_root,
            &privacy_boundary_root,
        );
        let check_root = acceptance_check_root(
            config,
            source,
            lane,
            ordinal,
            &source_root,
            &evidence_root,
            &acceptance_blocker_root,
            &release_hold_root,
            &decision_root,
        );

        Self {
            ordinal,
            lane,
            owner_lane: lane.owner_lane().to_string(),
            question: lane.question().to_string(),
            source_root,
            evidence_root,
            acceptance_blocker_root,
            release_hold_root,
            pq_authority_root,
            wallet_escape_root,
            privacy_boundary_root,
            decision_root,
            acceptance_permitted: config.acceptance_permit_count,
            final_release_accepted: config.final_release_accepted,
            linkage_exports_allowed: config.max_linkage_exports,
            check_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "lane": self.lane.as_str(),
            "owner_lane": self.owner_lane,
            "question": self.question,
            "source_root": self.source_root,
            "evidence_root": self.evidence_root,
            "acceptance_blocker_root": self.acceptance_blocker_root,
            "release_hold_root": self.release_hold_root,
            "pq_authority_root": self.pq_authority_root,
            "wallet_escape_root": self.wallet_escape_root,
            "privacy_boundary_root": self.privacy_boundary_root,
            "decision_root": self.decision_root,
            "acceptance_permitted": self.acceptance_permitted,
            "final_release_accepted": self.final_release_accepted,
            "linkage_exports_allowed": self.linkage_exports_allowed,
            "check_root": self.check_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectionCase {
    pub ordinal: u64,
    pub kind: RejectionCaseKind,
    pub expected_response: String,
    pub trigger_root: String,
    pub source_root: String,
    pub blocker_root: String,
    pub recovery_root: String,
    pub acceptance_permitted: u64,
    pub rejection_case_root: String,
}

impl RejectionCase {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        kind: RejectionCaseKind,
        ordinal: u64,
    ) -> Self {
        let trigger_root = rejection_trigger_root(config, source, kind);
        let source_root = rejection_source_root(config, source, kind, &trigger_root);
        let blocker_root = rejection_blocker_root(config, source, kind, &source_root);
        let recovery_root = rejection_recovery_root(config, source, kind, &blocker_root);
        let rejection_case_root = rejection_case_root(
            config,
            source,
            kind,
            ordinal,
            &trigger_root,
            &source_root,
            &blocker_root,
            &recovery_root,
        );

        Self {
            ordinal,
            kind,
            expected_response: kind.expected_response().to_string(),
            trigger_root,
            source_root,
            blocker_root,
            recovery_root,
            acceptance_permitted: config.acceptance_permit_count,
            rejection_case_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "kind": self.kind.as_str(),
            "expected_response": self.expected_response,
            "trigger_root": self.trigger_root,
            "source_root": self.source_root,
            "blocker_root": self.blocker_root,
            "recovery_root": self.recovery_root,
            "acceptance_permitted": self.acceptance_permitted,
            "rejection_case_root": self.rejection_case_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceVerdict {
    pub acceptance_lane_count: u64,
    pub rejection_case_count: u64,
    pub acceptance_permitted_count: u64,
    pub final_release_accepted_count: u64,
    pub release_hold_count: u64,
    pub rejection_block_count: u64,
    pub zero_linkage_count: u64,
    pub execution_permitted_count: u64,
    pub final_gate_decisions_allowed: u64,
    pub plan_production_allowed: u64,
    pub cargo_deferred_count: u64,
    pub production_release_allowed: u64,
    pub acceptance_status: String,
    pub verdict_root: String,
}

impl AcceptanceVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        checks: &[AcceptanceCheck],
        rejections: &[RejectionCase],
    ) -> Self {
        let acceptance_lane_count = checks.len() as u64;
        let rejection_case_count = rejections.len() as u64;
        let acceptance_permitted_count = checks
            .iter()
            .filter(|check| check.acceptance_permitted == 1)
            .count() as u64;
        let final_release_accepted_count = checks
            .iter()
            .filter(|check| check.final_release_accepted == 1)
            .count() as u64;
        let release_hold_count = checks
            .iter()
            .filter(|check| check.acceptance_permitted == 0)
            .count() as u64;
        let rejection_block_count = rejections
            .iter()
            .filter(|case| case.acceptance_permitted == 0)
            .count() as u64;
        let zero_linkage_count = checks
            .iter()
            .filter(|check| check.linkage_exports_allowed <= config.max_linkage_exports)
            .count() as u64;
        let execution_permitted_count = source.execution_permitted_count;
        let final_gate_decisions_allowed = source.final_gate_decisions_allowed;
        let plan_production_allowed = source.execution_release_plan_production_allowed;
        let cargo_deferred_count = source.execution_release_plan_cargo_deferred;
        let production_release_allowed = config.production_release_allowed;
        let acceptance_status = if acceptance_lane_count >= config.min_acceptance_lanes
            && rejection_case_count >= config.min_rejection_cases
            && acceptance_permitted_count == config.acceptance_permit_count
            && final_release_accepted_count == config.final_release_accepted
            && release_hold_count == acceptance_lane_count
            && rejection_block_count == rejection_case_count
            && zero_linkage_count == acceptance_lane_count
            && execution_permitted_count == 0
            && plan_production_allowed == 0
            && production_release_allowed == 0
        {
            "final_release_acceptance_binding_ready_release_held"
        } else {
            "final_release_acceptance_binding_gap_release_held"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.acceptance_policy),
                HashPart::Str(&source.execution_binding_root),
                HashPart::Str(&source.final_release_gate_state_root),
                HashPart::Str(&source.execution_release_plan_root),
                HashPart::U64(acceptance_lane_count),
                HashPart::U64(rejection_case_count),
                HashPart::U64(acceptance_permitted_count),
                HashPart::U64(final_release_accepted_count),
                HashPart::U64(release_hold_count),
                HashPart::U64(rejection_block_count),
                HashPart::U64(zero_linkage_count),
                HashPart::U64(execution_permitted_count),
                HashPart::U64(final_gate_decisions_allowed),
                HashPart::U64(plan_production_allowed),
                HashPart::U64(cargo_deferred_count),
                HashPart::U64(production_release_allowed),
                HashPart::Str(&acceptance_status),
            ],
            32,
        );

        Self {
            acceptance_lane_count,
            rejection_case_count,
            acceptance_permitted_count,
            final_release_accepted_count,
            release_hold_count,
            rejection_block_count,
            zero_linkage_count,
            execution_permitted_count,
            final_gate_decisions_allowed,
            plan_production_allowed,
            cargo_deferred_count,
            production_release_allowed,
            acceptance_status,
            verdict_root,
        }
    }

    pub fn fallback(config: &Config, reason: &str) -> Self {
        let acceptance_status =
            "final_release_acceptance_binding_construction_gap_release_held".to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:fallback-verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.acceptance_policy),
                HashPart::Str(reason),
                HashPart::Str(&acceptance_status),
            ],
            32,
        );

        Self {
            acceptance_lane_count: 0,
            rejection_case_count: 0,
            acceptance_permitted_count: 0,
            final_release_accepted_count: 0,
            release_hold_count: 1,
            rejection_block_count: 1,
            zero_linkage_count: 0,
            execution_permitted_count: 0,
            final_gate_decisions_allowed: 0,
            plan_production_allowed: 0,
            cargo_deferred_count: 1,
            production_release_allowed: 0,
            acceptance_status,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "acceptance_lane_count": self.acceptance_lane_count,
            "rejection_case_count": self.rejection_case_count,
            "acceptance_permitted_count": self.acceptance_permitted_count,
            "final_release_accepted_count": self.final_release_accepted_count,
            "release_hold_count": self.release_hold_count,
            "rejection_block_count": self.rejection_block_count,
            "zero_linkage_count": self.zero_linkage_count,
            "execution_permitted_count": self.execution_permitted_count,
            "final_gate_decisions_allowed": self.final_gate_decisions_allowed,
            "plan_production_allowed": self.plan_production_allowed,
            "cargo_deferred_count": self.cargo_deferred_count,
            "production_release_allowed": self.production_release_allowed,
            "acceptance_status": self.acceptance_status,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub acceptance_checks: Vec<AcceptanceCheck>,
    pub rejection_cases: Vec<RejectionCase>,
    pub verdict: AcceptanceVerdict,
    pub acceptance_check_root: String,
    pub rejection_case_root: String,
    pub acceptance_hold_root: String,
    pub acceptance_blocker_root: String,
    pub final_acceptance_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&config, &source)?;
        let acceptance_checks = AcceptanceLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| AcceptanceCheck::devnet(&config, &source, *lane, index as u64 + 1))
            .collect::<Vec<_>>();
        let rejection_cases = RejectionCaseKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, kind)| RejectionCase::devnet(&config, &source, *kind, index as u64 + 1))
            .collect::<Vec<_>>();
        let verdict =
            AcceptanceVerdict::new(&config, &source, &acceptance_checks, &rejection_cases);
        let acceptance_check_root = acceptance_check_vector_root(&acceptance_checks);
        let rejection_case_root = rejection_case_vector_root(&rejection_cases);
        let acceptance_hold_root = aggregate_acceptance_hold_root(
            &config,
            &source,
            &acceptance_check_root,
            &rejection_case_root,
            &verdict,
        );
        let acceptance_blocker_root = aggregate_acceptance_blocker_root(
            &config,
            &source,
            &acceptance_checks,
            &rejection_cases,
            &verdict,
        );
        let final_acceptance_root = final_acceptance_root(
            &config,
            &source,
            &acceptance_check_root,
            &rejection_case_root,
            &acceptance_hold_root,
            &acceptance_blocker_root,
            &verdict,
        );

        Ok(Self {
            config,
            source,
            acceptance_checks,
            rejection_cases,
            verdict,
            acceptance_check_root,
            rejection_case_root,
            acceptance_hold_root,
            acceptance_blocker_root,
            final_acceptance_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_security_review_final_release_acceptance_binding_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "acceptance_check_root": self.acceptance_check_root,
            "rejection_case_root": self.rejection_case_root,
            "acceptance_hold_root": self.acceptance_hold_root,
            "acceptance_blocker_root": self.acceptance_blocker_root,
            "final_acceptance_root": self.final_acceptance_root,
            "verdict": self.verdict.public_record(),
            "acceptance_checks": self
                .acceptance_checks
                .iter()
                .map(AcceptanceCheck::public_record)
                .collect::<Vec<_>>(),
            "rejection_cases": self
                .rejection_cases
                .iter()
                .map(RejectionCase::public_record)
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
                "acceptance_check_root": self.acceptance_check_root,
                "rejection_case_root": self.rejection_case_root,
                "acceptance_hold_root": self.acceptance_hold_root,
                "acceptance_blocker_root": self.acceptance_blocker_root,
                "final_acceptance_root": self.final_acceptance_root,
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
        return Err("final acceptance config chain id mismatch".to_string());
    }
    if config.min_acceptance_lanes < AcceptanceLane::ordered().len() as u64 {
        return Err("final acceptance requires every lane".to_string());
    }
    if config.min_rejection_cases < RejectionCaseKind::ordered().len() as u64 {
        return Err("final acceptance requires every rejection case".to_string());
    }
    if config.require_execution_binding != 1 {
        return Err("final acceptance requires execution binding".to_string());
    }
    if config.require_final_release_gate != 1 {
        return Err("final acceptance requires final release gate".to_string());
    }
    if config.require_execution_release_plan != 1 {
        return Err("final acceptance requires execution release plan".to_string());
    }
    if config.require_reviewer_receipts != 1 {
        return Err("final acceptance requires reviewer receipts".to_string());
    }
    if config.require_pq_custody_authority != 1 {
        return Err("final acceptance requires pq custody authority".to_string());
    }
    if config.require_wallet_escape_priority != 1 {
        return Err("final acceptance requires wallet escape priority".to_string());
    }
    if config.require_roots_only_outputs != 1 {
        return Err("final acceptance requires roots only outputs".to_string());
    }
    if config.require_zero_linkage_exports != 1 {
        return Err("final acceptance requires zero linkage exports".to_string());
    }
    if config.require_runtime_cargo_evidence != 1 {
        return Err("final acceptance requires runtime cargo evidence".to_string());
    }
    if config.acceptance_permit_count != 0 {
        return Err("final acceptance cannot permit release by default".to_string());
    }
    if config.final_release_accepted != 0 {
        return Err("final acceptance must remain denied".to_string());
    }
    if config.production_release_allowed != 0 {
        return Err("final acceptance production release must be held".to_string());
    }
    if config.max_linkage_exports != 0 {
        return Err("final acceptance must export zero linkage material".to_string());
    }
    Ok(())
}

fn validate_source(config: &Config, source: &SourceBundle) -> Result<()> {
    if source.execution_binding_root.is_empty() {
        return Err("final acceptance missing execution binding root".to_string());
    }
    if source.final_release_gate_state_root.is_empty() {
        return Err("final acceptance missing final gate root".to_string());
    }
    if source.execution_release_plan_root.is_empty() {
        return Err("final acceptance missing execution plan root".to_string());
    }
    if source.execution_permitted_count != config.acceptance_permit_count {
        return Err("final acceptance saw execution permit".to_string());
    }
    if source.execution_production_release_allowed != config.production_release_allowed {
        return Err("final acceptance saw execution production release flag".to_string());
    }
    if source.execution_release_plan_production_allowed != config.production_release_allowed {
        return Err("final acceptance saw plan production release flag".to_string());
    }
    if source.execution_zero_linkage_count == 0 {
        return Err("final acceptance missing zero linkage evidence".to_string());
    }
    Ok(())
}

fn acceptance_lane_source_root(source: &SourceBundle, lane: AcceptanceLane) -> String {
    match lane {
        AcceptanceLane::ExecutionBindingVerdict => source.execution_verdict_root.clone(),
        AcceptanceLane::ExecutionHoldRoot => source.execution_hold_root.clone(),
        AcceptanceLane::FinalReleaseGateReport => source.final_release_gate_report_root.clone(),
        AcceptanceLane::ExecutionReleasePlan => source.execution_release_plan_root.clone(),
        AcceptanceLane::ReviewerReceiptClosure => source.reviewer_receipt_gate_state_root.clone(),
        AcceptanceLane::PqCustodyAuthority => source.pq_authority_verification_state_root.clone(),
        AcceptanceLane::WalletEscapePriority => source.wallet_handoff_state_root.clone(),
        AcceptanceLane::PrivacyBoundary => source.execution_blocker_root.clone(),
        AcceptanceLane::CargoRuntimeEvidence => source.execution_release_plan_blocker_root.clone(),
        AcceptanceLane::AcceptanceDecisionSeal => source.final_release_gate_blocker_root.clone(),
    }
}

fn acceptance_evidence_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    lane_source_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:acceptance-evidence"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_suite),
            HashPart::Str(lane.as_str()),
            HashPart::Str(lane_source_root),
            HashPart::Str(&source.execution_binding_root),
            HashPart::Str(&source.final_release_gate_state_root),
            HashPart::Str(&source.execution_release_plan_root),
        ],
        32,
    )
}

fn acceptance_blocker_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:acceptance-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_policy),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.execution_blocker_root),
            HashPart::Str(&source.final_release_gate_blocker_root),
            HashPart::Str(&source.execution_release_plan_blocker_root),
            HashPart::U64(config.acceptance_permit_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn acceptance_release_hold_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    acceptance_blocker_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:acceptance-release-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(acceptance_blocker_root),
            HashPart::Str(&source.execution_hold_root),
            HashPart::Str(&source.final_release_gate_status_root),
            HashPart::U64(config.final_release_accepted),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn acceptance_pq_authority_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:pq-authority"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.pq_authority_verification_state_root),
            HashPart::Str(&source.custody_release_authority_spec_state_root),
            HashPart::Str(&source.authority_crosscheck_state_root),
            HashPart::U64(config.require_pq_custody_authority),
        ],
        32,
    )
}

fn acceptance_wallet_escape_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:wallet-escape"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.forced_exit_dry_run_state_root),
            HashPart::Str(&source.wallet_handoff_state_root),
            HashPart::U64(config.require_wallet_escape_priority),
        ],
        32,
    )
}

fn acceptance_privacy_boundary_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:privacy-boundary"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.execution_blocker_root),
            HashPart::Str(&source.final_release_gate_blocker_root),
            HashPart::U64(config.require_roots_only_outputs),
            HashPart::U64(config.require_zero_linkage_exports),
            HashPart::U64(config.max_linkage_exports),
        ],
        32,
    )
}

fn acceptance_decision_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    acceptance_blocker_root: &str,
    release_hold_root: &str,
    pq_authority_root: &str,
    wallet_escape_root: &str,
    privacy_boundary_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:acceptance-decision"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_policy),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&source.source_root),
            HashPart::Str(acceptance_blocker_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(pq_authority_root),
            HashPart::Str(wallet_escape_root),
            HashPart::Str(privacy_boundary_root),
            HashPart::U64(config.acceptance_permit_count),
            HashPart::U64(config.final_release_accepted),
        ],
        32,
    )
}

fn acceptance_check_root(
    config: &Config,
    source: &SourceBundle,
    lane: AcceptanceLane,
    ordinal: u64,
    lane_source_root: &str,
    evidence_root: &str,
    acceptance_blocker_root: &str,
    release_hold_root: &str,
    decision_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:acceptance-check"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_suite),
            HashPart::U64(ordinal),
            HashPart::Str(lane.as_str()),
            HashPart::Str(lane.owner_lane()),
            HashPart::Str(lane_source_root),
            HashPart::Str(evidence_root),
            HashPart::Str(acceptance_blocker_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(decision_root),
            HashPart::Str(&source.source_root),
        ],
        32,
    )
}

fn rejection_trigger_root(
    config: &Config,
    source: &SourceBundle,
    kind: RejectionCaseKind,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:rejection-trigger"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_suite),
            HashPart::Str(kind.as_str()),
            HashPart::Str(&source.execution_binding_root),
            HashPart::Str(&source.final_release_gate_state_root),
            HashPart::Str(&source.execution_release_plan_root),
        ],
        32,
    )
}

fn rejection_source_root(
    config: &Config,
    source: &SourceBundle,
    kind: RejectionCaseKind,
    trigger_root: &str,
) -> String {
    let affected_root = match kind {
        RejectionCaseKind::ExecutionBindingMissing => &source.execution_binding_root,
        RejectionCaseKind::FinalGateWatchOnly => &source.final_release_gate_status_root,
        RejectionCaseKind::ReleasePlanCargoDeferred => &source.execution_release_plan_blocker_root,
        RejectionCaseKind::ReviewerReceiptGap => &source.reviewer_receipt_gate_state_root,
        RejectionCaseKind::PqCustodyGap => &source.pq_authority_verification_state_root,
        RejectionCaseKind::WalletEscapeGap => &source.wallet_handoff_state_root,
        RejectionCaseKind::MetadataLeakProbe => &source.execution_blocker_root,
    };
    domain_hash(
        &format!("{DOMAIN}:rejection-source"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_policy),
            HashPart::Str(kind.as_str()),
            HashPart::Str(trigger_root),
            HashPart::Str(affected_root),
            HashPart::Str(&source.source_root),
        ],
        32,
    )
}

fn rejection_blocker_root(
    config: &Config,
    source: &SourceBundle,
    kind: RejectionCaseKind,
    source_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:rejection-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(source_root),
            HashPart::Str(kind.expected_response()),
            HashPart::Str(&source.execution_hold_root),
            HashPart::Str(&source.final_release_gate_blocker_root),
            HashPart::U64(config.acceptance_permit_count),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn rejection_recovery_root(
    config: &Config,
    source: &SourceBundle,
    kind: RejectionCaseKind,
    blocker_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:rejection-recovery"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind.as_str()),
            HashPart::Str(blocker_root),
            HashPart::Str(&source.forced_exit_dry_run_state_root),
            HashPart::Str(&source.wallet_handoff_state_root),
            HashPart::Str(&source.authority_crosscheck_state_root),
            HashPart::U64(config.require_wallet_escape_priority),
            HashPart::U64(config.require_pq_custody_authority),
        ],
        32,
    )
}

fn rejection_case_root(
    config: &Config,
    source: &SourceBundle,
    kind: RejectionCaseKind,
    ordinal: u64,
    trigger_root: &str,
    source_root: &str,
    blocker_root: &str,
    recovery_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:rejection-case"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_suite),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::Str(trigger_root),
            HashPart::Str(source_root),
            HashPart::Str(blocker_root),
            HashPart::Str(recovery_root),
            HashPart::Str(&source.source_root),
            HashPart::U64(config.acceptance_permit_count),
        ],
        32,
    )
}

fn acceptance_check_vector_root(checks: &[AcceptanceCheck]) -> String {
    let leaves = checks
        .iter()
        .map(AcceptanceCheck::public_record)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:acceptance-checks"), &leaves)
}

fn rejection_case_vector_root(cases: &[RejectionCase]) -> String {
    let leaves = cases
        .iter()
        .map(RejectionCase::public_record)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:rejection-cases"), &leaves)
}

fn aggregate_acceptance_hold_root(
    config: &Config,
    source: &SourceBundle,
    acceptance_check_root: &str,
    rejection_case_root: &str,
    verdict: &AcceptanceVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:aggregate-acceptance-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_policy),
            HashPart::Str(&source.execution_hold_root),
            HashPart::Str(&source.final_release_gate_blocker_root),
            HashPart::Str(acceptance_check_root),
            HashPart::Str(rejection_case_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.release_hold_count),
            HashPart::U64(verdict.rejection_block_count),
            HashPart::U64(config.final_release_accepted),
        ],
        32,
    )
}

fn aggregate_acceptance_blocker_root(
    config: &Config,
    source: &SourceBundle,
    checks: &[AcceptanceCheck],
    rejections: &[RejectionCase],
    verdict: &AcceptanceVerdict,
) -> String {
    let check_blocker_root = merkle_root(
        &format!("{DOMAIN}:check-blockers"),
        &checks
            .iter()
            .map(|check| {
                json!({
                    "lane": check.lane.as_str(),
                    "acceptance_blocker_root": check.acceptance_blocker_root,
                    "acceptance_permitted": check.acceptance_permitted,
                })
            })
            .collect::<Vec<_>>(),
    );
    let rejection_blocker_root = merkle_root(
        &format!("{DOMAIN}:rejection-blockers"),
        &rejections
            .iter()
            .map(|case| {
                json!({
                    "kind": case.kind.as_str(),
                    "blocker_root": case.blocker_root,
                    "acceptance_permitted": case.acceptance_permitted,
                })
            })
            .collect::<Vec<_>>(),
    );
    domain_hash(
        &format!("{DOMAIN}:aggregate-acceptance-blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_policy),
            HashPart::Str(&source.execution_verdict_root),
            HashPart::Str(&source.final_release_gate_status_root),
            HashPart::Str(&source.execution_release_plan_blocker_root),
            HashPart::Str(&check_blocker_root),
            HashPart::Str(&rejection_blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.acceptance_permitted_count),
            HashPart::U64(verdict.production_release_allowed),
        ],
        32,
    )
}

fn final_acceptance_root(
    config: &Config,
    source: &SourceBundle,
    acceptance_check_root: &str,
    rejection_case_root: &str,
    acceptance_hold_root: &str,
    acceptance_blocker_root: &str,
    verdict: &AcceptanceVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:final-acceptance-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.acceptance_suite),
            HashPart::Str(&source.execution_binding_state_root),
            HashPart::Str(&source.final_release_gate_state_root),
            HashPart::Str(&source.execution_release_plan_state_root),
            HashPart::Str(acceptance_check_root),
            HashPart::Str(rejection_case_root),
            HashPart::Str(acceptance_hold_root),
            HashPart::Str(acceptance_blocker_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(config.acceptance_permit_count),
            HashPart::U64(config.final_release_accepted),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn source_bundle_root(
    execution: &execution_binding::State,
    final_release: &final_gate::State,
    plan: &release_plan::State,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:source-bundle"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&execution.execution_binding_root),
            HashPart::Str(&execution.execution_hold_root),
            HashPart::Str(&execution.execution_blocker_root),
            HashPart::Str(&execution.verdict.verdict_root),
            HashPart::Str(&final_release.state_root()),
            HashPart::Str(&final_release.roots.report_root),
            HashPart::Str(&final_release.roots.counters_root),
            HashPart::Str(&plan.state_root()),
            HashPart::Str(&plan.latest_plan.plan_root),
            HashPart::Str(&plan.latest_plan.blocker_root),
            HashPart::U64(execution.verdict.execution_permitted_count),
            HashPart::U64(bool_to_u64(plan.latest_plan.production_release_allowed)),
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
        execution_binding_state_root: record_root(
            "fallback-execution-binding-state",
            &json!({ "reason": reason_ref }),
        ),
        execution_binding_root: record_root(
            "fallback-execution-binding",
            &json!({ "reason": reason_ref }),
        ),
        execution_hold_root: record_root(
            "fallback-execution-hold",
            &json!({ "reason": reason_ref }),
        ),
        execution_blocker_root: record_root(
            "fallback-execution-blocker",
            &json!({ "reason": reason_ref }),
        ),
        execution_verdict_root: record_root(
            "fallback-execution-verdict",
            &json!({ "reason": reason_ref }),
        ),
        execution_permitted_count: 0,
        execution_release_hold_count: 1,
        execution_guard_block_count: 1,
        execution_zero_linkage_count: 0,
        execution_production_release_allowed: 0,
        final_release_gate_state_root: record_root(
            "fallback-final-release-gate",
            &json!({ "reason": reason_ref }),
        ),
        final_release_gate_report_root: record_root(
            "fallback-final-release-report",
            &json!({ "reason": reason_ref }),
        ),
        final_release_gate_blocker_root: record_root(
            "fallback-final-release-blocker",
            &json!({ "reason": reason_ref }),
        ),
        final_release_gate_status_root: record_root(
            "fallback-final-release-status",
            &json!({ "reason": reason_ref }),
        ),
        final_gate_decisions_allowed: 0,
        final_gate_decisions_blocked: 1,
        final_gate_deferred_gates: 1,
        final_gate_production_blockers: 1,
        execution_release_plan_state_root: record_root(
            "fallback-execution-plan-state",
            &json!({ "reason": reason_ref }),
        ),
        execution_release_plan_root: record_root(
            "fallback-execution-plan",
            &json!({ "reason": reason_ref }),
        ),
        execution_release_plan_gate_root: record_root(
            "fallback-execution-plan-gate",
            &json!({ "reason": reason_ref }),
        ),
        execution_release_plan_blocker_root: record_root(
            "fallback-execution-plan-blocker",
            &json!({ "reason": reason_ref }),
        ),
        execution_release_plan_production_allowed: 0,
        execution_release_plan_cargo_deferred: 1,
        reviewer_receipt_gate_state_root: record_root(
            "fallback-reviewer-receipt-gate",
            &json!({ "reason": reason_ref }),
        ),
        pq_authority_verification_state_root: record_root(
            "fallback-pq-authority",
            &json!({ "reason": reason_ref }),
        ),
        custody_release_authority_spec_state_root: record_root(
            "fallback-custody-authority",
            &json!({ "reason": reason_ref }),
        ),
        authority_crosscheck_state_root: record_root(
            "fallback-authority-crosscheck",
            &json!({ "reason": reason_ref }),
        ),
        forced_exit_dry_run_state_root: record_root(
            "fallback-forced-exit-dry-run",
            &json!({ "reason": reason_ref }),
        ),
        wallet_handoff_state_root: record_root(
            "fallback-wallet-handoff",
            &json!({ "reason": reason_ref }),
        ),
        source_root: record_root("fallback-source", &json!({ "reason": reason_ref })),
    };
    let acceptance_checks = Vec::new();
    let rejection_cases = Vec::new();
    let verdict = AcceptanceVerdict::fallback(&config, reason_ref);
    let acceptance_check_root = acceptance_check_vector_root(&acceptance_checks);
    let rejection_case_root = rejection_case_vector_root(&rejection_cases);
    let acceptance_hold_root = aggregate_acceptance_hold_root(
        &config,
        &source,
        &acceptance_check_root,
        &rejection_case_root,
        &verdict,
    );
    let acceptance_blocker_root = aggregate_acceptance_blocker_root(
        &config,
        &source,
        &acceptance_checks,
        &rejection_cases,
        &verdict,
    );
    let final_acceptance_root = final_acceptance_root(
        &config,
        &source,
        &acceptance_check_root,
        &rejection_case_root,
        &acceptance_hold_root,
        &acceptance_blocker_root,
        &verdict,
    );

    State {
        config,
        source,
        acceptance_checks,
        rejection_cases,
        verdict,
        acceptance_check_root,
        rejection_case_root,
        acceptance_hold_root,
        acceptance_blocker_root,
        final_acceptance_root,
    }
}
