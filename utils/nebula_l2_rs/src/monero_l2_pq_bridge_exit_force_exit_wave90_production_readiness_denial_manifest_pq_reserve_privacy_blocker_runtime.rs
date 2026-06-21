#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;

pub type Runtime = State;
pub type Result<T> = std::result::Result<T, RuntimeError>;

const DOMAIN: &str = "dinero.nebula.wave90.production_readiness_denial_manifest.v1";
const WAVE_89_NO_GO_ARCHIVE_ROOT: &str =
    "wave89.no_go.archive.evidence.root.pq_reserve_privacy_deferred";
const DEFAULT_OPERATOR: &str = "wave90.production.readiness.denial.runtime";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub manifest_id: String,
    pub archive_evidence_root: String,
    pub operator_root: String,
    pub environment: Environment,
    pub strict_denial: bool,
    pub require_pq_signer_epoch: bool,
    pub require_mldsa_authority: bool,
    pub require_slhdsa_authority: bool,
    pub require_reserve_coverage: bool,
    pub require_privacy_linkage_clearance: bool,
    pub require_metadata_redaction: bool,
    pub require_nullifier_separation: bool,
    pub reserve_policy: ReservePolicy,
    pub privacy_policy: PrivacyPolicy,
    pub metadata_policy: MetadataPolicy,
    pub nullifier_policy: NullifierPolicy,
}

impl Config {
    pub fn production_denial() -> Self {
        let archive = Wave89NoGoArchive::default_denial_archive();
        Self {
            manifest_id: "wave90-denial-manifest-pq-reserve-privacy".to_string(),
            archive_evidence_root: archive.archive_root,
            operator_root: stable_root(&[DOMAIN, DEFAULT_OPERATOR]),
            environment: Environment::ProductionReadinessReview,
            strict_denial: true,
            require_pq_signer_epoch: true,
            require_mldsa_authority: true,
            require_slhdsa_authority: true,
            require_reserve_coverage: true,
            require_privacy_linkage_clearance: true,
            require_metadata_redaction: true,
            require_nullifier_separation: true,
            reserve_policy: ReservePolicy::full_default(),
            privacy_policy: PrivacyPolicy::strict_default(),
            metadata_policy: MetadataPolicy::strict_default(),
            nullifier_policy: NullifierPolicy::strict_default(),
        }
    }

    pub fn devnet_denial() -> Self {
        let mut cfg = Self::production_denial();
        cfg.environment = Environment::DevnetMirror;
        cfg.manifest_id = "wave90-devnet-denial-manifest-pq-reserve-privacy".to_string();
        cfg
    }

    pub fn config_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "config",
            self.manifest_id.as_str(),
            self.archive_evidence_root.as_str(),
            self.operator_root.as_str(),
            self.environment.as_str(),
            bool_word(self.strict_denial),
            bool_word(self.require_pq_signer_epoch),
            bool_word(self.require_mldsa_authority),
            bool_word(self.require_slhdsa_authority),
            bool_word(self.require_reserve_coverage),
            bool_word(self.require_privacy_linkage_clearance),
            bool_word(self.require_metadata_redaction),
            bool_word(self.require_nullifier_separation),
            self.reserve_policy.policy_root().as_str(),
            self.privacy_policy.policy_root().as_str(),
            self.metadata_policy.policy_root().as_str(),
            self.nullifier_policy.policy_root().as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Environment {
    DevnetMirror,
    ProductionReadinessReview,
}

impl Environment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DevnetMirror => "devnet_mirror",
            Self::ProductionReadinessReview => "production_readiness_review",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub archive: Wave89NoGoArchive,
    pub evidence: EvidenceSet,
    pub blockers: BlockerSet,
    pub hints: Vec<OperatorActionHint>,
    pub verdict: Verdict,
}

impl State {
    pub fn new(config: Config, archive: Wave89NoGoArchive, evidence: EvidenceSet) -> Result<Self> {
        let blockers = BlockerSet::from_evidence(&config, &archive, &evidence);
        let hints = OperatorActionHint::from_blockers(&blockers);
        let verdict = Verdict::from_blockers(config.strict_denial, &blockers);
        let state = Self {
            config,
            archive,
            evidence,
            blockers,
            hints,
            verdict,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn deny_from_wave89() -> Self {
        let config = Config::production_denial();
        let archive = Wave89NoGoArchive::default_denial_archive();
        let evidence = EvidenceSet::deferred_from_archive(&archive);
        match Self::new(config, archive, evidence) {
            Ok(state) => state,
            Err(err) => Self::minimal_error_state(err),
        }
    }

    pub fn devnet() -> Self {
        let config = Config::devnet_denial();
        let archive = Wave89NoGoArchive::default_denial_archive();
        let evidence = EvidenceSet::deferred_from_archive(&archive);
        match Self::new(config, archive, evidence) {
            Ok(state) => state,
            Err(err) => Self::minimal_error_state(err),
        }
    }

    fn minimal_error_state(err: RuntimeError) -> Self {
        let config = Config::production_denial();
        let archive = Wave89NoGoArchive::default_denial_archive();
        let evidence = EvidenceSet::deferred_from_archive(&archive);
        let blockers = BlockerSet {
            pq: vec![PqDenialCriterion::ArchiveEvidenceInvalid],
            reserve: vec![ReserveCoverageBlocker::CoverageReceiptMissing],
            privacy: vec![PrivacyLinkageBlocker::LinkageAuditMissing],
            metadata: vec![MetadataRedactionBlocker::RedactionReceiptMissing],
            nullifier: vec![NullifierSeparationBlocker::SeparationReceiptMissing],
            archive: vec![ArchiveBlocker::RuntimeValidationFailed(
                err.code().to_string(),
            )],
        };
        let hints = OperatorActionHint::from_blockers(&blockers);
        let verdict = Verdict::from_blockers(true, &blockers);
        Self {
            config,
            archive,
            evidence,
            blockers,
            hints,
            verdict,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.archive_evidence_root != self.archive.archive_root {
            return Err(RuntimeError::ArchiveRootMismatch);
        }
        if self.archive.records.is_empty() {
            return Err(RuntimeError::ArchiveEmpty);
        }
        if self.evidence.archive_root != self.archive.archive_root {
            return Err(RuntimeError::EvidenceRootMismatch);
        }
        if self.config.strict_denial && self.blockers.is_clear() {
            return Err(RuntimeError::StrictDenialWithoutBlockers);
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            manifest_id: self.config.manifest_id.clone(),
            environment: self.config.environment,
            archive_root: self.archive.archive_root.clone(),
            evidence_root: self.evidence.evidence_root(),
            blockers_root: self.blockers.blockers_root(),
            hints_root: hint_root(&self.hints),
            verdict: self.verdict.clone(),
            state_root: self.state_root(),
        }
    }

    pub fn state_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "state",
            self.config.config_root().as_str(),
            self.archive.archive_root.as_str(),
            self.evidence.evidence_root().as_str(),
            self.blockers.blockers_root().as_str(),
            hint_root(&self.hints).as_str(),
            self.verdict.verdict_root().as_str(),
        ])
    }

    pub fn denies_production(&self) -> bool {
        matches!(
            self.verdict.status,
            ReadinessStatus::Denied | ReadinessStatus::Hold
        )
    }

    pub fn blocker_count(&self) -> usize {
        self.blockers.count()
    }

    pub fn action_hint_codes(&self) -> Vec<String> {
        self.hints.iter().map(|hint| hint.code.clone()).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave89NoGoArchive {
    pub archive_root: String,
    pub wave: u16,
    pub records: Vec<ArchiveEvidenceRecord>,
    pub no_go_reason_root: String,
}

impl Wave89NoGoArchive {
    pub fn default_denial_archive() -> Self {
        let records = vec![
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::PqSigner,
                "pq_epoch_deferred",
                "wave89.pq.epoch.receipts.deferred",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::PqAuthority,
                "mldsa_authority_deferred",
                "wave89.pq.mldsa.authority.deferred",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::PqAuthority,
                "slhdsa_authority_deferred",
                "wave89.pq.slhdsa.authority.deferred",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::Reserve,
                "reserve_coverage_missing",
                "wave89.reserve.coverage.receipts.missing",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::PrivacyLinkage,
                "privacy_linkage_unresolved",
                "wave89.privacy.linkage.unresolved",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::MetadataRedaction,
                "metadata_redaction_missing",
                "wave89.metadata.redaction.receipts.missing",
            ),
            ArchiveEvidenceRecord::deferred(
                EvidenceFamily::NullifierSeparation,
                "nullifier_separation_missing",
                "wave89.nullifier.separation.receipts.missing",
            ),
        ];
        let record_root = archive_records_root(&records);
        let archive_root = stable_root(&[
            DOMAIN,
            "wave89_archive",
            WAVE_89_NO_GO_ARCHIVE_ROOT,
            record_root.as_str(),
        ]);
        Self {
            archive_root,
            wave: 89,
            records,
            no_go_reason_root: stable_root(&[
                DOMAIN,
                "no_go_reason",
                "production_readiness_denied",
                "pq_reserve_privacy_deferred",
            ]),
        }
    }

    pub fn family_status(&self, family: EvidenceFamily) -> EvidenceDisposition {
        let mut saw_missing = false;
        let mut saw_deferred = false;
        let mut saw_present = false;
        for record in &self.records {
            if record.family == family {
                match record.disposition {
                    EvidenceDisposition::Present => saw_present = true,
                    EvidenceDisposition::Deferred => saw_deferred = true,
                    EvidenceDisposition::Missing => saw_missing = true,
                }
            }
        }
        if saw_missing {
            EvidenceDisposition::Missing
        } else if saw_deferred {
            EvidenceDisposition::Deferred
        } else if saw_present {
            EvidenceDisposition::Present
        } else {
            EvidenceDisposition::Missing
        }
    }

    pub fn contains_deferred_or_missing(&self, family: EvidenceFamily) -> bool {
        matches!(
            self.family_status(family),
            EvidenceDisposition::Deferred | EvidenceDisposition::Missing
        )
    }

    pub fn record_roots(&self) -> Vec<String> {
        self.records
            .iter()
            .map(ArchiveEvidenceRecord::record_root)
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveEvidenceRecord {
    pub family: EvidenceFamily,
    pub code: String,
    pub evidence_commitment: String,
    pub disposition: EvidenceDisposition,
}

impl ArchiveEvidenceRecord {
    pub fn deferred(family: EvidenceFamily, code: &str, commitment_seed: &str) -> Self {
        Self {
            family,
            code: code.to_string(),
            evidence_commitment: stable_root(&[DOMAIN, "archive_record", commitment_seed]),
            disposition: EvidenceDisposition::Deferred,
        }
    }

    pub fn present(family: EvidenceFamily, code: &str, commitment_seed: &str) -> Self {
        Self {
            family,
            code: code.to_string(),
            evidence_commitment: stable_root(&[DOMAIN, "archive_record", commitment_seed]),
            disposition: EvidenceDisposition::Present,
        }
    }

    pub fn missing(family: EvidenceFamily, code: &str, commitment_seed: &str) -> Self {
        Self {
            family,
            code: code.to_string(),
            evidence_commitment: stable_root(&[DOMAIN, "archive_record", commitment_seed]),
            disposition: EvidenceDisposition::Missing,
        }
    }

    pub fn record_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "archive_record_root",
            self.family.as_str(),
            self.code.as_str(),
            self.evidence_commitment.as_str(),
            self.disposition.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceFamily {
    PqSigner,
    PqAuthority,
    Reserve,
    PrivacyLinkage,
    MetadataRedaction,
    NullifierSeparation,
}

impl EvidenceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqSigner => "pq_signer",
            Self::PqAuthority => "pq_authority",
            Self::Reserve => "reserve",
            Self::PrivacyLinkage => "privacy_linkage",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceDisposition {
    Present,
    Deferred,
    Missing,
}

impl EvidenceDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Deferred => "deferred",
            Self::Missing => "missing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSet {
    pub archive_root: String,
    pub pq_signer: PqSignerEvidence,
    pub authority: PqAuthorityEvidence,
    pub reserve: ReserveEvidence,
    pub privacy: PrivacyEvidence,
    pub metadata: MetadataEvidence,
    pub nullifier: NullifierEvidence,
}

impl EvidenceSet {
    pub fn deferred_from_archive(archive: &Wave89NoGoArchive) -> Self {
        Self {
            archive_root: archive.archive_root.clone(),
            pq_signer: PqSignerEvidence::deferred(archive),
            authority: PqAuthorityEvidence::deferred(archive),
            reserve: ReserveEvidence::deferred(archive),
            privacy: PrivacyEvidence::deferred(archive),
            metadata: MetadataEvidence::deferred(archive),
            nullifier: NullifierEvidence::deferred(archive),
        }
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "evidence_set",
            self.archive_root.as_str(),
            self.pq_signer.evidence_root().as_str(),
            self.authority.evidence_root().as_str(),
            self.reserve.evidence_root().as_str(),
            self.privacy.evidence_root().as_str(),
            self.metadata.evidence_root().as_str(),
            self.nullifier.evidence_root().as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PqSignerEvidence {
    pub signer_epoch_root: Option<String>,
    pub epoch_receipt_root: Option<String>,
    pub signer_rotation_root: Option<String>,
    pub archive_commitment_root: String,
}

impl PqSignerEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            signer_epoch_root: None,
            epoch_receipt_root: None,
            signer_rotation_root: None,
            archive_commitment_root: family_archive_root(archive, EvidenceFamily::PqSigner),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.signer_epoch_root.is_some()
            && self.epoch_receipt_root.is_some()
            && self.signer_rotation_root.is_some()
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "pq_signer_evidence",
            option_root(self.signer_epoch_root.as_deref()).as_str(),
            option_root(self.epoch_receipt_root.as_deref()).as_str(),
            option_root(self.signer_rotation_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PqAuthorityEvidence {
    pub mldsa_authority_root: Option<String>,
    pub slhdsa_authority_root: Option<String>,
    pub mldsa_approval_receipt_root: Option<String>,
    pub slhdsa_approval_receipt_root: Option<String>,
    pub archive_commitment_root: String,
}

impl PqAuthorityEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            mldsa_authority_root: None,
            slhdsa_authority_root: None,
            mldsa_approval_receipt_root: None,
            slhdsa_approval_receipt_root: None,
            archive_commitment_root: family_archive_root(archive, EvidenceFamily::PqAuthority),
        }
    }

    pub fn has_mldsa(&self) -> bool {
        self.mldsa_authority_root.is_some() && self.mldsa_approval_receipt_root.is_some()
    }

    pub fn has_slhdsa(&self) -> bool {
        self.slhdsa_authority_root.is_some() && self.slhdsa_approval_receipt_root.is_some()
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "pq_authority_evidence",
            option_root(self.mldsa_authority_root.as_deref()).as_str(),
            option_root(self.slhdsa_authority_root.as_deref()).as_str(),
            option_root(self.mldsa_approval_receipt_root.as_deref()).as_str(),
            option_root(self.slhdsa_approval_receipt_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReserveEvidence {
    pub asset_coverage_root: Option<String>,
    pub reserve_attestation_root: Option<String>,
    pub liability_snapshot_root: Option<String>,
    pub exit_liquidity_root: Option<String>,
    pub archive_commitment_root: String,
}

impl ReserveEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            asset_coverage_root: None,
            reserve_attestation_root: None,
            liability_snapshot_root: None,
            exit_liquidity_root: None,
            archive_commitment_root: family_archive_root(archive, EvidenceFamily::Reserve),
        }
    }

    pub fn complete_against(&self, policy: &ReservePolicy) -> bool {
        (!policy.require_asset_coverage || self.asset_coverage_root.is_some())
            && (!policy.require_reserve_attestation || self.reserve_attestation_root.is_some())
            && (!policy.require_liability_snapshot || self.liability_snapshot_root.is_some())
            && (!policy.require_exit_liquidity || self.exit_liquidity_root.is_some())
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "reserve_evidence",
            option_root(self.asset_coverage_root.as_deref()).as_str(),
            option_root(self.reserve_attestation_root.as_deref()).as_str(),
            option_root(self.liability_snapshot_root.as_deref()).as_str(),
            option_root(self.exit_liquidity_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivacyEvidence {
    pub linkage_audit_root: Option<String>,
    pub unlinkability_receipt_root: Option<String>,
    pub withdrawal_graph_root: Option<String>,
    pub archive_commitment_root: String,
}

impl PrivacyEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            linkage_audit_root: None,
            unlinkability_receipt_root: None,
            withdrawal_graph_root: None,
            archive_commitment_root: family_archive_root(archive, EvidenceFamily::PrivacyLinkage),
        }
    }

    pub fn complete_against(&self, policy: &PrivacyPolicy) -> bool {
        (!policy.require_linkage_audit || self.linkage_audit_root.is_some())
            && (!policy.require_unlinkability_receipt || self.unlinkability_receipt_root.is_some())
            && (!policy.require_withdrawal_graph_clearance || self.withdrawal_graph_root.is_some())
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "privacy_evidence",
            option_root(self.linkage_audit_root.as_deref()).as_str(),
            option_root(self.unlinkability_receipt_root.as_deref()).as_str(),
            option_root(self.withdrawal_graph_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataEvidence {
    pub redaction_receipt_root: Option<String>,
    pub field_inventory_root: Option<String>,
    pub retention_policy_root: Option<String>,
    pub archive_commitment_root: String,
}

impl MetadataEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            redaction_receipt_root: None,
            field_inventory_root: None,
            retention_policy_root: None,
            archive_commitment_root: family_archive_root(
                archive,
                EvidenceFamily::MetadataRedaction,
            ),
        }
    }

    pub fn complete_against(&self, policy: &MetadataPolicy) -> bool {
        (!policy.require_redaction_receipt || self.redaction_receipt_root.is_some())
            && (!policy.require_field_inventory || self.field_inventory_root.is_some())
            && (!policy.require_retention_policy || self.retention_policy_root.is_some())
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "metadata_evidence",
            option_root(self.redaction_receipt_root.as_deref()).as_str(),
            option_root(self.field_inventory_root.as_deref()).as_str(),
            option_root(self.retention_policy_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullifierEvidence {
    pub separation_receipt_root: Option<String>,
    pub domain_registry_root: Option<String>,
    pub collision_scan_root: Option<String>,
    pub archive_commitment_root: String,
}

impl NullifierEvidence {
    pub fn deferred(archive: &Wave89NoGoArchive) -> Self {
        Self {
            separation_receipt_root: None,
            domain_registry_root: None,
            collision_scan_root: None,
            archive_commitment_root: family_archive_root(
                archive,
                EvidenceFamily::NullifierSeparation,
            ),
        }
    }

    pub fn complete_against(&self, policy: &NullifierPolicy) -> bool {
        (!policy.require_separation_receipt || self.separation_receipt_root.is_some())
            && (!policy.require_domain_registry || self.domain_registry_root.is_some())
            && (!policy.require_collision_scan || self.collision_scan_root.is_some())
    }

    pub fn evidence_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "nullifier_evidence",
            option_root(self.separation_receipt_root.as_deref()).as_str(),
            option_root(self.domain_registry_root.as_deref()).as_str(),
            option_root(self.collision_scan_root.as_deref()).as_str(),
            self.archive_commitment_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservePolicy {
    pub require_asset_coverage: bool,
    pub require_reserve_attestation: bool,
    pub require_liability_snapshot: bool,
    pub require_exit_liquidity: bool,
    pub minimum_coverage_bps: u32,
}

impl ReservePolicy {
    pub fn full_default() -> Self {
        Self {
            require_asset_coverage: true,
            require_reserve_attestation: true,
            require_liability_snapshot: true,
            require_exit_liquidity: true,
            minimum_coverage_bps: 10_000,
        }
    }

    pub fn policy_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "reserve_policy",
            bool_word(self.require_asset_coverage),
            bool_word(self.require_reserve_attestation),
            bool_word(self.require_liability_snapshot),
            bool_word(self.require_exit_liquidity),
            self.minimum_coverage_bps.to_string().as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivacyPolicy {
    pub require_linkage_audit: bool,
    pub require_unlinkability_receipt: bool,
    pub require_withdrawal_graph_clearance: bool,
}

impl PrivacyPolicy {
    pub fn strict_default() -> Self {
        Self {
            require_linkage_audit: true,
            require_unlinkability_receipt: true,
            require_withdrawal_graph_clearance: true,
        }
    }

    pub fn policy_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "privacy_policy",
            bool_word(self.require_linkage_audit),
            bool_word(self.require_unlinkability_receipt),
            bool_word(self.require_withdrawal_graph_clearance),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataPolicy {
    pub require_redaction_receipt: bool,
    pub require_field_inventory: bool,
    pub require_retention_policy: bool,
}

impl MetadataPolicy {
    pub fn strict_default() -> Self {
        Self {
            require_redaction_receipt: true,
            require_field_inventory: true,
            require_retention_policy: true,
        }
    }

    pub fn policy_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "metadata_policy",
            bool_word(self.require_redaction_receipt),
            bool_word(self.require_field_inventory),
            bool_word(self.require_retention_policy),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullifierPolicy {
    pub require_separation_receipt: bool,
    pub require_domain_registry: bool,
    pub require_collision_scan: bool,
}

impl NullifierPolicy {
    pub fn strict_default() -> Self {
        Self {
            require_separation_receipt: true,
            require_domain_registry: true,
            require_collision_scan: true,
        }
    }

    pub fn policy_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "nullifier_policy",
            bool_word(self.require_separation_receipt),
            bool_word(self.require_domain_registry),
            bool_word(self.require_collision_scan),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockerSet {
    pub pq: Vec<PqDenialCriterion>,
    pub reserve: Vec<ReserveCoverageBlocker>,
    pub privacy: Vec<PrivacyLinkageBlocker>,
    pub metadata: Vec<MetadataRedactionBlocker>,
    pub nullifier: Vec<NullifierSeparationBlocker>,
    pub archive: Vec<ArchiveBlocker>,
}

impl BlockerSet {
    pub fn from_evidence(
        config: &Config,
        archive: &Wave89NoGoArchive,
        evidence: &EvidenceSet,
    ) -> Self {
        Self {
            pq: collect_pq_blockers(config, archive, evidence),
            reserve: collect_reserve_blockers(config, archive, evidence),
            privacy: collect_privacy_blockers(config, archive, evidence),
            metadata: collect_metadata_blockers(config, archive, evidence),
            nullifier: collect_nullifier_blockers(config, archive, evidence),
            archive: collect_archive_blockers(config, archive, evidence),
        }
    }

    pub fn is_clear(&self) -> bool {
        self.count() == 0
    }

    pub fn count(&self) -> usize {
        self.pq.len()
            + self.reserve.len()
            + self.privacy.len()
            + self.metadata.len()
            + self.nullifier.len()
            + self.archive.len()
    }

    pub fn blockers_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "blocker_set",
            enum_vec_root("pq", &self.pq).as_str(),
            enum_vec_root("reserve", &self.reserve).as_str(),
            enum_vec_root("privacy", &self.privacy).as_str(),
            enum_vec_root("metadata", &self.metadata).as_str(),
            enum_vec_root("nullifier", &self.nullifier).as_str(),
            enum_vec_root("archive", &self.archive).as_str(),
        ])
    }

    pub fn categories(&self) -> BTreeMap<&'static str, usize> {
        let mut map = BTreeMap::new();
        map.insert("pq", self.pq.len());
        map.insert("reserve", self.reserve.len());
        map.insert("privacy", self.privacy.len());
        map.insert("metadata", self.metadata.len());
        map.insert("nullifier", self.nullifier.len());
        map.insert("archive", self.archive.len());
        map
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PqDenialCriterion {
    SignerEpochMissing,
    SignerEpochDeferred,
    EpochReceiptMissing,
    SignerRotationMissing,
    MldsaAuthorityMissing,
    MldsaApprovalReceiptMissing,
    SlhdsaAuthorityMissing,
    SlhdsaApprovalReceiptMissing,
    ArchiveEvidenceInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReserveCoverageBlocker {
    AssetCoverageRootMissing,
    ReserveAttestationMissing,
    LiabilitySnapshotMissing,
    ExitLiquidityMissing,
    CoverageReceiptMissing,
    ArchiveDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivacyLinkageBlocker {
    LinkageAuditMissing,
    UnlinkabilityReceiptMissing,
    WithdrawalGraphClearanceMissing,
    LinkageArchiveDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataRedactionBlocker {
    RedactionReceiptMissing,
    FieldInventoryMissing,
    RetentionPolicyMissing,
    MetadataArchiveDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NullifierSeparationBlocker {
    SeparationReceiptMissing,
    DomainRegistryMissing,
    CollisionScanMissing,
    NullifierArchiveDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArchiveBlocker {
    ArchiveRootMismatch,
    ArchiveEvidenceMissing,
    NoGoReasonActive,
    RuntimeValidationFailed(String),
}

impl fmt::Display for PqDenialCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SignerEpochMissing => "pq.signer_epoch_missing",
            Self::SignerEpochDeferred => "pq.signer_epoch_deferred",
            Self::EpochReceiptMissing => "pq.epoch_receipt_missing",
            Self::SignerRotationMissing => "pq.signer_rotation_missing",
            Self::MldsaAuthorityMissing => "pq.mldsa_authority_missing",
            Self::MldsaApprovalReceiptMissing => "pq.mldsa_approval_receipt_missing",
            Self::SlhdsaAuthorityMissing => "pq.slhdsa_authority_missing",
            Self::SlhdsaApprovalReceiptMissing => "pq.slhdsa_approval_receipt_missing",
            Self::ArchiveEvidenceInvalid => "pq.archive_evidence_invalid",
        })
    }
}

impl fmt::Display for ReserveCoverageBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AssetCoverageRootMissing => "reserve.asset_coverage_root_missing",
            Self::ReserveAttestationMissing => "reserve.attestation_missing",
            Self::LiabilitySnapshotMissing => "reserve.liability_snapshot_missing",
            Self::ExitLiquidityMissing => "reserve.exit_liquidity_missing",
            Self::CoverageReceiptMissing => "reserve.coverage_receipt_missing",
            Self::ArchiveDeferred => "reserve.archive_deferred",
        })
    }
}

impl fmt::Display for PrivacyLinkageBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::LinkageAuditMissing => "privacy.linkage_audit_missing",
            Self::UnlinkabilityReceiptMissing => "privacy.unlinkability_receipt_missing",
            Self::WithdrawalGraphClearanceMissing => "privacy.withdrawal_graph_clearance_missing",
            Self::LinkageArchiveDeferred => "privacy.linkage_archive_deferred",
        })
    }
}

impl fmt::Display for MetadataRedactionBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::RedactionReceiptMissing => "metadata.redaction_receipt_missing",
            Self::FieldInventoryMissing => "metadata.field_inventory_missing",
            Self::RetentionPolicyMissing => "metadata.retention_policy_missing",
            Self::MetadataArchiveDeferred => "metadata.archive_deferred",
        })
    }
}

impl fmt::Display for NullifierSeparationBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SeparationReceiptMissing => "nullifier.separation_receipt_missing",
            Self::DomainRegistryMissing => "nullifier.domain_registry_missing",
            Self::CollisionScanMissing => "nullifier.collision_scan_missing",
            Self::NullifierArchiveDeferred => "nullifier.archive_deferred",
        })
    }
}

impl fmt::Display for ArchiveBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArchiveRootMismatch => f.write_str("archive.root_mismatch"),
            Self::ArchiveEvidenceMissing => f.write_str("archive.evidence_missing"),
            Self::NoGoReasonActive => f.write_str("archive.no_go_reason_active"),
            Self::RuntimeValidationFailed(code) => {
                f.write_str("archive.runtime_validation_failed.")?;
                f.write_str(code)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorActionHint {
    pub code: String,
    pub severity: ActionSeverity,
    pub category: ActionCategory,
    pub required_root: String,
}

impl OperatorActionHint {
    pub fn new(
        code: &str,
        severity: ActionSeverity,
        category: ActionCategory,
        required_seed: &str,
    ) -> Self {
        Self {
            code: code.to_string(),
            severity,
            category,
            required_root: stable_root(&[DOMAIN, "operator_action_hint", code, required_seed]),
        }
    }

    pub fn from_blockers(blockers: &BlockerSet) -> Vec<Self> {
        let mut hints = Vec::new();
        if !blockers.pq.is_empty() {
            hints.push(Self::new(
                "publish_pq_signer_epoch_and_authority_receipts",
                ActionSeverity::Critical,
                ActionCategory::PqAuthority,
                "pq_epoch_mldsa_slhdsa",
            ));
        }
        if !blockers.reserve.is_empty() {
            hints.push(Self::new(
                "publish_reserve_coverage_and_exit_liquidity_receipts",
                ActionSeverity::Critical,
                ActionCategory::Reserve,
                "reserve_coverage_exit_liquidity",
            ));
        }
        if !blockers.privacy.is_empty() {
            hints.push(Self::new(
                "publish_privacy_linkage_clearance_roots",
                ActionSeverity::Critical,
                ActionCategory::Privacy,
                "privacy_linkage_clearance",
            ));
        }
        if !blockers.metadata.is_empty() {
            hints.push(Self::new(
                "publish_metadata_redaction_receipts",
                ActionSeverity::High,
                ActionCategory::Metadata,
                "metadata_redaction",
            ));
        }
        if !blockers.nullifier.is_empty() {
            hints.push(Self::new(
                "publish_nullifier_separation_receipts",
                ActionSeverity::Critical,
                ActionCategory::Nullifier,
                "nullifier_separation",
            ));
        }
        if !blockers.archive.is_empty() {
            hints.push(Self::new(
                "close_wave89_no_go_archive_findings",
                ActionSeverity::Critical,
                ActionCategory::Archive,
                "wave89_no_go_archive",
            ));
        }
        hints
    }

    pub fn hint_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "hint",
            self.code.as_str(),
            self.severity.as_str(),
            self.category.as_str(),
            self.required_root.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ActionSeverity {
    Critical,
    High,
    Medium,
}

impl ActionSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ActionCategory {
    PqAuthority,
    Reserve,
    Privacy,
    Metadata,
    Nullifier,
    Archive,
}

impl ActionCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqAuthority => "pq_authority",
            Self::Reserve => "reserve",
            Self::Privacy => "privacy",
            Self::Metadata => "metadata",
            Self::Nullifier => "nullifier",
            Self::Archive => "archive",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verdict {
    pub status: ReadinessStatus,
    pub blocker_count: usize,
    pub reason_root: String,
    pub deny_production: bool,
}

impl Verdict {
    pub fn from_blockers(strict_denial: bool, blockers: &BlockerSet) -> Self {
        let blocker_count = blockers.count();
        let status = if blocker_count > 0 {
            ReadinessStatus::Denied
        } else if strict_denial {
            ReadinessStatus::Hold
        } else {
            ReadinessStatus::Ready
        };
        Self {
            status,
            blocker_count,
            reason_root: stable_root(&[
                DOMAIN,
                "verdict_reason",
                status.as_str(),
                blocker_count.to_string().as_str(),
                blockers.blockers_root().as_str(),
            ]),
            deny_production: !matches!(status, ReadinessStatus::Ready),
        }
    }

    pub fn verdict_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "verdict",
            self.status.as_str(),
            self.blocker_count.to_string().as_str(),
            self.reason_root.as_str(),
            bool_word(self.deny_production),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReadinessStatus {
    Ready,
    Hold,
    Denied,
}

impl ReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Hold => "hold",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub manifest_id: String,
    pub environment: Environment,
    pub archive_root: String,
    pub evidence_root: String,
    pub blockers_root: String,
    pub hints_root: String,
    pub verdict: Verdict,
    pub state_root: String,
}

impl PublicRecord {
    pub fn public_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "public_record",
            self.manifest_id.as_str(),
            self.environment.as_str(),
            self.archive_root.as_str(),
            self.evidence_root.as_str(),
            self.blockers_root.as_str(),
            self.hints_root.as_str(),
            self.verdict.verdict_root().as_str(),
            self.state_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    ArchiveRootMismatch,
    EvidenceRootMismatch,
    ArchiveEmpty,
    StrictDenialWithoutBlockers,
}

impl RuntimeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ArchiveRootMismatch => "archive_root_mismatch",
            Self::EvidenceRootMismatch => "evidence_root_mismatch",
            Self::ArchiveEmpty => "archive_empty",
            Self::StrictDenialWithoutBlockers => "strict_denial_without_blockers",
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for RuntimeError {}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    State::deny_from_wave89().public_record()
}

pub fn state_root() -> String {
    State::deny_from_wave89().state_root()
}

pub fn evaluate(
    config: Config,
    archive: Wave89NoGoArchive,
    evidence: EvidenceSet,
) -> Result<State> {
    State::new(config, archive, evidence)
}

pub fn denial_manifest() -> Runtime {
    State::deny_from_wave89()
}

fn collect_pq_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<PqDenialCriterion> {
    let mut blockers = Vec::new();
    if config.require_pq_signer_epoch {
        if archive.contains_deferred_or_missing(EvidenceFamily::PqSigner) {
            blockers.push(PqDenialCriterion::SignerEpochDeferred);
        }
        if evidence.pq_signer.signer_epoch_root.is_none() {
            blockers.push(PqDenialCriterion::SignerEpochMissing);
        }
        if evidence.pq_signer.epoch_receipt_root.is_none() {
            blockers.push(PqDenialCriterion::EpochReceiptMissing);
        }
        if evidence.pq_signer.signer_rotation_root.is_none() {
            blockers.push(PqDenialCriterion::SignerRotationMissing);
        }
    }
    if config.require_mldsa_authority {
        if evidence.authority.mldsa_authority_root.is_none() {
            blockers.push(PqDenialCriterion::MldsaAuthorityMissing);
        }
        if evidence.authority.mldsa_approval_receipt_root.is_none() {
            blockers.push(PqDenialCriterion::MldsaApprovalReceiptMissing);
        }
    }
    if config.require_slhdsa_authority {
        if evidence.authority.slhdsa_authority_root.is_none() {
            blockers.push(PqDenialCriterion::SlhdsaAuthorityMissing);
        }
        if evidence.authority.slhdsa_approval_receipt_root.is_none() {
            blockers.push(PqDenialCriterion::SlhdsaApprovalReceiptMissing);
        }
    }
    blockers
}

fn collect_reserve_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<ReserveCoverageBlocker> {
    let mut blockers = Vec::new();
    if !config.require_reserve_coverage {
        return blockers;
    }
    if archive.contains_deferred_or_missing(EvidenceFamily::Reserve) {
        blockers.push(ReserveCoverageBlocker::ArchiveDeferred);
        blockers.push(ReserveCoverageBlocker::CoverageReceiptMissing);
    }
    if config.reserve_policy.require_asset_coverage
        && evidence.reserve.asset_coverage_root.is_none()
    {
        blockers.push(ReserveCoverageBlocker::AssetCoverageRootMissing);
    }
    if config.reserve_policy.require_reserve_attestation
        && evidence.reserve.reserve_attestation_root.is_none()
    {
        blockers.push(ReserveCoverageBlocker::ReserveAttestationMissing);
    }
    if config.reserve_policy.require_liability_snapshot
        && evidence.reserve.liability_snapshot_root.is_none()
    {
        blockers.push(ReserveCoverageBlocker::LiabilitySnapshotMissing);
    }
    if config.reserve_policy.require_exit_liquidity
        && evidence.reserve.exit_liquidity_root.is_none()
    {
        blockers.push(ReserveCoverageBlocker::ExitLiquidityMissing);
    }
    blockers
}

fn collect_privacy_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<PrivacyLinkageBlocker> {
    let mut blockers = Vec::new();
    if !config.require_privacy_linkage_clearance {
        return blockers;
    }
    if archive.contains_deferred_or_missing(EvidenceFamily::PrivacyLinkage) {
        blockers.push(PrivacyLinkageBlocker::LinkageArchiveDeferred);
    }
    if config.privacy_policy.require_linkage_audit && evidence.privacy.linkage_audit_root.is_none()
    {
        blockers.push(PrivacyLinkageBlocker::LinkageAuditMissing);
    }
    if config.privacy_policy.require_unlinkability_receipt
        && evidence.privacy.unlinkability_receipt_root.is_none()
    {
        blockers.push(PrivacyLinkageBlocker::UnlinkabilityReceiptMissing);
    }
    if config.privacy_policy.require_withdrawal_graph_clearance
        && evidence.privacy.withdrawal_graph_root.is_none()
    {
        blockers.push(PrivacyLinkageBlocker::WithdrawalGraphClearanceMissing);
    }
    blockers
}

fn collect_metadata_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<MetadataRedactionBlocker> {
    let mut blockers = Vec::new();
    if !config.require_metadata_redaction {
        return blockers;
    }
    if archive.contains_deferred_or_missing(EvidenceFamily::MetadataRedaction) {
        blockers.push(MetadataRedactionBlocker::MetadataArchiveDeferred);
    }
    if config.metadata_policy.require_redaction_receipt
        && evidence.metadata.redaction_receipt_root.is_none()
    {
        blockers.push(MetadataRedactionBlocker::RedactionReceiptMissing);
    }
    if config.metadata_policy.require_field_inventory
        && evidence.metadata.field_inventory_root.is_none()
    {
        blockers.push(MetadataRedactionBlocker::FieldInventoryMissing);
    }
    if config.metadata_policy.require_retention_policy
        && evidence.metadata.retention_policy_root.is_none()
    {
        blockers.push(MetadataRedactionBlocker::RetentionPolicyMissing);
    }
    blockers
}

fn collect_nullifier_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<NullifierSeparationBlocker> {
    let mut blockers = Vec::new();
    if !config.require_nullifier_separation {
        return blockers;
    }
    if archive.contains_deferred_or_missing(EvidenceFamily::NullifierSeparation) {
        blockers.push(NullifierSeparationBlocker::NullifierArchiveDeferred);
    }
    if config.nullifier_policy.require_separation_receipt
        && evidence.nullifier.separation_receipt_root.is_none()
    {
        blockers.push(NullifierSeparationBlocker::SeparationReceiptMissing);
    }
    if config.nullifier_policy.require_domain_registry
        && evidence.nullifier.domain_registry_root.is_none()
    {
        blockers.push(NullifierSeparationBlocker::DomainRegistryMissing);
    }
    if config.nullifier_policy.require_collision_scan
        && evidence.nullifier.collision_scan_root.is_none()
    {
        blockers.push(NullifierSeparationBlocker::CollisionScanMissing);
    }
    blockers
}

fn collect_archive_blockers(
    config: &Config,
    archive: &Wave89NoGoArchive,
    evidence: &EvidenceSet,
) -> Vec<ArchiveBlocker> {
    let mut blockers = Vec::new();
    if config.archive_evidence_root != archive.archive_root {
        blockers.push(ArchiveBlocker::ArchiveRootMismatch);
    }
    if evidence.archive_root != archive.archive_root {
        blockers.push(ArchiveBlocker::ArchiveRootMismatch);
    }
    if archive.records.is_empty() {
        blockers.push(ArchiveBlocker::ArchiveEvidenceMissing);
    }
    if archive.no_go_reason_root
        == stable_root(&[
            DOMAIN,
            "no_go_reason",
            "production_readiness_denied",
            "pq_reserve_privacy_deferred",
        ])
    {
        blockers.push(ArchiveBlocker::NoGoReasonActive);
    }
    blockers
}

fn family_archive_root(archive: &Wave89NoGoArchive, family: EvidenceFamily) -> String {
    let mut roots = Vec::new();
    roots.push(DOMAIN.to_string());
    roots.push("family_archive_root".to_string());
    roots.push(family.as_str().to_string());
    for record in &archive.records {
        if record.family == family {
            roots.push(record.record_root());
        }
    }
    stable_root_owned(&roots)
}

fn archive_records_root(records: &[ArchiveEvidenceRecord]) -> String {
    let mut roots = Vec::new();
    roots.push(DOMAIN.to_string());
    roots.push("archive_records".to_string());
    for record in records {
        roots.push(record.record_root());
    }
    stable_root_owned(&roots)
}

fn hint_root(hints: &[OperatorActionHint]) -> String {
    let mut roots = Vec::new();
    roots.push(DOMAIN.to_string());
    roots.push("hints".to_string());
    for hint in hints {
        roots.push(hint.hint_root());
    }
    stable_root_owned(&roots)
}

fn enum_vec_root<T: fmt::Display>(name: &str, values: &[T]) -> String {
    let mut roots = Vec::new();
    roots.push(DOMAIN.to_string());
    roots.push(name.to_string());
    for value in values {
        roots.push(value.to_string());
    }
    stable_root_owned(&roots)
}

fn option_root(value: Option<&str>) -> String {
    match value {
        Some(item) => stable_root(&[DOMAIN, "some", item]),
        None => stable_root(&[DOMAIN, "none"]),
    }
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn stable_root_owned(parts: &[String]) -> String {
    let borrowed: Vec<&str> = parts.iter().map(String::as_str).collect();
    stable_root(&borrowed)
}

fn stable_root(parts: &[&str]) -> String {
    let mut state = HashState::new();
    state.feed(DOMAIN.as_bytes());
    for part in parts {
        state.feed(&(part.len() as u64).to_le_bytes());
        state.feed(part.as_bytes());
    }
    state.finish_hex()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HashState {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl HashState {
    fn new() -> Self {
        Self {
            a: 0x243f_6a88_85a3_08d3,
            b: 0x1319_8a2e_0370_7344,
            c: 0xa409_3822_299f_31d0,
            d: 0x082e_fa98_ec4e_6c89,
        }
    }

    fn feed(&mut self, bytes: &[u8]) {
        for (index, byte) in bytes.iter().enumerate() {
            let lane = (*byte as u64) + ((index as u64) << 8) + 0x9e37_79b9_7f4a_7c15;
            self.a = self.a.rotate_left(5) ^ lane;
            self.b = self.b.wrapping_add(self.a ^ lane.rotate_left(17));
            self.c ^= self.b.rotate_left(23).wrapping_add(lane);
            self.d = self
                .d
                .wrapping_mul(0x1000_0000_01b3)
                .wrapping_add(self.c ^ lane.rotate_left(31));
        }
        self.a ^= (bytes.len() as u64).rotate_left(11);
        self.b = self.b.wrapping_add(self.d.rotate_left(7));
        self.c = self.c.wrapping_add(self.a.rotate_left(19));
        self.d ^= self.b.rotate_left(29);
    }

    fn finish_hex(self) -> String {
        let mut a = self.a ^ self.c.rotate_left(13);
        let mut b = self.b ^ self.d.rotate_left(17);
        let mut c = self.c.wrapping_add(a.rotate_left(31));
        let mut d = self.d.wrapping_add(b.rotate_left(37));
        for round in 0..8_u64 {
            a = a.wrapping_add(0x9e37_79b9_7f4a_7c15 ^ round).rotate_left(7) ^ d;
            b = b.wrapping_add(a).rotate_left(11) ^ c;
            c = c.wrapping_add(b).rotate_left(19) ^ a;
            d = d.wrapping_add(c).rotate_left(23) ^ b;
        }
        format!("{a:016x}{b:016x}{c:016x}{d:016x}")
    }
}
