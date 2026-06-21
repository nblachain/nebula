use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-pq-reserve-privacy-lane-runtime-v1";
const WAVE: u64 = 103;
const RECONCILIATION_WAVE: u64 = 102;
const MIN_QUARANTINE_CONFIRMATIONS: u64 = 720;
const MAX_FEE_CAP_BPS: u64 = 8;
const MIN_DECOY_SET_SIZE: u64 = 16;
const LANE_ID: &str =
    "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-pq-reserve-privacy";

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, BroadcastQuarantineError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineError {
    LaneMissing,
    BroadcastPlanMissing,
    Wave102ReconciliationRootMissing,
    ReleaseTransactionPlanRootMissing,
    ReserveAuthorizationRootMissing,
    PrivacyBudgetRootMissing,
    FeeCapRootMissing,
    DecoyPrivacyRootMissing,
    HybridPqAuthorizationRootMissing,
    MempoolQuarantineRootMissing,
    CircuitBreakerRootMissing,
    LiveHeavyGateEvidenceRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    QuarantineConfirmationsTooLow,
    FeeCapBpsTooHigh,
    DecoySetSizeTooLow,
    BroadcastDisabled,
    BroadcastStillBlocked,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "PQ reserve privacy broadcast quarantine guard",
        }
    }

    pub fn command_scope(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq-reserve-privacy-broadcast-quarantine",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastStatus {
    Empty,
    Blocked,
    Planned,
    Quarantined,
    Reviewed,
    Releasable,
}

impl BroadcastStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Blocked => "blocked",
            Self::Planned => "planned",
            Self::Quarantined => "quarantined",
            Self::Reviewed => "reviewed",
            Self::Releasable => "releasable",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastBlockerKind {
    MissingWave102ReconciliationRoot,
    MissingReleaseTransactionPlanRoot,
    MissingReserveAuthorizationRoot,
    MissingPrivacyBudgetRoot,
    MissingFeeCapRoot,
    MissingDecoyPrivacyRoot,
    MissingHybridPqAuthorizationRoot,
    MissingMempoolQuarantineRoot,
    MissingCircuitBreakerRoot,
    MissingLiveHeavyGateEvidenceRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    QuarantineConfirmationsTooLow,
    FeeCapBpsTooHigh,
    DecoySetSizeTooLow,
    BroadcastDisabled,
    BroadcastDenied,
    RootsOnlyBoundary,
}

impl BroadcastBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave102ReconciliationRoot => "missing_wave102_reconciliation_root",
            Self::MissingReleaseTransactionPlanRoot => "missing_release_transaction_plan_root",
            Self::MissingReserveAuthorizationRoot => "missing_reserve_authorization_root",
            Self::MissingPrivacyBudgetRoot => "missing_privacy_budget_root",
            Self::MissingFeeCapRoot => "missing_fee_cap_root",
            Self::MissingDecoyPrivacyRoot => "missing_decoy_privacy_root",
            Self::MissingHybridPqAuthorizationRoot => {
                "missing_hybrid_ml_dsa_slh_dsa_pq_authorization_root"
            }
            Self::MissingMempoolQuarantineRoot => "missing_mempool_quarantine_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingLiveHeavyGateEvidenceRoot => "missing_live_heavy_gate_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::QuarantineConfirmationsTooLow => "quarantine_confirmations_too_low",
            Self::FeeCapBpsTooHigh => "fee_cap_bps_too_high",
            Self::DecoySetSizeTooLow => "decoy_set_size_too_low",
            Self::BroadcastDisabled => "broadcast_disabled",
            Self::BroadcastDenied => "broadcast_denied",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub reconciliation_wave: u64,
    pub lane_id: String,
    pub active_lane: String,
    pub min_quarantine_confirmations: u64,
    pub max_fee_cap_bps: u64,
    pub min_decoy_set_size: u64,
    pub require_wave102_reconciliation_root: bool,
    pub require_release_transaction_plan_root: bool,
    pub require_reserve_authorization_root: bool,
    pub require_privacy_budget_root: bool,
    pub require_fee_cap_root: bool,
    pub require_decoy_privacy_root: bool,
    pub require_hybrid_pq_authorization_root: bool,
    pub require_mempool_quarantine_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub deny_broadcast_when_any_blocker_active: bool,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            reconciliation_wave: RECONCILIATION_WAVE,
            lane_id: LANE_ID.to_string(),
            active_lane: LaneKind::PqReservePrivacy.as_str().to_string(),
            min_quarantine_confirmations: MIN_QUARANTINE_CONFIRMATIONS,
            max_fee_cap_bps: MAX_FEE_CAP_BPS,
            min_decoy_set_size: MIN_DECOY_SET_SIZE,
            require_wave102_reconciliation_root: true,
            require_release_transaction_plan_root: true,
            require_reserve_authorization_root: true,
            require_privacy_budget_root: true,
            require_fee_cap_root: true,
            require_decoy_privacy_root: true,
            require_hybrid_pq_authorization_root: true,
            require_mempool_quarantine_root: true,
            require_circuit_breaker_root: true,
            require_live_heavy_gate_evidence: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            deny_broadcast_when_any_blocker_active: true,
            broadcast_allowed: false,
            broadcast_disabled: true,
            heavy_gates_ran: false,
            roots_only_public_records: true,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "wave": self.wave,
            "reconciliation_wave": self.reconciliation_wave,
            "lane_id": self.lane_id,
            "active_lane": self.active_lane,
            "min_quarantine_confirmations": self.min_quarantine_confirmations,
            "max_fee_cap_bps": self.max_fee_cap_bps,
            "min_decoy_set_size": self.min_decoy_set_size,
            "require_wave102_reconciliation_root": self.require_wave102_reconciliation_root,
            "require_release_transaction_plan_root": self.require_release_transaction_plan_root,
            "require_reserve_authorization_root": self.require_reserve_authorization_root,
            "require_privacy_budget_root": self.require_privacy_budget_root,
            "require_fee_cap_root": self.require_fee_cap_root,
            "require_decoy_privacy_root": self.require_decoy_privacy_root,
            "require_hybrid_pq_authorization_root": self.require_hybrid_pq_authorization_root,
            "require_mempool_quarantine_root": self.require_mempool_quarantine_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "deny_broadcast_when_any_blocker_active": self.deny_broadcast_when_any_blocker_active,
            "broadcast_allowed": self.broadcast_allowed,
            "broadcast_disabled": self.broadcast_disabled,
            "heavy_gates_ran": self.heavy_gates_ran,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BroadcastRoots {
    pub wave102_reconciliation_root: Option<String>,
    pub release_transaction_plan_root: Option<String>,
    pub reserve_authorization_root: Option<String>,
    pub privacy_budget_root: Option<String>,
    pub fee_cap_root: Option<String>,
    pub decoy_privacy_root: Option<String>,
    pub hybrid_pq_authorization_root: Option<String>,
    pub mempool_quarantine_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub live_heavy_gate_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
}

impl BroadcastRoots {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave102_reconciliation_root": self.wave102_reconciliation_root,
            "release_transaction_plan_root": self.release_transaction_plan_root,
            "reserve_authorization_root": self.reserve_authorization_root,
            "privacy_budget_root": self.privacy_budget_root,
            "fee_cap_root": self.fee_cap_root,
            "decoy_privacy_root": self.decoy_privacy_root,
            "hybrid_ml_dsa_slh_dsa_pq_authorization_root": self.hybrid_pq_authorization_root,
            "mempool_quarantine_root": self.mempool_quarantine_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "live_heavy_gate_evidence_root": self.live_heavy_gate_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_roots", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BroadcastMeasurements {
    pub quarantine_confirmations: u64,
    pub fee_cap_bps: u64,
    pub decoy_set_size: u64,
}

impl BroadcastMeasurements {
    pub fn blocked(config: &Config) -> Self {
        Self {
            quarantine_confirmations: config.min_quarantine_confirmations.saturating_sub(1),
            fee_cap_bps: config.max_fee_cap_bps.saturating_add(1),
            decoy_set_size: config.min_decoy_set_size.saturating_sub(1),
        }
    }

    pub fn public_record(self) -> PublicRecord {
        json!({
            "quarantine_confirmations": self.quarantine_confirmations,
            "fee_cap_bps": self.fee_cap_bps,
            "decoy_set_size": self.decoy_set_size,
        })
    }

    pub fn state_root(self) -> String {
        record_root("broadcast_measurements", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastPolicy {
    pub lane: LaneKind,
    pub plan_label: String,
    pub ordinal: u64,
    pub command_scope: String,
    pub command_hint: String,
    pub quarantine_hold_root: String,
    pub reserve_policy_root: String,
    pub privacy_policy_root: String,
    pub pq_authorization_policy_root: String,
    pub mempool_policy_root: String,
}

impl BroadcastPolicy {
    pub fn new(lane: LaneKind, plan_label: &str, ordinal: u64) -> Self {
        let command_scope = lane.command_scope().to_string();
        let command_hint = format!(
            "nebula wave103 broadcast-quarantine --lane {} --plan {} --hold-broadcast",
            lane.as_str(),
            plan_label
        );
        let quarantine_hold_root =
            label_root("quarantine_hold", lane.as_str(), plan_label, ordinal);
        let reserve_policy_root = label_root("reserve_policy", lane.as_str(), plan_label, ordinal);
        let privacy_policy_root = label_root("privacy_policy", lane.as_str(), plan_label, ordinal);
        let pq_authorization_policy_root = label_root(
            "pq_authorization_policy",
            lane.as_str(),
            plan_label,
            ordinal,
        );
        let mempool_policy_root = label_root("mempool_policy", lane.as_str(), plan_label, ordinal);
        Self {
            lane,
            plan_label: plan_label.to_string(),
            ordinal,
            command_scope,
            command_hint,
            quarantine_hold_root,
            reserve_policy_root,
            privacy_policy_root,
            pq_authorization_policy_root,
            mempool_policy_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "plan_label": self.plan_label,
            "ordinal": self.ordinal,
            "command_scope": self.command_scope,
            "command_hint": self.command_hint,
            "quarantine_hold_root": self.quarantine_hold_root,
            "reserve_policy_root": self.reserve_policy_root,
            "privacy_policy_root": self.privacy_policy_root,
            "pq_authorization_policy_root": self.pq_authorization_policy_root,
            "mempool_policy_root": self.mempool_policy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastQuarantine {
    pub lane: LaneKind,
    pub plan_label: String,
    pub ordinal: u64,
    pub roots: BroadcastRoots,
    pub measurements: BroadcastMeasurements,
    pub policy: BroadcastPolicy,
    pub status: BroadcastStatus,
    pub blockers: Vec<BroadcastBlockerKind>,
    pub broadcast_allowed: bool,
}

impl BroadcastQuarantine {
    pub fn empty(lane: LaneKind, plan_label: &str, ordinal: u64, config: &Config) -> Self {
        let policy = BroadcastPolicy::new(lane, plan_label, ordinal);
        Self {
            lane,
            plan_label: plan_label.to_string(),
            ordinal,
            roots: BroadcastRoots::default(),
            measurements: BroadcastMeasurements::blocked(config),
            policy,
            status: BroadcastStatus::Blocked,
            blockers: initial_blockers(config),
            broadcast_allowed: false,
        }
    }

    pub fn stage_quarantine(
        mut self,
        roots: BroadcastRoots,
        measurements: BroadcastMeasurements,
        config: &Config,
    ) -> Self {
        self.roots = roots;
        self.measurements = measurements;
        self.blockers = self.active_blockers(config);
        self.status = if self.blockers.is_empty() {
            BroadcastStatus::Reviewed
        } else if self.roots.mempool_quarantine_root.is_some() {
            BroadcastStatus::Quarantined
        } else if self.roots.release_transaction_plan_root.is_some() {
            BroadcastStatus::Planned
        } else {
            BroadcastStatus::Blocked
        };
        self.broadcast_allowed = false;
        self
    }

    pub fn mark_releasable(mut self, config: &Config) -> Result<Self> {
        self.blockers = self.active_blockers(config);
        if self.blockers.is_empty() {
            self.status = BroadcastStatus::Releasable;
            self.broadcast_allowed = true;
            Ok(self)
        } else {
            Err(BroadcastQuarantineError::BroadcastStillBlocked)
        }
    }

    pub fn active_blockers(&self, config: &Config) -> Vec<BroadcastBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_wave102_reconciliation_root
            && self.roots.wave102_reconciliation_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingWave102ReconciliationRoot);
        }
        if config.require_release_transaction_plan_root
            && self.roots.release_transaction_plan_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingReleaseTransactionPlanRoot);
        }
        if config.require_reserve_authorization_root
            && self.roots.reserve_authorization_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingReserveAuthorizationRoot);
        }
        if config.require_privacy_budget_root && self.roots.privacy_budget_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingPrivacyBudgetRoot);
        }
        if config.require_fee_cap_root && self.roots.fee_cap_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingFeeCapRoot);
        }
        if config.require_decoy_privacy_root && self.roots.decoy_privacy_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingDecoyPrivacyRoot);
        }
        if config.require_hybrid_pq_authorization_root
            && self.roots.hybrid_pq_authorization_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingHybridPqAuthorizationRoot);
        }
        if config.require_mempool_quarantine_root && self.roots.mempool_quarantine_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingMempoolQuarantineRoot);
        }
        if config.require_circuit_breaker_root && self.roots.circuit_breaker_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_live_heavy_gate_evidence
            && self.roots.live_heavy_gate_evidence_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingLiveHeavyGateEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.roots.operator_signoff_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.roots.reviewer_signoff_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingReviewerSignoffRoot);
        }
        if self.measurements.quarantine_confirmations < config.min_quarantine_confirmations {
            blockers.push(BroadcastBlockerKind::QuarantineConfirmationsTooLow);
        }
        if self.measurements.fee_cap_bps > config.max_fee_cap_bps {
            blockers.push(BroadcastBlockerKind::FeeCapBpsTooHigh);
        }
        if self.measurements.decoy_set_size < config.min_decoy_set_size {
            blockers.push(BroadcastBlockerKind::DecoySetSizeTooLow);
        }
        if config.broadcast_disabled {
            blockers.push(BroadcastBlockerKind::BroadcastDisabled);
        }
        if config.deny_broadcast_when_any_blocker_active || !config.broadcast_allowed {
            blockers.push(BroadcastBlockerKind::BroadcastDenied);
        }
        if config.roots_only_public_records {
            blockers.push(BroadcastBlockerKind::RootsOnlyBoundary);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title": self.lane.title(),
            "plan_label": self.plan_label,
            "ordinal": self.ordinal,
            "roots_root": self.roots.state_root(),
            "measurements_root": self.measurements.state_root(),
            "policy_root": self.policy.state_root(),
            "status": self.status.as_str(),
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "broadcast_allowed": self.broadcast_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_quarantine", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub lane: LaneKind,
    pub lane_title: String,
    pub quarantine_plans: Vec<BroadcastQuarantine>,
    pub command_hints: Vec<String>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
}

impl State {
    pub fn new(config: Config, lane: LaneKind, quarantine_plans: Vec<BroadcastQuarantine>) -> Self {
        let command_hints = quarantine_plans
            .iter()
            .map(|plan| plan.policy.command_hint.clone())
            .collect::<Vec<_>>();
        Self {
            broadcast_allowed: config.broadcast_allowed,
            broadcast_disabled: config.broadcast_disabled,
            heavy_gates_ran: config.heavy_gates_ran,
            config,
            lane,
            lane_title: lane.title().to_string(),
            quarantine_plans,
            command_hints,
        }
    }

    pub fn active_blockers(&self) -> Vec<BroadcastBlockerKind> {
        self.quarantine_plans
            .iter()
            .flat_map(|plan| plan.blockers.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn releasable_count(&self) -> usize {
        self.quarantine_plans
            .iter()
            .filter(|plan| plan.status == BroadcastStatus::Releasable)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.quarantine_plans
            .iter()
            .filter(|plan| !plan.blockers.is_empty())
            .count()
    }

    pub fn planned_root(&self) -> String {
        status_root(
            "wave103_broadcast_planned_candidates",
            &self.quarantine_plans,
            BroadcastStatus::Planned,
        )
    }

    pub fn quarantined_root(&self) -> String {
        status_root(
            "wave103_mempool_quarantined_candidates",
            &self.quarantine_plans,
            BroadcastStatus::Quarantined,
        )
    }

    pub fn reviewed_root(&self) -> String {
        status_root(
            "wave103_pq_reserve_privacy_reviewed_candidates",
            &self.quarantine_plans,
            BroadcastStatus::Reviewed,
        )
    }

    pub fn releasable_root(&self) -> String {
        status_root(
            "wave103_releasable_monero_tx_broadcast_candidates",
            &self.quarantine_plans,
            BroadcastStatus::Releasable,
        )
    }

    pub fn blocked_root(&self) -> String {
        blocked_root(&self.quarantine_plans)
    }

    pub fn command_root(&self) -> String {
        root_from_strings(
            "wave103_broadcast_quarantine_command_hints",
            self.command_hints.clone(),
        )
    }

    pub fn lane_summary_root(&self) -> String {
        domain_hash(
            "wave103-release-execution-monero-tx-broadcast-quarantine-lane-summary",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(LANE_ID),
                HashPart::Str(self.lane.as_str()),
                HashPart::U64(WAVE),
                HashPart::U64(self.quarantine_plans.len() as u64),
                HashPart::U64(self.blocked_count() as u64),
                HashPart::U64(self.releasable_count() as u64),
            ],
            32,
        )
    }

    pub fn broadcast_denial_root(&self) -> String {
        let blocker_labels = self
            .active_blockers()
            .into_iter()
            .map(|blocker| blocker.as_str().to_string())
            .collect::<Vec<_>>();
        root_from_strings(
            "wave103_monero_tx_broadcast_denial_blockers",
            blocker_labels,
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "lane": self.lane.as_str(),
            "lane_title": self.lane_title,
            "quarantine_plan_count": self.quarantine_plans.len(),
            "blocked_count": self.blocked_count(),
            "releasable_count": self.releasable_count(),
            "planned_root": self.planned_root(),
            "quarantined_root": self.quarantined_root(),
            "reviewed_root": self.reviewed_root(),
            "releasable_root": self.releasable_root(),
            "blocked_root": self.blocked_root(),
            "command_root": self.command_root(),
            "lane_summary_root": self.lane_summary_root(),
            "broadcast_denial_root": self.broadcast_denial_root(),
            "broadcast_allowed": self.broadcast_allowed,
            "broadcast_disabled": self.broadcast_disabled,
            "heavy_gates_ran": self.heavy_gates_ran,
            "quarantine_plans": self.quarantine_plans.iter().map(|plan| plan.public_record()).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> State {
    let config = Config::default();
    let lane = LaneKind::PqReservePrivacy;
    let plan_labels = [
        "wave102_reconciliation_handoff",
        "release_transaction_plan_hold",
        "reserve_authorization_review",
        "privacy_budget_lock",
        "fee_cap_lock",
        "decoy_privacy_review",
        "hybrid_pq_authorization_review",
        "mempool_quarantine_hold",
    ];
    let quarantine_plans = plan_labels
        .iter()
        .enumerate()
        .map(|(index, plan_label)| {
            BroadcastQuarantine::empty(lane, plan_label, (index + 1) as u64, &config)
        })
        .collect::<Vec<_>>();
    State::new(config, lane, quarantine_plans)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<BroadcastBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_wave102_reconciliation_root {
        blockers.push(BroadcastBlockerKind::MissingWave102ReconciliationRoot);
    }
    if config.require_release_transaction_plan_root {
        blockers.push(BroadcastBlockerKind::MissingReleaseTransactionPlanRoot);
    }
    if config.require_reserve_authorization_root {
        blockers.push(BroadcastBlockerKind::MissingReserveAuthorizationRoot);
    }
    if config.require_privacy_budget_root {
        blockers.push(BroadcastBlockerKind::MissingPrivacyBudgetRoot);
    }
    if config.require_fee_cap_root {
        blockers.push(BroadcastBlockerKind::MissingFeeCapRoot);
    }
    if config.require_decoy_privacy_root {
        blockers.push(BroadcastBlockerKind::MissingDecoyPrivacyRoot);
    }
    if config.require_hybrid_pq_authorization_root {
        blockers.push(BroadcastBlockerKind::MissingHybridPqAuthorizationRoot);
    }
    if config.require_mempool_quarantine_root {
        blockers.push(BroadcastBlockerKind::MissingMempoolQuarantineRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(BroadcastBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_live_heavy_gate_evidence {
        blockers.push(BroadcastBlockerKind::MissingLiveHeavyGateEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(BroadcastBlockerKind::QuarantineConfirmationsTooLow);
    blockers.push(BroadcastBlockerKind::FeeCapBpsTooHigh);
    blockers.push(BroadcastBlockerKind::DecoySetSizeTooLow);
    if config.broadcast_disabled {
        blockers.push(BroadcastBlockerKind::BroadcastDisabled);
    }
    if config.deny_broadcast_when_any_blocker_active || !config.broadcast_allowed {
        blockers.push(BroadcastBlockerKind::BroadcastDenied);
    }
    if config.roots_only_public_records {
        blockers.push(BroadcastBlockerKind::RootsOnlyBoundary);
    }
    blockers
}

fn blocked_root(plans: &[BroadcastQuarantine]) -> String {
    let leaves = plans
        .iter()
        .flat_map(|plan| {
            plan.blockers.iter().map(move |blocker| {
                json!({
                    "lane": plan.lane.as_str(),
                    "plan_label": plan.plan_label,
                    "blocker": blocker.as_str(),
                    "plan_root": plan.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "wave103_blocked_monero_tx_broadcast_quarantine_guards",
        &leaves,
    )
}

fn status_root(domain: &str, plans: &[BroadcastQuarantine], status: BroadcastStatus) -> String {
    root_from_strings(
        domain,
        plans.iter().filter_map(|plan| {
            if plan.status == status {
                Some(plan.state_root())
            } else {
                None
            }
        }),
    )
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn broadcast_quarantine_guard_root(
    guard_kind: &str,
    lane: LaneKind,
    plan_label: &str,
    ordinal: u64,
    first_guard_root: &str,
    second_guard_root: &str,
) -> String {
    domain_hash(
        "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(plan_label),
            HashPart::U64(ordinal),
            HashPart::Str(first_guard_root),
            HashPart::Str(second_guard_root),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-record",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn label_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-label",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

pub fn devnet_broadcast_quarantine_guard_root() -> String {
    let state = devnet();
    broadcast_quarantine_guard_root(
        "devnet_broadcast_hold",
        state.lane,
        LANE_ID,
        WAVE,
        &state.blocked_root(),
        &state.broadcast_denial_root(),
    )
}
