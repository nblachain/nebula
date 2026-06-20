use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletWatchtowerAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str = "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-watchtower-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_WATCHTOWER_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const IMPORT_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-wallet-watchtower-live-evidence-import-v1";
pub const DEFAULT_MIN_WALLET_TRANSCRIPTS: usize = 3;
pub const DEFAULT_MIN_WATCHTOWER_REPLAYS: usize = 3;
pub const DEFAULT_MIN_RECOVERY_NOTICES: usize = 2;
pub const DEFAULT_MIN_CHALLENGE_OBSERVATIONS: usize = 4;
pub const DEFAULT_MAX_IMPORT_LAG_L2_BLOCKS: u64 = 12;
pub const DEFAULT_MAX_NOTICE_LAG_L2_BLOCKS: u64 = 24;
pub const DEFAULT_MAX_REPLAY_DIVERGENCES: u64 = 0;
pub const DEFAULT_MIN_WATCHTOWER_QUORUM_BPS: u64 = 6_700;
pub const DEFAULT_MIN_WALLET_AUTHORIZATION_BPS: u64 = 10_000;
pub const DEFAULT_MIN_NOTICE_DELIVERY_BPS: u64 = 9_000;
pub const DEFAULT_USER_ESCAPE_AMOUNT_PICONERO: u64 = 12_500_000_000;
pub const DEFAULT_IMPORTED_AT_L2_HEIGHT: u64 = 8_192;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceImportStatus {
    Accepted,
    Quarantined,
    Rejected,
}

impl EvidenceImportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Quarantined => "quarantined",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletTranscriptKind {
    EscapeIntent,
    NullifierDisclosure,
    SpendAuthorization,
    RecoveryAddressBinding,
    FeeLimitAcceptance,
}

impl WalletTranscriptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EscapeIntent => "escape_intent",
            Self::NullifierDisclosure => "nullifier_disclosure",
            Self::SpendAuthorization => "spend_authorization",
            Self::RecoveryAddressBinding => "recovery_address_binding",
            Self::FeeLimitAcceptance => "fee_limit_acceptance",
        }
    }

    pub fn ordinal(self) -> u64 {
        match self {
            Self::EscapeIntent => 0,
            Self::NullifierDisclosure => 1,
            Self::SpendAuthorization => 2,
            Self::RecoveryAddressBinding => 3,
            Self::FeeLimitAcceptance => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFinding {
    MatchesWallet,
    MatchesContract,
    BenignLateFrame,
    DivergentStateRoot,
    MissingFrame,
    UnauthorizedOperatorDependency,
}

impl ReplayFinding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MatchesWallet => "matches_wallet",
            Self::MatchesContract => "matches_contract",
            Self::BenignLateFrame => "benign_late_frame",
            Self::DivergentStateRoot => "divergent_state_root",
            Self::MissingFrame => "missing_frame",
            Self::UnauthorizedOperatorDependency => "unauthorized_operator_dependency",
        }
    }

    pub fn is_divergent(self) -> bool {
        matches!(
            self,
            Self::DivergentStateRoot | Self::MissingFrame | Self::UnauthorizedOperatorDependency
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeDeliveryStatus {
    Delivered,
    DeliveredLate,
    Pending,
    Failed,
}

impl NoticeDeliveryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "delivered",
            Self::DeliveredLate => "delivered_late",
            Self::Pending => "pending",
            Self::Failed => "failed",
        }
    }

    pub fn counts_as_delivered(self) -> bool {
        matches!(self, Self::Delivered | Self::DeliveredLate)
    }

    pub fn is_blocking(self) -> bool {
        matches!(self, Self::Pending | Self::Failed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChallengeObservationKind {
    NoChallengeSeen,
    DuplicateNullifierChallenge,
    AuthorizationChallenge,
    MoneroFinalityChallenge,
    WatcherSilence,
    SequencerCensorship,
    TimeoutReleaseObserved,
}

impl ChallengeObservationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoChallengeSeen => "no_challenge_seen",
            Self::DuplicateNullifierChallenge => "duplicate_nullifier_challenge",
            Self::AuthorizationChallenge => "authorization_challenge",
            Self::MoneroFinalityChallenge => "monero_finality_challenge",
            Self::WatcherSilence => "watcher_silence",
            Self::SequencerCensorship => "sequencer_censorship",
            Self::TimeoutReleaseObserved => "timeout_release_observed",
        }
    }

    pub fn is_adversarial(self) -> bool {
        matches!(
            self,
            Self::DuplicateNullifierChallenge
                | Self::AuthorizationChallenge
                | Self::MoneroFinalityChallenge
                | Self::WatcherSilence
                | Self::SequencerCensorship
        )
    }

    pub fn is_liveness_fault(self) -> bool {
        matches!(self, Self::WatcherSilence | Self::SequencerCensorship)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserEscapeBlocker {
    MissingWalletTranscriptRoot,
    MissingWatchtowerReplayEvidence,
    MissingRecoveryNoticeRoot,
    MissingChallengeObservationRoot,
    WalletAuthorizationBelowThreshold,
    WatchtowerQuorumBelowThreshold,
    RecoveryNoticeDeliveryBelowThreshold,
    ReplayDivergenceDetected,
    EvidenceLagExceeded,
    UnresolvedChallengeObserved,
    OperatorDependencyObserved,
    ImportRejected,
}

impl UserEscapeBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingWalletTranscriptRoot => "missing_wallet_transcript_root",
            Self::MissingWatchtowerReplayEvidence => "missing_watchtower_replay_evidence",
            Self::MissingRecoveryNoticeRoot => "missing_recovery_notice_root",
            Self::MissingChallengeObservationRoot => "missing_challenge_observation_root",
            Self::WalletAuthorizationBelowThreshold => "wallet_authorization_below_threshold",
            Self::WatchtowerQuorumBelowThreshold => "watchtower_quorum_below_threshold",
            Self::RecoveryNoticeDeliveryBelowThreshold => {
                "recovery_notice_delivery_below_threshold"
            }
            Self::ReplayDivergenceDetected => "replay_divergence_detected",
            Self::EvidenceLagExceeded => "evidence_lag_exceeded",
            Self::UnresolvedChallengeObserved => "unresolved_challenge_observed",
            Self::OperatorDependencyObserved => "operator_dependency_observed",
            Self::ImportRejected => "import_rejected",
        }
    }

    pub fn severity(self) -> u64 {
        match self {
            Self::ImportRejected => 100,
            Self::ReplayDivergenceDetected => 95,
            Self::OperatorDependencyObserved => 92,
            Self::UnresolvedChallengeObserved => 90,
            Self::MissingWalletTranscriptRoot
            | Self::MissingWatchtowerReplayEvidence
            | Self::MissingRecoveryNoticeRoot
            | Self::MissingChallengeObservationRoot => 85,
            Self::WalletAuthorizationBelowThreshold
            | Self::WatchtowerQuorumBelowThreshold
            | Self::RecoveryNoticeDeliveryBelowThreshold => 75,
            Self::EvidenceLagExceeded => 70,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    Go,
    NoGo,
}

impl GovernanceDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserEscapeAnswer {
    AcceptedLiveEvidencePreservesEscape,
    FailClosedUntilEvidenceIsCanonical,
}

impl UserEscapeAnswer {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedLiveEvidencePreservesEscape => "accepted_live_evidence_preserves_escape",
            Self::FailClosedUntilEvidenceIsCanonical => "fail_closed_until_evidence_is_canonical",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub import_suite: String,
    pub min_wallet_transcripts: usize,
    pub min_watchtower_replays: usize,
    pub min_recovery_notices: usize,
    pub min_challenge_observations: usize,
    pub max_import_lag_l2_blocks: u64,
    pub max_notice_lag_l2_blocks: u64,
    pub max_replay_divergences: u64,
    pub min_watchtower_quorum_bps: u64,
    pub min_wallet_authorization_bps: u64,
    pub min_notice_delivery_bps: u64,
    pub fail_closed_on_any_blocker: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            import_suite: IMPORT_SUITE.to_string(),
            min_wallet_transcripts: DEFAULT_MIN_WALLET_TRANSCRIPTS,
            min_watchtower_replays: DEFAULT_MIN_WATCHTOWER_REPLAYS,
            min_recovery_notices: DEFAULT_MIN_RECOVERY_NOTICES,
            min_challenge_observations: DEFAULT_MIN_CHALLENGE_OBSERVATIONS,
            max_import_lag_l2_blocks: DEFAULT_MAX_IMPORT_LAG_L2_BLOCKS,
            max_notice_lag_l2_blocks: DEFAULT_MAX_NOTICE_LAG_L2_BLOCKS,
            max_replay_divergences: DEFAULT_MAX_REPLAY_DIVERGENCES,
            min_watchtower_quorum_bps: DEFAULT_MIN_WATCHTOWER_QUORUM_BPS,
            min_wallet_authorization_bps: DEFAULT_MIN_WALLET_AUTHORIZATION_BPS,
            min_notice_delivery_bps: DEFAULT_MIN_NOTICE_DELIVERY_BPS,
            fail_closed_on_any_blocker: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "import_suite": self.import_suite,
            "min_wallet_transcripts": self.min_wallet_transcripts,
            "min_watchtower_replays": self.min_watchtower_replays,
            "min_recovery_notices": self.min_recovery_notices,
            "min_challenge_observations": self.min_challenge_observations,
            "max_import_lag_l2_blocks": self.max_import_lag_l2_blocks,
            "max_notice_lag_l2_blocks": self.max_notice_lag_l2_blocks,
            "max_replay_divergences": self.max_replay_divergences,
            "min_watchtower_quorum_bps": self.min_watchtower_quorum_bps,
            "min_wallet_authorization_bps": self.min_wallet_authorization_bps,
            "min_notice_delivery_bps": self.min_notice_delivery_bps,
            "fail_closed_on_any_blocker": self.fail_closed_on_any_blocker,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletTranscript {
    pub transcript_id: String,
    pub wallet_id: String,
    pub kind: WalletTranscriptKind,
    pub l2_height: u64,
    pub monero_height: u64,
    pub authorization_bps: u64,
    pub transcript_root: String,
    pub pq_signature_root: String,
    pub note_commitment_root: String,
    pub status: EvidenceImportStatus,
}

impl WalletTranscript {
    pub fn new(
        wallet_id: &str,
        kind: WalletTranscriptKind,
        l2_height: u64,
        monero_height: u64,
        authorization_bps: u64,
    ) -> Self {
        let seed = seed_root("wallet-transcript", &[wallet_id, kind.as_str()]);
        let transcript_root = labeled_root("transcript", &seed);
        let pq_signature_root = labeled_root("pq-signature", &seed);
        let note_commitment_root = labeled_root("note-commitment", &seed);
        let transcript_id = domain_hash(
            "monero-l2-pq/user-escape/wallet-transcript/id",
            &[
                HashPart::Str(wallet_id),
                HashPart::Str(kind.as_str()),
                HashPart::U64(l2_height),
                HashPart::Str(&transcript_root),
            ],
            32,
        );
        Self {
            transcript_id,
            wallet_id: wallet_id.to_string(),
            kind,
            l2_height,
            monero_height,
            authorization_bps,
            transcript_root,
            pq_signature_root,
            note_commitment_root,
            status: EvidenceImportStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "transcript_id": self.transcript_id,
            "wallet_id": self.wallet_id,
            "kind": self.kind.as_str(),
            "kind_ordinal": self.kind.ordinal(),
            "l2_height": self.l2_height,
            "monero_height": self.monero_height,
            "authorization_bps": self.authorization_bps,
            "transcript_root": self.transcript_root,
            "pq_signature_root": self.pq_signature_root,
            "note_commitment_root": self.note_commitment_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wallet_transcript", &self.public_record())
    }

    pub fn is_usable(&self) -> bool {
        self.status.is_accepted() && self.authorization_bps > 0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WatchtowerReplayEvidence {
    pub replay_id: String,
    pub watchtower_id: String,
    pub replayed_claim_id: String,
    pub observed_l2_height: u64,
    pub canonical_state_root: String,
    pub replay_state_root: String,
    pub wallet_transcript_root: String,
    pub finding: ReplayFinding,
    pub watcher_weight_bps: u64,
    pub status: EvidenceImportStatus,
}

impl WatchtowerReplayEvidence {
    pub fn new(
        watchtower_id: &str,
        replayed_claim_id: &str,
        observed_l2_height: u64,
        wallet_transcript_root: &str,
        finding: ReplayFinding,
        watcher_weight_bps: u64,
    ) -> Self {
        let seed = seed_root(
            "watchtower-replay",
            &[watchtower_id, replayed_claim_id, finding.as_str()],
        );
        let canonical_state_root = labeled_root("canonical-state", &seed);
        let replay_state_root = if finding.is_divergent() {
            labeled_root("divergent-replay-state", &seed)
        } else {
            canonical_state_root.clone()
        };
        let replay_id = domain_hash(
            "monero-l2-pq/user-escape/watchtower-replay/id",
            &[
                HashPart::Str(watchtower_id),
                HashPart::Str(replayed_claim_id),
                HashPart::U64(observed_l2_height),
                HashPart::Str(&canonical_state_root),
                HashPart::Str(&replay_state_root),
            ],
            32,
        );
        Self {
            replay_id,
            watchtower_id: watchtower_id.to_string(),
            replayed_claim_id: replayed_claim_id.to_string(),
            observed_l2_height,
            canonical_state_root,
            replay_state_root,
            wallet_transcript_root: wallet_transcript_root.to_string(),
            finding,
            watcher_weight_bps,
            status: EvidenceImportStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replay_id": self.replay_id,
            "watchtower_id": self.watchtower_id,
            "replayed_claim_id": self.replayed_claim_id,
            "observed_l2_height": self.observed_l2_height,
            "canonical_state_root": self.canonical_state_root,
            "replay_state_root": self.replay_state_root,
            "wallet_transcript_root": self.wallet_transcript_root,
            "finding": self.finding.as_str(),
            "watcher_weight_bps": self.watcher_weight_bps,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("watchtower_replay_evidence", &self.public_record())
    }

    pub fn is_usable(&self) -> bool {
        self.status.is_accepted()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecoveryNotice {
    pub notice_id: String,
    pub wallet_id: String,
    pub channel: String,
    pub sent_l2_height: u64,
    pub acknowledged_l2_height: u64,
    pub delivery_status: NoticeDeliveryStatus,
    pub recovery_address_root: String,
    pub notice_payload_root: String,
    pub status: EvidenceImportStatus,
}

impl RecoveryNotice {
    pub fn new(
        wallet_id: &str,
        channel: &str,
        sent_l2_height: u64,
        acknowledged_l2_height: u64,
        delivery_status: NoticeDeliveryStatus,
    ) -> Self {
        let seed = seed_root("recovery-notice", &[wallet_id, channel]);
        let recovery_address_root = labeled_root("recovery-address", &seed);
        let notice_payload_root = labeled_root("notice-payload", &seed);
        let notice_id = domain_hash(
            "monero-l2-pq/user-escape/recovery-notice/id",
            &[
                HashPart::Str(wallet_id),
                HashPart::Str(channel),
                HashPart::U64(sent_l2_height),
                HashPart::U64(acknowledged_l2_height),
                HashPart::Str(&notice_payload_root),
            ],
            32,
        );
        Self {
            notice_id,
            wallet_id: wallet_id.to_string(),
            channel: channel.to_string(),
            sent_l2_height,
            acknowledged_l2_height,
            delivery_status,
            recovery_address_root,
            notice_payload_root,
            status: EvidenceImportStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "notice_id": self.notice_id,
            "wallet_id": self.wallet_id,
            "channel": self.channel,
            "sent_l2_height": self.sent_l2_height,
            "acknowledged_l2_height": self.acknowledged_l2_height,
            "delivery_status": self.delivery_status.as_str(),
            "recovery_address_root": self.recovery_address_root,
            "notice_payload_root": self.notice_payload_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("recovery_notice", &self.public_record())
    }

    pub fn delivery_lag(&self) -> u64 {
        self.acknowledged_l2_height
            .saturating_sub(self.sent_l2_height)
    }

    pub fn is_usable(&self) -> bool {
        self.status.is_accepted() && self.delivery_status.counts_as_delivered()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeObservation {
    pub observation_id: String,
    pub observer_id: String,
    pub observed_claim_id: String,
    pub observed_l2_height: u64,
    pub kind: ChallengeObservationKind,
    pub challenge_root: String,
    pub response_root: String,
    pub resolved: bool,
    pub status: EvidenceImportStatus,
}

impl ChallengeObservation {
    pub fn new(
        observer_id: &str,
        observed_claim_id: &str,
        observed_l2_height: u64,
        kind: ChallengeObservationKind,
        resolved: bool,
    ) -> Self {
        let seed = seed_root(
            "challenge-observation",
            &[observer_id, observed_claim_id, kind.as_str()],
        );
        let challenge_root = labeled_root("challenge", &seed);
        let response_root = if resolved {
            labeled_root("challenge-response", &seed)
        } else {
            labeled_root("unresolved-response-placeholder", &seed)
        };
        let observation_id = domain_hash(
            "monero-l2-pq/user-escape/challenge-observation/id",
            &[
                HashPart::Str(observer_id),
                HashPart::Str(observed_claim_id),
                HashPart::U64(observed_l2_height),
                HashPart::Str(kind.as_str()),
                HashPart::Str(&challenge_root),
            ],
            32,
        );
        Self {
            observation_id,
            observer_id: observer_id.to_string(),
            observed_claim_id: observed_claim_id.to_string(),
            observed_l2_height,
            kind,
            challenge_root,
            response_root,
            resolved,
            status: EvidenceImportStatus::Accepted,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_id": self.observation_id,
            "observer_id": self.observer_id,
            "observed_claim_id": self.observed_claim_id,
            "observed_l2_height": self.observed_l2_height,
            "kind": self.kind.as_str(),
            "challenge_root": self.challenge_root,
            "response_root": self.response_root,
            "resolved": self.resolved,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("challenge_observation", &self.public_record())
    }

    pub fn is_usable(&self) -> bool {
        self.status.is_accepted()
    }

    pub fn blocks_escape(&self) -> bool {
        self.kind.is_adversarial() && !self.resolved
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceImportBatch {
    pub batch_id: String,
    pub imported_at_l2_height: u64,
    pub source_feed_root: String,
    pub wallet_transcript_root: String,
    pub watchtower_replay_root: String,
    pub recovery_notice_root: String,
    pub challenge_observation_root: String,
    pub batch_root: String,
}

impl EvidenceImportBatch {
    pub fn from_parts(
        imported_at_l2_height: u64,
        source_feed_root: &str,
        wallet_transcripts: &[WalletTranscript],
        watchtower_replays: &[WatchtowerReplayEvidence],
        recovery_notices: &[RecoveryNotice],
        challenge_observations: &[ChallengeObservation],
    ) -> Self {
        let wallet_transcript_root = wallet_transcripts_root(wallet_transcripts);
        let watchtower_replay_root = watchtower_replays_root(watchtower_replays);
        let recovery_notice_root = recovery_notices_root(recovery_notices);
        let challenge_observation_root = challenge_observations_root(challenge_observations);
        let batch_record = json!({
            "imported_at_l2_height": imported_at_l2_height,
            "source_feed_root": source_feed_root,
            "wallet_transcript_root": wallet_transcript_root,
            "watchtower_replay_root": watchtower_replay_root,
            "recovery_notice_root": recovery_notice_root,
            "challenge_observation_root": challenge_observation_root,
        });
        let batch_root = record_root("evidence_import_batch", &batch_record);
        let batch_id = domain_hash(
            "monero-l2-pq/user-escape/evidence-import-batch/id",
            &[
                HashPart::U64(imported_at_l2_height),
                HashPart::Str(source_feed_root),
                HashPart::Str(&batch_root),
            ],
            32,
        );
        Self {
            batch_id,
            imported_at_l2_height,
            source_feed_root: source_feed_root.to_string(),
            wallet_transcript_root,
            watchtower_replay_root,
            recovery_notice_root,
            challenge_observation_root,
            batch_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "batch_id": self.batch_id,
            "imported_at_l2_height": self.imported_at_l2_height,
            "source_feed_root": self.source_feed_root,
            "wallet_transcript_root": self.wallet_transcript_root,
            "watchtower_replay_root": self.watchtower_replay_root,
            "recovery_notice_root": self.recovery_notice_root,
            "challenge_observation_root": self.challenge_observation_root,
            "batch_root": self.batch_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.batch_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceCoverage {
    pub wallet_transcript_count: usize,
    pub accepted_wallet_transcript_count: usize,
    pub watchtower_replay_count: usize,
    pub accepted_watchtower_replay_count: usize,
    pub recovery_notice_count: usize,
    pub accepted_recovery_notice_count: usize,
    pub challenge_observation_count: usize,
    pub accepted_challenge_observation_count: usize,
    pub wallet_authorization_bps: u64,
    pub watchtower_quorum_bps: u64,
    pub recovery_notice_delivery_bps: u64,
    pub replay_divergence_count: u64,
    pub unresolved_challenge_count: u64,
    pub liveness_fault_count: u64,
    pub operator_dependency_count: u64,
    pub max_import_lag_l2_blocks: u64,
    pub max_notice_lag_l2_blocks: u64,
}

impl EvidenceCoverage {
    pub fn from_state(config: &Config, state: &State) -> Self {
        let accepted_wallets = state
            .wallet_transcripts
            .iter()
            .filter(|item| item.is_usable())
            .collect::<Vec<_>>();
        let accepted_replays = state
            .watchtower_replays
            .iter()
            .filter(|item| item.is_usable())
            .collect::<Vec<_>>();
        let accepted_notices = state
            .recovery_notices
            .iter()
            .filter(|item| item.is_usable())
            .collect::<Vec<_>>();
        let accepted_observations = state
            .challenge_observations
            .iter()
            .filter(|item| item.is_usable())
            .collect::<Vec<_>>();
        let wallet_authorization_bps = min_bps_or_zero(
            accepted_wallets
                .iter()
                .map(|item| item.authorization_bps)
                .collect::<Vec<_>>(),
        );
        let watchtower_quorum_bps = accepted_replays
            .iter()
            .map(|item| item.watcher_weight_bps)
            .sum::<u64>()
            .min(10_000);
        let delivered_notices = state
            .recovery_notices
            .iter()
            .filter(|item| item.delivery_status.counts_as_delivered())
            .count();
        let recovery_notice_delivery_bps = ratio_bps(
            delivered_notices as u64,
            state.recovery_notices.len() as u64,
        );
        let replay_divergence_count = accepted_replays
            .iter()
            .filter(|item| item.finding.is_divergent())
            .count() as u64;
        let unresolved_challenge_count = accepted_observations
            .iter()
            .filter(|item| item.blocks_escape())
            .count() as u64;
        let liveness_fault_count = accepted_observations
            .iter()
            .filter(|item| item.kind.is_liveness_fault() && !item.resolved)
            .count() as u64;
        let operator_dependency_count = accepted_replays
            .iter()
            .filter(|item| matches!(item.finding, ReplayFinding::UnauthorizedOperatorDependency))
            .count() as u64;
        let max_import_lag_l2_blocks = state
            .oldest_evidence_l2_height()
            .map(|height| state.imported_at_l2_height.saturating_sub(height))
            .unwrap_or(config.max_import_lag_l2_blocks.saturating_add(1));
        let max_notice_lag_l2_blocks = state
            .recovery_notices
            .iter()
            .map(|item| item.delivery_lag())
            .max()
            .unwrap_or(config.max_notice_lag_l2_blocks.saturating_add(1));
        Self {
            wallet_transcript_count: state.wallet_transcripts.len(),
            accepted_wallet_transcript_count: accepted_wallets.len(),
            watchtower_replay_count: state.watchtower_replays.len(),
            accepted_watchtower_replay_count: accepted_replays.len(),
            recovery_notice_count: state.recovery_notices.len(),
            accepted_recovery_notice_count: accepted_notices.len(),
            challenge_observation_count: state.challenge_observations.len(),
            accepted_challenge_observation_count: accepted_observations.len(),
            wallet_authorization_bps,
            watchtower_quorum_bps,
            recovery_notice_delivery_bps,
            replay_divergence_count,
            unresolved_challenge_count,
            liveness_fault_count,
            operator_dependency_count,
            max_import_lag_l2_blocks,
            max_notice_lag_l2_blocks,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wallet_transcript_count": self.wallet_transcript_count,
            "accepted_wallet_transcript_count": self.accepted_wallet_transcript_count,
            "watchtower_replay_count": self.watchtower_replay_count,
            "accepted_watchtower_replay_count": self.accepted_watchtower_replay_count,
            "recovery_notice_count": self.recovery_notice_count,
            "accepted_recovery_notice_count": self.accepted_recovery_notice_count,
            "challenge_observation_count": self.challenge_observation_count,
            "accepted_challenge_observation_count": self.accepted_challenge_observation_count,
            "wallet_authorization_bps": self.wallet_authorization_bps,
            "watchtower_quorum_bps": self.watchtower_quorum_bps,
            "recovery_notice_delivery_bps": self.recovery_notice_delivery_bps,
            "replay_divergence_count": self.replay_divergence_count,
            "unresolved_challenge_count": self.unresolved_challenge_count,
            "liveness_fault_count": self.liveness_fault_count,
            "operator_dependency_count": self.operator_dependency_count,
            "max_import_lag_l2_blocks": self.max_import_lag_l2_blocks,
            "max_notice_lag_l2_blocks": self.max_notice_lag_l2_blocks,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("evidence_coverage", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GovernanceVerdict {
    pub decision: GovernanceDecision,
    pub user_escape_answer: UserEscapeAnswer,
    pub blockers: Vec<UserEscapeBlocker>,
    pub coverage_root: String,
    pub imported_batch_root: String,
    pub final_go_no_go_root: String,
}

impl GovernanceVerdict {
    pub fn from_state(config: &Config, state: &State) -> Self {
        let coverage = EvidenceCoverage::from_state(config, state);
        let mut blockers = Vec::new();
        if coverage.accepted_wallet_transcript_count < config.min_wallet_transcripts {
            blockers.push(UserEscapeBlocker::MissingWalletTranscriptRoot);
        }
        if coverage.accepted_watchtower_replay_count < config.min_watchtower_replays {
            blockers.push(UserEscapeBlocker::MissingWatchtowerReplayEvidence);
        }
        if coverage.accepted_recovery_notice_count < config.min_recovery_notices {
            blockers.push(UserEscapeBlocker::MissingRecoveryNoticeRoot);
        }
        if coverage.accepted_challenge_observation_count < config.min_challenge_observations {
            blockers.push(UserEscapeBlocker::MissingChallengeObservationRoot);
        }
        if coverage.wallet_authorization_bps < config.min_wallet_authorization_bps {
            blockers.push(UserEscapeBlocker::WalletAuthorizationBelowThreshold);
        }
        if coverage.watchtower_quorum_bps < config.min_watchtower_quorum_bps {
            blockers.push(UserEscapeBlocker::WatchtowerQuorumBelowThreshold);
        }
        if coverage.recovery_notice_delivery_bps < config.min_notice_delivery_bps {
            blockers.push(UserEscapeBlocker::RecoveryNoticeDeliveryBelowThreshold);
        }
        if coverage.replay_divergence_count > config.max_replay_divergences {
            blockers.push(UserEscapeBlocker::ReplayDivergenceDetected);
        }
        if coverage.max_import_lag_l2_blocks > config.max_import_lag_l2_blocks
            || coverage.max_notice_lag_l2_blocks > config.max_notice_lag_l2_blocks
        {
            blockers.push(UserEscapeBlocker::EvidenceLagExceeded);
        }
        if coverage.unresolved_challenge_count > 0 || coverage.liveness_fault_count > 0 {
            blockers.push(UserEscapeBlocker::UnresolvedChallengeObserved);
        }
        if coverage.operator_dependency_count > 0 {
            blockers.push(UserEscapeBlocker::OperatorDependencyObserved);
        }
        if state.import_status != EvidenceImportStatus::Accepted {
            blockers.push(UserEscapeBlocker::ImportRejected);
        }
        blockers.sort_by_key(|blocker| blocker.severity());
        blockers.reverse();
        blockers.dedup();
        let decision = if blockers.is_empty() || !config.fail_closed_on_any_blocker {
            GovernanceDecision::Go
        } else {
            GovernanceDecision::NoGo
        };
        let user_escape_answer = if decision == GovernanceDecision::Go {
            UserEscapeAnswer::AcceptedLiveEvidencePreservesEscape
        } else {
            UserEscapeAnswer::FailClosedUntilEvidenceIsCanonical
        };
        let coverage_root = coverage.state_root();
        let imported_batch_root = state.import_batch.state_root();
        let verdict_payload = json!({
            "decision": decision.as_str(),
            "user_escape_answer": user_escape_answer.as_str(),
            "blockers": blockers.iter().map(|item| item.as_str()).collect::<Vec<_>>(),
            "coverage_root": coverage_root,
            "imported_batch_root": imported_batch_root,
        });
        let final_go_no_go_root = record_root("final_go_no_go_governance", &verdict_payload);
        Self {
            decision,
            user_escape_answer,
            blockers,
            coverage_root,
            imported_batch_root,
            final_go_no_go_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "decision": self.decision.as_str(),
            "user_escape_answer": self.user_escape_answer.as_str(),
            "blockers": self.blockers.iter().map(|item| item.as_str()).collect::<Vec<_>>(),
            "coverage_root": self.coverage_root,
            "imported_batch_root": self.imported_batch_root,
            "final_go_no_go_root": self.final_go_no_go_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.final_go_no_go_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub import_status: EvidenceImportStatus,
    pub imported_at_l2_height: u64,
    pub force_exit_claim_id: String,
    pub user_escape_amount_piconero: u64,
    pub wallet_transcripts: Vec<WalletTranscript>,
    pub watchtower_replays: Vec<WatchtowerReplayEvidence>,
    pub recovery_notices: Vec<RecoveryNotice>,
    pub challenge_observations: Vec<ChallengeObservation>,
    pub import_batch: EvidenceImportBatch,
    pub coverage: EvidenceCoverage,
    pub governance_verdict: GovernanceVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let force_exit_claim_id = labeled_root("force-exit-claim", "devnet-user-escape");
        let imported_at_l2_height = DEFAULT_IMPORTED_AT_L2_HEIGHT;
        let wallet_transcripts = vec![
            WalletTranscript::new(
                "wallet-alpha",
                WalletTranscriptKind::EscapeIntent,
                imported_at_l2_height.saturating_sub(2),
                3_210_011,
                10_000,
            ),
            WalletTranscript::new(
                "wallet-alpha",
                WalletTranscriptKind::NullifierDisclosure,
                imported_at_l2_height.saturating_sub(2),
                3_210_011,
                10_000,
            ),
            WalletTranscript::new(
                "wallet-alpha",
                WalletTranscriptKind::SpendAuthorization,
                imported_at_l2_height.saturating_sub(1),
                3_210_012,
                10_000,
            ),
        ];
        let wallet_root = wallet_transcripts_root(&wallet_transcripts);
        let watchtower_replays = vec![
            WatchtowerReplayEvidence::new(
                "watchtower-north",
                &force_exit_claim_id,
                imported_at_l2_height.saturating_sub(1),
                &wallet_root,
                ReplayFinding::MatchesWallet,
                3_400,
            ),
            WatchtowerReplayEvidence::new(
                "watchtower-east",
                &force_exit_claim_id,
                imported_at_l2_height.saturating_sub(1),
                &wallet_root,
                ReplayFinding::MatchesContract,
                3_300,
            ),
            WatchtowerReplayEvidence::new(
                "watchtower-west",
                &force_exit_claim_id,
                imported_at_l2_height,
                &wallet_root,
                ReplayFinding::BenignLateFrame,
                3_300,
            ),
        ];
        let recovery_notices = vec![
            RecoveryNotice::new(
                "wallet-alpha",
                "pq-device-channel",
                imported_at_l2_height.saturating_sub(4),
                imported_at_l2_height.saturating_sub(3),
                NoticeDeliveryStatus::Delivered,
            ),
            RecoveryNotice::new(
                "wallet-alpha",
                "recovery-email-commitment",
                imported_at_l2_height.saturating_sub(4),
                imported_at_l2_height.saturating_sub(2),
                NoticeDeliveryStatus::Delivered,
            ),
        ];
        let challenge_observations = vec![
            ChallengeObservation::new(
                "watchtower-north",
                &force_exit_claim_id,
                imported_at_l2_height.saturating_sub(1),
                ChallengeObservationKind::NoChallengeSeen,
                true,
            ),
            ChallengeObservation::new(
                "watchtower-east",
                &force_exit_claim_id,
                imported_at_l2_height.saturating_sub(1),
                ChallengeObservationKind::NoChallengeSeen,
                true,
            ),
            ChallengeObservation::new(
                "watchtower-west",
                &force_exit_claim_id,
                imported_at_l2_height,
                ChallengeObservationKind::TimeoutReleaseObserved,
                true,
            ),
            ChallengeObservation::new(
                "wallet-alpha",
                &force_exit_claim_id,
                imported_at_l2_height,
                ChallengeObservationKind::NoChallengeSeen,
                true,
            ),
        ];
        let source_feed_root = labeled_root("accepted-live-evidence-feed", &force_exit_claim_id);
        Self::import_live_evidence(
            config,
            imported_at_l2_height,
            &force_exit_claim_id,
            DEFAULT_USER_ESCAPE_AMOUNT_PICONERO,
            &source_feed_root,
            wallet_transcripts,
            watchtower_replays,
            recovery_notices,
            challenge_observations,
        )
    }

    pub fn import_live_evidence(
        config: Config,
        imported_at_l2_height: u64,
        force_exit_claim_id: &str,
        user_escape_amount_piconero: u64,
        source_feed_root: &str,
        wallet_transcripts: Vec<WalletTranscript>,
        watchtower_replays: Vec<WatchtowerReplayEvidence>,
        recovery_notices: Vec<RecoveryNotice>,
        challenge_observations: Vec<ChallengeObservation>,
    ) -> Self {
        let import_batch = EvidenceImportBatch::from_parts(
            imported_at_l2_height,
            source_feed_root,
            &wallet_transcripts,
            &watchtower_replays,
            &recovery_notices,
            &challenge_observations,
        );
        let placeholder_coverage = EvidenceCoverage {
            wallet_transcript_count: 0,
            accepted_wallet_transcript_count: 0,
            watchtower_replay_count: 0,
            accepted_watchtower_replay_count: 0,
            recovery_notice_count: 0,
            accepted_recovery_notice_count: 0,
            challenge_observation_count: 0,
            accepted_challenge_observation_count: 0,
            wallet_authorization_bps: 0,
            watchtower_quorum_bps: 0,
            recovery_notice_delivery_bps: 0,
            replay_divergence_count: 0,
            unresolved_challenge_count: 0,
            liveness_fault_count: 0,
            operator_dependency_count: 0,
            max_import_lag_l2_blocks: 0,
            max_notice_lag_l2_blocks: 0,
        };
        let placeholder_verdict = GovernanceVerdict {
            decision: GovernanceDecision::NoGo,
            user_escape_answer: UserEscapeAnswer::FailClosedUntilEvidenceIsCanonical,
            blockers: vec![UserEscapeBlocker::MissingWalletTranscriptRoot],
            coverage_root: placeholder_coverage.state_root(),
            imported_batch_root: import_batch.state_root(),
            final_go_no_go_root: labeled_root("placeholder-governance", &import_batch.batch_root),
        };
        let mut state = Self {
            config,
            import_status: EvidenceImportStatus::Accepted,
            imported_at_l2_height,
            force_exit_claim_id: force_exit_claim_id.to_string(),
            user_escape_amount_piconero,
            wallet_transcripts,
            watchtower_replays,
            recovery_notices,
            challenge_observations,
            import_batch,
            coverage: placeholder_coverage,
            governance_verdict: placeholder_verdict,
        };
        state.coverage = EvidenceCoverage::from_state(&state.config, &state);
        state.governance_verdict = GovernanceVerdict::from_state(&state.config, &state);
        state.import_status = if state.governance_verdict.decision == GovernanceDecision::Go {
            EvidenceImportStatus::Accepted
        } else {
            EvidenceImportStatus::Quarantined
        };
        state.governance_verdict = GovernanceVerdict::from_state(&state.config, &state);
        state
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "import_status": self.import_status.as_str(),
            "imported_at_l2_height": self.imported_at_l2_height,
            "force_exit_claim_id": self.force_exit_claim_id,
            "user_escape_amount_piconero": self.user_escape_amount_piconero,
            "wallet_transcript_root": wallet_transcripts_root(&self.wallet_transcripts),
            "watchtower_replay_root": watchtower_replays_root(&self.watchtower_replays),
            "recovery_notice_root": recovery_notices_root(&self.recovery_notices),
            "challenge_observation_root": challenge_observations_root(&self.challenge_observations),
            "import_batch": self.import_batch.public_record(),
            "coverage": self.coverage.public_record(),
            "governance_verdict": self.governance_verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        let leaves = vec![
            json!(self.config.state_root()),
            json!(self.import_batch.state_root()),
            json!(self.coverage.state_root()),
            json!(self.governance_verdict.state_root()),
            json!(wallet_transcripts_root(&self.wallet_transcripts)),
            json!(watchtower_replays_root(&self.watchtower_replays)),
            json!(recovery_notices_root(&self.recovery_notices)),
            json!(challenge_observations_root(&self.challenge_observations)),
        ];
        merkle_root(
            "monero-l2-pq/user-escape/accepted-live-evidence-import/state",
            &leaves,
        )
    }

    pub fn final_go_no_go_governance_record(&self) -> Value {
        json!({
            "protocol_version": self.config.protocol_version,
            "force_exit_claim_id": self.force_exit_claim_id,
            "decision": self.governance_verdict.decision.as_str(),
            "user_escape_answer": self.governance_verdict.user_escape_answer.as_str(),
            "fail_closed_blockers": self
                .governance_verdict
                .blockers
                .iter()
                .map(|item| item.as_str())
                .collect::<Vec<_>>(),
            "state_root": self.state_root(),
            "final_go_no_go_root": self.governance_verdict.final_go_no_go_root,
        })
    }

    pub fn require_go(&self) -> Result<()> {
        if self.governance_verdict.decision == GovernanceDecision::Go {
            Ok(())
        } else {
            Err(format!(
                "user escape live evidence import is fail-closed: {}",
                self.governance_verdict
                    .blockers
                    .iter()
                    .map(|item| item.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        }
    }

    pub fn fail_closed_blockers(&self) -> Vec<UserEscapeBlocker> {
        self.governance_verdict.blockers.clone()
    }

    fn oldest_evidence_l2_height(&self) -> Option<u64> {
        let wallet_heights = self.wallet_transcripts.iter().map(|item| item.l2_height);
        let replay_heights = self
            .watchtower_replays
            .iter()
            .map(|item| item.observed_l2_height);
        let notice_heights = self.recovery_notices.iter().map(|item| item.sent_l2_height);
        let observation_heights = self
            .challenge_observations
            .iter()
            .map(|item| item.observed_l2_height);
        wallet_heights
            .chain(replay_heights)
            .chain(notice_heights)
            .chain(observation_heights)
            .min()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    State::devnet().public_record()
}

pub fn state_root() -> String {
    State::devnet().state_root()
}

pub fn wallet_transcripts_root(items: &[WalletTranscript]) -> String {
    let leaves = items
        .iter()
        .map(|item| item.public_record())
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/wallet-transcripts",
        &leaves,
    )
}

pub fn watchtower_replays_root(items: &[WatchtowerReplayEvidence]) -> String {
    let leaves = items
        .iter()
        .map(|item| item.public_record())
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/watchtower-replays",
        &leaves,
    )
}

pub fn recovery_notices_root(items: &[RecoveryNotice]) -> String {
    let leaves = items
        .iter()
        .map(|item| item.public_record())
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/recovery-notices",
        &leaves,
    )
}

pub fn challenge_observations_root(items: &[ChallengeObservation]) -> String {
    let leaves = items
        .iter()
        .map(|item| item.public_record())
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/challenge-observations",
        &leaves,
    )
}

fn record_root(label: &str, value: &Value) -> String {
    domain_hash(
        &format!("monero-l2-pq/user-escape/accepted-live-evidence-import/{label}"),
        &[HashPart::Json(value)],
        32,
    )
}

fn seed_root(label: &str, parts: &[&str]) -> String {
    let leaves = parts.iter().map(|part| json!(part)).collect::<Vec<_>>();
    let root = merkle_root(
        &format!("monero-l2-pq/user-escape/accepted-live-evidence-import/seed/{label}"),
        &leaves,
    );
    domain_hash(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/seed-root",
        &[HashPart::Str(label), HashPart::Str(&root)],
        32,
    )
}

fn labeled_root(label: &str, seed: &str) -> String {
    domain_hash(
        "monero-l2-pq/user-escape/accepted-live-evidence-import/labeled-root",
        &[HashPart::Str(label), HashPart::Str(seed)],
        32,
    )
}

fn ratio_bps(numerator: u64, denominator: u64) -> u64 {
    if denominator == 0 {
        0
    } else {
        numerator.saturating_mul(10_000) / denominator
    }
}

fn min_bps_or_zero(values: Vec<u64>) -> u64 {
    values.into_iter().min().unwrap_or(0)
}
