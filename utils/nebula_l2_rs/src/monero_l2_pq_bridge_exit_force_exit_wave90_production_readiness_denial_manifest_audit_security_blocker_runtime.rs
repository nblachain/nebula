use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

pub type Result<T> = std::result::Result<T, Error>;
pub type Runtime = State;

const DOMAIN_CONFIG: &str = "wave90.production_readiness_denial.config.v1";
const DOMAIN_EVIDENCE: &str = "wave90.production_readiness_denial.evidence.v1";
const DOMAIN_CRITERION: &str = "wave90.production_readiness_denial.criterion.v1";
const DOMAIN_BLOCKER: &str = "wave90.production_readiness_denial.blocker.v1";
const DOMAIN_RECEIPT: &str = "wave90.production_readiness_denial.receipt.v1";
const DOMAIN_ACTION: &str = "wave90.production_readiness_denial.action.v1";
const DOMAIN_RECORD: &str = "wave90.production_readiness_denial.public_record.v1";
const DOMAIN_STATE: &str = "wave90.production_readiness_denial.state.v1";
const DEFAULT_MIN_REVIEWER_RECEIPTS: usize = 5;
const DEFAULT_MIN_ACCEPTANCE_RECEIPTS: usize = 4;
const DEFAULT_MIN_ADVERSARIAL_RUNS: u32 = 12;
const DEFAULT_MIN_PRIVACY_REVIEWS: u32 = 3;
const DEFAULT_MIN_THREAT_ACCEPTANCES: u32 = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyField(&'static str),
    DuplicateEvidence(String),
    DuplicateCriterion(String),
    DuplicateBlocker(String),
    DuplicateReceipt(String),
    DuplicateAction(String),
    MissingEvidenceRoot(String),
    InvalidRoot(String),
    ReviewerNotAllowed(String),
    ReceiptDoesNotDeny(String),
    ClearanceRejected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(f, "empty required field: {}", field),
            Self::DuplicateEvidence(id) => write!(f, "duplicate evidence: {}", id),
            Self::DuplicateCriterion(id) => write!(f, "duplicate criterion: {}", id),
            Self::DuplicateBlocker(id) => write!(f, "duplicate blocker: {}", id),
            Self::DuplicateReceipt(id) => write!(f, "duplicate receipt: {}", id),
            Self::DuplicateAction(id) => write!(f, "duplicate action: {}", id),
            Self::MissingEvidenceRoot(id) => write!(f, "missing evidence root: {}", id),
            Self::InvalidRoot(id) => write!(f, "invalid root: {}", id),
            Self::ReviewerNotAllowed(id) => write!(f, "reviewer not allowed: {}", id),
            Self::ReceiptDoesNotDeny(id) => write!(f, "receipt does not deny readiness: {}", id),
            Self::ClearanceRejected(msg) => write!(f, "clearance rejected: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Root(String);

impl Root {
    pub fn new(domain: &str, parts: &[String]) -> Self {
        let mut hasher = DefaultHasher::new();
        domain.hash(&mut hasher);
        parts.len().hash(&mut hasher);
        for part in parts {
            part.len().hash(&mut hasher);
            part.hash(&mut hasher);
        }
        Self(format!("{:016x}", hasher.finish()))
    }

    pub fn from_public(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(Error::EmptyField("root"));
        }
        if value.len() < 12 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::InvalidRoot(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn require_text(value: &str, field: &'static str) -> Result<()> {
    if value.trim().is_empty() {
        Err(Error::EmptyField(field))
    } else {
        Ok(())
    }
}

fn bool_text(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn join_roots<I>(items: I) -> String
where
    I: IntoIterator<Item = String>,
{
    items.into_iter().collect::<Vec<_>>().join("|")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReviewLane {
    AuditReview,
    AdversarialTesting,
    PrivacyReview,
    ThreatModelAcceptance,
    ReviewerSignoff,
    ReleaseCaptainHold,
}

impl ReviewLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialTesting => "adversarial_testing",
            Self::PrivacyReview => "privacy_review",
            Self::ThreatModelAcceptance => "threat_model_acceptance",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::ReleaseCaptainHold => "release_captain_hold",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditReview,
            Self::AdversarialTesting,
            Self::PrivacyReview,
            Self::ThreatModelAcceptance,
            Self::ReviewerSignoff,
            Self::ReleaseCaptainHold,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EvidenceKind {
    Wave89NoGoArchive,
    AuditHoldRoot,
    AdversarialGapRoot,
    PrivacyHoldRoot,
    ThreatModelHoldRoot,
    ReviewerReceiptRoot,
    OperatorHoldRoot,
}

impl EvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave89NoGoArchive => "wave89_no_go_archive",
            Self::AuditHoldRoot => "audit_hold_root",
            Self::AdversarialGapRoot => "adversarial_gap_root",
            Self::PrivacyHoldRoot => "privacy_hold_root",
            Self::ThreatModelHoldRoot => "threat_model_hold_root",
            Self::ReviewerReceiptRoot => "reviewer_receipt_root",
            Self::OperatorHoldRoot => "operator_hold_root",
        }
    }

    pub fn default_lane(self) -> ReviewLane {
        match self {
            Self::Wave89NoGoArchive => ReviewLane::ReleaseCaptainHold,
            Self::AuditHoldRoot => ReviewLane::AuditReview,
            Self::AdversarialGapRoot => ReviewLane::AdversarialTesting,
            Self::PrivacyHoldRoot => ReviewLane::PrivacyReview,
            Self::ThreatModelHoldRoot => ReviewLane::ThreatModelAcceptance,
            Self::ReviewerReceiptRoot => ReviewLane::ReviewerSignoff,
            Self::OperatorHoldRoot => ReviewLane::ReleaseCaptainHold,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EvidenceDisposition {
    Present,
    Deferred,
    Missing,
    Rejected,
}

impl EvidenceDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Deferred => "deferred",
            Self::Missing => "missing",
            Self::Rejected => "rejected",
        }
    }

    pub fn blocks_production(self) -> bool {
        matches!(self, Self::Deferred | Self::Missing | Self::Rejected)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Severity {
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn weight(self) -> u64 {
        match self {
            Self::Medium => 5,
            Self::High => 13,
            Self::Critical => 29,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CriterionKind {
    AuditReviewComplete,
    AdversarialTestingComplete,
    PrivacyReviewComplete,
    ThreatModelAccepted,
    ReviewerReceiptsPresent,
    NoDeferredArchiveItems,
}

impl CriterionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReviewComplete => "audit_review_complete",
            Self::AdversarialTestingComplete => "adversarial_testing_complete",
            Self::PrivacyReviewComplete => "privacy_review_complete",
            Self::ThreatModelAccepted => "threat_model_accepted",
            Self::ReviewerReceiptsPresent => "reviewer_receipts_present",
            Self::NoDeferredArchiveItems => "no_deferred_archive_items",
        }
    }

    pub fn lane(self) -> ReviewLane {
        match self {
            Self::AuditReviewComplete => ReviewLane::AuditReview,
            Self::AdversarialTestingComplete => ReviewLane::AdversarialTesting,
            Self::PrivacyReviewComplete => ReviewLane::PrivacyReview,
            Self::ThreatModelAccepted => ReviewLane::ThreatModelAcceptance,
            Self::ReviewerReceiptsPresent => ReviewLane::ReviewerSignoff,
            Self::NoDeferredArchiveItems => ReviewLane::ReleaseCaptainHold,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CriterionStatus {
    Satisfied,
    Deferred,
    Missing,
    Failed,
}

impl CriterionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Deferred => "deferred",
            Self::Missing => "missing",
            Self::Failed => "failed",
        }
    }

    pub fn holds(self) -> bool {
        !matches!(self, Self::Satisfied)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlockerKind {
    AuditDenial,
    AdversarialBlocker,
    PrivacySignoffBlocker,
    ThreatModelBlocker,
    ReviewerReceiptBlocker,
    ArchiveDeferralBlocker,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditDenial => "audit_denial",
            Self::AdversarialBlocker => "adversarial_blocker",
            Self::PrivacySignoffBlocker => "privacy_signoff_blocker",
            Self::ThreatModelBlocker => "threat_model_blocker",
            Self::ReviewerReceiptBlocker => "reviewer_receipt_blocker",
            Self::ArchiveDeferralBlocker => "archive_deferral_blocker",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlockerStatus {
    Open,
    Deferred,
    WaitingReviewer,
    WaitingOperator,
    AcceptedHold,
}

impl BlockerStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Deferred => "deferred",
            Self::WaitingReviewer => "waiting_reviewer",
            Self::WaitingOperator => "waiting_operator",
            Self::AcceptedHold => "accepted_hold",
        }
    }

    pub fn holds(self) -> bool {
        matches!(
            self,
            Self::Open | Self::Deferred | Self::WaitingReviewer | Self::WaitingOperator
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReviewerRole {
    AuditLead,
    SecurityLead,
    PrivacyLead,
    ThreatModelOwner,
    ReleaseCaptain,
    OperatorLead,
}

impl ReviewerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditLead => "audit_lead",
            Self::SecurityLead => "security_lead",
            Self::PrivacyLead => "privacy_lead",
            Self::ThreatModelOwner => "threat_model_owner",
            Self::ReleaseCaptain => "release_captain",
            Self::OperatorLead => "operator_lead",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReceiptVerdict {
    DenyProduction,
    HoldForReview,
    HoldForEvidence,
    HoldForThreatAcceptance,
}

impl ReceiptVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyProduction => "deny_production",
            Self::HoldForReview => "hold_for_review",
            Self::HoldForEvidence => "hold_for_evidence",
            Self::HoldForThreatAcceptance => "hold_for_threat_acceptance",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OperatorActionKind {
    KeepReleaseHold,
    RequestAuditReview,
    RequestAdversarialRun,
    RequestPrivacyReview,
    RequestThreatModelAcceptance,
    RequestReviewerReceipts,
    PublishRootsOnlyRecord,
}

impl OperatorActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseHold => "keep_release_hold",
            Self::RequestAuditReview => "request_audit_review",
            Self::RequestAdversarialRun => "request_adversarial_run",
            Self::RequestPrivacyReview => "request_privacy_review",
            Self::RequestThreatModelAcceptance => "request_threat_model_acceptance",
            Self::RequestReviewerReceipts => "request_reviewer_receipts",
            Self::PublishRootsOnlyRecord => "publish_roots_only_record",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FinalVerdict {
    DenyProduction,
    Hold,
}

impl FinalVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyProduction => "deny_production",
            Self::Hold => "hold",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub wave: u32,
    pub source_wave: u32,
    pub release_id: String,
    pub network: String,
    pub fail_closed: bool,
    pub roots_only_public_record: bool,
    pub min_reviewer_receipts: usize,
    pub min_acceptance_receipts: usize,
    pub min_adversarial_runs: u32,
    pub min_privacy_reviews: u32,
    pub min_threat_acceptances: u32,
    pub allowed_reviewers: BTreeSet<String>,
}

impl Config {
    pub fn new(release_id: impl Into<String>, network: impl Into<String>) -> Result<Self> {
        let config = Self {
            wave: 90,
            source_wave: 89,
            release_id: release_id.into(),
            network: network.into(),
            fail_closed: true,
            roots_only_public_record: true,
            min_reviewer_receipts: DEFAULT_MIN_REVIEWER_RECEIPTS,
            min_acceptance_receipts: DEFAULT_MIN_ACCEPTANCE_RECEIPTS,
            min_adversarial_runs: DEFAULT_MIN_ADVERSARIAL_RUNS,
            min_privacy_reviews: DEFAULT_MIN_PRIVACY_REVIEWS,
            min_threat_acceptances: DEFAULT_MIN_THREAT_ACCEPTANCES,
            allowed_reviewers: BTreeSet::new(),
        };
        config.validate()?;
        Ok(config)
    }

    pub fn with_allowed_reviewer(mut self, reviewer_id: impl Into<String>) -> Result<Self> {
        let reviewer_id = reviewer_id.into();
        require_text(&reviewer_id, "reviewer_id")?;
        self.allowed_reviewers.insert(reviewer_id);
        Ok(self)
    }

    pub fn reviewer_allowed(&self, reviewer_id: &str) -> bool {
        self.allowed_reviewers.is_empty() || self.allowed_reviewers.contains(reviewer_id)
    }

    pub fn validate(&self) -> Result<()> {
        require_text(&self.release_id, "release_id")?;
        require_text(&self.network, "network")?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("wave".to_string(), self.wave.to_string());
        fields.insert("source_wave".to_string(), self.source_wave.to_string());
        fields.insert("release_id".to_string(), self.release_id.clone());
        fields.insert("network".to_string(), self.network.clone());
        fields.insert(
            "fail_closed".to_string(),
            bool_text(self.fail_closed).to_string(),
        );
        fields.insert(
            "roots_only_public_record".to_string(),
            bool_text(self.roots_only_public_record).to_string(),
        );
        fields.insert(
            "min_reviewer_receipts".to_string(),
            self.min_reviewer_receipts.to_string(),
        );
        fields.insert(
            "min_acceptance_receipts".to_string(),
            self.min_acceptance_receipts.to_string(),
        );
        fields.insert(
            "min_adversarial_runs".to_string(),
            self.min_adversarial_runs.to_string(),
        );
        fields.insert(
            "min_privacy_reviews".to_string(),
            self.min_privacy_reviews.to_string(),
        );
        fields.insert(
            "min_threat_acceptances".to_string(),
            self.min_threat_acceptances.to_string(),
        );
        fields.insert(
            "allowed_reviewer_count".to_string(),
            self.allowed_reviewers.len().to_string(),
        );
        PublicRecord::new("config", fields, BTreeMap::new())
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_CONFIG)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wave89ArchiveEvidence {
    pub evidence_id: String,
    pub kind: EvidenceKind,
    pub lane: ReviewLane,
    pub disposition: EvidenceDisposition,
    pub archive_root: Root,
    pub source_root: Root,
    pub reviewer_receipt_root: Option<Root>,
    pub item_count: u32,
    pub deferred_count: u32,
}

impl Wave89ArchiveEvidence {
    pub fn new(
        evidence_id: impl Into<String>,
        kind: EvidenceKind,
        disposition: EvidenceDisposition,
        archive_root: Root,
        source_root: Root,
    ) -> Result<Self> {
        let evidence = Self {
            evidence_id: evidence_id.into(),
            kind,
            lane: kind.default_lane(),
            disposition,
            archive_root,
            source_root,
            reviewer_receipt_root: None,
            item_count: 0,
            deferred_count: 0,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn with_counts(mut self, item_count: u32, deferred_count: u32) -> Self {
        self.item_count = item_count;
        self.deferred_count = deferred_count;
        self
    }

    pub fn with_receipt_root(mut self, root: Root) -> Self {
        self.reviewer_receipt_root = Some(root);
        self
    }

    pub fn validate(&self) -> Result<()> {
        require_text(&self.evidence_id, "evidence_id")
    }

    pub fn blocks_production(&self) -> bool {
        self.disposition.blocks_production() || self.deferred_count > 0
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("evidence_id".to_string(), self.evidence_id.clone());
        fields.insert("kind".to_string(), self.kind.as_str().to_string());
        fields.insert("lane".to_string(), self.lane.as_str().to_string());
        fields.insert(
            "disposition".to_string(),
            self.disposition.as_str().to_string(),
        );
        fields.insert("item_count".to_string(), self.item_count.to_string());
        fields.insert(
            "deferred_count".to_string(),
            self.deferred_count.to_string(),
        );
        fields.insert(
            "blocks_production".to_string(),
            bool_text(self.blocks_production()).to_string(),
        );
        let mut roots = BTreeMap::new();
        roots.insert("archive_root".to_string(), self.archive_root.to_string());
        roots.insert("source_root".to_string(), self.source_root.to_string());
        if let Some(root) = &self.reviewer_receipt_root {
            roots.insert("reviewer_receipt_root".to_string(), root.to_string());
        }
        PublicRecord::new("wave89_archive_evidence", fields, roots)
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_EVIDENCE)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditDenialCriterion {
    pub criterion_id: String,
    pub kind: CriterionKind,
    pub status: CriterionStatus,
    pub severity: Severity,
    pub evidence_root: Root,
    pub required_count: u32,
    pub observed_count: u32,
}

impl AuditDenialCriterion {
    pub fn new(
        criterion_id: impl Into<String>,
        kind: CriterionKind,
        status: CriterionStatus,
        severity: Severity,
        evidence_root: Root,
    ) -> Result<Self> {
        let criterion = Self {
            criterion_id: criterion_id.into(),
            kind,
            status,
            severity,
            evidence_root,
            required_count: 1,
            observed_count: 0,
        };
        criterion.validate()?;
        Ok(criterion)
    }

    pub fn with_counts(mut self, required_count: u32, observed_count: u32) -> Self {
        self.required_count = required_count;
        self.observed_count = observed_count;
        self
    }

    pub fn validate(&self) -> Result<()> {
        require_text(&self.criterion_id, "criterion_id")
    }

    pub fn denies_readiness(&self) -> bool {
        self.status.holds() || self.observed_count < self.required_count
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("criterion_id".to_string(), self.criterion_id.clone());
        fields.insert("kind".to_string(), self.kind.as_str().to_string());
        fields.insert("lane".to_string(), self.kind.lane().as_str().to_string());
        fields.insert("status".to_string(), self.status.as_str().to_string());
        fields.insert("severity".to_string(), self.severity.as_str().to_string());
        fields.insert(
            "required_count".to_string(),
            self.required_count.to_string(),
        );
        fields.insert(
            "observed_count".to_string(),
            self.observed_count.to_string(),
        );
        fields.insert(
            "denies_readiness".to_string(),
            bool_text(self.denies_readiness()).to_string(),
        );
        let mut roots = BTreeMap::new();
        roots.insert("evidence_root".to_string(), self.evidence_root.to_string());
        PublicRecord::new("audit_denial_criterion", fields, roots)
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_CRITERION)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionBlocker {
    pub blocker_id: String,
    pub kind: BlockerKind,
    pub lane: ReviewLane,
    pub status: BlockerStatus,
    pub severity: Severity,
    pub criterion_root: Root,
    pub evidence_root: Root,
    pub action_hint: OperatorActionKind,
}

impl ProductionBlocker {
    pub fn new(
        blocker_id: impl Into<String>,
        kind: BlockerKind,
        lane: ReviewLane,
        status: BlockerStatus,
        severity: Severity,
        criterion_root: Root,
        evidence_root: Root,
    ) -> Result<Self> {
        let action_hint = match kind {
            BlockerKind::AuditDenial => OperatorActionKind::RequestAuditReview,
            BlockerKind::AdversarialBlocker => OperatorActionKind::RequestAdversarialRun,
            BlockerKind::PrivacySignoffBlocker => OperatorActionKind::RequestPrivacyReview,
            BlockerKind::ThreatModelBlocker => OperatorActionKind::RequestThreatModelAcceptance,
            BlockerKind::ReviewerReceiptBlocker => OperatorActionKind::RequestReviewerReceipts,
            BlockerKind::ArchiveDeferralBlocker => OperatorActionKind::KeepReleaseHold,
        };
        let blocker = Self {
            blocker_id: blocker_id.into(),
            kind,
            lane,
            status,
            severity,
            criterion_root,
            evidence_root,
            action_hint,
        };
        blocker.validate()?;
        Ok(blocker)
    }

    pub fn validate(&self) -> Result<()> {
        require_text(&self.blocker_id, "blocker_id")
    }

    pub fn holds_release(&self) -> bool {
        self.status.holds()
    }

    pub fn deny_weight(&self) -> u64 {
        if self.holds_release() {
            self.severity.weight()
        } else {
            0
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("blocker_id".to_string(), self.blocker_id.clone());
        fields.insert("kind".to_string(), self.kind.as_str().to_string());
        fields.insert("lane".to_string(), self.lane.as_str().to_string());
        fields.insert("status".to_string(), self.status.as_str().to_string());
        fields.insert("severity".to_string(), self.severity.as_str().to_string());
        fields.insert(
            "action_hint".to_string(),
            self.action_hint.as_str().to_string(),
        );
        fields.insert("deny_weight".to_string(), self.deny_weight().to_string());
        fields.insert(
            "holds_release".to_string(),
            bool_text(self.holds_release()).to_string(),
        );
        let mut roots = BTreeMap::new();
        roots.insert(
            "criterion_root".to_string(),
            self.criterion_root.to_string(),
        );
        roots.insert("evidence_root".to_string(), self.evidence_root.to_string());
        PublicRecord::new("production_blocker", fields, roots)
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_BLOCKER)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewerReceipt {
    pub receipt_id: String,
    pub reviewer_id: String,
    pub role: ReviewerRole,
    pub verdict: ReceiptVerdict,
    pub covered_lane: ReviewLane,
    pub evidence_root: Root,
    pub blocker_root: Root,
    pub receipt_root: Root,
}

impl ReviewerReceipt {
    pub fn new(
        receipt_id: impl Into<String>,
        reviewer_id: impl Into<String>,
        role: ReviewerRole,
        verdict: ReceiptVerdict,
        covered_lane: ReviewLane,
        evidence_root: Root,
        blocker_root: Root,
    ) -> Result<Self> {
        let receipt_id = receipt_id.into();
        let reviewer_id = reviewer_id.into();
        require_text(&receipt_id, "receipt_id")?;
        require_text(&reviewer_id, "reviewer_id")?;
        let receipt_root = Root::new(
            DOMAIN_RECEIPT,
            &[
                receipt_id.clone(),
                reviewer_id.clone(),
                role.as_str().to_string(),
                verdict.as_str().to_string(),
                covered_lane.as_str().to_string(),
                evidence_root.to_string(),
                blocker_root.to_string(),
            ],
        );
        Ok(Self {
            receipt_id,
            reviewer_id,
            role,
            verdict,
            covered_lane,
            evidence_root,
            blocker_root,
            receipt_root,
        })
    }

    pub fn denies_readiness(&self) -> bool {
        matches!(
            self.verdict,
            ReceiptVerdict::DenyProduction
                | ReceiptVerdict::HoldForReview
                | ReceiptVerdict::HoldForEvidence
                | ReceiptVerdict::HoldForThreatAcceptance
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("receipt_id".to_string(), self.receipt_id.clone());
        fields.insert("reviewer_id".to_string(), self.reviewer_id.clone());
        fields.insert("role".to_string(), self.role.as_str().to_string());
        fields.insert("verdict".to_string(), self.verdict.as_str().to_string());
        fields.insert(
            "covered_lane".to_string(),
            self.covered_lane.as_str().to_string(),
        );
        fields.insert(
            "denies_readiness".to_string(),
            bool_text(self.denies_readiness()).to_string(),
        );
        let mut roots = BTreeMap::new();
        roots.insert("evidence_root".to_string(), self.evidence_root.to_string());
        roots.insert("blocker_root".to_string(), self.blocker_root.to_string());
        roots.insert("receipt_root".to_string(), self.receipt_root.to_string());
        PublicRecord::new("reviewer_receipt", fields, roots)
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_RECEIPT)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorActionHint {
    pub action_id: String,
    pub kind: OperatorActionKind,
    pub lane: ReviewLane,
    pub blocker_root: Root,
    pub required_before_clearance: bool,
}

impl OperatorActionHint {
    pub fn new(
        action_id: impl Into<String>,
        kind: OperatorActionKind,
        lane: ReviewLane,
        blocker_root: Root,
    ) -> Result<Self> {
        let action = Self {
            action_id: action_id.into(),
            kind,
            lane,
            blocker_root,
            required_before_clearance: true,
        };
        action.validate()?;
        Ok(action)
    }

    pub fn validate(&self) -> Result<()> {
        require_text(&self.action_id, "action_id")
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert("action_id".to_string(), self.action_id.clone());
        fields.insert("kind".to_string(), self.kind.as_str().to_string());
        fields.insert("lane".to_string(), self.lane.as_str().to_string());
        fields.insert(
            "required_before_clearance".to_string(),
            bool_text(self.required_before_clearance).to_string(),
        );
        let mut roots = BTreeMap::new();
        roots.insert("blocker_root".to_string(), self.blocker_root.to_string());
        PublicRecord::new("operator_action_hint", fields, roots)
    }

    pub fn root(&self) -> Root {
        self.public_record().root(DOMAIN_ACTION)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialSummary {
    pub final_verdict: FinalVerdict,
    pub blocked_lanes: BTreeSet<ReviewLane>,
    pub open_blockers: usize,
    pub deferred_or_missing_evidence: usize,
    pub reviewer_receipts: usize,
    pub acceptance_receipts: usize,
    pub adversarial_runs_observed: u32,
    pub privacy_reviews_observed: u32,
    pub threat_acceptances_observed: u32,
    pub deny_weight: u64,
}

impl DenialSummary {
    pub fn public_record(&self) -> PublicRecord {
        let mut fields = BTreeMap::new();
        fields.insert(
            "final_verdict".to_string(),
            self.final_verdict.as_str().to_string(),
        );
        fields.insert("open_blockers".to_string(), self.open_blockers.to_string());
        fields.insert(
            "deferred_or_missing_evidence".to_string(),
            self.deferred_or_missing_evidence.to_string(),
        );
        fields.insert(
            "reviewer_receipts".to_string(),
            self.reviewer_receipts.to_string(),
        );
        fields.insert(
            "acceptance_receipts".to_string(),
            self.acceptance_receipts.to_string(),
        );
        fields.insert(
            "adversarial_runs_observed".to_string(),
            self.adversarial_runs_observed.to_string(),
        );
        fields.insert(
            "privacy_reviews_observed".to_string(),
            self.privacy_reviews_observed.to_string(),
        );
        fields.insert(
            "threat_acceptances_observed".to_string(),
            self.threat_acceptances_observed.to_string(),
        );
        fields.insert("deny_weight".to_string(), self.deny_weight.to_string());
        fields.insert(
            "blocked_lanes".to_string(),
            self.blocked_lanes
                .iter()
                .map(|lane| lane.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
        PublicRecord::new("denial_summary", fields, BTreeMap::new())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub record_type: String,
    pub fields: BTreeMap<String, String>,
    pub roots: BTreeMap<String, String>,
}

impl PublicRecord {
    pub fn new(
        record_type: impl Into<String>,
        fields: BTreeMap<String, String>,
        roots: BTreeMap<String, String>,
    ) -> Self {
        Self {
            record_type: record_type.into(),
            fields,
            roots,
        }
    }

    pub fn root(&self, domain: &str) -> Root {
        let mut parts = vec![self.record_type.clone()];
        for (key, value) in &self.fields {
            parts.push(format!("{}={}", key, value));
        }
        for (key, value) in &self.roots {
            parts.push(format!("{}={}", key, value));
        }
        Root::new(domain, &parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub evidence: BTreeMap<String, Wave89ArchiveEvidence>,
    pub criteria: BTreeMap<String, AuditDenialCriterion>,
    pub blockers: BTreeMap<String, ProductionBlocker>,
    pub receipts: BTreeMap<String, ReviewerReceipt>,
    pub action_hints: BTreeMap<String, OperatorActionHint>,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config,
            evidence: BTreeMap::new(),
            criteria: BTreeMap::new(),
            blockers: BTreeMap::new(),
            receipts: BTreeMap::new(),
            action_hints: BTreeMap::new(),
        })
    }

    pub fn insert_evidence(&mut self, evidence: Wave89ArchiveEvidence) -> Result<Root> {
        evidence.validate()?;
        if self.evidence.contains_key(&evidence.evidence_id) {
            return Err(Error::DuplicateEvidence(evidence.evidence_id));
        }
        let root = evidence.root();
        self.evidence.insert(evidence.evidence_id.clone(), evidence);
        Ok(root)
    }

    pub fn insert_criterion(&mut self, criterion: AuditDenialCriterion) -> Result<Root> {
        criterion.validate()?;
        if self.criteria.contains_key(&criterion.criterion_id) {
            return Err(Error::DuplicateCriterion(criterion.criterion_id));
        }
        if !self
            .evidence
            .values()
            .any(|item| item.root() == criterion.evidence_root)
        {
            return Err(Error::MissingEvidenceRoot(
                criterion.evidence_root.to_string(),
            ));
        }
        let root = criterion.root();
        self.criteria
            .insert(criterion.criterion_id.clone(), criterion);
        Ok(root)
    }

    pub fn insert_blocker(&mut self, blocker: ProductionBlocker) -> Result<Root> {
        blocker.validate()?;
        if self.blockers.contains_key(&blocker.blocker_id) {
            return Err(Error::DuplicateBlocker(blocker.blocker_id));
        }
        let root = blocker.root();
        self.blockers.insert(blocker.blocker_id.clone(), blocker);
        Ok(root)
    }

    pub fn insert_receipt(&mut self, receipt: ReviewerReceipt) -> Result<Root> {
        if self.receipts.contains_key(&receipt.receipt_id) {
            return Err(Error::DuplicateReceipt(receipt.receipt_id));
        }
        if !self.config.reviewer_allowed(&receipt.reviewer_id) {
            return Err(Error::ReviewerNotAllowed(receipt.reviewer_id));
        }
        if !receipt.denies_readiness() {
            return Err(Error::ReceiptDoesNotDeny(receipt.receipt_id));
        }
        let root = receipt.root();
        self.receipts.insert(receipt.receipt_id.clone(), receipt);
        Ok(root)
    }

    pub fn insert_action_hint(&mut self, action: OperatorActionHint) -> Result<Root> {
        action.validate()?;
        if self.action_hints.contains_key(&action.action_id) {
            return Err(Error::DuplicateAction(action.action_id));
        }
        let root = action.root();
        self.action_hints.insert(action.action_id.clone(), action);
        Ok(root)
    }

    pub fn derive_missing_criteria(&mut self) -> Result<()> {
        let evidence_roots = self
            .evidence
            .values()
            .map(|item| (item.kind, item.root()))
            .collect::<BTreeMap<_, _>>();
        let archive_root = match evidence_roots.get(&EvidenceKind::Wave89NoGoArchive) {
            Some(root) => root.clone(),
            None => return Ok(()),
        };
        let planned = vec![
            (
                "criterion-audit-review",
                CriterionKind::AuditReviewComplete,
                CriterionStatus::Deferred,
                Severity::Critical,
                match evidence_roots.get(&EvidenceKind::AuditHoldRoot) {
                    Some(root) => root.clone(),
                    None => archive_root.clone(),
                },
                1,
                0,
            ),
            (
                "criterion-adversarial-testing",
                CriterionKind::AdversarialTestingComplete,
                CriterionStatus::Deferred,
                Severity::Critical,
                match evidence_roots.get(&EvidenceKind::AdversarialGapRoot) {
                    Some(root) => root.clone(),
                    None => archive_root.clone(),
                },
                self.config.min_adversarial_runs,
                0,
            ),
            (
                "criterion-privacy-review",
                CriterionKind::PrivacyReviewComplete,
                CriterionStatus::Deferred,
                Severity::Critical,
                match evidence_roots.get(&EvidenceKind::PrivacyHoldRoot) {
                    Some(root) => root.clone(),
                    None => archive_root.clone(),
                },
                self.config.min_privacy_reviews,
                0,
            ),
            (
                "criterion-threat-model",
                CriterionKind::ThreatModelAccepted,
                CriterionStatus::Missing,
                Severity::High,
                match evidence_roots.get(&EvidenceKind::ThreatModelHoldRoot) {
                    Some(root) => root.clone(),
                    None => archive_root.clone(),
                },
                self.config.min_threat_acceptances,
                0,
            ),
            (
                "criterion-reviewer-receipts",
                CriterionKind::ReviewerReceiptsPresent,
                CriterionStatus::Missing,
                Severity::High,
                match evidence_roots.get(&EvidenceKind::ReviewerReceiptRoot) {
                    Some(root) => root.clone(),
                    None => archive_root.clone(),
                },
                self.config.min_reviewer_receipts as u32,
                self.receipts.len() as u32,
            ),
            (
                "criterion-no-deferred-archive-items",
                CriterionKind::NoDeferredArchiveItems,
                CriterionStatus::Failed,
                Severity::Critical,
                archive_root,
                1,
                0,
            ),
        ];
        for (id, kind, status, severity, root, required, observed) in planned {
            if !self.criteria.contains_key(id) {
                let criterion = AuditDenialCriterion::new(id, kind, status, severity, root)?
                    .with_counts(required, observed);
                self.insert_criterion(criterion)?;
            }
        }
        Ok(())
    }

    pub fn derive_blockers_and_actions(&mut self) -> Result<()> {
        let criteria = self.criteria.values().cloned().collect::<Vec<_>>();
        for criterion in criteria {
            if !criterion.denies_readiness() {
                continue;
            }
            let kind = match criterion.kind {
                CriterionKind::AuditReviewComplete => BlockerKind::AuditDenial,
                CriterionKind::AdversarialTestingComplete => BlockerKind::AdversarialBlocker,
                CriterionKind::PrivacyReviewComplete => BlockerKind::PrivacySignoffBlocker,
                CriterionKind::ThreatModelAccepted => BlockerKind::ThreatModelBlocker,
                CriterionKind::ReviewerReceiptsPresent => BlockerKind::ReviewerReceiptBlocker,
                CriterionKind::NoDeferredArchiveItems => BlockerKind::ArchiveDeferralBlocker,
            };
            let blocker_id = format!("blocker-{}", criterion.criterion_id);
            if self.blockers.contains_key(&blocker_id) {
                continue;
            }
            let blocker = ProductionBlocker::new(
                blocker_id.clone(),
                kind,
                criterion.kind.lane(),
                BlockerStatus::Deferred,
                criterion.severity,
                criterion.root(),
                criterion.evidence_root.clone(),
            )?;
            let blocker_root = self.insert_blocker(blocker)?;
            let action_kind = match kind {
                BlockerKind::AuditDenial => OperatorActionKind::RequestAuditReview,
                BlockerKind::AdversarialBlocker => OperatorActionKind::RequestAdversarialRun,
                BlockerKind::PrivacySignoffBlocker => OperatorActionKind::RequestPrivacyReview,
                BlockerKind::ThreatModelBlocker => OperatorActionKind::RequestThreatModelAcceptance,
                BlockerKind::ReviewerReceiptBlocker => OperatorActionKind::RequestReviewerReceipts,
                BlockerKind::ArchiveDeferralBlocker => OperatorActionKind::KeepReleaseHold,
            };
            let action = OperatorActionHint::new(
                format!("action-{}", criterion.criterion_id),
                action_kind,
                criterion.kind.lane(),
                blocker_root,
            )?;
            self.insert_action_hint(action)?;
        }
        if !self
            .action_hints
            .values()
            .any(|action| action.kind == OperatorActionKind::PublishRootsOnlyRecord)
        {
            let root = self.blocker_root();
            let action = OperatorActionHint::new(
                "action-publish-roots-only-record",
                OperatorActionKind::PublishRootsOnlyRecord,
                ReviewLane::ReleaseCaptainHold,
                root,
            )?;
            self.insert_action_hint(action)?;
        }
        Ok(())
    }

    pub fn denial_summary(&self) -> DenialSummary {
        let blocked_lanes = self
            .blockers
            .values()
            .filter(|blocker| blocker.holds_release())
            .map(|blocker| blocker.lane)
            .collect::<BTreeSet<_>>();
        let open_blockers = self
            .blockers
            .values()
            .filter(|blocker| blocker.holds_release())
            .count();
        let deferred_or_missing_evidence = self
            .evidence
            .values()
            .filter(|evidence| evidence.blocks_production())
            .count();
        let acceptance_receipts = self
            .receipts
            .values()
            .filter(|receipt| {
                matches!(
                    receipt.role,
                    ReviewerRole::AuditLead
                        | ReviewerRole::SecurityLead
                        | ReviewerRole::PrivacyLead
                        | ReviewerRole::ThreatModelOwner
                )
            })
            .count();
        let deny_weight = self
            .blockers
            .values()
            .map(|blocker| blocker.deny_weight())
            .sum();
        let final_verdict = if open_blockers > 0
            || deferred_or_missing_evidence > 0
            || self.receipts.len() < self.config.min_reviewer_receipts
            || acceptance_receipts < self.config.min_acceptance_receipts
        {
            FinalVerdict::DenyProduction
        } else {
            FinalVerdict::Hold
        };
        DenialSummary {
            final_verdict,
            blocked_lanes,
            open_blockers,
            deferred_or_missing_evidence,
            reviewer_receipts: self.receipts.len(),
            acceptance_receipts,
            adversarial_runs_observed: 0,
            privacy_reviews_observed: 0,
            threat_acceptances_observed: 0,
            deny_weight,
        }
    }

    pub fn clearable(&self) -> Result<()> {
        let summary = self.denial_summary();
        if summary.final_verdict == FinalVerdict::DenyProduction {
            return Err(Error::ClearanceRejected(self.state_root()));
        }
        Err(Error::ClearanceRejected(
            "runtime only emits hold or deny verdicts".to_string(),
        ))
    }

    pub fn config_root(&self) -> Root {
        self.config.root()
    }

    pub fn evidence_root(&self) -> Root {
        let parts = self
            .evidence
            .values()
            .map(|item| item.root().to_string())
            .collect::<Vec<_>>();
        Root::new(DOMAIN_EVIDENCE, &[join_roots(parts)])
    }

    pub fn criteria_root(&self) -> Root {
        let parts = self
            .criteria
            .values()
            .map(|item| item.root().to_string())
            .collect::<Vec<_>>();
        Root::new(DOMAIN_CRITERION, &[join_roots(parts)])
    }

    pub fn blocker_root(&self) -> Root {
        let parts = self
            .blockers
            .values()
            .map(|item| item.root().to_string())
            .collect::<Vec<_>>();
        Root::new(DOMAIN_BLOCKER, &[join_roots(parts)])
    }

    pub fn receipt_root(&self) -> Root {
        let parts = self
            .receipts
            .values()
            .map(|item| item.root().to_string())
            .collect::<Vec<_>>();
        Root::new(DOMAIN_RECEIPT, &[join_roots(parts)])
    }

    pub fn action_root(&self) -> Root {
        let parts = self
            .action_hints
            .values()
            .map(|item| item.root().to_string())
            .collect::<Vec<_>>();
        Root::new(DOMAIN_ACTION, &[join_roots(parts)])
    }

    pub fn public_record(&self) -> PublicRecord {
        let summary = self.denial_summary();
        let mut fields = BTreeMap::new();
        fields.insert(
            "record_type".to_string(),
            "production_readiness_denial_manifest".to_string(),
        );
        fields.insert("wave".to_string(), self.config.wave.to_string());
        fields.insert(
            "source_wave".to_string(),
            self.config.source_wave.to_string(),
        );
        fields.insert("release_id".to_string(), self.config.release_id.clone());
        fields.insert("network".to_string(), self.config.network.clone());
        fields.insert(
            "final_verdict".to_string(),
            summary.final_verdict.as_str().to_string(),
        );
        fields.insert(
            "open_blockers".to_string(),
            summary.open_blockers.to_string(),
        );
        fields.insert(
            "deferred_or_missing_evidence".to_string(),
            summary.deferred_or_missing_evidence.to_string(),
        );
        fields.insert(
            "reviewer_receipts".to_string(),
            summary.reviewer_receipts.to_string(),
        );
        fields.insert(
            "acceptance_receipts".to_string(),
            summary.acceptance_receipts.to_string(),
        );
        fields.insert("deny_weight".to_string(), summary.deny_weight.to_string());
        fields.insert(
            "blocked_lanes".to_string(),
            summary
                .blocked_lanes
                .iter()
                .map(|lane| lane.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
        let mut roots = BTreeMap::new();
        roots.insert("config_root".to_string(), self.config_root().to_string());
        roots.insert(
            "evidence_root".to_string(),
            self.evidence_root().to_string(),
        );
        roots.insert(
            "criteria_root".to_string(),
            self.criteria_root().to_string(),
        );
        roots.insert("blocker_root".to_string(), self.blocker_root().to_string());
        roots.insert("receipt_root".to_string(), self.receipt_root().to_string());
        roots.insert("action_root".to_string(), self.action_root().to_string());
        roots.insert(
            "summary_root".to_string(),
            summary.public_record().root(DOMAIN_RECORD).to_string(),
        );
        PublicRecord::new("production_readiness_denial_manifest", fields, roots)
    }

    pub fn state_root(&self) -> String {
        self.public_record().root(DOMAIN_STATE).to_string()
    }
}

pub fn devnet() -> Runtime {
    let config = match Config::new("wave90-force-exit-denial", "devnet") {
        Ok(config) => config,
        Err(_) => Config {
            wave: 90,
            source_wave: 89,
            release_id: "wave90-force-exit-denial".to_string(),
            network: "devnet".to_string(),
            fail_closed: true,
            roots_only_public_record: true,
            min_reviewer_receipts: DEFAULT_MIN_REVIEWER_RECEIPTS,
            min_acceptance_receipts: DEFAULT_MIN_ACCEPTANCE_RECEIPTS,
            min_adversarial_runs: DEFAULT_MIN_ADVERSARIAL_RUNS,
            min_privacy_reviews: DEFAULT_MIN_PRIVACY_REVIEWS,
            min_threat_acceptances: DEFAULT_MIN_THREAT_ACCEPTANCES,
            allowed_reviewers: BTreeSet::new(),
        },
    };
    let mut state = match State::new(config) {
        Ok(state) => state,
        Err(_) => State {
            config: Config {
                wave: 90,
                source_wave: 89,
                release_id: "wave90-force-exit-denial".to_string(),
                network: "devnet".to_string(),
                fail_closed: true,
                roots_only_public_record: true,
                min_reviewer_receipts: DEFAULT_MIN_REVIEWER_RECEIPTS,
                min_acceptance_receipts: DEFAULT_MIN_ACCEPTANCE_RECEIPTS,
                min_adversarial_runs: DEFAULT_MIN_ADVERSARIAL_RUNS,
                min_privacy_reviews: DEFAULT_MIN_PRIVACY_REVIEWS,
                min_threat_acceptances: DEFAULT_MIN_THREAT_ACCEPTANCES,
                allowed_reviewers: BTreeSet::new(),
            },
            evidence: BTreeMap::new(),
            criteria: BTreeMap::new(),
            blockers: BTreeMap::new(),
            receipts: BTreeMap::new(),
            action_hints: BTreeMap::new(),
        },
    };
    seed_devnet(&mut state);
    state
}

fn seed_devnet(state: &mut State) {
    let wave89_archive_root = Root::new(
        "wave89.no_go.archive",
        &[
            "audit_security_gate".to_string(),
            "release_captain_no_go".to_string(),
        ],
    );
    let common_source_root = Root::new(
        "wave89.no_go.source",
        &["roots_only".to_string(), "deferred_review".to_string()],
    );
    let seeds = vec![
        (
            "evidence-wave89-no-go-archive",
            EvidenceKind::Wave89NoGoArchive,
            EvidenceDisposition::Deferred,
            wave89_archive_root.clone(),
            common_source_root.clone(),
            42,
            9,
        ),
        (
            "evidence-audit-review-hold",
            EvidenceKind::AuditHoldRoot,
            EvidenceDisposition::Deferred,
            Root::new("wave89.audit.hold", &["audit_review_deferred".to_string()]),
            common_source_root.clone(),
            7,
            3,
        ),
        (
            "evidence-adversarial-gap",
            EvidenceKind::AdversarialGapRoot,
            EvidenceDisposition::Missing,
            Root::new(
                "wave89.adversarial.gap",
                &["mutation_runs_missing".to_string()],
            ),
            common_source_root.clone(),
            12,
            12,
        ),
        (
            "evidence-privacy-signoff-hold",
            EvidenceKind::PrivacyHoldRoot,
            EvidenceDisposition::Deferred,
            Root::new(
                "wave89.privacy.hold",
                &["linkage_review_deferred".to_string()],
            ),
            common_source_root.clone(),
            4,
            2,
        ),
        (
            "evidence-threat-model-hold",
            EvidenceKind::ThreatModelHoldRoot,
            EvidenceDisposition::Missing,
            Root::new(
                "wave89.threat.hold",
                &["acceptance_receipts_missing".to_string()],
            ),
            common_source_root.clone(),
            2,
            2,
        ),
        (
            "evidence-reviewer-receipt-gap",
            EvidenceKind::ReviewerReceiptRoot,
            EvidenceDisposition::Missing,
            Root::new(
                "wave89.receipt.gap",
                &["signoff_receipts_missing".to_string()],
            ),
            common_source_root.clone(),
            5,
            5,
        ),
        (
            "evidence-operator-hold",
            EvidenceKind::OperatorHoldRoot,
            EvidenceDisposition::Present,
            Root::new(
                "wave89.operator.hold",
                &["pager_ack_roots_only".to_string()],
            ),
            common_source_root,
            3,
            0,
        ),
    ];
    for (id, kind, disposition, archive_root, source_root, item_count, deferred_count) in seeds {
        if let Ok(evidence) =
            Wave89ArchiveEvidence::new(id, kind, disposition, archive_root, source_root)
        {
            let _ = state.insert_evidence(evidence.with_counts(item_count, deferred_count));
        }
    }
    let _ = state.derive_missing_criteria();
    let _ = state.derive_blockers_and_actions();
    let blocker_roots = state
        .blockers
        .values()
        .map(|blocker| (blocker.lane, blocker.root(), blocker.evidence_root.clone()))
        .collect::<Vec<_>>();
    let receipt_specs = vec![
        (
            "receipt-audit-lead-deny",
            "audit-lead-rooted-reviewer",
            ReviewerRole::AuditLead,
            ReceiptVerdict::DenyProduction,
            ReviewLane::AuditReview,
        ),
        (
            "receipt-security-lead-hold",
            "security-lead-rooted-reviewer",
            ReviewerRole::SecurityLead,
            ReceiptVerdict::HoldForEvidence,
            ReviewLane::AdversarialTesting,
        ),
        (
            "receipt-privacy-lead-hold",
            "privacy-lead-rooted-reviewer",
            ReviewerRole::PrivacyLead,
            ReceiptVerdict::HoldForReview,
            ReviewLane::PrivacyReview,
        ),
        (
            "receipt-threat-owner-hold",
            "threat-owner-rooted-reviewer",
            ReviewerRole::ThreatModelOwner,
            ReceiptVerdict::HoldForThreatAcceptance,
            ReviewLane::ThreatModelAcceptance,
        ),
        (
            "receipt-release-captain-deny",
            "release-captain-rooted-reviewer",
            ReviewerRole::ReleaseCaptain,
            ReceiptVerdict::DenyProduction,
            ReviewLane::ReleaseCaptainHold,
        ),
    ];
    for (receipt_id, reviewer_id, role, verdict, lane) in receipt_specs {
        if let Some((_, blocker_root, evidence_root)) = blocker_roots
            .iter()
            .find(|(blocker_lane, _, _)| *blocker_lane == lane)
        {
            if let Ok(receipt) = ReviewerReceipt::new(
                receipt_id,
                reviewer_id,
                role,
                verdict,
                lane,
                evidence_root.clone(),
                blocker_root.clone(),
            ) {
                let _ = state.insert_receipt(receipt);
            }
        }
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}
