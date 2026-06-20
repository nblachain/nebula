use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_live_receipt_replay_harness_runtime as replay_harness,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceWalletEscapeTranscriptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_WALLET_ESCAPE_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-wallet-escape-transcript-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_WALLET_ESCAPE_TRANSCRIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WALLET_ESCAPE_TRANSCRIPT_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-wallet-escape-transcript-v1";
pub const DEFAULT_MIN_TRANSCRIPT_ENTRIES: u64 = 9;
pub const DEFAULT_MIN_USER_ESCAPE_ENTRIES: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub transcript_suite: String,
    pub min_transcript_entries: u64,
    pub min_user_escape_entries: u64,
    pub require_wallet_local_commands: bool,
    pub require_private_scan_roots: bool,
    pub require_pq_authorized_claim: bool,
    pub require_fail_closed_notice: bool,
    pub require_release_hold_notice: bool,
    pub require_metadata_redaction: bool,
    pub hold_production_until_wallet_replay_observed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            transcript_suite: WALLET_ESCAPE_TRANSCRIPT_SUITE.to_string(),
            min_transcript_entries: DEFAULT_MIN_TRANSCRIPT_ENTRIES,
            min_user_escape_entries: DEFAULT_MIN_USER_ESCAPE_ENTRIES,
            require_wallet_local_commands: true,
            require_private_scan_roots: true,
            require_pq_authorized_claim: true,
            require_fail_closed_notice: true,
            require_release_hold_notice: true,
            require_metadata_redaction: true,
            hold_production_until_wallet_replay_observed: true,
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
            "transcript_suite": self.transcript_suite,
            "min_transcript_entries": self.min_transcript_entries,
            "min_user_escape_entries": self.min_user_escape_entries,
            "require_wallet_local_commands": self.require_wallet_local_commands,
            "require_private_scan_roots": self.require_private_scan_roots,
            "require_pq_authorized_claim": self.require_pq_authorized_claim,
            "require_fail_closed_notice": self.require_fail_closed_notice,
            "require_release_hold_notice": self.require_release_hold_notice,
            "require_metadata_redaction": self.require_metadata_redaction,
            "hold_production_until_wallet_replay_observed": self.hold_production_until_wallet_replay_observed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletTranscriptEntryKind {
    MoneroHeaderCheckpoint,
    DepositLockClaim,
    PrivateNoteRecovery,
    PrivateActionReceipt,
    SettlementReceipt,
    ReserveLiquidityProof,
    PqAuthorityAuthorization,
    WalletPrivacyScan,
    ReleaseBlockerNotice,
}

impl WalletTranscriptEntryKind {
    pub fn from_replay_kind(kind: replay_harness::ReplayStepKind) -> Self {
        match kind {
            replay_harness::ReplayStepKind::MoneroHeaderCanonicality => {
                Self::MoneroHeaderCheckpoint
            }
            replay_harness::ReplayStepKind::DepositLockWatcher => Self::DepositLockClaim,
            replay_harness::ReplayStepKind::PrivateNoteState => Self::PrivateNoteRecovery,
            replay_harness::ReplayStepKind::TransferOrContractExecution => {
                Self::PrivateActionReceipt
            }
            replay_harness::ReplayStepKind::SettlementReceiptExecutor => Self::SettlementReceipt,
            replay_harness::ReplayStepKind::ReserveLiquidity => Self::ReserveLiquidityProof,
            replay_harness::ReplayStepKind::PqAuthorityQuorum => Self::PqAuthorityAuthorization,
            replay_harness::ReplayStepKind::WalletScannerPrivacy => Self::WalletPrivacyScan,
            replay_harness::ReplayStepKind::ReleaseBlockerClearing => Self::ReleaseBlockerNotice,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoneroHeaderCheckpoint => "monero_header_checkpoint",
            Self::DepositLockClaim => "deposit_lock_claim",
            Self::PrivateNoteRecovery => "private_note_recovery",
            Self::PrivateActionReceipt => "private_action_receipt",
            Self::SettlementReceipt => "settlement_receipt",
            Self::ReserveLiquidityProof => "reserve_liquidity_proof",
            Self::PqAuthorityAuthorization => "pq_authority_authorization",
            Self::WalletPrivacyScan => "wallet_privacy_scan",
            Self::ReleaseBlockerNotice => "release_blocker_notice",
        }
    }

    pub fn wallet_action(self) -> &'static str {
        match self {
            Self::MoneroHeaderCheckpoint => "record_canonical_header_window",
            Self::DepositLockClaim => "bind_deposit_lock_to_wallet_entry_claim",
            Self::PrivateNoteRecovery => "scan_encrypted_note_commitments_locally",
            Self::PrivateActionReceipt => "verify_private_transfer_or_contract_receipt",
            Self::SettlementReceipt => "attach_settlement_receipt_to_exit_package",
            Self::ReserveLiquidityProof => "check_reserve_liquidity_sufficiency_root",
            Self::PqAuthorityAuthorization => "verify_post_quantum_authority_quorum",
            Self::WalletPrivacyScan => "prove_wallet_discovery_without_metadata_export",
            Self::ReleaseBlockerNotice => "record_release_hold_or_blocker_clearance_notice",
        }
    }

    pub fn privacy_sensitive(self) -> bool {
        matches!(
            self,
            Self::DepositLockClaim
                | Self::PrivateNoteRecovery
                | Self::PrivateActionReceipt
                | Self::SettlementReceipt
                | Self::WalletPrivacyScan
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletTranscriptStatus {
    WalletReady,
    DeferredUntilLiveReplay,
    ReleaseHeld,
    FailClosed,
}

impl WalletTranscriptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletReady => "wallet_ready",
            Self::DeferredUntilLiveReplay => "deferred_until_live_replay",
            Self::ReleaseHeld => "release_held",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub replay_state_root: String,
    pub replay_step_root: String,
    pub command_vector_root: String,
    pub wallet_replay_root: String,
    pub fail_closed_replay_root: String,
    pub production_hold_root: String,
    pub replay_status: String,
    pub replay_user_escape_answer: String,
    pub replay_production_answer: String,
    pub replay_step_count: u64,
    pub ready_step_count: u64,
    pub deferred_step_count: u64,
    pub fail_closed_step_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub replay_executable: bool,
    pub user_escape_replay_executable: bool,
    pub production_blocked: bool,
}

impl SourceBundle {
    pub fn from_replay(state: &replay_harness::State) -> Self {
        Self {
            replay_state_root: state.state_root(),
            replay_step_root: state.replay_step_root.clone(),
            command_vector_root: state.command_vector_root.clone(),
            wallet_replay_root: state.wallet_replay_root.clone(),
            fail_closed_replay_root: state.fail_closed_replay_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            replay_status: state.verdict.replay_status.clone(),
            replay_user_escape_answer: state.verdict.user_escape_answer.clone(),
            replay_production_answer: state.verdict.production_answer.clone(),
            replay_step_count: state.verdict.replay_step_count,
            ready_step_count: state.verdict.ready_step_count,
            deferred_step_count: state.verdict.deferred_step_count,
            fail_closed_step_count: state.verdict.fail_closed_step_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            replay_executable: state.verdict.replay_executable,
            user_escape_replay_executable: state.verdict.user_escape_replay_executable,
            production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let replay = replay_harness::devnet();
        Self::from_replay(&replay)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_state_root": self.replay_state_root,
            "replay_step_root": self.replay_step_root,
            "command_vector_root": self.command_vector_root,
            "wallet_replay_root": self.wallet_replay_root,
            "fail_closed_replay_root": self.fail_closed_replay_root,
            "production_hold_root": self.production_hold_root,
            "replay_status": self.replay_status,
            "replay_user_escape_answer": self.replay_user_escape_answer,
            "replay_production_answer": self.replay_production_answer,
            "replay_step_count": self.replay_step_count,
            "ready_step_count": self.ready_step_count,
            "deferred_step_count": self.deferred_step_count,
            "fail_closed_step_count": self.fail_closed_step_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "replay_executable": self.replay_executable,
            "user_escape_replay_executable": self.user_escape_replay_executable,
            "production_blocked": self.production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletEscapeTranscriptEntry {
    pub entry_id: String,
    pub ordinal: u64,
    pub entry_kind: WalletTranscriptEntryKind,
    pub replay_step_id: String,
    pub replay_status: String,
    pub wallet_action_root: String,
    pub wallet_local_evidence_root: String,
    pub private_scan_root: String,
    pub pq_authorization_root: String,
    pub metadata_redaction_root: String,
    pub fail_closed_notice_root: String,
    pub release_hold_notice_root: String,
    pub transcript_entry_root: String,
    pub status: WalletTranscriptStatus,
    pub privacy_sensitive: bool,
    pub user_escape_entry: bool,
    pub blocks_user_release: bool,
    pub blocks_production: bool,
    pub wallet_action: String,
    pub wallet_instruction: String,
}

impl WalletEscapeTranscriptEntry {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        replay_step: &replay_harness::LiveReceiptReplayStep,
    ) -> Self {
        let entry_kind = WalletTranscriptEntryKind::from_replay_kind(replay_step.step_kind);
        let status = transcript_status(replay_step.status);
        let wallet_action_root = wallet_action_root(config, source, replay_step, entry_kind);
        let wallet_local_evidence_root =
            wallet_local_evidence_root(config, source, replay_step, entry_kind);
        let private_scan_root = private_scan_root(config, source, replay_step, entry_kind);
        let pq_authorization_root = pq_authorization_root(config, source, replay_step, entry_kind);
        let metadata_redaction_root =
            metadata_redaction_root(config, source, replay_step, entry_kind);
        let fail_closed_notice_root =
            fail_closed_notice_root(config, source, replay_step, entry_kind, status);
        let release_hold_notice_root =
            release_hold_notice_root(config, source, replay_step, entry_kind, status);
        let transcript_entry_root = transcript_entry_root(
            config,
            source,
            replay_step,
            entry_kind,
            status,
            &wallet_action_root,
            &wallet_local_evidence_root,
            &private_scan_root,
            &pq_authorization_root,
            &metadata_redaction_root,
            &fail_closed_notice_root,
            &release_hold_notice_root,
        );
        let entry_id = entry_id(entry_kind, replay_step.ordinal, &transcript_entry_root);
        let privacy_sensitive = entry_kind.privacy_sensitive();
        let user_escape_entry = replay_step.user_escape_step;
        let blocks_user_release =
            replay_step.blocks_user_release || status == WalletTranscriptStatus::FailClosed;
        let blocks_production =
            replay_step.blocks_production || status != WalletTranscriptStatus::WalletReady;
        Self {
            entry_id,
            ordinal: replay_step.ordinal,
            entry_kind,
            replay_step_id: replay_step.step_id.clone(),
            replay_status: replay_step.status.as_str().to_string(),
            wallet_action_root,
            wallet_local_evidence_root,
            private_scan_root,
            pq_authorization_root,
            metadata_redaction_root,
            fail_closed_notice_root,
            release_hold_notice_root,
            transcript_entry_root,
            status,
            privacy_sensitive,
            user_escape_entry,
            blocks_user_release,
            blocks_production,
            wallet_action: entry_kind.wallet_action().to_string(),
            wallet_instruction: wallet_instruction(status, entry_kind).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "entry_id": self.entry_id,
            "ordinal": self.ordinal,
            "entry_kind": self.entry_kind.as_str(),
            "replay_step_id": self.replay_step_id,
            "replay_status": self.replay_status,
            "wallet_action_root": self.wallet_action_root,
            "wallet_local_evidence_root": self.wallet_local_evidence_root,
            "private_scan_root": self.private_scan_root,
            "pq_authorization_root": self.pq_authorization_root,
            "metadata_redaction_root": self.metadata_redaction_root,
            "fail_closed_notice_root": self.fail_closed_notice_root,
            "release_hold_notice_root": self.release_hold_notice_root,
            "transcript_entry_root": self.transcript_entry_root,
            "status": self.status.as_str(),
            "privacy_sensitive": self.privacy_sensitive,
            "user_escape_entry": self.user_escape_entry,
            "blocks_user_release": self.blocks_user_release,
            "blocks_production": self.blocks_production,
            "wallet_action": self.wallet_action,
            "wallet_instruction": self.wallet_instruction,
        })
    }

    pub fn state_root(&self) -> String {
        self.transcript_entry_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletEscapeTranscriptVerdict {
    pub transcript_entry_count: u64,
    pub wallet_ready_count: u64,
    pub deferred_entry_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub user_escape_entry_count: u64,
    pub privacy_sensitive_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub replay_step_count: u64,
    pub replay_executable: bool,
    pub user_escape_replay_executable: bool,
    pub wallet_local_commands_present: bool,
    pub private_scan_roots_present: bool,
    pub pq_authorizations_present: bool,
    pub metadata_redaction_present: bool,
    pub fail_closed_notices_present: bool,
    pub release_hold_notices_present: bool,
    pub wallet_escape_transcript_complete: bool,
    pub user_escape_answerable: bool,
    pub production_blocked: bool,
    pub transcript_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl WalletEscapeTranscriptVerdict {
    pub fn new(
        config: &Config,
        source: &SourceBundle,
        entries: &[WalletEscapeTranscriptEntry],
    ) -> Self {
        let transcript_entry_count = entries.len() as u64;
        let wallet_ready_count = count_status(entries, WalletTranscriptStatus::WalletReady);
        let deferred_entry_count =
            count_status(entries, WalletTranscriptStatus::DeferredUntilLiveReplay);
        let release_held_count = count_status(entries, WalletTranscriptStatus::ReleaseHeld);
        let fail_closed_count = count_status(entries, WalletTranscriptStatus::FailClosed);
        let user_escape_entry_count = entries
            .iter()
            .filter(|entry| entry.user_escape_entry)
            .count() as u64;
        let privacy_sensitive_count = entries
            .iter()
            .filter(|entry| entry.privacy_sensitive)
            .count() as u64;
        let user_release_blocker_count = entries
            .iter()
            .filter(|entry| entry.blocks_user_release)
            .count() as u64;
        let production_blocker_count = entries
            .iter()
            .filter(|entry| entry.blocks_production)
            .count() as u64;
        let replay_step_count = source.replay_step_count;
        let replay_executable = source.replay_executable;
        let user_escape_replay_executable = source.user_escape_replay_executable;
        let wallet_local_commands_present = !config.require_wallet_local_commands
            || entries
                .iter()
                .all(|entry| !entry.wallet_action_root.is_empty());
        let private_scan_roots_present = !config.require_private_scan_roots
            || entries
                .iter()
                .filter(|entry| entry.privacy_sensitive)
                .all(|entry| !entry.private_scan_root.is_empty());
        let pq_authorizations_present = !config.require_pq_authorized_claim
            || entries
                .iter()
                .all(|entry| !entry.pq_authorization_root.is_empty());
        let metadata_redaction_present = !config.require_metadata_redaction
            || entries
                .iter()
                .all(|entry| !entry.metadata_redaction_root.is_empty());
        let fail_closed_notices_present = !config.require_fail_closed_notice
            || entries
                .iter()
                .all(|entry| !entry.fail_closed_notice_root.is_empty());
        let release_hold_notices_present = !config.require_release_hold_notice
            || entries
                .iter()
                .all(|entry| !entry.release_hold_notice_root.is_empty());
        let wallet_escape_transcript_complete = transcript_entry_count
            >= config.min_transcript_entries
            && transcript_entry_count == replay_step_count
            && user_escape_entry_count >= config.min_user_escape_entries
            && wallet_local_commands_present
            && private_scan_roots_present
            && pq_authorizations_present
            && metadata_redaction_present
            && fail_closed_notices_present
            && release_hold_notices_present;
        let user_escape_answerable = wallet_escape_transcript_complete
            && user_release_blocker_count == 0
            && replay_executable
            && user_escape_replay_executable;
        let production_blocked = source.production_blocked
            || production_blocker_count > 0
            || (config.hold_production_until_wallet_replay_observed && !user_escape_answerable);
        let transcript_status = if fail_closed_count > 0 {
            "fail_closed"
        } else if release_held_count > 0 {
            "release_held"
        } else if deferred_entry_count > 0 {
            "wallet_transcript_deferred_until_live_replay"
        } else if user_escape_answerable {
            "wallet_escape_transcript_complete"
        } else {
            "incomplete"
        }
        .to_string();
        let user_escape_answer = if user_escape_answerable {
            "wallet has a complete live-observed escape transcript"
        } else {
            "wallet escape transcript is structured but remains deferred until live replay evidence clears all user-release blockers"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains blocked until wallet transcript entries are live-observed, private, PQ-authorized, and replay-clean"
        } else {
            "bounded bridge/exit wallet transcript is ready for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            transcript_entry_count,
            wallet_ready_count,
            deferred_entry_count,
            release_held_count,
            fail_closed_count,
            user_release_blocker_count,
            production_blocker_count,
            wallet_escape_transcript_complete,
            user_escape_answerable,
            production_blocked,
            &transcript_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            transcript_entry_count,
            wallet_ready_count,
            deferred_entry_count,
            release_held_count,
            fail_closed_count,
            user_escape_entry_count,
            privacy_sensitive_count,
            user_release_blocker_count,
            production_blocker_count,
            replay_step_count,
            replay_executable,
            user_escape_replay_executable,
            wallet_local_commands_present,
            private_scan_roots_present,
            pq_authorizations_present,
            metadata_redaction_present,
            fail_closed_notices_present,
            release_hold_notices_present,
            wallet_escape_transcript_complete,
            user_escape_answerable,
            production_blocked,
            transcript_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "transcript_entry_count": self.transcript_entry_count,
            "wallet_ready_count": self.wallet_ready_count,
            "deferred_entry_count": self.deferred_entry_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "user_escape_entry_count": self.user_escape_entry_count,
            "privacy_sensitive_count": self.privacy_sensitive_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "replay_step_count": self.replay_step_count,
            "replay_executable": self.replay_executable,
            "user_escape_replay_executable": self.user_escape_replay_executable,
            "wallet_local_commands_present": self.wallet_local_commands_present,
            "private_scan_roots_present": self.private_scan_roots_present,
            "pq_authorizations_present": self.pq_authorizations_present,
            "metadata_redaction_present": self.metadata_redaction_present,
            "fail_closed_notices_present": self.fail_closed_notices_present,
            "release_hold_notices_present": self.release_hold_notices_present,
            "wallet_escape_transcript_complete": self.wallet_escape_transcript_complete,
            "user_escape_answerable": self.user_escape_answerable,
            "production_blocked": self.production_blocked,
            "transcript_status": self.transcript_status,
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
    pub transcript_entries: Vec<WalletEscapeTranscriptEntry>,
    pub verdict: WalletEscapeTranscriptVerdict,
    pub transcript_entry_root: String,
    pub wallet_evidence_root: String,
    pub private_scan_bundle_root: String,
    pub user_escape_package_root: String,
    pub production_hold_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, replay_state: replay_harness::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_replay(&replay_state);
        validate_source(&source)?;
        let transcript_entries = replay_state
            .replay_steps
            .iter()
            .map(|step| WalletEscapeTranscriptEntry::devnet(&config, &source, step))
            .collect::<Vec<_>>();
        let verdict = WalletEscapeTranscriptVerdict::new(&config, &source, &transcript_entries);
        let transcript_entry_root = transcript_entry_vector_root(&transcript_entries);
        let wallet_evidence_root =
            wallet_evidence_root(&config, &source, &transcript_entries, &verdict);
        let private_scan_bundle_root =
            private_scan_bundle_root(&config, &source, &transcript_entries, &verdict);
        let user_escape_package_root =
            user_escape_package_root(&config, &source, &transcript_entries, &verdict);
        let production_hold_root =
            production_hold_root(&config, &source, &transcript_entries, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &transcript_entry_root,
            &wallet_evidence_root,
            &private_scan_bundle_root,
            &user_escape_package_root,
            &production_hold_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            transcript_entries,
            verdict,
            transcript_entry_root,
            wallet_evidence_root,
            private_scan_bundle_root,
            user_escape_package_root,
            production_hold_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), replay_harness::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_wallet_escape_transcript_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "transcript_entry_root": self.transcript_entry_root,
            "wallet_evidence_root": self.wallet_evidence_root,
            "private_scan_bundle_root": self.private_scan_bundle_root,
            "user_escape_package_root": self.user_escape_package_root,
            "production_hold_root": self.production_hold_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "transcript_entries": self
                .transcript_entries
                .iter()
                .map(WalletEscapeTranscriptEntry::public_record)
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

fn transcript_status(status: replay_harness::ReplayStepStatus) -> WalletTranscriptStatus {
    match status {
        replay_harness::ReplayStepStatus::ReadyToReplay => WalletTranscriptStatus::WalletReady,
        replay_harness::ReplayStepStatus::DeferredUntilLiveReceipt => {
            WalletTranscriptStatus::DeferredUntilLiveReplay
        }
        replay_harness::ReplayStepStatus::ProductionHold => WalletTranscriptStatus::ReleaseHeld,
        replay_harness::ReplayStepStatus::FailClosed => WalletTranscriptStatus::FailClosed,
    }
}

fn wallet_action_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
) -> String {
    record_root(
        "wallet-action",
        &json!({
            "transcript_suite": config.transcript_suite,
            "entry_kind": entry_kind.as_str(),
            "wallet_action": entry_kind.wallet_action(),
            "replay_step_id": step.step_id,
            "command_root": step.command_root,
            "wallet_replay_root": source.wallet_replay_root,
        }),
    )
}

fn wallet_local_evidence_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
) -> String {
    record_root(
        "wallet-local-evidence",
        &json!({
            "required": config.require_wallet_local_commands,
            "entry_kind": entry_kind.as_str(),
            "replay_step_id": step.step_id,
            "wallet_instruction_root": step.wallet_instruction_root,
            "replay_result_root": step.replay_result_root,
            "user_escape_package_source_root": source.wallet_replay_root,
        }),
    )
}

fn private_scan_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
) -> String {
    record_root(
        "private-scan",
        &json!({
            "required": config.require_private_scan_roots,
            "privacy_sensitive": entry_kind.privacy_sensitive(),
            "entry_kind": entry_kind.as_str(),
            "replay_step_id": step.step_id,
            "wallet_instruction_root": step.wallet_instruction_root,
            "wallet_replay_root": source.wallet_replay_root,
            "scan_policy": "wallet_local_roots_only_no_linkage_export",
        }),
    )
}

fn pq_authorization_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
) -> String {
    record_root(
        "pq-authorization",
        &json!({
            "required": config.require_pq_authorized_claim,
            "entry_kind": entry_kind.as_str(),
            "replay_step_id": step.step_id,
            "operator_instruction_root": step.operator_instruction_root,
            "command_vector_root": source.command_vector_root,
            "authority_scope": "watcher_bridge_upgrade_withdrawal_authorization",
        }),
    )
}

fn metadata_redaction_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
) -> String {
    record_root(
        "metadata-redaction",
        &json!({
            "required": config.require_metadata_redaction,
            "entry_kind": entry_kind.as_str(),
            "privacy_sensitive": entry_kind.privacy_sensitive(),
            "replay_step_id": step.step_id,
            "wallet_replay_root": source.wallet_replay_root,
            "metadata_policy": "redacted_roots_no_deposit_exit_join",
        }),
    )
}

fn fail_closed_notice_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
    status: WalletTranscriptStatus,
) -> String {
    record_root(
        "fail-closed-notice",
        &json!({
            "required": config.require_fail_closed_notice,
            "entry_kind": entry_kind.as_str(),
            "status": status.as_str(),
            "replay_step_id": step.step_id,
            "fail_closed_receipt_root": step.fail_closed_receipt_root,
            "fail_closed_replay_root": source.fail_closed_replay_root,
            "policy": "wallet_preserves_escape_evidence_when_live_replay_is_missing_or_mismatched",
        }),
    )
}

fn release_hold_notice_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
    status: WalletTranscriptStatus,
) -> String {
    record_root(
        "release-hold-notice",
        &json!({
            "required": config.require_release_hold_notice,
            "entry_kind": entry_kind.as_str(),
            "status": status.as_str(),
            "replay_step_id": step.step_id,
            "step_release_hold_root": step.release_hold_root,
            "production_hold_root": source.production_hold_root,
            "blocks_user_release": step.blocks_user_release,
            "blocks_production": step.blocks_production,
        }),
    )
}

fn transcript_entry_root(
    config: &Config,
    source: &SourceBundle,
    step: &replay_harness::LiveReceiptReplayStep,
    entry_kind: WalletTranscriptEntryKind,
    status: WalletTranscriptStatus,
    wallet_action_root: &str,
    wallet_local_evidence_root: &str,
    private_scan_root: &str,
    pq_authorization_root: &str,
    metadata_redaction_root: &str,
    fail_closed_notice_root: &str,
    release_hold_notice_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-ENTRY",
        &[
            HashPart::Str(&config.transcript_suite),
            HashPart::Str(&source.replay_state_root),
            HashPart::Str(entry_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&step.step_id),
            HashPart::Str(&step.replay_result_root),
            HashPart::Str(wallet_action_root),
            HashPart::Str(wallet_local_evidence_root),
            HashPart::Str(private_scan_root),
            HashPart::Str(pq_authorization_root),
            HashPart::Str(metadata_redaction_root),
            HashPart::Str(fail_closed_notice_root),
            HashPart::Str(release_hold_notice_root),
        ],
        32,
    )
}

fn entry_id(
    entry_kind: WalletTranscriptEntryKind,
    ordinal: u64,
    transcript_entry_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-ENTRY-ID",
        &[
            HashPart::Str(entry_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(transcript_entry_root),
        ],
        16,
    )
}

fn wallet_instruction(
    status: WalletTranscriptStatus,
    entry_kind: WalletTranscriptEntryKind,
) -> &'static str {
    match status {
        WalletTranscriptStatus::WalletReady => {
            "retain local evidence roots and prepare force-exit package"
        }
        WalletTranscriptStatus::DeferredUntilLiveReplay => match entry_kind {
            WalletTranscriptEntryKind::DepositLockClaim => {
                "wait for live deposit lock replay before claiming entry"
            }
            WalletTranscriptEntryKind::PrivateNoteRecovery => {
                "keep scanning encrypted note roots locally until live replay confirms state"
            }
            WalletTranscriptEntryKind::SettlementReceipt => {
                "wait for live settlement receipt replay before presenting withdrawal evidence"
            }
            WalletTranscriptEntryKind::WalletPrivacyScan => {
                "do not export metadata while waiting for wallet scanner replay"
            }
            _ => "wait for live replay evidence before treating this transcript entry as complete",
        },
        WalletTranscriptStatus::ReleaseHeld => {
            "show release-held notice and preserve wallet-local evidence"
        }
        WalletTranscriptStatus::FailClosed => {
            "fail closed and preserve force-exit evidence for user-controlled recovery"
        }
    }
}

fn transcript_entry_vector_root(entries: &[WalletEscapeTranscriptEntry]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-ENTRIES",
        &entries
            .iter()
            .map(WalletEscapeTranscriptEntry::public_record)
            .collect::<Vec<_>>(),
    )
}

fn wallet_evidence_root(
    config: &Config,
    source: &SourceBundle,
    entries: &[WalletEscapeTranscriptEntry],
    verdict: &WalletEscapeTranscriptVerdict,
) -> String {
    let evidence = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_id": entry.entry_id,
                "entry_kind": entry.entry_kind.as_str(),
                "wallet_action_root": entry.wallet_action_root,
                "wallet_local_evidence_root": entry.wallet_local_evidence_root,
                "status": entry.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let evidence_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-EVIDENCE",
        &evidence,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-WALLET-EVIDENCE",
        &[
            HashPart::Str(&config.transcript_suite),
            HashPart::Str(&source.wallet_replay_root),
            HashPart::Str(&evidence_root),
            HashPart::U64(verdict.wallet_ready_count),
            HashPart::U64(verdict.deferred_entry_count),
        ],
        32,
    )
}

fn private_scan_bundle_root(
    config: &Config,
    source: &SourceBundle,
    entries: &[WalletEscapeTranscriptEntry],
    verdict: &WalletEscapeTranscriptVerdict,
) -> String {
    let private_entries = entries
        .iter()
        .filter(|entry| entry.privacy_sensitive)
        .map(|entry| {
            json!({
                "entry_id": entry.entry_id,
                "entry_kind": entry.entry_kind.as_str(),
                "private_scan_root": entry.private_scan_root,
                "metadata_redaction_root": entry.metadata_redaction_root,
            })
        })
        .collect::<Vec<_>>();
    let private_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-PRIVATE-SCAN",
        &private_entries,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-PRIVATE-SCAN-BUNDLE",
        &[
            HashPart::Str(&config.transcript_suite),
            HashPart::Str(&source.wallet_replay_root),
            HashPart::Str(&private_root),
            HashPart::U64(verdict.privacy_sensitive_count),
            HashPart::Str(bool_str(verdict.metadata_redaction_present)),
        ],
        32,
    )
}

fn user_escape_package_root(
    config: &Config,
    source: &SourceBundle,
    entries: &[WalletEscapeTranscriptEntry],
    verdict: &WalletEscapeTranscriptVerdict,
) -> String {
    let user_entries = entries
        .iter()
        .filter(|entry| entry.user_escape_entry)
        .map(|entry| {
            json!({
                "entry_id": entry.entry_id,
                "entry_kind": entry.entry_kind.as_str(),
                "transcript_entry_root": entry.transcript_entry_root,
                "blocks_user_release": entry.blocks_user_release,
                "wallet_instruction": entry.wallet_instruction,
            })
        })
        .collect::<Vec<_>>();
    let user_entry_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-USER-PACKAGE",
        &user_entries,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-USER-ESCAPE-PACKAGE",
        &[
            HashPart::Str(&config.transcript_suite),
            HashPart::Str(&source.wallet_replay_root),
            HashPart::Str(&source.fail_closed_replay_root),
            HashPart::Str(&user_entry_root),
            HashPart::U64(verdict.user_release_blocker_count),
            HashPart::Str(bool_str(verdict.user_escape_answerable)),
        ],
        32,
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    entries: &[WalletEscapeTranscriptEntry],
    verdict: &WalletEscapeTranscriptVerdict,
) -> String {
    let blockers = entries
        .iter()
        .filter(|entry| entry.blocks_production)
        .map(|entry| {
            json!({
                "entry_id": entry.entry_id,
                "entry_kind": entry.entry_kind.as_str(),
                "status": entry.status.as_str(),
                "release_hold_notice_root": entry.release_hold_notice_root,
            })
        })
        .collect::<Vec<_>>();
    let blocker_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-PRODUCTION-BLOCKERS",
        &blockers,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.transcript_suite),
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
    transcript_entry_root: &str,
    wallet_evidence_root: &str,
    private_scan_bundle_root: &str,
    user_escape_package_root: &str,
    production_hold_root: &str,
    verdict: &WalletEscapeTranscriptVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(transcript_entry_root),
            HashPart::Str(wallet_evidence_root),
            HashPart::Str(private_scan_bundle_root),
            HashPart::Str(user_escape_package_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    transcript_entry_count: u64,
    wallet_ready_count: u64,
    deferred_entry_count: u64,
    release_held_count: u64,
    fail_closed_count: u64,
    user_release_blocker_count: u64,
    production_blocker_count: u64,
    wallet_escape_transcript_complete: bool,
    user_escape_answerable: bool,
    production_blocked: bool,
    transcript_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-VERDICT",
        &[
            HashPart::Str(&config.transcript_suite),
            HashPart::Str(&source.replay_state_root),
            HashPart::Str(&source.wallet_replay_root),
            HashPart::U64(transcript_entry_count),
            HashPart::U64(wallet_ready_count),
            HashPart::U64(deferred_entry_count),
            HashPart::U64(release_held_count),
            HashPart::U64(fail_closed_count),
            HashPart::U64(user_release_blocker_count),
            HashPart::U64(production_blocker_count),
            HashPart::Str(bool_str(wallet_escape_transcript_complete)),
            HashPart::Str(bool_str(user_escape_answerable)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(transcript_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(entries: &[WalletEscapeTranscriptEntry], status: WalletTranscriptStatus) -> u64 {
    entries
        .iter()
        .filter(|entry| entry.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "wallet escape transcript chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "wallet escape transcript protocol mismatch",
    )?;
    ensure(
        config.min_transcript_entries > 0,
        "wallet escape transcript requires entries",
    )?;
    ensure(
        config.min_user_escape_entries > 0,
        "wallet escape transcript requires user escape entries",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.replay_state_root.is_empty(),
        "wallet escape transcript missing replay state root",
    )?;
    ensure(
        !source.wallet_replay_root.is_empty(),
        "wallet escape transcript missing wallet replay root",
    )?;
    ensure(
        source.replay_step_count > 0,
        "wallet escape transcript missing replay steps",
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
        replay_state_root: record_root("fallback-replay-state", &json!({"reason": &reason})),
        replay_step_root: record_root("fallback-replay-step", &json!({"reason": &reason})),
        command_vector_root: record_root("fallback-command", &json!({"reason": &reason})),
        wallet_replay_root: record_root("fallback-wallet-replay", &json!({"reason": &reason})),
        fail_closed_replay_root: record_root("fallback-fail-closed", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        replay_status: "fallback".to_string(),
        replay_user_escape_answer: reason.clone(),
        replay_production_answer: "fallback".to_string(),
        replay_step_count: 1,
        ready_step_count: 0,
        deferred_step_count: 0,
        fail_closed_step_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        replay_executable: false,
        user_escape_replay_executable: false,
        production_blocked: true,
    };
    let fallback_root = record_root("fallback-entry", &json!({"reason": &reason}));
    let fallback_entry = WalletEscapeTranscriptEntry {
        entry_id: entry_id(
            WalletTranscriptEntryKind::ReleaseBlockerNotice,
            1,
            &fallback_root,
        ),
        ordinal: 1,
        entry_kind: WalletTranscriptEntryKind::ReleaseBlockerNotice,
        replay_step_id: "fallback".to_string(),
        replay_status: "fallback".to_string(),
        wallet_action_root: source.command_vector_root.clone(),
        wallet_local_evidence_root: source.wallet_replay_root.clone(),
        private_scan_root: source.wallet_replay_root.clone(),
        pq_authorization_root: source.command_vector_root.clone(),
        metadata_redaction_root: source.wallet_replay_root.clone(),
        fail_closed_notice_root: source.fail_closed_replay_root.clone(),
        release_hold_notice_root: source.production_hold_root.clone(),
        transcript_entry_root: fallback_root,
        status: WalletTranscriptStatus::FailClosed,
        privacy_sensitive: true,
        user_escape_entry: true,
        blocks_user_release: true,
        blocks_production: true,
        wallet_action: "fallback".to_string(),
        wallet_instruction: reason,
    };
    let transcript_entries = vec![fallback_entry];
    let verdict = WalletEscapeTranscriptVerdict::new(&config, &source, &transcript_entries);
    let transcript_entry_root = transcript_entry_vector_root(&transcript_entries);
    let wallet_evidence_root =
        wallet_evidence_root(&config, &source, &transcript_entries, &verdict);
    let private_scan_bundle_root =
        private_scan_bundle_root(&config, &source, &transcript_entries, &verdict);
    let user_escape_package_root =
        user_escape_package_root(&config, &source, &transcript_entries, &verdict);
    let production_hold_root =
        production_hold_root(&config, &source, &transcript_entries, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &transcript_entry_root,
        &wallet_evidence_root,
        &private_scan_bundle_root,
        &user_escape_package_root,
        &production_hold_root,
        &verdict,
    );
    State {
        config,
        source,
        transcript_entries,
        verdict,
        transcript_entry_root,
        wallet_evidence_root,
        private_scan_bundle_root,
        user_escape_package_root,
        production_hold_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WALLET-ESCAPE-TRANSCRIPT-RECORD",
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
