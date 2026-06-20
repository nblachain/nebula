use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageHeavyGateEvidenceReplacementManifestRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_HEAVY_GATE_EVIDENCE_REPLACEMENT_MANIFEST_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-heavy-gate-evidence-replacement-manifest-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_HEAVY_GATE_EVIDENCE_REPLACEMENT_MANIFEST_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const MANIFEST_SUITE: &str =
    "monero-l2-pq-force-exit-package-heavy-gate-evidence-replacement-manifest-v1";
pub const DEFAULT_MIN_LIVE_REPLACEMENTS: u64 = 6;
pub const DEFAULT_MIN_REVIEWER_PACKETS: u64 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub manifest_suite: String,
    pub min_live_replacements: u64,
    pub min_reviewer_packets: u64,
    pub require_compile_replacement: bool,
    pub require_runtime_replacement: bool,
    pub require_security_privacy_replacement: bool,
    pub require_bridge_custody_replacement: bool,
    pub require_wallet_transcript_replacement: bool,
    pub require_watchtower_replacement: bool,
    pub production_hold_on_any_deferred_root: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            manifest_suite: MANIFEST_SUITE.to_string(),
            min_live_replacements: DEFAULT_MIN_LIVE_REPLACEMENTS,
            min_reviewer_packets: DEFAULT_MIN_REVIEWER_PACKETS,
            require_compile_replacement: true,
            require_runtime_replacement: true,
            require_security_privacy_replacement: true,
            require_bridge_custody_replacement: true,
            require_wallet_transcript_replacement: true,
            require_watchtower_replacement: true,
            production_hold_on_any_deferred_root: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_replacement_count(&self) -> u64 {
        [
            self.require_compile_replacement,
            self.require_runtime_replacement,
            self.require_security_privacy_replacement,
            self.require_bridge_custody_replacement,
            self.require_wallet_transcript_replacement,
            self.require_watchtower_replacement,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "manifest_suite": self.manifest_suite,
            "min_live_replacements": self.min_live_replacements,
            "min_reviewer_packets": self.min_reviewer_packets,
            "required_replacement_count": self.required_replacement_count(),
            "require_compile_replacement": self.require_compile_replacement,
            "require_runtime_replacement": self.require_runtime_replacement,
            "require_security_privacy_replacement": self.require_security_privacy_replacement,
            "require_bridge_custody_replacement": self.require_bridge_custody_replacement,
            "require_wallet_transcript_replacement": self.require_wallet_transcript_replacement,
            "require_watchtower_replacement": self.require_watchtower_replacement,
            "production_hold_on_any_deferred_root": self.production_hold_on_any_deferred_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementLane {
    CompileHeavyGate,
    RuntimeReplay,
    SecurityPrivacyAudit,
    BridgeCustodyRelease,
    WalletTranscriptAcceptance,
    WatchtowerChallengeReplay,
}

impl ReplacementLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileHeavyGate => "compile_heavy_gate",
            Self::RuntimeReplay => "runtime_replay",
            Self::SecurityPrivacyAudit => "security_privacy_audit",
            Self::BridgeCustodyRelease => "bridge_custody_release",
            Self::WalletTranscriptAcceptance => "wallet_transcript_acceptance",
            Self::WatchtowerChallengeReplay => "watchtower_challenge_replay",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementStatus {
    LiveAccepted,
    ReviewerPending,
    DeferredFixture,
    Rejected,
}

impl ReplacementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveAccepted => "live_accepted",
            Self::ReviewerPending => "reviewer_pending",
            Self::DeferredFixture => "deferred_fixture",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_live_accepted(self) -> bool {
        matches!(self, Self::LiveAccepted)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplacementRecord {
    pub replacement_id: String,
    pub lane: ReplacementLane,
    pub adapter_runtime: String,
    pub deferred_root: String,
    pub live_receipt_root: String,
    pub reviewer_packet_root: String,
    pub acceptance_height: u64,
    pub status: ReplacementStatus,
    pub production_hold_reason: String,
}

impl ReplacementRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lane: ReplacementLane,
        adapter_runtime: &str,
        deferred_root: &str,
        live_receipt_root: &str,
        reviewer_packet_root: &str,
        acceptance_height: u64,
        status: ReplacementStatus,
        production_hold_reason: &str,
    ) -> Self {
        let replacement_id = replacement_id(
            lane,
            adapter_runtime,
            deferred_root,
            live_receipt_root,
            reviewer_packet_root,
            acceptance_height,
            status,
            production_hold_reason,
        );
        Self {
            replacement_id,
            lane,
            adapter_runtime: adapter_runtime.to_string(),
            deferred_root: deferred_root.to_string(),
            live_receipt_root: live_receipt_root.to_string(),
            reviewer_packet_root: reviewer_packet_root.to_string(),
            acceptance_height,
            status,
            production_hold_reason: production_hold_reason.to_string(),
        }
    }

    pub fn is_live_accepted(&self) -> bool {
        self.status.is_live_accepted()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "replacement_id": self.replacement_id,
            "lane": self.lane.as_str(),
            "adapter_runtime": self.adapter_runtime,
            "deferred_root": self.deferred_root,
            "live_receipt_root": self.live_receipt_root,
            "reviewer_packet_root": self.reviewer_packet_root,
            "acceptance_height": self.acceptance_height,
            "status": self.status.as_str(),
            "production_hold_reason": self.production_hold_reason,
            "live_accepted": self.is_live_accepted(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub total_replacements: u64,
    pub live_accepted_replacements: u64,
    pub reviewer_pending_replacements: u64,
    pub deferred_fixture_replacements: u64,
    pub rejected_replacements: u64,
    pub reviewer_packets: u64,
    pub required_replacements: u64,
}

impl Counters {
    pub fn from_records(config: &Config, records: &[ReplacementRecord]) -> Self {
        Self {
            total_replacements: records.len() as u64,
            live_accepted_replacements: records
                .iter()
                .filter(|record| record.status == ReplacementStatus::LiveAccepted)
                .count() as u64,
            reviewer_pending_replacements: records
                .iter()
                .filter(|record| record.status == ReplacementStatus::ReviewerPending)
                .count() as u64,
            deferred_fixture_replacements: records
                .iter()
                .filter(|record| record.status == ReplacementStatus::DeferredFixture)
                .count() as u64,
            rejected_replacements: records
                .iter()
                .filter(|record| record.status == ReplacementStatus::Rejected)
                .count() as u64,
            reviewer_packets: records
                .iter()
                .filter(|record| !record.reviewer_packet_root.is_empty())
                .count() as u64,
            required_replacements: config
                .required_replacement_count()
                .max(config.min_live_replacements),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "total_replacements": self.total_replacements,
            "live_accepted_replacements": self.live_accepted_replacements,
            "reviewer_pending_replacements": self.reviewer_pending_replacements,
            "deferred_fixture_replacements": self.deferred_fixture_replacements,
            "rejected_replacements": self.rejected_replacements,
            "reviewer_packets": self.reviewer_packets,
            "required_replacements": self.required_replacements,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ManifestVerdict {
    pub verdict_id: String,
    pub replacement_record_root: String,
    pub live_accepted_root: String,
    pub deferred_fixture_root: String,
    pub reviewer_packet_root: String,
    pub hold_reason_root: String,
    pub all_required_replaced: bool,
    pub production_release_allowed: bool,
    pub production_hold: bool,
    pub status: String,
}

impl ManifestVerdict {
    pub fn from_parts(config: &Config, records: &[ReplacementRecord], counters: &Counters) -> Self {
        let replacement_records = records
            .iter()
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let live_records = records
            .iter()
            .filter(|record| record.is_live_accepted())
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let deferred_records = records
            .iter()
            .filter(|record| record.status == ReplacementStatus::DeferredFixture)
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let reviewer_packets = records
            .iter()
            .map(|record| {
                json!({
                    "lane": record.lane.as_str(),
                    "reviewer_packet_root": record.reviewer_packet_root,
                    "status": record.status.as_str(),
                })
            })
            .collect::<Vec<_>>();
        let hold_reasons = records
            .iter()
            .filter(|record| !record.is_live_accepted())
            .map(|record| {
                json!({
                    "lane": record.lane.as_str(),
                    "status": record.status.as_str(),
                    "production_hold_reason": record.production_hold_reason,
                })
            })
            .collect::<Vec<_>>();

        let replacement_record_root =
            merkle_root("HEAVY-GATE-REPLACEMENT-RECORD", &replacement_records);
        let live_accepted_root = merkle_root("HEAVY-GATE-LIVE-ACCEPTED", &live_records);
        let deferred_fixture_root = merkle_root("HEAVY-GATE-DEFERRED-FIXTURE", &deferred_records);
        let reviewer_packet_root = merkle_root("HEAVY-GATE-REVIEWER-PACKET", &reviewer_packets);
        let hold_reason_root = merkle_root("HEAVY-GATE-HOLD-REASON", &hold_reasons);
        let all_required_replaced =
            counters.live_accepted_replacements >= counters.required_replacements;
        let enough_reviewer_packets = counters.reviewer_packets >= config.min_reviewer_packets;
        let no_deferred = counters.deferred_fixture_replacements == 0
            || !config.production_hold_on_any_deferred_root;
        let no_rejected = counters.rejected_replacements == 0;
        let production_release_allowed =
            all_required_replaced && enough_reviewer_packets && no_deferred && no_rejected;
        let production_hold = !production_release_allowed;
        let status = if production_release_allowed {
            "release_allowed"
        } else if counters.rejected_replacements > 0 {
            "rejected_hold"
        } else if counters.deferred_fixture_replacements > 0 {
            "deferred_fixture_hold"
        } else {
            "reviewer_pending_hold"
        }
        .to_string();
        let verdict_id = manifest_verdict_id(
            &replacement_record_root,
            &live_accepted_root,
            &deferred_fixture_root,
            &reviewer_packet_root,
            &hold_reason_root,
            counters,
            production_release_allowed,
            &status,
        );
        Self {
            verdict_id,
            replacement_record_root,
            live_accepted_root,
            deferred_fixture_root,
            reviewer_packet_root,
            hold_reason_root,
            all_required_replaced,
            production_release_allowed,
            production_hold,
            status,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "replacement_record_root": self.replacement_record_root,
            "live_accepted_root": self.live_accepted_root,
            "deferred_fixture_root": self.deferred_fixture_root,
            "reviewer_packet_root": self.reviewer_packet_root,
            "hold_reason_root": self.hold_reason_root,
            "all_required_replaced": self.all_required_replaced,
            "production_release_allowed": self.production_release_allowed,
            "production_hold": self.production_hold,
            "status": self.status,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub replacement_record_root: String,
    pub live_accepted_root: String,
    pub deferred_fixture_root: String,
    pub reviewer_packet_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl Roots {
    pub fn from_parts(
        config: &Config,
        records: &[ReplacementRecord],
        counters: &Counters,
        verdict: &ManifestVerdict,
    ) -> Self {
        let config_record = config.public_record();
        let record_values = records
            .iter()
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let live_values = records
            .iter()
            .filter(|record| record.is_live_accepted())
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let deferred_values = records
            .iter()
            .filter(|record| record.status == ReplacementStatus::DeferredFixture)
            .map(ReplacementRecord::public_record)
            .collect::<Vec<_>>();
        let reviewer_values = records
            .iter()
            .map(|record| {
                json!({
                    "replacement_id": record.replacement_id,
                    "reviewer_packet_root": record.reviewer_packet_root,
                    "lane": record.lane.as_str(),
                })
            })
            .collect::<Vec<_>>();
        let counters_record = counters.public_record();
        let verdict_record = verdict.public_record();
        let config_root = record_root("config", &config_record);
        let replacement_record_root =
            merkle_root("HEAVY-GATE-MANIFEST-REPLACEMENT", &record_values);
        let live_accepted_root = merkle_root("HEAVY-GATE-MANIFEST-LIVE", &live_values);
        let deferred_fixture_root = merkle_root("HEAVY-GATE-MANIFEST-DEFERRED", &deferred_values);
        let reviewer_packet_root = merkle_root("HEAVY-GATE-MANIFEST-REVIEWER", &reviewer_values);
        let verdict_root = record_root("verdict", &verdict_record);
        let state_root = record_root(
            "state",
            &json!({
                "config_root": config_root,
                "replacement_record_root": replacement_record_root,
                "live_accepted_root": live_accepted_root,
                "deferred_fixture_root": deferred_fixture_root,
                "reviewer_packet_root": reviewer_packet_root,
                "counters": counters_record,
                "verdict_root": verdict_root,
            }),
        );
        Self {
            config_root,
            replacement_record_root,
            live_accepted_root,
            deferred_fixture_root,
            reviewer_packet_root,
            verdict_root,
            state_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "replacement_record_root": self.replacement_record_root,
            "live_accepted_root": self.live_accepted_root,
            "deferred_fixture_root": self.deferred_fixture_root,
            "reviewer_packet_root": self.reviewer_packet_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub replacement_records: Vec<ReplacementRecord>,
    pub counters: Counters,
    pub verdict: ManifestVerdict,
    pub roots: Roots,
}

impl State {
    pub fn new(config: Config, replacement_records: Vec<ReplacementRecord>) -> Self {
        let counters = Counters::from_records(&config, &replacement_records);
        let verdict = ManifestVerdict::from_parts(&config, &replacement_records, &counters);
        let roots = Roots::from_parts(&config, &replacement_records, &counters, &verdict);
        Self {
            config,
            replacement_records,
            counters,
            verdict,
            roots,
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet(), devnet_replacement_records())
    }

    pub fn production_hold(&self) -> bool {
        self.verdict.production_hold
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "replacement_records": self
                .replacement_records
                .iter()
                .map(ReplacementRecord::public_record)
                .collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "roots": self.roots.public_record(),
            "production_hold": self.production_hold(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root.clone()
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

fn devnet_replacement_records() -> Vec<ReplacementRecord> {
    vec![
        ReplacementRecord::new(
            ReplacementLane::CompileHeavyGate,
            "compile-heavy-gate-receipt-adapter",
            &fixture_root("compile-deferred", "cargo checks deferred"),
            &fixture_root("compile-live", "missing accepted cargo receipt"),
            &fixture_root("compile-reviewer", "compile reviewer packet pending"),
            76,
            ReplacementStatus::DeferredFixture,
            "compile, test, and clippy receipts are still deferred",
        ),
        ReplacementRecord::new(
            ReplacementLane::RuntimeReplay,
            "runtime-execution-replay-receipt-adapter",
            &fixture_root("runtime-deferred", "runtime execution deferred"),
            &fixture_root("runtime-live", "missing accepted replay receipt"),
            &fixture_root("runtime-reviewer", "runtime reviewer packet pending"),
            76,
            ReplacementStatus::DeferredFixture,
            "force-exit replay has not executed against captured live receipts",
        ),
        ReplacementRecord::new(
            ReplacementLane::SecurityPrivacyAudit,
            "security-privacy-audit-finding-closure",
            &fixture_root("audit-deferred", "security privacy audit deferred"),
            &fixture_root("audit-live", "missing accepted audit closure"),
            &fixture_root("audit-reviewer", "audit reviewer packet pending"),
            76,
            ReplacementStatus::ReviewerPending,
            "security and privacy finding closure requires reviewer acceptance",
        ),
        ReplacementRecord::new(
            ReplacementLane::BridgeCustodyRelease,
            "bridge-custody-live-release-receipt-verifier",
            &fixture_root("bridge-deferred", "bridge custody live receipt deferred"),
            &fixture_root("bridge-live", "live release receipt not accepted"),
            &fixture_root("bridge-reviewer", "bridge custody reviewer packet pending"),
            76,
            ReplacementStatus::ReviewerPending,
            "bridge custody release receipt must be matched to live Monero observations",
        ),
        ReplacementRecord::new(
            ReplacementLane::WalletTranscriptAcceptance,
            "wallet-transcript-live-acceptance-verifier",
            &fixture_root("wallet-deferred", "wallet transcript acceptance deferred"),
            &fixture_root("wallet-live", "wallet live acceptance missing"),
            &fixture_root("wallet-reviewer", "wallet reviewer packet pending"),
            76,
            ReplacementStatus::ReviewerPending,
            "wallet transcript acceptance must preserve private state continuity",
        ),
        ReplacementRecord::new(
            ReplacementLane::WatchtowerChallengeReplay,
            "watchtower-challenge-replay-acceptance",
            &fixture_root(
                "watchtower-deferred",
                "watchtower replay acceptance deferred",
            ),
            &fixture_root("watchtower-live", "watchtower live replay missing"),
            &fixture_root("watchtower-reviewer", "watchtower reviewer packet pending"),
            76,
            ReplacementStatus::ReviewerPending,
            "watchtower replay packet must be signed over live challenge evidence",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn replacement_id(
    lane: ReplacementLane,
    adapter_runtime: &str,
    deferred_root: &str,
    live_receipt_root: &str,
    reviewer_packet_root: &str,
    acceptance_height: u64,
    status: ReplacementStatus,
    production_hold_reason: &str,
) -> String {
    domain_hash(
        "HEAVY-GATE-REPLACEMENT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane.as_str()),
            HashPart::Str(adapter_runtime),
            HashPart::Str(deferred_root),
            HashPart::Str(live_receipt_root),
            HashPart::Str(reviewer_packet_root),
            HashPart::U64(acceptance_height),
            HashPart::Str(status.as_str()),
            HashPart::Str(production_hold_reason),
        ],
        32,
    )
}

pub fn manifest_verdict_id(
    replacement_record_root: &str,
    live_accepted_root: &str,
    deferred_fixture_root: &str,
    reviewer_packet_root: &str,
    hold_reason_root: &str,
    counters: &Counters,
    production_release_allowed: bool,
    status: &str,
) -> String {
    let counters_record = counters.public_record();
    domain_hash(
        "HEAVY-GATE-MANIFEST-VERDICT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(replacement_record_root),
            HashPart::Str(live_accepted_root),
            HashPart::Str(deferred_fixture_root),
            HashPart::Str(reviewer_packet_root),
            HashPart::Str(hold_reason_root),
            HashPart::Json(&counters_record),
            HashPart::Str(if production_release_allowed {
                "release-allowed"
            } else {
                "release-held"
            }),
            HashPart::Str(status),
        ],
        32,
    )
}

pub fn fixture_root(kind: &str, value: &str) -> String {
    domain_hash(
        "HEAVY-GATE-MANIFEST-FIXTURE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(value),
        ],
        32,
    )
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "HEAVY-GATE-MANIFEST-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}
