use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_wallet_escape_transcript_runtime as wallet_transcript,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceOperatorIndependentForceExitPackageRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_OPERATOR_INDEPENDENT_FORCE_EXIT_PACKAGE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-operator-independent-force-exit-package-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_OPERATOR_INDEPENDENT_FORCE_EXIT_PACKAGE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const OPERATOR_INDEPENDENT_FORCE_EXIT_PACKAGE_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-operator-independent-force-exit-package-v1";
pub const DEFAULT_MIN_PACKAGE_ACTIONS: u64 = 9;
pub const DEFAULT_MIN_WALLET_ACTIONS: u64 = 6;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub package_suite: String,
    pub min_package_actions: u64,
    pub min_wallet_actions: u64,
    pub challenge_window_blocks: u64,
    pub require_operator_independent_submission: bool,
    pub require_pq_authorization: bool,
    pub require_reserve_fallback: bool,
    pub require_wallet_local_privacy: bool,
    pub require_fail_closed_recovery: bool,
    pub require_release_hold_reporting: bool,
    pub hold_production_until_package_executed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            package_suite: OPERATOR_INDEPENDENT_FORCE_EXIT_PACKAGE_SUITE.to_string(),
            min_package_actions: DEFAULT_MIN_PACKAGE_ACTIONS,
            min_wallet_actions: DEFAULT_MIN_WALLET_ACTIONS,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            require_operator_independent_submission: true,
            require_pq_authorization: true,
            require_reserve_fallback: true,
            require_wallet_local_privacy: true,
            require_fail_closed_recovery: true,
            require_release_hold_reporting: true,
            hold_production_until_package_executed: true,
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
            "package_suite": self.package_suite,
            "min_package_actions": self.min_package_actions,
            "min_wallet_actions": self.min_wallet_actions,
            "challenge_window_blocks": self.challenge_window_blocks,
            "require_operator_independent_submission": self.require_operator_independent_submission,
            "require_pq_authorization": self.require_pq_authorization,
            "require_reserve_fallback": self.require_reserve_fallback,
            "require_wallet_local_privacy": self.require_wallet_local_privacy,
            "require_fail_closed_recovery": self.require_fail_closed_recovery,
            "require_release_hold_reporting": self.require_release_hold_reporting,
            "hold_production_until_package_executed": self.hold_production_until_package_executed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ForceExitActionKind {
    IntakeWalletTranscript,
    BuildEvidenceBundle,
    PqAuthorizeClaim,
    BindChallengeWindow,
    AttachReserveFallback,
    BroadcastExitClaim,
    WatchSettlementReceipt,
    FailClosedRecovery,
    ReportReleaseHold,
}

impl ForceExitActionKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::IntakeWalletTranscript,
            Self::BuildEvidenceBundle,
            Self::PqAuthorizeClaim,
            Self::BindChallengeWindow,
            Self::AttachReserveFallback,
            Self::BroadcastExitClaim,
            Self::WatchSettlementReceipt,
            Self::FailClosedRecovery,
            Self::ReportReleaseHold,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntakeWalletTranscript => "intake_wallet_transcript",
            Self::BuildEvidenceBundle => "build_evidence_bundle",
            Self::PqAuthorizeClaim => "pq_authorize_claim",
            Self::BindChallengeWindow => "bind_challenge_window",
            Self::AttachReserveFallback => "attach_reserve_fallback",
            Self::BroadcastExitClaim => "broadcast_exit_claim",
            Self::WatchSettlementReceipt => "watch_settlement_receipt",
            Self::FailClosedRecovery => "fail_closed_recovery",
            Self::ReportReleaseHold => "report_release_hold",
        }
    }

    pub fn command(self) -> &'static str {
        match self {
            Self::IntakeWalletTranscript => "force-exit intake-wallet-transcript",
            Self::BuildEvidenceBundle => "force-exit build-evidence-bundle",
            Self::PqAuthorizeClaim => "force-exit pq-authorize-claim",
            Self::BindChallengeWindow => "force-exit bind-challenge-window",
            Self::AttachReserveFallback => "force-exit attach-reserve-fallback",
            Self::BroadcastExitClaim => "force-exit broadcast-claim",
            Self::WatchSettlementReceipt => "force-exit watch-settlement",
            Self::FailClosedRecovery => "force-exit fail-closed-recovery",
            Self::ReportReleaseHold => "force-exit report-release-hold",
        }
    }

    pub fn wallet_visible(self) -> bool {
        matches!(
            self,
            Self::IntakeWalletTranscript
                | Self::BuildEvidenceBundle
                | Self::PqAuthorizeClaim
                | Self::BroadcastExitClaim
                | Self::WatchSettlementReceipt
                | Self::FailClosedRecovery
                | Self::ReportReleaseHold
        )
    }

    pub fn requires_pq(self) -> bool {
        matches!(self, Self::PqAuthorizeClaim | Self::BroadcastExitClaim)
    }

    pub fn requires_reserve(self) -> bool {
        matches!(
            self,
            Self::AttachReserveFallback | Self::WatchSettlementReceipt
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ForceExitPackageStatus {
    ReadyToSubmit,
    DeferredUntilWalletTranscriptLive,
    ReleaseHeld,
    FailClosed,
}

impl ForceExitPackageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToSubmit => "ready_to_submit",
            Self::DeferredUntilWalletTranscriptLive => "deferred_until_wallet_transcript_live",
            Self::ReleaseHeld => "release_held",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub transcript_state_root: String,
    pub transcript_entry_root: String,
    pub wallet_evidence_root: String,
    pub private_scan_bundle_root: String,
    pub user_escape_package_root: String,
    pub transcript_production_hold_root: String,
    pub transcript_status: String,
    pub transcript_user_escape_answer: String,
    pub transcript_production_answer: String,
    pub transcript_entry_count: u64,
    pub wallet_ready_count: u64,
    pub deferred_entry_count: u64,
    pub fail_closed_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub wallet_escape_transcript_complete: bool,
    pub user_escape_answerable: bool,
    pub transcript_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_transcript(state: &wallet_transcript::State) -> Self {
        Self {
            transcript_state_root: state.state_root(),
            transcript_entry_root: state.transcript_entry_root.clone(),
            wallet_evidence_root: state.wallet_evidence_root.clone(),
            private_scan_bundle_root: state.private_scan_bundle_root.clone(),
            user_escape_package_root: state.user_escape_package_root.clone(),
            transcript_production_hold_root: state.production_hold_root.clone(),
            transcript_status: state.verdict.transcript_status.clone(),
            transcript_user_escape_answer: state.verdict.user_escape_answer.clone(),
            transcript_production_answer: state.verdict.production_answer.clone(),
            transcript_entry_count: state.verdict.transcript_entry_count,
            wallet_ready_count: state.verdict.wallet_ready_count,
            deferred_entry_count: state.verdict.deferred_entry_count,
            fail_closed_count: state.verdict.fail_closed_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            wallet_escape_transcript_complete: state.verdict.wallet_escape_transcript_complete,
            user_escape_answerable: state.verdict.user_escape_answerable,
            transcript_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let transcript = wallet_transcript::devnet();
        Self::from_transcript(&transcript)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "transcript_state_root": self.transcript_state_root,
            "transcript_entry_root": self.transcript_entry_root,
            "wallet_evidence_root": self.wallet_evidence_root,
            "private_scan_bundle_root": self.private_scan_bundle_root,
            "user_escape_package_root": self.user_escape_package_root,
            "transcript_production_hold_root": self.transcript_production_hold_root,
            "transcript_status": self.transcript_status,
            "transcript_user_escape_answer": self.transcript_user_escape_answer,
            "transcript_production_answer": self.transcript_production_answer,
            "transcript_entry_count": self.transcript_entry_count,
            "wallet_ready_count": self.wallet_ready_count,
            "deferred_entry_count": self.deferred_entry_count,
            "fail_closed_count": self.fail_closed_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "wallet_escape_transcript_complete": self.wallet_escape_transcript_complete,
            "user_escape_answerable": self.user_escape_answerable,
            "transcript_production_blocked": self.transcript_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForceExitPackageAction {
    pub action_id: String,
    pub ordinal: u64,
    pub action_kind: ForceExitActionKind,
    pub command_root: String,
    pub transcript_dependency_root: String,
    pub wallet_submission_root: String,
    pub pq_authorization_root: String,
    pub challenge_window_root: String,
    pub reserve_fallback_root: String,
    pub privacy_redaction_root: String,
    pub fail_closed_recovery_root: String,
    pub release_hold_root: String,
    pub action_root: String,
    pub status: ForceExitPackageStatus,
    pub wallet_visible: bool,
    pub operator_independent: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub command: String,
    pub required_outcome: String,
}

impl ForceExitPackageAction {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        action_kind: ForceExitActionKind,
        ordinal: u64,
    ) -> Self {
        let status = package_status(source, action_kind);
        let command_root = command_root(config, source, action_kind, ordinal);
        let transcript_dependency_root =
            transcript_dependency_root(config, source, action_kind, ordinal);
        let wallet_submission_root = wallet_submission_root(config, source, action_kind, ordinal);
        let pq_authorization_root = pq_authorization_root(config, source, action_kind, ordinal);
        let challenge_window_root = challenge_window_root(config, source, action_kind, ordinal);
        let reserve_fallback_root = reserve_fallback_root(config, source, action_kind, ordinal);
        let privacy_redaction_root = privacy_redaction_root(config, source, action_kind, ordinal);
        let fail_closed_recovery_root =
            fail_closed_recovery_root(config, source, action_kind, status, ordinal);
        let release_hold_root = release_hold_root(config, source, action_kind, status, ordinal);
        let wallet_visible = action_kind.wallet_visible();
        let operator_independent = true;
        let blocks_user_release =
            source.user_release_blocker_count > 0 || status == ForceExitPackageStatus::FailClosed;
        let blocks_production =
            source.transcript_production_blocked || status != ForceExitPackageStatus::ReadyToSubmit;
        let action_root = action_root(
            config,
            source,
            action_kind,
            status,
            ordinal,
            &command_root,
            &transcript_dependency_root,
            &wallet_submission_root,
            &pq_authorization_root,
            &challenge_window_root,
            &reserve_fallback_root,
            &privacy_redaction_root,
            &fail_closed_recovery_root,
            &release_hold_root,
            operator_independent,
            blocks_user_release,
            blocks_production,
        );
        let action_id = action_id(action_kind, ordinal, &action_root);
        Self {
            action_id,
            ordinal,
            action_kind,
            command_root,
            transcript_dependency_root,
            wallet_submission_root,
            pq_authorization_root,
            challenge_window_root,
            reserve_fallback_root,
            privacy_redaction_root,
            fail_closed_recovery_root,
            release_hold_root,
            action_root,
            status,
            wallet_visible,
            operator_independent,
            blocks_user_release,
            blocks_production,
            command: action_kind.command().to_string(),
            required_outcome: required_outcome(status, action_kind).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "action_id": self.action_id,
            "ordinal": self.ordinal,
            "action_kind": self.action_kind.as_str(),
            "command_root": self.command_root,
            "transcript_dependency_root": self.transcript_dependency_root,
            "wallet_submission_root": self.wallet_submission_root,
            "pq_authorization_root": self.pq_authorization_root,
            "challenge_window_root": self.challenge_window_root,
            "reserve_fallback_root": self.reserve_fallback_root,
            "privacy_redaction_root": self.privacy_redaction_root,
            "fail_closed_recovery_root": self.fail_closed_recovery_root,
            "release_hold_root": self.release_hold_root,
            "action_root": self.action_root,
            "status": self.status.as_str(),
            "wallet_visible": self.wallet_visible,
            "operator_independent": self.operator_independent,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "command": self.command,
            "required_outcome": self.required_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.action_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForceExitPackageVerdict {
    pub package_action_count: u64,
    pub ready_action_count: u64,
    pub deferred_action_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub wallet_visible_count: u64,
    pub operator_independent_count: u64,
    pub pq_action_count: u64,
    pub reserve_action_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub transcript_entry_count: u64,
    pub wallet_escape_transcript_complete: bool,
    pub user_escape_answerable: bool,
    pub command_vector_present: bool,
    pub wallet_submission_present: bool,
    pub pq_authorization_present: bool,
    pub challenge_window_present: bool,
    pub reserve_fallback_present: bool,
    pub privacy_redaction_present: bool,
    pub fail_closed_recovery_present: bool,
    pub release_hold_reporting_present: bool,
    pub package_submit_ready: bool,
    pub production_blocked: bool,
    pub package_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ForceExitPackageVerdict {
    pub fn new(config: &Config, source: &SourceBundle, actions: &[ForceExitPackageAction]) -> Self {
        let package_action_count = actions.len() as u64;
        let ready_action_count = count_status(actions, ForceExitPackageStatus::ReadyToSubmit);
        let deferred_action_count = count_status(
            actions,
            ForceExitPackageStatus::DeferredUntilWalletTranscriptLive,
        );
        let release_held_count = count_status(actions, ForceExitPackageStatus::ReleaseHeld);
        let fail_closed_count = count_status(actions, ForceExitPackageStatus::FailClosed);
        let wallet_visible_count = actions
            .iter()
            .filter(|action| action.wallet_visible)
            .count() as u64;
        let operator_independent_count = actions
            .iter()
            .filter(|action| action.operator_independent)
            .count() as u64;
        let pq_action_count = actions
            .iter()
            .filter(|action| action.action_kind.requires_pq())
            .count() as u64;
        let reserve_action_count = actions
            .iter()
            .filter(|action| action.action_kind.requires_reserve())
            .count() as u64;
        let user_release_blocker_count = actions
            .iter()
            .filter(|action| action.blocks_user_release)
            .count() as u64;
        let production_blocker_count = actions
            .iter()
            .filter(|action| action.blocks_production)
            .count() as u64;
        let transcript_entry_count = source.transcript_entry_count;
        let wallet_escape_transcript_complete = source.wallet_escape_transcript_complete;
        let user_escape_answerable = source.user_escape_answerable;
        let command_vector_present = actions.iter().all(|action| !action.command_root.is_empty());
        let wallet_submission_present = actions
            .iter()
            .all(|action| !action.wallet_submission_root.is_empty());
        let pq_authorization_present = !config.require_pq_authorization
            || actions
                .iter()
                .filter(|action| action.action_kind.requires_pq())
                .all(|action| !action.pq_authorization_root.is_empty());
        let challenge_window_present = actions
            .iter()
            .all(|action| !action.challenge_window_root.is_empty());
        let reserve_fallback_present = !config.require_reserve_fallback
            || actions
                .iter()
                .filter(|action| action.action_kind.requires_reserve())
                .all(|action| !action.reserve_fallback_root.is_empty());
        let privacy_redaction_present = !config.require_wallet_local_privacy
            || actions
                .iter()
                .all(|action| !action.privacy_redaction_root.is_empty());
        let fail_closed_recovery_present = !config.require_fail_closed_recovery
            || actions
                .iter()
                .all(|action| !action.fail_closed_recovery_root.is_empty());
        let release_hold_reporting_present = !config.require_release_hold_reporting
            || actions
                .iter()
                .all(|action| !action.release_hold_root.is_empty());
        let package_submit_ready = package_action_count >= config.min_package_actions
            && wallet_visible_count >= config.min_wallet_actions
            && operator_independent_count == package_action_count
            && ready_action_count >= config.min_package_actions
            && deferred_action_count == 0
            && release_held_count == 0
            && fail_closed_count == 0
            && wallet_escape_transcript_complete
            && user_escape_answerable
            && command_vector_present
            && wallet_submission_present
            && pq_authorization_present
            && challenge_window_present
            && reserve_fallback_present
            && privacy_redaction_present
            && fail_closed_recovery_present
            && release_hold_reporting_present;
        let production_blocked = source.transcript_production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_package_executed && !package_submit_ready);
        let package_status = if fail_closed_count > 0 {
            "fail_closed"
        } else if release_held_count > 0 {
            "release_held"
        } else if deferred_action_count > 0 {
            "force_exit_package_deferred_until_wallet_transcript_live"
        } else if package_submit_ready {
            "operator_independent_force_exit_package_ready"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if package_submit_ready {
            "wallet can submit an operator-independent force-exit package"
        } else {
            "wallet force-exit package is structured but remains deferred until transcript evidence is live and user-release blockers clear"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains blocked until operator-independent force-exit package execution is observed"
        } else {
            "bounded bridge/exit package is ready for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            package_action_count,
            ready_action_count,
            deferred_action_count,
            release_held_count,
            fail_closed_count,
            user_release_blocker_count,
            production_blocker_count,
            package_submit_ready,
            production_blocked,
            &package_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            package_action_count,
            ready_action_count,
            deferred_action_count,
            release_held_count,
            fail_closed_count,
            wallet_visible_count,
            operator_independent_count,
            pq_action_count,
            reserve_action_count,
            user_release_blocker_count,
            production_blocker_count,
            transcript_entry_count,
            wallet_escape_transcript_complete,
            user_escape_answerable,
            command_vector_present,
            wallet_submission_present,
            pq_authorization_present,
            challenge_window_present,
            reserve_fallback_present,
            privacy_redaction_present,
            fail_closed_recovery_present,
            release_hold_reporting_present,
            package_submit_ready,
            production_blocked,
            package_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "package_action_count": self.package_action_count,
            "ready_action_count": self.ready_action_count,
            "deferred_action_count": self.deferred_action_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "wallet_visible_count": self.wallet_visible_count,
            "operator_independent_count": self.operator_independent_count,
            "pq_action_count": self.pq_action_count,
            "reserve_action_count": self.reserve_action_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "transcript_entry_count": self.transcript_entry_count,
            "wallet_escape_transcript_complete": self.wallet_escape_transcript_complete,
            "user_escape_answerable": self.user_escape_answerable,
            "command_vector_present": self.command_vector_present,
            "wallet_submission_present": self.wallet_submission_present,
            "pq_authorization_present": self.pq_authorization_present,
            "challenge_window_present": self.challenge_window_present,
            "reserve_fallback_present": self.reserve_fallback_present,
            "privacy_redaction_present": self.privacy_redaction_present,
            "fail_closed_recovery_present": self.fail_closed_recovery_present,
            "release_hold_reporting_present": self.release_hold_reporting_present,
            "package_submit_ready": self.package_submit_ready,
            "production_blocked": self.production_blocked,
            "package_status": self.package_status,
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
    pub package_actions: Vec<ForceExitPackageAction>,
    pub verdict: ForceExitPackageVerdict,
    pub package_action_root: String,
    pub command_bundle_root: String,
    pub wallet_submission_bundle_root: String,
    pub challenge_window_bundle_root: String,
    pub recovery_bundle_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, transcript_state: wallet_transcript::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_transcript(&transcript_state);
        validate_source(&source)?;
        let package_actions = ForceExitActionKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                ForceExitPackageAction::devnet(&config, &source, *kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = ForceExitPackageVerdict::new(&config, &source, &package_actions);
        let package_action_root = package_action_vector_root(&package_actions);
        let command_bundle_root = command_bundle_root(&config, &source, &package_actions, &verdict);
        let wallet_submission_bundle_root =
            wallet_submission_bundle_root(&config, &source, &package_actions, &verdict);
        let challenge_window_bundle_root =
            challenge_window_bundle_root(&config, &source, &package_actions, &verdict);
        let recovery_bundle_root =
            recovery_bundle_root(&config, &source, &package_actions, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &package_actions, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &package_action_root,
            &command_bundle_root,
            &wallet_submission_bundle_root,
            &challenge_window_bundle_root,
            &recovery_bundle_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            package_actions,
            verdict,
            package_action_root,
            command_bundle_root,
            wallet_submission_bundle_root,
            challenge_window_bundle_root,
            recovery_bundle_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), wallet_transcript::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_operator_independent_force_exit_package_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "package_action_root": self.package_action_root,
            "command_bundle_root": self.command_bundle_root,
            "wallet_submission_bundle_root": self.wallet_submission_bundle_root,
            "challenge_window_bundle_root": self.challenge_window_bundle_root,
            "recovery_bundle_root": self.recovery_bundle_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "package_actions": self
                .package_actions
                .iter()
                .map(ForceExitPackageAction::public_record)
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

fn package_status(
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
) -> ForceExitPackageStatus {
    if source.fail_closed_count > 0 {
        ForceExitPackageStatus::FailClosed
    } else if source.production_blocker_count > 0
        || action_kind == ForceExitActionKind::ReportReleaseHold
    {
        ForceExitPackageStatus::ReleaseHeld
    } else if source.user_escape_answerable && source.wallet_escape_transcript_complete {
        ForceExitPackageStatus::ReadyToSubmit
    } else {
        ForceExitPackageStatus::DeferredUntilWalletTranscriptLive
    }
}

fn command_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "command",
        &json!({
            "package_suite": &config.package_suite,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "command": action_kind.command(),
            "user_escape_package_root": &source.user_escape_package_root,
            "operator_independent": true,
        }),
    )
}

fn transcript_dependency_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "transcript-dependency",
        &json!({
            "package_suite": &config.package_suite,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "transcript_state_root": &source.transcript_state_root,
            "transcript_entry_root": &source.transcript_entry_root,
            "wallet_evidence_root": &source.wallet_evidence_root,
            "private_scan_bundle_root": &source.private_scan_bundle_root,
        }),
    )
}

fn wallet_submission_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "wallet-submission",
        &json!({
            "required": config.require_operator_independent_submission,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "wallet_visible": action_kind.wallet_visible(),
            "wallet_evidence_root": &source.wallet_evidence_root,
            "user_escape_package_root": &source.user_escape_package_root,
            "submission_policy": "wallet_can_submit_without_operator_cooperation",
        }),
    )
}

fn pq_authorization_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "pq-authorization",
        &json!({
            "required": config.require_pq_authorization,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "action_requires_pq": action_kind.requires_pq(),
            "wallet_evidence_root": &source.wallet_evidence_root,
            "authority_scope": "withdrawal_authorization_bridge_release_watcher_quorum",
        }),
    )
}

fn challenge_window_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "challenge-window",
        &json!({
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "challenge_window_blocks": config.challenge_window_blocks,
            "production_hold_root": &source.transcript_production_hold_root,
            "window_policy": "user_claim_remains_submit_or_watchable_until_challenge_window_closes",
        }),
    )
}

fn reserve_fallback_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "reserve-fallback",
        &json!({
            "required": config.require_reserve_fallback,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "action_requires_reserve": action_kind.requires_reserve(),
            "user_escape_package_root": &source.user_escape_package_root,
            "reserve_policy": "liquidity_or_reserve_shortfall_keeps_release_held_and_wallet_recoverable",
        }),
    )
}

fn privacy_redaction_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    ordinal: u64,
) -> String {
    record_root(
        "privacy-redaction",
        &json!({
            "required": config.require_wallet_local_privacy,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "private_scan_bundle_root": &source.private_scan_bundle_root,
            "metadata_policy": "roots_only_no_deposit_exit_linkage_export",
        }),
    )
}

fn fail_closed_recovery_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    status: ForceExitPackageStatus,
    ordinal: u64,
) -> String {
    record_root(
        "fail-closed-recovery",
        &json!({
            "required": config.require_fail_closed_recovery,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "status": status.as_str(),
            "fail_closed_count": source.fail_closed_count,
            "wallet_evidence_root": &source.wallet_evidence_root,
            "recovery_policy": "if_submission_or_receipt_mismatches_wallet_keeps_local_escape_package",
        }),
    )
}

fn release_hold_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    status: ForceExitPackageStatus,
    ordinal: u64,
) -> String {
    record_root(
        "release-hold",
        &json!({
            "required": config.require_release_hold_reporting,
            "ordinal": ordinal,
            "action_kind": action_kind.as_str(),
            "status": status.as_str(),
            "transcript_production_hold_root": &source.transcript_production_hold_root,
            "user_release_blocker_count": source.user_release_blocker_count,
            "production_blocker_count": source.production_blocker_count,
        }),
    )
}

fn action_root(
    config: &Config,
    source: &SourceBundle,
    action_kind: ForceExitActionKind,
    status: ForceExitPackageStatus,
    ordinal: u64,
    command_root: &str,
    transcript_dependency_root: &str,
    wallet_submission_root: &str,
    pq_authorization_root: &str,
    challenge_window_root: &str,
    reserve_fallback_root: &str,
    privacy_redaction_root: &str,
    fail_closed_recovery_root: &str,
    release_hold_root: &str,
    operator_independent: bool,
    blocks_user_release: bool,
    blocks_production: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-ACTION",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.transcript_state_root),
            HashPart::Str(action_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(command_root),
            HashPart::Str(transcript_dependency_root),
            HashPart::Str(wallet_submission_root),
            HashPart::Str(pq_authorization_root),
            HashPart::Str(challenge_window_root),
            HashPart::Str(reserve_fallback_root),
            HashPart::Str(privacy_redaction_root),
            HashPart::Str(fail_closed_recovery_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(bool_str(operator_independent)),
            HashPart::Str(bool_str(blocks_user_release)),
            HashPart::Str(bool_str(blocks_production)),
        ],
        32,
    )
}

fn action_id(action_kind: ForceExitActionKind, ordinal: u64, action_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-ACTION-ID",
        &[
            HashPart::Str(action_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(action_root),
        ],
        16,
    )
}

fn required_outcome(
    status: ForceExitPackageStatus,
    action_kind: ForceExitActionKind,
) -> &'static str {
    match status {
        ForceExitPackageStatus::ReadyToSubmit => {
            "wallet can submit or monitor this force-exit action without operator cooperation"
        }
        ForceExitPackageStatus::DeferredUntilWalletTranscriptLive => match action_kind {
            ForceExitActionKind::PqAuthorizeClaim => {
                "wait for wallet transcript and PQ authorization roots before claim submission"
            }
            ForceExitActionKind::BroadcastExitClaim => {
                "wait for complete wallet transcript before broadcasting force-exit claim"
            }
            ForceExitActionKind::WatchSettlementReceipt => {
                "wait for live receipt evidence before settlement watch is trusted"
            }
            _ => "wait for wallet transcript live evidence before submitting this action",
        },
        ForceExitPackageStatus::ReleaseHeld => {
            "keep release held and expose the blocker report to the wallet"
        }
        ForceExitPackageStatus::FailClosed => {
            "fail closed and preserve operator-independent recovery evidence"
        }
    }
}

fn package_action_vector_root(actions: &[ForceExitPackageAction]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-ACTIONS",
        &actions
            .iter()
            .map(ForceExitPackageAction::public_record)
            .collect::<Vec<_>>(),
    )
}

fn command_bundle_root(
    config: &Config,
    source: &SourceBundle,
    actions: &[ForceExitPackageAction],
    verdict: &ForceExitPackageVerdict,
) -> String {
    let commands = actions
        .iter()
        .map(|action| {
            json!({
                "action_id": &action.action_id,
                "action_kind": action.action_kind.as_str(),
                "command_root": &action.command_root,
                "command": &action.command,
            })
        })
        .collect::<Vec<_>>();
    let command_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-COMMANDS",
        &commands,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-COMMAND-BUNDLE",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.user_escape_package_root),
            HashPart::Str(&command_root),
            HashPart::U64(verdict.package_action_count),
        ],
        32,
    )
}

fn wallet_submission_bundle_root(
    config: &Config,
    source: &SourceBundle,
    actions: &[ForceExitPackageAction],
    verdict: &ForceExitPackageVerdict,
) -> String {
    let wallet_actions = actions
        .iter()
        .filter(|action| action.wallet_visible)
        .map(|action| {
            json!({
                "action_id": &action.action_id,
                "action_kind": action.action_kind.as_str(),
                "wallet_submission_root": &action.wallet_submission_root,
                "blocks_user_release": action.blocks_user_release,
            })
        })
        .collect::<Vec<_>>();
    let wallet_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-WALLET-ACTIONS",
        &wallet_actions,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-WALLET-SUBMISSION",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.wallet_evidence_root),
            HashPart::Str(&wallet_root),
            HashPart::U64(verdict.wallet_visible_count),
            HashPart::Str(bool_str(verdict.package_submit_ready)),
        ],
        32,
    )
}

fn challenge_window_bundle_root(
    config: &Config,
    source: &SourceBundle,
    actions: &[ForceExitPackageAction],
    verdict: &ForceExitPackageVerdict,
) -> String {
    let windows = actions
        .iter()
        .map(|action| {
            json!({
                "action_id": &action.action_id,
                "action_kind": action.action_kind.as_str(),
                "challenge_window_root": &action.challenge_window_root,
            })
        })
        .collect::<Vec<_>>();
    let window_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-CHALLENGE-WINDOWS",
        &windows,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-CHALLENGE-WINDOW",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.transcript_production_hold_root),
            HashPart::Str(&window_root),
            HashPart::U64(config.challenge_window_blocks),
            HashPart::U64(verdict.release_held_count),
        ],
        32,
    )
}

fn recovery_bundle_root(
    config: &Config,
    source: &SourceBundle,
    actions: &[ForceExitPackageAction],
    verdict: &ForceExitPackageVerdict,
) -> String {
    let recovery = actions
        .iter()
        .map(|action| {
            json!({
                "action_id": &action.action_id,
                "action_kind": action.action_kind.as_str(),
                "fail_closed_recovery_root": &action.fail_closed_recovery_root,
                "release_hold_root": &action.release_hold_root,
                "status": action.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let recovery_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-RECOVERY",
        &recovery,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-RECOVERY-BUNDLE",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.user_escape_package_root),
            HashPart::Str(&recovery_root),
            HashPart::U64(verdict.fail_closed_count),
            HashPart::U64(verdict.user_release_blocker_count),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    actions: &[ForceExitPackageAction],
    verdict: &ForceExitPackageVerdict,
) -> String {
    let blockers = actions
        .iter()
        .filter(|action| action.blocks_production)
        .map(|action| {
            json!({
                "action_id": &action.action_id,
                "action_kind": action.action_kind.as_str(),
                "status": action.status.as_str(),
                "release_hold_root": &action.release_hold_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.transcript_production_hold_root),
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
    package_action_root: &str,
    command_bundle_root: &str,
    wallet_submission_bundle_root: &str,
    challenge_window_bundle_root: &str,
    recovery_bundle_root: &str,
    production_hold_root: &str,
    verdict: &ForceExitPackageVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(package_action_root),
            HashPart::Str(command_bundle_root),
            HashPart::Str(wallet_submission_bundle_root),
            HashPart::Str(challenge_window_bundle_root),
            HashPart::Str(recovery_bundle_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    package_action_count: u64,
    ready_action_count: u64,
    deferred_action_count: u64,
    release_held_count: u64,
    fail_closed_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    package_submit_ready: bool,
    production_blocked: bool,
    package_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-VERDICT",
        &[
            HashPart::Str(&config.package_suite),
            HashPart::Str(&source.transcript_state_root),
            HashPart::Str(&source.user_escape_package_root),
            HashPart::U64(package_action_count),
            HashPart::U64(ready_action_count),
            HashPart::U64(deferred_action_count),
            HashPart::U64(release_held_count),
            HashPart::U64(fail_closed_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(package_submit_ready)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(package_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(actions: &[ForceExitPackageAction], status: ForceExitPackageStatus) -> u64 {
    actions
        .iter()
        .filter(|action| action.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "force-exit package chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "force-exit package protocol mismatch",
    )?;
    ensure(
        config.min_package_actions > 0,
        "force-exit package requires actions",
    )?;
    ensure(
        config.min_wallet_actions > 0,
        "force-exit package requires wallet actions",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.transcript_state_root.is_empty(),
        "force-exit package missing transcript state root",
    )?;
    ensure(
        !source.user_escape_package_root.is_empty(),
        "force-exit package missing user escape package root",
    )?;
    ensure(
        source.transcript_entry_count > 0,
        "force-exit package missing transcript entries",
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
        transcript_state_root: record_root(
            "fallback-transcript-state",
            &json!({"reason": &reason}),
        ),
        transcript_entry_root: record_root(
            "fallback-transcript-entry",
            &json!({"reason": &reason}),
        ),
        wallet_evidence_root: record_root("fallback-wallet-evidence", &json!({"reason": &reason})),
        private_scan_bundle_root: record_root("fallback-private-scan", &json!({"reason": &reason})),
        user_escape_package_root: record_root("fallback-user-escape", &json!({"reason": &reason})),
        transcript_production_hold_root: record_root(
            "fallback-production-hold",
            &json!({"reason": &reason}),
        ),
        transcript_status: "fallback".to_string(),
        transcript_user_escape_answer: reason.clone(),
        transcript_production_answer: "fallback".to_string(),
        transcript_entry_count: 1,
        wallet_ready_count: 0,
        deferred_entry_count: 0,
        fail_closed_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        wallet_escape_transcript_complete: false,
        user_escape_answerable: false,
        transcript_production_blocked: true,
    };
    let fallback_action = ForceExitPackageAction::devnet(
        &config,
        &source,
        ForceExitActionKind::FailClosedRecovery,
        1,
    );
    let package_actions = vec![fallback_action];
    let verdict = ForceExitPackageVerdict::new(&config, &source, &package_actions);
    let package_action_root = package_action_vector_root(&package_actions);
    let command_bundle_root = command_bundle_root(&config, &source, &package_actions, &verdict);
    let wallet_submission_bundle_root =
        wallet_submission_bundle_root(&config, &source, &package_actions, &verdict);
    let challenge_window_bundle_root =
        challenge_window_bundle_root(&config, &source, &package_actions, &verdict);
    let recovery_bundle_root = recovery_bundle_root(&config, &source, &package_actions, &verdict);
    let production_hold_root = production_hold_root(&config, &source, &package_actions, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &package_action_root,
        &command_bundle_root,
        &wallet_submission_bundle_root,
        &challenge_window_bundle_root,
        &recovery_bundle_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        package_actions,
        verdict,
        package_action_root,
        command_bundle_root,
        wallet_submission_bundle_root,
        challenge_window_bundle_root,
        recovery_bundle_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-OPERATOR-INDEPENDENT-FORCE-EXIT-PACKAGE-RECORD",
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
