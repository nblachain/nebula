use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-runtime-replay-lane-runtime-v1";
const WAVE: u64 = 103;
const RECONCILIATION_WAVE: u64 = 102;
const LANE_ID: &str =
    "wave103-live-heavy-gate-release-execution-monero-tx-broadcast-quarantine-guard-runtime-replay";

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, BroadcastQuarantineError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastQuarantineError {
    LaneMissing,
    ClaimMissing,
    Wave102ReconciliationRootMissing,
    ReleaseTransactionPlanRootMissing,
    DeterministicReplayRootMissing,
    FeeCapRootMissing,
    DecoyPrivacyRootMissing,
    PqAuthorizationRootMissing,
    MempoolQuarantineRootMissing,
    CircuitBreakerRootMissing,
    LiveHeavyGateEvidenceRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    BroadcastStillQuarantined,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    RuntimeReplay,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime_replay",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "Runtime replay Monero transaction broadcast quarantine guard",
        }
    }

    pub fn command_scope(self) -> &'static str {
        match self {
            Self::RuntimeReplay => "runtime-replay-broadcast-quarantine",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastStatus {
    Empty,
    Blocked,
    ReplayCandidate,
    Quarantined,
    ReleaseCandidate,
    BroadcastReady,
}

impl BroadcastStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Blocked => "blocked",
            Self::ReplayCandidate => "replay_candidate",
            Self::Quarantined => "quarantined",
            Self::ReleaseCandidate => "release_candidate",
            Self::BroadcastReady => "broadcast_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastBlockerKind {
    MissingWave102ReconciliationRoot,
    MissingReleaseTransactionPlanRoot,
    MissingDeterministicReplayRoot,
    MissingFeeCapRoot,
    MissingDecoyPrivacyRoot,
    MissingPqAuthorizationRoot,
    MissingMempoolQuarantineRoot,
    MissingCircuitBreakerRoot,
    MissingLiveHeavyGateEvidenceRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    BroadcastDisabled,
    BroadcastDenied,
    HeavyGatesNotRun,
    RootsOnlyBoundary,
}

impl BroadcastBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave102ReconciliationRoot => "missing_wave102_reconciliation_root",
            Self::MissingReleaseTransactionPlanRoot => "missing_release_transaction_plan_root",
            Self::MissingDeterministicReplayRoot => "missing_deterministic_replay_root",
            Self::MissingFeeCapRoot => "missing_fee_cap_root",
            Self::MissingDecoyPrivacyRoot => "missing_decoy_privacy_root",
            Self::MissingPqAuthorizationRoot => "missing_pq_authorization_root",
            Self::MissingMempoolQuarantineRoot => "missing_mempool_quarantine_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingLiveHeavyGateEvidenceRoot => "missing_live_heavy_gate_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::BroadcastDisabled => "broadcast_disabled",
            Self::BroadcastDenied => "broadcast_denied",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
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
    pub require_wave102_reconciliation_root: bool,
    pub require_release_transaction_plan_root: bool,
    pub require_deterministic_replay_root: bool,
    pub require_fee_cap_root: bool,
    pub require_decoy_privacy_root: bool,
    pub require_pq_authorization_root: bool,
    pub require_mempool_quarantine_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
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
            active_lane: LaneKind::RuntimeReplay.as_str().to_string(),
            require_wave102_reconciliation_root: true,
            require_release_transaction_plan_root: true,
            require_deterministic_replay_root: true,
            require_fee_cap_root: true,
            require_decoy_privacy_root: true,
            require_pq_authorization_root: true,
            require_mempool_quarantine_root: true,
            require_circuit_breaker_root: true,
            require_live_heavy_gate_evidence: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
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
            "require_wave102_reconciliation_root": self.require_wave102_reconciliation_root,
            "require_release_transaction_plan_root": self.require_release_transaction_plan_root,
            "require_deterministic_replay_root": self.require_deterministic_replay_root,
            "require_fee_cap_root": self.require_fee_cap_root,
            "require_decoy_privacy_root": self.require_decoy_privacy_root,
            "require_pq_authorization_root": self.require_pq_authorization_root,
            "require_mempool_quarantine_root": self.require_mempool_quarantine_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
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
    pub deterministic_replay_root: Option<String>,
    pub fee_cap_root: Option<String>,
    pub decoy_privacy_root: Option<String>,
    pub pq_authorization_root: Option<String>,
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
            "deterministic_replay_root": self.deterministic_replay_root,
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
        record_root("broadcast_roots", &self.public_record())
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
    pub replay_policy_root: String,
    pub fee_cap_policy_root: String,
    pub privacy_policy_root: String,
    pub authorization_policy_root: String,
}

impl BroadcastPolicy {
    pub fn new(lane: LaneKind, claim_label: &str, ordinal: u64) -> Self {
        let command_scope = lane.command_scope().to_string();
        let command_hint = format!(
            "nebula wave103 quarantine-broadcast --lane {} --claim {} --hold-broadcast",
            lane.as_str(),
            claim_label
        );
        let quarantine_policy_root =
            label_root("quarantine_policy", lane.as_str(), claim_label, ordinal);
        let replay_policy_root = label_root("replay_policy", lane.as_str(), claim_label, ordinal);
        let fee_cap_policy_root = label_root("fee_cap_policy", lane.as_str(), claim_label, ordinal);
        let privacy_policy_root = label_root("privacy_policy", lane.as_str(), claim_label, ordinal);
        let authorization_policy_root =
            label_root("authorization_policy", lane.as_str(), claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            command_scope,
            command_hint,
            quarantine_policy_root,
            replay_policy_root,
            fee_cap_policy_root,
            privacy_policy_root,
            authorization_policy_root,
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
            "replay_policy_root": self.replay_policy_root,
            "fee_cap_policy_root": self.fee_cap_policy_root,
            "privacy_policy_root": self.privacy_policy_root,
            "authorization_policy_root": self.authorization_policy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("broadcast_policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseTransactionBroadcast {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub roots: BroadcastRoots,
    pub policy: BroadcastPolicy,
    pub status: BroadcastStatus,
    pub blockers: Vec<BroadcastBlockerKind>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
}

impl ReleaseTransactionBroadcast {
    pub fn empty(lane: LaneKind, claim_label: &str, ordinal: u64, config: &Config) -> Self {
        let policy = BroadcastPolicy::new(lane, claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            roots: BroadcastRoots::default(),
            policy,
            status: BroadcastStatus::Blocked,
            blockers: initial_blockers(config),
            broadcast_allowed: false,
            broadcast_disabled: true,
            heavy_gates_ran: false,
        }
    }

    pub fn stage_roots(mut self, roots: BroadcastRoots, config: &Config) -> Self {
        self.roots = roots;
        self.blockers = self.active_blockers(config);
        self.status = if self.blockers.is_empty() {
            BroadcastStatus::ReleaseCandidate
        } else if self.roots.mempool_quarantine_root.is_some() {
            BroadcastStatus::Quarantined
        } else if self.roots.deterministic_replay_root.is_some() {
            BroadcastStatus::ReplayCandidate
        } else {
            BroadcastStatus::Blocked
        };
        self.broadcast_allowed = false;
        self.broadcast_disabled = true;
        self.heavy_gates_ran = config.heavy_gates_ran;
        self
    }

    pub fn mark_broadcast_ready(mut self, config: &Config) -> Result<Self> {
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
        if config.require_deterministic_replay_root
            && self.roots.deterministic_replay_root.is_none()
        {
            blockers.push(BroadcastBlockerKind::MissingDeterministicReplayRoot);
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
        if config.broadcast_disabled {
            blockers.push(BroadcastBlockerKind::BroadcastDisabled);
        }
        if !config.broadcast_allowed {
            blockers.push(BroadcastBlockerKind::BroadcastDenied);
        }
        if !config.heavy_gates_ran {
            blockers.push(BroadcastBlockerKind::HeavyGatesNotRun);
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
            "policy_root": self.policy.state_root(),
            "status": self.status.as_str(),
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "broadcast_allowed": self.broadcast_allowed,
            "broadcast_disabled": self.broadcast_disabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_transaction_broadcast", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub lane: LaneKind,
    pub lane_title: String,
    pub checkpoints: Vec<ReleaseTransactionBroadcast>,
    pub command_hints: Vec<String>,
    pub broadcast_allowed: bool,
    pub broadcast_disabled: bool,
    pub heavy_gates_ran: bool,
}

impl State {
    pub fn new(
        config: Config,
        lane: LaneKind,
        checkpoints: Vec<ReleaseTransactionBroadcast>,
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

    pub fn active_blockers(&self) -> Vec<BroadcastBlockerKind> {
        self.checkpoints
            .iter()
            .flat_map(|checkpoint| checkpoint.blockers.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn broadcast_ready_count(&self) -> usize {
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

    pub fn replay_candidate_root(&self) -> String {
        status_root(
            "wave103_broadcast_replay_candidates",
            &self.checkpoints,
            BroadcastStatus::ReplayCandidate,
        )
    }

    pub fn quarantined_root(&self) -> String {
        status_root(
            "wave103_broadcast_quarantined_candidates",
            &self.checkpoints,
            BroadcastStatus::Quarantined,
        )
    }

    pub fn release_candidate_root(&self) -> String {
        status_root(
            "wave103_broadcast_release_candidates",
            &self.checkpoints,
            BroadcastStatus::ReleaseCandidate,
        )
    }

    pub fn broadcast_ready_root(&self) -> String {
        status_root(
            "wave103_broadcast_ready_claims",
            &self.checkpoints,
            BroadcastStatus::BroadcastReady,
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
            "wave103-release-execution-monero-tx-broadcast-quarantine-lane-summary",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(LANE_ID),
                HashPart::Str(self.lane.as_str()),
                HashPart::U64(WAVE),
                HashPart::U64(RECONCILIATION_WAVE),
                HashPart::U64(self.checkpoints.len() as u64),
                HashPart::U64(self.blocked_count() as u64),
                HashPart::U64(self.broadcast_ready_count() as u64),
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
            "broadcast_ready_count": self.broadcast_ready_count(),
            "replay_candidate_root": self.replay_candidate_root(),
            "quarantined_root": self.quarantined_root(),
            "release_candidate_root": self.release_candidate_root(),
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
    let lane = LaneKind::RuntimeReplay;
    let claim_labels = [
        "wave102_reconciliation_replay",
        "release_transaction_plan_replay",
        "deterministic_runtime_replay",
        "fee_cap_replay",
        "decoy_privacy_replay",
        "pq_authorization_replay",
        "mempool_quarantine_replay",
        "circuit_breaker_replay",
        "live_heavy_gate_evidence_replay",
        "operator_reviewer_signoff_replay",
    ];
    let checkpoints = claim_labels
        .iter()
        .enumerate()
        .map(|(index, claim_label)| {
            ReleaseTransactionBroadcast::empty(lane, claim_label, (index + 1) as u64, &config)
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
    if config.require_deterministic_replay_root {
        blockers.push(BroadcastBlockerKind::MissingDeterministicReplayRoot);
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
    if config.require_live_heavy_gate_evidence {
        blockers.push(BroadcastBlockerKind::MissingLiveHeavyGateEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(BroadcastBlockerKind::MissingReviewerSignoffRoot);
    }
    if config.broadcast_disabled {
        blockers.push(BroadcastBlockerKind::BroadcastDisabled);
    }
    if !config.broadcast_allowed {
        blockers.push(BroadcastBlockerKind::BroadcastDenied);
    }
    if !config.heavy_gates_ran {
        blockers.push(BroadcastBlockerKind::HeavyGatesNotRun);
    }
    if config.roots_only_public_records {
        blockers.push(BroadcastBlockerKind::RootsOnlyBoundary);
    }
    blockers
}

fn blocked_root(checkpoints: &[ReleaseTransactionBroadcast]) -> String {
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
    merkle_root("wave103_blocked_broadcast_quarantine_guards", &leaves)
}

fn status_root(
    domain: &str,
    checkpoints: &[ReleaseTransactionBroadcast],
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
