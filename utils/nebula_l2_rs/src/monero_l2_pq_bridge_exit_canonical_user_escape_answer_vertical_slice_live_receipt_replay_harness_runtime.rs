use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_live_receipt_acceptance_runtime as live_acceptance,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceLiveReceiptReplayHarnessRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_LIVE_RECEIPT_REPLAY_HARNESS_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-live-receipt-replay-harness-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_LIVE_RECEIPT_REPLAY_HARNESS_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const LIVE_RECEIPT_REPLAY_HARNESS_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-live-receipt-replay-harness-v1";
pub const DEFAULT_MIN_REPLAY_STEPS: u64 = 9;
pub const DEFAULT_MIN_USER_ESCAPE_STEPS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub replay_harness_suite: String,
    pub min_replay_steps: u64,
    pub min_user_escape_steps: u64,
    pub require_wallet_commands: bool,
    pub require_operator_commands: bool,
    pub require_expected_observations: bool,
    pub require_fail_closed_receipts: bool,
    pub require_release_hold_outputs: bool,
    pub hold_production_until_replay_executed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            replay_harness_suite: LIVE_RECEIPT_REPLAY_HARNESS_SUITE.to_string(),
            min_replay_steps: DEFAULT_MIN_REPLAY_STEPS,
            min_user_escape_steps: DEFAULT_MIN_USER_ESCAPE_STEPS,
            require_wallet_commands: true,
            require_operator_commands: true,
            require_expected_observations: true,
            require_fail_closed_receipts: true,
            require_release_hold_outputs: true,
            hold_production_until_replay_executed: true,
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
            "replay_harness_suite": self.replay_harness_suite,
            "min_replay_steps": self.min_replay_steps,
            "min_user_escape_steps": self.min_user_escape_steps,
            "require_wallet_commands": self.require_wallet_commands,
            "require_operator_commands": self.require_operator_commands,
            "require_expected_observations": self.require_expected_observations,
            "require_fail_closed_receipts": self.require_fail_closed_receipts,
            "require_release_hold_outputs": self.require_release_hold_outputs,
            "hold_production_until_replay_executed": self.hold_production_until_replay_executed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStepKind {
    MoneroHeaderCanonicality,
    DepositLockWatcher,
    PrivateNoteState,
    TransferOrContractExecution,
    SettlementReceiptExecutor,
    ReserveLiquidity,
    PqAuthorityQuorum,
    WalletScannerPrivacy,
    ReleaseBlockerClearing,
}

impl ReplayStepKind {
    pub fn from_receipt_kind(kind: live_acceptance::LiveReceiptKind) -> Self {
        match kind {
            live_acceptance::LiveReceiptKind::MoneroHeaderCanonicality => {
                Self::MoneroHeaderCanonicality
            }
            live_acceptance::LiveReceiptKind::DepositLockWatcher => Self::DepositLockWatcher,
            live_acceptance::LiveReceiptKind::PrivateNoteState => Self::PrivateNoteState,
            live_acceptance::LiveReceiptKind::TransferOrContractExecution => {
                Self::TransferOrContractExecution
            }
            live_acceptance::LiveReceiptKind::SettlementReceiptExecutor => {
                Self::SettlementReceiptExecutor
            }
            live_acceptance::LiveReceiptKind::ReserveLiquidity => Self::ReserveLiquidity,
            live_acceptance::LiveReceiptKind::PqAuthorityQuorum => Self::PqAuthorityQuorum,
            live_acceptance::LiveReceiptKind::WalletScannerPrivacy => Self::WalletScannerPrivacy,
            live_acceptance::LiveReceiptKind::ReleaseBlockerClearing => {
                Self::ReleaseBlockerClearing
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroHeaderCanonicality => "monero_header_canonicality",
            Self::DepositLockWatcher => "deposit_lock_watcher",
            Self::PrivateNoteState => "private_note_state",
            Self::TransferOrContractExecution => "transfer_or_contract_execution",
            Self::SettlementReceiptExecutor => "settlement_receipt_executor",
            Self::ReserveLiquidity => "reserve_liquidity",
            Self::PqAuthorityQuorum => "pq_authority_quorum",
            Self::WalletScannerPrivacy => "wallet_scanner_privacy",
            Self::ReleaseBlockerClearing => "release_blocker_clearing",
        }
    }

    pub fn command(self) -> &'static str {
        match self {
            Self::MoneroHeaderCanonicality => {
                "replay_monero_header_finality_and_reorg_depth_receipt"
            }
            Self::DepositLockWatcher => "replay_deposit_lock_watcher_quorum_receipt",
            Self::PrivateNoteState => "replay_private_note_state_transition_receipt",
            Self::TransferOrContractExecution => {
                "replay_private_transfer_or_contract_execution_receipt"
            }
            Self::SettlementReceiptExecutor => "replay_settlement_exit_claim_receipt",
            Self::ReserveLiquidity => "replay_reserve_liquidity_sufficiency_receipt",
            Self::PqAuthorityQuorum => "replay_post_quantum_authority_quorum_receipt",
            Self::WalletScannerPrivacy => "replay_wallet_scanner_privacy_receipt",
            Self::ReleaseBlockerClearing => "replay_release_blocker_clearing_receipt",
        }
    }

    pub fn user_escape_step(self) -> bool {
        matches!(
            self,
            Self::DepositLockWatcher
                | Self::PrivateNoteState
                | Self::TransferOrContractExecution
                | Self::SettlementReceiptExecutor
                | Self::ReserveLiquidity
                | Self::WalletScannerPrivacy
                | Self::ReleaseBlockerClearing
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStepStatus {
    ReadyToReplay,
    DeferredUntilLiveReceipt,
    ProductionHold,
    FailClosed,
}

impl ReplayStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToReplay => "ready_to_replay",
            Self::DeferredUntilLiveReceipt => "deferred_until_live_receipt",
            Self::ProductionHold => "production_hold",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub acceptance_state_root: String,
    pub acceptance_case_root: String,
    pub observed_receipt_acceptance_root: String,
    pub user_escape_receipt_root: String,
    pub production_hold_root: String,
    pub acceptance_status: String,
    pub acceptance_user_escape_answer: String,
    pub acceptance_production_answer: String,
    pub acceptance_case_count: u64,
    pub accepted_case_count: u64,
    pub deferred_case_count: u64,
    pub production_hold_case_count: u64,
    pub fail_closed_case_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub user_escape_receipts_sufficient: bool,
    pub live_receipts_sufficient: bool,
    pub production_blocked: bool,
}

impl SourceBundle {
    pub fn from_acceptance(state: &live_acceptance::State) -> Self {
        Self {
            acceptance_state_root: state.state_root(),
            acceptance_case_root: state.acceptance_case_root.clone(),
            observed_receipt_acceptance_root: state.observed_receipt_acceptance_root.clone(),
            user_escape_receipt_root: state.user_escape_receipt_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            acceptance_status: state.verdict.acceptance_status.clone(),
            acceptance_user_escape_answer: state.verdict.user_escape_answer.clone(),
            acceptance_production_answer: state.verdict.production_answer.clone(),
            acceptance_case_count: state.verdict.acceptance_case_count,
            accepted_case_count: state.verdict.accepted_case_count,
            deferred_case_count: state.verdict.deferred_case_count,
            production_hold_case_count: state.verdict.production_hold_case_count,
            fail_closed_case_count: state.verdict.fail_closed_case_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            user_escape_receipts_sufficient: state.verdict.user_escape_receipts_sufficient,
            live_receipts_sufficient: state.verdict.live_receipts_sufficient,
            production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let acceptance = live_acceptance::devnet();
        Self::from_acceptance(&acceptance)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "acceptance_state_root": self.acceptance_state_root,
            "acceptance_case_root": self.acceptance_case_root,
            "observed_receipt_acceptance_root": self.observed_receipt_acceptance_root,
            "user_escape_receipt_root": self.user_escape_receipt_root,
            "production_hold_root": self.production_hold_root,
            "acceptance_status": self.acceptance_status,
            "acceptance_user_escape_answer": self.acceptance_user_escape_answer,
            "acceptance_production_answer": self.acceptance_production_answer,
            "acceptance_case_count": self.acceptance_case_count,
            "accepted_case_count": self.accepted_case_count,
            "deferred_case_count": self.deferred_case_count,
            "production_hold_case_count": self.production_hold_case_count,
            "fail_closed_case_count": self.fail_closed_case_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "user_escape_receipts_sufficient": self.user_escape_receipts_sufficient,
            "live_receipts_sufficient": self.live_receipts_sufficient,
            "production_blocked": self.production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveReceiptReplayStep {
    pub step_id: String,
    pub ordinal: u64,
    pub step_kind: ReplayStepKind,
    pub acceptance_case_id: String,
    pub acceptance_status: String,
    pub command_root: String,
    pub expected_observation_root: String,
    pub observed_input_root: String,
    pub wallet_instruction_root: String,
    pub operator_instruction_root: String,
    pub fail_closed_receipt_root: String,
    pub replay_result_root: String,
    pub release_hold_root: String,
    pub status: ReplayStepStatus,
    pub user_escape_step: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub replay_command: String,
    pub expected_outcome: String,
}

impl LiveReceiptReplayStep {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        acceptance_case: &live_acceptance::LiveReceiptAcceptanceCase,
    ) -> Self {
        let step_kind = ReplayStepKind::from_receipt_kind(acceptance_case.receipt_kind);
        let status = replay_status(acceptance_case.status);
        let command_root = command_root(config, source, acceptance_case, step_kind);
        let expected_observation_root =
            expected_observation_root(config, source, acceptance_case, step_kind);
        let observed_input_root = observed_input_root(
            config,
            source,
            acceptance_case,
            status,
            &expected_observation_root,
        );
        let wallet_instruction_root =
            wallet_instruction_root(config, source, acceptance_case, step_kind);
        let operator_instruction_root =
            operator_instruction_root(config, source, acceptance_case, step_kind);
        let fail_closed_receipt_root =
            fail_closed_receipt_root(config, source, acceptance_case, step_kind, status);
        let replay_result_root = replay_result_root(
            config,
            source,
            acceptance_case,
            step_kind,
            status,
            &command_root,
            &expected_observation_root,
            &observed_input_root,
            &wallet_instruction_root,
            &operator_instruction_root,
            &fail_closed_receipt_root,
        );
        let blocks_user_release =
            acceptance_case.blocks_user_release || status == ReplayStepStatus::FailClosed;
        let blocks_production =
            acceptance_case.blocks_production || status != ReplayStepStatus::ReadyToReplay;
        let release_hold_root = replay_release_hold_root(
            source,
            acceptance_case,
            status,
            &replay_result_root,
            blocks_user_release,
            blocks_production,
        );
        let step_id = step_id(step_kind, acceptance_case.ordinal, &replay_result_root);
        Self {
            step_id,
            ordinal: acceptance_case.ordinal,
            step_kind,
            acceptance_case_id: acceptance_case.case_id.clone(),
            acceptance_status: acceptance_case.status.as_str().to_string(),
            command_root,
            expected_observation_root,
            observed_input_root,
            wallet_instruction_root,
            operator_instruction_root,
            fail_closed_receipt_root,
            replay_result_root,
            release_hold_root,
            status,
            user_escape_step: step_kind.user_escape_step(),
            blocks_user_release,
            blocks_production,
            replay_command: step_kind.command().to_string(),
            expected_outcome: expected_outcome(status, step_kind).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "step_id": self.step_id,
            "ordinal": self.ordinal,
            "step_kind": self.step_kind.as_str(),
            "acceptance_case_id": self.acceptance_case_id,
            "acceptance_status": self.acceptance_status,
            "command_root": self.command_root,
            "expected_observation_root": self.expected_observation_root,
            "observed_input_root": self.observed_input_root,
            "wallet_instruction_root": self.wallet_instruction_root,
            "operator_instruction_root": self.operator_instruction_root,
            "fail_closed_receipt_root": self.fail_closed_receipt_root,
            "replay_result_root": self.replay_result_root,
            "release_hold_root": self.release_hold_root,
            "status": self.status.as_str(),
            "user_escape_step": self.user_escape_step,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "replay_command": self.replay_command,
            "expected_outcome": self.expected_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.replay_result_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveReceiptReplayVerdict {
    pub replay_step_count: u64,
    pub ready_step_count: u64,
    pub deferred_step_count: u64,
    pub production_hold_step_count: u64,
    pub fail_closed_step_count: u64,
    pub user_escape_step_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub acceptance_case_count: u64,
    pub accepted_case_count: u64,
    pub live_receipts_sufficient: bool,
    pub user_escape_receipts_sufficient: bool,
    pub all_replay_steps_present: bool,
    pub wallet_commands_present: bool,
    pub operator_commands_present: bool,
    pub expected_observations_present: bool,
    pub fail_closed_receipts_present: bool,
    pub release_holds_present: bool,
    pub replay_executable: bool,
    pub user_escape_replay_executable: bool,
    pub production_blocked: bool,
    pub replay_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl LiveReceiptReplayVerdict {
    pub fn new(config: &Config, source: &SourceBundle, steps: &[LiveReceiptReplayStep]) -> Self {
        let replay_step_count = steps.len() as u64;
        let ready_step_count = count_status(steps, ReplayStepStatus::ReadyToReplay);
        let deferred_step_count = count_status(steps, ReplayStepStatus::DeferredUntilLiveReceipt);
        let production_hold_step_count = count_status(steps, ReplayStepStatus::ProductionHold);
        let fail_closed_step_count = count_status(steps, ReplayStepStatus::FailClosed);
        let user_escape_step_count =
            steps.iter().filter(|step| step.user_escape_step).count() as u64;
        let user_release_blocker_count =
            steps.iter().filter(|step| step.blocks_user_release).count() as u64;
        let production_blocker_count =
            steps.iter().filter(|step| step.blocks_production).count() as u64;
        let acceptance_case_count = source.acceptance_case_count;
        let accepted_case_count = source.accepted_case_count;
        let live_receipts_sufficient = source.live_receipts_sufficient;
        let user_escape_receipts_sufficient = source.user_escape_receipts_sufficient;
        let all_replay_steps_present = replay_step_count >= config.min_replay_steps
            && replay_step_count == acceptance_case_count;
        let wallet_commands_present = !config.require_wallet_commands
            || steps
                .iter()
                .all(|step| !step.wallet_instruction_root.is_empty());
        let operator_commands_present = !config.require_operator_commands
            || steps
                .iter()
                .all(|step| !step.operator_instruction_root.is_empty());
        let expected_observations_present = !config.require_expected_observations
            || steps
                .iter()
                .all(|step| !step.expected_observation_root.is_empty());
        let fail_closed_receipts_present = !config.require_fail_closed_receipts
            || steps
                .iter()
                .all(|step| !step.fail_closed_receipt_root.is_empty());
        let release_holds_present = !config.require_release_hold_outputs
            || steps.iter().all(|step| !step.release_hold_root.is_empty());
        let replay_executable = all_replay_steps_present
            && ready_step_count >= config.min_replay_steps
            && deferred_step_count == 0
            && production_hold_step_count == 0
            && fail_closed_step_count == 0
            && live_receipts_sufficient
            && wallet_commands_present
            && operator_commands_present
            && expected_observations_present
            && fail_closed_receipts_present
            && release_holds_present;
        let user_escape_replay_executable = replay_executable
            && user_escape_step_count >= config.min_user_escape_steps
            && user_release_blocker_count == 0
            && user_escape_receipts_sufficient;
        let production_blocked = source.production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_replay_executed && !replay_executable);
        let replay_status = if fail_closed_step_count > 0 {
            "fail_closed"
        } else if production_hold_step_count > 0 {
            "production_hold"
        } else if deferred_step_count > 0 {
            "replay_deferred_until_live_receipts"
        } else if replay_executable {
            "replay_executable"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_replay_executable {
            "user escape replay is executable against accepted live receipts"
        } else {
            "user escape replay commands are specified but remain deferred until live receipts, wallet observations, and blocker-clearing outputs are observed"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains blocked until live receipt replay executes cleanly and heavy gates complete"
        } else {
            "bounded bridge/exit replay is executable for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            replay_step_count,
            ready_step_count,
            deferred_step_count,
            production_hold_step_count,
            fail_closed_step_count,
            user_release_blocker_count,
            production_blocker_count,
            all_replay_steps_present,
            replay_executable,
            user_escape_replay_executable,
            production_blocked,
            &replay_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            replay_step_count,
            ready_step_count,
            deferred_step_count,
            production_hold_step_count,
            fail_closed_step_count,
            user_escape_step_count,
            user_release_blocker_count,
            production_blocker_count,
            acceptance_case_count,
            accepted_case_count,
            live_receipts_sufficient,
            user_escape_receipts_sufficient,
            all_replay_steps_present,
            wallet_commands_present,
            operator_commands_present,
            expected_observations_present,
            fail_closed_receipts_present,
            release_holds_present,
            replay_executable,
            user_escape_replay_executable,
            production_blocked,
            replay_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_step_count": self.replay_step_count,
            "ready_step_count": self.ready_step_count,
            "deferred_step_count": self.deferred_step_count,
            "production_hold_step_count": self.production_hold_step_count,
            "fail_closed_step_count": self.fail_closed_step_count,
            "user_escape_step_count": self.user_escape_step_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "acceptance_case_count": self.acceptance_case_count,
            "accepted_case_count": self.accepted_case_count,
            "live_receipts_sufficient": self.live_receipts_sufficient,
            "user_escape_receipts_sufficient": self.user_escape_receipts_sufficient,
            "all_replay_steps_present": self.all_replay_steps_present,
            "wallet_commands_present": self.wallet_commands_present,
            "operator_commands_present": self.operator_commands_present,
            "expected_observations_present": self.expected_observations_present,
            "fail_closed_receipts_present": self.fail_closed_receipts_present,
            "release_holds_present": self.release_holds_present,
            "replay_executable": self.replay_executable,
            "user_escape_replay_executable": self.user_escape_replay_executable,
            "production_blocked": self.production_blocked,
            "replay_status": self.replay_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub replay_steps: Vec<LiveReceiptReplayStep>,
    pub verdict: LiveReceiptReplayVerdict,
    pub replay_step_root: String,
    pub command_vector_root: String,
    pub wallet_replay_root: String,
    pub fail_closed_replay_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, acceptance_state: live_acceptance::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_acceptance(&acceptance_state);
        validate_source(&source)?;
        let replay_steps = acceptance_state
            .acceptance_cases
            .iter()
            .map(|case| LiveReceiptReplayStep::devnet(&config, &source, case))
            .collect::<Vec<_>>();
        let verdict = LiveReceiptReplayVerdict::new(&config, &source, &replay_steps);
        let replay_step_root = replay_step_vector_root(&replay_steps);
        let command_vector_root = command_vector_root(&config, &source, &replay_steps, &verdict);
        let wallet_replay_root = wallet_replay_root(&config, &source, &replay_steps, &verdict);
        let fail_closed_replay_root =
            fail_closed_replay_root(&config, &source, &replay_steps, &verdict);
        let production_hold_root = production_hold_root(&config, &source, &replay_steps, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &replay_step_root,
            &command_vector_root,
            &wallet_replay_root,
            &fail_closed_replay_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            replay_steps,
            verdict,
            replay_step_root,
            command_vector_root,
            wallet_replay_root,
            fail_closed_replay_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), live_acceptance::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_live_receipt_replay_harness_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "replay_step_root": self.replay_step_root,
            "command_vector_root": self.command_vector_root,
            "wallet_replay_root": self.wallet_replay_root,
            "fail_closed_replay_root": self.fail_closed_replay_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "replay_steps": self
                .replay_steps
                .iter()
                .map(LiveReceiptReplayStep::public_record)
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

fn replay_status(status: live_acceptance::LiveReceiptAcceptanceStatus) -> ReplayStepStatus {
    match status {
        live_acceptance::LiveReceiptAcceptanceStatus::Accepted => ReplayStepStatus::ReadyToReplay,
        live_acceptance::LiveReceiptAcceptanceStatus::DeferredUntilLiveReceipt => {
            ReplayStepStatus::DeferredUntilLiveReceipt
        }
        live_acceptance::LiveReceiptAcceptanceStatus::ProductionHold => {
            ReplayStepStatus::ProductionHold
        }
        live_acceptance::LiveReceiptAcceptanceStatus::FailClosed => ReplayStepStatus::FailClosed,
    }
}

fn command_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
) -> String {
    record_root(
        "command",
        &json!({
            "replay_harness_suite": config.replay_harness_suite,
            "command": step_kind.command(),
            "acceptance_case_id": case.case_id,
            "acceptance_root": case.acceptance_root,
            "acceptance_state_root": source.acceptance_state_root,
            "required_live_receipt_root": case.required_live_receipt_root,
        }),
    )
}

fn expected_observation_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
) -> String {
    record_root(
        "expected-observation",
        &json!({
            "required": config.require_expected_observations,
            "step_kind": step_kind.as_str(),
            "acceptance_case_id": case.case_id,
            "observed_receipt_acceptance_root": source.observed_receipt_acceptance_root,
            "required_live_receipt_root": case.required_live_receipt_root,
            "wallet_visibility_root": case.wallet_visibility_root,
            "pq_authority_root": case.pq_authority_root,
            "privacy_boundary_root": case.privacy_boundary_root,
        }),
    )
}

fn observed_input_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    status: ReplayStepStatus,
    expected_observation_root: &str,
) -> String {
    if status == ReplayStepStatus::ReadyToReplay {
        expected_observation_root.to_string()
    } else {
        record_root(
            "deferred-observed-input",
            &json!({
                "replay_harness_suite": config.replay_harness_suite,
                "acceptance_case_id": case.case_id,
                "status": status.as_str(),
                "acceptance_status": case.status.as_str(),
                "observed_live_receipt_root": case.observed_live_receipt_root,
                "production_hold_root": source.production_hold_root,
                "reason": "observed live receipt replay input remains deferred under lightweight workflow",
            }),
        )
    }
}

fn wallet_instruction_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
) -> String {
    record_root(
        "wallet-instruction",
        &json!({
            "required": config.require_wallet_commands,
            "step_kind": step_kind.as_str(),
            "acceptance_case_id": case.case_id,
            "user_escape_step": step_kind.user_escape_step(),
            "wallet_visibility_root": case.wallet_visibility_root,
            "user_escape_receipt_root": source.user_escape_receipt_root,
            "instruction": "scan_roots_verify_receipt_and_prepare_force_exit_evidence",
        }),
    )
}

fn operator_instruction_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
) -> String {
    record_root(
        "operator-instruction",
        &json!({
            "required": config.require_operator_commands,
            "step_kind": step_kind.as_str(),
            "acceptance_case_id": case.case_id,
            "operator_action": case.operator_action,
            "acceptance_case_root": source.acceptance_case_root,
            "instruction": "supply_live_receipt_or_hold_release",
        }),
    )
}

fn fail_closed_receipt_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
    status: ReplayStepStatus,
) -> String {
    record_root(
        "fail-closed-receipt",
        &json!({
            "required": config.require_fail_closed_receipts,
            "step_kind": step_kind.as_str(),
            "acceptance_case_id": case.case_id,
            "status": status.as_str(),
            "case_release_hold_root": case.release_hold_root,
            "production_hold_root": source.production_hold_root,
            "policy": "missing_or_mismatched_live_receipt_keeps_user_safe_and_release_held",
        }),
    )
}

fn replay_result_root(
    config: &Config,
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    step_kind: ReplayStepKind,
    status: ReplayStepStatus,
    command_root: &str,
    expected_observation_root: &str,
    observed_input_root: &str,
    wallet_instruction_root: &str,
    operator_instruction_root: &str,
    fail_closed_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-RESULT",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.acceptance_state_root),
            HashPart::Str(step_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&case.case_id),
            HashPart::Str(&case.acceptance_root),
            HashPart::Str(command_root),
            HashPart::Str(expected_observation_root),
            HashPart::Str(observed_input_root),
            HashPart::Str(wallet_instruction_root),
            HashPart::Str(operator_instruction_root),
            HashPart::Str(fail_closed_receipt_root),
        ],
        32,
    )
}

fn replay_release_hold_root(
    source: &SourceBundle,
    case: &live_acceptance::LiveReceiptAcceptanceCase,
    status: ReplayStepStatus,
    replay_result_root: &str,
    blocks_user_release: bool,
    blocks_production: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-RELEASE-HOLD",
        &[
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(&case.release_hold_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(replay_result_root),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
        ],
        32,
    )
}

fn step_id(step_kind: ReplayStepKind, ordinal: u64, replay_result_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-STEP-ID",
        &[
            HashPart::Str(step_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(replay_result_root),
        ],
        16,
    )
}

fn expected_outcome(status: ReplayStepStatus, step_kind: ReplayStepKind) -> &'static str {
    match status {
        ReplayStepStatus::ReadyToReplay => {
            "live receipt replay should match the accepted observation root"
        }
        ReplayStepStatus::DeferredUntilLiveReceipt => match step_kind {
            ReplayStepKind::WalletScannerPrivacy => {
                "wallet must wait for roots-only scanner receipt before claiming live escape"
            }
            ReplayStepKind::PqAuthorityQuorum => {
                "operator must provide PQ authority receipt before replay can pass"
            }
            ReplayStepKind::SettlementReceiptExecutor => {
                "settlement receipt must be observed before withdrawal evidence can pass"
            }
            _ => "live receipt must be observed before this replay step can pass",
        },
        ReplayStepStatus::ProductionHold => {
            "replay keeps production held until blocker-clearing receipt is observed"
        }
        ReplayStepStatus::FailClosed => {
            "replay fails closed and preserves wallet-visible escape evidence"
        }
    }
}

fn replay_step_vector_root(steps: &[LiveReceiptReplayStep]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-STEPS",
        &steps
            .iter()
            .map(LiveReceiptReplayStep::public_record)
            .collect::<Vec<_>>(),
    )
}

fn command_vector_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[LiveReceiptReplayStep],
    verdict: &LiveReceiptReplayVerdict,
) -> String {
    let commands = steps
        .iter()
        .map(|step| {
            json!({
                "step_id": step.step_id,
                "step_kind": step.step_kind.as_str(),
                "command_root": step.command_root,
                "replay_command": step.replay_command,
            })
        })
        .collect::<Vec<_>>();
    let command_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-COMMANDS",
        &commands,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-COMMAND-VECTOR",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.acceptance_state_root),
            HashPart::Str(&command_root),
            HashPart::U64(verdict.replay_step_count),
        ],
        32,
    )
}

fn wallet_replay_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[LiveReceiptReplayStep],
    verdict: &LiveReceiptReplayVerdict,
) -> String {
    let wallet_steps = steps
        .iter()
        .filter(|step| step.user_escape_step)
        .map(|step| {
            json!({
                "step_id": step.step_id,
                "step_kind": step.step_kind.as_str(),
                "wallet_instruction_root": step.wallet_instruction_root,
                "blocks_user_release": step.blocks_user_release,
            })
        })
        .collect::<Vec<_>>();
    let wallet_step_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-WALLET-STEPS",
        &wallet_steps,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-WALLET",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.user_escape_receipt_root),
            HashPart::Str(&wallet_step_root),
            HashPart::U64(verdict.user_release_blocker_count),
            HashPart::Str(bool_str(verdict.user_escape_replay_executable)),
        ],
        32,
    )
}

fn fail_closed_replay_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[LiveReceiptReplayStep],
    verdict: &LiveReceiptReplayVerdict,
) -> String {
    let failed = steps
        .iter()
        .filter(|step| step.status == ReplayStepStatus::FailClosed || step.blocks_user_release)
        .map(|step| {
            json!({
                "step_id": step.step_id,
                "step_kind": step.step_kind.as_str(),
                "fail_closed_receipt_root": step.fail_closed_receipt_root,
                "release_hold_root": step.release_hold_root,
            })
        })
        .collect::<Vec<_>>();
    let failed_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-FAIL-CLOSED-STEPS",
        &failed,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-FAIL-CLOSED",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(&failed_root),
            HashPart::U64(verdict.fail_closed_step_count),
            HashPart::U64(verdict.user_release_blocker_count),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    steps: &[LiveReceiptReplayStep],
    verdict: &LiveReceiptReplayVerdict,
) -> String {
    let blockers = steps
        .iter()
        .filter(|step| step.blocks_production)
        .map(|step| {
            json!({
                "step_id": step.step_id,
                "step_kind": step.step_kind.as_str(),
                "status": step.status.as_str(),
                "release_hold_root": step.release_hold_root,
                "expected_outcome": step.expected_outcome,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.production_hold_root),
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
    replay_step_root: &str,
    command_vector_root: &str,
    wallet_replay_root: &str,
    fail_closed_replay_root: &str,
    production_hold_root: &str,
    verdict: &LiveReceiptReplayVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(replay_step_root),
            HashPart::Str(command_vector_root),
            HashPart::Str(wallet_replay_root),
            HashPart::Str(fail_closed_replay_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    replay_step_count: u64,
    ready_step_count: u64,
    deferred_step_count: u64,
    production_hold_step_count: u64,
    fail_closed_step_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    all_replay_steps_present: bool,
    replay_executable: bool,
    user_escape_replay_executable: bool,
    production_blocked: bool,
    replay_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-VERDICT",
        &[
            HashPart::Str(&config.replay_harness_suite),
            HashPart::Str(&source.acceptance_state_root),
            HashPart::Str(&source.observed_receipt_acceptance_root),
            HashPart::U64(replay_step_count),
            HashPart::U64(ready_step_count),
            HashPart::U64(deferred_step_count),
            HashPart::U64(production_hold_step_count),
            HashPart::U64(fail_closed_step_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(all_replay_steps_present)),
            HashPart::Str(bool_str(replay_executable)),
            HashPart::Str(bool_str(user_escape_replay_executable)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(replay_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(steps: &[LiveReceiptReplayStep], status: ReplayStepStatus) -> u64 {
    steps.iter().filter(|step| step.status == status).count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "live receipt replay harness chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "live receipt replay harness protocol mismatch",
    )?;
    ensure(
        config.min_replay_steps > 0,
        "live receipt replay harness requires replay steps",
    )?;
    ensure(
        config.min_user_escape_steps > 0,
        "live receipt replay harness requires user escape steps",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.acceptance_state_root.is_empty(),
        "live receipt replay harness missing acceptance state root",
    )?;
    ensure(
        !source.observed_receipt_acceptance_root.is_empty(),
        "live receipt replay harness missing observed receipt acceptance root",
    )?;
    ensure(
        source.acceptance_case_count > 0,
        "live receipt replay harness missing acceptance cases",
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
        acceptance_state_root: record_root(
            "fallback-acceptance-state",
            &json!({"reason": &reason}),
        ),
        acceptance_case_root: record_root("fallback-acceptance-case", &json!({"reason": &reason})),
        observed_receipt_acceptance_root: record_root(
            "fallback-observed-acceptance",
            &json!({"reason": &reason}),
        ),
        user_escape_receipt_root: record_root("fallback-user-escape", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        acceptance_status: "fallback".to_string(),
        acceptance_user_escape_answer: reason.clone(),
        acceptance_production_answer: "fallback".to_string(),
        acceptance_case_count: 1,
        accepted_case_count: 0,
        deferred_case_count: 0,
        production_hold_case_count: 0,
        fail_closed_case_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        user_escape_receipts_sufficient: false,
        live_receipts_sufficient: false,
        production_blocked: true,
    };
    let fallback_result = record_root("fallback-replay-result", &json!({"reason": &reason}));
    let fallback_step = LiveReceiptReplayStep {
        step_id: step_id(ReplayStepKind::ReleaseBlockerClearing, 1, &fallback_result),
        ordinal: 1,
        step_kind: ReplayStepKind::ReleaseBlockerClearing,
        acceptance_case_id: "fallback".to_string(),
        acceptance_status: "fallback".to_string(),
        command_root: source.acceptance_case_root.clone(),
        expected_observation_root: source.observed_receipt_acceptance_root.clone(),
        observed_input_root: source.observed_receipt_acceptance_root.clone(),
        wallet_instruction_root: source.user_escape_receipt_root.clone(),
        operator_instruction_root: source.acceptance_state_root.clone(),
        fail_closed_receipt_root: source.production_hold_root.clone(),
        replay_result_root: fallback_result,
        release_hold_root: source.production_hold_root.clone(),
        status: ReplayStepStatus::FailClosed,
        user_escape_step: true,
        blocks_user_release: true,
        blocks_production: true,
        replay_command: "fallback".to_string(),
        expected_outcome: reason,
    };
    let replay_steps = vec![fallback_step];
    let verdict = LiveReceiptReplayVerdict::new(&config, &source, &replay_steps);
    let replay_step_root = replay_step_vector_root(&replay_steps);
    let command_vector_root = command_vector_root(&config, &source, &replay_steps, &verdict);
    let wallet_replay_root = wallet_replay_root(&config, &source, &replay_steps, &verdict);
    let fail_closed_replay_root =
        fail_closed_replay_root(&config, &source, &replay_steps, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &replay_steps, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &replay_step_root,
        &command_vector_root,
        &wallet_replay_root,
        &fail_closed_replay_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        replay_steps,
        verdict,
        replay_step_root,
        command_vector_root,
        wallet_replay_root,
        fail_closed_replay_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-LIVE-RECEIPT-REPLAY-HARNESS-RECORD",
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
