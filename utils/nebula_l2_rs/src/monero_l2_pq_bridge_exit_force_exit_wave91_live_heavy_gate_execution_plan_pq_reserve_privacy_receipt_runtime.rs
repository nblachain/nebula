#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;

pub type Runtime = State;
pub type Result<T> = std::result::Result<T, RuntimeError>;

const DOMAIN: &str = "dinero.nebula.wave91.live_heavy_gate_execution_plan.pq_reserve_privacy";
const PROTOCOL: &str = "wave91-live-heavy-gate-execution-plan-pq-reserve-privacy-v1";
const HASH_SUITE: &str = "std-only-deterministic-domain-root-v1";
const DEFAULT_WAVE90_DENIAL_ROOT: &str =
    "wave90.production_readiness_denial_manifest.pq_reserve_privacy.blocker.root";
const DEFAULT_WAVE90_FINAL_TRANSCRIPT_ROOT: &str =
    "wave90.production_readiness_denial_manifest.final_transcript.root";
const DEFAULT_OPERATOR_RUNBOOK_ROOT: &str =
    "wave91.operator.runbook.pq_reserve_privacy.live_heavy_gate.root";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub protocol: String,
    pub lane: Lane,
    pub environment: Environment,
    pub wave90_denial_root: String,
    pub wave90_final_transcript_root: String,
    pub operator_runbook_root: String,
    pub min_authority_epoch: u64,
    pub min_pq_signer_quorum: u32,
    pub min_pq_distinct_families: u32,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub min_privacy_review_depth: u32,
    pub max_linkage_risk_bps: u64,
    pub require_ml_dsa: bool,
    pub require_slh_dsa: bool,
    pub require_quorum: bool,
    pub require_reserve_coverage: bool,
    pub require_privacy_linkage_review: bool,
    pub require_metadata_redaction: bool,
    pub require_nullifier_separation: bool,
    pub require_operator_signoff: bool,
    pub fail_closed: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            protocol: PROTOCOL.to_string(),
            lane: Lane::PqReservePrivacy,
            environment: Environment::Devnet,
            wave90_denial_root: DEFAULT_WAVE90_DENIAL_ROOT.to_string(),
            wave90_final_transcript_root: DEFAULT_WAVE90_FINAL_TRANSCRIPT_ROOT.to_string(),
            operator_runbook_root: DEFAULT_OPERATOR_RUNBOOK_ROOT.to_string(),
            min_authority_epoch: 91,
            min_pq_signer_quorum: 5,
            min_pq_distinct_families: 2,
            min_reserve_coverage_bps: 10_000,
            min_reserve_buffer_bps: 1_500,
            min_privacy_review_depth: 4,
            max_linkage_risk_bps: 50,
            require_ml_dsa: true,
            require_slh_dsa: true,
            require_quorum: true,
            require_reserve_coverage: true,
            require_privacy_linkage_review: true,
            require_metadata_redaction: true,
            require_nullifier_separation: true,
            require_operator_signoff: true,
            fail_closed: true,
        }
    }

    pub fn config_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "config",
            self.protocol.as_str(),
            self.lane.as_str(),
            self.environment.as_str(),
            self.wave90_denial_root.as_str(),
            self.wave90_final_transcript_root.as_str(),
            self.operator_runbook_root.as_str(),
            &self.min_authority_epoch.to_string(),
            &self.min_pq_signer_quorum.to_string(),
            &self.min_pq_distinct_families.to_string(),
            &self.min_reserve_coverage_bps.to_string(),
            &self.min_reserve_buffer_bps.to_string(),
            &self.min_privacy_review_depth.to_string(),
            &self.max_linkage_risk_bps.to_string(),
            bool_word(self.require_ml_dsa),
            bool_word(self.require_slh_dsa),
            bool_word(self.require_quorum),
            bool_word(self.require_reserve_coverage),
            bool_word(self.require_privacy_linkage_review),
            bool_word(self.require_metadata_redaction),
            bool_word(self.require_nullifier_separation),
            bool_word(self.require_operator_signoff),
            bool_word(self.fail_closed),
        ])
    }

    pub fn public_record(&self) -> ConfigRecord {
        ConfigRecord {
            protocol: self.protocol.clone(),
            lane: self.lane,
            environment: self.environment,
            wave90_denial_root: self.wave90_denial_root.clone(),
            wave90_final_transcript_root: self.wave90_final_transcript_root.clone(),
            operator_runbook_root: self.operator_runbook_root.clone(),
            min_authority_epoch: self.min_authority_epoch,
            min_pq_signer_quorum: self.min_pq_signer_quorum,
            min_pq_distinct_families: self.min_pq_distinct_families,
            min_reserve_coverage_bps: self.min_reserve_coverage_bps,
            min_reserve_buffer_bps: self.min_reserve_buffer_bps,
            min_privacy_review_depth: self.min_privacy_review_depth,
            max_linkage_risk_bps: self.max_linkage_risk_bps,
            fail_closed: self.fail_closed,
            config_root: self.config_root(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lane {
    PqReservePrivacy,
}

impl Lane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Environment {
    Devnet,
    LiveShadow,
    ProductionGate,
}

impl Environment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Devnet => "devnet",
            Self::LiveShadow => "live_shadow",
            Self::ProductionGate => "production_gate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub config: Config,
    pub denial_intake: DenialRootIntake,
    pub pq_plan: PqReceiptPlan,
    pub reserve_plan: ReserveReceiptPlan,
    pub privacy_plan: PrivacyReceiptPlan,
    pub criteria: AcceptanceCriteria,
    pub commands: Vec<OperatorCommandHint>,
    pub clearance: ClearanceVerdict,
}

impl State {
    pub fn new(
        config: Config,
        denial_intake: DenialRootIntake,
        pq_plan: PqReceiptPlan,
        reserve_plan: ReserveReceiptPlan,
        privacy_plan: PrivacyReceiptPlan,
        criteria: AcceptanceCriteria,
        commands: Vec<OperatorCommandHint>,
    ) -> Result<Self> {
        let clearance = ClearanceVerdict::from_runtime_parts(
            &config,
            &denial_intake,
            &pq_plan,
            &reserve_plan,
            &privacy_plan,
            &criteria,
        );
        let state = Self {
            config,
            denial_intake,
            pq_plan,
            reserve_plan,
            privacy_plan,
            criteria,
            commands,
            clearance,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let denial_intake = DenialRootIntake::wave90_pq_reserve_privacy();
        let pq_plan = PqReceiptPlan::from_config_and_denial(&config, &denial_intake);
        let reserve_plan = ReserveReceiptPlan::from_config_and_denial(&config, &denial_intake);
        let privacy_plan = PrivacyReceiptPlan::from_config_and_denial(&config, &denial_intake);
        let criteria = AcceptanceCriteria::from_config(&config);
        let commands = OperatorCommandHint::live_heavy_gate_sequence();
        match Self::new(
            config,
            denial_intake,
            pq_plan,
            reserve_plan,
            privacy_plan,
            criteria,
            commands,
        ) {
            Ok(state) => state,
            Err(err) => Self::closed_runtime(err),
        }
    }

    fn closed_runtime(err: RuntimeError) -> Self {
        let config = Config::devnet();
        let denial_intake = DenialRootIntake::wave90_pq_reserve_privacy();
        let mut pq_plan = PqReceiptPlan::from_config_and_denial(&config, &denial_intake);
        let mut reserve_plan = ReserveReceiptPlan::from_config_and_denial(&config, &denial_intake);
        let mut privacy_plan = PrivacyReceiptPlan::from_config_and_denial(&config, &denial_intake);
        pq_plan.status = PlanStatus::Blocked;
        reserve_plan.status = PlanStatus::Blocked;
        privacy_plan.status = PlanStatus::Blocked;
        let mut criteria = AcceptanceCriteria::from_config(&config);
        criteria
            .items
            .push(AcceptanceItem::runtime_guard(err.code()));
        let commands = OperatorCommandHint::closed_sequence(err.code());
        let clearance = ClearanceVerdict::from_runtime_parts(
            &config,
            &denial_intake,
            &pq_plan,
            &reserve_plan,
            &privacy_plan,
            &criteria,
        );
        Self {
            config,
            denial_intake,
            pq_plan,
            reserve_plan,
            privacy_plan,
            criteria,
            commands,
            clearance,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.wave90_denial_root != self.denial_intake.denial_root {
            return Err(RuntimeError::DenialRootMismatch);
        }
        if self.config.wave90_final_transcript_root != self.denial_intake.final_transcript_root {
            return Err(RuntimeError::TranscriptRootMismatch);
        }
        if self.denial_intake.blockers.is_empty() {
            return Err(RuntimeError::DenialBlockersMissing);
        }
        if self.criteria.items.is_empty() {
            return Err(RuntimeError::CriteriaMissing);
        }
        if self.config.fail_closed && self.clearance.status == ClearanceStatus::Clear {
            let all_ready = self.pq_plan.status == PlanStatus::Ready
                && self.reserve_plan.status == PlanStatus::Ready
                && self.privacy_plan.status == PlanStatus::Ready
                && self.criteria.all_satisfied();
            if !all_ready {
                return Err(RuntimeError::FailClosedBypass);
            }
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        PublicRecord {
            protocol: self.config.protocol.clone(),
            lane: self.config.lane,
            environment: self.config.environment,
            denial_intake_root: self.denial_intake.intake_root(),
            pq_plan_root: self.pq_plan.plan_root(),
            reserve_plan_root: self.reserve_plan.plan_root(),
            privacy_plan_root: self.privacy_plan.plan_root(),
            criteria_root: self.criteria.criteria_root(),
            command_root: command_root(&self.commands),
            clearance: self.clearance.clone(),
            state_root: self.state_root(),
        }
    }

    pub fn state_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "state",
            self.config.config_root().as_str(),
            self.denial_intake.intake_root().as_str(),
            self.pq_plan.plan_root().as_str(),
            self.reserve_plan.plan_root().as_str(),
            self.privacy_plan.plan_root().as_str(),
            self.criteria.criteria_root().as_str(),
            command_root(&self.commands).as_str(),
            self.clearance.verdict_root().as_str(),
        ])
    }

    pub fn fail_closed(&self) -> bool {
        self.clearance.status != ClearanceStatus::Clear
    }

    pub fn required_live_receipts(&self) -> Vec<ReceiptRequirement> {
        let mut receipts = Vec::new();
        receipts.extend(self.pq_plan.required_receipts.clone());
        receipts.extend(self.reserve_plan.required_receipts.clone());
        receipts.extend(self.privacy_plan.required_receipts.clone());
        receipts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigRecord {
    pub protocol: String,
    pub lane: Lane,
    pub environment: Environment,
    pub wave90_denial_root: String,
    pub wave90_final_transcript_root: String,
    pub operator_runbook_root: String,
    pub min_authority_epoch: u64,
    pub min_pq_signer_quorum: u32,
    pub min_pq_distinct_families: u32,
    pub min_reserve_coverage_bps: u64,
    pub min_reserve_buffer_bps: u64,
    pub min_privacy_review_depth: u32,
    pub max_linkage_risk_bps: u64,
    pub fail_closed: bool,
    pub config_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRecord {
    pub protocol: String,
    pub lane: Lane,
    pub environment: Environment,
    pub denial_intake_root: String,
    pub pq_plan_root: String,
    pub reserve_plan_root: String,
    pub privacy_plan_root: String,
    pub criteria_root: String,
    pub command_root: String,
    pub clearance: ClearanceVerdict,
    pub state_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialRootIntake {
    pub denial_root: String,
    pub final_transcript_root: String,
    pub source_wave: u32,
    pub source_lane: Lane,
    pub blockers: Vec<DenialBlocker>,
    pub consumed_roots: BTreeMap<String, String>,
}

impl DenialRootIntake {
    pub fn wave90_pq_reserve_privacy() -> Self {
        let blockers = vec![
            DenialBlocker::new(
                BlockerKind::MlDsaAuthorityEpoch,
                "wave90.pq.authority_epoch.ml_dsa.absent.root",
                "ml_dsa authority epoch receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::SlhDsaAuthorityEpoch,
                "wave90.pq.authority_epoch.slh_dsa.absent.root",
                "slh_dsa authority epoch receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::PqSignerQuorum,
                "wave90.pq.signer_quorum.absent.root",
                "pq signer quorum receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::ReserveCoverage,
                "wave90.reserve.coverage.absent.root",
                "reserve coverage receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::PrivacyLinkageReview,
                "wave90.privacy.linkage_review.absent.root",
                "privacy linkage review receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::MetadataRedaction,
                "wave90.privacy.metadata_redaction.absent.root",
                "metadata redaction receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::NullifierSeparation,
                "wave90.privacy.nullifier_separation.absent.root",
                "nullifier separation receipt absent",
            ),
            DenialBlocker::new(
                BlockerKind::OperatorSignoff,
                "wave90.operator.signoff.absent.root",
                "operator signoff receipt absent",
            ),
        ];
        let mut consumed_roots = BTreeMap::new();
        consumed_roots.insert(
            "wave90_denial_root".to_string(),
            DEFAULT_WAVE90_DENIAL_ROOT.to_string(),
        );
        consumed_roots.insert(
            "wave90_final_transcript_root".to_string(),
            DEFAULT_WAVE90_FINAL_TRANSCRIPT_ROOT.to_string(),
        );
        consumed_roots.insert(
            "wave90_pq_reserve_privacy_blocker_set_root".to_string(),
            blocker_set_root(&blockers),
        );
        Self {
            denial_root: DEFAULT_WAVE90_DENIAL_ROOT.to_string(),
            final_transcript_root: DEFAULT_WAVE90_FINAL_TRANSCRIPT_ROOT.to_string(),
            source_wave: 90,
            source_lane: Lane::PqReservePrivacy,
            blockers,
            consumed_roots,
        }
    }

    pub fn intake_root(&self) -> String {
        let consumed = self
            .consumed_roots
            .iter()
            .map(|(name, root)| stable_root(&["consumed", name.as_str(), root.as_str()]))
            .collect::<Vec<_>>();
        stable_root(&[
            DOMAIN,
            "denial_intake",
            self.denial_root.as_str(),
            self.final_transcript_root.as_str(),
            &self.source_wave.to_string(),
            self.source_lane.as_str(),
            blocker_set_root(&self.blockers).as_str(),
            merkle_root(&consumed).as_str(),
        ])
    }

    pub fn has_blocker(&self, kind: BlockerKind) -> bool {
        self.blockers.iter().any(|blocker| blocker.kind == kind)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenialBlocker {
    pub kind: BlockerKind,
    pub blocker_root: String,
    pub public_summary: String,
}

impl DenialBlocker {
    pub fn new(kind: BlockerKind, blocker_root: &str, public_summary: &str) -> Self {
        Self {
            kind,
            blocker_root: stable_root(&[DOMAIN, "wave90_blocker", blocker_root]),
            public_summary: public_summary.to_string(),
        }
    }

    pub fn blocker_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "denial_blocker",
            self.kind.as_str(),
            self.blocker_root.as_str(),
            self.public_summary.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockerKind {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqSignerQuorum,
    ReserveCoverage,
    PrivacyLinkageReview,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl BlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MlDsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqSignerQuorum => "pq_signer_quorum",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyLinkageReview => "privacy_linkage_review",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PqReceiptPlan {
    pub plan_id: String,
    pub status: PlanStatus,
    pub authority_epoch: AuthorityEpochPlan,
    pub quorum: PqSignerQuorumPlan,
    pub required_receipts: Vec<ReceiptRequirement>,
    pub actions: Vec<LiveAction>,
}

impl PqReceiptPlan {
    pub fn from_config_and_denial(config: &Config, denial: &DenialRootIntake) -> Self {
        let authority_epoch = AuthorityEpochPlan {
            min_epoch: config.min_authority_epoch,
            require_ml_dsa: config.require_ml_dsa,
            require_slh_dsa: config.require_slh_dsa,
            ml_dsa_epoch_root: receipt_placeholder_root("ml_dsa_authority_epoch"),
            slh_dsa_epoch_root: receipt_placeholder_root("slh_dsa_authority_epoch"),
            cross_algorithm_binding_root: receipt_placeholder_root(
                "pq_cross_algorithm_authority_binding",
            ),
        };
        let quorum = PqSignerQuorumPlan {
            min_signers: config.min_pq_signer_quorum,
            min_distinct_families: config.min_pq_distinct_families,
            signer_set_root: receipt_placeholder_root("pq_signer_set"),
            quorum_attestation_root: receipt_placeholder_root("pq_quorum_attestation"),
            epoch_binding_root: receipt_placeholder_root("pq_quorum_epoch_binding"),
        };
        let required_receipts = vec![
            ReceiptRequirement::new(
                ReceiptKind::MlDsaAuthorityEpoch,
                "publish ML-DSA authority epoch root at or above lane minimum",
                config.min_authority_epoch,
            ),
            ReceiptRequirement::new(
                ReceiptKind::SlhDsaAuthorityEpoch,
                "publish SLH-DSA authority epoch root at or above lane minimum",
                config.min_authority_epoch,
            ),
            ReceiptRequirement::new(
                ReceiptKind::PqSignerQuorum,
                "bind PQ signer quorum to authority epoch and denial root",
                config.min_authority_epoch,
            ),
        ];
        let actions = vec![
            LiveAction::new(
                ActionKind::CollectReceipt,
                ReceiptKind::MlDsaAuthorityEpoch,
                "collect_ml_dsa_authority_epoch",
                denial.denial_root.as_str(),
            ),
            LiveAction::new(
                ActionKind::CollectReceipt,
                ReceiptKind::SlhDsaAuthorityEpoch,
                "collect_slh_dsa_authority_epoch",
                denial.denial_root.as_str(),
            ),
            LiveAction::new(
                ActionKind::QuorumBind,
                ReceiptKind::PqSignerQuorum,
                "bind_pq_signer_quorum",
                denial.final_transcript_root.as_str(),
            ),
        ];
        let status = if denial.has_blocker(BlockerKind::PqSignerQuorum) {
            PlanStatus::PendingReceipts
        } else {
            PlanStatus::Ready
        };
        Self {
            plan_id: "wave91-pq-authority-quorum-plan".to_string(),
            status,
            authority_epoch,
            quorum,
            required_receipts,
            actions,
        }
    }

    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "pq_plan",
            self.plan_id.as_str(),
            self.status.as_str(),
            self.authority_epoch.plan_root().as_str(),
            self.quorum.plan_root().as_str(),
            receipt_requirements_root(&self.required_receipts).as_str(),
            live_actions_root(&self.actions).as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityEpochPlan {
    pub min_epoch: u64,
    pub require_ml_dsa: bool,
    pub require_slh_dsa: bool,
    pub ml_dsa_epoch_root: String,
    pub slh_dsa_epoch_root: String,
    pub cross_algorithm_binding_root: String,
}

impl AuthorityEpochPlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "authority_epoch_plan",
            &self.min_epoch.to_string(),
            bool_word(self.require_ml_dsa),
            bool_word(self.require_slh_dsa),
            self.ml_dsa_epoch_root.as_str(),
            self.slh_dsa_epoch_root.as_str(),
            self.cross_algorithm_binding_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PqSignerQuorumPlan {
    pub min_signers: u32,
    pub min_distinct_families: u32,
    pub signer_set_root: String,
    pub quorum_attestation_root: String,
    pub epoch_binding_root: String,
}

impl PqSignerQuorumPlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "pq_signer_quorum_plan",
            &self.min_signers.to_string(),
            &self.min_distinct_families.to_string(),
            self.signer_set_root.as_str(),
            self.quorum_attestation_root.as_str(),
            self.epoch_binding_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReserveReceiptPlan {
    pub plan_id: String,
    pub status: PlanStatus,
    pub coverage: ReserveCoveragePlan,
    pub required_receipts: Vec<ReceiptRequirement>,
    pub actions: Vec<LiveAction>,
}

impl ReserveReceiptPlan {
    pub fn from_config_and_denial(config: &Config, denial: &DenialRootIntake) -> Self {
        let coverage = ReserveCoveragePlan {
            min_coverage_bps: config.min_reserve_coverage_bps,
            min_buffer_bps: config.min_reserve_buffer_bps,
            reserve_commitment_root: receipt_placeholder_root("reserve_commitment"),
            liability_commitment_root: receipt_placeholder_root("release_liability_commitment"),
            coverage_attestation_root: receipt_placeholder_root("reserve_coverage_attestation"),
            withdrawal_freeze_root: receipt_placeholder_root("reserve_withdrawal_freeze"),
        };
        let required_receipts = vec![
            ReceiptRequirement::new(
                ReceiptKind::ReserveCoverage,
                "prove reserve coverage and buffer against release liability roots",
                config.min_authority_epoch,
            ),
            ReceiptRequirement::new(
                ReceiptKind::ReserveFreeze,
                "freeze reserve movement until coverage receipt enters heavy gate transcript",
                config.min_authority_epoch,
            ),
        ];
        let actions = vec![
            LiveAction::new(
                ActionKind::CollectReceipt,
                ReceiptKind::ReserveCoverage,
                "collect_reserve_coverage",
                denial.denial_root.as_str(),
            ),
            LiveAction::new(
                ActionKind::PolicyBind,
                ReceiptKind::ReserveFreeze,
                "bind_reserve_freeze",
                denial.final_transcript_root.as_str(),
            ),
        ];
        let status = if denial.has_blocker(BlockerKind::ReserveCoverage) {
            PlanStatus::PendingReceipts
        } else {
            PlanStatus::Ready
        };
        Self {
            plan_id: "wave91-reserve-coverage-plan".to_string(),
            status,
            coverage,
            required_receipts,
            actions,
        }
    }

    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "reserve_plan",
            self.plan_id.as_str(),
            self.status.as_str(),
            self.coverage.plan_root().as_str(),
            receipt_requirements_root(&self.required_receipts).as_str(),
            live_actions_root(&self.actions).as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReserveCoveragePlan {
    pub min_coverage_bps: u64,
    pub min_buffer_bps: u64,
    pub reserve_commitment_root: String,
    pub liability_commitment_root: String,
    pub coverage_attestation_root: String,
    pub withdrawal_freeze_root: String,
}

impl ReserveCoveragePlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "reserve_coverage_plan",
            &self.min_coverage_bps.to_string(),
            &self.min_buffer_bps.to_string(),
            self.reserve_commitment_root.as_str(),
            self.liability_commitment_root.as_str(),
            self.coverage_attestation_root.as_str(),
            self.withdrawal_freeze_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivacyReceiptPlan {
    pub plan_id: String,
    pub status: PlanStatus,
    pub linkage_review: PrivacyLinkageReviewPlan,
    pub metadata_redaction: MetadataRedactionPlan,
    pub nullifier_separation: NullifierSeparationPlan,
    pub required_receipts: Vec<ReceiptRequirement>,
    pub actions: Vec<LiveAction>,
}

impl PrivacyReceiptPlan {
    pub fn from_config_and_denial(config: &Config, denial: &DenialRootIntake) -> Self {
        let linkage_review = PrivacyLinkageReviewPlan {
            min_review_depth: config.min_privacy_review_depth,
            max_linkage_risk_bps: config.max_linkage_risk_bps,
            review_matrix_root: receipt_placeholder_root("privacy_linkage_review_matrix"),
            reviewer_attestation_root: receipt_placeholder_root(
                "privacy_linkage_reviewer_attestation",
            ),
            risk_floor_root: receipt_placeholder_root("privacy_linkage_risk_floor"),
        };
        let metadata_redaction = MetadataRedactionPlan {
            redaction_policy_root: receipt_placeholder_root("metadata_redaction_policy"),
            retained_fields_root: receipt_placeholder_root("metadata_retained_fields_roots_only"),
            removed_fields_root: receipt_placeholder_root("metadata_removed_fields_roots_only"),
            reviewer_attestation_root: receipt_placeholder_root(
                "metadata_redaction_reviewer_attestation",
            ),
        };
        let nullifier_separation = NullifierSeparationPlan {
            exit_nullifier_domain_root: receipt_placeholder_root("exit_nullifier_domain"),
            reserve_nullifier_domain_root: receipt_placeholder_root("reserve_nullifier_domain"),
            privacy_nullifier_domain_root: receipt_placeholder_root("privacy_nullifier_domain"),
            separation_proof_root: receipt_placeholder_root("nullifier_separation_proof"),
        };
        let required_receipts = vec![
            ReceiptRequirement::new(
                ReceiptKind::PrivacyLinkageReview,
                "submit roots-only linkage review with bounded risk",
                config.min_authority_epoch,
            ),
            ReceiptRequirement::new(
                ReceiptKind::MetadataRedaction,
                "submit roots-only metadata redaction receipt",
                config.min_authority_epoch,
            ),
            ReceiptRequirement::new(
                ReceiptKind::NullifierSeparation,
                "prove nullifier domains are separated before release",
                config.min_authority_epoch,
            ),
        ];
        let actions = vec![
            LiveAction::new(
                ActionKind::Review,
                ReceiptKind::PrivacyLinkageReview,
                "review_privacy_linkage",
                denial.denial_root.as_str(),
            ),
            LiveAction::new(
                ActionKind::Redact,
                ReceiptKind::MetadataRedaction,
                "redact_metadata_to_roots",
                denial.final_transcript_root.as_str(),
            ),
            LiveAction::new(
                ActionKind::Separate,
                ReceiptKind::NullifierSeparation,
                "separate_nullifier_domains",
                denial.denial_root.as_str(),
            ),
        ];
        let status = if denial.has_blocker(BlockerKind::PrivacyLinkageReview)
            || denial.has_blocker(BlockerKind::MetadataRedaction)
            || denial.has_blocker(BlockerKind::NullifierSeparation)
        {
            PlanStatus::PendingReceipts
        } else {
            PlanStatus::Ready
        };
        Self {
            plan_id: "wave91-privacy-receipt-plan".to_string(),
            status,
            linkage_review,
            metadata_redaction,
            nullifier_separation,
            required_receipts,
            actions,
        }
    }

    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "privacy_plan",
            self.plan_id.as_str(),
            self.status.as_str(),
            self.linkage_review.plan_root().as_str(),
            self.metadata_redaction.plan_root().as_str(),
            self.nullifier_separation.plan_root().as_str(),
            receipt_requirements_root(&self.required_receipts).as_str(),
            live_actions_root(&self.actions).as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivacyLinkageReviewPlan {
    pub min_review_depth: u32,
    pub max_linkage_risk_bps: u64,
    pub review_matrix_root: String,
    pub reviewer_attestation_root: String,
    pub risk_floor_root: String,
}

impl PrivacyLinkageReviewPlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "privacy_linkage_review_plan",
            &self.min_review_depth.to_string(),
            &self.max_linkage_risk_bps.to_string(),
            self.review_matrix_root.as_str(),
            self.reviewer_attestation_root.as_str(),
            self.risk_floor_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataRedactionPlan {
    pub redaction_policy_root: String,
    pub retained_fields_root: String,
    pub removed_fields_root: String,
    pub reviewer_attestation_root: String,
}

impl MetadataRedactionPlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "metadata_redaction_plan",
            self.redaction_policy_root.as_str(),
            self.retained_fields_root.as_str(),
            self.removed_fields_root.as_str(),
            self.reviewer_attestation_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullifierSeparationPlan {
    pub exit_nullifier_domain_root: String,
    pub reserve_nullifier_domain_root: String,
    pub privacy_nullifier_domain_root: String,
    pub separation_proof_root: String,
}

impl NullifierSeparationPlan {
    pub fn plan_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "nullifier_separation_plan",
            self.exit_nullifier_domain_root.as_str(),
            self.reserve_nullifier_domain_root.as_str(),
            self.privacy_nullifier_domain_root.as_str(),
            self.separation_proof_root.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanStatus {
    PendingReceipts,
    Ready,
    Blocked,
}

impl PlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingReceipts => "pending_receipts",
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptRequirement {
    pub kind: ReceiptKind,
    pub description: String,
    pub min_epoch: u64,
    pub denial_binding_root: String,
    pub receipt_schema_root: String,
}

impl ReceiptRequirement {
    pub fn new(kind: ReceiptKind, description: &str, min_epoch: u64) -> Self {
        Self {
            kind,
            description: description.to_string(),
            min_epoch,
            denial_binding_root: stable_root(&[DOMAIN, "denial_binding", kind.as_str()]),
            receipt_schema_root: stable_root(&[DOMAIN, "receipt_schema", kind.as_str()]),
        }
    }

    pub fn requirement_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "receipt_requirement",
            self.kind.as_str(),
            self.description.as_str(),
            &self.min_epoch.to_string(),
            self.denial_binding_root.as_str(),
            self.receipt_schema_root.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptKind {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqSignerQuorum,
    ReserveCoverage,
    ReserveFreeze,
    PrivacyLinkageReview,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
}

impl ReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MlDsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqSignerQuorum => "pq_signer_quorum",
            Self::ReserveCoverage => "reserve_coverage",
            Self::ReserveFreeze => "reserve_freeze",
            Self::PrivacyLinkageReview => "privacy_linkage_review",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveAction {
    pub kind: ActionKind,
    pub receipt_kind: ReceiptKind,
    pub command: String,
    pub binding_root: String,
    pub action_root: String,
}

impl LiveAction {
    pub fn new(
        kind: ActionKind,
        receipt_kind: ReceiptKind,
        command: &str,
        binding_root: &str,
    ) -> Self {
        let action_root = stable_root(&[
            DOMAIN,
            "live_action",
            kind.as_str(),
            receipt_kind.as_str(),
            command,
            binding_root,
        ]);
        Self {
            kind,
            receipt_kind,
            command: command.to_string(),
            binding_root: binding_root.to_string(),
            action_root,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionKind {
    CollectReceipt,
    QuorumBind,
    PolicyBind,
    Review,
    Redact,
    Separate,
    Signoff,
}

impl ActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CollectReceipt => "collect_receipt",
            Self::QuorumBind => "quorum_bind",
            Self::PolicyBind => "policy_bind",
            Self::Review => "review",
            Self::Redact => "redact",
            Self::Separate => "separate",
            Self::Signoff => "signoff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceCriteria {
    pub criteria_id: String,
    pub items: Vec<AcceptanceItem>,
}

impl AcceptanceCriteria {
    pub fn from_config(config: &Config) -> Self {
        let mut items = Vec::new();
        if config.require_ml_dsa {
            items.push(AcceptanceItem::new(
                CriteriaKind::MlDsaAuthorityEpoch,
                "ML-DSA authority epoch root is present and bound to Wave 90 denial root",
                config.min_authority_epoch,
            ));
        }
        if config.require_slh_dsa {
            items.push(AcceptanceItem::new(
                CriteriaKind::SlhDsaAuthorityEpoch,
                "SLH-DSA authority epoch root is present and bound to Wave 90 denial root",
                config.min_authority_epoch,
            ));
        }
        if config.require_quorum {
            items.push(AcceptanceItem::new(
                CriteriaKind::PqSignerQuorum,
                "PQ signer quorum meets signer count and family diversity minimums",
                config.min_authority_epoch,
            ));
        }
        if config.require_reserve_coverage {
            items.push(AcceptanceItem::new(
                CriteriaKind::ReserveCoverage,
                "reserve coverage meets coverage and buffer minimums",
                config.min_authority_epoch,
            ));
        }
        if config.require_privacy_linkage_review {
            items.push(AcceptanceItem::new(
                CriteriaKind::PrivacyLinkageReview,
                "privacy linkage review is roots-only and under risk limit",
                config.min_authority_epoch,
            ));
        }
        if config.require_metadata_redaction {
            items.push(AcceptanceItem::new(
                CriteriaKind::MetadataRedaction,
                "metadata redaction receipt contains roots only",
                config.min_authority_epoch,
            ));
        }
        if config.require_nullifier_separation {
            items.push(AcceptanceItem::new(
                CriteriaKind::NullifierSeparation,
                "exit reserve and privacy nullifier domains are separated",
                config.min_authority_epoch,
            ));
        }
        if config.require_operator_signoff {
            items.push(AcceptanceItem::new(
                CriteriaKind::OperatorSignoff,
                "operator signoff binds receipt set to heavy gate transcript",
                config.min_authority_epoch,
            ));
        }
        Self {
            criteria_id: "wave91-pq-reserve-privacy-clearance-criteria".to_string(),
            items,
        }
    }

    pub fn all_satisfied(&self) -> bool {
        self.items
            .iter()
            .all(|item| item.status == CriteriaStatus::Satisfied)
    }

    pub fn criteria_root(&self) -> String {
        let roots = self
            .items
            .iter()
            .map(AcceptanceItem::item_root)
            .collect::<Vec<_>>();
        stable_root(&[
            DOMAIN,
            "acceptance_criteria",
            self.criteria_id.as_str(),
            merkle_root(&roots).as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceItem {
    pub kind: CriteriaKind,
    pub description: String,
    pub min_epoch: u64,
    pub status: CriteriaStatus,
    pub evidence_root: String,
}

impl AcceptanceItem {
    pub fn new(kind: CriteriaKind, description: &str, min_epoch: u64) -> Self {
        Self {
            kind,
            description: description.to_string(),
            min_epoch,
            status: CriteriaStatus::Pending,
            evidence_root: receipt_placeholder_root(kind.as_str()),
        }
    }

    pub fn runtime_guard(code: &'static str) -> Self {
        Self {
            kind: CriteriaKind::RuntimeGuard,
            description: "runtime self validation closed the lane".to_string(),
            min_epoch: 0,
            status: CriteriaStatus::Blocked,
            evidence_root: stable_root(&[DOMAIN, "runtime_guard", code]),
        }
    }

    pub fn item_root(&self) -> String {
        stable_root(&[
            DOMAIN,
            "acceptance_item",
            self.kind.as_str(),
            self.description.as_str(),
            &self.min_epoch.to_string(),
            self.status.as_str(),
            self.evidence_root.as_str(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CriteriaKind {
    MlDsaAuthorityEpoch,
    SlhDsaAuthorityEpoch,
    PqSignerQuorum,
    ReserveCoverage,
    PrivacyLinkageReview,
    MetadataRedaction,
    NullifierSeparation,
    OperatorSignoff,
    RuntimeGuard,
}

impl CriteriaKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MlDsaAuthorityEpoch => "ml_dsa_authority_epoch",
            Self::SlhDsaAuthorityEpoch => "slh_dsa_authority_epoch",
            Self::PqSignerQuorum => "pq_signer_quorum",
            Self::ReserveCoverage => "reserve_coverage",
            Self::PrivacyLinkageReview => "privacy_linkage_review",
            Self::MetadataRedaction => "metadata_redaction",
            Self::NullifierSeparation => "nullifier_separation",
            Self::OperatorSignoff => "operator_signoff",
            Self::RuntimeGuard => "runtime_guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CriteriaStatus {
    Pending,
    Satisfied,
    Blocked,
}

impl CriteriaStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Satisfied => "satisfied",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorCommandHint {
    pub sequence: u32,
    pub command: String,
    pub purpose: String,
    pub produces_receipt: Option<ReceiptKind>,
    pub guard_root: String,
}

impl OperatorCommandHint {
    pub fn live_heavy_gate_sequence() -> Vec<Self> {
        vec![
            Self::new(
                10,
                "wave91-pq-authority-epoch-intake",
                "import ML-DSA and SLH-DSA authority epoch roots",
                Some(ReceiptKind::MlDsaAuthorityEpoch),
            ),
            Self::new(
                20,
                "wave91-pq-signer-quorum-bind",
                "bind PQ signer quorum to authority epoch roots",
                Some(ReceiptKind::PqSignerQuorum),
            ),
            Self::new(
                30,
                "wave91-reserve-coverage-attest",
                "attest reserve coverage and reserve movement freeze",
                Some(ReceiptKind::ReserveCoverage),
            ),
            Self::new(
                40,
                "wave91-privacy-linkage-review",
                "publish roots-only linkage review receipt",
                Some(ReceiptKind::PrivacyLinkageReview),
            ),
            Self::new(
                50,
                "wave91-metadata-redaction-receipt",
                "publish roots-only metadata redaction receipt",
                Some(ReceiptKind::MetadataRedaction),
            ),
            Self::new(
                60,
                "wave91-nullifier-separation-proof",
                "publish separated nullifier domain roots",
                Some(ReceiptKind::NullifierSeparation),
            ),
            Self::new(
                70,
                "wave91-operator-signoff",
                "bind all live receipts to heavy gate transcript",
                Some(ReceiptKind::OperatorSignoff),
            ),
        ]
    }

    pub fn closed_sequence(code: &'static str) -> Vec<Self> {
        vec![Self {
            sequence: 0,
            command: "wave91-lane-hold".to_string(),
            purpose: format!("lane remains closed by runtime guard {}", code),
            produces_receipt: None,
            guard_root: stable_root(&[DOMAIN, "closed_sequence", code]),
        }]
    }

    pub fn new(
        sequence: u32,
        command: &str,
        purpose: &str,
        produces_receipt: Option<ReceiptKind>,
    ) -> Self {
        let receipt = receipt_name(produces_receipt);
        Self {
            sequence,
            command: command.to_string(),
            purpose: purpose.to_string(),
            produces_receipt,
            guard_root: stable_root(&[
                DOMAIN,
                "operator_command_hint",
                &sequence.to_string(),
                command,
                purpose,
                receipt,
            ]),
        }
    }

    pub fn command_root(&self) -> String {
        let receipt = receipt_name(self.produces_receipt);
        stable_root(&[
            DOMAIN,
            "command",
            &self.sequence.to_string(),
            self.command.as_str(),
            self.purpose.as_str(),
            receipt,
            self.guard_root.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClearanceVerdict {
    pub status: ClearanceStatus,
    pub reason: String,
    pub missing_receipts: Vec<ReceiptKind>,
    pub hold_roots: Vec<String>,
    pub operator_signoff_required: bool,
}

impl ClearanceVerdict {
    pub fn from_runtime_parts(
        config: &Config,
        denial: &DenialRootIntake,
        pq_plan: &PqReceiptPlan,
        reserve_plan: &ReserveReceiptPlan,
        privacy_plan: &PrivacyReceiptPlan,
        criteria: &AcceptanceCriteria,
    ) -> Self {
        let mut missing_receipts = Vec::new();
        if config.require_ml_dsa && denial.has_blocker(BlockerKind::MlDsaAuthorityEpoch) {
            missing_receipts.push(ReceiptKind::MlDsaAuthorityEpoch);
        }
        if config.require_slh_dsa && denial.has_blocker(BlockerKind::SlhDsaAuthorityEpoch) {
            missing_receipts.push(ReceiptKind::SlhDsaAuthorityEpoch);
        }
        if config.require_quorum && denial.has_blocker(BlockerKind::PqSignerQuorum) {
            missing_receipts.push(ReceiptKind::PqSignerQuorum);
        }
        if config.require_reserve_coverage && denial.has_blocker(BlockerKind::ReserveCoverage) {
            missing_receipts.push(ReceiptKind::ReserveCoverage);
        }
        if config.require_privacy_linkage_review
            && denial.has_blocker(BlockerKind::PrivacyLinkageReview)
        {
            missing_receipts.push(ReceiptKind::PrivacyLinkageReview);
        }
        if config.require_metadata_redaction && denial.has_blocker(BlockerKind::MetadataRedaction) {
            missing_receipts.push(ReceiptKind::MetadataRedaction);
        }
        if config.require_nullifier_separation
            && denial.has_blocker(BlockerKind::NullifierSeparation)
        {
            missing_receipts.push(ReceiptKind::NullifierSeparation);
        }
        if config.require_operator_signoff && denial.has_blocker(BlockerKind::OperatorSignoff) {
            missing_receipts.push(ReceiptKind::OperatorSignoff);
        }
        let plan_roots = vec![
            pq_plan.plan_root(),
            reserve_plan.plan_root(),
            privacy_plan.plan_root(),
            criteria.criteria_root(),
        ];
        let hold_roots = missing_receipts
            .iter()
            .map(|kind| stable_root(&[DOMAIN, "clearance_hold", kind.as_str()]))
            .chain(plan_roots)
            .collect::<Vec<_>>();
        let status = if missing_receipts.is_empty() && criteria.all_satisfied() {
            ClearanceStatus::Clear
        } else if config.fail_closed {
            ClearanceStatus::Closed
        } else {
            ClearanceStatus::Hold
        };
        let reason = match status {
            ClearanceStatus::Clear => {
                "all PQ reserve privacy live receipts are present".to_string()
            }
            ClearanceStatus::Hold => {
                "one or more PQ reserve privacy receipts remain pending".to_string()
            }
            ClearanceStatus::Closed => {
                "fail closed until live receipt set and operator signoff clear all blockers"
                    .to_string()
            }
        };
        Self {
            status,
            reason,
            missing_receipts,
            hold_roots,
            operator_signoff_required: config.require_operator_signoff,
        }
    }

    pub fn verdict_root(&self) -> String {
        let missing = self
            .missing_receipts
            .iter()
            .map(|kind| kind.as_str().to_string())
            .collect::<Vec<_>>();
        stable_root(&[
            DOMAIN,
            "clearance_verdict",
            self.status.as_str(),
            self.reason.as_str(),
            merkle_root(&missing).as_str(),
            merkle_root(&self.hold_roots).as_str(),
            bool_word(self.operator_signoff_required),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClearanceStatus {
    Clear,
    Hold,
    Closed,
}

impl ClearanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Hold => "hold",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    DenialRootMismatch,
    TranscriptRootMismatch,
    DenialBlockersMissing,
    CriteriaMissing,
    FailClosedBypass,
}

impl RuntimeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DenialRootMismatch => "denial_root_mismatch",
            Self::TranscriptRootMismatch => "transcript_root_mismatch",
            Self::DenialBlockersMissing => "denial_blockers_missing",
            Self::CriteriaMissing => "criteria_missing",
            Self::FailClosedBypass => "fail_closed_bypass",
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for RuntimeError {}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn denial_root_intake() -> DenialRootIntake {
    DenialRootIntake::wave90_pq_reserve_privacy()
}

pub fn pq_receipt_plan() -> PqReceiptPlan {
    let config = Config::devnet();
    let denial = DenialRootIntake::wave90_pq_reserve_privacy();
    PqReceiptPlan::from_config_and_denial(&config, &denial)
}

pub fn reserve_receipt_plan() -> ReserveReceiptPlan {
    let config = Config::devnet();
    let denial = DenialRootIntake::wave90_pq_reserve_privacy();
    ReserveReceiptPlan::from_config_and_denial(&config, &denial)
}

pub fn privacy_receipt_plan() -> PrivacyReceiptPlan {
    let config = Config::devnet();
    let denial = DenialRootIntake::wave90_pq_reserve_privacy();
    PrivacyReceiptPlan::from_config_and_denial(&config, &denial)
}

pub fn acceptance_criteria() -> AcceptanceCriteria {
    AcceptanceCriteria::from_config(&Config::devnet())
}

pub fn operator_command_hints() -> Vec<OperatorCommandHint> {
    OperatorCommandHint::live_heavy_gate_sequence()
}

fn blocker_set_root(blockers: &[DenialBlocker]) -> String {
    let roots = blockers
        .iter()
        .map(DenialBlocker::blocker_root)
        .collect::<Vec<_>>();
    stable_root(&[DOMAIN, "blocker_set", merkle_root(&roots).as_str()])
}

fn receipt_requirements_root(requirements: &[ReceiptRequirement]) -> String {
    let roots = requirements
        .iter()
        .map(ReceiptRequirement::requirement_root)
        .collect::<Vec<_>>();
    stable_root(&[DOMAIN, "receipt_requirements", merkle_root(&roots).as_str()])
}

fn live_actions_root(actions: &[LiveAction]) -> String {
    let roots = actions
        .iter()
        .map(|action| action.action_root.clone())
        .collect::<Vec<_>>();
    stable_root(&[DOMAIN, "live_actions", merkle_root(&roots).as_str()])
}

fn command_root(commands: &[OperatorCommandHint]) -> String {
    let roots = commands
        .iter()
        .map(OperatorCommandHint::command_root)
        .collect::<Vec<_>>();
    stable_root(&[DOMAIN, "operator_commands", merkle_root(&roots).as_str()])
}

fn receipt_placeholder_root(name: &str) -> String {
    stable_root(&[DOMAIN, "receipt_placeholder", name, "roots_only"])
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn receipt_name(receipt: Option<ReceiptKind>) -> &'static str {
    match receipt {
        Some(kind) => kind.as_str(),
        None => "none",
    }
}

fn merkle_root(items: &[String]) -> String {
    if items.is_empty() {
        return stable_root(&[DOMAIN, "empty"]);
    }
    let mut level = items.to_vec();
    while level.len() > 1 {
        let mut next = Vec::new();
        let mut index = 0;
        while index < level.len() {
            let left = level[index].as_str();
            let right = if index + 1 < level.len() {
                level[index + 1].as_str()
            } else {
                left
            };
            next.push(stable_root(&[DOMAIN, "node", left, right]));
            index += 2;
        }
        level = next;
    }
    match level.first() {
        Some(root) => root.clone(),
        None => stable_root(&[DOMAIN, "empty"]),
    }
}

fn stable_root(parts: &[&str]) -> String {
    let mut a: u128 = 0x6c62_7269_6467_655f_7071_5f726f6f745u128;
    let mut b: u128 = 0x7265_7365_7276_655f_7072_6976616379u128;
    for part in parts {
        mix_part(&mut a, &mut b, part.as_bytes());
        mix_part(&mut a, &mut b, &[0xff]);
    }
    format!("{:032x}{:032x}", a, b)
}

fn mix_part(a: &mut u128, b: &mut u128, bytes: &[u8]) {
    for byte in bytes {
        *a ^= u128::from(*byte) + 0x9e37_79b9_7f4a_7c15u128;
        *a = a.rotate_left(11).wrapping_mul(0x1000_0000_01b3u128);
        *b ^= *a ^ (u128::from(*byte) << 1);
        *b = b.rotate_left(17).wrapping_mul(0x1000_0000_013bu128);
    }
}

pub fn hash_suite() -> &'static str {
    HASH_SUITE
}
