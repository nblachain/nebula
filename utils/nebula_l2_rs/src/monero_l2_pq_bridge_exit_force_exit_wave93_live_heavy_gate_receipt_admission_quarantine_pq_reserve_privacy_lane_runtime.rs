use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave93-live-heavy-gate-receipt-admission-quarantine-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ADMISSION_SUITE: &str =
    "monero-l2-wave93-future-receipt-root-admission-quarantine-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 93;
pub const SOURCE_WAVE: u64 = 92;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 93;
pub const DEFAULT_MIN_PQ_SIGNERS: u64 = 5;
pub const DEFAULT_MIN_PQ_FAMILIES: u64 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_500;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_REDACTION_DEPTH: u64 = 4;
pub const DEFAULT_MAX_CANDIDATES: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave93-live-heavy-gate-receipt-admission-quarantine-pq-reserve-privacy-lane-runtime";

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
pub enum CandidateStatus {
    Quarantined,
    AdmissibleShadow,
    Rejected,
}

impl CandidateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quarantined => "quarantined",
            Self::AdmissibleShadow => "admissible_shadow",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    FutureReceiptSlotEmpty,
    NotLiveReceipt,
    HeavyGateNotRun,
    MissingMlDsaAuthorityEpoch,
    MissingSlhDsaAuthorityEpoch,
    PqQuorumMissing,
    PqFamilyCoverageMissing,
    ReserveCoverageMissing,
    PrivacyLinkageMissing,
    MetadataRedactionMissing,
    NullifierSeparationMissing,
    OperatorSignoffMissing,
    RootOnlyAdmissionPending,
}

impl QuarantineReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FutureReceiptSlotEmpty => "future_receipt_slot_empty",
            Self::NotLiveReceipt => "not_live_receipt",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::MissingMlDsaAuthorityEpoch => "missing_ml_dsa_authority_epoch",
            Self::MissingSlhDsaAuthorityEpoch => "missing_slh_dsa_authority_epoch",
            Self::PqQuorumMissing => "pq_quorum_missing",
            Self::PqFamilyCoverageMissing => "pq_family_coverage_missing",
            Self::ReserveCoverageMissing => "reserve_coverage_missing",
            Self::PrivacyLinkageMissing => "privacy_linkage_missing",
            Self::MetadataRedactionMissing => "metadata_redaction_missing",
            Self::NullifierSeparationMissing => "nullifier_separation_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::RootOnlyAdmissionPending => "root_only_admission_pending",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave92FailClosedSlotRoots,
    AttachMlDsaAuthorityEpochRoot,
    AttachSlhDsaAuthorityEpochRoot,
    AttachPqQuorumRoot,
    AttachReserveCoverageRoot,
    AttachPrivacyLinkageRoot,
    AttachMetadataRedactionRoot,
    AttachNullifierSeparationRoot,
    AttachOperatorSignoffRoot,
    KeepFutureReceiptQuarantined,
    PublishRootsOnlyAdmissionRecord,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave92FailClosedSlotRoots,
            Self::AttachMlDsaAuthorityEpochRoot,
            Self::AttachSlhDsaAuthorityEpochRoot,
            Self::AttachPqQuorumRoot,
            Self::AttachReserveCoverageRoot,
            Self::AttachPrivacyLinkageRoot,
            Self::AttachMetadataRedactionRoot,
            Self::AttachNullifierSeparationRoot,
            Self::AttachOperatorSignoffRoot,
            Self::KeepFutureReceiptQuarantined,
            Self::PublishRootsOnlyAdmissionRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave92FailClosedSlotRoots => "import_wave92_fail_closed_slot_roots",
            Self::AttachMlDsaAuthorityEpochRoot => "attach_ml_dsa_authority_epoch_root",
            Self::AttachSlhDsaAuthorityEpochRoot => "attach_slh_dsa_authority_epoch_root",
            Self::AttachPqQuorumRoot => "attach_pq_quorum_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachPrivacyLinkageRoot => "attach_privacy_linkage_root",
            Self::AttachMetadataRedactionRoot => "attach_metadata_redaction_root",
            Self::AttachNullifierSeparationRoot => "attach_nullifier_separation_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
            Self::KeepFutureReceiptQuarantined => "keep_future_receipt_quarantined",
            Self::PublishRootsOnlyAdmissionRecord => "publish_roots_only_admission_record",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub admission_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_pq_signers: u64,
    pub min_pq_families: u64,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub min_redaction_depth: u64,
    pub require_ml_dsa_authority: bool,
    pub require_slh_dsa_authority: bool,
    pub require_pq_quorum: bool,
    pub require_reserve_coverage: bool,
    pub require_privacy_linkage: bool,
    pub require_metadata_redaction: bool,
    pub require_nullifier_separation: bool,
    pub require_operator_signoff: bool,
    pub fail_closed: bool,
    pub heavy_gate_ran: bool,
    pub max_candidates: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            admission_suite: ADMISSION_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: SOURCE_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_pq_signers: DEFAULT_MIN_PQ_SIGNERS,
            min_pq_families: DEFAULT_MIN_PQ_FAMILIES,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            min_redaction_depth: DEFAULT_MIN_REDACTION_DEPTH,
            require_ml_dsa_authority: true,
            require_slh_dsa_authority: true,
            require_pq_quorum: true,
            require_reserve_coverage: true,
            require_privacy_linkage: true,
            require_metadata_redaction: true,
            require_nullifier_separation: true,
            require_operator_signoff: true,
            fail_closed: true,
            heavy_gate_ran: false,
            max_candidates: DEFAULT_MAX_CANDIDATES,
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
            "admission_suite": self.admission_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_pq_signers": self.min_pq_signers,
            "min_pq_families": self.min_pq_families,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "min_reserve_buffer_bps": self.min_reserve_buffer_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "min_redaction_depth": self.min_redaction_depth,
            "require_ml_dsa_authority": self.require_ml_dsa_authority,
            "require_slh_dsa_authority": self.require_slh_dsa_authority,
            "require_pq_quorum": self.require_pq_quorum,
            "require_reserve_coverage": self.require_reserve_coverage,
            "require_privacy_linkage": self.require_privacy_linkage,
            "require_metadata_redaction": self.require_metadata_redaction,
            "require_nullifier_separation": self.require_nullifier_separation,
            "require_operator_signoff": self.require_operator_signoff,
            "fail_closed": self.fail_closed,
            "heavy_gate_ran": self.heavy_gate_ran,
            "max_candidates": self.max_candidates,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionRoots {
    pub future_receipt_root: String,
    pub ml_dsa_authority_epoch_root: String,
    pub slh_dsa_authority_epoch_root: String,
    pub pq_quorum_root: String,
    pub reserve_coverage_root: String,
    pub privacy_linkage_root: String,
    pub metadata_redaction_root: String,
    pub nullifier_separation_root: String,
    pub operator_signoff_root: String,
}

impl AdmissionRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "future_receipt_root": self.future_receipt_root,
            "ml_dsa_authority_epoch_root": self.ml_dsa_authority_epoch_root,
            "slh_dsa_authority_epoch_root": self.slh_dsa_authority_epoch_root,
            "pq_quorum_root": self.pq_quorum_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "privacy_linkage_root": self.privacy_linkage_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "nullifier_separation_root": self.nullifier_separation_root,
            "operator_signoff_root": self.operator_signoff_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission-roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionMetrics {
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
    pub root_only: bool,
    pub future_slot: bool,
    pub live_receipt: bool,
}

impl Default for AdmissionMetrics {
    fn default() -> Self {
        Self {
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
            root_only: true,
            future_slot: true,
            live_receipt: false,
        }
    }
}

impl AdmissionMetrics {
    pub fn public_record(&self) -> Value {
        json!({
            "ml_dsa_authority_epoch": self.ml_dsa_authority_epoch,
            "slh_dsa_authority_epoch": self.slh_dsa_authority_epoch,
            "pq_signer_count": self.pq_signer_count,
            "pq_family_count": self.pq_family_count,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "reserve_buffer_bps": self.reserve_buffer_bps,
            "linkage_risk_bps": self.linkage_risk_bps,
            "redaction_depth": self.redaction_depth,
            "nullifier_domains_separated": self.nullifier_domains_separated,
            "operator_signed": self.operator_signed,
            "root_only": self.root_only,
            "future_slot": self.future_slot,
            "live_receipt": self.live_receipt,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission-metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptCandidate {
    pub candidate_root: String,
    pub roots: AdmissionRoots,
    pub metrics: AdmissionMetrics,
    pub status: CandidateStatus,
    pub quarantine_reasons: Vec<QuarantineReason>,
}

impl ReceiptCandidate {
    pub fn empty_quarantined(config: &Config) -> Self {
        let roots = AdmissionRoots::empty();
        let metrics = AdmissionMetrics::default();
        let reasons = admission_reasons(config, &roots, &metrics);
        let candidate_root =
            candidate_root(&roots, &metrics, &reasons, CandidateStatus::Quarantined);
        Self {
            candidate_root,
            roots,
            metrics,
            status: CandidateStatus::Quarantined,
            quarantine_reasons: reasons,
        }
    }

    pub fn from_roots(config: &Config, roots: AdmissionRoots, metrics: AdmissionMetrics) -> Self {
        let reasons = admission_reasons(config, &roots, &metrics);
        let status = if reasons.is_empty() {
            CandidateStatus::AdmissibleShadow
        } else if reasons.contains(&QuarantineReason::RootOnlyAdmissionPending) {
            CandidateStatus::Rejected
        } else {
            CandidateStatus::Quarantined
        };
        let candidate_root = candidate_root(&roots, &metrics, &reasons, status);
        Self {
            candidate_root,
            roots,
            metrics,
            status,
            quarantine_reasons: reasons,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "candidate_root": self.candidate_root,
            "roots_root": self.roots.state_root(),
            "metrics_root": self.metrics.state_root(),
            "status": self.status.as_str(),
            "quarantine_root": quarantine_root(&self.quarantine_reasons),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("receipt-candidate", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionCounters {
    pub total_candidates: u64,
    pub admitted_shadow: u64,
    pub quarantined: u64,
    pub rejected: u64,
    pub empty_future_roots: u64,
}

impl AdmissionCounters {
    pub fn from_candidates(candidates: &[ReceiptCandidate]) -> Self {
        let mut counters = Self::default();
        for candidate in candidates {
            counters.total_candidates = counters.total_candidates.saturating_add(1);
            match candidate.status {
                CandidateStatus::AdmissibleShadow => {
                    counters.admitted_shadow = counters.admitted_shadow.saturating_add(1);
                }
                CandidateStatus::Quarantined => {
                    counters.quarantined = counters.quarantined.saturating_add(1);
                }
                CandidateStatus::Rejected => {
                    counters.rejected = counters.rejected.saturating_add(1);
                }
            }
            if candidate.roots.future_receipt_root.is_empty() {
                counters.empty_future_roots = counters.empty_future_roots.saturating_add(1);
            }
        }
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("admission-counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub candidates: Vec<ReceiptCandidate>,
    pub counters: AdmissionCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, candidates: Vec<ReceiptCandidate>) -> Result<Self> {
        if candidates.len() > config.max_candidates {
            return Err(format!(
                "candidate count {} exceeds configured max {}",
                candidates.len(),
                config.max_candidates
            ));
        }
        let counters = AdmissionCounters::from_candidates(&candidates);
        Ok(Self {
            config,
            candidates,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        let candidate_records = self
            .candidates
            .iter()
            .map(ReceiptCandidate::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "candidate_root": merkle_candidates(&self.candidates),
            "counter_root": self.counters.state_root(),
            "operator_command_root": operator_command_root(&self.operator_commands),
            "quarantine_reason_root": all_quarantine_reasons_root(&self.candidates),
            "state_root": self.state_root_without_public_record(),
            "counters": self.counters.public_record(),
            "candidate_records": candidate_records,
            "operator_commands": self.operator_commands
                .iter()
                .map(|command| command.as_str())
                .collect::<Vec<_>>(),
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
                HashPart::Str(&merkle_candidates(&self.candidates)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&operator_command_root(&self.operator_commands)),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let candidates = vec![ReceiptCandidate::empty_quarantined(&config)];
    match State::new(config, candidates) {
        Ok(state) => state,
        Err(reason) => fallback_state(reason),
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn admit_future_receipt_root(
    runtime: &Runtime,
    roots: AdmissionRoots,
    metrics: AdmissionMetrics,
) -> Result<Runtime> {
    let mut candidates = runtime.candidates.clone();
    candidates.push(ReceiptCandidate::from_roots(
        &runtime.config,
        roots,
        metrics,
    ));
    State::new(runtime.config.clone(), candidates)
}

fn admission_reasons(
    config: &Config,
    roots: &AdmissionRoots,
    metrics: &AdmissionMetrics,
) -> Vec<QuarantineReason> {
    let mut reasons = Vec::new();
    if roots.future_receipt_root.is_empty() || !metrics.future_slot {
        reasons.push(QuarantineReason::FutureReceiptSlotEmpty);
    }
    if !metrics.live_receipt {
        reasons.push(QuarantineReason::NotLiveReceipt);
    }
    if !config.heavy_gate_ran {
        reasons.push(QuarantineReason::HeavyGateNotRun);
    }
    if config.require_ml_dsa_authority
        && (roots.ml_dsa_authority_epoch_root.is_empty()
            || metrics.ml_dsa_authority_epoch < config.authority_epoch)
    {
        reasons.push(QuarantineReason::MissingMlDsaAuthorityEpoch);
    }
    if config.require_slh_dsa_authority
        && (roots.slh_dsa_authority_epoch_root.is_empty()
            || metrics.slh_dsa_authority_epoch < config.authority_epoch)
    {
        reasons.push(QuarantineReason::MissingSlhDsaAuthorityEpoch);
    }
    if config.require_pq_quorum
        && (roots.pq_quorum_root.is_empty() || metrics.pq_signer_count < config.min_pq_signers)
    {
        reasons.push(QuarantineReason::PqQuorumMissing);
    }
    if config.require_pq_quorum && metrics.pq_family_count < config.min_pq_families {
        reasons.push(QuarantineReason::PqFamilyCoverageMissing);
    }
    if config.require_reserve_coverage
        && (roots.reserve_coverage_root.is_empty()
            || metrics.reserve_coverage_bps < config.min_reserve_coverage_bps
            || metrics.reserve_buffer_bps < config.min_reserve_buffer_bps)
    {
        reasons.push(QuarantineReason::ReserveCoverageMissing);
    }
    if config.require_privacy_linkage
        && (roots.privacy_linkage_root.is_empty()
            || metrics.linkage_risk_bps > config.max_linkage_risk_bps)
    {
        reasons.push(QuarantineReason::PrivacyLinkageMissing);
    }
    if config.require_metadata_redaction
        && (roots.metadata_redaction_root.is_empty()
            || metrics.redaction_depth < config.min_redaction_depth)
    {
        reasons.push(QuarantineReason::MetadataRedactionMissing);
    }
    if config.require_nullifier_separation
        && (roots.nullifier_separation_root.is_empty() || !metrics.nullifier_domains_separated)
    {
        reasons.push(QuarantineReason::NullifierSeparationMissing);
    }
    if config.require_operator_signoff
        && (roots.operator_signoff_root.is_empty() || !metrics.operator_signed)
    {
        reasons.push(QuarantineReason::OperatorSignoffMissing);
    }
    if !metrics.root_only {
        reasons.push(QuarantineReason::RootOnlyAdmissionPending);
    }
    reasons
}

fn candidate_root(
    roots: &AdmissionRoots,
    metrics: &AdmissionMetrics,
    reasons: &[QuarantineReason],
    status: CandidateStatus,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:candidate"),
        &[
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&quarantine_root(reasons)),
        ],
        32,
    )
}

fn quarantine_root(reasons: &[QuarantineReason]) -> String {
    let leaves = reasons
        .iter()
        .map(|reason| json!({ "reason": reason.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:quarantine-reasons"), &leaves)
}

fn all_quarantine_reasons_root(candidates: &[ReceiptCandidate]) -> String {
    let leaves = candidates
        .iter()
        .flat_map(|candidate| {
            candidate
                .quarantine_reasons
                .iter()
                .map(|reason| json!({ "reason": reason.as_str() }))
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:all-quarantine-reasons"), &leaves)
}

fn merkle_candidates(candidates: &[ReceiptCandidate]) -> String {
    let leaves = candidates
        .iter()
        .map(|candidate| Value::String(candidate.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:candidates"), &leaves)
}

fn operator_command_root(commands: &[OperatorCommand]) -> String {
    let leaves = commands
        .iter()
        .map(|command| json!({ "command": command.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:operator-commands"), &leaves)
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::devnet();
    let reason_root = domain_hash(&format!("{DOMAIN}:fallback"), &[HashPart::Str(&reason)], 32);
    let mut roots = AdmissionRoots::empty();
    roots.future_receipt_root = reason_root;
    let candidate = ReceiptCandidate::from_roots(&config, roots, AdmissionMetrics::default());
    State {
        config,
        counters: AdmissionCounters::from_candidates(&[candidate.clone()]),
        candidates: vec![candidate],
        operator_commands: OperatorCommand::sequence(),
    }
}
