use std::collections::BTreeMap;

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const LANE_ID: &str = "bridge_custody";
const PROTOCOL_VERSION: &str =
    "wave106-live-heavy-gate-release-credit-private-note-activation-nullifier-guard-bridge-custody-lane-runtime-v1";
const WAVE: u64 = 106;
const PREVIOUS_WAVE: u64 = 105;
const MIN_REORG_CLEARANCE_DEPTH: u64 = 720;
const MIN_RELEASE_QUEUE_AGE: u64 = 48;
const MIN_HEAVY_GATE_ROUNDS: u64 = 7;
const DEFAULT_RESERVE_FLOOR_ATOMIC_UNITS: u128 = 1_000_000_000_000;
const DEFAULT_SIGNOFF_QUORUM: u16 = 5;
const MAX_ACCOUNTING_DELTA_ATOMIC_UNITS: i128 = 0;
const MAX_FEE_REBATE_DRIFT_ATOMIC_UNITS: i128 = 0;
const MAX_NOTE_COMMITMENT_DRIFT: i128 = 0;
const MAX_NULLIFIER_DRIFT: i128 = 0;

pub type PublicRecord = Record;
pub type Runtime = State;
pub type Result<T> = std::result::Result<T, ActivationGuardError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldValue {
    Text(String),
    Bool(bool),
    List(Vec<String>),
}

impl FieldValue {
    pub fn render(&self) -> String {
        match self {
            Self::Text(value) => value.clone(),
            Self::Bool(value) => value.to_string(),
            Self::List(values) => values.join(","),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Record {
    fields: BTreeMap<String, FieldValue>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }

    pub fn insert_text(&mut self, key: &str, value: &str) {
        self.fields
            .insert(key.to_string(), FieldValue::Text(value.to_string()));
    }

    pub fn insert_bool(&mut self, key: &str, value: bool) {
        self.fields.insert(key.to_string(), FieldValue::Bool(value));
    }

    pub fn insert_list(&mut self, key: &str, value: Vec<String>) {
        self.fields.insert(key.to_string(), FieldValue::List(value));
    }

    pub fn canonical_string(&self) -> String {
        let mut out = String::new();
        for (key, value) in &self.fields {
            out.push_str(key);
            out.push('=');
            out.push_str(&value.render());
            out.push(';');
        }
        out
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationGuardError {
    EmptyField(&'static str),
    InvalidRoot(&'static str),
    InvalidAmount(&'static str),
    InvalidThreshold(&'static str),
    MissingRoot(RootKind),
    MissingSignoff(SignoffRole),
    CircuitBreakerArmed,
    ReorgClearanceTooLow,
    ReleaseQueueOpen,
    ReserveLiabilityOpen,
    CustodyAccountingOpen,
    Wave105CreditRootMissing,
    NoteCommitmentMismatch,
    NullifierReservationMismatch,
    FeeRebateSettlementOpen,
    PqAuthorizationMissing,
    HeavyGateEvidenceMissing,
    HeavyGateRoundsTooLow,
    ReleaseCreditStillBlocked,
    NoteActivationStillBlocked,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RootKind {
    Wave105CreditAccounting,
    Wave105ReleaseLedger,
    CustodyReserve,
    LiabilityClosure,
    ReleaseQueueClosure,
    CustodyAccounting,
    NoteCommitment,
    NullifierReservation,
    FeeSettlement,
    RebateSettlement,
    PqAuthorization,
    CircuitBreaker,
    LiveHeavyGateEvidence,
    MoneroReserveAudit,
    OperatorSignoff,
    ReviewerSignoff,
    ReleaseCaptainSignoff,
    CustodyControllerSignoff,
    PrivacyOfficerSignoff,
}

impl RootKind {
    pub fn all() -> [Self; 19] {
        [
            Self::Wave105CreditAccounting,
            Self::Wave105ReleaseLedger,
            Self::CustodyReserve,
            Self::LiabilityClosure,
            Self::ReleaseQueueClosure,
            Self::CustodyAccounting,
            Self::NoteCommitment,
            Self::NullifierReservation,
            Self::FeeSettlement,
            Self::RebateSettlement,
            Self::PqAuthorization,
            Self::CircuitBreaker,
            Self::LiveHeavyGateEvidence,
            Self::MoneroReserveAudit,
            Self::OperatorSignoff,
            Self::ReviewerSignoff,
            Self::ReleaseCaptainSignoff,
            Self::CustodyControllerSignoff,
            Self::PrivacyOfficerSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave105CreditAccounting => "wave105_credit_accounting",
            Self::Wave105ReleaseLedger => "wave105_release_ledger",
            Self::CustodyReserve => "custody_reserve",
            Self::LiabilityClosure => "liability_closure",
            Self::ReleaseQueueClosure => "release_queue_closure",
            Self::CustodyAccounting => "custody_accounting",
            Self::NoteCommitment => "note_commitment",
            Self::NullifierReservation => "nullifier_reservation",
            Self::FeeSettlement => "fee_settlement",
            Self::RebateSettlement => "rebate_settlement",
            Self::PqAuthorization => "pq_authorization",
            Self::CircuitBreaker => "circuit_breaker",
            Self::LiveHeavyGateEvidence => "live_heavy_gate_evidence",
            Self::MoneroReserveAudit => "monero_reserve_audit",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::ReleaseCaptainSignoff => "release_captain_signoff",
            Self::CustodyControllerSignoff => "custody_controller_signoff",
            Self::PrivacyOfficerSignoff => "privacy_officer_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SignoffRole {
    Operator,
    Reviewer,
    ReleaseCaptain,
    CustodyController,
    PrivacyOfficer,
    PqKeyCustodian,
}

impl SignoffRole {
    pub fn all() -> [Self; 6] {
        [
            Self::Operator,
            Self::Reviewer,
            Self::ReleaseCaptain,
            Self::CustodyController,
            Self::PrivacyOfficer,
            Self::PqKeyCustodian,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::Reviewer => "reviewer",
            Self::ReleaseCaptain => "release_captain",
            Self::CustodyController => "custody_controller",
            Self::PrivacyOfficer => "privacy_officer",
            Self::PqKeyCustodian => "pq_key_custodian",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateStatus {
    Empty,
    Blocked,
    ReserveClosing,
    QueueClosing,
    NullifierReserved,
    Ready,
}

impl GateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Blocked => "blocked",
            Self::ReserveClosing => "reserve_closing",
            Self::QueueClosing => "queue_closing",
            Self::NullifierReserved => "nullifier_reserved",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub previous_wave: u64,
    pub min_reorg_clearance_depth: u64,
    pub min_release_queue_age: u64,
    pub min_heavy_gate_rounds: u64,
    pub reserve_floor_atomic_units: u128,
    pub signoff_quorum: u16,
    pub max_accounting_delta_atomic_units: i128,
    pub max_fee_rebate_drift_atomic_units: i128,
    pub max_note_commitment_drift: i128,
    pub max_nullifier_drift: i128,
    pub roots_only_public_records: bool,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            previous_wave: PREVIOUS_WAVE,
            min_reorg_clearance_depth: MIN_REORG_CLEARANCE_DEPTH,
            min_release_queue_age: MIN_RELEASE_QUEUE_AGE,
            min_heavy_gate_rounds: MIN_HEAVY_GATE_ROUNDS,
            reserve_floor_atomic_units: DEFAULT_RESERVE_FLOOR_ATOMIC_UNITS,
            signoff_quorum: DEFAULT_SIGNOFF_QUORUM,
            max_accounting_delta_atomic_units: MAX_ACCOUNTING_DELTA_ATOMIC_UNITS,
            max_fee_rebate_drift_atomic_units: MAX_FEE_REBATE_DRIFT_ATOMIC_UNITS,
            max_note_commitment_drift: MAX_NOTE_COMMITMENT_DRIFT,
            max_nullifier_drift: MAX_NULLIFIER_DRIFT,
            roots_only_public_records: true,
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_nonzero("wave", self.wave)?;
        ensure_nonzero("previous_wave", self.previous_wave)?;
        ensure_nonzero("min_reorg_clearance_depth", self.min_reorg_clearance_depth)?;
        ensure_nonzero("min_release_queue_age", self.min_release_queue_age)?;
        ensure_nonzero("min_heavy_gate_rounds", self.min_heavy_gate_rounds)?;
        ensure_nonzero("signoff_quorum", u64::from(self.signoff_quorum))?;
        if self.wave <= self.previous_wave {
            return Err(ActivationGuardError::InvalidThreshold("wave"));
        }
        if self.reserve_floor_atomic_units == 0 {
            return Err(ActivationGuardError::InvalidAmount(
                "reserve_floor_atomic_units",
            ));
        }
        if self.max_accounting_delta_atomic_units != 0 {
            return Err(ActivationGuardError::InvalidThreshold(
                "max_accounting_delta_atomic_units",
            ));
        }
        if self.max_fee_rebate_drift_atomic_units != 0 {
            return Err(ActivationGuardError::InvalidThreshold(
                "max_fee_rebate_drift_atomic_units",
            ));
        }
        if self.max_note_commitment_drift != 0 {
            return Err(ActivationGuardError::InvalidThreshold(
                "max_note_commitment_drift",
            ));
        }
        if self.max_nullifier_drift != 0 {
            return Err(ActivationGuardError::InvalidThreshold(
                "max_nullifier_drift",
            ));
        }
        if !self.roots_only_public_records {
            return Err(ActivationGuardError::InvalidThreshold(
                "roots_only_public_records",
            ));
        }
        if self.release_credit_allowed {
            return Err(ActivationGuardError::ReleaseCreditStillBlocked);
        }
        if self.note_activation_allowed {
            return Err(ActivationGuardError::NoteActivationStillBlocked);
        }
        if self.heavy_gates_ran {
            return Err(ActivationGuardError::HeavyGateEvidenceMissing);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRoot {
    pub kind: RootKind,
    pub root: String,
    pub source: String,
    pub observed_height: u64,
    pub accepted: bool,
}

impl EvidenceRoot {
    pub fn rejected(kind: RootKind, root: &str, source: &str, observed_height: u64) -> Self {
        Self {
            kind,
            root: root.to_string(),
            source: source.to_string(),
            observed_height,
            accepted: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("root", &self.root)?;
        ensure_text("source", &self.source)?;
        ensure_nonzero("observed_height", self.observed_height)?;
        if !self.accepted {
            return Err(ActivationGuardError::MissingRoot(self.kind));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signoff {
    pub role: SignoffRole,
    pub signer_root: String,
    pub attestation_root: String,
    pub approved: bool,
}

impl Signoff {
    pub fn missing(role: SignoffRole) -> Self {
        Self {
            role,
            signer_root: String::new(),
            attestation_root: String::new(),
            approved: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if !self.approved {
            return Err(ActivationGuardError::MissingSignoff(self.role));
        }
        ensure_root("signer_root", &self.signer_root)?;
        ensure_root("attestation_root", &self.attestation_root)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave105CreditRoots {
    pub credit_accounting_root: String,
    pub release_ledger_root: String,
    pub custody_lane_root: String,
    pub private_note_preimage_root: String,
    pub roots_accepted: bool,
}

impl Wave105CreditRoots {
    pub fn blocked() -> Self {
        Self {
            credit_accounting_root: "root:wave105-credit-accounting:withheld".to_string(),
            release_ledger_root: "root:wave105-release-ledger:withheld".to_string(),
            custody_lane_root: "root:wave105-custody-lane:withheld".to_string(),
            private_note_preimage_root: "root:wave105-private-note-preimage:withheld".to_string(),
            roots_accepted: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("credit_accounting_root", &self.credit_accounting_root)?;
        ensure_root("release_ledger_root", &self.release_ledger_root)?;
        ensure_root("custody_lane_root", &self.custody_lane_root)?;
        ensure_root(
            "private_note_preimage_root",
            &self.private_note_preimage_root,
        )?;
        if !self.roots_accepted {
            return Err(ActivationGuardError::Wave105CreditRootMissing);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReserveLiabilityClosure {
    pub reserve_root: String,
    pub liability_root: String,
    pub reserve_atomic_units: u128,
    pub liabilities_atomic_units: u128,
    pub reserve_floor_atomic_units: u128,
    pub closed: bool,
}

impl ReserveLiabilityClosure {
    pub fn blocked(config: &Config) -> Self {
        Self {
            reserve_root: "root:custody-reserve:unclosed".to_string(),
            liability_root: "root:custody-liability:unclosed".to_string(),
            reserve_atomic_units: 0,
            liabilities_atomic_units: config.reserve_floor_atomic_units,
            reserve_floor_atomic_units: config.reserve_floor_atomic_units,
            closed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("reserve_root", &self.reserve_root)?;
        ensure_root("liability_root", &self.liability_root)?;
        if self.reserve_atomic_units < self.reserve_floor_atomic_units {
            return Err(ActivationGuardError::ReserveLiabilityOpen);
        }
        if self.reserve_atomic_units < self.liabilities_atomic_units {
            return Err(ActivationGuardError::ReserveLiabilityOpen);
        }
        if !self.closed {
            return Err(ActivationGuardError::ReserveLiabilityOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseQueueClosure {
    pub queue_root: String,
    pub closure_root: String,
    pub queue_age_blocks: u64,
    pub accounted_credit_count: u64,
    pub pending_activation_count: u64,
    pub closed: bool,
}

impl ReleaseQueueClosure {
    pub fn blocked() -> Self {
        Self {
            queue_root: "root:release-queue:open".to_string(),
            closure_root: "root:release-queue-closure:missing".to_string(),
            queue_age_blocks: 0,
            accounted_credit_count: 0,
            pending_activation_count: 1,
            closed: false,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("queue_root", &self.queue_root)?;
        ensure_root("closure_root", &self.closure_root)?;
        if self.queue_age_blocks < config.min_release_queue_age {
            return Err(ActivationGuardError::ReleaseQueueOpen);
        }
        if self.accounted_credit_count == 0 {
            return Err(ActivationGuardError::InvalidAmount(
                "accounted_credit_count",
            ));
        }
        if self.pending_activation_count != 0 {
            return Err(ActivationGuardError::ReleaseQueueOpen);
        }
        if !self.closed {
            return Err(ActivationGuardError::ReleaseQueueOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustodyAccounting {
    pub debit_root: String,
    pub credit_root: String,
    pub accounted_release_credit_atomic_units: u128,
    pub custody_debit_atomic_units: u128,
    pub reserve_remainder_atomic_units: u128,
    pub net_delta_atomic_units: i128,
    pub closed: bool,
}

impl CustodyAccounting {
    pub fn blocked() -> Self {
        Self {
            debit_root: "root:custody-debit:withheld".to_string(),
            credit_root: "root:release-credit:accounted-only".to_string(),
            accounted_release_credit_atomic_units: 0,
            custody_debit_atomic_units: 0,
            reserve_remainder_atomic_units: 0,
            net_delta_atomic_units: 1,
            closed: false,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("debit_root", &self.debit_root)?;
        ensure_root("credit_root", &self.credit_root)?;
        if self.accounted_release_credit_atomic_units == 0 {
            return Err(ActivationGuardError::InvalidAmount(
                "accounted_release_credit_atomic_units",
            ));
        }
        if self.custody_debit_atomic_units == 0 {
            return Err(ActivationGuardError::InvalidAmount(
                "custody_debit_atomic_units",
            ));
        }
        if self.accounted_release_credit_atomic_units != self.custody_debit_atomic_units {
            return Err(ActivationGuardError::CustodyAccountingOpen);
        }
        if self.net_delta_atomic_units != config.max_accounting_delta_atomic_units {
            return Err(ActivationGuardError::CustodyAccountingOpen);
        }
        if !self.closed {
            return Err(ActivationGuardError::CustodyAccountingOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoteCommitmentGuard {
    pub commitment_root: String,
    pub activation_root: String,
    pub note_count: u64,
    pub accounted_credit_atomic_units: u128,
    pub committed_note_atomic_units: u128,
    pub commitment_drift_atomic_units: i128,
    pub activation_frozen: bool,
}

impl NoteCommitmentGuard {
    pub fn blocked() -> Self {
        Self {
            commitment_root: "root:note-commitment:blocked".to_string(),
            activation_root: "root:private-note-activation:frozen".to_string(),
            note_count: 0,
            accounted_credit_atomic_units: 0,
            committed_note_atomic_units: 0,
            commitment_drift_atomic_units: 1,
            activation_frozen: true,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("commitment_root", &self.commitment_root)?;
        ensure_root("activation_root", &self.activation_root)?;
        if self.note_count == 0 {
            return Err(ActivationGuardError::InvalidAmount("note_count"));
        }
        if self.accounted_credit_atomic_units == 0 || self.committed_note_atomic_units == 0 {
            return Err(ActivationGuardError::InvalidAmount(
                "note_commitment_atomic_units",
            ));
        }
        if self.accounted_credit_atomic_units != self.committed_note_atomic_units {
            return Err(ActivationGuardError::NoteCommitmentMismatch);
        }
        if self.commitment_drift_atomic_units != config.max_note_commitment_drift {
            return Err(ActivationGuardError::NoteCommitmentMismatch);
        }
        if self.activation_frozen {
            return Err(ActivationGuardError::NoteActivationStillBlocked);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullifierReservationGuard {
    pub reservation_root: String,
    pub nullifier_set_root: String,
    pub reserved_count: u64,
    pub activated_count: u64,
    pub duplicate_count: u64,
    pub reservation_drift: i128,
    pub reservations_locked: bool,
}

impl NullifierReservationGuard {
    pub fn blocked() -> Self {
        Self {
            reservation_root: "root:nullifier-reservation:blocked".to_string(),
            nullifier_set_root: "root:nullifier-set:unopened".to_string(),
            reserved_count: 0,
            activated_count: 0,
            duplicate_count: 1,
            reservation_drift: 1,
            reservations_locked: true,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("reservation_root", &self.reservation_root)?;
        ensure_root("nullifier_set_root", &self.nullifier_set_root)?;
        if self.reserved_count == 0 {
            return Err(ActivationGuardError::InvalidAmount("reserved_count"));
        }
        if self.reserved_count != self.activated_count {
            return Err(ActivationGuardError::NullifierReservationMismatch);
        }
        if self.duplicate_count != 0 {
            return Err(ActivationGuardError::NullifierReservationMismatch);
        }
        if self.reservation_drift != config.max_nullifier_drift {
            return Err(ActivationGuardError::NullifierReservationMismatch);
        }
        if self.reservations_locked {
            return Err(ActivationGuardError::NullifierReservationMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeRebateSettlement {
    pub fee_root: String,
    pub rebate_root: String,
    pub settlement_root: String,
    pub gross_fee_atomic_units: u128,
    pub gross_rebate_atomic_units: u128,
    pub net_drift_atomic_units: i128,
    pub settled: bool,
}

impl FeeRebateSettlement {
    pub fn blocked() -> Self {
        Self {
            fee_root: "root:fee:settlement-blocked".to_string(),
            rebate_root: "root:rebate:settlement-blocked".to_string(),
            settlement_root: "root:fee-rebate-settlement:open".to_string(),
            gross_fee_atomic_units: 0,
            gross_rebate_atomic_units: 0,
            net_drift_atomic_units: 1,
            settled: false,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("fee_root", &self.fee_root)?;
        ensure_root("rebate_root", &self.rebate_root)?;
        ensure_root("settlement_root", &self.settlement_root)?;
        if self.net_drift_atomic_units != config.max_fee_rebate_drift_atomic_units {
            return Err(ActivationGuardError::FeeRebateSettlementOpen);
        }
        if !self.settled {
            return Err(ActivationGuardError::FeeRebateSettlementOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PqAuthorization {
    pub authorization_root: String,
    pub keyset_root: String,
    pub transcript_root: String,
    pub ml_dsa_authorized: bool,
    pub slh_dsa_authorized: bool,
    pub rotation_closed: bool,
}

impl PqAuthorization {
    pub fn blocked() -> Self {
        Self {
            authorization_root: "root:pq-authorization:missing".to_string(),
            keyset_root: "root:pq-keyset:missing".to_string(),
            transcript_root: "root:pq-transcript:missing".to_string(),
            ml_dsa_authorized: false,
            slh_dsa_authorized: false,
            rotation_closed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("authorization_root", &self.authorization_root)?;
        ensure_root("keyset_root", &self.keyset_root)?;
        ensure_root("transcript_root", &self.transcript_root)?;
        if !self.ml_dsa_authorized || !self.slh_dsa_authorized || !self.rotation_closed {
            return Err(ActivationGuardError::PqAuthorizationMissing);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircuitBreakerPanel {
    pub breaker_root: String,
    pub pause_root: String,
    pub armed_count: u64,
    pub override_count: u64,
    pub release_breaker_armed: bool,
    pub note_activation_breaker_armed: bool,
}

impl CircuitBreakerPanel {
    pub fn blocked() -> Self {
        Self {
            breaker_root: "root:circuit-breaker:armed".to_string(),
            pause_root: "root:pause-window:open".to_string(),
            armed_count: 2,
            override_count: 0,
            release_breaker_armed: true,
            note_activation_breaker_armed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("breaker_root", &self.breaker_root)?;
        ensure_root("pause_root", &self.pause_root)?;
        if self.armed_count != 0 || self.override_count != 0 {
            return Err(ActivationGuardError::CircuitBreakerArmed);
        }
        if self.release_breaker_armed || self.note_activation_breaker_armed {
            return Err(ActivationGuardError::CircuitBreakerArmed);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReorgClearance {
    pub clearance_root: String,
    pub depth: u64,
    pub competing_branch_count: u64,
    pub cleared: bool,
}

impl ReorgClearance {
    pub fn blocked() -> Self {
        Self {
            clearance_root: "root:reorg-clearance:pending".to_string(),
            depth: 0,
            competing_branch_count: 1,
            cleared: false,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("clearance_root", &self.clearance_root)?;
        if self.depth < config.min_reorg_clearance_depth {
            return Err(ActivationGuardError::ReorgClearanceTooLow);
        }
        if self.competing_branch_count != 0 || !self.cleared {
            return Err(ActivationGuardError::ReorgClearanceTooLow);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeavyGateEvidence {
    pub evidence_root: String,
    pub live_feed_root: String,
    pub replay_root: String,
    pub rounds: u64,
    pub ran_live: bool,
    pub replay_matched: bool,
}

impl HeavyGateEvidence {
    pub fn blocked() -> Self {
        Self {
            evidence_root: "root:live-heavy-gate:missing".to_string(),
            live_feed_root: "root:live-feed:missing".to_string(),
            replay_root: "root:heavy-gate-replay:missing".to_string(),
            rounds: 0,
            ran_live: false,
            replay_matched: false,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("live_feed_root", &self.live_feed_root)?;
        ensure_root("replay_root", &self.replay_root)?;
        if !self.ran_live || !self.replay_matched {
            return Err(ActivationGuardError::HeavyGateEvidenceMissing);
        }
        if self.rounds < config.min_heavy_gate_rounds {
            return Err(ActivationGuardError::HeavyGateRoundsTooLow);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub roots: Vec<EvidenceRoot>,
    pub signoffs: Vec<Signoff>,
    pub wave105_credit_roots: Wave105CreditRoots,
    pub reserve_liability_closure: ReserveLiabilityClosure,
    pub release_queue_closure: ReleaseQueueClosure,
    pub custody_accounting: CustodyAccounting,
    pub note_commitment_guard: NoteCommitmentGuard,
    pub nullifier_reservation_guard: NullifierReservationGuard,
    pub fee_rebate_settlement: FeeRebateSettlement,
    pub pq_authorization: PqAuthorization,
    pub circuit_breakers: CircuitBreakerPanel,
    pub reorg_clearance: ReorgClearance,
    pub heavy_gate_evidence: HeavyGateEvidence,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub status: GateStatus,
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        Self {
            roots: fail_closed_roots(),
            signoffs: fail_closed_signoffs(),
            wave105_credit_roots: Wave105CreditRoots::blocked(),
            reserve_liability_closure: ReserveLiabilityClosure::blocked(&config),
            release_queue_closure: ReleaseQueueClosure::blocked(),
            custody_accounting: CustodyAccounting::blocked(),
            note_commitment_guard: NoteCommitmentGuard::blocked(),
            nullifier_reservation_guard: NullifierReservationGuard::blocked(),
            fee_rebate_settlement: FeeRebateSettlement::blocked(),
            pq_authorization: PqAuthorization::blocked(),
            circuit_breakers: CircuitBreakerPanel::blocked(),
            reorg_clearance: ReorgClearance::blocked(),
            heavy_gate_evidence: HeavyGateEvidence::blocked(),
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            status: GateStatus::Blocked,
            config,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.validate_required_roots()?;
        self.validate_signoffs()?;
        self.wave105_credit_roots.validate()?;
        self.reserve_liability_closure.validate()?;
        self.release_queue_closure.validate(&self.config)?;
        self.custody_accounting.validate(&self.config)?;
        self.note_commitment_guard.validate(&self.config)?;
        self.nullifier_reservation_guard.validate(&self.config)?;
        self.fee_rebate_settlement.validate(&self.config)?;
        self.pq_authorization.validate()?;
        self.circuit_breakers.validate()?;
        self.reorg_clearance.validate(&self.config)?;
        self.heavy_gate_evidence.validate(&self.config)?;
        if !self.release_credit_allowed {
            return Err(ActivationGuardError::ReleaseCreditStillBlocked);
        }
        if !self.note_activation_allowed {
            return Err(ActivationGuardError::NoteActivationStillBlocked);
        }
        if !self.heavy_gates_ran {
            return Err(ActivationGuardError::HeavyGateEvidenceMissing);
        }
        Ok(())
    }

    pub fn blockers(&self) -> Vec<String> {
        let mut blockers = Vec::new();
        collect_result(&mut blockers, self.config.validate());
        collect_result(&mut blockers, self.validate_required_roots());
        collect_result(&mut blockers, self.validate_signoffs());
        collect_result(&mut blockers, self.wave105_credit_roots.validate());
        collect_result(&mut blockers, self.reserve_liability_closure.validate());
        collect_result(
            &mut blockers,
            self.release_queue_closure.validate(&self.config),
        );
        collect_result(
            &mut blockers,
            self.custody_accounting.validate(&self.config),
        );
        collect_result(
            &mut blockers,
            self.note_commitment_guard.validate(&self.config),
        );
        collect_result(
            &mut blockers,
            self.nullifier_reservation_guard.validate(&self.config),
        );
        collect_result(
            &mut blockers,
            self.fee_rebate_settlement.validate(&self.config),
        );
        collect_result(&mut blockers, self.pq_authorization.validate());
        collect_result(&mut blockers, self.circuit_breakers.validate());
        collect_result(&mut blockers, self.reorg_clearance.validate(&self.config));
        collect_result(
            &mut blockers,
            self.heavy_gate_evidence.validate(&self.config),
        );
        if !self.release_credit_allowed {
            blockers.push("release_credit_allowed: false".to_string());
        }
        if !self.note_activation_allowed {
            blockers.push("note_activation_allowed: false".to_string());
        }
        if !self.heavy_gates_ran {
            blockers.push("heavy_gates_ran: false".to_string());
        }
        dedupe(blockers)
    }

    pub fn public_record(&self) -> PublicRecord {
        let blockers = self.blockers();
        let root_list = self
            .roots
            .iter()
            .map(|root| stable_root(root.kind.as_str(), &format!("{:?}", root)))
            .collect::<Vec<String>>();
        let signoff_list = self
            .signoffs
            .iter()
            .map(|signoff| stable_root(signoff.role.as_str(), &format!("{:?}", signoff)))
            .collect::<Vec<String>>();
        let mut record = Record::new();
        record.insert_text(
            "config_root",
            &stable_root("config", &format!("{:?}", self.config)),
        );
        record.insert_list("evidence_roots", root_list);
        record.insert_list("signoff_roots", signoff_list);
        record.insert_text(
            "wave105_credit_roots",
            &stable_root(
                "wave105_credit_roots",
                &format!("{:?}", self.wave105_credit_roots),
            ),
        );
        record.insert_text(
            "reserve_liability_closure_root",
            &stable_root(
                "reserve_liability_closure",
                &format!("{:?}", self.reserve_liability_closure),
            ),
        );
        record.insert_text(
            "release_queue_closure_root",
            &stable_root(
                "release_queue_closure",
                &format!("{:?}", self.release_queue_closure),
            ),
        );
        record.insert_text(
            "custody_accounting_root",
            &stable_root(
                "custody_accounting",
                &format!("{:?}", self.custody_accounting),
            ),
        );
        record.insert_text(
            "note_commitment_guard_root",
            &stable_root(
                "note_commitment_guard",
                &format!("{:?}", self.note_commitment_guard),
            ),
        );
        record.insert_text(
            "nullifier_reservation_guard_root",
            &stable_root(
                "nullifier_reservation_guard",
                &format!("{:?}", self.nullifier_reservation_guard),
            ),
        );
        record.insert_text(
            "fee_rebate_settlement_root",
            &stable_root(
                "fee_rebate_settlement",
                &format!("{:?}", self.fee_rebate_settlement),
            ),
        );
        record.insert_text(
            "pq_authorization_root",
            &stable_root("pq_authorization", &format!("{:?}", self.pq_authorization)),
        );
        record.insert_text(
            "circuit_breaker_root",
            &stable_root("circuit_breakers", &format!("{:?}", self.circuit_breakers)),
        );
        record.insert_text(
            "reorg_clearance_root",
            &stable_root("reorg_clearance", &format!("{:?}", self.reorg_clearance)),
        );
        record.insert_text(
            "heavy_gate_evidence_root",
            &stable_root(
                "heavy_gate_evidence",
                &format!("{:?}", self.heavy_gate_evidence),
            ),
        );
        record.insert_bool("release_credit_allowed", self.release_credit_allowed);
        record.insert_bool("note_activation_allowed", self.note_activation_allowed);
        record.insert_bool("heavy_gates_ran", self.heavy_gates_ran);
        record.insert_text("status", self.status.as_str());
        record.insert_list("blockers", blockers);
        record
    }

    pub fn root(&self) -> String {
        record_root("state", &self.public_record())
    }

    fn validate_required_roots(&self) -> Result<()> {
        for kind in RootKind::all() {
            if !self.has_accepted_root(kind) {
                return Err(ActivationGuardError::MissingRoot(kind));
            }
        }
        Ok(())
    }

    fn validate_signoffs(&self) -> Result<()> {
        let mut approved = 0_u16;
        for role in SignoffRole::all() {
            let mut found = false;
            for signoff in &self.signoffs {
                if signoff.role == role {
                    found = true;
                    signoff.validate()?;
                    if signoff.approved {
                        approved = approved.saturating_add(1);
                    }
                }
            }
            if !found {
                return Err(ActivationGuardError::MissingSignoff(role));
            }
        }
        if approved < self.config.signoff_quorum {
            return Err(ActivationGuardError::InvalidThreshold("signoff_quorum"));
        }
        Ok(())
    }

    fn has_accepted_root(&self, kind: RootKind) -> bool {
        self.roots
            .iter()
            .any(|root| root.kind == kind && root.accepted && root.validate().is_ok())
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record(runtime: &Runtime) -> PublicRecord {
    runtime.public_record()
}

pub fn state_root(state: &State) -> String {
    state.root()
}

fn fail_closed_roots() -> Vec<EvidenceRoot> {
    RootKind::all()
        .iter()
        .map(|kind| {
            EvidenceRoot::rejected(
                *kind,
                &format!(
                    "root:{}:withheld-until-wave106-private-note-nullifier-guard-clears",
                    kind.as_str()
                ),
                "fail-closed-devnet-default",
                1,
            )
        })
        .collect()
}

fn fail_closed_signoffs() -> Vec<Signoff> {
    SignoffRole::all()
        .iter()
        .map(|role| Signoff::missing(*role))
        .collect()
}

fn collect_result(blockers: &mut Vec<String>, result: Result<()>) {
    if let Err(error) = result {
        blockers.push(format!("{:?}", error));
    }
}

fn dedupe(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeMap::new();
    for value in values {
        seen.insert(value, true);
    }
    seen.keys().cloned().collect()
}

fn ensure_text(name: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ActivationGuardError::EmptyField(name));
    }
    Ok(())
}

fn ensure_nonzero(name: &'static str, value: u64) -> Result<()> {
    if value == 0 {
        return Err(ActivationGuardError::InvalidThreshold(name));
    }
    Ok(())
}

fn ensure_root(name: &'static str, value: &str) -> Result<()> {
    ensure_text(name, value)?;
    if !value.starts_with("root:") {
        return Err(ActivationGuardError::InvalidRoot(name));
    }
    Ok(())
}

fn record_root(domain: &str, record: &Record) -> String {
    stable_root(domain, &record.canonical_string())
}

fn stable_root(domain: &str, value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in domain.as_bytes().iter().chain(value.as_bytes()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("root:{}:{:016x}", domain, hash)
}
