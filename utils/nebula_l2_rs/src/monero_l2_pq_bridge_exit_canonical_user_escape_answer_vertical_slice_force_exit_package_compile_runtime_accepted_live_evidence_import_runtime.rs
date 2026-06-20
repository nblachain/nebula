use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileRuntimeAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-compile-runtime-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const IMPORT_SUITE: &str =
    "monero-l2-pq-force-exit-package-compile-runtime-accepted-live-evidence-import-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-devnet-v1";
pub const DEFAULT_FORCE_EXIT_PACKAGE_ID: &str =
    "force-exit-package-compile-runtime-accepted-live-evidence-import-devnet-0001";
pub const DEFAULT_GOVERNANCE_EPOCH: u64 = 81;
pub const DEFAULT_L2_HEIGHT: u64 = 884_480;
pub const DEFAULT_SOURCE_HEIGHT: u64 = 2_771_980;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 96;
pub const DEFAULT_MIN_ACCEPTED_RECEIPTS: u64 = 5;
pub const DEFAULT_MIN_ACCEPTED_FAMILIES: u64 = 5;
pub const DEFAULT_MIN_DISTINCT_OPERATORS: u64 = 3;
pub const DEFAULT_MIN_GOVERNANCE_SIGNATURES: u64 = 5;
pub const DEFAULT_MIN_IMPORT_BATCHES: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub import_suite: String,
    pub vertical_slice_id: String,
    pub force_exit_package_id: String,
    pub governance_epoch: u64,
    pub l2_height: u64,
    pub source_height: u64,
    pub max_evidence_age_blocks: u64,
    pub min_accepted_receipts: u64,
    pub min_accepted_families: u64,
    pub min_distinct_operators: u64,
    pub min_governance_signatures: u64,
    pub min_import_batches: u64,
    pub require_compile_receipt: bool,
    pub require_cargo_check_receipt: bool,
    pub require_rustfmt_receipt: bool,
    pub require_clippy_receipt: bool,
    pub require_cargo_test_receipt: bool,
    pub require_activation_root: bool,
    pub require_release_manifest_root: bool,
    pub require_governance_binding_root: bool,
    pub require_evidence_payload_root: bool,
    pub reject_stale_evidence: bool,
    pub fail_closed_on_missing_root: bool,
    pub fail_closed_on_rejected_receipt: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            import_suite: IMPORT_SUITE.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            force_exit_package_id: DEFAULT_FORCE_EXIT_PACKAGE_ID.to_string(),
            governance_epoch: DEFAULT_GOVERNANCE_EPOCH,
            l2_height: DEFAULT_L2_HEIGHT,
            source_height: DEFAULT_SOURCE_HEIGHT,
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            min_accepted_receipts: DEFAULT_MIN_ACCEPTED_RECEIPTS,
            min_accepted_families: DEFAULT_MIN_ACCEPTED_FAMILIES,
            min_distinct_operators: DEFAULT_MIN_DISTINCT_OPERATORS,
            min_governance_signatures: DEFAULT_MIN_GOVERNANCE_SIGNATURES,
            min_import_batches: DEFAULT_MIN_IMPORT_BATCHES,
            require_compile_receipt: true,
            require_cargo_check_receipt: true,
            require_rustfmt_receipt: true,
            require_clippy_receipt: true,
            require_cargo_test_receipt: true,
            require_activation_root: true,
            require_release_manifest_root: true,
            require_governance_binding_root: true,
            require_evidence_payload_root: true,
            reject_stale_evidence: true,
            fail_closed_on_missing_root: true,
            fail_closed_on_rejected_receipt: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_family_count(&self) -> u64 {
        [
            self.require_compile_receipt,
            self.require_cargo_check_receipt,
            self.require_rustfmt_receipt,
            self.require_clippy_receipt,
            self.require_cargo_test_receipt,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn effective_min_accepted_families(&self) -> u64 {
        self.min_accepted_families
            .max(self.required_family_count())
            .min(EvidenceFamily::ordered().len() as u64)
    }

    pub fn required_root_count(&self) -> u64 {
        [
            self.require_activation_root,
            self.require_release_manifest_root,
            self.require_governance_binding_root,
            self.require_evidence_payload_root,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "import_suite": self.import_suite,
            "vertical_slice_id": self.vertical_slice_id,
            "force_exit_package_id": self.force_exit_package_id,
            "governance_epoch": self.governance_epoch,
            "l2_height": self.l2_height,
            "source_height": self.source_height,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "min_accepted_receipts": self.min_accepted_receipts,
            "min_accepted_families": self.min_accepted_families,
            "effective_min_accepted_families": self.effective_min_accepted_families(),
            "min_distinct_operators": self.min_distinct_operators,
            "min_governance_signatures": self.min_governance_signatures,
            "min_import_batches": self.min_import_batches,
            "required_family_count": self.required_family_count(),
            "required_root_count": self.required_root_count(),
            "require_compile_receipt": self.require_compile_receipt,
            "require_cargo_check_receipt": self.require_cargo_check_receipt,
            "require_rustfmt_receipt": self.require_rustfmt_receipt,
            "require_clippy_receipt": self.require_clippy_receipt,
            "require_cargo_test_receipt": self.require_cargo_test_receipt,
            "require_activation_root": self.require_activation_root,
            "require_release_manifest_root": self.require_release_manifest_root,
            "require_governance_binding_root": self.require_governance_binding_root,
            "require_evidence_payload_root": self.require_evidence_payload_root,
            "reject_stale_evidence": self.reject_stale_evidence,
            "fail_closed_on_missing_root": self.fail_closed_on_missing_root,
            "fail_closed_on_rejected_receipt": self.fail_closed_on_rejected_receipt,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFamily {
    Compile,
    CargoCheck,
    Rustfmt,
    Clippy,
    CargoTest,
}

impl EvidenceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::CargoCheck => "cargo_check",
            Self::Rustfmt => "rustfmt",
            Self::Clippy => "clippy",
            Self::CargoTest => "cargo_test",
        }
    }

    pub fn required_by(self, config: &Config) -> bool {
        match self {
            Self::Compile => config.require_compile_receipt,
            Self::CargoCheck => config.require_cargo_check_receipt,
            Self::Rustfmt => config.require_rustfmt_receipt,
            Self::Clippy => config.require_clippy_receipt,
            Self::CargoTest => config.require_cargo_test_receipt,
        }
    }

    pub fn ordered() -> &'static [Self] {
        &[
            Self::Compile,
            Self::CargoCheck,
            Self::Rustfmt,
            Self::Clippy,
            Self::CargoTest,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveEvidenceStatus {
    AcceptedLive,
    AcceptedReplayOnly,
    Rejected,
    MissingRoot,
    Stale,
    GovernanceHeld,
}

impl LiveEvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedLive => "accepted_live",
            Self::AcceptedReplayOnly => "accepted_replay_only",
            Self::Rejected => "rejected",
            Self::MissingRoot => "missing_root",
            Self::Stale => "stale",
            Self::GovernanceHeld => "governance_held",
        }
    }

    pub fn imports(self) -> bool {
        matches!(self, Self::AcceptedLive | Self::AcceptedReplayOnly)
    }

    pub fn live(self) -> bool {
        matches!(self, Self::AcceptedLive)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportStatus {
    Imported,
    HeldMissingRoot,
    HeldStale,
    HeldRejected,
    HeldDuplicateFamily,
    HeldPolicyMismatch,
}

impl ImportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Imported => "imported",
            Self::HeldMissingRoot => "held_missing_root",
            Self::HeldStale => "held_stale",
            Self::HeldRejected => "held_rejected",
            Self::HeldDuplicateFamily => "held_duplicate_family",
            Self::HeldPolicyMismatch => "held_policy_mismatch",
        }
    }

    pub fn accepted(self) -> bool {
        matches!(self, Self::Imported)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    Go,
    NoGo,
}

impl GovernanceDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }

    pub fn permits_release(self) -> bool {
        matches!(self, Self::Go)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedLiveEvidence {
    pub evidence_id: String,
    pub family: EvidenceFamily,
    pub operator_id: String,
    pub command_label: String,
    pub activation_root: String,
    pub release_manifest_root: String,
    pub governance_binding_root: String,
    pub evidence_payload_root: String,
    pub stdout_root: String,
    pub stderr_root: String,
    pub source_height: u64,
    pub imported_at_l2_height: u64,
    pub status: LiveEvidenceStatus,
    pub status_reason: String,
}

impl AcceptedLiveEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        evidence_id: &str,
        family: EvidenceFamily,
        operator_id: &str,
        command_label: &str,
        activation_root: &str,
        release_manifest_root: &str,
        governance_binding_root: &str,
        evidence_payload_root: &str,
        stdout_root: &str,
        stderr_root: &str,
        source_height: u64,
        imported_at_l2_height: u64,
        status: LiveEvidenceStatus,
        status_reason: &str,
    ) -> Self {
        Self {
            evidence_id: evidence_id.to_string(),
            family,
            operator_id: operator_id.to_string(),
            command_label: command_label.to_string(),
            activation_root: activation_root.to_string(),
            release_manifest_root: release_manifest_root.to_string(),
            governance_binding_root: governance_binding_root.to_string(),
            evidence_payload_root: evidence_payload_root.to_string(),
            stdout_root: stdout_root.to_string(),
            stderr_root: stderr_root.to_string(),
            source_height,
            imported_at_l2_height,
            status,
            status_reason: status_reason.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "evidence_id": self.evidence_id,
            "family": self.family.as_str(),
            "operator_id": self.operator_id,
            "command_label": self.command_label,
            "activation_root": self.activation_root,
            "release_manifest_root": self.release_manifest_root,
            "governance_binding_root": self.governance_binding_root,
            "evidence_payload_root": self.evidence_payload_root,
            "stdout_root": self.stdout_root,
            "stderr_root": self.stderr_root,
            "source_height": self.source_height,
            "imported_at_l2_height": self.imported_at_l2_height,
            "status": self.status.as_str(),
            "status_reason": self.status_reason,
        })
    }

    pub fn root(&self) -> String {
        record_root("accepted-live-evidence", &self.public_record())
    }

    pub fn age_blocks(&self, config: &Config) -> u64 {
        config.source_height.saturating_sub(self.source_height)
    }

    pub fn missing_required_root_count(&self, config: &Config) -> u64 {
        let mut missing = 0;
        if config.require_activation_root && self.activation_root.is_empty() {
            missing += 1;
        }
        if config.require_release_manifest_root && self.release_manifest_root.is_empty() {
            missing += 1;
        }
        if config.require_governance_binding_root && self.governance_binding_root.is_empty() {
            missing += 1;
        }
        if config.require_evidence_payload_root && self.evidence_payload_root.is_empty() {
            missing += 1;
        }
        missing
    }

    pub fn import_status(&self, config: &Config, seen_family: bool) -> ImportStatus {
        if seen_family {
            return ImportStatus::HeldDuplicateFamily;
        }
        if !self.family.required_by(config) {
            return ImportStatus::HeldPolicyMismatch;
        }
        if config.fail_closed_on_missing_root && self.missing_required_root_count(config) > 0 {
            return ImportStatus::HeldMissingRoot;
        }
        if config.reject_stale_evidence && self.age_blocks(config) > config.max_evidence_age_blocks
        {
            return ImportStatus::HeldStale;
        }
        if config.fail_closed_on_rejected_receipt && !self.status.imports() {
            return ImportStatus::HeldRejected;
        }
        ImportStatus::Imported
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceImport {
    pub import_id: String,
    pub evidence_id: String,
    pub family: EvidenceFamily,
    pub operator_id: String,
    pub evidence_root: String,
    pub status: ImportStatus,
    pub imported_live: bool,
    pub age_blocks: u64,
    pub missing_required_roots: u64,
    pub hold_reason: String,
}

impl EvidenceImport {
    pub fn from_evidence(
        evidence: &AcceptedLiveEvidence,
        config: &Config,
        seen_family: bool,
    ) -> Self {
        let status = evidence.import_status(config, seen_family);
        let missing_required_roots = evidence.missing_required_root_count(config);
        let age_blocks = evidence.age_blocks(config);
        let hold_reason = import_hold_reason(status, missing_required_roots, age_blocks, config);
        let evidence_root = evidence.root();
        let import_id = evidence_import_id(evidence.family, &evidence.evidence_id, &evidence_root);
        Self {
            import_id,
            evidence_id: evidence.evidence_id.clone(),
            family: evidence.family,
            operator_id: evidence.operator_id.clone(),
            evidence_root,
            status,
            imported_live: evidence.status.live(),
            age_blocks,
            missing_required_roots,
            hold_reason,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "import_id": self.import_id,
            "evidence_id": self.evidence_id,
            "family": self.family.as_str(),
            "operator_id": self.operator_id,
            "evidence_root": self.evidence_root,
            "status": self.status.as_str(),
            "imported_live": self.imported_live,
            "age_blocks": self.age_blocks,
            "missing_required_roots": self.missing_required_roots,
            "hold_reason": self.hold_reason,
        })
    }

    pub fn root(&self) -> String {
        record_root("evidence-import", &self.public_record())
    }

    pub fn accepted(&self) -> bool {
        self.status.accepted()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportBatch {
    pub batch_id: String,
    pub batch_label: String,
    pub imported_by: String,
    pub import_ids: Vec<String>,
    pub import_root: String,
    pub accepted_import_count: u64,
    pub held_import_count: u64,
    pub accepted_family_root: String,
    pub accepted_operator_root: String,
    pub batch_governance_note: String,
}

impl ImportBatch {
    pub fn new(batch_label: &str, imported_by: &str, imports: &[EvidenceImport]) -> Self {
        let import_ids = imports
            .iter()
            .map(|import| import.import_id.clone())
            .collect::<Vec<_>>();
        let import_root = imports_root(imports);
        let accepted_import_count =
            imports.iter().filter(|import| import.accepted()).count() as u64;
        let held_import_count = imports.len() as u64 - accepted_import_count;
        let accepted_family_root = accepted_family_root(imports);
        let accepted_operator_root = accepted_operator_root(imports);
        let batch_id = import_batch_id(batch_label, &import_root, accepted_import_count);
        let batch_governance_note = if held_import_count == 0 {
            "all_imports_available_for_final_governance".to_string()
        } else {
            "held_imports_excluded_from_final_governance".to_string()
        };
        Self {
            batch_id,
            batch_label: batch_label.to_string(),
            imported_by: imported_by.to_string(),
            import_ids,
            import_root,
            accepted_import_count,
            held_import_count,
            accepted_family_root,
            accepted_operator_root,
            batch_governance_note,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "batch_id": self.batch_id,
            "batch_label": self.batch_label,
            "imported_by": self.imported_by,
            "import_ids": self.import_ids,
            "import_root": self.import_root,
            "accepted_import_count": self.accepted_import_count,
            "held_import_count": self.held_import_count,
            "accepted_family_root": self.accepted_family_root,
            "accepted_operator_root": self.accepted_operator_root,
            "batch_governance_note": self.batch_governance_note,
        })
    }

    pub fn root(&self) -> String {
        record_root("import-batch", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GovernanceSignature {
    pub signer_id: String,
    pub signer_role: String,
    pub evidence_import_root: String,
    pub signature_root: String,
    pub accepted: bool,
}

impl GovernanceSignature {
    pub fn new(
        signer_id: &str,
        signer_role: &str,
        evidence_import_root: &str,
        signature_root: &str,
        accepted: bool,
    ) -> Self {
        Self {
            signer_id: signer_id.to_string(),
            signer_role: signer_role.to_string(),
            evidence_import_root: evidence_import_root.to_string(),
            signature_root: signature_root.to_string(),
            accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signer_id": self.signer_id,
            "signer_role": self.signer_role,
            "evidence_import_root": self.evidence_import_root,
            "signature_root": self.signature_root,
            "accepted": self.accepted,
        })
    }

    pub fn root(&self) -> String {
        record_root("governance-signature", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImportSummary {
    pub accepted_receipt_count: u64,
    pub live_receipt_count: u64,
    pub replay_only_receipt_count: u64,
    pub held_receipt_count: u64,
    pub accepted_family_count: u64,
    pub missing_required_family_count: u64,
    pub distinct_operator_count: u64,
    pub import_batch_count: u64,
    pub accepted_batch_count: u64,
    pub governance_signature_count: u64,
    pub missing_required_root_count: u64,
    pub accepted_evidence_root: String,
    pub held_evidence_root: String,
    pub import_root: String,
    pub import_batch_root: String,
    pub accepted_family_root: String,
    pub missing_family_root: String,
    pub operator_root: String,
    pub governance_signature_root: String,
}

impl ImportSummary {
    pub fn from_state(
        config: &Config,
        evidence: &[AcceptedLiveEvidence],
        imports: &[EvidenceImport],
        batches: &[ImportBatch],
        signatures: &[GovernanceSignature],
    ) -> Self {
        let accepted_imports = imports
            .iter()
            .filter(|import| import.accepted())
            .cloned()
            .collect::<Vec<_>>();
        let held_imports = imports
            .iter()
            .filter(|import| !import.accepted())
            .cloned()
            .collect::<Vec<_>>();
        let accepted_receipt_count = accepted_imports.len() as u64;
        let live_receipt_count = accepted_imports
            .iter()
            .filter(|import| import.imported_live)
            .count() as u64;
        let replay_only_receipt_count = accepted_receipt_count.saturating_sub(live_receipt_count);
        let held_receipt_count = held_imports.len() as u64;
        let accepted_family_count = distinct_family_count(&accepted_imports);
        let missing_required_family_count =
            missing_required_family_count(config, &accepted_imports);
        let distinct_operator_count = distinct_operator_count(&accepted_imports);
        let import_batch_count = batches.len() as u64;
        let accepted_batch_count = batches
            .iter()
            .filter(|batch| batch.held_import_count == 0 && batch.accepted_import_count > 0)
            .count() as u64;
        let governance_signature_count = signatures
            .iter()
            .filter(|signature| signature.accepted)
            .count() as u64;
        let missing_required_root_count = evidence
            .iter()
            .map(|entry| entry.missing_required_root_count(config))
            .sum();
        Self {
            accepted_receipt_count,
            live_receipt_count,
            replay_only_receipt_count,
            held_receipt_count,
            accepted_family_count,
            missing_required_family_count,
            distinct_operator_count,
            import_batch_count,
            accepted_batch_count,
            governance_signature_count,
            missing_required_root_count,
            accepted_evidence_root: imports_root(&accepted_imports),
            held_evidence_root: imports_root(&held_imports),
            import_root: imports_root(imports),
            import_batch_root: import_batches_root(batches),
            accepted_family_root: accepted_family_root(imports),
            missing_family_root: missing_family_root(config, &accepted_imports),
            operator_root: accepted_operator_root(imports),
            governance_signature_root: governance_signatures_root(signatures),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "accepted_receipt_count": self.accepted_receipt_count,
            "live_receipt_count": self.live_receipt_count,
            "replay_only_receipt_count": self.replay_only_receipt_count,
            "held_receipt_count": self.held_receipt_count,
            "accepted_family_count": self.accepted_family_count,
            "missing_required_family_count": self.missing_required_family_count,
            "distinct_operator_count": self.distinct_operator_count,
            "import_batch_count": self.import_batch_count,
            "accepted_batch_count": self.accepted_batch_count,
            "governance_signature_count": self.governance_signature_count,
            "missing_required_root_count": self.missing_required_root_count,
            "accepted_evidence_root": self.accepted_evidence_root,
            "held_evidence_root": self.held_evidence_root,
            "import_root": self.import_root,
            "import_batch_root": self.import_batch_root,
            "accepted_family_root": self.accepted_family_root,
            "missing_family_root": self.missing_family_root,
            "operator_root": self.operator_root,
            "governance_signature_root": self.governance_signature_root,
        })
    }

    pub fn root(&self) -> String {
        record_root("import-summary", &self.public_record())
    }

    pub fn threshold_failures(&self, config: &Config) -> Vec<String> {
        let mut failures = Vec::new();
        if self.accepted_receipt_count < config.min_accepted_receipts {
            failures.push("accepted_receipt_count_below_threshold".to_string());
        }
        if self.accepted_family_count < config.effective_min_accepted_families() {
            failures.push("accepted_family_count_below_threshold".to_string());
        }
        if self.missing_required_family_count > 0 {
            failures.push("required_evidence_family_missing".to_string());
        }
        if self.distinct_operator_count < config.min_distinct_operators {
            failures.push("distinct_operator_count_below_threshold".to_string());
        }
        if self.import_batch_count < config.min_import_batches {
            failures.push("import_batch_count_below_threshold".to_string());
        }
        if self.governance_signature_count < config.min_governance_signatures {
            failures.push("governance_signature_count_below_threshold".to_string());
        }
        if config.fail_closed_on_missing_root && self.missing_required_root_count > 0 {
            failures.push("required_evidence_root_missing".to_string());
        }
        if config.fail_closed_on_rejected_receipt && self.held_receipt_count > 0 {
            failures.push("held_receipt_present".to_string());
        }
        failures
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FinalGovernanceImport {
    pub governance_id: String,
    pub decision: GovernanceDecision,
    pub decision_reason: String,
    pub threshold_failures: Vec<String>,
    pub imported_summary_root: String,
    pub accepted_evidence_root: String,
    pub held_evidence_root: String,
    pub import_batch_root: String,
    pub governance_signature_root: String,
    pub state_root_before_decision: String,
    pub release_permitted: bool,
}

impl FinalGovernanceImport {
    pub fn from_state(
        config: &Config,
        summary: &ImportSummary,
        state_root_before_decision: &str,
    ) -> Self {
        let threshold_failures = summary.threshold_failures(config);
        let decision = if threshold_failures.is_empty() {
            GovernanceDecision::Go
        } else {
            GovernanceDecision::NoGo
        };
        let decision_reason = if threshold_failures.is_empty() {
            "accepted_live_compile_runtime_evidence_imported_into_final_governance".to_string()
        } else {
            threshold_failures.join(",")
        };
        let imported_summary_root = summary.root();
        let governance_id = final_governance_import_id(
            config.governance_epoch,
            decision,
            &imported_summary_root,
            state_root_before_decision,
        );
        Self {
            governance_id,
            decision,
            decision_reason,
            threshold_failures,
            imported_summary_root,
            accepted_evidence_root: summary.accepted_evidence_root.clone(),
            held_evidence_root: summary.held_evidence_root.clone(),
            import_batch_root: summary.import_batch_root.clone(),
            governance_signature_root: summary.governance_signature_root.clone(),
            state_root_before_decision: state_root_before_decision.to_string(),
            release_permitted: decision.permits_release(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "governance_id": self.governance_id,
            "decision": self.decision.as_str(),
            "decision_reason": self.decision_reason,
            "threshold_failures": self.threshold_failures,
            "imported_summary_root": self.imported_summary_root,
            "accepted_evidence_root": self.accepted_evidence_root,
            "held_evidence_root": self.held_evidence_root,
            "import_batch_root": self.import_batch_root,
            "governance_signature_root": self.governance_signature_root,
            "state_root_before_decision": self.state_root_before_decision,
            "release_permitted": self.release_permitted,
        })
    }

    pub fn root(&self) -> String {
        record_root("final-governance-import", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub accepted_live_evidence: Vec<AcceptedLiveEvidence>,
    pub imports: Vec<EvidenceImport>,
    pub import_batches: Vec<ImportBatch>,
    pub governance_signatures: Vec<GovernanceSignature>,
    pub summary: ImportSummary,
    pub final_governance_import: FinalGovernanceImport,
}

impl State {
    pub fn new(
        config: Config,
        accepted_live_evidence: Vec<AcceptedLiveEvidence>,
        import_batches: Vec<ImportBatch>,
        governance_signatures: Vec<GovernanceSignature>,
    ) -> Result<Self> {
        validate_config(&config)?;
        let imports = build_imports(&config, &accepted_live_evidence);
        let batches = if import_batches.is_empty() {
            vec![ImportBatch::new(
                "default-compile-runtime-accepted-live-evidence-import",
                "devnet-governance-importer",
                &imports,
            )]
        } else {
            import_batches
        };
        let summary = ImportSummary::from_state(
            &config,
            &accepted_live_evidence,
            &imports,
            &batches,
            &governance_signatures,
        );
        let provisional_root = provisional_state_root(
            &config,
            &accepted_live_evidence,
            &imports,
            &batches,
            &governance_signatures,
            &summary,
        );
        let final_governance_import =
            FinalGovernanceImport::from_state(&config, &summary, &provisional_root);
        Ok(Self {
            config,
            accepted_live_evidence,
            imports,
            import_batches: batches,
            governance_signatures,
            summary,
            final_governance_import,
        })
    }

    pub fn devnet() -> Result<Self> {
        let config = Config::devnet();
        let accepted_live_evidence = devnet_accepted_live_evidence(&config);
        let imports = build_imports(&config, &accepted_live_evidence);
        let import_batches = vec![ImportBatch::new(
            "devnet-wave-81-compile-runtime-live-evidence",
            "devnet-final-governance-importer",
            &imports,
        )];
        let evidence_import_root = imports_root(&imports);
        let governance_signatures = devnet_governance_signatures(&evidence_import_root);
        Self::new(
            config,
            accepted_live_evidence,
            import_batches,
            governance_signatures,
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "accepted_live_evidence": self.accepted_live_evidence
                .iter()
                .map(|entry| entry.public_record())
                .collect::<Vec<_>>(),
            "imports": self.imports
                .iter()
                .map(|entry| entry.public_record())
                .collect::<Vec<_>>(),
            "import_batches": self.import_batches
                .iter()
                .map(|entry| entry.public_record())
                .collect::<Vec<_>>(),
            "governance_signatures": self.governance_signatures
                .iter()
                .map(|entry| entry.public_record())
                .collect::<Vec<_>>(),
            "summary": self.summary.public_record(),
            "final_governance_import": self.final_governance_import.public_record(),
            "state_root": self.state_root_without_self_reference(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }

    pub fn decision(&self) -> GovernanceDecision {
        self.final_governance_import.decision
    }

    pub fn release_permitted(&self) -> bool {
        self.final_governance_import.release_permitted
    }

    pub fn accepted_imports(&self) -> Vec<EvidenceImport> {
        self.imports
            .iter()
            .filter(|import| import.accepted())
            .cloned()
            .collect()
    }

    pub fn held_imports(&self) -> Vec<EvidenceImport> {
        self.imports
            .iter()
            .filter(|import| !import.accepted())
            .cloned()
            .collect()
    }

    fn state_root_without_self_reference(&self) -> String {
        provisional_state_root(
            &self.config,
            &self.accepted_live_evidence,
            &self.imports,
            &self.import_batches,
            &self.governance_signatures,
            &self.summary,
        )
    }
}

pub fn devnet() -> Result<State> {
    State::devnet()
}

pub fn public_record() -> Result<Value> {
    State::devnet().map(|state| state.public_record())
}

pub fn state_root() -> Result<String> {
    State::devnet().map(|state| state.state_root())
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id.is_empty() {
        return Err("chain_id must not be empty".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("protocol_version mismatch".to_string());
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("schema_version mismatch".to_string());
    }
    if config.min_accepted_receipts == 0 {
        return Err("min_accepted_receipts must be non-zero".to_string());
    }
    if config.effective_min_accepted_families() == 0 {
        return Err("min_accepted_families must be non-zero".to_string());
    }
    if config.min_distinct_operators == 0 {
        return Err("min_distinct_operators must be non-zero".to_string());
    }
    if config.min_governance_signatures == 0 {
        return Err("min_governance_signatures must be non-zero".to_string());
    }
    Ok(())
}

fn build_imports(config: &Config, evidence: &[AcceptedLiveEvidence]) -> Vec<EvidenceImport> {
    let mut seen_families: Vec<EvidenceFamily> = Vec::new();
    let mut imports = Vec::new();
    for entry in evidence {
        let seen_family = seen_families.contains(&entry.family);
        let import = EvidenceImport::from_evidence(entry, config, seen_family);
        if import.accepted() {
            seen_families.push(entry.family);
        }
        imports.push(import);
    }
    imports
}

fn import_hold_reason(
    status: ImportStatus,
    missing_required_roots: u64,
    age_blocks: u64,
    config: &Config,
) -> String {
    match status {
        ImportStatus::Imported => "accepted_for_final_governance_import".to_string(),
        ImportStatus::HeldMissingRoot => {
            format!("missing_required_roots={missing_required_roots}")
        }
        ImportStatus::HeldStale => {
            format!(
                "evidence_age_blocks={age_blocks};max={}",
                config.max_evidence_age_blocks
            )
        }
        ImportStatus::HeldRejected => "receipt_status_not_importable".to_string(),
        ImportStatus::HeldDuplicateFamily => "accepted_family_already_imported".to_string(),
        ImportStatus::HeldPolicyMismatch => "family_not_required_by_import_policy".to_string(),
    }
}

fn distinct_family_count(imports: &[EvidenceImport]) -> u64 {
    let mut families = Vec::new();
    for import in imports {
        if import.accepted() && !families.contains(&import.family) {
            families.push(import.family);
        }
    }
    families.len() as u64
}

fn distinct_operator_count(imports: &[EvidenceImport]) -> u64 {
    let mut operators: Vec<String> = Vec::new();
    for import in imports {
        if import.accepted() && !operators.contains(&import.operator_id) {
            operators.push(import.operator_id.clone());
        }
    }
    operators.len() as u64
}

fn missing_required_family_count(config: &Config, imports: &[EvidenceImport]) -> u64 {
    EvidenceFamily::ordered()
        .iter()
        .filter(|family| family.required_by(config))
        .filter(|family| {
            !imports
                .iter()
                .any(|import| import.accepted() && import.family == **family)
        })
        .count() as u64
}

fn accepted_family_root(imports: &[EvidenceImport]) -> String {
    let records = EvidenceFamily::ordered()
        .iter()
        .filter(|family| {
            imports
                .iter()
                .any(|import| import.accepted() && import.family == **family)
        })
        .map(|family| json!({ "family": family.as_str(), "accepted": true }))
        .collect::<Vec<_>>();
    values_root("accepted-family-root", &records)
}

fn missing_family_root(config: &Config, imports: &[EvidenceImport]) -> String {
    let records = EvidenceFamily::ordered()
        .iter()
        .filter(|family| family.required_by(config))
        .filter(|family| {
            !imports
                .iter()
                .any(|import| import.accepted() && import.family == **family)
        })
        .map(|family| json!({ "family": family.as_str(), "missing": true }))
        .collect::<Vec<_>>();
    values_root("missing-family-root", &records)
}

fn accepted_operator_root(imports: &[EvidenceImport]) -> String {
    let mut operators: Vec<String> = Vec::new();
    for import in imports {
        if import.accepted() && !operators.contains(&import.operator_id) {
            operators.push(import.operator_id.clone());
        }
    }
    operators.sort();
    let records = operators
        .iter()
        .map(|operator_id| json!({ "operator_id": operator_id }))
        .collect::<Vec<_>>();
    values_root("accepted-operator-root", &records)
}

fn imports_root(imports: &[EvidenceImport]) -> String {
    let records = imports
        .iter()
        .map(|import| import.public_record())
        .collect::<Vec<_>>();
    values_root("imports-root", &records)
}

fn import_batches_root(batches: &[ImportBatch]) -> String {
    let records = batches
        .iter()
        .map(|batch| batch.public_record())
        .collect::<Vec<_>>();
    values_root("import-batches-root", &records)
}

fn governance_signatures_root(signatures: &[GovernanceSignature]) -> String {
    let records = signatures
        .iter()
        .map(|signature| signature.public_record())
        .collect::<Vec<_>>();
    values_root("governance-signatures-root", &records)
}

fn provisional_state_root(
    config: &Config,
    evidence: &[AcceptedLiveEvidence],
    imports: &[EvidenceImport],
    batches: &[ImportBatch],
    signatures: &[GovernanceSignature],
    summary: &ImportSummary,
) -> String {
    let evidence_records = evidence
        .iter()
        .map(|entry| entry.public_record())
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-COMPILE-RUNTIME-ACCEPTED-LIVE-EVIDENCE-IMPORT-PROVISIONAL-STATE",
        &[
            HashPart::Json(&config.public_record()),
            HashPart::Str(&values_root(
                "accepted-live-evidence-root",
                &evidence_records,
            )),
            HashPart::Str(&imports_root(imports)),
            HashPart::Str(&import_batches_root(batches)),
            HashPart::Str(&governance_signatures_root(signatures)),
            HashPart::Json(&summary.public_record()),
        ],
        32,
    )
}

fn evidence_import_id(family: EvidenceFamily, evidence_id: &str, evidence_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-COMPILE-RUNTIME-ACCEPTED-LIVE-EVIDENCE-IMPORT-ID",
        &[
            HashPart::Str(family.as_str()),
            HashPart::Str(evidence_id),
            HashPart::Str(evidence_root),
        ],
        16,
    )
}

fn import_batch_id(batch_label: &str, import_root: &str, accepted_import_count: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-COMPILE-RUNTIME-ACCEPTED-LIVE-EVIDENCE-IMPORT-BATCH-ID",
        &[
            HashPart::Str(batch_label),
            HashPart::Str(import_root),
            HashPart::U64(accepted_import_count),
        ],
        16,
    )
}

fn final_governance_import_id(
    governance_epoch: u64,
    decision: GovernanceDecision,
    summary_root: &str,
    state_root_before_decision: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-COMPILE-RUNTIME-ACCEPTED-LIVE-EVIDENCE-FINAL-GOVERNANCE-ID",
        &[
            HashPart::U64(governance_epoch),
            HashPart::Str(decision.as_str()),
            HashPart::Str(summary_root),
            HashPart::Str(state_root_before_decision),
        ],
        16,
    )
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-COMPILE-RUNTIME-ACCEPTED-LIVE-EVIDENCE-IMPORT-RECORD",
        &[HashPart::Str(label), HashPart::Json(record)],
        32,
    )
}

fn values_root(label: &str, records: &[Value]) -> String {
    let leaves = records
        .iter()
        .map(|record| Value::String(record_root(label, record)))
        .collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn devnet_accepted_live_evidence(config: &Config) -> Vec<AcceptedLiveEvidence> {
    vec![
        devnet_evidence(
            config,
            "compile-runtime-receipt",
            EvidenceFamily::Compile,
            "operator-alpha",
            "cargo build --release --package nebula_l2_rs",
            0,
        ),
        devnet_evidence(
            config,
            "cargo-check-receipt",
            EvidenceFamily::CargoCheck,
            "operator-beta",
            "cargo check --package nebula_l2_rs",
            1,
        ),
        devnet_evidence(
            config,
            "rustfmt-receipt",
            EvidenceFamily::Rustfmt,
            "operator-gamma",
            "cargo fmt --check --package nebula_l2_rs",
            2,
        ),
        devnet_evidence(
            config,
            "clippy-receipt",
            EvidenceFamily::Clippy,
            "operator-alpha",
            "cargo clippy --package nebula_l2_rs",
            3,
        ),
        devnet_evidence(
            config,
            "cargo-test-receipt",
            EvidenceFamily::CargoTest,
            "operator-beta",
            "cargo test --package nebula_l2_rs",
            4,
        ),
    ]
}

fn devnet_evidence(
    config: &Config,
    evidence_suffix: &str,
    family: EvidenceFamily,
    operator_id: &str,
    command_label: &str,
    height_offset: u64,
) -> AcceptedLiveEvidence {
    let evidence_id = format!("{}-{evidence_suffix}", config.force_exit_package_id);
    let payload = json!({
        "vertical_slice_id": config.vertical_slice_id,
        "force_exit_package_id": config.force_exit_package_id,
        "evidence_suffix": evidence_suffix,
        "family": family.as_str(),
        "operator_id": operator_id,
        "command_label": command_label,
        "governance_epoch": config.governance_epoch,
    });
    let evidence_payload_root = record_root("devnet-evidence-payload", &payload);
    let activation_root = record_root(
        "devnet-activation-root",
        &json!({
            "evidence_id": evidence_id,
            "family": family.as_str(),
            "activation": "accepted_live",
        }),
    );
    let release_manifest_root = record_root(
        "devnet-release-manifest-root",
        &json!({
            "evidence_id": evidence_id,
            "release_manifest": config.force_exit_package_id,
            "family": family.as_str(),
        }),
    );
    let governance_binding_root = record_root(
        "devnet-governance-binding-root",
        &json!({
            "evidence_id": evidence_id,
            "binding": "compile_runtime_go_no_go_governance",
            "family": family.as_str(),
        }),
    );
    let stdout_root = record_root(
        "devnet-stdout-root",
        &json!({
            "evidence_id": evidence_id,
            "stdout": "canonicalized-success-output",
        }),
    );
    let stderr_root = record_root(
        "devnet-stderr-root",
        &json!({
            "evidence_id": evidence_id,
            "stderr": "canonicalized-empty-diagnostic-output",
        }),
    );
    AcceptedLiveEvidence::new(
        &evidence_id,
        family,
        operator_id,
        command_label,
        &activation_root,
        &release_manifest_root,
        &governance_binding_root,
        &evidence_payload_root,
        &stdout_root,
        &stderr_root,
        config.source_height.saturating_sub(8 + height_offset),
        config.l2_height.saturating_sub(4 + height_offset),
        LiveEvidenceStatus::AcceptedLive,
        "accepted_live_receipt_imported_for_final_governance",
    )
}

fn devnet_governance_signatures(evidence_import_root: &str) -> Vec<GovernanceSignature> {
    [
        ("governance-signer-alpha", "release_steward"),
        ("governance-signer-beta", "runtime_reviewer"),
        ("governance-signer-gamma", "security_reviewer"),
        ("governance-signer-delta", "bridge_operator"),
        ("governance-signer-epsilon", "watchtower_operator"),
    ]
    .iter()
    .map(|(signer_id, signer_role)| {
        let signature_root = record_root(
            "devnet-governance-signature-root",
            &json!({
                "signer_id": signer_id,
                "signer_role": signer_role,
                "evidence_import_root": evidence_import_root,
                "signature_scheme": "pq-devnet-attestation",
            }),
        );
        GovernanceSignature::new(
            signer_id,
            signer_role,
            evidence_import_root,
            &signature_root,
            true,
        )
    })
    .collect()
}
