use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWatchtowerChallengeReplayAcceptanceRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WATCHTOWER_CHALLENGE_REPLAY_ACCEPTANCE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-watchtower-challenge-replay-acceptance-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WATCHTOWER_CHALLENGE_REPLAY_ACCEPTANCE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WATCHTOWER_CHALLENGE_REPLAY_ACCEPTANCE_SUITE: &str =
    "monero-l2-pq-force-exit-package-watchtower-challenge-replay-acceptance-v1";
pub const DEFAULT_DA_SAMPLE_ROOT_COUNT: u64 = 6;
pub const DEFAULT_PROOF_STATUS_REPLAY_ROOT_COUNT: u64 = 5;
pub const DEFAULT_CHALLENGE_REPORT_ROOT_COUNT: u64 = 4;
pub const DEFAULT_REVIEWER_SIGNATURE_ROOT_COUNT: u64 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub challenge_replay_acceptance_suite: String,
    pub min_da_sample_roots: u64,
    pub min_proof_status_replay_roots: u64,
    pub min_challenge_report_roots: u64,
    pub min_reviewer_signature_roots: u64,
    pub require_bridge_root_replay: bool,
    pub require_privacy_aggregate_replay: bool,
    pub require_fail_closed_on_replay_mismatch: bool,
    pub require_evidence_replacement_before_acceptance: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            challenge_replay_acceptance_suite: WATCHTOWER_CHALLENGE_REPLAY_ACCEPTANCE_SUITE
                .to_string(),
            min_da_sample_roots: DEFAULT_DA_SAMPLE_ROOT_COUNT,
            min_proof_status_replay_roots: DEFAULT_PROOF_STATUS_REPLAY_ROOT_COUNT,
            min_challenge_report_roots: DEFAULT_CHALLENGE_REPORT_ROOT_COUNT,
            min_reviewer_signature_roots: DEFAULT_REVIEWER_SIGNATURE_ROOT_COUNT,
            require_bridge_root_replay: true,
            require_privacy_aggregate_replay: true,
            require_fail_closed_on_replay_mismatch: true,
            require_evidence_replacement_before_acceptance: true,
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
            "challenge_replay_acceptance_suite": self.challenge_replay_acceptance_suite,
            "min_da_sample_roots": self.min_da_sample_roots,
            "min_proof_status_replay_roots": self.min_proof_status_replay_roots,
            "min_challenge_report_roots": self.min_challenge_report_roots,
            "min_reviewer_signature_roots": self.min_reviewer_signature_roots,
            "require_bridge_root_replay": self.require_bridge_root_replay,
            "require_privacy_aggregate_replay": self.require_privacy_aggregate_replay,
            "require_fail_closed_on_replay_mismatch": self.require_fail_closed_on_replay_mismatch,
            "require_evidence_replacement_before_acceptance": self.require_evidence_replacement_before_acceptance,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub da_sample_roots_root: String,
    pub proof_status_replay_roots_root: String,
    pub bridge_root_replay_root: String,
    pub privacy_aggregate_replay_root: String,
    pub challenge_report_roots_root: String,
    pub reviewer_signature_roots_root: String,
    pub evidence_replacement_status_root: String,
    pub acceptance_verdict_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "da_sample_roots_root": self.da_sample_roots_root,
            "proof_status_replay_roots_root": self.proof_status_replay_roots_root,
            "bridge_root_replay_root": self.bridge_root_replay_root,
            "privacy_aggregate_replay_root": self.privacy_aggregate_replay_root,
            "challenge_report_roots_root": self.challenge_report_roots_root,
            "reviewer_signature_roots_root": self.reviewer_signature_roots_root,
            "evidence_replacement_status_root": self.evidence_replacement_status_root,
            "acceptance_verdict_root": self.acceptance_verdict_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub da_sample_root_count: u64,
    pub proof_status_replay_root_count: u64,
    pub bridge_root_replay_count: u64,
    pub privacy_aggregate_replay_count: u64,
    pub challenge_report_root_count: u64,
    pub reviewer_signature_root_count: u64,
    pub accepted_reviewer_signature_count: u64,
    pub replaced_evidence_count: u64,
    pub unresolved_evidence_count: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "da_sample_root_count": self.da_sample_root_count,
            "proof_status_replay_root_count": self.proof_status_replay_root_count,
            "bridge_root_replay_count": self.bridge_root_replay_count,
            "privacy_aggregate_replay_count": self.privacy_aggregate_replay_count,
            "challenge_report_root_count": self.challenge_report_root_count,
            "reviewer_signature_root_count": self.reviewer_signature_root_count,
            "accepted_reviewer_signature_count": self.accepted_reviewer_signature_count,
            "replaced_evidence_count": self.replaced_evidence_count,
            "unresolved_evidence_count": self.unresolved_evidence_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptanceVerdictStatus {
    Accepted,
    EvidenceReplacementRequired,
    ReviewerQuorumMissing,
    ReplayMismatchFailClosed,
    FailClosed,
}

impl AcceptanceVerdictStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::EvidenceReplacementRequired => "evidence_replacement_required",
            Self::ReviewerQuorumMissing => "reviewer_quorum_missing",
            Self::ReplayMismatchFailClosed => "replay_mismatch_fail_closed",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceReplacementStatus {
    Replaced,
    ReplacementPending,
    ReplacementRejected,
}

impl EvidenceReplacementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Replaced => "replaced",
            Self::ReplacementPending => "replacement_pending",
            Self::ReplacementRejected => "replacement_rejected",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptanceVerdict {
    pub status: AcceptanceVerdictStatus,
    pub evidence_replacement_status: EvidenceReplacementStatus,
    pub da_samples_accepted: bool,
    pub proof_status_replay_accepted: bool,
    pub bridge_root_replay_accepted: bool,
    pub privacy_aggregate_replay_accepted: bool,
    pub challenge_reports_accepted: bool,
    pub reviewer_quorum_met: bool,
    pub fail_closed: bool,
    pub production_release_allowed: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl AcceptanceVerdict {
    pub fn new(config: &Config, roots: &Roots, counters: &Counters) -> Self {
        let da_samples_accepted = counters.da_sample_root_count >= config.min_da_sample_roots;
        let proof_status_replay_accepted =
            counters.proof_status_replay_root_count >= config.min_proof_status_replay_roots;
        let bridge_root_replay_accepted =
            !config.require_bridge_root_replay || counters.bridge_root_replay_count > 0;
        let privacy_aggregate_replay_accepted =
            !config.require_privacy_aggregate_replay || counters.privacy_aggregate_replay_count > 0;
        let challenge_reports_accepted =
            counters.challenge_report_root_count >= config.min_challenge_report_roots;
        let reviewer_quorum_met =
            counters.accepted_reviewer_signature_count >= config.min_reviewer_signature_roots;
        let evidence_replacement_status =
            if counters.unresolved_evidence_count == 0 && counters.replaced_evidence_count > 0 {
                EvidenceReplacementStatus::Replaced
            } else if counters.replaced_evidence_count == 0 {
                EvidenceReplacementStatus::ReplacementPending
            } else {
                EvidenceReplacementStatus::ReplacementRejected
            };
        let replacement_satisfied = !config.require_evidence_replacement_before_acceptance
            || evidence_replacement_status == EvidenceReplacementStatus::Replaced;
        let replay_accepted = da_samples_accepted
            && proof_status_replay_accepted
            && bridge_root_replay_accepted
            && privacy_aggregate_replay_accepted
            && challenge_reports_accepted;
        let status = if replay_accepted && reviewer_quorum_met && replacement_satisfied {
            AcceptanceVerdictStatus::Accepted
        } else if !reviewer_quorum_met {
            AcceptanceVerdictStatus::ReviewerQuorumMissing
        } else if !replacement_satisfied {
            AcceptanceVerdictStatus::EvidenceReplacementRequired
        } else if config.require_fail_closed_on_replay_mismatch {
            AcceptanceVerdictStatus::ReplayMismatchFailClosed
        } else {
            AcceptanceVerdictStatus::FailClosed
        };
        let fail_closed = status != AcceptanceVerdictStatus::Accepted;
        let production_release_allowed = !fail_closed;
        let user_escape_answer = if fail_closed {
            "watchtower_challenge_replay_requires_fail_closed_escape_hold".to_string()
        } else {
            "watchtower_challenge_replay_acceptance_allows_escape_progress".to_string()
        };
        let production_answer = if production_release_allowed {
            "production_release_allowed_after_challenge_replay_acceptance".to_string()
        } else {
            "production_release_blocked_until_replay_evidence_is_replaced_and_reviewed".to_string()
        };
        let verdict_root = acceptance_verdict_root(
            config,
            roots,
            counters,
            status,
            evidence_replacement_status,
            da_samples_accepted,
            proof_status_replay_accepted,
            bridge_root_replay_accepted,
            privacy_aggregate_replay_accepted,
            challenge_reports_accepted,
            reviewer_quorum_met,
            fail_closed,
        );
        Self {
            status,
            evidence_replacement_status,
            da_samples_accepted,
            proof_status_replay_accepted,
            bridge_root_replay_accepted,
            privacy_aggregate_replay_accepted,
            challenge_reports_accepted,
            reviewer_quorum_met,
            fail_closed,
            production_release_allowed,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "evidence_replacement_status": self.evidence_replacement_status.as_str(),
            "da_samples_accepted": self.da_samples_accepted,
            "proof_status_replay_accepted": self.proof_status_replay_accepted,
            "bridge_root_replay_accepted": self.bridge_root_replay_accepted,
            "privacy_aggregate_replay_accepted": self.privacy_aggregate_replay_accepted,
            "challenge_reports_accepted": self.challenge_reports_accepted,
            "reviewer_quorum_met": self.reviewer_quorum_met,
            "fail_closed": self.fail_closed,
            "production_release_allowed": self.production_release_allowed,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: Roots,
    pub counters: Counters,
    pub verdict: AcceptanceVerdict,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        validate_config(&config)?;
        let counters = devnet_counters(&config);
        let mut roots = devnet_roots(&config, &counters);
        let verdict = AcceptanceVerdict::new(&config, &roots, &counters);
        roots.acceptance_verdict_root = verdict.verdict_root.clone();
        roots.state_commitment_root = state_commitment_root(&config, &roots, &counters, &verdict);
        Ok(Self {
            config,
            roots,
            counters,
            verdict,
        })
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "state_root": self.state_root(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-CHALLENGE-REPLAY-ACCEPTANCE-STATE-ROOT",
            &[
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.roots.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&self.verdict.verdict_root),
            ],
            32,
        )
    }
}

pub fn devnet() -> State {
    State::new(Config::devnet()).unwrap_or_else(fallback_state)
}

fn devnet_counters(config: &Config) -> Counters {
    Counters {
        da_sample_root_count: config.min_da_sample_roots,
        proof_status_replay_root_count: config.min_proof_status_replay_roots,
        bridge_root_replay_count: 1,
        privacy_aggregate_replay_count: 1,
        challenge_report_root_count: config.min_challenge_report_roots,
        reviewer_signature_root_count: config.min_reviewer_signature_roots,
        accepted_reviewer_signature_count: config.min_reviewer_signature_roots,
        replaced_evidence_count: 2,
        unresolved_evidence_count: 0,
    }
}

fn devnet_roots(config: &Config, counters: &Counters) -> Roots {
    let da_sample_roots_root = indexed_roots_root(
        config,
        "da-sample-root",
        counters.da_sample_root_count,
        "force_exit_package_da_sample",
    );
    let proof_status_replay_roots_root = indexed_roots_root(
        config,
        "proof-status-replay-root",
        counters.proof_status_replay_root_count,
        "force_exit_package_proof_status_replay",
    );
    let bridge_root_replay_root = record_root(
        "bridge-root-replay",
        &json!({
            "suite": config.challenge_replay_acceptance_suite,
            "bridge_root_replay_count": counters.bridge_root_replay_count,
            "binding": "canonical_bridge_root_replay",
        }),
    );
    let privacy_aggregate_replay_root = record_root(
        "privacy-aggregate-replay",
        &json!({
            "suite": config.challenge_replay_acceptance_suite,
            "privacy_aggregate_replay_count": counters.privacy_aggregate_replay_count,
            "binding": "privacy_aggregate_replay",
        }),
    );
    let challenge_report_roots_root = indexed_roots_root(
        config,
        "challenge-report-root",
        counters.challenge_report_root_count,
        "watchtower_challenge_report",
    );
    let reviewer_signature_roots_root = indexed_roots_root(
        config,
        "reviewer-signature-root",
        counters.reviewer_signature_root_count,
        "watchtower_reviewer_signature",
    );
    let evidence_replacement_status_root = record_root(
        "evidence-replacement-status",
        &json!({
            "suite": config.challenge_replay_acceptance_suite,
            "replaced_evidence_count": counters.replaced_evidence_count,
            "unresolved_evidence_count": counters.unresolved_evidence_count,
            "replacement_status": if counters.unresolved_evidence_count == 0 { "replaced" } else { "replacement_pending" },
        }),
    );
    Roots {
        da_sample_roots_root,
        proof_status_replay_roots_root,
        bridge_root_replay_root,
        privacy_aggregate_replay_root,
        challenge_report_roots_root,
        reviewer_signature_roots_root,
        evidence_replacement_status_root,
        acceptance_verdict_root: String::new(),
        state_commitment_root: String::new(),
    }
}

fn indexed_roots_root(config: &Config, kind: &str, count: u64, label: &str) -> String {
    let roots = (0..count)
        .map(|ordinal| {
            record_root(
                kind,
                &json!({
                    "suite": config.challenge_replay_acceptance_suite,
                    "ordinal": ordinal,
                    "label": label,
                    "chain_id": config.chain_id,
                }),
            )
        })
        .collect::<Vec<_>>();
    merkle_root(&roots)
}

fn acceptance_verdict_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    status: AcceptanceVerdictStatus,
    evidence_replacement_status: EvidenceReplacementStatus,
    da_samples_accepted: bool,
    proof_status_replay_accepted: bool,
    bridge_root_replay_accepted: bool,
    privacy_aggregate_replay_accepted: bool,
    challenge_reports_accepted: bool,
    reviewer_quorum_met: bool,
    fail_closed: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-CHALLENGE-REPLAY-ACCEPTANCE-VERDICT",
        &[
            HashPart::Str(&config.challenge_replay_acceptance_suite),
            HashPart::Str(&roots.da_sample_roots_root),
            HashPart::Str(&roots.proof_status_replay_roots_root),
            HashPart::Str(&roots.bridge_root_replay_root),
            HashPart::Str(&roots.privacy_aggregate_replay_root),
            HashPart::Str(&roots.challenge_report_roots_root),
            HashPart::Str(&roots.reviewer_signature_roots_root),
            HashPart::Str(&roots.evidence_replacement_status_root),
            HashPart::U64(counters.da_sample_root_count),
            HashPart::U64(counters.proof_status_replay_root_count),
            HashPart::U64(counters.challenge_report_root_count),
            HashPart::U64(counters.accepted_reviewer_signature_count),
            HashPart::U64(counters.replaced_evidence_count),
            HashPart::U64(counters.unresolved_evidence_count),
            HashPart::Str(status.as_str()),
            HashPart::Str(evidence_replacement_status.as_str()),
            HashPart::Str(bool_str(da_samples_accepted)),
            HashPart::Str(bool_str(proof_status_replay_accepted)),
            HashPart::Str(bool_str(bridge_root_replay_accepted)),
            HashPart::Str(bool_str(privacy_aggregate_replay_accepted)),
            HashPart::Str(bool_str(challenge_reports_accepted)),
            HashPart::Str(bool_str(reviewer_quorum_met)),
            HashPart::Str(bool_str(fail_closed)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    verdict: &AcceptanceVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-CHALLENGE-REPLAY-ACCEPTANCE-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&roots.da_sample_roots_root),
            HashPart::Str(&roots.proof_status_replay_roots_root),
            HashPart::Str(&roots.bridge_root_replay_root),
            HashPart::Str(&roots.privacy_aggregate_replay_root),
            HashPart::Str(&roots.challenge_report_roots_root),
            HashPart::Str(&roots.reviewer_signature_roots_root),
            HashPart::Str(&roots.evidence_replacement_status_root),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "watchtower challenge replay acceptance chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "watchtower challenge replay acceptance protocol mismatch",
    )?;
    ensure(
        config.schema_version == SCHEMA_VERSION,
        "watchtower challenge replay acceptance schema mismatch",
    )?;
    ensure(
        config.min_da_sample_roots > 0,
        "watchtower challenge replay acceptance requires DA sample roots",
    )?;
    ensure(
        config.min_proof_status_replay_roots > 0,
        "watchtower challenge replay acceptance requires proof-status replay roots",
    )?;
    ensure(
        config.min_reviewer_signature_roots > 0,
        "watchtower challenge replay acceptance requires reviewer signatures",
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
    let counters = Counters {
        da_sample_root_count: 0,
        proof_status_replay_root_count: 0,
        bridge_root_replay_count: 0,
        privacy_aggregate_replay_count: 0,
        challenge_report_root_count: 1,
        reviewer_signature_root_count: 0,
        accepted_reviewer_signature_count: 0,
        replaced_evidence_count: 0,
        unresolved_evidence_count: 1,
    };
    let mut roots = Roots {
        da_sample_roots_root: record_root("fallback-da-samples", &json!({"reason": &reason})),
        proof_status_replay_roots_root: record_root(
            "fallback-proof-status-replay",
            &json!({"reason": &reason}),
        ),
        bridge_root_replay_root: record_root(
            "fallback-bridge-root-replay",
            &json!({"reason": &reason}),
        ),
        privacy_aggregate_replay_root: record_root(
            "fallback-privacy-aggregate-replay",
            &json!({"reason": &reason}),
        ),
        challenge_report_roots_root: record_root(
            "fallback-challenge-report",
            &json!({"reason": &reason}),
        ),
        reviewer_signature_roots_root: record_root(
            "fallback-reviewer-signature",
            &json!({"reason": &reason}),
        ),
        evidence_replacement_status_root: record_root(
            "fallback-evidence-replacement",
            &json!({"reason": &reason}),
        ),
        acceptance_verdict_root: String::new(),
        state_commitment_root: String::new(),
    };
    let verdict = AcceptanceVerdict::new(&config, &roots, &counters);
    roots.acceptance_verdict_root = verdict.verdict_root.clone();
    roots.state_commitment_root = state_commitment_root(&config, &roots, &counters, &verdict);
    State {
        config,
        roots,
        counters,
        verdict,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-CHALLENGE-REPLAY-ACCEPTANCE-RECORD",
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

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}
