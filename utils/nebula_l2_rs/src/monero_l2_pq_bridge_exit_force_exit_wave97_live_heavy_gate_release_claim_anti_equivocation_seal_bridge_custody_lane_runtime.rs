use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave97-release-claim-anti-equivocation-seal-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE92_SLOT_REGISTRY_ROOT: &str =
    "root:wave92:bridge-custody-receipt-slot-registry-fail-closed-placeholder";
pub const WAVE93_ADMISSION_ROOT: &str =
    "root:wave93:bridge-custody-admission-quarantine-empty-placeholder";
pub const WAVE94_STAGED_FILL_ROOT: &str =
    "root:wave94:bridge-custody-staged-fill-empty-placeholder";
pub const WAVE95_PROMOTION_ROOT: &str =
    "root:wave95:bridge-custody-slot-promotion-empty-placeholder";
pub const WAVE96_RELEASE_READINESS_ROOT: &str =
    "root:wave96:bridge-custody-release-readiness-quorum-denied-placeholder";
pub const DEFAULT_MIN_SEAL_BLOCKERS: u16 = 6;
pub const DEFAULT_MIN_REPLAY_GUARDS: u16 = 4;
pub const DEFAULT_MIN_FORK_GUARDS: u16 = 3;
pub const DEFAULT_MIN_DUPLICATE_GUARDS: u16 = 3;
pub const DEFAULT_SEAL_HOLD_BLOCKS: u64 = 1_440;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave92_slot_registry_root: String,
    pub wave93_admission_root: String,
    pub wave94_staged_fill_root: String,
    pub wave95_promotion_root: String,
    pub wave96_release_readiness_root: String,
    pub min_seal_blockers: u16,
    pub min_replay_guards: u16,
    pub min_fork_guards: u16,
    pub min_duplicate_guards: u16,
    pub seal_hold_blocks: u64,
    pub roots_only_public_records: bool,
    pub sealed_lanes_enabled: bool,
    pub release_enabled: bool,
    pub release_claims_enabled: bool,
    pub anti_equivocation_blockers_active: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave92_slot_registry_root: WAVE92_SLOT_REGISTRY_ROOT.to_string(),
            wave93_admission_root: WAVE93_ADMISSION_ROOT.to_string(),
            wave94_staged_fill_root: WAVE94_STAGED_FILL_ROOT.to_string(),
            wave95_promotion_root: WAVE95_PROMOTION_ROOT.to_string(),
            wave96_release_readiness_root: WAVE96_RELEASE_READINESS_ROOT.to_string(),
            min_seal_blockers: DEFAULT_MIN_SEAL_BLOCKERS,
            min_replay_guards: DEFAULT_MIN_REPLAY_GUARDS,
            min_fork_guards: DEFAULT_MIN_FORK_GUARDS,
            min_duplicate_guards: DEFAULT_MIN_DUPLICATE_GUARDS,
            seal_hold_blocks: DEFAULT_SEAL_HOLD_BLOCKS,
            roots_only_public_records: true,
            sealed_lanes_enabled: false,
            release_enabled: false,
            release_claims_enabled: false,
            anti_equivocation_blockers_active: true,
            heavy_gates_ran: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_root_like("wave92_slot_registry_root", &self.wave92_slot_registry_root)?;
        ensure_root_like("wave93_admission_root", &self.wave93_admission_root)?;
        ensure_root_like("wave94_staged_fill_root", &self.wave94_staged_fill_root)?;
        ensure_root_like("wave95_promotion_root", &self.wave95_promotion_root)?;
        ensure_root_like(
            "wave96_release_readiness_root",
            &self.wave96_release_readiness_root,
        )?;
        if self.min_seal_blockers == 0 {
            return Err("seal blocker quorum must be nonzero".to_string());
        }
        if self.min_replay_guards == 0 {
            return Err("replay guard quorum must be nonzero".to_string());
        }
        if self.min_fork_guards == 0 {
            return Err("fork guard quorum must be nonzero".to_string());
        }
        if self.min_duplicate_guards == 0 {
            return Err("duplicate guard quorum must be nonzero".to_string());
        }
        if self.seal_hold_blocks == 0 {
            return Err("seal hold window must be nonzero".to_string());
        }
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.sealed_lanes_enabled || self.release_enabled || self.release_claims_enabled {
            return Err("release claim seal must remain denied on devnet".to_string());
        }
        if !self.anti_equivocation_blockers_active {
            return Err("anti equivocation blockers must remain active".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave97 must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "lane_id": self.lane_id,
            "protocol_version": self.protocol_version,
            "wave92_slot_registry_root": self.wave92_slot_registry_root,
            "wave93_admission_root": self.wave93_admission_root,
            "wave94_staged_fill_root": self.wave94_staged_fill_root,
            "wave95_promotion_root": self.wave95_promotion_root,
            "wave96_release_readiness_root": self.wave96_release_readiness_root,
            "min_seal_blockers": self.min_seal_blockers,
            "min_replay_guards": self.min_replay_guards,
            "min_fork_guards": self.min_fork_guards,
            "min_duplicate_guards": self.min_duplicate_guards,
            "seal_hold_blocks": self.seal_hold_blocks,
            "roots_only_public_records": self.roots_only_public_records,
            "sealed_lanes_enabled": self.sealed_lanes_enabled,
            "release_enabled": self.release_enabled,
            "release_claims_enabled": self.release_claims_enabled,
            "anti_equivocation_blockers_active": self.anti_equivocation_blockers_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimRootKind {
    CustodyReleaseIntent,
    CustodyAccountSnapshot,
    ReserveCoverageSnapshot,
    WatcherSealVote,
    SignerSealVote,
    OperatorSealHold,
}

impl ClaimRootKind {
    pub fn all() -> [Self; 6] {
        [
            Self::CustodyReleaseIntent,
            Self::CustodyAccountSnapshot,
            Self::ReserveCoverageSnapshot,
            Self::WatcherSealVote,
            Self::SignerSealVote,
            Self::OperatorSealHold,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustodyReleaseIntent => "custody_release_intent",
            Self::CustodyAccountSnapshot => "custody_account_snapshot",
            Self::ReserveCoverageSnapshot => "reserve_coverage_snapshot",
            Self::WatcherSealVote => "watcher_seal_vote",
            Self::SignerSealVote => "signer_seal_vote",
            Self::OperatorSealHold => "operator_seal_hold",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardKind {
    Replay,
    Fork,
    Duplicate,
    PriorSealConflict,
}

impl GuardKind {
    pub fn all() -> [Self; 4] {
        [
            Self::Replay,
            Self::Fork,
            Self::Duplicate,
            Self::PriorSealConflict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::Fork => "fork",
            Self::Duplicate => "duplicate",
            Self::PriorSealConflict => "prior_seal_conflict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerKind {
    ReleaseDenied,
    HeavyGateMissing,
    LaneNotSealed,
    AntiEquivocationActive,
    ReplayGuardActive,
    ForkGuardActive,
    DuplicateGuardActive,
    CustodyAccountRootOnly,
    ClaimRootPlaceholder,
    CommandReviewRequired,
}

impl BlockerKind {
    pub fn all() -> [Self; 10] {
        [
            Self::ReleaseDenied,
            Self::HeavyGateMissing,
            Self::LaneNotSealed,
            Self::AntiEquivocationActive,
            Self::ReplayGuardActive,
            Self::ForkGuardActive,
            Self::DuplicateGuardActive,
            Self::CustodyAccountRootOnly,
            Self::ClaimRootPlaceholder,
            Self::CommandReviewRequired,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseDenied => "release_denied",
            Self::HeavyGateMissing => "heavy_gate_missing",
            Self::LaneNotSealed => "lane_not_sealed",
            Self::AntiEquivocationActive => "anti_equivocation_active",
            Self::ReplayGuardActive => "replay_guard_active",
            Self::ForkGuardActive => "fork_guard_active",
            Self::DuplicateGuardActive => "duplicate_guard_active",
            Self::CustodyAccountRootOnly => "custody_account_root_only",
            Self::ClaimRootPlaceholder => "claim_root_placeholder",
            Self::CommandReviewRequired => "command_review_required",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealStatus {
    Blocked,
    Sealed,
}

impl SealStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Sealed => "sealed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepReleaseDenied,
    KeepLaneUnsealed,
    RecomputeReleaseClaimRoots,
    RecomputeCustodyAccountRoots,
    AuditReplayGuard,
    AuditForkGuard,
    AuditDuplicateGuard,
    PreserveRootOnlyRecord,
    RecheckHeavyGateReceipt,
    ReissueSealAfterReview,
}

impl CommandHintKind {
    pub fn all() -> [Self; 10] {
        [
            Self::KeepReleaseDenied,
            Self::KeepLaneUnsealed,
            Self::RecomputeReleaseClaimRoots,
            Self::RecomputeCustodyAccountRoots,
            Self::AuditReplayGuard,
            Self::AuditForkGuard,
            Self::AuditDuplicateGuard,
            Self::PreserveRootOnlyRecord,
            Self::RecheckHeavyGateReceipt,
            Self::ReissueSealAfterReview,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseDenied => "keep_release_denied",
            Self::KeepLaneUnsealed => "keep_lane_unsealed",
            Self::RecomputeReleaseClaimRoots => "recompute_release_claim_roots",
            Self::RecomputeCustodyAccountRoots => "recompute_custody_account_roots",
            Self::AuditReplayGuard => "audit_replay_guard",
            Self::AuditForkGuard => "audit_fork_guard",
            Self::AuditDuplicateGuard => "audit_duplicate_guard",
            Self::PreserveRootOnlyRecord => "preserve_root_only_record",
            Self::RecheckHeavyGateReceipt => "recheck_heavy_gate_receipt",
            Self::ReissueSealAfterReview => "reissue_seal_after_review",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimRoot {
    pub kind: ClaimRootKind,
    pub claim_root: String,
    pub custody_account_root: String,
    pub release_seal_root: String,
    pub placeholder: bool,
}

impl ReleaseClaimRoot {
    pub fn placeholder(kind: ClaimRootKind) -> Self {
        let claim_root = placeholder_root("release-claim", kind.as_str());
        let custody_account_root = placeholder_root("custody-account", kind.as_str());
        let release_seal_root = record_root(
            "release-seal-placeholder",
            &json!({
                "kind": kind.as_str(),
                "claim_root": claim_root,
                "custody_account_root": custody_account_root,
                "sealed": false,
            }),
        );
        Self {
            kind,
            claim_root,
            custody_account_root,
            release_seal_root,
            placeholder: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "claim_root": self.claim_root,
            "custody_account_root": self.custody_account_root,
            "release_seal_root": self.release_seal_root,
            "placeholder": self.placeholder,
        })
    }

    pub fn root(&self) -> String {
        record_root("release-claim-root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardSeal {
    pub kind: GuardKind,
    pub guard_root: String,
    pub blocker_root: String,
    pub active: bool,
}

impl GuardSeal {
    pub fn active(kind: GuardKind) -> Self {
        let guard_root = placeholder_root("guard", kind.as_str());
        let blocker_root = record_root(
            "guard-blocker",
            &json!({
                "kind": kind.as_str(),
                "guard_root": guard_root,
                "active": true,
            }),
        );
        Self {
            kind,
            guard_root,
            blocker_root,
            active: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "guard_root": self.guard_root,
            "blocker_root": self.blocker_root,
            "active": self.active,
        })
    }

    pub fn root(&self) -> String {
        record_root("guard-seal", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AntiEquivocationBlocker {
    pub kind: BlockerKind,
    pub blocker_root: String,
    pub active: bool,
}

impl AntiEquivocationBlocker {
    pub fn active(kind: BlockerKind) -> Self {
        let blocker_root = record_root(
            "anti-equivocation-blocker",
            &json!({
                "kind": kind.as_str(),
                "active": true,
            }),
        );
        Self {
            kind,
            blocker_root,
            active: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "blocker_root": self.blocker_root,
            "active": self.active,
        })
    }

    pub fn root(&self) -> String {
        record_root("anti-equivocation-blocker-root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub command_root: String,
    pub blocks_release: bool,
}

impl CommandHint {
    pub fn canonical() -> Vec<Self> {
        CommandHintKind::all()
            .iter()
            .map(|kind| Self::new(*kind))
            .collect()
    }

    pub fn new(kind: CommandHintKind) -> Self {
        let command_root = record_root(
            "command-hint",
            &json!({
                "kind": kind.as_str(),
                "release_allowed": false,
            }),
        );
        Self {
            kind,
            command_root,
            blocks_release: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "blocks_release": self.blocks_release,
        })
    }

    pub fn root(&self) -> String {
        record_root("command-hint-root", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SealSummary {
    pub status: SealStatus,
    pub fail_closed: bool,
    pub sealed_lane_count: u16,
    pub release_allowed: bool,
    pub release_claim_count: u16,
    pub custody_account_root_count: u16,
    pub active_blocker_count: u16,
    pub active_guard_count: u16,
    pub release_claim_root: String,
    pub custody_account_root: String,
    pub guard_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub anti_equivocation_seal_root: String,
    pub heavy_gates_ran: bool,
}

impl SealSummary {
    pub fn from_state(
        claims: &BTreeMap<String, ReleaseClaimRoot>,
        guards: &[GuardSeal],
        blockers: &[AntiEquivocationBlocker],
        command_hints: &[CommandHint],
        config: &Config,
    ) -> Self {
        let release_claim_root = roots_root(
            "release-claim-roots",
            claims.values().map(ReleaseClaimRoot::root).collect(),
        );
        let custody_account_root = roots_root(
            "custody-account-roots",
            claims
                .values()
                .map(|claim| claim.custody_account_root.clone())
                .collect(),
        );
        let guard_root = roots_root("guard-roots", guards.iter().map(GuardSeal::root).collect());
        let blocker_root = roots_root(
            "anti-equivocation-blocker-roots",
            blockers.iter().map(AntiEquivocationBlocker::root).collect(),
        );
        let command_hint_root = roots_root(
            "command-hint-roots",
            command_hints.iter().map(CommandHint::root).collect(),
        );
        let active_blocker_count = blockers.iter().filter(|blocker| blocker.active).count() as u16;
        let active_guard_count = guards.iter().filter(|guard| guard.active).count() as u16;
        let release_allowed = false;
        let status = if release_allowed {
            SealStatus::Sealed
        } else {
            SealStatus::Blocked
        };
        let seal_record = json!({
            "status": status.as_str(),
            "sealed_lane_count": 0,
            "release_allowed": release_allowed,
            "release_claim_root": release_claim_root,
            "custody_account_root": custody_account_root,
            "guard_root": guard_root,
            "blocker_root": blocker_root,
            "command_hint_root": command_hint_root,
            "active_blocker_count": active_blocker_count,
            "active_guard_count": active_guard_count,
            "anti_equivocation_blockers_active": config.anti_equivocation_blockers_active,
            "heavy_gates_ran": config.heavy_gates_ran,
        });
        Self {
            status,
            fail_closed: status == SealStatus::Blocked,
            sealed_lane_count: 0,
            release_allowed,
            release_claim_count: claims.len() as u16,
            custody_account_root_count: claims.len() as u16,
            active_blocker_count,
            active_guard_count,
            release_claim_root,
            custody_account_root,
            guard_root,
            blocker_root,
            command_hint_root,
            anti_equivocation_seal_root: record_root("anti-equivocation-seal", &seal_record),
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "sealed_lane_count": self.sealed_lane_count,
            "release_allowed": self.release_allowed,
            "release_claim_count": self.release_claim_count,
            "custody_account_root_count": self.custody_account_root_count,
            "active_blocker_count": self.active_blocker_count,
            "active_guard_count": self.active_guard_count,
            "release_claim_root": self.release_claim_root,
            "custody_account_root": self.custody_account_root,
            "guard_root": self.guard_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "anti_equivocation_seal_root": self.anti_equivocation_seal_root,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claim_roots: BTreeMap<String, ReleaseClaimRoot>,
    pub guards: Vec<GuardSeal>,
    pub blockers: Vec<AntiEquivocationBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub summary: SealSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let release_claim_roots = ClaimRootKind::all()
            .iter()
            .map(|kind| {
                let claim = ReleaseClaimRoot::placeholder(*kind);
                (kind.as_str().to_string(), claim)
            })
            .collect::<BTreeMap<_, _>>();
        let guards = GuardKind::all()
            .iter()
            .map(|kind| GuardSeal::active(*kind))
            .collect::<Vec<_>>();
        let blockers = BlockerKind::all()
            .iter()
            .map(|kind| AntiEquivocationBlocker::active(*kind))
            .collect::<Vec<_>>();
        let command_hints = CommandHint::canonical();
        let summary = SealSummary::from_state(
            &release_claim_roots,
            &guards,
            &blockers,
            &command_hints,
            &config,
        );
        Self {
            config,
            release_claim_roots,
            guards,
            blockers,
            command_hints,
            summary,
        }
    }

    pub fn recompute(&mut self) {
        self.summary = SealSummary::from_state(
            &self.release_claim_roots,
            &self.guards,
            &self.blockers,
            &self.command_hints,
            &self.config,
        );
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.summary.sealed_lane_count != 0 {
            return Err("devnet must expose zero sealed lanes".to_string());
        }
        if self.summary.release_allowed {
            return Err("devnet release must remain denied".to_string());
        }
        if !self.summary.fail_closed {
            return Err("release claim anti equivocation seal must fail closed".to_string());
        }
        if self.summary.active_blocker_count < self.config.min_seal_blockers {
            return Err("active blocker quorum is too small".to_string());
        }
        if self.summary.active_guard_count < self.config.min_replay_guards {
            return Err("active guard quorum is too small".to_string());
        }
        for claim in self.release_claim_roots.values() {
            ensure_root_like("claim_root", &claim.claim_root)?;
            ensure_root_like("custody_account_root", &claim.custody_account_root)?;
            ensure_root_like("release_seal_root", &claim.release_seal_root)?;
            if !claim.placeholder {
                return Err("devnet release claims must remain placeholders".to_string());
            }
        }
        for guard in &self.guards {
            ensure_root_like("guard_root", &guard.guard_root)?;
            ensure_root_like("guard_blocker_root", &guard.blocker_root)?;
            if !guard.active {
                return Err("devnet guards must remain active".to_string());
            }
        }
        for blocker in &self.blockers {
            ensure_root_like("anti_equivocation_blocker_root", &blocker.blocker_root)?;
            if !blocker.active {
                return Err("anti equivocation blockers must remain active".to_string());
            }
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave97_bridge_custody_release_claim_anti_equivocation_seal",
            "config": self.config.public_record(),
            "wave92_slot_registry_root": self.config.wave92_slot_registry_root,
            "wave93_admission_root": self.config.wave93_admission_root,
            "wave94_staged_fill_root": self.config.wave94_staged_fill_root,
            "wave95_promotion_root": self.config.wave95_promotion_root,
            "wave96_release_readiness_root": self.config.wave96_release_readiness_root,
            "release_claim_count": self.release_claim_roots.len() as u64,
            "release_claim_root": self.summary.release_claim_root,
            "custody_account_root": self.summary.custody_account_root,
            "anti_equivocation_guard_root": self.summary.guard_root,
            "anti_equivocation_blocker_root": self.summary.blocker_root,
            "command_hint_root": self.summary.command_hint_root,
            "anti_equivocation_seal_root": self.summary.anti_equivocation_seal_root,
            "summary": self.summary.public_record(),
            "roots_only_public_records": true,
            "sealed_lanes_enabled": false,
            "release_enabled": false,
            "release_allowed": false,
            "anti_equivocation_blockers_active": true,
            "heavy_gates_ran": false,
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

fn placeholder_root(domain: &str, label: &str) -> String {
    record_root(
        domain,
        &json!({
            "label": label,
            "placeholder": true,
            "release_allowed": false,
        }),
    )
}

fn roots_root(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<_>>();
    format!("root:wave97:{}", merkle_root(domain, &leaves))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE97-BRIDGE-CUSTODY-RELEASE-CLAIM-ANTI-EQUIVOCATION-SEAL",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave97:{hash}")
}

fn ensure_text(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{field} must be nonempty"));
    }
    Ok(())
}

fn ensure_root_like(field: &'static str, value: &str) -> Result<()> {
    ensure_text(field, value)?;
    if !(value.starts_with("root:") || value.len() >= 32) {
        return Err(format!("{field} must be a root commitment"));
    }
    for private_marker in [
        "txid",
        "address",
        "spend_key",
        "view_key",
        "signer_label",
        "payload",
        "route",
    ] {
        if value.contains(private_marker) {
            return Err(format!("{field} contains private material marker"));
        }
    }
    Ok(())
}
