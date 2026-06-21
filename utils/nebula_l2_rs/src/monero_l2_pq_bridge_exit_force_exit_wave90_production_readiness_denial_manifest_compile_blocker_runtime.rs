use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

pub type Result<T> = std::result::Result<T, DenialError>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave90-production-readiness-denial-manifest-compile-blocker-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const DEFAULT_WAVE: u64 = 90;
pub const SOURCE_ARCHIVE_WAVE: u64 = 89;
pub const DEFAULT_REVIEW_HEIGHT: u64 = 900_000;
pub const DEFAULT_SOURCE_ARCHIVE_HEIGHT: u64 = 890_000;
pub const DEFAULT_MIN_ARCHIVE_LANES: usize = 6;
pub const DEFAULT_MIN_DENIAL_CRITERIA: usize = 8;
pub const DEFAULT_MIN_MISSING_HEAVY_GATES: usize = 5;
pub const DEFAULT_MIN_ACTION_HINTS: usize = 6;
pub const DEFAULT_MAX_ARCHIVE_AGE_BLOCKS: u64 = 240;
pub const PRIVACY_MODE: &str = "roots-only";
pub const FINAL_DENIAL_LABEL: &str = "production-readiness-denied";
pub const HOLD_LABEL: &str = "compile-blocker-hold";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DenialError {
    EmptyField(&'static str),
    DuplicateId(String),
    UnknownEvidence(String),
    UnknownReceipt(String),
    UnknownCriterion(String),
    UnknownBlocker(String),
    InvalidRoot { field: &'static str, value: String },
    InvalidHeight,
    InvalidWave,
    MissingRequiredEvidence(&'static str),
    MissingRequiredReceipt(&'static str),
    InsufficientCriteria { required: usize, actual: usize },
    InsufficientBlockers { required: usize, actual: usize },
    InsufficientActions { required: usize, actual: usize },
    ReceiptWouldClearGate(String),
    ReadinessWouldOpen,
    PolicyViolation(String),
}

impl fmt::Display for DenialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(f, "empty required field: {}", field),
            Self::DuplicateId(id) => write!(f, "duplicate identifier: {}", id),
            Self::UnknownEvidence(id) => write!(f, "unknown archive evidence: {}", id),
            Self::UnknownReceipt(id) => write!(f, "unknown compile receipt: {}", id),
            Self::UnknownCriterion(id) => write!(f, "unknown denial criterion: {}", id),
            Self::UnknownBlocker(id) => write!(f, "unknown heavy gate blocker: {}", id),
            Self::InvalidRoot { field, value } => {
                write!(f, "invalid root for {}: {}", field, value)
            }
            Self::InvalidHeight => write!(f, "invalid review height"),
            Self::InvalidWave => write!(f, "invalid wave binding"),
            Self::MissingRequiredEvidence(name) => write!(f, "missing required evidence: {}", name),
            Self::MissingRequiredReceipt(name) => write!(f, "missing required receipt: {}", name),
            Self::InsufficientCriteria { required, actual } => {
                write!(
                    f,
                    "insufficient denial criteria: required {}, actual {}",
                    required, actual
                )
            }
            Self::InsufficientBlockers { required, actual } => {
                write!(
                    f,
                    "insufficient heavy gate blockers: required {}, actual {}",
                    required, actual
                )
            }
            Self::InsufficientActions { required, actual } => {
                write!(
                    f,
                    "insufficient operator actions: required {}, actual {}",
                    required, actual
                )
            }
            Self::ReceiptWouldClearGate(id) => write!(f, "receipt would clear heavy gate: {}", id),
            Self::ReadinessWouldOpen => write!(f, "production readiness would open"),
            Self::PolicyViolation(detail) => write!(f, "policy violation: {}", detail),
        }
    }
}

impl std::error::Error for DenialError {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ArchiveLane {
    CompileGate,
    RuntimeReplayGate,
    AuditSecurityGate,
    BridgeCustodyGate,
    WalletWatchtowerGate,
    PqReservePrivacyGate,
}

impl ArchiveLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CompileGate,
            Self::RuntimeReplayGate,
            Self::AuditSecurityGate,
            Self::BridgeCustodyGate,
            Self::WalletWatchtowerGate,
            Self::PqReservePrivacyGate,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileGate => "compile_gate",
            Self::RuntimeReplayGate => "runtime_replay_gate",
            Self::AuditSecurityGate => "audit_security_gate",
            Self::BridgeCustodyGate => "bridge_custody_gate",
            Self::WalletWatchtowerGate => "wallet_watchtower_gate",
            Self::PqReservePrivacyGate => "pq_reserve_privacy_gate",
        }
    }

    pub fn source_module_hint(self) -> &'static str {
        match self {
            Self::CompileGate => "wave89_compile_gate_archive",
            Self::RuntimeReplayGate => "wave89_runtime_replay_gate_archive",
            Self::AuditSecurityGate => "wave89_audit_security_gate_archive",
            Self::BridgeCustodyGate => "wave89_bridge_custody_gate_archive",
            Self::WalletWatchtowerGate => "wave89_wallet_watchtower_gate_archive",
            Self::PqReservePrivacyGate => "wave89_pq_reserve_privacy_gate_archive",
        }
    }

    pub fn is_compile_lane(self) -> bool {
        matches!(self, Self::CompileGate)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ArchiveEvidenceKind {
    FinalTranscriptRoot,
    CompileGateRoot,
    RuntimeReplayGateRoot,
    AuditSecurityGateRoot,
    BridgeCustodyGateRoot,
    WalletWatchtowerGateRoot,
    PqReservePrivacyGateRoot,
    ReleaseCaptainNoGoRoot,
    OperatorPagerAckRoot,
    RollbackCommandRoot,
}

impl ArchiveEvidenceKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FinalTranscriptRoot,
            Self::CompileGateRoot,
            Self::RuntimeReplayGateRoot,
            Self::AuditSecurityGateRoot,
            Self::BridgeCustodyGateRoot,
            Self::WalletWatchtowerGateRoot,
            Self::PqReservePrivacyGateRoot,
            Self::ReleaseCaptainNoGoRoot,
            Self::OperatorPagerAckRoot,
            Self::RollbackCommandRoot,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinalTranscriptRoot => "final_transcript_root",
            Self::CompileGateRoot => "compile_gate_root",
            Self::RuntimeReplayGateRoot => "runtime_replay_gate_root",
            Self::AuditSecurityGateRoot => "audit_security_gate_root",
            Self::BridgeCustodyGateRoot => "bridge_custody_gate_root",
            Self::WalletWatchtowerGateRoot => "wallet_watchtower_gate_root",
            Self::PqReservePrivacyGateRoot => "pq_reserve_privacy_gate_root",
            Self::ReleaseCaptainNoGoRoot => "release_captain_no_go_root",
            Self::OperatorPagerAckRoot => "operator_pager_ack_root",
            Self::RollbackCommandRoot => "rollback_command_root",
        }
    }

    pub fn required(self) -> bool {
        true
    }

    pub fn lane(self) -> Option<ArchiveLane> {
        match self {
            Self::CompileGateRoot => Some(ArchiveLane::CompileGate),
            Self::RuntimeReplayGateRoot => Some(ArchiveLane::RuntimeReplayGate),
            Self::AuditSecurityGateRoot => Some(ArchiveLane::AuditSecurityGate),
            Self::BridgeCustodyGateRoot => Some(ArchiveLane::BridgeCustodyGate),
            Self::WalletWatchtowerGateRoot => Some(ArchiveLane::WalletWatchtowerGate),
            Self::PqReservePrivacyGateRoot => Some(ArchiveLane::PqReservePrivacyGate),
            Self::FinalTranscriptRoot
            | Self::ReleaseCaptainNoGoRoot
            | Self::OperatorPagerAckRoot
            | Self::RollbackCommandRoot => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReceiptKind {
    CargoCheck,
    CargoTest,
    CargoClippy,
    CargoFmt,
    RustcSmoke,
    CompilerReceipt,
    CargoMetadata,
    RuntimeFixtureReplay,
}

impl ReceiptKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CargoCheck,
            Self::CargoTest,
            Self::CargoClippy,
            Self::CargoFmt,
            Self::RustcSmoke,
            Self::CompilerReceipt,
            Self::CargoMetadata,
            Self::RuntimeFixtureReplay,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::CargoClippy => "cargo_clippy",
            Self::CargoFmt => "cargo_fmt",
            Self::RustcSmoke => "rustc_smoke",
            Self::CompilerReceipt => "compiler_receipt",
            Self::CargoMetadata => "cargo_metadata",
            Self::RuntimeFixtureReplay => "runtime_fixture_replay",
        }
    }

    pub fn is_heavy(self) -> bool {
        true
    }

    pub fn blocks_by_absence(self) -> bool {
        matches!(
            self,
            Self::CargoCheck
                | Self::CargoTest
                | Self::CargoClippy
                | Self::CargoFmt
                | Self::RustcSmoke
                | Self::CompilerReceipt
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptStatus {
    Missing,
    Deferred,
    NotRun,
    Redacted,
    ReplacedByArchiveHold,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Deferred => "deferred",
            Self::NotRun => "not_run",
            Self::Redacted => "redacted",
            Self::ReplacedByArchiveHold => "replaced_by_archive_hold",
        }
    }

    pub fn blocks_release(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenialCriterionKind {
    Wave89NoGoArchivePresent,
    CompileReceiptsMissing,
    HeavyGatesDeferred,
    CargoCheckAbsent,
    TestsAbsent,
    ClippyAbsent,
    FormatReceiptAbsent,
    RustcReceiptAbsent,
    CompilerReceiptAbsent,
    PublicRecordRootsOnly,
    OperatorHoldActive,
    NoReleaseOverride,
}

impl DenialCriterionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Wave89NoGoArchivePresent,
            Self::CompileReceiptsMissing,
            Self::HeavyGatesDeferred,
            Self::CargoCheckAbsent,
            Self::TestsAbsent,
            Self::ClippyAbsent,
            Self::FormatReceiptAbsent,
            Self::RustcReceiptAbsent,
            Self::CompilerReceiptAbsent,
            Self::PublicRecordRootsOnly,
            Self::OperatorHoldActive,
            Self::NoReleaseOverride,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave89NoGoArchivePresent => "wave89_no_go_archive_present",
            Self::CompileReceiptsMissing => "compile_receipts_missing",
            Self::HeavyGatesDeferred => "heavy_gates_deferred",
            Self::CargoCheckAbsent => "cargo_check_absent",
            Self::TestsAbsent => "tests_absent",
            Self::ClippyAbsent => "clippy_absent",
            Self::FormatReceiptAbsent => "format_receipt_absent",
            Self::RustcReceiptAbsent => "rustc_receipt_absent",
            Self::CompilerReceiptAbsent => "compiler_receipt_absent",
            Self::PublicRecordRootsOnly => "public_record_roots_only",
            Self::OperatorHoldActive => "operator_hold_active",
            Self::NoReleaseOverride => "no_release_override",
        }
    }

    pub fn severity(self) -> DenialSeverity {
        match self {
            Self::Wave89NoGoArchivePresent => DenialSeverity::Critical,
            Self::CompileReceiptsMissing
            | Self::HeavyGatesDeferred
            | Self::CargoCheckAbsent
            | Self::TestsAbsent
            | Self::ClippyAbsent
            | Self::FormatReceiptAbsent
            | Self::RustcReceiptAbsent
            | Self::CompilerReceiptAbsent => DenialSeverity::Blocker,
            Self::PublicRecordRootsOnly | Self::OperatorHoldActive | Self::NoReleaseOverride => {
                DenialSeverity::Required
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DenialSeverity {
    Required,
    Critical,
    Blocker,
}

impl DenialSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Critical => "critical",
            Self::Blocker => "blocker",
        }
    }

    pub fn weight(self) -> u64 {
        match self {
            Self::Required => 25,
            Self::Critical => 50,
            Self::Blocker => 100,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ActionKind {
    KeepReleaseHold,
    PublishRootsOnlyDenial,
    RequestCargoCheckReceipt,
    RequestCargoTestReceipt,
    RequestClippyReceipt,
    RequestFormatReceipt,
    RequestRustcReceipt,
    RequestCompilerReceipt,
    PreserveWave89Archive,
    ReconcileAfterHeavyGates,
}

impl ActionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::KeepReleaseHold,
            Self::PublishRootsOnlyDenial,
            Self::RequestCargoCheckReceipt,
            Self::RequestCargoTestReceipt,
            Self::RequestClippyReceipt,
            Self::RequestFormatReceipt,
            Self::RequestRustcReceipt,
            Self::RequestCompilerReceipt,
            Self::PreserveWave89Archive,
            Self::ReconcileAfterHeavyGates,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseHold => "keep_release_hold",
            Self::PublishRootsOnlyDenial => "publish_roots_only_denial",
            Self::RequestCargoCheckReceipt => "request_cargo_check_receipt",
            Self::RequestCargoTestReceipt => "request_cargo_test_receipt",
            Self::RequestClippyReceipt => "request_clippy_receipt",
            Self::RequestFormatReceipt => "request_format_receipt",
            Self::RequestRustcReceipt => "request_rustc_receipt",
            Self::RequestCompilerReceipt => "request_compiler_receipt",
            Self::PreserveWave89Archive => "preserve_wave89_archive",
            Self::ReconcileAfterHeavyGates => "reconcile_after_heavy_gates",
        }
    }

    pub fn role(self) -> OperatorRole {
        match self {
            Self::KeepReleaseHold | Self::PublishRootsOnlyDenial => OperatorRole::ReleaseCaptain,
            Self::RequestCargoCheckReceipt
            | Self::RequestCargoTestReceipt
            | Self::RequestClippyReceipt
            | Self::RequestFormatReceipt
            | Self::RequestRustcReceipt
            | Self::RequestCompilerReceipt => OperatorRole::BuildOperator,
            Self::PreserveWave89Archive => OperatorRole::Archivist,
            Self::ReconcileAfterHeavyGates => OperatorRole::ReadinessReviewer,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OperatorRole {
    ReleaseCaptain,
    BuildOperator,
    Archivist,
    ReadinessReviewer,
}

impl OperatorRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCaptain => "release_captain",
            Self::BuildOperator => "build_operator",
            Self::Archivist => "archivist",
            Self::ReadinessReviewer => "readiness_reviewer",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinalVerdict {
    DenyProductionReadiness,
    HoldForHeavyGateReceipts,
}

impl FinalVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyProductionReadiness => "deny_production_readiness",
            Self::HoldForHeavyGateReceipts => "hold_for_heavy_gate_receipts",
        }
    }

    pub fn release_allowed(self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub wave: u64,
    pub source_archive_wave: u64,
    pub review_height: u64,
    pub source_archive_height: u64,
    pub max_archive_age_blocks: u64,
    pub min_archive_lanes: usize,
    pub min_denial_criteria: usize,
    pub min_missing_heavy_gates: usize,
    pub min_action_hints: usize,
    pub privacy_mode: String,
    pub protocol_version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wave: DEFAULT_WAVE,
            source_archive_wave: SOURCE_ARCHIVE_WAVE,
            review_height: DEFAULT_REVIEW_HEIGHT,
            source_archive_height: DEFAULT_SOURCE_ARCHIVE_HEIGHT,
            max_archive_age_blocks: DEFAULT_MAX_ARCHIVE_AGE_BLOCKS,
            min_archive_lanes: DEFAULT_MIN_ARCHIVE_LANES,
            min_denial_criteria: DEFAULT_MIN_DENIAL_CRITERIA,
            min_missing_heavy_gates: DEFAULT_MIN_MISSING_HEAVY_GATES,
            min_action_hints: DEFAULT_MIN_ACTION_HINTS,
            privacy_mode: PRIVACY_MODE.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        require_non_empty("privacy_mode", &self.privacy_mode)?;
        require_non_empty("protocol_version", &self.protocol_version)?;
        if self.wave != DEFAULT_WAVE || self.source_archive_wave != SOURCE_ARCHIVE_WAVE {
            return Err(DenialError::InvalidWave);
        }
        if self.review_height <= self.source_archive_height {
            return Err(DenialError::InvalidHeight);
        }
        let age = self
            .review_height
            .saturating_sub(self.source_archive_height);
        if age > self.max_archive_age_blocks {
            return Err(DenialError::PolicyViolation(
                "source archive age exceeds review window".to_string(),
            ));
        }
        if self.privacy_mode != PRIVACY_MODE {
            return Err(DenialError::PolicyViolation(
                "public record must remain roots only".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveEvidence {
    pub id: String,
    pub kind: ArchiveEvidenceKind,
    pub lane: Option<ArchiveLane>,
    pub root: String,
    pub archived_height: u64,
    pub source_wave: u64,
}

impl ArchiveEvidence {
    pub fn new(
        id: impl Into<String>,
        kind: ArchiveEvidenceKind,
        root: impl Into<String>,
        archived_height: u64,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            lane: kind.lane(),
            root: root.into(),
            archived_height,
            source_wave: SOURCE_ARCHIVE_WAVE,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("archive_evidence.id", &self.id)?;
        validate_root("archive_evidence.root", &self.root)?;
        if self.archived_height != DEFAULT_SOURCE_ARCHIVE_HEIGHT {
            return Err(DenialError::InvalidHeight);
        }
        if self.source_wave != SOURCE_ARCHIVE_WAVE {
            return Err(DenialError::InvalidWave);
        }
        Ok(())
    }

    pub fn record_parts(&self) -> Vec<String> {
        let lane = match self.lane {
            Some(lane) => lane.as_str(),
            None => "global",
        };
        vec![
            self.id.clone(),
            self.kind.as_str().to_string(),
            lane.to_string(),
            self.root.clone(),
            self.archived_height.to_string(),
            self.source_wave.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompileReceipt {
    pub id: String,
    pub kind: ReceiptKind,
    pub status: ReceiptStatus,
    pub evidence_root: String,
    pub deferred_until: String,
    pub heavy_gate: bool,
}

impl CompileReceipt {
    pub fn missing(kind: ReceiptKind, evidence_root: impl Into<String>) -> Self {
        Self {
            id: format!("{}-receipt", kind.as_str()),
            kind,
            status: ReceiptStatus::Missing,
            evidence_root: evidence_root.into(),
            deferred_until: "post-wave90-heavy-gate-window".to_string(),
            heavy_gate: kind.is_heavy(),
        }
    }

    pub fn deferred(kind: ReceiptKind, evidence_root: impl Into<String>) -> Self {
        Self {
            id: format!("{}-deferred-receipt", kind.as_str()),
            kind,
            status: ReceiptStatus::Deferred,
            evidence_root: evidence_root.into(),
            deferred_until: "after-release-captain-reopens-build-gates".to_string(),
            heavy_gate: kind.is_heavy(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("compile_receipt.id", &self.id)?;
        require_non_empty("compile_receipt.deferred_until", &self.deferred_until)?;
        validate_root("compile_receipt.evidence_root", &self.evidence_root)?;
        if !self.status.blocks_release() {
            return Err(DenialError::ReceiptWouldClearGate(self.id.clone()));
        }
        if !self.heavy_gate {
            return Err(DenialError::PolicyViolation(
                "compile receipt must represent a heavy gate".to_string(),
            ));
        }
        Ok(())
    }

    pub fn blocker_weight(&self) -> u64 {
        match self.status {
            ReceiptStatus::Missing => 100,
            ReceiptStatus::Deferred => 90,
            ReceiptStatus::NotRun => 85,
            ReceiptStatus::Redacted => 80,
            ReceiptStatus::ReplacedByArchiveHold => 75,
        }
    }

    pub fn record_parts(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.kind.as_str().to_string(),
            self.status.as_str().to_string(),
            self.evidence_root.clone(),
            self.deferred_until.clone(),
            self.heavy_gate.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialCriterion {
    pub id: String,
    pub kind: DenialCriterionKind,
    pub severity: DenialSeverity,
    pub evidence_root: String,
    pub satisfied: bool,
}

impl DenialCriterion {
    pub fn new(kind: DenialCriterionKind, evidence_root: impl Into<String>) -> Self {
        Self {
            id: kind.as_str().to_string(),
            kind,
            severity: kind.severity(),
            evidence_root: evidence_root.into(),
            satisfied: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("denial_criterion.id", &self.id)?;
        validate_root("denial_criterion.evidence_root", &self.evidence_root)?;
        if !self.satisfied {
            return Err(DenialError::PolicyViolation(format!(
                "criterion not satisfied: {}",
                self.id
            )));
        }
        Ok(())
    }

    pub fn record_parts(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.kind.as_str().to_string(),
            self.severity.as_str().to_string(),
            self.evidence_root.clone(),
            self.satisfied.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingHeavyGateBlocker {
    pub id: String,
    pub receipt_kind: ReceiptKind,
    pub status: ReceiptStatus,
    pub blocker_root: String,
    pub reason_code: String,
    pub release_blocking: bool,
}

impl MissingHeavyGateBlocker {
    pub fn from_receipt(receipt: &CompileReceipt) -> Self {
        Self {
            id: format!("{}-blocker", receipt.kind.as_str()),
            receipt_kind: receipt.kind,
            status: receipt.status,
            blocker_root: receipt.evidence_root.clone(),
            reason_code: format!("{}_{}", receipt.kind.as_str(), receipt.status.as_str()),
            release_blocking: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("heavy_gate_blocker.id", &self.id)?;
        require_non_empty("heavy_gate_blocker.reason_code", &self.reason_code)?;
        validate_root("heavy_gate_blocker.blocker_root", &self.blocker_root)?;
        if !self.release_blocking {
            return Err(DenialError::ReadinessWouldOpen);
        }
        Ok(())
    }

    pub fn record_parts(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.receipt_kind.as_str().to_string(),
            self.status.as_str().to_string(),
            self.blocker_root.clone(),
            self.reason_code.clone(),
            self.release_blocking.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorActionHint {
    pub id: String,
    pub action: ActionKind,
    pub role: OperatorRole,
    pub blocking_root: String,
    pub clears_release: bool,
}

impl OperatorActionHint {
    pub fn new(action: ActionKind, blocking_root: impl Into<String>) -> Self {
        Self {
            id: action.as_str().to_string(),
            action,
            role: action.role(),
            blocking_root: blocking_root.into(),
            clears_release: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("operator_action.id", &self.id)?;
        validate_root("operator_action.blocking_root", &self.blocking_root)?;
        if self.clears_release {
            return Err(DenialError::ReadinessWouldOpen);
        }
        Ok(())
    }

    pub fn record_parts(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.action.as_str().to_string(),
            self.role.as_str().to_string(),
            self.blocking_root.clone(),
            self.clears_release.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialSummary {
    pub final_verdict: FinalVerdict,
    pub release_allowed: bool,
    pub archive_evidence_count: usize,
    pub compile_receipt_count: usize,
    pub missing_heavy_gate_count: usize,
    pub denial_criterion_count: usize,
    pub operator_action_count: usize,
    pub denial_weight: u64,
    pub summary_root: String,
}

impl DenialSummary {
    pub fn validate(&self) -> Result<()> {
        validate_root("denial_summary.summary_root", &self.summary_root)?;
        if self.release_allowed || self.final_verdict.release_allowed() {
            return Err(DenialError::ReadinessWouldOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub protocol_version: String,
    pub schema_version: u64,
    pub wave: u64,
    pub source_archive_wave: u64,
    pub privacy_mode: String,
    pub final_verdict: FinalVerdict,
    pub release_allowed: bool,
    pub archive_root: String,
    pub compile_receipts_root: String,
    pub blockers_root: String,
    pub criteria_root: String,
    pub operator_actions_root: String,
    pub summary_root: String,
    pub state_root: String,
}

impl PublicRecord {
    pub fn validate(&self) -> Result<()> {
        require_non_empty("public_record.protocol_version", &self.protocol_version)?;
        if self.privacy_mode != PRIVACY_MODE {
            return Err(DenialError::PolicyViolation(
                "public record privacy mode drift".to_string(),
            ));
        }
        validate_root("public_record.archive_root", &self.archive_root)?;
        validate_root(
            "public_record.compile_receipts_root",
            &self.compile_receipts_root,
        )?;
        validate_root("public_record.blockers_root", &self.blockers_root)?;
        validate_root("public_record.criteria_root", &self.criteria_root)?;
        validate_root(
            "public_record.operator_actions_root",
            &self.operator_actions_root,
        )?;
        validate_root("public_record.summary_root", &self.summary_root)?;
        validate_root("public_record.state_root", &self.state_root)?;
        if self.release_allowed || self.final_verdict.release_allowed() {
            return Err(DenialError::ReadinessWouldOpen);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub archive_evidence: BTreeMap<String, ArchiveEvidence>,
    pub compile_receipts: BTreeMap<String, CompileReceipt>,
    pub denial_criteria: BTreeMap<String, DenialCriterion>,
    pub missing_heavy_gate_blockers: BTreeMap<String, MissingHeavyGateBlocker>,
    pub operator_actions: BTreeMap<String, OperatorActionHint>,
    pub final_verdict: FinalVerdict,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let mut state = Self {
            config,
            archive_evidence: BTreeMap::new(),
            compile_receipts: BTreeMap::new(),
            denial_criteria: BTreeMap::new(),
            missing_heavy_gate_blockers: BTreeMap::new(),
            operator_actions: BTreeMap::new(),
            final_verdict: FinalVerdict::DenyProductionReadiness,
        };
        state.seed_wave89_archive()?;
        state.seed_compile_receipts()?;
        state.seed_denial_criteria()?;
        state.seed_missing_heavy_gate_blockers()?;
        state.seed_operator_actions()?;
        state.validate()?;
        Ok(state)
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            protocol_version: self.config.protocol_version.clone(),
            schema_version: SCHEMA_VERSION,
            wave: self.config.wave,
            source_archive_wave: self.config.source_archive_wave,
            privacy_mode: self.config.privacy_mode.clone(),
            final_verdict: self.final_verdict,
            release_allowed: false,
            archive_root: self.archive_root(),
            compile_receipts_root: self.compile_receipts_root(),
            blockers_root: self.blockers_root(),
            criteria_root: self.criteria_root(),
            operator_actions_root: self.operator_actions_root(),
            summary_root: self.summary().summary_root,
            state_root: self.state_root(),
        }
    }

    pub fn state_root(&self) -> String {
        root_from_parts(
            "wave90_state",
            &[
                self.config.protocol_version.clone(),
                self.config.wave.to_string(),
                self.config.source_archive_wave.to_string(),
                self.archive_root(),
                self.compile_receipts_root(),
                self.criteria_root(),
                self.blockers_root(),
                self.operator_actions_root(),
                self.final_verdict.as_str().to_string(),
                false.to_string(),
            ],
        )
    }

    pub fn summary(&self) -> DenialSummary {
        let denial_weight = self
            .denial_criteria
            .values()
            .map(|criterion| criterion.severity.weight())
            .sum::<u64>()
            + self
                .compile_receipts
                .values()
                .map(CompileReceipt::blocker_weight)
                .sum::<u64>();
        let parts = vec![
            self.final_verdict.as_str().to_string(),
            false.to_string(),
            self.archive_evidence.len().to_string(),
            self.compile_receipts.len().to_string(),
            self.missing_heavy_gate_blockers.len().to_string(),
            self.denial_criteria.len().to_string(),
            self.operator_actions.len().to_string(),
            denial_weight.to_string(),
        ];
        DenialSummary {
            final_verdict: self.final_verdict,
            release_allowed: false,
            archive_evidence_count: self.archive_evidence.len(),
            compile_receipt_count: self.compile_receipts.len(),
            missing_heavy_gate_count: self.missing_heavy_gate_blockers.len(),
            denial_criterion_count: self.denial_criteria.len(),
            operator_action_count: self.operator_actions.len(),
            denial_weight,
            summary_root: root_from_parts("wave90_denial_summary", &parts),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.final_verdict.release_allowed() {
            return Err(DenialError::ReadinessWouldOpen);
        }
        validate_unique(self.archive_evidence.keys().cloned())?;
        validate_unique(self.compile_receipts.keys().cloned())?;
        validate_unique(self.denial_criteria.keys().cloned())?;
        validate_unique(self.missing_heavy_gate_blockers.keys().cloned())?;
        validate_unique(self.operator_actions.keys().cloned())?;
        for evidence in self.archive_evidence.values() {
            evidence.validate()?;
        }
        for receipt in self.compile_receipts.values() {
            receipt.validate()?;
        }
        for criterion in self.denial_criteria.values() {
            criterion.validate()?;
        }
        for blocker in self.missing_heavy_gate_blockers.values() {
            blocker.validate()?;
        }
        for action in self.operator_actions.values() {
            action.validate()?;
        }
        self.require_archive_lanes()?;
        self.require_denial_criteria()?;
        self.require_compile_receipts()?;
        self.require_blockers()?;
        self.require_actions()?;
        self.summary().validate()?;
        self.public_record().validate()?;
        Ok(())
    }

    pub fn archive_root(&self) -> String {
        let leaves = self
            .archive_evidence
            .values()
            .map(|evidence| root_from_parts("archive_evidence", &evidence.record_parts()))
            .collect::<Vec<_>>();
        root_from_parts("wave89_archive_evidence_set", &leaves)
    }

    pub fn compile_receipts_root(&self) -> String {
        let leaves = self
            .compile_receipts
            .values()
            .map(|receipt| root_from_parts("compile_receipt", &receipt.record_parts()))
            .collect::<Vec<_>>();
        root_from_parts("wave90_compile_receipt_set", &leaves)
    }

    pub fn criteria_root(&self) -> String {
        let leaves = self
            .denial_criteria
            .values()
            .map(|criterion| root_from_parts("denial_criterion", &criterion.record_parts()))
            .collect::<Vec<_>>();
        root_from_parts("wave90_denial_criteria_set", &leaves)
    }

    pub fn blockers_root(&self) -> String {
        let leaves = self
            .missing_heavy_gate_blockers
            .values()
            .map(|blocker| root_from_parts("heavy_gate_blocker", &blocker.record_parts()))
            .collect::<Vec<_>>();
        root_from_parts("wave90_missing_heavy_gate_blockers", &leaves)
    }

    pub fn operator_actions_root(&self) -> String {
        let leaves = self
            .operator_actions
            .values()
            .map(|action| root_from_parts("operator_action_hint", &action.record_parts()))
            .collect::<Vec<_>>();
        root_from_parts("wave90_operator_action_hints", &leaves)
    }

    pub fn insert_archive_evidence(&mut self, evidence: ArchiveEvidence) -> Result<()> {
        evidence.validate()?;
        if self.archive_evidence.contains_key(&evidence.id) {
            return Err(DenialError::DuplicateId(evidence.id));
        }
        self.archive_evidence.insert(evidence.id.clone(), evidence);
        Ok(())
    }

    pub fn insert_compile_receipt(&mut self, receipt: CompileReceipt) -> Result<()> {
        receipt.validate()?;
        if self.compile_receipts.contains_key(&receipt.id) {
            return Err(DenialError::DuplicateId(receipt.id));
        }
        self.compile_receipts.insert(receipt.id.clone(), receipt);
        Ok(())
    }

    pub fn insert_denial_criterion(&mut self, criterion: DenialCriterion) -> Result<()> {
        criterion.validate()?;
        if self.denial_criteria.contains_key(&criterion.id) {
            return Err(DenialError::DuplicateId(criterion.id));
        }
        self.denial_criteria.insert(criterion.id.clone(), criterion);
        Ok(())
    }

    pub fn insert_blocker(&mut self, blocker: MissingHeavyGateBlocker) -> Result<()> {
        blocker.validate()?;
        if self.missing_heavy_gate_blockers.contains_key(&blocker.id) {
            return Err(DenialError::DuplicateId(blocker.id));
        }
        self.missing_heavy_gate_blockers
            .insert(blocker.id.clone(), blocker);
        Ok(())
    }

    pub fn insert_operator_action(&mut self, action: OperatorActionHint) -> Result<()> {
        action.validate()?;
        if self.operator_actions.contains_key(&action.id) {
            return Err(DenialError::DuplicateId(action.id));
        }
        self.operator_actions.insert(action.id.clone(), action);
        Ok(())
    }

    pub fn missing_receipt_kinds(&self) -> Vec<ReceiptKind> {
        self.compile_receipts
            .values()
            .filter(|receipt| receipt.status.blocks_release())
            .map(|receipt| receipt.kind)
            .collect()
    }

    pub fn denied_readiness(&self) -> bool {
        !self.final_verdict.release_allowed()
            && !self.public_record().release_allowed
            && self.missing_heavy_gate_blockers.len() >= self.config.min_missing_heavy_gates
    }

    fn seed_wave89_archive(&mut self) -> Result<()> {
        for kind in ArchiveEvidenceKind::all() {
            let root = synthetic_root("wave89_archive", &[kind.as_str()]);
            let evidence =
                ArchiveEvidence::new(kind.as_str(), kind, root, self.config.source_archive_height);
            self.insert_archive_evidence(evidence)?;
        }
        Ok(())
    }

    fn seed_compile_receipts(&mut self) -> Result<()> {
        for kind in ReceiptKind::all() {
            let root = synthetic_root("wave90_missing_compile_receipt", &[kind.as_str()]);
            let receipt = if kind.blocks_by_absence() {
                CompileReceipt::missing(kind, root)
            } else {
                CompileReceipt::deferred(kind, root)
            };
            self.insert_compile_receipt(receipt)?;
        }
        Ok(())
    }

    fn seed_denial_criteria(&mut self) -> Result<()> {
        for kind in DenialCriterionKind::all() {
            let root = synthetic_root("wave90_denial_criterion", &[kind.as_str()]);
            self.insert_denial_criterion(DenialCriterion::new(kind, root))?;
        }
        Ok(())
    }

    fn seed_missing_heavy_gate_blockers(&mut self) -> Result<()> {
        let receipts = self.compile_receipts.values().cloned().collect::<Vec<_>>();
        for receipt in receipts {
            self.insert_blocker(MissingHeavyGateBlocker::from_receipt(&receipt))?;
        }
        Ok(())
    }

    fn seed_operator_actions(&mut self) -> Result<()> {
        for action in ActionKind::all() {
            let root = synthetic_root("wave90_operator_action", &[action.as_str()]);
            self.insert_operator_action(OperatorActionHint::new(action, root))?;
        }
        Ok(())
    }

    fn require_archive_lanes(&self) -> Result<()> {
        let lanes = self
            .archive_evidence
            .values()
            .filter_map(|evidence| evidence.lane)
            .collect::<BTreeSet<_>>();
        if lanes.len() < self.config.min_archive_lanes {
            return Err(DenialError::MissingRequiredEvidence("wave89 archive lanes"));
        }
        for kind in ArchiveEvidenceKind::all() {
            if kind.required()
                && !self
                    .archive_evidence
                    .values()
                    .any(|evidence| evidence.kind == kind)
            {
                return Err(DenialError::MissingRequiredEvidence(kind.as_str()));
            }
        }
        Ok(())
    }

    fn require_denial_criteria(&self) -> Result<()> {
        if self.denial_criteria.len() < self.config.min_denial_criteria {
            return Err(DenialError::InsufficientCriteria {
                required: self.config.min_denial_criteria,
                actual: self.denial_criteria.len(),
            });
        }
        for kind in DenialCriterionKind::all() {
            if !self
                .denial_criteria
                .values()
                .any(|criterion| criterion.kind == kind)
            {
                return Err(DenialError::MissingRequiredEvidence(kind.as_str()));
            }
        }
        Ok(())
    }

    fn require_compile_receipts(&self) -> Result<()> {
        for kind in ReceiptKind::all() {
            if !self
                .compile_receipts
                .values()
                .any(|receipt| receipt.kind == kind)
            {
                return Err(DenialError::MissingRequiredReceipt(kind.as_str()));
            }
        }
        if self
            .compile_receipts
            .values()
            .any(|receipt| !receipt.status.blocks_release())
        {
            return Err(DenialError::ReadinessWouldOpen);
        }
        Ok(())
    }

    fn require_blockers(&self) -> Result<()> {
        if self.missing_heavy_gate_blockers.len() < self.config.min_missing_heavy_gates {
            return Err(DenialError::InsufficientBlockers {
                required: self.config.min_missing_heavy_gates,
                actual: self.missing_heavy_gate_blockers.len(),
            });
        }
        for kind in ReceiptKind::all() {
            if kind.blocks_by_absence()
                && !self
                    .missing_heavy_gate_blockers
                    .values()
                    .any(|blocker| blocker.receipt_kind == kind)
            {
                return Err(DenialError::UnknownBlocker(kind.as_str().to_string()));
            }
        }
        Ok(())
    }

    fn require_actions(&self) -> Result<()> {
        if self.operator_actions.len() < self.config.min_action_hints {
            return Err(DenialError::InsufficientActions {
                required: self.config.min_action_hints,
                actual: self.operator_actions.len(),
            });
        }
        for action in ActionKind::all() {
            if !self
                .operator_actions
                .values()
                .any(|hint| hint.action == action)
            {
                return Err(DenialError::PolicyViolation(format!(
                    "missing operator action {}",
                    action.as_str()
                )));
            }
        }
        Ok(())
    }
}

pub fn devnet() -> Runtime {
    match State::new(Config::default()) {
        Ok(state) => state,
        Err(error) => fallback_runtime(error),
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn final_verdict() -> FinalVerdict {
    devnet().final_verdict
}

pub fn denied_readiness() -> bool {
    devnet().denied_readiness()
}

pub fn missing_heavy_gate_blockers() -> Vec<MissingHeavyGateBlocker> {
    devnet()
        .missing_heavy_gate_blockers
        .values()
        .cloned()
        .collect()
}

pub fn compile_receipts() -> Vec<CompileReceipt> {
    devnet().compile_receipts.values().cloned().collect()
}

pub fn archive_evidence_roots() -> BTreeMap<String, String> {
    devnet()
        .archive_evidence
        .values()
        .map(|evidence| (evidence.id.clone(), evidence.root.clone()))
        .collect()
}

fn fallback_runtime(error: DenialError) -> State {
    let config = Config::default();
    let mut archive_evidence = BTreeMap::new();
    let mut compile_receipts = BTreeMap::new();
    let mut denial_criteria = BTreeMap::new();
    let mut missing_heavy_gate_blockers = BTreeMap::new();
    let mut operator_actions = BTreeMap::new();
    let fallback_root = synthetic_root("wave90_fallback_denial", &[&error.to_string()]);
    let evidence = ArchiveEvidence::new(
        "fallback_wave89_no_go_archive",
        ArchiveEvidenceKind::FinalTranscriptRoot,
        fallback_root.clone(),
        DEFAULT_SOURCE_ARCHIVE_HEIGHT,
    );
    archive_evidence.insert(evidence.id.clone(), evidence);
    for kind in ReceiptKind::all() {
        let receipt = CompileReceipt::missing(kind, fallback_root.clone());
        missing_heavy_gate_blockers.insert(
            format!("{}-blocker", kind.as_str()),
            MissingHeavyGateBlocker::from_receipt(&receipt),
        );
        compile_receipts.insert(receipt.id.clone(), receipt);
    }
    for kind in DenialCriterionKind::all() {
        let criterion = DenialCriterion::new(kind, fallback_root.clone());
        denial_criteria.insert(criterion.id.clone(), criterion);
    }
    for action in ActionKind::all() {
        let hint = OperatorActionHint::new(action, fallback_root.clone());
        operator_actions.insert(hint.id.clone(), hint);
    }
    State {
        config,
        archive_evidence,
        compile_receipts,
        denial_criteria,
        missing_heavy_gate_blockers,
        operator_actions,
        final_verdict: FinalVerdict::HoldForHeavyGateReceipts,
    }
}

fn require_non_empty(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(DenialError::EmptyField(field))
    } else {
        Ok(())
    }
}

fn validate_root(field: &'static str, value: &str) -> Result<()> {
    require_non_empty(field, value)?;
    let valid_prefix = value.starts_with("root:");
    let valid_len = value.len() >= 21;
    let valid_chars = value
        .chars()
        .all(|ch| ch.is_ascii_hexdigit() || ch == ':' || ch == '_' || ch == '-');
    if valid_prefix && valid_len && valid_chars {
        Ok(())
    } else {
        Err(DenialError::InvalidRoot {
            field,
            value: value.to_string(),
        })
    }
}

fn validate_unique(values: impl IntoIterator<Item = String>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            return Err(DenialError::DuplicateId(value));
        }
    }
    Ok(())
}

fn synthetic_root(domain: &str, values: &[&str]) -> String {
    let parts = values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    root_from_parts(domain, &parts)
}

fn root_from_parts(domain: &str, parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    PROTOCOL_VERSION.hash(&mut hasher);
    domain.hash(&mut hasher);
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("root:{:016x}", hasher.finish())
}
