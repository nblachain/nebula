use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = std::result::Result<T, String>;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-wave106-release-credit-private-note-nullifier-compile-lane-v1";
pub const PUBLIC_RECORD_SCHEMA: &str =
    "monero-l2-pq-wave106-release-credit-private-note-nullifier-public-record-v1";
pub const DEVNET_HEIGHT: u64 = 106_000;
pub const DEVNET_EPOCH: u64 = 106;
pub const DEVNET_MONERO_NETWORK: &str = "monero-devnet";
pub const DEVNET_L2_NETWORK: &str = "nebula-private-l2-devnet";
pub const DEVNET_ASSET_ID: &str = "xmr-release-credit-devnet";
pub const DEVNET_FEE_ASSET_ID: &str = "piconero-fee-rebate-devnet";
pub const DEFAULT_MIN_OPERATOR_SIGNOFFS: u16 = 3;
pub const DEFAULT_MIN_REVIEWER_SIGNOFFS: u16 = 2;
pub const DEFAULT_MIN_PQ_SECURITY_BITS: u16 = 256;
pub const DEFAULT_MAX_ROOT_AGE_BLOCKS: u64 = 12;
pub const DEFAULT_HEAVY_GATE_TTL_BLOCKS: u64 = 24;
pub const DEFAULT_MAX_NOTE_DELAY_BLOCKS: u64 = 4;
pub const DEFAULT_MIN_HEAVY_GATE_SCORE_BPS: u64 = 10_000;
pub const DEFAULT_MAX_BPS: u64 = 10_000;
pub const EMPTY_ROOT_LABEL: &str = "wave106-private-note-nullifier-guard-empty-root";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootGateKind {
    Wave105CreditAccounting,
    PrivateNoteCommitment,
    NullifierReservation,
    AmountBucketPrivacy,
    BeneficiaryWalletHistory,
    BridgeLiabilityClosure,
    FeeRebateSettlement,
    PqAuthorization,
    CircuitBreaker,
}

impl RootGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave105CreditAccounting => "wave105_credit_accounting",
            Self::PrivateNoteCommitment => "private_note_commitment",
            Self::NullifierReservation => "nullifier_reservation",
            Self::AmountBucketPrivacy => "amount_bucket_privacy",
            Self::BeneficiaryWalletHistory => "beneficiary_wallet_history",
            Self::BridgeLiabilityClosure => "bridge_liability_closure",
            Self::FeeRebateSettlement => "fee_rebate_settlement",
            Self::PqAuthorization => "pq_authorization",
            Self::CircuitBreaker => "circuit_breaker",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Wave105CreditAccounting,
            Self::PrivateNoteCommitment,
            Self::NullifierReservation,
            Self::AmountBucketPrivacy,
            Self::BeneficiaryWalletHistory,
            Self::BridgeLiabilityClosure,
            Self::FeeRebateSettlement,
            Self::PqAuthorization,
            Self::CircuitBreaker,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeavyGateKind {
    CreditAccountingReplay,
    PrivateNoteActivationDryRun,
    NullifierReservationReplay,
    AmountBucketLeakageScan,
    WalletHistoryJoinScan,
    LiabilityClosureReplay,
    FeeRebateSettlementReplay,
    PqAuthorizationBatchVerify,
    CircuitBreakerTripDrill,
}

impl HeavyGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditAccountingReplay => "credit_accounting_replay",
            Self::PrivateNoteActivationDryRun => "private_note_activation_dry_run",
            Self::NullifierReservationReplay => "nullifier_reservation_replay",
            Self::AmountBucketLeakageScan => "amount_bucket_leakage_scan",
            Self::WalletHistoryJoinScan => "wallet_history_join_scan",
            Self::LiabilityClosureReplay => "liability_closure_replay",
            Self::FeeRebateSettlementReplay => "fee_rebate_settlement_replay",
            Self::PqAuthorizationBatchVerify => "pq_authorization_batch_verify",
            Self::CircuitBreakerTripDrill => "circuit_breaker_trip_drill",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::CreditAccountingReplay,
            Self::PrivateNoteActivationDryRun,
            Self::NullifierReservationReplay,
            Self::AmountBucketLeakageScan,
            Self::WalletHistoryJoinScan,
            Self::LiabilityClosureReplay,
            Self::FeeRebateSettlementReplay,
            Self::PqAuthorizationBatchVerify,
            Self::CircuitBreakerTripDrill,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffRole {
    Operator,
    Reviewer,
    Watchtower,
    PqCommittee,
}

impl SignoffRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::Reviewer => "reviewer",
            Self::Watchtower => "watchtower",
            Self::PqCommittee => "pq_committee",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffDecision {
    Approved,
    Abstained,
    Rejected,
    Revoked,
}

impl SignoffDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Abstained => "abstained",
            Self::Rejected => "rejected",
            Self::Revoked => "revoked",
        }
    }

    pub fn permits(self) -> bool {
        matches!(self, Self::Approved)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardSeverity {
    Info,
    Warning,
    Blocking,
}

impl GuardSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }

    pub fn blocks(self) -> bool {
        matches!(self, Self::Blocking)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub monero_network: String,
    pub l2_network: String,
    pub asset_id: String,
    pub fee_asset_id: String,
    pub min_operator_signoffs: u16,
    pub min_reviewer_signoffs: u16,
    pub min_pq_security_bits: u16,
    pub max_root_age_blocks: u64,
    pub heavy_gate_ttl_blocks: u64,
    pub max_note_delay_blocks: u64,
    pub min_heavy_gate_score_bps: u64,
    pub allow_activation_after_compile: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            monero_network: DEVNET_MONERO_NETWORK.to_string(),
            l2_network: DEVNET_L2_NETWORK.to_string(),
            asset_id: DEVNET_ASSET_ID.to_string(),
            fee_asset_id: DEVNET_FEE_ASSET_ID.to_string(),
            min_operator_signoffs: DEFAULT_MIN_OPERATOR_SIGNOFFS,
            min_reviewer_signoffs: DEFAULT_MIN_REVIEWER_SIGNOFFS,
            min_pq_security_bits: DEFAULT_MIN_PQ_SECURITY_BITS,
            max_root_age_blocks: DEFAULT_MAX_ROOT_AGE_BLOCKS,
            heavy_gate_ttl_blocks: DEFAULT_HEAVY_GATE_TTL_BLOCKS,
            max_note_delay_blocks: DEFAULT_MAX_NOTE_DELAY_BLOCKS,
            min_heavy_gate_score_bps: DEFAULT_MIN_HEAVY_GATE_SCORE_BPS,
            allow_activation_after_compile: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_text("chain_id", &self.chain_id)?;
        require_text("monero_network", &self.monero_network)?;
        require_text("l2_network", &self.l2_network)?;
        require_text("asset_id", &self.asset_id)?;
        require_text("fee_asset_id", &self.fee_asset_id)?;
        if self.min_operator_signoffs == 0 {
            return Err("min_operator_signoffs must be nonzero".to_string());
        }
        if self.min_reviewer_signoffs == 0 {
            return Err("min_reviewer_signoffs must be nonzero".to_string());
        }
        if self.min_pq_security_bits < DEFAULT_MIN_PQ_SECURITY_BITS {
            return Err("min_pq_security_bits below pq floor".to_string());
        }
        if self.max_root_age_blocks == 0 {
            return Err("max_root_age_blocks must be nonzero".to_string());
        }
        if self.heavy_gate_ttl_blocks == 0 {
            return Err("heavy_gate_ttl_blocks must be nonzero".to_string());
        }
        if self.min_heavy_gate_score_bps > DEFAULT_MAX_BPS {
            return Err("min_heavy_gate_score_bps above bps scale".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "monero_network": self.monero_network,
            "l2_network": self.l2_network,
            "asset_id": self.asset_id,
            "fee_asset_id": self.fee_asset_id,
            "min_operator_signoffs": self.min_operator_signoffs,
            "min_reviewer_signoffs": self.min_reviewer_signoffs,
            "min_pq_security_bits": self.min_pq_security_bits,
            "max_root_age_blocks": self.max_root_age_blocks,
            "heavy_gate_ttl_blocks": self.heavy_gate_ttl_blocks,
            "max_note_delay_blocks": self.max_note_delay_blocks,
            "min_heavy_gate_score_bps": self.min_heavy_gate_score_bps,
            "allow_activation_after_compile": self.allow_activation_after_compile,
        })
    }

    pub fn config_root(&self) -> String {
        hash_json("wave106-config", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootGate {
    pub kind: RootGateKind,
    pub root: String,
    pub source_height: u64,
    pub observed_height: u64,
    pub source_epoch: u64,
    pub cleared: bool,
    pub reviewer_label: String,
    pub evidence_root: String,
}

impl RootGate {
    pub fn new(
        kind: RootGateKind,
        label: &str,
        source_height: u64,
        observed_height: u64,
        cleared: bool,
    ) -> Self {
        let seed = json!({
            "kind": kind.as_str(),
            "label": label,
            "source_height": source_height,
            "observed_height": observed_height,
        });
        Self {
            kind,
            root: hash_json("wave106-root-gate-root", &seed),
            source_height,
            observed_height,
            source_epoch: DEVNET_EPOCH,
            cleared,
            reviewer_label: label.to_string(),
            evidence_root: hash_json("wave106-root-gate-evidence", &seed),
        }
    }

    pub fn fail_closed(kind: RootGateKind, label: &str) -> Self {
        let seed = json!({
            "kind": kind.as_str(),
            "label": label,
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
        });
        Self {
            kind,
            root: hash_json("wave106-root-gate-fail-closed", &seed),
            source_height: 0,
            observed_height: 0,
            source_epoch: DEVNET_EPOCH,
            cleared: false,
            reviewer_label: label.to_string(),
            evidence_root: hash_json("wave106-root-gate-fail-closed-evidence", &seed),
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        require_text("root", &self.root)?;
        require_text("reviewer_label", &self.reviewer_label)?;
        require_text("evidence_root", &self.evidence_root)?;
        if self.source_height == 0 || self.observed_height == 0 {
            return Err(format!("{} root height missing", self.kind.as_str()));
        }
        if self.observed_height < self.source_height {
            return Err(format!("{} observed before source", self.kind.as_str()));
        }
        if height.saturating_sub(self.observed_height) > config.max_root_age_blocks {
            return Err(format!("{} root stale", self.kind.as_str()));
        }
        if !self.cleared {
            return Err(format!("{} root not clear", self.kind.as_str()));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "root": self.root,
            "source_height": self.source_height,
            "observed_height": self.observed_height,
            "source_epoch": self.source_epoch,
            "cleared": self.cleared,
            "reviewer_label": self.reviewer_label,
            "evidence_root": self.evidence_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeavyGateEvidence {
    pub kind: HeavyGateKind,
    pub run_id: String,
    pub started_height: u64,
    pub completed_height: u64,
    pub score_bps: u64,
    pub transcript_root: String,
    pub artifact_root: String,
    pub passed: bool,
}

impl HeavyGateEvidence {
    pub fn new(
        kind: HeavyGateKind,
        run_id: &str,
        height: u64,
        score_bps: u64,
        passed: bool,
    ) -> Self {
        let seed = json!({
            "kind": kind.as_str(),
            "run_id": run_id,
            "height": height,
            "score_bps": score_bps,
            "passed": passed,
        });
        Self {
            kind,
            run_id: run_id.to_string(),
            started_height: height.saturating_sub(1),
            completed_height: height,
            score_bps,
            transcript_root: hash_json("wave106-heavy-gate-transcript", &seed),
            artifact_root: hash_json("wave106-heavy-gate-artifact", &seed),
            passed,
        }
    }

    pub fn fail_closed(kind: HeavyGateKind) -> Self {
        let seed = json!({
            "kind": kind.as_str(),
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
        });
        Self {
            kind,
            run_id: format!("{}-not-run", kind.as_str()),
            started_height: 0,
            completed_height: 0,
            score_bps: 0,
            transcript_root: hash_json("wave106-heavy-gate-not-run", &seed),
            artifact_root: hash_json("wave106-heavy-gate-not-run-artifact", &seed),
            passed: false,
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        require_text("run_id", &self.run_id)?;
        require_text("transcript_root", &self.transcript_root)?;
        require_text("artifact_root", &self.artifact_root)?;
        if self.started_height == 0 || self.completed_height == 0 {
            return Err(format!("{} heavy gate has no live run", self.kind.as_str()));
        }
        if self.completed_height < self.started_height {
            return Err(format!(
                "{} heavy gate height order invalid",
                self.kind.as_str()
            ));
        }
        if height.saturating_sub(self.completed_height) > config.heavy_gate_ttl_blocks {
            return Err(format!("{} heavy gate evidence stale", self.kind.as_str()));
        }
        if self.score_bps < config.min_heavy_gate_score_bps {
            return Err(format!(
                "{} heavy gate score below threshold",
                self.kind.as_str()
            ));
        }
        if !self.passed {
            return Err(format!("{} heavy gate did not pass", self.kind.as_str()));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "run_id": self.run_id,
            "started_height": self.started_height,
            "completed_height": self.completed_height,
            "score_bps": self.score_bps,
            "transcript_root": self.transcript_root,
            "artifact_root": self.artifact_root,
            "passed": self.passed,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signoff {
    pub signer_id: String,
    pub role: SignoffRole,
    pub decision: SignoffDecision,
    pub signed_height: u64,
    pub pq_security_bits: u16,
    pub message_root: String,
    pub signature_root: String,
}

impl Signoff {
    pub fn approved(role: SignoffRole, signer_id: &str, height: u64) -> Self {
        let seed = json!({
            "role": role.as_str(),
            "signer_id": signer_id,
            "height": height,
            "decision": SignoffDecision::Approved.as_str(),
        });
        Self {
            signer_id: signer_id.to_string(),
            role,
            decision: SignoffDecision::Approved,
            signed_height: height,
            pq_security_bits: DEFAULT_MIN_PQ_SECURITY_BITS,
            message_root: hash_json("wave106-signoff-message", &seed),
            signature_root: hash_json("wave106-signoff-signature", &seed),
        }
    }

    pub fn fail_closed(role: SignoffRole, signer_id: &str) -> Self {
        let seed = json!({
            "role": role.as_str(),
            "signer_id": signer_id,
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
        });
        Self {
            signer_id: signer_id.to_string(),
            role,
            decision: SignoffDecision::Abstained,
            signed_height: 0,
            pq_security_bits: 0,
            message_root: hash_json("wave106-signoff-fail-closed-message", &seed),
            signature_root: hash_json("wave106-signoff-fail-closed-signature", &seed),
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        require_text("signer_id", &self.signer_id)?;
        require_text("message_root", &self.message_root)?;
        require_text("signature_root", &self.signature_root)?;
        if !self.decision.permits() {
            return Err(format!("{} signoff not approved", self.signer_id));
        }
        if self.signed_height == 0 || self.signed_height > height {
            return Err(format!("{} signoff height invalid", self.signer_id));
        }
        if height.saturating_sub(self.signed_height) > config.heavy_gate_ttl_blocks {
            return Err(format!("{} signoff stale", self.signer_id));
        }
        if self.pq_security_bits < config.min_pq_security_bits {
            return Err(format!(
                "{} signoff pq security below floor",
                self.signer_id
            ));
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "signer_id": self.signer_id,
            "role": self.role.as_str(),
            "decision": self.decision.as_str(),
            "signed_height": self.signed_height,
            "pq_security_bits": self.pq_security_bits,
            "message_root": self.message_root,
            "signature_root": self.signature_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCreditAccount {
    pub account_id: String,
    pub credit_root: String,
    pub amount_bucket: String,
    pub beneficiary_wallet_root: String,
    pub liability_root: String,
    pub fee_rebate_root: String,
    pub accounted_piconero: u128,
    pub spendable_piconero: u128,
}

impl ReleaseCreditAccount {
    pub fn devnet(account_id: &str, amount_bucket: &str, amount: u128) -> Self {
        let seed = json!({
            "account_id": account_id,
            "amount_bucket": amount_bucket,
            "accounted_piconero": amount.to_string(),
        });
        Self {
            account_id: account_id.to_string(),
            credit_root: hash_json("wave106-credit-root", &seed),
            amount_bucket: amount_bucket.to_string(),
            beneficiary_wallet_root: hash_json("wave106-beneficiary-wallet-root", &seed),
            liability_root: hash_json("wave106-liability-root", &seed),
            fee_rebate_root: hash_json("wave106-fee-rebate-root", &seed),
            accounted_piconero: amount,
            spendable_piconero: 0,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_text("account_id", &self.account_id)?;
        require_text("credit_root", &self.credit_root)?;
        require_text("amount_bucket", &self.amount_bucket)?;
        require_text("beneficiary_wallet_root", &self.beneficiary_wallet_root)?;
        require_text("liability_root", &self.liability_root)?;
        require_text("fee_rebate_root", &self.fee_rebate_root)?;
        if self.accounted_piconero == 0 {
            return Err("accounted_piconero must be nonzero".to_string());
        }
        if self.spendable_piconero != 0 {
            return Err("release credit already spendable".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "account_id": self.account_id,
            "credit_root": self.credit_root,
            "amount_bucket": self.amount_bucket,
            "beneficiary_wallet_root": self.beneficiary_wallet_root,
            "liability_root": self.liability_root,
            "fee_rebate_root": self.fee_rebate_root,
            "accounted_piconero": self.accounted_piconero.to_string(),
            "spendable_piconero": self.spendable_piconero.to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateNoteActivationPlan {
    pub plan_id: String,
    pub note_commitment_root: String,
    pub nullifier_reservation_root: String,
    pub encrypted_note_hint_root: String,
    pub activation_height: u64,
    pub reserved_until_height: u64,
    pub activation_allowed: bool,
}

impl PrivateNoteActivationPlan {
    pub fn devnet(plan_id: &str, height: u64) -> Self {
        let seed = json!({
            "plan_id": plan_id,
            "height": height,
            "release_credit_allowed": false,
            "note_activation_allowed": false,
        });
        Self {
            plan_id: plan_id.to_string(),
            note_commitment_root: hash_json("wave106-note-commitment-root", &seed),
            nullifier_reservation_root: hash_json("wave106-nullifier-reservation-root", &seed),
            encrypted_note_hint_root: hash_json("wave106-encrypted-note-hint-root", &seed),
            activation_height: height,
            reserved_until_height: height.saturating_add(DEFAULT_MAX_NOTE_DELAY_BLOCKS),
            activation_allowed: false,
        }
    }

    pub fn validate(&self, config: &Config, height: u64) -> Result<()> {
        require_text("plan_id", &self.plan_id)?;
        require_text("note_commitment_root", &self.note_commitment_root)?;
        require_text(
            "nullifier_reservation_root",
            &self.nullifier_reservation_root,
        )?;
        require_text("encrypted_note_hint_root", &self.encrypted_note_hint_root)?;
        if self.activation_height == 0 || self.activation_height > height {
            return Err("activation height invalid".to_string());
        }
        if self.reserved_until_height < self.activation_height {
            return Err("nullifier reservation ends before activation".to_string());
        }
        if self
            .reserved_until_height
            .saturating_sub(self.activation_height)
            > config.max_note_delay_blocks
        {
            return Err("note activation delay too wide".to_string());
        }
        if self.activation_allowed {
            return Err("note activation opened before compile lane release".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "plan_id": self.plan_id,
            "note_commitment_root": self.note_commitment_root,
            "nullifier_reservation_root": self.nullifier_reservation_root,
            "encrypted_note_hint_root": self.encrypted_note_hint_root,
            "activation_height": self.activation_height,
            "reserved_until_height": self.reserved_until_height,
            "activation_allowed": self.activation_allowed,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardFinding {
    pub finding_id: String,
    pub severity: GuardSeverity,
    pub lane: String,
    pub detail: String,
    pub evidence_root: String,
    pub resolved: bool,
}

impl GuardFinding {
    pub fn blocking(lane: &str, detail: &str) -> Self {
        let seed = json!({
            "lane": lane,
            "detail": detail,
            "severity": GuardSeverity::Blocking.as_str(),
        });
        Self {
            finding_id: hash_json("wave106-finding-id", &seed),
            severity: GuardSeverity::Blocking,
            lane: lane.to_string(),
            detail: detail.to_string(),
            evidence_root: hash_json("wave106-finding-evidence", &seed),
            resolved: false,
        }
    }

    pub fn info(lane: &str, detail: &str) -> Self {
        let seed = json!({
            "lane": lane,
            "detail": detail,
            "severity": GuardSeverity::Info.as_str(),
        });
        Self {
            finding_id: hash_json("wave106-finding-id", &seed),
            severity: GuardSeverity::Info,
            lane: lane.to_string(),
            detail: detail.to_string(),
            evidence_root: hash_json("wave106-finding-evidence", &seed),
            resolved: true,
        }
    }

    pub fn blocks(&self) -> bool {
        self.severity.blocks() && !self.resolved
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "finding_id": self.finding_id,
            "severity": self.severity.as_str(),
            "lane": self.lane,
            "detail": self.detail,
            "evidence_root": self.evidence_root,
            "resolved": self.resolved,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardCounters {
    pub root_gates_total: u64,
    pub root_gates_cleared: u64,
    pub heavy_gates_total: u64,
    pub heavy_gates_passed: u64,
    pub operator_signoffs: u64,
    pub reviewer_signoffs: u64,
    pub blocking_findings: u64,
}

impl GuardCounters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "root_gates_total": self.root_gates_total,
            "root_gates_cleared": self.root_gates_cleared,
            "heavy_gates_total": self.heavy_gates_total,
            "heavy_gates_passed": self.heavy_gates_passed,
            "operator_signoffs": self.operator_signoffs,
            "reviewer_signoffs": self.reviewer_signoffs,
            "blocking_findings": self.blocking_findings,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDecision {
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub reason: String,
    pub decision_root: String,
}

impl ReleaseDecision {
    pub fn fail_closed(reason: &str) -> Self {
        let record = json!({
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
            "reason": reason,
        });
        Self {
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            reason: reason.to_string(),
            decision_root: hash_json("wave106-release-decision-fail-closed", &record),
        }
    }

    pub fn allowed(reason: &str, config: &Config) -> Self {
        let can_activate = config.allow_activation_after_compile;
        let record = json!({
            "release_credit_allowed": true,
            "note_activation_allowed": can_activate,
            "heavy_gates_ran": true,
            "reason": reason,
        });
        Self {
            release_credit_allowed: true,
            note_activation_allowed: can_activate,
            heavy_gates_ran: true,
            reason: reason.to_string(),
            decision_root: hash_json("wave106-release-decision-allowed", &record),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "reason": self.reason,
            "decision_root": self.decision_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub height: u64,
    pub lane_id: String,
    pub root_gates: Vec<RootGate>,
    pub heavy_gate_evidence: Vec<HeavyGateEvidence>,
    pub signoffs: Vec<Signoff>,
    pub release_accounts: Vec<ReleaseCreditAccount>,
    pub note_plans: Vec<PrivateNoteActivationPlan>,
    pub findings: Vec<GuardFinding>,
    pub counters: GuardCounters,
    pub decision: ReleaseDecision,
    pub roots: BTreeMap<String, String>,
}

impl State {
    pub fn new(config: Config, height: u64, lane_id: &str) -> Result<Self> {
        config.validate()?;
        require_text("lane_id", lane_id)?;
        let root_gates = RootGateKind::all()
            .iter()
            .map(|kind| RootGate::fail_closed(*kind, "fail-closed-root-not-cleared"))
            .collect::<Vec<_>>();
        let heavy_gate_evidence = HeavyGateKind::all()
            .iter()
            .map(|kind| HeavyGateEvidence::fail_closed(*kind))
            .collect::<Vec<_>>();
        let signoffs = vec![
            Signoff::fail_closed(SignoffRole::Operator, "operator-signoff-missing"),
            Signoff::fail_closed(SignoffRole::Reviewer, "reviewer-signoff-missing"),
        ];
        let release_accounts = vec![ReleaseCreditAccount::devnet(
            "devnet-beneficiary-account-0",
            "bucket-p2-16",
            1_000_000_000_000,
        )];
        let note_plans = vec![PrivateNoteActivationPlan::devnet(
            "devnet-private-note-activation-plan-0",
            height,
        )];
        let findings = vec![GuardFinding::blocking(
            "compile_lane",
            "fail closed until wave105 roots, note roots, nullifier roots, heavy gates, and signoffs clear",
        )];
        let counters = GuardCounters {
            root_gates_total: root_gates.len() as u64,
            root_gates_cleared: 0,
            heavy_gates_total: heavy_gate_evidence.len() as u64,
            heavy_gates_passed: 0,
            operator_signoffs: 0,
            reviewer_signoffs: 0,
            blocking_findings: 1,
        };
        let decision = ReleaseDecision::fail_closed("compile lane initialized fail closed");
        let mut state = Self {
            config,
            height,
            lane_id: lane_id.to_string(),
            root_gates,
            heavy_gate_evidence,
            signoffs,
            release_accounts,
            note_plans,
            findings,
            counters,
            decision,
            roots: BTreeMap::new(),
        };
        state.recompute_roots();
        Ok(state)
    }

    pub fn with_live_devnet_evidence(mut self) -> Self {
        self.root_gates = RootGateKind::all()
            .iter()
            .map(|kind| {
                RootGate::new(
                    *kind,
                    "wave106-live-heavy-gate-reviewer",
                    self.height.saturating_sub(2),
                    self.height.saturating_sub(1),
                    true,
                )
            })
            .collect::<Vec<_>>();
        self.heavy_gate_evidence = HeavyGateKind::all()
            .iter()
            .map(|kind| {
                HeavyGateEvidence::new(
                    *kind,
                    &format!("wave106-{}-run", kind.as_str()),
                    self.height,
                    DEFAULT_MAX_BPS,
                    true,
                )
            })
            .collect::<Vec<_>>();
        self.signoffs = vec![
            Signoff::approved(SignoffRole::Operator, "operator-alpha", self.height),
            Signoff::approved(SignoffRole::Operator, "operator-beta", self.height),
            Signoff::approved(SignoffRole::Operator, "operator-gamma", self.height),
            Signoff::approved(SignoffRole::Reviewer, "reviewer-one", self.height),
            Signoff::approved(SignoffRole::Reviewer, "reviewer-two", self.height),
            Signoff::approved(SignoffRole::Watchtower, "watchtower-quorum", self.height),
            Signoff::approved(SignoffRole::PqCommittee, "pq-committee-quorum", self.height),
        ];
        self.findings = vec![GuardFinding::info(
            "compile_lane",
            "live heavy-gate evidence and operator reviewer signoffs are present",
        )];
        self.recompute_decision();
        self.recompute_roots();
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        require_text("lane_id", &self.lane_id)?;
        if self.height == 0 {
            return Err("height must be nonzero".to_string());
        }
        self.validate_required_root_gates()?;
        self.validate_required_heavy_gates()?;
        self.validate_accounts_and_notes()?;
        self.validate_signoffs()?;
        let blockers = self
            .findings
            .iter()
            .filter(|finding| finding.blocks())
            .count();
        if blockers > 0 {
            return Err("blocking findings remain open".to_string());
        }
        Ok(())
    }

    pub fn recompute_decision(&mut self) {
        self.counters = self.compute_counters();
        let decision = match self.validate() {
            Ok(()) => ReleaseDecision::allowed("all compile lane gates cleared", &self.config),
            Err(reason) => ReleaseDecision::fail_closed(&reason),
        };
        self.decision = decision;
    }

    pub fn recompute_roots(&mut self) {
        let mut roots = BTreeMap::new();
        roots.insert("config_root".to_string(), self.config.config_root());
        roots.insert(
            "root_gate_root".to_string(),
            merkle_json("wave106-root-gates", &self.root_gate_records()),
        );
        roots.insert(
            "heavy_gate_root".to_string(),
            merkle_json("wave106-heavy-gates", &self.heavy_gate_records()),
        );
        roots.insert(
            "signoff_root".to_string(),
            merkle_json("wave106-signoffs", &self.signoff_records()),
        );
        roots.insert(
            "release_credit_root".to_string(),
            merkle_json("wave106-release-credits", &self.release_account_records()),
        );
        roots.insert(
            "private_note_plan_root".to_string(),
            merkle_json("wave106-private-note-plans", &self.note_plan_records()),
        );
        roots.insert(
            "finding_root".to_string(),
            merkle_json("wave106-findings", &self.finding_records()),
        );
        roots.insert(
            "counter_root".to_string(),
            hash_json("wave106-counters", &self.counters.public_record()),
        );
        roots.insert(
            "decision_root".to_string(),
            self.decision.decision_root.clone(),
        );
        let aggregate_record = json!({
            "protocol_version": PROTOCOL_VERSION,
            "lane_id": self.lane_id,
            "height": self.height,
            "roots": roots.clone(),
            "release_credit_allowed": self.decision.release_credit_allowed,
            "note_activation_allowed": self.decision.note_activation_allowed,
            "heavy_gates_ran": self.decision.heavy_gates_ran,
        });
        let aggregate_root = hash_json("wave106-state-root", &aggregate_record);
        roots.insert("state_root".to_string(), aggregate_root);
        self.roots = roots;
    }

    pub fn state_root(&self) -> String {
        match self.roots.get("state_root") {
            Some(root) => root.clone(),
            None => hash_json(
                "wave106-state-root-missing",
                &self.public_record_without_roots(),
            ),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "schema": PUBLIC_RECORD_SCHEMA,
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": self.config.chain_id,
            "height": self.height,
            "lane_id": self.lane_id,
            "release_credit_allowed": self.decision.release_credit_allowed,
            "note_activation_allowed": self.decision.note_activation_allowed,
            "heavy_gates_ran": self.decision.heavy_gates_ran,
            "config": self.config.public_record(),
            "root_gates": self.root_gate_records(),
            "heavy_gate_evidence": self.heavy_gate_records(),
            "signoffs": self.signoff_records(),
            "release_accounts": self.release_account_records(),
            "note_plans": self.note_plan_records(),
            "findings": self.finding_records(),
            "counters": self.counters.public_record(),
            "decision": self.decision.public_record(),
            "roots": self.roots,
            "state_root": self.state_root(),
        })
    }

    fn public_record_without_roots(&self) -> PublicRecord {
        json!({
            "schema": PUBLIC_RECORD_SCHEMA,
            "protocol_version": PROTOCOL_VERSION,
            "height": self.height,
            "lane_id": self.lane_id,
            "release_credit_allowed": self.decision.release_credit_allowed,
            "note_activation_allowed": self.decision.note_activation_allowed,
            "heavy_gates_ran": self.decision.heavy_gates_ran,
            "config": self.config.public_record(),
            "root_gates": self.root_gate_records(),
            "heavy_gate_evidence": self.heavy_gate_records(),
            "signoffs": self.signoff_records(),
            "release_accounts": self.release_account_records(),
            "note_plans": self.note_plan_records(),
            "findings": self.finding_records(),
            "counters": self.counters.public_record(),
            "decision": self.decision.public_record(),
        })
    }

    fn validate_required_root_gates(&self) -> Result<()> {
        let mut seen = BTreeSet::new();
        for gate in &self.root_gates {
            gate.validate(&self.config, self.height)?;
            seen.insert(gate.kind);
        }
        for kind in RootGateKind::all() {
            if !seen.contains(kind) {
                return Err(format!("{} root gate missing", kind.as_str()));
            }
        }
        Ok(())
    }

    fn validate_required_heavy_gates(&self) -> Result<()> {
        let mut seen = BTreeSet::new();
        for gate in &self.heavy_gate_evidence {
            gate.validate(&self.config, self.height)?;
            seen.insert(gate.kind);
        }
        for kind in HeavyGateKind::all() {
            if !seen.contains(kind) {
                return Err(format!("{} heavy gate missing", kind.as_str()));
            }
        }
        Ok(())
    }

    fn validate_accounts_and_notes(&self) -> Result<()> {
        if self.release_accounts.is_empty() {
            return Err("release account set empty".to_string());
        }
        if self.note_plans.is_empty() {
            return Err("private note activation plan set empty".to_string());
        }
        for account in &self.release_accounts {
            account.validate()?;
        }
        for plan in &self.note_plans {
            plan.validate(&self.config, self.height)?;
        }
        Ok(())
    }

    fn validate_signoffs(&self) -> Result<()> {
        let mut operator_ids = BTreeSet::new();
        let mut reviewer_ids = BTreeSet::new();
        for signoff in &self.signoffs {
            signoff.validate(&self.config, self.height)?;
            match signoff.role {
                SignoffRole::Operator => {
                    operator_ids.insert(signoff.signer_id.clone());
                }
                SignoffRole::Reviewer => {
                    reviewer_ids.insert(signoff.signer_id.clone());
                }
                SignoffRole::Watchtower | SignoffRole::PqCommittee => {}
            }
        }
        if operator_ids.len() < usize::from(self.config.min_operator_signoffs) {
            return Err("operator signoff threshold not met".to_string());
        }
        if reviewer_ids.len() < usize::from(self.config.min_reviewer_signoffs) {
            return Err("reviewer signoff threshold not met".to_string());
        }
        Ok(())
    }

    fn compute_counters(&self) -> GuardCounters {
        GuardCounters {
            root_gates_total: self.root_gates.len() as u64,
            root_gates_cleared: self.root_gates.iter().filter(|gate| gate.cleared).count() as u64,
            heavy_gates_total: self.heavy_gate_evidence.len() as u64,
            heavy_gates_passed: self
                .heavy_gate_evidence
                .iter()
                .filter(|gate| gate.passed)
                .count() as u64,
            operator_signoffs: self
                .signoffs
                .iter()
                .filter(|signoff| {
                    signoff.role == SignoffRole::Operator && signoff.decision.permits()
                })
                .count() as u64,
            reviewer_signoffs: self
                .signoffs
                .iter()
                .filter(|signoff| {
                    signoff.role == SignoffRole::Reviewer && signoff.decision.permits()
                })
                .count() as u64,
            blocking_findings: self
                .findings
                .iter()
                .filter(|finding| finding.blocks())
                .count() as u64,
        }
    }

    fn root_gate_records(&self) -> Vec<Value> {
        self.root_gates
            .iter()
            .map(RootGate::public_record)
            .collect::<Vec<_>>()
    }

    fn heavy_gate_records(&self) -> Vec<Value> {
        self.heavy_gate_evidence
            .iter()
            .map(HeavyGateEvidence::public_record)
            .collect::<Vec<_>>()
    }

    fn signoff_records(&self) -> Vec<Value> {
        self.signoffs
            .iter()
            .map(Signoff::public_record)
            .collect::<Vec<_>>()
    }

    fn release_account_records(&self) -> Vec<Value> {
        self.release_accounts
            .iter()
            .map(ReleaseCreditAccount::public_record)
            .collect::<Vec<_>>()
    }

    fn note_plan_records(&self) -> Vec<Value> {
        self.note_plans
            .iter()
            .map(PrivateNoteActivationPlan::public_record)
            .collect::<Vec<_>>()
    }

    fn finding_records(&self) -> Vec<Value> {
        self.findings
            .iter()
            .map(GuardFinding::public_record)
            .collect::<Vec<_>>()
    }
}

pub fn devnet() -> Result<State> {
    let config = Config::devnet();
    let mut state = State::new(
        config,
        DEVNET_HEIGHT,
        "wave106-monero-release-credit-private-note-compile",
    )?;
    state.recompute_decision();
    state.recompute_roots();
    Ok(state)
}

pub fn public_record() -> PublicRecord {
    match devnet() {
        Ok(state) => state.public_record(),
        Err(reason) => {
            let decision = ReleaseDecision::fail_closed(&reason);
            json!({
                "schema": PUBLIC_RECORD_SCHEMA,
                "protocol_version": PROTOCOL_VERSION,
                "release_credit_allowed": false,
                "note_activation_allowed": false,
                "heavy_gates_ran": false,
                "decision": decision.public_record(),
                "state_root": hash_json("wave106-public-record-fail-closed", &decision.public_record()),
            })
        }
    }
}

pub fn state_root() -> String {
    match devnet() {
        Ok(state) => state.state_root(),
        Err(reason) => hash_json(
            "wave106-state-root-fail-closed",
            &json!({
                "release_credit_allowed": false,
                "note_activation_allowed": false,
                "heavy_gates_ran": false,
                "reason": reason,
            }),
        ),
    }
}

pub fn live_devnet_candidate() -> Result<State> {
    let config = Config {
        allow_activation_after_compile: false,
        ..Config::devnet()
    };
    let mut state = State::new(
        config,
        DEVNET_HEIGHT,
        "wave106-monero-release-credit-private-note-compile",
    )?
    .with_live_devnet_evidence();
    state.recompute_decision();
    state.recompute_roots();
    Ok(state)
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{field} must be set"));
    }
    Ok(())
}

fn merkle_json(domain: &str, records: &[Value]) -> String {
    if records.is_empty() {
        return hash_json(domain, &json!({ "empty": EMPTY_ROOT_LABEL }));
    }
    merkle_root(domain, records)
}

fn hash_json(domain: &str, value: &Value) -> String {
    domain_hash(domain, &[HashPart::Json(value)], 32)
}
