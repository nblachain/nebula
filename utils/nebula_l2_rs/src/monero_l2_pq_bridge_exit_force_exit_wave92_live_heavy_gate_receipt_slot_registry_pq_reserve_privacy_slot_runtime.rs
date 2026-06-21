#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub type Runtime = State;
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave92-live-heavy-gate-receipt-slot-registry-pq-reserve-privacy-v1";
pub const DOMAIN: &str =
    "dinero.nebula.wave92.live_heavy_gate.receipt_slot_registry.pq_reserve_privacy";
pub const HASH_SUITE: &str = "std-only-deterministic-root-v1";
pub const DEFAULT_WAVE: u64 = 92;
pub const SOURCE_WAVE: u64 = 91;
pub const DEFAULT_PLAN_ROOT: &str =
    "wave91.live_heavy_gate.execution_plan.pq_reserve_privacy.receipt.root";
pub const DEFAULT_FINAL_TRANSCRIPT_ROOT: &str =
    "wave91.live_heavy_gate.execution_plan.final_transcript.root";
pub const DEFAULT_MIN_AUTHORITY_EPOCH: u64 = 92;
pub const DEFAULT_MIN_PQ_SIGNER_QUORUM: u16 = 5;
pub const DEFAULT_MIN_PQ_FAMILIES: u16 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_000;
pub const DEFAULT_MIN_RESERVE_BUFFER_BPS: u64 = 1_500;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_REDACTION_DEPTH: u16 = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub protocol_version: String,
    pub wave: u64,
    pub source_wave: u64,
    pub lane: RegistryLane,
    pub mode: RegistryMode,
    pub plan_root: String,
    pub final_transcript_root: String,
    pub min_authority_epoch: u64,
    pub min_pq_signer_quorum: u16,
    pub min_pq_families: u16,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub min_redaction_depth: u16,
    pub require_ml_dsa_authority: bool,
    pub require_slh_dsa_authority: bool,
    pub require_pq_quorum: bool,
    pub require_reserve_coverage: bool,
    pub require_privacy_linkage_review: bool,
    pub require_metadata_redaction: bool,
    pub require_nullifier_separation: bool,
    pub require_operator_signoff: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: SOURCE_WAVE,
            lane: RegistryLane::PqReservePrivacy,
            mode: RegistryMode::DevnetLiveShadow,
            plan_root: DEFAULT_PLAN_ROOT.to_string(),
            final_transcript_root: DEFAULT_FINAL_TRANSCRIPT_ROOT.to_string(),
            min_authority_epoch: DEFAULT_MIN_AUTHORITY_EPOCH,
            min_pq_signer_quorum: DEFAULT_MIN_PQ_SIGNER_QUORUM,
            min_pq_families: DEFAULT_MIN_PQ_FAMILIES,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            min_reserve_buffer_bps: DEFAULT_MIN_RESERVE_BUFFER_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            min_redaction_depth: DEFAULT_MIN_REDACTION_DEPTH,
            require_ml_dsa_authority: true,
            require_slh_dsa_authority: true,
            require_pq_quorum: true,
            require_reserve_coverage: true,
            require_privacy_linkage_review: true,
            require_metadata_redaction: true,
            require_nullifier_separation: true,
            require_operator_signoff: true,
            fail_closed: true,
        }
    }

    pub fn root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "config",
            self.protocol_version.as_str(),
            &self.wave.to_string(),
            &self.source_wave.to_string(),
            self.lane.as_str(),
            self.mode.as_str(),
            self.plan_root.as_str(),
            self.final_transcript_root.as_str(),
            &self.min_authority_epoch.to_string(),
            &self.min_pq_signer_quorum.to_string(),
            &self.min_pq_families.to_string(),
            &self.min_reserve_coverage_bps.to_string(),
            &self.min_reserve_buffer_bps.to_string(),
            &self.max_linkage_risk_bps.to_string(),
            &self.min_redaction_depth.to_string(),
            bool_word(self.require_ml_dsa_authority),
            bool_word(self.require_slh_dsa_authority),
            bool_word(self.require_pq_quorum),
            bool_word(self.require_reserve_coverage),
            bool_word(self.require_privacy_linkage_review),
            bool_word(self.require_metadata_redaction),
            bool_word(self.require_nullifier_separation),
            bool_word(self.require_operator_signoff),
            bool_word(self.fail_closed),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RegistryLane {
    PqReservePrivacy,
}

impl RegistryLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RegistryMode {
    DevnetLiveShadow,
    OperatorDryRun,
    ProductionCandidate,
}

impl RegistryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DevnetLiveShadow => "devnet_live_shadow",
            Self::OperatorDryRun => "operator_dry_run",
            Self::ProductionCandidate => "production_candidate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReceiptSlotKind {
    MldsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqSignerQuorum,
    ReserveCoverage,
    PrivacyLinkageReview,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl ReceiptSlotKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::MldsaAuthorityEpoch,
            Self::SlhDsaAuthorityEpoch,
            Self::PqSignerQuorum,
            Self::ReserveCoverage,
            Self::PrivacyLinkageReview,
            Self::MetadataRedaction,
            Self::NullifierSeparation,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MldsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqSignerQuorum => "pq_signer_quorum",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyLinkageReview => "privacy_linkage_review",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn command_hint(self) -> OperatorCommandHint {
        match self {
            Self::MldsaAuthorityEpoch => OperatorCommandHint::AttachMldsaAuthorityRoot,
            Self::SlhDsaAuthorityEpoch => OperatorCommandHint::AttachSlhDsaAuthorityRoot,
            Self::PqSignerQuorum => OperatorCommandHint::AttachPqQuorumRoot,
            Self::ReserveCoverage => OperatorCommandHint::AttachReserveCoverageRoot,
            Self::PrivacyLinkageReview => OperatorCommandHint::AttachPrivacyReviewRoot,
            Self::MetadataRedaction => OperatorCommandHint::AttachMetadataRedactionRoot,
            Self::NullifierSeparation => OperatorCommandHint::AttachNullifierSeparationRoot,
            Self::OperatorSignoff => OperatorCommandHint::AttachOperatorSignoffRoot,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlotStatus {
    Empty,
    RootAttached,
    Clearable,
    Blocked,
}

impl SlotStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::RootAttached => "root_attached",
            Self::Clearable => "clearable",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportRule {
    LiveAcceptedRootOnly,
    RootsOnlyPublicRecord,
    SourcePlanRootMustMatch,
    FutureReceiptSlotOnly,
    NoRawIdentityMaterial,
    NoRawTransactionMaterial,
    NoPayloadMaterial,
    FailClosedOnGap,
}

impl ImportRule {
    pub fn all() -> Vec<Self> {
        vec![
            Self::LiveAcceptedRootOnly,
            Self::RootsOnlyPublicRecord,
            Self::SourcePlanRootMustMatch,
            Self::FutureReceiptSlotOnly,
            Self::NoRawIdentityMaterial,
            Self::NoRawTransactionMaterial,
            Self::NoPayloadMaterial,
            Self::FailClosedOnGap,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveAcceptedRootOnly => "live_accepted_root_only",
            Self::RootsOnlyPublicRecord => "roots_only_public_record",
            Self::SourcePlanRootMustMatch => "source_plan_root_must_match",
            Self::FutureReceiptSlotOnly => "future_receipt_slot_only",
            Self::NoRawIdentityMaterial => "no_raw_identity_material",
            Self::NoRawTransactionMaterial => "no_raw_transaction_material",
            Self::NoPayloadMaterial => "no_payload_material",
            Self::FailClosedOnGap => "fail_closed_on_gap",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlotBlocker {
    EmptySlot,
    RootNotLiveAccepted,
    PlanRootMismatch,
    FinalTranscriptMismatch,
    AuthorityEpochTooLow,
    PqQuorumTooSmall,
    PqFamilyCoverageTooSmall,
    ReserveCoverageTooLow,
    ReserveBufferTooLow,
    LinkageRiskTooHigh,
    RedactionDepthTooShallow,
    NullifierDomainsNotSeparated,
    OperatorSignoffMissing,
    PrivacyReviewMissing,
    ImportRuleMissing,
}

impl SlotBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptySlot => "empty_slot",
            Self::RootNotLiveAccepted => "root_not_live_accepted",
            Self::PlanRootMismatch => "plan_root_mismatch",
            Self::FinalTranscriptMismatch => "final_transcript_mismatch",
            Self::AuthorityEpochTooLow => "authority_epoch_too_low",
            Self::PqQuorumTooSmall => "pq_quorum_too_small",
            Self::PqFamilyCoverageTooSmall => "pq_family_coverage_too_small",
            Self::ReserveCoverageTooLow => "reserve_coverage_too_low",
            Self::ReserveBufferTooLow => "reserve_buffer_too_low",
            Self::LinkageRiskTooHigh => "linkage_risk_too_high",
            Self::RedactionDepthTooShallow => "redaction_depth_too_shallow",
            Self::NullifierDomainsNotSeparated => "nullifier_domains_not_separated",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::PrivacyReviewMissing => "privacy_review_missing",
            Self::ImportRuleMissing => "import_rule_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryVerdict {
    FailClosedEmpty,
    Blocked,
    Clearable,
}

impl RegistryVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedEmpty => "fail_closed_empty",
            Self::Blocked => "blocked",
            Self::Clearable => "clearable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorCommandHint {
    ImportWave91PlanRoot,
    AttachMldsaAuthorityRoot,
    AttachSlhDsaAuthorityRoot,
    AttachPqQuorumRoot,
    AttachReserveCoverageRoot,
    AttachPrivacyReviewRoot,
    AttachMetadataRedactionRoot,
    AttachNullifierSeparationRoot,
    AttachOperatorSignoffRoot,
    PublishRootsOnlyRegistryRecord,
}

impl OperatorCommandHint {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave91PlanRoot,
            Self::AttachMldsaAuthorityRoot,
            Self::AttachSlhDsaAuthorityRoot,
            Self::AttachPqQuorumRoot,
            Self::AttachReserveCoverageRoot,
            Self::AttachPrivacyReviewRoot,
            Self::AttachMetadataRedactionRoot,
            Self::AttachNullifierSeparationRoot,
            Self::AttachOperatorSignoffRoot,
            Self::PublishRootsOnlyRegistryRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave91PlanRoot => "import_wave91_plan_root",
            Self::AttachMldsaAuthorityRoot => "attach_ml_dsa_authority_root",
            Self::AttachSlhDsaAuthorityRoot => "attach_slh_dsa_authority_root",
            Self::AttachPqQuorumRoot => "attach_pq_quorum_root",
            Self::AttachReserveCoverageRoot => "attach_reserve_coverage_root",
            Self::AttachPrivacyReviewRoot => "attach_privacy_review_root",
            Self::AttachMetadataRedactionRoot => "attach_metadata_redaction_root",
            Self::AttachNullifierSeparationRoot => "attach_nullifier_separation_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
            Self::PublishRootsOnlyRegistryRecord => "publish_roots_only_registry_record",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave91PlanRootIntake {
    pub plan_root: String,
    pub final_transcript_root: String,
    pub source_wave: u64,
    pub lane: RegistryLane,
    pub imported: bool,
    pub import_rules: Vec<ImportRule>,
}

impl Wave91PlanRootIntake {
    pub fn from_config(config: &Config) -> Self {
        Self {
            plan_root: config.plan_root.clone(),
            final_transcript_root: config.final_transcript_root.clone(),
            source_wave: config.source_wave,
            lane: config.lane,
            imported: true,
            import_rules: ImportRule::all(),
        }
    }

    pub fn blockers(&self, config: &Config) -> Vec<SlotBlocker> {
        let mut blockers = Vec::new();
        if !self.imported {
            blockers.push(SlotBlocker::EmptySlot);
        }
        if self.plan_root != config.plan_root {
            blockers.push(SlotBlocker::PlanRootMismatch);
        }
        if self.final_transcript_root != config.final_transcript_root {
            blockers.push(SlotBlocker::FinalTranscriptMismatch);
        }
        for rule in ImportRule::all() {
            if !self.import_rules.contains(&rule) {
                blockers.push(SlotBlocker::ImportRuleMissing);
                break;
            }
        }
        blockers
    }

    pub fn root(&self) -> String {
        let mut parts = vec![
            DOMAIN.to_string(),
            "wave91_plan_root_intake".to_string(),
            self.plan_root.clone(),
            self.final_transcript_root.clone(),
            self.source_wave.to_string(),
            self.lane.as_str().to_string(),
            bool_word(self.imported).to_string(),
        ];
        parts.extend(
            self.import_rules
                .iter()
                .map(|rule| rule.as_str().to_string()),
        );
        stable_owned_root(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptEvidence {
    pub receipt_root: String,
    pub accepted_root: String,
    pub source_plan_root: String,
    pub final_transcript_root: String,
    pub live_accepted: bool,
    pub authority_epoch: Option<u64>,
    pub signer_quorum: Option<u16>,
    pub pq_family_count: Option<u16>,
    pub reserve_coverage_bps: Option<u64>,
    pub reserve_buffer_bps: Option<u64>,
    pub linkage_risk_bps: Option<u64>,
    pub redaction_depth: Option<u16>,
    pub nullifier_domains_separated: Option<bool>,
    pub privacy_review_complete: Option<bool>,
    pub operator_signed: Option<bool>,
}

impl ReceiptEvidence {
    pub fn live_root(
        receipt_root: &str,
        accepted_root: &str,
        source_plan_root: &str,
        final_transcript_root: &str,
    ) -> Self {
        Self {
            receipt_root: receipt_root.to_string(),
            accepted_root: accepted_root.to_string(),
            source_plan_root: source_plan_root.to_string(),
            final_transcript_root: final_transcript_root.to_string(),
            live_accepted: true,
            authority_epoch: None,
            signer_quorum: None,
            pq_family_count: None,
            reserve_coverage_bps: None,
            reserve_buffer_bps: None,
            linkage_risk_bps: None,
            redaction_depth: None,
            nullifier_domains_separated: None,
            privacy_review_complete: None,
            operator_signed: None,
        }
    }

    pub fn with_authority_epoch(mut self, epoch: u64) -> Self {
        self.authority_epoch = Some(epoch);
        self
    }

    pub fn with_pq_quorum(mut self, quorum: u16, families: u16) -> Self {
        self.signer_quorum = Some(quorum);
        self.pq_family_count = Some(families);
        self
    }

    pub fn with_reserve(mut self, coverage_bps: u64, buffer_bps: u64) -> Self {
        self.reserve_coverage_bps = Some(coverage_bps);
        self.reserve_buffer_bps = Some(buffer_bps);
        self
    }

    pub fn with_privacy_review(mut self, linkage_risk_bps: u64) -> Self {
        self.linkage_risk_bps = Some(linkage_risk_bps);
        self.privacy_review_complete = Some(true);
        self
    }

    pub fn with_redaction_depth(mut self, depth: u16) -> Self {
        self.redaction_depth = Some(depth);
        self
    }

    pub fn with_nullifier_separation(mut self, separated: bool) -> Self {
        self.nullifier_domains_separated = Some(separated);
        self
    }

    pub fn with_operator_signed(mut self, signed: bool) -> Self {
        self.operator_signed = Some(signed);
        self
    }

    pub fn root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "accepted_receipt_placeholder",
            self.receipt_root.as_str(),
            self.accepted_root.as_str(),
            self.source_plan_root.as_str(),
            self.final_transcript_root.as_str(),
            bool_word(self.live_accepted),
            &option_u64(self.authority_epoch),
            &option_u16(self.signer_quorum),
            &option_u16(self.pq_family_count),
            &option_u64(self.reserve_coverage_bps),
            &option_u64(self.reserve_buffer_bps),
            &option_u64(self.linkage_risk_bps),
            &option_u16(self.redaction_depth),
            &option_bool(self.nullifier_domains_separated),
            &option_bool(self.privacy_review_complete),
            &option_bool(self.operator_signed),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptSlot {
    pub kind: ReceiptSlotKind,
    pub required: bool,
    pub status: SlotStatus,
    pub evidence: Option<ReceiptEvidence>,
    pub blockers: Vec<SlotBlocker>,
    pub command_hint: OperatorCommandHint,
}

impl ReceiptSlot {
    pub fn empty(kind: ReceiptSlotKind) -> Self {
        Self {
            kind,
            required: true,
            status: SlotStatus::Empty,
            evidence: None,
            blockers: vec![SlotBlocker::EmptySlot],
            command_hint: kind.command_hint(),
        }
    }

    pub fn attach(&self, evidence: ReceiptEvidence, config: &Config) -> Self {
        let blockers = Self::evaluate(self.kind, &evidence, config);
        let status = if blockers.is_empty() {
            SlotStatus::Clearable
        } else if evidence.live_accepted {
            SlotStatus::RootAttached
        } else {
            SlotStatus::Blocked
        };
        Self {
            kind: self.kind,
            required: self.required,
            status,
            evidence: Some(evidence),
            blockers,
            command_hint: self.command_hint,
        }
    }

    fn evaluate(
        kind: ReceiptSlotKind,
        evidence: &ReceiptEvidence,
        config: &Config,
    ) -> Vec<SlotBlocker> {
        let mut blockers = Vec::new();
        if !evidence.live_accepted {
            blockers.push(SlotBlocker::RootNotLiveAccepted);
        }
        if evidence.source_plan_root != config.plan_root {
            blockers.push(SlotBlocker::PlanRootMismatch);
        }
        if evidence.final_transcript_root != config.final_transcript_root {
            blockers.push(SlotBlocker::FinalTranscriptMismatch);
        }
        match kind {
            ReceiptSlotKind::MldsaAuthorityEpoch | ReceiptSlotKind::SlhDsaAuthorityEpoch => {
                if u64_value_or_zero(evidence.authority_epoch) < config.min_authority_epoch {
                    blockers.push(SlotBlocker::AuthorityEpochTooLow);
                }
            }
            ReceiptSlotKind::PqSignerQuorum => {
                if u16_value_or_zero(evidence.signer_quorum) < config.min_pq_signer_quorum {
                    blockers.push(SlotBlocker::PqQuorumTooSmall);
                }
                if u16_value_or_zero(evidence.pq_family_count) < config.min_pq_families {
                    blockers.push(SlotBlocker::PqFamilyCoverageTooSmall);
                }
            }
            ReceiptSlotKind::ReserveCoverage => {
                if u64_value_or_zero(evidence.reserve_coverage_bps)
                    < config.min_reserve_coverage_bps
                {
                    blockers.push(SlotBlocker::ReserveCoverageTooLow);
                }
                if u64_value_or_zero(evidence.reserve_buffer_bps) < config.min_reserve_buffer_bps {
                    blockers.push(SlotBlocker::ReserveBufferTooLow);
                }
            }
            ReceiptSlotKind::PrivacyLinkageReview => {
                if !bool_value_or_false(evidence.privacy_review_complete) {
                    blockers.push(SlotBlocker::PrivacyReviewMissing);
                }
                match evidence.linkage_risk_bps {
                    Some(risk) if risk <= config.max_linkage_risk_bps => {}
                    _ => blockers.push(SlotBlocker::LinkageRiskTooHigh),
                }
            }
            ReceiptSlotKind::MetadataRedaction => {
                if u16_value_or_zero(evidence.redaction_depth) < config.min_redaction_depth {
                    blockers.push(SlotBlocker::RedactionDepthTooShallow);
                }
            }
            ReceiptSlotKind::NullifierSeparation => {
                if !bool_value_or_false(evidence.nullifier_domains_separated) {
                    blockers.push(SlotBlocker::NullifierDomainsNotSeparated);
                }
            }
            ReceiptSlotKind::OperatorSignoff => {
                if !bool_value_or_false(evidence.operator_signed) {
                    blockers.push(SlotBlocker::OperatorSignoffMissing);
                }
            }
        }
        blockers
    }

    pub fn clearable(&self) -> bool {
        self.status == SlotStatus::Clearable && self.blockers.is_empty()
    }

    pub fn root(&self) -> String {
        let evidence_root = match &self.evidence {
            Some(evidence) => evidence.root(),
            None => "empty".to_string(),
        };
        stable_root(&[
            DOMAIN,
            "receipt_slot",
            self.kind.as_str(),
            bool_word(self.required),
            self.status.as_str(),
            self.command_hint.as_str(),
            evidence_root.as_str(),
            &blocker_root(&self.blockers),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotRegistry {
    pub slots: BTreeMap<ReceiptSlotKind, ReceiptSlot>,
}

impl SlotRegistry {
    pub fn empty() -> Self {
        let slots = ReceiptSlotKind::all()
            .into_iter()
            .map(|kind| (kind, ReceiptSlot::empty(kind)))
            .collect();
        Self { slots }
    }

    pub fn attach_receipt(
        &mut self,
        kind: ReceiptSlotKind,
        evidence: ReceiptEvidence,
        config: &Config,
    ) -> Result<()> {
        let current = self
            .slots
            .get(&kind)
            .cloned()
            .ok_or(RuntimeError::MissingSlot(kind))?;
        self.slots.insert(kind, current.attach(evidence, config));
        Ok(())
    }

    pub fn slot(&self, kind: ReceiptSlotKind) -> Result<&ReceiptSlot> {
        self.slots.get(&kind).ok_or(RuntimeError::MissingSlot(kind))
    }

    pub fn blockers(&self) -> Vec<SlotBlocker> {
        let mut blockers = Vec::new();
        for slot in self.slots.values() {
            blockers.extend(slot.blockers.iter().copied());
        }
        blockers
    }

    pub fn clearable_slot_count(&self) -> usize {
        self.slots.values().filter(|slot| slot.clearable()).count()
    }

    pub fn all_clearable(&self) -> bool {
        self.slots.values().all(ReceiptSlot::clearable)
    }

    pub fn empty_slot_count(&self) -> usize {
        self.slots
            .values()
            .filter(|slot| slot.status == SlotStatus::Empty)
            .count()
    }

    pub fn root(&self) -> String {
        let mut parts = vec![DOMAIN.to_string(), "slot_registry".to_string()];
        for (kind, slot) in &self.slots {
            parts.push(kind.as_str().to_string());
            parts.push(slot.root());
        }
        stable_owned_root(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryDecision {
    pub verdict: RegistryVerdict,
    pub blockers: Vec<SlotBlocker>,
    pub clearable_slots: usize,
    pub empty_slots: usize,
    pub decision_root: String,
}

impl RegistryDecision {
    pub fn from_parts(
        config: &Config,
        intake: &Wave91PlanRootIntake,
        registry: &SlotRegistry,
    ) -> Self {
        let mut blockers = intake.blockers(config);
        blockers.extend(registry.blockers());
        dedupe_blockers(&mut blockers);
        let empty_slots = registry.empty_slot_count();
        let verdict = if config.fail_closed && empty_slots > 0 {
            RegistryVerdict::FailClosedEmpty
        } else if blockers.is_empty() && registry.all_clearable() {
            RegistryVerdict::Clearable
        } else {
            RegistryVerdict::Blocked
        };
        let clearable_slots = registry.clearable_slot_count();
        let decision_root = stable_root(&[
            DOMAIN,
            "registry_decision",
            verdict.as_str(),
            &clearable_slots.to_string(),
            &empty_slots.to_string(),
            &blocker_root(&blockers),
            &config.root(),
            &intake.root(),
            &registry.root(),
        ]);
        Self {
            verdict,
            blockers,
            clearable_slots,
            empty_slots,
            decision_root,
        }
    }

    pub fn clearable(&self) -> bool {
        self.verdict == RegistryVerdict::Clearable && self.blockers.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub plan_intake: Wave91PlanRootIntake,
    pub registry: SlotRegistry,
    pub decision: RegistryDecision,
    pub operator_commands: Vec<OperatorCommandHint>,
}

impl State {
    pub fn new(
        config: Config,
        plan_intake: Wave91PlanRootIntake,
        registry: SlotRegistry,
        operator_commands: Vec<OperatorCommandHint>,
    ) -> Result<Self> {
        let decision = RegistryDecision::from_parts(&config, &plan_intake, &registry);
        let state = Self {
            config,
            plan_intake,
            registry,
            decision,
            operator_commands,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let plan_intake = Wave91PlanRootIntake::from_config(&config);
        let registry = SlotRegistry::empty();
        let operator_commands = OperatorCommandHint::sequence();
        match Self::new(config, plan_intake, registry, operator_commands) {
            Ok(state) => state,
            Err(err) => Self::closed_state(err),
        }
    }

    fn closed_state(err: RuntimeError) -> Self {
        let config = Config::devnet();
        let mut plan_intake = Wave91PlanRootIntake::from_config(&config);
        plan_intake.imported = false;
        let registry = SlotRegistry::empty();
        let mut decision = RegistryDecision::from_parts(&config, &plan_intake, &registry);
        decision.blockers.push(err.to_blocker());
        dedupe_blockers(&mut decision.blockers);
        decision.verdict = RegistryVerdict::FailClosedEmpty;
        decision.decision_root = stable_root(&[
            DOMAIN,
            "closed_state",
            decision.verdict.as_str(),
            &blocker_root(&decision.blockers),
        ]);
        Self {
            config,
            plan_intake,
            registry,
            decision,
            operator_commands: OperatorCommandHint::sequence(),
        }
    }

    pub fn attach_receipt(&self, kind: ReceiptSlotKind, evidence: ReceiptEvidence) -> Result<Self> {
        let mut next_registry = self.registry.clone();
        next_registry.attach_receipt(kind, evidence, &self.config)?;
        Self::new(
            self.config.clone(),
            self.plan_intake.clone(),
            next_registry,
            self.operator_commands.clone(),
        )
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.protocol_version != PROTOCOL_VERSION {
            return Err(RuntimeError::ProtocolVersionMismatch);
        }
        if self.config.wave != DEFAULT_WAVE {
            return Err(RuntimeError::WaveMismatch);
        }
        if self.config.source_wave != SOURCE_WAVE {
            return Err(RuntimeError::SourceWaveMismatch);
        }
        if self.config.lane != RegistryLane::PqReservePrivacy {
            return Err(RuntimeError::LaneMismatch);
        }
        let seen: BTreeSet<ReceiptSlotKind> = self.registry.slots.keys().copied().collect();
        for kind in ReceiptSlotKind::all() {
            if !seen.contains(&kind) {
                return Err(RuntimeError::MissingSlot(kind));
            }
        }
        Ok(())
    }

    pub fn state_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "state",
            &self.config.root(),
            &self.plan_intake.root(),
            &self.registry.root(),
            &self.decision.decision_root,
            &command_root(&self.operator_commands),
        ])
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord::from_state(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub protocol_version: String,
    pub wave: u64,
    pub source_wave: u64,
    pub lane: String,
    pub mode: String,
    pub config_root: String,
    pub plan_intake_root: String,
    pub registry_root: String,
    pub decision_root: String,
    pub state_root: String,
    pub verdict: String,
    pub clearable_slots: usize,
    pub empty_slots: usize,
    pub slot_roots: BTreeMap<String, String>,
    pub attached_accepted_roots: BTreeMap<String, String>,
    pub blockers: Vec<String>,
    pub operator_command_hints: Vec<String>,
    pub import_rules: Vec<String>,
}

impl PublicRecord {
    pub fn from_state(state: &State) -> Self {
        let mut slot_roots = BTreeMap::new();
        let mut attached_accepted_roots = BTreeMap::new();
        for (kind, slot) in &state.registry.slots {
            slot_roots.insert(kind.as_str().to_string(), slot.root());
            if let Some(evidence) = &slot.evidence {
                attached_accepted_roots
                    .insert(kind.as_str().to_string(), evidence.accepted_root.clone());
            }
        }
        Self {
            protocol_version: state.config.protocol_version.clone(),
            wave: state.config.wave,
            source_wave: state.config.source_wave,
            lane: state.config.lane.as_str().to_string(),
            mode: state.config.mode.as_str().to_string(),
            config_root: state.config.root(),
            plan_intake_root: state.plan_intake.root(),
            registry_root: state.registry.root(),
            decision_root: state.decision.decision_root.clone(),
            state_root: state.state_root(),
            verdict: state.decision.verdict.as_str().to_string(),
            clearable_slots: state.decision.clearable_slots,
            empty_slots: state.decision.empty_slots,
            slot_roots,
            attached_accepted_roots,
            blockers: state
                .decision
                .blockers
                .iter()
                .map(|blocker| blocker.as_str().to_string())
                .collect(),
            operator_command_hints: state
                .operator_commands
                .iter()
                .map(|hint| hint.as_str().to_string())
                .collect(),
            import_rules: state
                .plan_intake
                .import_rules
                .iter()
                .map(|rule| rule.as_str().to_string())
                .collect(),
        }
    }

    pub fn root(&self) -> String {
        let mut parts = vec![
            DOMAIN.to_string(),
            "public_record".to_string(),
            self.protocol_version.clone(),
            self.wave.to_string(),
            self.source_wave.to_string(),
            self.lane.clone(),
            self.mode.clone(),
            self.config_root.clone(),
            self.plan_intake_root.clone(),
            self.registry_root.clone(),
            self.decision_root.clone(),
            self.state_root.clone(),
            self.verdict.clone(),
            self.clearable_slots.to_string(),
            self.empty_slots.to_string(),
        ];
        for (kind, root) in &self.slot_roots {
            parts.push(kind.clone());
            parts.push(root.clone());
        }
        for (kind, root) in &self.attached_accepted_roots {
            parts.push(kind.clone());
            parts.push(root.clone());
        }
        parts.extend(self.blockers.iter().cloned());
        parts.extend(self.operator_command_hints.iter().cloned());
        parts.extend(self.import_rules.iter().cloned());
        stable_owned_root(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    ProtocolVersionMismatch,
    WaveMismatch,
    SourceWaveMismatch,
    LaneMismatch,
    MissingSlot(ReceiptSlotKind),
}

impl RuntimeError {
    pub fn to_blocker(&self) -> SlotBlocker {
        match self {
            Self::ProtocolVersionMismatch
            | Self::WaveMismatch
            | Self::SourceWaveMismatch
            | Self::LaneMismatch => SlotBlocker::ImportRuleMissing,
            Self::MissingSlot(_) => SlotBlocker::EmptySlot,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProtocolVersionMismatch => f.write_str("protocol version mismatch"),
            Self::WaveMismatch => f.write_str("wave mismatch"),
            Self::SourceWaveMismatch => f.write_str("source wave mismatch"),
            Self::LaneMismatch => f.write_str("lane mismatch"),
            Self::MissingSlot(kind) => write!(f, "missing receipt slot {}", kind.as_str()),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    State::devnet().public_record()
}

pub fn state_root() -> String {
    State::devnet().state_root()
}

pub fn attach_live_receipt_root(
    runtime: &Runtime,
    kind: ReceiptSlotKind,
    receipt_root: &str,
    accepted_root: &str,
) -> Result<Runtime> {
    let evidence = ReceiptEvidence::live_root(
        receipt_root,
        accepted_root,
        runtime.config.plan_root.as_str(),
        runtime.config.final_transcript_root.as_str(),
    );
    runtime.attach_receipt(kind, evidence)
}

fn command_root(commands: &[OperatorCommandHint]) -> String {
    let mut parts = vec![DOMAIN.to_string(), "operator_command_hints".to_string()];
    parts.extend(commands.iter().map(|command| command.as_str().to_string()));
    stable_owned_root(&parts)
}

fn blocker_root(blockers: &[SlotBlocker]) -> String {
    let mut parts = vec![DOMAIN.to_string(), "slot_blockers".to_string()];
    parts.extend(blockers.iter().map(|blocker| blocker.as_str().to_string()));
    stable_owned_root(&parts)
}

fn dedupe_blockers(blockers: &mut Vec<SlotBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn option_u64(value: Option<u64>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "none".to_string(),
    }
}

fn option_u16(value: Option<u16>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "none".to_string(),
    }
}

fn option_bool(value: Option<bool>) -> String {
    match value {
        Some(value) => bool_word(value).to_string(),
        None => "none".to_string(),
    }
}

fn u64_value_or_zero(value: Option<u64>) -> u64 {
    match value {
        Some(value) => value,
        None => 0,
    }
}

fn u16_value_or_zero(value: Option<u16>) -> u16 {
    match value {
        Some(value) => value,
        None => 0,
    }
}

fn bool_value_or_false(value: Option<bool>) -> bool {
    match value {
        Some(value) => value,
        None => false,
    }
}

fn stable_owned_root(parts: &[String]) -> String {
    let borrowed: Vec<&str> = parts.iter().map(String::as_str).collect();
    stable_root(&borrowed)
}

fn stable_root(parts: &[&str]) -> String {
    let mut state = 0x6a09_e667_f3bc_c909_u64;
    for part in parts {
        state = mix_u64(state, part.len() as u64);
        for byte in part.as_bytes() {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x1000_0000_01b3);
            state = state.rotate_left(13) ^ 0x9e37_79b9_7f4a_7c15;
        }
        state = mix_u64(state, 0xff);
    }
    format!("{:016x}{:016x}", state, finish_u64(state))
}

fn mix_u64(state: u64, value: u64) -> u64 {
    let mut next = state ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    next ^= next >> 33;
    next = next.wrapping_mul(0xff51_afd7_ed55_8ccd);
    next ^= next >> 33;
    next
}

fn finish_u64(state: u64) -> u64 {
    let mut next = state ^ 0xbf58_476d_1ce4_e5b9;
    next ^= next >> 30;
    next = next.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    next ^= next >> 27;
    next = next.wrapping_mul(0x94d0_49bb_1331_11eb);
    next ^ (next >> 31)
}
