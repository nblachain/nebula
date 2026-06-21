use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileRuntimeAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillIncidentHandoffRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-compile-runtime-accepted-live-evidence-operator-dashboard-release-policy-deployment-guard-rollback-drill-incident-handoff-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_RUNTIME_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_INCIDENT_HANDOFF_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const HANDOFF_SUITE: &str =
    "monero-l2-pq-force-exit-compile-runtime-rollback-drill-incident-handoff-v1";
pub const DEFAULT_RELEASE_EPOCH: u64 = 86;
pub const DEFAULT_SOURCE_ROLLBACK_DRILL_EPOCH: u64 = 85;
pub const DEFAULT_HANDOFF_HEIGHT: u64 = 860_000;
pub const DEFAULT_EVIDENCE_HEIGHT: u64 = 850_000;
pub const DEFAULT_MAX_EVIDENCE_AGE_BLOCKS: u64 = 12_000;
pub const DEFAULT_MIN_OPERATOR_ACKS: u16 = 4;
pub const DEFAULT_MIN_HANDOFF_SECTIONS: u16 = 6;
pub const DEFAULT_MIN_BLOCKER_RECEIPTS: u16 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub handoff_suite: String,
    pub release_epoch: u64,
    pub source_rollback_drill_epoch: u64,
    pub handoff_height: u64,
    pub evidence_height: u64,
    pub release_channel: String,
    pub deployment_environment: String,
    pub max_evidence_age_blocks: u64,
    pub min_operator_acks: u16,
    pub min_handoff_sections: u16,
    pub min_blocker_receipts: u16,
    pub require_wave_85_rollback_drill_root: bool,
    pub require_fail_closed_release_hold: bool,
    pub require_operator_handoff_quorum: bool,
    pub require_blocker_receipts: bool,
    pub require_private_payload_redaction: bool,
    pub allow_production_deploy: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            handoff_suite: HANDOFF_SUITE.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            source_rollback_drill_epoch: DEFAULT_SOURCE_ROLLBACK_DRILL_EPOCH,
            handoff_height: DEFAULT_HANDOFF_HEIGHT,
            evidence_height: DEFAULT_EVIDENCE_HEIGHT,
            release_channel: "devnet-compile-runtime-incident-handoff".to_string(),
            deployment_environment: "devnet-production-shadow".to_string(),
            max_evidence_age_blocks: DEFAULT_MAX_EVIDENCE_AGE_BLOCKS,
            min_operator_acks: DEFAULT_MIN_OPERATOR_ACKS,
            min_handoff_sections: DEFAULT_MIN_HANDOFF_SECTIONS,
            min_blocker_receipts: DEFAULT_MIN_BLOCKER_RECEIPTS,
            require_wave_85_rollback_drill_root: true,
            require_fail_closed_release_hold: true,
            require_operator_handoff_quorum: true,
            require_blocker_receipts: true,
            require_private_payload_redaction: true,
            allow_production_deploy: false,
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
        ensure_non_empty("handoff_suite", &self.handoff_suite)?;
        ensure_non_empty("release_channel", &self.release_channel)?;
        ensure_non_empty("deployment_environment", &self.deployment_environment)?;
        ensure(self.schema_version > 0, "schema version must be non-zero")?;
        ensure(self.release_epoch > 0, "release epoch must be non-zero")?;
        ensure(
            self.source_rollback_drill_epoch > 0,
            "source rollback drill epoch must be non-zero",
        )?;
        ensure(
            self.release_epoch > self.source_rollback_drill_epoch,
            "incident handoff release epoch must follow rollback drill epoch",
        )?;
        ensure(self.handoff_height > 0, "handoff height must be non-zero")?;
        ensure(self.evidence_height > 0, "evidence height must be non-zero")?;
        ensure(
            self.handoff_height >= self.evidence_height,
            "handoff height must not precede evidence height",
        )?;
        ensure(
            self.handoff_height - self.evidence_height <= self.max_evidence_age_blocks,
            "rollback drill evidence is too old for handoff",
        )?;
        ensure(
            self.min_operator_acks > 0,
            "operator acknowledgement minimum must be non-zero",
        )?;
        ensure(
            self.min_handoff_sections > 0,
            "handoff section minimum must be non-zero",
        )?;
        ensure(
            self.min_blocker_receipts > 0,
            "blocker receipt minimum must be non-zero",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "handoff_suite": self.handoff_suite,
            "release_epoch": self.release_epoch,
            "source_rollback_drill_epoch": self.source_rollback_drill_epoch,
            "handoff_height": self.handoff_height,
            "evidence_height": self.evidence_height,
            "release_channel": self.release_channel,
            "deployment_environment": self.deployment_environment,
            "max_evidence_age_blocks": self.max_evidence_age_blocks,
            "min_operator_acks": self.min_operator_acks,
            "min_handoff_sections": self.min_handoff_sections,
            "min_blocker_receipts": self.min_blocker_receipts,
            "require_wave_85_rollback_drill_root": self.require_wave_85_rollback_drill_root,
            "require_fail_closed_release_hold": self.require_fail_closed_release_hold,
            "require_operator_handoff_quorum": self.require_operator_handoff_quorum,
            "require_blocker_receipts": self.require_blocker_receipts,
            "require_private_payload_redaction": self.require_private_payload_redaction,
            "allow_production_deploy": self.allow_production_deploy,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSectionKind {
    IncidentSummary,
    RollbackDrillEvidence,
    BlockerLedger,
    OperatorAssignments,
    DashboardPosting,
    ReleaseHoldOrder,
    EscalationPath,
}

impl HandoffSectionKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::IncidentSummary,
            Self::RollbackDrillEvidence,
            Self::BlockerLedger,
            Self::OperatorAssignments,
            Self::DashboardPosting,
            Self::ReleaseHoldOrder,
            Self::EscalationPath,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::IncidentSummary => "incident_summary",
            Self::RollbackDrillEvidence => "rollback_drill_evidence",
            Self::BlockerLedger => "blocker_ledger",
            Self::OperatorAssignments => "operator_assignments",
            Self::DashboardPosting => "dashboard_posting",
            Self::ReleaseHoldOrder => "release_hold_order",
            Self::EscalationPath => "escalation_path",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentBlockerKind {
    DeferredHeavyGate,
    ActiveAbortCriterion,
    ReleaseHoldRequired,
    OperatorQuorumPending,
    DashboardRootDrift,
}

impl IncidentBlockerKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::DeferredHeavyGate,
            Self::ActiveAbortCriterion,
            Self::ReleaseHoldRequired,
            Self::OperatorQuorumPending,
            Self::DashboardRootDrift,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeferredHeavyGate => "deferred_heavy_gate",
            Self::ActiveAbortCriterion => "active_abort_criterion",
            Self::ReleaseHoldRequired => "release_hold_required",
            Self::OperatorQuorumPending => "operator_quorum_pending",
            Self::DashboardRootDrift => "dashboard_root_drift",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorRole {
    ReleaseCaptain,
    RuntimeOwner,
    SecurityWatch,
    BridgeCustody,
}

impl OperatorRole {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReleaseCaptain,
            Self::RuntimeOwner,
            Self::SecurityWatch,
            Self::BridgeCustody,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCaptain => "release_captain",
            Self::RuntimeOwner => "runtime_owner",
            Self::SecurityWatch => "security_watch",
            Self::BridgeCustody => "bridge_custody",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RollbackDrillEvidence {
    pub source_epoch: u64,
    pub observed_height: u64,
    pub rollback_drill_root: String,
    pub verdict_root: String,
    pub blocker_root: String,
    pub evidence_bundle_root: String,
    pub blocker_count: u16,
    pub heavy_gate_blocker_count: u16,
    pub active_abort_count: u16,
    pub release_held: bool,
}

impl RollbackDrillEvidence {
    pub fn devnet(config: &Config) -> Self {
        let rollback_drill_root = sample_root("wave-85-rollback-drill-state", "compile-runtime", 1);
        let verdict_root = sample_root("wave-85-rollback-drill-verdict", "fail-closed", 2);
        let blocker_root = sample_root("wave-85-rollback-drill-blockers", "active", 3);
        let evidence_bundle_root = roots_root(
            "incident-handoff-source-evidence",
            [
                rollback_drill_root.clone(),
                verdict_root.clone(),
                blocker_root.clone(),
            ],
        );
        Self {
            source_epoch: config.source_rollback_drill_epoch,
            observed_height: config.evidence_height,
            rollback_drill_root,
            verdict_root,
            blocker_root,
            evidence_bundle_root,
            blocker_count: 5,
            heavy_gate_blocker_count: 3,
            active_abort_count: 2,
            release_held: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "source_epoch": self.source_epoch,
            "observed_height": self.observed_height,
            "rollback_drill_root": self.rollback_drill_root,
            "verdict_root": self.verdict_root,
            "blocker_root": self.blocker_root,
            "evidence_bundle_root": self.evidence_bundle_root,
            "blocker_count": self.blocker_count,
            "heavy_gate_blocker_count": self.heavy_gate_blocker_count,
            "active_abort_count": self.active_abort_count,
            "release_held": self.release_held,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rollback-drill-evidence", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("rollback drill root", &self.rollback_drill_root)?;
        ensure_non_empty("rollback drill verdict root", &self.verdict_root)?;
        ensure_non_empty("rollback drill blocker root", &self.blocker_root)?;
        ensure_non_empty("evidence bundle root", &self.evidence_bundle_root)?;
        ensure(self.source_epoch > 0, "source epoch must be non-zero")?;
        ensure(
            self.observed_height > 0,
            "observed evidence height must be non-zero",
        )?;
        ensure(self.blocker_count > 0, "handoff must carry blockers")?;
        ensure(
            self.release_held,
            "wave 85 evidence must show release held for fail-closed handoff",
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffSectionReceipt {
    pub section: HandoffSectionKind,
    pub label: String,
    pub content_root: String,
    pub redaction_root: String,
    pub section_root: String,
}

impl HandoffSectionReceipt {
    pub fn devnet(section: HandoffSectionKind, ordinal: u64) -> Self {
        let label = section.as_str().to_string();
        let content_root = sample_root("incident-handoff-section-content", &label, ordinal);
        let redaction_root = sample_root("incident-handoff-section-redaction", &label, ordinal);
        let section_root = roots_root(
            "incident-handoff-section",
            [content_root.clone(), redaction_root.clone()],
        );
        Self {
            section,
            label,
            content_root,
            redaction_root,
            section_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "section": self.section.as_str(),
            "label": self.label,
            "content_root": self.content_root,
            "redaction_root": self.redaction_root,
            "section_root": self.section_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("handoff-section", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("handoff section label", &self.label)?;
        ensure_non_empty("handoff section content root", &self.content_root)?;
        ensure_non_empty("handoff section redaction root", &self.redaction_root)?;
        ensure_non_empty("handoff section root", &self.section_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockerReceipt {
    pub kind: IncidentBlockerKind,
    pub label: String,
    pub blocker_root: String,
    pub mitigation_commitment_root: String,
    pub fail_closed: bool,
}

impl BlockerReceipt {
    pub fn devnet(kind: IncidentBlockerKind, ordinal: u64) -> Self {
        let label = kind.as_str().to_string();
        Self {
            kind,
            label: label.clone(),
            blocker_root: sample_root("incident-handoff-blocker", &label, ordinal),
            mitigation_commitment_root: sample_root(
                "incident-handoff-mitigation-commitment",
                &label,
                ordinal,
            ),
            fail_closed: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "label": self.label,
            "blocker_root": self.blocker_root,
            "mitigation_commitment_root": self.mitigation_commitment_root,
            "fail_closed": self.fail_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("blocker-receipt", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker label", &self.label)?;
        ensure_non_empty("blocker root", &self.blocker_root)?;
        ensure_non_empty(
            "blocker mitigation commitment root",
            &self.mitigation_commitment_root,
        )?;
        ensure(self.fail_closed, "blocker receipt must be fail-closed")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorHandoffAck {
    pub operator_label: String,
    pub role: OperatorRole,
    pub acknowledged_height: u64,
    pub handoff_package_root: String,
    pub ack_root: String,
}

impl OperatorHandoffAck {
    pub fn devnet(role: OperatorRole, config: &Config, ordinal: u64) -> Self {
        let operator_label = format!("operator-{}", role.as_str().replace('_', "-"));
        let handoff_package_root = sample_root("incident-handoff-package", role.as_str(), ordinal);
        let ack_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-OPERATOR-ACK",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(role.as_str()),
                HashPart::Str(&operator_label),
                HashPart::U64(config.handoff_height),
                HashPart::Str(&handoff_package_root),
            ],
            32,
        );
        Self {
            operator_label,
            role,
            acknowledged_height: config.handoff_height,
            handoff_package_root,
            ack_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "operator_label": self.operator_label,
            "role": self.role.as_str(),
            "acknowledged_height": self.acknowledged_height,
            "handoff_package_root": self.handoff_package_root,
            "ack_root": self.ack_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator-handoff-ack", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("operator label", &self.operator_label)?;
        ensure_non_empty("handoff package root", &self.handoff_package_root)?;
        ensure_non_empty("operator acknowledgement root", &self.ack_root)?;
        ensure(
            self.acknowledged_height > 0,
            "operator acknowledgement height must be non-zero",
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandoffVerdict {
    pub fail_closed: bool,
    pub release_held: bool,
    pub production_deploy_allowed: bool,
    pub private_payloads_redacted: bool,
    pub evidence_root: String,
    pub handoff_section_root: String,
    pub blocker_receipt_root: String,
    pub operator_ack_root: String,
    pub blocker_root: String,
    pub handoff_package_root: String,
    pub blocker_count: usize,
    pub handoff_section_count: usize,
    pub blocker_receipt_count: usize,
    pub operator_ack_count: usize,
}

impl HandoffVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "fail_closed": self.fail_closed,
            "release_held": self.release_held,
            "production_deploy_allowed": self.production_deploy_allowed,
            "private_payloads_redacted": self.private_payloads_redacted,
            "evidence_root": self.evidence_root,
            "handoff_section_root": self.handoff_section_root,
            "blocker_receipt_root": self.blocker_receipt_root,
            "operator_ack_root": self.operator_ack_root,
            "blocker_root": self.blocker_root,
            "handoff_package_root": self.handoff_package_root,
            "blocker_count": self.blocker_count,
            "handoff_section_count": self.handoff_section_count,
            "blocker_receipt_count": self.blocker_receipt_count,
            "operator_ack_count": self.operator_ack_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("handoff-verdict", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        ensure(self.fail_closed, "handoff verdict must be fail-closed")?;
        ensure(self.release_held, "handoff verdict must keep release held")?;
        ensure(
            !self.production_deploy_allowed,
            "handoff verdict must not allow production deploy",
        )?;
        ensure(
            self.private_payloads_redacted,
            "handoff verdict must confirm private payload redaction",
        )?;
        ensure_non_empty("handoff package root", &self.handoff_package_root)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub rollback_drill_evidence: RollbackDrillEvidence,
    pub handoff_sections: Vec<HandoffSectionReceipt>,
    pub blocker_receipts: Vec<BlockerReceipt>,
    pub operator_acks: Vec<OperatorHandoffAck>,
    pub blockers: BTreeMap<String, Vec<String>>,
    pub verdict: HandoffVerdict,
}

impl State {
    pub fn devnet() -> Self {
        match Self::try_devnet() {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn try_devnet() -> Result<Self> {
        let config = Config::devnet();
        config.validate()?;
        let rollback_drill_evidence = RollbackDrillEvidence::devnet(&config);
        rollback_drill_evidence.validate()?;
        let handoff_sections = HandoffSectionKind::all()
            .into_iter()
            .enumerate()
            .map(|(index, section)| HandoffSectionReceipt::devnet(section, one_based(index)))
            .collect::<Vec<_>>();
        let blocker_receipts = IncidentBlockerKind::all()
            .into_iter()
            .enumerate()
            .map(|(index, blocker)| BlockerReceipt::devnet(blocker, one_based(index)))
            .collect::<Vec<_>>();
        let operator_acks = OperatorRole::all()
            .into_iter()
            .enumerate()
            .map(|(index, role)| OperatorHandoffAck::devnet(role, &config, one_based(index)))
            .collect::<Vec<_>>();
        for section in &handoff_sections {
            section.validate()?;
        }
        for receipt in &blocker_receipts {
            receipt.validate()?;
        }
        for ack in &operator_acks {
            ack.validate()?;
        }
        validate_unique_roots(
            "handoff section roots",
            handoff_sections.iter().map(|section| &section.section_root),
        )?;
        validate_unique_roots(
            "blocker receipt roots",
            blocker_receipts.iter().map(|receipt| &receipt.blocker_root),
        )?;
        validate_unique_roots(
            "operator acknowledgement roots",
            operator_acks.iter().map(|ack| &ack.ack_root),
        )?;
        let blockers = evaluate_blockers(
            &config,
            &rollback_drill_evidence,
            &handoff_sections,
            &blocker_receipts,
            &operator_acks,
        );
        let verdict = build_verdict(
            &config,
            &rollback_drill_evidence,
            &handoff_sections,
            &blocker_receipts,
            &operator_acks,
            &blockers,
        );
        verdict.validate()?;
        Ok(Self {
            config,
            rollback_drill_evidence,
            handoff_sections,
            blocker_receipts,
            operator_acks,
            blockers,
            verdict,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "chain_id": CHAIN_ID,
            "config": self.config.public_record(),
            "rollback_drill_evidence": self.rollback_drill_evidence.public_record(),
            "handoff_sections": self.handoff_sections.iter().map(HandoffSectionReceipt::public_record).collect::<Vec<_>>(),
            "blocker_receipts": self.blocker_receipts.iter().map(BlockerReceipt::public_record).collect::<Vec<_>>(),
            "operator_acks": self.operator_acks.iter().map(OperatorHandoffAck::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers,
            "verdict": self.verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.release_channel),
                HashPart::U64(self.config.release_epoch),
                HashPart::U64(self.config.handoff_height),
                HashPart::Str(&self.verdict.handoff_package_root),
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

fn build_verdict(
    config: &Config,
    evidence: &RollbackDrillEvidence,
    handoff_sections: &[HandoffSectionReceipt],
    blocker_receipts: &[BlockerReceipt],
    operator_acks: &[OperatorHandoffAck],
    blockers: &BTreeMap<String, Vec<String>>,
) -> HandoffVerdict {
    let evidence_root = evidence.state_root();
    let handoff_section_root = roots_root(
        "incident-handoff-sections",
        handoff_sections
            .iter()
            .map(HandoffSectionReceipt::state_root),
    );
    let blocker_receipt_root = roots_root(
        "incident-handoff-blocker-receipts",
        blocker_receipts.iter().map(BlockerReceipt::state_root),
    );
    let operator_ack_root = roots_root(
        "incident-handoff-operator-acks",
        operator_acks.iter().map(OperatorHandoffAck::state_root),
    );
    let blocker_root = blockers_root(blockers);
    let blocker_count = blockers.values().map(Vec::len).sum::<usize>();
    let fail_closed = config.require_fail_closed_release_hold || !config.allow_production_deploy;
    let release_held = fail_closed || blocker_count > 0 || evidence.blocker_count > 0;
    let production_deploy_allowed = config.allow_production_deploy && !release_held;
    let private_payloads_redacted = config.require_private_payload_redaction;
    let handoff_package_root = roots_root(
        "incident-handoff-package",
        [
            config.state_root(),
            evidence_root.clone(),
            handoff_section_root.clone(),
            blocker_receipt_root.clone(),
            operator_ack_root.clone(),
            blocker_root.clone(),
        ],
    );
    HandoffVerdict {
        fail_closed,
        release_held,
        production_deploy_allowed,
        private_payloads_redacted,
        evidence_root,
        handoff_section_root,
        blocker_receipt_root,
        operator_ack_root,
        blocker_root,
        handoff_package_root,
        blocker_count,
        handoff_section_count: handoff_sections.len(),
        blocker_receipt_count: blocker_receipts.len(),
        operator_ack_count: operator_acks.len(),
    }
}

fn evaluate_blockers(
    config: &Config,
    evidence: &RollbackDrillEvidence,
    handoff_sections: &[HandoffSectionReceipt],
    blocker_receipts: &[BlockerReceipt],
    operator_acks: &[OperatorHandoffAck],
) -> BTreeMap<String, Vec<String>> {
    let mut blockers = BTreeMap::<String, Vec<String>>::new();
    if config.require_wave_85_rollback_drill_root && evidence.rollback_drill_root.trim().is_empty()
    {
        push_blocker(
            &mut blockers,
            "wave_85_evidence",
            "missing_wave_85_rollback_drill_root",
        );
    }
    if !evidence.release_held {
        push_blocker(
            &mut blockers,
            "wave_85_evidence",
            "source_rollback_drill_not_fail_closed",
        );
    }
    if evidence.blocker_count > 0 {
        push_blocker(
            &mut blockers,
            "wave_85_evidence",
            "source_rollback_drill_blockers_remain_active",
        );
    }
    if handoff_sections.len() < usize::from(config.min_handoff_sections) {
        push_blocker(
            &mut blockers,
            "handoff_sections",
            "handoff_section_quorum_missing",
        );
    }
    if config.require_blocker_receipts
        && blocker_receipts.len() < usize::from(config.min_blocker_receipts)
    {
        push_blocker(
            &mut blockers,
            "blocker_receipts",
            "blocker_receipt_quorum_missing",
        );
    }
    if blocker_receipts.iter().any(|receipt| !receipt.fail_closed) {
        push_blocker(
            &mut blockers,
            "blocker_receipts",
            "blocker_receipt_not_fail_closed",
        );
    }
    if config.require_operator_handoff_quorum
        && operator_acks.len() < usize::from(config.min_operator_acks)
    {
        push_blocker(
            &mut blockers,
            "operator_acks",
            "operator_handoff_quorum_missing",
        );
    }
    if config.require_fail_closed_release_hold {
        push_blocker(
            &mut blockers,
            "release_hold",
            "fail_closed_release_hold_required_for_incident_handoff",
        );
    }
    if !config.allow_production_deploy {
        push_blocker(
            &mut blockers,
            "production_deploy",
            "production_deploy_disabled_by_incident_handoff_policy",
        );
    }
    blockers
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let rollback_drill_evidence = RollbackDrillEvidence {
        source_epoch: config.source_rollback_drill_epoch,
        observed_height: config.evidence_height,
        rollback_drill_root: sample_root("fallback-rollback-drill-root", "missing", 0),
        verdict_root: sample_root("fallback-rollback-drill-verdict", "abort", 0),
        blocker_root: sample_root("fallback-rollback-drill-blockers", "abort", 0),
        evidence_bundle_root: sample_root("fallback-evidence-bundle", "abort", 0),
        blocker_count: 1,
        heavy_gate_blocker_count: 1,
        active_abort_count: 1,
        release_held: true,
    };
    let handoff_sections = Vec::new();
    let blocker_receipts = vec![BlockerReceipt::devnet(
        IncidentBlockerKind::DashboardRootDrift,
        1,
    )];
    let operator_acks = Vec::new();
    let mut blockers = BTreeMap::<String, Vec<String>>::new();
    push_blocker(
        &mut blockers,
        "fallback",
        "incident_handoff_state_construction_failed",
    );
    push_blocker(&mut blockers, "fallback", &reason);
    let verdict = build_verdict(
        &config,
        &rollback_drill_evidence,
        &handoff_sections,
        &blocker_receipts,
        &operator_acks,
        &blockers,
    );
    State {
        config,
        rollback_drill_evidence,
        handoff_sections,
        blocker_receipts,
        operator_acks,
        blockers,
        verdict,
    }
}

fn push_blocker(blockers: &mut BTreeMap<String, Vec<String>>, subject: &str, blocker: &str) {
    blockers
        .entry(subject.to_string())
        .or_default()
        .push(blocker.to_string());
}

fn one_based(index: usize) -> u64 {
    index as u64 + 1
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

fn blockers_root(blockers: &BTreeMap<String, Vec<String>>) -> String {
    let leaves = blockers
        .iter()
        .map(|(subject, blocker_list)| {
            json!({
                "subject": subject,
                "blockers": blocker_list,
            })
        })
        .collect::<Vec<_>>();
    merkle_root("incident-handoff-blockers", &leaves)
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
        "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-RECORD",
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
        "MONERO-L2-PQ-BRIDGE-INCIDENT-HANDOFF-SAMPLE-ROOT",
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
