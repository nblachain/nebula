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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave96-live-heavy-gate-receipt-release-readiness-quorum-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RELEASE_SUITE: &str =
    "monero-l2-wave96-wave95-promoted-slot-root-release-readiness-quorum-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 96;
pub const PROMOTION_WAVE: u64 = 95;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 96;
pub const DEFAULT_MIN_PQ_SIGNERS: u64 = 5;
pub const DEFAULT_MIN_PQ_FAMILIES: u64 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_500;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_REDACTION_DEPTH: u64 = 4;
pub const DEFAULT_MAX_RELEASE_CLAIMS: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave96-live-heavy-gate-receipt-release-readiness-quorum-pq-reserve-privacy-lane-runtime";

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
pub enum ReadinessDimension {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqQuorum,
    ReserveCoverage,
    PrivacyLinkage,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl ReadinessDimension {
    pub fn all() -> [Self; 8] {
        [
            Self::MlDsaAuthorityEpoch,
            Self::SlhDsaAuthorityEpoch,
            Self::PqQuorum,
            Self::ReserveCoverage,
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
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyLinkage => "privacy_linkage",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseStatus {
    Blocked,
    Placeholder,
    ReadyShadow,
    Rejected,
}

impl ReleaseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Placeholder => "placeholder",
            Self::ReadyShadow => "ready_shadow",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumBlocker {
    NoReleaseClaim,
    ProductionDenied,
    HeavyGateNotRun,
    PromotedSlotRootMissing,
    PromotedSlotQuorumUnmet,
    MlDsaAuthorityEpochMissing,
    SlhDsaAuthorityEpochMissing,
    PqQuorumMissing,
    PqFamilyCoverageMissing,
    ReserveCoverageMissing,
    ReserveBufferMissing,
    PrivacyLinkageMissing,
    MetadataRedactionMissing,
    NullifierSeparationMissing,
    OperatorSignoffMissing,
    ReleaseClaimPlaceholderOnly,
    RootsOnlyBoundary,
    DuplicateReleaseRoot,
    ReleaseCapacityReached,
}

impl QuorumBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoReleaseClaim => "no_release_claim",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::PromotedSlotRootMissing => "promoted_slot_root_missing",
            Self::PromotedSlotQuorumUnmet => "promoted_slot_quorum_unmet",
            Self::MlDsaAuthorityEpochMissing => "ml_dsa_authority_epoch_missing",
            Self::SlhDsaAuthorityEpochMissing => "slh_dsa_authority_epoch_missing",
            Self::PqQuorumMissing => "pq_quorum_missing",
            Self::PqFamilyCoverageMissing => "pq_family_coverage_missing",
            Self::ReserveCoverageMissing => "reserve_coverage_missing",
            Self::ReserveBufferMissing => "reserve_buffer_missing",
            Self::PrivacyLinkageMissing => "privacy_linkage_missing",
            Self::MetadataRedactionMissing => "metadata_redaction_missing",
            Self::NullifierSeparationMissing => "nullifier_separation_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ReleaseClaimPlaceholderOnly => "release_claim_placeholder_only",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DuplicateReleaseRoot => "duplicate_release_root",
            Self::ReleaseCapacityReached => "release_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    Blocked,
    ReadyShadow,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Blocked => "blocked",
            Self::ReadyShadow => "ready_shadow",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave95PromotedSlotRoots,
    AttachPromotedSlotQuorumRoot,
    AttachMlDsaAuthorityEpochRoot,
    AttachSlhDsaAuthorityEpochRoot,
    AttachPqQuorumRoot,
    AttachReserveCoverageRoot,
    AttachPrivacyLinkageRoot,
    AttachMetadataRedactionRoot,
    AttachNullifierSeparationRoot,
    AttachOperatorSignoffRoot,
    KeepReleaseClaimPlaceholder,
    KeepProductionDenied,
    PublishRootsOnlyReadinessRecord,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave95PromotedSlotRoots,
            Self::AttachPromotedSlotQuorumRoot,
            Self::AttachMlDsaAuthorityEpochRoot,
            Self::AttachSlhDsaAuthorityEpochRoot,
            Self::AttachPqQuorumRoot,
            Self::AttachReserveCoverageRoot,
            Self::AttachPrivacyLinkageRoot,
            Self::AttachMetadataRedactionRoot,
            Self::AttachNullifierSeparationRoot,
            Self::AttachOperatorSignoffRoot,
            Self::KeepReleaseClaimPlaceholder,
            Self::KeepProductionDenied,
            Self::PublishRootsOnlyReadinessRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave95PromotedSlotRoots => "import_wave95_promoted_slot_roots",
            Self::AttachPromotedSlotQuorumRoot => "attach_promoted_slot_quorum_root",
            Self::AttachMlDsaAuthorityEpochRoot => "attach_ml_dsa_authority_epoch_root",
            Self::AttachSlhDsaAuthorityEpochRoot => "attach_slh_dsa_authority_epoch_root",
            Self::AttachPqQuorumRoot => "attach_pq_quorum_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachPrivacyLinkageRoot => "attach_privacy_linkage_root",
            Self::AttachMetadataRedactionRoot => "attach_metadata_redaction_root",
            Self::AttachNullifierSeparationRoot => "attach_nullifier_separation_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
            Self::KeepReleaseClaimPlaceholder => "keep_release_claim_placeholder",
            Self::KeepProductionDenied => "keep_production_denied",
            Self::PublishRootsOnlyReadinessRecord => "publish_roots_only_readiness_record",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub release_suite: String,
    pub wave: u64,
    pub promotion_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_promoted_slot_roots: u64,
    pub min_pq_signers: u64,
    pub min_pq_families: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub min_redaction_depth: u64,
    pub fail_closed: bool,
    pub heavy_gate_ran: bool,
    pub production_allowed: bool,
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
            release_suite: RELEASE_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            promotion_wave: PROMOTION_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_promoted_slot_roots: ReadinessDimension::all().len() as u64,
            min_pq_signers: DEFAULT_MIN_PQ_SIGNERS,
            min_pq_families: DEFAULT_MIN_PQ_FAMILIES,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            min_redaction_depth: DEFAULT_MIN_REDACTION_DEPTH,
            fail_closed: true,
            heavy_gate_ran: false,
            production_allowed: false,
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
            "release_suite": self.release_suite,
            "wave": self.wave,
            "promotion_wave": self.promotion_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_promoted_slot_roots": self.min_promoted_slot_roots,
            "min_pq_signers": self.min_pq_signers,
            "min_pq_families": self.min_pq_families,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_reserve_buffer_bps": self.min_reserve_buffer_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "min_redaction_depth": self.min_redaction_depth,
            "fail_closed": self.fail_closed,
            "heavy_gate_ran": self.heavy_gate_ran,
            "production_allowed": self.production_allowed,
            "roots_only_public_record": self.roots_only_public_record,
            "max_release_claims": self.max_release_claims,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseRoots {
    pub wave95_promoted_slot_root: String,
    pub promoted_slot_quorum_root: String,
    pub ml_dsa_authority_epoch_root: String,
    pub slh_dsa_authority_epoch_root: String,
    pub pq_quorum_root: String,
    pub reserve_coverage_root: String,
    pub privacy_linkage_root: String,
    pub metadata_redaction_root: String,
    pub nullifier_separation_root: String,
    pub operator_signoff_root: String,
    pub release_claim_placeholder_root: String,
}

impl ReleaseRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave95_promoted_slot_root": self.wave95_promoted_slot_root,
            "promoted_slot_quorum_root": self.promoted_slot_quorum_root,
            "ml_dsa_authority_epoch_root": self.ml_dsa_authority_epoch_root,
            "slh_dsa_authority_epoch_root": self.slh_dsa_authority_epoch_root,
            "pq_quorum_root": self.pq_quorum_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_linkage_root": self.privacy_linkage_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "nullifier_separation_root": self.nullifier_separation_root,
            "operator_signoff_root": self.operator_signoff_root,
            "release_claim_placeholder_root": self.release_claim_placeholder_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseMetrics {
    pub promoted_slot_root_count: u64,
    pub ml_dsa_authority_epoch: u64,
    pub slh_dsa_authority_epoch: u64,
    pub pq_signer_count: u64,
    pub pq_family_count: u64,
    pub reserve_coverage_bps: u64,
    pub reserve_buffer_bps: u64,
    pub linkage_risk_bps: u64,
    pub redaction_depth: u64,
    pub nullifier_domains_separated: bool,
    pub operator_signed: bool,
    pub release_claim_placeholder: bool,
    pub root_only: bool,
}

impl Default for ReleaseMetrics {
    fn default() -> Self {
        Self {
            promoted_slot_root_count: 0,
            ml_dsa_authority_epoch: 0,
            slh_dsa_authority_epoch: 0,
            pq_signer_count: 0,
            pq_family_count: 0,
            reserve_coverage_bps: 0,
            reserve_buffer_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            redaction_depth: 0,
            nullifier_domains_separated: false,
            operator_signed: false,
            release_claim_placeholder: true,
            root_only: true,
        }
    }
}

impl ReleaseMetrics {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("release_metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaim {
    pub claim_root: String,
    pub dimension: ReadinessDimension,
    pub status: ReleaseStatus,
    pub roots: ReleaseRoots,
    pub metrics: ReleaseMetrics,
    pub blockers: Vec<QuorumBlocker>,
}

impl ReleaseClaim {
    pub fn evaluate(
        config: &Config,
        dimension: ReadinessDimension,
        roots: ReleaseRoots,
        metrics: ReleaseMetrics,
    ) -> Self {
        let blockers = quorum_blockers(config, &roots, &metrics);
        let status = if !metrics.root_only {
            ReleaseStatus::Rejected
        } else if metrics.release_claim_placeholder {
            ReleaseStatus::Placeholder
        } else if blockers.is_empty() {
            ReleaseStatus::ReadyShadow
        } else {
            ReleaseStatus::Blocked
        };
        let claim_root = release_claim_root(dimension, status, &roots, &metrics, &blockers);
        Self {
            claim_root,
            dimension,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "claim_root": self.claim_root,
            "dimension": self.dimension.as_str(),
            "status": self.status.as_str(),
            "roots_root": self.roots.state_root(),
            "metrics_root": self.metrics.state_root(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DimensionState {
    pub dimension: ReadinessDimension,
    pub release_ready: bool,
    pub blocker_root: String,
    pub release_claim_root: String,
    pub promoted_slot_root: String,
}

impl DimensionState {
    pub fn blocked(dimension: ReadinessDimension) -> Self {
        let blockers = vec![
            QuorumBlocker::NoReleaseClaim,
            QuorumBlocker::ProductionDenied,
            QuorumBlocker::HeavyGateNotRun,
            QuorumBlocker::ReleaseClaimPlaceholderOnly,
        ];
        Self {
            dimension,
            release_ready: false,
            blocker_root: blocker_root(&blockers),
            release_claim_root: empty_root("release_claim", dimension.as_str()),
            promoted_slot_root: empty_root("promoted_slot", dimension.as_str()),
        }
    }

    pub fn from_claim(dimension: ReadinessDimension, claim: &ReleaseClaim) -> Self {
        let ready = claim.status == ReleaseStatus::ReadyShadow;
        Self {
            dimension,
            release_ready: ready,
            blocker_root: blocker_root(&claim.blockers),
            release_claim_root: claim.state_root(),
            promoted_slot_root: if ready {
                claim.roots.wave95_promoted_slot_root.clone()
            } else {
                empty_root("promoted_slot", dimension.as_str())
            },
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "dimension": self.dimension.as_str(),
            "release_ready": self.release_ready,
            "blocker_root": self.blocker_root,
            "release_claim_root": self.release_claim_root,
            "promoted_slot_root": self.promoted_slot_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("dimension_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCounters {
    pub release_claim_count: u64,
    pub ready_shadow_count: u64,
    pub placeholder_count: u64,
    pub blocked_count: u64,
    pub rejected_count: u64,
    pub release_ready_lane_count: u64,
    pub blocked_lane_count: u64,
}

impl ReleaseCounters {
    pub fn from_parts(
        claims: &[ReleaseClaim],
        dimensions: &BTreeMap<ReadinessDimension, DimensionState>,
    ) -> Self {
        let mut counters = Self::default();
        for claim in claims {
            counters.release_claim_count = counters.release_claim_count.saturating_add(1);
            match claim.status {
                ReleaseStatus::ReadyShadow => {
                    counters.ready_shadow_count = counters.ready_shadow_count.saturating_add(1);
                }
                ReleaseStatus::Placeholder => {
                    counters.placeholder_count = counters.placeholder_count.saturating_add(1);
                }
                ReleaseStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                ReleaseStatus::Rejected => {
                    counters.rejected_count = counters.rejected_count.saturating_add(1);
                }
            }
        }
        counters.release_ready_lane_count = dimensions
            .values()
            .filter(|item| item.release_ready)
            .count() as u64;
        counters.blocked_lane_count = dimensions
            .values()
            .filter(|item| !item.release_ready)
            .count() as u64;
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("release_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claims: Vec<ReleaseClaim>,
    pub dimension_states: BTreeMap<ReadinessDimension, DimensionState>,
    pub counters: ReleaseCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, release_claims: Vec<ReleaseClaim>) -> Result<Self> {
        if release_claims.len() > config.max_release_claims {
            return Err(QuorumBlocker::ReleaseCapacityReached.as_str().to_string());
        }
        let mut seen = BTreeSet::new();
        for claim in &release_claims {
            if !seen.insert(claim.claim_root.clone()) {
                return Err(QuorumBlocker::DuplicateReleaseRoot.as_str().to_string());
            }
        }
        let mut dimension_states = BTreeMap::new();
        for dimension in ReadinessDimension::all() {
            let maybe_claim = release_claims
                .iter()
                .rev()
                .find(|claim| claim.dimension == dimension);
            let state = match maybe_claim {
                Some(claim) => DimensionState::from_claim(dimension, claim),
                None => DimensionState::blocked(dimension),
            };
            dimension_states.insert(dimension, state);
        }
        let counters = ReleaseCounters::from_parts(&release_claims, &dimension_states);
        Ok(Self {
            config,
            release_claims,
            dimension_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn aggregate_promoted_slot_roots(
        &self,
        dimension: ReadinessDimension,
        roots: ReleaseRoots,
        metrics: ReleaseMetrics,
    ) -> Result<Self> {
        let mut release_claims = self.release_claims.clone();
        if release_claims.len() >= self.config.max_release_claims {
            return Err(QuorumBlocker::ReleaseCapacityReached.as_str().to_string());
        }
        let claim = ReleaseClaim::evaluate(&self.config, dimension, roots, metrics);
        if release_claims
            .iter()
            .any(|item| item.claim_root == claim.claim_root)
        {
            return Err(QuorumBlocker::DuplicateReleaseRoot.as_str().to_string());
        }
        release_claims.push(claim);
        Self::new(self.config.clone(), release_claims)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.release_claims.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.release_ready_lane_count == ReadinessDimension::all().len() as u64
            && self.counters.blocked_lane_count == 0
        {
            RuntimeVerdict::ReadyShadow
        } else {
            RuntimeVerdict::Blocked
        }
    }

    pub fn public_record(&self) -> Value {
        let dimension_records = self
            .dimension_states
            .values()
            .map(DimensionState::public_record)
            .collect::<Vec<_>>();
        let claim_records = self
            .release_claims
            .iter()
            .map(ReleaseClaim::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "promoted_slot_root": promoted_slot_root(&self.dimension_states),
            "dimension_state_root": list_root("dimension_states", dimension_records),
            "release_claim_root": list_root("release_claims", claim_records),
            "release_claim_placeholder_root": empty_root("release_claim_placeholder", "lane"),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.release_claims, &self.dimension_states),
            "active_quorum_blocker_root": blocker_root(&default_active_quorum_blockers()),
            "operator_command_root": operator_command_root(&self.operator_commands),
            "verdict": self.verdict().as_str(),
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
                HashPart::Str(&claims_root(&self.release_claims)),
                HashPart::Str(&dimensions_root(&self.dimension_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_quorum_blockers())),
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
            let roots = ReleaseRoots {
                release_claim_placeholder_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..ReleaseRoots::empty()
            };
            let claim = ReleaseClaim::evaluate(
                &config,
                ReadinessDimension::OperatorSignoff,
                roots,
                ReleaseMetrics::default(),
            );
            State {
                config,
                release_claims: Vec::new(),
                dimension_states: ReadinessDimension::all()
                    .into_iter()
                    .map(|dimension| (dimension, DimensionState::blocked(dimension)))
                    .collect(),
                counters: ReleaseCounters::from_parts(&[claim], &BTreeMap::new()),
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

fn quorum_blockers(
    config: &Config,
    roots: &ReleaseRoots,
    metrics: &ReleaseMetrics,
) -> Vec<QuorumBlocker> {
    let mut blockers = Vec::new();
    if !config.production_allowed {
        blockers.push(QuorumBlocker::ProductionDenied);
    }
    if !config.heavy_gate_ran {
        blockers.push(QuorumBlocker::HeavyGateNotRun);
    }
    if roots.wave95_promoted_slot_root.is_empty() {
        blockers.push(QuorumBlocker::PromotedSlotRootMissing);
    }
    if roots.promoted_slot_quorum_root.is_empty()
        || metrics.promoted_slot_root_count < config.min_promoted_slot_roots
    {
        blockers.push(QuorumBlocker::PromotedSlotQuorumUnmet);
    }
    if roots.ml_dsa_authority_epoch_root.is_empty()
        || metrics.ml_dsa_authority_epoch < config.authority_epoch
    {
        blockers.push(QuorumBlocker::MlDsaAuthorityEpochMissing);
    }
    if roots.slh_dsa_authority_epoch_root.is_empty()
        || metrics.slh_dsa_authority_epoch < config.authority_epoch
    {
        blockers.push(QuorumBlocker::SlhDsaAuthorityEpochMissing);
    }
    if roots.pq_quorum_root.is_empty() || metrics.pq_signer_count < config.min_pq_signers {
        blockers.push(QuorumBlocker::PqQuorumMissing);
    }
    if metrics.pq_family_count < config.min_pq_families {
        blockers.push(QuorumBlocker::PqFamilyCoverageMissing);
    }
    if roots.reserve_coverage_root.is_empty()
        || metrics.reserve_coverage_bps < config.min_reserve_coverage_bps
    {
        blockers.push(QuorumBlocker::ReserveCoverageMissing);
    }
    if metrics.reserve_buffer_bps < config.min_reserve_buffer_bps {
        blockers.push(QuorumBlocker::ReserveBufferMissing);
    }
    if roots.privacy_linkage_root.is_empty()
        || metrics.linkage_risk_bps > config.max_linkage_risk_bps
    {
        blockers.push(QuorumBlocker::PrivacyLinkageMissing);
    }
    if roots.metadata_redaction_root.is_empty()
        || metrics.redaction_depth < config.min_redaction_depth
    {
        blockers.push(QuorumBlocker::MetadataRedactionMissing);
    }
    if roots.nullifier_separation_root.is_empty() || !metrics.nullifier_domains_separated {
        blockers.push(QuorumBlocker::NullifierSeparationMissing);
    }
    if roots.operator_signoff_root.is_empty() || !metrics.operator_signed {
        blockers.push(QuorumBlocker::OperatorSignoffMissing);
    }
    if roots.release_claim_placeholder_root.is_empty() || metrics.release_claim_placeholder {
        blockers.push(QuorumBlocker::ReleaseClaimPlaceholderOnly);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(QuorumBlocker::RootsOnlyBoundary);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_quorum_blockers() -> Vec<QuorumBlocker> {
    vec![
        QuorumBlocker::NoReleaseClaim,
        QuorumBlocker::ProductionDenied,
        QuorumBlocker::HeavyGateNotRun,
        QuorumBlocker::PromotedSlotRootMissing,
        QuorumBlocker::PromotedSlotQuorumUnmet,
        QuorumBlocker::MlDsaAuthorityEpochMissing,
        QuorumBlocker::SlhDsaAuthorityEpochMissing,
        QuorumBlocker::PqQuorumMissing,
        QuorumBlocker::PqFamilyCoverageMissing,
        QuorumBlocker::ReserveCoverageMissing,
        QuorumBlocker::ReserveBufferMissing,
        QuorumBlocker::PrivacyLinkageMissing,
        QuorumBlocker::MetadataRedactionMissing,
        QuorumBlocker::NullifierSeparationMissing,
        QuorumBlocker::OperatorSignoffMissing,
        QuorumBlocker::ReleaseClaimPlaceholderOnly,
        QuorumBlocker::RootsOnlyBoundary,
    ]
}

fn release_claim_root(
    dimension: ReadinessDimension,
    status: ReleaseStatus,
    roots: &ReleaseRoots,
    metrics: &ReleaseMetrics,
    blockers: &[QuorumBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:release-claim"),
        &[
            HashPart::Str(dimension.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn claims_root(claims: &[ReleaseClaim]) -> String {
    let leaves = claims
        .iter()
        .map(|claim| Value::String(claim.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:claims"), &leaves)
}

fn dimensions_root(dimensions: &BTreeMap<ReadinessDimension, DimensionState>) -> String {
    let leaves = dimensions
        .values()
        .map(|dimension| Value::String(dimension.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:dimensions"), &leaves)
}

fn promoted_slot_root(dimensions: &BTreeMap<ReadinessDimension, DimensionState>) -> String {
    let leaves = dimensions
        .values()
        .map(|dimension| {
            json!({
                "dimension": dimension.dimension.as_str(),
                "promoted_slot_root": dimension.promoted_slot_root,
                "release_ready": dimension.release_ready,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:promoted-slot-roots"), &leaves)
}

fn blocker_root(blockers: &[QuorumBlocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| json!({ "blocker": blocker.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn all_blockers_root(
    claims: &[ReleaseClaim],
    dimensions: &BTreeMap<ReadinessDimension, DimensionState>,
) -> String {
    let mut leaves = claims
        .iter()
        .flat_map(|claim| {
            claim.blockers.iter().map(|blocker| {
                json!({
                    "claim_root": claim.claim_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(
        dimensions
            .values()
            .filter(|dimension| !dimension.release_ready)
            .map(|dimension| {
                json!({
                    "dimension": dimension.dimension.as_str(),
                    "blocker_root": dimension.blocker_root,
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

fn empty_root(kind: &str, dimension: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:empty"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(dimension),
        ],
        32,
    )
}

fn dedupe_blockers(blockers: &mut Vec<QuorumBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
