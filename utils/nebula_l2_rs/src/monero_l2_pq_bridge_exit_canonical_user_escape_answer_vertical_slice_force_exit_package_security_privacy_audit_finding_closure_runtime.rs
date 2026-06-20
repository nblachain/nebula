use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageSecurityPrivacyAuditFindingClosureRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_SECURITY_PRIVACY_AUDIT_FINDING_CLOSURE_RUNTIME_PROTOCOL_VERSION:
    &str = "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-security-privacy-audit-finding-closure-runtime/v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_SECURITY_PRIVACY_AUDIT_FINDING_CLOSURE_RUNTIME_PROTOCOL_VERSION;
pub const AUDIT_CLOSURE_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-security-privacy-audit-finding-closure-suite-v1";
pub const DEFAULT_CLOSURE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-security-privacy-audit-finding-closure-devnet-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-forced-exit-vertical-slice-devnet-v1";
pub const DEFAULT_FORCE_EXIT_PACKAGE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-devnet-v1";
pub const DEFAULT_MIN_REVIEWER_QUORUM_BPS: u64 = 8_000;
pub const DEFAULT_MAX_PRIVACY_LEAK_BUDGET_UNITS: u64 = 0;
pub const DEFAULT_MAX_RESIDUAL_RISK_BPS: u64 = 100;
pub const DEFAULT_MAX_OPEN_FINDINGS: u64 = 0;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-audit-closure";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditFindingLane {
    ReviewerCommitment,
    PrivacyLeakBudget,
    PqMisuse,
    BridgeCustody,
    WalletDeanonymization,
    RemediationReceipt,
    ResidualRisk,
}

impl AuditFindingLane {
    pub fn all() -> [Self; 7] {
        [
            Self::ReviewerCommitment,
            Self::PrivacyLeakBudget,
            Self::PqMisuse,
            Self::BridgeCustody,
            Self::WalletDeanonymization,
            Self::RemediationReceipt,
            Self::ResidualRisk,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReviewerCommitment => "reviewer_commitment",
            Self::PrivacyLeakBudget => "privacy_leak_budget",
            Self::PqMisuse => "pq_misuse",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletDeanonymization => "wallet_deanonymization",
            Self::RemediationReceipt => "remediation_receipt",
            Self::ResidualRisk => "residual_risk",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ReviewerCommitment => "Reviewer commitment closure",
            Self::PrivacyLeakBudget => "Privacy leak budget closure",
            Self::PqMisuse => "Post-quantum misuse closure",
            Self::BridgeCustody => "Bridge custody closure",
            Self::WalletDeanonymization => "Wallet deanonymization closure",
            Self::RemediationReceipt => "Remediation receipt closure",
            Self::ResidualRisk => "Residual risk closure",
        }
    }

    pub fn weight_bps(self) -> u64 {
        match self {
            Self::ReviewerCommitment => 1_300,
            Self::PrivacyLeakBudget => 1_700,
            Self::PqMisuse => 1_700,
            Self::BridgeCustody => 1_500,
            Self::WalletDeanonymization => 1_500,
            Self::RemediationReceipt => 1_200,
            Self::ResidualRisk => 1_100,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn release_blocking(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingClosureStatus {
    Closed,
    ClosedWithResidualRisk,
    DeferredToLiveReplacement,
    ReleaseBlockingOpen,
    Rejected,
}

impl FindingClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::ClosedWithResidualRisk => "closed_with_residual_risk",
            Self::DeferredToLiveReplacement => "deferred_to_live_replacement",
            Self::ReleaseBlockingOpen => "release_blocking_open",
            Self::Rejected => "rejected",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Closed | Self::ClosedWithResidualRisk)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementVerdict {
    NotDeferred,
    DeferredToLiveReplacement,
    LiveReplacementAccepted,
    LiveReplacementRejected,
}

impl ReplacementVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotDeferred => "not_deferred",
            Self::DeferredToLiveReplacement => "deferred_to_live_replacement",
            Self::LiveReplacementAccepted => "live_replacement_accepted",
            Self::LiveReplacementRejected => "live_replacement_rejected",
        }
    }

    pub fn release_blocking(self) -> bool {
        matches!(
            self,
            Self::DeferredToLiveReplacement | Self::LiveReplacementRejected
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseHoldStatus {
    Clear,
    HeldForAuditClosure,
    HeldForLiveReplacement,
    Rejected,
}

impl ReleaseHoldStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::HeldForAuditClosure => "held_for_audit_closure",
            Self::HeldForLiveReplacement => "held_for_live_replacement",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub vertical_slice_id: String,
    pub force_exit_package_id: String,
    pub closure_id: String,
    pub audit_reference_height: u64,
    pub package_build_height: u64,
    pub min_reviewer_quorum_bps: u64,
    pub max_privacy_leak_budget_units: u64,
    pub max_residual_risk_bps: u64,
    pub max_open_findings: u64,
    pub live_replacement_allowed: bool,
    pub release_requires_zero_critical_findings: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            force_exit_package_id: DEFAULT_FORCE_EXIT_PACKAGE_ID.to_string(),
            closure_id: DEFAULT_CLOSURE_ID.to_string(),
            audit_reference_height: 1_058_940,
            package_build_height: 10_620,
            min_reviewer_quorum_bps: DEFAULT_MIN_REVIEWER_QUORUM_BPS,
            max_privacy_leak_budget_units: DEFAULT_MAX_PRIVACY_LEAK_BUDGET_UNITS,
            max_residual_risk_bps: DEFAULT_MAX_RESIDUAL_RISK_BPS,
            max_open_findings: DEFAULT_MAX_OPEN_FINDINGS,
            live_replacement_allowed: false,
            release_requires_zero_critical_findings: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "vertical_slice_id": self.vertical_slice_id,
            "force_exit_package_id": self.force_exit_package_id,
            "closure_id": self.closure_id,
            "audit_reference_height": self.audit_reference_height,
            "package_build_height": self.package_build_height,
            "min_reviewer_quorum_bps": self.min_reviewer_quorum_bps,
            "max_privacy_leak_budget_units": self.max_privacy_leak_budget_units,
            "max_residual_risk_bps": self.max_residual_risk_bps,
            "max_open_findings": self.max_open_findings,
            "live_replacement_allowed": bool_label(self.live_replacement_allowed),
            "release_requires_zero_critical_findings": bool_label(
                self.release_requires_zero_critical_findings
            ),
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
pub struct AuditFindingClosure {
    pub lane: AuditFindingLane,
    pub finding_id: String,
    pub severity: FindingSeverity,
    pub status: FindingClosureStatus,
    pub reviewer_commitment_root: String,
    pub privacy_leak_budget_root: String,
    pub pq_misuse_finding_root: String,
    pub bridge_custody_finding_root: String,
    pub wallet_deanonymization_finding_root: String,
    pub remediation_receipt_root: String,
    pub residual_risk_root: String,
    pub reviewer_quorum_bps: u64,
    pub privacy_leak_budget_units: u64,
    pub residual_risk_bps: u64,
    pub live_replacement_required: bool,
    pub note: String,
}

impl AuditFindingClosure {
    pub fn devnet(lane: AuditFindingLane, config: &Config) -> Self {
        let severity = match lane {
            AuditFindingLane::ReviewerCommitment
            | AuditFindingLane::RemediationReceipt
            | AuditFindingLane::ResidualRisk => FindingSeverity::Low,
            AuditFindingLane::PrivacyLeakBudget
            | AuditFindingLane::PqMisuse
            | AuditFindingLane::BridgeCustody
            | AuditFindingLane::WalletDeanonymization => FindingSeverity::High,
        };
        let live_replacement_required = matches!(
            lane,
            AuditFindingLane::PqMisuse | AuditFindingLane::WalletDeanonymization
        );
        let status = if live_replacement_required {
            FindingClosureStatus::DeferredToLiveReplacement
        } else if severity.release_blocking() {
            FindingClosureStatus::ReleaseBlockingOpen
        } else {
            FindingClosureStatus::ClosedWithResidualRisk
        };

        Self {
            lane,
            finding_id: stable_id("audit_finding", lane.as_str()),
            severity,
            status,
            reviewer_commitment_root: lane_root("reviewer_commitment", lane, config),
            privacy_leak_budget_root: lane_root("privacy_leak_budget", lane, config),
            pq_misuse_finding_root: lane_root("pq_misuse_finding", lane, config),
            bridge_custody_finding_root: lane_root("bridge_custody_finding", lane, config),
            wallet_deanonymization_finding_root: lane_root(
                "wallet_deanonymization_finding",
                lane,
                config,
            ),
            remediation_receipt_root: lane_root("remediation_receipt", lane, config),
            residual_risk_root: lane_root("residual_risk", lane, config),
            reviewer_quorum_bps: config.min_reviewer_quorum_bps,
            privacy_leak_budget_units: if lane == AuditFindingLane::PrivacyLeakBudget {
                1
            } else {
                0
            },
            residual_risk_bps: if severity.release_blocking() { 250 } else { 50 },
            live_replacement_required,
            note: finding_note(lane).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane.as_str(),
            "lane_label": self.lane.label(),
            "finding_id": self.finding_id,
            "severity": self.severity.as_str(),
            "status": self.status.as_str(),
            "reviewer_commitment_root": self.reviewer_commitment_root,
            "privacy_leak_budget_root": self.privacy_leak_budget_root,
            "pq_misuse_finding_root": self.pq_misuse_finding_root,
            "bridge_custody_finding_root": self.bridge_custody_finding_root,
            "wallet_deanonymization_finding_root": self.wallet_deanonymization_finding_root,
            "remediation_receipt_root": self.remediation_receipt_root,
            "residual_risk_root": self.residual_risk_root,
            "reviewer_quorum_bps": self.reviewer_quorum_bps,
            "privacy_leak_budget_units": self.privacy_leak_budget_units,
            "residual_risk_bps": self.residual_risk_bps,
            "live_replacement_required": bool_label(self.live_replacement_required),
            "note": self.note,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("audit_finding_closure", &self.public_record())
    }

    pub fn is_release_blocking(&self, config: &Config) -> bool {
        self.status.blocks_release()
            || self.reviewer_quorum_bps < config.min_reviewer_quorum_bps
            || self.privacy_leak_budget_units > config.max_privacy_leak_budget_units
            || self.residual_risk_bps > config.max_residual_risk_bps
            || (config.release_requires_zero_critical_findings
                && self.severity == FindingSeverity::Critical)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub reviewer_commitment_root: String,
    pub privacy_leak_budget_root: String,
    pub pq_misuse_findings_root: String,
    pub bridge_custody_findings_root: String,
    pub wallet_deanonymization_findings_root: String,
    pub remediation_receipt_root: String,
    pub residual_risk_root: String,
    pub finding_closure_root: String,
    pub force_exit_package_security_privacy_audit_root: String,
}

impl Roots {
    pub fn from_findings(findings: &[AuditFindingClosure]) -> Self {
        let reviewer_commitment_root = root_from_records(
            "reviewer_commitment_roots",
            findings
                .iter()
                .map(|finding| finding.reviewer_commitment_root.clone())
                .collect(),
        );
        let privacy_leak_budget_root = root_from_records(
            "privacy_leak_budget_roots",
            findings
                .iter()
                .map(|finding| finding.privacy_leak_budget_root.clone())
                .collect(),
        );
        let pq_misuse_findings_root = root_from_records(
            "pq_misuse_findings",
            findings
                .iter()
                .map(|finding| finding.pq_misuse_finding_root.clone())
                .collect(),
        );
        let bridge_custody_findings_root = root_from_records(
            "bridge_custody_findings",
            findings
                .iter()
                .map(|finding| finding.bridge_custody_finding_root.clone())
                .collect(),
        );
        let wallet_deanonymization_findings_root = root_from_records(
            "wallet_deanonymization_findings",
            findings
                .iter()
                .map(|finding| finding.wallet_deanonymization_finding_root.clone())
                .collect(),
        );
        let remediation_receipt_root = root_from_records(
            "remediation_receipt_roots",
            findings
                .iter()
                .map(|finding| finding.remediation_receipt_root.clone())
                .collect(),
        );
        let residual_risk_root = root_from_records(
            "residual_risk_roots",
            findings
                .iter()
                .map(|finding| finding.residual_risk_root.clone())
                .collect(),
        );
        let finding_closure_root = merkle_root(
            findings
                .iter()
                .map(AuditFindingClosure::state_root)
                .collect::<Vec<_>>(),
        );
        let force_exit_package_security_privacy_audit_root = record_root(
            "force_exit_package_security_privacy_audit_root",
            &json!({
                "reviewer_commitment_root": reviewer_commitment_root,
                "privacy_leak_budget_root": privacy_leak_budget_root,
                "pq_misuse_findings_root": pq_misuse_findings_root,
                "bridge_custody_findings_root": bridge_custody_findings_root,
                "wallet_deanonymization_findings_root": wallet_deanonymization_findings_root,
                "remediation_receipt_root": remediation_receipt_root,
                "residual_risk_root": residual_risk_root,
                "finding_closure_root": finding_closure_root,
            }),
        );

        Self {
            reviewer_commitment_root,
            privacy_leak_budget_root,
            pq_misuse_findings_root,
            bridge_custody_findings_root,
            wallet_deanonymization_findings_root,
            remediation_receipt_root,
            residual_risk_root,
            finding_closure_root,
            force_exit_package_security_privacy_audit_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_commitment_root": self.reviewer_commitment_root,
            "privacy_leak_budget_root": self.privacy_leak_budget_root,
            "pq_misuse_findings_root": self.pq_misuse_findings_root,
            "bridge_custody_findings_root": self.bridge_custody_findings_root,
            "wallet_deanonymization_findings_root": self.wallet_deanonymization_findings_root,
            "remediation_receipt_root": self.remediation_receipt_root,
            "residual_risk_root": self.residual_risk_root,
            "finding_closure_root": self.finding_closure_root,
            "force_exit_package_security_privacy_audit_root":
                self.force_exit_package_security_privacy_audit_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub finding_count: u64,
    pub closed_count: u64,
    pub closed_with_residual_risk_count: u64,
    pub deferred_to_live_replacement_count: u64,
    pub release_blocking_open_count: u64,
    pub rejected_count: u64,
    pub critical_count: u64,
    pub privacy_leak_budget_units: u64,
    pub max_residual_risk_bps: u64,
    pub weighted_closure_bps: u64,
}

impl Counters {
    pub fn from_findings(findings: &[AuditFindingClosure], config: &Config) -> Self {
        let finding_count = findings.len() as u64;
        let closed_count = findings
            .iter()
            .filter(|finding| finding.status == FindingClosureStatus::Closed)
            .count() as u64;
        let closed_with_residual_risk_count = findings
            .iter()
            .filter(|finding| finding.status == FindingClosureStatus::ClosedWithResidualRisk)
            .count() as u64;
        let deferred_to_live_replacement_count = findings
            .iter()
            .filter(|finding| {
                finding.status == FindingClosureStatus::DeferredToLiveReplacement
                    || finding.live_replacement_required
            })
            .count() as u64;
        let release_blocking_open_count = findings
            .iter()
            .filter(|finding| finding.is_release_blocking(config))
            .count() as u64;
        let rejected_count = findings
            .iter()
            .filter(|finding| finding.status == FindingClosureStatus::Rejected)
            .count() as u64;
        let critical_count = findings
            .iter()
            .filter(|finding| finding.severity == FindingSeverity::Critical)
            .count() as u64;
        let privacy_leak_budget_units = findings
            .iter()
            .map(|finding| finding.privacy_leak_budget_units)
            .sum();
        let max_residual_risk_bps = findings.iter().fold(0, |max_seen, finding| {
            max_seen.max(finding.residual_risk_bps)
        });
        let possible_weight: u64 = findings
            .iter()
            .map(|finding| finding.lane.weight_bps())
            .sum();
        let closed_weight: u64 = findings
            .iter()
            .filter(|finding| !finding.is_release_blocking(config))
            .map(|finding| finding.lane.weight_bps())
            .sum();
        let weighted_closure_bps = if possible_weight == 0 {
            0
        } else {
            closed_weight.saturating_mul(10_000) / possible_weight
        };

        Self {
            finding_count,
            closed_count,
            closed_with_residual_risk_count,
            deferred_to_live_replacement_count,
            release_blocking_open_count,
            rejected_count,
            critical_count,
            privacy_leak_budget_units,
            max_residual_risk_bps,
            weighted_closure_bps,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "finding_count": self.finding_count,
            "closed_count": self.closed_count,
            "closed_with_residual_risk_count": self.closed_with_residual_risk_count,
            "deferred_to_live_replacement_count": self.deferred_to_live_replacement_count,
            "release_blocking_open_count": self.release_blocking_open_count,
            "rejected_count": self.rejected_count,
            "critical_count": self.critical_count,
            "privacy_leak_budget_units": self.privacy_leak_budget_units,
            "max_residual_risk_bps": self.max_residual_risk_bps,
            "weighted_closure_bps": self.weighted_closure_bps,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub findings: Vec<AuditFindingClosure>,
    pub roots: Roots,
    pub counters: Counters,
    pub deferred_to_live_replacement_verdict: ReplacementVerdict,
    pub release_hold_status: ReleaseHoldStatus,
    pub hold_reasons: BTreeMap<String, String>,
}

impl State {
    pub fn new(
        config: Config,
        findings: Vec<AuditFindingClosure>,
    ) -> MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageSecurityPrivacyAuditFindingClosureRuntimeResult<Self>
    {
        if findings.is_empty() {
            return Err("audit finding closure state requires at least one finding".to_string());
        }

        let roots = Roots::from_findings(&findings);
        let counters = Counters::from_findings(&findings, &config);
        let deferred_to_live_replacement_verdict = replacement_verdict(&findings, &config);
        let release_hold_status =
            release_hold_status(&counters, deferred_to_live_replacement_verdict, &config);
        let hold_reasons = hold_reasons(
            &findings,
            &counters,
            deferred_to_live_replacement_verdict,
            release_hold_status,
            &config,
        );

        Ok(Self {
            config,
            findings,
            roots,
            counters,
            deferred_to_live_replacement_verdict,
            release_hold_status,
            hold_reasons,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let findings = AuditFindingLane::all()
            .into_iter()
            .map(|lane| AuditFindingClosure::devnet(lane, &config))
            .collect();

        match Self::new(config, findings) {
            Ok(state) => state,
            Err(_) => {
                let fallback_config = Config::devnet();
                let fallback_findings = AuditFindingLane::all()
                    .into_iter()
                    .map(|lane| AuditFindingClosure::devnet(lane, &fallback_config))
                    .collect::<Vec<_>>();
                let fallback_roots = Roots::from_findings(&fallback_findings);
                let fallback_counters =
                    Counters::from_findings(&fallback_findings, &fallback_config);
                let fallback_replacement =
                    replacement_verdict(&fallback_findings, &fallback_config);
                let fallback_status =
                    release_hold_status(&fallback_counters, fallback_replacement, &fallback_config);
                let fallback_hold_reasons = hold_reasons(
                    &fallback_findings,
                    &fallback_counters,
                    fallback_replacement,
                    fallback_status,
                    &fallback_config,
                );

                Self {
                    config: fallback_config,
                    findings: fallback_findings,
                    roots: fallback_roots,
                    counters: fallback_counters,
                    deferred_to_live_replacement_verdict: fallback_replacement,
                    release_hold_status: fallback_status,
                    hold_reasons: fallback_hold_reasons,
                }
            }
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "audit_closure_suite": AUDIT_CLOSURE_SUITE,
            "config": self.config.public_record(),
            "findings": self
                .findings
                .iter()
                .map(AuditFindingClosure::public_record)
                .collect::<Vec<_>>(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "deferred_to_live_replacement_verdict":
                self.deferred_to_live_replacement_verdict.as_str(),
            "release_hold_status": self.release_hold_status.as_str(),
            "hold_reasons": self.hold_reasons,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn replacement_verdict(findings: &[AuditFindingClosure], config: &Config) -> ReplacementVerdict {
    let replacement_required = findings
        .iter()
        .any(|finding| finding.live_replacement_required);
    let rejected = findings
        .iter()
        .any(|finding| finding.status == FindingClosureStatus::Rejected);

    if rejected {
        ReplacementVerdict::LiveReplacementRejected
    } else if replacement_required && config.live_replacement_allowed {
        ReplacementVerdict::LiveReplacementAccepted
    } else if replacement_required {
        ReplacementVerdict::DeferredToLiveReplacement
    } else {
        ReplacementVerdict::NotDeferred
    }
}

fn release_hold_status(
    counters: &Counters,
    replacement: ReplacementVerdict,
    config: &Config,
) -> ReleaseHoldStatus {
    if counters.rejected_count > 0 || replacement == ReplacementVerdict::LiveReplacementRejected {
        ReleaseHoldStatus::Rejected
    } else if replacement.release_blocking() {
        ReleaseHoldStatus::HeldForLiveReplacement
    } else if counters.release_blocking_open_count > config.max_open_findings
        || counters.privacy_leak_budget_units > config.max_privacy_leak_budget_units
        || counters.max_residual_risk_bps > config.max_residual_risk_bps
    {
        ReleaseHoldStatus::HeldForAuditClosure
    } else {
        ReleaseHoldStatus::Clear
    }
}

fn hold_reasons(
    findings: &[AuditFindingClosure],
    counters: &Counters,
    replacement: ReplacementVerdict,
    status: ReleaseHoldStatus,
    config: &Config,
) -> BTreeMap<String, String> {
    let mut reasons = BTreeMap::new();
    reasons.insert(
        "release_hold_status".to_string(),
        format!(
            "release hold status is {} with replacement verdict {}",
            status.as_str(),
            replacement.as_str()
        ),
    );

    if replacement.release_blocking() {
        reasons.insert(
            "deferred_to_live_replacement".to_string(),
            "audit closure defers at least one release-blocking finding to live replacement"
                .to_string(),
        );
    }

    if counters.privacy_leak_budget_units > config.max_privacy_leak_budget_units {
        reasons.insert(
            "privacy_leak_budget".to_string(),
            format!(
                "privacy leak budget units {} exceed allowed {}",
                counters.privacy_leak_budget_units, config.max_privacy_leak_budget_units
            ),
        );
    }

    if counters.max_residual_risk_bps > config.max_residual_risk_bps {
        reasons.insert(
            "residual_risk".to_string(),
            format!(
                "max residual risk {} bps exceeds allowed {} bps",
                counters.max_residual_risk_bps, config.max_residual_risk_bps
            ),
        );
    }

    for finding in findings {
        if finding.is_release_blocking(config) {
            reasons.insert(
                format!("finding_{}", finding.finding_id),
                format!(
                    "{} is {} with {} severity",
                    finding.lane.as_str(),
                    finding.status.as_str(),
                    finding.severity.as_str()
                ),
            );
        }
    }

    reasons
}

fn finding_note(lane: AuditFindingLane) -> &'static str {
    match lane {
        AuditFindingLane::ReviewerCommitment => {
            "reviewer commitments must bind independent security and privacy signoff roots"
        }
        AuditFindingLane::PrivacyLeakBudget => {
            "privacy leak budgets must close at zero excess disclosure for user escape answers"
        }
        AuditFindingLane::PqMisuse => {
            "post-quantum misuse findings must prove key separation and signature policy hygiene"
        }
        AuditFindingLane::BridgeCustody => {
            "bridge custody findings must bind reserve, release authority, and forced-exit custody evidence"
        }
        AuditFindingLane::WalletDeanonymization => {
            "wallet deanonymization findings must close metadata linkage and scan-recovery exposure"
        }
        AuditFindingLane::RemediationReceipt => {
            "remediation receipts must bind reviewer acceptance to package roots"
        }
        AuditFindingLane::ResidualRisk => {
            "residual risk roots must remain below release threshold and be reviewer committed"
        }
    }
}

fn lane_root(kind: &str, lane: AuditFindingLane, config: &Config) -> String {
    record_root(
        kind,
        &json!({
            "chain_id": config.chain_id,
            "vertical_slice_id": config.vertical_slice_id,
            "force_exit_package_id": config.force_exit_package_id,
            "closure_id": config.closure_id,
            "lane": lane.as_str(),
            "package_build_height": config.package_build_height,
            "suite": AUDIT_CLOSURE_SUITE,
        }),
    )
}

fn root_from_records(kind: &str, records: Vec<String>) -> String {
    let records_root = merkle_root(records.clone());
    record_root(
        kind,
        &json!({
            "kind": kind,
            "records": records,
            "merkle_root": records_root,
        }),
    )
}

fn stable_id(kind: &str, label: &str) -> String {
    domain_hash(
        DOMAIN,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        16,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        DOMAIN,
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
