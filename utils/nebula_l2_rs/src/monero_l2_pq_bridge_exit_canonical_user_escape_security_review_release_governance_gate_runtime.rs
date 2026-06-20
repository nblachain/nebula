use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeSecurityReviewReleaseGovernanceGateRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_RELEASE_GOVERNANCE_GATE_RUNTIME_PROTOCOL_VERSION:
    &str = "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-security-review-release-governance-gate-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_SECURITY_REVIEW_RELEASE_GOVERNANCE_GATE_RUNTIME_PROTOCOL_VERSION;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-security-review-release-governance-gate";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub governance_suite: String,
    pub release_policy: String,
    pub min_governance_conditions: u64,
    pub require_reviewer_receipt_gate: u64,
    pub require_final_release_gate: u64,
    pub require_pq_authority_verification: u64,
    pub require_custody_policy: u64,
    pub require_user_forced_exit_priority: u64,
    pub require_roots_only_publication: u64,
    pub require_release_hold_until_all_conditions: u64,
    pub max_release_allowed_conditions: u64,
    pub max_linkage_exports: u64,
    pub production_release_allowed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            governance_suite:
                "monero-l2-pq-bridge-exit-canonical-user-escape-security-review-release-governance-gate-v1"
                    .to_string(),
            release_policy:
                "reviewer-receipt-final-gate-pq-authority-custody-policy-release-held-v1"
                    .to_string(),
            min_governance_conditions: 11,
            require_reviewer_receipt_gate: 1,
            require_final_release_gate: 1,
            require_pq_authority_verification: 1,
            require_custody_policy: 1,
            require_user_forced_exit_priority: 1,
            require_roots_only_publication: 1,
            require_release_hold_until_all_conditions: 1,
            max_release_allowed_conditions: 0,
            max_linkage_exports: 0,
            production_release_allowed: 0,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "governance_suite": self.governance_suite,
            "release_policy": self.release_policy,
            "min_governance_conditions": self.min_governance_conditions,
            "require_reviewer_receipt_gate": self.require_reviewer_receipt_gate,
            "require_final_release_gate": self.require_final_release_gate,
            "require_pq_authority_verification": self.require_pq_authority_verification,
            "require_custody_policy": self.require_custody_policy,
            "require_user_forced_exit_priority": self.require_user_forced_exit_priority,
            "require_roots_only_publication": self.require_roots_only_publication,
            "require_release_hold_until_all_conditions": self.require_release_hold_until_all_conditions,
            "max_release_allowed_conditions": self.max_release_allowed_conditions,
            "max_linkage_exports": self.max_linkage_exports,
            "production_release_allowed": self.production_release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDomain {
    ReviewerReceiptClosure,
    LiveRuntimeClosure,
    HumanAuditClosure,
    PqAuthorityQuorum,
    CustodyReleasePolicy,
    AuthorityCrosscheck,
    FinalReleaseGate,
    ExecutionGatePlan,
    UserForcedExitPriority,
    PrivacyRedactionBoundary,
    EmergencyPauseTimelock,
}

impl GovernanceDomain {
    pub fn ordered() -> [Self; 11] {
        [
            Self::ReviewerReceiptClosure,
            Self::LiveRuntimeClosure,
            Self::HumanAuditClosure,
            Self::PqAuthorityQuorum,
            Self::CustodyReleasePolicy,
            Self::AuthorityCrosscheck,
            Self::FinalReleaseGate,
            Self::ExecutionGatePlan,
            Self::UserForcedExitPriority,
            Self::PrivacyRedactionBoundary,
            Self::EmergencyPauseTimelock,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReviewerReceiptClosure => "reviewer_receipt_closure",
            Self::LiveRuntimeClosure => "live_runtime_closure",
            Self::HumanAuditClosure => "human_audit_closure",
            Self::PqAuthorityQuorum => "pq_authority_quorum",
            Self::CustodyReleasePolicy => "custody_release_policy",
            Self::AuthorityCrosscheck => "authority_crosscheck",
            Self::FinalReleaseGate => "final_release_gate",
            Self::ExecutionGatePlan => "execution_gate_plan",
            Self::UserForcedExitPriority => "user_forced_exit_priority",
            Self::PrivacyRedactionBoundary => "privacy_redaction_boundary",
            Self::EmergencyPauseTimelock => "emergency_pause_timelock",
        }
    }

    pub fn owner_lane(self) -> &'static str {
        match self {
            Self::ReviewerReceiptClosure => "security_review_receipt_gate_owner",
            Self::LiveRuntimeClosure => "runtime_execution_gate_owner",
            Self::HumanAuditClosure => "security_audit_owner",
            Self::PqAuthorityQuorum => "pq_release_authority_owner",
            Self::CustodyReleasePolicy => "custody_release_policy_owner",
            Self::AuthorityCrosscheck => "authority_crosscheck_owner",
            Self::FinalReleaseGate => "final_release_gate_owner",
            Self::ExecutionGatePlan => "execution_release_gate_owner",
            Self::UserForcedExitPriority => "wallet_forced_exit_owner",
            Self::PrivacyRedactionBoundary => "privacy_redaction_owner",
            Self::EmergencyPauseTimelock => "emergency_pause_timelock_owner",
        }
    }

    pub fn release_question(self) -> &'static str {
        match self {
            Self::ReviewerReceiptClosure => {
                "Are all reviewer receipts present and release-held until live and human review closes?"
            }
            Self::LiveRuntimeClosure => {
                "Are cargo/runtime execution receipts present for the bridge-exit security lanes?"
            }
            Self::HumanAuditClosure => {
                "Has the security audit manifest produced reviewer-accepted closure roots?"
            }
            Self::PqAuthorityQuorum => {
                "Is PQ release authority fresh, quorum-bound, replay-checked, and withdrawal-authorized?"
            }
            Self::CustodyReleasePolicy => {
                "Does custody policy prevent unilateral release and preserve user forced-exit priority?"
            }
            Self::AuthorityCrosscheck => {
                "Do authority crosschecks agree with the trust-minimized bridge-exit spine?"
            }
            Self::FinalReleaseGate => {
                "Does the final release gate have zero user-release or production blockers?"
            }
            Self::ExecutionGatePlan => {
                "Does the execution release plan prove every required release gate has a receipt?"
            }
            Self::UserForcedExitPriority => {
                "Can a wallet force exit without sequencer cooperation while release remains held on mismatch?"
            }
            Self::PrivacyRedactionBoundary => {
                "Are governance records roots-only with no linkable wallet, note, or receipt metadata?"
            }
            Self::EmergencyPauseTimelock => {
                "Can emergency pause and upgrade timelock controls stop release without blocking wallet escape?"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceStatus {
    PendingEvidence,
    ReleaseHeldByPolicy,
    BlockedRelease,
}

impl GovernanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingEvidence => "pending_evidence",
            Self::ReleaseHeldByPolicy => "release_held_by_policy",
            Self::BlockedRelease => "blocked_release",
        }
    }

    pub fn release_allowed(self) -> u64 {
        match self {
            Self::PendingEvidence | Self::ReleaseHeldByPolicy | Self::BlockedRelease => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRoots {
    pub reviewer_receipt_gate_state_root: String,
    pub reviewer_receipt_gate_root: String,
    pub reviewer_requirement_root: String,
    pub reviewer_release_hold_root: String,
    pub reviewer_verdict_root: String,
    pub signoff_execution_manifest_state_root: String,
    pub signoff_execution_manifest_root: String,
    pub signoff_execution_release_hold_root: String,
    pub signoff_bundle_state_root: String,
    pub signoff_bundle_root: String,
    pub signoff_bundle_release_hold_root: String,
    pub final_release_gate_state_root: String,
    pub execution_release_gate_plan_state_root: String,
    pub pq_authority_verification_state_root: String,
    pub pq_release_authority_quorum_replay_state_root: String,
    pub custody_release_authority_spec_state_root: String,
    pub authority_crosscheck_state_root: String,
    pub security_audit_manifest_state_root: String,
    pub forced_exit_dry_run_state_root: String,
    pub wallet_handoff_state_root: String,
    pub go_no_go_matrix_root: String,
    pub source_root: String,
}

impl SourceRoots {
    pub fn devnet() -> Self {
        let reviewer_gate =
            crate::monero_l2_pq_bridge_exit_canonical_user_escape_security_review_signoff_reviewer_receipt_gate_runtime::devnet();
        let execution_manifest =
            crate::monero_l2_pq_bridge_exit_canonical_user_escape_security_review_signoff_execution_manifest_runtime::devnet();
        let signoff_bundle =
            crate::monero_l2_pq_bridge_exit_canonical_user_escape_security_review_signoff_bundle_runtime::devnet();
        let final_release_gate =
            crate::monero_l2_pq_bridge_exit_final_release_gate_runtime::devnet();
        let execution_release_gate_plan =
            crate::monero_l2_pq_bridge_exit_execution_release_gate_plan_runtime::devnet();
        let pq_authority_verification =
            crate::monero_l2_pq_bridge_exit_pq_authority_verification_contract_runtime::devnet();
        let pq_quorum_replay =
            crate::monero_l2_pq_bridge_exit_canonical_pq_release_authority_quorum_replay_runtime::devnet();
        let custody_release_authority =
            crate::monero_l2_pq_bridge_exit_custody_release_authority_spec_runtime::devnet();
        let authority_crosscheck =
            crate::monero_l2_pq_bridge_exit_authority_crosscheck_verifier_runtime::devnet();
        let security_audit =
            crate::monero_l2_pq_bridge_exit_security_audit_signoff_manifest_runtime::devnet();
        let forced_exit_dry_run =
            crate::monero_l2_pq_bridge_exit_canonical_user_escape_forced_exit_vertical_dry_run_runtime::devnet();
        let wallet_handoff =
            crate::monero_l2_pq_bridge_exit_canonical_user_escape_forced_exit_dry_run_wallet_handoff_runtime::devnet();
        let reviewer_receipt_gate_state_root = reviewer_gate.state_root();
        let reviewer_receipt_gate_root = reviewer_gate.gate_root.clone();
        let reviewer_requirement_root = reviewer_gate.requirement_root.clone();
        let reviewer_release_hold_root = reviewer_gate.release_hold_root.clone();
        let reviewer_verdict_root = reviewer_gate.verdict.verdict_root.clone();
        let signoff_execution_manifest_state_root = execution_manifest.state_root();
        let signoff_execution_manifest_root = execution_manifest.manifest_root.clone();
        let signoff_execution_release_hold_root = execution_manifest.release_hold_root.clone();
        let signoff_bundle_state_root = signoff_bundle.state_root();
        let signoff_bundle_root = signoff_bundle.bundle_root.clone();
        let signoff_bundle_release_hold_root = signoff_bundle.release_hold_root.clone();
        let final_release_gate_state_root = final_release_gate.state_root();
        let execution_release_gate_plan_state_root = execution_release_gate_plan.state_root();
        let pq_authority_verification_state_root = pq_authority_verification.state_root();
        let pq_release_authority_quorum_replay_state_root = pq_quorum_replay.state_root();
        let custody_release_authority_spec_state_root = custody_release_authority.state_root();
        let authority_crosscheck_state_root = authority_crosscheck.state_root();
        let security_audit_manifest_state_root = security_audit.state_root();
        let forced_exit_dry_run_state_root = forced_exit_dry_run.state_root();
        let wallet_handoff_state_root = wallet_handoff.state_root();
        let go_no_go_matrix_root =
            crate::monero_l2_pq_bridge_exit_canonical_release_candidate_go_no_go_matrix_runtime::state_root();
        let source_root = source_root(
            &reviewer_receipt_gate_state_root,
            &reviewer_receipt_gate_root,
            &reviewer_release_hold_root,
            &signoff_execution_manifest_root,
            &signoff_execution_release_hold_root,
            &final_release_gate_state_root,
            &execution_release_gate_plan_state_root,
            &pq_authority_verification_state_root,
            &pq_release_authority_quorum_replay_state_root,
            &custody_release_authority_spec_state_root,
            &authority_crosscheck_state_root,
            &security_audit_manifest_state_root,
            &forced_exit_dry_run_state_root,
            &wallet_handoff_state_root,
            &go_no_go_matrix_root,
        );

        Self {
            reviewer_receipt_gate_state_root,
            reviewer_receipt_gate_root,
            reviewer_requirement_root,
            reviewer_release_hold_root,
            reviewer_verdict_root,
            signoff_execution_manifest_state_root,
            signoff_execution_manifest_root,
            signoff_execution_release_hold_root,
            signoff_bundle_state_root,
            signoff_bundle_root,
            signoff_bundle_release_hold_root,
            final_release_gate_state_root,
            execution_release_gate_plan_state_root,
            pq_authority_verification_state_root,
            pq_release_authority_quorum_replay_state_root,
            custody_release_authority_spec_state_root,
            authority_crosscheck_state_root,
            security_audit_manifest_state_root,
            forced_exit_dry_run_state_root,
            wallet_handoff_state_root,
            go_no_go_matrix_root,
            source_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_receipt_gate_state_root": self.reviewer_receipt_gate_state_root,
            "reviewer_receipt_gate_root": self.reviewer_receipt_gate_root,
            "reviewer_requirement_root": self.reviewer_requirement_root,
            "reviewer_release_hold_root": self.reviewer_release_hold_root,
            "reviewer_verdict_root": self.reviewer_verdict_root,
            "signoff_execution_manifest_state_root": self.signoff_execution_manifest_state_root,
            "signoff_execution_manifest_root": self.signoff_execution_manifest_root,
            "signoff_execution_release_hold_root": self.signoff_execution_release_hold_root,
            "signoff_bundle_state_root": self.signoff_bundle_state_root,
            "signoff_bundle_root": self.signoff_bundle_root,
            "signoff_bundle_release_hold_root": self.signoff_bundle_release_hold_root,
            "final_release_gate_state_root": self.final_release_gate_state_root,
            "execution_release_gate_plan_state_root": self.execution_release_gate_plan_state_root,
            "pq_authority_verification_state_root": self.pq_authority_verification_state_root,
            "pq_release_authority_quorum_replay_state_root": self.pq_release_authority_quorum_replay_state_root,
            "custody_release_authority_spec_state_root": self.custody_release_authority_spec_state_root,
            "authority_crosscheck_state_root": self.authority_crosscheck_state_root,
            "security_audit_manifest_state_root": self.security_audit_manifest_state_root,
            "forced_exit_dry_run_state_root": self.forced_exit_dry_run_state_root,
            "wallet_handoff_state_root": self.wallet_handoff_state_root,
            "go_no_go_matrix_root": self.go_no_go_matrix_root,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-roots", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceCondition {
    pub ordinal: u64,
    pub domain: GovernanceDomain,
    pub owner_lane: String,
    pub release_question: String,
    pub status: GovernanceStatus,
    pub evidence_root: String,
    pub authority_root: String,
    pub wallet_escape_root: String,
    pub privacy_boundary_root: String,
    pub blocker_root: String,
    pub release_decision_root: String,
    pub release_hold_root: String,
    pub roots_only_required: u64,
    pub release_allowed: u64,
    pub linkage_exports_allowed: u64,
    pub condition_root: String,
}

impl GovernanceCondition {
    pub fn devnet(
        config: &Config,
        source: &SourceRoots,
        domain: GovernanceDomain,
        ordinal: u64,
    ) -> Self {
        let status = GovernanceStatus::PendingEvidence;
        let evidence_root = governance_evidence_root(config, source, domain);
        let authority_root = governance_authority_root(config, source, domain, &evidence_root);
        let wallet_escape_root = governance_wallet_escape_root(config, source, domain);
        let privacy_boundary_root =
            governance_privacy_boundary_root(config, source, domain, &evidence_root);
        let blocker_root = governance_blocker_root(
            config,
            source,
            domain,
            &evidence_root,
            &authority_root,
            &wallet_escape_root,
            &privacy_boundary_root,
        );
        let release_decision_root = governance_release_decision_root(
            config,
            source,
            domain,
            status,
            &evidence_root,
            &authority_root,
            &blocker_root,
        );
        let release_hold_root =
            governance_release_hold_root(config, source, domain, &release_decision_root);
        let condition_root = governance_condition_root(
            config,
            source,
            domain,
            ordinal,
            status,
            &evidence_root,
            &authority_root,
            &wallet_escape_root,
            &privacy_boundary_root,
            &blocker_root,
            &release_decision_root,
            &release_hold_root,
        );

        Self {
            ordinal,
            domain,
            owner_lane: domain.owner_lane().to_string(),
            release_question: domain.release_question().to_string(),
            status,
            evidence_root,
            authority_root,
            wallet_escape_root,
            privacy_boundary_root,
            blocker_root,
            release_decision_root,
            release_hold_root,
            roots_only_required: config.require_roots_only_publication,
            release_allowed: status.release_allowed(),
            linkage_exports_allowed: config.max_linkage_exports,
            condition_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ordinal": self.ordinal,
            "domain": self.domain.as_str(),
            "owner_lane": self.owner_lane,
            "release_question": self.release_question,
            "status": self.status.as_str(),
            "evidence_root": self.evidence_root,
            "authority_root": self.authority_root,
            "wallet_escape_root": self.wallet_escape_root,
            "privacy_boundary_root": self.privacy_boundary_root,
            "blocker_root": self.blocker_root,
            "release_decision_root": self.release_decision_root,
            "release_hold_root": self.release_hold_root,
            "roots_only_required": self.roots_only_required,
            "release_allowed": self.release_allowed,
            "linkage_exports_allowed": self.linkage_exports_allowed,
            "condition_root": self.condition_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseGovernanceVerdict {
    pub governance_condition_count: u64,
    pub pending_evidence_count: u64,
    pub policy_hold_count: u64,
    pub blocked_release_count: u64,
    pub release_allowed_count: u64,
    pub release_hold_count: u64,
    pub zero_linkage_condition_count: u64,
    pub roots_only_condition_count: u64,
    pub production_release_allowed: u64,
    pub governance_status: String,
    pub verdict_root: String,
}

impl ReleaseGovernanceVerdict {
    pub fn new(config: &Config, conditions: &[GovernanceCondition]) -> Self {
        let governance_condition_count = conditions.len() as u64;
        let pending_evidence_count = count_status(conditions, GovernanceStatus::PendingEvidence);
        let policy_hold_count = count_status(conditions, GovernanceStatus::ReleaseHeldByPolicy);
        let blocked_release_count = count_status(conditions, GovernanceStatus::BlockedRelease);
        let release_allowed_count = conditions
            .iter()
            .filter(|condition| condition.release_allowed == 1)
            .count() as u64;
        let release_hold_count = conditions
            .iter()
            .filter(|condition| condition.release_allowed == 0)
            .count() as u64;
        let zero_linkage_condition_count = conditions
            .iter()
            .filter(|condition| condition.linkage_exports_allowed <= config.max_linkage_exports)
            .count() as u64;
        let roots_only_condition_count = conditions
            .iter()
            .filter(|condition| condition.roots_only_required == 1)
            .count() as u64;
        let production_release_allowed = config.production_release_allowed;
        let governance_status = if governance_condition_count >= config.min_governance_conditions
            && pending_evidence_count == governance_condition_count
            && release_allowed_count <= config.max_release_allowed_conditions
            && release_hold_count == governance_condition_count
            && zero_linkage_condition_count == governance_condition_count
            && roots_only_condition_count == governance_condition_count
            && production_release_allowed == 0
        {
            "release_governance_gate_ready_for_receipts_release_held"
        } else {
            "release_governance_gate_gap_release_held"
        }
        .to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.release_policy),
                HashPart::U64(governance_condition_count),
                HashPart::U64(pending_evidence_count),
                HashPart::U64(policy_hold_count),
                HashPart::U64(blocked_release_count),
                HashPart::U64(release_allowed_count),
                HashPart::U64(release_hold_count),
                HashPart::U64(zero_linkage_condition_count),
                HashPart::U64(roots_only_condition_count),
                HashPart::U64(production_release_allowed),
                HashPart::Str(&governance_status),
            ],
            32,
        );

        Self {
            governance_condition_count,
            pending_evidence_count,
            policy_hold_count,
            blocked_release_count,
            release_allowed_count,
            release_hold_count,
            zero_linkage_condition_count,
            roots_only_condition_count,
            production_release_allowed,
            governance_status,
            verdict_root,
        }
    }

    pub fn fallback(config: &Config, reason: &str) -> Self {
        let governance_status = "release_governance_gate_construction_gap_release_held".to_string();
        let verdict_root = domain_hash(
            &format!("{DOMAIN}:fallback-verdict"),
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.release_policy),
                HashPart::Str(reason),
                HashPart::Str(&governance_status),
            ],
            32,
        );

        Self {
            governance_condition_count: 0,
            pending_evidence_count: 0,
            policy_hold_count: 0,
            blocked_release_count: 1,
            release_allowed_count: 0,
            release_hold_count: 1,
            zero_linkage_condition_count: 0,
            roots_only_condition_count: 0,
            production_release_allowed: 0,
            governance_status,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "governance_condition_count": self.governance_condition_count,
            "pending_evidence_count": self.pending_evidence_count,
            "policy_hold_count": self.policy_hold_count,
            "blocked_release_count": self.blocked_release_count,
            "release_allowed_count": self.release_allowed_count,
            "release_hold_count": self.release_hold_count,
            "zero_linkage_condition_count": self.zero_linkage_condition_count,
            "roots_only_condition_count": self.roots_only_condition_count,
            "production_release_allowed": self.production_release_allowed,
            "governance_status": self.governance_status,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub source_roots: SourceRoots,
    pub governance_conditions: Vec<GovernanceCondition>,
    pub verdict: ReleaseGovernanceVerdict,
    pub condition_root: String,
    pub reviewer_receipt_closure_root: String,
    pub live_runtime_closure_root: String,
    pub human_audit_closure_root: String,
    pub pq_authority_quorum_root: String,
    pub custody_policy_root: String,
    pub authority_crosscheck_root: String,
    pub final_release_gate_root: String,
    pub execution_gate_plan_root: String,
    pub user_forced_exit_priority_root: String,
    pub privacy_redaction_root: String,
    pub emergency_timelock_root: String,
    pub release_hold_root: String,
    pub governance_root: String,
}

impl State {
    pub fn new(config: Config, source_roots: SourceRoots) -> Result<Self> {
        validate_config(&config)?;
        let governance_conditions = GovernanceDomain::ordered()
            .iter()
            .enumerate()
            .map(|(index, domain)| {
                GovernanceCondition::devnet(&config, &source_roots, *domain, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = ReleaseGovernanceVerdict::new(&config, &governance_conditions);
        let condition_root = condition_vector_root(&governance_conditions);
        let reviewer_receipt_closure_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::ReviewerReceiptClosure,
        );
        let live_runtime_closure_root =
            domain_condition_root(&governance_conditions, GovernanceDomain::LiveRuntimeClosure);
        let human_audit_closure_root =
            domain_condition_root(&governance_conditions, GovernanceDomain::HumanAuditClosure);
        let pq_authority_quorum_root =
            domain_condition_root(&governance_conditions, GovernanceDomain::PqAuthorityQuorum);
        let custody_policy_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::CustodyReleasePolicy,
        );
        let authority_crosscheck_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::AuthorityCrosscheck,
        );
        let final_release_gate_root =
            domain_condition_root(&governance_conditions, GovernanceDomain::FinalReleaseGate);
        let execution_gate_plan_root =
            domain_condition_root(&governance_conditions, GovernanceDomain::ExecutionGatePlan);
        let user_forced_exit_priority_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::UserForcedExitPriority,
        );
        let privacy_redaction_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::PrivacyRedactionBoundary,
        );
        let emergency_timelock_root = domain_condition_root(
            &governance_conditions,
            GovernanceDomain::EmergencyPauseTimelock,
        );
        let release_hold_root =
            aggregate_release_hold_root(&config, &source_roots, &governance_conditions, &verdict);
        let governance_root = governance_root(
            &config,
            &source_roots,
            &condition_root,
            &release_hold_root,
            &verdict,
        );

        Ok(Self {
            config,
            source_roots,
            governance_conditions,
            verdict,
            condition_root,
            reviewer_receipt_closure_root,
            live_runtime_closure_root,
            human_audit_closure_root,
            pq_authority_quorum_root,
            custody_policy_root,
            authority_crosscheck_root,
            final_release_gate_root,
            execution_gate_plan_root,
            user_forced_exit_priority_root,
            privacy_redaction_root,
            emergency_timelock_root,
            release_hold_root,
            governance_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), SourceRoots::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_security_review_release_governance_gate_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source_roots": self.source_roots.public_record(),
            "condition_root": self.condition_root,
            "reviewer_receipt_closure_root": self.reviewer_receipt_closure_root,
            "live_runtime_closure_root": self.live_runtime_closure_root,
            "human_audit_closure_root": self.human_audit_closure_root,
            "pq_authority_quorum_root": self.pq_authority_quorum_root,
            "custody_policy_root": self.custody_policy_root,
            "authority_crosscheck_root": self.authority_crosscheck_root,
            "final_release_gate_root": self.final_release_gate_root,
            "execution_gate_plan_root": self.execution_gate_plan_root,
            "user_forced_exit_priority_root": self.user_forced_exit_priority_root,
            "privacy_redaction_root": self.privacy_redaction_root,
            "emergency_timelock_root": self.emergency_timelock_root,
            "release_hold_root": self.release_hold_root,
            "governance_root": self.governance_root,
            "verdict": self.verdict.public_record(),
            "governance_conditions": self
                .governance_conditions
                .iter()
                .map(GovernanceCondition::public_record)
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
                "source_root": self.source_roots.state_root(),
                "condition_root": self.condition_root,
                "release_hold_root": self.release_hold_root,
                "governance_root": self.governance_root,
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
        return Err("config chain id must match crate chain id".to_string());
    }
    if config.min_governance_conditions < GovernanceDomain::ordered().len() as u64 {
        return Err("release governance gate must cover every governance condition".to_string());
    }
    if config.require_release_hold_until_all_conditions != 1 {
        return Err(
            "release governance gate must hold release until every condition closes".to_string(),
        );
    }
    if config.production_release_allowed != 0 {
        return Err(
            "production release must remain held before reviewer receipts and runtime execution"
                .to_string(),
        );
    }
    if config.max_release_allowed_conditions != 0 {
        return Err(
            "release governance gate cannot allow release before all conditions close".to_string(),
        );
    }
    if config.max_linkage_exports != 0 {
        return Err("release governance gate must publish roots only".to_string());
    }
    Ok(())
}

fn governance_evidence_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
) -> String {
    match domain {
        GovernanceDomain::ReviewerReceiptClosure => domain_hash(
            &format!("{DOMAIN}:reviewer-receipt-evidence"),
            &[
                HashPart::Str(&source.reviewer_receipt_gate_root),
                HashPart::Str(&source.reviewer_requirement_root),
                HashPart::Str(&source.reviewer_verdict_root),
                HashPart::Str(&source.reviewer_release_hold_root),
            ],
            32,
        ),
        GovernanceDomain::LiveRuntimeClosure => domain_hash(
            &format!("{DOMAIN}:live-runtime-evidence"),
            &[
                HashPart::Str(&source.signoff_execution_manifest_root),
                HashPart::Str(&source.signoff_execution_release_hold_root),
                HashPart::Str(&source.final_release_gate_state_root),
                HashPart::Str(&source.execution_release_gate_plan_state_root),
            ],
            32,
        ),
        GovernanceDomain::HumanAuditClosure => domain_hash(
            &format!("{DOMAIN}:human-audit-evidence"),
            &[
                HashPart::Str(&source.security_audit_manifest_state_root),
                HashPart::Str(&source.signoff_bundle_root),
                HashPart::Str(&source.signoff_bundle_release_hold_root),
                HashPart::Str(&source.reviewer_receipt_gate_state_root),
            ],
            32,
        ),
        GovernanceDomain::PqAuthorityQuorum => domain_hash(
            &format!("{DOMAIN}:pq-authority-evidence"),
            &[
                HashPart::Str(&source.pq_authority_verification_state_root),
                HashPart::Str(&source.pq_release_authority_quorum_replay_state_root),
                HashPart::Str(&source.go_no_go_matrix_root),
                HashPart::U64(config.require_pq_authority_verification),
            ],
            32,
        ),
        GovernanceDomain::CustodyReleasePolicy => domain_hash(
            &format!("{DOMAIN}:custody-policy-evidence"),
            &[
                HashPart::Str(&source.custody_release_authority_spec_state_root),
                HashPart::Str(&source.authority_crosscheck_state_root),
                HashPart::Str(&source.wallet_handoff_state_root),
                HashPart::U64(config.require_custody_policy),
            ],
            32,
        ),
        GovernanceDomain::AuthorityCrosscheck => domain_hash(
            &format!("{DOMAIN}:authority-crosscheck-evidence"),
            &[
                HashPart::Str(&source.authority_crosscheck_state_root),
                HashPart::Str(&source.pq_authority_verification_state_root),
                HashPart::Str(&source.custody_release_authority_spec_state_root),
            ],
            32,
        ),
        GovernanceDomain::FinalReleaseGate => domain_hash(
            &format!("{DOMAIN}:final-release-gate-evidence"),
            &[
                HashPart::Str(&source.final_release_gate_state_root),
                HashPart::Str(&source.go_no_go_matrix_root),
                HashPart::Str(&source.reviewer_release_hold_root),
                HashPart::U64(config.require_final_release_gate),
            ],
            32,
        ),
        GovernanceDomain::ExecutionGatePlan => domain_hash(
            &format!("{DOMAIN}:execution-gate-plan-evidence"),
            &[
                HashPart::Str(&source.execution_release_gate_plan_state_root),
                HashPart::Str(&source.signoff_execution_manifest_state_root),
                HashPart::Str(&source.reviewer_receipt_gate_state_root),
            ],
            32,
        ),
        GovernanceDomain::UserForcedExitPriority => domain_hash(
            &format!("{DOMAIN}:user-forced-exit-evidence"),
            &[
                HashPart::Str(&source.forced_exit_dry_run_state_root),
                HashPart::Str(&source.wallet_handoff_state_root),
                HashPart::Str(&source.custody_release_authority_spec_state_root),
                HashPart::U64(config.require_user_forced_exit_priority),
            ],
            32,
        ),
        GovernanceDomain::PrivacyRedactionBoundary => domain_hash(
            &format!("{DOMAIN}:privacy-redaction-evidence"),
            &[
                HashPart::Str(&source.signoff_bundle_release_hold_root),
                HashPart::Str(&source.reviewer_release_hold_root),
                HashPart::Str(&source.wallet_handoff_state_root),
                HashPart::U64(config.require_roots_only_publication),
            ],
            32,
        ),
        GovernanceDomain::EmergencyPauseTimelock => domain_hash(
            &format!("{DOMAIN}:emergency-pause-timelock-evidence"),
            &[
                HashPart::Str(&source.custody_release_authority_spec_state_root),
                HashPart::Str(&source.authority_crosscheck_state_root),
                HashPart::Str(&source.go_no_go_matrix_root),
            ],
            32,
        ),
    }
}

fn governance_authority_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:authority"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_policy),
            HashPart::Str(domain.as_str()),
            HashPart::Str(domain.owner_lane()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.pq_authority_verification_state_root),
            HashPart::Str(&source.custody_release_authority_spec_state_root),
            HashPart::Str(&source.authority_crosscheck_state_root),
        ],
        32,
    )
}

fn governance_wallet_escape_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:wallet-escape"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(domain.as_str()),
            HashPart::Str(&source.forced_exit_dry_run_state_root),
            HashPart::Str(&source.wallet_handoff_state_root),
            HashPart::Str(&source.reviewer_release_hold_root),
            HashPart::U64(config.require_user_forced_exit_priority),
        ],
        32,
    )
}

fn governance_privacy_boundary_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    evidence_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:privacy-boundary"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(domain.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(&source.signoff_bundle_release_hold_root),
            HashPart::Str(&source.reviewer_release_hold_root),
            HashPart::U64(config.require_roots_only_publication),
            HashPart::U64(config.max_linkage_exports),
        ],
        32,
    )
}

fn governance_blocker_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    evidence_root: &str,
    authority_root: &str,
    wallet_escape_root: &str,
    privacy_boundary_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:blocker"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_policy),
            HashPart::Str(domain.as_str()),
            HashPart::Str(&source.reviewer_release_hold_root),
            HashPart::Str(&source.final_release_gate_state_root),
            HashPart::Str(evidence_root),
            HashPart::Str(authority_root),
            HashPart::Str(wallet_escape_root),
            HashPart::Str(privacy_boundary_root),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn governance_release_decision_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    status: GovernanceStatus,
    evidence_root: &str,
    authority_root: &str,
    blocker_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:release-decision"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.governance_suite),
            HashPart::Str(domain.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&source.source_root),
            HashPart::Str(evidence_root),
            HashPart::Str(authority_root),
            HashPart::Str(blocker_root),
            HashPart::U64(status.release_allowed()),
            HashPart::U64(config.production_release_allowed),
        ],
        32,
    )
}

fn governance_release_hold_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    release_decision_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:release-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_policy),
            HashPart::Str(domain.as_str()),
            HashPart::Str(&source.reviewer_release_hold_root),
            HashPart::Str(&source.signoff_execution_release_hold_root),
            HashPart::Str(&source.signoff_bundle_release_hold_root),
            HashPart::Str(release_decision_root),
            HashPart::U64(config.require_release_hold_until_all_conditions),
            HashPart::U64(0),
        ],
        32,
    )
}

fn governance_condition_root(
    config: &Config,
    source: &SourceRoots,
    domain: GovernanceDomain,
    ordinal: u64,
    status: GovernanceStatus,
    evidence_root: &str,
    authority_root: &str,
    wallet_escape_root: &str,
    privacy_boundary_root: &str,
    blocker_root: &str,
    release_decision_root: &str,
    release_hold_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:condition"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_policy),
            HashPart::Str(&source.source_root),
            HashPart::U64(ordinal),
            HashPart::Str(domain.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(evidence_root),
            HashPart::Str(authority_root),
            HashPart::Str(wallet_escape_root),
            HashPart::Str(privacy_boundary_root),
            HashPart::Str(blocker_root),
            HashPart::Str(release_decision_root),
            HashPart::Str(release_hold_root),
            HashPart::U64(status.release_allowed()),
        ],
        32,
    )
}

fn count_status(conditions: &[GovernanceCondition], status: GovernanceStatus) -> u64 {
    conditions
        .iter()
        .filter(|condition| condition.status == status)
        .count() as u64
}

fn condition_vector_root(conditions: &[GovernanceCondition]) -> String {
    let leaves = conditions
        .iter()
        .map(GovernanceCondition::public_record)
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:condition-root"), &leaves)
}

fn domain_condition_root(conditions: &[GovernanceCondition], domain: GovernanceDomain) -> String {
    let leaves = conditions
        .iter()
        .filter(|condition| condition.domain == domain)
        .map(GovernanceCondition::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        &format!("{DOMAIN}:{}-condition-root", domain.as_str()),
        &leaves,
    )
}

fn aggregate_release_hold_root(
    config: &Config,
    source: &SourceRoots,
    conditions: &[GovernanceCondition],
    verdict: &ReleaseGovernanceVerdict,
) -> String {
    let leaves = conditions
        .iter()
        .filter(|condition| condition.release_allowed == 0)
        .map(GovernanceCondition::public_record)
        .collect::<Vec<_>>();
    let held_condition_root = merkle_root(&format!("{DOMAIN}:held-condition-root"), &leaves);
    domain_hash(
        &format!("{DOMAIN}:aggregate-release-hold"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.release_policy),
            HashPart::Str(&source.source_root),
            HashPart::Str(&held_condition_root),
            HashPart::Str(&source.reviewer_release_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.release_hold_count),
            HashPart::U64(config.require_release_hold_until_all_conditions),
        ],
        32,
    )
}

fn governance_root(
    config: &Config,
    source: &SourceRoots,
    condition_root: &str,
    release_hold_root: &str,
    verdict: &ReleaseGovernanceVerdict,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:governance"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.governance_suite),
            HashPart::Str(&source.source_root),
            HashPart::Str(condition_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(&verdict.verdict_root),
            HashPart::U64(verdict.release_allowed_count),
            HashPart::U64(verdict.production_release_allowed),
        ],
        32,
    )
}

fn source_root(
    reviewer_receipt_gate_state_root: &str,
    reviewer_receipt_gate_root: &str,
    reviewer_release_hold_root: &str,
    signoff_execution_manifest_root: &str,
    signoff_execution_release_hold_root: &str,
    final_release_gate_state_root: &str,
    execution_release_gate_plan_state_root: &str,
    pq_authority_verification_state_root: &str,
    pq_release_authority_quorum_replay_state_root: &str,
    custody_release_authority_spec_state_root: &str,
    authority_crosscheck_state_root: &str,
    security_audit_manifest_state_root: &str,
    forced_exit_dry_run_state_root: &str,
    wallet_handoff_state_root: &str,
    go_no_go_matrix_root: &str,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:source"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(reviewer_receipt_gate_state_root),
            HashPart::Str(reviewer_receipt_gate_root),
            HashPart::Str(reviewer_release_hold_root),
            HashPart::Str(signoff_execution_manifest_root),
            HashPart::Str(signoff_execution_release_hold_root),
            HashPart::Str(final_release_gate_state_root),
            HashPart::Str(execution_release_gate_plan_state_root),
            HashPart::Str(pq_authority_verification_state_root),
            HashPart::Str(pq_release_authority_quorum_replay_state_root),
            HashPart::Str(custody_release_authority_spec_state_root),
            HashPart::Str(authority_crosscheck_state_root),
            HashPart::Str(security_audit_manifest_state_root),
            HashPart::Str(forced_exit_dry_run_state_root),
            HashPart::Str(wallet_handoff_state_root),
            HashPart::Str(go_no_go_matrix_root),
        ],
        32,
    )
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
    let source_roots = SourceRoots {
        reviewer_receipt_gate_state_root: record_root(
            "fallback-reviewer-gate-state",
            &json!({ "reason": reason_ref }),
        ),
        reviewer_receipt_gate_root: record_root(
            "fallback-reviewer-gate",
            &json!({ "reason": reason_ref }),
        ),
        reviewer_requirement_root: record_root(
            "fallback-reviewer-requirement",
            &json!({ "reason": reason_ref }),
        ),
        reviewer_release_hold_root: record_root(
            "fallback-reviewer-release-hold",
            &json!({ "reason": reason_ref }),
        ),
        reviewer_verdict_root: record_root(
            "fallback-reviewer-verdict",
            &json!({ "reason": reason_ref }),
        ),
        signoff_execution_manifest_state_root: record_root(
            "fallback-execution-manifest-state",
            &json!({ "reason": reason_ref }),
        ),
        signoff_execution_manifest_root: record_root(
            "fallback-execution-manifest",
            &json!({ "reason": reason_ref }),
        ),
        signoff_execution_release_hold_root: record_root(
            "fallback-execution-release-hold",
            &json!({ "reason": reason_ref }),
        ),
        signoff_bundle_state_root: record_root(
            "fallback-signoff-bundle-state",
            &json!({ "reason": reason_ref }),
        ),
        signoff_bundle_root: record_root("fallback-signoff", &json!({ "reason": reason_ref })),
        signoff_bundle_release_hold_root: record_root(
            "fallback-signoff-release-hold",
            &json!({ "reason": reason_ref }),
        ),
        final_release_gate_state_root: record_root(
            "fallback-final-release-gate",
            &json!({ "reason": reason_ref }),
        ),
        execution_release_gate_plan_state_root: record_root(
            "fallback-execution-release-plan",
            &json!({ "reason": reason_ref }),
        ),
        pq_authority_verification_state_root: record_root(
            "fallback-pq-authority",
            &json!({ "reason": reason_ref }),
        ),
        pq_release_authority_quorum_replay_state_root: record_root(
            "fallback-pq-quorum-replay",
            &json!({ "reason": reason_ref }),
        ),
        custody_release_authority_spec_state_root: record_root(
            "fallback-custody-policy",
            &json!({ "reason": reason_ref }),
        ),
        authority_crosscheck_state_root: record_root(
            "fallback-authority-crosscheck",
            &json!({ "reason": reason_ref }),
        ),
        security_audit_manifest_state_root: record_root(
            "fallback-security-audit",
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
        go_no_go_matrix_root: record_root("fallback-go-no-go", &json!({ "reason": reason_ref })),
        source_root: record_root("fallback-source", &json!({ "reason": reason_ref })),
    };
    let governance_conditions = Vec::new();
    let verdict = ReleaseGovernanceVerdict::fallback(&config, reason_ref);
    let condition_root = condition_vector_root(&governance_conditions);
    let reviewer_receipt_closure_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::ReviewerReceiptClosure,
    );
    let live_runtime_closure_root =
        domain_condition_root(&governance_conditions, GovernanceDomain::LiveRuntimeClosure);
    let human_audit_closure_root =
        domain_condition_root(&governance_conditions, GovernanceDomain::HumanAuditClosure);
    let pq_authority_quorum_root =
        domain_condition_root(&governance_conditions, GovernanceDomain::PqAuthorityQuorum);
    let custody_policy_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::CustodyReleasePolicy,
    );
    let authority_crosscheck_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::AuthorityCrosscheck,
    );
    let final_release_gate_root =
        domain_condition_root(&governance_conditions, GovernanceDomain::FinalReleaseGate);
    let execution_gate_plan_root =
        domain_condition_root(&governance_conditions, GovernanceDomain::ExecutionGatePlan);
    let user_forced_exit_priority_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::UserForcedExitPriority,
    );
    let privacy_redaction_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::PrivacyRedactionBoundary,
    );
    let emergency_timelock_root = domain_condition_root(
        &governance_conditions,
        GovernanceDomain::EmergencyPauseTimelock,
    );
    let release_hold_root =
        aggregate_release_hold_root(&config, &source_roots, &governance_conditions, &verdict);
    let governance_root = governance_root(
        &config,
        &source_roots,
        &condition_root,
        &release_hold_root,
        &verdict,
    );

    State {
        config,
        source_roots,
        governance_conditions,
        verdict,
        condition_root,
        reviewer_receipt_closure_root,
        live_runtime_closure_root,
        human_audit_closure_root,
        pq_authority_quorum_root,
        custody_policy_root,
        authority_crosscheck_root,
        final_release_gate_root,
        execution_gate_plan_root,
        user_forced_exit_priority_root,
        privacy_redaction_root,
        emergency_timelock_root,
        release_hold_root,
        governance_root,
    }
}
