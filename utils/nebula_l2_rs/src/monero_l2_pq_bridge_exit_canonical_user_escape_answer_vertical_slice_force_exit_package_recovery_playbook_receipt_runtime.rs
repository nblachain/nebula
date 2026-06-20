use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution_receipt,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRecoveryPlaybookReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RECOVERY_PLAYBOOK_RECEIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-recovery-playbook-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RECOVERY_PLAYBOOK_RECEIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const RECOVERY_PLAYBOOK_RECEIPT_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-recovery-playbook-receipt-v1";
pub const DEFAULT_MIN_RECOVERY_STEPS: u64 = 7;
pub const DEFAULT_DEADLINE_GRACE_BLOCKS: u64 = 144;
pub const DEFAULT_SETTLEMENT_DEADLINE_BLOCKS: u64 = 720;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub playbook_suite: String,
    pub min_recovery_steps: u64,
    pub deadline_grace_blocks: u64,
    pub settlement_deadline_blocks: u64,
    pub require_wallet_instructions: bool,
    pub require_evidence_roots: bool,
    pub require_deadline_receipts: bool,
    pub require_fail_closed_holds: bool,
    pub hold_production_until_recovery_settled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            playbook_suite: RECOVERY_PLAYBOOK_RECEIPT_SUITE.to_string(),
            min_recovery_steps: DEFAULT_MIN_RECOVERY_STEPS,
            deadline_grace_blocks: DEFAULT_DEADLINE_GRACE_BLOCKS,
            settlement_deadline_blocks: DEFAULT_SETTLEMENT_DEADLINE_BLOCKS,
            require_wallet_instructions: true,
            require_evidence_roots: true,
            require_deadline_receipts: true,
            require_fail_closed_holds: true,
            hold_production_until_recovery_settled: true,
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
            "playbook_suite": self.playbook_suite,
            "min_recovery_steps": self.min_recovery_steps,
            "deadline_grace_blocks": self.deadline_grace_blocks,
            "settlement_deadline_blocks": self.settlement_deadline_blocks,
            "require_wallet_instructions": self.require_wallet_instructions,
            "require_evidence_roots": self.require_evidence_roots,
            "require_deadline_receipts": self.require_deadline_receipts,
            "require_fail_closed_holds": self.require_fail_closed_holds,
            "hold_production_until_recovery_settled": self.hold_production_until_recovery_settled,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStepKind {
    PreserveWalletTranscript,
    PinEvidenceBundle,
    VerifyPqAuthorization,
    RebuildSubmission,
    WatchChallengeDeadline,
    ClaimReserveFallback,
    EscalateFailClosedHold,
    RequestReleaseReview,
    ArchiveRecoveryReceipt,
}

impl RecoveryStepKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::PreserveWalletTranscript,
            Self::PinEvidenceBundle,
            Self::VerifyPqAuthorization,
            Self::RebuildSubmission,
            Self::WatchChallengeDeadline,
            Self::ClaimReserveFallback,
            Self::EscalateFailClosedHold,
            Self::RequestReleaseReview,
            Self::ArchiveRecoveryReceipt,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreserveWalletTranscript => "preserve_wallet_transcript",
            Self::PinEvidenceBundle => "pin_evidence_bundle",
            Self::VerifyPqAuthorization => "verify_pq_authorization",
            Self::RebuildSubmission => "rebuild_submission",
            Self::WatchChallengeDeadline => "watch_challenge_deadline",
            Self::ClaimReserveFallback => "claim_reserve_fallback",
            Self::EscalateFailClosedHold => "escalate_fail_closed_hold",
            Self::RequestReleaseReview => "request_release_review",
            Self::ArchiveRecoveryReceipt => "archive_recovery_receipt",
        }
    }

    pub fn wallet_instruction(self) -> &'static str {
        match self {
            Self::PreserveWalletTranscript => {
                "keep the wallet transcript, local scan notes, and unsigned claim material intact"
            }
            Self::PinEvidenceBundle => {
                "pin the operator-independent evidence roots before attempting rebroadcast"
            }
            Self::VerifyPqAuthorization => {
                "verify the PQ withdrawal authorization against the saved command root"
            }
            Self::RebuildSubmission => {
                "rebuild the force-exit submission from the wallet bundle and observed receipts"
            }
            Self::WatchChallengeDeadline => {
                "watch the challenge and settlement deadlines before releasing wallet state"
            }
            Self::ClaimReserveFallback => {
                "prepare reserve fallback evidence while settlement remains unsettled"
            }
            Self::EscalateFailClosedHold => {
                "treat missing execution settlement as fail-closed and keep production held"
            }
            Self::RequestReleaseReview => {
                "request release review only after all recovery evidence roots are present"
            }
            Self::ArchiveRecoveryReceipt => {
                "archive the recovery receipt root for later replay or dispute review"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStepStatus {
    ReadyForWallet,
    WaitingForSettlement,
    HeldFailClosed,
    BlockedByMissingEvidence,
}

impl RecoveryStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForWallet => "ready_for_wallet",
            Self::WaitingForSettlement => "waiting_for_settlement",
            Self::HeldFailClosed => "held_fail_closed",
            Self::BlockedByMissingEvidence => "blocked_by_missing_evidence",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_state_root: String,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub challenge_settlement_bundle_root: String,
    pub pq_privacy_receipt_root: String,
    pub recovery_receipt_root: String,
    pub execution_production_hold_root: String,
    pub execution_status: String,
    pub execution_user_escape_answer: String,
    pub execution_production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub package_execution_observed: bool,
    pub user_escape_execution_observed: bool,
    pub execution_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_execution(state: &execution_receipt::State) -> Self {
        Self {
            execution_state_root: state.state_root(),
            execution_receipt_root: state.execution_receipt_root.clone(),
            observed_submission_bundle_root: state.observed_submission_bundle_root.clone(),
            challenge_settlement_bundle_root: state.challenge_settlement_bundle_root.clone(),
            pq_privacy_receipt_root: state.pq_privacy_receipt_root.clone(),
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            execution_production_hold_root: state.production_hold_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            execution_user_escape_answer: state.verdict.user_escape_answer.clone(),
            execution_production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            deferred_receipt_count: state.verdict.deferred_receipt_count,
            release_held_count: state.verdict.release_held_count,
            fail_closed_count: state.verdict.fail_closed_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            package_execution_observed: state.verdict.package_execution_observed,
            user_escape_execution_observed: state.verdict.user_escape_execution_observed,
            execution_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let state = execution_receipt::devnet();
        Self::from_execution(&state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "pq_privacy_receipt_root": self.pq_privacy_receipt_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "execution_production_hold_root": self.execution_production_hold_root,
            "execution_status": self.execution_status,
            "execution_user_escape_answer": self.execution_user_escape_answer,
            "execution_production_answer": self.execution_production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecoveryPlaybookStep {
    pub step_id: String,
    pub ordinal: u64,
    pub step_kind: RecoveryStepKind,
    pub wallet_instruction: String,
    pub evidence_root: String,
    pub deadline_root: String,
    pub hold_root: String,
    pub receipt_root: String,
    pub status: RecoveryStepStatus,
    pub requires_wallet_action: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub fail_closed_required: bool,
    pub required_outcome: String,
}

impl RecoveryPlaybookStep {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        step_kind: RecoveryStepKind,
        ordinal: u64,
    ) -> Self {
        let status = step_status(source, step_kind);
        let evidence_root = evidence_root(config, source, step_kind, ordinal);
        let deadline_root = deadline_root(config, source, step_kind, ordinal, status);
        let fail_closed_required = fail_closed_required(source, status);
        let blocks_user_release = source.user_release_blocker_count > 0
            || status == RecoveryStepStatus::HeldFailClosed
            || status == RecoveryStepStatus::BlockedByMissingEvidence;
        let blocks_production = config.hold_production_until_recovery_settled
            && (!source.package_execution_observed
                || source.execution_production_blocked
                || status != RecoveryStepStatus::ReadyForWallet);
        let hold_root = hold_root(
            config,
            source,
            step_kind,
            status,
            blocks_user_release,
            blocks_production,
            fail_closed_required,
        );
        let receipt_root = playbook_step_root(
            config,
            source,
            step_kind,
            status,
            ordinal,
            &evidence_root,
            &deadline_root,
            &hold_root,
            blocks_user_release,
            blocks_production,
            fail_closed_required,
        );
        let step_id = step_id(step_kind, ordinal, &receipt_root);
        Self {
            step_id,
            ordinal,
            step_kind,
            wallet_instruction: step_kind.wallet_instruction().to_string(),
            evidence_root,
            deadline_root,
            hold_root,
            receipt_root,
            status,
            requires_wallet_action: true,
            blocks_user_release,
            blocks_production,
            fail_closed_required,
            required_outcome: required_outcome(source, step_kind, status).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "step_id": self.step_id,
            "ordinal": self.ordinal,
            "step_kind": self.step_kind.as_str(),
            "wallet_instruction": self.wallet_instruction,
            "evidence_root": self.evidence_root,
            "deadline_root": self.deadline_root,
            "hold_root": self.hold_root,
            "receipt_root": self.receipt_root,
            "status": self.status.as_str(),
            "requires_wallet_action": self.requires_wallet_action,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "fail_closed_required": self.fail_closed_required,
            "required_outcome": self.required_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecoveryPlaybookVerdict {
    pub recovery_step_count: u64,
    pub wallet_action_count: u64,
    pub ready_step_count: u64,
    pub waiting_step_count: u64,
    pub fail_closed_step_count: u64,
    pub missing_evidence_step_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub evidence_roots_present: bool,
    pub deadline_receipts_present: bool,
    pub fail_closed_holds_present: bool,
    pub execution_fully_settled: bool,
    pub wallet_recovery_answerable: bool,
    pub production_blocked: bool,
    pub recovery_status: String,
    pub user_recovery_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl RecoveryPlaybookVerdict {
    pub fn new(config: &Config, source: &SourceBundle, steps: &[RecoveryPlaybookStep]) -> Self {
        let recovery_step_count = steps.len() as u64;
        let wallet_action_count = steps
            .iter()
            .filter(|step| step.requires_wallet_action)
            .count() as u64;
        let ready_step_count = count_status(steps, RecoveryStepStatus::ReadyForWallet);
        let waiting_step_count = count_status(steps, RecoveryStepStatus::WaitingForSettlement);
        let fail_closed_step_count = count_status(steps, RecoveryStepStatus::HeldFailClosed);
        let missing_evidence_step_count =
            count_status(steps, RecoveryStepStatus::BlockedByMissingEvidence);
        let user_release_blocker_count =
            steps.iter().filter(|step| step.blocks_user_release).count() as u64;
        let production_blocker_count =
            steps.iter().filter(|step| step.blocks_production).count() as u64;
        let evidence_roots_present = !source.execution_receipt_root.is_empty()
            && !source.observed_submission_bundle_root.is_empty()
            && !source.recovery_receipt_root.is_empty();
        let deadline_receipts_present = !source.challenge_settlement_bundle_root.is_empty()
            && source.settlement_deadline_blocks() >= config.deadline_grace_blocks;
        let fail_closed_holds_present =
            !source.execution_production_hold_root.is_empty() && fail_closed_step_count > 0;
        let execution_fully_settled = source.package_execution_observed
            && source.user_escape_execution_observed
            && source.deferred_receipt_count == 0
            && source.release_held_count == 0
            && source.fail_closed_count == 0;
        let wallet_recovery_answerable = recovery_step_count >= config.min_recovery_steps
            && evidence_roots_present
            && deadline_receipts_present
            && (execution_fully_settled || fail_closed_holds_present);
        let production_blocked = config.hold_production_until_recovery_settled
            && (!execution_fully_settled
                || source.execution_production_blocked
                || production_blocker_count > 0);
        let recovery_status = recovery_status(
            execution_fully_settled,
            fail_closed_step_count,
            missing_evidence_step_count,
            waiting_step_count,
        )
        .to_string();
        let user_recovery_answer =
            user_recovery_answer(wallet_recovery_answerable, execution_fully_settled).to_string();
        let production_answer = production_answer(production_blocked).to_string();
        let verdict_root = verdict_root(
            config,
            source,
            recovery_step_count,
            ready_step_count,
            waiting_step_count,
            fail_closed_step_count,
            missing_evidence_step_count,
            user_release_blocker_count,
            production_blocker_count,
            evidence_roots_present,
            deadline_receipts_present,
            fail_closed_holds_present,
            execution_fully_settled,
            wallet_recovery_answerable,
            production_blocked,
            &recovery_status,
            &user_recovery_answer,
            &production_answer,
        );
        Self {
            recovery_step_count,
            wallet_action_count,
            ready_step_count,
            waiting_step_count,
            fail_closed_step_count,
            missing_evidence_step_count,
            user_release_blocker_count,
            production_blocker_count,
            evidence_roots_present,
            deadline_receipts_present,
            fail_closed_holds_present,
            execution_fully_settled,
            wallet_recovery_answerable,
            production_blocked,
            recovery_status,
            user_recovery_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "recovery_step_count": self.recovery_step_count,
            "wallet_action_count": self.wallet_action_count,
            "ready_step_count": self.ready_step_count,
            "waiting_step_count": self.waiting_step_count,
            "fail_closed_step_count": self.fail_closed_step_count,
            "missing_evidence_step_count": self.missing_evidence_step_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "evidence_roots_present": self.evidence_roots_present,
            "deadline_receipts_present": self.deadline_receipts_present,
            "fail_closed_holds_present": self.fail_closed_holds_present,
            "execution_fully_settled": self.execution_fully_settled,
            "wallet_recovery_answerable": self.wallet_recovery_answerable,
            "production_blocked": self.production_blocked,
            "recovery_status": self.recovery_status,
            "user_recovery_answer": self.user_recovery_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub recovery_steps: Vec<RecoveryPlaybookStep>,
    pub verdict: RecoveryPlaybookVerdict,
    pub recovery_step_root: String,
    pub wallet_instruction_root: String,
    pub evidence_bundle_root: String,
    pub deadline_bundle_root: String,
    pub fail_closed_hold_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution_receipt::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution(&execution_state);
        validate_source(&source)?;
        let recovery_steps = RecoveryStepKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, step_kind)| {
                RecoveryPlaybookStep::devnet(&config, &source, *step_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = RecoveryPlaybookVerdict::new(&config, &source, &recovery_steps);
        let recovery_step_root = recovery_step_vector_root(&recovery_steps);
        let wallet_instruction_root = wallet_instruction_root(&config, &source, &recovery_steps);
        let evidence_bundle_root = evidence_bundle_root(&config, &source, &recovery_steps);
        let deadline_bundle_root = deadline_bundle_root(&config, &source, &recovery_steps);
        let fail_closed_hold_root =
            fail_closed_hold_root(&config, &source, &recovery_steps, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &recovery_steps, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &recovery_step_root,
            &wallet_instruction_root,
            &evidence_bundle_root,
            &deadline_bundle_root,
            &fail_closed_hold_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            recovery_steps,
            verdict,
            recovery_step_root,
            wallet_instruction_root,
            evidence_bundle_root,
            deadline_bundle_root,
            fail_closed_hold_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), execution_receipt::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_recovery_playbook_receipt_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "recovery_step_root": self.recovery_step_root,
            "wallet_instruction_root": self.wallet_instruction_root,
            "evidence_bundle_root": self.evidence_bundle_root,
            "deadline_bundle_root": self.deadline_bundle_root,
            "fail_closed_hold_root": self.fail_closed_hold_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "recovery_steps": self
                .recovery_steps
                .iter()
                .map(RecoveryPlaybookStep::public_record)
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

impl SourceBundle {
    fn settlement_deadline_blocks(&self) -> u64 {
        if self.deferred_receipt_count > 0 || self.release_held_count > 0 {
            DEFAULT_SETTLEMENT_DEADLINE_BLOCKS
        } else {
            DEFAULT_DEADLINE_GRACE_BLOCKS
        }
    }
}

fn step_status(source: &SourceBundle, step_kind: RecoveryStepKind) -> RecoveryStepStatus {
    if source.execution_receipt_count == 0 || source.execution_receipt_root.is_empty() {
        return RecoveryStepStatus::BlockedByMissingEvidence;
    }
    if source.fail_closed_count > 0 {
        return RecoveryStepStatus::HeldFailClosed;
    }
    if source.deferred_receipt_count > 0 || source.release_held_count > 0 {
        return match step_kind {
            RecoveryStepKind::EscalateFailClosedHold | RecoveryStepKind::ArchiveRecoveryReceipt => {
                RecoveryStepStatus::HeldFailClosed
            }
            _ => RecoveryStepStatus::WaitingForSettlement,
        };
    }
    RecoveryStepStatus::ReadyForWallet
}

fn fail_closed_required(source: &SourceBundle, status: RecoveryStepStatus) -> bool {
    source.fail_closed_count > 0
        || source.release_held_count > 0
        || status == RecoveryStepStatus::HeldFailClosed
}

fn evidence_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: RecoveryStepKind,
    ordinal: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-EVIDENCE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(&source.observed_submission_bundle_root),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(step_kind.as_str()),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn deadline_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: RecoveryStepKind,
    ordinal: u64,
    status: RecoveryStepStatus,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-DEADLINE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.challenge_settlement_bundle_root),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::U64(ordinal),
            HashPart::U64(config.deadline_grace_blocks),
            HashPart::U64(config.settlement_deadline_blocks),
        ],
        32,
    )
}

fn hold_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: RecoveryStepKind,
    status: RecoveryStepStatus,
    blocks_user_release: bool,
    blocks_production: bool,
    fail_closed_required: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-HOLD",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_production_hold_root),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
            HashPart::Str(bool_str(fail_closed_required)),
        ],
        32,
    )
}

fn playbook_step_root(
    config: &Config,
    source: &SourceBundle,
    step_kind: RecoveryStepKind,
    status: RecoveryStepStatus,
    ordinal: u64,
    evidence_root: &str,
    deadline_root: &str,
    hold_root: &str,
    blocks_user_release: bool,
    blocks_production: bool,
    fail_closed_required: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-STEP",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(evidence_root),
            HashPart::Str(deadline_root),
            HashPart::Str(hold_root),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
            HashPart::Str(bool_str(fail_closed_required)),
        ],
        32,
    )
}

fn step_id(step_kind: RecoveryStepKind, ordinal: u64, receipt_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-STEP-ID",
        &[
            HashPart::Str(step_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(receipt_root),
        ],
        16,
    )
}

fn required_outcome(
    source: &SourceBundle,
    step_kind: RecoveryStepKind,
    status: RecoveryStepStatus,
) -> &'static str {
    match status {
        RecoveryStepStatus::ReadyForWallet => {
            "wallet recovery playbook is ready to use with settled execution receipts"
        }
        RecoveryStepStatus::WaitingForSettlement => match step_kind {
            RecoveryStepKind::WatchChallengeDeadline => {
                "keep watching challenge and settlement deadlines before final release"
            }
            RecoveryStepKind::ClaimReserveFallback => {
                "prepare reserve fallback claim while execution remains unsettled"
            }
            _ => "keep wallet recovery material available until execution receipts settle",
        },
        RecoveryStepStatus::HeldFailClosed => {
            if source.fail_closed_count > 0 {
                "fail-closed execution receipt requires recovery hold and review"
            } else {
                "unsettled execution requires fail-closed hold until receipt recovery completes"
            }
        }
        RecoveryStepStatus::BlockedByMissingEvidence => {
            "missing evidence roots block wallet release and production promotion"
        }
    }
}

fn recovery_step_vector_root(steps: &[RecoveryPlaybookStep]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-STEPS",
        &steps
            .iter()
            .map(RecoveryPlaybookStep::public_record)
            .collect::<Vec<_>>(),
    )
}

fn wallet_instruction_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[RecoveryPlaybookStep],
) -> String {
    let records = steps
        .iter()
        .map(|step| {
            json!({
                "step_id": &step.step_id,
                "step_kind": step.step_kind.as_str(),
                "wallet_instruction": &step.wallet_instruction,
                "status": step.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let instruction_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-WALLET-INSTRUCTIONS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-WALLET-INSTRUCTION-BUNDLE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_user_escape_answer),
            HashPart::Str(&instruction_root),
            HashPart::U64(steps.len() as u64),
        ],
        32,
    )
}

fn evidence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[RecoveryPlaybookStep],
) -> String {
    let records = steps
        .iter()
        .map(|step| {
            json!({
                "step_id": &step.step_id,
                "evidence_root": &step.evidence_root,
                "receipt_root": &step.receipt_root,
            })
        })
        .collect::<Vec<_>>();
    let step_evidence_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-EVIDENCE-ROOTS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-EVIDENCE-BUNDLE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(&source.observed_submission_bundle_root),
            HashPart::Str(&source.pq_privacy_receipt_root),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&step_evidence_root),
        ],
        32,
    )
}

fn deadline_bundle_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[RecoveryPlaybookStep],
) -> String {
    let records = steps
        .iter()
        .map(|step| {
            json!({
                "step_id": &step.step_id,
                "deadline_root": &step.deadline_root,
                "status": step.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let step_deadline_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-DEADLINES",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-DEADLINE-BUNDLE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.challenge_settlement_bundle_root),
            HashPart::Str(&step_deadline_root),
            HashPart::U64(config.deadline_grace_blocks),
            HashPart::U64(config.settlement_deadline_blocks),
        ],
        32,
    )
}

fn fail_closed_hold_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[RecoveryPlaybookStep],
    verdict: &RecoveryPlaybookVerdict,
) -> String {
    let records = steps
        .iter()
        .filter(|step| step.fail_closed_required)
        .map(|step| {
            json!({
                "step_id": &step.step_id,
                "step_kind": step.step_kind.as_str(),
                "hold_root": &step.hold_root,
                "status": step.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let hold_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-FAIL-CLOSED-HOLDS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-FAIL-CLOSED-HOLD-BUNDLE",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_production_hold_root),
            HashPart::Str(&hold_vector_root),
            HashPart::U64(verdict.fail_closed_step_count),
            HashPart::Str(bool_str(verdict.fail_closed_holds_present)),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[RecoveryPlaybookStep],
    verdict: &RecoveryPlaybookVerdict,
) -> String {
    let records = steps
        .iter()
        .filter(|step| step.blocks_production)
        .map(|step| {
            json!({
                "step_id": &step.step_id,
                "step_kind": step.step_kind.as_str(),
                "hold_root": &step.hold_root,
                "status": step.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PRODUCTION-BLOCKERS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_production_hold_root),
            HashPart::Str(&blocker_root),
            HashPart::U64(verdict.production_blocker_count),
            HashPart::Str(bool_str(verdict.production_blocked)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    recovery_step_root: &str,
    wallet_instruction_root: &str,
    evidence_bundle_root: &str,
    deadline_bundle_root: &str,
    fail_closed_hold_root: &str,
    production_hold_root: &str,
    verdict: &RecoveryPlaybookVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(recovery_step_root),
            HashPart::Str(wallet_instruction_root),
            HashPart::Str(evidence_bundle_root),
            HashPart::Str(deadline_bundle_root),
            HashPart::Str(fail_closed_hold_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    recovery_step_count: u64,
    ready_step_count: u64,
    waiting_step_count: u64,
    fail_closed_step_count: u64,
    missing_evidence_step_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    evidence_roots_present: bool,
    deadline_receipts_present: bool,
    fail_closed_holds_present: bool,
    execution_fully_settled: bool,
    wallet_recovery_answerable: bool,
    production_blocked: bool,
    recovery_status: &str,
    user_recovery_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-VERDICT",
        &[
            HashPart::Str(&config.playbook_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::U64(recovery_step_count),
            HashPart::U64(ready_step_count),
            HashPart::U64(waiting_step_count),
            HashPart::U64(fail_closed_step_count),
            HashPart::U64(missing_evidence_step_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(evidence_roots_present)),
            HashPart::Str(bool_str(deadline_receipts_present)),
            HashPart::Str(bool_str(fail_closed_holds_present)),
            HashPart::Str(bool_str(execution_fully_settled)),
            HashPart::Str(bool_str(wallet_recovery_answerable)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(recovery_status),
            HashPart::Str(user_recovery_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn recovery_status(
    execution_fully_settled: bool,
    fail_closed_step_count: u64,
    missing_evidence_step_count: u64,
    waiting_step_count: u64,
) -> &'static str {
    if missing_evidence_step_count > 0 {
        "blocked_by_missing_evidence"
    } else if fail_closed_step_count > 0 {
        "fail_closed_recovery_hold"
    } else if waiting_step_count > 0 {
        "waiting_for_execution_settlement"
    } else if execution_fully_settled {
        "settled_recovery_receipts_ready"
    } else {
        "recovery_playbook_available"
    }
}

fn user_recovery_answer(
    wallet_recovery_answerable: bool,
    execution_fully_settled: bool,
) -> &'static str {
    if wallet_recovery_answerable && execution_fully_settled {
        "wallet recovery playbook is answerable with settled execution receipts"
    } else if wallet_recovery_answerable {
        "wallet recovery playbook is answerable under fail-closed hold"
    } else {
        "wallet recovery playbook is not answerable until evidence and deadlines are complete"
    }
}

fn production_answer(production_blocked: bool) -> &'static str {
    if production_blocked {
        "production remains held until force-exit recovery receipts settle"
    } else {
        "production may proceed after settled recovery playbook receipt"
    }
}

fn count_status(steps: &[RecoveryPlaybookStep], status: RecoveryStepStatus) -> u64 {
    steps.iter().filter(|step| step.status == status).count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "force-exit recovery playbook receipt chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "force-exit recovery playbook receipt protocol mismatch",
    )?;
    ensure(
        config.min_recovery_steps > 0,
        "force-exit recovery playbook receipt requires recovery steps",
    )?;
    ensure(
        config.settlement_deadline_blocks >= config.deadline_grace_blocks,
        "force-exit recovery playbook receipt deadline order invalid",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_state_root.is_empty(),
        "force-exit recovery playbook receipt missing execution state root",
    )?;
    ensure(
        !source.execution_receipt_root.is_empty(),
        "force-exit recovery playbook receipt missing execution receipt root",
    )?;
    ensure(
        source.execution_receipt_count > 0,
        "force-exit recovery playbook receipt missing execution receipts",
    )?;
    Ok(())
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle {
        execution_state_root: record_root("fallback-execution-state", &json!({"reason": &reason})),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({"reason": &reason}),
        ),
        observed_submission_bundle_root: record_root(
            "fallback-observed-submission",
            &json!({"reason": &reason}),
        ),
        challenge_settlement_bundle_root: record_root(
            "fallback-challenge-settlement",
            &json!({"reason": &reason}),
        ),
        pq_privacy_receipt_root: record_root("fallback-pq-privacy", &json!({"reason": &reason})),
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        execution_production_hold_root: record_root(
            "fallback-production-hold",
            &json!({"reason": &reason}),
        ),
        execution_status: "fallback".to_string(),
        execution_user_escape_answer: reason.clone(),
        execution_production_answer: "fallback production held".to_string(),
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        deferred_receipt_count: 1,
        release_held_count: 0,
        fail_closed_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        package_execution_observed: false,
        user_escape_execution_observed: false,
        execution_production_blocked: true,
    };
    let recovery_steps = RecoveryStepKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, step_kind)| {
            RecoveryPlaybookStep::devnet(&config, &source, *step_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = RecoveryPlaybookVerdict::new(&config, &source, &recovery_steps);
    let recovery_step_root = recovery_step_vector_root(&recovery_steps);
    let wallet_instruction_root = wallet_instruction_root(&config, &source, &recovery_steps);
    let evidence_bundle_root = evidence_bundle_root(&config, &source, &recovery_steps);
    let deadline_bundle_root = deadline_bundle_root(&config, &source, &recovery_steps);
    let fail_closed_hold_root = fail_closed_hold_root(&config, &source, &recovery_steps, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &recovery_steps, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &recovery_step_root,
        &wallet_instruction_root,
        &evidence_bundle_root,
        &deadline_bundle_root,
        &fail_closed_hold_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        recovery_steps,
        verdict,
        recovery_step_root,
        wallet_instruction_root,
        evidence_bundle_root,
        deadline_bundle_root,
        fail_closed_hold_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-RECOVERY-PLAYBOOK-RECEIPT-RECORD",
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
