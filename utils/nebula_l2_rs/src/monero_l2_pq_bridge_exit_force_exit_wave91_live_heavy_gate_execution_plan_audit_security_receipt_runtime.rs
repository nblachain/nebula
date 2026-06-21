use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, RuntimeError>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave91-live-heavy-gate-execution-plan-audit-security-receipt-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const RECEIPT_SUITE: &str = "roots-only-live-heavy-gate-audit-security-receipts-v1";
pub const DEFAULT_CHAIN_ID: &str = "nebula-l2-devnet";
pub const DEFAULT_WAVE: u64 = 91;
pub const DEFAULT_SOURCE_WAVE: u64 = 90;
pub const DEFAULT_MIN_REVIEWER_SIGNOFFS: usize = 4;
pub const DEFAULT_MIN_OPERATOR_SIGNOFFS: usize = 2;
pub const DEFAULT_MIN_ADVERSARIAL_SCENARIOS: usize = 8;
pub const DEFAULT_MIN_AUDIT_SCOPE_COVERAGE_BPS: u16 = 9_800;
pub const DEFAULT_MIN_PRIVACY_COVERAGE_BPS: u16 = 9_850;
pub const DEFAULT_MIN_THREAT_MODEL_COVERAGE_BPS: u16 = 9_850;
pub const DEFAULT_MIN_COMMAND_HINTS: usize = 6;
pub const DEFAULT_MAX_OPEN_BLOCKERS: usize = 0;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RuntimeError {
    EmptyRoot { field: String },
    MissingReceipt { receipt: String },
    ClearanceBlocked { reason: String },
}

impl RuntimeError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRoot { .. } => "empty_root",
            Self::MissingReceipt { .. } => "missing_receipt",
            Self::ClearanceBlocked { .. } => "clearance_blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDomain {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
    ReleaseControl,
}

impl GateDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialScenario => "adversarial_scenario",
            Self::ThreatModel => "threat_model",
            Self::PrivacyReview => "privacy_review",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorSignoff => "operator_signoff",
            Self::ReleaseControl => "release_control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialKind {
    AuditReceiptAbsent,
    AdversarialCoverageAbsent,
    ThreatModelAcceptanceAbsent,
    PrivacyReceiptAbsent,
    ReviewerQuorumAbsent,
    OperatorQuorumAbsent,
    HeavyGateDeferred,
    ProductionLaneHeld,
}

impl DenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReceiptAbsent => "audit_receipt_absent",
            Self::AdversarialCoverageAbsent => "adversarial_coverage_absent",
            Self::ThreatModelAcceptanceAbsent => "threat_model_acceptance_absent",
            Self::PrivacyReceiptAbsent => "privacy_receipt_absent",
            Self::ReviewerQuorumAbsent => "reviewer_quorum_absent",
            Self::OperatorQuorumAbsent => "operator_quorum_absent",
            Self::HeavyGateDeferred => "heavy_gate_deferred",
            Self::ProductionLaneHeld => "production_lane_held",
        }
    }

    pub fn domain(self) -> GateDomain {
        match self {
            Self::AuditReceiptAbsent => GateDomain::AuditReview,
            Self::AdversarialCoverageAbsent => GateDomain::AdversarialScenario,
            Self::ThreatModelAcceptanceAbsent => GateDomain::ThreatModel,
            Self::PrivacyReceiptAbsent => GateDomain::PrivacyReview,
            Self::ReviewerQuorumAbsent => GateDomain::ReviewerSignoff,
            Self::OperatorQuorumAbsent => GateDomain::OperatorSignoff,
            Self::HeavyGateDeferred | Self::ProductionLaneHeld => GateDomain::ReleaseControl,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Planned,
    Collected,
    Accepted,
    Rejected,
}

impl ReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Collected => "collected",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }

    pub fn clears_gate(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioFamily {
    MoneroReorg,
    WatcherCensorship,
    SequencerFailure,
    PqSignerCompromise,
    LiquidityExhaustion,
    NullifierReplay,
    ReceiptForgery,
    MetadataCorrelation,
    EmergencyPause,
    OperatorMistake,
}

impl ScenarioFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroReorg => "monero_reorg",
            Self::WatcherCensorship => "watcher_censorship",
            Self::SequencerFailure => "sequencer_failure",
            Self::PqSignerCompromise => "pq_signer_compromise",
            Self::LiquidityExhaustion => "liquidity_exhaustion",
            Self::NullifierReplay => "nullifier_replay",
            Self::ReceiptForgery => "receipt_forgery",
            Self::MetadataCorrelation => "metadata_correlation",
            Self::EmergencyPause => "emergency_pause",
            Self::OperatorMistake => "operator_mistake",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffRole {
    AuditReviewer,
    SecurityReviewer,
    PrivacyReviewer,
    ThreatModelOwner,
    ReleaseOperator,
    IncidentCommander,
}

impl SignoffRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReviewer => "audit_reviewer",
            Self::SecurityReviewer => "security_reviewer",
            Self::PrivacyReviewer => "privacy_reviewer",
            Self::ThreatModelOwner => "threat_model_owner",
            Self::ReleaseOperator => "release_operator",
            Self::IncidentCommander => "incident_commander",
        }
    }

    pub fn is_reviewer(self) -> bool {
        matches!(
            self,
            Self::AuditReviewer
                | Self::SecurityReviewer
                | Self::PrivacyReviewer
                | Self::ThreatModelOwner
        )
    }

    pub fn is_operator(self) -> bool {
        matches!(self, Self::ReleaseOperator | Self::IncidentCommander)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorAction {
    CollectAuditRoot,
    RunAdversarialReview,
    CollectThreatModelAcceptance,
    CollectPrivacyRoot,
    FreezeReleaseLane,
    EnableReleaseLane,
    PublishClearanceRecord,
    KeepFailClosed,
}

impl OperatorAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CollectAuditRoot => "collect_audit_root",
            Self::RunAdversarialReview => "run_adversarial_review",
            Self::CollectThreatModelAcceptance => "collect_threat_model_acceptance",
            Self::CollectPrivacyRoot => "collect_privacy_root",
            Self::FreezeReleaseLane => "freeze_release_lane",
            Self::EnableReleaseLane => "enable_release_lane",
            Self::PublishClearanceRecord => "publish_clearance_record",
            Self::KeepFailClosed => "keep_fail_closed",
        }
    }

    pub fn domain(self) -> GateDomain {
        match self {
            Self::CollectAuditRoot => GateDomain::AuditReview,
            Self::RunAdversarialReview => GateDomain::AdversarialScenario,
            Self::CollectThreatModelAcceptance => GateDomain::ThreatModel,
            Self::CollectPrivacyRoot => GateDomain::PrivacyReview,
            Self::FreezeReleaseLane
            | Self::EnableReleaseLane
            | Self::PublishClearanceRecord
            | Self::KeepFailClosed => GateDomain::ReleaseControl,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceVerdict {
    Clear,
    HoldFailClosed,
}

impl ClearanceVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::HoldFailClosed => "hold_fail_closed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub wave: u64,
    pub source_wave: u64,
    pub receipt_suite: String,
    pub min_reviewer_signoffs: usize,
    pub min_operator_signoffs: usize,
    pub min_adversarial_scenarios: usize,
    pub min_audit_scope_coverage_bps: u16,
    pub min_privacy_coverage_bps: u16,
    pub min_threat_model_coverage_bps: u16,
    pub min_command_hints: usize,
    pub max_open_blockers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEFAULT_CHAIN_ID.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            receipt_suite: RECEIPT_SUITE.to_string(),
            min_reviewer_signoffs: DEFAULT_MIN_REVIEWER_SIGNOFFS,
            min_operator_signoffs: DEFAULT_MIN_OPERATOR_SIGNOFFS,
            min_adversarial_scenarios: DEFAULT_MIN_ADVERSARIAL_SCENARIOS,
            min_audit_scope_coverage_bps: DEFAULT_MIN_AUDIT_SCOPE_COVERAGE_BPS,
            min_privacy_coverage_bps: DEFAULT_MIN_PRIVACY_COVERAGE_BPS,
            min_threat_model_coverage_bps: DEFAULT_MIN_THREAT_MODEL_COVERAGE_BPS,
            min_command_hints: DEFAULT_MIN_COMMAND_HINTS,
            max_open_blockers: DEFAULT_MAX_OPEN_BLOCKERS,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "receipt_suite": self.receipt_suite,
            "min_reviewer_signoffs": self.min_reviewer_signoffs,
            "min_operator_signoffs": self.min_operator_signoffs,
            "min_adversarial_scenarios": self.min_adversarial_scenarios,
            "min_audit_scope_coverage_bps": self.min_audit_scope_coverage_bps,
            "min_privacy_coverage_bps": self.min_privacy_coverage_bps,
            "min_threat_model_coverage_bps": self.min_threat_model_coverage_bps,
            "min_command_hints": self.min_command_hints,
            "max_open_blockers": self.max_open_blockers,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wave90DenialIntake {
    pub intake_id: String,
    pub denial_output_root: String,
    pub denied_plan_root: String,
    pub blocker_roots: Vec<String>,
    pub denial_kinds: Vec<DenialKind>,
    pub observed_at_height: u64,
}

impl Wave90DenialIntake {
    pub fn devnet() -> Self {
        let denial_output_root = seed_root("wave90_denial_output", 0);
        let denied_plan_root = seed_root("wave90_denied_plan", 1);
        let blocker_roots = (0..8)
            .map(|index| seed_root("wave90_blocker_root", index))
            .collect::<Vec<_>>();
        let denial_kinds = vec![
            DenialKind::AuditReceiptAbsent,
            DenialKind::AdversarialCoverageAbsent,
            DenialKind::ThreatModelAcceptanceAbsent,
            DenialKind::PrivacyReceiptAbsent,
            DenialKind::ReviewerQuorumAbsent,
            DenialKind::OperatorQuorumAbsent,
            DenialKind::HeavyGateDeferred,
            DenialKind::ProductionLaneHeld,
        ];
        let intake_id = domain_hash(
            "WAVE91-DENIAL-INTAKE-ID",
            &[
                HashPart::Str(&denial_output_root),
                HashPart::Str(&denied_plan_root),
                HashPart::U64(blocker_roots.len() as u64),
            ],
            16,
        );
        Self {
            intake_id,
            denial_output_root,
            denied_plan_root,
            blocker_roots,
            denial_kinds,
            observed_at_height: 91_090,
        }
    }

    pub fn domains(&self) -> BTreeSet<GateDomain> {
        self.denial_kinds.iter().map(|kind| kind.domain()).collect()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "intake_id": self.intake_id,
            "denial_output_root": self.denial_output_root,
            "denied_plan_root": self.denied_plan_root,
            "blocker_roots": self.blocker_roots,
            "denial_kinds": self.denial_kinds.iter().map(|kind| kind.as_str()).collect::<Vec<_>>(),
            "observed_at_height": self.observed_at_height,
            "domain_count": self.domains().len(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave90_denial_intake", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        require_root("denial_output_root", &self.denial_output_root)?;
        require_root("denied_plan_root", &self.denied_plan_root)?;
        for root in &self.blocker_roots {
            require_root("blocker_root", root)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannedAuditReceipt {
    pub receipt_id: String,
    pub audit_scope_root: String,
    pub audit_finding_root: String,
    pub mitigation_plan_root: String,
    pub coverage_bps: u16,
    pub open_blockers: usize,
    pub status: ReceiptStatus,
}

impl PlannedAuditReceipt {
    pub fn devnet() -> Self {
        Self::from_roots(
            seed_root("audit_scope", 0),
            seed_root("audit_findings_closed", 1),
            seed_root("audit_mitigation_plan", 2),
            9_920,
            0,
            ReceiptStatus::Accepted,
        )
    }

    pub fn from_roots(
        audit_scope_root: String,
        audit_finding_root: String,
        mitigation_plan_root: String,
        coverage_bps: u16,
        open_blockers: usize,
        status: ReceiptStatus,
    ) -> Self {
        let receipt_id = domain_hash(
            "WAVE91-AUDIT-RECEIPT-ID",
            &[
                HashPart::Str(&audit_scope_root),
                HashPart::Str(&audit_finding_root),
                HashPart::Str(&mitigation_plan_root),
                HashPart::U64(coverage_bps as u64),
            ],
            16,
        );
        Self {
            receipt_id,
            audit_scope_root,
            audit_finding_root,
            mitigation_plan_root,
            coverage_bps,
            open_blockers,
            status,
        }
    }

    pub fn clears(&self, config: &Config) -> bool {
        self.status.clears_gate()
            && self.coverage_bps >= config.min_audit_scope_coverage_bps
            && self.open_blockers <= config.max_open_blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "receipt_id": self.receipt_id,
            "audit_scope_root": self.audit_scope_root,
            "audit_finding_root": self.audit_finding_root,
            "mitigation_plan_root": self.mitigation_plan_root,
            "coverage_bps": self.coverage_bps,
            "open_blockers": self.open_blockers,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("planned_audit_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdversarialScenarioReceipt {
    pub scenario_id: String,
    pub family: ScenarioFamily,
    pub scenario_root: String,
    pub defense_root: String,
    pub replay_root: String,
    pub residual_risk_root: String,
    pub status: ReceiptStatus,
}

impl AdversarialScenarioReceipt {
    pub fn new(family: ScenarioFamily, index: u64, status: ReceiptStatus) -> Self {
        let scenario_root = seed_root(family.as_str(), index);
        let defense_root = seed_root("defense_root", index);
        let replay_root = seed_root("replay_root", index);
        let residual_risk_root = seed_root("residual_risk_root", index);
        let scenario_id = domain_hash(
            "WAVE91-ADVERSARIAL-SCENARIO-ID",
            &[
                HashPart::Str(family.as_str()),
                HashPart::Str(&scenario_root),
                HashPart::Str(&defense_root),
                HashPart::Str(&replay_root),
            ],
            16,
        );
        Self {
            scenario_id,
            family,
            scenario_root,
            defense_root,
            replay_root,
            residual_risk_root,
            status,
        }
    }

    pub fn clears(&self) -> bool {
        self.status.clears_gate()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "scenario_id": self.scenario_id,
            "family": self.family.as_str(),
            "scenario_root": self.scenario_root,
            "defense_root": self.defense_root,
            "replay_root": self.replay_root,
            "residual_risk_root": self.residual_risk_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("adversarial_scenario_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ThreatModelAcceptance {
    pub acceptance_id: String,
    pub model_root: String,
    pub abuse_case_root: String,
    pub control_map_root: String,
    pub residual_risk_acceptance_root: String,
    pub coverage_bps: u16,
    pub status: ReceiptStatus,
}

impl ThreatModelAcceptance {
    pub fn devnet() -> Self {
        Self::from_roots(
            seed_root("threat_model", 0),
            seed_root("abuse_cases", 1),
            seed_root("control_map", 2),
            seed_root("risk_acceptance", 3),
            9_910,
            ReceiptStatus::Accepted,
        )
    }

    pub fn from_roots(
        model_root: String,
        abuse_case_root: String,
        control_map_root: String,
        residual_risk_acceptance_root: String,
        coverage_bps: u16,
        status: ReceiptStatus,
    ) -> Self {
        let acceptance_id = domain_hash(
            "WAVE91-THREAT-MODEL-ACCEPTANCE-ID",
            &[
                HashPart::Str(&model_root),
                HashPart::Str(&abuse_case_root),
                HashPart::Str(&control_map_root),
                HashPart::Str(&residual_risk_acceptance_root),
            ],
            16,
        );
        Self {
            acceptance_id,
            model_root,
            abuse_case_root,
            control_map_root,
            residual_risk_acceptance_root,
            coverage_bps,
            status,
        }
    }

    pub fn clears(&self, config: &Config) -> bool {
        self.status.clears_gate() && self.coverage_bps >= config.min_threat_model_coverage_bps
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "acceptance_id": self.acceptance_id,
            "model_root": self.model_root,
            "abuse_case_root": self.abuse_case_root,
            "control_map_root": self.control_map_root,
            "residual_risk_acceptance_root": self.residual_risk_acceptance_root,
            "coverage_bps": self.coverage_bps,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("threat_model_acceptance", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivacyReviewReceipt {
    pub receipt_id: String,
    pub privacy_scope_root: String,
    pub non_linkage_evidence_root: String,
    pub disclosure_budget_root: String,
    pub retention_policy_root: String,
    pub coverage_bps: u16,
    pub status: ReceiptStatus,
}

impl PrivacyReviewReceipt {
    pub fn devnet() -> Self {
        Self::from_roots(
            seed_root("privacy_scope", 0),
            seed_root("non_linkage_evidence", 1),
            seed_root("disclosure_budget", 2),
            seed_root("retention_policy", 3),
            9_930,
            ReceiptStatus::Accepted,
        )
    }

    pub fn from_roots(
        privacy_scope_root: String,
        non_linkage_evidence_root: String,
        disclosure_budget_root: String,
        retention_policy_root: String,
        coverage_bps: u16,
        status: ReceiptStatus,
    ) -> Self {
        let receipt_id = domain_hash(
            "WAVE91-PRIVACY-REVIEW-RECEIPT-ID",
            &[
                HashPart::Str(&privacy_scope_root),
                HashPart::Str(&non_linkage_evidence_root),
                HashPart::Str(&disclosure_budget_root),
                HashPart::Str(&retention_policy_root),
            ],
            16,
        );
        Self {
            receipt_id,
            privacy_scope_root,
            non_linkage_evidence_root,
            disclosure_budget_root,
            retention_policy_root,
            coverage_bps,
            status,
        }
    }

    pub fn clears(&self, config: &Config) -> bool {
        self.status.clears_gate() && self.coverage_bps >= config.min_privacy_coverage_bps
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "receipt_id": self.receipt_id,
            "privacy_scope_root": self.privacy_scope_root,
            "non_linkage_evidence_root": self.non_linkage_evidence_root,
            "disclosure_budget_root": self.disclosure_budget_root,
            "retention_policy_root": self.retention_policy_root,
            "coverage_bps": self.coverage_bps,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("privacy_review_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignoffReceipt {
    pub signoff_id: String,
    pub role: SignoffRole,
    pub authority_root: String,
    pub statement_root: String,
    pub observed_state_root: String,
    pub status: ReceiptStatus,
    pub signed_at_height: u64,
}

impl SignoffReceipt {
    pub fn new(role: SignoffRole, index: u64, observed_state_root: String) -> Self {
        let authority_root = seed_root(role.as_str(), index);
        let statement_root = seed_root("signoff_statement", index);
        let signoff_id = domain_hash(
            "WAVE91-SIGNOFF-RECEIPT-ID",
            &[
                HashPart::Str(role.as_str()),
                HashPart::Str(&authority_root),
                HashPart::Str(&statement_root),
                HashPart::Str(&observed_state_root),
                HashPart::U64(index),
            ],
            16,
        );
        Self {
            signoff_id,
            role,
            authority_root,
            statement_root,
            observed_state_root,
            status: ReceiptStatus::Accepted,
            signed_at_height: 91_200 + index,
        }
    }

    pub fn clears(&self) -> bool {
        self.status.clears_gate()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "signoff_id": self.signoff_id,
            "role": self.role.as_str(),
            "authority_root": self.authority_root,
            "statement_root": self.statement_root,
            "observed_state_root": self.observed_state_root,
            "status": self.status.as_str(),
            "signed_at_height": self.signed_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("signoff_receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorCommandHint {
    pub hint_id: String,
    pub action: OperatorAction,
    pub domain: GateDomain,
    pub command_root: String,
    pub guard_root: String,
    pub receipt_sink_root: String,
    pub fail_closed_on_absence: bool,
}

impl OperatorCommandHint {
    pub fn new(action: OperatorAction, index: u64) -> Self {
        let domain = action.domain();
        let command_root = seed_root(action.as_str(), index);
        let guard_root = seed_root("operator_guard", index);
        let receipt_sink_root = seed_root("receipt_sink", index);
        let hint_id = domain_hash(
            "WAVE91-OPERATOR-COMMAND-HINT-ID",
            &[
                HashPart::Str(action.as_str()),
                HashPart::Str(domain.as_str()),
                HashPart::Str(&command_root),
                HashPart::Str(&guard_root),
            ],
            16,
        );
        Self {
            hint_id,
            action,
            domain,
            command_root,
            guard_root,
            receipt_sink_root,
            fail_closed_on_absence: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "hint_id": self.hint_id,
            "action": self.action.as_str(),
            "domain": self.domain.as_str(),
            "command_root": self.command_root,
            "guard_root": self.guard_root,
            "receipt_sink_root": self.receipt_sink_root,
            "fail_closed_on_absence": self.fail_closed_on_absence,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("operator_command_hint", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcceptanceCriteria {
    pub criteria_id: String,
    pub required_domains: Vec<GateDomain>,
    pub required_denial_kinds: Vec<DenialKind>,
    pub min_reviewer_signoffs: usize,
    pub min_operator_signoffs: usize,
    pub min_adversarial_scenarios: usize,
    pub max_open_blockers: usize,
    pub fail_closed_default: bool,
}

impl AcceptanceCriteria {
    pub fn from_config_and_denial(config: &Config, denial: &Wave90DenialIntake) -> Self {
        let required_domains = denial.domains().into_iter().collect::<Vec<_>>();
        let required_denial_kinds = denial.denial_kinds.clone();
        let criteria_id = domain_hash(
            "WAVE91-ACCEPTANCE-CRITERIA-ID",
            &[
                HashPart::Str(&config.state_root()),
                HashPart::Str(&denial.state_root()),
                HashPart::U64(required_domains.len() as u64),
            ],
            16,
        );
        Self {
            criteria_id,
            required_domains,
            required_denial_kinds,
            min_reviewer_signoffs: config.min_reviewer_signoffs,
            min_operator_signoffs: config.min_operator_signoffs,
            min_adversarial_scenarios: config.min_adversarial_scenarios,
            max_open_blockers: config.max_open_blockers,
            fail_closed_default: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "criteria_id": self.criteria_id,
            "required_domains": self.required_domains.iter().map(|domain| domain.as_str()).collect::<Vec<_>>(),
            "required_denial_kinds": self.required_denial_kinds.iter().map(|kind| kind.as_str()).collect::<Vec<_>>(),
            "min_reviewer_signoffs": self.min_reviewer_signoffs,
            "min_operator_signoffs": self.min_operator_signoffs,
            "min_adversarial_scenarios": self.min_adversarial_scenarios,
            "max_open_blockers": self.max_open_blockers,
            "fail_closed_default": self.fail_closed_default,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("acceptance_criteria", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClearanceReport {
    pub verdict: ClearanceVerdict,
    pub cleared_domains: Vec<GateDomain>,
    pub blocked_domains: Vec<GateDomain>,
    pub missing_receipts: Vec<String>,
    pub action_hint_root: String,
    pub receipt_bundle_root: String,
}

impl ClearanceReport {
    pub fn is_clear(&self) -> bool {
        self.verdict == ClearanceVerdict::Clear
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "verdict": self.verdict.as_str(),
            "cleared_domains": self.cleared_domains.iter().map(|domain| domain.as_str()).collect::<Vec<_>>(),
            "blocked_domains": self.blocked_domains.iter().map(|domain| domain.as_str()).collect::<Vec<_>>(),
            "missing_receipts": self.missing_receipts,
            "action_hint_root": self.action_hint_root,
            "receipt_bundle_root": self.receipt_bundle_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("clearance_report", &self.public_record())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub denial_intake: Wave90DenialIntake,
    pub acceptance_criteria: AcceptanceCriteria,
    pub audit_receipt: PlannedAuditReceipt,
    pub adversarial_scenarios: Vec<AdversarialScenarioReceipt>,
    pub threat_model_acceptance: ThreatModelAcceptance,
    pub privacy_review: PrivacyReviewReceipt,
    pub reviewer_signoffs: Vec<SignoffReceipt>,
    pub operator_signoffs: Vec<SignoffReceipt>,
    pub operator_command_hints: Vec<OperatorCommandHint>,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let denial_intake = Wave90DenialIntake::devnet();
        let acceptance_criteria =
            AcceptanceCriteria::from_config_and_denial(&config, &denial_intake);
        let audit_receipt = PlannedAuditReceipt::devnet();
        let adversarial_scenarios = devnet_adversarial_scenarios();
        let threat_model_acceptance = ThreatModelAcceptance::devnet();
        let privacy_review = PrivacyReviewReceipt::devnet();
        let observed_state_root = merkle_root(&[
            audit_receipt.state_root(),
            scenario_root(&adversarial_scenarios),
            threat_model_acceptance.state_root(),
            privacy_review.state_root(),
        ]);
        let reviewer_signoffs = vec![
            SignoffReceipt::new(SignoffRole::AuditReviewer, 0, observed_state_root.clone()),
            SignoffReceipt::new(
                SignoffRole::SecurityReviewer,
                1,
                observed_state_root.clone(),
            ),
            SignoffReceipt::new(SignoffRole::PrivacyReviewer, 2, observed_state_root.clone()),
            SignoffReceipt::new(
                SignoffRole::ThreatModelOwner,
                3,
                observed_state_root.clone(),
            ),
        ];
        let operator_signoffs = vec![
            SignoffReceipt::new(SignoffRole::ReleaseOperator, 4, observed_state_root.clone()),
            SignoffReceipt::new(SignoffRole::IncidentCommander, 5, observed_state_root),
        ];
        let operator_command_hints = vec![
            OperatorCommandHint::new(OperatorAction::CollectAuditRoot, 0),
            OperatorCommandHint::new(OperatorAction::RunAdversarialReview, 1),
            OperatorCommandHint::new(OperatorAction::CollectThreatModelAcceptance, 2),
            OperatorCommandHint::new(OperatorAction::CollectPrivacyRoot, 3),
            OperatorCommandHint::new(OperatorAction::FreezeReleaseLane, 4),
            OperatorCommandHint::new(OperatorAction::PublishClearanceRecord, 5),
            OperatorCommandHint::new(OperatorAction::EnableReleaseLane, 6),
            OperatorCommandHint::new(OperatorAction::KeepFailClosed, 7),
        ];
        Self {
            config,
            denial_intake,
            acceptance_criteria,
            audit_receipt,
            adversarial_scenarios,
            threat_model_acceptance,
            privacy_review,
            reviewer_signoffs,
            operator_signoffs,
            operator_command_hints,
        }
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "config": self.config.public_record(),
            "denial_intake": self.denial_intake.public_record(),
            "acceptance_criteria": self.acceptance_criteria.public_record(),
            "audit_receipt": self.audit_receipt.public_record(),
            "adversarial_scenarios": self.adversarial_scenarios.iter().map(|receipt| receipt.public_record()).collect::<Vec<_>>(),
            "threat_model_acceptance": self.threat_model_acceptance.public_record(),
            "privacy_review": self.privacy_review.public_record(),
            "reviewer_signoffs": self.reviewer_signoffs.iter().map(|receipt| receipt.public_record()).collect::<Vec<_>>(),
            "operator_signoffs": self.operator_signoffs.iter().map(|receipt| receipt.public_record()).collect::<Vec<_>>(),
            "operator_command_hints": self.operator_command_hints.iter().map(|hint| hint.public_record()).collect::<Vec<_>>(),
            "clearance_report": self.clearance_report().public_record(),
            "roots": self.roots(),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Value::Object(fields) = &mut record {
            fields.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record_without_state_root())
    }

    pub fn roots(&self) -> BTreeMap<String, String> {
        let mut roots = BTreeMap::new();
        roots.insert("config_root".to_string(), self.config.state_root());
        roots.insert(
            "denial_intake_root".to_string(),
            self.denial_intake.state_root(),
        );
        roots.insert(
            "acceptance_criteria_root".to_string(),
            self.acceptance_criteria.state_root(),
        );
        roots.insert(
            "audit_receipt_root".to_string(),
            self.audit_receipt.state_root(),
        );
        roots.insert(
            "adversarial_scenario_root".to_string(),
            scenario_root(&self.adversarial_scenarios),
        );
        roots.insert(
            "threat_model_acceptance_root".to_string(),
            self.threat_model_acceptance.state_root(),
        );
        roots.insert(
            "privacy_review_root".to_string(),
            self.privacy_review.state_root(),
        );
        roots.insert(
            "reviewer_signoff_root".to_string(),
            signoff_root(&self.reviewer_signoffs),
        );
        roots.insert(
            "operator_signoff_root".to_string(),
            signoff_root(&self.operator_signoffs),
        );
        roots.insert(
            "operator_command_hint_root".to_string(),
            command_hint_root(&self.operator_command_hints),
        );
        roots
    }

    pub fn validate_roots(&self) -> Result<()> {
        self.denial_intake.validate()?;
        require_root("audit_scope_root", &self.audit_receipt.audit_scope_root)?;
        require_root("audit_finding_root", &self.audit_receipt.audit_finding_root)?;
        require_root(
            "mitigation_plan_root",
            &self.audit_receipt.mitigation_plan_root,
        )?;
        for scenario in &self.adversarial_scenarios {
            require_root("scenario_root", &scenario.scenario_root)?;
            require_root("defense_root", &scenario.defense_root)?;
            require_root("replay_root", &scenario.replay_root)?;
            require_root("residual_risk_root", &scenario.residual_risk_root)?;
        }
        require_root("model_root", &self.threat_model_acceptance.model_root)?;
        require_root(
            "privacy_scope_root",
            &self.privacy_review.privacy_scope_root,
        )?;
        for signoff in self
            .reviewer_signoffs
            .iter()
            .chain(self.operator_signoffs.iter())
        {
            require_root("authority_root", &signoff.authority_root)?;
            require_root("statement_root", &signoff.statement_root)?;
            require_root("observed_state_root", &signoff.observed_state_root)?;
        }
        for hint in &self.operator_command_hints {
            require_root("command_root", &hint.command_root)?;
            require_root("guard_root", &hint.guard_root)?;
            require_root("receipt_sink_root", &hint.receipt_sink_root)?;
        }
        Ok(())
    }

    pub fn clearance_report(&self) -> ClearanceReport {
        let mut cleared = BTreeSet::new();
        let mut blocked = BTreeSet::new();
        let mut missing = Vec::new();
        if self.audit_receipt.clears(&self.config) {
            cleared.insert(GateDomain::AuditReview);
        } else {
            blocked.insert(GateDomain::AuditReview);
            missing.push("accepted_audit_review_receipt".to_string());
        }
        let accepted_scenarios = self
            .adversarial_scenarios
            .iter()
            .filter(|scenario| scenario.clears())
            .count();
        if accepted_scenarios >= self.config.min_adversarial_scenarios {
            cleared.insert(GateDomain::AdversarialScenario);
        } else {
            blocked.insert(GateDomain::AdversarialScenario);
            missing.push("accepted_adversarial_scenario_receipts".to_string());
        }
        if self.threat_model_acceptance.clears(&self.config) {
            cleared.insert(GateDomain::ThreatModel);
        } else {
            blocked.insert(GateDomain::ThreatModel);
            missing.push("accepted_threat_model_acceptance".to_string());
        }
        if self.privacy_review.clears(&self.config) {
            cleared.insert(GateDomain::PrivacyReview);
        } else {
            blocked.insert(GateDomain::PrivacyReview);
            missing.push("accepted_privacy_review_receipt".to_string());
        }
        let reviewer_count = self
            .reviewer_signoffs
            .iter()
            .filter(|receipt| receipt.role.is_reviewer() && receipt.clears())
            .count();
        if reviewer_count >= self.config.min_reviewer_signoffs {
            cleared.insert(GateDomain::ReviewerSignoff);
        } else {
            blocked.insert(GateDomain::ReviewerSignoff);
            missing.push("reviewer_signoff_quorum".to_string());
        }
        let operator_count = self
            .operator_signoffs
            .iter()
            .filter(|receipt| receipt.role.is_operator() && receipt.clears())
            .count();
        if operator_count >= self.config.min_operator_signoffs
            && self.operator_command_hints.len() >= self.config.min_command_hints
        {
            cleared.insert(GateDomain::OperatorSignoff);
            cleared.insert(GateDomain::ReleaseControl);
        } else {
            blocked.insert(GateDomain::OperatorSignoff);
            blocked.insert(GateDomain::ReleaseControl);
            missing.push("operator_signoff_quorum_or_command_hints".to_string());
        }
        for domain in &self.acceptance_criteria.required_domains {
            if !cleared.contains(domain) {
                blocked.insert(*domain);
            }
        }
        let verdict = if blocked.is_empty() && missing.is_empty() {
            ClearanceVerdict::Clear
        } else {
            ClearanceVerdict::HoldFailClosed
        };
        ClearanceReport {
            verdict,
            cleared_domains: cleared.into_iter().collect(),
            blocked_domains: blocked.into_iter().collect(),
            missing_receipts: missing,
            action_hint_root: command_hint_root(&self.operator_command_hints),
            receipt_bundle_root: self.receipt_bundle_root(),
        }
    }

    pub fn fail_closed_clearance_verdict(&self) -> ClearanceVerdict {
        self.clearance_report().verdict
    }

    pub fn receipt_bundle_root(&self) -> String {
        merkle_root(&[
            self.audit_receipt.state_root(),
            scenario_root(&self.adversarial_scenarios),
            self.threat_model_acceptance.state_root(),
            self.privacy_review.state_root(),
            signoff_root(&self.reviewer_signoffs),
            signoff_root(&self.operator_signoffs),
        ])
    }

    pub fn operator_action_hints(&self) -> Vec<OperatorCommandHint> {
        self.operator_command_hints.clone()
    }

    pub fn require_clearance(&self) -> Result<String> {
        self.validate_roots()?;
        let report = self.clearance_report();
        if report.is_clear() {
            Ok(report.state_root())
        } else {
            Err(RuntimeError::ClearanceBlocked {
                reason: report.state_root(),
            })
        }
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn fail_closed_clearance_verdict() -> ClearanceVerdict {
    devnet().fail_closed_clearance_verdict()
}

pub fn wave90_denial_root() -> String {
    devnet().denial_intake.state_root()
}

pub fn live_receipt_bundle_root() -> String {
    devnet().receipt_bundle_root()
}

fn devnet_adversarial_scenarios() -> Vec<AdversarialScenarioReceipt> {
    vec![
        AdversarialScenarioReceipt::new(ScenarioFamily::MoneroReorg, 0, ReceiptStatus::Accepted),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::WatcherCensorship,
            1,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::SequencerFailure,
            2,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::PqSignerCompromise,
            3,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::LiquidityExhaustion,
            4,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::NullifierReplay,
            5,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(ScenarioFamily::ReceiptForgery, 6, ReceiptStatus::Accepted),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::MetadataCorrelation,
            7,
            ReceiptStatus::Accepted,
        ),
        AdversarialScenarioReceipt::new(ScenarioFamily::EmergencyPause, 8, ReceiptStatus::Accepted),
        AdversarialScenarioReceipt::new(
            ScenarioFamily::OperatorMistake,
            9,
            ReceiptStatus::Accepted,
        ),
    ]
}

fn scenario_root(receipts: &[AdversarialScenarioReceipt]) -> String {
    let roots = receipts
        .iter()
        .map(|receipt| receipt.state_root())
        .collect::<Vec<_>>();
    merkle_root(&roots)
}

fn signoff_root(receipts: &[SignoffReceipt]) -> String {
    let roots = receipts
        .iter()
        .map(|receipt| receipt.state_root())
        .collect::<Vec<_>>();
    merkle_root(&roots)
}

fn command_hint_root(hints: &[OperatorCommandHint]) -> String {
    let roots = hints
        .iter()
        .map(|hint| hint.state_root())
        .collect::<Vec<_>>();
    merkle_root(&roots)
}

fn seed_root(seed: &str, index: u64) -> String {
    domain_hash(
        "WAVE91-LIVE-HEAVY-GATE-ROOT-SEED",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(seed),
            HashPart::U64(index),
        ],
        32,
    )
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    domain_hash(
        "WAVE91-LIVE-HEAVY-GATE-RECORD-ROOT",
        &[HashPart::Str(domain), HashPart::Json(record)],
        32,
    )
}

fn require_root(field: &str, root: &str) -> Result<()> {
    if root.is_empty() {
        Err(RuntimeError::EmptyRoot {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}
