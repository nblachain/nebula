use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution_receipt,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_operator_independent_force_exit_package_runtime as package,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageChallengeWindowMonitorRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CHALLENGE_WINDOW_MONITOR_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-challenge-window-monitor-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CHALLENGE_WINDOW_MONITOR_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const CHALLENGE_MONITOR_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-challenge-window-monitor-v1";
pub const DEFAULT_CURRENT_L2_HEIGHT: u64 = 9_420_360;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_WATCHER_RESPONSE_BLOCKS: u64 = 72;
pub const DEFAULT_USER_ESCAPE_GRACE_BLOCKS: u64 = 144;
pub const DEFAULT_MIN_WATCHER_OBJECTIONS: u64 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub challenge_monitor_suite: String,
    pub current_l2_height: u64,
    pub challenge_window_blocks: u64,
    pub watcher_response_blocks: u64,
    pub user_escape_grace_blocks: u64,
    pub min_watcher_objections: u64,
    pub require_execution_receipt_observed: bool,
    pub require_watcher_objection_roots: bool,
    pub require_user_escape_deadline: bool,
    pub fail_closed_on_unresolved_window: bool,
    pub hold_production_until_window_resolved: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            challenge_monitor_suite: CHALLENGE_MONITOR_SUITE.to_string(),
            current_l2_height: DEFAULT_CURRENT_L2_HEIGHT,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            watcher_response_blocks: DEFAULT_WATCHER_RESPONSE_BLOCKS,
            user_escape_grace_blocks: DEFAULT_USER_ESCAPE_GRACE_BLOCKS,
            min_watcher_objections: DEFAULT_MIN_WATCHER_OBJECTIONS,
            require_execution_receipt_observed: true,
            require_watcher_objection_roots: true,
            require_user_escape_deadline: true,
            fail_closed_on_unresolved_window: true,
            hold_production_until_window_resolved: true,
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
            "challenge_monitor_suite": self.challenge_monitor_suite,
            "current_l2_height": self.current_l2_height,
            "challenge_window_blocks": self.challenge_window_blocks,
            "watcher_response_blocks": self.watcher_response_blocks,
            "user_escape_grace_blocks": self.user_escape_grace_blocks,
            "min_watcher_objections": self.min_watcher_objections,
            "require_execution_receipt_observed": self.require_execution_receipt_observed,
            "require_watcher_objection_roots": self.require_watcher_objection_roots,
            "require_user_escape_deadline": self.require_user_escape_deadline,
            "fail_closed_on_unresolved_window": self.fail_closed_on_unresolved_window,
            "hold_production_until_window_resolved": self.hold_production_until_window_resolved,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChallengeWindowStatus {
    NotOpened,
    Open,
    WatcherObjectionOpen,
    ObjectionResolved,
    ExpiredClean,
    ExpiredUnresolved,
    FailClosedHold,
}

impl ChallengeWindowStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotOpened => "not_opened",
            Self::Open => "open",
            Self::WatcherObjectionOpen => "watcher_objection_open",
            Self::ObjectionResolved => "objection_resolved",
            Self::ExpiredClean => "expired_clean",
            Self::ExpiredUnresolved => "expired_unresolved",
            Self::FailClosedHold => "fail_closed_hold",
        }
    }

    pub fn unresolved(self) -> bool {
        matches!(
            self,
            Self::NotOpened
                | Self::Open
                | Self::WatcherObjectionOpen
                | Self::ExpiredUnresolved
                | Self::FailClosedHold
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorVerdictStatus {
    ReleaseEligible,
    WaitingChallengeWindow,
    WaitingWatcherResolution,
    UserEscapeDeadlineActive,
    FailClosedHold,
}

impl MonitorVerdictStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEligible => "release_eligible",
            Self::WaitingChallengeWindow => "waiting_challenge_window",
            Self::WaitingWatcherResolution => "waiting_watcher_resolution",
            Self::UserEscapeDeadlineActive => "user_escape_deadline_active",
            Self::FailClosedHold => "fail_closed_hold",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub package_state_root: String,
    pub execution_receipt_state_root: String,
    pub package_challenge_window_bundle_root: String,
    pub execution_challenge_settlement_bundle_root: String,
    pub package_production_hold_root: String,
    pub execution_production_hold_root: String,
    pub package_status: String,
    pub execution_status: String,
    pub package_submit_ready: bool,
    pub package_production_blocked: bool,
    pub execution_observed: bool,
    pub execution_production_blocked: bool,
    pub package_action_count: u64,
    pub execution_receipt_count: u64,
    pub challenge_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
}

impl SourceBundle {
    pub fn from_states(
        package_state: &package::State,
        receipt_state: &execution_receipt::State,
    ) -> Self {
        let challenge_receipt_count = receipt_state
            .execution_receipts
            .iter()
            .filter(|receipt| {
                receipt.receipt_kind == execution_receipt::ExecutionReceiptKind::BindChallengeWindow
                    && receipt.status == execution_receipt::ExecutionReceiptStatus::Observed
            })
            .count() as u64;
        Self {
            package_state_root: package_state.state_root(),
            execution_receipt_state_root: receipt_state.state_root(),
            package_challenge_window_bundle_root: package_state
                .challenge_window_bundle_root
                .clone(),
            execution_challenge_settlement_bundle_root: receipt_state
                .challenge_settlement_bundle_root
                .clone(),
            package_production_hold_root: package_state.production_hold_root.clone(),
            execution_production_hold_root: receipt_state.production_hold_root.clone(),
            package_status: package_state.verdict.package_status.clone(),
            execution_status: receipt_state.verdict.execution_status.clone(),
            package_submit_ready: package_state.verdict.package_submit_ready,
            package_production_blocked: package_state.verdict.production_blocked,
            execution_observed: receipt_state.verdict.package_execution_observed,
            execution_production_blocked: receipt_state.verdict.production_blocked,
            package_action_count: package_state.verdict.package_action_count,
            execution_receipt_count: receipt_state.verdict.execution_receipt_count,
            challenge_receipt_count,
            release_held_count: receipt_state.verdict.release_held_count,
            fail_closed_count: receipt_state.verdict.fail_closed_count,
        }
    }

    pub fn devnet() -> Self {
        let package_state = package::devnet();
        let receipt_state = execution_receipt::devnet();
        Self::from_states(&package_state, &receipt_state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "package_state_root": self.package_state_root,
            "execution_receipt_state_root": self.execution_receipt_state_root,
            "package_challenge_window_bundle_root": self.package_challenge_window_bundle_root,
            "execution_challenge_settlement_bundle_root": self.execution_challenge_settlement_bundle_root,
            "package_production_hold_root": self.package_production_hold_root,
            "execution_production_hold_root": self.execution_production_hold_root,
            "package_status": self.package_status,
            "execution_status": self.execution_status,
            "package_submit_ready": self.package_submit_ready,
            "package_production_blocked": self.package_production_blocked,
            "execution_observed": self.execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
            "package_action_count": self.package_action_count,
            "execution_receipt_count": self.execution_receipt_count,
            "challenge_receipt_count": self.challenge_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatcherObjectionRecord {
    pub watcher_id: String,
    pub objection_kind: String,
    pub objection_root: String,
    pub evidence_root: String,
    pub opened_at_height: u64,
    pub response_deadline_height: u64,
    pub resolved_at_height: u64,
    pub sustained: bool,
}

impl WatcherObjectionRecord {
    pub fn devnet(config: &Config, source: &SourceBundle, ordinal: u64, sustained: bool) -> Self {
        let watcher_id = format!("pq-watchdog-{ordinal}");
        let opened_at_height = config
            .current_l2_height
            .saturating_sub(config.watcher_response_blocks / 2)
            .saturating_add(ordinal);
        let response_deadline_height =
            opened_at_height.saturating_add(config.watcher_response_blocks);
        let resolved_at_height = if sustained {
            0
        } else {
            opened_at_height.saturating_add(config.watcher_response_blocks / 3)
        };
        let objection_kind = if sustained {
            "unresolved_watcher_objection"
        } else {
            "resolved_window_liveness_check"
        }
        .to_string();
        let evidence_root = record_root(
            "watcher-objection-evidence",
            &json!({
                "watcher_id": &watcher_id,
                "objection_kind": &objection_kind,
                "package_state_root": &source.package_state_root,
                "execution_receipt_state_root": &source.execution_receipt_state_root,
                "ordinal": ordinal,
            }),
        );
        let objection_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-WATCHER-OBJECTION",
            &[
                HashPart::Str(&watcher_id),
                HashPart::Str(&objection_kind),
                HashPart::Str(&evidence_root),
                HashPart::U64(opened_at_height),
                HashPart::U64(response_deadline_height),
                HashPart::Str(bool_str(sustained)),
            ],
            32,
        );
        Self {
            watcher_id,
            objection_kind,
            objection_root,
            evidence_root,
            opened_at_height,
            response_deadline_height,
            resolved_at_height,
            sustained,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "watcher_id": self.watcher_id,
            "objection_kind": self.objection_kind,
            "objection_root": self.objection_root,
            "evidence_root": self.evidence_root,
            "opened_at_height": self.opened_at_height,
            "response_deadline_height": self.response_deadline_height,
            "resolved_at_height": self.resolved_at_height,
            "sustained": self.sustained,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeMonitorRecord {
    pub monitor_id: String,
    pub opened_at_height: u64,
    pub challenge_deadline_height: u64,
    pub watcher_response_deadline_height: u64,
    pub user_escape_deadline_height: u64,
    pub watcher_objection_root: String,
    pub deadline_root: String,
    pub status: ChallengeWindowStatus,
    pub production_hold: bool,
    pub user_escape_active: bool,
    pub release_allowed: bool,
    pub monitor_root: String,
}

impl ChallengeMonitorRecord {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        objections: &[WatcherObjectionRecord],
    ) -> Self {
        let opened_at_height = config
            .current_l2_height
            .saturating_sub(config.challenge_window_blocks / 2);
        let challenge_deadline_height =
            opened_at_height.saturating_add(config.challenge_window_blocks);
        let watcher_response_deadline_height =
            challenge_deadline_height.saturating_add(config.watcher_response_blocks);
        let user_escape_deadline_height =
            challenge_deadline_height.saturating_add(config.user_escape_grace_blocks);
        let watcher_objection_root = watcher_objection_vector_root(objections);
        let sustained_objections =
            objections.iter().filter(|record| record.sustained).count() as u64;
        let status = monitor_status(
            config,
            source,
            sustained_objections,
            challenge_deadline_height,
        );
        let production_hold = status.unresolved()
            || source.package_production_blocked
            || source.execution_production_blocked
            || (config.hold_production_until_window_resolved
                && status != ChallengeWindowStatus::ExpiredClean
                && status != ChallengeWindowStatus::ObjectionResolved);
        let user_escape_active = config.current_l2_height <= user_escape_deadline_height
            && (status.unresolved() || source.package_production_blocked);
        let release_allowed = !production_hold
            && !user_escape_active
            && source.execution_observed
            && status == ChallengeWindowStatus::ExpiredClean;
        let deadline_root = deadline_root(
            config,
            opened_at_height,
            challenge_deadline_height,
            watcher_response_deadline_height,
            user_escape_deadline_height,
        );
        let monitor_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-MONITOR-ID",
            &[
                HashPart::Str(&source.package_state_root),
                HashPart::Str(&source.execution_receipt_state_root),
                HashPart::U64(opened_at_height),
            ],
            16,
        );
        let monitor_root = monitor_record_root(
            &monitor_id,
            source,
            &watcher_objection_root,
            &deadline_root,
            status,
            production_hold,
            user_escape_active,
            release_allowed,
        );
        Self {
            monitor_id,
            opened_at_height,
            challenge_deadline_height,
            watcher_response_deadline_height,
            user_escape_deadline_height,
            watcher_objection_root,
            deadline_root,
            status,
            production_hold,
            user_escape_active,
            release_allowed,
            monitor_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "monitor_id": self.monitor_id,
            "opened_at_height": self.opened_at_height,
            "challenge_deadline_height": self.challenge_deadline_height,
            "watcher_response_deadline_height": self.watcher_response_deadline_height,
            "user_escape_deadline_height": self.user_escape_deadline_height,
            "watcher_objection_root": self.watcher_objection_root,
            "deadline_root": self.deadline_root,
            "status": self.status.as_str(),
            "production_hold": self.production_hold,
            "user_escape_active": self.user_escape_active,
            "release_allowed": self.release_allowed,
            "monitor_root": self.monitor_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeMonitorVerdict {
    pub status: MonitorVerdictStatus,
    pub challenge_window_resolved: bool,
    pub watcher_objection_roots_present: bool,
    pub user_escape_deadline_present: bool,
    pub fail_closed_hold_active: bool,
    pub production_blocked: bool,
    pub release_allowed: bool,
    pub watcher_objection_count: u64,
    pub sustained_objection_count: u64,
    pub verdict_root: String,
}

impl ChallengeMonitorVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        monitor: &ChallengeMonitorRecord,
        objections: &[WatcherObjectionRecord],
    ) -> Self {
        let watcher_objection_count = objections.len() as u64;
        let sustained_objection_count =
            objections.iter().filter(|record| record.sustained).count() as u64;
        let challenge_window_resolved = matches!(
            monitor.status,
            ChallengeWindowStatus::ObjectionResolved | ChallengeWindowStatus::ExpiredClean
        );
        let watcher_objection_roots_present = !monitor.watcher_objection_root.is_empty()
            && watcher_objection_count >= config.min_watcher_objections;
        let user_escape_deadline_present = config.require_user_escape_deadline
            && monitor.user_escape_deadline_height > monitor.challenge_deadline_height;
        let fail_closed_hold_active = config.fail_closed_on_unresolved_window
            && (monitor.status == ChallengeWindowStatus::FailClosedHold
                || monitor.status == ChallengeWindowStatus::ExpiredUnresolved
                || sustained_objection_count > 0
                || !source.execution_observed);
        let production_blocked =
            monitor.production_hold || fail_closed_hold_active || source.package_production_blocked;
        let release_allowed = monitor.release_allowed
            && challenge_window_resolved
            && watcher_objection_roots_present
            && user_escape_deadline_present
            && !fail_closed_hold_active
            && !production_blocked;
        let status = if release_allowed {
            MonitorVerdictStatus::ReleaseEligible
        } else if fail_closed_hold_active {
            MonitorVerdictStatus::FailClosedHold
        } else if sustained_objection_count > 0 {
            MonitorVerdictStatus::WaitingWatcherResolution
        } else if monitor.user_escape_active {
            MonitorVerdictStatus::UserEscapeDeadlineActive
        } else {
            MonitorVerdictStatus::WaitingChallengeWindow
        };
        let verdict_root = record_root(
            "challenge-monitor-verdict",
            &json!({
                "status": status.as_str(),
                "challenge_window_resolved": challenge_window_resolved,
                "watcher_objection_roots_present": watcher_objection_roots_present,
                "user_escape_deadline_present": user_escape_deadline_present,
                "fail_closed_hold_active": fail_closed_hold_active,
                "production_blocked": production_blocked,
                "release_allowed": release_allowed,
                "watcher_objection_count": watcher_objection_count,
                "sustained_objection_count": sustained_objection_count,
            }),
        );
        Self {
            status,
            challenge_window_resolved,
            watcher_objection_roots_present,
            user_escape_deadline_present,
            fail_closed_hold_active,
            production_blocked,
            release_allowed,
            watcher_objection_count,
            sustained_objection_count,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "challenge_window_resolved": self.challenge_window_resolved,
            "watcher_objection_roots_present": self.watcher_objection_roots_present,
            "user_escape_deadline_present": self.user_escape_deadline_present,
            "fail_closed_hold_active": self.fail_closed_hold_active,
            "production_blocked": self.production_blocked,
            "release_allowed": self.release_allowed,
            "watcher_objection_count": self.watcher_objection_count,
            "sustained_objection_count": self.sustained_objection_count,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub watcher_objections: Vec<WatcherObjectionRecord>,
    pub monitor: ChallengeMonitorRecord,
    pub verdict: ChallengeMonitorVerdict,
    pub watcher_objection_bundle_root: String,
    pub deadline_bundle_root: String,
    pub fail_closed_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(
        config: Config,
        package_state: package::State,
        receipt_state: execution_receipt::State,
    ) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_states(&package_state, &receipt_state);
        validate_source(&source)?;
        let watcher_objections = devnet_watcher_objections(&config, &source);
        let monitor = ChallengeMonitorRecord::new(&config, &source, &watcher_objections);
        let verdict = ChallengeMonitorVerdict::new(&config, &source, &monitor, &watcher_objections);
        let watcher_objection_bundle_root = watcher_objection_vector_root(&watcher_objections);
        let deadline_bundle_root = monitor.deadline_root.clone();
        let fail_closed_hold_root = fail_closed_hold_root(&config, &source, &monitor, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &watcher_objection_bundle_root,
            &deadline_bundle_root,
            &fail_closed_hold_root,
            &monitor,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            watcher_objections,
            monitor,
            verdict,
            watcher_objection_bundle_root,
            deadline_bundle_root,
            fail_closed_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(
            Config::default(),
            package::devnet(),
            execution_receipt::devnet(),
        ) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_monitor_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "watcher_objection_bundle_root": self.watcher_objection_bundle_root,
            "deadline_bundle_root": self.deadline_bundle_root,
            "fail_closed_hold_root": self.fail_closed_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "monitor": self.monitor.public_record(),
            "verdict": self.verdict.public_record(),
            "watcher_objections": self
                .watcher_objections
                .iter()
                .map(WatcherObjectionRecord::public_record)
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

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CHALLENGE-WINDOW-MONITOR-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn devnet_watcher_objections(
    config: &Config,
    source: &SourceBundle,
) -> Vec<WatcherObjectionRecord> {
    (0..config.min_watcher_objections)
        .map(|offset| {
            let sustained = config.fail_closed_on_unresolved_window
                && (!source.execution_observed || source.fail_closed_count > 0)
                && offset == 0;
            WatcherObjectionRecord::devnet(config, source, offset.saturating_add(1), sustained)
        })
        .collect()
}

fn monitor_status(
    config: &Config,
    source: &SourceBundle,
    sustained_objections: u64,
    challenge_deadline_height: u64,
) -> ChallengeWindowStatus {
    if !source.package_submit_ready {
        return ChallengeWindowStatus::NotOpened;
    }
    if config.require_execution_receipt_observed && !source.execution_observed {
        return ChallengeWindowStatus::FailClosedHold;
    }
    if sustained_objections > 0 && config.current_l2_height <= challenge_deadline_height {
        return ChallengeWindowStatus::WatcherObjectionOpen;
    }
    if sustained_objections > 0 {
        return ChallengeWindowStatus::ExpiredUnresolved;
    }
    if config.current_l2_height < challenge_deadline_height {
        return ChallengeWindowStatus::Open;
    }
    ChallengeWindowStatus::ExpiredClean
}

fn watcher_objection_vector_root(objections: &[WatcherObjectionRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-WATCHER-OBJECTION-BUNDLE",
        &objections
            .iter()
            .map(WatcherObjectionRecord::public_record)
            .collect::<Vec<_>>(),
    )
}

fn deadline_root(
    config: &Config,
    opened_at_height: u64,
    challenge_deadline_height: u64,
    watcher_response_deadline_height: u64,
    user_escape_deadline_height: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DEADLINE-ROOT",
        &[
            HashPart::Str(&config.challenge_monitor_suite),
            HashPart::U64(opened_at_height),
            HashPart::U64(challenge_deadline_height),
            HashPart::U64(watcher_response_deadline_height),
            HashPart::U64(user_escape_deadline_height),
        ],
        32,
    )
}

fn monitor_record_root(
    monitor_id: &str,
    source: &SourceBundle,
    watcher_objection_root: &str,
    deadline_root: &str,
    status: ChallengeWindowStatus,
    production_hold: bool,
    user_escape_active: bool,
    release_allowed: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-MONITOR-ROOT",
        &[
            HashPart::Str(monitor_id),
            HashPart::Str(&source.package_state_root),
            HashPart::Str(&source.execution_receipt_state_root),
            HashPart::Str(watcher_objection_root),
            HashPart::Str(deadline_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(production_hold)),
            HashPart::Str(bool_str(user_escape_active)),
            HashPart::Str(bool_str(release_allowed)),
        ],
        32,
    )
}

fn fail_closed_hold_root(
    config: &Config,
    source: &SourceBundle,
    monitor: &ChallengeMonitorRecord,
    verdict: &ChallengeMonitorVerdict,
) -> String {
    record_root(
        "fail-closed-hold",
        &json!({
            "challenge_monitor_suite": &config.challenge_monitor_suite,
            "package_production_hold_root": &source.package_production_hold_root,
            "execution_production_hold_root": &source.execution_production_hold_root,
            "monitor_root": &monitor.monitor_root,
            "verdict_root": &verdict.verdict_root,
            "fail_closed_hold_active": verdict.fail_closed_hold_active,
            "production_blocked": verdict.production_blocked,
            "reason": if verdict.fail_closed_hold_active {
                "challenge window unresolved or execution observation missing"
            } else {
                "challenge window resolved"
            },
        }),
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    watcher_objection_bundle_root: &str,
    deadline_bundle_root: &str,
    fail_closed_hold_root: &str,
    monitor: &ChallengeMonitorRecord,
    verdict: &ChallengeMonitorVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CHALLENGE-WINDOW-MONITOR-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(watcher_objection_bundle_root),
            HashPart::Str(deadline_bundle_root),
            HashPart::Str(fail_closed_hold_root),
            HashPart::Str(&monitor.monitor_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("challenge window monitor chain id mismatch".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("challenge window monitor protocol version mismatch".to_string());
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("challenge window monitor schema version mismatch".to_string());
    }
    if config.challenge_window_blocks == 0 {
        return Err("challenge window monitor requires nonzero challenge window".to_string());
    }
    if config.watcher_response_blocks == 0 {
        return Err(
            "challenge window monitor requires nonzero watcher response window".to_string(),
        );
    }
    if config.min_watcher_objections == 0 {
        return Err("challenge window monitor requires watcher objection roots".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.package_state_root.is_empty() {
        return Err("challenge window monitor missing package state root".to_string());
    }
    if source.execution_receipt_state_root.is_empty() {
        return Err("challenge window monitor missing execution receipt state root".to_string());
    }
    if source.package_action_count == 0 {
        return Err("challenge window monitor requires package actions".to_string());
    }
    if source.execution_receipt_count == 0 {
        return Err("challenge window monitor requires execution receipts".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle {
        package_state_root: record_root("fallback-package-state", &json!({"reason": &reason})),
        execution_receipt_state_root: record_root(
            "fallback-execution-state",
            &json!({"reason": &reason}),
        ),
        package_challenge_window_bundle_root: record_root(
            "fallback-package-window",
            &json!({"reason": &reason}),
        ),
        execution_challenge_settlement_bundle_root: record_root(
            "fallback-execution-window",
            &json!({"reason": &reason}),
        ),
        package_production_hold_root: record_root(
            "fallback-package-hold",
            &json!({"reason": &reason}),
        ),
        execution_production_hold_root: record_root(
            "fallback-execution-hold",
            &json!({"reason": &reason}),
        ),
        package_status: "fallback".to_string(),
        execution_status: "fallback".to_string(),
        package_submit_ready: false,
        package_production_blocked: true,
        execution_observed: false,
        execution_production_blocked: true,
        package_action_count: 1,
        execution_receipt_count: 1,
        challenge_receipt_count: 0,
        release_held_count: 1,
        fail_closed_count: 1,
    };
    let watcher_objections = vec![WatcherObjectionRecord::devnet(&config, &source, 1, true)];
    let monitor = ChallengeMonitorRecord::new(&config, &source, &watcher_objections);
    let verdict = ChallengeMonitorVerdict::new(&config, &source, &monitor, &watcher_objections);
    let watcher_objection_bundle_root = watcher_objection_vector_root(&watcher_objections);
    let deadline_bundle_root = monitor.deadline_root.clone();
    let fail_closed_hold_root = fail_closed_hold_root(&config, &source, &monitor, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &watcher_objection_bundle_root,
        &deadline_bundle_root,
        &fail_closed_hold_root,
        &monitor,
        &verdict,
    );
    State {
        config,
        source,
        watcher_objections,
        monitor,
        verdict,
        watcher_objection_bundle_root,
        deadline_bundle_root,
        fail_closed_hold_root,
        state_commitment_root,
    }
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
