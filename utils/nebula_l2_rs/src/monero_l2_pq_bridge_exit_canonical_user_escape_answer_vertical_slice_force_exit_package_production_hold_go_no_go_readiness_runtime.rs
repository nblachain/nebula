use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageProductionHoldGoNoGoReadinessRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PRODUCTION_HOLD_GO_NO_GO_READINESS_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-production-hold-go-no-go-readiness-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PRODUCTION_HOLD_GO_NO_GO_READINESS_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const READINESS_SUITE: &str =
    "monero-l2-pq-force-exit-package-production-hold-go-no-go-readiness-v1";
pub const DEFAULT_MIN_ACCEPTANCE_TRACKS: u64 = 8;
pub const DEFAULT_MIN_LIVE_EVIDENCE_PACKETS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub readiness_suite: String,
    pub min_acceptance_tracks: u64,
    pub min_live_evidence_packets: u64,
    pub require_compile_gate: bool,
    pub require_runtime_gate: bool,
    pub require_security_privacy_gate: bool,
    pub require_bridge_custody_gate: bool,
    pub require_wallet_recovery_gate: bool,
    pub require_watchtower_gate: bool,
    pub require_reserve_gate: bool,
    pub require_pq_quorum_gate: bool,
    pub fail_closed_on_deferred_gate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            readiness_suite: READINESS_SUITE.to_string(),
            min_acceptance_tracks: DEFAULT_MIN_ACCEPTANCE_TRACKS,
            min_live_evidence_packets: DEFAULT_MIN_LIVE_EVIDENCE_PACKETS,
            require_compile_gate: true,
            require_runtime_gate: true,
            require_security_privacy_gate: true,
            require_bridge_custody_gate: true,
            require_wallet_recovery_gate: true,
            require_watchtower_gate: true,
            require_reserve_gate: true,
            require_pq_quorum_gate: true,
            fail_closed_on_deferred_gate: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_track_count(&self) -> u64 {
        [
            self.require_compile_gate,
            self.require_runtime_gate,
            self.require_security_privacy_gate,
            self.require_bridge_custody_gate,
            self.require_wallet_recovery_gate,
            self.require_watchtower_gate,
            self.require_reserve_gate,
            self.require_pq_quorum_gate,
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
            "readiness_suite": self.readiness_suite,
            "min_acceptance_tracks": self.min_acceptance_tracks,
            "min_live_evidence_packets": self.min_live_evidence_packets,
            "required_track_count": self.required_track_count(),
            "require_compile_gate": self.require_compile_gate,
            "require_runtime_gate": self.require_runtime_gate,
            "require_security_privacy_gate": self.require_security_privacy_gate,
            "require_bridge_custody_gate": self.require_bridge_custody_gate,
            "require_wallet_recovery_gate": self.require_wallet_recovery_gate,
            "require_watchtower_gate": self.require_watchtower_gate,
            "require_reserve_gate": self.require_reserve_gate,
            "require_pq_quorum_gate": self.require_pq_quorum_gate,
            "fail_closed_on_deferred_gate": self.fail_closed_on_deferred_gate,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessTrackKind {
    CompileGate,
    RuntimeExecution,
    SecurityPrivacyAudit,
    BridgeCustodyDrill,
    WalletRecoveryRelease,
    WatchtowerAcceptance,
    ReserveLiquidity,
    PqQuorumRotation,
}

impl ReadinessTrackKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileGate => "compile_gate",
            Self::RuntimeExecution => "runtime_execution",
            Self::SecurityPrivacyAudit => "security_privacy_audit",
            Self::BridgeCustodyDrill => "bridge_custody_drill",
            Self::WalletRecoveryRelease => "wallet_recovery_release",
            Self::WatchtowerAcceptance => "watchtower_acceptance",
            Self::ReserveLiquidity => "reserve_liquidity",
            Self::PqQuorumRotation => "pq_quorum_rotation",
        }
    }

    pub fn all_required() -> Vec<Self> {
        vec![
            Self::CompileGate,
            Self::RuntimeExecution,
            Self::SecurityPrivacyAudit,
            Self::BridgeCustodyDrill,
            Self::WalletRecoveryRelease,
            Self::WatchtowerAcceptance,
            Self::ReserveLiquidity,
            Self::PqQuorumRotation,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Passed,
    Deferred,
    Blocked,
}

impl GateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Deferred => "deferred",
            Self::Blocked => "blocked",
        }
    }

    pub fn permits_production(self) -> bool {
        matches!(self, Self::Passed)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidencePacket {
    pub packet_id: String,
    pub track: ReadinessTrackKind,
    pub source_runtime: String,
    pub source_state_root: String,
    pub evidence_root: String,
    pub observed_at_height: u64,
    pub reviewer_commitment: String,
    pub live_observation: bool,
    pub status: GateStatus,
    pub hold_reason: String,
}

impl EvidencePacket {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        track: ReadinessTrackKind,
        source_runtime: &str,
        source_state_root: &str,
        evidence_root: &str,
        observed_at_height: u64,
        reviewer_commitment: &str,
        live_observation: bool,
        status: GateStatus,
        hold_reason: &str,
    ) -> Self {
        let packet_id = evidence_packet_id(
            track,
            source_runtime,
            source_state_root,
            evidence_root,
            observed_at_height,
            reviewer_commitment,
            live_observation,
            status,
            hold_reason,
        );
        Self {
            packet_id,
            track,
            source_runtime: source_runtime.to_string(),
            source_state_root: source_state_root.to_string(),
            evidence_root: evidence_root.to_string(),
            observed_at_height,
            reviewer_commitment: reviewer_commitment.to_string(),
            live_observation,
            status,
            hold_reason: hold_reason.to_string(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.status.permits_production() && self.live_observation
    }

    pub fn public_record(&self) -> Value {
        json!({
            "packet_id": self.packet_id,
            "track": self.track.as_str(),
            "source_runtime": self.source_runtime,
            "source_state_root": self.source_state_root,
            "evidence_root": self.evidence_root,
            "observed_at_height": self.observed_at_height,
            "reviewer_commitment": self.reviewer_commitment,
            "live_observation": self.live_observation,
            "status": self.status.as_str(),
            "hold_reason": self.hold_reason,
            "ready": self.is_ready(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub total_tracks: u64,
    pub live_tracks: u64,
    pub passed_tracks: u64,
    pub deferred_tracks: u64,
    pub blocked_tracks: u64,
    pub required_tracks: u64,
}

impl Counters {
    pub fn from_packets(config: &Config, packets: &[EvidencePacket]) -> Self {
        Self {
            total_tracks: packets.len() as u64,
            live_tracks: packets
                .iter()
                .filter(|packet| packet.live_observation)
                .count() as u64,
            passed_tracks: packets
                .iter()
                .filter(|packet| packet.status == GateStatus::Passed)
                .count() as u64,
            deferred_tracks: packets
                .iter()
                .filter(|packet| packet.status == GateStatus::Deferred)
                .count() as u64,
            blocked_tracks: packets
                .iter()
                .filter(|packet| packet.status == GateStatus::Blocked)
                .count() as u64,
            required_tracks: config
                .required_track_count()
                .max(config.min_acceptance_tracks),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "total_tracks": self.total_tracks,
            "live_tracks": self.live_tracks,
            "passed_tracks": self.passed_tracks,
            "deferred_tracks": self.deferred_tracks,
            "blocked_tracks": self.blocked_tracks,
            "required_tracks": self.required_tracks,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessDecision {
    pub decision_id: String,
    pub ready_track_root: String,
    pub deferred_track_root: String,
    pub blocked_track_root: String,
    pub reason_root: String,
    pub production_go: bool,
    pub production_hold: bool,
    pub go_no_go_status: String,
    pub release_memo: String,
}

impl ReadinessDecision {
    pub fn from_parts(config: &Config, packets: &[EvidencePacket], counters: &Counters) -> Self {
        let ready_tracks = packets
            .iter()
            .filter(|packet| packet.is_ready())
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let deferred_tracks = packets
            .iter()
            .filter(|packet| packet.status == GateStatus::Deferred)
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let blocked_tracks = packets
            .iter()
            .filter(|packet| packet.status == GateStatus::Blocked)
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let hold_reasons = packets
            .iter()
            .filter(|packet| !packet.is_ready())
            .map(|packet| {
                json!({
                    "track": packet.track.as_str(),
                    "status": packet.status.as_str(),
                    "hold_reason": packet.hold_reason,
                    "live_observation": packet.live_observation,
                })
            })
            .collect::<Vec<_>>();

        let ready_track_root = merkle_root("PRODUCTION-READY-TRACK", &ready_tracks);
        let deferred_track_root = merkle_root("PRODUCTION-DEFERRED-TRACK", &deferred_tracks);
        let blocked_track_root = merkle_root("PRODUCTION-BLOCKED-TRACK", &blocked_tracks);
        let reason_root = merkle_root("PRODUCTION-HOLD-REASON", &hold_reasons);
        let enough_live_evidence = counters.live_tracks >= config.min_live_evidence_packets;
        let enough_ready_tracks = ready_tracks.len() as u64 >= counters.required_tracks;
        let no_blocked = counters.blocked_tracks == 0;
        let no_deferred = counters.deferred_tracks == 0 || !config.fail_closed_on_deferred_gate;
        let production_go =
            enough_live_evidence && enough_ready_tracks && no_blocked && no_deferred;
        let production_hold = !production_go;
        let go_no_go_status = if production_go {
            "go"
        } else if counters.blocked_tracks > 0 {
            "blocked_hold"
        } else {
            "deferred_hold"
        }
        .to_string();
        let release_memo = if production_go {
            "all required live drill evidence is present"
        } else {
            "production release remains held until deferred and non-live evidence is replaced by accepted live receipts"
        }
        .to_string();
        let decision_id = readiness_decision_id(
            &ready_track_root,
            &deferred_track_root,
            &blocked_track_root,
            &reason_root,
            counters,
            production_go,
            &go_no_go_status,
        );
        Self {
            decision_id,
            ready_track_root,
            deferred_track_root,
            blocked_track_root,
            reason_root,
            production_go,
            production_hold,
            go_no_go_status,
            release_memo,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "decision_id": self.decision_id,
            "ready_track_root": self.ready_track_root,
            "deferred_track_root": self.deferred_track_root,
            "blocked_track_root": self.blocked_track_root,
            "reason_root": self.reason_root,
            "production_go": self.production_go,
            "production_hold": self.production_hold,
            "go_no_go_status": self.go_no_go_status,
            "release_memo": self.release_memo,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub evidence_packet_root: String,
    pub live_evidence_root: String,
    pub deferred_evidence_root: String,
    pub decision_root: String,
    pub production_hold_root: String,
    pub state_root: String,
}

impl Roots {
    pub fn from_parts(
        config: &Config,
        packets: &[EvidencePacket],
        counters: &Counters,
        decision: &ReadinessDecision,
    ) -> Self {
        let config_record = config.public_record();
        let packet_records = packets
            .iter()
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let live_records = packets
            .iter()
            .filter(|packet| packet.live_observation)
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let deferred_records = packets
            .iter()
            .filter(|packet| packet.status == GateStatus::Deferred)
            .map(EvidencePacket::public_record)
            .collect::<Vec<_>>();
        let counters_record = counters.public_record();
        let decision_record = decision.public_record();
        let config_root = record_root("config", &config_record);
        let evidence_packet_root = merkle_root("PRODUCTION-READINESS-EVIDENCE", &packet_records);
        let live_evidence_root = merkle_root("PRODUCTION-LIVE-EVIDENCE", &live_records);
        let deferred_evidence_root = merkle_root("PRODUCTION-DEFERRED-EVIDENCE", &deferred_records);
        let decision_root = record_root("decision", &decision_record);
        let production_hold_root = record_root(
            "production-hold",
            &json!({
                "production_hold": decision.production_hold,
                "go_no_go_status": decision.go_no_go_status,
                "reason_root": decision.reason_root,
                "deferred_evidence_root": deferred_evidence_root,
            }),
        );
        let state_root = record_root(
            "state",
            &json!({
                "config_root": config_root,
                "evidence_packet_root": evidence_packet_root,
                "live_evidence_root": live_evidence_root,
                "deferred_evidence_root": deferred_evidence_root,
                "counters": counters_record,
                "decision_root": decision_root,
                "production_hold_root": production_hold_root,
            }),
        );
        Self {
            config_root,
            evidence_packet_root,
            live_evidence_root,
            deferred_evidence_root,
            decision_root,
            production_hold_root,
            state_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "evidence_packet_root": self.evidence_packet_root,
            "live_evidence_root": self.live_evidence_root,
            "deferred_evidence_root": self.deferred_evidence_root,
            "decision_root": self.decision_root,
            "production_hold_root": self.production_hold_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub evidence_packets: Vec<EvidencePacket>,
    pub counters: Counters,
    pub decision: ReadinessDecision,
    pub roots: Roots,
}

impl State {
    pub fn new(config: Config, evidence_packets: Vec<EvidencePacket>) -> Self {
        let counters = Counters::from_packets(&config, &evidence_packets);
        let decision = ReadinessDecision::from_parts(&config, &evidence_packets, &counters);
        let roots = Roots::from_parts(&config, &evidence_packets, &counters, &decision);
        Self {
            config,
            evidence_packets,
            counters,
            decision,
            roots,
        }
    }

    pub fn devnet() -> Self {
        Self::new(Config::devnet(), devnet_evidence_packets())
    }

    pub fn production_go(&self) -> bool {
        self.decision.production_go
    }

    pub fn production_hold(&self) -> bool {
        self.decision.production_hold
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "evidence_packets": self
                .evidence_packets
                .iter()
                .map(EvidencePacket::public_record)
                .collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "decision": self.decision.public_record(),
            "roots": self.roots.public_record(),
            "production_go": self.production_go(),
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

fn devnet_evidence_packets() -> Vec<EvidencePacket> {
    vec![
        EvidencePacket::new(
            ReadinessTrackKind::CompileGate,
            "cargo-check-test-clippy",
            &fixture_root("compile-state", "cargo checks intentionally deferred"),
            &fixture_root(
                "compile-evidence",
                "no cargo or rustc gate run in this wave",
            ),
            75,
            &fixture_root("reviewer", "compile-gate-reviewer"),
            false,
            GateStatus::Deferred,
            "cargo, test, and clippy gates are intentionally deferred",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::RuntimeExecution,
            "force-exit-closure-runtime-execution",
            &fixture_root(
                "runtime-state",
                "closure bundle not executed against live devnet",
            ),
            &fixture_root("runtime-evidence", "runtime execution deferred"),
            75,
            &fixture_root("reviewer", "runtime-gate-reviewer"),
            false,
            GateStatus::Deferred,
            "closure bundle has deterministic roots but no live runtime execution receipt",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::SecurityPrivacyAudit,
            "security-privacy-review",
            &fixture_root("audit-state", "audit packet pending"),
            &fixture_root("audit-evidence", "manual audit not complete"),
            75,
            &fixture_root("reviewer", "security-privacy-reviewer"),
            false,
            GateStatus::Deferred,
            "security and privacy controls need adversarial review before release",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::BridgeCustodyDrill,
            "live-settlement-drill-evidence",
            &fixture_root("bridge-state", "settlement drill packet prepared"),
            &fixture_root("bridge-evidence", "custody release live receipt pending"),
            75,
            &fixture_root("reviewer", "bridge-custody-reviewer"),
            true,
            GateStatus::Deferred,
            "live bridge custody release evidence is not accepted yet",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::WalletRecoveryRelease,
            "wallet-recovery-release-drill",
            &fixture_root("wallet-state", "wallet recovery release drill prepared"),
            &fixture_root("wallet-evidence", "wallet transcript acceptance pending"),
            75,
            &fixture_root("reviewer", "wallet-recovery-reviewer"),
            true,
            GateStatus::Deferred,
            "wallet recovery release transcript still needs accepted live replay",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::WatchtowerAcceptance,
            "watchtower-audit-acceptance-packet",
            &fixture_root("watchtower-state", "watchtower packet prepared"),
            &fixture_root("watchtower-evidence", "watchtower acceptance pending"),
            75,
            &fixture_root("reviewer", "watchtower-reviewer"),
            true,
            GateStatus::Deferred,
            "watchtower acceptance packet must be signed over live samples",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::ReserveLiquidity,
            "reserve-liquidity-reconciliation-drill",
            &fixture_root("reserve-state", "reserve reconciliation prepared"),
            &fixture_root("reserve-evidence", "reserve drill pending accepted report"),
            75,
            &fixture_root("reviewer", "reserve-reviewer"),
            true,
            GateStatus::Deferred,
            "reserve and withdrawal-liquidity reconciliation still needs live acceptance",
        ),
        EvidencePacket::new(
            ReadinessTrackKind::PqQuorumRotation,
            "pq-quorum-rotation-drill-receipt",
            &fixture_root("pq-state", "pq quorum rotation prepared"),
            &fixture_root("pq-evidence", "rotation activation pending"),
            75,
            &fixture_root("reviewer", "pq-quorum-reviewer"),
            true,
            GateStatus::Deferred,
            "PQ quorum rotation receipt must be activated under live timelock evidence",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn evidence_packet_id(
    track: ReadinessTrackKind,
    source_runtime: &str,
    source_state_root: &str,
    evidence_root: &str,
    observed_at_height: u64,
    reviewer_commitment: &str,
    live_observation: bool,
    status: GateStatus,
    hold_reason: &str,
) -> String {
    domain_hash(
        "PRODUCTION-READINESS-EVIDENCE-PACKET-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(track.as_str()),
            HashPart::Str(source_runtime),
            HashPart::Str(source_state_root),
            HashPart::Str(evidence_root),
            HashPart::U64(observed_at_height),
            HashPart::Str(reviewer_commitment),
            HashPart::Str(if live_observation { "live" } else { "not-live" }),
            HashPart::Str(status.as_str()),
            HashPart::Str(hold_reason),
        ],
        32,
    )
}

pub fn readiness_decision_id(
    ready_track_root: &str,
    deferred_track_root: &str,
    blocked_track_root: &str,
    reason_root: &str,
    counters: &Counters,
    production_go: bool,
    go_no_go_status: &str,
) -> String {
    let counter_record = counters.public_record();
    domain_hash(
        "PRODUCTION-READINESS-DECISION-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(ready_track_root),
            HashPart::Str(deferred_track_root),
            HashPart::Str(blocked_track_root),
            HashPart::Str(reason_root),
            HashPart::Json(&counter_record),
            HashPart::Str(if production_go { "go" } else { "hold" }),
            HashPart::Str(go_no_go_status),
        ],
        32,
    )
}

pub fn fixture_root(kind: &str, value: &str) -> String {
    domain_hash(
        "PRODUCTION-READINESS-FIXTURE",
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
        "PRODUCTION-READINESS-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}
