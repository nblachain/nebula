use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-final-transcript-runtime-v1";
const WAVE: u64 = 103;
const RECONCILIATION_WAVE: u64 = 102;
const MIN_QUARANTINE_DEPTH: u64 = 1_030_000;
const MIN_DECOY_SET_SIZE: u64 = 16;
const MAX_BROADCAST_FEE_BPS: u64 = 12;
const MIN_MEMPOOL_OBSERVATION_ROUNDS: u64 = 12;
const LANE_ID: &str =
    "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-final-transcript";

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, BroadcastQuarantineError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineError {
    LaneMissing,
    CheckpointMissing,
    Wave102ReconciliationRootMissing,
    ReleaseTransactionPlanRootMissing,
    FeeCapRootMissing,
    DecoyPrivacyRootMissing,
    PqAuthorizationRootMissing,
    MempoolQuarantineRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    LiveHeavyGateEvidenceRootMissing,
    QuarantineDepthTooLow,
    DecoySetTooSmall,
    BroadcastFeeTooHigh,
    ObservationRoundsTooLow,
    BroadcastStillQuarantined,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    Compile,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
    FinalTranscript,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
            Self::FinalTranscript => "final_transcript",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Compile => "Compile Monero tx broadcast quarantine guard",
            Self::RuntimeReplay => "Runtime replay Monero tx broadcast quarantine guard",
            Self::AuditSecurity => "Audit security Monero tx broadcast quarantine guard",
            Self::BridgeCustody => "Bridge custody Monero tx broadcast quarantine guard",
            Self::WalletWatchtower => "Wallet watchtower Monero tx broadcast quarantine guard",
            Self::PqReservePrivacy => "PQ reserve privacy Monero tx broadcast quarantine guard",
            Self::FinalTranscript => "Final transcript Monero tx broadcast quarantine guard",
        }
    }

    pub fn command_scope(self) -> &'static str {
        match self {
            Self::Compile => "compile-monero-broadcast-quarantine",
            Self::RuntimeReplay => "runtime-replay-monero-broadcast-quarantine",
            Self::AuditSecurity => "audit-security-monero-broadcast-quarantine",
            Self::BridgeCustody => "bridge-custody-monero-broadcast-quarantine",
            Self::WalletWatchtower => "wallet-watchtower-monero-broadcast-quarantine",
            Self::PqReservePrivacy => "pq-reserve-privacy-monero-broadcast-quarantine",
            Self::FinalTranscript => "final-transcript-monero-broadcast-quarantine",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastStatus {
    Empty,
    Quarantined,
    PlanCandidate,
    MempoolObserved,
    PrivacyChecked,
    BroadcastReady,
}

impl BroadcastStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Quarantined => "quarantined",
            Self::PlanCandidate => "plan_candidate",
            Self::MempoolObserved => "mempool_observed",
            Self::PrivacyChecked => "privacy_checked",
            Self::BroadcastReady => "broadcast_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastBlockerKind {
    MissingWave102ReconciliationRoot,
    MissingReleaseTransactionPlanRoot,
    MissingFeeCapRoot,
    MissingDecoyPrivacyRoot,
    MissingPqAuthorizationRoot,
    MissingMempoolQuarantineRoot,
    MissingCircuitBreakerRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    MissingLiveHeavyGateEvidenceRoot,
    QuarantineDepthTooLow,
    DecoySetTooSmall,
    BroadcastFeeTooHigh,
    ObservationRoundsTooLow,
    CircuitBreakerArmed,
    BroadcastDisabled,
    RootsOnlyBoundary,
}

impl BroadcastBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave102ReconciliationRoot => "missing_wave102_reconciliation_root",
            Self::MissingReleaseTransactionPlanRoot => "missing_release_transaction_plan_root",
            Self::MissingFeeCapRoot => "missing_fee_cap_root",
            Self::MissingDecoyPrivacyRoot => "missing_decoy_privacy_root",
            Self::MissingPqAuthorizationRoot => "missing_pq_authorization_root",
            Self::MissingMempoolQuarantineRoot => "missing_mempool_quarantine_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::MissingLiveHeavyGateEvidenceRoot => "missing_live_heavy_gate_evidence_root",
            Self::QuarantineDepthTooLow => "quarantine_depth_too_low",
            Self::DecoySetTooSmall => "decoy_set_too_small",
            Self::BroadcastFeeTooHigh => "broadcast_fee_too_high",
            Self::ObservationRoundsTooLow => "observation_rounds_too_low",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::BroadcastDisabled => "broadcast_disabled",
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
    pub min_quarantine_depth: u64,
    pub min_decoy_set_size: u64,
    pub max_broadcast_fee_bps: u64,
    pub min_mempool_observation_rounds: u64,
    pub require_wave102_reconciliation_root: bool,
    pub require_release_transaction_plan_root: bool,
    pub require_fee_cap_root: bool,
    pub require_decoy_privacy_root: bool,
    pub require_pq_authorization_root: bool,
    pub require_mempool_quarantine_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_live_heavy_gate_evidence: bool,
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
            min_quarantine_depth: MIN_QUARANTINE_DEPTH,
            min_decoy_set_size: MIN_DECOY_SET_SIZE,
            max_broadcast_fee_bps: MAX_BROADCAST_FEE_BPS,
            min_mempool_observation_rounds: MIN_MEMPOOL_OBSERVATION_ROUNDS,
            require_wave102_reconciliation_root: true,
            require_release_transaction_plan_root: true,
            require_fee_cap_root: true,
            require_decoy_privacy_root: true,
            require_pq_authorization_root: true,
            require_mempool_quarantine_root: true,
            require_circuit_breaker_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_live_heavy_gate_evidence: true,
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
            "min_quarantine_depth": self.min_quarantine_depth,
            "min_decoy_set_size": self.min_decoy_set_size,
            "max_broadcast_fee_bps": self.max_broadcast_fee_bps,
            "min_mempool_observation_rounds": self.min_mempool_observation_rounds,
            "require_wave102_reconciliation_root": self.require_wave102_reconciliation_root,
            "require_release_transaction_plan_root": self.require_release_transaction_plan_root,
            "require_fee_cap_root": self.require_fee_cap_root,
            "require_decoy_privacy_root": self.require_decoy_privacy_root,
            "require_pq_authorization_root": self.require_pq_authorization_root,
            "require_mempool_quarantine_root": self.require_mempool_quarantine_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
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
pub struct BroadcastRoots {
    pub wave102_reconciliation_root: Option<String>,
    pub release_transaction_plan_root: Option<String>,
    pub fee_cap_root: Option<String>,
    pub decoy_privacy_root: Option<String>,
    pub pq_authorization_root: Option<String>,
    pub mempool_quarantine_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub live_heavy_gate_evidence_root: Option<String>,
}

impl BroadcastRoots {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave102_reconciliation_root": self.wave102_reconciliation_root,
            "release_transaction_plan_root": self.release_transaction_plan_root,
            "fee_cap_root": self.fee_cap_root,
            "decoy_privacy_root": self.decoy_privacy_root,
            "pq_authorization_root": self.pq_authorization_root,
            "mempool_quarantine_root": self.mempool_quarantine_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "live_heavy_gate_evidence_root": self.live_heavy_gate_evidence_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_roots", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BroadcastMeasurements {
    pub quarantine_depth: u64,
    pub decoy_set_size: u64,
    pub broadcast_fee_bps: u64,
    pub mempool_observation_rounds: u64,
}

impl BroadcastMeasurements {
    pub fn blocked(config: &Config) -> Self {
        Self {
            quarantine_depth: config.min_quarantine_depth.saturating_sub(1),
            decoy_set_size: config.min_decoy_set_size.saturating_sub(1),
            broadcast_fee_bps: config.max_broadcast_fee_bps.saturating_add(1),
            mempool_observation_rounds: config.min_mempool_observation_rounds.saturating_sub(1),
        }
    }

    pub fn public_record(self) -> PublicRecord {
        json!({
            "quarantine_depth": self.quarantine_depth,
            "decoy_set_size": self.decoy_set_size,
            "broadcast_fee_bps": self.broadcast_fee_bps,
            "mempool_observation_rounds": self.mempool_observation_rounds,
        })
    }

    pub fn state_root(self) -> String {
        record_root("broadcast_measurements", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastPolicy {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub command_scope: String,
    pub command_hint: String,
    pub quarantine_policy_root: String,
    pub fee_policy_root: String,
    pub privacy_policy_root: String,
    pub pq_policy_root: String,
    pub mempool_policy_root: String,
}

impl BroadcastPolicy {
    pub fn new(lane: LaneKind, claim_label: &str, ordinal: u64) -> Self {
        let command_scope = lane.command_scope().to_string();
        let command_hint = format!(
            "nebula wave103 quarantine-broadcast --lane {} --claim {} --hold-broadcast",
            lane.as_str(),
            claim_label
        );
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            command_scope,
            command_hint,
            quarantine_policy_root: label_root(
                "quarantine_policy",
                lane.as_str(),
                claim_label,
                ordinal,
            ),
            fee_policy_root: label_root("fee_policy", lane.as_str(), claim_label, ordinal),
            privacy_policy_root: label_root("privacy_policy", lane.as_str(), claim_label, ordinal),
            pq_policy_root: label_root("pq_policy", lane.as_str(), claim_label, ordinal),
            mempool_policy_root: label_root("mempool_policy", lane.as_str(), claim_label, ordinal),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "claim_label": self.claim_label,
            "ordinal": self.ordinal,
            "command_scope": self.command_scope,
            "command_hint": self.command_hint,
            "quarantine_policy_root": self.quarantine_policy_root,
            "fee_policy_root": self.fee_policy_root,
            "privacy_policy_root": self.privacy_policy_root,
            "pq_policy_root": self.pq_policy_root,
            "mempool_policy_root": self.mempool_policy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastCheckpoint {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub roots: BroadcastRoots,
    pub measurements: BroadcastMeasurements,
    pub policy: BroadcastPolicy,
    pub status: BroadcastStatus,
    pub blockers: Vec<BroadcastBlockerKind>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
}

impl BroadcastCheckpoint {
    pub fn empty(lane: LaneKind, claim_label: &str, ordinal: u64, config: &Config) -> Self {
        let policy = BroadcastPolicy::new(lane, claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            roots: BroadcastRoots::default(),
            measurements: BroadcastMeasurements::blocked(config),
            policy,
            status: BroadcastStatus::Quarantined,
            blockers: initial_blockers(config),
            broadcast_allowed: false,
            broadcast_disabled: true,
        }
    }

    pub fn stage_broadcast(
        mut self,
        roots: BroadcastRoots,
        measurements: BroadcastMeasurements,
        config: &Config,
    ) -> Self {
        self.roots = roots;
        self.measurements = measurements;
        self.blockers = self.active_blockers(config);
        self.status = if self.blockers.is_empty() {
            BroadcastStatus::PrivacyChecked
        } else if self.roots.mempool_quarantine_root.is_some() {
            BroadcastStatus::MempoolObserved
        } else if self.roots.release_transaction_plan_root.is_some() {
            BroadcastStatus::PlanCandidate
        } else {
            BroadcastStatus::Quarantined
        };
        self.broadcast_allowed = false;
        self.broadcast_disabled = true;
        self
    }

    pub fn release_from_quarantine(mut self, config: &Config) -> Result<Self> {
        self.blockers = self.active_blockers(config);
        if self.blockers.is_empty() {
            self.status = BroadcastStatus::BroadcastReady;
            self.broadcast_allowed = true;
            self.broadcast_disabled = false;
            Ok(self)
        } else {
            Err(BroadcastQuarantineError::BroadcastStillQuarantined)
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
        if config.require_fee_cap_root && self.roots.fee_cap_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingFeeCapRoot);
        }
        if config.require_decoy_privacy_root && self.roots.decoy_privacy_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingDecoyPrivacyRoot);
        }
        if config.require_pq_authorization_root && self.roots.pq_authorization_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingPqAuthorizationRoot);
        }
        if config.require_mempool_quarantine_root && self.roots.mempool_quarantine_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingMempoolQuarantineRoot);
        }
        if config.require_circuit_breaker_root && self.roots.circuit_breaker_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_operator_signoff_root && self.roots.operator_signoff_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.roots.reviewer_signoff_root.is_none() {
            blockers.push(BroadcastBlockerKind::MissingReviewerSignoffRoot);
        }
        if config.require_live_heavy_gate_evidence
            && self.roots.live_heavy_gate_evidence_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingLiveHeavyGateEvidenceRoot);
        }
        if self.measurements.quarantine_depth < config.min_quarantine_depth {
            blockers.push(BroadcastBlockerKind::QuarantineDepthTooLow);
        }
        if self.measurements.decoy_set_size < config.min_decoy_set_size {
            blockers.push(BroadcastBlockerKind::DecoySetTooSmall);
        }
        if self.measurements.broadcast_fee_bps > config.max_broadcast_fee_bps {
            blockers.push(BroadcastBlockerKind::BroadcastFeeTooHigh);
        }
        if self.measurements.mempool_observation_rounds < config.min_mempool_observation_rounds {
            blockers.push(BroadcastBlockerKind::ObservationRoundsTooLow);
        }
        if config.arm_circuit_breaker_by_default {
            blockers.push(BroadcastBlockerKind::CircuitBreakerArmed);
        }
        if config.broadcast_disabled || !config.broadcast_allowed {
            blockers.push(BroadcastBlockerKind::BroadcastDisabled);
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
        record_root("broadcast_checkpoint", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub lane: LaneKind,
    pub lane_title: String,
    pub checkpoints: Vec<BroadcastCheckpoint>,
    pub command_hints: Vec<String>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
}

impl State {
    pub fn new(config: Config, lane: LaneKind, checkpoints: Vec<BroadcastCheckpoint>) -> Self {
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

    pub fn active_blockers(&self) -> Vec<BroadcastBlockerKind> {
        self.checkpoints
            .iter()
            .flat_map(|checkpoint| checkpoint.blockers.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn ready_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.status == BroadcastStatus::BroadcastReady)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| !checkpoint.blockers.is_empty())
            .count()
    }

    pub fn plan_candidate_root(&self) -> String {
        status_root(
            "wave103_monero_tx_plan_candidates",
            &self.checkpoints,
            BroadcastStatus::PlanCandidate,
        )
    }

    pub fn mempool_observed_root(&self) -> String {
        status_root(
            "wave103_monero_tx_mempool_observed",
            &self.checkpoints,
            BroadcastStatus::MempoolObserved,
        )
    }

    pub fn privacy_checked_root(&self) -> String {
        status_root(
            "wave103_monero_tx_privacy_checked",
            &self.checkpoints,
            BroadcastStatus::PrivacyChecked,
        )
    }

    pub fn broadcast_ready_root(&self) -> String {
        status_root(
            "wave103_monero_tx_broadcast_ready",
            &self.checkpoints,
            BroadcastStatus::BroadcastReady,
        )
    }

    pub fn blocked_root(&self) -> String {
        blocked_root(&self.checkpoints)
    }

    pub fn command_root(&self) -> String {
        root_from_strings(
            "wave103_monero_tx_broadcast_command_hints",
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
                HashPart::U64(self.ready_count() as u64),
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
            "checkpoint_count": self.checkpoints.len(),
            "blocked_count": self.blocked_count(),
            "ready_count": self.ready_count(),
            "plan_candidate_root": self.plan_candidate_root(),
            "mempool_observed_root": self.mempool_observed_root(),
            "privacy_checked_root": self.privacy_checked_root(),
            "broadcast_ready_root": self.broadcast_ready_root(),
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
    let lane = LaneKind::FinalTranscript;
    let claim_labels = [
        "compile_lane_broadcast_quarantine",
        "runtime_lane_broadcast_quarantine",
        "audit_lane_broadcast_quarantine",
        "custody_lane_broadcast_quarantine",
        "wallet_lane_broadcast_quarantine",
        "pq_privacy_lane_broadcast_quarantine",
        "global_monero_broadcast_hold",
    ];
    let checkpoints = claim_labels
        .iter()
        .enumerate()
        .map(|(index, claim_label)| {
            BroadcastCheckpoint::empty(lane, claim_label, (index + 1) as u64, &config)
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

fn initial_blockers(config: &Config) -> Vec<BroadcastBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_wave102_reconciliation_root {
        blockers.push(BroadcastBlockerKind::MissingWave102ReconciliationRoot);
    }
    if config.require_release_transaction_plan_root {
        blockers.push(BroadcastBlockerKind::MissingReleaseTransactionPlanRoot);
    }
    if config.require_fee_cap_root {
        blockers.push(BroadcastBlockerKind::MissingFeeCapRoot);
    }
    if config.require_decoy_privacy_root {
        blockers.push(BroadcastBlockerKind::MissingDecoyPrivacyRoot);
    }
    if config.require_pq_authorization_root {
        blockers.push(BroadcastBlockerKind::MissingPqAuthorizationRoot);
    }
    if config.require_mempool_quarantine_root {
        blockers.push(BroadcastBlockerKind::MissingMempoolQuarantineRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(BroadcastBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingReviewerSignoffRoot);
    }
    if config.require_live_heavy_gate_evidence {
        blockers.push(BroadcastBlockerKind::MissingLiveHeavyGateEvidenceRoot);
    }
    blockers.push(BroadcastBlockerKind::QuarantineDepthTooLow);
    blockers.push(BroadcastBlockerKind::DecoySetTooSmall);
    blockers.push(BroadcastBlockerKind::BroadcastFeeTooHigh);
    blockers.push(BroadcastBlockerKind::ObservationRoundsTooLow);
    if config.arm_circuit_breaker_by_default {
        blockers.push(BroadcastBlockerKind::CircuitBreakerArmed);
    }
    if config.broadcast_disabled || !config.broadcast_allowed {
        blockers.push(BroadcastBlockerKind::BroadcastDisabled);
    }
    if config.roots_only_public_records {
        blockers.push(BroadcastBlockerKind::RootsOnlyBoundary);
    }
    blockers
}

fn blocked_root(checkpoints: &[BroadcastCheckpoint]) -> String {
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
    checkpoints: &[BroadcastCheckpoint],
    status: BroadcastStatus,
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

pub fn devnet_broadcast_quarantine_root() -> String {
    let state = devnet();
    domain_hash(
        "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-root",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LANE_ID),
            HashPart::Str(&state.blocked_root()),
            HashPart::Str(&state.broadcast_denial_root()),
        ],
        32,
    )
}
