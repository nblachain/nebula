use crate::hash::merkle_root;
use serde_json::{json, Value};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave106-live-heavy-gate-release-execution-monero-release-credit-private-note-activation-nullifier-guard-wallet-watchtower-lane-runtime-v1";
const LANE_ID: &str =
    "wave106-live-heavy-gate-release-execution-monero-release-credit-private-note-activation-nullifier-guard-wallet-watchtower";
const WAVE: u64 = 106;
const ROOT_HEX_LEN: usize = 64;
const MIN_WATCHTOWER_QUORUM: u16 = 3;
const MIN_WALLET_HISTORY_EPOCH: u64 = 106_000;
const MIN_ACCOUNTING_EPOCH: u64 = 105_100;
const MIN_NOTE_COMMITMENT_COUNT: u32 = 1;
const MIN_NULLIFIER_RESERVATION_COUNT: u32 = 1;
const MIN_AMOUNT_BUCKET_COUNT: u32 = 1;
const MAX_AMOUNT_BUCKET_DRIFT_BPS: u16 = 1;
const MAX_FEE_REBATE_DRIFT_BPS: u16 = 2;
const MIN_LIABILITY_CLOSURE_BPS: u16 = 10_000;
const MIN_LIVE_EVIDENCE_ITEMS: u16 = 3;
const MIN_SIGNOFF_COUNT: u16 = 2;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = std::result::Result<T, ActivationGuardError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivationGuardError {
    InvalidConfig(&'static str),
    InvalidRoot(&'static str),
    MissingRoot(&'static str),
    PrivacyBoundaryViolated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GateKind {
    WatchtowerLane,
    NoteCommitment,
    NullifierReservation,
    WalletHistory,
    AmountBucket,
    Wave105Accounting,
    LiabilityClosure,
    FeeRebateSettlement,
    PqAuthorization,
    CircuitBreaker,
    LiveEvidence,
    Signoff,
    RootsOnly,
}

impl GateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WatchtowerLane => "watchtower_lane",
            Self::NoteCommitment => "note_commitment",
            Self::NullifierReservation => "nullifier_reservation",
            Self::WalletHistory => "wallet_history",
            Self::AmountBucket => "amount_bucket",
            Self::Wave105Accounting => "wave105_accounting",
            Self::LiabilityClosure => "liability_closure",
            Self::FeeRebateSettlement => "fee_rebate_settlement",
            Self::PqAuthorization => "pq_authorization",
            Self::CircuitBreaker => "circuit_breaker",
            Self::LiveEvidence => "live_evidence",
            Self::Signoff => "signoff",
            Self::RootsOnly => "roots_only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateStatus {
    Missing,
    Failed,
    Clear,
}

impl GateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Failed => "failed",
            Self::Clear => "clear",
        }
    }

    pub fn is_clear(self) -> bool {
        self == Self::Clear
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationDecision {
    FailClosed,
    RootsObserved,
    HeavyGateCandidate,
    ReleaseCreditCandidate,
    PrivateNoteActivationAllowed,
}

impl ActivationDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::RootsObserved => "roots_observed",
            Self::HeavyGateCandidate => "heavy_gate_candidate",
            Self::ReleaseCreditCandidate => "release_credit_candidate",
            Self::PrivateNoteActivationAllowed => "private_note_activation_allowed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub lane_id: String,
    pub min_watchtower_quorum: u16,
    pub min_wallet_history_epoch: u64,
    pub min_accounting_epoch: u64,
    pub min_note_commitment_count: u32,
    pub min_nullifier_reservation_count: u32,
    pub min_amount_bucket_count: u32,
    pub max_amount_bucket_drift_bps: u16,
    pub max_fee_rebate_drift_bps: u16,
    pub min_liability_closure_bps: u16,
    pub min_live_evidence_items: u16,
    pub min_signoff_count: u16,
    pub require_watchtower_lane_root: bool,
    pub require_note_commitment_root: bool,
    pub require_nullifier_reservation_root: bool,
    pub require_wallet_history_root: bool,
    pub require_amount_bucket_root: bool,
    pub require_wave105_accounting_root: bool,
    pub require_liability_closure_root: bool,
    pub require_fee_rebate_settlement_root: bool,
    pub require_pq_authorization_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_live_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
    pub require_roots_only_manifest_root: bool,
    pub require_private_payload_absent: bool,
    pub arm_circuit_breaker_by_default: bool,
    pub heavy_gates_ran: bool,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            lane_id: LANE_ID.to_string(),
            min_watchtower_quorum: MIN_WATCHTOWER_QUORUM,
            min_wallet_history_epoch: MIN_WALLET_HISTORY_EPOCH,
            min_accounting_epoch: MIN_ACCOUNTING_EPOCH,
            min_note_commitment_count: MIN_NOTE_COMMITMENT_COUNT,
            min_nullifier_reservation_count: MIN_NULLIFIER_RESERVATION_COUNT,
            min_amount_bucket_count: MIN_AMOUNT_BUCKET_COUNT,
            max_amount_bucket_drift_bps: MAX_AMOUNT_BUCKET_DRIFT_BPS,
            max_fee_rebate_drift_bps: MAX_FEE_REBATE_DRIFT_BPS,
            min_liability_closure_bps: MIN_LIABILITY_CLOSURE_BPS,
            min_live_evidence_items: MIN_LIVE_EVIDENCE_ITEMS,
            min_signoff_count: MIN_SIGNOFF_COUNT,
            require_watchtower_lane_root: true,
            require_note_commitment_root: true,
            require_nullifier_reservation_root: true,
            require_wallet_history_root: true,
            require_amount_bucket_root: true,
            require_wave105_accounting_root: true,
            require_liability_closure_root: true,
            require_fee_rebate_settlement_root: true,
            require_pq_authorization_root: true,
            require_circuit_breaker_root: true,
            require_live_evidence_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
            require_roots_only_manifest_root: true,
            require_private_payload_absent: true,
            arm_circuit_breaker_by_default: true,
            heavy_gates_ran: false,
            release_credit_allowed: false,
            note_activation_allowed: false,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.chain_id != CHAIN_ID {
            return Err(ActivationGuardError::InvalidConfig("chain_id"));
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(ActivationGuardError::InvalidConfig("protocol_version"));
        }
        if self.wave != WAVE {
            return Err(ActivationGuardError::InvalidConfig("wave"));
        }
        if self.lane_id != LANE_ID {
            return Err(ActivationGuardError::InvalidConfig("lane_id"));
        }
        if self.min_watchtower_quorum == 0 {
            return Err(ActivationGuardError::InvalidConfig("min_watchtower_quorum"));
        }
        if self.min_liability_closure_bps < MIN_LIABILITY_CLOSURE_BPS {
            return Err(ActivationGuardError::InvalidConfig(
                "min_liability_closure_bps",
            ));
        }
        if self.min_signoff_count < 2 {
            return Err(ActivationGuardError::InvalidConfig("min_signoff_count"));
        }
        if !self.require_roots_only_manifest_root || !self.require_private_payload_absent {
            return Err(ActivationGuardError::PrivacyBoundaryViolated);
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "wave": self.wave,
            "lane_id": self.lane_id,
            "min_watchtower_quorum": self.min_watchtower_quorum,
            "min_wallet_history_epoch": self.min_wallet_history_epoch,
            "min_accounting_epoch": self.min_accounting_epoch,
            "min_note_commitment_count": self.min_note_commitment_count,
            "min_nullifier_reservation_count": self.min_nullifier_reservation_count,
            "min_amount_bucket_count": self.min_amount_bucket_count,
            "max_amount_bucket_drift_bps": self.max_amount_bucket_drift_bps,
            "max_fee_rebate_drift_bps": self.max_fee_rebate_drift_bps,
            "min_liability_closure_bps": self.min_liability_closure_bps,
            "min_live_evidence_items": self.min_live_evidence_items,
            "min_signoff_count": self.min_signoff_count,
            "require_roots_only_manifest_root": self.require_roots_only_manifest_root,
            "require_private_payload_absent": self.require_private_payload_absent,
            "arm_circuit_breaker_by_default": self.arm_circuit_breaker_by_default,
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "fail_closed_defaults": fail_closed_snippets(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootBundle {
    pub watchtower_lane_root: String,
    pub note_commitment_root: String,
    pub nullifier_reservation_root: String,
    pub wallet_history_root: String,
    pub amount_bucket_root: String,
    pub wave105_accounting_root: String,
    pub liability_closure_root: String,
    pub fee_rebate_settlement_root: String,
    pub pq_authorization_root: String,
    pub circuit_breaker_root: String,
    pub live_evidence_root: String,
    pub operator_signoff_root: String,
    pub reviewer_signoff_root: String,
    pub roots_only_manifest_root: String,
}

impl Default for RootBundle {
    fn default() -> Self {
        Self {
            watchtower_lane_root: String::new(),
            note_commitment_root: String::new(),
            nullifier_reservation_root: String::new(),
            wallet_history_root: String::new(),
            amount_bucket_root: String::new(),
            wave105_accounting_root: String::new(),
            liability_closure_root: String::new(),
            fee_rebate_settlement_root: String::new(),
            pq_authorization_root: String::new(),
            circuit_breaker_root: String::new(),
            live_evidence_root: String::new(),
            operator_signoff_root: String::new(),
            reviewer_signoff_root: String::new(),
            roots_only_manifest_root: String::new(),
        }
    }
}

impl RootBundle {
    pub fn root_for(&self, gate: GateKind) -> &str {
        match gate {
            GateKind::WatchtowerLane => &self.watchtower_lane_root,
            GateKind::NoteCommitment => &self.note_commitment_root,
            GateKind::NullifierReservation => &self.nullifier_reservation_root,
            GateKind::WalletHistory => &self.wallet_history_root,
            GateKind::AmountBucket => &self.amount_bucket_root,
            GateKind::Wave105Accounting => &self.wave105_accounting_root,
            GateKind::LiabilityClosure => &self.liability_closure_root,
            GateKind::FeeRebateSettlement => &self.fee_rebate_settlement_root,
            GateKind::PqAuthorization => &self.pq_authorization_root,
            GateKind::CircuitBreaker => &self.circuit_breaker_root,
            GateKind::LiveEvidence => &self.live_evidence_root,
            GateKind::Signoff => &self.operator_signoff_root,
            GateKind::RootsOnly => &self.roots_only_manifest_root,
        }
    }

    pub fn validate_required(&self, config: &Config) -> Result<()> {
        for check in required_root_checks(self, config) {
            validate_root_when_required(check.name, check.root, check.required)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "roots",
            "watchtower_lane_root": root_value(&self.watchtower_lane_root),
            "note_commitment_root": root_value(&self.note_commitment_root),
            "nullifier_reservation_root": root_value(&self.nullifier_reservation_root),
            "wallet_history_root": root_value(&self.wallet_history_root),
            "amount_bucket_root": root_value(&self.amount_bucket_root),
            "wave105_accounting_root": root_value(&self.wave105_accounting_root),
            "liability_closure_root": root_value(&self.liability_closure_root),
            "fee_rebate_settlement_root": root_value(&self.fee_rebate_settlement_root),
            "pq_authorization_root": root_value(&self.pq_authorization_root),
            "circuit_breaker_root": root_value(&self.circuit_breaker_root),
            "live_evidence_root": root_value(&self.live_evidence_root),
            "operator_signoff_root": root_value(&self.operator_signoff_root),
            "reviewer_signoff_root": root_value(&self.reviewer_signoff_root),
            "roots_only_manifest_root": root_value(&self.roots_only_manifest_root),
        })
    }

    pub fn has_any_root(&self) -> bool {
        required_root_checks(self, &Config::default())
            .iter()
            .any(|entry| !entry.root.is_empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceStatus {
    pub watchtower_quorum: u16,
    pub note_commitment_count: u32,
    pub nullifier_reservation_count: u32,
    pub wallet_history_epoch: u64,
    pub amount_bucket_count: u32,
    pub amount_bucket_drift_bps: u16,
    pub wave105_accounting_epoch: u64,
    pub liability_closure_bps: u16,
    pub fee_rebate_drift_bps: u16,
    pub pq_authorized: bool,
    pub circuit_breaker_armed: bool,
    pub live_evidence_items: u16,
    pub operator_signed: bool,
    pub reviewer_signed: bool,
    pub release_captain_signed: bool,
    pub roots_only: bool,
    pub private_payload_absent: bool,
    pub wallet_identity_disclosed: bool,
    pub note_opening_disclosed: bool,
    pub nullifier_preimage_disclosed: bool,
}

impl Default for EvidenceStatus {
    fn default() -> Self {
        Self {
            watchtower_quorum: 0,
            note_commitment_count: 0,
            nullifier_reservation_count: 0,
            wallet_history_epoch: 0,
            amount_bucket_count: 0,
            amount_bucket_drift_bps: u16::MAX,
            wave105_accounting_epoch: 0,
            liability_closure_bps: 0,
            fee_rebate_drift_bps: u16::MAX,
            pq_authorized: false,
            circuit_breaker_armed: true,
            live_evidence_items: 0,
            operator_signed: false,
            reviewer_signed: false,
            release_captain_signed: false,
            roots_only: true,
            private_payload_absent: true,
            wallet_identity_disclosed: false,
            note_opening_disclosed: false,
            nullifier_preimage_disclosed: false,
        }
    }
}

impl EvidenceStatus {
    pub fn signoff_count(&self) -> u16 {
        [
            self.operator_signed,
            self.reviewer_signed,
            self.release_captain_signed,
        ]
        .iter()
        .filter(|flag| **flag)
        .count() as u16
    }

    pub fn privacy_boundary_clear(&self) -> bool {
        self.roots_only
            && self.private_payload_absent
            && !self.wallet_identity_disclosed
            && !self.note_opening_disclosed
            && !self.nullifier_preimage_disclosed
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "evidence_status",
            "watchtower_quorum": self.watchtower_quorum,
            "note_commitment_count": self.note_commitment_count,
            "nullifier_reservation_count": self.nullifier_reservation_count,
            "wallet_history_epoch": self.wallet_history_epoch,
            "amount_bucket_count": self.amount_bucket_count,
            "amount_bucket_drift_bps": self.amount_bucket_drift_bps,
            "wave105_accounting_epoch": self.wave105_accounting_epoch,
            "liability_closure_bps": self.liability_closure_bps,
            "fee_rebate_drift_bps": self.fee_rebate_drift_bps,
            "pq_authorized": self.pq_authorized,
            "circuit_breaker_armed": self.circuit_breaker_armed,
            "live_evidence_items": self.live_evidence_items,
            "signoff_count": self.signoff_count(),
            "roots_only": self.roots_only,
            "private_payload_absent": self.private_payload_absent,
            "wallet_identity_disclosed": self.wallet_identity_disclosed,
            "note_opening_disclosed": self.note_opening_disclosed,
            "nullifier_preimage_disclosed": self.nullifier_preimage_disclosed,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateReport {
    pub gate: GateKind,
    pub status: GateStatus,
    pub reason: String,
    pub root: String,
}

impl GateReport {
    pub fn new(gate: GateKind, status: GateStatus, reason: &str, root: &str) -> Self {
        Self {
            gate,
            status,
            reason: reason.to_string(),
            root: root.to_string(),
        }
    }

    pub fn clear(gate: GateKind, root: &str) -> Self {
        Self::new(gate, GateStatus::Clear, "clear", root)
    }

    pub fn failed(gate: GateKind, reason: &str, root: &str) -> Self {
        Self::new(gate, GateStatus::Failed, reason, root)
    }

    pub fn missing(gate: GateKind, reason: &str) -> Self {
        Self::new(gate, GateStatus::Missing, reason, "")
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "gate_report",
            "gate": self.gate.as_str(),
            "status": self.status.as_str(),
            "reason": self.reason,
            "root": root_value(&self.root),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivacyBoundary {
    pub roots_only: bool,
    pub private_payload_absent: bool,
    pub wallet_identity_disclosed: bool,
    pub note_opening_disclosed: bool,
    pub nullifier_preimage_disclosed: bool,
}

impl PrivacyBoundary {
    pub fn from_evidence(evidence: &EvidenceStatus) -> Self {
        Self {
            roots_only: evidence.roots_only,
            private_payload_absent: evidence.private_payload_absent,
            wallet_identity_disclosed: evidence.wallet_identity_disclosed,
            note_opening_disclosed: evidence.note_opening_disclosed,
            nullifier_preimage_disclosed: evidence.nullifier_preimage_disclosed,
        }
    }

    pub fn is_clear(&self) -> bool {
        self.roots_only
            && self.private_payload_absent
            && !self.wallet_identity_disclosed
            && !self.note_opening_disclosed
            && !self.nullifier_preimage_disclosed
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "privacy_boundary",
            "roots_only": self.roots_only,
            "private_payload_absent": self.private_payload_absent,
            "wallet_identity_disclosed": self.wallet_identity_disclosed,
            "note_opening_disclosed": self.note_opening_disclosed,
            "nullifier_preimage_disclosed": self.nullifier_preimage_disclosed,
            "boundary_clear": self.is_clear(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub config: Config,
    pub roots: RootBundle,
    pub evidence: EvidenceStatus,
    pub privacy_boundary: PrivacyBoundary,
    pub gate_reports: Vec<GateReport>,
    pub decision: ActivationDecision,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub state_root: String,
}

impl Default for State {
    fn default() -> Self {
        let config = Config::default();
        Self::fail_closed_with(config)
    }
}

impl State {
    pub fn new(config: Config, roots: RootBundle, evidence: EvidenceStatus) -> Result<Self> {
        config.validate()?;
        roots.validate_required(&config)?;
        if config.require_private_payload_absent && !evidence.privacy_boundary_clear() {
            return Err(ActivationGuardError::PrivacyBoundaryViolated);
        }
        let privacy_boundary = PrivacyBoundary::from_evidence(&evidence);
        let gate_reports = evaluate_gates(&config, &roots, &evidence);
        let all_clear = gate_reports.iter().all(|report| report.status.is_clear());
        let heavy_gates_ran = config.heavy_gates_ran && all_clear && privacy_boundary.is_clear();
        let release_credit_allowed = config.release_credit_allowed && heavy_gates_ran;
        let note_activation_allowed =
            config.note_activation_allowed && release_credit_allowed && heavy_gates_ran;
        let decision = activation_decision(
            &roots,
            &evidence,
            heavy_gates_ran,
            release_credit_allowed,
            note_activation_allowed,
        );
        let state_root = compute_state_root(
            &config,
            &roots,
            &evidence,
            &privacy_boundary,
            &gate_reports,
            decision,
        );
        Ok(Self {
            config,
            roots,
            evidence,
            privacy_boundary,
            gate_reports,
            decision,
            release_credit_allowed,
            note_activation_allowed,
            heavy_gates_ran,
            state_root,
        })
    }

    pub fn fail_closed_with(config: Config) -> Self {
        let roots = RootBundle::default();
        let evidence = EvidenceStatus::default();
        let privacy_boundary = PrivacyBoundary::from_evidence(&evidence);
        let gate_reports = evaluate_gates(&config, &roots, &evidence);
        let decision = ActivationDecision::FailClosed;
        let state_root = compute_state_root(
            &config,
            &roots,
            &evidence,
            &privacy_boundary,
            &gate_reports,
            decision,
        );
        Self {
            config,
            roots,
            evidence,
            privacy_boundary,
            gate_reports,
            decision,
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            state_root,
        }
    }

    pub fn denied_reasons(&self) -> Vec<String> {
        self.gate_reports
            .iter()
            .filter(|report| !report.status.is_clear())
            .map(|report| format!("{}:{}", report.gate.as_str(), report.reason))
            .collect()
    }

    pub fn gate_report_records(&self) -> Vec<PublicRecord> {
        self.gate_reports
            .iter()
            .map(GateReport::public_record)
            .collect()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "record": "wallet_watchtower_private_note_activation_status",
            "chain_id": self.config.chain_id,
            "protocol_version": self.config.protocol_version,
            "wave": self.config.wave,
            "lane_id": self.config.lane_id,
            "decision": self.decision.as_str(),
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "state_root": self.state_root,
            "roots": self.roots.public_record(),
            "evidence_status": self.evidence.public_record(),
            "privacy_boundary": self.privacy_boundary.public_record(),
            "denied_reasons": self.denied_reasons(),
            "fail_closed_defaults": fail_closed_snippets(),
            "gate_reports": self.gate_report_records(),
        })
    }
}

pub fn devnet() -> Runtime {
    State::fail_closed_with(Config::default())
}

pub fn public_record(runtime: &Runtime) -> PublicRecord {
    runtime.public_record()
}

pub fn state_root(runtime: &Runtime) -> String {
    runtime.state_root.clone()
}

pub fn evaluate_gates(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> Vec<GateReport> {
    vec![
        evaluate_watchtower_lane(config, roots, evidence),
        evaluate_note_commitment(config, roots, evidence),
        evaluate_nullifier_reservation(config, roots, evidence),
        evaluate_wallet_history(config, roots, evidence),
        evaluate_amount_bucket(config, roots, evidence),
        evaluate_wave105_accounting(config, roots, evidence),
        evaluate_liability_closure(config, roots, evidence),
        evaluate_fee_rebate_settlement(config, roots, evidence),
        evaluate_pq_authorization(config, roots, evidence),
        evaluate_circuit_breaker(config, roots, evidence),
        evaluate_live_evidence(config, roots, evidence),
        evaluate_signoff(config, roots, evidence),
        evaluate_roots_only(config, roots, evidence),
    ]
}

fn evaluate_watchtower_lane(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::WatchtowerLane;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_watchtower_lane_root,
        "missing_watchtower_lane_root",
        "invalid_watchtower_lane_root",
    ) {
        return report;
    }
    if evidence.watchtower_quorum < config.min_watchtower_quorum {
        return GateReport::failed(gate, "watchtower_quorum_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_note_commitment(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::NoteCommitment;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_note_commitment_root,
        "missing_note_commitment_root",
        "invalid_note_commitment_root",
    ) {
        return report;
    }
    if evidence.note_commitment_count < config.min_note_commitment_count {
        return GateReport::failed(gate, "note_commitment_count_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_nullifier_reservation(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::NullifierReservation;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_nullifier_reservation_root,
        "missing_nullifier_reservation_root",
        "invalid_nullifier_reservation_root",
    ) {
        return report;
    }
    if evidence.nullifier_reservation_count < config.min_nullifier_reservation_count {
        return GateReport::failed(gate, "nullifier_reservation_count_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_wallet_history(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::WalletHistory;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_wallet_history_root,
        "missing_wallet_history_root",
        "invalid_wallet_history_root",
    ) {
        return report;
    }
    if evidence.wallet_history_epoch < config.min_wallet_history_epoch {
        return GateReport::failed(gate, "wallet_history_epoch_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_amount_bucket(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::AmountBucket;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_amount_bucket_root,
        "missing_amount_bucket_root",
        "invalid_amount_bucket_root",
    ) {
        return report;
    }
    if evidence.amount_bucket_count < config.min_amount_bucket_count {
        return GateReport::failed(gate, "amount_bucket_count_too_low", root);
    }
    if evidence.amount_bucket_drift_bps > config.max_amount_bucket_drift_bps {
        return GateReport::failed(gate, "amount_bucket_drift_too_high", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_wave105_accounting(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::Wave105Accounting;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_wave105_accounting_root,
        "missing_wave105_accounting_root",
        "invalid_wave105_accounting_root",
    ) {
        return report;
    }
    if evidence.wave105_accounting_epoch < config.min_accounting_epoch {
        return GateReport::failed(gate, "wave105_accounting_epoch_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_liability_closure(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::LiabilityClosure;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_liability_closure_root,
        "missing_liability_closure_root",
        "invalid_liability_closure_root",
    ) {
        return report;
    }
    if evidence.liability_closure_bps < config.min_liability_closure_bps {
        return GateReport::failed(gate, "liability_closure_incomplete", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_fee_rebate_settlement(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::FeeRebateSettlement;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_fee_rebate_settlement_root,
        "missing_fee_rebate_settlement_root",
        "invalid_fee_rebate_settlement_root",
    ) {
        return report;
    }
    if evidence.fee_rebate_drift_bps > config.max_fee_rebate_drift_bps {
        return GateReport::failed(gate, "fee_rebate_drift_too_high", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_pq_authorization(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::PqAuthorization;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_pq_authorization_root,
        "missing_pq_authorization_root",
        "invalid_pq_authorization_root",
    ) {
        return report;
    }
    if !evidence.pq_authorized {
        return GateReport::failed(gate, "pq_authorization_missing", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_circuit_breaker(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::CircuitBreaker;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_circuit_breaker_root,
        "missing_circuit_breaker_root",
        "invalid_circuit_breaker_root",
    ) {
        return report;
    }
    if config.arm_circuit_breaker_by_default || evidence.circuit_breaker_armed {
        return GateReport::failed(gate, "circuit_breaker_armed", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_live_evidence(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::LiveEvidence;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_live_evidence_root,
        "missing_live_evidence_root",
        "invalid_live_evidence_root",
    ) {
        return report;
    }
    if evidence.live_evidence_items < config.min_live_evidence_items {
        return GateReport::failed(gate, "live_evidence_items_too_low", root);
    }
    GateReport::clear(gate, root)
}

fn evaluate_signoff(config: &Config, roots: &RootBundle, evidence: &EvidenceStatus) -> GateReport {
    let gate = GateKind::Signoff;
    if config.require_operator_signoff_root && roots.operator_signoff_root.is_empty() {
        return GateReport::missing(gate, "missing_operator_signoff_root");
    }
    if config.require_reviewer_signoff_root && roots.reviewer_signoff_root.is_empty() {
        return GateReport::missing(gate, "missing_reviewer_signoff_root");
    }
    if !is_valid_root(&roots.operator_signoff_root) {
        return GateReport::failed(
            gate,
            "invalid_operator_signoff_root",
            &roots.operator_signoff_root,
        );
    }
    if !is_valid_root(&roots.reviewer_signoff_root) {
        return GateReport::failed(
            gate,
            "invalid_reviewer_signoff_root",
            &roots.reviewer_signoff_root,
        );
    }
    if evidence.signoff_count() < config.min_signoff_count {
        return GateReport::failed(gate, "signoff_quorum_too_low", &roots.operator_signoff_root);
    }
    GateReport::clear(gate, &roots.operator_signoff_root)
}

fn evaluate_roots_only(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
) -> GateReport {
    let gate = GateKind::RootsOnly;
    let root = roots.root_for(gate);
    if let Some(report) = root_gate_report(
        gate,
        root,
        config.require_roots_only_manifest_root,
        "missing_roots_only_manifest_root",
        "invalid_roots_only_manifest_root",
    ) {
        return report;
    }
    if !evidence.privacy_boundary_clear() {
        return GateReport::failed(gate, "roots_only_privacy_boundary_violated", root);
    }
    GateReport::clear(gate, root)
}

#[derive(Clone, Copy)]
struct RootCheck<'a> {
    name: &'static str,
    root: &'a str,
    required: bool,
}

fn required_root_checks<'a>(roots: &'a RootBundle, config: &Config) -> Vec<RootCheck<'a>> {
    vec![
        RootCheck {
            name: "watchtower_lane_root",
            root: &roots.watchtower_lane_root,
            required: config.require_watchtower_lane_root,
        },
        RootCheck {
            name: "note_commitment_root",
            root: &roots.note_commitment_root,
            required: config.require_note_commitment_root,
        },
        RootCheck {
            name: "nullifier_reservation_root",
            root: &roots.nullifier_reservation_root,
            required: config.require_nullifier_reservation_root,
        },
        RootCheck {
            name: "wallet_history_root",
            root: &roots.wallet_history_root,
            required: config.require_wallet_history_root,
        },
        RootCheck {
            name: "amount_bucket_root",
            root: &roots.amount_bucket_root,
            required: config.require_amount_bucket_root,
        },
        RootCheck {
            name: "wave105_accounting_root",
            root: &roots.wave105_accounting_root,
            required: config.require_wave105_accounting_root,
        },
        RootCheck {
            name: "liability_closure_root",
            root: &roots.liability_closure_root,
            required: config.require_liability_closure_root,
        },
        RootCheck {
            name: "fee_rebate_settlement_root",
            root: &roots.fee_rebate_settlement_root,
            required: config.require_fee_rebate_settlement_root,
        },
        RootCheck {
            name: "pq_authorization_root",
            root: &roots.pq_authorization_root,
            required: config.require_pq_authorization_root,
        },
        RootCheck {
            name: "circuit_breaker_root",
            root: &roots.circuit_breaker_root,
            required: config.require_circuit_breaker_root,
        },
        RootCheck {
            name: "live_evidence_root",
            root: &roots.live_evidence_root,
            required: config.require_live_evidence_root,
        },
        RootCheck {
            name: "operator_signoff_root",
            root: &roots.operator_signoff_root,
            required: config.require_operator_signoff_root,
        },
        RootCheck {
            name: "reviewer_signoff_root",
            root: &roots.reviewer_signoff_root,
            required: config.require_reviewer_signoff_root,
        },
        RootCheck {
            name: "roots_only_manifest_root",
            root: &roots.roots_only_manifest_root,
            required: config.require_roots_only_manifest_root,
        },
    ]
}

fn validate_root_when_required(name: &'static str, root: &str, required: bool) -> Result<()> {
    if required && root.is_empty() {
        return Err(ActivationGuardError::MissingRoot(name));
    }
    if !root.is_empty() && !is_valid_root(root) {
        return Err(ActivationGuardError::InvalidRoot(name));
    }
    Ok(())
}

fn root_gate_report(
    gate: GateKind,
    root: &str,
    required: bool,
    missing: &str,
    invalid: &str,
) -> Option<GateReport> {
    if required && root.is_empty() {
        return Some(GateReport::missing(gate, missing));
    }
    if !root.is_empty() && !is_valid_root(root) {
        return Some(GateReport::failed(gate, invalid, root));
    }
    if required && !is_valid_root(root) {
        return Some(GateReport::failed(gate, invalid, root));
    }
    None
}

fn is_valid_root(root: &str) -> bool {
    root.len() == ROOT_HEX_LEN && root.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn root_value(root: &str) -> Value {
    if root.is_empty() {
        json!("missing")
    } else {
        json!(root)
    }
}

fn fail_closed_snippets() -> Vec<String> {
    vec![
        "release_credit_allowed: false".to_string(),
        "note_activation_allowed: false".to_string(),
        "heavy_gates_ran: false".to_string(),
    ]
}

fn activation_decision(
    roots: &RootBundle,
    evidence: &EvidenceStatus,
    heavy_gates_ran: bool,
    release_credit_allowed: bool,
    note_activation_allowed: bool,
) -> ActivationDecision {
    if note_activation_allowed {
        ActivationDecision::PrivateNoteActivationAllowed
    } else if release_credit_allowed {
        ActivationDecision::ReleaseCreditCandidate
    } else if heavy_gates_ran {
        ActivationDecision::HeavyGateCandidate
    } else if roots.has_any_root() || evidence.live_evidence_items > 0 {
        ActivationDecision::RootsObserved
    } else {
        ActivationDecision::FailClosed
    }
}

fn compute_state_root(
    config: &Config,
    roots: &RootBundle,
    evidence: &EvidenceStatus,
    privacy_boundary: &PrivacyBoundary,
    gate_reports: &[GateReport],
    decision: ActivationDecision,
) -> String {
    let leaves = vec![
        config.public_record(),
        roots.public_record(),
        evidence.public_record(),
        privacy_boundary.public_record(),
        json!({
            "record": "decision",
            "decision": decision.as_str(),
            "fail_closed_defaults": fail_closed_snippets(),
        }),
        gate_report_root(gate_reports),
    ];
    merkle_root(PROTOCOL_VERSION, &leaves)
}

fn gate_report_root(gate_reports: &[GateReport]) -> PublicRecord {
    let leaves = gate_reports
        .iter()
        .map(GateReport::public_record)
        .collect::<Vec<_>>();
    json!({
        "record": "gate_report_root",
        "gate_count": leaves.len(),
        "root": merkle_root("wave106-gate-report-root", &leaves),
    })
}
