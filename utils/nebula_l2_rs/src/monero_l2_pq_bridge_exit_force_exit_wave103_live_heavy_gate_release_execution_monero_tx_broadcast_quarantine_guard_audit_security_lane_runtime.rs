use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-audit-security-lane-runtime-v1";
const WAVE: u64 = 103;
const RECONCILIATION_WAVE: u64 = 102;
const MIN_QUARANTINE_CONFIRMATIONS: u64 = 720;
const MAX_FEE_CAP_BPS: u64 = 25;
const LANE_ID: &str =
    "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-audit-security";

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, BroadcastQuarantineError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineError {
    LaneMissing,
    ClaimMissing,
    Wave102ReconciliationRootMissing,
    ReleaseTransactionPlanRootMissing,
    AuditApprovalRootMissing,
    FeeCapRootMissing,
    DecoyPrivacyRootMissing,
    PqAuthorizationRootMissing,
    MempoolQuarantineRootMissing,
    CircuitBreakerRootMissing,
    LiveHeavyGateEvidenceRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    QuarantineConfirmationsTooLow,
    FeeCapBpsTooHigh,
    CircuitBreakerArmed,
    BroadcastStillQuarantined,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    AuditSecurity,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditSecurity => "audit_security",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::AuditSecurity => "Audit security Monero tx broadcast quarantine guard",
        }
    }

    pub fn command_scope(self) -> &'static str {
        match self {
            Self::AuditSecurity => "audit-security-broadcast-quarantine",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineStatus {
    Empty,
    Blocked,
    Planned,
    Audited,
    Quarantined,
    Released,
}

impl BroadcastQuarantineStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Blocked => "blocked",
            Self::Planned => "planned",
            Self::Audited => "audited",
            Self::Quarantined => "quarantined",
            Self::Released => "released",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineBlockerKind {
    MissingWave102ReconciliationRoot,
    MissingReleaseTransactionPlanRoot,
    MissingAuditApprovalRoot,
    MissingFeeCapRoot,
    MissingDecoyPrivacyRoot,
    MissingPqAuthorizationRoot,
    MissingMempoolQuarantineRoot,
    MissingCircuitBreakerRoot,
    MissingLiveHeavyGateEvidenceRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    QuarantineConfirmationsTooLow,
    FeeCapBpsTooHigh,
    CircuitBreakerArmed,
    BroadcastDenied,
    RootsOnlyBoundary,
}

impl BroadcastQuarantineBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave102ReconciliationRoot => "missing_wave102_reconciliation_root",
            Self::MissingReleaseTransactionPlanRoot => "missing_release_transaction_plan_root",
            Self::MissingAuditApprovalRoot => "missing_audit_approval_root",
            Self::MissingFeeCapRoot => "missing_fee_cap_root",
            Self::MissingDecoyPrivacyRoot => "missing_decoy_privacy_root",
            Self::MissingPqAuthorizationRoot => "missing_pq_authorization_root",
            Self::MissingMempoolQuarantineRoot => "missing_mempool_quarantine_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingLiveHeavyGateEvidenceRoot => "missing_live_heavy_gate_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::QuarantineConfirmationsTooLow => "quarantine_confirmations_too_low",
            Self::FeeCapBpsTooHigh => "fee_cap_bps_too_high",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
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
    pub require_wave102_reconciliation_root: bool,
    pub require_release_transaction_plan_root: bool,
    pub require_audit_approval_root: bool,
    pub require_fee_cap_root: bool,
    pub require_decoy_privacy_root: bool,
    pub require_pq_authorization_root: bool,
    pub require_mempool_quarantine_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_live_heavy_gate_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub deny_broadcast_when_any_blocker_active: bool,
    pub arm_circuit_breaker_by_default: bool,
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
            active_lane: LaneKind::AuditSecurity.as_str().to_string(),
            min_quarantine_confirmations: MIN_QUARANTINE_CONFIRMATIONS,
            max_fee_cap_bps: MAX_FEE_CAP_BPS,
            require_wave102_reconciliation_root: true,
            require_release_transaction_plan_root: true,
            require_audit_approval_root: true,
            require_fee_cap_root: true,
            require_decoy_privacy_root: true,
            require_pq_authorization_root: true,
            require_mempool_quarantine_root: true,
            require_circuit_breaker_root: true,
            require_live_heavy_gate_evidence_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            deny_broadcast_when_any_blocker_active: true,
            arm_circuit_breaker_by_default: true,
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
            "require_wave102_reconciliation_root": self.require_wave102_reconciliation_root,
            "require_release_transaction_plan_root": self.require_release_transaction_plan_root,
            "require_audit_approval_root": self.require_audit_approval_root,
            "require_fee_cap_root": self.require_fee_cap_root,
            "require_decoy_privacy_root": self.require_decoy_privacy_root,
            "require_pq_authorization_root": self.require_pq_authorization_root,
            "require_mempool_quarantine_root": self.require_mempool_quarantine_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_live_heavy_gate_evidence_root": self.require_live_heavy_gate_evidence_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "deny_broadcast_when_any_blocker_active": self.deny_broadcast_when_any_blocker_active,
            "arm_circuit_breaker_by_default": self.arm_circuit_breaker_by_default,
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
pub struct BroadcastQuarantineRoots {
    pub wave102_reconciliation_root: Option<String>,
    pub release_transaction_plan_root: Option<String>,
    pub audit_approval_root: Option<String>,
    pub fee_cap_root: Option<String>,
    pub decoy_privacy_root: Option<String>,
    pub pq_authorization_root: Option<String>,
    pub mempool_quarantine_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub live_heavy_gate_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
}

impl BroadcastQuarantineRoots {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave102_reconciliation_root": self.wave102_reconciliation_root,
            "release_transaction_plan_root": self.release_transaction_plan_root,
            "audit_approval_root": self.audit_approval_root,
            "fee_cap_root": self.fee_cap_root,
            "decoy_privacy_root": self.decoy_privacy_root,
            "pq_authorization_root": self.pq_authorization_root,
            "mempool_quarantine_root": self.mempool_quarantine_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "live_heavy_gate_evidence_root": self.live_heavy_gate_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_quarantine_roots", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BroadcastQuarantineMeasurements {
    pub quarantine_confirmations: u64,
    pub fee_cap_bps: u64,
}

impl BroadcastQuarantineMeasurements {
    pub fn blocked(config: &Config) -> Self {
        Self {
            quarantine_confirmations: config.min_quarantine_confirmations.saturating_sub(1),
            fee_cap_bps: config.max_fee_cap_bps.saturating_add(1),
        }
    }

    pub fn public_record(self) -> PublicRecord {
        json!({
            "quarantine_confirmations": self.quarantine_confirmations,
            "fee_cap_bps": self.fee_cap_bps,
        })
    }

    pub fn state_root(self) -> String {
        record_root("broadcast_quarantine_measurements", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastQuarantinePolicy {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub command_scope: String,
    pub command_hint: String,
    pub broadcast_hold_root: String,
    pub quarantine_policy_root: String,
    pub fee_policy_root: String,
    pub privacy_policy_root: String,
    pub pq_policy_root: String,
}

impl BroadcastQuarantinePolicy {
    pub fn new(lane: LaneKind, claim_label: &str, ordinal: u64) -> Self {
        let command_scope = lane.command_scope().to_string();
        let command_hint = format!(
            "nebula wave103 quarantine-broadcast --lane {} --claim {} --hold-broadcast",
            lane.as_str(),
            claim_label
        );
        let broadcast_hold_root = label_root("broadcast_hold", lane.as_str(), claim_label, ordinal);
        let quarantine_policy_root =
            label_root("quarantine_policy", lane.as_str(), claim_label, ordinal);
        let fee_policy_root = label_root("fee_policy", lane.as_str(), claim_label, ordinal);
        let privacy_policy_root = label_root("privacy_policy", lane.as_str(), claim_label, ordinal);
        let pq_policy_root = label_root("pq_policy", lane.as_str(), claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            command_scope,
            command_hint,
            broadcast_hold_root,
            quarantine_policy_root,
            fee_policy_root,
            privacy_policy_root,
            pq_policy_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "claim_label": self.claim_label,
            "ordinal": self.ordinal,
            "command_scope": self.command_scope,
            "command_hint": self.command_hint,
            "broadcast_hold_root": self.broadcast_hold_root,
            "quarantine_policy_root": self.quarantine_policy_root,
            "fee_policy_root": self.fee_policy_root,
            "privacy_policy_root": self.privacy_policy_root,
            "pq_policy_root": self.pq_policy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_quarantine_policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseTransactionBroadcastQuarantine {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub roots: BroadcastQuarantineRoots,
    pub measurements: BroadcastQuarantineMeasurements,
    pub policy: BroadcastQuarantinePolicy,
    pub status: BroadcastQuarantineStatus,
    pub blockers: Vec<BroadcastQuarantineBlockerKind>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
}

impl ReleaseTransactionBroadcastQuarantine {
    pub fn empty(lane: LaneKind, claim_label: &str, ordinal: u64, config: &Config) -> Self {
        let policy = BroadcastQuarantinePolicy::new(lane, claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            roots: BroadcastQuarantineRoots::default(),
            measurements: BroadcastQuarantineMeasurements::blocked(config),
            policy,
            status: BroadcastQuarantineStatus::Blocked,
            blockers: initial_blockers(config),
            broadcast_allowed: false,
            broadcast_disabled: true,
        }
    }

    pub fn stage_quarantine(
        mut self,
        roots: BroadcastQuarantineRoots,
        measurements: BroadcastQuarantineMeasurements,
        config: &Config,
    ) -> Self {
        self.roots = roots;
        self.measurements = measurements;
        self.blockers = self.active_blockers(config);
        self.status = if self.blockers.is_empty() {
            BroadcastQuarantineStatus::Released
        } else if self.roots.mempool_quarantine_root.is_some() {
            BroadcastQuarantineStatus::Quarantined
        } else if self.roots.audit_approval_root.is_some() {
            BroadcastQuarantineStatus::Audited
        } else if self.roots.release_transaction_plan_root.is_some() {
            BroadcastQuarantineStatus::Planned
        } else {
            BroadcastQuarantineStatus::Blocked
        };
        self.broadcast_allowed = false;
        self.broadcast_disabled = true;
        self
    }

    pub fn mark_released(mut self, config: &Config) -> Result<Self> {
        self.blockers = self.active_blockers(config);
        if self.blockers.is_empty() {
            self.status = BroadcastQuarantineStatus::Released;
            self.broadcast_allowed = true;
            self.broadcast_disabled = false;
            Ok(self)
        } else {
            Err(BroadcastQuarantineError::BroadcastStillQuarantined)
        }
    }

    pub fn active_blockers(&self, config: &Config) -> Vec<BroadcastQuarantineBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_wave102_reconciliation_root
            && self.roots.wave102_reconciliation_root.is_none()
        {
            blockers.push(BroadcastQuarantineBlockerKind::MissingWave102ReconciliationRoot);
        }
        if config.require_release_transaction_plan_root
            && self.roots.release_transaction_plan_root.is_none()
        {
            blockers.push(BroadcastQuarantineBlockerKind::MissingReleaseTransactionPlanRoot);
        }
        if config.require_audit_approval_root && self.roots.audit_approval_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingAuditApprovalRoot);
        }
        if config.require_fee_cap_root && self.roots.fee_cap_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingFeeCapRoot);
        }
        if config.require_decoy_privacy_root && self.roots.decoy_privacy_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingDecoyPrivacyRoot);
        }
        if config.require_pq_authorization_root && self.roots.pq_authorization_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingPqAuthorizationRoot);
        }
        if config.require_mempool_quarantine_root && self.roots.mempool_quarantine_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingMempoolQuarantineRoot);
        }
        if config.require_circuit_breaker_root && self.roots.circuit_breaker_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_live_heavy_gate_evidence_root
            && self.roots.live_heavy_gate_evidence_root.is_none()
        {
            blockers.push(BroadcastQuarantineBlockerKind::MissingLiveHeavyGateEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.roots.operator_signoff_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.roots.reviewer_signoff_root.is_none() {
            blockers.push(BroadcastQuarantineBlockerKind::MissingReviewerSignoffRoot);
        }
        if self.measurements.quarantine_confirmations < config.min_quarantine_confirmations {
            blockers.push(BroadcastQuarantineBlockerKind::QuarantineConfirmationsTooLow);
        }
        if self.measurements.fee_cap_bps > config.max_fee_cap_bps {
            blockers.push(BroadcastQuarantineBlockerKind::FeeCapBpsTooHigh);
        }
        if config.arm_circuit_breaker_by_default {
            blockers.push(BroadcastQuarantineBlockerKind::CircuitBreakerArmed);
        }
        if config.deny_broadcast_when_any_blocker_active {
            blockers.push(BroadcastQuarantineBlockerKind::BroadcastDenied);
        }
        if config.roots_only_public_records {
            blockers.push(BroadcastQuarantineBlockerKind::RootsOnlyBoundary);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title": self.lane.title(),
            "claim_label": self.claim_label,
            "ordinal": self.ordinal,
            "roots_root": self.roots.state_root(),
            "measurements_root": self.measurements.state_root(),
            "policy_root": self.policy.state_root(),
            "status": self.status.as_str(),
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "broadcast_allowed": self.broadcast_allowed,
            "broadcast_disabled": self.broadcast_disabled,
        })
    }

    pub fn state_root(&self) -> String {
        record_root(
            "release_transaction_broadcast_quarantine",
            &self.public_record(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub lane: LaneKind,
    pub lane_title: String,
    pub checkpoints: Vec<ReleaseTransactionBroadcastQuarantine>,
    pub command_hints: Vec<String>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
}

impl State {
    pub fn new(
        config: Config,
        lane: LaneKind,
        checkpoints: Vec<ReleaseTransactionBroadcastQuarantine>,
    ) -> Self {
        let command_hints = checkpoints
            .iter()
            .map(|checkpoint| checkpoint.policy.command_hint.clone())
            .collect::<Vec<_>>();
        Self {
            config,
            lane,
            lane_title: lane.title().to_string(),
            checkpoints,
            command_hints,
            broadcast_allowed: false,
            broadcast_disabled: true,
            heavy_gates_ran: false,
        }
    }

    pub fn active_blockers(&self) -> Vec<BroadcastQuarantineBlockerKind> {
        self.checkpoints
            .iter()
            .flat_map(|checkpoint| checkpoint.blockers.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn released_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.status == BroadcastQuarantineStatus::Released)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| !checkpoint.blockers.is_empty())
            .count()
    }

    pub fn planned_root(&self) -> String {
        status_root(
            "wave103_broadcast_quarantine_planned_claims",
            &self.checkpoints,
            BroadcastQuarantineStatus::Planned,
        )
    }

    pub fn audited_root(&self) -> String {
        status_root(
            "wave103_broadcast_quarantine_audited_claims",
            &self.checkpoints,
            BroadcastQuarantineStatus::Audited,
        )
    }

    pub fn quarantined_root(&self) -> String {
        status_root(
            "wave103_broadcast_quarantine_mempool_claims",
            &self.checkpoints,
            BroadcastQuarantineStatus::Quarantined,
        )
    }

    pub fn released_root(&self) -> String {
        status_root(
            "wave103_broadcast_quarantine_released_claims",
            &self.checkpoints,
            BroadcastQuarantineStatus::Released,
        )
    }

    pub fn blocked_root(&self) -> String {
        blocked_root(&self.checkpoints)
    }

    pub fn command_root(&self) -> String {
        root_from_strings(
            "wave103_broadcast_quarantine_command_hints",
            self.command_hints.clone(),
        )
    }

    pub fn lane_summary_root(&self) -> String {
        domain_hash(
            "wave103-monero-tx-broadcast-quarantine-lane-summary",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(LANE_ID),
                HashPart::Str(self.lane.as_str()),
                HashPart::U64(WAVE),
                HashPart::U64(self.checkpoints.len() as u64),
                HashPart::U64(self.blocked_count() as u64),
                HashPart::U64(self.released_count() as u64),
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
        root_from_strings("wave103_broadcast_denial_blockers", blocker_labels)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "lane": self.lane.as_str(),
            "lane_title": self.lane_title,
            "checkpoint_count": self.checkpoints.len(),
            "blocked_count": self.blocked_count(),
            "released_count": self.released_count(),
            "planned_root": self.planned_root(),
            "audited_root": self.audited_root(),
            "quarantined_root": self.quarantined_root(),
            "released_root": self.released_root(),
            "blocked_root": self.blocked_root(),
            "command_root": self.command_root(),
            "lane_summary_root": self.lane_summary_root(),
            "broadcast_denial_root": self.broadcast_denial_root(),
            "broadcast_allowed": self.broadcast_allowed,
            "broadcast_disabled": self.broadcast_disabled,
            "heavy_gates_ran": self.heavy_gates_ran,
            "checkpoints": self.checkpoints.iter().map(|checkpoint| checkpoint.public_record()).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> State {
    let config = Config::default();
    let lane = LaneKind::AuditSecurity;
    let claim_labels = [
        "wave102_reconciliation_anchor",
        "release_transaction_plan",
        "audit_approval_packet",
        "fee_cap_review",
        "decoy_privacy_review",
        "pq_authorization_review",
        "mempool_quarantine_hold",
        "operator_reviewer_signoff",
    ];
    let checkpoints = claim_labels
        .iter()
        .enumerate()
        .map(|(index, claim_label)| {
            ReleaseTransactionBroadcastQuarantine::empty(
                lane,
                claim_label,
                (index + 1) as u64,
                &config,
            )
        })
        .collect::<Vec<_>>();
    State::new(config, lane, checkpoints)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<BroadcastQuarantineBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_wave102_reconciliation_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingWave102ReconciliationRoot);
    }
    if config.require_release_transaction_plan_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingReleaseTransactionPlanRoot);
    }
    if config.require_audit_approval_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingAuditApprovalRoot);
    }
    if config.require_fee_cap_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingFeeCapRoot);
    }
    if config.require_decoy_privacy_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingDecoyPrivacyRoot);
    }
    if config.require_pq_authorization_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingPqAuthorizationRoot);
    }
    if config.require_mempool_quarantine_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingMempoolQuarantineRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_live_heavy_gate_evidence_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingLiveHeavyGateEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(BroadcastQuarantineBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(BroadcastQuarantineBlockerKind::QuarantineConfirmationsTooLow);
    blockers.push(BroadcastQuarantineBlockerKind::FeeCapBpsTooHigh);
    if config.arm_circuit_breaker_by_default {
        blockers.push(BroadcastQuarantineBlockerKind::CircuitBreakerArmed);
    }
    if config.deny_broadcast_when_any_blocker_active {
        blockers.push(BroadcastQuarantineBlockerKind::BroadcastDenied);
    }
    if config.roots_only_public_records {
        blockers.push(BroadcastQuarantineBlockerKind::RootsOnlyBoundary);
    }
    blockers
}

fn blocked_root(checkpoints: &[ReleaseTransactionBroadcastQuarantine]) -> String {
    let leaves = checkpoints
        .iter()
        .flat_map(|checkpoint| {
            checkpoint.blockers.iter().map(move |blocker| {
                json!({
                    "lane": checkpoint.lane.as_str(),
                    "claim_label": checkpoint.claim_label,
                    "blocker": blocker.as_str(),
                    "checkpoint_root": checkpoint.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "wave103_blocked_monero_tx_broadcast_quarantine_guards",
        &leaves,
    )
}

fn status_root(
    domain: &str,
    checkpoints: &[ReleaseTransactionBroadcastQuarantine],
    status: BroadcastQuarantineStatus,
) -> String {
    root_from_strings(
        domain,
        checkpoints.iter().filter_map(|checkpoint| {
            if checkpoint.status == status {
                Some(checkpoint.state_root())
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
    claim_label: &str,
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
            HashPart::Str(claim_label),
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
