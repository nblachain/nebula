use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_monitor_runtime as challenge_window,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_pq_authority_receipt_verifier_runtime as pq_authority,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_recovery_playbook_receipt_runtime as recovery,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_release_hold_clearance_receipt_runtime as hold_clearance,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_reserve_fallback_observation_runtime as reserve,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_settlement_observation_runtime as settlement,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_wallet_scan_receipt_observer_runtime as wallet_scan,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCustodyReleasePolicyRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CUSTODY_RELEASE_POLICY_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-custody-release-policy-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CUSTODY_RELEASE_POLICY_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const CUSTODY_RELEASE_POLICY_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-custody-release-policy-v1";
pub const DEFAULT_MIN_POLICY_RECORDS: u64 = 7;
pub const DEFAULT_MIN_RELEASE_ROOTS: u64 = 7;
pub const DEFAULT_RELEASE_DELAY_BLOCKS: u64 = 720;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub policy_suite: String,
    pub min_policy_records: u64,
    pub min_release_roots: u64,
    pub release_delay_blocks: u64,
    pub require_pq_quorum: bool,
    pub require_reserve_root: bool,
    pub require_settlement_root: bool,
    pub require_challenge_window_root: bool,
    pub require_wallet_scan_root: bool,
    pub require_recovery_root: bool,
    pub require_hold_clearance_root: bool,
    pub require_operator_independent_custody: bool,
    pub require_zero_user_release_blockers: bool,
    pub require_zero_production_blockers: bool,
    pub lock_value_until_release_policy_moves: bool,
    pub hold_production_until_custody_release_clear: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            policy_suite: CUSTODY_RELEASE_POLICY_SUITE.to_string(),
            min_policy_records: DEFAULT_MIN_POLICY_RECORDS,
            min_release_roots: DEFAULT_MIN_RELEASE_ROOTS,
            release_delay_blocks: DEFAULT_RELEASE_DELAY_BLOCKS,
            require_pq_quorum: true,
            require_reserve_root: true,
            require_settlement_root: true,
            require_challenge_window_root: true,
            require_wallet_scan_root: true,
            require_recovery_root: true,
            require_hold_clearance_root: true,
            require_operator_independent_custody: true,
            require_zero_user_release_blockers: true,
            require_zero_production_blockers: true,
            lock_value_until_release_policy_moves: true,
            hold_production_until_custody_release_clear: true,
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
            "policy_suite": self.policy_suite,
            "min_policy_records": self.min_policy_records,
            "min_release_roots": self.min_release_roots,
            "release_delay_blocks": self.release_delay_blocks,
            "require_pq_quorum": self.require_pq_quorum,
            "require_reserve_root": self.require_reserve_root,
            "require_settlement_root": self.require_settlement_root,
            "require_challenge_window_root": self.require_challenge_window_root,
            "require_wallet_scan_root": self.require_wallet_scan_root,
            "require_recovery_root": self.require_recovery_root,
            "require_hold_clearance_root": self.require_hold_clearance_root,
            "require_operator_independent_custody": self.require_operator_independent_custody,
            "require_zero_user_release_blockers": self.require_zero_user_release_blockers,
            "require_zero_production_blockers": self.require_zero_production_blockers,
            "lock_value_until_release_policy_moves": self.lock_value_until_release_policy_moves,
            "hold_production_until_custody_release_clear": self.hold_production_until_custody_release_clear,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub pq_authority_state_root: String,
    pub pq_quorum_receipt_root: String,
    pub pq_release_hold_root: String,
    pub reserve_state_root: String,
    pub reserve_observation_root: String,
    pub reserve_coverage_root: String,
    pub reserve_production_hold_root: String,
    pub settlement_state_root: String,
    pub settlement_observation_root: String,
    pub settlement_evidence_bundle_root: String,
    pub settlement_production_hold_root: String,
    pub challenge_state_root: String,
    pub challenge_deadline_bundle_root: String,
    pub challenge_fail_closed_hold_root: String,
    pub wallet_scan_state_root: String,
    pub wallet_scan_receipt_root: String,
    pub wallet_scan_readiness_root: String,
    pub recovery_state_root: String,
    pub recovery_step_root: String,
    pub recovery_fail_closed_hold_root: String,
    pub hold_clearance_state_root: String,
    pub hold_clearance_receipt_root: String,
    pub hold_clearance_unresolved_blocker_root: String,
    pub hold_clearance_production_root: String,
    pub pq_quorum_verified: bool,
    pub reserve_ready: bool,
    pub settlement_observed: bool,
    pub challenge_window_clear: bool,
    pub wallet_scan_ready: bool,
    pub recovery_ready: bool,
    pub hold_clearance_ready: bool,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
}

impl SourceBundle {
    pub fn from_states(
        pq: &pq_authority::State,
        reserve_state: &reserve::State,
        settlement_state: &settlement::State,
        challenge: &challenge_window::State,
        scan: &wallet_scan::State,
        recovery_state: &recovery::State,
        clearance: &hold_clearance::State,
    ) -> Self {
        let pq_quorum_verified = pq.verdict.quorum_verified;
        let reserve_ready = reserve_state.verdict.reserve_covered
            && reserve_state.verdict.fallback_payout_roots_present
            && reserve_state.verdict.settlement_reserve_evidence_present;
        let settlement_observed = settlement_state.verdict.settlement_observed
            && settlement_state.verdict.settlement_evidence_present;
        let challenge_window_clear = challenge.verdict.challenge_window_clear
            && challenge.verdict.user_escape_deadline_present;
        let wallet_scan_ready = scan.verdict.wallet_scan_ready
            && scan.verdict.user_escape_ready_count > 0
            && scan.verdict.encrypted_scan_bundle_present;
        let recovery_ready = recovery_state.verdict.wallet_recovery_answerable
            && recovery_state.verdict.evidence_roots_present
            && recovery_state.verdict.deadline_receipts_present;
        let hold_clearance_ready = clearance.verdict.all_required_roots_present
            && clearance.verdict.clearable_count >= clearance.verdict.clearance_receipt_count
            && clearance.verdict.user_release_blocker_count == 0;
        let user_release_blocker_count = pq.verdict.fail_closed_count
            + reserve_state.verdict.evidence_missing_count
            + settlement_state.verdict.user_release_blocker_count
            + recovery_state.verdict.user_release_blocker_count
            + clearance.verdict.user_release_blocker_count;
        let production_blocker_count = pq.verdict.production_blocker_count
            + reserve_state.verdict.production_blocker_count
            + settlement_state.verdict.production_blocker_count
            + challenge.verdict.sustained_objection_count
            + scan.verdict.release_held_count
            + recovery_state.verdict.production_blocker_count
            + clearance.verdict.production_blocker_count;
        Self {
            pq_authority_state_root: pq.state_root(),
            pq_quorum_receipt_root: pq.quorum_receipt_root.clone(),
            pq_release_hold_root: pq.release_hold_root.clone(),
            reserve_state_root: reserve_state.state_root(),
            reserve_observation_root: reserve_state.reserve_observation_root.clone(),
            reserve_coverage_root: reserve_state.reserve_coverage_root.clone(),
            reserve_production_hold_root: reserve_state.production_hold_root.clone(),
            settlement_state_root: settlement_state.state_root(),
            settlement_observation_root: settlement_state.settlement_observation_root.clone(),
            settlement_evidence_bundle_root: settlement_state
                .settlement_evidence_bundle_root
                .clone(),
            settlement_production_hold_root: settlement_state.production_hold_root.clone(),
            challenge_state_root: challenge.state_root(),
            challenge_deadline_bundle_root: challenge.deadline_bundle_root.clone(),
            challenge_fail_closed_hold_root: challenge.fail_closed_hold_root.clone(),
            wallet_scan_state_root: scan.state_root(),
            wallet_scan_receipt_root: scan.wallet_scan_receipt_root.clone(),
            wallet_scan_readiness_root: scan.user_escape_readiness_root.clone(),
            recovery_state_root: recovery_state.state_root(),
            recovery_step_root: recovery_state.recovery_step_root.clone(),
            recovery_fail_closed_hold_root: recovery_state.fail_closed_hold_root.clone(),
            hold_clearance_state_root: clearance.state_root(),
            hold_clearance_receipt_root: clearance.clearance_receipt_root.clone(),
            hold_clearance_unresolved_blocker_root: clearance.unresolved_blocker_root.clone(),
            hold_clearance_production_root: clearance.production_hold_clearance_root.clone(),
            pq_quorum_verified,
            reserve_ready,
            settlement_observed,
            challenge_window_clear,
            wallet_scan_ready,
            recovery_ready,
            hold_clearance_ready,
            user_release_blocker_count,
            production_blocker_count,
        }
    }

    pub fn devnet() -> Self {
        let pq = pq_authority::devnet();
        let reserve_state = reserve::devnet();
        let settlement_state = settlement::devnet();
        let challenge = challenge_window::devnet();
        let scan = wallet_scan::devnet();
        let recovery_state = recovery::devnet();
        let clearance = hold_clearance::devnet();
        Self::from_states(
            &pq,
            &reserve_state,
            &settlement_state,
            &challenge,
            &scan,
            &recovery_state,
            &clearance,
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "pq_authority_state_root": self.pq_authority_state_root,
            "pq_quorum_receipt_root": self.pq_quorum_receipt_root,
            "pq_release_hold_root": self.pq_release_hold_root,
            "reserve_state_root": self.reserve_state_root,
            "reserve_observation_root": self.reserve_observation_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "reserve_production_hold_root": self.reserve_production_hold_root,
            "settlement_state_root": self.settlement_state_root,
            "settlement_observation_root": self.settlement_observation_root,
            "settlement_evidence_bundle_root": self.settlement_evidence_bundle_root,
            "settlement_production_hold_root": self.settlement_production_hold_root,
            "challenge_state_root": self.challenge_state_root,
            "challenge_deadline_bundle_root": self.challenge_deadline_bundle_root,
            "challenge_fail_closed_hold_root": self.challenge_fail_closed_hold_root,
            "wallet_scan_state_root": self.wallet_scan_state_root,
            "wallet_scan_receipt_root": self.wallet_scan_receipt_root,
            "wallet_scan_readiness_root": self.wallet_scan_readiness_root,
            "recovery_state_root": self.recovery_state_root,
            "recovery_step_root": self.recovery_step_root,
            "recovery_fail_closed_hold_root": self.recovery_fail_closed_hold_root,
            "hold_clearance_state_root": self.hold_clearance_state_root,
            "hold_clearance_receipt_root": self.hold_clearance_receipt_root,
            "hold_clearance_unresolved_blocker_root": self.hold_clearance_unresolved_blocker_root,
            "hold_clearance_production_root": self.hold_clearance_production_root,
            "pq_quorum_verified": self.pq_quorum_verified,
            "reserve_ready": self.reserve_ready,
            "settlement_observed": self.settlement_observed,
            "challenge_window_clear": self.challenge_window_clear,
            "wallet_scan_ready": self.wallet_scan_ready,
            "recovery_ready": self.recovery_ready,
            "hold_clearance_ready": self.hold_clearance_ready,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyActor {
    UserWallet,
    PqAuthorityQuorum,
    ReserveFallbackVault,
    SettlementObserver,
    RecoveryGuardian,
    HoldClearanceCommittee,
    PolicyEngine,
}

impl CustodyActor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserWallet => "user_wallet",
            Self::PqAuthorityQuorum => "pq_authority_quorum",
            Self::ReserveFallbackVault => "reserve_fallback_vault",
            Self::SettlementObserver => "settlement_observer",
            Self::RecoveryGuardian => "recovery_guardian",
            Self::HoldClearanceCommittee => "hold_clearance_committee",
            Self::PolicyEngine => "policy_engine",
        }
    }

    pub fn may_release_value(self) -> bool {
        matches!(self, Self::PolicyEngine)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyLockStatus {
    Locked,
    ReleaseEligible,
    ReleaseBlocked,
}

impl CustodyLockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Locked => "locked",
            Self::ReleaseEligible => "release_eligible",
            Self::ReleaseBlocked => "release_blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyStatus {
    Movable,
    HeldForMissingRoot,
    HeldForBlocker,
    HeldForUnauthorizedActor,
}

impl ReleasePolicyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Movable => "movable",
            Self::HeldForMissingRoot => "held_for_missing_root",
            Self::HeldForBlocker => "held_for_blocker",
            Self::HeldForUnauthorizedActor => "held_for_unauthorized_actor",
        }
    }

    pub fn moves_value(self) -> bool {
        self == Self::Movable
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodyLockRecord {
    pub lock_id: String,
    pub ordinal: u64,
    pub actor: CustodyActor,
    pub can_custody: bool,
    pub can_lock: bool,
    pub can_release: bool,
    pub locked_value_units: u64,
    pub custody_scope: String,
    pub custody_root: String,
    pub release_gate_root: String,
    pub status: CustodyLockStatus,
}

impl CustodyLockRecord {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        actor: CustodyActor,
        ordinal: u64,
    ) -> Self {
        let gates_clear = release_gates_clear(config, source);
        let can_release = actor.may_release_value() && gates_clear;
        let status = if can_release {
            CustodyLockStatus::ReleaseEligible
        } else if gates_clear {
            CustodyLockStatus::Locked
        } else {
            CustodyLockStatus::ReleaseBlocked
        };
        let custody_root = custody_actor_root(config, source, actor, ordinal);
        let release_gate_root = release_gate_root(config, source);
        Self {
            lock_id: format!("force-exit-custody-lock-{ordinal:02}"),
            ordinal,
            actor,
            can_custody: true,
            can_lock: true,
            can_release,
            locked_value_units: 1_000_000,
            custody_scope: "force_exit_package_value_cannot_move_until_release_policy_is_movable"
                .to_string(),
            custody_root,
            release_gate_root,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lock_id": self.lock_id,
            "ordinal": self.ordinal,
            "actor": self.actor.as_str(),
            "can_custody": self.can_custody,
            "can_lock": self.can_lock,
            "can_release": self.can_release,
            "locked_value_units": self.locked_value_units,
            "custody_scope": self.custody_scope,
            "custody_root": self.custody_root,
            "release_gate_root": self.release_gate_root,
            "status": self.status.as_str(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleasePolicyRecord {
    pub policy_id: String,
    pub ordinal: u64,
    pub actor: CustodyActor,
    pub pq_quorum_root: String,
    pub reserve_root: String,
    pub settlement_root: String,
    pub challenge_window_root: String,
    pub wallet_scan_root: String,
    pub recovery_root: String,
    pub hold_clearance_root: String,
    pub required_roots_present: bool,
    pub no_user_release_blockers: bool,
    pub no_production_blockers: bool,
    pub release_delay_blocks: u64,
    pub status: ReleasePolicyStatus,
    pub moves_value: bool,
    pub policy_root: String,
    pub required_outcome: String,
}

impl ReleasePolicyRecord {
    pub fn from_lock(config: &Config, source: &SourceBundle, lock: &CustodyLockRecord) -> Self {
        let required_roots_present = required_release_roots_present(config, source);
        let no_user_release_blockers =
            !config.require_zero_user_release_blockers || source.user_release_blocker_count == 0;
        let no_production_blockers =
            !config.require_zero_production_blockers || source.production_blocker_count == 0;
        let status = release_policy_status(
            lock.actor,
            required_roots_present,
            no_user_release_blockers,
            no_production_blockers,
        );
        let moves_value = status.moves_value();
        let policy_root = release_policy_root(config, source, lock, status);
        let required_outcome = if moves_value {
            "release policy may move force-exit package value after PQ, reserve, settlement, challenge, wallet scan, recovery, and hold-clearance roots agree"
        } else {
            "custody remains locked because release policy prerequisites are incomplete"
        }
        .to_string();
        Self {
            policy_id: format!("force-exit-release-policy-{:02}", lock.ordinal),
            ordinal: lock.ordinal,
            actor: lock.actor,
            pq_quorum_root: source.pq_quorum_receipt_root.clone(),
            reserve_root: source.reserve_coverage_root.clone(),
            settlement_root: source.settlement_observation_root.clone(),
            challenge_window_root: source.challenge_deadline_bundle_root.clone(),
            wallet_scan_root: source.wallet_scan_readiness_root.clone(),
            recovery_root: source.recovery_step_root.clone(),
            hold_clearance_root: source.hold_clearance_receipt_root.clone(),
            required_roots_present,
            no_user_release_blockers,
            no_production_blockers,
            release_delay_blocks: config.release_delay_blocks,
            status,
            moves_value,
            policy_root,
            required_outcome,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "policy_id": self.policy_id,
            "ordinal": self.ordinal,
            "actor": self.actor.as_str(),
            "pq_quorum_root": self.pq_quorum_root,
            "reserve_root": self.reserve_root,
            "settlement_root": self.settlement_root,
            "challenge_window_root": self.challenge_window_root,
            "wallet_scan_root": self.wallet_scan_root,
            "recovery_root": self.recovery_root,
            "hold_clearance_root": self.hold_clearance_root,
            "required_roots_present": self.required_roots_present,
            "no_user_release_blockers": self.no_user_release_blockers,
            "no_production_blockers": self.no_production_blockers,
            "release_delay_blocks": self.release_delay_blocks,
            "status": self.status.as_str(),
            "moves_value": self.moves_value,
            "policy_root": self.policy_root,
            "required_outcome": self.required_outcome,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodyReleaseVerdict {
    pub custody_record_count: u64,
    pub release_policy_count: u64,
    pub release_movable_count: u64,
    pub release_blocked_count: u64,
    pub unauthorized_release_count: u64,
    pub required_release_root_count: u64,
    pub required_roots_present: bool,
    pub pq_quorum_ready: bool,
    pub reserve_ready: bool,
    pub settlement_ready: bool,
    pub challenge_window_ready: bool,
    pub wallet_scan_ready: bool,
    pub recovery_ready: bool,
    pub hold_clearance_ready: bool,
    pub custody_locked: bool,
    pub value_can_move: bool,
    pub production_blocked: bool,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl CustodyReleaseVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        locks: &[CustodyLockRecord],
        policies: &[ReleasePolicyRecord],
    ) -> Self {
        let release_movable_count =
            policies.iter().filter(|policy| policy.moves_value).count() as u64;
        let release_blocked_count =
            policies.iter().filter(|policy| !policy.moves_value).count() as u64;
        let unauthorized_release_count = policies
            .iter()
            .filter(|policy| policy.status == ReleasePolicyStatus::HeldForUnauthorizedActor)
            .count() as u64;
        let required_roots_present = required_release_roots_present(config, source);
        let value_can_move = release_movable_count > 0
            && required_roots_present
            && source.user_release_blocker_count == 0
            && source.production_blocker_count == 0;
        let custody_locked = config.lock_value_until_release_policy_moves && !value_can_move;
        let production_blocked =
            config.hold_production_until_custody_release_clear && !value_can_move;
        let user_escape_answer = if value_can_move {
            "force_exit_package_custody_release_policy_can_move_user_value".to_string()
        } else {
            "force_exit_package_custody_release_policy_keeps_value_locked".to_string()
        };
        let production_answer = if production_blocked {
            "production_release_blocked_until_custody_release_policy_moves".to_string()
        } else {
            "production_release_custody_policy_clear".to_string()
        };
        let verdict_root = custody_release_verdict_root(
            config,
            source,
            locks,
            policies,
            value_can_move,
            production_blocked,
        );
        Self {
            custody_record_count: locks.len() as u64,
            release_policy_count: policies.len() as u64,
            release_movable_count,
            release_blocked_count,
            unauthorized_release_count,
            required_release_root_count: DEFAULT_MIN_RELEASE_ROOTS,
            required_roots_present,
            pq_quorum_ready: source.pq_quorum_verified,
            reserve_ready: source.reserve_ready,
            settlement_ready: source.settlement_observed,
            challenge_window_ready: source.challenge_window_clear,
            wallet_scan_ready: source.wallet_scan_ready,
            recovery_ready: source.recovery_ready,
            hold_clearance_ready: source.hold_clearance_ready,
            custody_locked,
            value_can_move,
            production_blocked,
            user_release_blocker_count: source.user_release_blocker_count,
            production_blocker_count: source.production_blocker_count,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "custody_record_count": self.custody_record_count,
            "release_policy_count": self.release_policy_count,
            "release_movable_count": self.release_movable_count,
            "release_blocked_count": self.release_blocked_count,
            "unauthorized_release_count": self.unauthorized_release_count,
            "required_release_root_count": self.required_release_root_count,
            "required_roots_present": self.required_roots_present,
            "pq_quorum_ready": self.pq_quorum_ready,
            "reserve_ready": self.reserve_ready,
            "settlement_ready": self.settlement_ready,
            "challenge_window_ready": self.challenge_window_ready,
            "wallet_scan_ready": self.wallet_scan_ready,
            "recovery_ready": self.recovery_ready,
            "hold_clearance_ready": self.hold_clearance_ready,
            "custody_locked": self.custody_locked,
            "value_can_move": self.value_can_move,
            "production_blocked": self.production_blocked,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
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
    pub custody_locks: Vec<CustodyLockRecord>,
    pub release_policies: Vec<ReleasePolicyRecord>,
    pub verdict: CustodyReleaseVerdict,
    pub custody_lock_root: String,
    pub release_policy_root: String,
    pub release_gate_bundle_root: String,
    pub custody_authority_root: String,
    pub blocked_release_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let actors = [
            CustodyActor::UserWallet,
            CustodyActor::PqAuthorityQuorum,
            CustodyActor::ReserveFallbackVault,
            CustodyActor::SettlementObserver,
            CustodyActor::RecoveryGuardian,
            CustodyActor::HoldClearanceCommittee,
            CustodyActor::PolicyEngine,
        ];
        let custody_locks = actors
            .iter()
            .enumerate()
            .map(|(index, actor)| {
                CustodyLockRecord::devnet(&config, &source, *actor, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let release_policies = custody_locks
            .iter()
            .map(|lock| ReleasePolicyRecord::from_lock(&config, &source, lock))
            .collect::<Vec<_>>();
        let verdict =
            CustodyReleaseVerdict::new(&config, &source, &custody_locks, &release_policies);
        let custody_lock_root = custody_lock_vector_root(&custody_locks);
        let release_policy_root = release_policy_vector_root(&release_policies);
        let release_gate_bundle_root = release_gate_bundle_root(&config, &source, &verdict);
        let custody_authority_root =
            custody_authority_bundle_root(&config, &source, &custody_locks);
        let blocked_release_root = blocked_release_root(&config, &source, &release_policies);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &custody_lock_root,
            &release_policy_root,
            &release_gate_bundle_root,
            &custody_authority_root,
            &blocked_release_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            custody_locks,
            release_policies,
            verdict,
            custody_lock_root,
            release_policy_root,
            release_gate_bundle_root,
            custody_authority_root,
            blocked_release_root,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_custody_release_policy_runtime",
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "custody_locks": self
                .custody_locks
                .iter()
                .map(CustodyLockRecord::public_record)
                .collect::<Vec<_>>(),
            "release_policies": self
                .release_policies
                .iter()
                .map(ReleasePolicyRecord::public_record)
                .collect::<Vec<_>>(),
            "verdict": self.verdict.public_record(),
            "custody_lock_root": self.custody_lock_root,
            "release_policy_root": self.release_policy_root,
            "release_gate_bundle_root": self.release_gate_bundle_root,
            "custody_authority_root": self.custody_authority_root,
            "blocked_release_root": self.blocked_release_root,
            "state_commitment_root": self.state_commitment_root,
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

fn release_policy_status(
    actor: CustodyActor,
    required_roots_present: bool,
    no_user_release_blockers: bool,
    no_production_blockers: bool,
) -> ReleasePolicyStatus {
    if !actor.may_release_value() {
        ReleasePolicyStatus::HeldForUnauthorizedActor
    } else if !required_roots_present {
        ReleasePolicyStatus::HeldForMissingRoot
    } else if !no_user_release_blockers || !no_production_blockers {
        ReleasePolicyStatus::HeldForBlocker
    } else {
        ReleasePolicyStatus::Movable
    }
}

fn release_gates_clear(config: &Config, source: &SourceBundle) -> bool {
    required_release_roots_present(config, source)
        && (!config.require_zero_user_release_blockers || source.user_release_blocker_count == 0)
        && (!config.require_zero_production_blockers || source.production_blocker_count == 0)
}

fn required_release_roots_present(config: &Config, source: &SourceBundle) -> bool {
    (!config.require_pq_quorum || source.pq_quorum_verified)
        && (!config.require_reserve_root || source.reserve_ready)
        && (!config.require_settlement_root || source.settlement_observed)
        && (!config.require_challenge_window_root || source.challenge_window_clear)
        && (!config.require_wallet_scan_root || source.wallet_scan_ready)
        && (!config.require_recovery_root || source.recovery_ready)
        && (!config.require_hold_clearance_root || source.hold_clearance_ready)
}

fn custody_actor_root(
    config: &Config,
    source: &SourceBundle,
    actor: CustodyActor,
    ordinal: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-ACTOR",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(actor.as_str()),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn release_gate_root(config: &Config, source: &SourceBundle) -> String {
    let root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RELEASE-GATES",
        &[
            source.pq_quorum_receipt_root.clone(),
            source.reserve_coverage_root.clone(),
            source.settlement_observation_root.clone(),
            source.challenge_deadline_bundle_root.clone(),
            source.wallet_scan_readiness_root.clone(),
            source.recovery_step_root.clone(),
            source.hold_clearance_receipt_root.clone(),
        ],
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RELEASE-GATE-ROOT",
        &[HashPart::Str(&config.state_root()), HashPart::Str(&root)],
        32,
    )
}

fn release_policy_root(
    config: &Config,
    source: &SourceBundle,
    lock: &CustodyLockRecord,
    status: ReleasePolicyStatus,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RELEASE-POLICY",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&lock.custody_root),
            HashPart::Str(status.as_str()),
        ],
        32,
    )
}

fn custody_lock_vector_root(records: &[CustodyLockRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| record.custody_root.clone())
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-LOCKS",
        &leaves,
    )
}

fn release_policy_vector_root(records: &[ReleasePolicyRecord]) -> String {
    let leaves = records
        .iter()
        .map(|record| record.policy_root.clone())
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-RELEASE-POLICIES",
        &leaves,
    )
}

fn release_gate_bundle_root(
    config: &Config,
    source: &SourceBundle,
    verdict: &CustodyReleaseVerdict,
) -> String {
    record_root(
        "release-gate-bundle",
        &json!({
            "config_root": config.state_root(),
            "source_root": source.state_root(),
            "verdict_root": verdict.verdict_root,
            "pq_quorum_ready": verdict.pq_quorum_ready,
            "reserve_ready": verdict.reserve_ready,
            "settlement_ready": verdict.settlement_ready,
            "challenge_window_ready": verdict.challenge_window_ready,
            "wallet_scan_ready": verdict.wallet_scan_ready,
            "recovery_ready": verdict.recovery_ready,
            "hold_clearance_ready": verdict.hold_clearance_ready,
        }),
    )
}

fn custody_authority_bundle_root(
    config: &Config,
    source: &SourceBundle,
    locks: &[CustodyLockRecord],
) -> String {
    let leaves = locks
        .iter()
        .map(|lock| lock.custody_root.clone())
        .collect::<Vec<_>>();
    let lock_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-AUTHORITY-LEAVES",
        &leaves,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-AUTHORITY-BUNDLE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&lock_root),
        ],
        32,
    )
}

fn blocked_release_root(
    config: &Config,
    source: &SourceBundle,
    policies: &[ReleasePolicyRecord],
) -> String {
    let leaves = policies
        .iter()
        .filter(|policy| !policy.moves_value)
        .map(|policy| policy.policy_root.clone())
        .collect::<Vec<_>>();
    let blocked_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-BLOCKED-RELEASE-LEAVES",
        &leaves,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-BLOCKED-RELEASE-BUNDLE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&blocked_root),
        ],
        32,
    )
}

fn custody_release_verdict_root(
    config: &Config,
    source: &SourceBundle,
    locks: &[CustodyLockRecord],
    policies: &[ReleasePolicyRecord],
    value_can_move: bool,
    production_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-RELEASE-VERDICT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&custody_lock_vector_root(locks)),
            HashPart::Str(&release_policy_vector_root(policies)),
            HashPart::Str(bool_str(value_can_move)),
            HashPart::Str(bool_str(production_blocked)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    custody_lock_root: &str,
    release_policy_root: &str,
    release_gate_bundle_root: &str,
    custody_authority_root: &str,
    blocked_release_root: &str,
    verdict: &CustodyReleaseVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-RELEASE-POLICY-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(custody_lock_root),
            HashPart::Str(release_policy_root),
            HashPart::Str(release_gate_bundle_root),
            HashPart::Str(custody_authority_root),
            HashPart::Str(blocked_release_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("custody release policy chain mismatch".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("custody release policy protocol mismatch".to_string());
    }
    if config.min_policy_records == 0 {
        return Err("custody release policy requires policy records".to_string());
    }
    if config.min_release_roots < DEFAULT_MIN_RELEASE_ROOTS {
        return Err("custody release policy requires all release roots".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.pq_authority_state_root.is_empty() {
        return Err("custody release policy missing PQ authority root".to_string());
    }
    if source.reserve_state_root.is_empty() {
        return Err("custody release policy missing reserve root".to_string());
    }
    if source.settlement_state_root.is_empty() {
        return Err("custody release policy missing settlement root".to_string());
    }
    if source.challenge_state_root.is_empty() {
        return Err("custody release policy missing challenge-window root".to_string());
    }
    if source.wallet_scan_state_root.is_empty() {
        return Err("custody release policy missing wallet-scan root".to_string());
    }
    if source.recovery_state_root.is_empty() {
        return Err("custody release policy missing recovery root".to_string());
    }
    if source.hold_clearance_state_root.is_empty() {
        return Err("custody release policy missing hold-clearance root".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let fallback_root = record_root("fallback-source", &json!({"reason": &reason}));
    let source = SourceBundle {
        pq_authority_state_root: fallback_root.clone(),
        pq_quorum_receipt_root: fallback_root.clone(),
        pq_release_hold_root: fallback_root.clone(),
        reserve_state_root: fallback_root.clone(),
        reserve_observation_root: fallback_root.clone(),
        reserve_coverage_root: fallback_root.clone(),
        reserve_production_hold_root: fallback_root.clone(),
        settlement_state_root: fallback_root.clone(),
        settlement_observation_root: fallback_root.clone(),
        settlement_evidence_bundle_root: fallback_root.clone(),
        settlement_production_hold_root: fallback_root.clone(),
        challenge_state_root: fallback_root.clone(),
        challenge_deadline_bundle_root: fallback_root.clone(),
        challenge_fail_closed_hold_root: fallback_root.clone(),
        wallet_scan_state_root: fallback_root.clone(),
        wallet_scan_receipt_root: fallback_root.clone(),
        wallet_scan_readiness_root: fallback_root.clone(),
        recovery_state_root: fallback_root.clone(),
        recovery_step_root: fallback_root.clone(),
        recovery_fail_closed_hold_root: fallback_root.clone(),
        hold_clearance_state_root: fallback_root.clone(),
        hold_clearance_receipt_root: fallback_root.clone(),
        hold_clearance_unresolved_blocker_root: fallback_root.clone(),
        hold_clearance_production_root: fallback_root,
        pq_quorum_verified: false,
        reserve_ready: false,
        settlement_observed: false,
        challenge_window_clear: false,
        wallet_scan_ready: false,
        recovery_ready: false,
        hold_clearance_ready: false,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
    };
    let custody_locks = vec![CustodyLockRecord::devnet(
        &config,
        &source,
        CustodyActor::PolicyEngine,
        1,
    )];
    let release_policies = custody_locks
        .iter()
        .map(|lock| ReleasePolicyRecord::from_lock(&config, &source, lock))
        .collect::<Vec<_>>();
    let verdict = CustodyReleaseVerdict::new(&config, &source, &custody_locks, &release_policies);
    let custody_lock_root = custody_lock_vector_root(&custody_locks);
    let release_policy_root = release_policy_vector_root(&release_policies);
    let release_gate_bundle_root = release_gate_bundle_root(&config, &source, &verdict);
    let custody_authority_root = custody_authority_bundle_root(&config, &source, &custody_locks);
    let blocked_release_root = blocked_release_root(&config, &source, &release_policies);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &custody_lock_root,
        &release_policy_root,
        &release_gate_bundle_root,
        &custody_authority_root,
        &blocked_release_root,
        &verdict,
    );
    State {
        config,
        source,
        custody_locks,
        release_policies,
        verdict,
        custody_lock_root,
        release_policy_root,
        release_gate_bundle_root,
        custody_authority_root,
        blocked_release_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CUSTODY-RELEASE-POLICY-RECORD",
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
