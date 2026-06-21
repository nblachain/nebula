use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard-v1";
const WAVE: u64 = 101;
const SETTLEMENT_WAVE: u64 = 100;
const MIN_LIQUIDITY_DEPTH: u64 = 1_010_000;
const MAX_FEE_BPS: u64 = 15;
const MIN_THROTTLE_BLOCKS: u64 = 360;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, DisbursementError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DisbursementError {
    LaneMissing,
    ClaimMissing,
    SettlementNotaryRootMissing,
    LiquidityReservationRootMissing,
    FeeNettingRootMissing,
    PayoutThrottleRootMissing,
    DisbursementBatchRootMissing,
    PrivacyBudgetRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    LiquidityDepthTooLow,
    FeeRateTooHigh,
    ThrottleWindowOpen,
    CircuitBreakerArmed,
    DisbursementStillBlocked,
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
            Self::Compile => "Compile disbursement liquidity throttle guard",
            Self::RuntimeReplay => "Runtime replay disbursement liquidity throttle guard",
            Self::AuditSecurity => "Audit security disbursement liquidity throttle guard",
            Self::BridgeCustody => "Bridge custody disbursement liquidity throttle guard",
            Self::WalletWatchtower => "Wallet watchtower disbursement liquidity throttle guard",
            Self::PqReservePrivacy => "PQ reserve privacy disbursement liquidity throttle guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DisbursementStatus {
    Empty,
    Blocked,
    LiquidityCandidate,
    ThrottleGuarded,
    DisbursementReady,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DisbursementBlockerKind {
    MissingSettlementNotaryRoot,
    MissingLiquidityReservationRoot,
    MissingFeeNettingRoot,
    MissingPayoutThrottleRoot,
    MissingDisbursementBatchRoot,
    MissingPrivacyBudgetRoot,
    MissingCircuitBreakerRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    LiquidityDepthTooLow,
    FeeRateTooHigh,
    ThrottleWindowOpen,
    CircuitBreakerArmed,
    LiveHeavyGateEvidenceMissing,
}

impl DisbursementBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingSettlementNotaryRoot => "missing_settlement_notary_root",
            Self::MissingLiquidityReservationRoot => "missing_liquidity_reservation_root",
            Self::MissingFeeNettingRoot => "missing_fee_netting_root",
            Self::MissingPayoutThrottleRoot => "missing_payout_throttle_root",
            Self::MissingDisbursementBatchRoot => "missing_disbursement_batch_root",
            Self::MissingPrivacyBudgetRoot => "missing_privacy_budget_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::LiquidityDepthTooLow => "liquidity_depth_too_low",
            Self::FeeRateTooHigh => "fee_rate_too_high",
            Self::ThrottleWindowOpen => "throttle_window_open",
            Self::CircuitBreakerArmed => "circuit_breaker_armed",
            Self::LiveHeavyGateEvidenceMissing => "live_heavy_gate_evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub settlement_wave: u64,
    pub min_liquidity_depth: u64,
    pub max_fee_bps: u64,
    pub min_throttle_blocks: u64,
    pub lane_disbursement_threshold: u64,
    pub global_disbursement_threshold: u64,
    pub require_settlement_notary_root: bool,
    pub require_liquidity_reservation_root: bool,
    pub require_fee_netting_root: bool,
    pub require_payout_throttle_root: bool,
    pub require_disbursement_batch_root: bool,
    pub require_privacy_budget_root: bool,
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
            settlement_wave: SETTLEMENT_WAVE,
            min_liquidity_depth: MIN_LIQUIDITY_DEPTH,
            max_fee_bps: MAX_FEE_BPS,
            min_throttle_blocks: MIN_THROTTLE_BLOCKS,
            lane_disbursement_threshold: 1,
            global_disbursement_threshold: 6,
            require_settlement_notary_root: true,
            require_liquidity_reservation_root: true,
            require_fee_netting_root: true,
            require_payout_throttle_root: true,
            require_disbursement_batch_root: true,
            require_privacy_budget_root: true,
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
            "settlement_wave": self.settlement_wave,
            "min_liquidity_depth": self.min_liquidity_depth,
            "max_fee_bps": self.max_fee_bps,
            "min_throttle_blocks": self.min_throttle_blocks,
            "lane_disbursement_threshold": self.lane_disbursement_threshold,
            "global_disbursement_threshold": self.global_disbursement_threshold,
            "require_settlement_notary_root": self.require_settlement_notary_root,
            "require_liquidity_reservation_root": self.require_liquidity_reservation_root,
            "require_fee_netting_root": self.require_fee_netting_root,
            "require_payout_throttle_root": self.require_payout_throttle_root,
            "require_disbursement_batch_root": self.require_disbursement_batch_root,
            "require_privacy_budget_root": self.require_privacy_budget_root,
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
pub struct ReleaseExecutionDisbursement {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave100_settlement_notary_root: String,
    pub settlement_notary_root: Option<String>,
    pub liquidity_reservation_root: Option<String>,
    pub fee_netting_root: Option<String>,
    pub payout_throttle_root: Option<String>,
    pub disbursement_batch_root: Option<String>,
    pub privacy_budget_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub liquidity_depth: Option<u64>,
    pub fee_bps: Option<u64>,
    pub throttle_blocks: Option<u64>,
    pub liquidity_guard_root: String,
    pub fee_guard_root: String,
    pub payout_guard_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<DisbursementBlockerKind>,
    pub status: DisbursementStatus,
}

impl ReleaseExecutionDisbursement {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave100_settlement_notary_root: label_root(
                "wave100_settlement_notary",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            settlement_notary_root: None,
            liquidity_reservation_root: None,
            fee_netting_root: None,
            payout_throttle_root: None,
            disbursement_batch_root: None,
            privacy_budget_root: None,
            circuit_breaker_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            liquidity_depth: None,
            fee_bps: None,
            throttle_blocks: None,
            liquidity_guard_root: label_root("liquidity_guard", lane.as_str(), slot_label, ordinal),
            fee_guard_root: label_root("fee_guard", lane.as_str(), slot_label, ordinal),
            payout_guard_root: label_root("payout_guard", lane.as_str(), slot_label, ordinal),
            command_hint_root: label_root(
                "release_execution_disbursement_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: DisbursementStatus::Blocked,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_disbursement(
        &self,
        settlement_notary_root: &str,
        liquidity_reservation_root: &str,
        fee_netting_root: &str,
        payout_throttle_root: &str,
        disbursement_batch_root: &str,
        privacy_budget_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        liquidity_depth: u64,
        fee_bps: u64,
        throttle_blocks: u64,
        config: &Config,
        throttle_open: bool,
        circuit_breaker_armed: bool,
    ) -> Result<Self> {
        if settlement_notary_root.is_empty() {
            return Err(DisbursementError::SettlementNotaryRootMissing);
        }
        if liquidity_reservation_root.is_empty() {
            return Err(DisbursementError::LiquidityReservationRootMissing);
        }
        if fee_netting_root.is_empty() {
            return Err(DisbursementError::FeeNettingRootMissing);
        }
        if payout_throttle_root.is_empty() {
            return Err(DisbursementError::PayoutThrottleRootMissing);
        }
        if disbursement_batch_root.is_empty() {
            return Err(DisbursementError::DisbursementBatchRootMissing);
        }
        if privacy_budget_root.is_empty() {
            return Err(DisbursementError::PrivacyBudgetRootMissing);
        }
        if circuit_breaker_root.is_empty() {
            return Err(DisbursementError::CircuitBreakerRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(DisbursementError::OperatorSignoffRootMissing);
        }
        if reviewer_signoff_root.is_empty() {
            return Err(DisbursementError::ReviewerSignoffRootMissing);
        }
        if liquidity_depth < config.min_liquidity_depth {
            return Err(DisbursementError::LiquidityDepthTooLow);
        }
        if fee_bps > config.max_fee_bps {
            return Err(DisbursementError::FeeRateTooHigh);
        }
        if throttle_open || throttle_blocks < config.min_throttle_blocks {
            return Err(DisbursementError::ThrottleWindowOpen);
        }
        if circuit_breaker_armed {
            return Err(DisbursementError::CircuitBreakerArmed);
        }

        let mut next = self.clone();
        next.settlement_notary_root = Some(settlement_notary_root.to_string());
        next.liquidity_reservation_root = Some(liquidity_reservation_root.to_string());
        next.fee_netting_root = Some(fee_netting_root.to_string());
        next.payout_throttle_root = Some(payout_throttle_root.to_string());
        next.disbursement_batch_root = Some(disbursement_batch_root.to_string());
        next.privacy_budget_root = Some(privacy_budget_root.to_string());
        next.circuit_breaker_root = Some(circuit_breaker_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.liquidity_depth = Some(liquidity_depth);
        next.fee_bps = Some(fee_bps);
        next.throttle_blocks = Some(throttle_blocks);
        next.liquidity_guard_root = disbursement_guard_root(
            "liquidity",
            self.lane,
            &self.slot_label,
            settlement_notary_root,
            liquidity_reservation_root,
            disbursement_batch_root,
            liquidity_depth,
            fee_bps,
            throttle_blocks,
        );
        next.fee_guard_root = disbursement_guard_root(
            "fee",
            self.lane,
            &self.slot_label,
            settlement_notary_root,
            fee_netting_root,
            privacy_budget_root,
            liquidity_depth,
            fee_bps,
            throttle_blocks,
        );
        next.payout_guard_root = disbursement_guard_root(
            "payout",
            self.lane,
            &self.slot_label,
            settlement_notary_root,
            payout_throttle_root,
            circuit_breaker_root,
            liquidity_depth,
            fee_bps,
            throttle_blocks,
        );
        next.blockers = next.active_blockers(config, throttle_open, circuit_breaker_armed);
        next.status = if next.blockers.is_empty() {
            DisbursementStatus::ThrottleGuarded
        } else {
            DisbursementStatus::Blocked
        };
        Ok(next)
    }

    pub fn mark_disbursement_ready(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(DisbursementError::DisbursementStillBlocked);
        }
        let mut next = self.clone();
        next.status = DisbursementStatus::DisbursementReady;
        Ok(next)
    }

    fn active_blockers(
        &self,
        config: &Config,
        throttle_open: bool,
        circuit_breaker_armed: bool,
    ) -> Vec<DisbursementBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_settlement_notary_root && self.settlement_notary_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingSettlementNotaryRoot);
        }
        if config.require_liquidity_reservation_root && self.liquidity_reservation_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingLiquidityReservationRoot);
        }
        if config.require_fee_netting_root && self.fee_netting_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingFeeNettingRoot);
        }
        if config.require_payout_throttle_root && self.payout_throttle_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingPayoutThrottleRoot);
        }
        if config.require_disbursement_batch_root && self.disbursement_batch_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingDisbursementBatchRoot);
        }
        if config.require_privacy_budget_root && self.privacy_budget_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingPrivacyBudgetRoot);
        }
        if config.require_circuit_breaker_root && self.circuit_breaker_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(DisbursementBlockerKind::MissingReviewerSignoffRoot);
        }
        match self.liquidity_depth {
            Some(depth) if depth >= config.min_liquidity_depth => {}
            _ => blockers.push(DisbursementBlockerKind::LiquidityDepthTooLow),
        }
        match self.fee_bps {
            Some(fee) if fee <= config.max_fee_bps => {}
            _ => blockers.push(DisbursementBlockerKind::FeeRateTooHigh),
        }
        match self.throttle_blocks {
            Some(blocks) if blocks >= config.min_throttle_blocks => {}
            _ => blockers.push(DisbursementBlockerKind::ThrottleWindowOpen),
        }
        if throttle_open {
            blockers.push(DisbursementBlockerKind::ThrottleWindowOpen);
        }
        if circuit_breaker_armed {
            blockers.push(DisbursementBlockerKind::CircuitBreakerArmed);
        }
        if config.require_live_heavy_gate_evidence {
            blockers.push(DisbursementBlockerKind::LiveHeavyGateEvidenceMissing);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave100_settlement_notary_root": self.wave100_settlement_notary_root,
            "settlement_notary_root": self.settlement_notary_root,
            "liquidity_reservation_root": self.liquidity_reservation_root,
            "fee_netting_root": self.fee_netting_root,
            "payout_throttle_root": self.payout_throttle_root,
            "disbursement_batch_root": self.disbursement_batch_root,
            "privacy_budget_root": self.privacy_budget_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "liquidity_depth": self.liquidity_depth,
            "fee_bps": self.fee_bps,
            "throttle_blocks": self.throttle_blocks,
            "liquidity_guard_root": self.liquidity_guard_root,
            "fee_guard_root": self.fee_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_execution_disbursement", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneDisbursementLiquidityGuard {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave100_settlement_transcript_root: String,
    pub disbursements: Vec<ReleaseExecutionDisbursement>,
    pub blocked_root: String,
    pub candidate_root: String,
    pub throttle_guarded_root: String,
    pub disbursement_ready_root: String,
    pub liquidity_guard_root: String,
    pub fee_guard_root: String,
    pub payout_guard_root: String,
    pub command_root: String,
    pub lane_status: DisbursementStatus,
}

impl LaneDisbursementLiquidityGuard {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let disbursements = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                ReleaseExecutionDisbursement::empty(lane, label, index as u64, config)
            })
            .collect::<Vec<_>>();
        Self::from_disbursements(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root(
                "wave100_settlement_transcript",
                lane.as_str(),
                "source",
                WAVE,
            ),
            disbursements,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_disbursement(
        &self,
        slot_label: &str,
        settlement_notary_root: &str,
        liquidity_reservation_root: &str,
        fee_netting_root: &str,
        payout_throttle_root: &str,
        disbursement_batch_root: &str,
        privacy_budget_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        liquidity_depth: u64,
        fee_bps: u64,
        throttle_blocks: u64,
        config: &Config,
        throttle_open: bool,
        circuit_breaker_armed: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut disbursements = Vec::with_capacity(self.disbursements.len());
        for disbursement in &self.disbursements {
            if disbursement.slot_label == slot_label {
                disbursements.push(disbursement.stage_disbursement(
                    settlement_notary_root,
                    liquidity_reservation_root,
                    fee_netting_root,
                    payout_throttle_root,
                    disbursement_batch_root,
                    privacy_budget_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    liquidity_depth,
                    fee_bps,
                    throttle_blocks,
                    config,
                    throttle_open,
                    circuit_breaker_armed,
                )?);
                found = true;
            } else {
                disbursements.push(disbursement.clone());
            }
        }
        if !found {
            return Err(DisbursementError::ClaimMissing);
        }
        Ok(Self::from_disbursements(
            self.lane,
            self.lane_title_root.clone(),
            self.wave100_settlement_transcript_root.clone(),
            disbursements,
        ))
    }

    fn from_disbursements(
        lane: LaneKind,
        lane_title_root: String,
        wave100_settlement_transcript_root: String,
        disbursements: Vec<ReleaseExecutionDisbursement>,
    ) -> Self {
        let blocked_root = blocked_root(&disbursements);
        let candidate_root = status_root(
            "wave101_candidate_disbursements",
            &disbursements,
            DisbursementStatus::LiquidityCandidate,
        );
        let throttle_guarded_root = status_root(
            "wave101_throttle_guarded_disbursements",
            &disbursements,
            DisbursementStatus::ThrottleGuarded,
        );
        let disbursement_ready_root = status_root(
            "wave101_disbursement_ready",
            &disbursements,
            DisbursementStatus::DisbursementReady,
        );
        let liquidity_guard_root = root_from_strings(
            "wave101_liquidity_guards",
            disbursements
                .iter()
                .map(|disbursement| disbursement.liquidity_guard_root.clone()),
        );
        let fee_guard_root = root_from_strings(
            "wave101_fee_guards",
            disbursements
                .iter()
                .map(|disbursement| disbursement.fee_guard_root.clone()),
        );
        let payout_guard_root = root_from_strings(
            "wave101_payout_guards",
            disbursements
                .iter()
                .map(|disbursement| disbursement.payout_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave101_disbursement_commands",
            disbursements
                .iter()
                .map(|disbursement| disbursement.command_hint_root.clone()),
        );
        let lane_status = if disbursements
            .iter()
            .all(|disbursement| disbursement.status == DisbursementStatus::DisbursementReady)
        {
            DisbursementStatus::DisbursementReady
        } else if disbursements
            .iter()
            .any(|disbursement| disbursement.status == DisbursementStatus::ThrottleGuarded)
        {
            DisbursementStatus::ThrottleGuarded
        } else if disbursements
            .iter()
            .any(|disbursement| disbursement.status == DisbursementStatus::LiquidityCandidate)
        {
            DisbursementStatus::LiquidityCandidate
        } else {
            DisbursementStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave100_settlement_transcript_root,
            disbursements,
            blocked_root,
            candidate_root,
            throttle_guarded_root,
            disbursement_ready_root,
            liquidity_guard_root,
            fee_guard_root,
            payout_guard_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.disbursements
            .iter()
            .filter(|disbursement| !disbursement.blockers.is_empty())
            .count()
    }

    pub fn throttle_guarded_count(&self) -> usize {
        self.disbursements
            .iter()
            .filter(|disbursement| disbursement.status == DisbursementStatus::ThrottleGuarded)
            .count()
    }

    pub fn disbursement_ready_count(&self) -> usize {
        self.disbursements
            .iter()
            .filter(|disbursement| disbursement.status == DisbursementStatus::DisbursementReady)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave100_settlement_transcript_root": self.wave100_settlement_transcript_root,
            "disbursement_roots": self.disbursements.iter().map(ReleaseExecutionDisbursement::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "throttle_guarded_root": self.throttle_guarded_root,
            "disbursement_ready_root": self.disbursement_ready_root,
            "liquidity_guard_root": self.liquidity_guard_root,
            "fee_guard_root": self.fee_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "command_root": self.command_root,
            "claim_count": self.disbursements.len(),
            "blocked_count": self.blocked_count(),
            "throttle_guarded_count": self.throttle_guarded_count(),
            "disbursement_ready_count": self.disbursement_ready_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_disbursement_liquidity_guard", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisbursementSummary {
    pub disbursement_root: String,
    pub blocked_root: String,
    pub throttle_guarded_root: String,
    pub disbursement_ready_root: String,
    pub liquidity_guard_root: String,
    pub fee_guard_root: String,
    pub payout_guard_root: String,
    pub command_root: String,
    pub release_execution_denial_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub throttle_guarded_count: usize,
    pub disbursement_ready_count: usize,
    pub all_lanes_disbursement_ready: bool,
    pub live_heavy_gates_ran: bool,
    pub production_release_execution_denied: bool,
}

impl DisbursementSummary {
    pub fn from_guards(config: &Config, guards: &[LaneDisbursementLiquidityGuard]) -> Self {
        let disbursement_root = root_from_strings(
            "wave101_disbursement_root",
            guards
                .iter()
                .map(LaneDisbursementLiquidityGuard::state_root),
        );
        let blocked_root = root_from_strings(
            "wave101_blocked_root",
            guards.iter().map(|guard| guard.blocked_root.clone()),
        );
        let throttle_guarded_root = root_from_strings(
            "wave101_throttle_guarded_root",
            guards
                .iter()
                .map(|guard| guard.throttle_guarded_root.clone()),
        );
        let disbursement_ready_root = root_from_strings(
            "wave101_disbursement_ready_root",
            guards
                .iter()
                .map(|guard| guard.disbursement_ready_root.clone()),
        );
        let liquidity_guard_root = root_from_strings(
            "wave101_liquidity_guard_root",
            guards
                .iter()
                .map(|guard| guard.liquidity_guard_root.clone()),
        );
        let fee_guard_root = root_from_strings(
            "wave101_fee_guard_root",
            guards.iter().map(|guard| guard.fee_guard_root.clone()),
        );
        let payout_guard_root = root_from_strings(
            "wave101_payout_guard_root",
            guards.iter().map(|guard| guard.payout_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave101_command_root",
            guards.iter().map(|guard| guard.command_root.clone()),
        );
        let claim_count = guards
            .iter()
            .map(|guard| guard.disbursements.len())
            .sum::<usize>();
        let blocked_count = guards
            .iter()
            .map(LaneDisbursementLiquidityGuard::blocked_count)
            .sum::<usize>();
        let throttle_guarded_count = guards
            .iter()
            .map(LaneDisbursementLiquidityGuard::throttle_guarded_count)
            .sum::<usize>();
        let disbursement_ready_count = guards
            .iter()
            .map(LaneDisbursementLiquidityGuard::disbursement_ready_count)
            .sum::<usize>();
        let all_lanes_disbursement_ready = guards.len() as u64
            >= config.global_disbursement_threshold
            && guards
                .iter()
                .all(|guard| guard.lane_status == DisbursementStatus::DisbursementReady);
        let live_heavy_gates_ran = false;
        let production_release_execution_denied = config
            .deny_release_execution_when_any_lane_blocked
            && (!all_lanes_disbursement_ready || !live_heavy_gates_ran);
        let denial_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "settlement_wave": config.settlement_wave,
            "blocked_count": blocked_count,
            "throttle_guarded_count": throttle_guarded_count,
            "disbursement_ready_count": disbursement_ready_count,
            "all_lanes_disbursement_ready": all_lanes_disbursement_ready,
            "live_heavy_gates_ran": live_heavy_gates_ran,
            "production_release_execution_denied": production_release_execution_denied,
        });
        let release_execution_denial_root = record_root("release_execution_denial", &denial_record);
        Self {
            disbursement_root,
            blocked_root,
            throttle_guarded_root,
            disbursement_ready_root,
            liquidity_guard_root,
            fee_guard_root,
            payout_guard_root,
            command_root,
            release_execution_denial_root,
            lane_count: guards.len(),
            claim_count,
            blocked_count,
            throttle_guarded_count,
            disbursement_ready_count,
            all_lanes_disbursement_ready,
            live_heavy_gates_ran,
            production_release_execution_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "disbursement_root": self.disbursement_root,
            "blocked_root": self.blocked_root,
            "throttle_guarded_root": self.throttle_guarded_root,
            "disbursement_ready_root": self.disbursement_ready_root,
            "liquidity_guard_root": self.liquidity_guard_root,
            "fee_guard_root": self.fee_guard_root,
            "payout_guard_root": self.payout_guard_root,
            "command_root": self.command_root,
            "release_execution_denial_root": self.release_execution_denial_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "throttle_guarded_count": self.throttle_guarded_count,
            "disbursement_ready_count": self.disbursement_ready_count,
            "all_lanes_disbursement_ready": self.all_lanes_disbursement_ready,
            "live_heavy_gates_ran": self.live_heavy_gates_ran,
            "production_release_execution_denied": self.production_release_execution_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("disbursement_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave100_settlement_transcript_root: String,
    pub guards: Vec<LaneDisbursementLiquidityGuard>,
    pub summary: DisbursementSummary,
}

impl State {
    pub fn new(config: Config, guards: Vec<LaneDisbursementLiquidityGuard>) -> Self {
        let wave100_settlement_transcript_root =
            label_root("wave100_settlement_transcript", "all_lanes", "source", WAVE);
        let summary = DisbursementSummary::from_guards(&config, &guards);
        Self {
            config,
            wave100_settlement_transcript_root,
            guards,
            summary,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_release_execution_disbursement(
        &self,
        lane: LaneKind,
        slot_label: &str,
        settlement_notary_root: &str,
        liquidity_reservation_root: &str,
        fee_netting_root: &str,
        payout_throttle_root: &str,
        disbursement_batch_root: &str,
        privacy_budget_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        liquidity_depth: u64,
        fee_bps: u64,
        throttle_blocks: u64,
        throttle_open: bool,
        circuit_breaker_armed: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut guards = Vec::with_capacity(self.guards.len());
        for guard in &self.guards {
            if guard.lane == lane {
                guards.push(guard.stage_disbursement(
                    slot_label,
                    settlement_notary_root,
                    liquidity_reservation_root,
                    fee_netting_root,
                    payout_throttle_root,
                    disbursement_batch_root,
                    privacy_budget_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    liquidity_depth,
                    fee_bps,
                    throttle_blocks,
                    &self.config,
                    throttle_open,
                    circuit_breaker_armed,
                )?);
                found = true;
            } else {
                guards.push(guard.clone());
            }
        }
        if !found {
            return Err(DisbursementError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), guards))
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave100_settlement_transcript_root": self.wave100_settlement_transcript_root,
            "guard_roots": self.guards.iter().map(LaneDisbursementLiquidityGuard::state_root).collect::<Vec<_>>(),
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
        LaneDisbursementLiquidityGuard::new(
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
        LaneDisbursementLiquidityGuard::new(
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
        LaneDisbursementLiquidityGuard::new(
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
        LaneDisbursementLiquidityGuard::new(
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
        LaneDisbursementLiquidityGuard::new(
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
        LaneDisbursementLiquidityGuard::new(
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

fn initial_blockers(config: &Config) -> Vec<DisbursementBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_settlement_notary_root {
        blockers.push(DisbursementBlockerKind::MissingSettlementNotaryRoot);
    }
    if config.require_liquidity_reservation_root {
        blockers.push(DisbursementBlockerKind::MissingLiquidityReservationRoot);
    }
    if config.require_fee_netting_root {
        blockers.push(DisbursementBlockerKind::MissingFeeNettingRoot);
    }
    if config.require_payout_throttle_root {
        blockers.push(DisbursementBlockerKind::MissingPayoutThrottleRoot);
    }
    if config.require_disbursement_batch_root {
        blockers.push(DisbursementBlockerKind::MissingDisbursementBatchRoot);
    }
    if config.require_privacy_budget_root {
        blockers.push(DisbursementBlockerKind::MissingPrivacyBudgetRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(DisbursementBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(DisbursementBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(DisbursementBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(DisbursementBlockerKind::LiquidityDepthTooLow);
    blockers.push(DisbursementBlockerKind::FeeRateTooHigh);
    blockers.push(DisbursementBlockerKind::ThrottleWindowOpen);
    blockers.push(DisbursementBlockerKind::CircuitBreakerArmed);
    if config.require_live_heavy_gate_evidence {
        blockers.push(DisbursementBlockerKind::LiveHeavyGateEvidenceMissing);
    }
    blockers
}

fn blocked_root(disbursements: &[ReleaseExecutionDisbursement]) -> String {
    let leaves = disbursements
        .iter()
        .flat_map(|disbursement| {
            disbursement.blockers.iter().map(move |blocker| {
                json!({
                    "lane": disbursement.lane.as_str(),
                    "slot_label": disbursement.slot_label,
                    "blocker": blocker.as_str(),
                    "disbursement_root": disbursement.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave101_blocked_disbursement_liquidity_guards", &leaves)
}

fn status_root(
    domain: &str,
    disbursements: &[ReleaseExecutionDisbursement],
    status: DisbursementStatus,
) -> String {
    root_from_strings(
        domain,
        disbursements.iter().filter_map(|disbursement| {
            if disbursement.status == status {
                Some(disbursement.state_root())
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

fn disbursement_guard_root(
    guard_kind: &str,
    lane: LaneKind,
    slot_label: &str,
    settlement_notary_root: &str,
    first_guard_root: &str,
    second_guard_root: &str,
    liquidity_depth: u64,
    fee_bps: u64,
    throttle_blocks: u64,
) -> String {
    domain_hash(
        "wave101-live-heavy-gate-release-execution-disbursement-liquidity-throttle-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::Str(settlement_notary_root),
            HashPart::Str(first_guard_root),
            HashPart::Str(second_guard_root),
            HashPart::U64(liquidity_depth),
            HashPart::U64(fee_bps),
            HashPart::U64(throttle_blocks),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave101-live-heavy-gate-release-execution-disbursement-liquidity-record",
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
        "wave101-live-heavy-gate-release-execution-disbursement-liquidity-label",
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
