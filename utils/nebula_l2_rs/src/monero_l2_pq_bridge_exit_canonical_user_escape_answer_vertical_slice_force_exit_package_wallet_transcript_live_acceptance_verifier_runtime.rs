use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletTranscriptLiveAcceptanceVerifierRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_TRANSCRIPT_LIVE_ACCEPTANCE_VERIFIER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-transcript-live-acceptance-verifier-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_TRANSCRIPT_LIVE_ACCEPTANCE_VERIFIER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WALLET_TRANSCRIPT_LIVE_ACCEPTANCE_VERIFIER_SUITE: &str =
    "monero-l2-pq-force-exit-package-wallet-transcript-live-acceptance-verifier-v1";
pub const DEFAULT_MIN_SCAN_TAG_ROOTS: u64 = 6;
pub const DEFAULT_MIN_FINALITY_CONFIRMATIONS: u64 = 12;
pub const DEFAULT_MIN_REDACTION_ROOTS: u64 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub verifier_suite: String,
    pub min_scan_tag_roots: u64,
    pub min_finality_confirmations: u64,
    pub min_redaction_roots: u64,
    pub require_wallet_scan_tags: bool,
    pub require_recovery_transcripts: bool,
    pub require_owned_note_continuity: bool,
    pub require_nullifier_continuity: bool,
    pub require_monero_finality: bool,
    pub require_fee_rebate_accounting: bool,
    pub require_private_transcript_redaction: bool,
    pub hold_release_until_live_acceptance: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            verifier_suite: WALLET_TRANSCRIPT_LIVE_ACCEPTANCE_VERIFIER_SUITE.to_string(),
            min_scan_tag_roots: DEFAULT_MIN_SCAN_TAG_ROOTS,
            min_finality_confirmations: DEFAULT_MIN_FINALITY_CONFIRMATIONS,
            min_redaction_roots: DEFAULT_MIN_REDACTION_ROOTS,
            require_wallet_scan_tags: true,
            require_recovery_transcripts: true,
            require_owned_note_continuity: true,
            require_nullifier_continuity: true,
            require_monero_finality: true,
            require_fee_rebate_accounting: true,
            require_private_transcript_redaction: true,
            hold_release_until_live_acceptance: true,
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
            "verifier_suite": self.verifier_suite,
            "min_scan_tag_roots": self.min_scan_tag_roots,
            "min_finality_confirmations": self.min_finality_confirmations,
            "min_redaction_roots": self.min_redaction_roots,
            "require_wallet_scan_tags": self.require_wallet_scan_tags,
            "require_recovery_transcripts": self.require_recovery_transcripts,
            "require_owned_note_continuity": self.require_owned_note_continuity,
            "require_nullifier_continuity": self.require_nullifier_continuity,
            "require_monero_finality": self.require_monero_finality,
            "require_fee_rebate_accounting": self.require_fee_rebate_accounting,
            "require_private_transcript_redaction": self.require_private_transcript_redaction,
            "hold_release_until_live_acceptance": self.hold_release_until_live_acceptance,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptLaneKind {
    WalletScanTags,
    RecoveryTranscripts,
    OwnedNoteContinuity,
    NullifierContinuity,
    MoneroFinality,
    FeeRebateAccounting,
    PrivateTranscriptRedaction,
}

impl TranscriptLaneKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::WalletScanTags,
            Self::RecoveryTranscripts,
            Self::OwnedNoteContinuity,
            Self::NullifierContinuity,
            Self::MoneroFinality,
            Self::FeeRebateAccounting,
            Self::PrivateTranscriptRedaction,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletScanTags => "wallet_scan_tags",
            Self::RecoveryTranscripts => "recovery_transcripts",
            Self::OwnedNoteContinuity => "owned_note_continuity",
            Self::NullifierContinuity => "nullifier_continuity",
            Self::MoneroFinality => "monero_finality",
            Self::FeeRebateAccounting => "fee_rebate_accounting",
            Self::PrivateTranscriptRedaction => "private_transcript_redaction",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptLaneStatus {
    Accepted,
    Observed,
    ReleaseHeld,
}

impl TranscriptLaneStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Observed => "observed",
            Self::ReleaseHeld => "release_held",
        }
    }

    pub fn counts_as_accepted(self) -> bool {
        matches!(self, Self::Accepted | Self::Observed)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TranscriptLane {
    pub lane_id: String,
    pub ordinal: u64,
    pub kind: TranscriptLaneKind,
    pub status: TranscriptLaneStatus,
    pub evidence_root: String,
    pub live_observation_root: String,
    pub continuity_root: String,
    pub release_guard_root: String,
    pub statement: String,
}

impl TranscriptLane {
    pub fn public_record(&self) -> Value {
        json!({
            "lane_id": self.lane_id,
            "ordinal": self.ordinal,
            "kind": self.kind.as_str(),
            "status": self.status.as_str(),
            "evidence_root": self.evidence_root,
            "live_observation_root": self.live_observation_root,
            "continuity_root": self.continuity_root,
            "release_guard_root": self.release_guard_root,
            "statement": self.statement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("transcript-lane", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub wallet_scan_tag_root: String,
    pub recovery_transcript_root: String,
    pub owned_note_continuity_root: String,
    pub nullifier_continuity_root: String,
    pub monero_finality_root: String,
    pub fee_rebate_root: String,
    pub private_transcript_redaction_root: String,
    pub acceptance_verdict_root: String,
    pub release_hold_status_root: String,
    pub transcript_lane_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_tag_root": self.wallet_scan_tag_root,
            "recovery_transcript_root": self.recovery_transcript_root,
            "owned_note_continuity_root": self.owned_note_continuity_root,
            "nullifier_continuity_root": self.nullifier_continuity_root,
            "monero_finality_root": self.monero_finality_root,
            "fee_rebate_root": self.fee_rebate_root,
            "private_transcript_redaction_root": self.private_transcript_redaction_root,
            "acceptance_verdict_root": self.acceptance_verdict_root,
            "release_hold_status_root": self.release_hold_status_root,
            "transcript_lane_root": self.transcript_lane_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub total_lanes: u64,
    pub accepted_lanes: u64,
    pub release_held_lanes: u64,
    pub scan_tag_roots: u64,
    pub finality_confirmations: u64,
    pub redaction_roots: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "total_lanes": self.total_lanes,
            "accepted_lanes": self.accepted_lanes,
            "release_held_lanes": self.release_held_lanes,
            "scan_tag_roots": self.scan_tag_roots,
            "finality_confirmations": self.finality_confirmations,
            "redaction_roots": self.redaction_roots,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptanceVerdict {
    pub accepted: bool,
    pub release_hold_active: bool,
    pub verdict_status: String,
    pub verdict_root: String,
}

impl AcceptanceVerdict {
    pub fn public_record(&self) -> Value {
        json!({
            "accepted": self.accepted,
            "release_hold_active": self.release_hold_active,
            "verdict_status": self.verdict_status,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("acceptance-verdict", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub lanes: Vec<TranscriptLane>,
    pub roots: Roots,
    pub counters: Counters,
    pub verdict: AcceptanceVerdict,
}

impl State {
    pub fn new(config: Config, lanes: Vec<TranscriptLane>) -> Result<Self> {
        validate_config(&config)?;
        ensure(!lanes.is_empty(), "wallet transcript verifier has no lanes")?;

        let counters = counters(&config, &lanes);
        let wallet_scan_tag_root = lane_kind_root(&lanes, TranscriptLaneKind::WalletScanTags);
        let recovery_transcript_root =
            lane_kind_root(&lanes, TranscriptLaneKind::RecoveryTranscripts);
        let owned_note_continuity_root =
            lane_kind_root(&lanes, TranscriptLaneKind::OwnedNoteContinuity);
        let nullifier_continuity_root =
            lane_kind_root(&lanes, TranscriptLaneKind::NullifierContinuity);
        let monero_finality_root = lane_kind_root(&lanes, TranscriptLaneKind::MoneroFinality);
        let fee_rebate_root = lane_kind_root(&lanes, TranscriptLaneKind::FeeRebateAccounting);
        let private_transcript_redaction_root =
            lane_kind_root(&lanes, TranscriptLaneKind::PrivateTranscriptRedaction);
        let accepted = acceptance_allowed(&config, &counters);
        let release_hold_active = !accepted && config.hold_release_until_live_acceptance;
        let acceptance_verdict_root = acceptance_verdict_root(
            &config,
            &counters,
            &wallet_scan_tag_root,
            &recovery_transcript_root,
            &owned_note_continuity_root,
            &nullifier_continuity_root,
            &monero_finality_root,
            &fee_rebate_root,
            &private_transcript_redaction_root,
            accepted,
            release_hold_active,
        );
        let release_hold_status_root = release_hold_status_root(
            &config,
            &counters,
            &acceptance_verdict_root,
            release_hold_active,
        );
        let transcript_lane_root = merkle_root(
            "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-LANES",
            &lanes
                .iter()
                .map(TranscriptLane::public_record)
                .collect::<Vec<_>>(),
        );
        let state_commitment_root = state_commitment_root(
            &config,
            &counters,
            &wallet_scan_tag_root,
            &recovery_transcript_root,
            &owned_note_continuity_root,
            &nullifier_continuity_root,
            &monero_finality_root,
            &fee_rebate_root,
            &private_transcript_redaction_root,
            &acceptance_verdict_root,
            &release_hold_status_root,
            &transcript_lane_root,
            accepted,
        );
        let roots = Roots {
            wallet_scan_tag_root,
            recovery_transcript_root,
            owned_note_continuity_root,
            nullifier_continuity_root,
            monero_finality_root,
            fee_rebate_root,
            private_transcript_redaction_root,
            acceptance_verdict_root: acceptance_verdict_root.clone(),
            release_hold_status_root,
            transcript_lane_root,
            state_commitment_root,
        };
        let verdict = AcceptanceVerdict {
            accepted,
            release_hold_active,
            verdict_status: if accepted {
                "wallet_transcript_live_acceptance_verified"
            } else {
                "wallet_transcript_live_acceptance_release_held"
            }
            .to_string(),
            verdict_root: acceptance_verdict_root,
        };

        Ok(Self {
            config,
            lanes,
            roots,
            counters,
            verdict,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "lanes": self.lanes.iter().map(TranscriptLane::public_record).collect::<Vec<_>>(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_commitment_root.clone()
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let lanes = TranscriptLaneKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, kind)| transcript_lane(&config, *kind, index as u64 + 1))
        .collect::<Vec<_>>();
    match State::new(config, lanes) {
        Ok(state) => state,
        Err(reason) => fallback_state(reason),
    }
}

fn transcript_lane(config: &Config, kind: TranscriptLaneKind, ordinal: u64) -> TranscriptLane {
    let status = match kind {
        TranscriptLaneKind::WalletScanTags
        | TranscriptLaneKind::RecoveryTranscripts
        | TranscriptLaneKind::OwnedNoteContinuity
        | TranscriptLaneKind::NullifierContinuity
        | TranscriptLaneKind::MoneroFinality
        | TranscriptLaneKind::FeeRebateAccounting
        | TranscriptLaneKind::PrivateTranscriptRedaction => TranscriptLaneStatus::Accepted,
    };
    let evidence_root = evidence_root(config, kind, ordinal);
    let live_observation_root = record_root(
        "live-observation",
        &json!({
            "kind": kind.as_str(),
            "ordinal": ordinal,
            "evidence_root": &evidence_root,
            "observation": live_observation(kind),
        }),
    );
    let continuity_root = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-CONTINUITY",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(&evidence_root),
            HashPart::Str(&live_observation_root),
        ],
        32,
    );
    let release_guard_root = record_root(
        "release-guard",
        &json!({
            "kind": kind.as_str(),
            "ordinal": ordinal,
            "status": status.as_str(),
            "guard": release_guard(kind),
        }),
    );
    let lane_id = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-LANE-ID",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::Str(&continuity_root),
        ],
        32,
    );

    TranscriptLane {
        lane_id,
        ordinal,
        kind,
        status,
        evidence_root,
        live_observation_root,
        continuity_root,
        release_guard_root,
        statement: acceptance_statement(kind).to_string(),
    }
}

fn counters(config: &Config, lanes: &[TranscriptLane]) -> Counters {
    let total_lanes = lanes.len() as u64;
    let accepted_lanes = lanes
        .iter()
        .filter(|lane| lane.status.counts_as_accepted())
        .count() as u64;
    let release_held_lanes = total_lanes.saturating_sub(accepted_lanes);

    Counters {
        total_lanes,
        accepted_lanes,
        release_held_lanes,
        scan_tag_roots: config.min_scan_tag_roots,
        finality_confirmations: config.min_finality_confirmations,
        redaction_roots: config.min_redaction_roots,
    }
}

fn lane_kind_root(lanes: &[TranscriptLane], kind: TranscriptLaneKind) -> String {
    let records = lanes
        .iter()
        .filter(|lane| lane.kind == kind)
        .map(TranscriptLane::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-KIND",
        &records,
    )
}

fn evidence_root(config: &Config, kind: TranscriptLaneKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-EVIDENCE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.verifier_suite),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::U64(config.min_scan_tag_roots),
            HashPart::U64(config.min_finality_confirmations),
            HashPart::U64(config.min_redaction_roots),
        ],
        32,
    )
}

fn acceptance_allowed(config: &Config, counters: &Counters) -> bool {
    counters.accepted_lanes == counters.total_lanes
        && counters.scan_tag_roots >= config.min_scan_tag_roots
        && counters.finality_confirmations >= config.min_finality_confirmations
        && counters.redaction_roots >= config.min_redaction_roots
}

fn acceptance_verdict_root(
    config: &Config,
    counters: &Counters,
    wallet_scan_tag_root: &str,
    recovery_transcript_root: &str,
    owned_note_continuity_root: &str,
    nullifier_continuity_root: &str,
    monero_finality_root: &str,
    fee_rebate_root: &str,
    private_transcript_redaction_root: &str,
    accepted: bool,
    release_hold_active: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(wallet_scan_tag_root),
            HashPart::Str(recovery_transcript_root),
            HashPart::Str(owned_note_continuity_root),
            HashPart::Str(nullifier_continuity_root),
            HashPart::Str(monero_finality_root),
            HashPart::Str(fee_rebate_root),
            HashPart::Str(private_transcript_redaction_root),
            HashPart::U64(counters.accepted_lanes),
            HashPart::Str(bool_str(accepted)),
            HashPart::Str(bool_str(release_hold_active)),
        ],
        32,
    )
}

fn release_hold_status_root(
    config: &Config,
    counters: &Counters,
    acceptance_verdict_root: &str,
    release_hold_active: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-RELEASE-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(acceptance_verdict_root),
            HashPart::U64(counters.release_held_lanes),
            HashPart::Str(bool_str(config.hold_release_until_live_acceptance)),
            HashPart::Str(bool_str(release_hold_active)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    counters: &Counters,
    wallet_scan_tag_root: &str,
    recovery_transcript_root: &str,
    owned_note_continuity_root: &str,
    nullifier_continuity_root: &str,
    monero_finality_root: &str,
    fee_rebate_root: &str,
    private_transcript_redaction_root: &str,
    acceptance_verdict_root: &str,
    release_hold_status_root: &str,
    transcript_lane_root: &str,
    accepted: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.state_root()),
            HashPart::Str(wallet_scan_tag_root),
            HashPart::Str(recovery_transcript_root),
            HashPart::Str(owned_note_continuity_root),
            HashPart::Str(nullifier_continuity_root),
            HashPart::Str(monero_finality_root),
            HashPart::Str(fee_rebate_root),
            HashPart::Str(private_transcript_redaction_root),
            HashPart::Str(acceptance_verdict_root),
            HashPart::Str(release_hold_status_root),
            HashPart::Str(transcript_lane_root),
            HashPart::U64(counters.total_lanes),
            HashPart::U64(counters.accepted_lanes),
            HashPart::Str(bool_str(accepted)),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "wallet transcript verifier chain id mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "wallet transcript verifier protocol version mismatch",
    )?;
    ensure(
        config.min_scan_tag_roots > 0,
        "wallet transcript verifier requires scan tag roots",
    )?;
    ensure(
        config.min_finality_confirmations > 0,
        "wallet transcript verifier requires Monero finality confirmations",
    )?;
    ensure(
        config.min_redaction_roots > 0,
        "wallet transcript verifier requires redaction roots",
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
    let lanes = TranscriptLaneKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, kind)| {
            let mut lane = transcript_lane(&config, *kind, index as u64 + 1);
            lane.status = TranscriptLaneStatus::ReleaseHeld;
            lane.statement = reason.clone();
            lane
        })
        .collect::<Vec<_>>();
    let counters = counters(&config, &lanes);
    let wallet_scan_tag_root = lane_kind_root(&lanes, TranscriptLaneKind::WalletScanTags);
    let recovery_transcript_root = lane_kind_root(&lanes, TranscriptLaneKind::RecoveryTranscripts);
    let owned_note_continuity_root =
        lane_kind_root(&lanes, TranscriptLaneKind::OwnedNoteContinuity);
    let nullifier_continuity_root = lane_kind_root(&lanes, TranscriptLaneKind::NullifierContinuity);
    let monero_finality_root = lane_kind_root(&lanes, TranscriptLaneKind::MoneroFinality);
    let fee_rebate_root = lane_kind_root(&lanes, TranscriptLaneKind::FeeRebateAccounting);
    let private_transcript_redaction_root =
        lane_kind_root(&lanes, TranscriptLaneKind::PrivateTranscriptRedaction);
    let acceptance_verdict_root = record_root(
        "fallback-acceptance-verdict",
        &json!({"reason": reason, "accepted": false}),
    );
    let release_hold_status_root =
        release_hold_status_root(&config, &counters, &acceptance_verdict_root, true);
    let transcript_lane_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-FALLBACK-LANES",
        &lanes
            .iter()
            .map(TranscriptLane::public_record)
            .collect::<Vec<_>>(),
    );
    let state_commitment_root = state_commitment_root(
        &config,
        &counters,
        &wallet_scan_tag_root,
        &recovery_transcript_root,
        &owned_note_continuity_root,
        &nullifier_continuity_root,
        &monero_finality_root,
        &fee_rebate_root,
        &private_transcript_redaction_root,
        &acceptance_verdict_root,
        &release_hold_status_root,
        &transcript_lane_root,
        false,
    );
    let roots = Roots {
        wallet_scan_tag_root,
        recovery_transcript_root,
        owned_note_continuity_root,
        nullifier_continuity_root,
        monero_finality_root,
        fee_rebate_root,
        private_transcript_redaction_root,
        acceptance_verdict_root: acceptance_verdict_root.clone(),
        release_hold_status_root,
        transcript_lane_root,
        state_commitment_root,
    };
    let verdict = AcceptanceVerdict {
        accepted: false,
        release_hold_active: true,
        verdict_status: "wallet_transcript_live_acceptance_fallback_release_held".to_string(),
        verdict_root: acceptance_verdict_root,
    };

    State {
        config,
        lanes,
        roots,
        counters,
        verdict,
    }
}

fn live_observation(kind: TranscriptLaneKind) -> &'static str {
    match kind {
        TranscriptLaneKind::WalletScanTags => {
            "wallet scan tag roots are live-observed before force-exit acceptance"
        }
        TranscriptLaneKind::RecoveryTranscripts => {
            "recovery transcript roots are bound to wallet-visible acceptance evidence"
        }
        TranscriptLaneKind::OwnedNoteContinuity => {
            "owned-note continuity roots preserve the user's private note lineage"
        }
        TranscriptLaneKind::NullifierContinuity => {
            "nullifier continuity roots prevent duplicate release while retaining privacy"
        }
        TranscriptLaneKind::MoneroFinality => {
            "Monero finality roots satisfy live acceptance confirmation depth"
        }
        TranscriptLaneKind::FeeRebateAccounting => {
            "fee and rebate roots match the accepted force-exit transcript"
        }
        TranscriptLaneKind::PrivateTranscriptRedaction => {
            "private transcript redaction roots keep wallet acceptance minimally disclosed"
        }
    }
}

fn release_guard(kind: TranscriptLaneKind) -> &'static str {
    match kind {
        TranscriptLaneKind::WalletScanTags => "release held unless scan tag roots are accepted",
        TranscriptLaneKind::RecoveryTranscripts => {
            "release held unless recovery transcript roots are accepted"
        }
        TranscriptLaneKind::OwnedNoteContinuity => {
            "release held unless owned-note continuity is accepted"
        }
        TranscriptLaneKind::NullifierContinuity => {
            "release held unless nullifier continuity is accepted"
        }
        TranscriptLaneKind::MoneroFinality => "release held unless Monero finality is accepted",
        TranscriptLaneKind::FeeRebateAccounting => {
            "release held unless fee and rebate accounting is accepted"
        }
        TranscriptLaneKind::PrivateTranscriptRedaction => {
            "release held unless private transcript redaction is accepted"
        }
    }
}

fn acceptance_statement(kind: TranscriptLaneKind) -> &'static str {
    match kind {
        TranscriptLaneKind::WalletScanTags => {
            "wallet scan tag roots are committed for live acceptance verification"
        }
        TranscriptLaneKind::RecoveryTranscripts => {
            "recovery transcript roots are committed for user escape recovery"
        }
        TranscriptLaneKind::OwnedNoteContinuity => {
            "owned-note continuity roots are committed across the acceptance boundary"
        }
        TranscriptLaneKind::NullifierContinuity => {
            "nullifier continuity roots are committed across the acceptance boundary"
        }
        TranscriptLaneKind::MoneroFinality => {
            "Monero finality roots are committed before release eligibility"
        }
        TranscriptLaneKind::FeeRebateAccounting => {
            "fee and rebate roots are committed for wallet transcript settlement"
        }
        TranscriptLaneKind::PrivateTranscriptRedaction => {
            "private transcript redaction roots are committed before public acceptance"
        }
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-TRANSCRIPT-LIVE-ACCEPTANCE-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
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
