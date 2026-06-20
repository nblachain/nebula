use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageSettlementObservationRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_SETTLEMENT_OBSERVATION_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-settlement-observation-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_SETTLEMENT_OBSERVATION_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const SETTLEMENT_OBSERVATION_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-settlement-observation-v1";
pub const DEFAULT_MIN_SETTLEMENT_OBSERVATIONS: u64 = 9;
pub const DEFAULT_MIN_USER_ESCAPE_SETTLEMENT_OBSERVATIONS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub settlement_observation_suite: String,
    pub min_settlement_observations: u64,
    pub min_user_escape_settlement_observations: u64,
    pub require_execution_observed: bool,
    pub require_settlement_evidence: bool,
    pub require_wallet_visible_settlement: bool,
    pub require_release_hold_evidence: bool,
    pub require_fail_closed_recovery_evidence: bool,
    pub hold_production_until_settlement_observed: bool,
    pub fail_closed_on_missing_settlement_evidence: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            settlement_observation_suite: SETTLEMENT_OBSERVATION_SUITE.to_string(),
            min_settlement_observations: DEFAULT_MIN_SETTLEMENT_OBSERVATIONS,
            min_user_escape_settlement_observations:
                DEFAULT_MIN_USER_ESCAPE_SETTLEMENT_OBSERVATIONS,
            require_execution_observed: true,
            require_settlement_evidence: true,
            require_wallet_visible_settlement: true,
            require_release_hold_evidence: true,
            require_fail_closed_recovery_evidence: true,
            hold_production_until_settlement_observed: true,
            fail_closed_on_missing_settlement_evidence: true,
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
            "settlement_observation_suite": self.settlement_observation_suite,
            "min_settlement_observations": self.min_settlement_observations,
            "min_user_escape_settlement_observations": self.min_user_escape_settlement_observations,
            "require_execution_observed": self.require_execution_observed,
            "require_settlement_evidence": self.require_settlement_evidence,
            "require_wallet_visible_settlement": self.require_wallet_visible_settlement,
            "require_release_hold_evidence": self.require_release_hold_evidence,
            "require_fail_closed_recovery_evidence": self.require_fail_closed_recovery_evidence,
            "hold_production_until_settlement_observed": self.hold_production_until_settlement_observed,
            "fail_closed_on_missing_settlement_evidence": self.fail_closed_on_missing_settlement_evidence,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_state_root: String,
    pub execution_receipt_root: String,
    pub observed_submission_bundle_root: String,
    pub challenge_settlement_bundle_root: String,
    pub recovery_receipt_root: String,
    pub production_hold_root: String,
    pub execution_status: String,
    pub execution_user_escape_answer: String,
    pub execution_production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub settlement_receipts_present: bool,
    pub package_execution_observed: bool,
    pub user_escape_execution_observed: bool,
    pub execution_production_blocked: bool,
    pub package_action_count: u64,
}

impl SourceBundle {
    pub fn from_execution(state: &execution::State) -> Self {
        Self {
            execution_state_root: state.state_root(),
            execution_receipt_root: state.execution_receipt_root.clone(),
            observed_submission_bundle_root: state.observed_submission_bundle_root.clone(),
            challenge_settlement_bundle_root: state.challenge_settlement_bundle_root.clone(),
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            execution_user_escape_answer: state.verdict.user_escape_answer.clone(),
            execution_production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            settlement_receipts_present: state.verdict.settlement_receipts_present,
            package_execution_observed: state.verdict.package_execution_observed,
            user_escape_execution_observed: state.verdict.user_escape_execution_observed,
            execution_production_blocked: state.verdict.production_blocked,
            package_action_count: state.verdict.package_action_count,
        }
    }

    pub fn devnet() -> Self {
        let state = execution::devnet();
        Self::from_execution(&state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "observed_submission_bundle_root": self.observed_submission_bundle_root,
            "challenge_settlement_bundle_root": self.challenge_settlement_bundle_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "production_hold_root": self.production_hold_root,
            "execution_status": self.execution_status,
            "execution_user_escape_answer": self.execution_user_escape_answer,
            "execution_production_answer": self.execution_production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "settlement_receipts_present": self.settlement_receipts_present,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
            "package_action_count": self.package_action_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementObservationStatus {
    Observed,
    MissingEvidence,
    ExecutionDeferred,
    ReleaseHeld,
    FailClosed,
}

impl SettlementObservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::MissingEvidence => "missing_evidence",
            Self::ExecutionDeferred => "execution_deferred",
            Self::ReleaseHeld => "release_held",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementObservationRecord {
    pub observation_id: String,
    pub ordinal: u64,
    pub receipt_id: String,
    pub receipt_kind: String,
    pub package_action_id: String,
    pub execution_receipt_root: String,
    pub settlement_receipt_root: String,
    pub observed_settlement_root: String,
    pub settlement_evidence_root: String,
    pub release_hold_evidence_root: String,
    pub fail_closed_evidence_root: String,
    pub status: SettlementObservationStatus,
    pub user_escape_observation: bool,
    pub wallet_visible: bool,
    pub evidence_present: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub required_outcome: String,
    pub observation_root: String,
}

impl SettlementObservationRecord {
    pub fn from_execution_receipt(
        config: &Config,
        source: &SourceBundle,
        receipt: &execution::ForceExitExecutionReceipt,
    ) -> Self {
        let evidence_present = settlement_evidence_present(config, source, receipt);
        let status = observation_status(config, source, receipt, evidence_present);
        let settlement_evidence_root =
            settlement_evidence_root(config, source, receipt, evidence_present);
        let release_hold_evidence_root =
            release_hold_evidence_root(config, source, receipt, status);
        let fail_closed_evidence_root = fail_closed_evidence_root(config, source, receipt, status);
        let observed_settlement_root = observed_settlement_root(
            config,
            source,
            receipt,
            status,
            evidence_present,
            &settlement_evidence_root,
        );
        let blocks_user_release =
            receipt.blocks_user_release || status == SettlementObservationStatus::FailClosed;
        let blocks_production =
            receipt.blocks_production || status != SettlementObservationStatus::Observed;
        let observation_root = observation_root(
            config,
            source,
            receipt,
            status,
            evidence_present,
            &observed_settlement_root,
            &settlement_evidence_root,
            &release_hold_evidence_root,
            &fail_closed_evidence_root,
            blocks_user_release,
            blocks_production,
        );
        let observation_id =
            observation_id(&receipt.receipt_id, receipt.ordinal, &observation_root);
        Self {
            observation_id,
            ordinal: receipt.ordinal,
            receipt_id: receipt.receipt_id.clone(),
            receipt_kind: receipt.receipt_kind.as_str().to_string(),
            package_action_id: receipt.package_action_id.clone(),
            execution_receipt_root: receipt.execution_receipt_root.clone(),
            settlement_receipt_root: receipt.settlement_receipt_root.clone(),
            observed_settlement_root,
            settlement_evidence_root,
            release_hold_evidence_root,
            fail_closed_evidence_root,
            status,
            user_escape_observation: receipt.user_escape_receipt,
            wallet_visible: receipt.user_escape_receipt,
            evidence_present,
            blocks_user_release,
            blocks_production,
            required_outcome: required_outcome(status).to_string(),
            observation_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_id": self.observation_id,
            "ordinal": self.ordinal,
            "receipt_id": self.receipt_id,
            "receipt_kind": self.receipt_kind,
            "package_action_id": self.package_action_id,
            "execution_receipt_root": self.execution_receipt_root,
            "settlement_receipt_root": self.settlement_receipt_root,
            "observed_settlement_root": self.observed_settlement_root,
            "settlement_evidence_root": self.settlement_evidence_root,
            "release_hold_evidence_root": self.release_hold_evidence_root,
            "fail_closed_evidence_root": self.fail_closed_evidence_root,
            "status": self.status.as_str(),
            "user_escape_observation": self.user_escape_observation,
            "wallet_visible": self.wallet_visible,
            "evidence_present": self.evidence_present,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "required_outcome": self.required_outcome,
            "observation_root": self.observation_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.observation_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SettlementObservationVerdict {
    pub settlement_observation_count: u64,
    pub observed_settlement_count: u64,
    pub missing_evidence_count: u64,
    pub execution_deferred_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub user_escape_observation_count: u64,
    pub wallet_visible_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub execution_receipt_count: u64,
    pub package_action_count: u64,
    pub settlement_evidence_present: bool,
    pub settlement_observed: bool,
    pub user_escape_settlement_observed: bool,
    pub production_blocked: bool,
    pub observation_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl SettlementObservationVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        observations: &[SettlementObservationRecord],
    ) -> Self {
        let settlement_observation_count = observations.len() as u64;
        let observed_settlement_count =
            count_status(observations, SettlementObservationStatus::Observed);
        let missing_evidence_count =
            count_status(observations, SettlementObservationStatus::MissingEvidence);
        let execution_deferred_count =
            count_status(observations, SettlementObservationStatus::ExecutionDeferred);
        let release_held_count =
            count_status(observations, SettlementObservationStatus::ReleaseHeld);
        let fail_closed_count = count_status(observations, SettlementObservationStatus::FailClosed);
        let user_escape_observation_count = observations
            .iter()
            .filter(|observation| observation.user_escape_observation)
            .count() as u64;
        let wallet_visible_count = observations
            .iter()
            .filter(|observation| observation.wallet_visible)
            .count() as u64;
        let user_release_blocker_count = observations
            .iter()
            .filter(|observation| observation.blocks_user_release)
            .count() as u64;
        let production_blocker_count = observations
            .iter()
            .filter(|observation| observation.blocks_production)
            .count() as u64;
        let settlement_evidence_present = !config.require_settlement_evidence
            || observations
                .iter()
                .all(|observation| observation.evidence_present);
        let execution_ready =
            !config.require_execution_observed || source.package_execution_observed;
        let settlement_observed = settlement_observation_count
            >= config.min_settlement_observations
            && settlement_observation_count == source.execution_receipt_count
            && source.execution_receipt_count == source.package_action_count
            && execution_ready
            && observed_settlement_count >= config.min_settlement_observations
            && missing_evidence_count == 0
            && execution_deferred_count == 0
            && release_held_count == 0
            && fail_closed_count == 0
            && settlement_evidence_present;
        let user_escape_settlement_observed = settlement_observed
            && user_escape_observation_count >= config.min_user_escape_settlement_observations
            && (!config.require_wallet_visible_settlement
                || wallet_visible_count >= config.min_user_escape_settlement_observations)
            && user_release_blocker_count == 0;
        let production_blocked = source.execution_production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_settlement_observed && !settlement_observed)
            || (config.fail_closed_on_missing_settlement_evidence && missing_evidence_count > 0);
        let observation_status = if fail_closed_count > 0
            || (config.fail_closed_on_missing_settlement_evidence && missing_evidence_count > 0)
        {
            "fail_closed"
        } else if release_held_count > 0 {
            "release_held"
        } else if execution_deferred_count > 0 {
            "execution_deferred"
        } else if missing_evidence_count > 0 {
            "missing_settlement_evidence"
        } else if settlement_observed {
            "settlement_observed"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_settlement_observed {
            "user escape settlement receipts are observed for every executed force-exit package action"
        } else {
            "user escape remains fail-closed until executed force-exit package settlement evidence is observed"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains held because settlement observation evidence is missing, deferred, held, or fail-closed"
        } else {
            "force-exit package settlement observation is complete for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            settlement_observation_count,
            observed_settlement_count,
            missing_evidence_count,
            execution_deferred_count,
            release_held_count,
            fail_closed_count,
            user_release_blocker_count,
            production_blocker_count,
            settlement_evidence_present,
            settlement_observed,
            user_escape_settlement_observed,
            production_blocked,
            &observation_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            settlement_observation_count,
            observed_settlement_count,
            missing_evidence_count,
            execution_deferred_count,
            release_held_count,
            fail_closed_count,
            user_escape_observation_count,
            wallet_visible_count,
            user_release_blocker_count,
            production_blocker_count,
            execution_receipt_count: source.execution_receipt_count,
            package_action_count: source.package_action_count,
            settlement_evidence_present,
            settlement_observed,
            user_escape_settlement_observed,
            production_blocked,
            observation_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "settlement_observation_count": self.settlement_observation_count,
            "observed_settlement_count": self.observed_settlement_count,
            "missing_evidence_count": self.missing_evidence_count,
            "execution_deferred_count": self.execution_deferred_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "user_escape_observation_count": self.user_escape_observation_count,
            "wallet_visible_count": self.wallet_visible_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "execution_receipt_count": self.execution_receipt_count,
            "package_action_count": self.package_action_count,
            "settlement_evidence_present": self.settlement_evidence_present,
            "settlement_observed": self.settlement_observed,
            "user_escape_settlement_observed": self.user_escape_settlement_observed,
            "production_blocked": self.production_blocked,
            "observation_status": self.observation_status,
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
    pub settlement_observations: Vec<SettlementObservationRecord>,
    pub verdict: SettlementObservationVerdict,
    pub settlement_observation_root: String,
    pub settlement_evidence_bundle_root: String,
    pub release_hold_bundle_root: String,
    pub fail_closed_bundle_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution(&execution_state);
        validate_source(&source)?;
        let settlement_observations = execution_state
            .execution_receipts
            .iter()
            .map(|receipt| {
                SettlementObservationRecord::from_execution_receipt(&config, &source, receipt)
            })
            .collect::<Vec<_>>();
        let verdict = SettlementObservationVerdict::new(&config, &source, &settlement_observations);
        let settlement_observation_root =
            settlement_observation_vector_root(&settlement_observations);
        let settlement_evidence_bundle_root =
            settlement_evidence_bundle_root(&config, &source, &settlement_observations, &verdict);
        let release_hold_bundle_root =
            release_hold_bundle_root(&config, &source, &settlement_observations, &verdict);
        let fail_closed_bundle_root =
            fail_closed_bundle_root(&config, &source, &settlement_observations, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &settlement_observations, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &settlement_observation_root,
            &settlement_evidence_bundle_root,
            &release_hold_bundle_root,
            &fail_closed_bundle_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            settlement_observations,
            verdict,
            settlement_observation_root,
            settlement_evidence_bundle_root,
            release_hold_bundle_root,
            fail_closed_bundle_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), execution::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_settlement_observation_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "settlement_observation_root": self.settlement_observation_root,
            "settlement_evidence_bundle_root": self.settlement_evidence_bundle_root,
            "release_hold_bundle_root": self.release_hold_bundle_root,
            "fail_closed_bundle_root": self.fail_closed_bundle_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "settlement_observations": self
                .settlement_observations
                .iter()
                .map(SettlementObservationRecord::public_record)
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

fn settlement_evidence_present(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
) -> bool {
    (!config.require_execution_observed
        || receipt.status == execution::ExecutionReceiptStatus::Observed)
        && (!config.require_settlement_evidence || source.settlement_receipts_present)
        && !receipt.settlement_receipt_root.is_empty()
}

fn observation_status(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    evidence_present: bool,
) -> SettlementObservationStatus {
    match receipt.status {
        execution::ExecutionReceiptStatus::Observed => {
            if evidence_present && source.package_execution_observed {
                SettlementObservationStatus::Observed
            } else if config.fail_closed_on_missing_settlement_evidence {
                SettlementObservationStatus::FailClosed
            } else {
                SettlementObservationStatus::MissingEvidence
            }
        }
        execution::ExecutionReceiptStatus::DeferredUntilLiveSubmission => {
            SettlementObservationStatus::ExecutionDeferred
        }
        execution::ExecutionReceiptStatus::ReleaseHeld => SettlementObservationStatus::ReleaseHeld,
        execution::ExecutionReceiptStatus::FailClosed => SettlementObservationStatus::FailClosed,
    }
}

fn settlement_evidence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    evidence_present: bool,
) -> String {
    record_root(
        "settlement-evidence",
        &json!({
            "settlement_observation_suite": &config.settlement_observation_suite,
            "receipt_id": &receipt.receipt_id,
            "receipt_kind": receipt.receipt_kind.as_str(),
            "package_action_id": &receipt.package_action_id,
            "execution_receipt_root": &receipt.execution_receipt_root,
            "settlement_receipt_root": &receipt.settlement_receipt_root,
            "challenge_settlement_bundle_root": &source.challenge_settlement_bundle_root,
            "evidence_present": evidence_present,
            "policy": "executed_force_exit_package_action_requires_wallet_visible_settlement_evidence",
        }),
    )
}

fn release_hold_evidence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    status: SettlementObservationStatus,
) -> String {
    record_root(
        "release-hold-evidence",
        &json!({
            "required": config.require_release_hold_evidence,
            "receipt_id": &receipt.receipt_id,
            "status": status.as_str(),
            "release_hold_receipt_root": &receipt.release_hold_receipt_root,
            "source_production_hold_root": &source.production_hold_root,
            "blocks_user_release": receipt.blocks_user_release,
            "blocks_production": receipt.blocks_production,
        }),
    )
}

fn fail_closed_evidence_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    status: SettlementObservationStatus,
) -> String {
    record_root(
        "fail-closed-evidence",
        &json!({
            "required": config.require_fail_closed_recovery_evidence,
            "receipt_id": &receipt.receipt_id,
            "status": status.as_str(),
            "fail_closed_receipt_root": &receipt.fail_closed_receipt_root,
            "recovery_receipt_root": &source.recovery_receipt_root,
            "policy": "missing_settlement_evidence_keeps_recovery_package_authoritative",
        }),
    )
}

fn observed_settlement_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    status: SettlementObservationStatus,
    evidence_present: bool,
    settlement_evidence_root: &str,
) -> String {
    if status == SettlementObservationStatus::Observed {
        settlement_evidence_root.to_string()
    } else {
        record_root(
            "deferred-observed-settlement",
            &json!({
                "settlement_observation_suite": &config.settlement_observation_suite,
                "receipt_id": &receipt.receipt_id,
                "package_action_id": &receipt.package_action_id,
                "status": status.as_str(),
                "execution_status": &source.execution_status,
                "evidence_present": evidence_present,
                "production_hold_root": &source.production_hold_root,
                "reason": "live settlement evidence is not accepted for this executed force-exit package action",
            }),
        )
    }
}

fn observation_root(
    config: &Config,
    source: &SourceBundle,
    receipt: &execution::ForceExitExecutionReceipt,
    status: SettlementObservationStatus,
    evidence_present: bool,
    observed_settlement_root: &str,
    settlement_evidence_root: &str,
    release_hold_evidence_root: &str,
    fail_closed_evidence_root: &str,
    blocks_user_release: bool,
    blocks_production: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATION",
        &[
            HashPart::Str(&config.settlement_observation_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&receipt.receipt_id),
            HashPart::Str(receipt.receipt_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&receipt.execution_receipt_root),
            HashPart::Str(&receipt.settlement_receipt_root),
            HashPart::Str(observed_settlement_root),
            HashPart::Str(settlement_evidence_root),
            HashPart::Str(release_hold_evidence_root),
            HashPart::Str(fail_closed_evidence_root),
            HashPart::Str(bool_str(evidence_present)),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
        ],
        32,
    )
}

fn observation_id(receipt_id: &str, ordinal: u64, observation_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATION-ID",
        &[
            HashPart::Str(receipt_id),
            HashPart::U64(ordinal),
            HashPart::Str(observation_root),
        ],
        16,
    )
}

fn required_outcome(status: SettlementObservationStatus) -> &'static str {
    match status {
        SettlementObservationStatus::Observed => {
            "settlement evidence is observed for the executed force-exit package action"
        }
        SettlementObservationStatus::MissingEvidence => {
            "wait for wallet-visible settlement evidence before release"
        }
        SettlementObservationStatus::ExecutionDeferred => {
            "wait for the force-exit package action execution receipt before observing settlement"
        }
        SettlementObservationStatus::ReleaseHeld => {
            "release remains held while settlement observation continues"
        }
        SettlementObservationStatus::FailClosed => {
            "fail closed and keep recovery evidence authoritative until settlement evidence is present"
        }
    }
}

fn settlement_observation_vector_root(observations: &[SettlementObservationRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATIONS",
        &observations
            .iter()
            .map(SettlementObservationRecord::public_record)
            .collect::<Vec<_>>(),
    )
}

fn settlement_evidence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[SettlementObservationRecord],
    verdict: &SettlementObservationVerdict,
) -> String {
    let records = observations
        .iter()
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "receipt_id": &observation.receipt_id,
                "settlement_evidence_root": &observation.settlement_evidence_root,
                "observed_settlement_root": &observation.observed_settlement_root,
                "status": observation.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let evidence_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-EVIDENCE",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-EVIDENCE-BUNDLE",
        &[
            HashPart::Str(&config.settlement_observation_suite),
            HashPart::Str(&source.challenge_settlement_bundle_root),
            HashPart::Str(&evidence_root),
            HashPart::U64(verdict.observed_settlement_count),
            HashPart::U64(verdict.missing_evidence_count),
            HashPart::Str(bool_str(verdict.settlement_evidence_present)),
        ],
        32,
    )
}

fn release_hold_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[SettlementObservationRecord],
    verdict: &SettlementObservationVerdict,
) -> String {
    let records = observations
        .iter()
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "release_hold_evidence_root": &observation.release_hold_evidence_root,
                "blocks_user_release": observation.blocks_user_release,
                "blocks_production": observation.blocks_production,
            })
        })
        .collect::<Vec<_>>();
    let hold_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-RELEASE-HOLDS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-RELEASE-HOLD-BUNDLE",
        &[
            HashPart::Str(&config.settlement_observation_suite),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(&hold_root),
            HashPart::U64(verdict.user_release_blocker_count),
            HashPart::U64(verdict.production_blocker_count),
        ],
        32,
    )
}

fn fail_closed_bundle_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[SettlementObservationRecord],
    verdict: &SettlementObservationVerdict,
) -> String {
    let records = observations
        .iter()
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "fail_closed_evidence_root": &observation.fail_closed_evidence_root,
                "status": observation.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let fail_closed_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-FAIL-CLOSED",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-FAIL-CLOSED-BUNDLE",
        &[
            HashPart::Str(&config.settlement_observation_suite),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&fail_closed_root),
            HashPart::U64(verdict.fail_closed_count),
            HashPart::U64(verdict.missing_evidence_count),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    observations: &[SettlementObservationRecord],
    verdict: &SettlementObservationVerdict,
) -> String {
    let records = observations
        .iter()
        .filter(|observation| observation.blocks_production)
        .map(|observation| {
            json!({
                "observation_id": &observation.observation_id,
                "receipt_id": &observation.receipt_id,
                "status": observation.status.as_str(),
                "release_hold_evidence_root": &observation.release_hold_evidence_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-PRODUCTION-BLOCKERS",
        &records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.settlement_observation_suite),
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
    settlement_observation_root: &str,
    settlement_evidence_bundle_root: &str,
    release_hold_bundle_root: &str,
    fail_closed_bundle_root: &str,
    production_hold_root: &str,
    verdict: &SettlementObservationVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATION-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(settlement_observation_root),
            HashPart::Str(settlement_evidence_bundle_root),
            HashPart::Str(release_hold_bundle_root),
            HashPart::Str(fail_closed_bundle_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    settlement_observation_count: u64,
    observed_settlement_count: u64,
    missing_evidence_count: u64,
    execution_deferred_count: u64,
    release_held_count: u64,
    fail_closed_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    settlement_evidence_present: bool,
    settlement_observed: bool,
    user_escape_settlement_observed: bool,
    production_blocked: bool,
    observation_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATION-VERDICT",
        &[
            HashPart::Str(&config.settlement_observation_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::U64(settlement_observation_count),
            HashPart::U64(observed_settlement_count),
            HashPart::U64(missing_evidence_count),
            HashPart::U64(execution_deferred_count),
            HashPart::U64(release_held_count),
            HashPart::U64(fail_closed_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(settlement_evidence_present)),
            HashPart::Str(bool_str(settlement_observed)),
            HashPart::Str(bool_str(user_escape_settlement_observed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(observation_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(
    observations: &[SettlementObservationRecord],
    status: SettlementObservationStatus,
) -> u64 {
    observations
        .iter()
        .filter(|observation| observation.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "force-exit settlement observation chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "force-exit settlement observation protocol mismatch",
    )?;
    ensure(
        config.min_settlement_observations > 0,
        "force-exit settlement observation requires observations",
    )?;
    ensure(
        config.min_user_escape_settlement_observations > 0,
        "force-exit settlement observation requires user escape observations",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_state_root.is_empty(),
        "force-exit settlement observation missing execution state root",
    )?;
    ensure(
        source.execution_receipt_count > 0,
        "force-exit settlement observation missing execution receipts",
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
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        execution_status: "fallback".to_string(),
        execution_user_escape_answer: reason.clone(),
        execution_production_answer: "fallback".to_string(),
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        settlement_receipts_present: false,
        package_execution_observed: false,
        user_escape_execution_observed: false,
        execution_production_blocked: true,
        package_action_count: 1,
    };
    let observation_root = record_root(
        "fallback-settlement-observation",
        &json!({
            "reason": &reason,
            "policy": "fallback settlement observation fails closed",
        }),
    );
    let settlement_observations = vec![SettlementObservationRecord {
        observation_id: observation_id("fallback", 1, &observation_root),
        ordinal: 1,
        receipt_id: "fallback".to_string(),
        receipt_kind: "fail_closed_recovery".to_string(),
        package_action_id: "fallback".to_string(),
        execution_receipt_root: source.execution_receipt_root.clone(),
        settlement_receipt_root: record_root(
            "fallback-missing-settlement",
            &json!({"reason": &reason}),
        ),
        observed_settlement_root: record_root(
            "fallback-observed-settlement",
            &json!({"reason": &reason}),
        ),
        settlement_evidence_root: record_root(
            "fallback-settlement-evidence",
            &json!({"reason": &reason}),
        ),
        release_hold_evidence_root: source.production_hold_root.clone(),
        fail_closed_evidence_root: source.recovery_receipt_root.clone(),
        status: SettlementObservationStatus::FailClosed,
        user_escape_observation: true,
        wallet_visible: true,
        evidence_present: false,
        blocks_user_release: true,
        blocks_production: true,
        required_outcome: required_outcome(SettlementObservationStatus::FailClosed).to_string(),
        observation_root,
    }];
    let verdict = SettlementObservationVerdict::new(&config, &source, &settlement_observations);
    let settlement_observation_root = settlement_observation_vector_root(&settlement_observations);
    let settlement_evidence_bundle_root =
        settlement_evidence_bundle_root(&config, &source, &settlement_observations, &verdict);
    let release_hold_bundle_root =
        release_hold_bundle_root(&config, &source, &settlement_observations, &verdict);
    let fail_closed_bundle_root =
        fail_closed_bundle_root(&config, &source, &settlement_observations, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &settlement_observations, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &settlement_observation_root,
        &settlement_evidence_bundle_root,
        &release_hold_bundle_root,
        &fail_closed_bundle_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        settlement_observations,
        verdict,
        settlement_observation_root,
        settlement_evidence_bundle_root,
        release_hold_bundle_root,
        fail_closed_bundle_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-SETTLEMENT-OBSERVATION-RECORD",
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
