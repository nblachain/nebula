use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeReplayAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-replay-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const EVIDENCE_IMPORT_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-replay-accepted-live-evidence-import-v1";
pub const DEFAULT_MIN_ACCEPTED_EVIDENCE: u64 = 8;
pub const DEFAULT_MIN_REPLAY_DOMAINS: u64 = 6;
pub const DEFAULT_MIN_FINALITY_DEPTH: u64 = 36;
pub const DEFAULT_MAX_GOVERNANCE_RISK_SCORE: u64 = 18;
pub const DEFAULT_MAX_IMPORTS: usize = 256;
pub const DEFAULT_MAX_DECISIONS: usize = 256;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayEvidenceKind {
    UserEscapeIntent,
    CanonicalExitPackage,
    ForceExitTranscript,
    L2InclusionReceipt,
    MoneroHeaderFinality,
    PqAuthorityAttestation,
    WalletPrivacyReceipt,
    OperatorBondSlashability,
    RemediationAnswer,
    GovernanceReadiness,
}

impl ReplayEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserEscapeIntent => "user_escape_intent",
            Self::CanonicalExitPackage => "canonical_exit_package",
            Self::ForceExitTranscript => "force_exit_transcript",
            Self::L2InclusionReceipt => "l2_inclusion_receipt",
            Self::MoneroHeaderFinality => "monero_header_finality",
            Self::PqAuthorityAttestation => "pq_authority_attestation",
            Self::WalletPrivacyReceipt => "wallet_privacy_receipt",
            Self::OperatorBondSlashability => "operator_bond_slashability",
            Self::RemediationAnswer => "remediation_answer",
            Self::GovernanceReadiness => "governance_readiness",
        }
    }

    pub fn all() -> [Self; 10] {
        [
            Self::UserEscapeIntent,
            Self::CanonicalExitPackage,
            Self::ForceExitTranscript,
            Self::L2InclusionReceipt,
            Self::MoneroHeaderFinality,
            Self::PqAuthorityAttestation,
            Self::WalletPrivacyReceipt,
            Self::OperatorBondSlashability,
            Self::RemediationAnswer,
            Self::GovernanceReadiness,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDomain {
    Admission,
    PackageCanonicality,
    ExecutionReplay,
    FinalityBridge,
    PrivacySurface,
    AuthorityControl,
    SlashingControl,
    GovernanceControl,
}

impl ReplayDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::PackageCanonicality => "package_canonicality",
            Self::ExecutionReplay => "execution_replay",
            Self::FinalityBridge => "finality_bridge",
            Self::PrivacySurface => "privacy_surface",
            Self::AuthorityControl => "authority_control",
            Self::SlashingControl => "slashing_control",
            Self::GovernanceControl => "governance_control",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAcceptanceStatus {
    Accepted,
    Quarantined,
    Rejected,
}

impl EvidenceAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Quarantined => "quarantined",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    Go,
    NoGo,
    Watch,
}

impl GovernanceDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
            Self::Watch => "watch",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub evidence_import_suite: String,
    pub min_accepted_evidence: u64,
    pub min_replay_domains: u64,
    pub min_finality_depth: u64,
    pub max_governance_risk_score: u64,
    pub live_governance_enabled: bool,
    pub cargo_checks_deferred: bool,
    pub production_release_allowed: bool,
    pub max_imports: usize,
    pub max_decisions: usize,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            evidence_import_suite: EVIDENCE_IMPORT_SUITE.to_string(),
            min_accepted_evidence: DEFAULT_MIN_ACCEPTED_EVIDENCE,
            min_replay_domains: DEFAULT_MIN_REPLAY_DOMAINS,
            min_finality_depth: DEFAULT_MIN_FINALITY_DEPTH,
            max_governance_risk_score: DEFAULT_MAX_GOVERNANCE_RISK_SCORE,
            live_governance_enabled: true,
            cargo_checks_deferred: true,
            production_release_allowed: false,
            max_imports: DEFAULT_MAX_IMPORTS,
            max_decisions: DEFAULT_MAX_DECISIONS,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "evidence_import_suite": self.evidence_import_suite,
            "min_accepted_evidence": self.min_accepted_evidence,
            "min_replay_domains": self.min_replay_domains,
            "min_finality_depth": self.min_finality_depth,
            "max_governance_risk_score": self.max_governance_risk_score,
            "live_governance_enabled": self.live_governance_enabled,
            "cargo_checks_deferred": self.cargo_checks_deferred,
            "production_release_allowed": self.production_release_allowed,
            "max_imports": self.max_imports,
            "max_decisions": self.max_decisions,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayAnchor {
    pub anchor_id: String,
    pub package_id: String,
    pub replay_round: u64,
    pub source_runtime: String,
    pub source_state_root: String,
    pub accepted_replay_root: String,
    pub live_evidence_root: String,
    pub governance_input_root: String,
}

impl ReplayAnchor {
    pub fn new(
        package_id: impl Into<String>,
        replay_round: u64,
        source_runtime: impl Into<String>,
        source_state_root: impl Into<String>,
        accepted_replay_root: impl Into<String>,
        live_evidence_root: impl Into<String>,
    ) -> Self {
        let package_id = package_id.into();
        let source_runtime = source_runtime.into();
        let source_state_root = source_state_root.into();
        let accepted_replay_root = accepted_replay_root.into();
        let live_evidence_root = live_evidence_root.into();
        let governance_input_root = domain_hash(
            "MONERO-FORCE-EXIT-REPLAY-GOVERNANCE-INPUT",
            &[
                HashPart::Str(&package_id),
                HashPart::U64(replay_round),
                HashPart::Str(&accepted_replay_root),
                HashPart::Str(&live_evidence_root),
            ],
            32,
        );
        let anchor_id = domain_hash(
            "MONERO-FORCE-EXIT-REPLAY-ANCHOR-ID",
            &[
                HashPart::Str(&package_id),
                HashPart::U64(replay_round),
                HashPart::Str(&source_runtime),
                HashPart::Str(&source_state_root),
                HashPart::Str(&governance_input_root),
            ],
            16,
        );
        Self {
            anchor_id,
            package_id,
            replay_round,
            source_runtime,
            source_state_root,
            accepted_replay_root,
            live_evidence_root,
            governance_input_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "anchor_id": self.anchor_id,
            "package_id": self.package_id,
            "replay_round": self.replay_round,
            "source_runtime": self.source_runtime,
            "source_state_root": self.source_state_root,
            "accepted_replay_root": self.accepted_replay_root,
            "live_evidence_root": self.live_evidence_root,
            "governance_input_root": self.governance_input_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay_anchor", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveEvidenceImport {
    pub import_id: String,
    pub anchor_id: String,
    pub package_id: String,
    pub evidence_kind: ReplayEvidenceKind,
    pub replay_domain: ReplayDomain,
    pub acceptance_status: EvidenceAcceptanceStatus,
    pub subject_id: String,
    pub source_label: String,
    pub observed: String,
    pub replay_root: String,
    pub evidence_root: String,
    pub witness_root: String,
    pub finality_depth: u64,
    pub risk_score: u64,
    pub blocks_governance_go: bool,
    pub imported_at_height: u64,
}

impl LiveEvidenceImport {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        anchor: &ReplayAnchor,
        evidence_kind: ReplayEvidenceKind,
        replay_domain: ReplayDomain,
        acceptance_status: EvidenceAcceptanceStatus,
        subject_id: impl Into<String>,
        source_label: impl Into<String>,
        observed: impl Into<String>,
        replay_root: impl Into<String>,
        evidence_record: Value,
        witness_record: Value,
        finality_depth: u64,
        risk_score: u64,
        imported_at_height: u64,
    ) -> Self {
        let subject_id = subject_id.into();
        let source_label = source_label.into();
        let observed = observed.into();
        let replay_root = replay_root.into();
        let evidence_root = record_root("live_evidence_payload", &evidence_record);
        let witness_root = record_root("live_evidence_witness", &witness_record);
        let blocks_governance_go = !acceptance_status.is_accepted() || risk_score > 0;
        let import_id = domain_hash(
            "MONERO-FORCE-EXIT-LIVE-EVIDENCE-IMPORT-ID",
            &[
                HashPart::Str(&anchor.anchor_id),
                HashPart::Str(evidence_kind.as_str()),
                HashPart::Str(replay_domain.as_str()),
                HashPart::Str(&subject_id),
                HashPart::Str(&replay_root),
                HashPart::Str(&evidence_root),
                HashPart::U64(imported_at_height),
            ],
            16,
        );
        Self {
            import_id,
            anchor_id: anchor.anchor_id.clone(),
            package_id: anchor.package_id.clone(),
            evidence_kind,
            replay_domain,
            acceptance_status,
            subject_id,
            source_label,
            observed,
            replay_root,
            evidence_root,
            witness_root,
            finality_depth,
            risk_score,
            blocks_governance_go,
            imported_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "import_id": self.import_id,
            "anchor_id": self.anchor_id,
            "package_id": self.package_id,
            "evidence_kind": self.evidence_kind.as_str(),
            "replay_domain": self.replay_domain.as_str(),
            "acceptance_status": self.acceptance_status.as_str(),
            "subject_id": self.subject_id,
            "source_label": self.source_label,
            "observed": self.observed,
            "replay_root": self.replay_root,
            "evidence_root": self.evidence_root,
            "witness_root": self.witness_root,
            "finality_depth": self.finality_depth,
            "risk_score": self.risk_score,
            "blocks_governance_go": self.blocks_governance_go,
            "imported_at_height": self.imported_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("live_evidence_import", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImportCounters {
    pub anchors: u64,
    pub imports: u64,
    pub accepted_imports: u64,
    pub quarantined_imports: u64,
    pub rejected_imports: u64,
    pub unique_domains: u64,
    pub governance_decisions: u64,
    pub go_decisions: u64,
    pub watch_decisions: u64,
    pub no_go_decisions: u64,
}

impl ImportCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "anchors": self.anchors,
            "imports": self.imports,
            "accepted_imports": self.accepted_imports,
            "quarantined_imports": self.quarantined_imports,
            "rejected_imports": self.rejected_imports,
            "unique_domains": self.unique_domains,
            "governance_decisions": self.governance_decisions,
            "go_decisions": self.go_decisions,
            "watch_decisions": self.watch_decisions,
            "no_go_decisions": self.no_go_decisions,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("import_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportRoots {
    pub config_root: String,
    pub anchor_root: String,
    pub import_root: String,
    pub accepted_import_root: String,
    pub rejected_import_root: String,
    pub governance_decision_root: String,
    pub counters_root: String,
}

impl ImportRoots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "anchor_root": self.anchor_root,
            "import_root": self.import_root,
            "accepted_import_root": self.accepted_import_root,
            "rejected_import_root": self.rejected_import_root,
            "governance_decision_root": self.governance_decision_root,
            "counters_root": self.counters_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("import_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GovernanceReadiness {
    pub package_id: String,
    pub anchor_id: String,
    pub accepted_evidence_count: u64,
    pub replay_domain_count: u64,
    pub min_finality_depth: u64,
    pub aggregate_risk_score: u64,
    pub has_rejected_evidence: bool,
    pub has_quarantined_evidence: bool,
    pub missing_domains: Vec<String>,
    pub missing_kinds: Vec<String>,
    pub ready_for_governance_go: bool,
}

impl GovernanceReadiness {
    pub fn from_imports(
        config: &Config,
        anchor: &ReplayAnchor,
        imports: &[LiveEvidenceImport],
    ) -> Self {
        let mut domains = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        let mut min_finality_depth = u64::MAX;
        let mut accepted_evidence_count = 0_u64;
        let mut aggregate_risk_score = 0_u64;
        let mut has_rejected_evidence = false;
        let mut has_quarantined_evidence = false;

        for import in imports {
            match import.acceptance_status {
                EvidenceAcceptanceStatus::Accepted => {
                    accepted_evidence_count += 1;
                    domains.insert(import.replay_domain.as_str().to_string());
                    kinds.insert(import.evidence_kind.as_str().to_string());
                    min_finality_depth = min_finality_depth.min(import.finality_depth);
                    aggregate_risk_score = aggregate_risk_score.saturating_add(import.risk_score);
                }
                EvidenceAcceptanceStatus::Quarantined => has_quarantined_evidence = true,
                EvidenceAcceptanceStatus::Rejected => has_rejected_evidence = true,
            }
        }

        if accepted_evidence_count == 0 {
            min_finality_depth = 0;
        }

        let missing_domains = required_domain_names()
            .into_iter()
            .filter(|domain| !domains.contains(*domain))
            .map(str::to_string)
            .collect::<Vec<_>>();
        let missing_kinds = ReplayEvidenceKind::all()
            .into_iter()
            .map(|kind| kind.as_str().to_string())
            .filter(|kind| !kinds.contains(kind))
            .collect::<Vec<_>>();
        let ready_for_governance_go = accepted_evidence_count >= config.min_accepted_evidence
            && domains.len() as u64 >= config.min_replay_domains
            && min_finality_depth >= config.min_finality_depth
            && aggregate_risk_score <= config.max_governance_risk_score
            && !has_rejected_evidence
            && !has_quarantined_evidence;

        Self {
            package_id: anchor.package_id.clone(),
            anchor_id: anchor.anchor_id.clone(),
            accepted_evidence_count,
            replay_domain_count: domains.len() as u64,
            min_finality_depth,
            aggregate_risk_score,
            has_rejected_evidence,
            has_quarantined_evidence,
            missing_domains,
            missing_kinds,
            ready_for_governance_go,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "package_id": self.package_id,
            "anchor_id": self.anchor_id,
            "accepted_evidence_count": self.accepted_evidence_count,
            "replay_domain_count": self.replay_domain_count,
            "min_finality_depth": self.min_finality_depth,
            "aggregate_risk_score": self.aggregate_risk_score,
            "has_rejected_evidence": self.has_rejected_evidence,
            "has_quarantined_evidence": self.has_quarantined_evidence,
            "missing_domains": self.missing_domains,
            "missing_kinds": self.missing_kinds,
            "ready_for_governance_go": self.ready_for_governance_go,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("governance_readiness", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GovernanceImportDecision {
    pub decision_id: String,
    pub package_id: String,
    pub anchor_id: String,
    pub decision: GovernanceDecision,
    pub reason: String,
    pub readiness_root: String,
    pub accepted_import_root: String,
    pub governance_input_root: String,
    pub decided_at_height: u64,
    pub public_release_allowed: bool,
}

impl GovernanceImportDecision {
    pub fn new(
        config: &Config,
        anchor: &ReplayAnchor,
        readiness: &GovernanceReadiness,
        accepted_import_root: impl Into<String>,
        decided_at_height: u64,
    ) -> Self {
        let accepted_import_root = accepted_import_root.into();
        let readiness_root = readiness.state_root();
        let decision = if readiness.ready_for_governance_go && config.live_governance_enabled {
            GovernanceDecision::Go
        } else if readiness.has_rejected_evidence
            || readiness.aggregate_risk_score > config.max_governance_risk_score
        {
            GovernanceDecision::NoGo
        } else {
            GovernanceDecision::Watch
        };
        let reason = decision_reason(config, readiness, decision);
        let public_release_allowed =
            decision == GovernanceDecision::Go && config.production_release_allowed;
        let decision_id = domain_hash(
            "MONERO-FORCE-EXIT-GOVERNANCE-IMPORT-DECISION-ID",
            &[
                HashPart::Str(&anchor.anchor_id),
                HashPart::Str(decision.as_str()),
                HashPart::Str(&readiness_root),
                HashPart::Str(&accepted_import_root),
                HashPart::U64(decided_at_height),
            ],
            16,
        );
        Self {
            decision_id,
            package_id: anchor.package_id.clone(),
            anchor_id: anchor.anchor_id.clone(),
            decision,
            reason,
            readiness_root,
            accepted_import_root,
            governance_input_root: anchor.governance_input_root.clone(),
            decided_at_height,
            public_release_allowed,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "decision_id": self.decision_id,
            "package_id": self.package_id,
            "anchor_id": self.anchor_id,
            "decision": self.decision.as_str(),
            "reason": self.reason,
            "readiness_root": self.readiness_root,
            "accepted_import_root": self.accepted_import_root,
            "governance_input_root": self.governance_input_root,
            "decided_at_height": self.decided_at_height,
            "public_release_allowed": self.public_release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("governance_import_decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub config: Config,
    pub anchors: BTreeMap<String, ReplayAnchor>,
    pub imports: BTreeMap<String, LiveEvidenceImport>,
    pub decisions: BTreeMap<String, GovernanceImportDecision>,
}

impl State {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            anchors: BTreeMap::new(),
            imports: BTreeMap::new(),
            decisions: BTreeMap::new(),
        }
    }

    pub fn devnet(operator_label: &str) -> Result<Self> {
        let mut state = Self::new(Config::devnet());
        let anchor = ReplayAnchor::new(
            "devnet-canonical-user-escape-force-exit-package-001",
            1,
            "canonical-user-escape-answer-vertical-slice-runtime",
            seed_root(operator_label, "source-state"),
            seed_root(operator_label, "accepted-replay"),
            seed_root(operator_label, "live-evidence"),
        );
        state.register_anchor(anchor.clone())?;

        let records = [
            (
                ReplayEvidenceKind::UserEscapeIntent,
                ReplayDomain::Admission,
                "escape intent replay accepted with matching user nullifier",
                42_u64,
                1_u64,
            ),
            (
                ReplayEvidenceKind::CanonicalExitPackage,
                ReplayDomain::PackageCanonicality,
                "canonical force-exit package hash matches governance intake",
                44,
                1,
            ),
            (
                ReplayEvidenceKind::ForceExitTranscript,
                ReplayDomain::ExecutionReplay,
                "force-exit transcript replay accepted by live verifier",
                45,
                2,
            ),
            (
                ReplayEvidenceKind::L2InclusionReceipt,
                ReplayDomain::ExecutionReplay,
                "l2 inclusion receipt anchored in accepted replay",
                46,
                2,
            ),
            (
                ReplayEvidenceKind::MoneroHeaderFinality,
                ReplayDomain::FinalityBridge,
                "monero header finality exceeds governance floor",
                50,
                1,
            ),
            (
                ReplayEvidenceKind::PqAuthorityAttestation,
                ReplayDomain::AuthorityControl,
                "pq authority attestation binds replay output to package",
                47,
                2,
            ),
            (
                ReplayEvidenceKind::WalletPrivacyReceipt,
                ReplayDomain::PrivacySurface,
                "wallet privacy receipt preserves public/private split",
                48,
                2,
            ),
            (
                ReplayEvidenceKind::OperatorBondSlashability,
                ReplayDomain::SlashingControl,
                "operator bond can be slashed on replay divergence",
                49,
                1,
            ),
            (
                ReplayEvidenceKind::RemediationAnswer,
                ReplayDomain::GovernanceControl,
                "canonical user escape answer remediation imported live",
                43,
                2,
            ),
            (
                ReplayEvidenceKind::GovernanceReadiness,
                ReplayDomain::GovernanceControl,
                "final go/no-go evidence bundle assembled",
                41,
                1,
            ),
        ];

        for (index, (kind, domain, observed, finality_depth, risk_score)) in
            records.into_iter().enumerate()
        {
            let subject_id = format!("devnet-live-evidence-{index:02}");
            let evidence_record = json!({
                "operator_label": operator_label,
                "subject_id": subject_id.clone(),
                "kind": kind.as_str(),
                "domain": domain.as_str(),
                "observed": observed,
            });
            let witness_record = json!({
                "anchor_id": anchor.anchor_id.clone(),
                "package_id": anchor.package_id.clone(),
                "witness_lane": "devnet-governance-import",
                "index": index as u64,
            });
            let import = LiveEvidenceImport::new(
                &anchor,
                kind,
                domain,
                EvidenceAcceptanceStatus::Accepted,
                subject_id,
                "devnet-live-replay-accepted-importer",
                observed,
                seed_root(operator_label, kind.as_str()),
                evidence_record,
                witness_record,
                finality_depth,
                risk_score,
                7_000 + index as u64,
            );
            state.import_live_evidence(import)?;
        }

        let _decision = state.decide_governance(&anchor.anchor_id, 7_100)?;
        Ok(state)
    }

    pub fn register_anchor(&mut self, anchor: ReplayAnchor) -> Result<String> {
        self.ensure_import_capacity(0)?;
        if self.anchors.contains_key(&anchor.anchor_id) {
            return Err(format!(
                "replay anchor already registered: {}",
                anchor.anchor_id
            ));
        }
        let anchor_root = anchor.state_root();
        self.anchors.insert(anchor.anchor_id.clone(), anchor);
        Ok(anchor_root)
    }

    pub fn import_live_evidence(&mut self, import: LiveEvidenceImport) -> Result<String> {
        self.ensure_import_capacity(1)?;
        if !self.anchors.contains_key(&import.anchor_id) {
            return Err(format!(
                "unknown replay anchor for import: {}",
                import.anchor_id
            ));
        }
        if self.imports.contains_key(&import.import_id) {
            return Err(format!(
                "live evidence import already exists: {}",
                import.import_id
            ));
        }
        let import_root = import.state_root();
        self.imports.insert(import.import_id.clone(), import);
        Ok(import_root)
    }

    pub fn decide_governance(
        &mut self,
        anchor_id: &str,
        decided_at_height: u64,
    ) -> Result<GovernanceImportDecision> {
        self.ensure_decision_capacity(1)?;
        let anchor = self
            .anchors
            .get(anchor_id)
            .ok_or_else(|| format!("unknown replay anchor for governance decision: {anchor_id}"))?;
        let imports = self.imports_for_anchor(anchor_id);
        let readiness = GovernanceReadiness::from_imports(&self.config, anchor, &imports);
        let accepted_import_root = accepted_imports_root(&imports);
        let decision = GovernanceImportDecision::new(
            &self.config,
            anchor,
            &readiness,
            accepted_import_root,
            decided_at_height,
        );
        self.decisions
            .insert(decision.decision_id.clone(), decision.clone());
        Ok(decision)
    }

    pub fn readiness_for_anchor(&self, anchor_id: &str) -> Result<GovernanceReadiness> {
        let anchor = self
            .anchors
            .get(anchor_id)
            .ok_or_else(|| format!("unknown replay anchor for readiness: {anchor_id}"))?;
        let imports = self.imports_for_anchor(anchor_id);
        Ok(GovernanceReadiness::from_imports(
            &self.config,
            anchor,
            &imports,
        ))
    }

    pub fn imports_for_anchor(&self, anchor_id: &str) -> Vec<LiveEvidenceImport> {
        self.imports
            .values()
            .filter(|import| import.anchor_id == anchor_id)
            .cloned()
            .collect()
    }

    pub fn latest_decision_for_anchor(&self, anchor_id: &str) -> Option<GovernanceImportDecision> {
        self.decisions
            .values()
            .filter(|decision| decision.anchor_id == anchor_id)
            .max_by_key(|decision| decision.decided_at_height)
            .cloned()
    }

    pub fn counters(&self) -> ImportCounters {
        let mut domain_names = BTreeSet::new();
        let mut counters = ImportCounters {
            anchors: self.anchors.len() as u64,
            imports: self.imports.len() as u64,
            governance_decisions: self.decisions.len() as u64,
            ..ImportCounters::default()
        };

        for import in self.imports.values() {
            domain_names.insert(import.replay_domain.as_str().to_string());
            match import.acceptance_status {
                EvidenceAcceptanceStatus::Accepted => counters.accepted_imports += 1,
                EvidenceAcceptanceStatus::Quarantined => counters.quarantined_imports += 1,
                EvidenceAcceptanceStatus::Rejected => counters.rejected_imports += 1,
            }
        }

        for decision in self.decisions.values() {
            match decision.decision {
                GovernanceDecision::Go => counters.go_decisions += 1,
                GovernanceDecision::NoGo => counters.no_go_decisions += 1,
                GovernanceDecision::Watch => counters.watch_decisions += 1,
            }
        }

        counters.unique_domains = domain_names.len() as u64;
        counters
    }

    pub fn roots(&self) -> ImportRoots {
        let counters = self.counters();
        ImportRoots {
            config_root: self.config.state_root(),
            anchor_root: values_merkle_root(
                "MONERO-FORCE-EXIT-REPLAY-ANCHORS",
                self.anchors
                    .values()
                    .map(ReplayAnchor::public_record)
                    .collect(),
            ),
            import_root: values_merkle_root(
                "MONERO-FORCE-EXIT-LIVE-EVIDENCE-IMPORTS",
                self.imports
                    .values()
                    .map(LiveEvidenceImport::public_record)
                    .collect(),
            ),
            accepted_import_root: values_merkle_root(
                "MONERO-FORCE-EXIT-ACCEPTED-LIVE-EVIDENCE-IMPORTS",
                self.imports
                    .values()
                    .filter(|import| import.acceptance_status.is_accepted())
                    .map(LiveEvidenceImport::public_record)
                    .collect(),
            ),
            rejected_import_root: values_merkle_root(
                "MONERO-FORCE-EXIT-NON-ACCEPTED-LIVE-EVIDENCE-IMPORTS",
                self.imports
                    .values()
                    .filter(|import| !import.acceptance_status.is_accepted())
                    .map(LiveEvidenceImport::public_record)
                    .collect(),
            ),
            governance_decision_root: values_merkle_root(
                "MONERO-FORCE-EXIT-GOVERNANCE-IMPORT-DECISIONS",
                self.decisions
                    .values()
                    .map(GovernanceImportDecision::public_record)
                    .collect(),
            ),
            counters_root: counters.state_root(),
        }
    }

    pub fn state_root(&self) -> String {
        let roots = self.roots();
        domain_hash(
            "MONERO-FORCE-EXIT-REPLAY-ACCEPTED-LIVE-EVIDENCE-IMPORT-STATE",
            &[
                HashPart::Json(&self.config.public_record()),
                HashPart::Json(&roots.public_record()),
                HashPart::Json(&self.counters().public_record()),
            ],
            32,
        )
    }

    pub fn public_record(&self) -> Value {
        let roots = self.roots();
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "state_root": self.state_root(),
            "config": self.config.public_record(),
            "roots": roots.public_record(),
            "counters": self.counters().public_record(),
            "anchors": self.anchors
                .iter()
                .map(|(anchor_id, anchor)| (anchor_id.clone(), anchor.public_record()))
                .collect::<BTreeMap<_, _>>(),
            "imports": self.imports
                .iter()
                .map(|(import_id, import)| (import_id.clone(), import.public_record()))
                .collect::<BTreeMap<_, _>>(),
            "decisions": self.decisions
                .iter()
                .map(|(decision_id, decision)| (decision_id.clone(), decision.public_record()))
                .collect::<BTreeMap<_, _>>(),
        })
    }

    fn ensure_import_capacity(&self, additional: usize) -> Result<()> {
        if self.imports.len().saturating_add(additional) > self.config.max_imports {
            return Err(format!(
                "live evidence import capacity exceeded: {} + {} > {}",
                self.imports.len(),
                additional,
                self.config.max_imports
            ));
        }
        Ok(())
    }

    fn ensure_decision_capacity(&self, additional: usize) -> Result<()> {
        if self.decisions.len().saturating_add(additional) > self.config.max_decisions {
            return Err(format!(
                "governance decision capacity exceeded: {} + {} > {}",
                self.decisions.len(),
                additional,
                self.config.max_decisions
            ));
        }
        Ok(())
    }
}

pub fn devnet(operator_label: &str) -> Result<State> {
    State::devnet(operator_label)
}

pub fn public_record(state: &State) -> Value {
    state.public_record()
}

pub fn state_root(state: &State) -> String {
    state.state_root()
}

pub fn replay_anchor_id(package_id: &str, replay_round: u64, accepted_replay_root: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REPLAY-ANCHOR-REFERENCE",
        &[
            HashPart::Str(package_id),
            HashPart::U64(replay_round),
            HashPart::Str(accepted_replay_root),
        ],
        16,
    )
}

pub fn governance_import_package_root(
    anchor: &ReplayAnchor,
    imports: &[LiveEvidenceImport],
    decision: &GovernanceImportDecision,
) -> String {
    let import_records = imports
        .iter()
        .map(LiveEvidenceImport::public_record)
        .collect::<Vec<_>>();
    let import_root = values_merkle_root(
        "MONERO-FORCE-EXIT-GOVERNANCE-PACKAGE-IMPORTS",
        import_records,
    );
    domain_hash(
        "MONERO-FORCE-EXIT-GOVERNANCE-PACKAGE-ROOT",
        &[
            HashPart::Json(&anchor.public_record()),
            HashPart::Str(&import_root),
            HashPart::Json(&decision.public_record()),
        ],
        32,
    )
}

fn decision_reason(
    config: &Config,
    readiness: &GovernanceReadiness,
    decision: GovernanceDecision,
) -> String {
    match decision {
        GovernanceDecision::Go => {
            "accepted replay evidence meets live governance go thresholds".to_string()
        }
        GovernanceDecision::NoGo => {
            if readiness.has_rejected_evidence {
                "rejected live replay evidence prevents final governance go".to_string()
            } else {
                format!(
                    "aggregate risk score {} exceeds governance maximum {}",
                    readiness.aggregate_risk_score, config.max_governance_risk_score
                )
            }
        }
        GovernanceDecision::Watch => {
            let mut reasons = Vec::new();
            if readiness.accepted_evidence_count < config.min_accepted_evidence {
                reasons.push(format!(
                    "accepted evidence {} below required {}",
                    readiness.accepted_evidence_count, config.min_accepted_evidence
                ));
            }
            if readiness.replay_domain_count < config.min_replay_domains {
                reasons.push(format!(
                    "replay domains {} below required {}",
                    readiness.replay_domain_count, config.min_replay_domains
                ));
            }
            if readiness.min_finality_depth < config.min_finality_depth {
                reasons.push(format!(
                    "minimum finality depth {} below required {}",
                    readiness.min_finality_depth, config.min_finality_depth
                ));
            }
            if readiness.has_quarantined_evidence {
                reasons.push("quarantined evidence requires review".to_string());
            }
            if !config.live_governance_enabled {
                reasons.push("live governance import disabled by config".to_string());
            }
            if reasons.is_empty() {
                "live evidence import remains under governance watch".to_string()
            } else {
                reasons.join("; ")
            }
        }
    }
}

fn accepted_imports_root(imports: &[LiveEvidenceImport]) -> String {
    values_merkle_root(
        "MONERO-FORCE-EXIT-ANCHOR-ACCEPTED-IMPORTS",
        imports
            .iter()
            .filter(|import| import.acceptance_status.is_accepted())
            .map(LiveEvidenceImport::public_record)
            .collect(),
    )
}

fn required_domain_names() -> Vec<&'static str> {
    vec![
        ReplayDomain::Admission.as_str(),
        ReplayDomain::PackageCanonicality.as_str(),
        ReplayDomain::ExecutionReplay.as_str(),
        ReplayDomain::FinalityBridge.as_str(),
        ReplayDomain::PrivacySurface.as_str(),
        ReplayDomain::AuthorityControl.as_str(),
        ReplayDomain::SlashingControl.as_str(),
        ReplayDomain::GovernanceControl.as_str(),
    ]
}

fn seed_root(operator_label: &str, label: &str) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REPLAY-ACCEPTED-LIVE-EVIDENCE-IMPORT-SEED",
        &[HashPart::Str(operator_label), HashPart::Str(label)],
        32,
    )
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-FORCE-EXIT-REPLAY-ACCEPTED-LIVE-EVIDENCE-IMPORT-RECORD",
        &[HashPart::Str(label), HashPart::Json(record)],
        32,
    )
}

fn values_merkle_root(domain: &str, values: Vec<Value>) -> String {
    merkle_root(domain, &values)
}
