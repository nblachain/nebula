use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave100-live-heavy-gate-release-execution-settlement-notary-guard-v1";
const WAVE: u64 = 100;
const FINALITY_WAVE: u64 = 99;
const MIN_NOTARY_HEIGHT: u64 = 1_000_000;
const MIN_NOTARY_QUORUM: u64 = 4;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, SettlementError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlementError {
    LaneMissing,
    ClaimMissing,
    FinalityUnlockRootMissing,
    ExecutionBundleRootMissing,
    SettlementAccountingRootMissing,
    NotaryQuorumRootMissing,
    PayoutEnvelopeRootMissing,
    RollbackSentinelRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    NotaryHeightTooLow,
    NotaryQuorumTooSmall,
    CircuitBreakerArmed,
    RollbackSentinelOpen,
    SettlementStillBlocked,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    Compile,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
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
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Compile => "Compile settlement notary guard",
            Self::RuntimeReplay => "Runtime replay settlement notary guard",
            Self::AuditSecurity => "Audit security settlement notary guard",
            Self::BridgeCustody => "Bridge custody settlement notary guard",
            Self::WalletWatchtower => "Wallet watchtower settlement notary guard",
            Self::PqReservePrivacy => "PQ reserve privacy settlement notary guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlementStatus {
    Empty,
    Blocked,
    NotaryCandidate,
    NotaryGuarded,
    SettlementReady,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlementBlockerKind {
    MissingFinalityUnlockRoot,
    MissingExecutionBundleRoot,
    MissingSettlementAccountingRoot,
    MissingNotaryQuorumRoot,
    MissingPayoutEnvelopeRoot,
    MissingRollbackSentinelRoot,
    MissingCircuitBreakerRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    NotaryHeightTooLow,
    NotaryQuorumTooSmall,
    CircuitBreakerArmed,
    RollbackSentinelOpen,
    LiveHeavyGateEvidenceMissing,
}

impl SettlementBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingFinalityUnlockRoot => "missing_finality_unlock_root",
            Self::MissingExecutionBundleRoot => "missing_execution_bundle_root",
            Self::MissingSettlementAccountingRoot => "missing_settlement_accounting_root",
            Self::MissingNotaryQuorumRoot => "missing_notary_quorum_root",
            Self::MissingPayoutEnvelopeRoot => "missing_payout_envelope_root",
            Self::MissingRollbackSentinelRoot => "missing_rollback_sentinel_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::NotaryHeightTooLow => "notary_height_too_low",
            Self::NotaryQuorumTooSmall => "notary_quorum_too_small",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::RollbackSentinelOpen => "rollback_sentinel_open",
            Self::LiveHeavyGateEvidenceMissing => "live_heavy_gate_evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub finality_wave: u64,
    pub min_notary_height: u64,
    pub min_notary_quorum: u64,
    pub lane_settlement_threshold: u64,
    pub global_settlement_threshold: u64,
    pub require_finality_unlock_root: bool,
    pub require_execution_bundle_root: bool,
    pub require_settlement_accounting_root: bool,
    pub require_notary_quorum_root: bool,
    pub require_payout_envelope_root: bool,
    pub require_rollback_sentinel_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub deny_release_execution_when_any_lane_blocked: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            finality_wave: FINALITY_WAVE,
            min_notary_height: MIN_NOTARY_HEIGHT,
            min_notary_quorum: MIN_NOTARY_QUORUM,
            lane_settlement_threshold: 1,
            global_settlement_threshold: 6,
            require_finality_unlock_root: true,
            require_execution_bundle_root: true,
            require_settlement_accounting_root: true,
            require_notary_quorum_root: true,
            require_payout_envelope_root: true,
            require_rollback_sentinel_root: true,
            require_circuit_breaker_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_live_heavy_gate_evidence: true,
            deny_release_execution_when_any_lane_blocked: true,
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
            "finality_wave": self.finality_wave,
            "min_notary_height": self.min_notary_height,
            "min_notary_quorum": self.min_notary_quorum,
            "lane_settlement_threshold": self.lane_settlement_threshold,
            "global_settlement_threshold": self.global_settlement_threshold,
            "require_finality_unlock_root": self.require_finality_unlock_root,
            "require_execution_bundle_root": self.require_execution_bundle_root,
            "require_settlement_accounting_root": self.require_settlement_accounting_root,
            "require_notary_quorum_root": self.require_notary_quorum_root,
            "require_payout_envelope_root": self.require_payout_envelope_root,
            "require_rollback_sentinel_root": self.require_rollback_sentinel_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "deny_release_execution_when_any_lane_blocked": self.deny_release_execution_when_any_lane_blocked,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseExecutionSettlement {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave99_finality_unlock_root: String,
    pub finality_unlock_root: Option<String>,
    pub execution_bundle_root: Option<String>,
    pub settlement_accounting_root: Option<String>,
    pub notary_quorum_root: Option<String>,
    pub payout_envelope_root: Option<String>,
    pub rollback_sentinel_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub notary_height: Option<u64>,
    pub notary_quorum: Option<u64>,
    pub settlement_guard_root: String,
    pub payout_guard_root: String,
    pub rollback_guard_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<SettlementBlockerKind>,
    pub status: SettlementStatus,
}

impl ReleaseExecutionSettlement {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave99_finality_unlock_root: label_root(
                "wave99_finality_unlock",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            finality_unlock_root: None,
            execution_bundle_root: None,
            settlement_accounting_root: None,
            notary_quorum_root: None,
            payout_envelope_root: None,
            rollback_sentinel_root: None,
            circuit_breaker_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            notary_height: None,
            notary_quorum: None,
            settlement_guard_root: label_root(
                "settlement_guard",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            payout_guard_root: label_root("payout_guard", lane.as_str(), slot_label, ordinal),
            rollback_guard_root: label_root("rollback_guard", lane.as_str(), slot_label, ordinal),
            command_hint_root: label_root(
                "release_execution_settlement_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: SettlementStatus::Blocked,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_settlement(
        &self,
        finality_unlock_root: &str,
        execution_bundle_root: &str,
        settlement_accounting_root: &str,
        notary_quorum_root: &str,
        payout_envelope_root: &str,
        rollback_sentinel_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        notary_height: u64,
        notary_quorum: u64,
        config: &Config,
        circuit_breaker_armed: bool,
        rollback_sentinel_open: bool,
    ) -> Result<Self> {
        if finality_unlock_root.is_empty() {
            return Err(SettlementError::FinalityUnlockRootMissing);
        }
        if execution_bundle_root.is_empty() {
            return Err(SettlementError::ExecutionBundleRootMissing);
        }
        if settlement_accounting_root.is_empty() {
            return Err(SettlementError::SettlementAccountingRootMissing);
        }
        if notary_quorum_root.is_empty() {
            return Err(SettlementError::NotaryQuorumRootMissing);
        }
        if payout_envelope_root.is_empty() {
            return Err(SettlementError::PayoutEnvelopeRootMissing);
        }
        if rollback_sentinel_root.is_empty() {
            return Err(SettlementError::RollbackSentinelRootMissing);
        }
        if circuit_breaker_root.is_empty() {
            return Err(SettlementError::CircuitBreakerRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(SettlementError::OperatorSignoffRootMissing);
        }
        if reviewer_signoff_root.is_empty() {
            return Err(SettlementError::ReviewerSignoffRootMissing);
        }
        if notary_height < config.min_notary_height {
            return Err(SettlementError::NotaryHeightTooLow);
        }
        if notary_quorum < config.min_notary_quorum {
            return Err(SettlementError::NotaryQuorumTooSmall);
        }
        if circuit_breaker_armed {
            return Err(SettlementError::CircuitBreakerArmed);
        }
        if rollback_sentinel_open {
            return Err(SettlementError::RollbackSentinelOpen);
        }

        let mut next = self.clone();
        next.finality_unlock_root = Some(finality_unlock_root.to_string());
        next.execution_bundle_root = Some(execution_bundle_root.to_string());
        next.settlement_accounting_root = Some(settlement_accounting_root.to_string());
        next.notary_quorum_root = Some(notary_quorum_root.to_string());
        next.payout_envelope_root = Some(payout_envelope_root.to_string());
        next.rollback_sentinel_root = Some(rollback_sentinel_root.to_string());
        next.circuit_breaker_root = Some(circuit_breaker_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.notary_height = Some(notary_height);
        next.notary_quorum = Some(notary_quorum);
        next.settlement_guard_root = settlement_guard_root(
            "settlement",
            self.lane,
            &self.slot_label,
            finality_unlock_root,
            execution_bundle_root,
            settlement_accounting_root,
            notary_height,
            notary_quorum,
        );
        next.payout_guard_root = settlement_guard_root(
            "payout",
            self.lane,
            &self.slot_label,
            finality_unlock_root,
            payout_envelope_root,
            notary_quorum_root,
            notary_height,
            notary_quorum,
        );
        next.rollback_guard_root = settlement_guard_root(
            "rollback",
            self.lane,
            &self.slot_label,
            finality_unlock_root,
            rollback_sentinel_root,
            circuit_breaker_root,
            notary_height,
            notary_quorum,
        );
        next.blockers = next.active_blockers(config, circuit_breaker_armed, rollback_sentinel_open);
        next.status = if next.blockers.is_empty() {
            SettlementStatus::NotaryGuarded
        } else {
            SettlementStatus::Blocked
        };
        Ok(next)
    }

    pub fn mark_settlement_ready(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(SettlementError::SettlementStillBlocked);
        }
        let mut next = self.clone();
        next.status = SettlementStatus::SettlementReady;
        Ok(next)
    }

    fn active_blockers(
        &self,
        config: &Config,
        circuit_breaker_armed: bool,
        rollback_sentinel_open: bool,
    ) -> Vec<SettlementBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_finality_unlock_root && self.finality_unlock_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingFinalityUnlockRoot);
        }
        if config.require_execution_bundle_root && self.execution_bundle_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingExecutionBundleRoot);
        }
        if config.require_settlement_accounting_root && self.settlement_accounting_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingSettlementAccountingRoot);
        }
        if config.require_notary_quorum_root && self.notary_quorum_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingNotaryQuorumRoot);
        }
        if config.require_payout_envelope_root && self.payout_envelope_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingPayoutEnvelopeRoot);
        }
        if config.require_rollback_sentinel_root && self.rollback_sentinel_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingRollbackSentinelRoot);
        }
        if config.require_circuit_breaker_root && self.circuit_breaker_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(SettlementBlockerKind::MissingReviewerSignoffRoot);
        }
        match self.notary_height {
            Some(height) if height >= config.min_notary_height => {}
            _ => blockers.push(SettlementBlockerKind::NotaryHeightTooLow),
        }
        match self.notary_quorum {
            Some(quorum) if quorum >= config.min_notary_quorum => {}
            _ => blockers.push(SettlementBlockerKind::NotaryQuorumTooSmall),
        }
        if circuit_breaker_armed {
            blockers.push(SettlementBlockerKind::CircuitBreakerArmed);
        }
        if rollback_sentinel_open {
            blockers.push(SettlementBlockerKind::RollbackSentinelOpen);
        }
        if config.require_live_heavy_gate_evidence {
            blockers.push(SettlementBlockerKind::LiveHeavyGateEvidenceMissing);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave99_finality_unlock_root": self.wave99_finality_unlock_root,
            "finality_unlock_root": self.finality_unlock_root,
            "execution_bundle_root": self.execution_bundle_root,
            "settlement_accounting_root": self.settlement_accounting_root,
            "notary_quorum_root": self.notary_quorum_root,
            "payout_envelope_root": self.payout_envelope_root,
            "rollback_sentinel_root": self.rollback_sentinel_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "notary_height": self.notary_height,
            "notary_quorum": self.notary_quorum,
            "settlement_guard_root": self.settlement_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_execution_settlement", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneSettlementNotaryGuard {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave99_finality_transcript_root: String,
    pub settlements: Vec<ReleaseExecutionSettlement>,
    pub blocked_root: String,
    pub candidate_root: String,
    pub notary_guarded_root: String,
    pub settlement_ready_root: String,
    pub settlement_guard_root: String,
    pub payout_guard_root: String,
    pub rollback_guard_root: String,
    pub command_root: String,
    pub lane_status: SettlementStatus,
}

impl LaneSettlementNotaryGuard {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let settlements = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                ReleaseExecutionSettlement::empty(lane, label, index as u64, config)
            })
            .collect::<Vec<_>>();
        Self::from_settlements(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave99_finality_transcript", lane.as_str(), "source", WAVE),
            settlements,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_settlement(
        &self,
        slot_label: &str,
        finality_unlock_root: &str,
        execution_bundle_root: &str,
        settlement_accounting_root: &str,
        notary_quorum_root: &str,
        payout_envelope_root: &str,
        rollback_sentinel_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        notary_height: u64,
        notary_quorum: u64,
        config: &Config,
        circuit_breaker_armed: bool,
        rollback_sentinel_open: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut settlements = Vec::with_capacity(self.settlements.len());
        for settlement in &self.settlements {
            if settlement.slot_label == slot_label {
                settlements.push(settlement.stage_settlement(
                    finality_unlock_root,
                    execution_bundle_root,
                    settlement_accounting_root,
                    notary_quorum_root,
                    payout_envelope_root,
                    rollback_sentinel_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    notary_height,
                    notary_quorum,
                    config,
                    circuit_breaker_armed,
                    rollback_sentinel_open,
                )?);
                found = true;
            } else {
                settlements.push(settlement.clone());
            }
        }
        if !found {
            return Err(SettlementError::ClaimMissing);
        }
        Ok(Self::from_settlements(
            self.lane,
            self.lane_title_root.clone(),
            self.wave99_finality_transcript_root.clone(),
            settlements,
        ))
    }

    fn from_settlements(
        lane: LaneKind,
        lane_title_root: String,
        wave99_finality_transcript_root: String,
        settlements: Vec<ReleaseExecutionSettlement>,
    ) -> Self {
        let blocked_root = blocked_root(&settlements);
        let candidate_root = status_root(
            "wave100_candidate_settlements",
            &settlements,
            SettlementStatus::NotaryCandidate,
        );
        let notary_guarded_root = status_root(
            "wave100_notary_guarded_settlements",
            &settlements,
            SettlementStatus::NotaryGuarded,
        );
        let settlement_ready_root = status_root(
            "wave100_settlement_ready",
            &settlements,
            SettlementStatus::SettlementReady,
        );
        let settlement_guard_root = root_from_strings(
            "wave100_settlement_guards",
            settlements
                .iter()
                .map(|settlement| settlement.settlement_guard_root.clone()),
        );
        let payout_guard_root = root_from_strings(
            "wave100_payout_guards",
            settlements
                .iter()
                .map(|settlement| settlement.payout_guard_root.clone()),
        );
        let rollback_guard_root = root_from_strings(
            "wave100_rollback_guards",
            settlements
                .iter()
                .map(|settlement| settlement.rollback_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave100_settlement_commands",
            settlements
                .iter()
                .map(|settlement| settlement.command_hint_root.clone()),
        );
        let lane_status = if settlements
            .iter()
            .all(|settlement| settlement.status == SettlementStatus::SettlementReady)
        {
            SettlementStatus::SettlementReady
        } else if settlements
            .iter()
            .any(|settlement| settlement.status == SettlementStatus::NotaryGuarded)
        {
            SettlementStatus::NotaryGuarded
        } else if settlements
            .iter()
            .any(|settlement| settlement.status == SettlementStatus::NotaryCandidate)
        {
            SettlementStatus::NotaryCandidate
        } else {
            SettlementStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave99_finality_transcript_root,
            settlements,
            blocked_root,
            candidate_root,
            notary_guarded_root,
            settlement_ready_root,
            settlement_guard_root,
            payout_guard_root,
            rollback_guard_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.settlements
            .iter()
            .filter(|settlement| !settlement.blockers.is_empty())
            .count()
    }

    pub fn notary_guarded_count(&self) -> usize {
        self.settlements
            .iter()
            .filter(|settlement| settlement.status == SettlementStatus::NotaryGuarded)
            .count()
    }

    pub fn settlement_ready_count(&self) -> usize {
        self.settlements
            .iter()
            .filter(|settlement| settlement.status == SettlementStatus::SettlementReady)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave99_finality_transcript_root": self.wave99_finality_transcript_root,
            "settlement_roots": self.settlements.iter().map(ReleaseExecutionSettlement::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "notary_guarded_root": self.notary_guarded_root,
            "settlement_ready_root": self.settlement_ready_root,
            "settlement_guard_root": self.settlement_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "command_root": self.command_root,
            "claim_count": self.settlements.len(),
            "blocked_count": self.blocked_count(),
            "notary_guarded_count": self.notary_guarded_count(),
            "settlement_ready_count": self.settlement_ready_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_settlement_notary_guard", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementSummary {
    pub settlement_root: String,
    pub blocked_root: String,
    pub notary_guarded_root: String,
    pub settlement_ready_root: String,
    pub settlement_guard_root: String,
    pub payout_guard_root: String,
    pub rollback_guard_root: String,
    pub command_root: String,
    pub release_execution_denial_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub notary_guarded_count: usize,
    pub settlement_ready_count: usize,
    pub all_lanes_settlement_ready: bool,
    pub live_heavy_gates_ran: bool,
    pub production_release_execution_denied: bool,
}

impl SettlementSummary {
    pub fn from_guards(config: &Config, guards: &[LaneSettlementNotaryGuard]) -> Self {
        let settlement_root = root_from_strings(
            "wave100_settlement_root",
            guards.iter().map(LaneSettlementNotaryGuard::state_root),
        );
        let blocked_root = root_from_strings(
            "wave100_blocked_root",
            guards.iter().map(|guard| guard.blocked_root.clone()),
        );
        let notary_guarded_root = root_from_strings(
            "wave100_notary_guarded_root",
            guards.iter().map(|guard| guard.notary_guarded_root.clone()),
        );
        let settlement_ready_root = root_from_strings(
            "wave100_settlement_ready_root",
            guards
                .iter()
                .map(|guard| guard.settlement_ready_root.clone()),
        );
        let settlement_guard_root = root_from_strings(
            "wave100_settlement_guard_root",
            guards
                .iter()
                .map(|guard| guard.settlement_guard_root.clone()),
        );
        let payout_guard_root = root_from_strings(
            "wave100_payout_guard_root",
            guards.iter().map(|guard| guard.payout_guard_root.clone()),
        );
        let rollback_guard_root = root_from_strings(
            "wave100_rollback_guard_root",
            guards.iter().map(|guard| guard.rollback_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave100_command_root",
            guards.iter().map(|guard| guard.command_root.clone()),
        );
        let claim_count = guards
            .iter()
            .map(|guard| guard.settlements.len())
            .sum::<usize>();
        let blocked_count = guards
            .iter()
            .map(LaneSettlementNotaryGuard::blocked_count)
            .sum::<usize>();
        let notary_guarded_count = guards
            .iter()
            .map(LaneSettlementNotaryGuard::notary_guarded_count)
            .sum::<usize>();
        let settlement_ready_count = guards
            .iter()
            .map(LaneSettlementNotaryGuard::settlement_ready_count)
            .sum::<usize>();
        let all_lanes_settlement_ready = guards.len() as u64 >= config.global_settlement_threshold
            && guards
                .iter()
                .all(|guard| guard.lane_status == SettlementStatus::SettlementReady);
        let live_heavy_gates_ran = false;
        let production_release_execution_denied = config
            .deny_release_execution_when_any_lane_blocked
            && (!all_lanes_settlement_ready || !live_heavy_gates_ran);
        let denial_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "finality_wave": config.finality_wave,
            "blocked_count": blocked_count,
            "notary_guarded_count": notary_guarded_count,
            "settlement_ready_count": settlement_ready_count,
            "all_lanes_settlement_ready": all_lanes_settlement_ready,
            "live_heavy_gates_ran": live_heavy_gates_ran,
            "production_release_execution_denied": production_release_execution_denied,
        });
        let release_execution_denial_root = record_root("release_execution_denial", &denial_record);
        Self {
            settlement_root,
            blocked_root,
            notary_guarded_root,
            settlement_ready_root,
            settlement_guard_root,
            payout_guard_root,
            rollback_guard_root,
            command_root,
            release_execution_denial_root,
            lane_count: guards.len(),
            claim_count,
            blocked_count,
            notary_guarded_count,
            settlement_ready_count,
            all_lanes_settlement_ready,
            live_heavy_gates_ran,
            production_release_execution_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "settlement_root": self.settlement_root,
            "blocked_root": self.blocked_root,
            "notary_guarded_root": self.notary_guarded_root,
            "settlement_ready_root": self.settlement_ready_root,
            "settlement_guard_root": self.settlement_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "command_root": self.command_root,
            "release_execution_denial_root": self.release_execution_denial_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "notary_guarded_count": self.notary_guarded_count,
            "settlement_ready_count": self.settlement_ready_count,
            "all_lanes_settlement_ready": self.all_lanes_settlement_ready,
            "live_heavy_gates_ran": self.live_heavy_gates_ran,
            "production_release_execution_denied": self.production_release_execution_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("settlement_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave99_finality_transcript_root: String,
    pub guards: Vec<LaneSettlementNotaryGuard>,
    pub summary: SettlementSummary,
}

impl State {
    pub fn new(config: Config, guards: Vec<LaneSettlementNotaryGuard>) -> Self {
        let wave99_finality_transcript_root =
            label_root("wave99_finality_transcript", "all_lanes", "source", WAVE);
        let summary = SettlementSummary::from_guards(&config, &guards);
        Self {
            config,
            wave99_finality_transcript_root,
            guards,
            summary,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_release_execution_settlement(
        &self,
        lane: LaneKind,
        slot_label: &str,
        finality_unlock_root: &str,
        execution_bundle_root: &str,
        settlement_accounting_root: &str,
        notary_quorum_root: &str,
        payout_envelope_root: &str,
        rollback_sentinel_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        notary_height: u64,
        notary_quorum: u64,
        circuit_breaker_armed: bool,
        rollback_sentinel_open: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut guards = Vec::with_capacity(self.guards.len());
        for guard in &self.guards {
            if guard.lane == lane {
                guards.push(guard.stage_settlement(
                    slot_label,
                    finality_unlock_root,
                    execution_bundle_root,
                    settlement_accounting_root,
                    notary_quorum_root,
                    payout_envelope_root,
                    rollback_sentinel_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    notary_height,
                    notary_quorum,
                    &self.config,
                    circuit_breaker_armed,
                    rollback_sentinel_open,
                )?);
                found = true;
            } else {
                guards.push(guard.clone());
            }
        }
        if !found {
            return Err(SettlementError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), guards))
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave99_finality_transcript_root": self.wave99_finality_transcript_root,
            "guard_roots": self.guards.iter().map(LaneSettlementNotaryGuard::state_root).collect::<Vec<_>>(),
            "summary": self.summary.public_record(),
            "roots_only": self.config.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::default();
    let guards = vec![
        LaneSettlementNotaryGuard::new(
            LaneKind::Compile,
            &[
                "cargo_check",
                "cargo_test",
                "clippy",
                "rustfmt",
                "rustc",
                "build_metadata",
                "operator_signoff",
            ],
            &config,
        ),
        LaneSettlementNotaryGuard::new(
            LaneKind::RuntimeReplay,
            &[
                "replay_run",
                "rollback_drill",
                "adversarial_replay",
                "stale_archive_replacement",
                "live_execution_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneSettlementNotaryGuard::new(
            LaneKind::AuditSecurity,
            &[
                "audit_review",
                "adversarial_scenario",
                "threat_model",
                "privacy_review",
                "reviewer_signoff",
                "operator_signoff",
            ],
            &config,
        ),
        LaneSettlementNotaryGuard::new(
            LaneKind::BridgeCustody,
            &[
                "monero_watcher_quorum",
                "withdrawal_release",
                "reserve_coverage",
                "signer_quorum",
                "challenge_hold_review",
                "custody_operator_signoff",
            ],
            &config,
        ),
        LaneSettlementNotaryGuard::new(
            LaneKind::WalletWatchtower,
            &[
                "wallet_escape_dry_run",
                "watchtower_quorum",
                "user_runbook_replay",
                "redacted_recovery_proof",
                "wallet_visible_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneSettlementNotaryGuard::new(
            LaneKind::PqReservePrivacy,
            &[
                "ml_dsa_slh_dsa_authority_epoch",
                "pq_quorum",
                "reserve_coverage",
                "privacy_linkage",
                "metadata_redaction",
                "nullifier_separation",
                "operator_signoff",
            ],
            &config,
        ),
    ];
    State::new(config, guards)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<SettlementBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_finality_unlock_root {
        blockers.push(SettlementBlockerKind::MissingFinalityUnlockRoot);
    }
    if config.require_execution_bundle_root {
        blockers.push(SettlementBlockerKind::MissingExecutionBundleRoot);
    }
    if config.require_settlement_accounting_root {
        blockers.push(SettlementBlockerKind::MissingSettlementAccountingRoot);
    }
    if config.require_notary_quorum_root {
        blockers.push(SettlementBlockerKind::MissingNotaryQuorumRoot);
    }
    if config.require_payout_envelope_root {
        blockers.push(SettlementBlockerKind::MissingPayoutEnvelopeRoot);
    }
    if config.require_rollback_sentinel_root {
        blockers.push(SettlementBlockerKind::MissingRollbackSentinelRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(SettlementBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(SettlementBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(SettlementBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(SettlementBlockerKind::NotaryHeightTooLow);
    blockers.push(SettlementBlockerKind::NotaryQuorumTooSmall);
    blockers.push(SettlementBlockerKind::CircuitBreakerArmed);
    blockers.push(SettlementBlockerKind::RollbackSentinelOpen);
    if config.require_live_heavy_gate_evidence {
        blockers.push(SettlementBlockerKind::LiveHeavyGateEvidenceMissing);
    }
    blockers
}

fn blocked_root(settlements: &[ReleaseExecutionSettlement]) -> String {
    let leaves = settlements
        .iter()
        .flat_map(|settlement| {
            settlement.blockers.iter().map(move |blocker| {
                json!({
                    "lane": settlement.lane.as_str(),
                    "slot_label": settlement.slot_label,
                    "blocker": blocker.as_str(),
                    "settlement_root": settlement.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave100_blocked_settlement_notary_guards", &leaves)
}

fn status_root(
    domain: &str,
    settlements: &[ReleaseExecutionSettlement],
    status: SettlementStatus,
) -> String {
    root_from_strings(
        domain,
        settlements.iter().filter_map(|settlement| {
            if settlement.status == status {
                Some(settlement.state_root())
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

fn settlement_guard_root(
    guard_kind: &str,
    lane: LaneKind,
    slot_label: &str,
    finality_unlock_root: &str,
    first_guard_root: &str,
    second_guard_root: &str,
    notary_height: u64,
    notary_quorum: u64,
) -> String {
    domain_hash(
        "wave100-live-heavy-gate-release-execution-settlement-notary-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::Str(finality_unlock_root),
            HashPart::Str(first_guard_root),
            HashPart::Str(second_guard_root),
            HashPart::U64(notary_height),
            HashPart::U64(notary_quorum),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave100-live-heavy-gate-release-execution-settlement-notary-record",
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
        "wave100-live-heavy-gate-release-execution-settlement-notary-label",
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
