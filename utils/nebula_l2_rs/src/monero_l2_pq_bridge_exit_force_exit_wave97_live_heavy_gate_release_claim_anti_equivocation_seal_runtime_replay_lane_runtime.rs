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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave97-release-claim-anti-equivocation-seal-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave97-live-heavy-gate-release-claim-anti-equivocation-seal-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 97;
pub const DEFAULT_READINESS_WAVE: u64 = 96;
pub const DEFAULT_PROMOTION_WAVE: u64 = 95;
pub const DEFAULT_RELEASE_CLAIMS_REQUIRED: u64 = 6;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    ReleaseClaim,
    ReplayToken,
    AntiEquivocationSeal,
    ForkGuard,
    DuplicateGuard,
    HeavyGateWitness,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReleaseClaim,
            Self::ReplayToken,
            Self::AntiEquivocationSeal,
            Self::ForkGuard,
            Self::DuplicateGuard,
            Self::HeavyGateWitness,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseClaim => "release_claim",
            Self::ReplayToken => "replay_token",
            Self::AntiEquivocationSeal => "anti_equivocation_seal",
            Self::ForkGuard => "fork_guard",
            Self::DuplicateGuard => "duplicate_guard",
            Self::HeavyGateWitness => "heavy_gate_witness",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseStatus {
    Denied,
    Sealed,
}

impl ReleaseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::Sealed => "sealed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AntiEquivocationBlocker {
    FailClosedArmed,
    ProductionDenied,
    NoSealedLaneDefault,
    HeavyGateReceiptAbsent,
    ReleaseClaimRootPlaceholder,
    ReplayTokenRootPlaceholder,
    PriorSealRootPlaceholder,
    EquivocationScanNotLive,
    ReplayGuardActive,
    ForkGuardActive,
    DuplicateGuardActive,
    OperatorSealMissing,
}

impl AntiEquivocationBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ProductionDenied,
            Self::NoSealedLaneDefault,
            Self::HeavyGateReceiptAbsent,
            Self::ReleaseClaimRootPlaceholder,
            Self::ReplayTokenRootPlaceholder,
            Self::PriorSealRootPlaceholder,
            Self::EquivocationScanNotLive,
            Self::ReplayGuardActive,
            Self::ForkGuardActive,
            Self::DuplicateGuardActive,
            Self::OperatorSealMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ProductionDenied => "production_denied",
            Self::NoSealedLaneDefault => "no_sealed_lane_default",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::ReleaseClaimRootPlaceholder => "release_claim_root_placeholder",
            Self::ReplayTokenRootPlaceholder => "replay_token_root_placeholder",
            Self::PriorSealRootPlaceholder => "prior_seal_root_placeholder",
            Self::EquivocationScanNotLive => "equivocation_scan_not_live",
            Self::ReplayGuardActive => "replay_guard_active",
            Self::ForkGuardActive => "fork_guard_active",
            Self::DuplicateGuardActive => "duplicate_guard_active",
            Self::OperatorSealMissing => "operator_seal_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    PublishReleaseClaimRoots,
    BindReplayTokenRoots,
    KeepAntiEquivocationBlockers,
    ReconcileReplayGuard,
    ReconcileForkGuard,
    ReconcileDuplicateGuard,
    SealOnlyAfterLiveHeavyGate,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::PublishReleaseClaimRoots,
            Self::BindReplayTokenRoots,
            Self::KeepAntiEquivocationBlockers,
            Self::ReconcileReplayGuard,
            Self::ReconcileForkGuard,
            Self::ReconcileDuplicateGuard,
            Self::SealOnlyAfterLiveHeavyGate,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PublishReleaseClaimRoots => "publish_release_claim_roots",
            Self::BindReplayTokenRoots => "bind_replay_token_roots",
            Self::KeepAntiEquivocationBlockers => "keep_anti_equivocation_blockers",
            Self::ReconcileReplayGuard => "reconcile_replay_guard",
            Self::ReconcileForkGuard => "reconcile_fork_guard",
            Self::ReconcileDuplicateGuard => "reconcile_duplicate_guard",
            Self::SealOnlyAfterLiveHeavyGate => "seal_only_after_live_heavy_gate",
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
    pub readiness_wave: u64,
    pub promotion_wave: u64,
    pub release_claims_required: u64,
    pub wave96_readiness_quorum_root: String,
    pub wave95_promotion_root: String,
    pub fail_closed_armed: bool,
    pub production_denied: bool,
    pub anti_equivocation_blockers_active: bool,
    pub release_denied: bool,
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
            readiness_wave: DEFAULT_READINESS_WAVE,
            promotion_wave: DEFAULT_PROMOTION_WAVE,
            release_claims_required: DEFAULT_RELEASE_CLAIMS_REQUIRED,
            wave96_readiness_quorum_root: stable_root("wave96-readiness-quorum", "runtime-replay"),
            wave95_promotion_root: stable_root("wave95-slot-promotion", "runtime-replay"),
            fail_closed_armed: true,
            production_denied: true,
            anti_equivocation_blockers_active: true,
            release_denied: true,
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
        ensure_positive("readiness_wave", self.readiness_wave)?;
        ensure_positive("promotion_wave", self.promotion_wave)?;
        ensure_positive("release_claims_required", self.release_claims_required)?;
        ensure_root(
            "wave96_readiness_quorum_root",
            &self.wave96_readiness_quorum_root,
        )?;
        ensure_root("wave95_promotion_root", &self.wave95_promotion_root)?;
        if !(self.promotion_wave < self.readiness_wave && self.readiness_wave < self.wave) {
            return Err("wave ordering must be promotion, readiness, seal".to_string());
        }
        if !self.fail_closed_armed {
            return Err("anti-equivocation seal fail closed guard is disarmed".to_string());
        }
        if !self.production_denied {
            return Err("devnet release claim seal must deny production".to_string());
        }
        if !self.anti_equivocation_blockers_active {
            return Err("anti-equivocation blockers must remain active by default".to_string());
        }
        if !self.release_denied {
            return Err("release must remain denied by default".to_string());
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
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "readiness_wave": self.readiness_wave,
            "promotion_wave": self.promotion_wave,
            "release_claims_required": self.release_claims_required,
            "wave96_readiness_quorum_root": self.wave96_readiness_quorum_root,
            "wave95_promotion_root": self.wave95_promotion_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_denied": self.production_denied,
            "anti_equivocation_blockers_active": self.anti_equivocation_blockers_active,
            "release_denied": self.release_denied,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimRoot {
    pub lane: ReplayLane,
    pub claim_root: String,
    pub readiness_root: String,
    pub replay_token_root: String,
    pub command_hint_root: String,
    pub status: ReleaseStatus,
}

impl ReleaseClaimRoot {
    pub fn blocked(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            claim_root: stable_root("release-claim", lane.as_str()),
            readiness_root: config.wave96_readiness_quorum_root.clone(),
            replay_token_root: replay_token_root(lane),
            command_hint_root: lane_command_hint_root(lane),
            status: ReleaseStatus::Denied,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("claim_root", &self.claim_root)?;
        ensure_root("readiness_root", &self.readiness_root)?;
        ensure_root("replay_token_root", &self.replay_token_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.status != ReleaseStatus::Denied {
            return Err("devnet release claim must remain denied".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "claim_root": self.claim_root,
            "readiness_root": self.readiness_root,
            "replay_token_root": self.replay_token_root,
            "command_hint_root": self.command_hint_root,
            "status": self.status.as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-RELEASE-CLAIM-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayTokenRoot {
    pub lane: ReplayLane,
    pub token_root: String,
    pub prior_token_root: String,
    pub replay_guard_root: String,
    pub fork_guard_root: String,
    pub duplicate_guard_root: String,
    pub sealed: bool,
}

impl ReplayTokenRoot {
    pub fn blocked(lane: ReplayLane) -> Self {
        Self {
            lane,
            token_root: replay_token_root(lane),
            prior_token_root: stable_root("prior-replay-token", lane.as_str()),
            replay_guard_root: guard_root("replay", lane),
            fork_guard_root: guard_root("fork", lane),
            duplicate_guard_root: guard_root("duplicate", lane),
            sealed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("token_root", &self.token_root)?;
        ensure_root("prior_token_root", &self.prior_token_root)?;
        ensure_root("replay_guard_root", &self.replay_guard_root)?;
        ensure_root("fork_guard_root", &self.fork_guard_root)?;
        ensure_root("duplicate_guard_root", &self.duplicate_guard_root)?;
        if self.sealed {
            return Err("default runtime replay token cannot be sealed".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "token_root": self.token_root,
            "prior_token_root": self.prior_token_root,
            "replay_guard_root": self.replay_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "sealed": self.sealed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-REPLAY-TOKEN-ROOT", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AntiEquivocationSeal {
    pub lane: ReplayLane,
    pub claim_root: String,
    pub replay_token_root: String,
    pub blocker_roots: Vec<String>,
    pub release_seal_root: String,
    pub release_allowed: bool,
}

impl AntiEquivocationSeal {
    pub fn blocked(claim: &ReleaseClaimRoot, token: &ReplayTokenRoot) -> Self {
        let blocker_roots = AntiEquivocationBlocker::all()
            .iter()
            .map(|blocker| blocker_root(claim.lane, *blocker))
            .collect::<Vec<_>>();
        Self {
            lane: claim.lane,
            claim_root: claim.state_root(),
            replay_token_root: token.state_root(),
            blocker_roots,
            release_seal_root: stable_root("release-seal-placeholder", claim.lane.as_str()),
            release_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("claim_root", &self.claim_root)?;
        ensure_root("replay_token_root", &self.replay_token_root)?;
        ensure_root("release_seal_root", &self.release_seal_root)?;
        if self.blocker_roots.is_empty() {
            return Err("anti-equivocation blocker roots must be active".to_string());
        }
        for root in self.blocker_roots.iter() {
            ensure_root("blocker_root", root)?;
        }
        if self.release_allowed {
            return Err("release must remain denied until seal is live".to_string());
        }
        Ok(())
    }

    pub fn blocker_root(&self) -> String {
        list_root("WAVE97-SEAL-BLOCKER-ROOTS", self.blocker_roots.clone())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "claim_root": self.claim_root,
            "replay_token_root": self.replay_token_root,
            "blocker_roots_root": self.blocker_root(),
            "blocker_count": self.blocker_roots.len(),
            "release_seal_root": self.release_seal_root,
            "release_allowed": self.release_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-ANTI-EQUIVOCATION-SEAL", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardLedger {
    pub replay_guard_roots: BTreeMap<String, String>,
    pub fork_guard_roots: BTreeMap<String, String>,
    pub duplicate_guard_roots: BTreeMap<String, String>,
    pub blocker_roots: BTreeMap<String, String>,
}

impl GuardLedger {
    pub fn devnet(lanes: &[ReplayLane]) -> Self {
        let replay_guard_roots = lanes
            .iter()
            .map(|lane| (lane.as_str().to_string(), guard_root("replay", *lane)))
            .collect::<BTreeMap<_, _>>();
        let fork_guard_roots = lanes
            .iter()
            .map(|lane| (lane.as_str().to_string(), guard_root("fork", *lane)))
            .collect::<BTreeMap<_, _>>();
        let duplicate_guard_roots = lanes
            .iter()
            .map(|lane| (lane.as_str().to_string(), guard_root("duplicate", *lane)))
            .collect::<BTreeMap<_, _>>();
        let blocker_roots = AntiEquivocationBlocker::all()
            .iter()
            .map(|blocker| (blocker.as_str().to_string(), blocker_kind_root(*blocker)))
            .collect::<BTreeMap<_, _>>();
        Self {
            replay_guard_roots,
            fork_guard_roots,
            duplicate_guard_roots,
            blocker_roots,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_map_roots("replay_guard_roots", &self.replay_guard_roots)?;
        ensure_map_roots("fork_guard_roots", &self.fork_guard_roots)?;
        ensure_map_roots("duplicate_guard_roots", &self.duplicate_guard_roots)?;
        ensure_map_roots("blocker_roots", &self.blocker_roots)?;
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "replay_guard_root": map_root("WAVE97-REPLAY-GUARD-MAP", &self.replay_guard_roots),
            "fork_guard_root": map_root("WAVE97-FORK-GUARD-MAP", &self.fork_guard_roots),
            "duplicate_guard_root": map_root("WAVE97-DUPLICATE-GUARD-MAP", &self.duplicate_guard_roots),
            "blocker_catalog_root": map_root("WAVE97-BLOCKER-CATALOG-MAP", &self.blocker_roots),
            "replay_guard_count": self.replay_guard_roots.len(),
            "fork_guard_count": self.fork_guard_roots.len(),
            "duplicate_guard_count": self.duplicate_guard_roots.len(),
            "blocker_count": self.blocker_roots.len(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-GUARD-LEDGER", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub release_claim_roots: u64,
    pub replay_token_roots: u64,
    pub anti_equivocation_seals: u64,
    pub command_hints: u64,
    pub sealed_lanes: u64,
    pub release_allowed_lanes: u64,
    pub raw_public_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "release_claim_roots": self.release_claim_roots,
            "replay_token_roots": self.replay_token_roots,
            "anti_equivocation_seals": self.anti_equivocation_seals,
            "command_hints": self.command_hints,
            "sealed_lanes": self.sealed_lanes,
            "release_allowed_lanes": self.release_allowed_lanes,
            "raw_public_records": self.raw_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claim_roots: Vec<ReleaseClaimRoot>,
    pub replay_token_roots: Vec<ReplayTokenRoot>,
    pub anti_equivocation_seals: Vec<AntiEquivocationSeal>,
    pub guard_ledger: GuardLedger,
    pub command_hints: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        release_claim_roots: Vec<ReleaseClaimRoot>,
        replay_token_roots: Vec<ReplayTokenRoot>,
        anti_equivocation_seals: Vec<AntiEquivocationSeal>,
        guard_ledger: GuardLedger,
        command_hints: BTreeMap<String, String>,
    ) -> Result<Self> {
        let counters = Counters {
            release_claim_roots: release_claim_roots.len() as u64,
            replay_token_roots: replay_token_roots.len() as u64,
            anti_equivocation_seals: anti_equivocation_seals.len() as u64,
            command_hints: command_hints.len() as u64,
            sealed_lanes: replay_token_roots
                .iter()
                .filter(|token| token.sealed)
                .count() as u64,
            release_allowed_lanes: anti_equivocation_seals
                .iter()
                .filter(|seal| seal.release_allowed)
                .count() as u64,
            raw_public_records: 0,
        };
        let state = Self {
            config,
            release_claim_roots,
            replay_token_roots,
            anti_equivocation_seals,
            guard_ledger,
            command_hints,
            counters,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_unique_lanes(
            "release_claim_roots",
            self.release_claim_roots
                .iter()
                .map(|claim| claim.lane)
                .collect(),
        )?;
        ensure_unique_lanes(
            "replay_token_roots",
            self.replay_token_roots
                .iter()
                .map(|token| token.lane)
                .collect(),
        )?;
        ensure_unique_lanes(
            "anti_equivocation_seals",
            self.anti_equivocation_seals
                .iter()
                .map(|seal| seal.lane)
                .collect(),
        )?;
        for claim in self.release_claim_roots.iter() {
            claim.validate()?;
        }
        for token in self.replay_token_roots.iter() {
            token.validate()?;
        }
        for seal in self.anti_equivocation_seals.iter() {
            seal.validate()?;
        }
        self.guard_ledger.validate()?;
        ensure_map_roots("command_hints", &self.command_hints)?;
        if self.counters.sealed_lanes != 0 {
            return Err("devnet must have zero sealed replay lanes".to_string());
        }
        if self.counters.release_allowed_lanes != 0 {
            return Err("devnet must deny release for every lane".to_string());
        }
        if self.counters.raw_public_records != 0 {
            return Err("public record counter must remain roots only".to_string());
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "release_claim_roots_root": release_claim_roots_root(&self.release_claim_roots),
            "replay_token_roots_root": replay_token_roots_root(&self.replay_token_roots),
            "anti_equivocation_seals_root": anti_equivocation_seals_root(&self.anti_equivocation_seals),
            "guard_ledger_root": self.guard_ledger.state_root(),
            "command_hints_root": command_hints_root(&self.command_hints),
            "counters_root": self.counters.state_root(),
            "release_status": ReleaseStatus::Denied.as_str(),
            "anti_equivocation_blockers_active": self.config.anti_equivocation_blockers_active,
            "release_denied": self.config.release_denied,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "sealed_lane_count": self.counters.sealed_lanes,
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
        value_root("WAVE97-STATE", &self.public_record_without_state_root())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let lanes = ReplayLane::all();
    let release_claim_roots = lanes
        .iter()
        .map(|lane| ReleaseClaimRoot::blocked(*lane, &config))
        .collect::<Vec<_>>();
    let replay_token_roots = lanes
        .iter()
        .map(|lane| ReplayTokenRoot::blocked(*lane))
        .collect::<Vec<_>>();
    let anti_equivocation_seals = release_claim_roots
        .iter()
        .zip(replay_token_roots.iter())
        .map(|(claim, token)| AntiEquivocationSeal::blocked(claim, token))
        .collect::<Vec<_>>();
    let guard_ledger = GuardLedger::devnet(&lanes);
    let command_hints = CommandHint::all()
        .iter()
        .map(|hint| (hint.as_str().to_string(), command_hint_kind_root(*hint)))
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        release_claim_roots,
        replay_token_roots,
        anti_equivocation_seals,
        guard_ledger,
        command_hints,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            release_claim_roots: Vec::new(),
            replay_token_roots: Vec::new(),
            anti_equivocation_seals: Vec::new(),
            guard_ledger: GuardLedger::devnet(&[]),
            command_hints: BTreeMap::new(),
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

fn release_claim_roots_root(claims: &[ReleaseClaimRoot]) -> String {
    list_root(
        "WAVE97-RELEASE-CLAIM-STATE-ROOTS",
        claims.iter().map(ReleaseClaimRoot::state_root).collect(),
    )
}

fn replay_token_roots_root(tokens: &[ReplayTokenRoot]) -> String {
    list_root(
        "WAVE97-REPLAY-TOKEN-STATE-ROOTS",
        tokens.iter().map(ReplayTokenRoot::state_root).collect(),
    )
}

fn anti_equivocation_seals_root(seals: &[AntiEquivocationSeal]) -> String {
    list_root(
        "WAVE97-ANTI-EQUIVOCATION-SEAL-STATE-ROOTS",
        seals.iter().map(AntiEquivocationSeal::state_root).collect(),
    )
}

fn command_hints_root(hints: &BTreeMap<String, String>) -> String {
    map_root("WAVE97-COMMAND-HINTS", hints)
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

fn lane_command_hint_root(lane: ReplayLane) -> String {
    domain_hash(
        "WAVE97-LANE-COMMAND-HINT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn command_hint_kind_root(hint: CommandHint) -> String {
    domain_hash(
        "WAVE97-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: AntiEquivocationBlocker) -> String {
    domain_hash(
        "WAVE97-ANTI-EQUIVOCATION-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn blocker_kind_root(blocker: AntiEquivocationBlocker) -> String {
    domain_hash(
        "WAVE97-ANTI-EQUIVOCATION-BLOCKER-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn replay_token_root(lane: ReplayLane) -> String {
    domain_hash(
        "WAVE97-REPLAY-TOKEN-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str("release-claim-anti-equivocation-seal"),
        ],
        32,
    )
}

fn guard_root(kind: &str, lane: ReplayLane) -> String {
    domain_hash(
        "WAVE97-RUNTIME-REPLAY-GUARD",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn stable_root(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE97-STABLE-ROOT",
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
