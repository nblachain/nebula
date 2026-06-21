use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-compile-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_ID: &str = "wave98";
pub const PRIOR_WAVE_ID: &str = "wave97";
pub const LANE_ID: &str =
    "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-compile-lane";
pub const PRIOR_LANE_ID: &str =
    "wave97-live-heavy-gate-release-claim-anti-equivocation-seal-compile-lane";
pub const DEFAULT_MIN_WAVE97_SEAL_ROOTS: usize = 7;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompileLaneGate {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    Rustc,
    BuildMetadata,
    OperatorSignoff,
}

impl CompileLaneGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::Clippy => "clippy",
            Self::Rustfmt => "rustfmt",
            Self::Rustc => "rustc",
            Self::BuildMetadata => "build_metadata",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> [Self; 7] {
        [
            Self::CargoCheck,
            Self::CargoTest,
            Self::Clippy,
            Self::Rustfmt,
            Self::Rustc,
            Self::BuildMetadata,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffVerdict {
    FailClosed,
    ChallengeWindowOpen,
    HoldActive,
    ReleaseBlocked,
    ReleaseAllowed,
}

impl HoldoffVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::HoldActive => "hold_active",
            Self::ReleaseBlocked => "release_blocked",
            Self::ReleaseAllowed => "release_allowed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffBlockerKind {
    DefaultFailClosed,
    HeavyGateNotRun,
    Wave97SealMissing,
    ChallengeWindowOpen,
    ObjectionWindowUncleared,
    ReviewerHoldActive,
    DisputeBondUnsettled,
    AppealDeadlineOpen,
    ReleaseClaimNotFinal,
    RootsOnlyBoundary,
}

impl HoldoffBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultFailClosed => "default_fail_closed",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
            Self::Wave97SealMissing => "wave97_seal_missing",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::ObjectionWindowUncleared => "objection_window_uncleared",
            Self::ReviewerHoldActive => "reviewer_hold_active",
            Self::DisputeBondUnsettled => "dispute_bond_unsettled",
            Self::AppealDeadlineOpen => "appeal_deadline_open",
            Self::ReleaseClaimNotFinal => "release_claim_not_final",
            Self::RootsOnlyBoundary => "roots_only_boundary",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_id: String,
    pub prior_wave_id: String,
    pub lane_id: String,
    pub prior_lane_id: String,
    pub min_wave97_seal_roots: usize,
    pub challenge_window_blocks: u64,
    pub roots_only_public_record: bool,
    pub release_allowed: bool,
    pub heavy_gates_ran: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_id: WAVE_ID.to_string(),
            prior_wave_id: PRIOR_WAVE_ID.to_string(),
            lane_id: LANE_ID.to_string(),
            prior_lane_id: PRIOR_LANE_ID.to_string(),
            min_wave97_seal_roots: DEFAULT_MIN_WAVE97_SEAL_ROOTS,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            roots_only_public_record: true,
            release_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_id": self.wave_id,
            "prior_wave_id": self.prior_wave_id,
            "lane_id": self.lane_id,
            "prior_lane_id": self.prior_lane_id,
            "min_wave97_seal_roots": self.min_wave97_seal_roots,
            "challenge_window_blocks": self.challenge_window_blocks,
            "roots_only_public_record": self.roots_only_public_record,
            "release_allowed": self.release_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave97SealRoot {
    pub gate: CompileLaneGate,
    pub release_claim_seal_root: String,
    pub anti_equivocation_root: String,
    pub prior_command_root: String,
    pub sealed: bool,
}

impl Wave97SealRoot {
    pub fn blocked(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            release_claim_seal_root: placeholder_root("wave97-release-claim-seal", gate.as_str()),
            anti_equivocation_root: placeholder_root("wave97-anti-equivocation", gate.as_str()),
            prior_command_root: placeholder_root("wave97-command", gate.as_str()),
            sealed: false,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "release_claim_seal_root": self.release_claim_seal_root,
            "anti_equivocation_root": self.anti_equivocation_root,
            "prior_command_root": self.prior_command_root,
            "sealed": self.sealed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("wave97_seal_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeWindowRoot {
    pub gate: CompileLaneGate,
    pub window_open_root: String,
    pub window_close_root: String,
    pub holdoff_ledger_root: String,
    pub open: bool,
    pub blocked: bool,
}

impl ChallengeWindowRoot {
    pub fn open(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            window_open_root: placeholder_root("challenge-window-open", gate.as_str()),
            window_close_root: placeholder_root("challenge-window-close", gate.as_str()),
            holdoff_ledger_root: placeholder_root("challenge-window-holdoff-ledger", gate.as_str()),
            open: true,
            blocked: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "window_open_root": self.window_open_root,
            "window_close_root": self.window_close_root,
            "holdoff_ledger_root": self.holdoff_ledger_root,
            "open": self.open,
            "blocked": self.blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("challenge_window_root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffLedgerEntry {
    pub gate: CompileLaneGate,
    pub objection_root: String,
    pub reviewer_hold_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub command_hint_root: String,
    pub hold_active: bool,
}

impl HoldoffLedgerEntry {
    pub fn active(gate: CompileLaneGate) -> Self {
        Self {
            gate,
            objection_root: placeholder_root("objection", gate.as_str()),
            reviewer_hold_root: placeholder_root("reviewer-hold", gate.as_str()),
            dispute_bond_root: placeholder_root("dispute-bond", gate.as_str()),
            appeal_deadline_root: placeholder_root("appeal-deadline", gate.as_str()),
            command_hint_root: placeholder_root("holdoff-command-hint", gate.as_str()),
            hold_active: true,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "gate": self.gate.as_str(),
            "objection_root": self.objection_root,
            "reviewer_hold_root": self.reviewer_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "command_hint_root": self.command_hint_root,
            "hold_active": self.hold_active,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_ledger_entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffBlocker {
    pub kind: HoldoffBlockerKind,
    pub gate: Option<CompileLaneGate>,
    pub blocker_root: String,
}

impl HoldoffBlocker {
    pub fn new(kind: HoldoffBlockerKind, gate: Option<CompileLaneGate>) -> Self {
        let label = match gate {
            Some(value) => value.as_str(),
            None => "compile_lane",
        };
        Self {
            kind,
            gate,
            blocker_root: placeholder_root(kind.as_str(), label),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": self.kind.as_str(),
            "gate": self.gate.map(CompileLaneGate::as_str),
            "blocker_root": self.blocker_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_blocker", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffCounters {
    pub wave97_seal_roots: usize,
    pub challenge_window_roots: usize,
    pub objection_roots: usize,
    pub reviewer_hold_roots: usize,
    pub dispute_bond_roots: usize,
    pub appeal_deadline_roots: usize,
    pub active_holds: usize,
    pub active_blockers: usize,
    pub released_claims: usize,
}

impl HoldoffCounters {
    pub fn public_record(&self) -> Value {
        json!({
            "wave97_seal_roots": self.wave97_seal_roots,
            "challenge_window_roots": self.challenge_window_roots,
            "objection_roots": self.objection_roots,
            "reviewer_hold_roots": self.reviewer_hold_roots,
            "dispute_bond_roots": self.dispute_bond_roots,
            "appeal_deadline_roots": self.appeal_deadline_roots,
            "active_holds": self.active_holds,
            "active_blockers": self.active_blockers,
            "released_claims": self.released_claims,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave97_seal_roots: Vec<Wave97SealRoot>,
    pub challenge_window_roots: Vec<ChallengeWindowRoot>,
    pub holdoff_ledger: Vec<HoldoffLedgerEntry>,
    pub blockers: Vec<HoldoffBlocker>,
    pub counters: HoldoffCounters,
    pub verdict: HoldoffVerdict,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave97_seal_roots = CompileLaneGate::all()
            .into_iter()
            .map(Wave97SealRoot::blocked)
            .collect::<Vec<_>>();
        let challenge_window_roots = CompileLaneGate::all()
            .into_iter()
            .map(ChallengeWindowRoot::open)
            .collect::<Vec<_>>();
        let holdoff_ledger = CompileLaneGate::all()
            .into_iter()
            .map(HoldoffLedgerEntry::active)
            .collect::<Vec<_>>();
        let mut blockers = vec![
            HoldoffBlocker::new(HoldoffBlockerKind::DefaultFailClosed, None),
            HoldoffBlocker::new(HoldoffBlockerKind::HeavyGateNotRun, None),
            HoldoffBlocker::new(HoldoffBlockerKind::ChallengeWindowOpen, None),
            HoldoffBlocker::new(HoldoffBlockerKind::ReleaseClaimNotFinal, None),
            HoldoffBlocker::new(HoldoffBlockerKind::RootsOnlyBoundary, None),
        ];
        for gate in CompileLaneGate::all() {
            blockers.push(HoldoffBlocker::new(
                HoldoffBlockerKind::Wave97SealMissing,
                Some(gate),
            ));
            blockers.push(HoldoffBlocker::new(
                HoldoffBlockerKind::ObjectionWindowUncleared,
                Some(gate),
            ));
            blockers.push(HoldoffBlocker::new(
                HoldoffBlockerKind::ReviewerHoldActive,
                Some(gate),
            ));
            blockers.push(HoldoffBlocker::new(
                HoldoffBlockerKind::DisputeBondUnsettled,
                Some(gate),
            ));
            blockers.push(HoldoffBlocker::new(
                HoldoffBlockerKind::AppealDeadlineOpen,
                Some(gate),
            ));
        }
        let counters = HoldoffCounters {
            wave97_seal_roots: wave97_seal_roots.len(),
            challenge_window_roots: challenge_window_roots.len(),
            objection_roots: holdoff_ledger.len(),
            reviewer_hold_roots: holdoff_ledger.len(),
            dispute_bond_roots: holdoff_ledger.len(),
            appeal_deadline_roots: holdoff_ledger.len(),
            active_holds: holdoff_ledger.len(),
            active_blockers: blockers.len(),
            released_claims: 0,
        };
        Self {
            config,
            wave97_seal_roots,
            challenge_window_roots,
            holdoff_ledger,
            blockers,
            counters,
            verdict: HoldoffVerdict::ReleaseBlocked,
        }
    }

    pub fn public_record(&self) -> Value {
        let wave97_roots = record_roots(
            "wave97-seal-roots",
            self.wave97_seal_roots
                .iter()
                .map(Wave97SealRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let challenge_roots = record_roots(
            "challenge-window-roots",
            self.challenge_window_roots
                .iter()
                .map(ChallengeWindowRoot::state_root)
                .collect::<Vec<_>>(),
        );
        let objection_roots = record_roots(
            "objection-roots",
            self.holdoff_ledger
                .iter()
                .map(|entry| entry.objection_root.clone())
                .collect::<Vec<_>>(),
        );
        let reviewer_hold_roots = record_roots(
            "reviewer-hold-roots",
            self.holdoff_ledger
                .iter()
                .map(|entry| entry.reviewer_hold_root.clone())
                .collect::<Vec<_>>(),
        );
        let dispute_bond_roots = record_roots(
            "dispute-bond-roots",
            self.holdoff_ledger
                .iter()
                .map(|entry| entry.dispute_bond_root.clone())
                .collect::<Vec<_>>(),
        );
        let appeal_deadline_roots = record_roots(
            "appeal-deadline-roots",
            self.holdoff_ledger
                .iter()
                .map(|entry| entry.appeal_deadline_root.clone())
                .collect::<Vec<_>>(),
        );
        let command_hint_roots = record_roots(
            "command-hint-roots",
            self.holdoff_ledger
                .iter()
                .map(|entry| entry.command_hint_root.clone())
                .collect::<Vec<_>>(),
        );
        let blocker_roots = record_roots(
            "holdoff-blocker-roots",
            self.blockers
                .iter()
                .map(HoldoffBlocker::state_root)
                .collect::<Vec<_>>(),
        );
        let ledger_entry_roots = record_roots(
            "holdoff-ledger-entry-roots",
            self.holdoff_ledger
                .iter()
                .map(HoldoffLedgerEntry::state_root)
                .collect::<Vec<_>>(),
        );
        let deterministic_roots = record_roots(
            "deterministic-holdoff-roots",
            vec![
                wave97_roots.merkle.clone(),
                challenge_roots.merkle.clone(),
                objection_roots.merkle.clone(),
                reviewer_hold_roots.merkle.clone(),
                dispute_bond_roots.merkle.clone(),
                appeal_deadline_roots.merkle.clone(),
                command_hint_roots.merkle.clone(),
                blocker_roots.merkle.clone(),
                ledger_entry_roots.merkle.clone(),
            ],
        );

        json!({
            "config_root": self.config.state_root(),
            "wave97_seal_root_count": self.wave97_seal_roots.len(),
            "wave97_seal_roots": wave97_roots.items,
            "wave97_seal_merkle": wave97_roots.merkle,
            "challenge_window_root_count": self.challenge_window_roots.len(),
            "challenge_window_roots": challenge_roots.items,
            "challenge_window_merkle": challenge_roots.merkle,
            "objection_roots": objection_roots.items,
            "objection_merkle": objection_roots.merkle,
            "reviewer_hold_roots": reviewer_hold_roots.items,
            "reviewer_hold_merkle": reviewer_hold_roots.merkle,
            "dispute_bond_roots": dispute_bond_roots.items,
            "dispute_bond_merkle": dispute_bond_roots.merkle,
            "appeal_deadline_roots": appeal_deadline_roots.items,
            "appeal_deadline_merkle": appeal_deadline_roots.merkle,
            "command_hint_roots": command_hint_roots.items,
            "command_hint_merkle": command_hint_roots.merkle,
            "holdoff_ledger_entry_roots": ledger_entry_roots.items,
            "holdoff_ledger_merkle": ledger_entry_roots.merkle,
            "holdoff_blocker_roots": blocker_roots.items,
            "holdoff_blocker_merkle": blocker_roots.merkle,
            "deterministic_roots": deterministic_roots.items,
            "deterministic_merkle": deterministic_roots.merkle,
            "counters_root": self.counters.state_root(),
            "released_claims": self.counters.released_claims,
            "challenge_windows_open": self.challenge_window_roots.iter().filter(|window| window.open).count(),
            "active_holds": self.counters.active_holds,
            "release_allowed": self.config.release_allowed,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "verdict": self.verdict.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            "WAVE98-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER-COMPILE-LANE-STATE",
            &[
                HashPart::Str(CHAIN_ID),
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Json(&self.public_record()),
            ],
            32,
        )
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

struct RootList {
    items: Vec<String>,
    merkle: String,
}

fn record_roots(kind: &str, roots: Vec<String>) -> RootList {
    let leaves = roots
        .iter()
        .map(|root| Value::String(domain_hash(kind, &[HashPart::Str(root)], 32)))
        .collect::<Vec<_>>();
    RootList {
        items: roots,
        merkle: merkle_root(&format!("{DOMAIN}:{kind}"), &leaves),
    }
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "WAVE98-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER-COMPILE-LANE-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn placeholder_root(kind: &str, label: &str) -> String {
    domain_hash(
        "WAVE98-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER-COMPILE-LANE-PLACEHOLDER",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(label),
        ],
        32,
    )
}

const DOMAIN: &str = "WAVE98-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER-COMPILE-LANE";
