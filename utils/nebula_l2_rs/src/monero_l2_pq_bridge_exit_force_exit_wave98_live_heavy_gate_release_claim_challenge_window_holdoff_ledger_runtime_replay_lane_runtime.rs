use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave98-release-claim-challenge-window-holdoff-ledger-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 98;
pub const DEFAULT_SEAL_WAVE: u64 = 97;
pub const DEFAULT_QUORUM_WAVE: u64 = 96;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 7_200;
pub const DEFAULT_APPEAL_WINDOW_BLOCKS: u64 = 3_600;
pub const DEFAULT_REQUIRED_DISPUTE_BOND_ROOTS: u64 = 6;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    ReleaseClaimSeal,
    ReplayChallenge,
    RollbackObjection,
    StaleArchiveChallenge,
    DisputeBond,
    AppealDeadline,
    OperatorHoldoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReleaseClaimSeal,
            Self::ReplayChallenge,
            Self::RollbackObjection,
            Self::StaleArchiveChallenge,
            Self::DisputeBond,
            Self::AppealDeadline,
            Self::OperatorHoldoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseClaimSeal => "release_claim_seal",
            Self::ReplayChallenge => "replay_challenge",
            Self::RollbackObjection => "rollback_objection",
            Self::StaleArchiveChallenge => "stale_archive_challenge",
            Self::DisputeBond => "dispute_bond",
            Self::AppealDeadline => "appeal_deadline",
            Self::OperatorHoldoff => "operator_holdoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffStatus {
    Open,
    Blocked,
    Released,
}

impl HoldoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Blocked => "blocked",
            Self::Released => "released",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffBlocker {
    FailClosedArmed,
    ProductionDenied,
    HeavyGateReceiptAbsent,
    ChallengeWindowOpen,
    ChallengeWindowBlocked,
    ReleaseClaimSealRootCarriedFromWave97,
    ReplayChallengeRootOpen,
    RollbackObjectionRootOpen,
    StaleArchiveChallengeRootOpen,
    DisputeBondRootOpen,
    AppealDeadlineRootOpen,
    OperatorReleaseMissing,
}

impl HoldoffBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ProductionDenied,
            Self::HeavyGateReceiptAbsent,
            Self::ChallengeWindowOpen,
            Self::ChallengeWindowBlocked,
            Self::ReleaseClaimSealRootCarriedFromWave97,
            Self::ReplayChallengeRootOpen,
            Self::RollbackObjectionRootOpen,
            Self::StaleArchiveChallengeRootOpen,
            Self::DisputeBondRootOpen,
            Self::AppealDeadlineRootOpen,
            Self::OperatorReleaseMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::ChallengeWindowBlocked => "challenge_window_blocked",
            Self::ReleaseClaimSealRootCarriedFromWave97 => {
                "release_claim_seal_root_carried_from_wave97"
            }
            Self::ReplayChallengeRootOpen => "replay_challenge_root_open",
            Self::RollbackObjectionRootOpen => "rollback_objection_root_open",
            Self::StaleArchiveChallengeRootOpen => "stale_archive_challenge_root_open",
            Self::DisputeBondRootOpen => "dispute_bond_root_open",
            Self::AppealDeadlineRootOpen => "appeal_deadline_root_open",
            Self::OperatorReleaseMissing => "operator_release_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    CarryWave97SealRoots,
    KeepChallengeWindowOpen,
    HoldReplayChallenges,
    HoldRollbackObjections,
    HoldStaleArchiveChallenges,
    BindDisputeBondRoots,
    BindAppealDeadlineRoots,
    DenyReleaseUntilHoldoffClears,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CarryWave97SealRoots,
            Self::KeepChallengeWindowOpen,
            Self::HoldReplayChallenges,
            Self::HoldRollbackObjections,
            Self::HoldStaleArchiveChallenges,
            Self::BindDisputeBondRoots,
            Self::BindAppealDeadlineRoots,
            Self::DenyReleaseUntilHoldoffClears,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CarryWave97SealRoots => "carry_wave97_seal_roots",
            Self::KeepChallengeWindowOpen => "keep_challenge_window_open",
            Self::HoldReplayChallenges => "hold_replay_challenges",
            Self::HoldRollbackObjections => "hold_rollback_objections",
            Self::HoldStaleArchiveChallenges => "hold_stale_archive_challenges",
            Self::BindDisputeBondRoots => "bind_dispute_bond_roots",
            Self::BindAppealDeadlineRoots => "bind_appeal_deadline_roots",
            Self::DenyReleaseUntilHoldoffClears => "deny_release_until_holdoff_clears",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub lane_suite: String,
    pub wave: u64,
    pub seal_wave: u64,
    pub quorum_wave: u64,
    pub challenge_window_blocks: u64,
    pub appeal_window_blocks: u64,
    pub required_dispute_bond_roots: u64,
    pub wave97_release_claim_seal_root: String,
    pub wave97_runtime_replay_seal_root: String,
    pub wave96_release_readiness_quorum_root: String,
    pub fail_closed_armed: bool,
    pub production_denied: bool,
    pub challenge_windows_open: bool,
    pub challenge_windows_blocked: bool,
    pub holds_active: bool,
    pub release_claims_released: bool,
    pub heavy_gates_ran: bool,
    pub max_public_raw_records: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            lane_suite: LANE_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            seal_wave: DEFAULT_SEAL_WAVE,
            quorum_wave: DEFAULT_QUORUM_WAVE,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            appeal_window_blocks: DEFAULT_APPEAL_WINDOW_BLOCKS,
            required_dispute_bond_roots: DEFAULT_REQUIRED_DISPUTE_BOND_ROOTS,
            wave97_release_claim_seal_root: stable_root("wave97-release-claim-seal", "all"),
            wave97_runtime_replay_seal_root: stable_root(
                "wave97-runtime-replay-release-claim-seal",
                "runtime-replay",
            ),
            wave96_release_readiness_quorum_root: stable_root(
                "wave96-release-readiness-quorum",
                "runtime-replay",
            ),
            fail_closed_armed: true,
            production_denied: true,
            challenge_windows_open: true,
            challenge_windows_blocked: true,
            holds_active: true,
            release_claims_released: false,
            heavy_gates_ran: false,
            max_public_raw_records: DEFAULT_MAX_PUBLIC_RAW_RECORDS,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("lane_suite", &self.lane_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("seal_wave", self.seal_wave)?;
        ensure_positive("quorum_wave", self.quorum_wave)?;
        ensure_positive("challenge_window_blocks", self.challenge_window_blocks)?;
        ensure_positive("appeal_window_blocks", self.appeal_window_blocks)?;
        ensure_positive(
            "required_dispute_bond_roots",
            self.required_dispute_bond_roots,
        )?;
        ensure_root(
            "wave97_release_claim_seal_root",
            &self.wave97_release_claim_seal_root,
        )?;
        ensure_root(
            "wave97_runtime_replay_seal_root",
            &self.wave97_runtime_replay_seal_root,
        )?;
        ensure_root(
            "wave96_release_readiness_quorum_root",
            &self.wave96_release_readiness_quorum_root,
        )?;
        if !(self.quorum_wave < self.seal_wave && self.seal_wave < self.wave) {
            return Err("wave ordering must be quorum, seal, holdoff".to_string());
        }
        if !self.fail_closed_armed {
            return Err("challenge holdoff fail closed guard is disarmed".to_string());
        }
        if !self.production_denied {
            return Err("devnet challenge holdoff must deny production".to_string());
        }
        if !self.challenge_windows_open {
            return Err("devnet challenge windows must remain open".to_string());
        }
        if !self.challenge_windows_blocked {
            return Err("devnet challenge windows must remain blocked".to_string());
        }
        if !self.holds_active {
            return Err("devnet holdoff ledger must keep holds active".to_string());
        }
        if self.release_claims_released {
            return Err("devnet must not release claims".to_string());
        }
        if self.heavy_gates_ran {
            return Err("runtime replay lane cannot claim live heavy gate execution".to_string());
        }
        if self.max_public_raw_records != 0 {
            return Err("public records must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_challenge_window_holdoff_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "seal_wave": self.seal_wave,
            "quorum_wave": self.quorum_wave,
            "challenge_window_blocks": self.challenge_window_blocks,
            "appeal_window_blocks": self.appeal_window_blocks,
            "required_dispute_bond_roots": self.required_dispute_bond_roots,
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "wave97_runtime_replay_seal_root": self.wave97_runtime_replay_seal_root,
            "wave96_release_readiness_quorum_root": self.wave96_release_readiness_quorum_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_denied": self.production_denied,
            "challenge_windows_open": self.challenge_windows_open,
            "challenge_windows_blocked": self.challenge_windows_blocked,
            "holds_active": self.holds_active,
            "release_claims_released": self.release_claims_released,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffEntry {
    pub lane: ReplayLane,
    pub wave97_release_claim_seal_root: String,
    pub replay_challenge_root: String,
    pub rollback_objection_root: String,
    pub stale_archive_challenge_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub command_hint_root: String,
    pub blocker_roots: Vec<String>,
    pub status: HoldoffStatus,
}

impl HoldoffEntry {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            wave97_release_claim_seal_root: lane_wave97_seal_root(lane, config),
            replay_challenge_root: lane_root("replay-challenge", lane),
            rollback_objection_root: lane_root("rollback-objection", lane),
            stale_archive_challenge_root: lane_root("stale-archive-challenge", lane),
            dispute_bond_root: lane_root("dispute-bond", lane),
            appeal_deadline_root: lane_root("appeal-deadline", lane),
            command_hint_root: lane_root("command-hint", lane),
            blocker_roots: HoldoffBlocker::all()
                .iter()
                .map(|blocker| blocker_root(lane, *blocker))
                .collect(),
            status: HoldoffStatus::Blocked,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root(
            "wave97_release_claim_seal_root",
            &self.wave97_release_claim_seal_root,
        )?;
        ensure_root("replay_challenge_root", &self.replay_challenge_root)?;
        ensure_root("rollback_objection_root", &self.rollback_objection_root)?;
        ensure_root(
            "stale_archive_challenge_root",
            &self.stale_archive_challenge_root,
        )?;
        ensure_root("dispute_bond_root", &self.dispute_bond_root)?;
        ensure_root("appeal_deadline_root", &self.appeal_deadline_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.blocker_roots.is_empty() {
            return Err("blocked holdoff entry requires blocker roots".to_string());
        }
        for root in self.blocker_roots.iter() {
            ensure_root("blocker_root", root)?;
        }
        if self.status == HoldoffStatus::Released {
            return Err("devnet holdoff entry cannot be released".to_string());
        }
        Ok(())
    }

    pub fn blocker_root(&self) -> String {
        list_root("WAVE98-HOLDOFF-BLOCKER-ROOTS", self.blocker_roots.clone())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_holdoff_entry",
            "lane": self.lane.as_str(),
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "replay_challenge_root": self.replay_challenge_root,
            "rollback_objection_root": self.rollback_objection_root,
            "stale_archive_challenge_root": self.stale_archive_challenge_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "command_hint_root": self.command_hint_root,
            "blocker_roots_root": self.blocker_root(),
            "blocker_count": self.blocker_roots.len(),
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-HOLDOFF-ENTRY", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeWindow {
    pub lane: ReplayLane,
    pub open_root: String,
    pub blocked_root: String,
    pub close_deadline_root: String,
    pub appeal_deadline_root: String,
    pub release_after_root: String,
    pub open: bool,
    pub blocked: bool,
    pub release_allowed: bool,
}

impl ChallengeWindow {
    pub fn open_blocked(lane: ReplayLane) -> Self {
        Self {
            lane,
            open_root: lane_root("challenge-window-open", lane),
            blocked_root: lane_root("challenge-window-blocked", lane),
            close_deadline_root: lane_root("challenge-window-close-deadline", lane),
            appeal_deadline_root: lane_root("challenge-window-appeal-deadline", lane),
            release_after_root: lane_root("challenge-window-release-after", lane),
            open: true,
            blocked: true,
            release_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("open_root", &self.open_root)?;
        ensure_root("blocked_root", &self.blocked_root)?;
        ensure_root("close_deadline_root", &self.close_deadline_root)?;
        ensure_root("appeal_deadline_root", &self.appeal_deadline_root)?;
        ensure_root("release_after_root", &self.release_after_root)?;
        if !self.open {
            return Err("devnet challenge window must remain open".to_string());
        }
        if !self.blocked {
            return Err("devnet challenge window must remain blocked".to_string());
        }
        if self.release_allowed {
            return Err("challenge window holdoff cannot allow release".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_challenge_window",
            "lane": self.lane.as_str(),
            "open_root": self.open_root,
            "blocked_root": self.blocked_root,
            "close_deadline_root": self.close_deadline_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "release_after_root": self.release_after_root,
            "open": self.open,
            "blocked": self.blocked,
            "release_allowed": self.release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-CHALLENGE-WINDOW", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffLedger {
    pub replay_challenge_roots: BTreeMap<String, String>,
    pub rollback_objection_roots: BTreeMap<String, String>,
    pub stale_archive_challenge_roots: BTreeMap<String, String>,
    pub dispute_bond_roots: BTreeMap<String, String>,
    pub appeal_deadline_roots: BTreeMap<String, String>,
    pub command_hints: BTreeMap<String, String>,
}

impl HoldoffLedger {
    pub fn devnet(lanes: &[ReplayLane]) -> Self {
        Self {
            replay_challenge_roots: lane_map(lanes, "replay-challenge"),
            rollback_objection_roots: lane_map(lanes, "rollback-objection"),
            stale_archive_challenge_roots: lane_map(lanes, "stale-archive-challenge"),
            dispute_bond_roots: lane_map(lanes, "dispute-bond"),
            appeal_deadline_roots: lane_map(lanes, "appeal-deadline"),
            command_hints: CommandHint::all()
                .iter()
                .map(|hint| (hint.as_str().to_string(), command_hint_kind_root(*hint)))
                .collect(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_map_roots("replay_challenge_roots", &self.replay_challenge_roots)?;
        ensure_map_roots("rollback_objection_roots", &self.rollback_objection_roots)?;
        ensure_map_roots(
            "stale_archive_challenge_roots",
            &self.stale_archive_challenge_roots,
        )?;
        ensure_map_roots("dispute_bond_roots", &self.dispute_bond_roots)?;
        ensure_map_roots("appeal_deadline_roots", &self.appeal_deadline_roots)?;
        ensure_map_roots("command_hints", &self.command_hints)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_holdoff_ledger",
            "replay_challenge_root": map_root("WAVE98-REPLAY-CHALLENGE-MAP", &self.replay_challenge_roots),
            "rollback_objection_root": map_root("WAVE98-ROLLBACK-OBJECTION-MAP", &self.rollback_objection_roots),
            "stale_archive_challenge_root": map_root("WAVE98-STALE-ARCHIVE-CHALLENGE-MAP", &self.stale_archive_challenge_roots),
            "dispute_bond_root": map_root("WAVE98-DISPUTE-BOND-MAP", &self.dispute_bond_roots),
            "appeal_deadline_root": map_root("WAVE98-APPEAL-DEADLINE-MAP", &self.appeal_deadline_roots),
            "command_hint_root": map_root("WAVE98-COMMAND-HINT-MAP", &self.command_hints),
            "replay_challenge_count": self.replay_challenge_roots.len(),
            "rollback_objection_count": self.rollback_objection_roots.len(),
            "stale_archive_challenge_count": self.stale_archive_challenge_roots.len(),
            "dispute_bond_count": self.dispute_bond_roots.len(),
            "appeal_deadline_count": self.appeal_deadline_roots.len(),
            "command_hint_count": self.command_hints.len(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-HOLDOFF-LEDGER", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_count: u64,
    pub holdoff_entry_count: u64,
    pub challenge_window_count: u64,
    pub open_challenge_windows: u64,
    pub blocked_challenge_windows: u64,
    pub released_claims: u64,
    pub active_hold_count: u64,
    pub replay_challenge_roots: u64,
    pub rollback_objection_roots: u64,
    pub stale_archive_challenge_roots: u64,
    pub dispute_bond_roots: u64,
    pub appeal_deadline_roots: u64,
    pub raw_public_records: u64,
}

impl Counters {
    pub fn from_parts(
        entries: &[HoldoffEntry],
        windows: &[ChallengeWindow],
        ledger: &HoldoffLedger,
    ) -> Self {
        Self {
            lane_count: ReplayLane::all().len() as u64,
            holdoff_entry_count: entries.len() as u64,
            challenge_window_count: windows.len() as u64,
            open_challenge_windows: windows.iter().filter(|window| window.open).count() as u64,
            blocked_challenge_windows: windows.iter().filter(|window| window.blocked).count()
                as u64,
            released_claims: entries
                .iter()
                .filter(|entry| entry.status == HoldoffStatus::Released)
                .count() as u64,
            active_hold_count: entries
                .iter()
                .filter(|entry| entry.status != HoldoffStatus::Released)
                .count() as u64,
            replay_challenge_roots: ledger.replay_challenge_roots.len() as u64,
            rollback_objection_roots: ledger.rollback_objection_roots.len() as u64,
            stale_archive_challenge_roots: ledger.stale_archive_challenge_roots.len() as u64,
            dispute_bond_roots: ledger.dispute_bond_roots.len() as u64,
            appeal_deadline_roots: ledger.appeal_deadline_roots.len() as u64,
            raw_public_records: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_holdoff_counters",
            "lane_count": self.lane_count,
            "holdoff_entry_count": self.holdoff_entry_count,
            "challenge_window_count": self.challenge_window_count,
            "open_challenge_windows": self.open_challenge_windows,
            "blocked_challenge_windows": self.blocked_challenge_windows,
            "released_claims": self.released_claims,
            "active_hold_count": self.active_hold_count,
            "replay_challenge_roots": self.replay_challenge_roots,
            "rollback_objection_roots": self.rollback_objection_roots,
            "stale_archive_challenge_roots": self.stale_archive_challenge_roots,
            "dispute_bond_roots": self.dispute_bond_roots,
            "appeal_deadline_roots": self.appeal_deadline_roots,
            "raw_public_records": self.raw_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub holdoff_entries: Vec<HoldoffEntry>,
    pub challenge_windows: Vec<ChallengeWindow>,
    pub holdoff_ledger: HoldoffLedger,
    pub blocker_catalog: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        holdoff_entries: Vec<HoldoffEntry>,
        challenge_windows: Vec<ChallengeWindow>,
        holdoff_ledger: HoldoffLedger,
        blocker_catalog: BTreeMap<String, String>,
    ) -> Result<Self> {
        let counters = Counters::from_parts(&holdoff_entries, &challenge_windows, &holdoff_ledger);
        let state = Self {
            config,
            holdoff_entries,
            challenge_windows,
            holdoff_ledger,
            blocker_catalog,
            counters,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_unique_lanes(
            "holdoff_entries",
            self.holdoff_entries
                .iter()
                .map(|entry| entry.lane)
                .collect(),
        )?;
        ensure_unique_lanes(
            "challenge_windows",
            self.challenge_windows
                .iter()
                .map(|window| window.lane)
                .collect(),
        )?;
        for entry in self.holdoff_entries.iter() {
            entry.validate()?;
        }
        for window in self.challenge_windows.iter() {
            window.validate()?;
        }
        self.holdoff_ledger.validate()?;
        ensure_map_roots("blocker_catalog", &self.blocker_catalog)?;
        if self.counters.released_claims != 0 {
            return Err("devnet must have zero released claims".to_string());
        }
        if self.counters.open_challenge_windows != self.counters.challenge_window_count {
            return Err("all devnet challenge windows must remain open".to_string());
        }
        if self.counters.blocked_challenge_windows != self.counters.challenge_window_count {
            return Err("all devnet challenge windows must remain blocked".to_string());
        }
        if self.counters.active_hold_count != self.counters.holdoff_entry_count {
            return Err("all devnet holdoff entries must remain active".to_string());
        }
        if self.counters.raw_public_records != 0 {
            return Err("public record counter must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave98_challenge_window_holdoff_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "wave97_release_claim_seal_root": self.config.wave97_release_claim_seal_root,
            "wave97_runtime_replay_seal_root": self.config.wave97_runtime_replay_seal_root,
            "wave96_release_readiness_quorum_root": self.config.wave96_release_readiness_quorum_root,
            "holdoff_entries_root": holdoff_entries_root(&self.holdoff_entries),
            "challenge_windows_root": challenge_windows_root(&self.challenge_windows),
            "holdoff_ledger_root": self.holdoff_ledger.state_root(),
            "blocker_catalog_root": map_root("WAVE98-BLOCKER-CATALOG", &self.blocker_catalog),
            "command_hints_root": map_root("WAVE98-COMMAND-HINTS", &self.holdoff_ledger.command_hints),
            "replay_challenge_root": map_root("WAVE98-REPLAY-CHALLENGE-ROOTS", &self.holdoff_ledger.replay_challenge_roots),
            "rollback_objection_root": map_root("WAVE98-ROLLBACK-OBJECTION-ROOTS", &self.holdoff_ledger.rollback_objection_roots),
            "stale_archive_challenge_root": map_root("WAVE98-STALE-ARCHIVE-CHALLENGE-ROOTS", &self.holdoff_ledger.stale_archive_challenge_roots),
            "dispute_bond_root": map_root("WAVE98-DISPUTE-BOND-ROOTS", &self.holdoff_ledger.dispute_bond_roots),
            "appeal_deadline_root": map_root("WAVE98-APPEAL-DEADLINE-ROOTS", &self.holdoff_ledger.appeal_deadline_roots),
            "counters_root": self.counters.state_root(),
            "holdoff_status": HoldoffStatus::Blocked.as_str(),
            "challenge_windows_open": self.config.challenge_windows_open,
            "challenge_windows_blocked": self.config.challenge_windows_blocked,
            "holds_active": self.config.holds_active,
            "release_claims_released": self.config.release_claims_released,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "raw_public_records": self.counters.raw_public_records,
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Value::Object(fields) = &mut record {
            fields.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-STATE", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let lanes = ReplayLane::all();
    let holdoff_entries = lanes
        .iter()
        .map(|lane| HoldoffEntry::blocked(*lane, &config))
        .collect::<Vec<_>>();
    let challenge_windows = lanes
        .iter()
        .map(|lane| ChallengeWindow::open_blocked(*lane))
        .collect::<Vec<_>>();
    let holdoff_ledger = HoldoffLedger::devnet(&lanes);
    let blocker_catalog = HoldoffBlocker::all()
        .iter()
        .map(|blocker| (blocker.as_str().to_string(), blocker_kind_root(*blocker)))
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        holdoff_entries,
        challenge_windows,
        holdoff_ledger,
        blocker_catalog,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            holdoff_entries: Vec::new(),
            challenge_windows: Vec::new(),
            holdoff_ledger: HoldoffLedger::devnet(&[]),
            blocker_catalog: BTreeMap::new(),
            counters: Counters::default(),
        },
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn holdoff_entries_root(entries: &[HoldoffEntry]) -> String {
    list_root(
        "WAVE98-HOLDOFF-ENTRY-STATE-ROOTS",
        entries.iter().map(HoldoffEntry::state_root).collect(),
    )
}

fn challenge_windows_root(windows: &[ChallengeWindow]) -> String {
    list_root(
        "WAVE98-CHALLENGE-WINDOW-STATE-ROOTS",
        windows.iter().map(ChallengeWindow::state_root).collect(),
    )
}

fn lane_map(lanes: &[ReplayLane], kind: &str) -> BTreeMap<String, String> {
    lanes
        .iter()
        .map(|lane| (lane.as_str().to_string(), lane_root(kind, *lane)))
        .collect()
}

fn map_root(domain: &str, roots: &BTreeMap<String, String>) -> String {
    let entries = roots
        .iter()
        .map(|(label, root)| {
            domain_hash(
                domain,
                &[
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(label),
                    HashPart::Str(root),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    list_root(domain, entries)
}

fn lane_wave97_seal_root(lane: ReplayLane, config: &Config) -> String {
    domain_hash(
        "WAVE98-LANE-WAVE97-SEAL-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(&config.wave97_runtime_replay_seal_root),
        ],
        32,
    )
}

fn lane_root(kind: &str, lane: ReplayLane) -> String {
    domain_hash(
        "WAVE98-LANE-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn command_hint_kind_root(hint: CommandHint) -> String {
    domain_hash(
        "WAVE98-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: HoldoffBlocker) -> String {
    domain_hash(
        "WAVE98-HOLDOFF-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn blocker_kind_root(blocker: HoldoffBlocker) -> String {
    domain_hash(
        "WAVE98-HOLDOFF-BLOCKER-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn stable_root(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE98-STABLE-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(domain, &[HashPart::Json(value)], 32)
}

fn list_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots
        .iter()
        .map(|root| Value::String(domain_hash(domain, &[HashPart::Str(root)], 32)))
        .collect::<Vec<_>>();
    merkle_root(domain, leaves.as_slice())
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.is_empty() {
        Err(format!("{} must be non-empty", field))
    } else {
        Ok(())
    }
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        Err(format!("{} must be positive", field))
    } else {
        Ok(())
    }
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 {
        return Err(format!("{} must be a deterministic root", field));
    }
    Ok(())
}

fn ensure_map_roots(field: &str, roots: &BTreeMap<String, String>) -> Result<()> {
    if roots.is_empty() {
        return Err(format!("{} must not be empty", field));
    }
    for (label, root) in roots.iter() {
        ensure_non_empty(field, label)?;
        ensure_root(field, root)?;
    }
    Ok(())
}

fn ensure_unique_lanes(field: &str, lanes: Vec<ReplayLane>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for lane in lanes.iter() {
        if !seen.insert(*lane) {
            return Err(format!("{} contains duplicate lane", field));
        }
    }
    Ok(())
}
