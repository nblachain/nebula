use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave97-live-heavy-gate-release-claim-anti-equivocation-seal-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const SEAL_SUITE: &str =
    "monero-l2-wave97-wave96-release-readiness-claim-anti-equivocation-seal-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 97;
pub const RELEASE_READINESS_WAVE: u64 = 96;
pub const PROMOTION_WAVE: u64 = 95;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 97;
pub const DEFAULT_MIN_PQ_SIGNERS: u64 = 5;
pub const DEFAULT_MIN_PQ_FAMILIES: u64 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_500;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS: u64 = 8_000;
pub const DEFAULT_MIN_REDACTION_DEPTH: u64 = 4;
pub const DEFAULT_MAX_RELEASE_CLAIMS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave97-live-heavy-gate-release-claim-anti-equivocation-seal-pq-reserve-privacy-lane-runtime";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimKind {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqQuorum,
    ReserveProof,
    ReserveCoverage,
    PrivacyBudget,
    PrivacyLinkage,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl ClaimKind {
    pub fn all() -> [Self; 10] {
        [
            Self::MlDsaAuthorityEpoch,
            Self::SlhDsaAuthorityEpoch,
            Self::PqQuorum,
            Self::ReserveProof,
            Self::ReserveCoverage,
            Self::PrivacyBudget,
            Self::PrivacyLinkage,
            Self::MetadataRedaction,
            Self::NullifierSeparation,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MlDsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqQuorum => "pq_quorum",
            Self::ReserveProof => "reserve_proof",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyBudget => "privacy_budget",
            Self::PrivacyLinkage => "privacy_linkage",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealStatus {
    Blocked,
    SealedShadow,
    Rejected,
}

impl SealStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::SealedShadow => "sealed_shadow",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealBlocker {
    NoReleaseClaim,
    ProductionDenied,
    HeavyGateNotRun,
    Wave96ReadinessRootMissing,
    Wave95PromotionRootMissing,
    ReleaseClaimRootMissing,
    ReserveProofRootMissing,
    ReserveCoverageRootMissing,
    PrivacyBudgetRootMissing,
    PrivacyLinkageRootMissing,
    MetadataRedactionRootMissing,
    NullifierSeparationRootMissing,
    PqAuthorityRootMissing,
    PqQuorumRootMissing,
    OperatorSignoffRootMissing,
    AntiEquivocationRootMissing,
    ReplayGuardRootMissing,
    ForkGuardRootMissing,
    DuplicateGuardRootMissing,
    SealCommitmentRootMissing,
    SealReceiptRootMissing,
    AuthorityEpochTooLow,
    PqQuorumTooSmall,
    PqFamilyCoverageMissing,
    ReserveCoverageMissing,
    ReserveBufferMissing,
    PrivacyBudgetTooLow,
    PrivacyLinkageTooHigh,
    MetadataRedactionTooShallow,
    NullifierSeparationMissing,
    AntiEquivocationViolation,
    ReplayObserved,
    ForkObserved,
    DuplicateObserved,
    RootsOnlyBoundary,
    DuplicateSealRoot,
    ClaimCapacityReached,
}

impl SealBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoReleaseClaim => "no_release_claim",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::Wave96ReadinessRootMissing => "wave96_readiness_root_missing",
            Self::Wave95PromotionRootMissing => "wave95_promotion_root_missing",
            Self::ReleaseClaimRootMissing => "release_claim_root_missing",
            Self::ReserveProofRootMissing => "reserve_proof_root_missing",
            Self::ReserveCoverageRootMissing => "reserve_coverage_root_missing",
            Self::PrivacyBudgetRootMissing => "privacy_budget_root_missing",
            Self::PrivacyLinkageRootMissing => "privacy_linkage_root_missing",
            Self::MetadataRedactionRootMissing => "metadata_redaction_root_missing",
            Self::NullifierSeparationRootMissing => "nullifier_separation_root_missing",
            Self::PqAuthorityRootMissing => "pq_authority_root_missing",
            Self::PqQuorumRootMissing => "pq_quorum_root_missing",
            Self::OperatorSignoffRootMissing => "operator_signoff_root_missing",
            Self::AntiEquivocationRootMissing => "anti_equivocation_root_missing",
            Self::ReplayGuardRootMissing => "replay_guard_root_missing",
            Self::ForkGuardRootMissing => "fork_guard_root_missing",
            Self::DuplicateGuardRootMissing => "duplicate_guard_root_missing",
            Self::SealCommitmentRootMissing => "seal_commitment_root_missing",
            Self::SealReceiptRootMissing => "seal_receipt_root_missing",
            Self::AuthorityEpochTooLow => "authority_epoch_too_low",
            Self::PqQuorumTooSmall => "pq_quorum_too_small",
            Self::PqFamilyCoverageMissing => "pq_family_coverage_missing",
            Self::ReserveCoverageMissing => "reserve_coverage_missing",
            Self::ReserveBufferMissing => "reserve_buffer_missing",
            Self::PrivacyBudgetTooLow => "privacy_budget_too_low",
            Self::PrivacyLinkageTooHigh => "privacy_linkage_too_high",
            Self::MetadataRedactionTooShallow => "metadata_redaction_too_shallow",
            Self::NullifierSeparationMissing => "nullifier_separation_missing",
            Self::AntiEquivocationViolation => "anti_equivocation_violation",
            Self::ReplayObserved => "replay_observed",
            Self::ForkObserved => "fork_observed",
            Self::DuplicateObserved => "duplicate_observed",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DuplicateSealRoot => "duplicate_seal_root",
            Self::ClaimCapacityReached => "claim_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    Blocked,
    SealedShadow,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Blocked => "blocked",
            Self::SealedShadow => "sealed_shadow",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave96ReadinessRoot,
    ImportWave95PromotionRoot,
    AttachReleaseClaimRoot,
    AttachReserveProofRoot,
    AttachReserveCoverageRoot,
    AttachPrivacyBudgetRoot,
    AttachPrivacyLinkageRoot,
    AttachMetadataRedactionRoot,
    AttachNullifierSeparationRoot,
    AttachPqAuthorityRoot,
    AttachPqQuorumRoot,
    AttachAntiEquivocationRoot,
    AttachReplayForkDuplicateGuardRoots,
    AttachSealCommitmentRoot,
    AttachSealReceiptRoot,
    KeepReleaseDenied,
    PublishRootsOnlySealRecord,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave96ReadinessRoot,
            Self::ImportWave95PromotionRoot,
            Self::AttachReleaseClaimRoot,
            Self::AttachReserveProofRoot,
            Self::AttachReserveCoverageRoot,
            Self::AttachPrivacyBudgetRoot,
            Self::AttachPrivacyLinkageRoot,
            Self::AttachMetadataRedactionRoot,
            Self::AttachNullifierSeparationRoot,
            Self::AttachPqAuthorityRoot,
            Self::AttachPqQuorumRoot,
            Self::AttachAntiEquivocationRoot,
            Self::AttachReplayForkDuplicateGuardRoots,
            Self::AttachSealCommitmentRoot,
            Self::AttachSealReceiptRoot,
            Self::KeepReleaseDenied,
            Self::PublishRootsOnlySealRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave96ReadinessRoot => "import_wave96_readiness_root",
            Self::ImportWave95PromotionRoot => "import_wave95_promotion_root",
            Self::AttachReleaseClaimRoot => "attach_release_claim_root",
            Self::AttachReserveProofRoot => "attach_reserve_proof_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachPrivacyBudgetRoot => "attach_privacy_budget_root",
            Self::AttachPrivacyLinkageRoot => "attach_privacy_linkage_root",
            Self::AttachMetadataRedactionRoot => "attach_metadata_redaction_root",
            Self::AttachNullifierSeparationRoot => "attach_nullifier_separation_root",
            Self::AttachPqAuthorityRoot => "attach_pq_authority_root",
            Self::AttachPqQuorumRoot => "attach_pq_quorum_root",
            Self::AttachAntiEquivocationRoot => "attach_anti_equivocation_root",
            Self::AttachReplayForkDuplicateGuardRoots => "attach_replay_fork_duplicate_guard_roots",
            Self::AttachSealCommitmentRoot => "attach_seal_commitment_root",
            Self::AttachSealReceiptRoot => "attach_seal_receipt_root",
            Self::KeepReleaseDenied => "keep_release_denied",
            Self::PublishRootsOnlySealRecord => "publish_roots_only_seal_record",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub seal_suite: String,
    pub wave: u64,
    pub release_readiness_wave: u64,
    pub promotion_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_pq_signers: u64,
    pub min_pq_families: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub min_privacy_budget_remaining_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub min_redaction_depth: u64,
    pub fail_closed: bool,
    pub heavy_gate_ran: bool,
    pub production_allowed: bool,
    pub release_allowed: bool,
    pub anti_equivocation_blockers_active: bool,
    pub roots_only_public_record: bool,
    pub max_release_claims: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            seal_suite: SEAL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            release_readiness_wave: RELEASE_READINESS_WAVE,
            promotion_wave: PROMOTION_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_pq_signers: DEFAULT_MIN_PQ_SIGNERS,
            min_pq_families: DEFAULT_MIN_PQ_FAMILIES,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            min_privacy_budget_remaining_bps: DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            min_redaction_depth: DEFAULT_MIN_REDACTION_DEPTH,
            fail_closed: true,
            heavy_gate_ran: false,
            production_allowed: false,
            release_allowed: false,
            anti_equivocation_blockers_active: true,
            roots_only_public_record: true,
            max_release_claims: DEFAULT_MAX_RELEASE_CLAIMS,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "seal_suite": self.seal_suite,
            "wave": self.wave,
            "release_readiness_wave": self.release_readiness_wave,
            "promotion_wave": self.promotion_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_pq_signers": self.min_pq_signers,
            "min_pq_families": self.min_pq_families,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_reserve_buffer_bps": self.min_reserve_buffer_bps,
            "min_privacy_budget_remaining_bps": self.min_privacy_budget_remaining_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "min_redaction_depth": self.min_redaction_depth,
            "fail_closed": self.fail_closed,
            "heavy_gate_ran": self.heavy_gate_ran,
            "production_allowed": self.production_allowed,
            "release_allowed": self.release_allowed,
            "anti_equivocation_blockers_active": self.anti_equivocation_blockers_active,
            "roots_only_public_record": self.roots_only_public_record,
            "max_release_claims": self.max_release_claims,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRoots {
    pub wave96_readiness_root: String,
    pub wave95_promotion_root: String,
    pub release_claim_root: String,
    pub reserve_proof_root: String,
    pub reserve_coverage_root: String,
    pub privacy_budget_root: String,
    pub privacy_linkage_root: String,
    pub metadata_redaction_root: String,
    pub nullifier_separation_root: String,
    pub pq_authority_root: String,
    pub pq_quorum_root: String,
    pub operator_signoff_root: String,
    pub anti_equivocation_root: String,
    pub replay_guard_root: String,
    pub fork_guard_root: String,
    pub duplicate_guard_root: String,
    pub seal_commitment_root: String,
    pub seal_receipt_root: String,
}

impl ClaimRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave96_readiness_root": self.wave96_readiness_root,
            "wave95_promotion_root": self.wave95_promotion_root,
            "release_claim_root": self.release_claim_root,
            "reserve_proof_root": self.reserve_proof_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_budget_root": self.privacy_budget_root,
            "privacy_linkage_root": self.privacy_linkage_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "nullifier_separation_root": self.nullifier_separation_root,
            "pq_authority_root": self.pq_authority_root,
            "pq_quorum_root": self.pq_quorum_root,
            "operator_signoff_root": self.operator_signoff_root,
            "anti_equivocation_root": self.anti_equivocation_root,
            "replay_guard_root": self.replay_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "seal_commitment_root": self.seal_commitment_root,
            "seal_receipt_root": self.seal_receipt_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("claim_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimMetrics {
    pub ml_dsa_authority_epoch: u64,
    pub slh_dsa_authority_epoch: u64,
    pub pq_signer_count: u64,
    pub pq_family_count: u64,
    pub reserve_coverage_bps: u64,
    pub reserve_buffer_bps: u64,
    pub privacy_budget_remaining_bps: u64,
    pub linkage_risk_bps: u64,
    pub redaction_depth: u64,
    pub nullifier_domains_separated: bool,
    pub anti_equivocation_clear: bool,
    pub replay_guard_clear: bool,
    pub fork_guard_clear: bool,
    pub duplicate_guard_clear: bool,
    pub operator_signed: bool,
    pub root_only: bool,
}

impl Default for ClaimMetrics {
    fn default() -> Self {
        Self {
            ml_dsa_authority_epoch: 0,
            slh_dsa_authority_epoch: 0,
            pq_signer_count: 0,
            pq_family_count: 0,
            reserve_coverage_bps: 0,
            reserve_buffer_bps: 0,
            privacy_budget_remaining_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            redaction_depth: 0,
            nullifier_domains_separated: false,
            anti_equivocation_clear: false,
            replay_guard_clear: false,
            fork_guard_clear: false,
            duplicate_guard_clear: false,
            operator_signed: false,
            root_only: true,
        }
    }
}

impl ClaimMetrics {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("claim_metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimSeal {
    pub seal_root: String,
    pub claim: ClaimKind,
    pub status: SealStatus,
    pub roots: ClaimRoots,
    pub metrics: ClaimMetrics,
    pub blockers: Vec<SealBlocker>,
}

impl ReleaseClaimSeal {
    pub fn evaluate(
        config: &Config,
        claim: ClaimKind,
        roots: ClaimRoots,
        metrics: ClaimMetrics,
    ) -> Self {
        let blockers = seal_blockers(config, &roots, &metrics);
        let status = if !metrics.root_only || !config.roots_only_public_record {
            SealStatus::Rejected
        } else if blockers.is_empty() {
            SealStatus::SealedShadow
        } else {
            SealStatus::Blocked
        };
        let seal_root = release_seal_root(claim, status, &roots, &metrics, &blockers);
        Self {
            seal_root,
            claim,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "seal_root": self.seal_root,
            "claim": self.claim.as_str(),
            "status": self.status.as_str(),
            "roots_root": self.roots.state_root(),
            "metrics_root": self.metrics.state_root(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_seal", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimState {
    pub claim: ClaimKind,
    pub sealed: bool,
    pub release_denied: bool,
    pub blocker_root: String,
    pub seal_root: String,
    pub command_hint_root: String,
}

impl ClaimState {
    pub fn blocked(claim: ClaimKind) -> Self {
        let blockers = vec![
            SealBlocker::NoReleaseClaim,
            SealBlocker::ProductionDenied,
            SealBlocker::HeavyGateNotRun,
            SealBlocker::AntiEquivocationRootMissing,
            SealBlocker::ReplayGuardRootMissing,
            SealBlocker::ForkGuardRootMissing,
            SealBlocker::DuplicateGuardRootMissing,
        ];
        Self {
            claim,
            sealed: false,
            release_denied: true,
            blocker_root: blocker_root(&blockers),
            seal_root: empty_root("release_claim_seal", claim.as_str()),
            command_hint_root: command_hint_root(claim),
        }
    }

    pub fn from_seal(claim: ClaimKind, seal: &ReleaseClaimSeal) -> Self {
        let sealed = seal.status == SealStatus::SealedShadow;
        Self {
            claim,
            sealed,
            release_denied: !sealed,
            blocker_root: blocker_root(&seal.blockers),
            seal_root: seal.state_root(),
            command_hint_root: command_hint_root(claim),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "claim": self.claim.as_str(),
            "sealed": self.sealed,
            "release_denied": self.release_denied,
            "blocker_root": self.blocker_root,
            "seal_root": self.seal_root,
            "command_hint_root": self.command_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("claim_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SealCounters {
    pub release_claim_count: u64,
    pub sealed_shadow_count: u64,
    pub blocked_count: u64,
    pub rejected_count: u64,
    pub sealed_claim_count: u64,
    pub release_denied_claim_count: u64,
}

impl SealCounters {
    pub fn from_parts(
        seals: &[ReleaseClaimSeal],
        claims: &BTreeMap<ClaimKind, ClaimState>,
    ) -> Self {
        let mut counters = Self::default();
        for seal in seals {
            counters.release_claim_count = counters.release_claim_count.saturating_add(1);
            match seal.status {
                SealStatus::SealedShadow => {
                    counters.sealed_shadow_count = counters.sealed_shadow_count.saturating_add(1);
                }
                SealStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                SealStatus::Rejected => {
                    counters.rejected_count = counters.rejected_count.saturating_add(1);
                }
            }
        }
        counters.sealed_claim_count = claims.values().filter(|claim| claim.sealed).count() as u64;
        counters.release_denied_claim_count =
            claims.values().filter(|claim| claim.release_denied).count() as u64;
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("seal_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claim_seals: Vec<ReleaseClaimSeal>,
    pub claim_states: BTreeMap<ClaimKind, ClaimState>,
    pub counters: SealCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, release_claim_seals: Vec<ReleaseClaimSeal>) -> Result<Self> {
        if release_claim_seals.len() > config.max_release_claims {
            return Err(SealBlocker::ClaimCapacityReached.as_str().to_string());
        }
        let mut seen = BTreeSet::new();
        for seal in &release_claim_seals {
            if !seen.insert(seal.seal_root.clone()) {
                return Err(SealBlocker::DuplicateSealRoot.as_str().to_string());
            }
        }
        let mut claim_states = BTreeMap::new();
        for claim in ClaimKind::all() {
            let maybe_seal = release_claim_seals
                .iter()
                .rev()
                .find(|seal| seal.claim == claim);
            let state = match maybe_seal {
                Some(seal) => ClaimState::from_seal(claim, seal),
                None => ClaimState::blocked(claim),
            };
            claim_states.insert(claim, state);
        }
        let counters = SealCounters::from_parts(&release_claim_seals, &claim_states);
        Ok(Self {
            config,
            release_claim_seals,
            claim_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn seal_release_claim(
        &self,
        claim: ClaimKind,
        roots: ClaimRoots,
        metrics: ClaimMetrics,
    ) -> Result<Self> {
        let mut release_claim_seals = self.release_claim_seals.clone();
        if release_claim_seals.len() >= self.config.max_release_claims {
            return Err(SealBlocker::ClaimCapacityReached.as_str().to_string());
        }
        let seal = ReleaseClaimSeal::evaluate(&self.config, claim, roots, metrics);
        if release_claim_seals
            .iter()
            .any(|item| item.seal_root == seal.seal_root)
        {
            return Err(SealBlocker::DuplicateSealRoot.as_str().to_string());
        }
        release_claim_seals.push(seal);
        Self::new(self.config.clone(), release_claim_seals)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.release_claim_seals.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.sealed_claim_count == ClaimKind::all().len() as u64
            && self.counters.release_denied_claim_count == 0
        {
            RuntimeVerdict::SealedShadow
        } else {
            RuntimeVerdict::Blocked
        }
    }

    pub fn public_record(&self) -> Value {
        let claim_records = self
            .claim_states
            .values()
            .map(ClaimState::public_record)
            .collect::<Vec<_>>();
        let seal_records = self
            .release_claim_seals
            .iter()
            .map(ReleaseClaimSeal::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "claim_state_root": list_root("claim_states", claim_records),
            "release_claim_seal_root": list_root("release_claim_seals", seal_records),
            "reserve_proof_root": reserve_proof_root(&self.release_claim_seals),
            "privacy_budget_root": privacy_budget_root(&self.release_claim_seals),
            "anti_equivocation_root": anti_equivocation_root(&self.release_claim_seals, &self.claim_states),
            "replay_fork_duplicate_guard_root": guard_root(&self.release_claim_seals),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.release_claim_seals, &self.claim_states),
            "active_anti_equivocation_blocker_root": blocker_root(&default_active_seal_blockers()),
            "operator_command_root": operator_command_root(&self.operator_commands),
            "verdict": self.verdict().as_str(),
            "release_denied": self.verdict() != RuntimeVerdict::SealedShadow,
            "counters": self.counters.public_record(),
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state"),
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state-core"),
            &[
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&seals_root(&self.release_claim_seals)),
                HashPart::Str(&claims_root(&self.claim_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_seal_blockers())),
                HashPart::Str(&operator_command_root(&self.operator_commands)),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    match State::new(config.clone(), Vec::new()) {
        Ok(state) => state,
        Err(reason) => {
            let roots = ClaimRoots {
                anti_equivocation_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..ClaimRoots::empty()
            };
            let seal = ReleaseClaimSeal::evaluate(
                &config,
                ClaimKind::OperatorSignoff,
                roots,
                ClaimMetrics::default(),
            );
            State {
                config,
                release_claim_seals: Vec::new(),
                claim_states: ClaimKind::all()
                    .into_iter()
                    .map(|claim| (claim, ClaimState::blocked(claim)))
                    .collect(),
                counters: SealCounters::from_parts(&[seal], &BTreeMap::new()),
                operator_commands: OperatorCommand::sequence(),
            }
        }
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn seal_blockers(config: &Config, roots: &ClaimRoots, metrics: &ClaimMetrics) -> Vec<SealBlocker> {
    let mut blockers = Vec::new();
    if !config.production_allowed || !config.release_allowed {
        blockers.push(SealBlocker::ProductionDenied);
    }
    if !config.heavy_gate_ran {
        blockers.push(SealBlocker::HeavyGateNotRun);
    }
    if roots.wave96_readiness_root.is_empty() {
        blockers.push(SealBlocker::Wave96ReadinessRootMissing);
    }
    if roots.wave95_promotion_root.is_empty() {
        blockers.push(SealBlocker::Wave95PromotionRootMissing);
    }
    if roots.release_claim_root.is_empty() {
        blockers.push(SealBlocker::ReleaseClaimRootMissing);
    }
    if roots.reserve_proof_root.is_empty() {
        blockers.push(SealBlocker::ReserveProofRootMissing);
    }
    if roots.reserve_coverage_root.is_empty() {
        blockers.push(SealBlocker::ReserveCoverageRootMissing);
    }
    if roots.privacy_budget_root.is_empty() {
        blockers.push(SealBlocker::PrivacyBudgetRootMissing);
    }
    if roots.privacy_linkage_root.is_empty() {
        blockers.push(SealBlocker::PrivacyLinkageRootMissing);
    }
    if roots.metadata_redaction_root.is_empty() {
        blockers.push(SealBlocker::MetadataRedactionRootMissing);
    }
    if roots.nullifier_separation_root.is_empty() {
        blockers.push(SealBlocker::NullifierSeparationRootMissing);
    }
    if roots.pq_authority_root.is_empty() {
        blockers.push(SealBlocker::PqAuthorityRootMissing);
    }
    if roots.pq_quorum_root.is_empty() {
        blockers.push(SealBlocker::PqQuorumRootMissing);
    }
    if roots.operator_signoff_root.is_empty() {
        blockers.push(SealBlocker::OperatorSignoffRootMissing);
    }
    if roots.anti_equivocation_root.is_empty() {
        blockers.push(SealBlocker::AntiEquivocationRootMissing);
    }
    if roots.replay_guard_root.is_empty() {
        blockers.push(SealBlocker::ReplayGuardRootMissing);
    }
    if roots.fork_guard_root.is_empty() {
        blockers.push(SealBlocker::ForkGuardRootMissing);
    }
    if roots.duplicate_guard_root.is_empty() {
        blockers.push(SealBlocker::DuplicateGuardRootMissing);
    }
    if roots.seal_commitment_root.is_empty() {
        blockers.push(SealBlocker::SealCommitmentRootMissing);
    }
    if roots.seal_receipt_root.is_empty() {
        blockers.push(SealBlocker::SealReceiptRootMissing);
    }
    if metrics.ml_dsa_authority_epoch < config.authority_epoch
        || metrics.slh_dsa_authority_epoch < config.authority_epoch
    {
        blockers.push(SealBlocker::AuthorityEpochTooLow);
    }
    if metrics.pq_signer_count < config.min_pq_signers {
        blockers.push(SealBlocker::PqQuorumTooSmall);
    }
    if metrics.pq_family_count < config.min_pq_families {
        blockers.push(SealBlocker::PqFamilyCoverageMissing);
    }
    if metrics.reserve_coverage_bps < config.min_reserve_coverage_bps {
        blockers.push(SealBlocker::ReserveCoverageMissing);
    }
    if metrics.reserve_buffer_bps < config.min_reserve_buffer_bps {
        blockers.push(SealBlocker::ReserveBufferMissing);
    }
    if metrics.privacy_budget_remaining_bps < config.min_privacy_budget_remaining_bps {
        blockers.push(SealBlocker::PrivacyBudgetTooLow);
    }
    if metrics.linkage_risk_bps > config.max_linkage_risk_bps {
        blockers.push(SealBlocker::PrivacyLinkageTooHigh);
    }
    if metrics.redaction_depth < config.min_redaction_depth {
        blockers.push(SealBlocker::MetadataRedactionTooShallow);
    }
    if !metrics.nullifier_domains_separated {
        blockers.push(SealBlocker::NullifierSeparationMissing);
    }
    if config.anti_equivocation_blockers_active && !metrics.anti_equivocation_clear {
        blockers.push(SealBlocker::AntiEquivocationViolation);
    }
    if !metrics.replay_guard_clear {
        blockers.push(SealBlocker::ReplayObserved);
    }
    if !metrics.fork_guard_clear {
        blockers.push(SealBlocker::ForkObserved);
    }
    if !metrics.duplicate_guard_clear {
        blockers.push(SealBlocker::DuplicateObserved);
    }
    if !metrics.operator_signed {
        blockers.push(SealBlocker::OperatorSignoffRootMissing);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(SealBlocker::RootsOnlyBoundary);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_seal_blockers() -> Vec<SealBlocker> {
    vec![
        SealBlocker::NoReleaseClaim,
        SealBlocker::ProductionDenied,
        SealBlocker::HeavyGateNotRun,
        SealBlocker::Wave96ReadinessRootMissing,
        SealBlocker::Wave95PromotionRootMissing,
        SealBlocker::ReleaseClaimRootMissing,
        SealBlocker::ReserveProofRootMissing,
        SealBlocker::ReserveCoverageRootMissing,
        SealBlocker::PrivacyBudgetRootMissing,
        SealBlocker::PrivacyLinkageRootMissing,
        SealBlocker::MetadataRedactionRootMissing,
        SealBlocker::NullifierSeparationRootMissing,
        SealBlocker::PqAuthorityRootMissing,
        SealBlocker::PqQuorumRootMissing,
        SealBlocker::OperatorSignoffRootMissing,
        SealBlocker::AntiEquivocationRootMissing,
        SealBlocker::ReplayGuardRootMissing,
        SealBlocker::ForkGuardRootMissing,
        SealBlocker::DuplicateGuardRootMissing,
        SealBlocker::SealCommitmentRootMissing,
        SealBlocker::SealReceiptRootMissing,
        SealBlocker::AntiEquivocationViolation,
        SealBlocker::ReplayObserved,
        SealBlocker::ForkObserved,
        SealBlocker::DuplicateObserved,
        SealBlocker::RootsOnlyBoundary,
    ]
}

fn release_seal_root(
    claim: ClaimKind,
    status: SealStatus,
    roots: &ClaimRoots,
    metrics: &ClaimMetrics,
    blockers: &[SealBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:release-claim-anti-equivocation-seal"),
        &[
            HashPart::Str(claim.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn seals_root(seals: &[ReleaseClaimSeal]) -> String {
    let leaves = seals
        .iter()
        .map(|seal| Value::String(seal.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:seals"), &leaves)
}

fn claims_root(claims: &BTreeMap<ClaimKind, ClaimState>) -> String {
    let leaves = claims
        .values()
        .map(|claim| Value::String(claim.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:claims"), &leaves)
}

fn reserve_proof_root(seals: &[ReleaseClaimSeal]) -> String {
    let leaves = seals
        .iter()
        .map(|seal| {
            json!({
                "claim": seal.claim.as_str(),
                "reserve_proof_root": seal.roots.reserve_proof_root,
                "reserve_coverage_root": seal.roots.reserve_coverage_root,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:reserve-proof-roots"), &leaves)
}

fn privacy_budget_root(seals: &[ReleaseClaimSeal]) -> String {
    let leaves = seals
        .iter()
        .map(|seal| {
            json!({
                "claim": seal.claim.as_str(),
                "privacy_budget_root": seal.roots.privacy_budget_root,
                "privacy_budget_remaining_bps": seal.metrics.privacy_budget_remaining_bps,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:privacy-budget-roots"), &leaves)
}

fn anti_equivocation_root(
    seals: &[ReleaseClaimSeal],
    claims: &BTreeMap<ClaimKind, ClaimState>,
) -> String {
    let mut leaves = seals
        .iter()
        .map(|seal| {
            json!({
                "claim": seal.claim.as_str(),
                "anti_equivocation_root": seal.roots.anti_equivocation_root,
                "seal_root": seal.seal_root,
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(claims.values().filter(|claim| !claim.sealed).map(|claim| {
        json!({
            "claim": claim.claim.as_str(),
            "blocker_root": claim.blocker_root,
            "release_denied": claim.release_denied,
        })
    }));
    merkle_root(&format!("{DOMAIN}:anti-equivocation"), &leaves)
}

fn guard_root(seals: &[ReleaseClaimSeal]) -> String {
    let leaves = seals
        .iter()
        .map(|seal| {
            json!({
                "claim": seal.claim.as_str(),
                "replay_guard_root": seal.roots.replay_guard_root,
                "fork_guard_root": seal.roots.fork_guard_root,
                "duplicate_guard_root": seal.roots.duplicate_guard_root,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:replay-fork-duplicate-guards"), &leaves)
}

fn blocker_root(blockers: &[SealBlocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| json!({ "blocker": blocker.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn all_blockers_root(
    seals: &[ReleaseClaimSeal],
    claims: &BTreeMap<ClaimKind, ClaimState>,
) -> String {
    let mut leaves = seals
        .iter()
        .flat_map(|seal| {
            seal.blockers.iter().map(|blocker| {
                json!({
                    "seal_root": seal.seal_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(
        claims
            .values()
            .filter(|claim| claim.release_denied)
            .map(|claim| {
                json!({
                    "claim": claim.claim.as_str(),
                    "blocker_root": claim.blocker_root,
                })
            }),
    );
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn operator_command_root(commands: &[OperatorCommand]) -> String {
    let leaves = commands
        .iter()
        .map(|command| json!({ "command": command.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:operator-commands"), &leaves)
}

fn command_hint_root(claim: ClaimKind) -> String {
    domain_hash(
        &format!("{DOMAIN}:command-hint"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(claim.as_str()),
            HashPart::Str(SEAL_SUITE),
        ],
        32,
    )
}

fn list_root(kind: &str, leaves: Vec<Value>) -> String {
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn empty_root(kind: &str, claim: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:empty"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(claim),
        ],
        32,
    )
}

fn dedupe_blockers(blockers: &mut Vec<SealBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
