use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageBridgeCustodyAcceptedLiveEvidenceImportRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-bridge-custody-accepted-live-evidence-import-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_ACCEPTED_LIVE_EVIDENCE_IMPORT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const CUSTODY_ACCEPTED_LIVE_EVIDENCE_IMPORT_SUITE: &str =
    "monero-l2-pq-force-exit-package-bridge-custody-accepted-live-evidence-import-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-canonical-user-escape-force-exit-package-devnet-v1";
pub const DEFAULT_IMPORT_BATCH_ID: &str =
    "monero-l2-pq-bridge-custody-accepted-live-evidence-import-devnet-v1";
pub const DEFAULT_GOVERNANCE_EPOCH_ID: &str =
    "monero-l2-pq-bridge-final-go-no-go-governance-devnet-epoch-v1";
pub const DEFAULT_MIN_CUSTODY_SIGNER_QUORUM_BPS: u64 = 7_000;
pub const DEFAULT_MIN_OBSERVER_QUORUM_BPS: u64 = 6_700;
pub const DEFAULT_MIN_RESERVE_HANDOFF_QUORUM_BPS: u64 = 6_700;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_MIN_MONERO_CONFIRMATIONS: u64 = 18;
pub const DEFAULT_MAX_IMPORT_LAG_BLOCKS: u64 = 24;
pub const DEFAULT_MAX_RELEASE_OBSERVATION_DRIFT: u64 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodySignerRole {
    PrimaryCustodian,
    ReserveCustodian,
    Watchtower,
    GovernanceObserver,
    EmergencyRecovery,
}

impl CustodySignerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryCustodian => "primary_custodian",
            Self::ReserveCustodian => "reserve_custodian",
            Self::Watchtower => "watchtower",
            Self::GovernanceObserver => "governance_observer",
            Self::EmergencyRecovery => "emergency_recovery",
        }
    }

    pub fn default_weight_bps(self) -> u64 {
        match self {
            Self::PrimaryCustodian => 2_200,
            Self::ReserveCustodian => 1_700,
            Self::Watchtower => 1_600,
            Self::GovernanceObserver => 1_300,
            Self::EmergencyRecovery => 1_200,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Accepted,
    Observed,
    Challenged,
    Expired,
    Rejected,
    Missing,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Observed => "observed",
            Self::Challenged => "challenged",
            Self::Expired => "expired",
            Self::Rejected => "rejected",
            Self::Missing => "missing",
        }
    }

    pub fn passes_governance_import(self) -> bool {
        matches!(self, Self::Accepted | Self::Observed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MoneroReleaseStatus {
    NotObserved,
    MempoolOnly,
    ObservedConfirmed,
    ReorgRisk,
    ConflictingSpend,
}

impl MoneroReleaseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotObserved => "not_observed",
            Self::MempoolOnly => "mempool_only",
            Self::ObservedConfirmed => "observed_confirmed",
            Self::ReorgRisk => "reorg_risk",
            Self::ConflictingSpend => "conflicting_spend",
        }
    }

    pub fn confirms_release(self) -> bool {
        matches!(self, Self::ObservedConfirmed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReserveHandoffStatus {
    NotRequired,
    Pending,
    Accepted,
    Challenged,
    Rejected,
}

impl ReserveHandoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Challenged => "challenged",
            Self::Rejected => "rejected",
        }
    }

    pub fn passes(self) -> bool {
        matches!(self, Self::NotRequired | Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChallengeWindowStatus {
    Open,
    Matured,
    Challenged,
    TimelockMismatch,
}

impl ChallengeWindowStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Matured => "matured",
            Self::Challenged => "challenged",
            Self::TimelockMismatch => "timelock_mismatch",
        }
    }

    pub fn permits_governance(self) -> bool {
        matches!(self, Self::Matured)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyBlockerKind {
    SignerQuorumMissing,
    DuplicateSigner,
    SignerWeightTooLow,
    MoneroReleaseNotConfirmed,
    ReleaseConflict,
    ReserveHandoffMissing,
    ReserveHandoffChallenged,
    ChallengeWindowOpen,
    ChallengePresent,
    ImportLagExceeded,
    InvariantRootMismatch,
    GovernanceHold,
}

impl CustodyBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignerQuorumMissing => "signer_quorum_missing",
            Self::DuplicateSigner => "duplicate_signer",
            Self::SignerWeightTooLow => "signer_weight_too_low",
            Self::MoneroReleaseNotConfirmed => "monero_release_not_confirmed",
            Self::ReleaseConflict => "release_conflict",
            Self::ReserveHandoffMissing => "reserve_handoff_missing",
            Self::ReserveHandoffChallenged => "reserve_handoff_challenged",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::ChallengePresent => "challenge_present",
            Self::ImportLagExceeded => "import_lag_exceeded",
            Self::InvariantRootMismatch => "invariant_root_mismatch",
            Self::GovernanceHold => "governance_hold",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    Go,
    NoGo,
    Hold,
}

impl GovernanceDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
            Self::Hold => "hold",
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
    pub vertical_slice_id: String,
    pub import_batch_id: String,
    pub governance_epoch_id: String,
    pub l2_reference_height: u64,
    pub monero_reference_height: u64,
    pub min_custody_signer_quorum_bps: u64,
    pub min_observer_quorum_bps: u64,
    pub min_reserve_handoff_quorum_bps: u64,
    pub challenge_window_blocks: u64,
    pub min_monero_confirmations: u64,
    pub max_import_lag_blocks: u64,
    pub max_release_observation_drift: u64,
    pub require_distinct_signers: bool,
    pub require_primary_custodian: bool,
    pub require_reserve_handoff_when_primary_unavailable: bool,
    pub require_monero_release_confirmation: bool,
    pub require_challenge_window_matured: bool,
    pub fail_closed_on_any_blocker: bool,
    pub final_governance_release_allowed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            import_suite: CUSTODY_ACCEPTED_LIVE_EVIDENCE_IMPORT_SUITE.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            import_batch_id: DEFAULT_IMPORT_BATCH_ID.to_string(),
            governance_epoch_id: DEFAULT_GOVERNANCE_EPOCH_ID.to_string(),
            l2_reference_height: 1_060_800,
            monero_reference_height: 3_161_280,
            min_custody_signer_quorum_bps: DEFAULT_MIN_CUSTODY_SIGNER_QUORUM_BPS,
            min_observer_quorum_bps: DEFAULT_MIN_OBSERVER_QUORUM_BPS,
            min_reserve_handoff_quorum_bps: DEFAULT_MIN_RESERVE_HANDOFF_QUORUM_BPS,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            min_monero_confirmations: DEFAULT_MIN_MONERO_CONFIRMATIONS,
            max_import_lag_blocks: DEFAULT_MAX_IMPORT_LAG_BLOCKS,
            max_release_observation_drift: DEFAULT_MAX_RELEASE_OBSERVATION_DRIFT,
            require_distinct_signers: true,
            require_primary_custodian: true,
            require_reserve_handoff_when_primary_unavailable: true,
            require_monero_release_confirmation: true,
            require_challenge_window_matured: true,
            fail_closed_on_any_blocker: true,
            final_governance_release_allowed: false,
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
            "import_suite": self.import_suite,
            "vertical_slice_id": self.vertical_slice_id,
            "import_batch_id": self.import_batch_id,
            "governance_epoch_id": self.governance_epoch_id,
            "l2_reference_height": self.l2_reference_height,
            "monero_reference_height": self.monero_reference_height,
            "min_custody_signer_quorum_bps": self.min_custody_signer_quorum_bps,
            "min_observer_quorum_bps": self.min_observer_quorum_bps,
            "min_reserve_handoff_quorum_bps": self.min_reserve_handoff_quorum_bps,
            "challenge_window_blocks": self.challenge_window_blocks,
            "min_monero_confirmations": self.min_monero_confirmations,
            "max_import_lag_blocks": self.max_import_lag_blocks,
            "max_release_observation_drift": self.max_release_observation_drift,
            "require_distinct_signers": self.require_distinct_signers,
            "require_primary_custodian": self.require_primary_custodian,
            "require_reserve_handoff_when_primary_unavailable": self.require_reserve_handoff_when_primary_unavailable,
            "require_monero_release_confirmation": self.require_monero_release_confirmation,
            "require_challenge_window_matured": self.require_challenge_window_matured,
            "fail_closed_on_any_blocker": self.fail_closed_on_any_blocker,
            "final_governance_release_allowed": self.final_governance_release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodySignerAttestation {
    pub signer_id: String,
    pub role: CustodySignerRole,
    pub weight_bps: u64,
    pub accepted_custody_root: String,
    pub evidence_root: String,
    pub pq_signature_root: String,
    pub l2_height: u64,
    pub statement: String,
}

impl CustodySignerAttestation {
    pub fn devnet(
        role: CustodySignerRole,
        ordinal: u64,
        config: &Config,
        accepted_custody_root: &str,
    ) -> Self {
        let signer_id = signer_id(role, ordinal, &config.import_batch_id);
        let evidence_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-SIGNER-EVIDENCE",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&signer_id),
                HashPart::Str(accepted_custody_root),
                HashPart::U64(config.l2_reference_height),
            ],
            32,
        );
        let pq_signature_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-SIGNER-PQ-SIGNATURE",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&signer_id),
                HashPart::Str(&evidence_root),
            ],
            32,
        );
        Self {
            signer_id,
            role,
            weight_bps: role.default_weight_bps(),
            accepted_custody_root: accepted_custody_root.to_string(),
            evidence_root,
            pq_signature_root,
            l2_height: config.l2_reference_height,
            statement: format!("{} accepted live custody evidence", role.as_str()),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signer_id": self.signer_id,
            "role": self.role.as_str(),
            "weight_bps": self.weight_bps,
            "accepted_custody_root": self.accepted_custody_root,
            "evidence_root": self.evidence_root,
            "pq_signature_root": self.pq_signature_root,
            "l2_height": self.l2_height,
            "statement": self.statement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("custody_signer_attestation", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodySignerQuorum {
    pub status: EvidenceStatus,
    pub required_weight_bps: u64,
    pub observed_weight_bps: u64,
    pub distinct_signer_count: u64,
    pub primary_present: bool,
    pub duplicate_signer_count: u64,
    pub attestation_root: String,
    pub blocker_root: String,
}

impl CustodySignerQuorum {
    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "required_weight_bps": self.required_weight_bps,
            "observed_weight_bps": self.observed_weight_bps,
            "distinct_signer_count": self.distinct_signer_count,
            "primary_present": self.primary_present,
            "duplicate_signer_count": self.duplicate_signer_count,
            "attestation_root": self.attestation_root,
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("custody_signer_quorum", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MoneroReleaseObservation {
    pub observation_id: String,
    pub status: MoneroReleaseStatus,
    pub release_txid: String,
    pub release_amount_piconero: u64,
    pub destination_view_tag_root: String,
    pub observed_height: u64,
    pub confirmations: u64,
    pub observer_weight_bps: u64,
    pub observer_quorum_root: String,
    pub conflicting_spend_root: String,
    pub finality_root: String,
}

impl MoneroReleaseObservation {
    pub fn devnet(config: &Config, accepted_custody_root: &str) -> Self {
        let observation_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-MONERO-RELEASE-OBSERVATION-ID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.import_batch_id),
                HashPart::Str(accepted_custody_root),
            ],
            32,
        );
        let release_txid = domain_hash(
            "MONERO-L2-PQ-BRIDGE-MONERO-RELEASE-TXID",
            &[HashPart::Str(&observation_id)],
            32,
        );
        let destination_view_tag_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-MONERO-RELEASE-VIEW-TAG-ROOT",
            &[HashPart::Str(&release_txid), HashPart::U64(0)],
            32,
        );
        let observer_quorum_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-MONERO-RELEASE-OBSERVER-QUORUM",
            &[
                HashPart::Str(&observation_id),
                HashPart::U64(DEFAULT_MIN_OBSERVER_QUORUM_BPS + 900),
            ],
            32,
        );
        let finality_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-MONERO-RELEASE-FINALITY",
            &[
                HashPart::Str(&release_txid),
                HashPart::U64(config.monero_reference_height),
                HashPart::U64(config.min_monero_confirmations + 6),
            ],
            32,
        );
        Self {
            observation_id,
            status: MoneroReleaseStatus::ObservedConfirmed,
            release_txid,
            release_amount_piconero: 25_000_000_000,
            destination_view_tag_root,
            observed_height: config.monero_reference_height,
            confirmations: config.min_monero_confirmations + 6,
            observer_weight_bps: DEFAULT_MIN_OBSERVER_QUORUM_BPS + 900,
            observer_quorum_root,
            conflicting_spend_root: no_conflict_root("monero_release"),
            finality_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "observation_id": self.observation_id,
            "status": self.status.as_str(),
            "release_txid": self.release_txid,
            "release_amount_piconero": self.release_amount_piconero,
            "destination_view_tag_root": self.destination_view_tag_root,
            "observed_height": self.observed_height,
            "confirmations": self.confirmations,
            "observer_weight_bps": self.observer_weight_bps,
            "observer_quorum_root": self.observer_quorum_root,
            "conflicting_spend_root": self.conflicting_spend_root,
            "finality_root": self.finality_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("monero_release_observation", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveHandoff {
    pub handoff_id: String,
    pub status: ReserveHandoffStatus,
    pub source_custodian_root: String,
    pub reserve_custodian_root: String,
    pub handoff_evidence_root: String,
    pub handoff_weight_bps: u64,
    pub accepted_at_l2_height: u64,
    pub challenge_root: String,
    pub statement: String,
}

impl ReserveHandoff {
    pub fn devnet(config: &Config, accepted_custody_root: &str) -> Self {
        let handoff_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-RESERVE-HANDOFF-ID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&config.import_batch_id),
                HashPart::Str(accepted_custody_root),
            ],
            32,
        );
        let source_custodian_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-RESERVE-HANDOFF-SOURCE",
            &[HashPart::Str(&handoff_id), HashPart::Str("primary")],
            32,
        );
        let reserve_custodian_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-RESERVE-HANDOFF-RESERVE",
            &[HashPart::Str(&handoff_id), HashPart::Str("reserve")],
            32,
        );
        let handoff_evidence_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-RESERVE-HANDOFF-EVIDENCE",
            &[
                HashPart::Str(&handoff_id),
                HashPart::Str(&source_custodian_root),
                HashPart::Str(&reserve_custodian_root),
            ],
            32,
        );
        Self {
            handoff_id,
            status: ReserveHandoffStatus::Accepted,
            source_custodian_root,
            reserve_custodian_root,
            handoff_evidence_root,
            handoff_weight_bps: config.min_reserve_handoff_quorum_bps + 800,
            accepted_at_l2_height: config.l2_reference_height,
            challenge_root: no_conflict_root("reserve_handoff"),
            statement: "Reserve handoff accepted with live custody evidence import quorum"
                .to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "handoff_id": self.handoff_id,
            "status": self.status.as_str(),
            "source_custodian_root": self.source_custodian_root,
            "reserve_custodian_root": self.reserve_custodian_root,
            "handoff_evidence_root": self.handoff_evidence_root,
            "handoff_weight_bps": self.handoff_weight_bps,
            "accepted_at_l2_height": self.accepted_at_l2_height,
            "challenge_root": self.challenge_root,
            "statement": self.statement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("reserve_handoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeWindow {
    pub window_id: String,
    pub status: ChallengeWindowStatus,
    pub opened_at_l2_height: u64,
    pub closes_at_l2_height: u64,
    pub observed_at_l2_height: u64,
    pub challenge_count: u64,
    pub challenge_root: String,
    pub timelock_root: String,
}

impl ChallengeWindow {
    pub fn devnet(config: &Config, accepted_custody_root: &str) -> Self {
        let opened_at_l2_height = config
            .l2_reference_height
            .saturating_sub(config.challenge_window_blocks + 12);
        let closes_at_l2_height = opened_at_l2_height + config.challenge_window_blocks;
        let window_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-CHALLENGE-WINDOW-ID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(accepted_custody_root),
                HashPart::U64(opened_at_l2_height),
                HashPart::U64(closes_at_l2_height),
            ],
            32,
        );
        let timelock_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-CHALLENGE-TIMELOCK",
            &[
                HashPart::Str(&window_id),
                HashPart::U64(opened_at_l2_height),
                HashPart::U64(closes_at_l2_height),
            ],
            32,
        );
        Self {
            window_id,
            status: ChallengeWindowStatus::Matured,
            opened_at_l2_height,
            closes_at_l2_height,
            observed_at_l2_height: config.l2_reference_height,
            challenge_count: 0,
            challenge_root: no_conflict_root("challenge_window"),
            timelock_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "window_id": self.window_id,
            "status": self.status.as_str(),
            "opened_at_l2_height": self.opened_at_l2_height,
            "closes_at_l2_height": self.closes_at_l2_height,
            "observed_at_l2_height": self.observed_at_l2_height,
            "challenge_count": self.challenge_count,
            "challenge_root": self.challenge_root,
            "timelock_root": self.timelock_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("challenge_window", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustodyBlocker {
    pub blocker_id: String,
    pub kind: CustodyBlockerKind,
    pub severity: u64,
    pub evidence_root: String,
    pub message: String,
}

impl CustodyBlocker {
    pub fn new(
        kind: CustodyBlockerKind,
        severity: u64,
        evidence_root: String,
        message: &str,
    ) -> Self {
        let blocker_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-BLOCKER-ID",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(kind.as_str()),
                HashPart::U64(severity),
                HashPart::Str(&evidence_root),
                HashPart::Str(message),
            ],
            32,
        );
        Self {
            blocker_id,
            kind,
            severity,
            evidence_root,
            message: message.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "severity": self.severity,
            "evidence_root": self.evidence_root,
            "message": self.message,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("custody_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedLiveEvidenceImport {
    pub import_id: String,
    pub status: EvidenceStatus,
    pub accepted_custody_root: String,
    pub custody_signer_quorum_root: String,
    pub monero_release_observation_root: String,
    pub reserve_handoff_root: String,
    pub challenge_window_root: String,
    pub blocker_root: String,
    pub imported_at_l2_height: u64,
    pub imported_at_monero_height: u64,
    pub statement: String,
}

impl AcceptedLiveEvidenceImport {
    pub fn public_record(&self) -> Value {
        json!({
            "import_id": self.import_id,
            "status": self.status.as_str(),
            "accepted_custody_root": self.accepted_custody_root,
            "custody_signer_quorum_root": self.custody_signer_quorum_root,
            "monero_release_observation_root": self.monero_release_observation_root,
            "reserve_handoff_root": self.reserve_handoff_root,
            "challenge_window_root": self.challenge_window_root,
            "blocker_root": self.blocker_root,
            "imported_at_l2_height": self.imported_at_l2_height,
            "imported_at_monero_height": self.imported_at_monero_height,
            "statement": self.statement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("accepted_live_evidence_import", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GovernanceVerdict {
    pub verdict_id: String,
    pub decision: GovernanceDecision,
    pub go_no_go_root: String,
    pub import_root: String,
    pub blocker_root: String,
    pub release_observation_root: String,
    pub reserve_handoff_root: String,
    pub challenge_window_root: String,
    pub reason: String,
}

impl GovernanceVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "decision": self.decision.as_str(),
            "go_no_go_root": self.go_no_go_root,
            "import_root": self.import_root,
            "blocker_root": self.blocker_root,
            "release_observation_root": self.release_observation_root,
            "reserve_handoff_root": self.reserve_handoff_root,
            "challenge_window_root": self.challenge_window_root,
            "reason": self.reason,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("governance_verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub signer_attestations: u64,
    pub distinct_signers: u64,
    pub total_signer_weight_bps: u64,
    pub blockers: u64,
    pub release_confirmations: u64,
    pub active_challenges: u64,
    pub reserve_handoff_weight_bps: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "signer_attestations": self.signer_attestations,
            "distinct_signers": self.distinct_signers,
            "total_signer_weight_bps": self.total_signer_weight_bps,
            "blockers": self.blockers,
            "release_confirmations": self.release_confirmations,
            "active_challenges": self.active_challenges,
            "reserve_handoff_weight_bps": self.reserve_handoff_weight_bps,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub signer_attestation_root: String,
    pub custody_signer_quorum_root: String,
    pub monero_release_observation_root: String,
    pub reserve_handoff_root: String,
    pub challenge_window_root: String,
    pub blocker_root: String,
    pub accepted_live_evidence_import_root: String,
    pub governance_verdict_root: String,
    pub counters_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "signer_attestation_root": self.signer_attestation_root,
            "custody_signer_quorum_root": self.custody_signer_quorum_root,
            "monero_release_observation_root": self.monero_release_observation_root,
            "reserve_handoff_root": self.reserve_handoff_root,
            "challenge_window_root": self.challenge_window_root,
            "blocker_root": self.blocker_root,
            "accepted_live_evidence_import_root": self.accepted_live_evidence_import_root,
            "governance_verdict_root": self.governance_verdict_root,
            "counters_root": self.counters_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn compute_state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-BRIDGE-CUSTODY-ACCEPTED-LIVE-EVIDENCE-IMPORT-STATE",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config_root),
                HashPart::Str(&self.signer_attestation_root),
                HashPart::Str(&self.custody_signer_quorum_root),
                HashPart::Str(&self.monero_release_observation_root),
                HashPart::Str(&self.reserve_handoff_root),
                HashPart::Str(&self.challenge_window_root),
                HashPart::Str(&self.blocker_root),
                HashPart::Str(&self.accepted_live_evidence_import_root),
                HashPart::Str(&self.governance_verdict_root),
                HashPart::Str(&self.counters_root),
            ],
            32,
        )
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub signer_attestations: Vec<CustodySignerAttestation>,
    pub custody_signer_quorum: CustodySignerQuorum,
    pub monero_release_observation: MoneroReleaseObservation,
    pub reserve_handoff: ReserveHandoff,
    pub challenge_window: ChallengeWindow,
    pub blockers: Vec<CustodyBlocker>,
    pub accepted_live_evidence_import: AcceptedLiveEvidenceImport,
    pub governance_verdict: GovernanceVerdict,
    pub counters: Counters,
    pub roots: Roots,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let accepted_custody_root = accepted_custody_root(&config);
        let signer_attestations = vec![
            CustodySignerAttestation::devnet(
                CustodySignerRole::PrimaryCustodian,
                0,
                &config,
                &accepted_custody_root,
            ),
            CustodySignerAttestation::devnet(
                CustodySignerRole::ReserveCustodian,
                1,
                &config,
                &accepted_custody_root,
            ),
            CustodySignerAttestation::devnet(
                CustodySignerRole::Watchtower,
                2,
                &config,
                &accepted_custody_root,
            ),
            CustodySignerAttestation::devnet(
                CustodySignerRole::GovernanceObserver,
                3,
                &config,
                &accepted_custody_root,
            ),
            CustodySignerAttestation::devnet(
                CustodySignerRole::EmergencyRecovery,
                4,
                &config,
                &accepted_custody_root,
            ),
        ];
        let monero_release_observation =
            MoneroReleaseObservation::devnet(&config, &accepted_custody_root);
        let reserve_handoff = ReserveHandoff::devnet(&config, &accepted_custody_root);
        let challenge_window = ChallengeWindow::devnet(&config, &accepted_custody_root);
        Self::from_import(
            config,
            signer_attestations,
            monero_release_observation,
            reserve_handoff,
            challenge_window,
        )
    }

    pub fn from_import(
        config: Config,
        signer_attestations: Vec<CustodySignerAttestation>,
        monero_release_observation: MoneroReleaseObservation,
        reserve_handoff: ReserveHandoff,
        challenge_window: ChallengeWindow,
    ) -> Self {
        let mut blockers = Vec::new();
        let custody_signer_quorum =
            evaluate_custody_signer_quorum(&config, &signer_attestations, &mut blockers);
        evaluate_monero_release(&config, &monero_release_observation, &mut blockers);
        evaluate_reserve_handoff(&config, &reserve_handoff, &mut blockers);
        evaluate_challenge_window(&config, &challenge_window, &mut blockers);
        let counters = counters_for(
            &signer_attestations,
            &blockers,
            &monero_release_observation,
            &reserve_handoff,
            &challenge_window,
        );
        let blocker_root = blocker_root(&blockers);
        let accepted_custody_root = common_accepted_custody_root(&signer_attestations)
            .unwrap_or_else(|| accepted_custody_root(&config));
        let accepted_live_evidence_import = build_import_record(
            &config,
            &accepted_custody_root,
            &custody_signer_quorum,
            &monero_release_observation,
            &reserve_handoff,
            &challenge_window,
            &blocker_root,
            blockers.is_empty(),
        );
        let governance_verdict = build_governance_verdict(
            &config,
            &accepted_live_evidence_import,
            &monero_release_observation,
            &reserve_handoff,
            &challenge_window,
            &blocker_root,
            blockers.is_empty(),
        );
        let roots = compute_roots(
            &config,
            &signer_attestations,
            &custody_signer_quorum,
            &monero_release_observation,
            &reserve_handoff,
            &challenge_window,
            &blockers,
            &accepted_live_evidence_import,
            &governance_verdict,
            &counters,
        );
        Self {
            config,
            signer_attestations,
            custody_signer_quorum,
            monero_release_observation,
            reserve_handoff,
            challenge_window,
            blockers,
            accepted_live_evidence_import,
            governance_verdict,
            counters,
            roots,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "signer_attestations": self.signer_attestations.iter().map(|item| item.public_record()).collect::<Vec<_>>(),
            "custody_signer_quorum": self.custody_signer_quorum.public_record(),
            "monero_release_observation": self.monero_release_observation.public_record(),
            "reserve_handoff": self.reserve_handoff.public_record(),
            "challenge_window": self.challenge_window.public_record(),
            "blockers": self.blockers.iter().map(|item| item.public_record()).collect::<Vec<_>>(),
            "accepted_live_evidence_import": self.accepted_live_evidence_import.public_record(),
            "governance_verdict": self.governance_verdict.public_record(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_commitment_root.clone()
    }

    pub fn is_governance_go(&self) -> bool {
        matches!(self.governance_verdict.decision, GovernanceDecision::Go)
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

fn evaluate_custody_signer_quorum(
    config: &Config,
    attestations: &[CustodySignerAttestation],
    blockers: &mut Vec<CustodyBlocker>,
) -> CustodySignerQuorum {
    let mut distinct = BTreeSet::new();
    let mut duplicate_signer_count = 0_u64;
    let mut observed_weight_bps = 0_u64;
    let mut primary_present = false;

    for attestation in attestations {
        if distinct.insert(attestation.signer_id.clone()) {
            observed_weight_bps = observed_weight_bps.saturating_add(attestation.weight_bps);
        } else {
            duplicate_signer_count = duplicate_signer_count.saturating_add(1);
        }
        if matches!(attestation.role, CustodySignerRole::PrimaryCustodian) {
            primary_present = true;
        }
    }

    let attestation_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-SIGNER-ATTESTATIONS",
        &attestations
            .iter()
            .map(|attestation| attestation.public_record())
            .collect::<Vec<_>>(),
    );
    if config.require_distinct_signers && duplicate_signer_count > 0 {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::DuplicateSigner,
            80,
            attestation_root.clone(),
            "custody signer set contains duplicate signer ids",
        ));
    }
    if observed_weight_bps < config.min_custody_signer_quorum_bps {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::SignerQuorumMissing,
            100,
            attestation_root.clone(),
            "custody signer quorum weight is below required threshold",
        ));
    }
    if attestations.is_empty() {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::SignerWeightTooLow,
            100,
            attestation_root.clone(),
            "custody signer attestation set is empty",
        ));
    }
    if config.require_primary_custodian && !primary_present {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::SignerQuorumMissing,
            95,
            attestation_root.clone(),
            "primary custodian attestation is missing",
        ));
    }
    let blocker_root = blocker_root(blockers);
    let status = if observed_weight_bps >= config.min_custody_signer_quorum_bps
        && duplicate_signer_count == 0
        && (!config.require_primary_custodian || primary_present)
    {
        EvidenceStatus::Accepted
    } else {
        EvidenceStatus::Rejected
    };
    CustodySignerQuorum {
        status,
        required_weight_bps: config.min_custody_signer_quorum_bps,
        observed_weight_bps,
        distinct_signer_count: distinct.len() as u64,
        primary_present,
        duplicate_signer_count,
        attestation_root,
        blocker_root,
    }
}

fn evaluate_monero_release(
    config: &Config,
    observation: &MoneroReleaseObservation,
    blockers: &mut Vec<CustodyBlocker>,
) {
    if config.require_monero_release_confirmation && !observation.status.confirms_release() {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::MoneroReleaseNotConfirmed,
            100,
            observation.state_root(),
            "monero release transaction is not final-confirmed",
        ));
    }
    if matches!(observation.status, MoneroReleaseStatus::ConflictingSpend) {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::ReleaseConflict,
            100,
            observation.conflicting_spend_root.clone(),
            "monero release observation reports a conflicting spend",
        ));
    }
    if observation.confirmations < config.min_monero_confirmations {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::MoneroReleaseNotConfirmed,
            90,
            observation.finality_root.clone(),
            "monero release confirmations are below governance import threshold",
        ));
    }
    if observation.observer_weight_bps < config.min_observer_quorum_bps {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::SignerQuorumMissing,
            85,
            observation.observer_quorum_root.clone(),
            "release observer quorum is below configured threshold",
        ));
    }
    let reference_gap = observation
        .observed_height
        .abs_diff(config.monero_reference_height);
    if reference_gap > config.max_release_observation_drift {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::ImportLagExceeded,
            70,
            observation.state_root(),
            "monero release observation drift exceeds configured import tolerance",
        ));
    }
}

fn evaluate_reserve_handoff(
    config: &Config,
    handoff: &ReserveHandoff,
    blockers: &mut Vec<CustodyBlocker>,
) {
    if config.require_reserve_handoff_when_primary_unavailable && !handoff.status.passes() {
        let kind = if matches!(handoff.status, ReserveHandoffStatus::Challenged) {
            CustodyBlockerKind::ReserveHandoffChallenged
        } else {
            CustodyBlockerKind::ReserveHandoffMissing
        };
        blockers.push(CustodyBlocker::new(
            kind,
            90,
            handoff.state_root(),
            "reserve handoff does not pass final governance import",
        ));
    }
    if matches!(handoff.status, ReserveHandoffStatus::Accepted)
        && handoff.handoff_weight_bps < config.min_reserve_handoff_quorum_bps
    {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::ReserveHandoffMissing,
            80,
            handoff.handoff_evidence_root.clone(),
            "reserve handoff quorum weight is below configured threshold",
        ));
    }
}

fn evaluate_challenge_window(
    config: &Config,
    window: &ChallengeWindow,
    blockers: &mut Vec<CustodyBlocker>,
) {
    if config.require_challenge_window_matured && !window.status.permits_governance() {
        let kind = if matches!(window.status, ChallengeWindowStatus::Challenged) {
            CustodyBlockerKind::ChallengePresent
        } else {
            CustodyBlockerKind::ChallengeWindowOpen
        };
        blockers.push(CustodyBlocker::new(
            kind,
            100,
            window.state_root(),
            "challenge window does not permit final go/no-go governance",
        ));
    }
    if window.challenge_count > 0 {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::ChallengePresent,
            100,
            window.challenge_root.clone(),
            "challenge records are present in custody evidence import",
        ));
    }
    if window.closes_at_l2_height < window.opened_at_l2_height {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::InvariantRootMismatch,
            100,
            window.timelock_root.clone(),
            "challenge window close height precedes open height",
        ));
    }
    if window
        .observed_at_l2_height
        .saturating_sub(window.closes_at_l2_height)
        > config.max_import_lag_blocks
    {
        blockers.push(CustodyBlocker::new(
            CustodyBlockerKind::ImportLagExceeded,
            60,
            window.state_root(),
            "challenge window import lag exceeds configured bound",
        ));
    }
}

fn build_import_record(
    config: &Config,
    accepted_custody_root: &str,
    quorum: &CustodySignerQuorum,
    release: &MoneroReleaseObservation,
    handoff: &ReserveHandoff,
    window: &ChallengeWindow,
    blocker_root: &str,
    blocker_free: bool,
) -> AcceptedLiveEvidenceImport {
    let status = if blocker_free && quorum.status.passes_governance_import() {
        EvidenceStatus::Accepted
    } else if blocker_free {
        EvidenceStatus::Observed
    } else {
        EvidenceStatus::Rejected
    };
    let import_id = domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-ACCEPTED-LIVE-EVIDENCE-IMPORT-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.import_batch_id),
            HashPart::Str(accepted_custody_root),
            HashPart::Str(&quorum.state_root()),
            HashPart::Str(&release.state_root()),
            HashPart::Str(&handoff.state_root()),
            HashPart::Str(&window.state_root()),
            HashPart::Str(blocker_root),
        ],
        32,
    );
    let statement = if status.passes_governance_import() {
        "Accepted live bridge custody evidence imported into final go/no-go governance"
    } else {
        "Bridge custody evidence import failed closed before final go/no-go governance"
    };
    AcceptedLiveEvidenceImport {
        import_id,
        status,
        accepted_custody_root: accepted_custody_root.to_string(),
        custody_signer_quorum_root: quorum.state_root(),
        monero_release_observation_root: release.state_root(),
        reserve_handoff_root: handoff.state_root(),
        challenge_window_root: window.state_root(),
        blocker_root: blocker_root.to_string(),
        imported_at_l2_height: config.l2_reference_height,
        imported_at_monero_height: config.monero_reference_height,
        statement: statement.to_string(),
    }
}

fn build_governance_verdict(
    config: &Config,
    import_record: &AcceptedLiveEvidenceImport,
    release: &MoneroReleaseObservation,
    handoff: &ReserveHandoff,
    window: &ChallengeWindow,
    blocker_root: &str,
    blocker_free: bool,
) -> GovernanceVerdict {
    let decision = if blocker_free && config.final_governance_release_allowed {
        GovernanceDecision::Go
    } else if blocker_free {
        GovernanceDecision::Hold
    } else {
        GovernanceDecision::NoGo
    };
    let go_no_go_root = domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-FINAL-GO-NO-GO-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.governance_epoch_id),
            HashPart::Str(&import_record.state_root()),
            HashPart::Str(&release.state_root()),
            HashPart::Str(&handoff.state_root()),
            HashPart::Str(&window.state_root()),
            HashPart::Str(blocker_root),
            HashPart::Str(decision.as_str()),
        ],
        32,
    );
    let verdict_id = domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-FINAL-GOVERNANCE-VERDICT-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.governance_epoch_id),
            HashPart::Str(&go_no_go_root),
        ],
        32,
    );
    let reason = match decision {
        GovernanceDecision::Go => "all custody evidence import gates passed and release is allowed",
        GovernanceDecision::Hold => {
            "custody evidence import gates passed but final governance release flag is closed"
        }
        GovernanceDecision::NoGo => "custody evidence import has fail-closed blockers",
    };
    GovernanceVerdict {
        verdict_id,
        decision,
        go_no_go_root,
        import_root: import_record.state_root(),
        blocker_root: blocker_root.to_string(),
        release_observation_root: release.state_root(),
        reserve_handoff_root: handoff.state_root(),
        challenge_window_root: window.state_root(),
        reason: reason.to_string(),
    }
}

fn counters_for(
    attestations: &[CustodySignerAttestation],
    blockers: &[CustodyBlocker],
    release: &MoneroReleaseObservation,
    handoff: &ReserveHandoff,
    window: &ChallengeWindow,
) -> Counters {
    let distinct_signers = attestations
        .iter()
        .map(|attestation| attestation.signer_id.clone())
        .collect::<BTreeSet<_>>()
        .len() as u64;
    let total_signer_weight_bps = attestations
        .iter()
        .fold(0_u64, |total, item| total.saturating_add(item.weight_bps));
    Counters {
        signer_attestations: attestations.len() as u64,
        distinct_signers,
        total_signer_weight_bps,
        blockers: blockers.len() as u64,
        release_confirmations: release.confirmations,
        active_challenges: window.challenge_count,
        reserve_handoff_weight_bps: handoff.handoff_weight_bps,
    }
}

fn compute_roots(
    config: &Config,
    attestations: &[CustodySignerAttestation],
    quorum: &CustodySignerQuorum,
    release: &MoneroReleaseObservation,
    handoff: &ReserveHandoff,
    window: &ChallengeWindow,
    blockers: &[CustodyBlocker],
    import_record: &AcceptedLiveEvidenceImport,
    verdict: &GovernanceVerdict,
    counters: &Counters,
) -> Roots {
    let signer_attestation_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-SIGNER-ATTESTATION-ROOT",
        &attestations
            .iter()
            .map(|attestation| attestation.public_record())
            .collect::<Vec<_>>(),
    );
    let blocker_root = blocker_root(blockers);
    let mut roots = Roots {
        config_root: config.state_root(),
        signer_attestation_root,
        custody_signer_quorum_root: quorum.state_root(),
        monero_release_observation_root: release.state_root(),
        reserve_handoff_root: handoff.state_root(),
        challenge_window_root: window.state_root(),
        blocker_root,
        accepted_live_evidence_import_root: import_record.state_root(),
        governance_verdict_root: verdict.state_root(),
        counters_root: counters.state_root(),
        state_commitment_root: String::new(),
    };
    roots.state_commitment_root = roots.compute_state_root();
    roots
}

fn accepted_custody_root(config: &Config) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-ACCEPTED-CUSTODY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.vertical_slice_id),
            HashPart::Str(&config.import_batch_id),
            HashPart::U64(config.l2_reference_height),
            HashPart::U64(config.monero_reference_height),
        ],
        32,
    )
}

fn signer_id(role: CustodySignerRole, ordinal: u64, batch_id: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-SIGNER-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(role.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(batch_id),
        ],
        20,
    )
}

fn no_conflict_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-NO-CONFLICT",
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Str(label)],
        32,
    )
}

fn blocker_root(blockers: &[CustodyBlocker]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-BLOCKERS",
        &blockers
            .iter()
            .map(|blocker| blocker.public_record())
            .collect::<Vec<_>>(),
    )
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-CUSTODY-ACCEPTED-LIVE-EVIDENCE-IMPORT-RECORD",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn common_accepted_custody_root(attestations: &[CustodySignerAttestation]) -> Option<String> {
    let first = attestations.first()?;
    if attestations
        .iter()
        .all(|item| item.accepted_custody_root == first.accepted_custody_root)
    {
        Some(first.accepted_custody_root.clone())
    } else {
        None
    }
}
