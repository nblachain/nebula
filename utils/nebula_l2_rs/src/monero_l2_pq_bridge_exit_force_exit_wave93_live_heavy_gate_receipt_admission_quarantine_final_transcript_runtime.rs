use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str = "wave93-live-heavy-gate-receipt-admission-quarantine-v1";
const WAVE: u64 = 93;
const PRIOR_PLAN_WAVE: u64 = 91;
const PRIOR_SLOT_WAVE: u64 = 92;
const DEFAULT_MIN_ATTACH_HEIGHT: u64 = 930_000;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, AdmissionError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdmissionError {
    LaneNotFound,
    SlotNotFound,
    EmptyReceiptRoot,
    EmptyGateEvidenceRoot,
    EmptyOperatorSignoffRoot,
    EmptyWave92SlotRoot,
    HeightBelowFloor,
    DuplicateReceiptRoot,
    QuarantineStillActive,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    Compile,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Compile => "Compile and build gate",
            Self::RuntimeReplay => "Runtime replay and rollback gate",
            Self::AuditSecurity => "Audit and security gate",
            Self::BridgeCustody => "Bridge custody gate",
            Self::WalletWatchtower => "Wallet and watchtower gate",
            Self::PqReservePrivacy => "PQ reserve and privacy gate",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdmissionStatus {
    Empty,
    Quarantined,
    ReviewReady,
    Accepted,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuarantineCause {
    MissingReceiptRoot,
    MissingWave92SlotRoot,
    MissingPlanRoot,
    MissingGateEvidenceRoot,
    MissingOperatorSignoffRoot,
    AttachHeightTooLow,
    DuplicateReceiptRoot,
    LaneBindingMissing,
    PolicyRootMissing,
}

impl QuarantineCause {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReceiptRoot => "missing_receipt_root",
            Self::MissingWave92SlotRoot => "missing_wave92_slot_root",
            Self::MissingPlanRoot => "missing_wave91_plan_root",
            Self::MissingGateEvidenceRoot => "missing_gate_evidence_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::AttachHeightTooLow => "attach_height_too_low",
            Self::DuplicateReceiptRoot => "duplicate_receipt_root",
            Self::LaneBindingMissing => "lane_binding_missing",
            Self::PolicyRootMissing => "policy_root_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdmissionRuleKind {
    RootShape,
    PriorPlanBinding,
    PriorSlotBinding,
    GateEvidenceBinding,
    OperatorSignoff,
    PrivacyRedaction,
    DuplicateRejection,
}

impl AdmissionRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootShape => "root_shape",
            Self::PriorPlanBinding => "wave91_plan_binding",
            Self::PriorSlotBinding => "wave92_slot_binding",
            Self::GateEvidenceBinding => "gate_evidence_binding",
            Self::OperatorSignoff => "operator_signoff",
            Self::PrivacyRedaction => "privacy_redaction",
            Self::DuplicateRejection => "duplicate_rejection",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub prior_plan_wave: u64,
    pub prior_slot_wave: u64,
    pub min_attach_height: u64,
    pub require_wave91_plan_root: bool,
    pub require_wave92_slot_root: bool,
    pub require_gate_evidence_root: bool,
    pub require_operator_signoff_root: bool,
    pub reject_duplicate_receipt_roots: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            prior_plan_wave: PRIOR_PLAN_WAVE,
            prior_slot_wave: PRIOR_SLOT_WAVE,
            min_attach_height: DEFAULT_MIN_ATTACH_HEIGHT,
            require_wave91_plan_root: true,
            require_wave92_slot_root: true,
            require_gate_evidence_root: true,
            require_operator_signoff_root: true,
            reject_duplicate_receipt_roots: true,
            roots_only_public_records: true,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "wave": self.wave,
            "prior_plan_wave": self.prior_plan_wave,
            "prior_slot_wave": self.prior_slot_wave,
            "min_attach_height": self.min_attach_height,
            "require_wave91_plan_root": self.require_wave91_plan_root,
            "require_wave92_slot_root": self.require_wave92_slot_root,
            "require_gate_evidence_root": self.require_gate_evidence_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "reject_duplicate_receipt_roots": self.reject_duplicate_receipt_roots,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdmissionRule {
    pub lane: LaneKind,
    pub kind: AdmissionRuleKind,
    pub ordinal: u64,
    pub rule_root: String,
    pub title_root: String,
    pub active: bool,
}

impl AdmissionRule {
    pub fn new(lane: LaneKind, kind: AdmissionRuleKind, ordinal: u64) -> Self {
        let rule_root = label_root("admission_rule", lane.as_str(), kind.as_str(), ordinal);
        let title_root = label_root(
            "admission_rule_title",
            lane.as_str(),
            kind.as_str(),
            ordinal,
        );
        Self {
            lane,
            kind,
            ordinal,
            rule_root,
            title_root,
            active: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "kind": self.kind.as_str(),
            "ordinal": self.ordinal,
            "rule_root": self.rule_root,
            "title_root": self.title_root,
            "active": self.active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission_rule", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiptCandidate {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave91_plan_root: String,
    pub wave92_slot_root: String,
    pub candidate_receipt_root: Option<String>,
    pub gate_evidence_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub observed_height: Option<u64>,
    pub intake_policy_root: String,
    pub redaction_policy_root: String,
    pub lane_binding_root: String,
    pub command_hint_root: String,
    pub quarantine_causes: Vec<QuarantineCause>,
    pub status: AdmissionStatus,
}

impl ReceiptCandidate {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        let lane_name = lane.as_str();
        let wave91_plan_root =
            label_root("wave91_plan_placeholder", lane_name, slot_label, ordinal);
        let wave92_slot_root =
            label_root("wave92_slot_placeholder", lane_name, slot_label, ordinal);
        let intake_policy_root =
            label_root("admission_intake_policy", lane_name, slot_label, ordinal);
        let redaction_policy_root =
            label_root("admission_redaction_policy", lane_name, slot_label, ordinal);
        let lane_binding_root =
            label_root("admission_lane_binding", lane_name, slot_label, ordinal);
        let command_hint_root =
            label_root("admission_operator_hint", lane_name, slot_label, ordinal);
        let quarantine_causes = initial_causes(config);
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave91_plan_root,
            wave92_slot_root,
            candidate_receipt_root: None,
            gate_evidence_root: None,
            operator_signoff_root: None,
            observed_height: None,
            intake_policy_root,
            redaction_policy_root,
            lane_binding_root,
            command_hint_root,
            quarantine_causes,
            status: AdmissionStatus::Quarantined,
        }
    }

    pub fn with_candidate_roots(
        &self,
        receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        observed_height: u64,
        config: &Config,
        duplicate: bool,
    ) -> Result<Self> {
        if receipt_root.is_empty() {
            return Err(AdmissionError::EmptyReceiptRoot);
        }
        if gate_evidence_root.is_empty() {
            return Err(AdmissionError::EmptyGateEvidenceRoot);
        }
        if operator_signoff_root.is_empty() {
            return Err(AdmissionError::EmptyOperatorSignoffRoot);
        }
        if self.wave92_slot_root.is_empty() {
            return Err(AdmissionError::EmptyWave92SlotRoot);
        }
        if observed_height < config.min_attach_height {
            return Err(AdmissionError::HeightBelowFloor);
        }
        if duplicate {
            return Err(AdmissionError::DuplicateReceiptRoot);
        }
        let mut next = self.clone();
        next.candidate_receipt_root = Some(receipt_root.to_string());
        next.gate_evidence_root = Some(gate_evidence_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.observed_height = Some(observed_height);
        next.quarantine_causes = next.active_quarantine_causes(config, duplicate);
        next.status = if next.quarantine_causes.is_empty() {
            AdmissionStatus::ReviewReady
        } else {
            AdmissionStatus::Quarantined
        };
        Ok(next)
    }

    pub fn accept_if_clear(&self) -> Result<Self> {
        if !self.quarantine_causes.is_empty() {
            return Err(AdmissionError::QuarantineStillActive);
        }
        let mut next = self.clone();
        next.status = AdmissionStatus::Accepted;
        Ok(next)
    }

    fn active_quarantine_causes(&self, config: &Config, duplicate: bool) -> Vec<QuarantineCause> {
        let mut causes = Vec::new();
        if self.candidate_receipt_root.is_none() {
            causes.push(QuarantineCause::MissingReceiptRoot);
        }
        if config.require_wave91_plan_root && self.wave91_plan_root.is_empty() {
            causes.push(QuarantineCause::MissingPlanRoot);
        }
        if config.require_wave92_slot_root && self.wave92_slot_root.is_empty() {
            causes.push(QuarantineCause::MissingWave92SlotRoot);
        }
        if config.require_gate_evidence_root && self.gate_evidence_root.is_none() {
            causes.push(QuarantineCause::MissingGateEvidenceRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            causes.push(QuarantineCause::MissingOperatorSignoffRoot);
        }
        if self.lane_binding_root.is_empty() {
            causes.push(QuarantineCause::LaneBindingMissing);
        }
        if self.intake_policy_root.is_empty() || self.redaction_policy_root.is_empty() {
            causes.push(QuarantineCause::PolicyRootMissing);
        }
        match self.observed_height {
            Some(height) if height >= config.min_attach_height => {}
            _ => causes.push(QuarantineCause::AttachHeightTooLow),
        }
        if duplicate && config.reject_duplicate_receipt_roots {
            causes.push(QuarantineCause::DuplicateReceiptRoot);
        }
        causes
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave91_plan_root": self.wave91_plan_root,
            "wave92_slot_root": self.wave92_slot_root,
            "candidate_receipt_root": self.candidate_receipt_root,
            "gate_evidence_root": self.gate_evidence_root,
            "operator_signoff_root": self.operator_signoff_root,
            "observed_height": self.observed_height,
            "intake_policy_root": self.intake_policy_root,
            "redaction_policy_root": self.redaction_policy_root,
            "lane_binding_root": self.lane_binding_root,
            "command_hint_root": self.command_hint_root,
            "quarantine_causes": self.quarantine_causes.iter().map(|cause| cause.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("receipt_candidate", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneAdmissionRegistry {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub source_wave91_plan_root: String,
    pub source_wave92_registry_root: String,
    pub rules: Vec<AdmissionRule>,
    pub candidates: Vec<ReceiptCandidate>,
    pub quarantine_root: String,
    pub accepted_root: String,
    pub command_root: String,
    pub lane_state: AdmissionStatus,
}

impl LaneAdmissionRegistry {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let lane_name = lane.as_str();
        let rules = vec![
            AdmissionRule::new(lane, AdmissionRuleKind::RootShape, 0),
            AdmissionRule::new(lane, AdmissionRuleKind::PriorPlanBinding, 1),
            AdmissionRule::new(lane, AdmissionRuleKind::PriorSlotBinding, 2),
            AdmissionRule::new(lane, AdmissionRuleKind::GateEvidenceBinding, 3),
            AdmissionRule::new(lane, AdmissionRuleKind::OperatorSignoff, 4),
            AdmissionRule::new(lane, AdmissionRuleKind::PrivacyRedaction, 5),
            AdmissionRule::new(lane, AdmissionRuleKind::DuplicateRejection, 6),
        ];
        let candidates = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| ReceiptCandidate::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        let lane_title_root = label_root("lane_title", lane_name, lane.title(), WAVE);
        let source_wave91_plan_root =
            label_root("wave91_source_plan", lane_name, "execution_plan", WAVE);
        let source_wave92_registry_root = label_root(
            "wave92_source_slot_registry",
            lane_name,
            "slot_registry",
            WAVE,
        );
        let quarantine_root = quarantine_root(&candidates);
        let accepted_root = accepted_root(&candidates);
        let command_root = root_from_strings(
            "lane_command_hints",
            candidates
                .iter()
                .map(|candidate| candidate.command_hint_root.clone()),
        );
        let lane_state = if candidates
            .iter()
            .all(|candidate| candidate.status == AdmissionStatus::Accepted)
        {
            AdmissionStatus::Accepted
        } else {
            AdmissionStatus::Quarantined
        };
        Self {
            lane,
            lane_title_root,
            source_wave91_plan_root,
            source_wave92_registry_root,
            rules,
            candidates,
            quarantine_root,
            accepted_root,
            command_root,
            lane_state,
        }
    }

    pub fn attach_candidate(
        &self,
        slot_label: &str,
        receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        observed_height: u64,
        config: &Config,
        known_receipt_roots: &[String],
    ) -> Result<Self> {
        let mut found = false;
        let duplicate = known_receipt_roots
            .iter()
            .any(|known| known.as_str() == receipt_root);
        let mut candidates = Vec::with_capacity(self.candidates.len());
        for candidate in &self.candidates {
            if candidate.slot_label == slot_label {
                let next = candidate.with_candidate_roots(
                    receipt_root,
                    gate_evidence_root,
                    operator_signoff_root,
                    observed_height,
                    config,
                    duplicate,
                )?;
                candidates.push(next);
                found = true;
            } else {
                candidates.push(candidate.clone());
            }
        }
        if !found {
            return Err(AdmissionError::SlotNotFound);
        }
        Ok(Self::from_parts(
            self.lane,
            self.lane_title_root.clone(),
            self.source_wave91_plan_root.clone(),
            self.source_wave92_registry_root.clone(),
            self.rules.clone(),
            candidates,
        ))
    }

    pub fn from_parts(
        lane: LaneKind,
        lane_title_root: String,
        source_wave91_plan_root: String,
        source_wave92_registry_root: String,
        rules: Vec<AdmissionRule>,
        candidates: Vec<ReceiptCandidate>,
    ) -> Self {
        let quarantine_root = quarantine_root(&candidates);
        let accepted_root = accepted_root(&candidates);
        let command_root = root_from_strings(
            "lane_command_hints",
            candidates
                .iter()
                .map(|candidate| candidate.command_hint_root.clone()),
        );
        let lane_state = if candidates
            .iter()
            .all(|candidate| candidate.status == AdmissionStatus::Accepted)
        {
            AdmissionStatus::Accepted
        } else {
            AdmissionStatus::Quarantined
        };
        Self {
            lane,
            lane_title_root,
            source_wave91_plan_root,
            source_wave92_registry_root,
            rules,
            candidates,
            quarantine_root,
            accepted_root,
            command_root,
            lane_state,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "source_wave91_plan_root": self.source_wave91_plan_root,
            "source_wave92_registry_root": self.source_wave92_registry_root,
            "rule_roots": self.rules.iter().map(AdmissionRule::state_root).collect::<Vec<_>>(),
            "candidate_roots": self.candidates.iter().map(ReceiptCandidate::state_root).collect::<Vec<_>>(),
            "quarantine_root": self.quarantine_root,
            "accepted_root": self.accepted_root,
            "command_root": self.command_root,
            "candidate_count": self.candidates.len(),
            "quarantined_count": self.quarantined_count(),
            "accepted_count": self.accepted_count(),
            "lane_state": format!("{:?}", self.lane_state),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_admission_registry", &self.public_record())
    }

    pub fn quarantined_count(&self) -> usize {
        self.candidates
            .iter()
            .filter(|candidate| !candidate.quarantine_causes.is_empty())
            .count()
    }

    pub fn accepted_count(&self) -> usize {
        self.candidates
            .iter()
            .filter(|candidate| candidate.status == AdmissionStatus::Accepted)
            .count()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptSummary {
    pub registry_root: String,
    pub quarantine_root: String,
    pub accepted_root: String,
    pub command_root: String,
    pub readiness_root: String,
    pub lane_count: usize,
    pub candidate_count: usize,
    pub quarantined_count: usize,
    pub accepted_count: usize,
    pub all_lanes_clear: bool,
    pub production_release_denied: bool,
}

impl TranscriptSummary {
    pub fn from_lanes(config: &Config, lanes: &[LaneAdmissionRegistry]) -> Self {
        let registry_root = root_from_strings(
            "wave93_lane_registry_root",
            lanes.iter().map(LaneAdmissionRegistry::state_root),
        );
        let quarantine_root = root_from_strings(
            "wave93_quarantine_root",
            lanes.iter().map(|lane| lane.quarantine_root.clone()),
        );
        let accepted_root = root_from_strings(
            "wave93_accepted_root",
            lanes.iter().map(|lane| lane.accepted_root.clone()),
        );
        let command_root = root_from_strings(
            "wave93_command_root",
            lanes.iter().map(|lane| lane.command_root.clone()),
        );
        let candidate_count = lanes
            .iter()
            .map(|lane| lane.candidates.len())
            .sum::<usize>();
        let quarantined_count = lanes
            .iter()
            .map(LaneAdmissionRegistry::quarantined_count)
            .sum::<usize>();
        let accepted_count = lanes
            .iter()
            .map(LaneAdmissionRegistry::accepted_count)
            .sum::<usize>();
        let all_lanes_clear = candidate_count > 0 && accepted_count == candidate_count;
        let production_release_denied = !all_lanes_clear;
        let readiness_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "candidate_count": candidate_count,
            "quarantined_count": quarantined_count,
            "accepted_count": accepted_count,
            "all_lanes_clear": all_lanes_clear,
            "production_release_denied": production_release_denied,
        });
        let readiness_root = record_root("wave93_readiness", &readiness_record);
        Self {
            registry_root,
            quarantine_root,
            accepted_root,
            command_root,
            readiness_root,
            lane_count: lanes.len(),
            candidate_count,
            quarantined_count,
            accepted_count,
            all_lanes_clear,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "registry_root": self.registry_root,
            "quarantine_root": self.quarantine_root,
            "accepted_root": self.accepted_root,
            "command_root": self.command_root,
            "readiness_root": self.readiness_root,
            "lane_count": self.lane_count,
            "candidate_count": self.candidate_count,
            "quarantined_count": self.quarantined_count,
            "accepted_count": self.accepted_count,
            "all_lanes_clear": self.all_lanes_clear,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("transcript_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave91_execution_plan_transcript_root: String,
    pub wave92_receipt_slot_registry_root: String,
    pub lane_registries: Vec<LaneAdmissionRegistry>,
    pub summary: TranscriptSummary,
}

impl State {
    pub fn new(config: Config, lane_registries: Vec<LaneAdmissionRegistry>) -> Self {
        let wave91_execution_plan_transcript_root = label_root(
            "wave91_execution_plan_transcript",
            "all_lanes",
            "source",
            config.wave,
        );
        let wave92_receipt_slot_registry_root = label_root(
            "wave92_receipt_slot_registry",
            "all_lanes",
            "source",
            config.wave,
        );
        let summary = TranscriptSummary::from_lanes(&config, &lane_registries);
        Self {
            config,
            wave91_execution_plan_transcript_root,
            wave92_receipt_slot_registry_root,
            lane_registries,
            summary,
        }
    }

    pub fn attach_candidate(
        &self,
        lane: LaneKind,
        slot_label: &str,
        receipt_root: &str,
        gate_evidence_root: &str,
        operator_signoff_root: &str,
        observed_height: u64,
    ) -> Result<Self> {
        let known_receipt_roots = self.known_receipt_roots();
        let mut found = false;
        let mut lanes = Vec::with_capacity(self.lane_registries.len());
        for registry in &self.lane_registries {
            if registry.lane == lane {
                lanes.push(registry.attach_candidate(
                    slot_label,
                    receipt_root,
                    gate_evidence_root,
                    operator_signoff_root,
                    observed_height,
                    &self.config,
                    &known_receipt_roots,
                )?);
                found = true;
            } else {
                lanes.push(registry.clone());
            }
        }
        if !found {
            return Err(AdmissionError::LaneNotFound);
        }
        Ok(Self::new(self.config.clone(), lanes))
    }

    pub fn known_receipt_roots(&self) -> Vec<String> {
        self.lane_registries
            .iter()
            .flat_map(|lane| {
                lane.candidates
                    .iter()
                    .filter_map(|candidate| candidate.candidate_receipt_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave91_execution_plan_transcript_root": self.wave91_execution_plan_transcript_root,
            "wave92_receipt_slot_registry_root": self.wave92_receipt_slot_registry_root,
            "lane_registry_roots": self.lane_registries.iter().map(LaneAdmissionRegistry::state_root).collect::<Vec<_>>(),
            "summary": self.summary.public_record(),
            "roots_only": self.config.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::default();
    let lanes = vec![
        LaneAdmissionRegistry::new(
            LaneKind::Compile,
            &[
                "cargo_check",
                "cargo_test",
                "clippy",
                "rustfmt",
                "rustc",
                "build_metadata",
                "operator_signoff",
            ],
            &config,
        ),
        LaneAdmissionRegistry::new(
            LaneKind::RuntimeReplay,
            &[
                "replay_run",
                "rollback_drill",
                "adversarial_replay",
                "stale_archive_replacement",
                "live_execution_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneAdmissionRegistry::new(
            LaneKind::AuditSecurity,
            &[
                "audit_review",
                "adversarial_scenario",
                "threat_model",
                "privacy_review",
                "reviewer_signoff",
                "operator_signoff",
            ],
            &config,
        ),
        LaneAdmissionRegistry::new(
            LaneKind::BridgeCustody,
            &[
                "monero_watcher_quorum",
                "withdrawal_release",
                "reserve_coverage",
                "signer_quorum",
                "challenge_hold_review",
                "custody_operator_signoff",
            ],
            &config,
        ),
        LaneAdmissionRegistry::new(
            LaneKind::WalletWatchtower,
            &[
                "wallet_escape_dry_run",
                "watchtower_quorum",
                "user_runbook_replay",
                "redacted_recovery_proof",
                "wallet_visible_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneAdmissionRegistry::new(
            LaneKind::PqReservePrivacy,
            &[
                "ml_dsa_slh_dsa_authority_epoch",
                "pq_quorum",
                "reserve_coverage",
                "privacy_linkage",
                "metadata_redaction",
                "nullifier_separation",
                "operator_signoff",
            ],
            &config,
        ),
    ];
    State::new(config, lanes)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_causes(config: &Config) -> Vec<QuarantineCause> {
    let mut causes = vec![QuarantineCause::MissingReceiptRoot];
    if config.require_gate_evidence_root {
        causes.push(QuarantineCause::MissingGateEvidenceRoot);
    }
    if config.require_operator_signoff_root {
        causes.push(QuarantineCause::MissingOperatorSignoffRoot);
    }
    causes.push(QuarantineCause::AttachHeightTooLow);
    causes
}

fn quarantine_root(candidates: &[ReceiptCandidate]) -> String {
    let leaves = candidates
        .iter()
        .flat_map(|candidate| {
            candidate.quarantine_causes.iter().map(move |cause| {
                json!({
                    "lane": candidate.lane.as_str(),
                    "slot_label": candidate.slot_label,
                    "cause": cause.as_str(),
                    "candidate_root": candidate.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave93_quarantine_causes", &leaves)
}

fn accepted_root(candidates: &[ReceiptCandidate]) -> String {
    let leaves = candidates
        .iter()
        .filter(|candidate| candidate.status == AdmissionStatus::Accepted)
        .map(ReceiptCandidate::public_record)
        .collect::<Vec<_>>();
    merkle_root("wave93_accepted_candidates", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave93-live-heavy-gate-receipt-admission-quarantine-record",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn label_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "wave93-live-heavy-gate-receipt-admission-quarantine-label",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}
