use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileRuntimeLiveReceiptReleaseManifestEnforcementRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-compile-runtime-live-receipt-release-manifest-enforcement-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const MANIFEST_SUITE: &str =
    "monero-l2-pq-force-exit-compile-runtime-live-receipt-release-manifest-enforcement-v1";
pub const DEFAULT_LANE_ID: &str = "canonical-user-escape-answer-force-exit-release-lane";
pub const DEFAULT_PACKAGE_ID: &str =
    "canonical-user-escape-answer-vertical-slice-force-exit-package";
pub const DEFAULT_REPLACEMENT_MANIFEST_LABEL: &str =
    "force-exit-compile-runtime-live-receipt-replacement-manifest";
pub const DEFAULT_MIN_REVIEWERS: u16 = 3;
pub const DEFAULT_MIN_ACCEPTED_RECEIPTS: u16 = 4;
pub const DEFAULT_MIN_ADJUDICATOR_SIGNERS: u16 = 2;
pub const DEFAULT_RELEASE_EPOCH: u64 = 79;
pub const DEFAULT_MAX_RECEIPTS: usize = 256;
pub const DEFAULT_MAX_REVIEWERS: usize = 128;
pub const DEFAULT_MAX_HOLD_EXPORTS: usize = 64;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptKind {
    CargoCheck,
    Rustfmt,
    Clippy,
    CargoTest,
}

impl ReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::Rustfmt => "rustfmt",
            Self::Clippy => "clippy",
            Self::CargoTest => "cargo_test",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Accepted,
    Rejected,
    Missing,
    Superseded,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Missing => "missing",
            Self::Superseded => "superseded",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerDecision {
    Approve,
    Hold,
    Reject,
}

impl ReviewerDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Hold => "hold",
            Self::Reject => "reject",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationStatus {
    Pending,
    Adjudicated,
    Activated,
    Held,
}

impl ActivationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Adjudicated => "adjudicated",
            Self::Activated => "activated",
            Self::Held => "held",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestReplacementStatus {
    Proposed,
    Rooted,
    Applied,
    Blocked,
}

impl ManifestReplacementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Rooted => "rooted",
            Self::Applied => "applied",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseHoldExportStatus {
    Exported,
    FailClosed,
    Cleared,
}

impl ReleaseHoldExportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exported => "exported",
            Self::FailClosed => "fail_closed",
            Self::Cleared => "cleared",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub manifest_suite: String,
    pub lane_id: String,
    pub package_id: String,
    pub release_epoch: u64,
    pub min_reviewer_quorum: u16,
    pub min_accepted_receipts: u16,
    pub min_adjudicator_signers: u16,
    pub require_cargo_check: bool,
    pub require_rustfmt: bool,
    pub require_clippy: bool,
    pub require_cargo_test: bool,
    pub require_replacement_manifest_root: bool,
    pub fail_closed_release_hold_export: bool,
    pub production_release_allowed: bool,
    pub max_receipts: usize,
    pub max_reviewers: usize,
    pub max_hold_exports: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            manifest_suite: MANIFEST_SUITE.to_string(),
            lane_id: DEFAULT_LANE_ID.to_string(),
            package_id: DEFAULT_PACKAGE_ID.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            min_reviewer_quorum: DEFAULT_MIN_REVIEWERS,
            min_accepted_receipts: DEFAULT_MIN_ACCEPTED_RECEIPTS,
            min_adjudicator_signers: DEFAULT_MIN_ADJUDICATOR_SIGNERS,
            require_cargo_check: true,
            require_rustfmt: true,
            require_clippy: true,
            require_cargo_test: true,
            require_replacement_manifest_root: true,
            fail_closed_release_hold_export: true,
            production_release_allowed: false,
            max_receipts: DEFAULT_MAX_RECEIPTS,
            max_reviewers: DEFAULT_MAX_REVIEWERS,
            max_hold_exports: DEFAULT_MAX_HOLD_EXPORTS,
        }
    }

    pub fn required_receipt_kinds(&self) -> BTreeSet<ReceiptKind> {
        let mut kinds = BTreeSet::new();
        if self.require_cargo_check {
            kinds.insert(ReceiptKind::CargoCheck);
        }
        if self.require_rustfmt {
            kinds.insert(ReceiptKind::Rustfmt);
        }
        if self.require_clippy {
            kinds.insert(ReceiptKind::Clippy);
        }
        if self.require_cargo_test {
            kinds.insert(ReceiptKind::CargoTest);
        }
        kinds
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "manifest_suite": self.manifest_suite,
            "lane_id": self.lane_id,
            "package_id": self.package_id,
            "release_epoch": self.release_epoch,
            "min_reviewer_quorum": self.min_reviewer_quorum,
            "min_accepted_receipts": self.min_accepted_receipts,
            "min_adjudicator_signers": self.min_adjudicator_signers,
            "require_cargo_check": self.require_cargo_check,
            "require_rustfmt": self.require_rustfmt,
            "require_clippy": self.require_clippy,
            "require_cargo_test": self.require_cargo_test,
            "require_replacement_manifest_root": self.require_replacement_manifest_root,
            "fail_closed_release_hold_export": self.fail_closed_release_hold_export,
            "production_release_allowed": self.production_release_allowed,
            "max_receipts": self.max_receipts,
            "max_reviewers": self.max_reviewers,
            "max_hold_exports": self.max_hold_exports,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementReceipt {
    pub receipt_id: String,
    pub kind: ReceiptKind,
    pub status: ReceiptStatus,
    pub package_id: String,
    pub command_line: String,
    pub toolchain_root: String,
    pub stdout_root: String,
    pub stderr_root: String,
    pub artifact_root: String,
    pub started_at_height: u64,
    pub completed_at_height: u64,
    pub accepted_by: String,
    pub acceptance_root: String,
    pub failure_reason: String,
}

impl EnforcementReceipt {
    pub fn accepted(
        kind: ReceiptKind,
        package_id: impl Into<String>,
        command_line: impl Into<String>,
        toolchain_root: impl Into<String>,
        artifact_root: impl Into<String>,
        accepted_by: impl Into<String>,
        completed_at_height: u64,
    ) -> Self {
        let package_id = package_id.into();
        let command_line = command_line.into();
        let toolchain_root = toolchain_root.into();
        let artifact_root = artifact_root.into();
        let accepted_by = accepted_by.into();
        let stdout_root = deterministic_leaf("receipt-stdout", kind.as_str(), &artifact_root);
        let stderr_root = deterministic_leaf("receipt-stderr", kind.as_str(), "empty");
        let acceptance_root = acceptance_root(
            kind,
            &package_id,
            &artifact_root,
            &accepted_by,
            completed_at_height,
        );
        let receipt_id = receipt_id(kind, &package_id, &acceptance_root);
        Self {
            receipt_id,
            kind,
            status: ReceiptStatus::Accepted,
            package_id,
            command_line,
            toolchain_root,
            stdout_root,
            stderr_root,
            artifact_root,
            started_at_height: completed_at_height.saturating_sub(1),
            completed_at_height,
            accepted_by,
            acceptance_root,
            failure_reason: String::new(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "kind": self.kind.as_str(),
            "status": self.status.as_str(),
            "package_id": self.package_id,
            "command_line": self.command_line,
            "toolchain_root": self.toolchain_root,
            "stdout_root": self.stdout_root,
            "stderr_root": self.stderr_root,
            "artifact_root": self.artifact_root,
            "started_at_height": self.started_at_height,
            "completed_at_height": self.completed_at_height,
            "accepted_by": self.accepted_by,
            "acceptance_root": self.acceptance_root,
            "failure_reason": self.failure_reason,
        })
    }

    pub fn root(&self) -> String {
        record_root("enforcement-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReviewerAttestation {
    pub reviewer_id: String,
    pub decision: ReviewerDecision,
    pub lane_id: String,
    pub package_id: String,
    pub receipt_root: String,
    pub manifest_root: String,
    pub signed_at_height: u64,
    pub signature_root: String,
    pub objection: String,
}

impl ReviewerAttestation {
    pub fn approve(
        reviewer_id: impl Into<String>,
        lane_id: impl Into<String>,
        package_id: impl Into<String>,
        receipt_root: impl Into<String>,
        manifest_root: impl Into<String>,
        signed_at_height: u64,
    ) -> Self {
        let reviewer_id = reviewer_id.into();
        let lane_id = lane_id.into();
        let package_id = package_id.into();
        let receipt_root = receipt_root.into();
        let manifest_root = manifest_root.into();
        let signature_root = reviewer_signature_root(
            &reviewer_id,
            &lane_id,
            &package_id,
            &receipt_root,
            &manifest_root,
            signed_at_height,
        );
        Self {
            reviewer_id,
            decision: ReviewerDecision::Approve,
            lane_id,
            package_id,
            receipt_root,
            manifest_root,
            signed_at_height,
            signature_root,
            objection: String::new(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_id": self.reviewer_id,
            "decision": self.decision.as_str(),
            "lane_id": self.lane_id,
            "package_id": self.package_id,
            "receipt_root": self.receipt_root,
            "manifest_root": self.manifest_root,
            "signed_at_height": self.signed_at_height,
            "signature_root": self.signature_root,
            "objection": self.objection,
        })
    }

    pub fn root(&self) -> String {
        record_root("reviewer-attestation", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationAdjudicator {
    pub adjudicator_id: String,
    pub status: ActivationStatus,
    pub lane_id: String,
    pub package_id: String,
    pub receipt_root: String,
    pub reviewer_quorum_root: String,
    pub replacement_manifest_root: String,
    pub signer_ids: Vec<String>,
    pub signer_root: String,
    pub activated_at_height: u64,
    pub activation_adjudicator_root: String,
    pub hold_reason: String,
}

impl ActivationAdjudicator {
    pub fn adjudicated(
        lane_id: impl Into<String>,
        package_id: impl Into<String>,
        receipt_root: impl Into<String>,
        reviewer_quorum_root: impl Into<String>,
        replacement_manifest_root: impl Into<String>,
        signer_ids: Vec<String>,
        activated_at_height: u64,
    ) -> Self {
        let lane_id = lane_id.into();
        let package_id = package_id.into();
        let receipt_root = receipt_root.into();
        let reviewer_quorum_root = reviewer_quorum_root.into();
        let replacement_manifest_root = replacement_manifest_root.into();
        let signer_root = signer_root(&signer_ids);
        let adjudicator_id = adjudicator_id(
            &lane_id,
            &package_id,
            &receipt_root,
            &reviewer_quorum_root,
            &replacement_manifest_root,
        );
        let activation_adjudicator_root = activation_adjudicator_root(
            &adjudicator_id,
            &signer_root,
            &replacement_manifest_root,
            activated_at_height,
        );
        Self {
            adjudicator_id,
            status: ActivationStatus::Adjudicated,
            lane_id,
            package_id,
            receipt_root,
            reviewer_quorum_root,
            replacement_manifest_root,
            signer_ids,
            signer_root,
            activated_at_height,
            activation_adjudicator_root,
            hold_reason: String::new(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "adjudicator_id": self.adjudicator_id,
            "status": self.status.as_str(),
            "lane_id": self.lane_id,
            "package_id": self.package_id,
            "receipt_root": self.receipt_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "replacement_manifest_root": self.replacement_manifest_root,
            "signer_ids": self.signer_ids,
            "signer_root": self.signer_root,
            "activated_at_height": self.activated_at_height,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "hold_reason": self.hold_reason,
        })
    }

    pub fn root(&self) -> String {
        record_root("activation-adjudicator", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplacementManifest {
    pub manifest_id: String,
    pub status: ManifestReplacementStatus,
    pub label: String,
    pub lane_id: String,
    pub package_id: String,
    pub previous_manifest_root: String,
    pub replacement_manifest_root: String,
    pub receipt_root: String,
    pub reviewer_quorum_root: String,
    pub activation_adjudicator_root: String,
    pub release_notes_root: String,
    pub applied_at_height: u64,
}

impl ReplacementManifest {
    pub fn rooted(
        lane_id: impl Into<String>,
        package_id: impl Into<String>,
        previous_manifest_root: impl Into<String>,
        receipt_root: impl Into<String>,
        reviewer_quorum_root: impl Into<String>,
        activation_adjudicator_root: impl Into<String>,
        applied_at_height: u64,
    ) -> Self {
        let lane_id = lane_id.into();
        let package_id = package_id.into();
        let previous_manifest_root = previous_manifest_root.into();
        let receipt_root = receipt_root.into();
        let reviewer_quorum_root = reviewer_quorum_root.into();
        let activation_adjudicator_root = activation_adjudicator_root.into();
        let label = DEFAULT_REPLACEMENT_MANIFEST_LABEL.to_string();
        let release_notes_root = deterministic_leaf("release-notes", &lane_id, &package_id);
        let replacement_manifest_root = replacement_manifest_root(
            &lane_id,
            &package_id,
            &previous_manifest_root,
            &receipt_root,
            &reviewer_quorum_root,
            &activation_adjudicator_root,
        );
        let manifest_id =
            replacement_manifest_id(&lane_id, &package_id, &replacement_manifest_root);
        Self {
            manifest_id,
            status: ManifestReplacementStatus::Rooted,
            label,
            lane_id,
            package_id,
            previous_manifest_root,
            replacement_manifest_root,
            receipt_root,
            reviewer_quorum_root,
            activation_adjudicator_root,
            release_notes_root,
            applied_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "manifest_id": self.manifest_id,
            "status": self.status.as_str(),
            "label": self.label,
            "lane_id": self.lane_id,
            "package_id": self.package_id,
            "previous_manifest_root": self.previous_manifest_root,
            "replacement_manifest_root": self.replacement_manifest_root,
            "receipt_root": self.receipt_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "release_notes_root": self.release_notes_root,
            "applied_at_height": self.applied_at_height,
        })
    }

    pub fn root(&self) -> String {
        record_root("replacement-manifest", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseHoldExport {
    pub export_id: String,
    pub status: ReleaseHoldExportStatus,
    pub lane_id: String,
    pub package_id: String,
    pub reason: String,
    pub missing_receipt_kinds: Vec<String>,
    pub rejected_receipt_ids: Vec<String>,
    pub reviewer_deficit: u16,
    pub adjudicator_deficit: u16,
    pub replacement_manifest_root: String,
    pub exported_at_height: u64,
    pub fail_closed_root: String,
}

impl ReleaseHoldExport {
    pub fn fail_closed(
        lane_id: impl Into<String>,
        package_id: impl Into<String>,
        reason: impl Into<String>,
        missing_receipt_kinds: Vec<String>,
        rejected_receipt_ids: Vec<String>,
        reviewer_deficit: u16,
        adjudicator_deficit: u16,
        replacement_manifest_root: impl Into<String>,
        exported_at_height: u64,
    ) -> Self {
        let lane_id = lane_id.into();
        let package_id = package_id.into();
        let reason = reason.into();
        let replacement_manifest_root = replacement_manifest_root.into();
        let fail_closed_root = fail_closed_root(
            &lane_id,
            &package_id,
            &reason,
            &replacement_manifest_root,
            exported_at_height,
        );
        let export_id = hold_export_id(&lane_id, &package_id, &fail_closed_root);
        Self {
            export_id,
            status: ReleaseHoldExportStatus::FailClosed,
            lane_id,
            package_id,
            reason,
            missing_receipt_kinds,
            rejected_receipt_ids,
            reviewer_deficit,
            adjudicator_deficit,
            replacement_manifest_root,
            exported_at_height,
            fail_closed_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "export_id": self.export_id,
            "status": self.status.as_str(),
            "lane_id": self.lane_id,
            "package_id": self.package_id,
            "reason": self.reason,
            "missing_receipt_kinds": self.missing_receipt_kinds,
            "rejected_receipt_ids": self.rejected_receipt_ids,
            "reviewer_deficit": self.reviewer_deficit,
            "adjudicator_deficit": self.adjudicator_deficit,
            "replacement_manifest_root": self.replacement_manifest_root,
            "exported_at_height": self.exported_at_height,
            "fail_closed_root": self.fail_closed_root,
        })
    }

    pub fn root(&self) -> String {
        record_root("release-hold-export", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementCounters {
    pub total_receipts: u64,
    pub accepted_receipts: u64,
    pub rejected_receipts: u64,
    pub missing_required_receipt_kinds: u64,
    pub approving_reviewers: u64,
    pub holding_reviewers: u64,
    pub rejecting_reviewers: u64,
    pub adjudicator_signers: u64,
    pub replacement_manifests: u64,
    pub fail_closed_exports: u64,
}

impl EnforcementCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "total_receipts": self.total_receipts,
            "accepted_receipts": self.accepted_receipts,
            "rejected_receipts": self.rejected_receipts,
            "missing_required_receipt_kinds": self.missing_required_receipt_kinds,
            "approving_reviewers": self.approving_reviewers,
            "holding_reviewers": self.holding_reviewers,
            "rejecting_reviewers": self.rejecting_reviewers,
            "adjudicator_signers": self.adjudicator_signers,
            "replacement_manifests": self.replacement_manifests,
            "fail_closed_exports": self.fail_closed_exports,
        })
    }

    pub fn root(&self) -> String {
        record_root("enforcement-counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementVerdict {
    pub release_allowed: bool,
    pub fail_closed_hold_required: bool,
    pub accepted_receipt_root: String,
    pub reviewer_quorum_root: String,
    pub activation_adjudicator_root: String,
    pub replacement_manifest_root: String,
    pub hold_export_root: String,
    pub reason: String,
}

impl EnforcementVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "release_allowed": self.release_allowed,
            "fail_closed_hold_required": self.fail_closed_hold_required,
            "accepted_receipt_root": self.accepted_receipt_root,
            "reviewer_quorum_root": self.reviewer_quorum_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "replacement_manifest_root": self.replacement_manifest_root,
            "hold_export_root": self.hold_export_root,
            "reason": self.reason,
        })
    }

    pub fn root(&self) -> String {
        record_root("enforcement-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub receipts: BTreeMap<String, EnforcementReceipt>,
    pub reviewer_attestations: BTreeMap<String, ReviewerAttestation>,
    pub activation_adjudicator: ActivationAdjudicator,
    pub replacement_manifest: ReplacementManifest,
    pub hold_exports: BTreeMap<String, ReleaseHoldExport>,
    pub counters: EnforcementCounters,
}

impl State {
    pub fn new(
        config: Config,
        receipts: Vec<EnforcementReceipt>,
        reviewer_attestations: Vec<ReviewerAttestation>,
        activation_adjudicator: ActivationAdjudicator,
        replacement_manifest: ReplacementManifest,
        hold_exports: Vec<ReleaseHoldExport>,
    ) -> Result<Self> {
        validate_config(&config)?;
        let receipts = keyed_receipts(receipts, config.max_receipts)?;
        let reviewer_attestations = keyed_reviewers(reviewer_attestations, config.max_reviewers)?;
        let hold_exports = keyed_hold_exports(hold_exports, config.max_hold_exports)?;
        let counters = counters_for(
            &config,
            &receipts,
            &reviewer_attestations,
            &activation_adjudicator,
            &replacement_manifest,
            &hold_exports,
        );
        let state = Self {
            config,
            receipts,
            reviewer_attestations,
            activation_adjudicator,
            replacement_manifest,
            hold_exports,
            counters,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let toolchain_root = deterministic_leaf("toolchain", "rust-stable", "devnet");
        let cargo_check = EnforcementReceipt::accepted(
            ReceiptKind::CargoCheck,
            config.package_id.clone(),
            "cargo check -p nebula_l2_rs",
            toolchain_root.clone(),
            deterministic_leaf("artifact", "cargo-check", DEFAULT_PACKAGE_ID),
            "reviewer-build-1",
            790_010,
        );
        let rustfmt = EnforcementReceipt::accepted(
            ReceiptKind::Rustfmt,
            config.package_id.clone(),
            "cargo fmt --check -p nebula_l2_rs",
            toolchain_root.clone(),
            deterministic_leaf("artifact", "rustfmt", DEFAULT_PACKAGE_ID),
            "reviewer-style-1",
            790_011,
        );
        let clippy = EnforcementReceipt::accepted(
            ReceiptKind::Clippy,
            config.package_id.clone(),
            "cargo clippy -p nebula_l2_rs -- -D warnings",
            toolchain_root.clone(),
            deterministic_leaf("artifact", "clippy", DEFAULT_PACKAGE_ID),
            "reviewer-lint-1",
            790_012,
        );
        let cargo_test = EnforcementReceipt::accepted(
            ReceiptKind::CargoTest,
            config.package_id.clone(),
            "cargo test -p nebula_l2_rs force_exit_package",
            toolchain_root,
            deterministic_leaf("artifact", "cargo-test", DEFAULT_PACKAGE_ID),
            "reviewer-test-1",
            790_013,
        );
        let receipts = vec![cargo_check, rustfmt, clippy, cargo_test];
        let receipt_root = root_for_records(
            "receipt-root",
            receipts.iter().map(EnforcementReceipt::public_record),
        );
        let previous_manifest_root =
            deterministic_leaf("previous-manifest", DEFAULT_LANE_ID, DEFAULT_PACKAGE_ID);
        let provisional_manifest_root = replacement_manifest_root(
            DEFAULT_LANE_ID,
            DEFAULT_PACKAGE_ID,
            &previous_manifest_root,
            &receipt_root,
            &deterministic_leaf("reviewer-quorum", DEFAULT_LANE_ID, "provisional"),
            &deterministic_leaf("activation-adjudicator", DEFAULT_LANE_ID, "provisional"),
        );
        let reviewers = vec![
            ReviewerAttestation::approve(
                "release-reviewer-a",
                config.lane_id.clone(),
                config.package_id.clone(),
                receipt_root.clone(),
                provisional_manifest_root.clone(),
                790_020,
            ),
            ReviewerAttestation::approve(
                "release-reviewer-b",
                config.lane_id.clone(),
                config.package_id.clone(),
                receipt_root.clone(),
                provisional_manifest_root.clone(),
                790_021,
            ),
            ReviewerAttestation::approve(
                "release-reviewer-c",
                config.lane_id.clone(),
                config.package_id.clone(),
                receipt_root.clone(),
                provisional_manifest_root.clone(),
                790_022,
            ),
        ];
        let reviewer_quorum_root = root_for_records(
            "reviewer-quorum-root",
            reviewers.iter().map(ReviewerAttestation::public_record),
        );
        let activation = ActivationAdjudicator::adjudicated(
            config.lane_id.clone(),
            config.package_id.clone(),
            receipt_root.clone(),
            reviewer_quorum_root.clone(),
            provisional_manifest_root.clone(),
            vec![
                "activation-adjudicator-a".to_string(),
                "activation-adjudicator-b".to_string(),
            ],
            790_030,
        );
        let replacement = ReplacementManifest::rooted(
            config.lane_id.clone(),
            config.package_id.clone(),
            previous_manifest_root,
            receipt_root,
            reviewer_quorum_root,
            activation.activation_adjudicator_root.clone(),
            790_031,
        );
        let hold = ReleaseHoldExport::fail_closed(
            config.lane_id.clone(),
            config.package_id.clone(),
            "production release remains held until operator imports release manifest lane",
            Vec::new(),
            Vec::new(),
            0,
            0,
            replacement.replacement_manifest_root.clone(),
            790_032,
        );
        match Self::new(
            config,
            receipts,
            reviewers,
            activation,
            replacement,
            vec![hold],
        ) {
            Ok(state) => state,
            Err(_) => Self::minimal_fail_closed(),
        }
    }

    pub fn minimal_fail_closed() -> Self {
        let config = Config::devnet();
        let empty_root = empty_collection_root("empty-runtime");
        let activation = ActivationAdjudicator::adjudicated(
            config.lane_id.clone(),
            config.package_id.clone(),
            empty_root.clone(),
            empty_root.clone(),
            empty_root.clone(),
            vec![
                "activation-adjudicator-a".to_string(),
                "activation-adjudicator-b".to_string(),
            ],
            0,
        );
        let replacement = ReplacementManifest::rooted(
            config.lane_id.clone(),
            config.package_id.clone(),
            empty_root.clone(),
            empty_root.clone(),
            empty_root.clone(),
            activation.activation_adjudicator_root.clone(),
            0,
        );
        let hold = ReleaseHoldExport::fail_closed(
            config.lane_id.clone(),
            config.package_id.clone(),
            "minimal fail-closed state",
            vec![
                ReceiptKind::CargoCheck.as_str().to_string(),
                ReceiptKind::Rustfmt.as_str().to_string(),
                ReceiptKind::Clippy.as_str().to_string(),
                ReceiptKind::CargoTest.as_str().to_string(),
            ],
            Vec::new(),
            config.min_reviewer_quorum,
            config.min_adjudicator_signers,
            replacement.replacement_manifest_root.clone(),
            0,
        );
        let mut hold_exports = BTreeMap::new();
        hold_exports.insert(hold.export_id.clone(), hold);
        let counters = counters_for(
            &config,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &activation,
            &replacement,
            &hold_exports,
        );
        Self {
            config,
            receipts: BTreeMap::new(),
            reviewer_attestations: BTreeMap::new(),
            activation_adjudicator: activation,
            replacement_manifest: replacement,
            hold_exports,
            counters,
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_config(&self.config)?;
        require(
            self.receipts.len() <= self.config.max_receipts,
            "receipt count exceeds configured maximum",
        )?;
        require(
            self.reviewer_attestations.len() <= self.config.max_reviewers,
            "reviewer count exceeds configured maximum",
        )?;
        require(
            self.hold_exports.len() <= self.config.max_hold_exports,
            "hold export count exceeds configured maximum",
        )?;
        for receipt in self.receipts.values() {
            require(
                receipt.package_id == self.config.package_id,
                "receipt package id does not match config",
            )?;
            require_non_empty("receipt_id", &receipt.receipt_id)?;
            require_non_empty("acceptance_root", &receipt.acceptance_root)?;
        }
        for reviewer in self.reviewer_attestations.values() {
            require(
                reviewer.lane_id == self.config.lane_id,
                "reviewer lane id does not match config",
            )?;
            require(
                reviewer.package_id == self.config.package_id,
                "reviewer package id does not match config",
            )?;
        }
        require(
            self.activation_adjudicator.lane_id == self.config.lane_id,
            "activation lane id does not match config",
        )?;
        require(
            self.activation_adjudicator.package_id == self.config.package_id,
            "activation package id does not match config",
        )?;
        require_non_empty(
            "activation_replacement_manifest_root",
            &self.activation_adjudicator.replacement_manifest_root,
        )?;
        require_non_empty(
            "replacement_manifest_root",
            &self.replacement_manifest.replacement_manifest_root,
        )?;
        require(
            self.replacement_manifest.activation_adjudicator_root
                == self.activation_adjudicator.activation_adjudicator_root,
            "replacement manifest must bind activation adjudicator root",
        )?;
        Ok(())
    }

    pub fn receipt_root(&self) -> String {
        root_for_records(
            "receipt-root",
            self.receipts
                .values()
                .map(EnforcementReceipt::public_record),
        )
    }

    pub fn accepted_receipt_root(&self) -> String {
        root_for_records(
            "accepted-receipt-root",
            self.receipts
                .values()
                .filter(|receipt| receipt.status == ReceiptStatus::Accepted)
                .map(EnforcementReceipt::public_record),
        )
    }

    pub fn reviewer_quorum_root(&self) -> String {
        root_for_records(
            "reviewer-quorum-root",
            self.reviewer_attestations
                .values()
                .map(ReviewerAttestation::public_record),
        )
    }

    pub fn hold_export_root(&self) -> String {
        root_for_records(
            "hold-export-root",
            self.hold_exports
                .values()
                .map(ReleaseHoldExport::public_record),
        )
    }

    pub fn replacement_manifest_root(&self) -> String {
        self.replacement_manifest.replacement_manifest_root.clone()
    }

    pub fn activation_adjudicator_root(&self) -> String {
        self.activation_adjudicator
            .activation_adjudicator_root
            .clone()
    }

    pub fn missing_required_receipt_kinds(&self) -> Vec<ReceiptKind> {
        let accepted: BTreeSet<ReceiptKind> = self
            .receipts
            .values()
            .filter(|receipt| receipt.status == ReceiptStatus::Accepted)
            .map(|receipt| receipt.kind)
            .collect();
        self.config
            .required_receipt_kinds()
            .difference(&accepted)
            .copied()
            .collect()
    }

    pub fn rejected_receipt_ids(&self) -> Vec<String> {
        self.receipts
            .values()
            .filter(|receipt| receipt.status == ReceiptStatus::Rejected)
            .map(|receipt| receipt.receipt_id.clone())
            .collect()
    }

    pub fn reviewer_approval_count(&self) -> u16 {
        saturating_u16(
            self.reviewer_attestations
                .values()
                .filter(|reviewer| reviewer.decision == ReviewerDecision::Approve)
                .count(),
        )
    }

    pub fn adjudicator_signer_count(&self) -> u16 {
        saturating_u16(self.activation_adjudicator.signer_ids.len())
    }

    pub fn release_verdict(&self) -> EnforcementVerdict {
        let missing = self.missing_required_receipt_kinds();
        let rejected = self.rejected_receipt_ids();
        let reviewer_deficit = self
            .config
            .min_reviewer_quorum
            .saturating_sub(self.reviewer_approval_count());
        let adjudicator_deficit = self
            .config
            .min_adjudicator_signers
            .saturating_sub(self.adjudicator_signer_count());
        let replacement_present = !self
            .replacement_manifest
            .replacement_manifest_root
            .is_empty();
        let adjudicated = self.activation_adjudicator.status == ActivationStatus::Adjudicated
            || self.activation_adjudicator.status == ActivationStatus::Activated;
        let release_allowed = missing.is_empty()
            && rejected.is_empty()
            && reviewer_deficit == 0
            && adjudicator_deficit == 0
            && replacement_present
            && adjudicated
            && self.config.production_release_allowed;
        let reason = if release_allowed {
            "release manifest lane has complete receipts, quorum, adjudication, and manifest root"
                .to_string()
        } else {
            "release held fail-closed until receipts, quorum, adjudication, replacement manifest, and production flag all pass".to_string()
        };
        EnforcementVerdict {
            release_allowed,
            fail_closed_hold_required: !release_allowed
                && self.config.fail_closed_release_hold_export,
            accepted_receipt_root: self.accepted_receipt_root(),
            reviewer_quorum_root: self.reviewer_quorum_root(),
            activation_adjudicator_root: self.activation_adjudicator_root(),
            replacement_manifest_root: self.replacement_manifest_root(),
            hold_export_root: self.hold_export_root(),
            reason,
        }
    }

    pub fn public_record(&self) -> Value {
        let verdict = self.release_verdict();
        json!({
            "config": self.config.public_record(),
            "receipt_root": self.receipt_root(),
            "accepted_receipt_root": self.accepted_receipt_root(),
            "reviewer_quorum_root": self.reviewer_quorum_root(),
            "activation_adjudicator_root": self.activation_adjudicator_root(),
            "replacement_manifest_root": self.replacement_manifest_root(),
            "hold_export_root": self.hold_export_root(),
            "counters": self.counters.public_record(),
            "verdict": verdict.public_record(),
            "receipts": self.receipts.values().map(EnforcementReceipt::public_record).collect::<Vec<_>>(),
            "reviewer_attestations": self.reviewer_attestations.values().map(ReviewerAttestation::public_record).collect::<Vec<_>>(),
            "activation_adjudicator": self.activation_adjudicator.public_record(),
            "replacement_manifest": self.replacement_manifest.public_record(),
            "hold_exports": self.hold_exports.values().map(ReleaseHoldExport::public_record).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    State::devnet().public_record()
}

pub fn state_root() -> String {
    State::devnet().state_root()
}

fn validate_config(config: &Config) -> Result<()> {
    require_non_empty("chain_id", &config.chain_id)?;
    require_non_empty("protocol_version", &config.protocol_version)?;
    require_non_empty("hash_suite", &config.hash_suite)?;
    require_non_empty("manifest_suite", &config.manifest_suite)?;
    require_non_empty("lane_id", &config.lane_id)?;
    require_non_empty("package_id", &config.package_id)?;
    require(
        config.schema_version == SCHEMA_VERSION,
        "schema version must match runtime schema",
    )?;
    require(
        config.min_reviewer_quorum > 0,
        "reviewer quorum must be nonzero",
    )?;
    require(
        config.min_accepted_receipts > 0,
        "accepted receipt threshold must be nonzero",
    )?;
    require(
        config.min_adjudicator_signers > 0,
        "adjudicator signer threshold must be nonzero",
    )?;
    require(config.max_receipts > 0, "max receipts must be nonzero")?;
    require(config.max_reviewers > 0, "max reviewers must be nonzero")?;
    require(
        config.max_hold_exports > 0,
        "max hold exports must be nonzero",
    )?;
    Ok(())
}

fn keyed_receipts(
    receipts: Vec<EnforcementReceipt>,
    max_receipts: usize,
) -> Result<BTreeMap<String, EnforcementReceipt>> {
    require(
        receipts.len() <= max_receipts,
        "receipt count exceeds configured maximum",
    )?;
    let mut keyed = BTreeMap::new();
    for receipt in receipts {
        require_non_empty("receipt_id", &receipt.receipt_id)?;
        require(
            keyed.insert(receipt.receipt_id.clone(), receipt).is_none(),
            "duplicate receipt id",
        )?;
    }
    Ok(keyed)
}

fn keyed_reviewers(
    reviewers: Vec<ReviewerAttestation>,
    max_reviewers: usize,
) -> Result<BTreeMap<String, ReviewerAttestation>> {
    require(
        reviewers.len() <= max_reviewers,
        "reviewer count exceeds configured maximum",
    )?;
    let mut keyed = BTreeMap::new();
    for reviewer in reviewers {
        require_non_empty("reviewer_id", &reviewer.reviewer_id)?;
        require(
            keyed
                .insert(reviewer.reviewer_id.clone(), reviewer)
                .is_none(),
            "duplicate reviewer id",
        )?;
    }
    Ok(keyed)
}

fn keyed_hold_exports(
    hold_exports: Vec<ReleaseHoldExport>,
    max_hold_exports: usize,
) -> Result<BTreeMap<String, ReleaseHoldExport>> {
    require(
        hold_exports.len() <= max_hold_exports,
        "hold export count exceeds configured maximum",
    )?;
    let mut keyed = BTreeMap::new();
    for export in hold_exports {
        require_non_empty("export_id", &export.export_id)?;
        require(
            keyed.insert(export.export_id.clone(), export).is_none(),
            "duplicate hold export id",
        )?;
    }
    Ok(keyed)
}

fn counters_for(
    config: &Config,
    receipts: &BTreeMap<String, EnforcementReceipt>,
    reviewers: &BTreeMap<String, ReviewerAttestation>,
    activation: &ActivationAdjudicator,
    replacement: &ReplacementManifest,
    hold_exports: &BTreeMap<String, ReleaseHoldExport>,
) -> EnforcementCounters {
    let accepted: BTreeSet<ReceiptKind> = receipts
        .values()
        .filter(|receipt| receipt.status == ReceiptStatus::Accepted)
        .map(|receipt| receipt.kind)
        .collect();
    let missing_required_receipt_kinds = config
        .required_receipt_kinds()
        .difference(&accepted)
        .count() as u64;
    EnforcementCounters {
        total_receipts: receipts.len() as u64,
        accepted_receipts: receipts
            .values()
            .filter(|receipt| receipt.status == ReceiptStatus::Accepted)
            .count() as u64,
        rejected_receipts: receipts
            .values()
            .filter(|receipt| receipt.status == ReceiptStatus::Rejected)
            .count() as u64,
        missing_required_receipt_kinds,
        approving_reviewers: reviewers
            .values()
            .filter(|reviewer| reviewer.decision == ReviewerDecision::Approve)
            .count() as u64,
        holding_reviewers: reviewers
            .values()
            .filter(|reviewer| reviewer.decision == ReviewerDecision::Hold)
            .count() as u64,
        rejecting_reviewers: reviewers
            .values()
            .filter(|reviewer| reviewer.decision == ReviewerDecision::Reject)
            .count() as u64,
        adjudicator_signers: activation.signer_ids.len() as u64,
        replacement_manifests: if replacement.replacement_manifest_root.is_empty() {
            0
        } else {
            1
        },
        fail_closed_exports: hold_exports
            .values()
            .filter(|export| export.status == ReleaseHoldExportStatus::FailClosed)
            .count() as u64,
    }
}

fn receipt_id(kind: ReceiptKind, package_id: &str, acceptance_root: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-LIVE-RECEIPT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind.as_str()),
            HashPart::Str(package_id),
            HashPart::Str(acceptance_root),
        ],
        32,
    )
}

fn acceptance_root(
    kind: ReceiptKind,
    package_id: &str,
    artifact_root: &str,
    accepted_by: &str,
    completed_at_height: u64,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-LIVE-RECEIPT-ACCEPTANCE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind.as_str()),
            HashPart::Str(package_id),
            HashPart::Str(artifact_root),
            HashPart::Str(accepted_by),
            HashPart::Int(completed_at_height as i128),
        ],
        32,
    )
}

fn reviewer_signature_root(
    reviewer_id: &str,
    lane_id: &str,
    package_id: &str,
    receipt_root: &str,
    manifest_root: &str,
    signed_at_height: u64,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REVIEWER-SIGNATURE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(reviewer_id),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(receipt_root),
            HashPart::Str(manifest_root),
            HashPart::Int(signed_at_height as i128),
        ],
        32,
    )
}

fn adjudicator_id(
    lane_id: &str,
    package_id: &str,
    receipt_root: &str,
    reviewer_quorum_root: &str,
    replacement_manifest_root: &str,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-ACTIVATION-ADJUDICATOR-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(receipt_root),
            HashPart::Str(reviewer_quorum_root),
            HashPart::Str(replacement_manifest_root),
        ],
        32,
    )
}

fn activation_adjudicator_root(
    adjudicator_id: &str,
    signer_root: &str,
    replacement_manifest_root: &str,
    activated_at_height: u64,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-ACTIVATION-ADJUDICATOR-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(adjudicator_id),
            HashPart::Str(signer_root),
            HashPart::Str(replacement_manifest_root),
            HashPart::Int(activated_at_height as i128),
        ],
        32,
    )
}

fn replacement_manifest_root(
    lane_id: &str,
    package_id: &str,
    previous_manifest_root: &str,
    receipt_root: &str,
    reviewer_quorum_root: &str,
    activation_adjudicator_root: &str,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REPLACEMENT-MANIFEST-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(previous_manifest_root),
            HashPart::Str(receipt_root),
            HashPart::Str(reviewer_quorum_root),
            HashPart::Str(activation_adjudicator_root),
        ],
        32,
    )
}

fn replacement_manifest_id(lane_id: &str, package_id: &str, manifest_root: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REPLACEMENT-MANIFEST-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(manifest_root),
        ],
        32,
    )
}

fn fail_closed_root(
    lane_id: &str,
    package_id: &str,
    reason: &str,
    replacement_manifest_root: &str,
    exported_at_height: u64,
) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-FAIL-CLOSED-RELEASE-HOLD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(reason),
            HashPart::Str(replacement_manifest_root),
            HashPart::Int(exported_at_height as i128),
        ],
        32,
    )
}

fn hold_export_id(lane_id: &str, package_id: &str, fail_closed_root: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-HOLD-EXPORT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_id),
            HashPart::Str(package_id),
            HashPart::Str(fail_closed_root),
        ],
        32,
    )
}

fn signer_root(signers: &[String]) -> String {
    root_for_records(
        "adjudicator-signer-root",
        signers.iter().map(|signer| {
            json!({
                "signer_id": signer,
                "chain_id": CHAIN_ID,
            })
        }),
    )
}

fn deterministic_leaf(domain: &str, left: &str, right: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-DETERMINISTIC-LEAF",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(domain),
            HashPart::Str(left),
            HashPart::Str(right),
        ],
        32,
    )
}

fn record_root(record_kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-RECORD-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(record_kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn root_for_records<I>(domain: &str, records: I) -> String
where
    I: IntoIterator<Item = Value>,
{
    let leaves: Vec<Value> = records
        .into_iter()
        .map(|record| {
            json!({
                "record_root": record_root(domain, &record),
                "record": record,
            })
        })
        .collect();
    if leaves.is_empty() {
        empty_collection_root(domain)
    } else {
        merkle_root(domain, &leaves)
    }
}

fn empty_collection_root(domain: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-EMPTY-COLLECTION",
        &[HashPart::Str(CHAIN_ID), HashPart::Str(domain)],
        32,
    )
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    require(
        !value.trim().is_empty(),
        &format!("{field} must be nonempty"),
    )
}

fn saturating_u16(value: usize) -> u16 {
    if value > u16::MAX as usize {
        u16::MAX
    } else {
        value as u16
    }
}
