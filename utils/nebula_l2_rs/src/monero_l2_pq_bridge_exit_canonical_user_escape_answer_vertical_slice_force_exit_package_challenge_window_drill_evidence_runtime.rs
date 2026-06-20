use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_monitor_runtime as monitor,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageChallengeWindowDrillEvidenceRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CHALLENGE_WINDOW_DRILL_EVIDENCE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-challenge-window-drill-evidence-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_CHALLENGE_WINDOW_DRILL_EVIDENCE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const DRILL_EVIDENCE_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-challenge-window-drill-evidence-v1";
pub const DEFAULT_MIN_DRILL_CASES: u64 = 6;
pub const DEFAULT_MIN_DISPUTE_RECEIPTS: u64 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub drill_evidence_suite: String,
    pub min_drill_cases: u64,
    pub min_dispute_receipts: u64,
    pub require_window_opening_roots: bool,
    pub require_dispute_receipt_roots: bool,
    pub require_response_deadline_roots: bool,
    pub require_release_hold_roots: bool,
    pub require_replay_containment_roots: bool,
    pub require_fail_closed_acceptance: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            drill_evidence_suite: DRILL_EVIDENCE_SUITE.to_string(),
            min_drill_cases: DEFAULT_MIN_DRILL_CASES,
            min_dispute_receipts: DEFAULT_MIN_DISPUTE_RECEIPTS,
            require_window_opening_roots: true,
            require_dispute_receipt_roots: true,
            require_response_deadline_roots: true,
            require_release_hold_roots: true,
            require_replay_containment_roots: true,
            require_fail_closed_acceptance: true,
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
            "drill_evidence_suite": self.drill_evidence_suite,
            "min_drill_cases": self.min_drill_cases,
            "min_dispute_receipts": self.min_dispute_receipts,
            "require_window_opening_roots": self.require_window_opening_roots,
            "require_dispute_receipt_roots": self.require_dispute_receipt_roots,
            "require_response_deadline_roots": self.require_response_deadline_roots,
            "require_release_hold_roots": self.require_release_hold_roots,
            "require_replay_containment_roots": self.require_replay_containment_roots,
            "require_fail_closed_acceptance": self.require_fail_closed_acceptance,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub monitor_state_root: String,
    pub monitor_window_root: String,
    pub monitor_deadline_root: String,
    pub monitor_watcher_objection_root: String,
    pub monitor_fail_closed_hold_root: String,
    pub monitor_verdict_root: String,
    pub monitor_status: String,
    pub challenge_window_resolved: bool,
    pub fail_closed_hold_active: bool,
    pub monitor_production_blocked: bool,
    pub monitor_release_allowed: bool,
    pub watcher_objection_count: u64,
    pub sustained_objection_count: u64,
    pub opened_at_height: u64,
    pub challenge_deadline_height: u64,
    pub watcher_response_deadline_height: u64,
    pub user_escape_deadline_height: u64,
}

impl SourceBundle {
    pub fn from_monitor(state: &monitor::State) -> Self {
        Self {
            monitor_state_root: state.state_root(),
            monitor_window_root: state.monitor.monitor_root.clone(),
            monitor_deadline_root: state.deadline_bundle_root.clone(),
            monitor_watcher_objection_root: state.watcher_objection_bundle_root.clone(),
            monitor_fail_closed_hold_root: state.fail_closed_hold_root.clone(),
            monitor_verdict_root: state.verdict.verdict_root.clone(),
            monitor_status: state.monitor.status.as_str().to_string(),
            challenge_window_resolved: state.verdict.challenge_window_resolved,
            fail_closed_hold_active: state.verdict.fail_closed_hold_active,
            monitor_production_blocked: state.verdict.production_blocked,
            monitor_release_allowed: state.verdict.release_allowed,
            watcher_objection_count: state.verdict.watcher_objection_count,
            sustained_objection_count: state.verdict.sustained_objection_count,
            opened_at_height: state.monitor.opened_at_height,
            challenge_deadline_height: state.monitor.challenge_deadline_height,
            watcher_response_deadline_height: state.monitor.watcher_response_deadline_height,
            user_escape_deadline_height: state.monitor.user_escape_deadline_height,
        }
    }

    pub fn devnet() -> Self {
        Self::from_monitor(&monitor::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "monitor_state_root": self.monitor_state_root,
            "monitor_window_root": self.monitor_window_root,
            "monitor_deadline_root": self.monitor_deadline_root,
            "monitor_watcher_objection_root": self.monitor_watcher_objection_root,
            "monitor_fail_closed_hold_root": self.monitor_fail_closed_hold_root,
            "monitor_verdict_root": self.monitor_verdict_root,
            "monitor_status": self.monitor_status,
            "challenge_window_resolved": self.challenge_window_resolved,
            "fail_closed_hold_active": self.fail_closed_hold_active,
            "monitor_production_blocked": self.monitor_production_blocked,
            "monitor_release_allowed": self.monitor_release_allowed,
            "watcher_objection_count": self.watcher_objection_count,
            "sustained_objection_count": self.sustained_objection_count,
            "opened_at_height": self.opened_at_height,
            "challenge_deadline_height": self.challenge_deadline_height,
            "watcher_response_deadline_height": self.watcher_response_deadline_height,
            "user_escape_deadline_height": self.user_escape_deadline_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillEvidenceKind {
    WindowOpening,
    DisputeReceipt,
    ResponseDeadline,
    ReleaseHold,
    ReplayContainment,
    FailClosedAcceptance,
}

impl DrillEvidenceKind {
    pub fn ordered() -> [Self; 6] {
        [
            Self::WindowOpening,
            Self::DisputeReceipt,
            Self::ResponseDeadline,
            Self::ReleaseHold,
            Self::ReplayContainment,
            Self::FailClosedAcceptance,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WindowOpening => "window_opening",
            Self::DisputeReceipt => "dispute_receipt",
            Self::ResponseDeadline => "response_deadline",
            Self::ReleaseHold => "release_hold",
            Self::ReplayContainment => "replay_containment",
            Self::FailClosedAcceptance => "fail_closed_acceptance",
        }
    }

    pub fn evidence_statement(self) -> &'static str {
        match self {
            Self::WindowOpening => "challenge window opening is bound to the monitor state root",
            Self::DisputeReceipt => {
                "watcher dispute receipts are retained for every objection lane"
            }
            Self::ResponseDeadline => "response deadlines are deterministic and replayable",
            Self::ReleaseHold => {
                "release hold remains active while the drill has unresolved evidence"
            }
            Self::ReplayContainment => "replay attempts are contained to this challenge window",
            Self::FailClosedAcceptance => {
                "acceptance fails closed unless all drill gates are clean"
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrillEvidence {
    pub evidence_id: String,
    pub ordinal: u64,
    pub evidence_kind: DrillEvidenceKind,
    pub window_opening_root: String,
    pub dispute_receipt_root: String,
    pub response_deadline_root: String,
    pub release_hold_root: String,
    pub replay_containment_root: String,
    pub fail_closed_acceptance_root: String,
    pub evidence_root: String,
    pub accepted: bool,
    pub release_blocked: bool,
    pub dispute_receipt_count: u64,
    pub evidence_statement: String,
}

impl DrillEvidence {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        evidence_kind: DrillEvidenceKind,
        ordinal: u64,
    ) -> Self {
        let dispute_receipt_count = dispute_receipt_count(source, evidence_kind);
        let accepted = drill_case_accepted(config, source, evidence_kind, dispute_receipt_count);
        let release_blocked =
            !accepted || source.fail_closed_hold_active || source.monitor_production_blocked;
        let window_opening_root = window_opening_root(config, source, evidence_kind, ordinal);
        let dispute_receipt_root = dispute_receipt_root(
            config,
            source,
            evidence_kind,
            ordinal,
            dispute_receipt_count,
        );
        let response_deadline_root = response_deadline_root(config, source, evidence_kind, ordinal);
        let release_hold_root =
            release_hold_root(config, source, evidence_kind, ordinal, release_blocked);
        let replay_containment_root =
            replay_containment_root(config, source, evidence_kind, ordinal, release_blocked);
        let fail_closed_acceptance_root =
            fail_closed_acceptance_root(config, source, evidence_kind, ordinal, accepted);
        let evidence_root = drill_evidence_root(
            config,
            source,
            evidence_kind,
            ordinal,
            &window_opening_root,
            &dispute_receipt_root,
            &response_deadline_root,
            &release_hold_root,
            &replay_containment_root,
            &fail_closed_acceptance_root,
            accepted,
            release_blocked,
            dispute_receipt_count,
        );
        let evidence_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE-ID",
            &[
                HashPart::Str(evidence_kind.as_str()),
                HashPart::U64(ordinal),
                HashPart::Str(&evidence_root),
            ],
            16,
        );
        Self {
            evidence_id,
            ordinal,
            evidence_kind,
            window_opening_root,
            dispute_receipt_root,
            response_deadline_root,
            release_hold_root,
            replay_containment_root,
            fail_closed_acceptance_root,
            evidence_root,
            accepted,
            release_blocked,
            dispute_receipt_count,
            evidence_statement: evidence_kind.evidence_statement().to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "evidence_id": self.evidence_id,
            "ordinal": self.ordinal,
            "evidence_kind": self.evidence_kind.as_str(),
            "window_opening_root": self.window_opening_root,
            "dispute_receipt_root": self.dispute_receipt_root,
            "response_deadline_root": self.response_deadline_root,
            "release_hold_root": self.release_hold_root,
            "replay_containment_root": self.replay_containment_root,
            "fail_closed_acceptance_root": self.fail_closed_acceptance_root,
            "evidence_root": self.evidence_root,
            "accepted": self.accepted,
            "release_blocked": self.release_blocked,
            "dispute_receipt_count": self.dispute_receipt_count,
            "evidence_statement": self.evidence_statement,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub window_openings_root: String,
    pub dispute_receipts_root: String,
    pub response_deadlines_root: String,
    pub release_holds_root: String,
    pub replay_containment_root: String,
    pub fail_closed_acceptance_root: String,
    pub evidence_bundle_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "window_openings_root": self.window_openings_root,
            "dispute_receipts_root": self.dispute_receipts_root,
            "response_deadlines_root": self.response_deadlines_root,
            "release_holds_root": self.release_holds_root,
            "replay_containment_root": self.replay_containment_root,
            "fail_closed_acceptance_root": self.fail_closed_acceptance_root,
            "evidence_bundle_root": self.evidence_bundle_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub drill_case_count: u64,
    pub accepted_count: u64,
    pub release_blocked_count: u64,
    pub dispute_receipt_count: u64,
    pub fail_closed_count: u64,
}

impl Counters {
    pub fn from_evidence(source: &SourceBundle, evidence: &[DrillEvidence]) -> Self {
        let accepted_count = evidence.iter().filter(|record| record.accepted).count() as u64;
        let release_blocked_count = evidence
            .iter()
            .filter(|record| record.release_blocked)
            .count() as u64;
        let dispute_receipt_count = evidence
            .iter()
            .map(|record| record.dispute_receipt_count)
            .fold(0_u64, u64::saturating_add);
        let fail_closed_count = evidence
            .iter()
            .filter(|record| !record.accepted || source.fail_closed_hold_active)
            .count() as u64;
        Self {
            drill_case_count: evidence.len() as u64,
            accepted_count,
            release_blocked_count,
            dispute_receipt_count,
            fail_closed_count,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "drill_case_count": self.drill_case_count,
            "accepted_count": self.accepted_count,
            "release_blocked_count": self.release_blocked_count,
            "dispute_receipt_count": self.dispute_receipt_count,
            "fail_closed_count": self.fail_closed_count,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub evidence: Vec<DrillEvidence>,
    pub roots: Roots,
    pub counters: Counters,
    pub release_accepted: bool,
    pub fail_closed: bool,
}

impl State {
    pub fn new(config: Config, monitor_state: monitor::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_monitor(&monitor_state);
        validate_source(&source)?;
        let evidence = DrillEvidenceKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, evidence_kind)| {
                DrillEvidence::devnet(&config, &source, *evidence_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let counters = Counters::from_evidence(&source, &evidence);
        let release_accepted = release_accepted(&config, &source, &counters);
        let fail_closed = !release_accepted || source.fail_closed_hold_active;
        let roots = build_roots(
            &config,
            &source,
            &evidence,
            &counters,
            release_accepted,
            fail_closed,
        );
        Ok(Self {
            config,
            source,
            evidence,
            roots,
            counters,
            release_accepted,
            fail_closed,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), monitor::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_challenge_window_drill_evidence_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "release_accepted": self.release_accepted,
            "fail_closed": self.fail_closed,
            "evidence": self
                .evidence
                .iter()
                .map(DrillEvidence::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_commitment_root.clone()
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
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-CHALLENGE-WINDOW-DRILL-EVIDENCE-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn build_roots(
    config: &Config,
    source: &SourceBundle,
    evidence: &[DrillEvidence],
    counters: &Counters,
    release_accepted: bool,
    fail_closed: bool,
) -> Roots {
    let window_openings_root = root_family("window-openings", evidence, |record| {
        record.window_opening_root.clone()
    });
    let dispute_receipts_root = root_family("dispute-receipts", evidence, |record| {
        record.dispute_receipt_root.clone()
    });
    let response_deadlines_root = root_family("response-deadlines", evidence, |record| {
        record.response_deadline_root.clone()
    });
    let release_holds_root = root_family("release-holds", evidence, |record| {
        record.release_hold_root.clone()
    });
    let replay_containment_root = root_family("replay-containment", evidence, |record| {
        record.replay_containment_root.clone()
    });
    let fail_closed_acceptance_root = root_family("fail-closed-acceptance", evidence, |record| {
        record.fail_closed_acceptance_root.clone()
    });
    let evidence_bundle_root = evidence_bundle_root(config, source, evidence, counters);
    let state_commitment_root = state_commitment_root(
        config,
        source,
        counters,
        &window_openings_root,
        &dispute_receipts_root,
        &response_deadlines_root,
        &release_holds_root,
        &replay_containment_root,
        &fail_closed_acceptance_root,
        &evidence_bundle_root,
        release_accepted,
        fail_closed,
    );
    Roots {
        window_openings_root,
        dispute_receipts_root,
        response_deadlines_root,
        release_holds_root,
        replay_containment_root,
        fail_closed_acceptance_root,
        evidence_bundle_root,
        state_commitment_root,
    }
}

fn root_family<F>(kind: &str, evidence: &[DrillEvidence], select: F) -> String
where
    F: Fn(&DrillEvidence) -> String,
{
    let leaves = evidence
        .iter()
        .map(|record| {
            json!({
                "evidence_id": record.evidence_id,
                "evidence_kind": record.evidence_kind.as_str(),
                "root": select(record),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        &format!(
            "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE-{}",
            kind.to_ascii_uppercase()
        ),
        &leaves,
    )
}

fn window_opening_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
) -> String {
    record_root(
        "window-opening",
        &json!({
            "suite": config.drill_evidence_suite,
            "evidence_kind": evidence_kind.as_str(),
            "ordinal": ordinal,
            "monitor_state_root": source.monitor_state_root,
            "monitor_window_root": source.monitor_window_root,
            "opened_at_height": source.opened_at_height,
            "challenge_deadline_height": source.challenge_deadline_height,
        }),
    )
}

fn dispute_receipt_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
    dispute_receipt_count: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-DISPUTE-RECEIPT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(evidence_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(&source.monitor_watcher_objection_root),
            HashPart::U64(source.watcher_objection_count),
            HashPart::U64(source.sustained_objection_count),
            HashPart::U64(dispute_receipt_count),
        ],
        32,
    )
}

fn response_deadline_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
) -> String {
    record_root(
        "response-deadline",
        &json!({
            "suite": config.drill_evidence_suite,
            "evidence_kind": evidence_kind.as_str(),
            "ordinal": ordinal,
            "monitor_deadline_root": source.monitor_deadline_root,
            "challenge_deadline_height": source.challenge_deadline_height,
            "watcher_response_deadline_height": source.watcher_response_deadline_height,
            "user_escape_deadline_height": source.user_escape_deadline_height,
        }),
    )
}

fn release_hold_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
    release_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-RELEASE-HOLD",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.monitor_fail_closed_hold_root),
            HashPart::Str(evidence_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(bool_str(release_blocked)),
            HashPart::Str(bool_str(source.monitor_production_blocked)),
            HashPart::Str(bool_str(source.monitor_release_allowed)),
        ],
        32,
    )
}

fn replay_containment_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
    release_blocked: bool,
) -> String {
    record_root(
        "replay-containment",
        &json!({
            "chain_id": config.chain_id,
            "protocol_version": PROTOCOL_VERSION,
            "evidence_kind": evidence_kind.as_str(),
            "ordinal": ordinal,
            "monitor_state_root": source.monitor_state_root,
            "monitor_window_root": source.monitor_window_root,
            "monitor_status": source.monitor_status,
            "release_blocked": release_blocked,
            "containment_policy": "single-window-single-monitor-root",
        }),
    )
}

fn fail_closed_acceptance_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
    accepted: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-FAIL-CLOSED-ACCEPTANCE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&source.monitor_verdict_root),
            HashPart::Str(evidence_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(bool_str(accepted)),
            HashPart::Str(bool_str(source.fail_closed_hold_active)),
            HashPart::Str(bool_str(config.require_fail_closed_acceptance)),
        ],
        32,
    )
}

fn drill_evidence_root(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    ordinal: u64,
    window_opening_root: &str,
    dispute_receipt_root: &str,
    response_deadline_root: &str,
    release_hold_root: &str,
    replay_containment_root: &str,
    fail_closed_acceptance_root: &str,
    accepted: bool,
    release_blocked: bool,
    dispute_receipt_count: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.monitor_state_root),
            HashPart::Str(evidence_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(window_opening_root),
            HashPart::Str(dispute_receipt_root),
            HashPart::Str(response_deadline_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(replay_containment_root),
            HashPart::Str(fail_closed_acceptance_root),
            HashPart::Str(bool_str(accepted)),
            HashPart::Str(bool_str(release_blocked)),
            HashPart::U64(dispute_receipt_count),
        ],
        32,
    )
}

fn evidence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    evidence: &[DrillEvidence],
    counters: &Counters,
) -> String {
    let leaves = evidence
        .iter()
        .map(DrillEvidence::public_record)
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE-BUNDLE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&source.monitor_state_root),
            HashPart::Str(&merkle_root(
                "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE-LEAVES",
                &leaves,
            )),
            HashPart::U64(counters.drill_case_count),
            HashPart::U64(counters.accepted_count),
            HashPart::U64(counters.release_blocked_count),
            HashPart::U64(counters.fail_closed_count),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    window_openings_root: &str,
    dispute_receipts_root: &str,
    response_deadlines_root: &str,
    release_holds_root: &str,
    replay_containment_root: &str,
    fail_closed_acceptance_root: &str,
    evidence_bundle_root: &str,
    release_accepted: bool,
    fail_closed: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-CHALLENGE-WINDOW-DRILL-EVIDENCE-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(window_openings_root),
            HashPart::Str(dispute_receipts_root),
            HashPart::Str(response_deadlines_root),
            HashPart::Str(release_holds_root),
            HashPart::Str(replay_containment_root),
            HashPart::Str(fail_closed_acceptance_root),
            HashPart::Str(evidence_bundle_root),
            HashPart::U64(counters.drill_case_count),
            HashPart::Str(bool_str(release_accepted)),
            HashPart::Str(bool_str(fail_closed)),
        ],
        32,
    )
}

fn dispute_receipt_count(source: &SourceBundle, evidence_kind: DrillEvidenceKind) -> u64 {
    match evidence_kind {
        DrillEvidenceKind::DisputeReceipt => source.watcher_objection_count,
        DrillEvidenceKind::FailClosedAcceptance => source.sustained_objection_count,
        _ => source.watcher_objection_count.min(1),
    }
}

fn drill_case_accepted(
    config: &Config,
    source: &SourceBundle,
    evidence_kind: DrillEvidenceKind,
    dispute_receipt_count: u64,
) -> bool {
    match evidence_kind {
        DrillEvidenceKind::WindowOpening => {
            config.require_window_opening_roots && !source.monitor_window_root.is_empty()
        }
        DrillEvidenceKind::DisputeReceipt => {
            config.require_dispute_receipt_roots
                && dispute_receipt_count >= config.min_dispute_receipts
                && !source.monitor_watcher_objection_root.is_empty()
        }
        DrillEvidenceKind::ResponseDeadline => {
            config.require_response_deadline_roots
                && source.watcher_response_deadline_height > source.challenge_deadline_height
                && source.user_escape_deadline_height >= source.challenge_deadline_height
        }
        DrillEvidenceKind::ReleaseHold => {
            config.require_release_hold_roots && !source.monitor_fail_closed_hold_root.is_empty()
        }
        DrillEvidenceKind::ReplayContainment => {
            config.require_replay_containment_roots && !source.monitor_state_root.is_empty()
        }
        DrillEvidenceKind::FailClosedAcceptance => {
            config.require_fail_closed_acceptance
                && source.challenge_window_resolved
                && !source.fail_closed_hold_active
                && source.monitor_release_allowed
        }
    }
}

fn release_accepted(config: &Config, source: &SourceBundle, counters: &Counters) -> bool {
    counters.drill_case_count >= config.min_drill_cases
        && counters.accepted_count == counters.drill_case_count
        && counters.release_blocked_count == 0
        && counters.dispute_receipt_count >= config.min_dispute_receipts
        && source.monitor_release_allowed
        && !source.fail_closed_hold_active
}

fn validate_config(config: &Config) -> Result<()> {
    if config.chain_id != CHAIN_ID {
        return Err("challenge window drill evidence chain id mismatch".to_string());
    }
    if config.protocol_version != PROTOCOL_VERSION {
        return Err("challenge window drill evidence protocol version mismatch".to_string());
    }
    if config.schema_version != SCHEMA_VERSION {
        return Err("challenge window drill evidence schema version mismatch".to_string());
    }
    if config.min_drill_cases == 0 {
        return Err("challenge window drill evidence requires drill cases".to_string());
    }
    if config.min_dispute_receipts == 0 {
        return Err("challenge window drill evidence requires dispute receipts".to_string());
    }
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    if source.monitor_state_root.is_empty() {
        return Err("challenge window drill evidence missing monitor state root".to_string());
    }
    if source.monitor_window_root.is_empty() {
        return Err("challenge window drill evidence missing monitor window root".to_string());
    }
    if source.monitor_deadline_root.is_empty() {
        return Err("challenge window drill evidence missing monitor deadline root".to_string());
    }
    Ok(())
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle {
        monitor_state_root: record_root("fallback-monitor-state", &json!({"reason": &reason})),
        monitor_window_root: record_root("fallback-monitor-window", &json!({"reason": &reason})),
        monitor_deadline_root: record_root(
            "fallback-monitor-deadline",
            &json!({"reason": &reason}),
        ),
        monitor_watcher_objection_root: record_root(
            "fallback-watcher-objections",
            &json!({"reason": &reason}),
        ),
        monitor_fail_closed_hold_root: record_root(
            "fallback-fail-closed-hold",
            &json!({"reason": &reason}),
        ),
        monitor_verdict_root: record_root("fallback-monitor-verdict", &json!({"reason": &reason})),
        monitor_status: "fallback".to_string(),
        challenge_window_resolved: false,
        fail_closed_hold_active: true,
        monitor_production_blocked: true,
        monitor_release_allowed: false,
        watcher_objection_count: 1,
        sustained_objection_count: 1,
        opened_at_height: 1,
        challenge_deadline_height: 2,
        watcher_response_deadline_height: 3,
        user_escape_deadline_height: 4,
    };
    let evidence = DrillEvidenceKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, evidence_kind)| {
            DrillEvidence::devnet(&config, &source, *evidence_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let counters = Counters::from_evidence(&source, &evidence);
    let release_accepted = false;
    let fail_closed = true;
    let roots = build_roots(
        &config,
        &source,
        &evidence,
        &counters,
        release_accepted,
        fail_closed,
    );
    State {
        config,
        source,
        evidence,
        roots,
        counters,
        release_accepted,
        fail_closed,
    }
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
