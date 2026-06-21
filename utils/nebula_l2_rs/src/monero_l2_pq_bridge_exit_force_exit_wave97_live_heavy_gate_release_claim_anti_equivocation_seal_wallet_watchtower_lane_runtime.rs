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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave97-live-heavy-gate-release-claim-anti-equivocation-seal-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave97";
pub const SOURCE_WAVE_LABEL: &str = "wave96";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-receipt-release-readiness-quorum-wallet-watchtower-lane";
pub const SEAL_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-anti-equivocation-seal-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave97-wallet-watchtower-anti-equivocation-seal-root";
pub const DEFAULT_SEAL_EPOCH: u64 = 97;
pub const DEFAULT_MIN_RELEASE_CLAIM_ROOTS: u64 = 1;
pub const DEFAULT_MIN_WATCHER_QUORUM_ROOTS: u64 = 4;
pub const DEFAULT_MIN_ANTI_EQUIVOCATION_ROOTS: u64 = 3;
pub const DEFAULT_MIN_REPLAY_GUARD_ROOTS: u64 = 2;
pub const DEFAULT_MIN_FORK_GUARD_ROOTS: u64 = 2;
pub const DEFAULT_MIN_DUPLICATE_GUARD_ROOTS: u64 = 2;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub source_lane: String,
    pub seal_lane: String,
    pub empty_root_marker: String,
    pub seal_epoch: u64,
    pub min_release_claim_roots: u64,
    pub min_watcher_quorum_roots: u64,
    pub min_anti_equivocation_roots: u64,
    pub min_replay_guard_roots: u64,
    pub min_fork_guard_roots: u64,
    pub min_duplicate_guard_roots: u64,
    pub require_roots_only_public_record: bool,
    pub require_wave96_readiness_root: bool,
    pub require_release_claim_roots: bool,
    pub require_watcher_quorum_roots: bool,
    pub require_anti_equivocation_roots: bool,
    pub require_replay_guard_roots: bool,
    pub require_fork_guard_roots: bool,
    pub require_duplicate_guard_roots: bool,
    pub fail_closed_on_empty_seal: bool,
    pub release_claim_publication_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            wave_label: WAVE_LABEL.to_string(),
            source_wave_label: SOURCE_WAVE_LABEL.to_string(),
            source_lane: SOURCE_LANE.to_string(),
            seal_lane: SEAL_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            seal_epoch: DEFAULT_SEAL_EPOCH,
            min_release_claim_roots: DEFAULT_MIN_RELEASE_CLAIM_ROOTS,
            min_watcher_quorum_roots: DEFAULT_MIN_WATCHER_QUORUM_ROOTS,
            min_anti_equivocation_roots: DEFAULT_MIN_ANTI_EQUIVOCATION_ROOTS,
            min_replay_guard_roots: DEFAULT_MIN_REPLAY_GUARD_ROOTS,
            min_fork_guard_roots: DEFAULT_MIN_FORK_GUARD_ROOTS,
            min_duplicate_guard_roots: DEFAULT_MIN_DUPLICATE_GUARD_ROOTS,
            require_roots_only_public_record: true,
            require_wave96_readiness_root: true,
            require_release_claim_roots: true,
            require_watcher_quorum_roots: true,
            require_anti_equivocation_roots: true,
            require_replay_guard_roots: true,
            require_fork_guard_roots: true,
            require_duplicate_guard_roots: true,
            fail_closed_on_empty_seal: true,
            release_claim_publication_enabled: false,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_label": self.wave_label,
            "source_wave_label": self.source_wave_label,
            "source_lane": self.source_lane,
            "seal_lane": self.seal_lane,
            "empty_root_marker": self.empty_root_marker,
            "seal_epoch": self.seal_epoch,
            "min_release_claim_roots": self.min_release_claim_roots,
            "min_watcher_quorum_roots": self.min_watcher_quorum_roots,
            "min_anti_equivocation_roots": self.min_anti_equivocation_roots,
            "min_replay_guard_roots": self.min_replay_guard_roots,
            "min_fork_guard_roots": self.min_fork_guard_roots,
            "min_duplicate_guard_roots": self.min_duplicate_guard_roots,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave96_readiness_root": self.require_wave96_readiness_root,
            "require_release_claim_roots": self.require_release_claim_roots,
            "require_watcher_quorum_roots": self.require_watcher_quorum_roots,
            "require_anti_equivocation_roots": self.require_anti_equivocation_roots,
            "require_replay_guard_roots": self.require_replay_guard_roots,
            "require_fork_guard_roots": self.require_fork_guard_roots,
            "require_duplicate_guard_roots": self.require_duplicate_guard_roots,
            "fail_closed_on_empty_seal": self.fail_closed_on_empty_seal,
            "release_claim_publication_enabled": self.release_claim_publication_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealSlotKind {
    WalletReleaseClaim,
    WatcherQuorum,
    AntiEquivocation,
    ReplayGuard,
    ForkGuard,
    DuplicateGuard,
    OperatorCommand,
}

impl SealSlotKind {
    pub fn all() -> [Self; 7] {
        [
            Self::WalletReleaseClaim,
            Self::WatcherQuorum,
            Self::AntiEquivocation,
            Self::ReplayGuard,
            Self::ForkGuard,
            Self::DuplicateGuard,
            Self::OperatorCommand,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletReleaseClaim => "wallet_release_claim",
            Self::WatcherQuorum => "watcher_quorum",
            Self::AntiEquivocation => "anti_equivocation",
            Self::ReplayGuard => "replay_guard",
            Self::ForkGuard => "fork_guard",
            Self::DuplicateGuard => "duplicate_guard",
            Self::OperatorCommand => "operator_command",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealStatus {
    EmptyBlocked,
    GuardBlocked,
    Sealed,
    Denied,
}

impl SealStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBlocked => "empty_blocked",
            Self::GuardBlocked => "guard_blocked",
            Self::Sealed => "sealed",
            Self::Denied => "denied",
        }
    }

    pub fn can_release(self) -> bool {
        matches!(self, Self::Sealed)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealBlocker {
    EmptySeal,
    Wave96ReadinessRootMissing,
    ReleaseClaimRootMissing,
    WatcherQuorumRootMissing,
    AntiEquivocationRootMissing,
    ReplayGuardRootMissing,
    ForkGuardRootMissing,
    DuplicateGuardRootMissing,
    ReleaseClaimRootDuplicate,
    ReleaseClaimRootForked,
    ReleaseClaimRootReplayed,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    PublicationDenied,
    HeavyGatesNotRun,
}

impl SealBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptySeal => "empty_seal",
            Self::Wave96ReadinessRootMissing => "wave96_readiness_root_missing",
            Self::ReleaseClaimRootMissing => "release_claim_root_missing",
            Self::WatcherQuorumRootMissing => "watcher_quorum_root_missing",
            Self::AntiEquivocationRootMissing => "anti_equivocation_root_missing",
            Self::ReplayGuardRootMissing => "replay_guard_root_missing",
            Self::ForkGuardRootMissing => "fork_guard_root_missing",
            Self::DuplicateGuardRootMissing => "duplicate_guard_root_missing",
            Self::ReleaseClaimRootDuplicate => "release_claim_root_duplicate",
            Self::ReleaseClaimRootForked => "release_claim_root_forked",
            Self::ReleaseClaimRootReplayed => "release_claim_root_replayed",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::PublicationDenied => "publication_denied",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldReleaseClaim,
    ImportWave96ReadinessRoot,
    ImportReleaseClaimRoot,
    ImportWatcherQuorumRoot,
    ImportAntiEquivocationRoot,
    ImportReplayGuardRoot,
    ImportForkGuardRoot,
    ImportDuplicateGuardRoot,
    ResolveEquivocation,
    SealReleaseClaim,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldReleaseClaim => "hold_release_claim",
            Self::ImportWave96ReadinessRoot => "import_wave96_readiness_root",
            Self::ImportReleaseClaimRoot => "import_release_claim_root",
            Self::ImportWatcherQuorumRoot => "import_watcher_quorum_root",
            Self::ImportAntiEquivocationRoot => "import_anti_equivocation_root",
            Self::ImportReplayGuardRoot => "import_replay_guard_root",
            Self::ImportForkGuardRoot => "import_fork_guard_root",
            Self::ImportDuplicateGuardRoot => "import_duplicate_guard_root",
            Self::ResolveEquivocation => "resolve_equivocation",
            Self::SealReleaseClaim => "seal_release_claim",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub command_root: String,
    pub blocker_root: String,
}

impl CommandHint {
    pub fn new(slot_kind: SealSlotKind, kind: CommandHintKind, blockers: &[SealBlocker]) -> Self {
        let blocker_root = blockers_root("command-next-blockers", blockers);
        let command_root = record_root(
            "command-hint",
            &json!({
                "slot_kind": slot_kind.as_str(),
                "kind": kind.as_str(),
                "blocker_root": blocker_root,
                "raw_command_absent": true,
            }),
        );
        Self {
            kind,
            command_root,
            blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SealAttempt {
    pub slot_kind: SealSlotKind,
    pub wave96_readiness_root: String,
    pub release_claim_roots: Vec<String>,
    pub watcher_quorum_roots: Vec<String>,
    pub anti_equivocation_roots: Vec<String>,
    pub replay_guard_roots: Vec<String>,
    pub fork_guard_roots: Vec<String>,
    pub duplicate_guard_roots: Vec<String>,
    pub release_claim_root: String,
    pub watcher_quorum_root: String,
    pub anti_equivocation_root: String,
    pub replay_guard_root: String,
    pub fork_guard_root: String,
    pub duplicate_guard_root: String,
    pub anti_equivocation_seal_root: String,
    pub blockers: Vec<SealBlocker>,
    pub status: SealStatus,
    pub command_hint: CommandHint,
    pub release_allowed: bool,
}

impl SealAttempt {
    pub fn empty(slot_kind: SealSlotKind, config: &Config) -> Self {
        Self::from_roots(
            slot_kind,
            empty_root("wave96-readiness"),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            config,
        )
    }

    pub fn from_roots(
        slot_kind: SealSlotKind,
        wave96_readiness_root: impl Into<String>,
        release_claim_roots: Vec<String>,
        watcher_quorum_roots: Vec<String>,
        anti_equivocation_roots: Vec<String>,
        replay_guard_roots: Vec<String>,
        fork_guard_roots: Vec<String>,
        duplicate_guard_roots: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut attempt = Self {
            slot_kind,
            wave96_readiness_root: wave96_readiness_root.into(),
            release_claim_roots,
            watcher_quorum_roots,
            anti_equivocation_roots,
            replay_guard_roots,
            fork_guard_roots,
            duplicate_guard_roots,
            release_claim_root: empty_root("release-claim"),
            watcher_quorum_root: empty_root("watcher-quorum"),
            anti_equivocation_root: empty_root("anti-equivocation"),
            replay_guard_root: empty_root("replay-guard"),
            fork_guard_root: empty_root("fork-guard"),
            duplicate_guard_root: empty_root("duplicate-guard"),
            anti_equivocation_seal_root: empty_root("anti-equivocation-seal"),
            blockers: Vec::new(),
            status: SealStatus::EmptyBlocked,
            command_hint: CommandHint::new(slot_kind, CommandHintKind::HoldReleaseClaim, &[]),
            release_allowed: false,
        };
        attempt.recompute(config);
        attempt
    }

    pub fn recompute(&mut self, config: &Config) {
        self.release_claim_root = aggregate_root("release-claim-roots", &self.release_claim_roots);
        self.watcher_quorum_root =
            aggregate_root("watcher-quorum-roots", &self.watcher_quorum_roots);
        self.anti_equivocation_root =
            aggregate_root("anti-equivocation-roots", &self.anti_equivocation_roots);
        self.replay_guard_root = aggregate_root("replay-guard-roots", &self.replay_guard_roots);
        self.fork_guard_root = aggregate_root("fork-guard-roots", &self.fork_guard_roots);
        self.duplicate_guard_root =
            aggregate_root("duplicate-guard-roots", &self.duplicate_guard_roots);
        self.anti_equivocation_seal_root = seal_root(self);
        self.blockers = seal_blockers(self, config);
        self.status = if !config.release_claim_publication_enabled {
            SealStatus::Denied
        } else if all_guard_sets_empty(self) {
            SealStatus::EmptyBlocked
        } else if self.blockers.is_empty() {
            SealStatus::Sealed
        } else {
            SealStatus::GuardBlocked
        };
        self.release_allowed = self.status.can_release()
            && config.release_claim_publication_enabled
            && config.heavy_gates_ran;
        self.command_hint =
            CommandHint::new(self.slot_kind, command_for_attempt(self), &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        blockers_root("attempt-blockers", &self.blockers)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave96_readiness_root": self.wave96_readiness_root,
            "release_claim_root": self.release_claim_root,
            "watcher_quorum_root": self.watcher_quorum_root,
            "anti_equivocation_root": self.anti_equivocation_root,
            "replay_guard_root": self.replay_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "anti_equivocation_seal_root": self.anti_equivocation_seal_root,
            "blocker_root": self.blocker_root(),
            "status": self.status.as_str(),
            "command_hint": self.command_hint.public_record(),
            "release_allowed": self.release_allowed,
            "roots_only": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("seal-attempt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SealSummary {
    pub fail_closed: bool,
    pub release_denied: bool,
    pub sealed_lanes: u64,
    pub blocked_lanes: u64,
    pub denied_lanes: u64,
    pub release_claim_root: String,
    pub watcher_quorum_root: String,
    pub anti_equivocation_root: String,
    pub replay_guard_root: String,
    pub fork_guard_root: String,
    pub duplicate_guard_root: String,
    pub blocker_root: String,
    pub command_root: String,
    pub seal_root: String,
    pub heavy_gates_ran: bool,
}

impl SealSummary {
    pub fn from_attempts(config: &Config, attempts: &BTreeMap<String, SealAttempt>) -> Self {
        let sealed_lanes = attempts
            .values()
            .filter(|attempt| attempt.status == SealStatus::Sealed)
            .count() as u64;
        let blocked_lanes = attempts
            .values()
            .filter(|attempt| {
                matches!(
                    attempt.status,
                    SealStatus::EmptyBlocked | SealStatus::GuardBlocked
                )
            })
            .count() as u64;
        let denied_lanes = attempts
            .values()
            .filter(|attempt| attempt.status == SealStatus::Denied)
            .count() as u64;
        let release_claim_root = attempt_field_root(
            "summary-release-claim-roots",
            attempts
                .values()
                .map(|attempt| attempt.release_claim_root.clone()),
        );
        let watcher_quorum_root = attempt_field_root(
            "summary-watcher-quorum-roots",
            attempts
                .values()
                .map(|attempt| attempt.watcher_quorum_root.clone()),
        );
        let anti_equivocation_root = attempt_field_root(
            "summary-anti-equivocation-roots",
            attempts
                .values()
                .map(|attempt| attempt.anti_equivocation_root.clone()),
        );
        let replay_guard_root = attempt_field_root(
            "summary-replay-guard-roots",
            attempts
                .values()
                .map(|attempt| attempt.replay_guard_root.clone()),
        );
        let fork_guard_root = attempt_field_root(
            "summary-fork-guard-roots",
            attempts
                .values()
                .map(|attempt| attempt.fork_guard_root.clone()),
        );
        let duplicate_guard_root = attempt_field_root(
            "summary-duplicate-guard-roots",
            attempts
                .values()
                .map(|attempt| attempt.duplicate_guard_root.clone()),
        );
        let blocker_root = attempt_field_root(
            "summary-blockers",
            attempts.values().map(SealAttempt::blocker_root),
        );
        let command_root = merkle_root(
            "WAVE97-WALLET-WATCHTOWER-COMMAND-HINTS",
            &attempts
                .values()
                .map(|attempt| attempt.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let seal_root = attempt_field_root(
            "summary-anti-equivocation-seals",
            attempts
                .values()
                .map(|attempt| attempt.anti_equivocation_seal_root.clone()),
        );
        let fail_closed = sealed_lanes == 0
            || blocked_lanes > 0
            || denied_lanes > 0
            || !config.release_claim_publication_enabled
            || !config.heavy_gates_ran;
        Self {
            fail_closed,
            release_denied: fail_closed,
            sealed_lanes,
            blocked_lanes,
            denied_lanes,
            release_claim_root,
            watcher_quorum_root,
            anti_equivocation_root,
            replay_guard_root,
            fork_guard_root,
            duplicate_guard_root,
            blocker_root,
            command_root,
            seal_root,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "release_denied": self.release_denied,
            "sealed_lanes": self.sealed_lanes,
            "blocked_lanes": self.blocked_lanes,
            "denied_lanes": self.denied_lanes,
            "release_claim_root": self.release_claim_root,
            "watcher_quorum_root": self.watcher_quorum_root,
            "anti_equivocation_root": self.anti_equivocation_root,
            "replay_guard_root": self.replay_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "seal_root": self.seal_root,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("seal-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub seal_attempts: BTreeMap<String, SealAttempt>,
    pub summary: SealSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let seal_attempts = SealSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let attempt = SealAttempt::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), attempt)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = SealSummary::from_attempts(&config, &seal_attempts);
        Self {
            config,
            seal_attempts,
            summary,
        }
    }

    pub fn stage_seal_attempt(
        mut self,
        slot_kind: SealSlotKind,
        wave96_readiness_root: impl Into<String>,
        release_claim_roots: Vec<String>,
        watcher_quorum_roots: Vec<String>,
        anti_equivocation_roots: Vec<String>,
        replay_guard_roots: Vec<String>,
        fork_guard_roots: Vec<String>,
        duplicate_guard_roots: Vec<String>,
    ) -> Result<Self> {
        let attempt = SealAttempt::from_roots(
            slot_kind,
            wave96_readiness_root,
            release_claim_roots,
            watcher_quorum_roots,
            anti_equivocation_roots,
            replay_guard_roots,
            fork_guard_roots,
            duplicate_guard_roots,
            &self.config,
        );
        self.seal_attempts
            .insert(slot_kind.as_str().to_string(), attempt);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for attempt in self.seal_attempts.values_mut() {
            attempt.recompute(&self.config);
        }
        self.summary = SealSummary::from_attempts(&self.config, &self.seal_attempts);
    }

    pub fn seal_attempt_roots(&self) -> BTreeMap<String, String> {
        self.seal_attempts
            .iter()
            .map(|(key, attempt)| (key.clone(), attempt.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn seal_attempts_root(&self) -> String {
        merkle_root(
            "WAVE97-WALLET-WATCHTOWER-SEAL-ATTEMPT-ROOTS",
            &self
                .seal_attempt_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "seal_attempt_roots": self.seal_attempt_roots(),
            "seal_attempts_root": self.seal_attempts_root(),
            "summary": self.summary.public_record(),
            "roots_only": true,
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_state_root();
        if let Some(map) = record.as_object_mut() {
            map.insert("state_root".to_string(), Value::String(self.state_root()));
        }
        record
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    State::default()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn wallet_watchtower_anti_equivocation_seal_runtime() -> Runtime {
    devnet()
}

fn seal_blockers(attempt: &SealAttempt, config: &Config) -> Vec<SealBlocker> {
    let mut blockers = Vec::new();
    if config.fail_closed_on_empty_seal && all_guard_sets_empty(attempt) {
        blockers.push(SealBlocker::EmptySeal);
    }
    if config.require_wave96_readiness_root && is_empty_root(&attempt.wave96_readiness_root) {
        blockers.push(SealBlocker::Wave96ReadinessRootMissing);
    }
    if config.require_release_claim_roots
        && attempt.release_claim_roots.len() < config.min_release_claim_roots as usize
    {
        blockers.push(SealBlocker::ReleaseClaimRootMissing);
    }
    if config.require_watcher_quorum_roots
        && attempt.watcher_quorum_roots.len() < config.min_watcher_quorum_roots as usize
    {
        blockers.push(SealBlocker::WatcherQuorumRootMissing);
    }
    if config.require_anti_equivocation_roots
        && attempt.anti_equivocation_roots.len() < config.min_anti_equivocation_roots as usize
    {
        blockers.push(SealBlocker::AntiEquivocationRootMissing);
    }
    if config.require_replay_guard_roots
        && attempt.replay_guard_roots.len() < config.min_replay_guard_roots as usize
    {
        blockers.push(SealBlocker::ReplayGuardRootMissing);
    }
    if config.require_fork_guard_roots
        && attempt.fork_guard_roots.len() < config.min_fork_guard_roots as usize
    {
        blockers.push(SealBlocker::ForkGuardRootMissing);
    }
    if config.require_duplicate_guard_roots
        && attempt.duplicate_guard_roots.len() < config.min_duplicate_guard_roots as usize
    {
        blockers.push(SealBlocker::DuplicateGuardRootMissing);
    }
    if has_duplicate(&attempt.release_claim_roots) {
        blockers.push(SealBlocker::ReleaseClaimRootDuplicate);
    }
    if !attempt.fork_guard_roots.is_empty()
        && attempt.fork_guard_roots.len() < attempt.release_claim_roots.len()
    {
        blockers.push(SealBlocker::ReleaseClaimRootForked);
    }
    if !attempt.replay_guard_roots.is_empty()
        && attempt.replay_guard_roots.len() < attempt.release_claim_roots.len()
    {
        blockers.push(SealBlocker::ReleaseClaimRootReplayed);
    }
    if !roots_shape_valid(attempt) {
        blockers.push(SealBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(attempt) {
        blockers.push(SealBlocker::RootsOnlyRecordMissing);
    }
    if !config.release_claim_publication_enabled {
        blockers.push(SealBlocker::PublicationDenied);
    }
    if !config.heavy_gates_ran {
        blockers.push(SealBlocker::HeavyGatesNotRun);
    }
    blockers
}

fn command_for_attempt(attempt: &SealAttempt) -> CommandHintKind {
    if attempt.blockers.is_empty() {
        return CommandHintKind::SealReleaseClaim;
    }
    match attempt.blockers[0] {
        SealBlocker::Wave96ReadinessRootMissing => CommandHintKind::ImportWave96ReadinessRoot,
        SealBlocker::ReleaseClaimRootMissing => CommandHintKind::ImportReleaseClaimRoot,
        SealBlocker::WatcherQuorumRootMissing => CommandHintKind::ImportWatcherQuorumRoot,
        SealBlocker::AntiEquivocationRootMissing => CommandHintKind::ImportAntiEquivocationRoot,
        SealBlocker::ReplayGuardRootMissing => CommandHintKind::ImportReplayGuardRoot,
        SealBlocker::ForkGuardRootMissing => CommandHintKind::ImportForkGuardRoot,
        SealBlocker::DuplicateGuardRootMissing => CommandHintKind::ImportDuplicateGuardRoot,
        SealBlocker::ReleaseClaimRootDuplicate
        | SealBlocker::ReleaseClaimRootForked
        | SealBlocker::ReleaseClaimRootReplayed => CommandHintKind::ResolveEquivocation,
        SealBlocker::EmptySeal
        | SealBlocker::RootShapeInvalid
        | SealBlocker::RootsOnlyRecordMissing
        | SealBlocker::PublicationDenied
        | SealBlocker::HeavyGatesNotRun => CommandHintKind::HoldReleaseClaim,
    }
}

fn roots_only_record_present(attempt: &SealAttempt) -> bool {
    is_root_like(&attempt.wave96_readiness_root)
        && is_root_like(&attempt.release_claim_root)
        && is_root_like(&attempt.watcher_quorum_root)
        && is_root_like(&attempt.anti_equivocation_root)
        && is_root_like(&attempt.replay_guard_root)
        && is_root_like(&attempt.fork_guard_root)
        && is_root_like(&attempt.duplicate_guard_root)
        && is_root_like(&attempt.anti_equivocation_seal_root)
}

fn roots_shape_valid(attempt: &SealAttempt) -> bool {
    is_root_like(&attempt.wave96_readiness_root)
        && all_roots_like(&attempt.release_claim_roots)
        && all_roots_like(&attempt.watcher_quorum_roots)
        && all_roots_like(&attempt.anti_equivocation_roots)
        && all_roots_like(&attempt.replay_guard_roots)
        && all_roots_like(&attempt.fork_guard_roots)
        && all_roots_like(&attempt.duplicate_guard_roots)
}

fn all_roots_like(roots: &[String]) -> bool {
    roots.iter().all(|root| is_root_like(root))
}

fn all_guard_sets_empty(attempt: &SealAttempt) -> bool {
    attempt.release_claim_roots.is_empty()
        && attempt.watcher_quorum_roots.is_empty()
        && attempt.anti_equivocation_roots.is_empty()
        && attempt.replay_guard_roots.is_empty()
        && attempt.fork_guard_roots.is_empty()
        && attempt.duplicate_guard_roots.is_empty()
}

fn has_duplicate(roots: &[String]) -> bool {
    let mut seen = BTreeSet::new();
    roots.iter().any(|root| !seen.insert(root))
}

fn seal_root(attempt: &SealAttempt) -> String {
    record_root(
        "anti-equivocation-seal",
        &json!({
            "slot_kind": attempt.slot_kind.as_str(),
            "wave96_readiness_root": attempt.wave96_readiness_root,
            "release_claim_root": attempt.release_claim_root,
            "watcher_quorum_root": attempt.watcher_quorum_root,
            "anti_equivocation_root": attempt.anti_equivocation_root,
            "replay_guard_root": attempt.replay_guard_root,
            "fork_guard_root": attempt.fork_guard_root,
            "duplicate_guard_root": attempt.duplicate_guard_root,
            "wallet_material_absent": true,
            "watcher_identity_material_absent": true,
            "route_material_absent": true,
            "payload_material_absent": true,
            "roots_only": true,
        }),
    )
}

fn aggregate_root(domain: &str, roots: &[String]) -> String {
    if roots.is_empty() {
        return empty_root(domain);
    }
    merkle_root(
        "WAVE97-WALLET-WATCHTOWER-ROOT-AGGREGATE",
        &roots
            .iter()
            .cloned()
            .map(|root| {
                json!({
                    "domain": domain,
                    "root": root,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn attempt_field_root<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    merkle_root(
        domain,
        &values.into_iter().map(Value::String).collect::<Vec<_>>(),
    )
}

fn blockers_root(domain: &str, blockers: &[SealBlocker]) -> String {
    merkle_root(
        domain,
        &blockers
            .iter()
            .map(|blocker| Value::String(blocker.as_str().to_string()))
            .collect::<Vec<_>>(),
    )
}

fn empty_root(marker_name: &str) -> String {
    let root = record_root(
        "empty-seal-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "marker_name": marker_name,
        }),
    );
    format!("{EMPTY_ROOT_MARKER}:{root}")
}

fn is_empty_root(root: &str) -> bool {
    root.is_empty() || root.contains(EMPTY_ROOT_MARKER)
}

fn is_root_like(root: &str) -> bool {
    !root.is_empty()
        && root.len() >= 16
        && root
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ':' | '-' | '_' | '.'))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    domain_hash(
        "WAVE97-WALLET-WATCHTOWER-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
