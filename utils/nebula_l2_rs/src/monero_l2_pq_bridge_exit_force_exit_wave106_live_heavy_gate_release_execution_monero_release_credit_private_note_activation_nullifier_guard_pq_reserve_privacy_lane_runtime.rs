use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type PublicRecord = Value;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str = "nebula-monero-l2-pq-bridge-exit-force-exit-wave106-live-heavy-gate-release-execution-monero-release-credit-private-note-activation-nullifier-guard-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DEFAULT_MIN_PQ_AUTH_ROOTS: usize = 3;
pub const DEFAULT_MIN_RESERVE_ROOTS: usize = 3;
pub const DEFAULT_MIN_PRIVACY_BUCKET_ROOTS: usize = 2;
pub const DEFAULT_MIN_NOTE_ROOTS: usize = 2;
pub const DEFAULT_MIN_NULLIFIER_ROOTS: usize = 2;
pub const DEFAULT_MIN_WAVE105_ROOTS: usize = 3;
pub const DEFAULT_MIN_HEAVY_GATE_EVIDENCE: usize = 6;
pub const DEFAULT_MIN_SIGNOFFS: usize = 7;
pub const DEFAULT_MIN_PQ_WEIGHT_BPS: u64 = 7_500;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_500;
pub const DEFAULT_MIN_BUCKET_ANONYMITY_SET: u64 = 16_384;
pub const DEFAULT_MAX_BUCKET_AMOUNT_PPM: u64 = 70_000;
pub const DEFAULT_MAX_FEE_BPS: u64 = 35;
pub const DEFAULT_MAX_REBATE_DRIFT_BPS: u64 = 5;
pub const DEFAULT_MAX_STALENESS_SLOTS: u64 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RootLane {
    PqAuthorization,
    ReserveLiabilityClosure,
    AmountBucketPrivacy,
    NoteCommitment,
    NullifierReservation,
    Wave105CreditAccounting,
    FeeRebateSettlement,
}

impl RootLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqAuthorization => "pq_authorization",
            Self::ReserveLiabilityClosure => "reserve_liability_closure",
            Self::AmountBucketPrivacy => "amount_bucket_privacy",
            Self::NoteCommitment => "note_commitment",
            Self::NullifierReservation => "nullifier_reservation",
            Self::Wave105CreditAccounting => "wave105_credit_accounting",
            Self::FeeRebateSettlement => "fee_rebate_settlement",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RootStatus {
    Accepted,
    Pending,
    Stale,
    Rejected,
}

impl RootStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_clear(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Passed,
    Pending,
    Failed,
    Disabled,
}

impl GateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Pending => "pending",
            Self::Failed => "failed",
            Self::Disabled => "disabled",
        }
    }

    pub fn clears(self) -> bool {
        matches!(self, Self::Passed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakerStatus {
    ArmedClosed,
    Open,
    DisarmedByGovernance,
}

impl BreakerStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArmedClosed => "armed_closed",
            Self::Open => "open",
            Self::DisarmedByGovernance => "disarmed_by_governance",
        }
    }

    pub fn blocks(self) -> bool {
        !matches!(self, Self::ArmedClosed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffRole {
    PqSecurity,
    ReserveAccounting,
    PrivacyEngineering,
    NullifierOps,
    Wave105Credit,
    FeeSettlement,
    HeavyGateOperator,
    ReleaseCaptain,
    IncidentCommander,
}

impl SignoffRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqSecurity => "pq_security",
            Self::ReserveAccounting => "reserve_accounting",
            Self::PrivacyEngineering => "privacy_engineering",
            Self::NullifierOps => "nullifier_ops",
            Self::Wave105Credit => "wave105_credit",
            Self::FeeSettlement => "fee_settlement",
            Self::HeavyGateOperator => "heavy_gate_operator",
            Self::ReleaseCaptain => "release_captain",
            Self::IncidentCommander => "incident_commander",
        }
    }

    pub fn required_for_release(self) -> bool {
        !matches!(self, Self::IncidentCommander)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub min_pq_authorization_roots: usize,
    pub min_reserve_liability_roots: usize,
    pub min_amount_bucket_privacy_roots: usize,
    pub min_note_commitment_roots: usize,
    pub min_nullifier_reservation_roots: usize,
    pub min_wave105_credit_roots: usize,
    pub min_heavy_gate_evidence: usize,
    pub min_signoffs: usize,
    pub min_pq_weight_bps: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_bucket_anonymity_set: u64,
    pub max_bucket_amount_ppm: u64,
    pub max_fee_bps: u64,
    pub max_rebate_drift_bps: u64,
    pub max_staleness_slots: u64,
    pub fail_closed_on_missing_root: bool,
    pub fail_closed_on_breaker_open: bool,
    pub fail_closed_on_signoff_gap: bool,
    pub fail_closed_on_heavy_gate_gap: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            min_pq_authorization_roots: DEFAULT_MIN_PQ_AUTH_ROOTS,
            min_reserve_liability_roots: DEFAULT_MIN_RESERVE_ROOTS,
            min_amount_bucket_privacy_roots: DEFAULT_MIN_PRIVACY_BUCKET_ROOTS,
            min_note_commitment_roots: DEFAULT_MIN_NOTE_ROOTS,
            min_nullifier_reservation_roots: DEFAULT_MIN_NULLIFIER_ROOTS,
            min_wave105_credit_roots: DEFAULT_MIN_WAVE105_ROOTS,
            min_heavy_gate_evidence: DEFAULT_MIN_HEAVY_GATE_EVIDENCE,
            min_signoffs: DEFAULT_MIN_SIGNOFFS,
            min_pq_weight_bps: DEFAULT_MIN_PQ_WEIGHT_BPS,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_bucket_anonymity_set: DEFAULT_MIN_BUCKET_ANONYMITY_SET,
            max_bucket_amount_ppm: DEFAULT_MAX_BUCKET_AMOUNT_PPM,
            max_fee_bps: DEFAULT_MAX_FEE_BPS,
            max_rebate_drift_bps: DEFAULT_MAX_REBATE_DRIFT_BPS,
            max_staleness_slots: DEFAULT_MAX_STALENESS_SLOTS,
            fail_closed_on_missing_root: true,
            fail_closed_on_breaker_open: true,
            fail_closed_on_signoff_gap: true,
            fail_closed_on_heavy_gate_gap: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "thresholds": {
                "min_pq_authorization_roots": self.min_pq_authorization_roots,
                "min_reserve_liability_roots": self.min_reserve_liability_roots,
                "min_amount_bucket_privacy_roots": self.min_amount_bucket_privacy_roots,
                "min_note_commitment_roots": self.min_note_commitment_roots,
                "min_nullifier_reservation_roots": self.min_nullifier_reservation_roots,
                "min_wave105_credit_roots": self.min_wave105_credit_roots,
                "min_heavy_gate_evidence": self.min_heavy_gate_evidence,
                "min_signoffs": self.min_signoffs,
                "min_pq_weight_bps": self.min_pq_weight_bps,
                "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
                "min_bucket_anonymity_set": self.min_bucket_anonymity_set,
                "max_bucket_amount_ppm": self.max_bucket_amount_ppm,
                "max_fee_bps": self.max_fee_bps,
                "max_rebate_drift_bps": self.max_rebate_drift_bps,
                "max_staleness_slots": self.max_staleness_slots
            },
            "fail_closed": {
                "missing_root": self.fail_closed_on_missing_root,
                "breaker_open": self.fail_closed_on_breaker_open,
                "signoff_gap": self.fail_closed_on_signoff_gap,
                "heavy_gate_gap": self.fail_closed_on_heavy_gate_gap
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizationRoot {
    pub root_id: String,
    pub lane: RootLane,
    pub root: String,
    pub epoch: u64,
    pub staleness_slots: u64,
    pub signer_set_root: String,
    pub aggregate_weight_bps: u64,
    pub status: RootStatus,
}

impl AuthorizationRoot {
    pub fn clears(&self, config: &Config) -> bool {
        self.status.is_clear()
            && self.staleness_slots <= config.max_staleness_slots
            && self.aggregate_weight_bps >= config.min_pq_weight_bps
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "root_id": self.root_id,
            "lane": self.lane.as_str(),
            "root": self.root,
            "epoch": self.epoch,
            "staleness_slots": self.staleness_slots,
            "signer_set_root": self.signer_set_root,
            "aggregate_weight_bps": self.aggregate_weight_bps,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveClosureRoot {
    pub closure_id: String,
    pub reserve_root: String,
    pub liability_root: String,
    pub asset: String,
    pub covered_liability_piconero: u128,
    pub reserve_piconero: u128,
    pub coverage_bps: u64,
    pub status: RootStatus,
}

impl ReserveClosureRoot {
    pub fn clears(&self, config: &Config) -> bool {
        self.status.is_clear()
            && self.coverage_bps >= config.min_reserve_coverage_bps
            && self.reserve_piconero >= self.covered_liability_piconero
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "closure_id": self.closure_id,
            "reserve_root": self.reserve_root,
            "liability_root": self.liability_root,
            "asset": self.asset,
            "covered_liability_piconero": self.covered_liability_piconero.to_string(),
            "reserve_piconero": self.reserve_piconero.to_string(),
            "coverage_bps": self.coverage_bps,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AmountBucketPrivacyRoot {
    pub bucket_id: String,
    pub bucket_root: String,
    pub amount_bucket_label: String,
    pub anonymity_set_size: u64,
    pub bucket_amount_ppm: u64,
    pub redaction_root: String,
    pub leakage_audit_root: String,
    pub status: RootStatus,
}

impl AmountBucketPrivacyRoot {
    pub fn clears(&self, config: &Config) -> bool {
        self.status.is_clear()
            && self.anonymity_set_size >= config.min_bucket_anonymity_set
            && self.bucket_amount_ppm <= config.max_bucket_amount_ppm
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "bucket_id": self.bucket_id,
            "bucket_root": self.bucket_root,
            "amount_bucket_label": self.amount_bucket_label,
            "anonymity_set_size": self.anonymity_set_size,
            "bucket_amount_ppm": self.bucket_amount_ppm,
            "redaction_root": self.redaction_root,
            "leakage_audit_root": self.leakage_audit_root,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NoteCommitmentRoot {
    pub note_set_id: String,
    pub commitment_root: String,
    pub activation_epoch: u64,
    pub note_count: u64,
    pub asset_root: String,
    pub owner_view_tag_root: String,
    pub status: RootStatus,
}

impl NoteCommitmentRoot {
    pub fn clears(&self) -> bool {
        self.status.is_clear() && self.note_count > 0
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "note_set_id": self.note_set_id,
            "commitment_root": self.commitment_root,
            "activation_epoch": self.activation_epoch,
            "note_count": self.note_count,
            "asset_root": self.asset_root,
            "owner_view_tag_root": self.owner_view_tag_root,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NullifierReservationRoot {
    pub reservation_id: String,
    pub nullifier_root: String,
    pub reservation_epoch: u64,
    pub reserved_count: u64,
    pub collision_scan_root: String,
    pub replay_guard_root: String,
    pub status: RootStatus,
}

impl NullifierReservationRoot {
    pub fn clears(&self) -> bool {
        self.status.is_clear() && self.reserved_count > 0
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "reservation_id": self.reservation_id,
            "nullifier_root": self.nullifier_root,
            "reservation_epoch": self.reservation_epoch,
            "reserved_count": self.reserved_count,
            "collision_scan_root": self.collision_scan_root,
            "replay_guard_root": self.replay_guard_root,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave105CreditRoot {
    pub credit_root_id: String,
    pub credit_accounting_root: String,
    pub debit_root: String,
    pub release_credit_root: String,
    pub carried_forward_liability_root: String,
    pub balanced: bool,
    pub status: RootStatus,
}

impl Wave105CreditRoot {
    pub fn clears(&self) -> bool {
        self.status.is_clear() && self.balanced
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "credit_root_id": self.credit_root_id,
            "credit_accounting_root": self.credit_accounting_root,
            "debit_root": self.debit_root,
            "release_credit_root": self.release_credit_root,
            "carried_forward_liability_root": self.carried_forward_liability_root,
            "balanced": self.balanced,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeeRebateSettlement {
    pub settlement_id: String,
    pub fee_root: String,
    pub rebate_root: String,
    pub fee_bps: u64,
    pub rebate_drift_bps: u64,
    pub operator_surplus_root: String,
    pub user_credit_root: String,
    pub status: RootStatus,
}

impl FeeRebateSettlement {
    pub fn clears(&self, config: &Config) -> bool {
        self.status.is_clear()
            && self.fee_bps <= config.max_fee_bps
            && self.rebate_drift_bps <= config.max_rebate_drift_bps
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "settlement_id": self.settlement_id,
            "fee_root": self.fee_root,
            "rebate_root": self.rebate_root,
            "fee_bps": self.fee_bps,
            "rebate_drift_bps": self.rebate_drift_bps,
            "operator_surplus_root": self.operator_surplus_root,
            "user_credit_root": self.user_credit_root,
            "status": self.status.as_str()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitBreaker {
    pub breaker_id: String,
    pub lane: RootLane,
    pub status: BreakerStatus,
    pub reason: String,
    pub opened_at_slot: u64,
    pub evidence_root: String,
}

impl CircuitBreaker {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "breaker_id": self.breaker_id,
            "lane": self.lane.as_str(),
            "status": self.status.as_str(),
            "reason": self.reason,
            "opened_at_slot": self.opened_at_slot,
            "evidence_root": self.evidence_root
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HeavyGateEvidence {
    pub evidence_id: String,
    pub gate_name: String,
    pub status: GateStatus,
    pub transcript_root: String,
    pub vector_root: String,
    pub negative_case_root: String,
    pub operator_signature_root: String,
    pub ran_at_slot: u64,
}

impl HeavyGateEvidence {
    pub fn clears(&self) -> bool {
        self.status.clears()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "evidence_id": self.evidence_id,
            "gate_name": self.gate_name,
            "status": self.status.as_str(),
            "transcript_root": self.transcript_root,
            "vector_root": self.vector_root,
            "negative_case_root": self.negative_case_root,
            "operator_signature_root": self.operator_signature_root,
            "ran_at_slot": self.ran_at_slot
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signoff {
    pub signoff_id: String,
    pub role: SignoffRole,
    pub signer_commitment: String,
    pub signed_root: String,
    pub approved: bool,
    pub slot: u64,
}

impl Signoff {
    pub fn clears(&self) -> bool {
        self.approved && self.role.required_for_release()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "signoff_id": self.signoff_id,
            "role": self.role.as_str(),
            "signer_commitment": self.signer_commitment,
            "signed_root": self.signed_root,
            "approved": self.approved,
            "slot": self.slot
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeDecision {
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub fail_closed: bool,
    pub blockers: Vec<String>,
}

impl RuntimeDecision {
    pub fn fail_closed(blockers: Vec<String>) -> Self {
        Self {
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            fail_closed: true,
            blockers,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "fail_closed": self.fail_closed,
            "blockers": self.blockers
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub pq_authorization_roots: Vec<AuthorizationRoot>,
    pub reserve_closure_roots: Vec<ReserveClosureRoot>,
    pub amount_bucket_privacy_roots: Vec<AmountBucketPrivacyRoot>,
    pub note_commitment_roots: Vec<NoteCommitmentRoot>,
    pub nullifier_reservation_roots: Vec<NullifierReservationRoot>,
    pub wave105_credit_roots: Vec<Wave105CreditRoot>,
    pub fee_rebate_settlement: FeeRebateSettlement,
    pub circuit_breakers: Vec<CircuitBreaker>,
    pub heavy_gate_evidence: Vec<HeavyGateEvidence>,
    pub signoffs: Vec<Signoff>,
    pub activation_note: String,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let pq_authorization_roots = vec![
            pq_root("pq-auth-root-a", 91, 8_200, RootStatus::Accepted),
            pq_root("pq-auth-root-b", 92, 7_900, RootStatus::Accepted),
            pq_root("pq-auth-root-c", 93, 8_500, RootStatus::Accepted),
        ];
        let reserve_closure_roots = vec![
            reserve_root(
                "reserve-close-a",
                1_100_000_000_000,
                1_000_000_000_000,
                11_000,
            ),
            reserve_root(
                "reserve-close-b",
                1_050_000_000_000,
                1_000_000_000_000,
                10_500,
            ),
            reserve_root(
                "reserve-close-c",
                1_080_000_000_000,
                1_000_000_000_000,
                10_800,
            ),
        ];
        let amount_bucket_privacy_roots = vec![
            privacy_root("bucket-p2-18", "2^18 piconero bucket", 20_480, 55_000),
            privacy_root("bucket-p2-19", "2^19 piconero bucket", 24_576, 61_000),
        ];
        let note_commitment_roots = vec![
            note_root("note-set-force-exit-a", 101, 2048),
            note_root("note-set-force-exit-b", 102, 1024),
        ];
        let nullifier_reservation_roots = vec![
            nullifier_root("nullifier-reserve-a", 101, 2048),
            nullifier_root("nullifier-reserve-b", 102, 1024),
        ];
        let wave105_credit_roots = vec![
            credit_root("wave105-credit-a"),
            credit_root("wave105-credit-b"),
            credit_root("wave105-credit-c"),
        ];
        let fee_rebate_settlement = FeeRebateSettlement {
            settlement_id: "fee-rebate-wave106-release".to_string(),
            fee_root: scoped_hash("fee-root", "fee-rebate-wave106-release"),
            rebate_root: scoped_hash("rebate-root", "fee-rebate-wave106-release"),
            fee_bps: 22,
            rebate_drift_bps: 2,
            operator_surplus_root: scoped_hash("operator-surplus", "fee-rebate-wave106-release"),
            user_credit_root: scoped_hash("user-credit", "fee-rebate-wave106-release"),
            status: RootStatus::Accepted,
        };
        let circuit_breakers = vec![
            breaker("breaker-pq", RootLane::PqAuthorization),
            breaker("breaker-reserve", RootLane::ReserveLiabilityClosure),
            breaker("breaker-privacy", RootLane::AmountBucketPrivacy),
            breaker("breaker-nullifier", RootLane::NullifierReservation),
        ];
        let heavy_gate_evidence = vec![
            heavy_gate("gate-pq-root-quorum", 201),
            heavy_gate("gate-reserve-liability-closure", 202),
            heavy_gate("gate-bucket-privacy-leakage", 203),
            heavy_gate("gate-note-commitment-membership", 204),
            heavy_gate("gate-nullifier-reservation-collision", 205),
            heavy_gate("gate-wave105-credit-carry", 206),
        ];
        let release_root = merkle_root(
            "monero-l2-wave106-release-signoff-base",
            &[
                json!(pq_authorization_roots
                    .iter()
                    .map(AuthorizationRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(reserve_closure_roots
                    .iter()
                    .map(ReserveClosureRoot::public_record)
                    .collect::<Vec<_>>()),
                fee_rebate_settlement.public_record(),
            ],
        );
        let signoffs = vec![
            signoff("signoff-pq", SignoffRole::PqSecurity, &release_root, 301),
            signoff(
                "signoff-reserve",
                SignoffRole::ReserveAccounting,
                &release_root,
                302,
            ),
            signoff(
                "signoff-privacy",
                SignoffRole::PrivacyEngineering,
                &release_root,
                303,
            ),
            signoff(
                "signoff-nullifier",
                SignoffRole::NullifierOps,
                &release_root,
                304,
            ),
            signoff(
                "signoff-wave105",
                SignoffRole::Wave105Credit,
                &release_root,
                305,
            ),
            signoff(
                "signoff-fee",
                SignoffRole::FeeSettlement,
                &release_root,
                306,
            ),
            signoff(
                "signoff-heavy-gate",
                SignoffRole::HeavyGateOperator,
                &release_root,
                307,
            ),
            signoff(
                "signoff-release",
                SignoffRole::ReleaseCaptain,
                &release_root,
                308,
            ),
        ];
        Self {
            config,
            pq_authorization_roots,
            reserve_closure_roots,
            amount_bucket_privacy_roots,
            note_commitment_roots,
            nullifier_reservation_roots,
            wave105_credit_roots,
            fee_rebate_settlement,
            circuit_breakers,
            heavy_gate_evidence,
            signoffs,
            activation_note: "private-note activation remains blocked until every PQ, reserve, privacy, nullifier, credit, settlement, heavy-gate, breaker, and signoff lane is clear".to_string(),
        }
    }

    pub fn decision(&self) -> RuntimeDecision {
        let blockers = self.blockers();
        if !blockers.is_empty() {
            return RuntimeDecision::fail_closed(blockers);
        }
        RuntimeDecision {
            release_credit_allowed: true,
            note_activation_allowed: true,
            heavy_gates_ran: true,
            fail_closed: false,
            blockers,
        }
    }

    pub fn validate(&self) -> Result<()> {
        let blockers = self.blockers();
        if blockers.is_empty() {
            Ok(())
        } else {
            Err(blockers.join("; "))
        }
    }

    pub fn blockers(&self) -> Vec<String> {
        let mut blockers = Vec::new();
        self.require_count(
            "pq_authorization_roots",
            self.pq_authorization_roots
                .iter()
                .filter(|root| root.clears(&self.config))
                .count(),
            self.config.min_pq_authorization_roots,
            &mut blockers,
        );
        self.require_count(
            "reserve_liability_closure_roots",
            self.reserve_closure_roots
                .iter()
                .filter(|root| root.clears(&self.config))
                .count(),
            self.config.min_reserve_liability_roots,
            &mut blockers,
        );
        self.require_count(
            "amount_bucket_privacy_roots",
            self.amount_bucket_privacy_roots
                .iter()
                .filter(|root| root.clears(&self.config))
                .count(),
            self.config.min_amount_bucket_privacy_roots,
            &mut blockers,
        );
        self.require_count(
            "note_commitment_roots",
            self.note_commitment_roots
                .iter()
                .filter(|root| root.clears())
                .count(),
            self.config.min_note_commitment_roots,
            &mut blockers,
        );
        self.require_count(
            "nullifier_reservation_roots",
            self.nullifier_reservation_roots
                .iter()
                .filter(|root| root.clears())
                .count(),
            self.config.min_nullifier_reservation_roots,
            &mut blockers,
        );
        self.require_count(
            "wave105_credit_accounting_roots",
            self.wave105_credit_roots
                .iter()
                .filter(|root| root.clears())
                .count(),
            self.config.min_wave105_credit_roots,
            &mut blockers,
        );
        if !self.fee_rebate_settlement.clears(&self.config) {
            blockers.push("fee_rebate_settlement_not_clear".to_string());
        }
        for breaker in self
            .circuit_breakers
            .iter()
            .filter(|breaker| breaker.status.blocks())
        {
            blockers.push(format!("circuit_breaker_open:{}", breaker.breaker_id));
        }
        self.require_count(
            "live_heavy_gate_evidence",
            self.heavy_gate_evidence
                .iter()
                .filter(|evidence| evidence.clears())
                .count(),
            self.config.min_heavy_gate_evidence,
            &mut blockers,
        );
        self.require_distinct_signoffs(&mut blockers);
        self.require_unique_roots(&mut blockers);
        blockers
    }

    pub fn release_credit_allowed(&self) -> bool {
        self.decision().release_credit_allowed
    }

    pub fn note_activation_allowed(&self) -> bool {
        self.decision().note_activation_allowed
    }

    pub fn heavy_gates_ran(&self) -> bool {
        self.decision().heavy_gates_ran
    }

    pub fn public_record(&self) -> PublicRecord {
        let decision = self.decision();
        json!({
            "kind": "monero_l2_pq_bridge_exit_force_exit_wave106_live_heavy_gate_release_execution_monero_release_credit_private_note_activation_nullifier_guard_pq_reserve_privacy_lane_runtime_state",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "hash_suite": HASH_SUITE,
            "config": self.config.public_record(),
            "pq_authorization_roots": self.pq_authorization_roots.iter().map(AuthorizationRoot::public_record).collect::<Vec<_>>(),
            "reserve_closure_roots": self.reserve_closure_roots.iter().map(ReserveClosureRoot::public_record).collect::<Vec<_>>(),
            "amount_bucket_privacy_roots": self.amount_bucket_privacy_roots.iter().map(AmountBucketPrivacyRoot::public_record).collect::<Vec<_>>(),
            "note_commitment_roots": self.note_commitment_roots.iter().map(NoteCommitmentRoot::public_record).collect::<Vec<_>>(),
            "nullifier_reservation_roots": self.nullifier_reservation_roots.iter().map(NullifierReservationRoot::public_record).collect::<Vec<_>>(),
            "wave105_credit_roots": self.wave105_credit_roots.iter().map(Wave105CreditRoot::public_record).collect::<Vec<_>>(),
            "fee_rebate_settlement": self.fee_rebate_settlement.public_record(),
            "circuit_breakers": self.circuit_breakers.iter().map(CircuitBreaker::public_record).collect::<Vec<_>>(),
            "heavy_gate_evidence": self.heavy_gate_evidence.iter().map(HeavyGateEvidence::public_record).collect::<Vec<_>>(),
            "signoffs": self.signoffs.iter().map(Signoff::public_record).collect::<Vec<_>>(),
            "activation_note": self.activation_note,
            "decision": decision.public_record(),
            "release_credit_allowed": decision.release_credit_allowed,
            "note_activation_allowed": decision.note_activation_allowed,
            "heavy_gates_ran": decision.heavy_gates_ran,
            "state_root": self.state_root()
        })
    }

    pub fn state_root(&self) -> String {
        merkle_root(
            "monero-l2-wave106-pq-reserve-privacy-lane-runtime:state",
            &[
                self.config.public_record(),
                json!(self
                    .pq_authorization_roots
                    .iter()
                    .map(AuthorizationRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .reserve_closure_roots
                    .iter()
                    .map(ReserveClosureRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .amount_bucket_privacy_roots
                    .iter()
                    .map(AmountBucketPrivacyRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .note_commitment_roots
                    .iter()
                    .map(NoteCommitmentRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .nullifier_reservation_roots
                    .iter()
                    .map(NullifierReservationRoot::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .wave105_credit_roots
                    .iter()
                    .map(Wave105CreditRoot::public_record)
                    .collect::<Vec<_>>()),
                self.fee_rebate_settlement.public_record(),
                json!(self
                    .circuit_breakers
                    .iter()
                    .map(CircuitBreaker::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .heavy_gate_evidence
                    .iter()
                    .map(HeavyGateEvidence::public_record)
                    .collect::<Vec<_>>()),
                json!(self
                    .signoffs
                    .iter()
                    .map(Signoff::public_record)
                    .collect::<Vec<_>>()),
                self.decision().public_record(),
            ],
        )
    }

    fn require_count(
        &self,
        label: &str,
        actual: usize,
        required: usize,
        blockers: &mut Vec<String>,
    ) {
        if actual < required {
            blockers.push(format!("{label}_below_threshold:{actual}/{required}"));
        }
    }

    fn require_distinct_signoffs(&self, blockers: &mut Vec<String>) {
        let clear_roles = self
            .signoffs
            .iter()
            .filter(|signoff| signoff.clears())
            .map(|signoff| signoff.role)
            .collect::<BTreeSet<_>>();
        if clear_roles.len() < self.config.min_signoffs {
            blockers.push(format!(
                "release_signoffs_below_threshold:{}/{}",
                clear_roles.len(),
                self.config.min_signoffs
            ));
        }
        for role in [
            SignoffRole::PqSecurity,
            SignoffRole::ReserveAccounting,
            SignoffRole::PrivacyEngineering,
            SignoffRole::NullifierOps,
            SignoffRole::Wave105Credit,
            SignoffRole::FeeSettlement,
            SignoffRole::HeavyGateOperator,
            SignoffRole::ReleaseCaptain,
        ] {
            if !clear_roles.contains(&role) {
                blockers.push(format!("missing_required_signoff:{}", role.as_str()));
            }
        }
    }

    fn require_unique_roots(&self, blockers: &mut Vec<String>) {
        let mut seen = BTreeSet::new();
        for root in self
            .pq_authorization_roots
            .iter()
            .map(|root| root.root.as_str())
        {
            if !seen.insert(root) {
                blockers.push("duplicate_pq_authorization_root".to_string());
            }
        }
        for root in self
            .reserve_closure_roots
            .iter()
            .map(|root| root.reserve_root.as_str())
        {
            if !seen.insert(root) {
                blockers.push("duplicate_reserve_root".to_string());
            }
        }
        for root in self
            .nullifier_reservation_roots
            .iter()
            .map(|root| root.nullifier_root.as_str())
        {
            if !seen.insert(root) {
                blockers.push("duplicate_nullifier_root".to_string());
            }
        }
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn pq_root(
    root_id: &str,
    epoch: u64,
    aggregate_weight_bps: u64,
    status: RootStatus,
) -> AuthorizationRoot {
    AuthorizationRoot {
        root_id: root_id.to_string(),
        lane: RootLane::PqAuthorization,
        root: scoped_hash("pq-authorization-root", root_id),
        epoch,
        staleness_slots: 0,
        signer_set_root: scoped_hash("pq-signer-set", root_id),
        aggregate_weight_bps,
        status,
    }
}

fn reserve_root(
    closure_id: &str,
    reserve_piconero: u128,
    covered_liability_piconero: u128,
    coverage_bps: u64,
) -> ReserveClosureRoot {
    ReserveClosureRoot {
        closure_id: closure_id.to_string(),
        reserve_root: scoped_hash("reserve-root", closure_id),
        liability_root: scoped_hash("liability-root", closure_id),
        asset: "xmr-native-private-l2".to_string(),
        covered_liability_piconero,
        reserve_piconero,
        coverage_bps,
        status: RootStatus::Accepted,
    }
}

fn privacy_root(
    bucket_id: &str,
    amount_bucket_label: &str,
    anonymity_set_size: u64,
    bucket_amount_ppm: u64,
) -> AmountBucketPrivacyRoot {
    AmountBucketPrivacyRoot {
        bucket_id: bucket_id.to_string(),
        bucket_root: scoped_hash("amount-bucket-root", bucket_id),
        amount_bucket_label: amount_bucket_label.to_string(),
        anonymity_set_size,
        bucket_amount_ppm,
        redaction_root: scoped_hash("privacy-redaction-root", bucket_id),
        leakage_audit_root: scoped_hash("privacy-leakage-audit-root", bucket_id),
        status: RootStatus::Accepted,
    }
}

fn note_root(note_set_id: &str, activation_epoch: u64, note_count: u64) -> NoteCommitmentRoot {
    NoteCommitmentRoot {
        note_set_id: note_set_id.to_string(),
        commitment_root: scoped_hash("note-commitment-root", note_set_id),
        activation_epoch,
        note_count,
        asset_root: scoped_hash("note-asset-root", note_set_id),
        owner_view_tag_root: scoped_hash("owner-view-tag-root", note_set_id),
        status: RootStatus::Accepted,
    }
}

fn nullifier_root(
    reservation_id: &str,
    reservation_epoch: u64,
    reserved_count: u64,
) -> NullifierReservationRoot {
    NullifierReservationRoot {
        reservation_id: reservation_id.to_string(),
        nullifier_root: scoped_hash("nullifier-reservation-root", reservation_id),
        reservation_epoch,
        reserved_count,
        collision_scan_root: scoped_hash("nullifier-collision-scan", reservation_id),
        replay_guard_root: scoped_hash("nullifier-replay-guard", reservation_id),
        status: RootStatus::Accepted,
    }
}

fn credit_root(credit_root_id: &str) -> Wave105CreditRoot {
    Wave105CreditRoot {
        credit_root_id: credit_root_id.to_string(),
        credit_accounting_root: scoped_hash("wave105-credit-accounting", credit_root_id),
        debit_root: scoped_hash("wave105-debit", credit_root_id),
        release_credit_root: scoped_hash("wave105-release-credit", credit_root_id),
        carried_forward_liability_root: scoped_hash("wave105-carried-liability", credit_root_id),
        balanced: true,
        status: RootStatus::Accepted,
    }
}

fn breaker(breaker_id: &str, lane: RootLane) -> CircuitBreaker {
    CircuitBreaker {
        breaker_id: breaker_id.to_string(),
        lane,
        status: BreakerStatus::ArmedClosed,
        reason: "armed fail-closed guard is clear".to_string(),
        opened_at_slot: 0,
        evidence_root: scoped_hash("breaker-evidence", breaker_id),
    }
}

fn heavy_gate(gate_name: &str, ran_at_slot: u64) -> HeavyGateEvidence {
    HeavyGateEvidence {
        evidence_id: scoped_hash("heavy-gate-evidence-id", gate_name),
        gate_name: gate_name.to_string(),
        status: GateStatus::Passed,
        transcript_root: scoped_hash("heavy-gate-transcript", gate_name),
        vector_root: scoped_hash("heavy-gate-vector", gate_name),
        negative_case_root: scoped_hash("heavy-gate-negative-case", gate_name),
        operator_signature_root: scoped_hash("heavy-gate-operator-signature", gate_name),
        ran_at_slot,
    }
}

fn signoff(signoff_id: &str, role: SignoffRole, signed_root: &str, slot: u64) -> Signoff {
    Signoff {
        signoff_id: signoff_id.to_string(),
        role,
        signer_commitment: scoped_hash("signer-commitment", signoff_id),
        signed_root: signed_root.to_string(),
        approved: true,
        slot,
    }
}

fn scoped_hash(domain: &str, value: &str) -> String {
    domain_hash(
        &format!("monero-l2-wave106-pq-reserve-privacy-lane:{domain}"),
        &[HashPart::Str(CHAIN_ID), HashPart::Str(value)],
        32,
    )
}
