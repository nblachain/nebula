use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqReservePrivacyAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-reserve-privacy-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;

pub const DEFAULT_HEIGHT: u64 = 95_086;
pub const DEFAULT_WAVE_NUMBER: u16 = 86;
pub const DEFAULT_SOURCE_WAVE_NUMBER: u16 = 85;
pub const DEFAULT_KEY_EPOCH: u64 = 85;
pub const DEFAULT_MIN_PQ_POLICY_ROOTS: u16 = 3;
pub const DEFAULT_MIN_RESERVE_ATTESTATIONS: u16 = 3;
pub const DEFAULT_MIN_PRIVACY_BUDGET_ROOTS: u16 = 3;
pub const DEFAULT_MIN_AMOUNT_BUCKETS: u16 = 4;
pub const DEFAULT_MIN_ROTATION_READY_ROOTS: u16 = 2;
pub const DEFAULT_MIN_COMMAND_BLOCKERS: u16 = 2;
pub const DEFAULT_MIN_OPERATOR_ACKS: u16 = 4;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u16 = 10_500;
pub const DEFAULT_MAX_PRIVACY_LEAK_BPS: u16 = 2_500;
pub const DEFAULT_MAX_BUCKET_EXPOSURE_BPS: u16 = 1_250;
pub const DEFAULT_INCIDENT_ID: &str = "wave-86-pq-reserve-privacy-incident-handoff";
pub const DEFAULT_PACKAGE_ID: &str = "force-exit-package-pq-reserve-privacy";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDecision {
    TransferBlocked,
    FailClosed,
    ReadyForHumanHandoff,
}

impl HandoffDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TransferBlocked => "transfer_blocked",
            Self::FailClosed => "fail_closed",
            Self::ReadyForHumanHandoff => "ready_for_human_handoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    PqSignerPolicyMissing,
    ReserveAttestationMissing,
    ReserveCoverageShortfall,
    PrivacyBudgetMissing,
    PrivacyLeakBudgetExceeded,
    AmountBucketExposureExceeded,
    AmountBucketMissing,
    KeyRotationNotReady,
    CommandTransferBlocked,
    OperatorAckShortfall,
    RootMismatch,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqSignerPolicyMissing => "pq_signer_policy_missing",
            Self::ReserveAttestationMissing => "reserve_attestation_missing",
            Self::ReserveCoverageShortfall => "reserve_coverage_shortfall",
            Self::PrivacyBudgetMissing => "privacy_budget_missing",
            Self::PrivacyLeakBudgetExceeded => "privacy_leak_budget_exceeded",
            Self::AmountBucketExposureExceeded => "amount_bucket_exposure_exceeded",
            Self::AmountBucketMissing => "amount_bucket_missing",
            Self::KeyRotationNotReady => "key_rotation_not_ready",
            Self::CommandTransferBlocked => "command_transfer_blocked",
            Self::OperatorAckShortfall => "operator_ack_shortfall",
            Self::RootMismatch => "root_mismatch",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub incident_id: String,
    pub package_id: String,
    pub wave_number: u16,
    pub source_wave_number: u16,
    pub key_epoch: u64,
    pub min_pq_policy_roots: u16,
    pub min_reserve_attestations: u16,
    pub min_privacy_budget_roots: u16,
    pub min_amount_buckets: u16,
    pub min_rotation_ready_roots: u16,
    pub min_command_blockers: u16,
    pub min_operator_acks: u16,
    pub min_reserve_coverage_bps: u16,
    pub max_privacy_leak_bps: u16,
    pub max_bucket_exposure_bps: u16,
    pub require_fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            incident_id: DEFAULT_INCIDENT_ID.to_string(),
            package_id: DEFAULT_PACKAGE_ID.to_string(),
            wave_number: DEFAULT_WAVE_NUMBER,
            source_wave_number: DEFAULT_SOURCE_WAVE_NUMBER,
            key_epoch: DEFAULT_KEY_EPOCH,
            min_pq_policy_roots: DEFAULT_MIN_PQ_POLICY_ROOTS,
            min_reserve_attestations: DEFAULT_MIN_RESERVE_ATTESTATIONS,
            min_privacy_budget_roots: DEFAULT_MIN_PRIVACY_BUDGET_ROOTS,
            min_amount_buckets: DEFAULT_MIN_AMOUNT_BUCKETS,
            min_rotation_ready_roots: DEFAULT_MIN_ROTATION_READY_ROOTS,
            min_command_blockers: DEFAULT_MIN_COMMAND_BLOCKERS,
            min_operator_acks: DEFAULT_MIN_OPERATOR_ACKS,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            max_privacy_leak_bps: DEFAULT_MAX_PRIVACY_LEAK_BPS,
            max_bucket_exposure_bps: DEFAULT_MAX_BUCKET_EXPOSURE_BPS,
            require_fail_closed: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "incident_id": self.incident_id,
            "package_id": self.package_id,
            "wave_number": self.wave_number,
            "source_wave_number": self.source_wave_number,
            "key_epoch": self.key_epoch,
            "min_pq_policy_roots": self.min_pq_policy_roots,
            "min_reserve_attestations": self.min_reserve_attestations,
            "min_privacy_budget_roots": self.min_privacy_budget_roots,
            "min_amount_buckets": self.min_amount_buckets,
            "min_rotation_ready_roots": self.min_rotation_ready_roots,
            "min_command_blockers": self.min_command_blockers,
            "min_operator_acks": self.min_operator_acks,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "max_privacy_leak_bps": self.max_privacy_leak_bps,
            "max_bucket_exposure_bps": self.max_bucket_exposure_bps,
            "require_fail_closed": self.require_fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("incident_id", &self.incident_id)?;
        ensure_non_empty("package_id", &self.package_id)?;
        ensure(
            self.wave_number > self.source_wave_number,
            "wave ordering invalid",
        )?;
        ensure_bps_at_least(
            "min_reserve_coverage_bps",
            self.min_reserve_coverage_bps,
            10_000,
        )?;
        ensure_bps("max_privacy_leak_bps", self.max_privacy_leak_bps)?;
        ensure_bps("max_bucket_exposure_bps", self.max_bucket_exposure_bps)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncidentEvidence {
    pub evidence_id: String,
    pub pq_signer_policy_roots: Vec<String>,
    pub reserve_attestation_roots: Vec<String>,
    pub privacy_leak_budget_roots: Vec<String>,
    pub amount_bucket_safeguard_roots: Vec<String>,
    pub key_rotation_readiness_roots: Vec<String>,
    pub operator_ack_roots: Vec<String>,
    pub reserve_coverage_bps: u16,
    pub privacy_leak_budget_bps: u16,
    pub max_amount_bucket_exposure_bps: u16,
    pub key_epoch: u64,
}

impl IncidentEvidence {
    pub fn public_record(&self) -> Value {
        json!({
            "evidence_id": self.evidence_id,
            "pq_signer_policy_root": root_set("pq-signer-policy-roots", &self.pq_signer_policy_roots),
            "reserve_attestation_root": root_set("reserve-attestation-roots", &self.reserve_attestation_roots),
            "privacy_leak_budget_root": root_set("privacy-leak-budget-roots", &self.privacy_leak_budget_roots),
            "amount_bucket_safeguard_root": root_set("amount-bucket-safeguard-roots", &self.amount_bucket_safeguard_roots),
            "key_rotation_readiness_root": root_set("key-rotation-readiness-roots", &self.key_rotation_readiness_roots),
            "operator_ack_root": root_set("operator-ack-roots", &self.operator_ack_roots),
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "privacy_leak_budget_bps": self.privacy_leak_budget_bps,
            "max_amount_bucket_exposure_bps": self.max_amount_bucket_exposure_bps,
            "key_epoch": self.key_epoch,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("incident-evidence", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("evidence_id", &self.evidence_id)?;
        ensure_roots("pq_signer_policy_roots", &self.pq_signer_policy_roots)?;
        ensure_roots("reserve_attestation_roots", &self.reserve_attestation_roots)?;
        ensure_roots("privacy_leak_budget_roots", &self.privacy_leak_budget_roots)?;
        ensure_roots(
            "amount_bucket_safeguard_roots",
            &self.amount_bucket_safeguard_roots,
        )?;
        ensure_roots(
            "key_rotation_readiness_roots",
            &self.key_rotation_readiness_roots,
        )?;
        ensure_roots("operator_ack_roots", &self.operator_ack_roots)?;
        ensure_bps_at_least("reserve_coverage_bps", self.reserve_coverage_bps, 10_000)?;
        ensure_bps("privacy_leak_budget_bps", self.privacy_leak_budget_bps)?;
        ensure_bps(
            "max_amount_bucket_exposure_bps",
            self.max_amount_bucket_exposure_bps,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandTransferBlocker {
    pub blocker_id: String,
    pub command_family: String,
    pub blocker_root: String,
    pub fail_closed: bool,
}

impl CommandTransferBlocker {
    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "command_family": self.command_family,
            "blocker_root": self.blocker_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_non_empty("command_family", &self.command_family)?;
        ensure_root("blocker_root", &self.blocker_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffBlocker {
    pub kind: BlockerKind,
    pub detail: String,
    pub blocking_root: String,
}

impl HandoffBlocker {
    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "detail": self.detail,
            "blocking_root": self.blocking_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub evidence: IncidentEvidence,
    pub command_transfer_blockers: Vec<CommandTransferBlocker>,
    pub observed_at_height: u64,
    pub handoff_operator: String,
}

impl State {
    pub fn devnet() -> Self {
        Self {
            config: Config::devnet(),
            evidence: devnet_evidence(),
            command_transfer_blockers: vec![
                command_blocker("block-force-exit-release", "force_exit_release"),
                command_blocker("block-operator-transfer", "operator_command_transfer"),
            ],
            observed_at_height: DEFAULT_HEIGHT,
            handoff_operator: "operator-dashboard-incident-handoff".to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        let blockers = self
            .handoff_blockers()
            .into_iter()
            .map(|blocker| blocker.public_record())
            .collect::<Vec<_>>();
        json!({
            "config": self.config.public_record(),
            "evidence": self.evidence.public_record(),
            "command_transfer_blocker_root": root_set(
                "command-transfer-blockers",
                &self
                    .command_transfer_blockers
                    .iter()
                    .map(|blocker| record_root("command-transfer-blocker", &blocker.public_record()))
                    .collect::<Vec<_>>(),
            ),
            "observed_at_height": self.observed_at_height,
            "handoff_operator": self.handoff_operator,
            "decision": self.decision().as_str(),
            "fail_closed": self.fail_closed(),
            "blockers": blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("incident-handoff-state", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        self.evidence.validate()?;
        ensure_non_empty("handoff_operator", &self.handoff_operator)?;
        ensure(
            self.evidence.key_epoch == self.config.key_epoch,
            "key epoch must match configured handoff epoch",
        )?;
        for blocker in &self.command_transfer_blockers {
            blocker.validate()?;
        }
        if self.config.require_fail_closed {
            ensure(self.fail_closed(), "incident handoff must fail closed")?;
        }
        Ok(())
    }

    pub fn decision(&self) -> HandoffDecision {
        if !self.command_transfer_blockers.is_empty() {
            return HandoffDecision::TransferBlocked;
        }
        if self.fail_closed() {
            HandoffDecision::FailClosed
        } else {
            HandoffDecision::ReadyForHumanHandoff
        }
    }

    pub fn fail_closed(&self) -> bool {
        !self.handoff_blockers().is_empty()
            || self
                .command_transfer_blockers
                .iter()
                .any(|blocker| blocker.fail_closed)
    }

    pub fn handoff_blockers(&self) -> Vec<HandoffBlocker> {
        let mut blockers = Vec::new();
        push_min_blocker(
            &mut blockers,
            BlockerKind::PqSignerPolicyMissing,
            "pq signer policy roots below quorum",
            self.evidence.pq_signer_policy_roots.len(),
            self.config.min_pq_policy_roots,
        );
        push_min_blocker(
            &mut blockers,
            BlockerKind::ReserveAttestationMissing,
            "reserve attestations below quorum",
            self.evidence.reserve_attestation_roots.len(),
            self.config.min_reserve_attestations,
        );
        push_min_blocker(
            &mut blockers,
            BlockerKind::PrivacyBudgetMissing,
            "privacy leak budget roots below quorum",
            self.evidence.privacy_leak_budget_roots.len(),
            self.config.min_privacy_budget_roots,
        );
        push_min_blocker(
            &mut blockers,
            BlockerKind::AmountBucketMissing,
            "amount bucket safeguard roots below quorum",
            self.evidence.amount_bucket_safeguard_roots.len(),
            self.config.min_amount_buckets,
        );
        push_min_blocker(
            &mut blockers,
            BlockerKind::KeyRotationNotReady,
            "key rotation readiness roots below quorum",
            self.evidence.key_rotation_readiness_roots.len(),
            self.config.min_rotation_ready_roots,
        );
        push_min_blocker(
            &mut blockers,
            BlockerKind::OperatorAckShortfall,
            "operator acknowledgements below quorum",
            self.evidence.operator_ack_roots.len(),
            self.config.min_operator_acks,
        );
        if self.evidence.reserve_coverage_bps < self.config.min_reserve_coverage_bps {
            blockers.push(blocker(
                BlockerKind::ReserveCoverageShortfall,
                "reserve coverage below required fail-closed threshold",
            ));
        }
        if self.evidence.privacy_leak_budget_bps > self.config.max_privacy_leak_bps {
            blockers.push(blocker(
                BlockerKind::PrivacyLeakBudgetExceeded,
                "privacy leak budget exceeds release policy",
            ));
        }
        if self.evidence.max_amount_bucket_exposure_bps > self.config.max_bucket_exposure_bps {
            blockers.push(blocker(
                BlockerKind::AmountBucketExposureExceeded,
                "amount bucket exposure exceeds safeguard",
            ));
        }
        if self.evidence.key_epoch != self.config.key_epoch {
            blockers.push(blocker(
                BlockerKind::KeyRotationNotReady,
                "key rotation epoch does not match handoff epoch",
            ));
        }
        if self.command_transfer_blockers.len() < self.config.min_command_blockers as usize
            || self
                .command_transfer_blockers
                .iter()
                .any(|item| item.fail_closed)
        {
            blockers.push(blocker(
                BlockerKind::CommandTransferBlocked,
                "command transfer blockers keep release lane fail-closed",
            ));
        }
        blockers
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

fn devnet_evidence() -> IncidentEvidence {
    IncidentEvidence {
        evidence_id: DEFAULT_INCIDENT_ID.to_string(),
        pq_signer_policy_roots: sample_roots("pq-signer-policy", DEFAULT_MIN_PQ_POLICY_ROOTS),
        reserve_attestation_roots: sample_roots(
            "reserve-attestation",
            DEFAULT_MIN_RESERVE_ATTESTATIONS,
        ),
        privacy_leak_budget_roots: sample_roots(
            "privacy-leak-budget",
            DEFAULT_MIN_PRIVACY_BUDGET_ROOTS,
        ),
        amount_bucket_safeguard_roots: sample_roots("amount-bucket", DEFAULT_MIN_AMOUNT_BUCKETS),
        key_rotation_readiness_roots: sample_roots(
            "key-rotation-ready",
            DEFAULT_MIN_ROTATION_READY_ROOTS,
        ),
        operator_ack_roots: sample_roots("operator-ack", DEFAULT_MIN_OPERATOR_ACKS),
        reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
        privacy_leak_budget_bps: DEFAULT_MAX_PRIVACY_LEAK_BPS,
        max_amount_bucket_exposure_bps: DEFAULT_MAX_BUCKET_EXPOSURE_BPS,
        key_epoch: DEFAULT_KEY_EPOCH,
    }
}

fn command_blocker(blocker_id: &str, command_family: &str) -> CommandTransferBlocker {
    CommandTransferBlocker {
        blocker_id: blocker_id.to_string(),
        command_family: command_family.to_string(),
        blocker_root: sample_root(blocker_id),
        fail_closed: true,
    }
}

fn push_min_blocker(
    blockers: &mut Vec<HandoffBlocker>,
    kind: BlockerKind,
    detail: &str,
    observed: usize,
    required: u16,
) {
    if observed < required as usize {
        blockers.push(blocker(kind, detail));
    }
}

fn blocker(kind: BlockerKind, detail: &str) -> HandoffBlocker {
    HandoffBlocker {
        kind,
        detail: detail.to_string(),
        blocking_root: sample_root(kind.as_str()),
    }
}

fn sample_roots(label: &str, count: u16) -> Vec<String> {
    (0..count)
        .map(|index| sample_root(&format!("{label}-{index}")))
        .collect()
}

fn sample_root(label: &str) -> String {
    domain_hash(
        "PQ-RESERVE-PRIVACY-INCIDENT-HANDOFF-SAMPLE-ROOT",
        &[HashPart::Str(CHAIN_ID), HashPart::Str(label)],
        32,
    )
}

fn root_set(domain: &str, roots: &[String]) -> String {
    let leaves = sorted_unique(roots)
        .into_iter()
        .map(|root| json!({ "root": root }))
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "PQ-RESERVE-PRIVACY-INCIDENT-HANDOFF-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn sorted_unique(values: &[String]) -> Vec<String> {
    values
        .iter()
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
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

fn ensure_root(label: &str, value: &str) -> Result<()> {
    ensure_non_empty(label, value)?;
    ensure(value.len() >= 32, &format!("{label} must be root-like"))
}

fn ensure_roots(label: &str, values: &[String]) -> Result<()> {
    ensure(!values.is_empty(), &format!("{label} must be non-empty"))?;
    for value in values {
        ensure_root(label, value)?;
    }
    Ok(())
}

fn ensure_bps(label: &str, value: u16) -> Result<()> {
    ensure(value <= 10_000, &format!("{label} must be <= 10000"))
}

fn ensure_bps_at_least(label: &str, value: u16, floor: u16) -> Result<()> {
    ensure(value >= floor, &format!("{label} must be >= {floor}"))
}
