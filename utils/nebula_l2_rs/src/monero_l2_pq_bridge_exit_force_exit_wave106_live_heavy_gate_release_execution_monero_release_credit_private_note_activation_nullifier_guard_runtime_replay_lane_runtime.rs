use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = std::result::Result<T, ReplayError>;

pub const MODULE_ID: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave106-release-credit-private-note-activation-nullifier-guard-runtime-replay-lane";
pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave106-release-credit-private-note-activation-nullifier-guard-v1";
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const FAIL_CLOSED_RELEASE: &str = "release_credit_allowed: false";
pub const FAIL_CLOSED_NOTE: &str = "note_activation_allowed: false";
pub const FAIL_CLOSED_HEAVY_GATE: &str = "heavy_gates_ran: false";
pub const DEFAULT_WAVE: u64 = 106;
pub const PRIOR_ACCOUNTING_WAVE: u64 = 105;
pub const MIN_SIGNOFFS: usize = 6;
pub const MIN_HEAVY_GATE_EVIDENCE: usize = 4;
pub const MIN_REPLAY_STEPS: usize = 12;
pub const MIN_PQ_APPROVALS: usize = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub lane_id: String,
    pub protocol_version: String,
    pub hash_suite: String,
    pub wave: u64,
    pub prior_accounting_wave: u64,
    pub min_signoffs: usize,
    pub min_heavy_gate_evidence: usize,
    pub min_pq_approvals: usize,
    pub require_accounting_roots: bool,
    pub require_note_commitment_roots: bool,
    pub require_nullifier_reservation_roots: bool,
    pub require_amount_bucket_privacy_roots: bool,
    pub require_wallet_history_roots: bool,
    pub require_bridge_liability_closure_roots: bool,
    pub require_fee_rebate_settlement_roots: bool,
    pub require_pq_authorization: bool,
    pub require_circuit_breakers_clear: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
}

impl Config {
    pub fn fail_closed() -> Self {
        Self {
            lane_id: MODULE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            hash_suite: HASH_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            prior_accounting_wave: PRIOR_ACCOUNTING_WAVE,
            min_signoffs: MIN_SIGNOFFS,
            min_heavy_gate_evidence: MIN_HEAVY_GATE_EVIDENCE,
            min_pq_approvals: MIN_PQ_APPROVALS,
            require_accounting_roots: true,
            require_note_commitment_roots: true,
            require_nullifier_reservation_roots: true,
            require_amount_bucket_privacy_roots: true,
            require_wallet_history_roots: true,
            require_bridge_liability_closure_roots: true,
            require_fee_rebate_settlement_roots: true,
            require_pq_authorization: true,
            require_circuit_breakers_clear: true,
            require_live_heavy_gate_evidence: true,
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn devnet() -> Self {
        Self {
            min_signoffs: 7,
            min_heavy_gate_evidence: 5,
            min_pq_approvals: 4,
            ..Self::fail_closed()
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("lane_id", &self.lane_id)?;
        require_non_empty("protocol_version", &self.protocol_version)?;
        require_non_empty("hash_suite", &self.hash_suite)?;
        if self.wave != DEFAULT_WAVE {
            return Err(ReplayError::InvalidConfig(
                "wave must remain bound to Wave 106".to_string(),
            ));
        }
        if self.prior_accounting_wave != PRIOR_ACCOUNTING_WAVE {
            return Err(ReplayError::InvalidConfig(
                "prior accounting wave must remain bound to Wave 105".to_string(),
            ));
        }
        if self.min_signoffs < MIN_SIGNOFFS {
            return Err(ReplayError::InvalidConfig(
                "signoff quorum below Wave 106 floor".to_string(),
            ));
        }
        if self.min_heavy_gate_evidence < MIN_HEAVY_GATE_EVIDENCE {
            return Err(ReplayError::InvalidConfig(
                "heavy gate evidence below Wave 106 floor".to_string(),
            ));
        }
        if self.min_pq_approvals < MIN_PQ_APPROVALS {
            return Err(ReplayError::InvalidConfig(
                "PQ approval quorum below Wave 106 floor".to_string(),
            ));
        }
        if self.release_credit_allowed || self.note_activation_allowed || self.heavy_gates_ran {
            return Err(ReplayError::InvalidConfig(format!(
                "{}; {}; {}",
                FAIL_CLOSED_RELEASE, FAIL_CLOSED_NOTE, FAIL_CLOSED_HEAVY_GATE
            )));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane_id": self.lane_id,
            "protocol_version": self.protocol_version,
            "hash_suite": self.hash_suite,
            "wave": self.wave,
            "prior_accounting_wave": self.prior_accounting_wave,
            "minimums": {
                "signoffs": self.min_signoffs,
                "heavy_gate_evidence": self.min_heavy_gate_evidence,
                "pq_approvals": self.min_pq_approvals
            },
            "required": {
                "accounting_roots": self.require_accounting_roots,
                "note_commitment_roots": self.require_note_commitment_roots,
                "nullifier_reservation_roots": self.require_nullifier_reservation_roots,
                "amount_bucket_privacy_roots": self.require_amount_bucket_privacy_roots,
                "wallet_history_roots": self.require_wallet_history_roots,
                "bridge_liability_closure_roots": self.require_bridge_liability_closure_roots,
                "fee_rebate_settlement_roots": self.require_fee_rebate_settlement_roots,
                "pq_authorization": self.require_pq_authorization,
                "circuit_breakers_clear": self.require_circuit_breakers_clear,
                "live_heavy_gate_evidence": self.require_live_heavy_gate_evidence
            },
            "fail_closed": {
                "release_credit_allowed": self.release_credit_allowed,
                "note_activation_allowed": self.note_activation_allowed,
                "heavy_gates_ran": self.heavy_gates_ran,
                "record": fail_closed_record()
            }
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::fail_closed()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub sequence: ReplaySequence,
    pub verdict: ReplayVerdict,
    pub public_record: PublicRecord,
}

impl State {
    pub fn new(config: Config, sequence: ReplaySequence) -> Result<Self> {
        config.validate()?;
        sequence.validate()?;
        let verdict = evaluate_sequence(&config, &sequence);
        let mut state = Self {
            config,
            sequence,
            verdict,
            public_record: fail_closed_public_record("state root pending"),
        };
        state.public_record = public_record(&state);
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.sequence.validate()?;
        self.verdict.validate()?;
        let actual = state_root(self);
        let recorded = match self.public_record.get("state_root").and_then(Value::as_str) {
            Some(value) => value,
            None => "",
        };
        if recorded != actual {
            return Err(ReplayError::RootMismatch(
                "public record state root mismatch".to_string(),
            ));
        }
        Ok(())
    }

    pub fn release_credit_allowed(&self) -> bool {
        self.verdict.release_credit_allowed
    }

    pub fn note_activation_allowed(&self) -> bool {
        self.verdict.note_activation_allowed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplaySequence {
    pub sequence_id: String,
    pub claim_id: String,
    pub wave105_accounting: RootBundle,
    pub note_commitments: RootBundle,
    pub nullifier_reservations: RootBundle,
    pub amount_bucket_privacy: RootBundle,
    pub wallet_history: RootBundle,
    pub bridge_liability_closure: RootBundle,
    pub fee_rebate_settlement: RootBundle,
    pub pq_authorization: PqAuthorization,
    pub circuit_breakers: CircuitBreakerSet,
    pub live_heavy_gate: LiveHeavyGateEvidence,
    pub signoffs: SignoffSet,
    pub replay_steps: Vec<ReplayStep>,
}

impl ReplaySequence {
    pub fn validate(&self) -> Result<()> {
        require_non_empty("sequence_id", &self.sequence_id)?;
        require_non_empty("claim_id", &self.claim_id)?;
        self.wave105_accounting
            .validate(RootDomain::Wave105Accounting)?;
        self.note_commitments.validate(RootDomain::NoteCommitment)?;
        self.nullifier_reservations
            .validate(RootDomain::NullifierReservation)?;
        self.amount_bucket_privacy
            .validate(RootDomain::AmountBucketPrivacy)?;
        self.wallet_history.validate(RootDomain::WalletHistory)?;
        self.bridge_liability_closure
            .validate(RootDomain::BridgeLiabilityClosure)?;
        self.fee_rebate_settlement
            .validate(RootDomain::FeeRebateSettlement)?;
        self.pq_authorization.validate()?;
        self.circuit_breakers.validate()?;
        self.live_heavy_gate.validate()?;
        self.signoffs.validate()?;
        validate_replay_steps(&self.replay_steps)?;
        Ok(())
    }

    pub fn devnet() -> Self {
        let claim_id = "force-exit-claim-devnet-106";
        Self {
            sequence_id: "wave106-private-note-activation-replay-devnet".to_string(),
            claim_id: claim_id.to_string(),
            wave105_accounting: RootBundle::devnet(RootDomain::Wave105Accounting, claim_id, 105),
            note_commitments: RootBundle::devnet(RootDomain::NoteCommitment, claim_id, 106),
            nullifier_reservations: RootBundle::devnet(
                RootDomain::NullifierReservation,
                claim_id,
                106,
            ),
            amount_bucket_privacy: RootBundle::devnet(
                RootDomain::AmountBucketPrivacy,
                claim_id,
                106,
            ),
            wallet_history: RootBundle::devnet(RootDomain::WalletHistory, claim_id, 106),
            bridge_liability_closure: RootBundle::devnet(
                RootDomain::BridgeLiabilityClosure,
                claim_id,
                106,
            ),
            fee_rebate_settlement: RootBundle::devnet(
                RootDomain::FeeRebateSettlement,
                claim_id,
                106,
            ),
            pq_authorization: PqAuthorization::devnet(claim_id),
            circuit_breakers: CircuitBreakerSet::devnet(),
            live_heavy_gate: LiveHeavyGateEvidence::devnet(claim_id),
            signoffs: SignoffSet::devnet(),
            replay_steps: devnet_replay_steps(claim_id),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "sequence_id": self.sequence_id,
            "claim_id": self.claim_id,
            "roots": {
                "wave105_accounting": self.wave105_accounting.public_record(),
                "note_commitments": self.note_commitments.public_record(),
                "nullifier_reservations": self.nullifier_reservations.public_record(),
                "amount_bucket_privacy": self.amount_bucket_privacy.public_record(),
                "wallet_history": self.wallet_history.public_record(),
                "bridge_liability_closure": self.bridge_liability_closure.public_record(),
                "fee_rebate_settlement": self.fee_rebate_settlement.public_record()
            },
            "pq_authorization": self.pq_authorization.public_record(),
            "circuit_breakers": self.circuit_breakers.public_record(),
            "live_heavy_gate": self.live_heavy_gate.public_record(),
            "signoffs": self.signoffs.public_record(),
            "replay_steps": self.replay_steps.iter().map(ReplayStep::public_record).collect::<Vec<_>>(),
            "sequence_root": self.sequence_root()
        })
    }

    pub fn sequence_root(&self) -> String {
        root_from_value(
            "wave106.sequence",
            &json!({
                "sequence_id": self.sequence_id,
                "claim_id": self.claim_id,
                "wave105_accounting": self.wave105_accounting.bundle_root,
                "note_commitments": self.note_commitments.bundle_root,
                "nullifier_reservations": self.nullifier_reservations.bundle_root,
                "amount_bucket_privacy": self.amount_bucket_privacy.bundle_root,
                "wallet_history": self.wallet_history.bundle_root,
                "bridge_liability_closure": self.bridge_liability_closure.bundle_root,
                "fee_rebate_settlement": self.fee_rebate_settlement.bundle_root,
                "pq_authorization": self.pq_authorization.authorization_root,
                "circuit_breakers": self.circuit_breakers.breaker_root(),
                "live_heavy_gate": self.live_heavy_gate.evidence_root(),
                "signoffs": self.signoffs.signoff_root(),
                "replay_steps": replay_step_root(&self.replay_steps)
            }),
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RootDomain {
    Wave105Accounting,
    NoteCommitment,
    NullifierReservation,
    AmountBucketPrivacy,
    WalletHistory,
    BridgeLiabilityClosure,
    FeeRebateSettlement,
}

impl RootDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave105Accounting => "wave105_accounting",
            Self::NoteCommitment => "note_commitment",
            Self::NullifierReservation => "nullifier_reservation",
            Self::AmountBucketPrivacy => "amount_bucket_privacy",
            Self::WalletHistory => "wallet_history",
            Self::BridgeLiabilityClosure => "bridge_liability_closure",
            Self::FeeRebateSettlement => "fee_rebate_settlement",
        }
    }
}

impl fmt::Display for RootDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootBundle {
    pub domain: RootDomain,
    pub source_wave: u64,
    pub accounting_root: String,
    pub note_root: String,
    pub nullifier_root: String,
    pub privacy_root: String,
    pub wallet_history_root: String,
    pub liability_root: String,
    pub fee_rebate_root: String,
    pub bundle_root: String,
    pub accepted: bool,
}

impl RootBundle {
    pub fn devnet(domain: RootDomain, claim_id: &str, source_wave: u64) -> Self {
        let accounting_root = named_root(domain, "accounting", claim_id);
        let note_root = named_root(domain, "note", claim_id);
        let nullifier_root = named_root(domain, "nullifier", claim_id);
        let privacy_root = named_root(domain, "privacy", claim_id);
        let wallet_history_root = named_root(domain, "wallet-history", claim_id);
        let liability_root = named_root(domain, "liability", claim_id);
        let fee_rebate_root = named_root(domain, "fee-rebate", claim_id);
        let bundle_root = root_from_value(
            "wave106.root_bundle",
            &json!({
                "domain": domain.as_str(),
                "source_wave": source_wave,
                "accounting_root": accounting_root,
                "note_root": note_root,
                "nullifier_root": nullifier_root,
                "privacy_root": privacy_root,
                "wallet_history_root": wallet_history_root,
                "liability_root": liability_root,
                "fee_rebate_root": fee_rebate_root
            }),
        );
        Self {
            domain,
            source_wave,
            accounting_root,
            note_root,
            nullifier_root,
            privacy_root,
            wallet_history_root,
            liability_root,
            fee_rebate_root,
            bundle_root,
            accepted: true,
        }
    }

    pub fn validate(&self, domain: RootDomain) -> Result<()> {
        if self.domain != domain {
            return Err(ReplayError::InvalidEvidence(format!(
                "root bundle domain mismatch for {}",
                domain
            )));
        }
        require_root("accounting_root", &self.accounting_root)?;
        require_root("note_root", &self.note_root)?;
        require_root("nullifier_root", &self.nullifier_root)?;
        require_root("privacy_root", &self.privacy_root)?;
        require_root("wallet_history_root", &self.wallet_history_root)?;
        require_root("liability_root", &self.liability_root)?;
        require_root("fee_rebate_root", &self.fee_rebate_root)?;
        require_root("bundle_root", &self.bundle_root)?;
        if !self.accepted {
            return Err(ReplayError::ReplayDenied(format!(
                "{} root bundle is not accepted",
                domain
            )));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "domain": self.domain.as_str(),
            "source_wave": self.source_wave,
            "accounting_root": self.accounting_root,
            "note_root": self.note_root,
            "nullifier_root": self.nullifier_root,
            "privacy_root": self.privacy_root,
            "wallet_history_root": self.wallet_history_root,
            "liability_root": self.liability_root,
            "fee_rebate_root": self.fee_rebate_root,
            "bundle_root": self.bundle_root,
            "accepted": self.accepted
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqAuthorization {
    pub authority_set_root: String,
    pub authorization_root: String,
    pub approvals: Vec<String>,
    pub revoked_keys_root: String,
    pub accepted: bool,
}

impl PqAuthorization {
    pub fn devnet(claim_id: &str) -> Self {
        let approvals = (1..=4)
            .map(|index| {
                named_root(
                    RootDomain::NullifierReservation,
                    "pq-approval",
                    &format!("{claim_id}-{index}"),
                )
            })
            .collect::<Vec<_>>();
        let authority_set_root =
            named_root(RootDomain::NullifierReservation, "pq-authority", claim_id);
        let revoked_keys_root =
            named_root(RootDomain::NullifierReservation, "pq-revoked", claim_id);
        let authorization_root = root_from_value(
            "wave106.pq_authorization",
            &json!({
                "authority_set_root": authority_set_root,
                "approvals": approvals,
                "revoked_keys_root": revoked_keys_root
            }),
        );
        Self {
            authority_set_root,
            authorization_root,
            approvals,
            revoked_keys_root,
            accepted: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_root("authority_set_root", &self.authority_set_root)?;
        require_root("authorization_root", &self.authorization_root)?;
        require_root("revoked_keys_root", &self.revoked_keys_root)?;
        require_unique_roots("pq approvals", &self.approvals)?;
        if !self.accepted {
            return Err(ReplayError::ReplayDenied(
                "PQ authorization is not accepted".to_string(),
            ));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "authority_set_root": self.authority_set_root,
            "authorization_root": self.authorization_root,
            "approvals": self.approvals,
            "revoked_keys_root": self.revoked_keys_root,
            "accepted": self.accepted
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitBreakerSet {
    pub global_halt: bool,
    pub release_halt: bool,
    pub note_activation_halt: bool,
    pub nullifier_reservation_halt: bool,
    pub privacy_budget_halt: bool,
    pub breaker_observation_root: String,
}

impl CircuitBreakerSet {
    pub fn devnet() -> Self {
        Self {
            global_halt: false,
            release_halt: false,
            note_activation_halt: false,
            nullifier_reservation_halt: false,
            privacy_budget_halt: false,
            breaker_observation_root: named_root(
                RootDomain::BridgeLiabilityClosure,
                "circuit-breakers",
                "devnet-106",
            ),
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_root("breaker_observation_root", &self.breaker_observation_root)
    }

    pub fn all_clear(&self) -> bool {
        !self.global_halt
            && !self.release_halt
            && !self.note_activation_halt
            && !self.nullifier_reservation_halt
            && !self.privacy_budget_halt
    }

    pub fn breaker_root(&self) -> String {
        root_from_value("wave106.circuit_breakers", &self.public_record())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "global_halt": self.global_halt,
            "release_halt": self.release_halt,
            "note_activation_halt": self.note_activation_halt,
            "nullifier_reservation_halt": self.nullifier_reservation_halt,
            "privacy_budget_halt": self.privacy_budget_halt,
            "breaker_observation_root": self.breaker_observation_root,
            "all_clear": self.all_clear()
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveHeavyGateEvidence {
    pub run_id: String,
    pub heavy_gates_ran: bool,
    pub observation_roots: Vec<String>,
    pub transcript_root: String,
    pub accepted: bool,
}

impl LiveHeavyGateEvidence {
    pub fn devnet(claim_id: &str) -> Self {
        Self {
            run_id: "wave106-live-heavy-gate-devnet".to_string(),
            heavy_gates_ran: true,
            observation_roots: (1..=5)
                .map(|index| {
                    named_root(
                        RootDomain::AmountBucketPrivacy,
                        "heavy-gate-observation",
                        &format!("{claim_id}-{index}"),
                    )
                })
                .collect(),
            transcript_root: named_root(
                RootDomain::AmountBucketPrivacy,
                "heavy-gate-transcript",
                claim_id,
            ),
            accepted: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("run_id", &self.run_id)?;
        require_unique_roots("heavy gate observation roots", &self.observation_roots)?;
        require_root("transcript_root", &self.transcript_root)?;
        Ok(())
    }

    pub fn evidence_root(&self) -> String {
        root_from_value("wave106.live_heavy_gate", &self.public_record())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "run_id": self.run_id,
            "heavy_gates_ran": self.heavy_gates_ran,
            "observation_roots": self.observation_roots,
            "transcript_root": self.transcript_root,
            "accepted": self.accepted
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignoffSet {
    pub release_captain: String,
    pub runtime_replay: String,
    pub pq_security: String,
    pub privacy: String,
    pub bridge_custody: String,
    pub wallet_watchtower: String,
    pub audit_security: String,
    pub accepted: Vec<String>,
}

impl SignoffSet {
    pub fn devnet() -> Self {
        let labels = [
            "release-captain",
            "runtime-replay",
            "pq-security",
            "privacy",
            "bridge-custody",
            "wallet-watchtower",
            "audit-security",
        ];
        Self {
            release_captain: named_root(RootDomain::WalletHistory, "signoff", labels[0]),
            runtime_replay: named_root(RootDomain::WalletHistory, "signoff", labels[1]),
            pq_security: named_root(RootDomain::WalletHistory, "signoff", labels[2]),
            privacy: named_root(RootDomain::WalletHistory, "signoff", labels[3]),
            bridge_custody: named_root(RootDomain::WalletHistory, "signoff", labels[4]),
            wallet_watchtower: named_root(RootDomain::WalletHistory, "signoff", labels[5]),
            audit_security: named_root(RootDomain::WalletHistory, "signoff", labels[6]),
            accepted: labels.iter().map(|value| value.to_string()).collect(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_root("release_captain", &self.release_captain)?;
        require_root("runtime_replay", &self.runtime_replay)?;
        require_root("pq_security", &self.pq_security)?;
        require_root("privacy", &self.privacy)?;
        require_root("bridge_custody", &self.bridge_custody)?;
        require_root("wallet_watchtower", &self.wallet_watchtower)?;
        require_root("audit_security", &self.audit_security)?;
        require_non_empty_values("accepted signoffs", &self.accepted)?;
        Ok(())
    }

    pub fn accepted_count(&self) -> usize {
        self.accepted.iter().collect::<BTreeSet<_>>().len()
    }

    pub fn signoff_root(&self) -> String {
        root_from_value("wave106.signoffs", &self.public_record())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "release_captain": self.release_captain,
            "runtime_replay": self.runtime_replay,
            "pq_security": self.pq_security,
            "privacy": self.privacy,
            "bridge_custody": self.bridge_custody,
            "wallet_watchtower": self.wallet_watchtower,
            "audit_security": self.audit_security,
            "accepted": self.accepted,
            "accepted_count": self.accepted_count()
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDomain {
    Wave105Accounting,
    NoteCommitmentRoot,
    NullifierReservationRoot,
    AmountBucketPrivacyRoot,
    WalletHistoryRoot,
    BridgeLiabilityClosureRoot,
    FeeRebateSettlementRoot,
    PqAuthorization,
    CircuitBreakers,
    LiveHeavyGateEvidence,
    SignoffQuorum,
    SpendablePrivateNoteState,
}

impl ReplayDomain {
    pub fn all_required() -> Vec<Self> {
        vec![
            Self::Wave105Accounting,
            Self::NoteCommitmentRoot,
            Self::NullifierReservationRoot,
            Self::AmountBucketPrivacyRoot,
            Self::WalletHistoryRoot,
            Self::BridgeLiabilityClosureRoot,
            Self::FeeRebateSettlementRoot,
            Self::PqAuthorization,
            Self::CircuitBreakers,
            Self::LiveHeavyGateEvidence,
            Self::SignoffQuorum,
            Self::SpendablePrivateNoteState,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave105Accounting => "wave105_accounting",
            Self::NoteCommitmentRoot => "note_commitment_root",
            Self::NullifierReservationRoot => "nullifier_reservation_root",
            Self::AmountBucketPrivacyRoot => "amount_bucket_privacy_root",
            Self::WalletHistoryRoot => "wallet_history_root",
            Self::BridgeLiabilityClosureRoot => "bridge_liability_closure_root",
            Self::FeeRebateSettlementRoot => "fee_rebate_settlement_root",
            Self::PqAuthorization => "pq_authorization",
            Self::CircuitBreakers => "circuit_breakers",
            Self::LiveHeavyGateEvidence => "live_heavy_gate_evidence",
            Self::SignoffQuorum => "signoff_quorum",
            Self::SpendablePrivateNoteState => "spendable_private_note_state",
        }
    }
}

impl fmt::Display for ReplayDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayStep {
    pub step_index: usize,
    pub domain: ReplayDomain,
    pub input_root: String,
    pub output_root: String,
    pub binds_release_credit: bool,
    pub binds_private_note_state: bool,
    pub accepted: bool,
}

impl ReplayStep {
    pub fn new(
        step_index: usize,
        domain: ReplayDomain,
        input_root: String,
        output_root: String,
        binds_release_credit: bool,
        binds_private_note_state: bool,
    ) -> Self {
        Self {
            step_index,
            domain,
            input_root,
            output_root,
            binds_release_credit,
            binds_private_note_state,
            accepted: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.step_index == 0 {
            return Err(ReplayError::InvalidEvidence(
                "replay step index must be one based".to_string(),
            ));
        }
        require_root("input_root", &self.input_root)?;
        require_root("output_root", &self.output_root)?;
        if !self.accepted {
            return Err(ReplayError::ReplayDenied(format!(
                "replay step {} was not accepted",
                self.step_index
            )));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "step_index": self.step_index,
            "domain": self.domain.as_str(),
            "input_root": self.input_root,
            "output_root": self.output_root,
            "binds_release_credit": self.binds_release_credit,
            "binds_private_note_state": self.binds_private_note_state,
            "accepted": self.accepted
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayVerdict {
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub denied_reasons: Vec<String>,
    pub accepted_checks: Vec<String>,
    pub fail_closed_record: String,
}

impl ReplayVerdict {
    pub fn fail_closed(reason: &str) -> Self {
        Self {
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            denied_reasons: vec![reason.to_string()],
            accepted_checks: Vec::new(),
            fail_closed_record: fail_closed_record(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.fail_closed_record != fail_closed_record() {
            return Err(ReplayError::InvalidEvidence(
                "fail closed record text mismatch".to_string(),
            ));
        }
        if (!self.release_credit_allowed || !self.note_activation_allowed || !self.heavy_gates_ran)
            && self.denied_reasons.is_empty()
        {
            return Err(ReplayError::ReplayDenied(
                "denied verdict must carry a reason".to_string(),
            ));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "denied_reasons": self.denied_reasons,
            "accepted_checks": self.accepted_checks,
            "fail_closed_record": self.fail_closed_record
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayError {
    InvalidConfig(String),
    InvalidEvidence(String),
    ReplayDenied(String),
    RootMismatch(String),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => write!(formatter, "invalid config: {message}"),
            Self::InvalidEvidence(message) => write!(formatter, "invalid evidence: {message}"),
            Self::ReplayDenied(message) => write!(formatter, "replay denied: {message}"),
            Self::RootMismatch(message) => write!(formatter, "root mismatch: {message}"),
        }
    }
}

impl std::error::Error for ReplayError {}

pub fn devnet() -> State {
    match State::new(Config::devnet(), ReplaySequence::devnet()) {
        Ok(state) => state,
        Err(error) => State {
            config: Config::fail_closed(),
            sequence: ReplaySequence::devnet(),
            verdict: ReplayVerdict::fail_closed(&error.to_string()),
            public_record: fail_closed_public_record(&error.to_string()),
        },
    }
}

pub fn public_record(state: &State) -> PublicRecord {
    let record_without_root = public_record_without_root(state);
    let state_root = root_from_value("wave106.state", &record_without_root);
    json!({
        "module_id": MODULE_ID,
        "state_root": state_root,
        "record": record_without_root
    })
}

pub fn state_root(state: &State) -> String {
    root_from_value("wave106.state", &public_record_without_root(state))
}

fn public_record_without_root(state: &State) -> PublicRecord {
    json!({
        "module_id": MODULE_ID,
        "config": state.config.public_record(),
        "sequence": state.sequence.public_record(),
        "verdict": state.verdict.public_record(),
        "summary": {
            "release_credit_allowed": state.verdict.release_credit_allowed,
            "note_activation_allowed": state.verdict.note_activation_allowed,
            "heavy_gates_ran": state.verdict.heavy_gates_ran,
            "denied_count": state.verdict.denied_reasons.len(),
            "accepted_count": state.verdict.accepted_checks.len(),
            "fail_closed_record": fail_closed_record()
        }
    })
}

fn evaluate_sequence(config: &Config, sequence: &ReplaySequence) -> ReplayVerdict {
    let mut accepted = Vec::new();
    let mut denied = Vec::new();
    collect_result(
        "config accepted",
        config.validate(),
        &mut accepted,
        &mut denied,
    );
    collect_result(
        "sequence evidence accepted",
        sequence.validate(),
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "Wave 105 accounted release credit roots bound",
        !config.require_accounting_roots || sequence.wave105_accounting.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "note commitment roots bound",
        !config.require_note_commitment_roots || sequence.note_commitments.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "nullifier reservation roots bound",
        !config.require_nullifier_reservation_roots || sequence.nullifier_reservations.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "amount bucket privacy roots bound",
        !config.require_amount_bucket_privacy_roots || sequence.amount_bucket_privacy.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "wallet history roots bound",
        !config.require_wallet_history_roots || sequence.wallet_history.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "bridge liability closure roots bound",
        !config.require_bridge_liability_closure_roots
            || sequence.bridge_liability_closure.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "fee rebate settlement roots bound",
        !config.require_fee_rebate_settlement_roots || sequence.fee_rebate_settlement.accepted,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "PQ authorization quorum accepted",
        !config.require_pq_authorization
            || (sequence.pq_authorization.accepted
                && sequence.pq_authorization.approvals.len() >= config.min_pq_approvals),
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "circuit breakers clear",
        !config.require_circuit_breakers_clear || sequence.circuit_breakers.all_clear(),
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "live heavy gate evidence accepted",
        !config.require_live_heavy_gate_evidence
            || (sequence.live_heavy_gate.accepted
                && sequence.live_heavy_gate.heavy_gates_ran
                && sequence.live_heavy_gate.observation_roots.len()
                    >= config.min_heavy_gate_evidence),
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "signoff quorum accepted",
        sequence.signoffs.accepted_count() >= config.min_signoffs,
        &mut accepted,
        &mut denied,
    );
    collect_bool(
        "release credit replay binding present",
        sequence.replay_steps.iter().any(|step| {
            step.binds_release_credit && step.domain == ReplayDomain::Wave105Accounting
        }),
        &mut accepted,
        &mut denied,
    );
    let final_step_binds_private_note = match sequence.replay_steps.last() {
        Some(step) => {
            step.binds_private_note_state && step.domain == ReplayDomain::SpendablePrivateNoteState
        }
        None => false,
    };
    collect_bool(
        "spendable private note replay binding last",
        final_step_binds_private_note,
        &mut accepted,
        &mut denied,
    );

    if denied.is_empty() {
        ReplayVerdict {
            release_credit_allowed: true,
            note_activation_allowed: true,
            heavy_gates_ran: sequence.live_heavy_gate.heavy_gates_ran,
            denied_reasons: Vec::new(),
            accepted_checks: accepted,
            fail_closed_record: fail_closed_record(),
        }
    } else {
        ReplayVerdict {
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            denied_reasons: denied,
            accepted_checks: accepted,
            fail_closed_record: fail_closed_record(),
        }
    }
}

fn validate_replay_steps(steps: &[ReplayStep]) -> Result<()> {
    if steps.len() < MIN_REPLAY_STEPS {
        return Err(ReplayError::ReplayDenied(
            "runtime replay lane must bind every private-note activation domain".to_string(),
        ));
    }
    let mut domains = BTreeSet::new();
    let mut previous_output = String::new();
    for (offset, step) in steps.iter().enumerate() {
        step.validate()?;
        if step.step_index != offset + 1 {
            return Err(ReplayError::ReplayDenied(
                "replay steps must be contiguous".to_string(),
            ));
        }
        if offset > 0 && step.input_root != previous_output {
            return Err(ReplayError::ReplayDenied(
                "replay steps must chain output roots into input roots".to_string(),
            ));
        }
        previous_output = step.output_root.clone();
        domains.insert(step.domain);
    }
    for domain in ReplayDomain::all_required() {
        if !domains.contains(&domain) {
            return Err(ReplayError::ReplayDenied(format!(
                "missing replay domain {}",
                domain
            )));
        }
    }
    Ok(())
}

fn devnet_replay_steps(claim_id: &str) -> Vec<ReplayStep> {
    let mut steps = Vec::new();
    let mut prior = named_root(RootDomain::Wave105Accounting, "replay-start", claim_id);
    for (offset, domain) in ReplayDomain::all_required().into_iter().enumerate() {
        let index = offset + 1;
        let output = root_from_value(
            "wave106.replay_step",
            &json!({
                "claim_id": claim_id,
                "index": index,
                "domain": domain.as_str(),
                "input_root": prior
            }),
        );
        let binds_release_credit = domain == ReplayDomain::Wave105Accounting;
        let binds_private_note_state = domain == ReplayDomain::SpendablePrivateNoteState;
        steps.push(ReplayStep::new(
            index,
            domain,
            prior,
            output.clone(),
            binds_release_credit,
            binds_private_note_state,
        ));
        prior = output;
    }
    steps
}

fn replay_step_root(steps: &[ReplayStep]) -> String {
    merkle_root(
        "wave106.replay_steps",
        &steps
            .iter()
            .map(ReplayStep::public_record)
            .collect::<Vec<_>>(),
    )
}

fn named_root(domain: RootDomain, lane: &str, label: &str) -> String {
    domain_hash(
        "wave106.named_root",
        &[
            HashPart::Str(domain.as_str()),
            HashPart::Str(lane),
            HashPart::Str(label),
        ],
        32,
    )
}

fn root_from_value(domain: &str, value: &Value) -> String {
    domain_hash(domain, &[HashPart::Json(value)], 32)
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ReplayError::InvalidEvidence(format!(
            "{} must not be empty",
            field
        )));
    }
    Ok(())
}

fn require_non_empty_values(field: &str, values: &[String]) -> Result<()> {
    if values.is_empty() {
        return Err(ReplayError::InvalidEvidence(format!(
            "{} must not be empty",
            field
        )));
    }
    for value in values {
        require_non_empty(field, value)?;
    }
    Ok(())
}

fn require_root(field: &str, value: &str) -> Result<()> {
    require_non_empty(field, value)?;
    if value.len() != 64 || !value.chars().all(|item| item.is_ascii_hexdigit()) {
        return Err(ReplayError::InvalidEvidence(format!(
            "{} must be a 32 byte hex root",
            field
        )));
    }
    Ok(())
}

fn require_unique_roots(field: &str, roots: &[String]) -> Result<()> {
    if roots.is_empty() {
        return Err(ReplayError::InvalidEvidence(format!(
            "{} must not be empty",
            field
        )));
    }
    let mut unique = BTreeSet::new();
    for root in roots {
        require_root(field, root)?;
        if !unique.insert(root) {
            return Err(ReplayError::InvalidEvidence(format!(
                "{} must be unique",
                field
            )));
        }
    }
    Ok(())
}

fn collect_result(
    label: &str,
    result: Result<()>,
    accepted: &mut Vec<String>,
    denied: &mut Vec<String>,
) {
    match result {
        Ok(()) => accepted.push(label.to_string()),
        Err(error) => denied.push(format!("{}: {}", label, error)),
    }
}

fn collect_bool(label: &str, passed: bool, accepted: &mut Vec<String>, denied: &mut Vec<String>) {
    if passed {
        accepted.push(label.to_string());
    } else {
        denied.push(label.to_string());
    }
}

fn fail_closed_record() -> String {
    format!(
        "{}; {}; {}",
        FAIL_CLOSED_RELEASE, FAIL_CLOSED_NOTE, FAIL_CLOSED_HEAVY_GATE
    )
}

fn fail_closed_public_record(reason: &str) -> PublicRecord {
    json!({
        "module_id": MODULE_ID,
        "state_root": root_from_value("wave106.fail_closed", &json!({"reason": reason})),
        "record": {
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
            "fail_closed_record": fail_closed_record(),
            "reason": reason
        }
    })
}
