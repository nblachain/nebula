use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_cross_receipt_consistency_runtime as consistency,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageUserEscapeVerdictBundleRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_USER_ESCAPE_VERDICT_BUNDLE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-user-escape-verdict-bundle-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_USER_ESCAPE_VERDICT_BUNDLE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const USER_ESCAPE_VERDICT_BUNDLE_SUITE: &str =
    "monero-l2-pq-force-exit-package-user-escape-verdict-bundle-v1";
pub const DEFAULT_MIN_READY_LANES: u64 = 7;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub verdict_suite: String,
    pub min_ready_lanes: u64,
    pub require_cross_receipt_consistency: bool,
    pub require_wallet_ready: bool,
    pub require_recovery_answerable: bool,
    pub require_release_hold_clearance: bool,
    pub hold_production_until_user_escape_live: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            verdict_suite: USER_ESCAPE_VERDICT_BUNDLE_SUITE.to_string(),
            min_ready_lanes: DEFAULT_MIN_READY_LANES,
            require_cross_receipt_consistency: true,
            require_wallet_ready: true,
            require_recovery_answerable: true,
            require_release_hold_clearance: true,
            hold_production_until_user_escape_live: true,
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
            "verdict_suite": self.verdict_suite,
            "min_ready_lanes": self.min_ready_lanes,
            "require_cross_receipt_consistency": self.require_cross_receipt_consistency,
            "require_wallet_ready": self.require_wallet_ready,
            "require_recovery_answerable": self.require_recovery_answerable,
            "require_release_hold_clearance": self.require_release_hold_clearance,
            "hold_production_until_user_escape_live": self.hold_production_until_user_escape_live,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserEscapeStepKind {
    VerifyCrossReceiptConsistency,
    ReadSettlementVerdict,
    CheckChallengeWindow,
    ConfirmReserveFallback,
    VerifyPqAuthority,
    ScanWalletPrivacy,
    LoadRecoveryPlaybook,
    CheckReleaseHoldClearance,
}

impl UserEscapeStepKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VerifyCrossReceiptConsistency => "verify_cross_receipt_consistency",
            Self::ReadSettlementVerdict => "read_settlement_verdict",
            Self::CheckChallengeWindow => "check_challenge_window",
            Self::ConfirmReserveFallback => "confirm_reserve_fallback",
            Self::VerifyPqAuthority => "verify_pq_authority",
            Self::ScanWalletPrivacy => "scan_wallet_privacy",
            Self::LoadRecoveryPlaybook => "load_recovery_playbook",
            Self::CheckReleaseHoldClearance => "check_release_hold_clearance",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserEscapeStepStatus {
    Ready,
    WaitingForEvidence,
    Blocked,
}

impl UserEscapeStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::WaitingForEvidence => "waiting_for_evidence",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub consistency_state_root: String,
    pub consistency_record_root: String,
    pub dependency_bundle_root: String,
    pub contradiction_bundle_root: String,
    pub production_hold_root: String,
    pub consistency_verdict_root: String,
    pub lane_count: u64,
    pub consistent_count: u64,
    pub missing_count: u64,
    pub contradictory_count: u64,
    pub release_held_count: u64,
    pub production_blocker_count: u64,
    pub all_lanes_consistent: bool,
    pub user_escape_consistent: bool,
    pub production_release_consistent: bool,
    pub settlement_ready: bool,
    pub challenge_ready: bool,
    pub reserve_ready: bool,
    pub pq_ready: bool,
    pub wallet_ready: bool,
    pub recovery_ready: bool,
    pub clearance_ready: bool,
}

impl SourceBundle {
    pub fn from_consistency(state: &consistency::State) -> Self {
        Self {
            consistency_state_root: state.state_root(),
            consistency_record_root: state.consistency_record_root.clone(),
            dependency_bundle_root: state.dependency_bundle_root.clone(),
            contradiction_bundle_root: state.contradiction_bundle_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            consistency_verdict_root: state.verdict.verdict_root.clone(),
            lane_count: state.verdict.lane_count,
            consistent_count: state.verdict.consistent_count,
            missing_count: state.verdict.missing_count,
            contradictory_count: state.verdict.contradictory_count,
            release_held_count: state.verdict.release_held_count,
            production_blocker_count: state.verdict.production_blocker_count,
            all_lanes_consistent: state.verdict.all_lanes_consistent,
            user_escape_consistent: state.verdict.user_escape_consistent,
            production_release_consistent: state.verdict.production_release_consistent,
            settlement_ready: state.source.settlement_ready,
            challenge_ready: state.source.challenge_ready,
            reserve_ready: state.source.reserve_ready,
            pq_ready: state.source.pq_ready,
            wallet_ready: state.source.wallet_ready,
            recovery_ready: state.source.recovery_ready,
            clearance_ready: state.source.clearance_ready,
        }
    }

    pub fn devnet() -> Self {
        Self::from_consistency(&consistency::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "consistency_state_root": self.consistency_state_root,
            "consistency_record_root": self.consistency_record_root,
            "dependency_bundle_root": self.dependency_bundle_root,
            "contradiction_bundle_root": self.contradiction_bundle_root,
            "production_hold_root": self.production_hold_root,
            "consistency_verdict_root": self.consistency_verdict_root,
            "lane_count": self.lane_count,
            "consistent_count": self.consistent_count,
            "missing_count": self.missing_count,
            "contradictory_count": self.contradictory_count,
            "release_held_count": self.release_held_count,
            "production_blocker_count": self.production_blocker_count,
            "all_lanes_consistent": self.all_lanes_consistent,
            "user_escape_consistent": self.user_escape_consistent,
            "production_release_consistent": self.production_release_consistent,
            "settlement_ready": self.settlement_ready,
            "challenge_ready": self.challenge_ready,
            "reserve_ready": self.reserve_ready,
            "pq_ready": self.pq_ready,
            "wallet_ready": self.wallet_ready,
            "recovery_ready": self.recovery_ready,
            "clearance_ready": self.clearance_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserEscapeStep {
    pub step_kind: UserEscapeStepKind,
    pub required_root: String,
    pub wallet_instruction_root: String,
    pub blocker_root: String,
    pub status: UserEscapeStepStatus,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub instruction: String,
}

impl UserEscapeStep {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        step_kind: UserEscapeStepKind,
        ready: bool,
        required_root: String,
        blocks_production: bool,
    ) -> Self {
        let status = if ready {
            UserEscapeStepStatus::Ready
        } else if blocks_production || source.contradictory_count > 0 {
            UserEscapeStepStatus::Blocked
        } else {
            UserEscapeStepStatus::WaitingForEvidence
        };
        let blocks_user_release = status != UserEscapeStepStatus::Ready;
        let wallet_instruction_root =
            wallet_instruction_root(config, source, step_kind, status, &required_root);
        let blocker_root = blocker_root(config, source, step_kind, status, blocks_user_release);
        let instruction = wallet_instruction(step_kind, status).to_string();
        Self {
            step_kind,
            required_root,
            wallet_instruction_root,
            blocker_root,
            status,
            blocks_user_release,
            blocks_production,
            instruction,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "step_kind": self.step_kind.as_str(),
            "required_root": self.required_root,
            "wallet_instruction_root": self.wallet_instruction_root,
            "blocker_root": self.blocker_root,
            "status": self.status.as_str(),
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "instruction": self.instruction,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserEscapeVerdict {
    pub step_count: u64,
    pub ready_step_count: u64,
    pub waiting_step_count: u64,
    pub blocked_step_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub wallet_escape_answerable: bool,
    pub force_exit_safe_to_attempt: bool,
    pub production_release_allowed: bool,
    pub verdict_status: String,
    pub wallet_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl UserEscapeVerdict {
    pub fn new(config: &Config, source: &SourceBundle, steps: &[UserEscapeStep]) -> Self {
        let step_count = steps.len() as u64;
        let ready_step_count = count_status(steps, UserEscapeStepStatus::Ready);
        let waiting_step_count = count_status(steps, UserEscapeStepStatus::WaitingForEvidence);
        let blocked_step_count = count_status(steps, UserEscapeStepStatus::Blocked);
        let user_release_blocker_count =
            steps.iter().filter(|step| step.blocks_user_release).count() as u64;
        let production_blocker_count =
            steps.iter().filter(|step| step.blocks_production).count() as u64;
        let wallet_escape_answerable = step_count >= config.min_ready_lanes
            && ready_step_count >= config.min_ready_lanes
            && user_release_blocker_count == 0
            && (!config.require_cross_receipt_consistency || source.user_escape_consistent)
            && (!config.require_wallet_ready || source.wallet_ready)
            && (!config.require_recovery_answerable || source.recovery_ready)
            && (!config.require_release_hold_clearance || source.clearance_ready);
        let force_exit_safe_to_attempt = wallet_escape_answerable
            && source.missing_count == 0
            && source.contradictory_count == 0;
        let production_release_allowed = force_exit_safe_to_attempt
            && source.production_release_consistent
            && !config.hold_production_until_user_escape_live
            && production_blocker_count == 0;
        let verdict_status = if blocked_step_count > 0 || source.contradictory_count > 0 {
            "blocked"
        } else if waiting_step_count > 0 || source.release_held_count > 0 {
            "waiting_for_live_evidence"
        } else if force_exit_safe_to_attempt {
            "wallet_escape_answerable"
        } else {
            "incomplete"
        }
        .to_string();
        let wallet_answer = if force_exit_safe_to_attempt {
            "wallet has a consistent forced-exit answer bundle for user review"
        } else {
            "wallet must keep forced-exit answer held until every receipt lane is live and consistent"
        }
        .to_string();
        let production_answer = if production_release_allowed {
            "production release can be reviewed after live user-escape execution evidence"
        } else {
            "production release remains held; user-escape verdict is not a production release permit"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            step_count,
            ready_step_count,
            waiting_step_count,
            blocked_step_count,
            user_release_blocker_count,
            production_blocker_count,
            wallet_escape_answerable,
            force_exit_safe_to_attempt,
            production_release_allowed,
            &verdict_status,
            &wallet_answer,
            &production_answer,
        );
        Self {
            step_count,
            ready_step_count,
            waiting_step_count,
            blocked_step_count,
            user_release_blocker_count,
            production_blocker_count,
            wallet_escape_answerable,
            force_exit_safe_to_attempt,
            production_release_allowed,
            verdict_status,
            wallet_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "step_count": self.step_count,
            "ready_step_count": self.ready_step_count,
            "waiting_step_count": self.waiting_step_count,
            "blocked_step_count": self.blocked_step_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "wallet_escape_answerable": self.wallet_escape_answerable,
            "force_exit_safe_to_attempt": self.force_exit_safe_to_attempt,
            "production_release_allowed": self.production_release_allowed,
            "verdict_status": self.verdict_status,
            "wallet_answer": self.wallet_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub steps: Vec<UserEscapeStep>,
    pub verdict: UserEscapeVerdict,
    pub step_bundle_root: String,
    pub wallet_instruction_bundle_root: String,
    pub release_blocker_bundle_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let steps = user_escape_steps(&config, &source);
        let verdict = UserEscapeVerdict::new(&config, &source, &steps);
        let step_bundle_root = step_bundle_root(&steps);
        let wallet_instruction_bundle_root =
            wallet_instruction_bundle_root(&config, &source, &steps, &verdict);
        let release_blocker_bundle_root =
            release_blocker_bundle_root(&config, &source, &steps, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &steps, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &step_bundle_root,
            &wallet_instruction_bundle_root,
            &release_blocker_bundle_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            steps,
            verdict,
            step_bundle_root,
            wallet_instruction_bundle_root,
            release_blocker_bundle_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), SourceBundle::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_user_escape_verdict_bundle_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "step_bundle_root": self.step_bundle_root,
            "wallet_instruction_bundle_root": self.wallet_instruction_bundle_root,
            "release_blocker_bundle_root": self.release_blocker_bundle_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "steps": self
                .steps
                .iter()
                .map(UserEscapeStep::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
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

fn user_escape_steps(config: &Config, source: &SourceBundle) -> Vec<UserEscapeStep> {
    vec![
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::VerifyCrossReceiptConsistency,
            !config.require_cross_receipt_consistency || source.user_escape_consistent,
            source.consistency_record_root.clone(),
            source.production_blocker_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::ReadSettlementVerdict,
            source.settlement_ready,
            source.dependency_bundle_root.clone(),
            source.production_blocker_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::CheckChallengeWindow,
            source.challenge_ready,
            source.contradiction_bundle_root.clone(),
            source.contradictory_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::ConfirmReserveFallback,
            source.reserve_ready,
            source.production_hold_root.clone(),
            source.production_blocker_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::VerifyPqAuthority,
            source.pq_ready,
            source.consistency_verdict_root.clone(),
            source.contradictory_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::ScanWalletPrivacy,
            !config.require_wallet_ready || source.wallet_ready,
            source.consistency_state_root.clone(),
            false,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::LoadRecoveryPlaybook,
            !config.require_recovery_answerable || source.recovery_ready,
            source.dependency_bundle_root.clone(),
            source.release_held_count > 0,
        ),
        UserEscapeStep::new(
            config,
            source,
            UserEscapeStepKind::CheckReleaseHoldClearance,
            !config.require_release_hold_clearance || source.clearance_ready,
            source.production_hold_root.clone(),
            true,
        ),
    ]
}

fn wallet_instruction_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: UserEscapeStepKind,
    status: UserEscapeStepStatus,
    required_root: &str,
) -> String {
    record_root(
        "wallet-instruction",
        &json!({
            "config_root": config.state_root(),
            "source_root": source.state_root(),
            "step_kind": step_kind.as_str(),
            "status": status.as_str(),
            "required_root": required_root,
            "instruction": wallet_instruction(step_kind, status),
        }),
    )
}

fn blocker_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: UserEscapeStepKind,
    status: UserEscapeStepStatus,
    blocks_user_release: bool,
) -> String {
    record_root(
        "release-blocker",
        &json!({
            "config_root": config.state_root(),
            "source_root": source.state_root(),
            "step_kind": step_kind.as_str(),
            "status": status.as_str(),
            "blocks_user_release": blocks_user_release,
        }),
    )
}

fn step_bundle_root(steps: &[UserEscapeStep]) -> String {
    let leaves = steps
        .iter()
        .map(UserEscapeStep::public_record)
        .collect::<Vec<_>>();
    merkle_root("MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-STEPS", &leaves)
}

fn wallet_instruction_bundle_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[UserEscapeStep],
    verdict: &UserEscapeVerdict,
) -> String {
    let leaves = steps
        .iter()
        .map(|step| {
            json!({
                "step_kind": step.step_kind.as_str(),
                "wallet_instruction_root": step.wallet_instruction_root,
                "instruction": step.instruction,
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-WALLET-INSTRUCTION-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&verdict.verdict_root),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-WALLET-INSTRUCTION-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn release_blocker_bundle_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[UserEscapeStep],
    verdict: &UserEscapeVerdict,
) -> String {
    let leaves = steps
        .iter()
        .filter(|step| step.blocks_user_release)
        .map(UserEscapeStep::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-BLOCKER-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(verdict.user_release_blocker_count),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-BLOCKER-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[UserEscapeStep],
    verdict: &UserEscapeVerdict,
) -> String {
    let leaves = steps
        .iter()
        .filter(|step| step.blocks_production)
        .map(UserEscapeStep::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(bool_str(verdict.production_release_allowed)),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-PRODUCTION-HOLD-LEAVES",
                &leaves,
            )),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    step_bundle_root: &str,
    wallet_instruction_bundle_root: &str,
    release_blocker_bundle_root: &str,
    production_hold_root: &str,
    verdict: &UserEscapeVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-VERDICT-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&source.state_root()),
            HashPart::Str(step_bundle_root),
            HashPart::Str(wallet_instruction_bundle_root),
            HashPart::Str(release_blocker_bundle_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    step_count: u64,
    ready_step_count: u64,
    waiting_step_count: u64,
    blocked_step_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    wallet_escape_answerable: bool,
    force_exit_safe_to_attempt: bool,
    production_release_allowed: bool,
    verdict_status: &str,
    wallet_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.state_root()),
            HashPart::U64(step_count),
            HashPart::U64(ready_step_count),
            HashPart::U64(waiting_step_count),
            HashPart::U64(blocked_step_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(wallet_escape_answerable)),
            HashPart::Str(bool_str(force_exit_safe_to_attempt)),
            HashPart::Str(bool_str(production_release_allowed)),
            HashPart::Str(verdict_status),
            HashPart::Str(wallet_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(steps: &[UserEscapeStep], status: UserEscapeStepStatus) -> u64 {
    steps.iter().filter(|step| step.status == status).count() as u64
}

fn wallet_instruction(step_kind: UserEscapeStepKind, status: UserEscapeStepStatus) -> &'static str {
    match status {
        UserEscapeStepStatus::Ready => "retain this root in the wallet escape bundle",
        UserEscapeStepStatus::WaitingForEvidence => "wait for live evidence before force exit",
        UserEscapeStepStatus::Blocked => match step_kind {
            UserEscapeStepKind::CheckReleaseHoldClearance => {
                "release hold is not clear; keep fail-closed recovery active"
            }
            UserEscapeStepKind::VerifyCrossReceiptConsistency => {
                "receipt lanes disagree; do not submit the escape package"
            }
            _ => "required escape evidence is blocked",
        },
    }
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id.is_empty() {
        return Err("chain id is required".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("unexpected user escape verdict protocol version".to_string());
    }
    if config.min_ready_lanes == 0 {
        return Err("at least one ready lane is required".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.consistency_state_root.is_empty() {
        return Err("consistency state root is required".to_string());
    }
    if source.consistency_verdict_root.is_empty() {
        return Err("consistency verdict root is required".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle::devnet();
    let step = UserEscapeStep {
        step_kind: UserEscapeStepKind::CheckReleaseHoldClearance,
        required_root: source.production_hold_root.clone(),
        wallet_instruction_root: record_root(
            "fallback-wallet-instruction",
            &json!({"reason": &reason}),
        ),
        blocker_root: record_root("fallback-blocker", &json!({"reason": &reason})),
        status: UserEscapeStepStatus::Blocked,
        blocks_user_release: true,
        blocks_production: true,
        instruction: reason,
    };
    let steps = vec![step];
    let verdict = UserEscapeVerdict::new(&config, &source, &steps);
    let step_bundle_root = step_bundle_root(&steps);
    let wallet_instruction_bundle_root =
        wallet_instruction_bundle_root(&config, &source, &steps, &verdict);
    let release_blocker_bundle_root =
        release_blocker_bundle_root(&config, &source, &steps, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &steps, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &step_bundle_root,
        &wallet_instruction_bundle_root,
        &release_blocker_bundle_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        steps,
        verdict,
        step_bundle_root,
        wallet_instruction_bundle_root,
        release_blocker_bundle_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-USER-ESCAPE-VERDICT-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
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
