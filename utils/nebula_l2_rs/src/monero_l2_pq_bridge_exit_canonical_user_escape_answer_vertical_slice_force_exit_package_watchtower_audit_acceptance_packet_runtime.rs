use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_user_escape_verdict_bundle_runtime as verdict_bundle,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWatchtowerAuditAcceptancePacketRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WATCHTOWER_AUDIT_ACCEPTANCE_PACKET_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-watchtower-audit-acceptance-packet-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WATCHTOWER_AUDIT_ACCEPTANCE_PACKET_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WATCHTOWER_AUDIT_ACCEPTANCE_PACKET_SUITE: &str =
    "monero-l2-pq-force-exit-package-watchtower-audit-acceptance-packet-v1";
pub const DEFAULT_MIN_DA_SAMPLES: u64 = 6;
pub const DEFAULT_MIN_WATCHTOWER_SIGNATURES: u64 = 4;
pub const DEFAULT_MIN_ACCEPTING_WATCHTOWERS: u64 = 4;
pub const DEFAULT_MAX_CHALLENGE_REPORTS: u64 = 0;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub acceptance_packet_suite: String,
    pub min_da_samples: u64,
    pub min_watchtower_signatures: u64,
    pub min_accepting_watchtowers: u64,
    pub max_challenge_reports: u64,
    pub require_bridge_roots_match: bool,
    pub require_validity_aggregate: bool,
    pub require_privacy_aggregate: bool,
    pub hold_production_until_watchtower_acceptance: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            acceptance_packet_suite: WATCHTOWER_AUDIT_ACCEPTANCE_PACKET_SUITE.to_string(),
            min_da_samples: DEFAULT_MIN_DA_SAMPLES,
            min_watchtower_signatures: DEFAULT_MIN_WATCHTOWER_SIGNATURES,
            min_accepting_watchtowers: DEFAULT_MIN_ACCEPTING_WATCHTOWERS,
            max_challenge_reports: DEFAULT_MAX_CHALLENGE_REPORTS,
            require_bridge_roots_match: true,
            require_validity_aggregate: true,
            require_privacy_aggregate: true,
            hold_production_until_watchtower_acceptance: true,
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
            "acceptance_packet_suite": self.acceptance_packet_suite,
            "min_da_samples": self.min_da_samples,
            "min_watchtower_signatures": self.min_watchtower_signatures,
            "min_accepting_watchtowers": self.min_accepting_watchtowers,
            "max_challenge_reports": self.max_challenge_reports,
            "require_bridge_roots_match": self.require_bridge_roots_match,
            "require_validity_aggregate": self.require_validity_aggregate,
            "require_privacy_aggregate": self.require_privacy_aggregate,
            "hold_production_until_watchtower_acceptance": self.hold_production_until_watchtower_acceptance,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub user_escape_state_root: String,
    pub step_bundle_root: String,
    pub wallet_instruction_bundle_root: String,
    pub release_blocker_bundle_root: String,
    pub production_hold_root: String,
    pub verdict_root: String,
    pub step_count: u64,
    pub ready_step_count: u64,
    pub waiting_step_count: u64,
    pub blocked_step_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
    pub wallet_escape_answerable: bool,
    pub force_exit_safe_to_attempt: bool,
    pub production_release_allowed: bool,
    pub verdict_status: String,
}

impl SourceBundle {
    pub fn from_user_escape_verdict(state: &verdict_bundle::State) -> Self {
        Self {
            user_escape_state_root: state.state_root(),
            step_bundle_root: state.step_bundle_root.clone(),
            wallet_instruction_bundle_root: state.wallet_instruction_bundle_root.clone(),
            release_blocker_bundle_root: state.release_blocker_bundle_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            verdict_root: state.verdict.verdict_root.clone(),
            step_count: state.verdict.step_count,
            ready_step_count: state.verdict.ready_step_count,
            waiting_step_count: state.verdict.waiting_step_count,
            blocked_step_count: state.verdict.blocked_step_count,
            user_release_blocker_count: state.verdict.user_release_blocker_count,
            production_blocker_count: state.verdict.production_blocker_count,
            wallet_escape_answerable: state.verdict.wallet_escape_answerable,
            force_exit_safe_to_attempt: state.verdict.force_exit_safe_to_attempt,
            production_release_allowed: state.verdict.production_release_allowed,
            verdict_status: state.verdict.verdict_status.clone(),
        }
    }

    pub fn devnet() -> Self {
        Self::from_user_escape_verdict(&verdict_bundle::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "user_escape_state_root": self.user_escape_state_root,
            "step_bundle_root": self.step_bundle_root,
            "wallet_instruction_bundle_root": self.wallet_instruction_bundle_root,
            "release_blocker_bundle_root": self.release_blocker_bundle_root,
            "production_hold_root": self.production_hold_root,
            "verdict_root": self.verdict_root,
            "step_count": self.step_count,
            "ready_step_count": self.ready_step_count,
            "waiting_step_count": self.waiting_step_count,
            "blocked_step_count": self.blocked_step_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
            "wallet_escape_answerable": self.wallet_escape_answerable,
            "force_exit_safe_to_attempt": self.force_exit_safe_to_attempt,
            "production_release_allowed": self.production_release_allowed,
            "verdict_status": self.verdict_status,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditPacketStatus {
    Accepted,
    AwaitingQuorum,
    Challenged,
    Rejected,
}

impl AuditPacketStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::AwaitingQuorum => "awaiting_quorum",
            Self::Challenged => "challenged",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuditSignature {
    pub watchtower_id: String,
    pub ordinal: u64,
    pub signing_key_root: String,
    pub signed_packet_root: String,
    pub signature_root: String,
    pub accepted: bool,
}

impl AuditSignature {
    pub fn devnet(
        config: &Config,
        packet_seed_root: &str,
        ordinal: u64,
        watchtower_id: &str,
    ) -> Self {
        let signing_key_root = record_root(
            "watchtower-signing-key",
            &json!({
                "acceptance_packet_suite": &config.acceptance_packet_suite,
                "watchtower_id": watchtower_id,
                "ordinal": ordinal,
                "key_scope": "force_exit_watchtower_audit_acceptance",
            }),
        );
        let signed_packet_root = domain_hash(
            "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-SIGNED-PACKET",
            &[
                HashPart::Str(&config.acceptance_packet_suite),
                HashPart::Str(packet_seed_root),
                HashPart::Str(watchtower_id),
                HashPart::U64(ordinal),
            ],
            32,
        );
        let signature_root = domain_hash(
            "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-SIGNATURE",
            &[
                HashPart::Str(&signing_key_root),
                HashPart::Str(&signed_packet_root),
                HashPart::Str(watchtower_id),
            ],
            32,
        );
        Self {
            watchtower_id: watchtower_id.to_string(),
            ordinal,
            signing_key_root,
            signed_packet_root,
            signature_root,
            accepted: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "watchtower_id": self.watchtower_id,
            "ordinal": self.ordinal,
            "signing_key_root": self.signing_key_root,
            "signed_packet_root": self.signed_packet_root,
            "signature_root": self.signature_root,
            "accepted": self.accepted,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeReport {
    pub report_id: String,
    pub ordinal: u64,
    pub challenge_root: String,
    pub resolved: bool,
    pub blocks_acceptance: bool,
}

impl ChallengeReport {
    pub fn devnet(config: &Config, source: &SourceBundle, ordinal: u64) -> Self {
        let challenge_root = record_root(
            "challenge-report",
            &json!({
                "acceptance_packet_suite": &config.acceptance_packet_suite,
                "source_root": source.state_root(),
                "ordinal": ordinal,
                "challenge_kind": "no_open_challenge",
            }),
        );
        let report_id = domain_hash(
            "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-CHALLENGE-REPORT-ID",
            &[HashPart::Str(&challenge_root), HashPart::U64(ordinal)],
            16,
        );
        Self {
            report_id,
            ordinal,
            challenge_root,
            resolved: true,
            blocks_acceptance: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "report_id": self.report_id,
            "ordinal": self.ordinal,
            "challenge_root": self.challenge_root,
            "resolved": self.resolved,
            "blocks_acceptance": self.blocks_acceptance,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub da_sample_root: String,
    pub bridge_root: String,
    pub validity_aggregate_root: String,
    pub privacy_aggregate_root: String,
    pub audit_signature_root: String,
    pub challenge_report_root: String,
    pub acceptance_verdict_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "da_sample_root": self.da_sample_root,
            "bridge_root": self.bridge_root,
            "validity_aggregate_root": self.validity_aggregate_root,
            "privacy_aggregate_root": self.privacy_aggregate_root,
            "audit_signature_root": self.audit_signature_root,
            "challenge_report_root": self.challenge_report_root,
            "acceptance_verdict_root": self.acceptance_verdict_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub da_sample_count: u64,
    pub bridge_root_count: u64,
    pub audit_signature_count: u64,
    pub accepting_watchtower_count: u64,
    pub challenge_report_count: u64,
    pub blocking_challenge_count: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "da_sample_count": self.da_sample_count,
            "bridge_root_count": self.bridge_root_count,
            "audit_signature_count": self.audit_signature_count,
            "accepting_watchtower_count": self.accepting_watchtower_count,
            "challenge_report_count": self.challenge_report_count,
            "blocking_challenge_count": self.blocking_challenge_count,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptanceVerdict {
    pub status: AuditPacketStatus,
    pub da_samples_satisfied: bool,
    pub bridge_roots_match: bool,
    pub validity_accepted: bool,
    pub privacy_accepted: bool,
    pub signature_quorum_met: bool,
    pub challenges_clear: bool,
    pub production_blocked: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl AcceptanceVerdict {
    pub fn new(config: &Config, source: &SourceBundle, counters: &Counters, roots: &Roots) -> Self {
        let da_samples_satisfied = counters.da_sample_count >= config.min_da_samples;
        let bridge_roots_match =
            !config.require_bridge_roots_match || roots.bridge_root == expected_bridge_root(source);
        let validity_accepted =
            !config.require_validity_aggregate || source.force_exit_safe_to_attempt;
        let privacy_accepted = !config.require_privacy_aggregate || source.wallet_escape_answerable;
        let signature_quorum_met = counters.audit_signature_count
            >= config.min_watchtower_signatures
            && counters.accepting_watchtower_count >= config.min_accepting_watchtowers;
        let challenges_clear = counters.blocking_challenge_count == 0
            && counters.challenge_report_count <= config.max_challenge_reports;
        let accepted = da_samples_satisfied
            && bridge_roots_match
            && validity_accepted
            && privacy_accepted
            && signature_quorum_met
            && challenges_clear;
        let production_blocked = !accepted
            || source.production_blocker_count > 0
            || config.hold_production_until_watchtower_acceptance;
        let status = if accepted {
            AuditPacketStatus::Accepted
        } else if !challenges_clear {
            AuditPacketStatus::Challenged
        } else if signature_quorum_met {
            AuditPacketStatus::Rejected
        } else {
            AuditPacketStatus::AwaitingQuorum
        };
        let user_escape_answer = if accepted {
            "watchtower audit packet accepts the force-exit user escape evidence roots"
        } else if !challenges_clear {
            "watchtower audit packet has open challenge reports and cannot accept the escape"
        } else {
            "watchtower audit packet awaits sufficient DA samples, bridge roots, and signatures"
        }
        .to_string();
        let production_answer = if production_blocked {
            "production release remains held after watchtower audit acceptance review"
        } else {
            "watchtower audit acceptance packet is ready for production release review"
        }
        .to_string();
        let verdict_root = acceptance_verdict_root(
            config,
            source,
            counters,
            roots,
            status,
            da_samples_satisfied,
            bridge_roots_match,
            validity_accepted,
            privacy_accepted,
            signature_quorum_met,
            challenges_clear,
            production_blocked,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            status,
            da_samples_satisfied,
            bridge_roots_match,
            validity_accepted,
            privacy_accepted,
            signature_quorum_met,
            challenges_clear,
            production_blocked,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "da_samples_satisfied": self.da_samples_satisfied,
            "bridge_roots_match": self.bridge_roots_match,
            "validity_accepted": self.validity_accepted,
            "privacy_accepted": self.privacy_accepted,
            "signature_quorum_met": self.signature_quorum_met,
            "challenges_clear": self.challenges_clear,
            "production_blocked": self.production_blocked,
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
    pub roots: Roots,
    pub counters: Counters,
    pub audit_signatures: Vec<AuditSignature>,
    pub challenge_reports: Vec<ChallengeReport>,
    pub verdict: AcceptanceVerdict,
}

impl State {
    pub fn new(config: Config, source: SourceBundle) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        let da_sample_root = da_sample_root(&config, &source);
        let bridge_root = expected_bridge_root(&source);
        let validity_aggregate_root = validity_aggregate_root(&config, &source);
        let privacy_aggregate_root = privacy_aggregate_root(&config, &source);
        let packet_seed_root = packet_seed_root(
            &config,
            &source,
            &da_sample_root,
            &bridge_root,
            &validity_aggregate_root,
            &privacy_aggregate_root,
        );
        let audit_signatures = audit_signatures(&config, &packet_seed_root);
        let challenge_reports = challenge_reports(&config, &source);
        let audit_signature_root = audit_signature_root(&audit_signatures);
        let challenge_report_root = challenge_report_root(&challenge_reports);
        let counters = counters(&config, &audit_signatures, &challenge_reports);
        let mut roots = Roots {
            da_sample_root,
            bridge_root,
            validity_aggregate_root,
            privacy_aggregate_root,
            audit_signature_root,
            challenge_report_root,
            acceptance_verdict_root: String::new(),
            state_commitment_root: String::new(),
        };
        let verdict = AcceptanceVerdict::new(&config, &source, &counters, &roots);
        roots.acceptance_verdict_root = verdict.verdict_root.clone();
        roots.state_commitment_root = state_commitment_root(&config, &source, &roots, &counters);
        Ok(Self {
            config,
            source,
            roots,
            counters,
            audit_signatures,
            challenge_reports,
            verdict,
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
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_watchtower_audit_acceptance_packet_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "audit_signatures": self
                .audit_signatures
                .iter()
                .map(AuditSignature::public_record)
                .collect::<Vec<_>>(),
            "challenge_reports": self
                .challenge_reports
                .iter()
                .map(ChallengeReport::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root()
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

fn da_sample_root(config: &Config, source: &SourceBundle) -> String {
    let samples = (1..=config.min_da_samples)
        .map(|ordinal| {
            json!({
                "ordinal": ordinal,
                "user_escape_state_root": &source.user_escape_state_root,
                "wallet_instruction_bundle_root": &source.wallet_instruction_bundle_root,
                "sample_root": domain_hash(
                    "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-DA-SAMPLE",
                    &[
                        HashPart::Str(&config.acceptance_packet_suite),
                        HashPart::Str(&source.user_escape_state_root),
                        HashPart::U64(ordinal),
                    ],
                    32,
                ),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-DA-SAMPLES",
        &samples,
    )
}

fn expected_bridge_root(source: &SourceBundle) -> String {
    record_root(
        "bridge-roots",
        &json!({
            "user_escape_state_root": &source.user_escape_state_root,
            "step_bundle_root": &source.step_bundle_root,
            "release_blocker_bundle_root": &source.release_blocker_bundle_root,
            "production_hold_root": &source.production_hold_root,
            "verdict_root": &source.verdict_root,
        }),
    )
}

fn validity_aggregate_root(config: &Config, source: &SourceBundle) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-VALIDITY-AGGREGATE",
        &[
            HashPart::Str(&config.acceptance_packet_suite),
            HashPart::Str(&source.user_escape_state_root),
            HashPart::U64(source.step_count),
            HashPart::U64(source.ready_step_count),
            HashPart::U64(source.blocked_step_count),
            HashPart::Str(bool_str(source.force_exit_safe_to_attempt)),
            HashPart::Str(&source.verdict_status),
        ],
        32,
    )
}

fn privacy_aggregate_root(config: &Config, source: &SourceBundle) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-PRIVACY-AGGREGATE",
        &[
            HashPart::Str(&config.acceptance_packet_suite),
            HashPart::Str(&source.wallet_instruction_bundle_root),
            HashPart::Str(&source.release_blocker_bundle_root),
            HashPart::U64(source.user_release_blocker_count),
            HashPart::Str(bool_str(source.wallet_escape_answerable)),
        ],
        32,
    )
}

fn packet_seed_root(
    config: &Config,
    source: &SourceBundle,
    da_sample_root: &str,
    bridge_root: &str,
    validity_aggregate_root: &str,
    privacy_aggregate_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-PACKET-SEED",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(da_sample_root),
            HashPart::Str(bridge_root),
            HashPart::Str(validity_aggregate_root),
            HashPart::Str(privacy_aggregate_root),
        ],
        32,
    )
}

fn audit_signatures(config: &Config, packet_seed_root: &str) -> Vec<AuditSignature> {
    [
        "watchtower:atlas",
        "watchtower:boreal",
        "watchtower:cygnus",
        "watchtower:drift",
    ]
    .iter()
    .enumerate()
    .map(|(index, watchtower_id)| {
        AuditSignature::devnet(config, packet_seed_root, index as u64 + 1, watchtower_id)
    })
    .collect()
}

fn challenge_reports(config: &Config, source: &SourceBundle) -> Vec<ChallengeReport> {
    (1..=config.max_challenge_reports)
        .map(|ordinal| ChallengeReport::devnet(config, source, ordinal))
        .collect()
}

fn audit_signature_root(signatures: &[AuditSignature]) -> String {
    merkle_root(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-SIGNATURES",
        &signatures
            .iter()
            .map(AuditSignature::public_record)
            .collect::<Vec<_>>(),
    )
}

fn challenge_report_root(reports: &[ChallengeReport]) -> String {
    merkle_root(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-CHALLENGE-REPORTS",
        &reports
            .iter()
            .map(ChallengeReport::public_record)
            .collect::<Vec<_>>(),
    )
}

fn counters(
    config: &Config,
    signatures: &[AuditSignature],
    challenge_reports: &[ChallengeReport],
) -> Counters {
    Counters {
        da_sample_count: config.min_da_samples,
        bridge_root_count: 1,
        audit_signature_count: signatures.len() as u64,
        accepting_watchtower_count: signatures
            .iter()
            .filter(|signature| signature.accepted)
            .count() as u64,
        challenge_report_count: challenge_reports.len() as u64,
        blocking_challenge_count: challenge_reports
            .iter()
            .filter(|report| report.blocks_acceptance)
            .count() as u64,
    }
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    roots: &Roots,
    counters: &Counters,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-ACCEPTANCE-PACKET-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(&roots.da_sample_root),
            HashPart::Str(&roots.bridge_root),
            HashPart::Str(&roots.validity_aggregate_root),
            HashPart::Str(&roots.privacy_aggregate_root),
            HashPart::Str(&roots.audit_signature_root),
            HashPart::Str(&roots.challenge_report_root),
            HashPart::Str(&roots.acceptance_verdict_root),
            HashPart::U64(counters.audit_signature_count),
            HashPart::U64(counters.blocking_challenge_count),
        ],
        32,
    )
}

fn acceptance_verdict_root(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    roots: &Roots,
    status: AuditPacketStatus,
    da_samples_satisfied: bool,
    bridge_roots_match: bool,
    validity_accepted: bool,
    privacy_accepted: bool,
    signature_quorum_met: bool,
    challenges_clear: bool,
    production_blocked: bool,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-ACCEPTANCE-VERDICT",
        &[
            HashPart::Str(&config.acceptance_packet_suite),
            HashPart::Str(&source.user_escape_state_root),
            HashPart::Str(&roots.da_sample_root),
            HashPart::Str(&roots.bridge_root),
            HashPart::Str(&roots.validity_aggregate_root),
            HashPart::Str(&roots.privacy_aggregate_root),
            HashPart::U64(counters.da_sample_count),
            HashPart::U64(counters.audit_signature_count),
            HashPart::U64(counters.accepting_watchtower_count),
            HashPart::U64(counters.challenge_report_count),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(da_samples_satisfied)),
            HashPart::Str(bool_str(bridge_roots_match)),
            HashPart::Str(bool_str(validity_accepted)),
            HashPart::Str(bool_str(privacy_accepted)),
            HashPart::Str(bool_str(signature_quorum_met)),
            HashPart::Str(bool_str(challenges_clear)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "watchtower audit acceptance packet chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "watchtower audit acceptance packet protocol mismatch",
    )?;
    ensure(
        config.min_da_samples > 0,
        "watchtower audit acceptance packet requires DA samples",
    )?;
    ensure(
        config.min_watchtower_signatures > 0,
        "watchtower audit acceptance packet requires watchtower signatures",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.user_escape_state_root.is_empty(),
        "watchtower audit acceptance packet missing user escape state root",
    )?;
    ensure(
        !source.verdict_root.is_empty(),
        "watchtower audit acceptance packet missing verdict root",
    )?;
    ensure(
        source.step_count > 0,
        "watchtower audit acceptance packet missing user escape steps",
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
        user_escape_state_root: record_root(
            "fallback-user-escape-state",
            &json!({"reason": &reason}),
        ),
        step_bundle_root: record_root("fallback-step-bundle", &json!({"reason": &reason})),
        wallet_instruction_bundle_root: record_root(
            "fallback-wallet-instruction-bundle",
            &json!({"reason": &reason}),
        ),
        release_blocker_bundle_root: record_root(
            "fallback-release-blocker-bundle",
            &json!({"reason": &reason}),
        ),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        verdict_root: record_root("fallback-verdict", &json!({"reason": &reason})),
        step_count: 1,
        ready_step_count: 0,
        waiting_step_count: 0,
        blocked_step_count: 1,
        user_release_blocker_count: 1,
        production_blocker_count: 1,
        wallet_escape_answerable: false,
        force_exit_safe_to_attempt: false,
        production_release_allowed: false,
        verdict_status: "fallback".to_string(),
    };
    let da_sample_root = da_sample_root(&config, &source);
    let bridge_root = expected_bridge_root(&source);
    let validity_aggregate_root = validity_aggregate_root(&config, &source);
    let privacy_aggregate_root = privacy_aggregate_root(&config, &source);
    let packet_seed_root = packet_seed_root(
        &config,
        &source,
        &da_sample_root,
        &bridge_root,
        &validity_aggregate_root,
        &privacy_aggregate_root,
    );
    let audit_signatures = audit_signatures(&config, &packet_seed_root);
    let challenge_reports = vec![ChallengeReport {
        report_id: record_root("fallback-challenge-report-id", &json!({"reason": &reason})),
        ordinal: 1,
        challenge_root: record_root("fallback-challenge-report", &json!({"reason": &reason})),
        resolved: false,
        blocks_acceptance: true,
    }];
    let counters = counters(&config, &audit_signatures, &challenge_reports);
    let mut roots = Roots {
        da_sample_root,
        bridge_root,
        validity_aggregate_root,
        privacy_aggregate_root,
        audit_signature_root: audit_signature_root(&audit_signatures),
        challenge_report_root: challenge_report_root(&challenge_reports),
        acceptance_verdict_root: String::new(),
        state_commitment_root: String::new(),
    };
    let verdict = AcceptanceVerdict::new(&config, &source, &counters, &roots);
    roots.acceptance_verdict_root = verdict.verdict_root.clone();
    roots.state_commitment_root = state_commitment_root(&config, &source, &roots, &counters);
    State {
        config,
        source,
        roots,
        counters,
        audit_signatures,
        challenge_reports,
        verdict,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WATCHTOWER-AUDIT-ACCEPTANCE-PACKET-RECORD",
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
