use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageReleasePolicyAcceptedLiveEvidenceOperatorDashboardGoNoGoBindingRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_GO_NO_GO_BINDING_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-canonical-force-exit-release-policy-accepted-live-evidence-operator-dashboard-go-no-go-binding-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RELEASE_POLICY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_GO_NO_GO_BINDING_RUNTIME_PROTOCOL_VERSION;
pub const DEFAULT_HEIGHT: u64 = 94_000;
pub const DEFAULT_MAX_BINDING_AGE_BLOCKS: u64 = 72;
pub const DEFAULT_MIN_COORDINATOR_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_LANE_SCORE: u64 = 92;
pub const DEFAULT_MIN_PRIVACY_SCORE: u64 = 94;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyLane {
    CompileRuntime,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl ReleasePolicyLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileRuntime,
            Self::RuntimeReplay,
            Self::AuditSecurity,
            Self::BridgeCustody,
            Self::WalletWatchtower,
            Self::PqReservePrivacy,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile_runtime",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn binding_module(self) -> &'static str {
        match self {
            Self::CompileRuntime => "compile_runtime_operator_dashboard_release_policy_binding",
            Self::RuntimeReplay => "runtime_replay_operator_dashboard_release_policy_binding",
            Self::AuditSecurity => "audit_security_operator_dashboard_release_policy_binding",
            Self::BridgeCustody => "bridge_custody_operator_dashboard_release_policy_binding",
            Self::WalletWatchtower => "wallet_watchtower_operator_dashboard_release_policy_binding",
            Self::PqReservePrivacy => {
                "pq_reserve_privacy_operator_dashboard_release_policy_binding"
            }
        }
    }

    pub fn requires_private_boundary(self) -> bool {
        matches!(
            self,
            Self::AuditSecurity | Self::WalletWatchtower | Self::PqReservePrivacy
        )
    }

    pub fn required_controls(self) -> Vec<&'static str> {
        match self {
            Self::CompileRuntime => vec![
                "rustfmt_receipt_root",
                "cargo_check_deferred_marker",
                "test_gate_deferred_marker",
                "clippy_gate_deferred_marker",
                "operator_compile_ack_root",
            ],
            Self::RuntimeReplay => vec![
                "replay_command_root",
                "expected_receipt_root",
                "observed_receipt_root",
                "mismatch_blocker_root",
                "operator_replay_ack_root",
            ],
            Self::AuditSecurity => vec![
                "audit_closure_root",
                "finding_disposition_root",
                "privacy_review_root",
                "reviewer_signoff_root",
                "operator_audit_ack_root",
            ],
            Self::BridgeCustody => vec![
                "custody_signer_root",
                "monero_release_observation_root",
                "reserve_handoff_root",
                "challenge_window_clearance_root",
                "operator_custody_ack_root",
            ],
            Self::WalletWatchtower => vec![
                "wallet_scan_root",
                "watchtower_replay_root",
                "user_escape_action_root",
                "freshness_window_root",
                "operator_wallet_ack_root",
            ],
            Self::PqReservePrivacy => vec![
                "pq_quorum_root",
                "reserve_coverage_root",
                "privacy_boundary_root",
                "rotation_epoch_root",
                "operator_pq_ack_root",
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingStatus {
    Missing,
    Draft,
    PendingReview,
    Accepted,
    Held,
    Rejected,
    Expired,
}

impl BindingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Draft => "draft",
            Self::PendingReview => "pending_review",
            Self::Accepted => "accepted",
            Self::Held => "held",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn allows_release(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinatorDecision {
    Pending,
    Approve,
    Hold,
    Reject,
}

impl CoordinatorDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approve => "approve",
            Self::Hold => "hold",
            Self::Reject => "reject",
        }
    }

    pub fn approving(self) -> bool {
        matches!(self, Self::Approve)
    }

    pub fn blocking(self) -> bool {
        matches!(self, Self::Hold | Self::Reject)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GoNoGoBlockerKind {
    MissingLaneBinding,
    DuplicateLaneBinding,
    StaleLaneBinding,
    BindingNotAccepted,
    MissingDashboardRoot,
    MissingRunbookRoot,
    MissingImportedEvidenceRoot,
    MissingControlRoot,
    MissingPrivateBoundaryRoot,
    LaneScoreTooLow,
    PrivacyScoreTooLow,
    CoordinatorQuorumMissing,
    CoordinatorRejected,
    CoordinatorHold,
    EmergencyHoldActive,
    ReleaseManifestMismatch,
    CircuitBreakerNotCleared,
}

impl GoNoGoBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingLaneBinding => "missing_lane_binding",
            Self::DuplicateLaneBinding => "duplicate_lane_binding",
            Self::StaleLaneBinding => "stale_lane_binding",
            Self::BindingNotAccepted => "binding_not_accepted",
            Self::MissingDashboardRoot => "missing_dashboard_root",
            Self::MissingRunbookRoot => "missing_runbook_root",
            Self::MissingImportedEvidenceRoot => "missing_imported_evidence_root",
            Self::MissingControlRoot => "missing_control_root",
            Self::MissingPrivateBoundaryRoot => "missing_private_boundary_root",
            Self::LaneScoreTooLow => "lane_score_too_low",
            Self::PrivacyScoreTooLow => "privacy_score_too_low",
            Self::CoordinatorQuorumMissing => "coordinator_quorum_missing",
            Self::CoordinatorRejected => "coordinator_rejected",
            Self::CoordinatorHold => "coordinator_hold",
            Self::EmergencyHoldActive => "emergency_hold_active",
            Self::ReleaseManifestMismatch => "release_manifest_mismatch",
            Self::CircuitBreakerNotCleared => "circuit_breaker_not_cleared",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub release_policy_id: String,
    pub required_lanes: Vec<ReleasePolicyLane>,
    pub max_binding_age_blocks: u64,
    pub min_coordinator_weight: u64,
    pub min_lane_score: u64,
    pub min_privacy_score: u64,
    pub require_dashboard_root: bool,
    pub require_runbook_root: bool,
    pub require_imported_evidence_root: bool,
    pub require_private_boundary_roots: bool,
    pub require_release_manifest_match: bool,
    pub require_circuit_breaker_clearance: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            release_policy_id: release_policy_id("devnet-operator-dashboard-go-no-go-binding"),
            required_lanes: ReleasePolicyLane::all(),
            max_binding_age_blocks: DEFAULT_MAX_BINDING_AGE_BLOCKS,
            min_coordinator_weight: DEFAULT_MIN_COORDINATOR_WEIGHT,
            min_lane_score: DEFAULT_MIN_LANE_SCORE,
            min_privacy_score: DEFAULT_MIN_PRIVACY_SCORE,
            require_dashboard_root: true,
            require_runbook_root: true,
            require_imported_evidence_root: true,
            require_private_boundary_roots: true,
            require_release_manifest_match: true,
            require_circuit_breaker_clearance: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("release_policy_id", &self.release_policy_id)?;
        ensure(
            !self.required_lanes.is_empty(),
            "at least one release-policy lane is required",
        )?;
        ensure(
            self.max_binding_age_blocks > 0,
            "binding age window must be non-zero",
        )?;
        ensure(
            self.min_coordinator_weight > 0,
            "coordinator weight must be non-zero",
        )?;
        ensure(
            self.min_lane_score > 0,
            "lane score threshold must be non-zero",
        )?;
        ensure(
            self.min_privacy_score > 0,
            "privacy score threshold must be non-zero",
        )?;
        let mut seen = BTreeSet::new();
        for lane in &self.required_lanes {
            ensure(seen.insert(*lane), "duplicate required release-policy lane")?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "release_policy_id": self.release_policy_id,
            "required_lanes": self.required_lanes.iter().map(|lane| lane.as_str()).collect::<Vec<_>>(),
            "max_binding_age_blocks": self.max_binding_age_blocks,
            "min_coordinator_weight": self.min_coordinator_weight,
            "min_lane_score": self.min_lane_score,
            "min_privacy_score": self.min_privacy_score,
            "require_dashboard_root": self.require_dashboard_root,
            "require_runbook_root": self.require_runbook_root,
            "require_imported_evidence_root": self.require_imported_evidence_root,
            "require_private_boundary_roots": self.require_private_boundary_roots,
            "require_release_manifest_match": self.require_release_manifest_match,
            "require_circuit_breaker_clearance": self.require_circuit_breaker_clearance,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "release-policy-dashboard-go-no-go-config",
            &[
                HashPart::Str(&self.release_policy_id),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LaneBinding {
    pub lane: ReleasePolicyLane,
    pub binding_id: String,
    pub operator_dashboard_root: String,
    pub runbook_audit_root: String,
    pub imported_evidence_root: String,
    pub release_manifest_root: String,
    pub circuit_breaker_root: String,
    pub private_boundary_root: String,
    pub required_control_roots: BTreeMap<String, String>,
    pub status: BindingStatus,
    pub lane_score: u64,
    pub privacy_score: u64,
    pub observed_at_height: u64,
    pub reviewer_weight: u64,
    pub operator_acknowledgement_root: String,
    pub policy_note: String,
}

impl LaneBinding {
    pub fn new(
        lane: ReleasePolicyLane,
        binding_id: impl Into<String>,
        observed_at_height: u64,
    ) -> Self {
        let binding_id = binding_id.into();
        let required_control_roots = lane
            .required_controls()
            .into_iter()
            .map(|control| {
                (
                    control.to_string(),
                    control_root(lane, control, observed_at_height),
                )
            })
            .collect::<BTreeMap<_, _>>();
        Self {
            lane,
            binding_id: binding_id.clone(),
            operator_dashboard_root: lane_root(lane, &binding_id, "operator-dashboard"),
            runbook_audit_root: lane_root(lane, &binding_id, "runbook-audit"),
            imported_evidence_root: lane_root(lane, &binding_id, "accepted-import"),
            release_manifest_root: lane_root(lane, &binding_id, "release-manifest"),
            circuit_breaker_root: lane_root(lane, &binding_id, "circuit-breaker-clear"),
            private_boundary_root: lane_root(lane, &binding_id, "private-boundary"),
            required_control_roots,
            status: BindingStatus::Accepted,
            lane_score: DEFAULT_MIN_LANE_SCORE + 2,
            privacy_score: DEFAULT_MIN_PRIVACY_SCORE + 1,
            observed_at_height,
            reviewer_weight: 25,
            operator_acknowledgement_root: lane_root(lane, &binding_id, "operator-ack"),
            policy_note: format!(
                "{} release-policy binding accepts dashboard evidence while release remains gated by coordinator quorum",
                lane.as_str()
            ),
        }
    }

    pub fn with_status(mut self, status: BindingStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_scores(mut self, lane_score: u64, privacy_score: u64) -> Self {
        self.lane_score = lane_score;
        self.privacy_score = privacy_score;
        self
    }

    pub fn stale(&self, height: u64, max_age: u64) -> bool {
        height.saturating_sub(self.observed_at_height) > max_age
    }

    pub fn missing_required_roots(&self, config: &Config) -> Vec<GoNoGoBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_dashboard_root && self.operator_dashboard_root.is_empty() {
            blockers.push(GoNoGoBlockerKind::MissingDashboardRoot);
        }
        if config.require_runbook_root && self.runbook_audit_root.is_empty() {
            blockers.push(GoNoGoBlockerKind::MissingRunbookRoot);
        }
        if config.require_imported_evidence_root && self.imported_evidence_root.is_empty() {
            blockers.push(GoNoGoBlockerKind::MissingImportedEvidenceRoot);
        }
        if config.require_release_manifest_match && self.release_manifest_root.is_empty() {
            blockers.push(GoNoGoBlockerKind::ReleaseManifestMismatch);
        }
        if config.require_circuit_breaker_clearance && self.circuit_breaker_root.is_empty() {
            blockers.push(GoNoGoBlockerKind::CircuitBreakerNotCleared);
        }
        if config.require_private_boundary_roots
            && self.lane.requires_private_boundary()
            && self.private_boundary_root.is_empty()
        {
            blockers.push(GoNoGoBlockerKind::MissingPrivateBoundaryRoot);
        }
        for control in self.lane.required_controls() {
            match self.required_control_roots.get(control) {
                Some(root) if !root.is_empty() => {}
                _ => blockers.push(GoNoGoBlockerKind::MissingControlRoot),
            }
        }
        blockers
    }

    pub fn policy_blockers(&self, config: &Config, height: u64) -> Vec<GoNoGoBlockerKind> {
        let mut blockers = self.missing_required_roots(config);
        if self.stale(height, config.max_binding_age_blocks) {
            blockers.push(GoNoGoBlockerKind::StaleLaneBinding);
        }
        if !self.status.allows_release() {
            blockers.push(GoNoGoBlockerKind::BindingNotAccepted);
        }
        if self.lane_score < config.min_lane_score {
            blockers.push(GoNoGoBlockerKind::LaneScoreTooLow);
        }
        if self.lane.requires_private_boundary() && self.privacy_score < config.min_privacy_score {
            blockers.push(GoNoGoBlockerKind::PrivacyScoreTooLow);
        }
        blockers
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane.as_str(),
            "binding_module": self.lane.binding_module(),
            "binding_id": self.binding_id,
            "operator_dashboard_root": self.operator_dashboard_root,
            "runbook_audit_root": self.runbook_audit_root,
            "imported_evidence_root": self.imported_evidence_root,
            "release_manifest_root": self.release_manifest_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "private_boundary_root": self.private_boundary_root,
            "required_control_roots": self.required_control_roots,
            "status": self.status.as_str(),
            "lane_score": self.lane_score,
            "privacy_score": self.privacy_score,
            "observed_at_height": self.observed_at_height,
            "reviewer_weight": self.reviewer_weight,
            "operator_acknowledgement_root": self.operator_acknowledgement_root,
            "policy_note": self.policy_note,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "release-policy-dashboard-go-no-go-lane-binding",
            &[
                HashPart::Str(self.lane.as_str()),
                HashPart::Str(&self.binding_id),
                HashPart::U64(self.observed_at_height),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoordinatorApproval {
    pub coordinator_id: String,
    pub decision: CoordinatorDecision,
    pub approval_weight: u64,
    pub dashboard_snapshot_root: String,
    pub release_policy_root: String,
    pub signed_at_height: u64,
    pub note: String,
}

impl CoordinatorApproval {
    pub fn approving(
        coordinator_id: impl Into<String>,
        weight: u64,
        signed_at_height: u64,
    ) -> Self {
        let coordinator_id = coordinator_id.into();
        let dashboard_snapshot_root =
            coordinator_root(&coordinator_id, signed_at_height, "dashboard-snapshot");
        let release_policy_root =
            coordinator_root(&coordinator_id, signed_at_height, "release-policy");
        Self {
            coordinator_id: coordinator_id.clone(),
            decision: CoordinatorDecision::Approve,
            approval_weight: weight,
            dashboard_snapshot_root,
            release_policy_root,
            signed_at_height,
            note: format!(
                "{coordinator_id} approves dashboard-bound release policy with production release still subject to complete lane roots"
            ),
        }
    }

    pub fn hold(coordinator_id: impl Into<String>, signed_at_height: u64) -> Self {
        let coordinator_id = coordinator_id.into();
        Self {
            coordinator_id: coordinator_id.clone(),
            decision: CoordinatorDecision::Hold,
            approval_weight: 0,
            dashboard_snapshot_root: coordinator_root(
                &coordinator_id,
                signed_at_height,
                "dashboard-hold",
            ),
            release_policy_root: coordinator_root(
                &coordinator_id,
                signed_at_height,
                "release-policy-hold",
            ),
            signed_at_height,
            note: format!("{coordinator_id} holds release pending live heavy-gate evidence"),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "coordinator_id": self.coordinator_id,
            "decision": self.decision.as_str(),
            "approval_weight": self.approval_weight,
            "dashboard_snapshot_root": self.dashboard_snapshot_root,
            "release_policy_root": self.release_policy_root,
            "signed_at_height": self.signed_at_height,
            "note": self.note,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "release-policy-dashboard-go-no-go-coordinator",
            &[
                HashPart::Str(&self.coordinator_id),
                HashPart::Str(self.decision.as_str()),
                HashPart::U64(self.approval_weight),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmergencyHold {
    pub hold_id: String,
    pub active: bool,
    pub lane: Option<ReleasePolicyLane>,
    pub reason: String,
    pub evidence_root: String,
    pub opened_at_height: u64,
}

impl EmergencyHold {
    pub fn inactive(hold_id: impl Into<String>, opened_at_height: u64) -> Self {
        let hold_id = hold_id.into();
        Self {
            hold_id: hold_id.clone(),
            active: false,
            lane: None,
            reason: "no active emergency hold in dashboard-bound go-no-go policy".to_string(),
            evidence_root: domain_hash(
                "release-policy-dashboard-go-no-go-inactive-hold",
                &[HashPart::Str(&hold_id), HashPart::U64(opened_at_height)],
                32,
            ),
            opened_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "hold_id": self.hold_id,
            "active": self.active,
            "lane": self.lane.map(|lane| lane.as_str()),
            "reason": self.reason,
            "evidence_root": self.evidence_root,
            "opened_at_height": self.opened_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "release-policy-dashboard-go-no-go-emergency-hold",
            &[
                HashPart::Str(&self.hold_id),
                HashPart::U64(self.opened_at_height),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoNoGoDecision {
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub lane_count: usize,
    pub accepted_lane_count: usize,
    pub coordinator_weight: u64,
    pub blocker_count: usize,
    pub decision_root: String,
    pub blocker_root: String,
    pub lane_root: String,
    pub coordinator_root: String,
}

impl GoNoGoDecision {
    pub fn public_record(&self) -> Value {
        json!({
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "lane_count": self.lane_count,
            "accepted_lane_count": self.accepted_lane_count,
            "coordinator_weight": self.coordinator_weight,
            "blocker_count": self.blocker_count,
            "decision_root": self.decision_root,
            "blocker_root": self.blocker_root,
            "lane_root": self.lane_root,
            "coordinator_root": self.coordinator_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub lane_bindings: Vec<LaneBinding>,
    pub coordinator_approvals: Vec<CoordinatorApproval>,
    pub emergency_holds: Vec<EmergencyHold>,
    pub blockers: BTreeMap<String, Vec<GoNoGoBlockerKind>>,
    pub lane_binding_root: String,
    pub coordinator_approval_root: String,
    pub emergency_hold_root: String,
    pub blocker_root: String,
    pub decision: GoNoGoDecision,
}

impl State {
    pub fn new(
        config: Config,
        height: u64,
        lane_bindings: Vec<LaneBinding>,
        coordinator_approvals: Vec<CoordinatorApproval>,
        emergency_holds: Vec<EmergencyHold>,
    ) -> Result<Self> {
        config.validate()?;
        let blockers = evaluate_blockers(
            &config,
            height,
            &lane_bindings,
            &coordinator_approvals,
            &emergency_holds,
        );
        let lane_binding_root = merkle_root(
            "release-policy-dashboard-go-no-go-lane-bindings",
            &lane_bindings
                .iter()
                .map(LaneBinding::public_record)
                .collect::<Vec<_>>(),
        );
        let coordinator_approval_root = merkle_root(
            "release-policy-dashboard-go-no-go-coordinator-approvals",
            &coordinator_approvals
                .iter()
                .map(CoordinatorApproval::public_record)
                .collect::<Vec<_>>(),
        );
        let emergency_hold_root = merkle_root(
            "release-policy-dashboard-go-no-go-emergency-holds",
            &emergency_holds
                .iter()
                .map(EmergencyHold::public_record)
                .collect::<Vec<_>>(),
        );
        let blocker_root = merkle_root(
            "release-policy-dashboard-go-no-go-blockers",
            &blockers
                .iter()
                .map(|(key, blockers)| {
                    json!({
                        "subject": key,
                        "blockers": blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                    })
                })
                .collect::<Vec<_>>(),
        );
        let accepted_lane_count = lane_bindings
            .iter()
            .filter(|binding| binding.status.allows_release())
            .count();
        let coordinator_weight = coordinator_approvals
            .iter()
            .filter(|approval| approval.decision.approving())
            .map(|approval| approval.approval_weight)
            .sum::<u64>();
        let blocker_count = blockers.values().map(Vec::len).sum::<usize>();
        let release_allowed = blocker_count == 0
            && accepted_lane_count == config.required_lanes.len()
            && coordinator_weight >= config.min_coordinator_weight;
        let decision_root = domain_hash(
            "release-policy-dashboard-go-no-go-decision",
            &[
                HashPart::Str(&config.release_policy_id),
                HashPart::U64(height),
                HashPart::U64(accepted_lane_count as u64),
                HashPart::U64(coordinator_weight),
                HashPart::U64(blocker_count as u64),
                HashPart::Str(&lane_binding_root),
                HashPart::Str(&coordinator_approval_root),
                HashPart::Str(&emergency_hold_root),
                HashPart::Str(&blocker_root),
            ],
            32,
        );
        let decision = GoNoGoDecision {
            release_allowed,
            fail_closed: config.fail_closed && !release_allowed,
            lane_count: lane_bindings.len(),
            accepted_lane_count,
            coordinator_weight,
            blocker_count,
            decision_root,
            blocker_root: blocker_root.clone(),
            lane_root: lane_binding_root.clone(),
            coordinator_root: coordinator_approval_root.clone(),
        };
        Ok(Self {
            config,
            height,
            lane_bindings,
            coordinator_approvals,
            emergency_holds,
            blockers,
            lane_binding_root,
            coordinator_approval_root,
            emergency_hold_root,
            blocker_root,
            decision,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let height = DEFAULT_HEIGHT;
        let lane_bindings = ReleasePolicyLane::all()
            .into_iter()
            .enumerate()
            .map(|(index, lane)| {
                LaneBinding::new(
                    lane,
                    format!("wave83-{}-dashboard-policy-binding", lane.as_str()),
                    height.saturating_sub(6 + index as u64),
                )
            })
            .collect::<Vec<_>>();
        let coordinator_approvals = vec![
            CoordinatorApproval::approving("release-coordinator-alpha", 34, height),
            CoordinatorApproval::approving("release-coordinator-beta", 34, height),
            CoordinatorApproval::approving("release-coordinator-gamma", 17, height),
        ];
        let emergency_holds = vec![EmergencyHold::inactive("wave83-dashboard-go-no-go", height)];
        match Self::new(
            config,
            height,
            lane_bindings,
            coordinator_approvals,
            emergency_holds,
        ) {
            Ok(state) => state,
            Err(_) => Self::fallback(),
        }
    }

    pub fn fallback() -> Self {
        let config = Config {
            release_policy_id: "fallback-release-policy-dashboard-go-no-go".to_string(),
            required_lanes: vec![ReleasePolicyLane::CompileRuntime],
            max_binding_age_blocks: DEFAULT_MAX_BINDING_AGE_BLOCKS,
            min_coordinator_weight: DEFAULT_MIN_COORDINATOR_WEIGHT,
            min_lane_score: DEFAULT_MIN_LANE_SCORE,
            min_privacy_score: DEFAULT_MIN_PRIVACY_SCORE,
            require_dashboard_root: true,
            require_runbook_root: true,
            require_imported_evidence_root: true,
            require_private_boundary_roots: true,
            require_release_manifest_match: true,
            require_circuit_breaker_clearance: true,
            fail_closed: true,
        };
        let lane = LaneBinding::new(
            ReleasePolicyLane::CompileRuntime,
            "fallback-compile-runtime-binding",
            DEFAULT_HEIGHT,
        )
        .with_status(BindingStatus::Held);
        let lane_binding_root = merkle_root(
            "release-policy-dashboard-go-no-go-fallback-lanes",
            &[lane.public_record()],
        );
        let coordinator_approval_root = merkle_root(
            "release-policy-dashboard-go-no-go-fallback-coordinators",
            &[CoordinatorApproval::hold("fallback-coordinator", DEFAULT_HEIGHT).public_record()],
        );
        let emergency_hold =
            EmergencyHold::inactive("fallback-release-policy-dashboard-hold", DEFAULT_HEIGHT);
        let emergency_hold_root = merkle_root(
            "release-policy-dashboard-go-no-go-fallback-holds",
            &[emergency_hold.public_record()],
        );
        let mut blockers = BTreeMap::new();
        blockers.insert(
            ReleasePolicyLane::CompileRuntime.as_str().to_string(),
            vec![GoNoGoBlockerKind::BindingNotAccepted],
        );
        let blocker_root = merkle_root(
            "release-policy-dashboard-go-no-go-fallback-blockers",
            &[json!({
                "subject": ReleasePolicyLane::CompileRuntime.as_str(),
                "blockers": ["binding_not_accepted"],
            })],
        );
        let decision_root = domain_hash(
            "release-policy-dashboard-go-no-go-fallback-decision",
            &[
                HashPart::Str(&config.release_policy_id),
                HashPart::Str(&lane_binding_root),
                HashPart::Str(&coordinator_approval_root),
                HashPart::Str(&emergency_hold_root),
                HashPart::Str(&blocker_root),
            ],
            32,
        );
        Self {
            config,
            height: DEFAULT_HEIGHT,
            lane_bindings: vec![lane],
            coordinator_approvals: vec![CoordinatorApproval::hold(
                "fallback-coordinator",
                DEFAULT_HEIGHT,
            )],
            emergency_holds: vec![emergency_hold],
            blockers,
            lane_binding_root: lane_binding_root.clone(),
            coordinator_approval_root: coordinator_approval_root.clone(),
            emergency_hold_root,
            blocker_root: blocker_root.clone(),
            decision: GoNoGoDecision {
                release_allowed: false,
                fail_closed: true,
                lane_count: 1,
                accepted_lane_count: 0,
                coordinator_weight: 0,
                blocker_count: 1,
                decision_root,
                blocker_root,
                lane_root: lane_binding_root,
                coordinator_root: coordinator_approval_root,
            },
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "height": self.height,
            "config": self.config.public_record(),
            "lane_binding_root": self.lane_binding_root,
            "coordinator_approval_root": self.coordinator_approval_root,
            "emergency_hold_root": self.emergency_hold_root,
            "blocker_root": self.blocker_root,
            "decision": self.decision.public_record(),
            "lane_bindings": self.lane_bindings.iter().map(LaneBinding::public_record).collect::<Vec<_>>(),
            "coordinator_approvals": self.coordinator_approvals.iter().map(CoordinatorApproval::public_record).collect::<Vec<_>>(),
            "emergency_holds": self.emergency_holds.iter().map(EmergencyHold::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(|(subject, blockers)| {
                json!({
                    "subject": subject,
                    "blockers": blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                })
            }).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "release-policy-dashboard-go-no-go-state",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.release_policy_id),
                HashPart::U64(self.height),
                HashPart::Str(&self.lane_binding_root),
                HashPart::Str(&self.coordinator_approval_root),
                HashPart::Str(&self.emergency_hold_root),
                HashPart::Str(&self.blocker_root),
                HashPart::Str(&self.decision.decision_root),
                HashPart::Json(&self.public_record()),
            ],
            32,
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

fn evaluate_blockers(
    config: &Config,
    height: u64,
    lane_bindings: &[LaneBinding],
    coordinator_approvals: &[CoordinatorApproval],
    emergency_holds: &[EmergencyHold],
) -> BTreeMap<String, Vec<GoNoGoBlockerKind>> {
    let mut blockers = BTreeMap::<String, Vec<GoNoGoBlockerKind>>::new();
    let mut lanes_seen = BTreeSet::new();
    for binding in lane_bindings {
        let key = binding.lane.as_str().to_string();
        if !lanes_seen.insert(binding.lane) {
            blockers
                .entry(key.clone())
                .or_default()
                .push(GoNoGoBlockerKind::DuplicateLaneBinding);
        }
        let lane_blockers = binding.policy_blockers(config, height);
        if !lane_blockers.is_empty() {
            blockers.entry(key).or_default().extend(lane_blockers);
        }
    }
    for lane in &config.required_lanes {
        if !lanes_seen.contains(lane) {
            blockers
                .entry(lane.as_str().to_string())
                .or_default()
                .push(GoNoGoBlockerKind::MissingLaneBinding);
        }
    }
    let approval_weight = coordinator_approvals
        .iter()
        .filter(|approval| approval.decision.approving())
        .map(|approval| approval.approval_weight)
        .sum::<u64>();
    if approval_weight < config.min_coordinator_weight {
        blockers
            .entry("coordinator_quorum".to_string())
            .or_default()
            .push(GoNoGoBlockerKind::CoordinatorQuorumMissing);
    }
    for approval in coordinator_approvals {
        if approval.decision.blocking() {
            let blocker = match approval.decision {
                CoordinatorDecision::Reject => GoNoGoBlockerKind::CoordinatorRejected,
                CoordinatorDecision::Hold => GoNoGoBlockerKind::CoordinatorHold,
                CoordinatorDecision::Pending | CoordinatorDecision::Approve => {
                    GoNoGoBlockerKind::CoordinatorQuorumMissing
                }
            };
            blockers
                .entry(approval.coordinator_id.clone())
                .or_default()
                .push(blocker);
        }
    }
    for hold in emergency_holds {
        if hold.active {
            let key = hold
                .lane
                .map(|lane| lane.as_str().to_string())
                .unwrap_or_else(|| hold.hold_id.clone());
            blockers
                .entry(key)
                .or_default()
                .push(GoNoGoBlockerKind::EmergencyHoldActive);
        }
    }
    blockers
}

fn release_policy_id(label: &str) -> String {
    domain_hash(
        "release-policy-dashboard-go-no-go-id",
        &[HashPart::Str(CHAIN_ID), HashPart::Str(label)],
        16,
    )
}

fn lane_root(lane: ReleasePolicyLane, binding_id: &str, domain: &str) -> String {
    domain_hash(
        "release-policy-dashboard-go-no-go-lane-root",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(binding_id),
            HashPart::Str(domain),
        ],
        32,
    )
}

fn control_root(lane: ReleasePolicyLane, control: &str, observed_at_height: u64) -> String {
    domain_hash(
        "release-policy-dashboard-go-no-go-control-root",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(control),
            HashPart::U64(observed_at_height),
        ],
        32,
    )
}

fn coordinator_root(coordinator_id: &str, height: u64, domain: &str) -> String {
    domain_hash(
        "release-policy-dashboard-go-no-go-coordinator-root",
        &[
            HashPart::Str(coordinator_id),
            HashPart::U64(height),
            HashPart::Str(domain),
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

fn ensure_non_empty(name: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{name} must not be empty"),
    )
}
