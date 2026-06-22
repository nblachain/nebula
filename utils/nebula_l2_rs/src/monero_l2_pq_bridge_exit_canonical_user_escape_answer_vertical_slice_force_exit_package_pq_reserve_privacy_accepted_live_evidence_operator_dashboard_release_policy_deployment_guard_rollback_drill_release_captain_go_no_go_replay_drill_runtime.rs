use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqReservePrivacyAcceptedLiveEvidenceOperatorDashboardReleasePolicyDeploymentGuardRollbackDrillReleaseCaptainGoNoGoReplayDrillRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-pq-reserve-privacy-release-captain-go-no-go-replay-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_ACCEPTED_LIVE_EVIDENCE_OPERATOR_DASHBOARD_RELEASE_POLICY_DEPLOYMENT_GUARD_ROLLBACK_DRILL_RELEASE_CAPTAIN_GO_NO_GO_REPLAY_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const REPLAY_DRILL_SUITE: &str =
    "monero-l2-force-exit-package-pq-reserve-privacy-release-captain-go-no-go-replay-drill-v1";
pub const DEFAULT_WAVE: u64 = 88;
pub const DEFAULT_SOURCE_WAVE: u64 = 87;
pub const DEFAULT_RELEASE_HEIGHT: u64 = 4_288_800;
pub const DEFAULT_MAX_ROOT_AGE_BLOCKS: u64 = 144;
pub const DEFAULT_MIN_PQ_SIGNER_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_PQ_SIGNER_COUNT: u64 = 4;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u64 = 10_500;
pub const DEFAULT_MAX_PRIVACY_SPEND_BPS: u64 = 6_000;
pub const DEFAULT_MIN_BUCKET_COUNT: u64 = 8;
pub const DEFAULT_MIN_KEY_ROTATION_ROOTS: u64 = 3;
pub const DEFAULT_MIN_PRIVACY_SIGNOFFS: u64 = 3;
pub const DEFAULT_MIN_RELEASE_CAPTAIN_SIGNOFFS: u64 = 2;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillDecision {
    Go,
    NoGo,
}

impl DrillDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerSeverity {
    Advisory,
    Soft,
    Hard,
    Fatal,
}

impl BlockerSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Soft => "soft",
            Self::Hard => "hard",
            Self::Fatal => "fatal",
        }
    }

    pub fn fail_closed(self) -> bool {
        matches!(self, Self::Hard | Self::Fatal)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayBlockerKind {
    MissingWave87ChecklistRoot,
    StaleWave87ChecklistRoot,
    PqSignerPolicyReplayMissing,
    PqSignerWeightShortfall,
    PqSignerCountShortfall,
    PqSignerReplayRootMismatch,
    ReserveAttestationMissing,
    ReserveCoverageShortfall,
    ReserveBlockerActive,
    AmountBucketPrivacyMissing,
    AmountBucketCountShortfall,
    AmountBucketLeakage,
    KeyRotationReadinessMissing,
    KeyRotationRootShortfall,
    PrivacyBudgetSignoffMissing,
    PrivacyBudgetExceeded,
    ReleaseCaptainSignoffMissing,
    RollbackDrillMissing,
    RollbackDrillFailed,
    ReplayDecisionMismatch,
    DuplicateSignerPolicy,
    DuplicateReleaseCaptain,
}

impl ReplayBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWave87ChecklistRoot => "missing_wave87_checklist_root",
            Self::StaleWave87ChecklistRoot => "stale_wave87_checklist_root",
            Self::PqSignerPolicyReplayMissing => "pq_signer_policy_replay_missing",
            Self::PqSignerWeightShortfall => "pq_signer_weight_shortfall",
            Self::PqSignerCountShortfall => "pq_signer_count_shortfall",
            Self::PqSignerReplayRootMismatch => "pq_signer_replay_root_mismatch",
            Self::ReserveAttestationMissing => "reserve_attestation_missing",
            Self::ReserveCoverageShortfall => "reserve_coverage_shortfall",
            Self::ReserveBlockerActive => "reserve_blocker_active",
            Self::AmountBucketPrivacyMissing => "amount_bucket_privacy_missing",
            Self::AmountBucketCountShortfall => "amount_bucket_count_shortfall",
            Self::AmountBucketLeakage => "amount_bucket_leakage",
            Self::KeyRotationReadinessMissing => "key_rotation_readiness_missing",
            Self::KeyRotationRootShortfall => "key_rotation_root_shortfall",
            Self::PrivacyBudgetSignoffMissing => "privacy_budget_signoff_missing",
            Self::PrivacyBudgetExceeded => "privacy_budget_exceeded",
            Self::ReleaseCaptainSignoffMissing => "release_captain_signoff_missing",
            Self::RollbackDrillMissing => "rollback_drill_missing",
            Self::RollbackDrillFailed => "rollback_drill_failed",
            Self::ReplayDecisionMismatch => "replay_decision_mismatch",
            Self::DuplicateSignerPolicy => "duplicate_signer_policy",
            Self::DuplicateReleaseCaptain => "duplicate_release_captain",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub replay_drill_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub release_height: u64,
    pub max_root_age_blocks: u64,
    pub min_pq_signer_weight: u64,
    pub min_pq_signer_count: u64,
    pub min_reserve_coverage_bps: u64,
    pub max_privacy_spend_bps: u64,
    pub min_amount_bucket_count: u64,
    pub min_key_rotation_roots: u64,
    pub min_privacy_signoffs: u64,
    pub min_release_captain_signoffs: u64,
    pub fail_closed_on_any_blocker: bool,
    pub fail_closed_on_replay_mismatch: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_drill_suite: REPLAY_DRILL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            release_height: DEFAULT_RELEASE_HEIGHT,
            max_root_age_blocks: DEFAULT_MAX_ROOT_AGE_BLOCKS,
            min_pq_signer_weight: DEFAULT_MIN_PQ_SIGNER_WEIGHT,
            min_pq_signer_count: DEFAULT_MIN_PQ_SIGNER_COUNT,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            max_privacy_spend_bps: DEFAULT_MAX_PRIVACY_SPEND_BPS,
            min_amount_bucket_count: DEFAULT_MIN_BUCKET_COUNT,
            min_key_rotation_roots: DEFAULT_MIN_KEY_ROTATION_ROOTS,
            min_privacy_signoffs: DEFAULT_MIN_PRIVACY_SIGNOFFS,
            min_release_captain_signoffs: DEFAULT_MIN_RELEASE_CAPTAIN_SIGNOFFS,
            fail_closed_on_any_blocker: true,
            fail_closed_on_replay_mismatch: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "replay_drill_suite": self.replay_drill_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "release_height": self.release_height,
            "max_root_age_blocks": self.max_root_age_blocks,
            "min_pq_signer_weight": self.min_pq_signer_weight,
            "min_pq_signer_count": self.min_pq_signer_count,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "max_privacy_spend_bps": self.max_privacy_spend_bps,
            "min_amount_bucket_count": self.min_amount_bucket_count,
            "min_key_rotation_roots": self.min_key_rotation_roots,
            "min_privacy_signoffs": self.min_privacy_signoffs,
            "min_release_captain_signoffs": self.min_release_captain_signoffs,
            "fail_closed_on_any_blocker": self.fail_closed_on_any_blocker,
            "fail_closed_on_replay_mismatch": self.fail_closed_on_replay_mismatch,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave87ChecklistRoot {
    pub lane: String,
    pub checklist_root: String,
    pub accepted_height: u64,
    pub item_count: u64,
    pub go_no_go_root: String,
}

impl Wave87ChecklistRoot {
    pub fn public_record(&self) -> Value {
        json!({
            "lane": self.lane,
            "checklist_root": self.checklist_root,
            "accepted_height": self.accepted_height,
            "item_count": self.item_count,
            "go_no_go_root": self.go_no_go_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave87-checklist", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqSignerPolicyReplay {
    pub signer_policy_id: String,
    pub policy_root: String,
    pub replay_root: String,
    pub signer_weight: u64,
    pub signer_count: u64,
    pub key_rotation_readiness_root: String,
    pub replay_matches_policy: bool,
}

impl PqSignerPolicyReplay {
    pub fn public_record(&self) -> Value {
        json!({
            "signer_policy_id": self.signer_policy_id,
            "policy_root": self.policy_root,
            "replay_root": self.replay_root,
            "signer_weight": self.signer_weight,
            "signer_count": self.signer_count,
            "key_rotation_readiness_root": self.key_rotation_readiness_root,
            "replay_matches_policy": self.replay_matches_policy,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("pq-signer-policy-replay", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveAttestationReplay {
    pub reserve_attestation_id: String,
    pub reserve_attestation_root: String,
    pub reserve_coverage_bps: u64,
    pub oracle_count: u64,
    pub blocker_active: bool,
    pub accepted_live_evidence_root: String,
}

impl ReserveAttestationReplay {
    pub fn public_record(&self) -> Value {
        json!({
            "reserve_attestation_id": self.reserve_attestation_id,
            "reserve_attestation_root": self.reserve_attestation_root,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "oracle_count": self.oracle_count,
            "blocker_active": self.blocker_active,
            "accepted_live_evidence_root": self.accepted_live_evidence_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("reserve-attestation-replay", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AmountBucketPrivacySafeguard {
    pub bucket_set_id: String,
    pub bucket_policy_root: String,
    pub bucket_count: u64,
    pub privacy_spend_bps: u64,
    pub non_linkage_root: String,
    pub leakage_detected: bool,
}

impl AmountBucketPrivacySafeguard {
    pub fn public_record(&self) -> Value {
        json!({
            "bucket_set_id": self.bucket_set_id,
            "bucket_policy_root": self.bucket_policy_root,
            "bucket_count": self.bucket_count,
            "privacy_spend_bps": self.privacy_spend_bps,
            "non_linkage_root": self.non_linkage_root,
            "leakage_detected": self.leakage_detected,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("amount-bucket-privacy", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeyRotationReadinessRoot {
    pub rotation_id: String,
    pub readiness_root: String,
    pub next_epoch: u64,
    pub activation_guard_root: String,
    pub rollback_guard_root: String,
    pub ready: bool,
}

impl KeyRotationReadinessRoot {
    pub fn public_record(&self) -> Value {
        json!({
            "rotation_id": self.rotation_id,
            "readiness_root": self.readiness_root,
            "next_epoch": self.next_epoch,
            "activation_guard_root": self.activation_guard_root,
            "rollback_guard_root": self.rollback_guard_root,
            "ready": self.ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("key-rotation-readiness", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyBudgetSignoff {
    pub reviewer_id: String,
    pub signoff_root: String,
    pub privacy_budget_root: String,
    pub approved_spend_bps: u64,
    pub accepted: bool,
}

impl PrivacyBudgetSignoff {
    pub fn public_record(&self) -> Value {
        json!({
            "reviewer_id": self.reviewer_id,
            "signoff_root": self.signoff_root,
            "privacy_budget_root": self.privacy_budget_root,
            "approved_spend_bps": self.approved_spend_bps,
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("privacy-budget-signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseCaptainSignoff {
    pub captain_id: String,
    pub signoff_root: String,
    pub release_policy_root: String,
    pub rollback_drill_root: String,
    pub decision: DrillDecision,
    pub accepted: bool,
}

impl ReleaseCaptainSignoff {
    pub fn public_record(&self) -> Value {
        json!({
            "captain_id": self.captain_id,
            "signoff_root": self.signoff_root,
            "release_policy_root": self.release_policy_root,
            "rollback_drill_root": self.rollback_drill_root,
            "decision": self.decision.as_str(),
            "accepted": self.accepted,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release-captain-signoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RollbackReplayDrill {
    pub drill_id: String,
    pub replay_drill_root: String,
    pub deployment_guard_root: String,
    pub rollback_evidence_root: String,
    pub operator_dashboard_root: String,
    pub passed: bool,
}

impl RollbackReplayDrill {
    pub fn public_record(&self) -> Value {
        json!({
            "drill_id": self.drill_id,
            "replay_drill_root": self.replay_drill_root,
            "deployment_guard_root": self.deployment_guard_root,
            "rollback_evidence_root": self.rollback_evidence_root,
            "operator_dashboard_root": self.operator_dashboard_root,
            "passed": self.passed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("rollback-replay-drill", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayBlocker {
    pub kind: ReplayBlockerKind,
    pub severity: BlockerSeverity,
    pub evidence_root: String,
    pub detail: String,
}

impl ReplayBlocker {
    pub fn new(
        kind: ReplayBlockerKind,
        severity: BlockerSeverity,
        detail: &str,
        evidence_root: String,
    ) -> Self {
        Self {
            kind,
            severity,
            evidence_root,
            detail: detail.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "severity": self.severity.as_str(),
            "evidence_root": self.evidence_root,
            "detail": self.detail,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay-blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayVerdict {
    pub decision: DrillDecision,
    pub release_allowed: bool,
    pub fail_closed: bool,
    pub blocker_count: u64,
    pub hard_blocker_count: u64,
    pub go_no_go_root: String,
    pub fail_closed_root: String,
}

impl ReplayVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "decision": self.decision.as_str(),
            "release_allowed": self.release_allowed,
            "fail_closed": self.fail_closed,
            "blocker_count": self.blocker_count,
            "hard_blocker_count": self.hard_blocker_count,
            "go_no_go_root": self.go_no_go_root,
            "fail_closed_root": self.fail_closed_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("replay-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub wave87_checklist_root: String,
    pub pq_signer_policy_replay_root: String,
    pub reserve_attestation_root: String,
    pub amount_bucket_privacy_root: String,
    pub key_rotation_readiness_root: String,
    pub privacy_budget_signoff_root: String,
    pub release_captain_signoff_root: String,
    pub rollback_replay_drill_root: String,
    pub blocker_root: String,
    pub verdict_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "wave87_checklist_root": self.wave87_checklist_root,
            "pq_signer_policy_replay_root": self.pq_signer_policy_replay_root,
            "reserve_attestation_root": self.reserve_attestation_root,
            "amount_bucket_privacy_root": self.amount_bucket_privacy_root,
            "key_rotation_readiness_root": self.key_rotation_readiness_root,
            "privacy_budget_signoff_root": self.privacy_budget_signoff_root,
            "release_captain_signoff_root": self.release_captain_signoff_root,
            "rollback_replay_drill_root": self.rollback_replay_drill_root,
            "blocker_root": self.blocker_root,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub wave87_checklist_count: u64,
    pub pq_signer_policy_count: u64,
    pub reserve_attestation_count: u64,
    pub amount_bucket_policy_count: u64,
    pub key_rotation_ready_count: u64,
    pub privacy_budget_signoff_count: u64,
    pub release_captain_signoff_count: u64,
    pub rollback_drill_count: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "wave87_checklist_count": self.wave87_checklist_count,
            "pq_signer_policy_count": self.pq_signer_policy_count,
            "reserve_attestation_count": self.reserve_attestation_count,
            "amount_bucket_policy_count": self.amount_bucket_policy_count,
            "key_rotation_ready_count": self.key_rotation_ready_count,
            "privacy_budget_signoff_count": self.privacy_budget_signoff_count,
            "release_captain_signoff_count": self.release_captain_signoff_count,
            "rollback_drill_count": self.rollback_drill_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave87_checklists: Vec<Wave87ChecklistRoot>,
    pub pq_signer_policies: Vec<PqSignerPolicyReplay>,
    pub reserve_attestations: Vec<ReserveAttestationReplay>,
    pub amount_bucket_privacy: Vec<AmountBucketPrivacySafeguard>,
    pub key_rotation_readiness: Vec<KeyRotationReadinessRoot>,
    pub privacy_budget_signoffs: Vec<PrivacyBudgetSignoff>,
    pub release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
    pub rollback_drills: Vec<RollbackReplayDrill>,
    pub blockers: Vec<ReplayBlocker>,
    pub roots: Roots,
    pub counters: Counters,
    pub verdict: ReplayVerdict,
}

impl State {
    pub fn new(
        config: Config,
        wave87_checklists: Vec<Wave87ChecklistRoot>,
        pq_signer_policies: Vec<PqSignerPolicyReplay>,
        reserve_attestations: Vec<ReserveAttestationReplay>,
        amount_bucket_privacy: Vec<AmountBucketPrivacySafeguard>,
        key_rotation_readiness: Vec<KeyRotationReadinessRoot>,
        privacy_budget_signoffs: Vec<PrivacyBudgetSignoff>,
        release_captain_signoffs: Vec<ReleaseCaptainSignoff>,
        rollback_drills: Vec<RollbackReplayDrill>,
    ) -> Result<Self> {
        validate_config(&config)?;
        let blockers = collect_blockers(
            &config,
            &wave87_checklists,
            &pq_signer_policies,
            &reserve_attestations,
            &amount_bucket_privacy,
            &key_rotation_readiness,
            &privacy_budget_signoffs,
            &release_captain_signoffs,
            &rollback_drills,
        );
        let roots = build_roots(
            &wave87_checklists,
            &pq_signer_policies,
            &reserve_attestations,
            &amount_bucket_privacy,
            &key_rotation_readiness,
            &privacy_budget_signoffs,
            &release_captain_signoffs,
            &rollback_drills,
            &blockers,
            &ReplayVerdict {
                decision: DrillDecision::NoGo,
                release_allowed: false,
                fail_closed: true,
                blocker_count: blockers.len() as u64,
                hard_blocker_count: hard_blocker_count(&blockers),
                go_no_go_root: merkle_root("MONERO-L2-PQ-REPLAY-DRILL-PENDING-GO-NO-GO", &[]),
                fail_closed_root: merkle_root("MONERO-L2-PQ-REPLAY-DRILL-PENDING-FAIL-CLOSED", &[]),
            },
        );
        let verdict = build_verdict(&config, &blockers, &roots);
        let roots = build_roots(
            &wave87_checklists,
            &pq_signer_policies,
            &reserve_attestations,
            &amount_bucket_privacy,
            &key_rotation_readiness,
            &privacy_budget_signoffs,
            &release_captain_signoffs,
            &rollback_drills,
            &blockers,
            &verdict,
        );
        let counters = Counters {
            wave87_checklist_count: wave87_checklists.len() as u64,
            pq_signer_policy_count: pq_signer_policies.len() as u64,
            reserve_attestation_count: reserve_attestations.len() as u64,
            amount_bucket_policy_count: amount_bucket_privacy.len() as u64,
            key_rotation_ready_count: key_rotation_readiness
                .iter()
                .filter(|item| item.ready)
                .count() as u64,
            privacy_budget_signoff_count: privacy_budget_signoffs
                .iter()
                .filter(|item| item.accepted)
                .count() as u64,
            release_captain_signoff_count: release_captain_signoffs
                .iter()
                .filter(|item| item.accepted && item.decision == DrillDecision::Go)
                .count() as u64,
            rollback_drill_count: rollback_drills.iter().filter(|item| item.passed).count() as u64,
        };
        Ok(Self {
            config,
            wave87_checklists,
            pq_signer_policies,
            reserve_attestations,
            amount_bucket_privacy,
            key_rotation_readiness,
            privacy_budget_signoffs,
            release_captain_signoffs,
            rollback_drills,
            blockers,
            roots,
            counters,
            verdict,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave87_checklists = devnet_wave87_checklists(&config);
        let pq_signer_policies = devnet_pq_signer_policies(&config);
        let reserve_attestations = devnet_reserve_attestations(&config);
        let amount_bucket_privacy = devnet_amount_bucket_privacy(&config);
        let key_rotation_readiness = devnet_key_rotation_readiness(&config);
        let privacy_budget_signoffs = devnet_privacy_budget_signoffs(&config);
        let release_captain_signoffs = devnet_release_captain_signoffs(&config);
        let rollback_drills = devnet_rollback_drills(&config);
        match Self::new(
            config,
            wave87_checklists,
            pq_signer_policies,
            reserve_attestations,
            amount_bucket_privacy,
            key_rotation_readiness,
            privacy_budget_signoffs,
            release_captain_signoffs,
            rollback_drills,
        ) {
            Ok(state) => state,
            Err(_) => Self::fail_closed_devnet(),
        }
    }

    pub fn fail_closed_devnet() -> Self {
        let config = Config::devnet();
        let blocker = ReplayBlocker::new(
            ReplayBlockerKind::ReplayDecisionMismatch,
            BlockerSeverity::Fatal,
            "devnet construction failed closed",
            merkle_root("MONERO-L2-PQ-REPLAY-DRILL-DEVNET-FAIL-CLOSED", &[]),
        );
        let blockers = vec![blocker];
        let empty_verdict = ReplayVerdict {
            decision: DrillDecision::NoGo,
            release_allowed: false,
            fail_closed: true,
            blocker_count: 1,
            hard_blocker_count: 1,
            go_no_go_root: merkle_root("MONERO-L2-PQ-REPLAY-DRILL-DEVNET-NO-GO", &[]),
            fail_closed_root: merkle_root("MONERO-L2-PQ-REPLAY-DRILL-DEVNET-FAIL-CLOSED", &[]),
        };
        let roots = build_roots(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &blockers,
            &empty_verdict,
        );
        let verdict = build_verdict(&config, &blockers, &roots);
        Self {
            config,
            wave87_checklists: Vec::new(),
            pq_signer_policies: Vec::new(),
            reserve_attestations: Vec::new(),
            amount_bucket_privacy: Vec::new(),
            key_rotation_readiness: Vec::new(),
            privacy_budget_signoffs: Vec::new(),
            release_captain_signoffs: Vec::new(),
            rollback_drills: Vec::new(),
            blockers,
            roots,
            counters: Counters {
                wave87_checklist_count: 0,
                pq_signer_policy_count: 0,
                reserve_attestation_count: 0,
                amount_bucket_policy_count: 0,
                key_rotation_ready_count: 0,
                privacy_budget_signoff_count: 0,
                release_captain_signoff_count: 0,
                rollback_drill_count: 0,
            },
            verdict,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "blockers": self.blockers.iter().map(ReplayBlocker::public_record).collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-RESERVE-PRIVACY-RELEASE-CAPTAIN-REPLAY-DRILL-STATE",
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    pub fn validate(&self) -> Result<()> {
        validate_config(&self.config)?;
        let blockers = collect_blockers(
            &self.config,
            &self.wave87_checklists,
            &self.pq_signer_policies,
            &self.reserve_attestations,
            &self.amount_bucket_privacy,
            &self.key_rotation_readiness,
            &self.privacy_budget_signoffs,
            &self.release_captain_signoffs,
            &self.rollback_drills,
        );
        if blockers != self.blockers {
            return Err("replay drill blockers diverged from public inputs".to_string());
        }
        let verdict = build_verdict(&self.config, &self.blockers, &self.roots);
        if verdict.decision != self.verdict.decision
            || verdict.release_allowed != self.verdict.release_allowed
            || verdict.fail_closed != self.verdict.fail_closed
        {
            return Err("replay drill verdict diverged from blockers".to_string());
        }
        Ok(())
    }
}

pub fn devnet() -> State {
    State::devnet()
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("chain id mismatch".to_string());
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("schema version mismatch".to_string());
    }
    if config.min_pq_signer_weight == 0
        || config.min_pq_signer_count == 0
        || config.min_amount_bucket_count == 0
        || config.min_key_rotation_roots == 0
        || config.min_privacy_signoffs == 0
        || config.min_release_captain_signoffs == 0
    {
        return Err("replay drill thresholds must be nonzero".to_string());
    }
    if config.wave <= config.source_wave {
        return Err("replay drill wave must follow source wave".to_string());
    }
    Ok(())
}

fn collect_blockers(
    config: &Config,
    wave87_checklists: &[Wave87ChecklistRoot],
    pq_signer_policies: &[PqSignerPolicyReplay],
    reserve_attestations: &[ReserveAttestationReplay],
    amount_bucket_privacy: &[AmountBucketPrivacySafeguard],
    key_rotation_readiness: &[KeyRotationReadinessRoot],
    privacy_budget_signoffs: &[PrivacyBudgetSignoff],
    release_captain_signoffs: &[ReleaseCaptainSignoff],
    rollback_drills: &[RollbackReplayDrill],
) -> Vec<ReplayBlocker> {
    let mut blockers = Vec::new();
    collect_wave87_blockers(config, wave87_checklists, &mut blockers);
    collect_pq_blockers(config, pq_signer_policies, &mut blockers);
    collect_reserve_blockers(config, reserve_attestations, &mut blockers);
    collect_privacy_blockers(config, amount_bucket_privacy, &mut blockers);
    collect_rotation_blockers(config, key_rotation_readiness, &mut blockers);
    collect_signoff_blockers(
        config,
        privacy_budget_signoffs,
        release_captain_signoffs,
        &mut blockers,
    );
    collect_rollback_blockers(rollback_drills, &mut blockers);
    blockers
}

fn collect_wave87_blockers(
    config: &Config,
    wave87_checklists: &[Wave87ChecklistRoot],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if wave87_checklists.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::MissingWave87ChecklistRoot,
            BlockerSeverity::Fatal,
            "wave 87 checklist root is required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-WAVE87", &[]),
        ));
    }
    for checklist in wave87_checklists {
        if checklist.checklist_root.is_empty() || checklist.go_no_go_root.is_empty() {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::MissingWave87ChecklistRoot,
                BlockerSeverity::Fatal,
                "wave 87 checklist root was blank",
                checklist.state_root(),
            ));
        }
        if config
            .release_height
            .saturating_sub(checklist.accepted_height)
            > config.max_root_age_blocks
        {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::StaleWave87ChecklistRoot,
                BlockerSeverity::Hard,
                "wave 87 checklist root aged past replay window",
                checklist.state_root(),
            ));
        }
    }
}

fn collect_pq_blockers(
    config: &Config,
    pq_signer_policies: &[PqSignerPolicyReplay],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if pq_signer_policies.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::PqSignerPolicyReplayMissing,
            BlockerSeverity::Fatal,
            "pq signer policy replay is required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-PQ-POLICY", &[]),
        ));
    }
    let mut seen = BTreeSet::new();
    for policy in pq_signer_policies {
        if !seen.insert(policy.signer_policy_id.clone()) {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::DuplicateSignerPolicy,
                BlockerSeverity::Hard,
                "duplicate pq signer policy replay id",
                policy.state_root(),
            ));
        }
        if policy.signer_weight < config.min_pq_signer_weight {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::PqSignerWeightShortfall,
                BlockerSeverity::Hard,
                "pq signer policy replay weight below release threshold",
                policy.state_root(),
            ));
        }
        if policy.signer_count < config.min_pq_signer_count {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::PqSignerCountShortfall,
                BlockerSeverity::Hard,
                "pq signer policy replay signer count below release threshold",
                policy.state_root(),
            ));
        }
        if !policy.replay_matches_policy {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::PqSignerReplayRootMismatch,
                BlockerSeverity::Fatal,
                "pq signer policy replay root did not match policy root",
                policy.state_root(),
            ));
        }
    }
}

fn collect_reserve_blockers(
    config: &Config,
    reserve_attestations: &[ReserveAttestationReplay],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if reserve_attestations.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::ReserveAttestationMissing,
            BlockerSeverity::Fatal,
            "reserve attestation replay is required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-RESERVE", &[]),
        ));
    }
    for reserve in reserve_attestations {
        if reserve.reserve_coverage_bps < config.min_reserve_coverage_bps {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::ReserveCoverageShortfall,
                BlockerSeverity::Hard,
                "reserve attestation coverage below release threshold",
                reserve.state_root(),
            ));
        }
        if reserve.blocker_active {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::ReserveBlockerActive,
                BlockerSeverity::Fatal,
                "reserve attestation blocker is active",
                reserve.state_root(),
            ));
        }
    }
}

fn collect_privacy_blockers(
    config: &Config,
    amount_bucket_privacy: &[AmountBucketPrivacySafeguard],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if amount_bucket_privacy.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::AmountBucketPrivacyMissing,
            BlockerSeverity::Fatal,
            "amount bucket privacy safeguard is required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-BUCKET-PRIVACY", &[]),
        ));
    }
    for safeguard in amount_bucket_privacy {
        if safeguard.bucket_count < config.min_amount_bucket_count {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::AmountBucketCountShortfall,
                BlockerSeverity::Hard,
                "amount bucket privacy count below release threshold",
                safeguard.state_root(),
            ));
        }
        if safeguard.privacy_spend_bps > config.max_privacy_spend_bps {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::PrivacyBudgetExceeded,
                BlockerSeverity::Hard,
                "amount bucket privacy spend exceeded release budget",
                safeguard.state_root(),
            ));
        }
        if safeguard.leakage_detected {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::AmountBucketLeakage,
                BlockerSeverity::Fatal,
                "amount bucket privacy leakage detected",
                safeguard.state_root(),
            ));
        }
    }
}

fn collect_rotation_blockers(
    config: &Config,
    key_rotation_readiness: &[KeyRotationReadinessRoot],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if key_rotation_readiness.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::KeyRotationReadinessMissing,
            BlockerSeverity::Fatal,
            "key rotation readiness roots are required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-KEY-ROTATION", &[]),
        ));
    }
    let ready_count = key_rotation_readiness
        .iter()
        .filter(|item| item.ready)
        .count() as u64;
    if ready_count < config.min_key_rotation_roots {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::KeyRotationRootShortfall,
            BlockerSeverity::Hard,
            "key rotation readiness roots below release threshold",
            merkle_root(
                "MONERO-L2-PQ-REPLAY-KEY-ROTATION-ROOTS",
                &key_rotation_readiness
                    .iter()
                    .map(KeyRotationReadinessRoot::state_root)
                    .collect::<Vec<_>>(),
            ),
        ));
    }
}

fn collect_signoff_blockers(
    config: &Config,
    privacy_budget_signoffs: &[PrivacyBudgetSignoff],
    release_captain_signoffs: &[ReleaseCaptainSignoff],
    blockers: &mut Vec<ReplayBlocker>,
) {
    let privacy_count = privacy_budget_signoffs
        .iter()
        .filter(|item| item.accepted && item.approved_spend_bps <= config.max_privacy_spend_bps)
        .count() as u64;
    if privacy_count < config.min_privacy_signoffs {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::PrivacyBudgetSignoffMissing,
            BlockerSeverity::Hard,
            "privacy budget signoffs below release threshold",
            merkle_root(
                "MONERO-L2-PQ-REPLAY-PRIVACY-SIGNOFFS",
                &privacy_budget_signoffs
                    .iter()
                    .map(PrivacyBudgetSignoff::state_root)
                    .collect::<Vec<_>>(),
            ),
        ));
    }
    let mut captains = BTreeSet::new();
    let mut accepted_go_count = 0_u64;
    for signoff in release_captain_signoffs {
        if !captains.insert(signoff.captain_id.clone()) {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::DuplicateReleaseCaptain,
                BlockerSeverity::Hard,
                "duplicate release captain signoff id",
                signoff.state_root(),
            ));
        }
        if signoff.accepted && signoff.decision == DrillDecision::Go {
            accepted_go_count = accepted_go_count.saturating_add(1);
        }
        if signoff.accepted && signoff.decision == DrillDecision::NoGo {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::ReplayDecisionMismatch,
                BlockerSeverity::Fatal,
                "release captain recorded no-go",
                signoff.state_root(),
            ));
        }
    }
    if accepted_go_count < config.min_release_captain_signoffs {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::ReleaseCaptainSignoffMissing,
            BlockerSeverity::Fatal,
            "release captain go signoffs below release threshold",
            merkle_root(
                "MONERO-L2-PQ-REPLAY-CAPTAIN-SIGNOFFS",
                &release_captain_signoffs
                    .iter()
                    .map(ReleaseCaptainSignoff::state_root)
                    .collect::<Vec<_>>(),
            ),
        ));
    }
}

fn collect_rollback_blockers(
    rollback_drills: &[RollbackReplayDrill],
    blockers: &mut Vec<ReplayBlocker>,
) {
    if rollback_drills.is_empty() {
        blockers.push(ReplayBlocker::new(
            ReplayBlockerKind::RollbackDrillMissing,
            BlockerSeverity::Fatal,
            "rollback replay drill is required",
            merkle_root("MONERO-L2-PQ-REPLAY-MISSING-ROLLBACK", &[]),
        ));
    }
    for drill in rollback_drills {
        if !drill.passed {
            blockers.push(ReplayBlocker::new(
                ReplayBlockerKind::RollbackDrillFailed,
                BlockerSeverity::Fatal,
                "rollback replay drill failed",
                drill.state_root(),
            ));
        }
    }
}

fn build_verdict(config: &Config, blockers: &[ReplayBlocker], roots: &Roots) -> ReplayVerdict {
    let hard_count = hard_blocker_count(blockers);
    let any_blocker = !blockers.is_empty();
    let replay_mismatch = blockers
        .iter()
        .any(|blocker| blocker.kind == ReplayBlockerKind::ReplayDecisionMismatch);
    let fail_closed = hard_count > 0
        || (config.fail_closed_on_any_blocker && any_blocker)
        || (config.fail_closed_on_replay_mismatch && replay_mismatch);
    let release_allowed = !any_blocker && !fail_closed;
    let decision = if release_allowed {
        DrillDecision::Go
    } else {
        DrillDecision::NoGo
    };
    let go_no_go_root = domain_hash(
        "MONERO-L2-PQ-REPLAY-DRILL-GO-NO-GO",
        &[
            HashPart::Str(decision.as_str()),
            HashPart::Str(bool_str(release_allowed)),
            HashPart::Str(&roots.blocker_root),
        ],
        32,
    );
    let fail_closed_root = domain_hash(
        "MONERO-L2-PQ-REPLAY-DRILL-FAIL-CLOSED",
        &[
            HashPart::Str(bool_str(fail_closed)),
            HashPart::Str(&roots.blocker_root),
            HashPart::Str(&roots.rollback_replay_drill_root),
        ],
        32,
    );
    ReplayVerdict {
        decision,
        release_allowed,
        fail_closed,
        blocker_count: blockers.len() as u64,
        hard_blocker_count: hard_count,
        go_no_go_root,
        fail_closed_root,
    }
}

fn build_roots(
    wave87_checklists: &[Wave87ChecklistRoot],
    pq_signer_policies: &[PqSignerPolicyReplay],
    reserve_attestations: &[ReserveAttestationReplay],
    amount_bucket_privacy: &[AmountBucketPrivacySafeguard],
    key_rotation_readiness: &[KeyRotationReadinessRoot],
    privacy_budget_signoffs: &[PrivacyBudgetSignoff],
    release_captain_signoffs: &[ReleaseCaptainSignoff],
    rollback_drills: &[RollbackReplayDrill],
    blockers: &[ReplayBlocker],
    verdict: &ReplayVerdict,
) -> Roots {
    Roots {
        wave87_checklist_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-WAVE87-CHECKLIST",
            wave87_checklists,
            Wave87ChecklistRoot::public_record,
        ),
        pq_signer_policy_replay_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-PQ-SIGNER-POLICY",
            pq_signer_policies,
            PqSignerPolicyReplay::public_record,
        ),
        reserve_attestation_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-RESERVE-ATTESTATION",
            reserve_attestations,
            ReserveAttestationReplay::public_record,
        ),
        amount_bucket_privacy_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-AMOUNT-BUCKET-PRIVACY",
            amount_bucket_privacy,
            AmountBucketPrivacySafeguard::public_record,
        ),
        key_rotation_readiness_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-KEY-ROTATION-READINESS",
            key_rotation_readiness,
            KeyRotationReadinessRoot::public_record,
        ),
        privacy_budget_signoff_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-PRIVACY-BUDGET-SIGNOFF",
            privacy_budget_signoffs,
            PrivacyBudgetSignoff::public_record,
        ),
        release_captain_signoff_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-RELEASE-CAPTAIN-SIGNOFF",
            release_captain_signoffs,
            ReleaseCaptainSignoff::public_record,
        ),
        rollback_replay_drill_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-ROLLBACK-DRILL",
            rollback_drills,
            RollbackReplayDrill::public_record,
        ),
        blocker_root: merkle_from_records(
            "MONERO-L2-PQ-REPLAY-BLOCKER",
            blockers,
            ReplayBlocker::public_record,
        ),
        verdict_root: record_root("verdict", &verdict.public_record()),
    }
}

fn hard_blocker_count(blockers: &[ReplayBlocker]) -> u64 {
    blockers
        .iter()
        .filter(|blocker| blocker.severity.fail_closed())
        .count() as u64
}

fn merkle_from_records<T, F>(domain: &str, values: &[T], public_record: F) -> String
where
    F: Fn(&T) -> Value,
{
    let records = values
        .iter()
        .map(|value| domain_hash(domain, &[HashPart::Json(&public_record(value))], 32))
        .collect::<Vec<_>>();
    merkle_root(domain, &records)
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-RESERVE-PRIVACY-RELEASE-CAPTAIN-REPLAY-DRILL-RECORD",
        &[HashPart::Str(label), HashPart::Json(record)],
        32,
    )
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn devnet_root(config: &Config, label: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-REPLAY-DRILL-DEVNET-ROOT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn devnet_wave87_checklists(config: &Config) -> Vec<Wave87ChecklistRoot> {
    [
        "runtime_replay",
        "bridge_custody",
        "wallet_watchtower",
        "pq_reserve_privacy",
    ]
    .iter()
    .enumerate()
    .map(|(index, lane)| {
        let ordinal = index as u64 + 1;
        Wave87ChecklistRoot {
            lane: (*lane).to_string(),
            checklist_root: devnet_root(config, "wave87-checklist", ordinal),
            accepted_height: config.release_height.saturating_sub(24 + ordinal),
            item_count: 9,
            go_no_go_root: devnet_root(config, "wave87-go-no-go", ordinal),
        }
    })
    .collect()
}

fn devnet_pq_signer_policies(config: &Config) -> Vec<PqSignerPolicyReplay> {
    vec![PqSignerPolicyReplay {
        signer_policy_id: "pq-policy-replay-a".to_string(),
        policy_root: devnet_root(config, "pq-policy", 1),
        replay_root: devnet_root(config, "pq-policy-replay", 1),
        signer_weight: 72,
        signer_count: 5,
        key_rotation_readiness_root: devnet_root(config, "pq-key-rotation", 1),
        replay_matches_policy: true,
    }]
}

fn devnet_reserve_attestations(config: &Config) -> Vec<ReserveAttestationReplay> {
    vec![ReserveAttestationReplay {
        reserve_attestation_id: "reserve-attestation-replay-a".to_string(),
        reserve_attestation_root: devnet_root(config, "reserve-attestation", 1),
        reserve_coverage_bps: 10_800,
        oracle_count: 4,
        blocker_active: false,
        accepted_live_evidence_root: devnet_root(config, "accepted-live-evidence", 1),
    }]
}

fn devnet_amount_bucket_privacy(config: &Config) -> Vec<AmountBucketPrivacySafeguard> {
    vec![AmountBucketPrivacySafeguard {
        bucket_set_id: "amount-bucket-privacy-a".to_string(),
        bucket_policy_root: devnet_root(config, "bucket-policy", 1),
        bucket_count: 12,
        privacy_spend_bps: 4_200,
        non_linkage_root: devnet_root(config, "non-linkage", 1),
        leakage_detected: false,
    }]
}

fn devnet_key_rotation_readiness(config: &Config) -> Vec<KeyRotationReadinessRoot> {
    (1..=3)
        .map(|ordinal| KeyRotationReadinessRoot {
            rotation_id: format!("key-rotation-readiness-{ordinal}"),
            readiness_root: devnet_root(config, "rotation-readiness", ordinal),
            next_epoch: config.wave + ordinal,
            activation_guard_root: devnet_root(config, "activation-guard", ordinal),
            rollback_guard_root: devnet_root(config, "rollback-guard", ordinal),
            ready: true,
        })
        .collect()
}

fn devnet_privacy_budget_signoffs(config: &Config) -> Vec<PrivacyBudgetSignoff> {
    ["privacy-review-a", "privacy-review-b", "privacy-review-c"]
        .iter()
        .enumerate()
        .map(|(index, reviewer)| {
            let ordinal = index as u64 + 1;
            PrivacyBudgetSignoff {
                reviewer_id: (*reviewer).to_string(),
                signoff_root: devnet_root(config, "privacy-signoff", ordinal),
                privacy_budget_root: devnet_root(config, "privacy-budget", ordinal),
                approved_spend_bps: 4_500,
                accepted: true,
            }
        })
        .collect()
}

fn devnet_release_captain_signoffs(config: &Config) -> Vec<ReleaseCaptainSignoff> {
    ["release-captain-a", "release-captain-b"]
        .iter()
        .enumerate()
        .map(|(index, captain)| {
            let ordinal = index as u64 + 1;
            ReleaseCaptainSignoff {
                captain_id: (*captain).to_string(),
                signoff_root: devnet_root(config, "captain-signoff", ordinal),
                release_policy_root: devnet_root(config, "release-policy", ordinal),
                rollback_drill_root: devnet_root(config, "captain-rollback", ordinal),
                decision: DrillDecision::Go,
                accepted: true,
            }
        })
        .collect()
}

fn devnet_rollback_drills(config: &Config) -> Vec<RollbackReplayDrill> {
    vec![RollbackReplayDrill {
        drill_id: "rollback-replay-drill-a".to_string(),
        replay_drill_root: devnet_root(config, "rollback-replay", 1),
        deployment_guard_root: devnet_root(config, "deployment-guard", 1),
        rollback_evidence_root: devnet_root(config, "rollback-evidence", 1),
        operator_dashboard_root: devnet_root(config, "operator-dashboard", 1),
        passed: true,
    }]
}

pub fn summarize_blockers(blockers: &[ReplayBlocker]) -> BTreeMap<String, u64> {
    let mut counts = BTreeMap::new();
    for blocker in blockers {
        let counter = counts.entry(blocker.kind.as_str().to_string()).or_insert(0);
        *counter += 1;
    }
    counts
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}
