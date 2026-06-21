use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileRuntimeAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-compile-runtime-release-captain-go-no-go-replay-drill-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DRILL_SUITE: &str = "monero-l2-pq-force-exit-compile-runtime-go-no-go-replay-drill-v1";
pub const DEFAULT_RELEASE_EPOCH: u64 = 88;
pub const DEFAULT_SOURCE_CHECKLIST_EPOCH: u64 = 87;
pub const DEFAULT_DRILL_HEIGHT: u64 = 880_000;
pub const DEFAULT_CHECKLIST_HEIGHT: u64 = 870_000;
pub const DEFAULT_MAX_SOURCE_AGE_BLOCKS: u64 = 12_000;
pub const DEFAULT_MIN_CRITERIA: u16 = 6;
pub const DEFAULT_MIN_RUSTFMT_RECEIPTS: u16 = 1;
pub const DEFAULT_MIN_DEFERRED_CARGO_BLOCKERS: u16 = 3;
pub const DEFAULT_MIN_CAPTAIN_SIGNOFFS: u16 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub drill_suite: String,
    pub release_epoch: u64,
    pub source_checklist_epoch: u64,
    pub drill_height: u64,
    pub checklist_height: u64,
    pub release_channel: String,
    pub command_room_label: String,
    pub deployment_environment: String,
    pub max_source_age_blocks: u64,
    pub min_criteria: u16,
    pub min_rustfmt_receipts: u16,
    pub min_deferred_cargo_blockers: u16,
    pub min_captain_signoffs: u16,
    pub require_wave_87_checklist_roots: bool,
    pub require_compile_freeze: bool,
    pub require_rustfmt_receipts: bool,
    pub require_deferred_cargo_blockers: bool,
    pub require_replayable_criteria: bool,
    pub require_release_captain_signoffs: bool,
    pub require_fail_closed_verdict: bool,
    pub allow_release_go: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            drill_suite: DRILL_SUITE.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            source_checklist_epoch: DEFAULT_SOURCE_CHECKLIST_EPOCH,
            drill_height: DEFAULT_DRILL_HEIGHT,
            checklist_height: DEFAULT_CHECKLIST_HEIGHT,
            release_channel: "devnet-compile-runtime-release-captain-replay-drill".to_string(),
            command_room_label: "wave-88-compile-runtime-release-captain".to_string(),
            deployment_environment: "devnet-production-shadow".to_string(),
            max_source_age_blocks: DEFAULT_MAX_SOURCE_AGE_BLOCKS,
            min_criteria: DEFAULT_MIN_CRITERIA,
            min_rustfmt_receipts: DEFAULT_MIN_RUSTFMT_RECEIPTS,
            min_deferred_cargo_blockers: DEFAULT_MIN_DEFERRED_CARGO_BLOCKERS,
            min_captain_signoffs: DEFAULT_MIN_CAPTAIN_SIGNOFFS,
            require_wave_87_checklist_roots: true,
            require_compile_freeze: true,
            require_rustfmt_receipts: true,
            require_deferred_cargo_blockers: true,
            require_replayable_criteria: true,
            require_release_captain_signoffs: true,
            require_fail_closed_verdict: true,
            allow_release_go: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("drill_suite", &self.drill_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure_non_empty("command_room_label", &self.command_room_label)?;
        ensure_non_empty("deployment_environment", &self.deployment_environment)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.release_epoch > 0, "release epoch must be non-zero")?;
        ensure(
            self.release_epoch > self.source_checklist_epoch,
            "release epoch must follow source checklist epoch",
        )?;
        ensure(
            self.drill_height > self.checklist_height,
            "drill height must follow checklist height",
        )?;
        ensure(
            self.drill_height - self.checklist_height <= self.max_source_age_blocks,
            "wave 87 checklist roots are too old for replay drill binding",
        )?;
        ensure(self.min_criteria > 0, "minimum criteria must be non-zero")?;
        ensure(
            self.min_rustfmt_receipts > 0,
            "minimum rustfmt receipts must be non-zero",
        )?;
        ensure(
            self.min_deferred_cargo_blockers > 0,
            "minimum deferred cargo blockers must be non-zero",
        )?;
        ensure(
            self.min_captain_signoffs > 0,
            "minimum captain signoffs must be non-zero",
        )?;
        if self.require_fail_closed_verdict {
            ensure(
                !self.allow_release_go,
                "fail-closed verdict cannot allow release go",
            )?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "drill_suite": self.drill_suite,
            "release_epoch": self.release_epoch,
            "source_checklist_epoch": self.source_checklist_epoch,
            "drill_height": self.drill_height,
            "checklist_height": self.checklist_height,
            "release_channel": self.release_channel,
            "command_room_label": self.command_room_label,
            "deployment_environment": self.deployment_environment,
            "max_source_age_blocks": self.max_source_age_blocks,
            "min_criteria": self.min_criteria,
            "min_rustfmt_receipts": self.min_rustfmt_receipts,
            "min_deferred_cargo_blockers": self.min_deferred_cargo_blockers,
            "min_captain_signoffs": self.min_captain_signoffs,
            "require_wave_87_checklist_roots": self.require_wave_87_checklist_roots,
            "require_compile_freeze": self.require_compile_freeze,
            "require_rustfmt_receipts": self.require_rustfmt_receipts,
            "require_deferred_cargo_blockers": self.require_deferred_cargo_blockers,
            "require_replayable_criteria": self.require_replayable_criteria,
            "require_release_captain_signoffs": self.require_release_captain_signoffs,
            "require_fail_closed_verdict": self.require_fail_closed_verdict,
            "allow_release_go": self.allow_release_go,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CriterionKind {
    Wave87ChecklistRootsBound,
    CompileFreezeActive,
    RustfmtReceiptRootBound,
    DeferredCargoBlockersRecorded,
    GoNoGoInputsReplayable,
    ReleaseCaptainSignoffQuorum,
    VerdictFailClosed,
}

impl CriterionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Wave87ChecklistRootsBound,
            Self::CompileFreezeActive,
            Self::RustfmtReceiptRootBound,
            Self::DeferredCargoBlockersRecorded,
            Self::GoNoGoInputsReplayable,
            Self::ReleaseCaptainSignoffQuorum,
            Self::VerdictFailClosed,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave87ChecklistRootsBound => "wave_87_checklist_roots_bound",
            Self::CompileFreezeActive => "compile_freeze_active",
            Self::RustfmtReceiptRootBound => "rustfmt_receipt_root_bound",
            Self::DeferredCargoBlockersRecorded => "deferred_cargo_blockers_recorded",
            Self::GoNoGoInputsReplayable => "go_no_go_inputs_replayable",
            Self::ReleaseCaptainSignoffQuorum => "release_captain_signoff_quorum",
            Self::VerdictFailClosed => "verdict_fail_closed",
        }
    }

    pub fn release_blocking(self) -> bool {
        !matches!(self, Self::GoNoGoInputsReplayable)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CargoGate {
    Check,
    Test,
    Clippy,
}

impl CargoGate {
    pub fn all() -> Vec<Self> {
        vec![Self::Check, Self::Test, Self::Clippy]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Check => "cargo_check",
            Self::Test => "cargo_test",
            Self::Clippy => "cargo_clippy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptainRole {
    ReleaseCaptain,
    CompileRuntimeOwner,
    DeploymentAuthority,
}

impl CaptainRole {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReleaseCaptain,
            Self::CompileRuntimeOwner,
            Self::DeploymentAuthority,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCaptain => "release_captain",
            Self::CompileRuntimeOwner => "compile_runtime_owner",
            Self::DeploymentAuthority => "deployment_authority",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    Wave87RootsMissing,
    CompileFreezeRequired,
    RustfmtReceiptMissing,
    DeferredCargoBlockerMissing,
    CriteriaNotReplayable,
    CaptainSignoffQuorumMissing,
    ReleaseGoDisabled,
    VerdictHeldFailClosed,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave87RootsMissing => "wave_87_roots_missing",
            Self::CompileFreezeRequired => "compile_freeze_required",
            Self::RustfmtReceiptMissing => "rustfmt_receipt_missing",
            Self::DeferredCargoBlockerMissing => "deferred_cargo_blocker_missing",
            Self::CriteriaNotReplayable => "criteria_not_replayable",
            Self::CaptainSignoffQuorumMissing => "captain_signoff_quorum_missing",
            Self::ReleaseGoDisabled => "release_go_disabled",
            Self::VerdictHeldFailClosed => "verdict_held_fail_closed",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Self::ReleaseGoDisabled | Self::VerdictHeldFailClosed => 3,
            Self::DeferredCargoBlockerMissing | Self::CriteriaNotReplayable => 2,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave87ChecklistRoots {
    pub source_epoch: u64,
    pub observed_height: u64,
    pub operator_command_checklist_root: String,
    pub compile_freeze_root: String,
    pub rustfmt_receipt_root: String,
    pub deferred_cargo_blocker_root: String,
    pub owner_signoff_root: String,
    pub blocker_root: String,
    pub source_root: String,
}

impl Wave87ChecklistRoots {
    pub fn devnet(config: &Config) -> Self {
        let operator_command_checklist_root =
            sample_root("wave-87-operator-command-checklist", "compile-runtime", 1);
        let compile_freeze_root = sample_root("wave-87-compile-freeze", "compile-runtime", 2);
        let rustfmt_receipt_root = sample_root("wave-87-rustfmt-receipt", "compile-runtime", 3);
        let deferred_cargo_blocker_root =
            sample_root("wave-87-deferred-cargo-blocker", "compile-runtime", 4);
        let owner_signoff_root = sample_root("wave-87-owner-signoff", "compile-runtime", 5);
        let blocker_root = sample_root("wave-87-blocker-root", "compile-runtime", 6);
        let source_root = roots_root(
            "wave-87-checklist-roots",
            [
                operator_command_checklist_root.clone(),
                compile_freeze_root.clone(),
                rustfmt_receipt_root.clone(),
                deferred_cargo_blocker_root.clone(),
                owner_signoff_root.clone(),
                blocker_root.clone(),
            ],
        );
        Self {
            source_epoch: config.source_checklist_epoch,
            observed_height: config.checklist_height,
            operator_command_checklist_root,
            compile_freeze_root,
            rustfmt_receipt_root,
            deferred_cargo_blocker_root,
            owner_signoff_root,
            blocker_root,
            source_root,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty(
            "operator command checklist root",
            &self.operator_command_checklist_root,
        )?;
        ensure_non_empty("compile freeze root", &self.compile_freeze_root)?;
        ensure_non_empty("rustfmt receipt root", &self.rustfmt_receipt_root)?;
        ensure_non_empty(
            "deferred cargo blocker root",
            &self.deferred_cargo_blocker_root,
        )?;
        ensure_non_empty("owner signoff root", &self.owner_signoff_root)?;
        ensure_non_empty("blocker root", &self.blocker_root)?;
        ensure_non_empty("source root", &self.source_root)?;
        ensure(self.source_epoch > 0, "source epoch must be non-zero")?;
        ensure(
            self.observed_height > 0,
            "source observed height must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "source_epoch": self.source_epoch,
            "observed_height": self.observed_height,
            "operator_command_checklist_root": self.operator_command_checklist_root,
            "compile_freeze_root": self.compile_freeze_root,
            "rustfmt_receipt_root": self.rustfmt_receipt_root,
            "deferred_cargo_blocker_root": self.deferred_cargo_blocker_root,
            "owner_signoff_root": self.owner_signoff_root,
            "blocker_root": self.blocker_root,
            "source_root": self.source_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave-87-checklist-roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompileFreezeStatus {
    pub label: String,
    pub checklist_root: String,
    pub freeze_root: String,
    pub blocker_root: String,
    pub frozen: bool,
    pub fail_closed: bool,
}

impl CompileFreezeStatus {
    pub fn devnet(source: &Wave87ChecklistRoots) -> Self {
        Self {
            label: "compile-runtime-freeze-active".to_string(),
            checklist_root: source.operator_command_checklist_root.clone(),
            freeze_root: source.compile_freeze_root.clone(),
            blocker_root: source.blocker_root.clone(),
            frozen: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("compile freeze label", &self.label)?;
        ensure_non_empty("compile freeze checklist root", &self.checklist_root)?;
        ensure_non_empty("compile freeze root", &self.freeze_root)?;
        ensure_non_empty("compile freeze blocker root", &self.blocker_root)?;
        ensure(self.frozen, "compile freeze must remain active")?;
        ensure(self.fail_closed, "compile freeze must be fail-closed")
    }

    pub fn public_record(&self) -> Value {
        json!({
            "label": self.label,
            "checklist_root": self.checklist_root,
            "freeze_root": self.freeze_root,
            "blocker_root": self.blocker_root,
            "frozen": self.frozen,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("compile-freeze-status", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RustfmtReceipt {
    pub label: String,
    pub source_root: String,
    pub receipt_root: String,
    pub commitment_root: String,
    pub accepted: bool,
}

impl RustfmtReceipt {
    pub fn devnet(source: &Wave87ChecklistRoots, ordinal: u64) -> Self {
        let label = "rustfmt-edition-2021-owned-file-only".to_string();
        let source_root = source.rustfmt_receipt_root.clone();
        let commitment_root = sample_root("rustfmt-commitment", &label, ordinal);
        let receipt_root = roots_root(
            "rustfmt-receipt",
            [source_root.clone(), commitment_root.clone()],
        );
        Self {
            label,
            source_root,
            receipt_root,
            commitment_root,
            accepted: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("rustfmt label", &self.label)?;
        ensure_non_empty("rustfmt source root", &self.source_root)?;
        ensure_non_empty("rustfmt receipt root", &self.receipt_root)?;
        ensure_non_empty("rustfmt commitment root", &self.commitment_root)?;
        ensure(self.accepted, "rustfmt receipt must be accepted")
    }

    pub fn public_record(&self) -> Value {
        json!({
            "label": self.label,
            "source_root": self.source_root,
            "receipt_root": self.receipt_root,
            "commitment_root": self.commitment_root,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rustfmt-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeferredCargoBlocker {
    pub gate: CargoGate,
    pub label: String,
    pub source_root: String,
    pub blocker_root: String,
    pub fail_closed: bool,
}

impl DeferredCargoBlocker {
    pub fn devnet(gate: CargoGate, source: &Wave87ChecklistRoots, ordinal: u64) -> Self {
        let label = gate.as_str().to_string();
        let source_root = source.deferred_cargo_blocker_root.clone();
        let blocker_root = roots_root(
            "deferred-cargo-blocker",
            [
                source_root.clone(),
                sample_root("deferred-cargo-gate", &label, ordinal),
            ],
        );
        Self {
            gate,
            label,
            source_root,
            blocker_root,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("deferred cargo label", &self.label)?;
        ensure_non_empty("deferred cargo source root", &self.source_root)?;
        ensure_non_empty("deferred cargo blocker root", &self.blocker_root)?;
        ensure(
            self.fail_closed,
            "deferred cargo blocker must be fail-closed",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "label": self.label,
            "source_root": self.source_root,
            "blocker_root": self.blocker_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("deferred-cargo-blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCriterion {
    pub kind: CriterionKind,
    pub label: String,
    pub input_root: String,
    pub proof_root: String,
    pub replayable: bool,
    pub release_blocking: bool,
}

impl ReplayCriterion {
    pub fn devnet(kind: CriterionKind, source: &Wave87ChecklistRoots, ordinal: u64) -> Self {
        let label = kind.as_str().to_string();
        let input_root = match kind {
            CriterionKind::Wave87ChecklistRootsBound => source.source_root.clone(),
            CriterionKind::CompileFreezeActive => source.compile_freeze_root.clone(),
            CriterionKind::RustfmtReceiptRootBound => source.rustfmt_receipt_root.clone(),
            CriterionKind::DeferredCargoBlockersRecorded => {
                source.deferred_cargo_blocker_root.clone()
            }
            CriterionKind::ReleaseCaptainSignoffQuorum => source.owner_signoff_root.clone(),
            CriterionKind::VerdictFailClosed => source.blocker_root.clone(),
            CriterionKind::GoNoGoInputsReplayable => {
                sample_root("go-no-go-replay-inputs", &label, ordinal)
            }
        };
        let proof_root = roots_root(
            "go-no-go-replay-criterion",
            [
                input_root.clone(),
                sample_root("go-no-go-replay-proof", &label, ordinal),
            ],
        );
        Self {
            kind,
            label,
            input_root,
            proof_root,
            replayable: true,
            release_blocking: kind.release_blocking(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("criterion label", &self.label)?;
        ensure_non_empty("criterion input root", &self.input_root)?;
        ensure_non_empty("criterion proof root", &self.proof_root)?;
        ensure(self.replayable, "criterion must be replayable")
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "label": self.label,
            "input_root": self.input_root,
            "proof_root": self.proof_root,
            "replayable": self.replayable,
            "release_blocking": self.release_blocking,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay-criterion", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CaptainSignoff {
    pub role: CaptainRole,
    pub label: String,
    pub source_root: String,
    pub signoff_root: String,
    pub no_go: bool,
    pub fail_closed: bool,
}

impl CaptainSignoff {
    pub fn devnet(role: CaptainRole, source: &Wave87ChecklistRoots, ordinal: u64) -> Self {
        let label = role.as_str().to_string();
        let source_root = source.owner_signoff_root.clone();
        let signoff_root = roots_root(
            "release-captain-signoff",
            [
                source_root.clone(),
                sample_root("release-captain-role", &label, ordinal),
            ],
        );
        Self {
            role,
            label,
            source_root,
            signoff_root,
            no_go: true,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("captain signoff label", &self.label)?;
        ensure_non_empty("captain signoff source root", &self.source_root)?;
        ensure_non_empty("captain signoff root", &self.signoff_root)?;
        ensure(self.no_go, "captain signoff must record no-go")?;
        ensure(self.fail_closed, "captain signoff must be fail-closed")
    }

    pub fn public_record(&self) -> Value {
        json!({
            "role": self.role.as_str(),
            "label": self.label,
            "source_root": self.source_root,
            "signoff_root": self.signoff_root,
            "no_go": self.no_go,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("captain-signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseVerdict {
    pub label: String,
    pub criteria_root: String,
    pub blocker_root: String,
    pub signoff_root: String,
    pub release_go: bool,
    pub fail_closed: bool,
    pub verdict_root: String,
}

impl ReleaseVerdict {
    pub fn build(
        criteria_root: String,
        blocker_root: String,
        signoff_root: String,
        release_go: bool,
    ) -> Self {
        let label = if release_go {
            "release-go"
        } else {
            "release-no-go-fail-closed"
        }
        .to_string();
        let fail_closed = !release_go;
        let verdict_root = roots_root(
            "release-captain-verdict",
            [
                criteria_root.clone(),
                blocker_root.clone(),
                signoff_root.clone(),
                record_root(
                    "release-captain-verdict-label",
                    &json!({"label": label, "release_go": release_go, "fail_closed": fail_closed}),
                ),
            ],
        );
        Self {
            label,
            criteria_root,
            blocker_root,
            signoff_root,
            release_go,
            fail_closed,
            verdict_root,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("release verdict label", &self.label)?;
        ensure_non_empty("release verdict criteria root", &self.criteria_root)?;
        ensure_non_empty("release verdict blocker root", &self.blocker_root)?;
        ensure_non_empty("release verdict signoff root", &self.signoff_root)?;
        ensure_non_empty("release verdict root", &self.verdict_root)?;
        ensure(!self.release_go, "release verdict must be no-go")?;
        ensure(self.fail_closed, "release verdict must be fail-closed")
    }

    pub fn public_record(&self) -> Value {
        json!({
            "label": self.label,
            "criteria_root": self.criteria_root,
            "blocker_root": self.blocker_root,
            "signoff_root": self.signoff_root,
            "release_go": self.release_go,
            "fail_closed": self.fail_closed,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrillSummary {
    pub state: String,
    pub criterion_count: usize,
    pub replayable_criterion_count: usize,
    pub rustfmt_receipt_count: usize,
    pub deferred_cargo_blocker_count: usize,
    pub captain_signoff_count: usize,
    pub blocker_count: usize,
    pub max_blocker_severity: u8,
    pub compile_frozen: bool,
    pub release_go: bool,
    pub fail_closed: bool,
    pub package_root: String,
}

impl DrillSummary {
    pub fn build(
        compile_freeze: &CompileFreezeStatus,
        rustfmt_receipts: &[RustfmtReceipt],
        deferred_cargo_blockers: &[DeferredCargoBlocker],
        criteria: &[ReplayCriterion],
        captain_signoffs: &[CaptainSignoff],
        blockers: &BTreeMap<String, Vec<BlockerKind>>,
        verdict: &ReleaseVerdict,
    ) -> Self {
        let blocker_count = blockers.values().map(Vec::len).sum();
        let max_blocker_severity = match blockers
            .values()
            .flat_map(|items| items.iter().map(|blocker| blocker.severity()))
            .max()
        {
            Some(severity) => severity,
            None => 0,
        };
        let replayable_criterion_count = criteria.iter().filter(|item| item.replayable).count();
        let release_go = verdict.release_go && blocker_count == 0 && !compile_freeze.frozen;
        let fail_closed = verdict.fail_closed || blocker_count > 0 || compile_freeze.fail_closed;
        let state = if release_go {
            "release_go"
        } else if fail_closed {
            "release_no_go_fail_closed"
        } else {
            "release_no_go"
        }
        .to_string();
        let package_root = roots_root(
            "release-captain-go-no-go-package",
            [
                compile_freeze.state_root(),
                roots_root(
                    "release-captain-rustfmt-receipts",
                    rustfmt_receipts.iter().map(RustfmtReceipt::state_root),
                ),
                roots_root(
                    "release-captain-deferred-cargo-blockers",
                    deferred_cargo_blockers
                        .iter()
                        .map(DeferredCargoBlocker::state_root),
                ),
                roots_root(
                    "release-captain-replay-criteria",
                    criteria.iter().map(ReplayCriterion::state_root),
                ),
                roots_root(
                    "release-captain-signoffs",
                    captain_signoffs.iter().map(CaptainSignoff::state_root),
                ),
                blockers_root(blockers),
                verdict.state_root(),
            ],
        );
        Self {
            state,
            criterion_count: criteria.len(),
            replayable_criterion_count,
            rustfmt_receipt_count: rustfmt_receipts.len(),
            deferred_cargo_blocker_count: deferred_cargo_blockers.len(),
            captain_signoff_count: captain_signoffs.len(),
            blocker_count,
            max_blocker_severity,
            compile_frozen: compile_freeze.frozen,
            release_go,
            fail_closed,
            package_root,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure(self.compile_frozen, "summary must keep compile frozen")?;
        ensure(!self.release_go, "summary must record no-go")?;
        ensure(self.fail_closed, "summary must be fail-closed")?;
        ensure_non_empty("summary package root", &self.package_root)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "state": self.state,
            "criterion_count": self.criterion_count,
            "replayable_criterion_count": self.replayable_criterion_count,
            "rustfmt_receipt_count": self.rustfmt_receipt_count,
            "deferred_cargo_blocker_count": self.deferred_cargo_blocker_count,
            "captain_signoff_count": self.captain_signoff_count,
            "blocker_count": self.blocker_count,
            "max_blocker_severity": self.max_blocker_severity,
            "compile_frozen": self.compile_frozen,
            "release_go": self.release_go,
            "fail_closed": self.fail_closed,
            "package_root": self.package_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("drill-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave_87_roots: Wave87ChecklistRoots,
    pub compile_freeze: CompileFreezeStatus,
    pub rustfmt_receipts: Vec<RustfmtReceipt>,
    pub deferred_cargo_blockers: Vec<DeferredCargoBlocker>,
    pub criteria: Vec<ReplayCriterion>,
    pub captain_signoffs: Vec<CaptainSignoff>,
    pub blockers: BTreeMap<String, Vec<BlockerKind>>,
    pub rustfmt_receipt_root: String,
    pub deferred_cargo_blocker_root: String,
    pub criteria_root: String,
    pub captain_signoff_root: String,
    pub blocker_root: String,
    pub verdict: ReleaseVerdict,
    pub summary: DrillSummary,
}

impl State {
    pub fn new(
        config: Config,
        wave_87_roots: Wave87ChecklistRoots,
        compile_freeze: CompileFreezeStatus,
        rustfmt_receipts: Vec<RustfmtReceipt>,
        deferred_cargo_blockers: Vec<DeferredCargoBlocker>,
        criteria: Vec<ReplayCriterion>,
        captain_signoffs: Vec<CaptainSignoff>,
    ) -> Result<Self> {
        config.validate()?;
        wave_87_roots.validate()?;
        compile_freeze.validate()?;
        ensure(
            !rustfmt_receipts.is_empty(),
            "state must include rustfmt receipts",
        )?;
        ensure(
            !deferred_cargo_blockers.is_empty(),
            "state must include deferred cargo blockers",
        )?;
        ensure(!criteria.is_empty(), "state must include replay criteria")?;
        ensure(
            !captain_signoffs.is_empty(),
            "state must include captain signoffs",
        )?;
        for receipt in &rustfmt_receipts {
            receipt.validate()?;
        }
        for blocker in &deferred_cargo_blockers {
            blocker.validate()?;
        }
        for criterion in &criteria {
            criterion.validate()?;
        }
        for signoff in &captain_signoffs {
            signoff.validate()?;
        }
        validate_unique_roots(
            "rustfmt receipt roots",
            rustfmt_receipts.iter().map(|receipt| &receipt.receipt_root),
        )?;
        validate_unique_roots(
            "deferred cargo blocker roots",
            deferred_cargo_blockers
                .iter()
                .map(|blocker| &blocker.blocker_root),
        )?;
        validate_unique_roots(
            "replay criterion proof roots",
            criteria.iter().map(|criterion| &criterion.proof_root),
        )?;
        validate_unique_roots(
            "captain signoff roots",
            captain_signoffs.iter().map(|signoff| &signoff.signoff_root),
        )?;
        let rustfmt_receipt_root = roots_root(
            "release-captain-rustfmt-receipts",
            rustfmt_receipts.iter().map(RustfmtReceipt::state_root),
        );
        let deferred_cargo_blocker_root = roots_root(
            "release-captain-deferred-cargo-blockers",
            deferred_cargo_blockers
                .iter()
                .map(DeferredCargoBlocker::state_root),
        );
        let criteria_root = roots_root(
            "release-captain-replay-criteria",
            criteria.iter().map(ReplayCriterion::state_root),
        );
        let captain_signoff_root = roots_root(
            "release-captain-signoffs",
            captain_signoffs.iter().map(CaptainSignoff::state_root),
        );
        let blockers = evaluate_blockers(
            &config,
            &wave_87_roots,
            &compile_freeze,
            &rustfmt_receipts,
            &deferred_cargo_blockers,
            &criteria,
            &captain_signoffs,
        );
        let blocker_root = blockers_root(&blockers);
        let release_go = config.allow_release_go && blockers.is_empty() && !compile_freeze.frozen;
        let verdict = ReleaseVerdict::build(
            criteria_root.clone(),
            blocker_root.clone(),
            captain_signoff_root.clone(),
            release_go,
        );
        verdict.validate()?;
        let summary = DrillSummary::build(
            &compile_freeze,
            &rustfmt_receipts,
            &deferred_cargo_blockers,
            &criteria,
            &captain_signoffs,
            &blockers,
            &verdict,
        );
        summary.validate()?;
        Ok(Self {
            config,
            wave_87_roots,
            compile_freeze,
            rustfmt_receipts,
            deferred_cargo_blockers,
            criteria,
            captain_signoffs,
            blockers,
            rustfmt_receipt_root,
            deferred_cargo_blocker_root,
            criteria_root,
            captain_signoff_root,
            blocker_root,
            verdict,
            summary,
        })
    }

    pub fn devnet() -> Self {
        match Self::try_devnet() {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn try_devnet() -> Result<Self> {
        let config = Config::devnet();
        let wave_87_roots = Wave87ChecklistRoots::devnet(&config);
        let compile_freeze = CompileFreezeStatus::devnet(&wave_87_roots);
        let rustfmt_receipts = vec![RustfmtReceipt::devnet(&wave_87_roots, 1)];
        let deferred_cargo_blockers = CargoGate::all()
            .into_iter()
            .enumerate()
            .map(|(index, gate)| {
                DeferredCargoBlocker::devnet(gate, &wave_87_roots, one_based(index))
            })
            .collect::<Vec<_>>();
        let criteria = CriterionKind::all()
            .into_iter()
            .enumerate()
            .map(|(index, kind)| ReplayCriterion::devnet(kind, &wave_87_roots, one_based(index)))
            .collect::<Vec<_>>();
        let captain_signoffs = CaptainRole::all()
            .into_iter()
            .enumerate()
            .map(|(index, role)| CaptainSignoff::devnet(role, &wave_87_roots, one_based(index)))
            .collect::<Vec<_>>();
        Self::new(
            config,
            wave_87_roots,
            compile_freeze,
            rustfmt_receipts,
            deferred_cargo_blockers,
            criteria,
            captain_signoffs,
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "wave_87_roots": self.wave_87_roots.public_record(),
            "compile_freeze": self.compile_freeze.public_record(),
            "rustfmt_receipt_root": self.rustfmt_receipt_root,
            "deferred_cargo_blocker_root": self.deferred_cargo_blocker_root,
            "criteria_root": self.criteria_root,
            "captain_signoff_root": self.captain_signoff_root,
            "blocker_root": self.blocker_root,
            "verdict": self.verdict.public_record(),
            "summary": self.summary.public_record(),
            "rustfmt_receipts": self.rustfmt_receipts.iter().map(RustfmtReceipt::public_record).collect::<Vec<_>>(),
            "deferred_cargo_blockers": self.deferred_cargo_blockers.iter().map(DeferredCargoBlocker::public_record).collect::<Vec<_>>(),
            "criteria": self.criteria.iter().map(ReplayCriterion::public_record).collect::<Vec<_>>(),
            "captain_signoffs": self.captain_signoffs.iter().map(CaptainSignoff::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(|(subject, blockers)| {
                let max_severity = match blockers.iter().map(|blocker| blocker.severity()).max() {
                    Some(severity) => severity,
                    None => 0,
                };
                json!({
                    "subject": subject,
                    "blockers": blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                    "max_severity": max_severity,
                })
            }).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-BRIDGE-RELEASE-CAPTAIN-GO-NO-GO-REPLAY-DRILL-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.release_channel),
                HashPart::U64(self.config.release_epoch),
                HashPart::U64(self.config.drill_height),
                HashPart::Str(&self.wave_87_roots.source_root),
                HashPart::Str(&self.summary.package_root),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn evaluate_blockers(
    config: &Config,
    wave_87_roots: &Wave87ChecklistRoots,
    compile_freeze: &CompileFreezeStatus,
    rustfmt_receipts: &[RustfmtReceipt],
    deferred_cargo_blockers: &[DeferredCargoBlocker],
    criteria: &[ReplayCriterion],
    captain_signoffs: &[CaptainSignoff],
) -> BTreeMap<String, Vec<BlockerKind>> {
    let mut blockers = BTreeMap::<String, Vec<BlockerKind>>::new();
    if config.require_wave_87_checklist_roots && wave_87_roots.source_root.trim().is_empty() {
        push_blocker(
            &mut blockers,
            "wave_87_roots",
            BlockerKind::Wave87RootsMissing,
        );
    }
    if config.require_compile_freeze && !compile_freeze.frozen {
        push_blocker(
            &mut blockers,
            "compile_freeze",
            BlockerKind::CompileFreezeRequired,
        );
    }
    if config.require_rustfmt_receipts
        && rustfmt_receipts.len() < usize::from(config.min_rustfmt_receipts)
    {
        push_blocker(
            &mut blockers,
            "rustfmt_receipts",
            BlockerKind::RustfmtReceiptMissing,
        );
    }
    if config.require_deferred_cargo_blockers
        && deferred_cargo_blockers.len() < usize::from(config.min_deferred_cargo_blockers)
    {
        push_blocker(
            &mut blockers,
            "deferred_cargo_blockers",
            BlockerKind::DeferredCargoBlockerMissing,
        );
    }
    let replayable_count = criteria
        .iter()
        .filter(|criterion| criterion.replayable)
        .count();
    if config.require_replayable_criteria && replayable_count < usize::from(config.min_criteria) {
        push_blocker(
            &mut blockers,
            "replay_criteria",
            BlockerKind::CriteriaNotReplayable,
        );
    }
    if config.require_release_captain_signoffs
        && captain_signoffs.len() < usize::from(config.min_captain_signoffs)
    {
        push_blocker(
            &mut blockers,
            "captain_signoffs",
            BlockerKind::CaptainSignoffQuorumMissing,
        );
    }
    if !config.allow_release_go {
        push_blocker(&mut blockers, "release_go", BlockerKind::ReleaseGoDisabled);
    }
    if config.require_fail_closed_verdict {
        push_blocker(
            &mut blockers,
            "release_verdict",
            BlockerKind::VerdictHeldFailClosed,
        );
    }
    blockers
}

fn fallback_state(reason: String) -> State {
    let config = Config::devnet();
    let wave_87_roots = Wave87ChecklistRoots::devnet(&config);
    let compile_freeze = CompileFreezeStatus::devnet(&wave_87_roots);
    let rustfmt_receipts = vec![RustfmtReceipt::devnet(&wave_87_roots, 1)];
    let deferred_cargo_blockers = vec![DeferredCargoBlocker::devnet(
        CargoGate::Check,
        &wave_87_roots,
        1,
    )];
    let criteria = vec![ReplayCriterion::devnet(
        CriterionKind::VerdictFailClosed,
        &wave_87_roots,
        1,
    )];
    let captain_signoffs = vec![CaptainSignoff::devnet(
        CaptainRole::ReleaseCaptain,
        &wave_87_roots,
        1,
    )];
    let mut blockers = evaluate_blockers(
        &config,
        &wave_87_roots,
        &compile_freeze,
        &rustfmt_receipts,
        &deferred_cargo_blockers,
        &criteria,
        &captain_signoffs,
    );
    push_blocker(
        &mut blockers,
        "fallback",
        BlockerKind::VerdictHeldFailClosed,
    );
    if !reason.trim().is_empty() {
        push_blocker(
            &mut blockers,
            "fallback_reason",
            BlockerKind::ReleaseGoDisabled,
        );
    }
    let rustfmt_receipt_root = roots_root(
        "release-captain-rustfmt-receipts",
        rustfmt_receipts.iter().map(RustfmtReceipt::state_root),
    );
    let deferred_cargo_blocker_root = roots_root(
        "release-captain-deferred-cargo-blockers",
        deferred_cargo_blockers
            .iter()
            .map(DeferredCargoBlocker::state_root),
    );
    let criteria_root = roots_root(
        "release-captain-replay-criteria",
        criteria.iter().map(ReplayCriterion::state_root),
    );
    let captain_signoff_root = roots_root(
        "release-captain-signoffs",
        captain_signoffs.iter().map(CaptainSignoff::state_root),
    );
    let blocker_root = blockers_root(&blockers);
    let verdict = ReleaseVerdict::build(
        criteria_root.clone(),
        blocker_root.clone(),
        captain_signoff_root.clone(),
        false,
    );
    let summary = DrillSummary::build(
        &compile_freeze,
        &rustfmt_receipts,
        &deferred_cargo_blockers,
        &criteria,
        &captain_signoffs,
        &blockers,
        &verdict,
    );
    State {
        config,
        wave_87_roots,
        compile_freeze,
        rustfmt_receipts,
        deferred_cargo_blockers,
        criteria,
        captain_signoffs,
        blockers,
        rustfmt_receipt_root,
        deferred_cargo_blocker_root,
        criteria_root,
        captain_signoff_root,
        blocker_root,
        verdict,
        summary,
    }
}

fn push_blocker(
    blockers: &mut BTreeMap<String, Vec<BlockerKind>>,
    subject: &str,
    blocker: BlockerKind,
) {
    blockers
        .entry(subject.to_string())
        .or_default()
        .push(blocker);
}

fn validate_unique_roots<'a, I>(label: &str, roots: I) -> Result<()>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut seen = BTreeSet::new();
    for root in roots {
        ensure_non_empty(label, root)?;
        ensure(seen.insert(root), &format!("{label} must be unique"))?;
    }
    Ok(())
}

fn blockers_root(blockers: &BTreeMap<String, Vec<BlockerKind>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            let max_severity = match blocker_list.iter().map(|blocker| blocker.severity()).max() {
                Some(severity) => severity,
                None => 0,
            };
            json!({
                "subject": subject,
                "blockers": blocker_list.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
                "max_severity": max_severity,
            })
        })
        .collect::<Vec<_>>();
    merkle_root("release-captain-go-no-go-blockers", &leaves)
}

fn roots_root<I>(label: &str, roots: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(label, &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-RELEASE-CAPTAIN-GO-NO-GO-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn sample_root(kind: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-RELEASE-CAPTAIN-GO-NO-GO-SAMPLE-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn one_based(index: usize) -> u64 {
    index as u64 + 1
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty(),
        &format!("{label} must be non-empty"),
    )
}
