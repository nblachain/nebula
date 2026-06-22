use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave106-live-heavy-gate-release-execution-monero-tx-release-credit-private-note-activation-nullifier-guard-final-transcript-runtime-v1";
const WAVE: u64 = 106;
const CREDIT_ACCOUNTING_WAVE: u64 = 105;
const MIN_CREDIT_ACCOUNTING_HEIGHT: u64 = 1_060_000;
const MIN_CONFIRMATION_LADDER_DEPTH: u64 = 32;
const MIN_REORG_MONITOR_DEPTH: u64 = 40;
const MAX_NOTE_ACTIVATION_FEE_BPS: u64 = 6;
const LANE_ID: &str =
    "wave106-live-heavy-gate-release-execution-monero-tx-release-credit-private-note-activation-nullifier-guard-final-transcript";

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, NoteActivationError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoteActivationError {
    LaneMissing,
    ClaimMissing,
    Wave105CreditAccountingRootMissing,
    PrivateNoteCommitmentRootMissing,
    RelayWitnessRootMissing,
    NullifierReservationRootMissing,
    AmountBucketPrivacyRootMissing,
    BeneficiaryWalletHistoryRootMissing,
    BridgeLiabilityClosureRootMissing,
    FeeRebateSettlementRootMissing,
    PqAuthorizationRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    LiveHeavyGateEvidenceRootMissing,
    CreditAccountingHeightTooLow,
    ConfirmationLadderTooShallow,
    ReorgMonitorTooShallow,
    NoteActivationFeeTooHigh,
    ReleaseCreditStillDenied,
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
            Self::Compile => "Compile Monero tx release credit private note activation nullifier guard",
            Self::RuntimeReplay => "Runtime replay Monero tx release credit private note activation nullifier guard",
            Self::AuditSecurity => "Audit security Monero tx release credit private note activation nullifier guard",
            Self::BridgeCustody => "Bridge custody Monero tx release credit private note activation nullifier guard",
            Self::WalletWatchtower => {
                "Wallet watchtower Monero tx release credit private note activation nullifier guard"
            }
            Self::PqReservePrivacy => {
                "PQ reserve privacy Monero tx release credit private note activation nullifier guard"
            }
            Self::FinalTranscript => "Final transcript Monero tx release credit private note activation nullifier guard",
        }
    }

    pub fn command_scope(self) -> &'static str {
        match self {
            Self::Compile => "compile-monero-relay-confirmation",
            Self::RuntimeReplay => "runtime-replay-monero-relay-confirmation",
            Self::AuditSecurity => "audit-security-monero-relay-confirmation",
            Self::BridgeCustody => "bridge-custody-monero-relay-confirmation",
            Self::WalletWatchtower => "wallet-watchtower-monero-relay-confirmation",
            Self::PqReservePrivacy => "pq-reserve-privacy-monero-relay-confirmation",
            Self::FinalTranscript => "final-transcript-monero-relay-confirmation",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoteActivationStatus {
    Empty,
    Blocked,
    PrivateNoteCandidate,
    NullifierReserved,
    WalletHistoryReady,
    PrivateNoteActivationReady,
}

impl NoteActivationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Blocked => "blocked",
            Self::PrivateNoteCandidate => "private_note_candidate",
            Self::NullifierReserved => "nullifier_reserved",
            Self::WalletHistoryReady => "wallet_history_ready",
            Self::PrivateNoteActivationReady => "private_note_activation_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoteActivationBlockerKind {
    MissingWave105CreditAccountingRoot,
    MissingPrivateNoteCommitmentRoot,
    MissingRelayWitnessRoot,
    MissingNullifierReservationRoot,
    MissingAmountBucketPrivacyRoot,
    MissingBeneficiaryWalletHistoryRoot,
    MissingBridgeLiabilityClosureRoot,
    MissingFeeRebateSettlementRoot,
    MissingPqAuthorizationRoot,
    MissingCircuitBreakerRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    MissingLiveHeavyGateEvidenceRoot,
    CreditAccountingHeightTooLow,
    ConfirmationLadderTooShallow,
    ReorgMonitorTooShallow,
    NoteActivationFeeTooHigh,
    CircuitBreakerArmed,
    ReleaseCreditDenied,
    RootsOnlyBoundary,
}

impl NoteActivationBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave105CreditAccountingRoot => "missing_wave105_credit_accounting_root",
            Self::MissingPrivateNoteCommitmentRoot => "missing_private_note_commitment_root",
            Self::MissingRelayWitnessRoot => "missing_relay_witness_root",
            Self::MissingNullifierReservationRoot => "missing_nullifier_reservation_root",
            Self::MissingAmountBucketPrivacyRoot => "missing_amount_bucket_privacy_root",
            Self::MissingBeneficiaryWalletHistoryRoot => "missing_beneficiary_wallet_history_root",
            Self::MissingBridgeLiabilityClosureRoot => "missing_bridge_liability_closure_root",
            Self::MissingFeeRebateSettlementRoot => "missing_fee_rebate_settlement_root",
            Self::MissingPqAuthorizationRoot => "missing_pq_authorization_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::MissingLiveHeavyGateEvidenceRoot => "missing_live_heavy_gate_evidence_root",
            Self::CreditAccountingHeightTooLow => "credit_accounting_height_too_low",
            Self::ConfirmationLadderTooShallow => "confirmation_ladder_too_shallow",
            Self::ReorgMonitorTooShallow => "reorg_monitor_too_shallow",
            Self::NoteActivationFeeTooHigh => "note_activation_fee_too_high",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::ReleaseCreditDenied => "release_credit_denied",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub credit_accounting_wave: u64,
    pub lane_id: String,
    pub min_credit_accounting_height: u64,
    pub min_confirmation_ladder_depth: u64,
    pub min_reorg_monitor_depth: u64,
    pub max_note_activation_fee_bps: u64,
    pub require_wave105_credit_accounting_root: bool,
    pub require_private_note_commitment_root: bool,
    pub require_relay_witness_root: bool,
    pub require_nullifier_reservation_root: bool,
    pub require_amount_bucket_privacy_root: bool,
    pub require_beneficiary_wallet_history_root: bool,
    pub require_bridge_liability_closure_root: bool,
    pub require_fee_rebate_settlement_root: bool,
    pub require_pq_authorization_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub arm_circuit_breaker_by_default: bool,
    pub note_activation_allowed: bool,
    pub release_credit_allowed: bool,
    pub heavy_gates_ran: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            credit_accounting_wave: CREDIT_ACCOUNTING_WAVE,
            lane_id: LANE_ID.to_string(),
            min_credit_accounting_height: MIN_CREDIT_ACCOUNTING_HEIGHT,
            min_confirmation_ladder_depth: MIN_CONFIRMATION_LADDER_DEPTH,
            min_reorg_monitor_depth: MIN_REORG_MONITOR_DEPTH,
            max_note_activation_fee_bps: MAX_NOTE_ACTIVATION_FEE_BPS,
            require_wave105_credit_accounting_root: true,
            require_private_note_commitment_root: true,
            require_relay_witness_root: true,
            require_nullifier_reservation_root: true,
            require_amount_bucket_privacy_root: true,
            require_beneficiary_wallet_history_root: true,
            require_bridge_liability_closure_root: true,
            require_fee_rebate_settlement_root: true,
            require_pq_authorization_root: true,
            require_circuit_breaker_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_live_heavy_gate_evidence: true,
            arm_circuit_breaker_by_default: true,
            note_activation_allowed: false,
            release_credit_allowed: false,
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
            "credit_accounting_wave": self.credit_accounting_wave,
            "lane_id": self.lane_id,
            "min_credit_accounting_height": self.min_credit_accounting_height,
            "min_confirmation_ladder_depth": self.min_confirmation_ladder_depth,
            "min_reorg_monitor_depth": self.min_reorg_monitor_depth,
            "max_note_activation_fee_bps": self.max_note_activation_fee_bps,
            "require_wave105_credit_accounting_root": self.require_wave105_credit_accounting_root,
            "require_private_note_commitment_root": self.require_private_note_commitment_root,
            "require_relay_witness_root": self.require_relay_witness_root,
            "require_nullifier_reservation_root": self.require_nullifier_reservation_root,
            "require_amount_bucket_privacy_root": self.require_amount_bucket_privacy_root,
            "require_beneficiary_wallet_history_root": self.require_beneficiary_wallet_history_root,
            "require_bridge_liability_closure_root": self.require_bridge_liability_closure_root,
            "require_fee_rebate_settlement_root": self.require_fee_rebate_settlement_root,
            "require_pq_authorization_root": self.require_pq_authorization_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "arm_circuit_breaker_by_default": self.arm_circuit_breaker_by_default,
            "note_activation_allowed": self.note_activation_allowed,
            "release_credit_allowed": self.release_credit_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NoteActivationRoots {
    pub wave105_credit_accounting_root: Option<String>,
    pub private_note_commitment_root: Option<String>,
    pub relay_witness_root: Option<String>,
    pub nullifier_reservation_root: Option<String>,
    pub amount_bucket_privacy_root: Option<String>,
    pub beneficiary_wallet_history_root: Option<String>,
    pub bridge_liability_closure_root: Option<String>,
    pub fee_rebate_settlement_root: Option<String>,
    pub pq_authorization_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub live_heavy_gate_evidence_root: Option<String>,
}

impl NoteActivationRoots {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave105_credit_accounting_root": self.wave105_credit_accounting_root,
            "private_note_commitment_root": self.private_note_commitment_root,
            "relay_witness_root": self.relay_witness_root,
            "nullifier_reservation_root": self.nullifier_reservation_root,
            "amount_bucket_privacy_root": self.amount_bucket_privacy_root,
            "beneficiary_wallet_history_root": self.beneficiary_wallet_history_root,
            "bridge_liability_closure_root": self.bridge_liability_closure_root,
            "fee_rebate_settlement_root": self.fee_rebate_settlement_root,
            "pq_authorization_root": self.pq_authorization_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "live_heavy_gate_evidence_root": self.live_heavy_gate_evidence_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("note_activation_roots", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NoteActivationMeasurements {
    pub relay_witness_height: u64,
    pub confirmation_ladder_depth: u64,
    pub reorg_monitor_depth: u64,
    pub note_activation_fee_bps: u64,
}

impl NoteActivationMeasurements {
    pub fn blocked(config: &Config) -> Self {
        Self {
            relay_witness_height: config.min_credit_accounting_height.saturating_sub(1),
            confirmation_ladder_depth: config.min_confirmation_ladder_depth.saturating_sub(1),
            reorg_monitor_depth: config.min_reorg_monitor_depth.saturating_sub(1),
            note_activation_fee_bps: config.max_note_activation_fee_bps.saturating_add(1),
        }
    }

    pub fn public_record(self) -> PublicRecord {
        json!({
            "relay_witness_height": self.relay_witness_height,
            "confirmation_ladder_depth": self.confirmation_ladder_depth,
            "reorg_monitor_depth": self.reorg_monitor_depth,
            "note_activation_fee_bps": self.note_activation_fee_bps,
        })
    }

    pub fn state_root(self) -> String {
        record_root("note_activation_measurements", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteActivationPolicy {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub command_scope: String,
    pub command_hint: String,
    pub relay_policy_root: String,
    pub mempool_policy_root: String,
    pub confirmation_policy_root: String,
    pub reorg_policy_root: String,
    pub credit_policy_root: String,
}

impl NoteActivationPolicy {
    pub fn new(lane: LaneKind, claim_label: &str, ordinal: u64) -> Self {
        let command_scope = lane.command_scope().to_string();
        let command_hint = format!(
            "nebula wave106 confirm-relay --lane {} --claim {} --hold-credit",
            lane.as_str(),
            claim_label
        );
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            command_scope,
            command_hint,
            relay_policy_root: label_root("relay_policy", lane.as_str(), claim_label, ordinal),
            mempool_policy_root: label_root("mempool_policy", lane.as_str(), claim_label, ordinal),
            confirmation_policy_root: label_root(
                "confirmation_policy",
                lane.as_str(),
                claim_label,
                ordinal,
            ),
            reorg_policy_root: label_root("reorg_policy", lane.as_str(), claim_label, ordinal),
            credit_policy_root: label_root("credit_policy", lane.as_str(), claim_label, ordinal),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "claim_label": self.claim_label,
            "ordinal": self.ordinal,
            "command_scope": self.command_scope,
            "command_hint": self.command_hint,
            "relay_policy_root": self.relay_policy_root,
            "mempool_policy_root": self.mempool_policy_root,
            "confirmation_policy_root": self.confirmation_policy_root,
            "reorg_policy_root": self.reorg_policy_root,
            "credit_policy_root": self.credit_policy_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("note_activation_policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteActivationCheckpoint {
    pub lane: LaneKind,
    pub claim_label: String,
    pub ordinal: u64,
    pub roots: NoteActivationRoots,
    pub measurements: NoteActivationMeasurements,
    pub policy: NoteActivationPolicy,
    pub status: NoteActivationStatus,
    pub blockers: Vec<NoteActivationBlockerKind>,
    pub note_activation_allowed: bool,
    pub release_credit_allowed: bool,
}

impl NoteActivationCheckpoint {
    pub fn empty(lane: LaneKind, claim_label: &str, ordinal: u64, config: &Config) -> Self {
        let policy = NoteActivationPolicy::new(lane, claim_label, ordinal);
        Self {
            lane,
            claim_label: claim_label.to_string(),
            ordinal,
            roots: NoteActivationRoots::default(),
            measurements: NoteActivationMeasurements::blocked(config),
            policy,
            status: NoteActivationStatus::Blocked,
            blockers: initial_blockers(config),
            note_activation_allowed: false,
            release_credit_allowed: false,
        }
    }

    pub fn stage_confirmation(
        mut self,
        roots: NoteActivationRoots,
        measurements: NoteActivationMeasurements,
        config: &Config,
    ) -> Self {
        self.roots = roots;
        self.measurements = measurements;
        self.blockers = self.active_blockers(config);
        self.status = if self.blockers.is_empty() {
            NoteActivationStatus::WalletHistoryReady
        } else if self.roots.beneficiary_wallet_history_root.is_some() {
            NoteActivationStatus::WalletHistoryReady
        } else if self.roots.nullifier_reservation_root.is_some() {
            NoteActivationStatus::NullifierReserved
        } else if self.roots.relay_witness_root.is_some() {
            NoteActivationStatus::PrivateNoteCandidate
        } else {
            NoteActivationStatus::Blocked
        };
        self.note_activation_allowed = false;
        self.release_credit_allowed = false;
        self
    }

    pub fn release_credit(mut self, config: &Config) -> Result<Self> {
        self.blockers = self.active_blockers(config);
        if self.blockers.is_empty() {
            self.status = NoteActivationStatus::PrivateNoteActivationReady;
            self.note_activation_allowed = true;
            self.release_credit_allowed = true;
            Ok(self)
        } else {
            Err(NoteActivationError::ReleaseCreditStillDenied)
        }
    }

    pub fn active_blockers(&self, config: &Config) -> Vec<NoteActivationBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_wave105_credit_accounting_root
            && self.roots.wave105_credit_accounting_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingWave105CreditAccountingRoot);
        }
        if config.require_private_note_commitment_root
            && self.roots.private_note_commitment_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingPrivateNoteCommitmentRoot);
        }
        if config.require_relay_witness_root && self.roots.relay_witness_root.is_none() {
            blockers.push(NoteActivationBlockerKind::MissingRelayWitnessRoot);
        }
        if config.require_nullifier_reservation_root
            && self.roots.nullifier_reservation_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingNullifierReservationRoot);
        }
        if config.require_amount_bucket_privacy_root
            && self.roots.amount_bucket_privacy_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingAmountBucketPrivacyRoot);
        }
        if config.require_beneficiary_wallet_history_root
            && self.roots.beneficiary_wallet_history_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingBeneficiaryWalletHistoryRoot);
        }
        if config.require_bridge_liability_closure_root
            && self.roots.bridge_liability_closure_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingBridgeLiabilityClosureRoot);
        }
        if config.require_fee_rebate_settlement_root
            && self.roots.fee_rebate_settlement_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingFeeRebateSettlementRoot);
        }
        if config.require_pq_authorization_root && self.roots.pq_authorization_root.is_none() {
            blockers.push(NoteActivationBlockerKind::MissingPqAuthorizationRoot);
        }
        if config.require_circuit_breaker_root && self.roots.circuit_breaker_root.is_none() {
            blockers.push(NoteActivationBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_operator_signoff_root && self.roots.operator_signoff_root.is_none() {
            blockers.push(NoteActivationBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.roots.reviewer_signoff_root.is_none() {
            blockers.push(NoteActivationBlockerKind::MissingReviewerSignoffRoot);
        }
        if config.require_live_heavy_gate_evidence
            && self.roots.live_heavy_gate_evidence_root.is_none()
        {
            blockers.push(NoteActivationBlockerKind::MissingLiveHeavyGateEvidenceRoot);
        }
        if self.measurements.relay_witness_height < config.min_credit_accounting_height {
            blockers.push(NoteActivationBlockerKind::CreditAccountingHeightTooLow);
        }
        if self.measurements.confirmation_ladder_depth < config.min_confirmation_ladder_depth {
            blockers.push(NoteActivationBlockerKind::ConfirmationLadderTooShallow);
        }
        if self.measurements.reorg_monitor_depth < config.min_reorg_monitor_depth {
            blockers.push(NoteActivationBlockerKind::ReorgMonitorTooShallow);
        }
        if self.measurements.note_activation_fee_bps > config.max_note_activation_fee_bps {
            blockers.push(NoteActivationBlockerKind::NoteActivationFeeTooHigh);
        }
        if config.arm_circuit_breaker_by_default {
            blockers.push(NoteActivationBlockerKind::CircuitBreakerArmed);
        }
        if !config.note_activation_allowed || !config.release_credit_allowed {
            blockers.push(NoteActivationBlockerKind::ReleaseCreditDenied);
        }
        if config.roots_only_public_records {
            blockers.push(NoteActivationBlockerKind::RootsOnlyBoundary);
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
            "note_activation_allowed": self.note_activation_allowed,
            "release_credit_allowed": self.release_credit_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("note_activation_checkpoint", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub lane: LaneKind,
    pub lane_title: String,
    pub checkpoints: Vec<NoteActivationCheckpoint>,
    pub command_hints: Vec<String>,
    pub note_activation_allowed: bool,
    pub release_credit_allowed: bool,
    pub heavy_gates_ran: bool,
}

impl State {
    pub fn new(config: Config, lane: LaneKind, checkpoints: Vec<NoteActivationCheckpoint>) -> Self {
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
            note_activation_allowed: false,
            release_credit_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn active_blockers(&self) -> Vec<NoteActivationBlockerKind> {
        self.checkpoints
            .iter()
            .flat_map(|checkpoint| checkpoint.blockers.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn ready_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| {
                checkpoint.status == NoteActivationStatus::PrivateNoteActivationReady
            })
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| !checkpoint.blockers.is_empty())
            .count()
    }

    pub fn relay_witness_root(&self) -> String {
        status_root(
            "wave106_monero_tx_private_note_candidates",
            &self.checkpoints,
            NoteActivationStatus::PrivateNoteCandidate,
        )
    }

    pub fn nullifier_reservation_root(&self) -> String {
        status_root(
            "wave106_monero_tx_mempool_acceptance_candidates",
            &self.checkpoints,
            NoteActivationStatus::NullifierReserved,
        )
    }

    pub fn wallet_history_ready_root(&self) -> String {
        status_root(
            "wave106_monero_tx_wallet_history_readys",
            &self.checkpoints,
            NoteActivationStatus::WalletHistoryReady,
        )
    }

    pub fn private_note_activation_ready_root(&self) -> String {
        status_root(
            "wave106_monero_tx_private_note_activation_ready",
            &self.checkpoints,
            NoteActivationStatus::PrivateNoteActivationReady,
        )
    }

    pub fn blocked_root(&self) -> String {
        blocked_root(&self.checkpoints)
    }

    pub fn command_root(&self) -> String {
        root_from_strings(
            "wave106_monero_tx_note_activation_command_hints",
            self.command_hints.clone(),
        )
    }

    pub fn lane_summary_root(&self) -> String {
        domain_hash(
            "wave106-monero-tx-release-credit-private-note-activation-nullifier-lane-summary",
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

    pub fn release_credit_denial_root(&self) -> String {
        let blocker_labels = self
            .active_blockers()
            .into_iter()
            .map(|blocker| blocker.as_str().to_string())
            .collect::<Vec<_>>();
        root_from_strings(
            "wave106_monero_tx_release_credit_denial_blockers",
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
            "relay_witness_root": self.relay_witness_root(),
            "nullifier_reservation_root": self.nullifier_reservation_root(),
            "wallet_history_ready_root": self.wallet_history_ready_root(),
            "private_note_activation_ready_root": self.private_note_activation_ready_root(),
            "blocked_root": self.blocked_root(),
            "command_root": self.command_root(),
            "lane_summary_root": self.lane_summary_root(),
            "release_credit_denial_root": self.release_credit_denial_root(),
            "note_activation_allowed": self.note_activation_allowed,
            "release_credit_allowed": self.release_credit_allowed,
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
        "compile_lane_note_activation",
        "runtime_lane_note_activation",
        "audit_lane_note_activation",
        "custody_lane_note_activation",
        "wallet_lane_note_activation",
        "pq_privacy_lane_note_activation",
        "global_release_credit_hold",
    ];
    let checkpoints = claim_labels
        .iter()
        .enumerate()
        .map(|(index, claim_label)| {
            NoteActivationCheckpoint::empty(lane, claim_label, (index + 1) as u64, &config)
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

fn initial_blockers(config: &Config) -> Vec<NoteActivationBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_wave105_credit_accounting_root {
        blockers.push(NoteActivationBlockerKind::MissingWave105CreditAccountingRoot);
    }
    if config.require_private_note_commitment_root {
        blockers.push(NoteActivationBlockerKind::MissingPrivateNoteCommitmentRoot);
    }
    if config.require_relay_witness_root {
        blockers.push(NoteActivationBlockerKind::MissingRelayWitnessRoot);
    }
    if config.require_nullifier_reservation_root {
        blockers.push(NoteActivationBlockerKind::MissingNullifierReservationRoot);
    }
    if config.require_amount_bucket_privacy_root {
        blockers.push(NoteActivationBlockerKind::MissingAmountBucketPrivacyRoot);
    }
    if config.require_beneficiary_wallet_history_root {
        blockers.push(NoteActivationBlockerKind::MissingBeneficiaryWalletHistoryRoot);
    }
    if config.require_bridge_liability_closure_root {
        blockers.push(NoteActivationBlockerKind::MissingBridgeLiabilityClosureRoot);
    }
    if config.require_fee_rebate_settlement_root {
        blockers.push(NoteActivationBlockerKind::MissingFeeRebateSettlementRoot);
    }
    if config.require_pq_authorization_root {
        blockers.push(NoteActivationBlockerKind::MissingPqAuthorizationRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(NoteActivationBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(NoteActivationBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(NoteActivationBlockerKind::MissingReviewerSignoffRoot);
    }
    if config.require_live_heavy_gate_evidence {
        blockers.push(NoteActivationBlockerKind::MissingLiveHeavyGateEvidenceRoot);
    }
    blockers.push(NoteActivationBlockerKind::CreditAccountingHeightTooLow);
    blockers.push(NoteActivationBlockerKind::ConfirmationLadderTooShallow);
    blockers.push(NoteActivationBlockerKind::ReorgMonitorTooShallow);
    blockers.push(NoteActivationBlockerKind::NoteActivationFeeTooHigh);
    if config.arm_circuit_breaker_by_default {
        blockers.push(NoteActivationBlockerKind::CircuitBreakerArmed);
    }
    if !config.note_activation_allowed || !config.release_credit_allowed {
        blockers.push(NoteActivationBlockerKind::ReleaseCreditDenied);
    }
    if config.roots_only_public_records {
        blockers.push(NoteActivationBlockerKind::RootsOnlyBoundary);
    }
    blockers
}

fn blocked_root(checkpoints: &[NoteActivationCheckpoint]) -> String {
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
    merkle_root("wave106_blocked_monero_tx_note_activation_guards", &leaves)
}

fn status_root(
    domain: &str,
    checkpoints: &[NoteActivationCheckpoint],
    status: NoteActivationStatus,
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
        "wave106-live-heavy-gate-release-execution-monero-tx-relay-confirmation-record",
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
        "wave106-live-heavy-gate-release-execution-monero-tx-relay-confirmation-label",
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

pub fn devnet_note_activation_root() -> String {
    let state = devnet();
    domain_hash(
        "wave106-live-heavy-gate-release-execution-monero-tx-relay-confirmation-root",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LANE_ID),
            HashPart::Str(&state.blocked_root()),
            HashPart::Str(&state.release_credit_denial_root()),
        ],
        32,
    )
}
